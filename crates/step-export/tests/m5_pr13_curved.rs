//! M5 PR 13 acceptance: the curved STEP subset.
//!
//! What is under test is a **structural** mapping — kernel geometry to
//! native AP214 entities — so most rows here are ε-INDEPENDENT by
//! construction and say so where it matters. The one place the ambient
//! tolerance reaches the file is the `UNCERTAINTY_MEASURE_WITH_UNIT`
//! value, which the writer copies from `Tolerance::get()` (or the
//! explicit override the fixtures use); that single dependence is
//! pinned by [`epsilon_reaches_only_the_uncertainty_record`], and the
//! one new refusal arm is run at two tolerances by
//! [`curved_multi_shell_refuses_at_both_tolerances`]. No other row
//! would change value if ε moved, because no other row reads a
//! distance: they compare emitted floats to the body's OWN stored
//! floats, bit for bit.
//!
//! The rows fall in four groups:
//!
//! 1. **The exactness table** — every `Surface`/`Curve3` variant that
//!    a body at rest carries emits its native entity, with the
//!    kernel's stored fields reproduced bit-exactly, and a body emits
//!    `B_SPLINE_*` records exactly when its KERNEL geometry is NURBS
//!    (since M6-3 `loft_prism` genuinely is; every analytic kind still
//!    refuses the rational-quadratic road, never approximated).
//! 2. **The `same_sense` composition** — the S10 review's rule (bound
//!    orientation composes with `same_sense`; never double-compose)
//!    now that `.F.` faces really reach the emitter, with two pins
//!    aimed directly at the double-composition bug.
//! 3. **The NURBS containment pin** — spline geometry appears exactly
//!    where the kernel put it (loft_prism's 4 walls + 4 seams), and
//!    nowhere else.
//! 4. **Determinism and the refusal arms.**

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use std::collections::HashMap;

use geom_core::Tolerance;
use geom_curves::Curve3;
use geom_surfaces::Surface;
use step_export::{StepExportError, StepOptions, step_string};

/// The fixture options (matching `tests/export.rs` and the fixture
/// generator: an EXPLICIT uncertainty, so nothing here reads ambient ε
/// unless the row is about ambient ε).
fn export(body: &topo::Body<f64>, name: &str) -> String {
    let options = StepOptions {
        product_name: name.to_owned(),
        uncertainty_m: Some(1e-9),
        ..StepOptions::default()
    };
    step_string(body, &options).unwrap()
}

/// The curved half of the committed corpus.
fn curved_corpus() -> Vec<(&'static str, topo::Body<f64>)> {
    common::fixture_corpus()
        .into_iter()
        .filter(|(name, _)| !matches!(*name, "cube" | "die" | "kiss_assembly"))
        .collect()
}

/// Every `#id = KEYWORD(args);` record of the emitted text, by id.
/// (The writer emits exactly one record per line — `writer.rs`'s
/// `emit`; complex instances open with `(` and get an empty keyword.)
fn records(text: &str) -> HashMap<u64, (String, String)> {
    let mut out = HashMap::new();
    for line in text.lines() {
        let Some(rest) = line.strip_prefix('#') else {
            continue;
        };
        let Some((id, body)) = rest.split_once(" = ") else {
            continue;
        };
        let body = body.trim_end().strip_suffix(';').expect("record ends in ;");
        let (keyword, args) = match body.split_once('(') {
            Some((kw, rest)) => (
                kw.trim().to_owned(),
                rest.strip_suffix(')').expect("closes paren").to_owned(),
            ),
            None => (String::new(), body.to_owned()),
        };
        out.insert(id.parse().expect("entity id"), (keyword, args));
    }
    out
}

/// The `#id` references in argument text, in order.
fn refs(args: &str) -> Vec<u64> {
    let bytes = args.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'#' {
            let start = i + 1;
            let mut end = start;
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            if end > start {
                out.push(args[start..end].parse().expect("entity ref"));
            }
            i = end;
        } else {
            i += 1;
        }
    }
    out
}

/// The top-level (unparenthesized) arguments of a record.
fn top_args(args: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut current = String::new();
    for ch in args.chars() {
        match ch {
            '(' => {
                depth += 1;
                current.push(ch);
            }
            ')' => {
                depth -= 1;
                current.push(ch);
            }
            ',' if depth == 0 => out.push(std::mem::take(&mut current).trim().to_owned()),
            _ => current.push(ch),
        }
    }
    out.push(current.trim().to_owned());
    out
}

/// Parses a Part 21 real back to f64 — the round-trip the float
/// printer promises (`real.rs`): the token parses to the IDENTICAL
/// bits, so every comparison below can be `==` and not a tolerance.
fn real(token: &str) -> f64 {
    token.parse().expect("Part 21 real")
}

/// The three coordinates of a CARTESIAN_POINT / DIRECTION record.
fn triple(rec: &(String, String)) -> [f64; 3] {
    let open = rec.1.find('(').expect("coordinate list");
    let close = rec.1.rfind(')').expect("list closes");
    let nums: Vec<f64> = rec.1[open + 1..close]
        .split(',')
        .map(|s| real(s.trim()))
        .collect();
    assert_eq!(nums.len(), 3, "three components");
    [nums[0], nums[1], nums[2]]
}

/// `(location, axis, ref_direction)` of an AXIS2_PLACEMENT_3D.
fn placement(recs: &HashMap<u64, (String, String)>, id: u64) -> ([f64; 3], [f64; 3], [f64; 3]) {
    let rec = &recs[&id];
    assert_eq!(rec.0, "AXIS2_PLACEMENT_3D");
    let r = refs(&rec.1);
    (
        triple(&recs[&r[0]]),
        triple(&recs[&r[1]]),
        triple(&recs[&r[2]]),
    )
}

// ==================================================================
// 1. The exactness table
// ==================================================================

/// **Every curved fixture emits native analytic entities, and NOTHING
/// emits a B-spline.** The second half is the load-bearing one: a
/// writer that quietly approximated an ellipse as a rational quadratic
/// (or a torus as a NURBS patch) would still import, still measure the
/// right volume, and still pass every count — and would have thrown
/// away the exactness this crate exists for. There is exactly one way
/// to catch that, which is to assert the approximation is absent.
#[test]
fn the_curved_corpus_emits_native_entities_and_no_b_splines() {
    // (fixture, the entities it must contain)
    let expected: &[(&str, &[&str])] = &[
        (
            "cut_cylinder",
            &["CYLINDRICAL_SURFACE", "ELLIPSE", "CIRCLE"],
        ),
        ("boss_union", &["CYLINDRICAL_SURFACE", "CIRCLE"]),
        ("notched", &["CYLINDRICAL_SURFACE", "CIRCLE"]),
        ("washer", &["CYLINDRICAL_SURFACE", "CIRCLE"]),
        ("ball", &["SPHERICAL_SURFACE", "CIRCLE"]),
        ("cone", &["CONICAL_SURFACE", "CIRCLE"]),
        ("donut", &["TOROIDAL_SURFACE", "CIRCLE"]),
        // The M5 PR 12 die blank: three elementary kinds in ONE solid,
        // its blends and corners meeting along tangent trimlines that
        // are lines and circles — all four exact, none approximated.
        (
            "filleted_die",
            &[
                "PLANE",
                "CYLINDRICAL_SURFACE",
                "SPHERICAL_SURFACE",
                "CIRCLE",
                "LINE",
            ],
        ),
        // The M6 curation addition: 21 spherical dimples in a sharp
        // cube — planes, sphere caps, and their exact circle rims.
        (
            "die_pips",
            &["PLANE", "SPHERICAL_SURFACE", "CIRCLE", "LINE"],
        ),
        // The M6 globe-lily unit: a sphere ZONE (neither pole on the
        // face) meeting a CONE across an exact circle, capped by two
        // planar discs -- three elementary kinds on one revolve, and
        // the cone is a genuine frustum rather than an apex fan.
        (
            "lily_lantern",
            &[
                "PLANE",
                "SPHERICAL_SURFACE",
                "CONICAL_SURFACE",
                "CIRCLE",
                "LINE",
            ],
        ),
        // The M6 composed die (unit 1's surgery): the blank's three
        // kinds PLUS the rim-fillet tori — four elementary surface
        // kinds in one solid, every carrier a line or a circle, none
        // approximated.
        (
            "composed_die",
            &[
                "PLANE",
                "CYLINDRICAL_SURFACE",
                "SPHERICAL_SURFACE",
                "TOROIDAL_SURFACE",
                "CIRCLE",
                "LINE",
            ],
        ),
        // The M6-3 loft: the corpus's first body whose KERNEL geometry
        // is genuinely NURBS — 4 described non-rational walls and their
        // 4 seam carriers. For it the spline entities are the native
        // encoding, not an approximation; the no-B_SPLINE law below is
        // scoped off it by the body's own kernel state.
        (
            "loft_prism",
            &[
                "PLANE",
                "B_SPLINE_SURFACE_WITH_KNOTS",
                "B_SPLINE_CURVE_WITH_KNOTS",
                "LINE",
            ],
        ),
    ];
    for (name, body) in curved_corpus() {
        let text = export(&body, name);
        let wanted = expected
            .iter()
            .find(|(n, _)| *n == name)
            .unwrap_or_else(|| panic!("{name} is not in the exactness table"))
            .1;
        for entity in wanted {
            assert!(
                text.contains(&format!("= {entity}(")),
                "{name} must emit {entity}"
            );
        }
        // The no-B_SPLINE law, scoped HONESTLY (M6-3 reshape, second
        // shape of this assertion): the load-bearing claim is that
        // ANALYTIC kernel geometry is never approximated as a spline.
        // Whether a body may emit B_SPLINE is therefore read off the
        // body's own kernel state, not off a hand-kept list — a fixture
        // whose faces and carriers are all analytic must emit none, and
        // a genuinely NURBS-walled body (loft_prism is the first) must
        // emit exactly as many surface records as it has NURBS faces
        // (a writer converting splines to fitted analytics would be the
        // dual smuggle).
        let nurbs_faces = body
            .faces()
            .filter(|(_, f)| {
                matches!(
                    body.get_surface(f.surface).expect("surface resolves"),
                    Surface::Nurbs(_)
                )
            })
            .count();
        if nurbs_faces == 0 {
            assert!(
                !text.contains("B_SPLINE"),
                "{name} must not smuggle a spline approximation of analytic geometry"
            );
        } else {
            let emitted = text
                .lines()
                .filter(|l| l.contains("B_SPLINE_SURFACE_WITH_KNOTS"))
                .count();
            assert_eq!(
                emitted, nurbs_faces,
                "{name}: every kernel NURBS face exports natively, none approximated away"
            );
        }
        // The refusal doors stay shut on the corpus, too: nothing here
        // is a POLY_LOOP / faceted stand-in either.
        assert!(!text.contains("POLY_LOOP"), "{name}");
    }
    // And the fifth surface kind, CONICAL_SURFACE, uses the apex
    // placement: `radius = 0.0` with the location AT the apex, which
    // is the encoding that invents no offset constant.
    let text = export(&common::cone(), "cone");
    assert!(text.contains("CONICAL_SURFACE('', #"));
    for line in text.lines().filter(|l| l.contains("= CONICAL_SURFACE(")) {
        let args = top_args(line.split_once('(').unwrap().1.rsplit_once(')').unwrap().0);
        assert_eq!(args[2], "0.0", "apex placement: radius 0");
        // semi_angle in (0, π/2), the schema's WHERE rule on
        // conical_surface — and the kernel's own convention.
        let angle = real(&args[3]);
        assert!(
            angle > 0.0 && angle < core::f64::consts::FRAC_PI_2,
            "{angle}"
        );
    }
}

/// **The emitted fields ARE the kernel's fields, bit for bit.** For
/// every curved face and every conic carrier of every curved fixture,
/// walk to the entity's `AXIS2_PLACEMENT_3D` and compare location,
/// axis and ref_direction — and the radii/angles — with `==` against
/// the body's stored geometry. Exact equality is the right assertion:
/// the float printer round-trips to identical bits (`real.rs`), and
/// the writer is forbidden to renormalize or reorder anything.
///
/// This is the row behind the docs' word "identity". A writer that
/// merely produced a *geometrically equivalent* placement — a
/// renormalized axis, a rotated seam, a cone placement offset down the
/// axis — would pass every import check and fail here.
#[test]
fn emitted_placements_equal_the_kernel_frames_bitwise() {
    for (name, body) in curved_corpus() {
        let text = export(&body, name);
        let recs = records(&text);

        // Entity ids are allocated along the writer's fixed traversal,
        // so the k-th ADVANCED_FACE record belongs to the k-th face of
        // that walk (`common::walk_order` mirrors it — NOT arena
        // order, which diverges on boolean results).
        let mut faces: Vec<u64> = recs
            .iter()
            .filter(|(_, (kw, _))| kw == "ADVANCED_FACE")
            .map(|(&id, _)| id)
            .collect();
        faces.sort_unstable();
        let (kernel_faces, _) = common::walk_order(&body);
        assert_eq!(
            faces.len(),
            kernel_faces.len(),
            "{name}: one record per face"
        );

        for (face_id, face_key) in faces.iter().zip(&kernel_faces) {
            let face = body.get_face(*face_key).expect("face resolves");
            let surface_id = *refs(&recs[face_id].1).last().expect("surface ref");
            let (kw, args) = &recs[&surface_id];
            let surface = body.get_surface(face.surface).expect("surface");
            let a = top_args(args);
            // a[0] is the name, a[1] the placement ref.
            let place = |recs: &HashMap<u64, (String, String)>| {
                placement(recs, refs(&a[1]).first().copied().expect("placement ref"))
            };
            match *surface {
                Surface::Plane { .. } => assert_eq!(kw, "PLANE", "{name}"),
                Surface::Cylinder {
                    origin,
                    axis,
                    radius,
                    u_ref,
                } => {
                    assert_eq!(kw, "CYLINDRICAL_SURFACE", "{name}");
                    let (l, z, x) = place(&recs);
                    assert_eq!(l, [origin.x, origin.y, origin.z], "{name} cylinder origin");
                    assert_eq!(z, [axis.x, axis.y, axis.z], "{name} cylinder axis");
                    assert_eq!(x, [u_ref.x, u_ref.y, u_ref.z], "{name} cylinder u_ref");
                    assert_eq!(real(&a[2]), radius, "{name} cylinder radius");
                }
                Surface::Cone {
                    apex,
                    axis,
                    half_angle,
                    u_ref,
                } => {
                    assert_eq!(kw, "CONICAL_SURFACE", "{name}");
                    let (l, z, x) = place(&recs);
                    assert_eq!(l, [apex.x, apex.y, apex.z], "{name} cone apex");
                    assert_eq!(z, [axis.x, axis.y, axis.z], "{name} cone axis");
                    assert_eq!(x, [u_ref.x, u_ref.y, u_ref.z], "{name} cone u_ref");
                    assert_eq!(real(&a[2]), 0.0, "{name} cone radius at the apex");
                    assert_eq!(real(&a[3]), half_angle, "{name} cone semi-angle");
                }
                Surface::Sphere {
                    center,
                    radius,
                    axis,
                    u_ref,
                } => {
                    assert_eq!(kw, "SPHERICAL_SURFACE", "{name}");
                    let (l, z, x) = place(&recs);
                    assert_eq!(l, [center.x, center.y, center.z], "{name} sphere centre");
                    assert_eq!(z, [axis.x, axis.y, axis.z], "{name} sphere axis");
                    assert_eq!(x, [u_ref.x, u_ref.y, u_ref.z], "{name} sphere u_ref");
                    assert_eq!(real(&a[2]), radius, "{name} sphere radius");
                }
                Surface::Torus {
                    center,
                    axis,
                    major_radius,
                    minor_radius,
                    u_ref,
                } => {
                    assert_eq!(kw, "TOROIDAL_SURFACE", "{name}");
                    let (l, z, x) = place(&recs);
                    assert_eq!(l, [center.x, center.y, center.z], "{name} torus centre");
                    assert_eq!(z, [axis.x, axis.y, axis.z], "{name} torus axis");
                    assert_eq!(x, [u_ref.x, u_ref.y, u_ref.z], "{name} torus u_ref");
                    assert_eq!(real(&a[2]), major_radius, "{name} torus major radius");
                    assert_eq!(real(&a[3]), minor_radius, "{name} torus minor radius");
                }
                // Since M6-3 the loft walls are NURBS at rest. A spline
                // record has no AXIS2_PLACEMENT — the frame claim
                // becomes: every CONTROL POINT is the kernel's, bit for
                // bit, in the writer's row-major (u-outer) order, and
                // the degrees are the knot vectors' own.
                Surface::Nurbs(ref ns) => {
                    assert_eq!(kw, "B_SPLINE_SURFACE_WITH_KNOTS", "{name}: non-rational");
                    assert_eq!(real(&a[1]), ns.knots_u().degree() as f64, "{name} u-degree");
                    assert_eq!(real(&a[2]), ns.knots_v().degree() as f64, "{name} v-degree");
                    let control_refs = refs(&a[3]);
                    assert_eq!(control_refs.len(), ns.control().len(), "{name} net size");
                    for (r, p) in control_refs.iter().zip(ns.control()) {
                        assert_eq!(
                            triple(&recs[r]),
                            [p.x, p.y, p.z],
                            "{name}: control point bitwise"
                        );
                    }
                }
            }
        }
    }
}

/// The conic CARRIER half of the same claim, on the two fixtures that
/// carry each kind: `cut_cylinder` (the corpus's only `Ellipse`) and
/// `donut` (four `Circle` meridians/parallels). Each `EDGE_CURVE`'s
/// geometry record is matched against the edge's certified carrier.
#[test]
fn emitted_conic_carriers_equal_the_kernel_carriers_bitwise() {
    for (name, body) in [
        ("cut_cylinder", common::cut_cylinder()),
        ("donut", common::donut()),
        ("ball", common::ball()),
    ] {
        let text = export(&body, name);
        let recs = records(&text);
        let mut curves: Vec<u64> = recs
            .iter()
            .filter(|(_, (kw, _))| kw == "EDGE_CURVE")
            .map(|(&id, _)| id)
            .collect();
        curves.sort_unstable();
        // First-encounter order along the writer's walk, not arena
        // order (`common::walk_order`).
        let (_, kernel_edges) = common::walk_order(&body);
        assert_eq!(curves.len(), kernel_edges.len(), "{name}: one per edge");

        let mut saw_ellipse = false;
        for (ec_id, edge_key) in curves.iter().zip(&kernel_edges) {
            let geometry_id = refs(&recs[ec_id].1)[2];
            let (kw, args) = &recs[&geometry_id];
            let a = top_args(args);
            let carrier = common::certified_carrier(&body, *edge_key);
            let place = || placement(&recs, refs(&a[1])[0]);
            match *carrier {
                Curve3::Line { .. } => assert_eq!(kw, "LINE", "{name}"),
                Curve3::Circle {
                    center,
                    axis,
                    radius,
                    u_ref,
                } => {
                    assert_eq!(kw, "CIRCLE", "{name}");
                    let (l, z, x) = place();
                    assert_eq!(l, [center.x, center.y, center.z], "{name} circle centre");
                    assert_eq!(z, [axis.x, axis.y, axis.z], "{name} circle axis");
                    assert_eq!(x, [u_ref.x, u_ref.y, u_ref.z], "{name} circle u_ref");
                    assert_eq!(real(&a[2]), radius, "{name} circle radius");
                }
                Curve3::Ellipse {
                    center,
                    axis,
                    major,
                    minor,
                    u_ref,
                } => {
                    saw_ellipse = true;
                    assert_eq!(kw, "ELLIPSE", "{name}");
                    let (l, z, x) = place();
                    assert_eq!(l, [center.x, center.y, center.z], "{name} ellipse centre");
                    assert_eq!(z, [axis.x, axis.y, axis.z], "{name} ellipse axis");
                    assert_eq!(x, [u_ref.x, u_ref.y, u_ref.z], "{name} ellipse u_ref");
                    // semi_axis_1 along ref_direction is the MAJOR: the
                    // kernel's `u_ref` is the semi-major direction, and
                    // AP214's first semi-axis is measured along
                    // ref_direction. Swapping them would export a
                    // rotated ellipse that still imports.
                    assert_eq!(real(&a[2]), major, "{name} ellipse semi_axis_1");
                    assert_eq!(real(&a[3]), minor, "{name} ellipse semi_axis_2");
                    assert!(major > minor, "the kernel's strict ordering survives");
                }
                Curve3::Nurbs(_) => panic!("{name}: no body at rest carries a NURBS carrier"),
            }
        }
        if name == "cut_cylinder" {
            assert!(saw_ellipse, "the tilted section rim is an ellipse");
        }
    }
}

// ==================================================================
// 2. same_sense composition (the S10 review's rule)
// ==================================================================

/// **Anti-double-composition pin A: every bound orientation stays
/// `.T.`, on every fixture, including the ones with reversed faces.**
///
/// ISO 10303-42 COMPOSES a bound's orientation flag with the owning
/// face's `same_sense`. Our loops are stored CCW about the face's
/// OUTWARD normal, and `same_sense` already states how that normal
/// relates to the chart normal — so the composition comes out right
/// only if the bound flag is left alone. Flipping it as well (the
/// obvious "be consistent" mistake) would compose the reversal twice
/// and hand every reader an inside-out face. Before PR 13 this could
/// not be tested on real output, because no `.F.` face could reach the
/// emitter.
#[test]
fn every_bound_orientation_is_true_even_on_reversed_faces() {
    let mut reversed_seen = 0usize;
    for (name, body) in common::fixture_corpus() {
        let text = export(&body, name);
        for line in text.lines() {
            if line.contains("= FACE_OUTER_BOUND(") || line.contains("= FACE_BOUND(") {
                assert!(
                    line.ends_with(".T.);"),
                    "{name}: a bound orientation was flipped — double composition"
                );
            }
            if line.contains("= ADVANCED_FACE(") && line.contains(".F.") {
                reversed_seen += 1;
            }
        }
    }
    // The corpus really does contain reversed faces, so the row above
    // is not vacuous: 91 = notched 1 + washer 2 + cone 2 (the
    // original five) + die_pips 21·2 (each pip's two sense:false
    // half-band walls, S11 discipline) + the M6 composed die's 21·2
    // (the same half-caps, carried through the surgery) + the globe
    // lily's lantern 2 (its MOUTH disc's two half-bands: a revolve
    // mints both cap planes on the profile plane's own +y normal, so
    // the cap facing −y opposes the solid's outward normal and the
    // one facing +y agrees — exactly one of the two caps reverses,
    // and each cap is two half-bands). The M6-3 loft_prism adds ZERO:
    // it mirrors extrude's minting (M5-LOG item 6(i)) — the bottom
    // cap's LOOP is reversed at mint so its plane derives normal-down
    // (outward), and every skinned wall chart's normal S_u × S_v
    // follows the material-left traversal (loft.rs module docs,
    // "Orientation") — so all six faces keep sense = true.
    assert_eq!(reversed_seen, 91, "the corpus's reversed faces");
}

/// **Anti-double-composition pin B: a reversed face exports its TRUE
/// surface.** The other half of the same bug: instead of (or as well
/// as) flipping the flag, a writer might negate the surface axis. That
/// also "looks right" and is also a double count. Here the axis of
/// every `.F.` face's surface is compared bitwise with the body's
/// stored CHART normal — not the outward normal — so a negation fails.
#[test]
fn a_reversed_face_keeps_its_chart_axis() {
    let mut checked = 0usize;
    for (name, body) in curved_corpus() {
        let text = export(&body, name);
        let recs = records(&text);
        let mut faces: Vec<u64> = recs
            .iter()
            .filter(|(_, (kw, _))| kw == "ADVANCED_FACE")
            .map(|(&id, _)| id)
            .collect();
        faces.sort_unstable();
        let (kernel_faces, _) = common::walk_order(&body);
        for (face_id, face_key) in faces.iter().zip(&kernel_faces) {
            let face = body.get_face(*face_key).expect("face resolves");
            let args = &recs[face_id].1;
            let same_sense = args.ends_with(".T.");
            assert_eq!(same_sense, face.sense, "{name}: same_sense IS Face::sense");
            if face.sense {
                continue;
            }
            checked += 1;
            let surface_id = *refs(args).last().expect("surface ref");
            let a = top_args(&recs[&surface_id].1);
            let (_, z, _) = placement(&recs, refs(&a[1])[0]);
            let chart = match *body.get_surface(face.surface).expect("surface") {
                Surface::Plane { normal, .. } => normal,
                Surface::Cylinder { axis, .. }
                | Surface::Cone { axis, .. }
                | Surface::Sphere { axis, .. }
                | Surface::Torus { axis, .. } => axis,
                // NURBS faces exist at rest since M6-3 (loft walls) but
                // none reverses (pin A's derivation) — and a NURBS chart
                // has no single axis to compare; if a reversed one ever
                // appears, design its axis-negation pin then.
                Surface::Nurbs(_) => panic!("a REVERSED NURBS face reached pin B — extend it"),
            };
            assert_eq!(
                z,
                [chart.x, chart.y, chart.z],
                "{name}: a .F. face must keep the chart axis, not negate it"
            );
        }
    }
    assert_eq!(
        checked, 91,
        "all 91 reversed faces checked (5 original + die_pips' 42 + the composed \
         die's 42 + the lily lantern's 2; loft_prism contributes 0 — every face \
         sense-true, see pin A's derivation)"
    );
}

/// **The S12 revert row.** `Body::revert` reverses the loops AND flips
/// `sense` on the curved arm; export both the ball and its revert and
/// the difference in the text must be exactly (a) the `same_sense`
/// flags and (b) the loop winding — never the surface record. The
/// sphere's `SPHERICAL_SURFACE`/`AXIS2_PLACEMENT_3D` bytes are
/// identical across the pair, which is what "the reversal is stated
/// once, in the one field STEP provides for it" means concretely.
///
/// This row is text-level because the external oracle cannot see the
/// distinction: both files were imported into FreeCAD 1.1.2 and both
/// report `valid: True` with the identical POSITIVE volume
/// (4188790204.79 mm³) — OCC's ShapeHealing rectifies an inside-out
/// shell without comment, the same blindness the M4 review found on
/// `cube.step`. The plan anticipated this ("if FreeCAD's checker can
/// see it; else pin the emitted text"); it cannot, so the text is
/// pinned, and `orientation_oracle.rs`'s double-composition negative
/// control carries the part FreeCAD would otherwise have carried.
#[test]
fn reverting_a_sphere_moves_the_flag_and_nothing_about_the_surface() {
    let ball = common::ball();
    let reverted = ball.revert().expect("a ball reverts");
    let a = export(&ball, "ball");
    let b = export(&reverted, "ball");
    assert_ne!(a, b, "revert is visible in the text");

    let count = |text: &str, needle: &str| text.matches(needle).count();
    assert_eq!(count(&a, "= ADVANCED_FACE("), 2);
    assert_eq!(count(&b, "= ADVANCED_FACE("), 2);
    // Every face flips: the ball's two bands are sense: true, the
    // revert's two are sense: false.
    assert_eq!(
        a.lines()
            .filter(|l| l.contains("= ADVANCED_FACE(") && l.ends_with(".T.);"))
            .count(),
        2
    );
    assert_eq!(
        b.lines()
            .filter(|l| l.contains("= ADVANCED_FACE(") && l.ends_with(".F.);"))
            .count(),
        2
    );
    // The surface itself is byte-identical in both files (same
    // placement, same radius, same ids — the walk is unchanged).
    let spherical = |text: &str| {
        text.lines()
            .filter(|l| l.contains("= SPHERICAL_SURFACE("))
            .map(|l| l.split_once(" = ").unwrap().1.to_owned())
            .collect::<Vec<_>>()
    };
    assert_eq!(spherical(&a), spherical(&b), "the true surface is exported");
    assert_eq!(spherical(&a).len(), 2);
    // And the bound flags stayed `.T.` on the reverted body too — the
    // reversal is stated exactly once.
    assert!(
        b.lines()
            .filter(|l| l.contains("FACE_OUTER_BOUND("))
            .all(|l| l.ends_with(".T.);"))
    );
}

// ==================================================================
// 3. The NURBS carrier arm
// ==================================================================

/// **The successor of `no_body_at_rest_carries_a_nurbs_carrier_or_face`
/// (flipped at M6-2; the history is this name and this note).**
///
/// The retired row pinned a VACUITY positively: that nothing anywhere
/// reached a rung-3 carrier at rest, which is what made "every fitted
/// pcurve cache carries the full C2 certificate" a statement about the
/// empty set. M6-2 lifted the SSI enclosure/certification stack off
/// `f64` and landed `Pcurve::Fitted`, and
/// `topo/tests/m6_2_fitted_at_rest.rs` now pins the POSITIVE law: a
/// cylinder×sphere rung-3 edge reaches a body at rest carrying a fitted
/// chart image whose hull sup-norm AND uniqueness tube are RE-DERIVED
/// by the tier-3 pcurve pass, at `f64` and at the interval scalar.
///
/// **SECOND flip of this pin's lineage (M6-3; the retired name is
/// `no_export_corpus_body_carries_a_nurbs_carrier_or_face`).** The
/// first flip (M6-2, the paragraph above) narrowed the retired at-rest
/// vacuity to the EXPORT CORPUS; this one retires the corpus absence
/// itself, BOTH
/// halves at once: `loft_prism` brought the first NURBS faces (its four
/// skinned walls) AND the first NURBS carriers (its four wall–wall seam
/// edges — the seams store the walls' u-boundary control rows, so the
/// carrier half flipped WITH the face half, not later). The
/// `B_SPLINE_SURFACE_WITH_KNOTS` and `B_SPLINE_CURVE_WITH_KNOTS` arms
/// now have an end-to-end body behind them; the record-level pins in
/// `writer.rs` still own the field-level facts (rational complex
/// instance, knot run-length encoding).
///
/// What this row pins now is CONTAINMENT, kernel-side and text-side:
/// NURBS geometry appears exactly where the kernel put it — on
/// `loft_prism`, in known counts — and nowhere else. A corpus document
/// that silently acquired a NURBS carrier (fitted lane, live since
/// M6-2) or a writer that manufactured a spline out of an analytic
/// carrier still fails here.
#[test]
fn nurbs_geometry_appears_exactly_where_the_kernel_put_it() {
    for (name, body) in common::fixture_corpus() {
        let nurbs_carriers = body
            .edges()
            .filter(|&(edge_key, _)| {
                matches!(common::certified_carrier(&body, edge_key), Curve3::Nurbs(_))
            })
            .count();
        let nurbs_faces = body
            .faces()
            .filter(|(_, f)| {
                matches!(
                    body.get_surface(f.surface).expect("surface resolves"),
                    Surface::Nurbs(_)
                )
            })
            .count();
        // loft_prism: 4 skinned walls, 4 seam carriers (the wall–wall
        // edges; the 8 cap rims stay exact placed lines). Everyone
        // else: none — a corpus document that acquired one moves this
        // pin deliberately, with its export row.
        let (want_faces, want_carriers) = match name {
            "loft_prism" => (4usize, 4usize),
            _ => (0, 0),
        };
        assert_eq!(
            (nurbs_faces, nurbs_carriers),
            (want_faces, want_carriers),
            "{name}: kernel NURBS census"
        );
        let text = export(&body, name);
        assert_eq!(
            text.contains("B_SPLINE"),
            want_faces + want_carriers > 0,
            "{name}: B_SPLINE text tracks the kernel census exactly"
        );
    }
    // The surface refusal is live and names the frontier.
    let mut skeleton = topo::Body::<f64>::new();
    skeleton
        .mvfs(geom_core::Point3::new(0.0, 0.0, 0.0))
        .unwrap();
    match step_string(&skeleton, &StepOptions::default()) {
        Err(StepExportError::UnsupportedSurface { kind, .. }) => {
            assert_eq!(kind, "nurbs placeholder");
        }
        other => panic!("expected UnsupportedSurface, got {other:?}"),
    }
}

// ==================================================================
// 4. Determinism, ε, and the refusal arms
// ==================================================================

/// Byte-determinism over the whole curved corpus: same body value ⇒
/// byte-identical file, and same RECIPE ⇒ byte-identical file (the
/// second is the stronger claim — it says no arena address, hash
/// order, or allocation history reaches the output).
#[test]
fn curved_exports_are_byte_deterministic() {
    for (name, body) in curved_corpus() {
        let first = export(&body, name);
        assert_eq!(first, export(&body, name), "{name}: same value");
    }
    // Same recipe, rebuilt from scratch.
    for (name, rebuilt) in curved_corpus() {
        let again = curved_corpus()
            .into_iter()
            .find(|(n, _)| *n == name)
            .expect("rebuilt")
            .1;
        assert_eq!(
            export(&rebuilt, name),
            export(&again, name),
            "{name}: same recipe"
        );
    }
}

/// **Where ε reaches the file, and where it does not.** The exports
/// are exact structure: the only ε-dependent byte in the whole
/// document is the `UNCERTAINTY_MEASURE_WITH_UNIT` value, which the
/// writer copies from the run's ambient tolerance (or the explicit
/// override used here — the ambient axis is a per-PROCESS env setting,
/// so the CI `CAD_EPS` matrix is what varies it, and the override is
/// the same value through the same code path). Export a curved body at
/// two tolerances a thousand-fold apart and the two texts differ in
/// exactly that one line — which is why every other row in this file
/// is stated at a single ε without apology: they compare emitted
/// floats to the body's own stored floats and never read a distance.
#[test]
fn epsilon_reaches_only_the_uncertainty_record() {
    let body = common::washer();
    let at = |eps: f64| {
        let options = StepOptions {
            uncertainty_m: Some(eps),
            ..StepOptions::default()
        };
        step_string(&body, &options).unwrap()
    };
    let tight = at(1e-9);
    let loose = at(1e-6);
    let differing: Vec<(&str, &str)> = tight
        .lines()
        .zip(loose.lines())
        .filter(|(a, b)| a != b)
        .collect();
    assert_eq!(differing.len(), 1, "one differing line: {differing:?}");
    assert!(differing[0].0.contains("UNCERTAINTY_MEASURE_WITH_UNIT"));
    assert!(differing[0].0.contains("LENGTH_MEASURE(1.0E-9)"));
    assert!(differing[0].1.contains("LENGTH_MEASURE(1.0E-6)"));
}

/// **The one new refusal arm, at two tolerances.** The outward/void
/// classifier did not grow curved closed forms, so a MULTI-shell
/// curved solid refuses even though every one of its faces has a
/// printer — the message says exactly that. S12's two-stub
/// `boss ∖ plate` complement is the only such body constructible at
/// rest.
///
/// The refusal is reached by a **type-level match** on the surface
/// variant, before any arithmetic: no distance, no comparison, no ε.
/// It is therefore ε-independent by derivation, and the two-tolerance
/// run below checks that rather than asserting it — both the writer's
/// own ε input (the uncertainty override, the only tolerance this
/// crate reads) and, in CI, the ambient `CAD_EPS` lane that rebuilds
/// the body itself. The body construction is pinned too: two shells at
/// whatever ε the process was built under.
#[test]
fn curved_multi_shell_refuses_at_both_tolerances() {
    let stubs = common::two_stub_complement();
    assert_eq!(stubs.shells().count(), 2, "two disjoint stubs");
    assert!(
        Tolerance::get().eps > 0.0,
        "the body above was built at the run's ambient tolerance"
    );
    for eps in [1e-9, 1e-6] {
        let options = StepOptions {
            uncertainty_m: Some(eps),
            ..StepOptions::default()
        };
        match step_string(&stubs, &options) {
            Err(StepExportError::CurvedShellClassification { kind, .. }) => {
                // The classifier walks a shell face-first and each
                // face surface-then-carriers, so the entity it meets
                // first on a stub is the planar CAP's circular rim —
                // not the cylinder wall one face later. Either way the
                // refusal is typed and names the geometry; the exact
                // kind is pinned so a change in walk order is visible.
                assert_eq!(kind, "circle", "at eps = {eps}");
            }
            other => panic!("expected CurvedShellClassification at {eps}, got {other:?}"),
        }
    }
}

/// The single-shell curved bodies do NOT reach the classifier — the
/// complement of the row above, and the reason the refusal is narrow
/// rather than a curved-export blocker.
#[test]
fn single_shell_curved_solids_never_reach_the_classifier() {
    for (name, body) in curved_corpus() {
        assert_eq!(body.shells().count(), 1, "{name} is single-shell");
        assert!(
            step_string(&body, &StepOptions::default()).is_ok(),
            "{name}"
        );
    }
}

/// **Regression guard for `common::walk_order` (review probe).** The
/// helper exists because the writer's traversal and the arena's
/// insertion order are different orders, and every row above that
/// matches an emitted record to a kernel entity is silently wrong if
/// they are confused. The trap is that they AGREE on simple swept
/// bodies — so a suite built only on primitives passes with the bug in
/// place, and only a boolean result exposes it.
///
/// This row pins both halves of that fact: divergence on the boolean
/// result (`boss_union`), agreement on a swept primitive (`washer`).
/// If a future change made arena order the writer's order, the first
/// assertion fires and someone gets to delete the helper deliberately
/// instead of by accident.
#[test]
fn walk_order_diverges_from_arena_order_on_boolean_results() {
    let boolean = common::boss_union();
    let (walk, _) = common::walk_order(&boolean);
    let arena: Vec<_> = boolean.faces().map(|(k, _)| k).collect();
    assert_eq!(walk.len(), arena.len(), "same faces, different order");
    assert_ne!(
        walk, arena,
        "boss_union is the body that exposes the arena-vs-walk trap"
    );
    // Same multiset, so the divergence is ordering and nothing else.
    let mut a = walk.clone();
    let mut b = arena.clone();
    a.sort_unstable();
    b.sort_unstable();
    assert_eq!(a, b);

    // And the agreement half — why the trap is easy to miss.
    let swept = common::washer();
    let (walk, _) = common::walk_order(&swept);
    let arena: Vec<_> = swept.faces().map(|(k, _)| k).collect();
    assert_eq!(
        walk, arena,
        "a swept primitive's two orders coincide, which is the trap"
    );
}

/// **The wireframe splice fixture stays in lockstep with the writer
/// (review probe).** `tests/fixtures/nurbs_wireframe.step` carries the
/// writer's `RATIONAL_B_SPLINE_CURVE` complex instance verbatim inside
/// a `GEOMETRIC_CURVE_SET`, so that an independent READER meets that
/// record even though no body at rest produces one. The record text
/// below is the same literal `writer.rs`'s
/// `mixed_weights_emit_the_rational_complex_instance` pins, so a
/// change to the emitter fails there and a drifting fixture fails
/// here.
///
/// FreeCAD's side of this is `nurbs_wireframe.probe.py`, run by
/// `scripts/check_step.sh`: OCC comes back with a RATIONAL degree-2
/// B-spline carrying the identical weights, and every sampled point
/// sits on the exact unit circle to ~3.4e-16 relative — the arm's only
/// reader-level validation before the loft-assembly unit.
#[test]
fn the_wireframe_splice_carries_the_writers_own_rational_record() {
    let path = format!(
        "{}/tests/fixtures/nurbs_wireframe.step",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(&path).expect("the wireframe fixture");
    let record = "( BOUNDED_CURVE() B_SPLINE_CURVE(2, (#1, #2, #3), .UNSPECIFIED., .U., .U.) \
                  B_SPLINE_CURVE_WITH_KNOTS((3, 3), (0.0, 1.0), .UNSPECIFIED.) CURVE() \
                  GEOMETRIC_REPRESENTATION_ITEM() RATIONAL_B_SPLINE_CURVE((1.0, \
                  0.7071067811865476, 1.0)) REPRESENTATION_ITEM('') )";
    assert!(
        text.contains(record),
        "the spliced record drifted from the writer's pinned output"
    );
    // The control net is the Eq. 7.33 quarter circle the probe checks.
    for point in [
        "#1 = CARTESIAN_POINT('', (1.0, 0.0, 0.0));",
        "#2 = CARTESIAN_POINT('', (1.0, 1.0, 0.0));",
        "#3 = CARTESIAN_POINT('', (0.0, 1.0, 0.0));",
    ] {
        assert!(text.contains(point), "control net: {point}");
    }
    // Not a corpus fixture: it holds no body, so the byte-golden row
    // must not try to regenerate it.
    assert!(
        !common::fixture_corpus()
            .iter()
            .any(|(n, _)| *n == "nurbs_wireframe")
    );
}
