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
//! The claims fall in four groups. Groups 1–3 are all statements about
//! the SAME fixture corpus, so since the test-cost audit they are all
//! made by ONE test —
//! [`the_export_corpus_obeys_the_exactness_frame_sense_and_nurbs_laws`]
//! — which builds the corpus once and labels every assertion with the
//! law it belongs to:
//!
//! 1. **The exactness table** (`EXACTNESS` / `NO-B-SPLINE` /
//!    `NO-POLY-LOOP` / `APEX` / `FRAME`) — every `Surface`/`Curve3`
//!    variant that a body at rest carries emits its native entity, with
//!    the kernel's stored fields reproduced bit-exactly, and a body
//!    emits `B_SPLINE_*` records exactly when its KERNEL geometry is
//!    NURBS (since M6-3 `loft_prism` genuinely is; every analytic kind
//!    still refuses the rational-quadratic road, never approximated).
//!    The `FRAME` half is the row behind the docs' word "identity": a
//!    writer that merely produced a *geometrically equivalent*
//!    placement — a renormalized axis, a rotated seam, a cone placement
//!    offset down the axis — would pass every import check and fail
//!    there. The conic CARRIER half is still its own row,
//!    [`emitted_conic_carriers_equal_the_kernel_carriers_bitwise`].
//! 2. **The `same_sense` composition** (`BOUND` / `CHART-AXIS`) — the
//!    S10 review's rule (bound orientation composes with `same_sense`;
//!    never double-compose) now that `.F.` faces really reach the
//!    emitter, with two pins aimed directly at the double-composition
//!    bug. The S12 revert row keeps its own test.
//! 3. **The NURBS containment pin** (`CENSUS`) — spline geometry
//!    appears exactly where the kernel put it (loft_prism's 4 walls +
//!    4 seams, and the #210 fold's two skinned documents), and nowhere
//!    else, plus the placeholder-surface refusal.
//!
//!    *Lineage (the names are cited in `docs/M5-EXIT-WALK.md`)*:
//!    `CENSUS` succeeds `no_body_at_rest_carries_a_nurbs_carrier_or_face`,
//!    `no_export_corpus_body_carries_a_nurbs_carrier_or_face` and
//!    `nurbs_geometry_appears_exactly_where_the_kernel_put_it`. The
//!    record-level pins in `writer.rs` own the field-level facts
//!    (rational complex instance, knot run-length encoding); what
//!    survives here is CONTAINMENT: a corpus document that silently
//!    acquired a NURBS carrier (fitted lane) or a writer that
//!    manufactured a spline out of an analytic carrier still fails.
//! 4. **Determinism and the refusal arms** — their own rows, because
//!    [`curved_exports_are_byte_deterministic`] builds the corpus TWICE
//!    on purpose and that second build IS its content.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use std::collections::HashMap;

use geom::Curve3;
use geom::Surface;
use geom_core::Tolerance;
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
// 1. The corpus laws: exactness, bitwise frames, same_sense, NURBS
//    containment — all on ONE corpus build
// ==================================================================

/// **The corpus's export laws, on ONE corpus build.**
///
/// Five rows that all walked the same fixture corpus, now one:
///
/// 1. **Exactness table** (was `the_curved_corpus_emits_native_\
///    entities_and_no_b_splines`) — every curved fixture emits native
///    analytic entities, and NOTHING emits a B-spline it has no kernel
///    NURBS for. The second half is the load-bearing one: a writer that
///    quietly approximated an ellipse as a rational quadratic (or a
///    torus as a NURBS patch) would still import, still measure the
///    right volume, and still pass every count — and would have thrown
///    away the exactness this crate exists for. There is exactly one
///    way to catch that, which is to assert the approximation is
///    absent.
/// 2. **Bitwise frames** (was `emitted_placements_equal_the_kernel_\
///    frames_bitwise`) — see the `FRAME` block below.
/// 3. **`same_sense` pin A** (was `every_bound_orientation_is_true_\
///    even_on_reversed_faces`) — the `BOUND` assertions, run over the
///    WHOLE corpus, planar fixtures included.
/// 4. **`same_sense` pin B** (was `a_reversed_face_keeps_its_chart_\
///    axis`) — the `CHART-AXIS` assertions.
/// 5. **NURBS containment** (was `nurbs_geometry_appears_exactly_\
///    where_the_kernel_put_it`) — the `CENSUS` assertions, whole
///    corpus, plus the placeholder-surface refusal.
/// 6. **Single-shell reachability** (was `single_shell_curved_solids_\
///    never_reach_the_classifier`) — the `SINGLE-SHELL` assertions.
///
/// # One corpus build, every law on it
///
/// Each of those six rows called `common::fixture_corpus()` (directly
/// or through `curved_corpus`, which is that list filtered) and paid
/// the whole 17-document build — booleans, fillet surgeries, a
/// 21-dimple die, three skinned lofts — for a few hundred milliseconds
/// of parsing and comparison. Under nextest's process-per-test
/// isolation nothing was shared between them, so the corpus was built
/// six times per ε row. It is built ONCE here, and each body's STEP
/// text is emitted once and read by every law that wants it.
///
/// What the split bought and a merged row cannot is failure ISOLATION:
/// six independent properties now surface under one test id. So every
/// assertion below NAMES its law — `EXACTNESS`, `NO-B-SPLINE`,
/// `NO-POLY-LOOP`, `APEX`, `FRAME`, `BOUND`, `CHART-AXIS`, `CENSUS`,
/// `SINGLE-SHELL` — and the message alone says which one broke. Keep
/// that discipline when adding assertions here.
///
/// The byte-determinism row (`curved_exports_are_byte_deterministic`)
/// stays separate ON PURPOSE: it builds the corpus TWICE, and the
/// second build is its whole content.
#[test]
fn the_export_corpus_obeys_the_exactness_frame_sense_and_nurbs_laws() {
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
        // The #210 corpus fold: the same native-NURBS vocabulary on the
        // two bodies #207's skin-fit fix made exportable. Identical rows
        // to `loft_prism`, and that IS the row's content — a
        // non-uniformly spaced loft and a curved-path sweep have no new
        // entity kinds to offer. What they had until #207 was
        // RATIONAL_B_SPLINE_* records for geometry that is not rational;
        // that is pinned dead in `m7_swept_elbow.rs`.
        (
            "nonuniform_loft",
            &[
                "PLANE",
                "B_SPLINE_SURFACE_WITH_KNOTS",
                "B_SPLINE_CURVE_WITH_KNOTS",
                "LINE",
            ],
        ),
        (
            "swept_elbow",
            &[
                "PLANE",
                "B_SPLINE_SURFACE_WITH_KNOTS",
                "B_SPLINE_CURVE_WITH_KNOTS",
                "LINE",
            ],
        ),
    ];

    // THE one corpus build. INVARIANT: everything below reads THIS
    // list — no row may call `fixture_corpus()` / `curved_corpus()`
    // again, which is the whole point of the merge.
    let corpus = common::fixture_corpus();
    let corpus_names: Vec<&'static str> = corpus.iter().map(|(n, _)| *n).collect();
    // Pin A and pin B count reversed faces over DIFFERENT sets (the
    // whole corpus vs the curved half), and both must reach 91 — the
    // planar fixtures contribute none. Two counters, deliberately.
    let mut reversed_seen = 0usize;
    let mut chart_axis_checked = 0usize;
    // The corpus's cone text, kept for the apex-placement row: the
    // corpus's `cone` entry IS `common::cone()` (common/mod.rs), and
    // exporting the same body value twice is byte-identical by the
    // determinism row, so re-building it here would be pure waste.
    let mut cone_text: Option<String> = None;

    for (name, body) in &corpus {
        let name = *name;
        let text = export(body, name);
        if name == "cone" {
            cone_text = Some(text.clone());
        }

        // ---- BOUND (pin A): every bound orientation stays `.T.`, on
        // EVERY fixture including the planar ones, and the corpus
        // really does contain `.F.` faces so the pin is not vacuous.
        for line in text.lines() {
            if line.contains("= FACE_OUTER_BOUND(") || line.contains("= FACE_BOUND(") {
                assert!(
                    line.ends_with(".T.);"),
                    "BOUND: {name}: a bound orientation was flipped — double composition"
                );
            }
            if line.contains("= ADVANCED_FACE(") && line.contains(".F.") {
                reversed_seen += 1;
            }
        }

        // ---- CENSUS: NURBS geometry appears exactly where the kernel
        // put it, kernel-side and text-side, on EVERY fixture.
        let nurbs_carriers = body
            .edges()
            .filter(|&(edge_key, _)| {
                matches!(common::certified_carrier(body, edge_key), Curve3::Nurbs(_))
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
        // The three NURBS-walled documents — loft_prism and the #210
        // fold's two — each: 4 skinned walls, 4 seam carriers (the
        // wall–wall edges; the 8 cap rims stay exact placed lines).
        // Everyone else: none — a corpus document that acquired one
        // moves this pin deliberately, with its export row.
        let (want_faces, want_carriers) = match name {
            "loft_prism" | "nonuniform_loft" | "swept_elbow" => (4usize, 4usize),
            _ => (0, 0),
        };
        assert_eq!(
            (nurbs_faces, nurbs_carriers),
            (want_faces, want_carriers),
            "CENSUS: {name}: kernel NURBS census"
        );
        assert_eq!(
            text.contains("B_SPLINE"),
            want_faces + want_carriers > 0,
            "CENSUS: {name}: B_SPLINE text tracks the kernel census exactly"
        );

        // The planar third of the corpus is out of the curved rows'
        // scope, exactly as `curved_corpus()` scoped them.
        if matches!(name, "cube" | "die" | "kiss_assembly") {
            continue;
        }

        // ---- EXACTNESS: the native entities this fixture must emit.
        let wanted = expected
            .iter()
            .find(|(n, _)| *n == name)
            .unwrap_or_else(|| panic!("EXACTNESS: {name} is not in the exactness table"))
            .1;
        for entity in wanted {
            assert!(
                text.contains(&format!("= {entity}(")),
                "EXACTNESS: {name} must emit {entity}"
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
        if nurbs_faces == 0 {
            assert!(
                !text.contains("B_SPLINE"),
                "NO-B-SPLINE: {name} must not smuggle a spline approximation of analytic \
                 geometry"
            );
        } else {
            let emitted = text
                .lines()
                .filter(|l| l.contains("B_SPLINE_SURFACE_WITH_KNOTS"))
                .count();
            assert_eq!(
                emitted, nurbs_faces,
                "NO-B-SPLINE: {name}: every kernel NURBS face exports natively, none \
                 approximated away"
            );
        }
        // The refusal doors stay shut on the corpus, too: nothing here
        // is a POLY_LOOP / faceted stand-in either.
        assert!(!text.contains("POLY_LOOP"), "NO-POLY-LOOP: {name}");

        // ---- FRAME + CHART-AXIS: one face walk serves both.
        //
        // Entity ids are allocated along the writer's fixed traversal,
        // so the k-th ADVANCED_FACE record belongs to the k-th face of
        // that walk (`common::walk_order` mirrors it — NOT arena
        // order, which diverges on boolean results).
        let recs = records(&text);
        let mut faces: Vec<u64> = recs
            .iter()
            .filter(|(_, (kw, _))| kw == "ADVANCED_FACE")
            .map(|(&id, _)| id)
            .collect();
        faces.sort_unstable();
        let (kernel_faces, _) = common::walk_order(body);
        assert_eq!(
            faces.len(),
            kernel_faces.len(),
            "FRAME: {name}: one record per face"
        );

        for (face_id, face_key) in faces.iter().zip(&kernel_faces) {
            let face = body.get_face(*face_key).expect("face resolves");
            let face_args = &recs[face_id].1;
            let surface_id = *refs(face_args).last().expect("surface ref");
            let (kw, args) = &recs[&surface_id];
            let surface = body.get_surface(face.surface).expect("surface");
            let a = top_args(args);
            // a[0] is the name, a[1] the placement ref.
            let place = |recs: &HashMap<u64, (String, String)>| {
                placement(recs, refs(&a[1]).first().copied().expect("placement ref"))
            };
            match *surface {
                Surface::Plane { .. } => assert_eq!(kw, "PLANE", "FRAME: {name}"),
                Surface::Cylinder {
                    origin,
                    axis,
                    radius,
                    u_ref,
                } => {
                    assert_eq!(kw, "CYLINDRICAL_SURFACE", "FRAME: {name}");
                    let (l, z, x) = place(&recs);
                    assert_eq!(
                        l,
                        [origin.x, origin.y, origin.z],
                        "FRAME: {name} cylinder origin"
                    );
                    assert_eq!(z, [axis.x, axis.y, axis.z], "FRAME: {name} cylinder axis");
                    assert_eq!(
                        x,
                        [u_ref.x, u_ref.y, u_ref.z],
                        "FRAME: {name} cylinder u_ref"
                    );
                    assert_eq!(real(&a[2]), radius, "FRAME: {name} cylinder radius");
                }
                Surface::Cone {
                    apex,
                    axis,
                    half_angle,
                    u_ref,
                } => {
                    assert_eq!(kw, "CONICAL_SURFACE", "FRAME: {name}");
                    let (l, z, x) = place(&recs);
                    assert_eq!(l, [apex.x, apex.y, apex.z], "FRAME: {name} cone apex");
                    assert_eq!(z, [axis.x, axis.y, axis.z], "FRAME: {name} cone axis");
                    assert_eq!(x, [u_ref.x, u_ref.y, u_ref.z], "FRAME: {name} cone u_ref");
                    assert_eq!(real(&a[2]), 0.0, "FRAME: {name} cone radius at the apex");
                    assert_eq!(real(&a[3]), half_angle, "FRAME: {name} cone semi-angle");
                }
                Surface::Sphere {
                    center,
                    radius,
                    axis,
                    u_ref,
                } => {
                    assert_eq!(kw, "SPHERICAL_SURFACE", "FRAME: {name}");
                    let (l, z, x) = place(&recs);
                    assert_eq!(
                        l,
                        [center.x, center.y, center.z],
                        "FRAME: {name} sphere centre"
                    );
                    assert_eq!(z, [axis.x, axis.y, axis.z], "FRAME: {name} sphere axis");
                    assert_eq!(x, [u_ref.x, u_ref.y, u_ref.z], "FRAME: {name} sphere u_ref");
                    assert_eq!(real(&a[2]), radius, "FRAME: {name} sphere radius");
                }
                Surface::Torus {
                    center,
                    axis,
                    major_radius,
                    minor_radius,
                    u_ref,
                } => {
                    assert_eq!(kw, "TOROIDAL_SURFACE", "FRAME: {name}");
                    let (l, z, x) = place(&recs);
                    assert_eq!(
                        l,
                        [center.x, center.y, center.z],
                        "FRAME: {name} torus centre"
                    );
                    assert_eq!(z, [axis.x, axis.y, axis.z], "FRAME: {name} torus axis");
                    assert_eq!(x, [u_ref.x, u_ref.y, u_ref.z], "FRAME: {name} torus u_ref");
                    assert_eq!(
                        real(&a[2]),
                        major_radius,
                        "FRAME: {name} torus major radius"
                    );
                    assert_eq!(
                        real(&a[3]),
                        minor_radius,
                        "FRAME: {name} torus minor radius"
                    );
                }
                // Since M6-3 the loft walls are NURBS at rest. A spline
                // record has no AXIS2_PLACEMENT — the frame claim
                // becomes: every CONTROL POINT is the kernel's, bit for
                // bit, in the writer's row-major (u-outer) order, and
                // the degrees are the knot vectors' own.
                Surface::Nurbs(ref ns) => {
                    assert_eq!(
                        kw, "B_SPLINE_SURFACE_WITH_KNOTS",
                        "FRAME: {name}: non-rational"
                    );
                    assert_eq!(
                        real(&a[1]),
                        ns.knots_u().degree() as f64,
                        "FRAME: {name} u-degree"
                    );
                    assert_eq!(
                        real(&a[2]),
                        ns.knots_v().degree() as f64,
                        "FRAME: {name} v-degree"
                    );
                    let control_refs = refs(&a[3]);
                    assert_eq!(
                        control_refs.len(),
                        ns.control().len(),
                        "FRAME: {name} net size"
                    );
                    for (r, p) in control_refs.iter().zip(ns.control()) {
                        assert_eq!(
                            triple(&recs[r]),
                            [p.x, p.y, p.z],
                            "FRAME: {name}: control point bitwise"
                        );
                    }
                }
            }

            // ---- CHART-AXIS (pin B): `same_sense` IS `Face::sense`,
            // and a `.F.` face keeps its CHART axis rather than
            // negating it.
            let same_sense = face_args.ends_with(".T.");
            assert_eq!(
                same_sense, face.sense,
                "CHART-AXIS: {name}: same_sense IS Face::sense"
            );
            if face.sense {
                continue;
            }
            chart_axis_checked += 1;
            // The chart axis is read BEFORE the placement is walked: a
            // NURBS record has no `AXIS2_PLACEMENT_3D` at all, so a
            // reversed NURBS face must hit the panic that names the
            // missing pin, not an index panic inside `placement`.
            let chart = match *surface {
                Surface::Plane { normal, .. } => normal,
                Surface::Cylinder { axis, .. }
                | Surface::Cone { axis, .. }
                | Surface::Sphere { axis, .. }
                | Surface::Torus { axis, .. } => axis,
                // NURBS faces exist at rest since M6-3 (loft walls) but
                // none reverses (pin A's derivation) — and a NURBS chart
                // has no single axis to compare; if a reversed one ever
                // appears, design its axis-negation pin then.
                Surface::Nurbs(_) => {
                    panic!("CHART-AXIS: a REVERSED NURBS face reached pin B — extend it")
                }
            };
            let (_, z, _) = place(&recs);
            assert_eq!(
                z,
                [chart.x, chart.y, chart.z],
                "CHART-AXIS: {name}: a .F. face must keep the chart axis, not negate it"
            );
        }

        // ---- SINGLE-SHELL: the curved single-shell bodies never reach
        // the multi-shell classifier, on the DEFAULT options (a second
        // writer configuration, deliberately — this row is about the
        // classifier door, not about the fixture uncertainty).
        assert_eq!(
            body.shells().count(),
            1,
            "SINGLE-SHELL: {name} is single-shell"
        );
        assert!(
            step_string(body, &StepOptions::default()).is_ok(),
            "SINGLE-SHELL: {name}"
        );
    }

    // The corpus really does contain reversed faces, so pin A above is
    // not vacuous: 91 = notched 1 + washer 2 + cone 2 (the
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
    assert_eq!(reversed_seen, 91, "BOUND: the corpus's reversed faces");
    assert_eq!(
        chart_axis_checked, 91,
        "CHART-AXIS: all 91 reversed faces checked (5 original + die_pips' 42 + the \
         composed die's 42 + the lily lantern's 2; loft_prism contributes 0 — every \
         face sense-true, see pin A's derivation). Every one is on a CURVED fixture, \
         which is why this equals the whole-corpus count above."
    );

    // ---- APEX: the fifth surface kind, CONICAL_SURFACE, uses the apex
    // placement: `radius = 0.0` with the location AT the apex, which
    // is the encoding that invents no offset constant.
    let text = cone_text.expect("APEX: the corpus carries the cone fixture");
    assert!(text.contains("CONICAL_SURFACE('', #"), "APEX");
    for line in text.lines().filter(|l| l.contains("= CONICAL_SURFACE(")) {
        let args = top_args(line.split_once('(').unwrap().1.rsplit_once(')').unwrap().0);
        assert_eq!(args[2], "0.0", "APEX: apex placement: radius 0");
        // semi_angle in (0, π/2), the schema's WHERE rule on
        // conical_surface — and the kernel's own convention.
        let angle = real(&args[3]);
        assert!(
            angle > 0.0 && angle < core::f64::consts::FRAC_PI_2,
            "APEX: {angle}"
        );
    }

    // ---- CENSUS, the refusal half: the surface refusal is live and
    // names the frontier.
    let mut skeleton = topo::Body::<f64>::new();
    skeleton
        .mvfs(geom_core::Point3::new(0.0, 0.0, 0.0))
        .unwrap();
    match step_string(&skeleton, &StepOptions::default()) {
        Err(StepExportError::UnsupportedSurface { kind, .. }) => {
            assert_eq!(kind, "nurbs placeholder", "CENSUS");
        }
        other => panic!("CENSUS: expected UnsupportedSurface, got {other:?}"),
    }

    // The wireframe splice fixture is NOT a corpus fixture: it holds no
    // body, so the byte-golden row must not try to regenerate it. (The
    // record-level half of that row lives in
    // `the_wireframe_splice_carries_the_writers_own_rational_record`,
    // which needs no corpus build at all.)
    assert!(
        !corpus_names.contains(&"nurbs_wireframe"),
        "CENSUS: the wireframe splice is not a corpus document"
    );
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
// 2. same_sense composition (the S10 review's rule) — the two
//    anti-double-composition pins now live in the corpus row above
// ==================================================================

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
// 3. Determinism, ε, and the refusal arms
// ==================================================================

/// Byte-determinism over the whole curved corpus: same body value ⇒
/// byte-identical file, and same RECIPE ⇒ byte-identical file (the
/// second is the stronger claim — it says no arena address, hash
/// order, or allocation history reaches the output).
#[test]
fn curved_exports_are_byte_deterministic() {
    // Exactly TWO corpus builds: `first_build`, and `second_build`
    // rebuilt from scratch from the same recipes. INVARIANT: the corpus
    // is a fixed ordered list, so zipping pairs each document with its
    // OWN rebuild — the name equality asserted on every step is what
    // makes that pairing checkable rather than assumed.
    let first_build = curved_corpus();
    let second_build = curved_corpus();
    assert!(!first_build.is_empty(), "the curved corpus is non-empty");
    assert_eq!(
        first_build.len(),
        second_build.len(),
        "corpus length must not depend on the build"
    );
    for ((name, body), (rebuilt_name, rebuilt)) in first_build.into_iter().zip(second_build) {
        assert_eq!(name, rebuilt_name, "{name}: corpus order differs by build");
        let first = export(&body, name);
        // Same VALUE ⇒ same bytes.
        assert_eq!(first, export(&body, name), "{name}: same value");
        // Same RECIPE ⇒ same bytes: the stronger claim, and the reason
        // the independent rebuild has to exist at all.
        assert_eq!(first, export(&rebuilt, name), "{name}: same recipe");
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
///
/// The COMPLEMENT — the single-shell curved solids never reaching the
/// classifier, which is why this refusal is narrow rather than a
/// curved-export blocker — is the `SINGLE-SHELL` block of the corpus
/// row (retired name:
/// `single_shell_curved_solids_never_reach_the_classifier`).
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
///
/// The half of this row that needed a corpus build — "and it is NOT a
/// corpus fixture, so the byte-golden row must not try to regenerate
/// it" — moved to the `CENSUS` tail of the corpus row, which already
/// has the corpus in hand. This row now reads one file and nothing
/// else, which is why it costs milliseconds.
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
}
