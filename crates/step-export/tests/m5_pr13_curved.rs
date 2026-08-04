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
//!    kernel's stored fields reproduced bit-exactly, and NO body emits
//!    a `B_SPLINE_*` (the conics do not take the rational-quadratic
//!    road, the analytic surfaces are never approximated).
//! 2. **The `same_sense` composition** — the S10 review's rule (bound
//!    orientation composes with `same_sense`; never double-compose)
//!    now that `.F.` faces really reach the emitter, with two pins
//!    aimed directly at the double-composition bug.
//! 3. **The NURBS carrier arm**, which no body at rest exercises.
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
        assert!(
            !text.contains("B_SPLINE"),
            "{name} must not smuggle a spline approximation of analytic geometry"
        );
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
                Surface::Nurbs(_) => panic!("{name}: no body at rest carries a NURBS face"),
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
    // is not vacuous: 89 = notched 1 + washer 2 + cone 2 (the
    // original five) + die_pips 21·2 (each pip's two sense:false
    // half-band walls, S11 discipline) + the M6 composed die's 21·2
    // (the same half-caps, carried through the surgery).
    assert_eq!(reversed_seen, 89, "the corpus's reversed faces");
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
                Surface::Nurbs(_) => panic!("no NURBS face at rest"),
            };
            assert_eq!(
                z,
                [chart.x, chart.y, chart.z],
                "{name}: a .F. face must keep the chart axis, not negate it"
            );
        }
    }
    assert_eq!(
        checked, 89,
        "all 89 reversed faces checked (5 original + die_pips' 42 + the composed die's 42)"
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

/// The `B_SPLINE_CURVE_WITH_KNOTS` arm has **no body at rest behind
/// it**: the kernel's rung-3 SSI branches are the only mint site for a
/// `Curve3::Nurbs` carrier, and nothing in `sweep` or `topo`'s public
/// constructors reaches them (a hand-built scaffold in `topo`'s own
/// suite is the single certified rung-3 edge in the repo). The arm is
/// written because the entity is part of the curved subset the plan
/// names, and it is pinned at the record level in `writer.rs`'s unit
/// tests — including the rational complex instance and the knot
/// run-length encoding. This row states the frontier so the absence is
/// deliberate rather than an oversight: the loft-assembly unit brings
/// both the first NURBS carriers at rest and the first NURBS FACES,
/// and `B_SPLINE_SURFACE_WITH_KNOTS` lands with it.
///
/// The absence is checked on BOTH sides. At kernel level every carrier
/// and every surface of every corpus body is named, so the claim is
/// "the bodies do not have them" and not merely "the text does not
/// mention them"; at text level the `B_SPLINE` grep would still catch a
/// writer that manufactured a spline out of an analytic carrier.
#[test]
fn no_body_at_rest_carries_a_nurbs_carrier_or_face() {
    for (name, body) in common::fixture_corpus() {
        for (edge_key, _) in body.edges() {
            let kind = match common::certified_carrier(&body, edge_key) {
                Curve3::Line { .. } => "line",
                Curve3::Circle { .. } => "circle",
                Curve3::Ellipse { .. } => "ellipse",
                Curve3::Nurbs(_) => "nurbs",
            };
            assert_ne!(
                kind, "nurbs",
                "{name}: a NURBS carrier reached a body at rest"
            );
            assert!(
                matches!(kind, "line" | "circle" | "ellipse"),
                "{name}: unexpected carrier {kind}"
            );
        }
        for (_, face) in body.faces() {
            let surface = body.get_surface(face.surface).expect("surface resolves");
            assert!(
                !matches!(surface, Surface::Nurbs(_)),
                "{name}: a NURBS face reached a body at rest"
            );
        }
        let text = export(&body, name);
        assert!(!text.contains("B_SPLINE"), "{name}");
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
