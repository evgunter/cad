//! Second-review probes for PR 4286 (not for merge): the one-sided
//! strip region `straight_reach`'s `replaces` now carries, and the typed
//! refusal `reach::screened` now gives a support vertex inside it.
//!
//! Every row asserts the PR's contract: a request either builds a body
//! that passes tier 3, tier 3′, one genus-0 shell, naming totality and
//! ΔV at the closed form (`carve`), or refuses with a typed clearance
//! refusal. Never `SurgeryInvariant`, `BodyNotIntact`, a panic, or a
//! wrong body. Each row prints its outcome so the report can quote it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::panic::{AssertUnwindSafe, catch_unwind};

use geom_core::{Point2, Point3, Vec3};
use sweep::blend::BlendError;
use sweep::test_support::{block, prism_on, realized, sketch_from_axes};
use topo::{Body, ContactRecords, EdgeKey, validate_geometric, validate_pseudomanifold};

use crate::band_planar_cut_off::{D, Verb, carve, edge, tol};

#[derive(Debug, Clone, PartialEq)]
enum Outcome {
    Built,
    BuiltBad(String),
    Refused(String, String),
}

fn variant(e: &BlendError) -> String {
    let s = format!("{e:?}");
    s.split([' ', '{', '(']).next().unwrap_or("").to_string()
}

fn gap_of(e: &BlendError) -> String {
    match e {
        BlendError::FaceClearanceUncertified { gap, margin, .. } => format!(
            "gap {:?} margin {:?}",
            gap.diagnostic_f64_for_error_text(),
            margin.reading.diagnostic_f64_for_error_text()
        ),
        BlendError::RingClearance { margin, .. } => {
            format!(
                "margin {:?}",
                margin.reading.diagnostic_f64_for_error_text()
            )
        }
        BlendError::Escalated { site, decision, .. } => format!("{site:?}/{decision:?}"),
        BlendError::FaceClearance { bounded, .. } => format!("bounded {bounded}"),
        _ => String::new(),
    }
}

/// Run one request; a built body is held to `carve`'s checks plus tier 3′.
fn run(body: &Body<f64>, edges: &[EdgeKey], verb: Verb, removed: f64, what: &str) -> Outcome {
    match verb.run(body, edges) {
        Err(e) => Outcome::Refused(variant(&e), gap_of(&e)),
        Ok(_) => match catch_unwind(AssertUnwindSafe(|| {
            let out = carve(body, edges, verb, removed, what);
            validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
                .unwrap_or_else(|err| panic!("{what}: tier 3′, got {err:?}"));
        })) {
            Ok(()) => Outcome::Built,
            Err(p) => Outcome::BuiltBad(
                p.downcast_ref::<String>()
                    .cloned()
                    .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
                    .unwrap_or_default(),
            ),
        },
    }
}

/// The contract every probe row holds: built and checked, or a typed
/// clearance refusal.
fn sound(o: &Outcome, what: &str) {
    eprintln!("PROBE {what}: {o:?}");
    match o {
        Outcome::Built => {}
        Outcome::Refused(v, _)
            if [
                "FaceClearanceUncertified",
                "RingClearance",
                "FaceClearance",
                "Escalated",
            ]
            .contains(&v.as_str()) => {}
        other => panic!("{what}: neither a checked body nor a typed clearance refusal: {other:?}"),
    }
}

fn outline_body(outline: &[(f64, f64)], convex: bool) -> Body<f64> {
    let outline: Vec<(Point2<f64>, f64)> = outline
        .iter()
        .map(|&(x, y)| (Point2::new(x, y), 0.0))
        .collect();
    let body = if convex {
        realized(
            topo::boolean::BooleanOp::Subtract,
            &block(6.0, 5.0, 2.0, tol()),
            &sweep::test_support::prism_at(outline, 1.0, 2.0, tol()),
            tol(),
        )
    } else {
        realized(
            topo::boolean::BooleanOp::Union,
            &block(6.0, 5.0, 1.0, tol()),
            &sweep::test_support::prism_at(outline, 0.5, 1.5, tol()),
            tol(),
        )
    };
    validate_geometric(&body, tol()).expect("the probe input is tier-3 valid");
    body
}

const TAB: [(f64, f64); 2] = [(3.5, 2.5), (2.5, 2.5)];

/// The tabbed outline with a peninsula of the support rising from the
/// pocket's far wall to `(x, 2.5 − gap)`, `half` wide at its base.
fn far_side_at(x: f64, gap: f64, half: f64, open: bool) -> Vec<(f64, f64)> {
    let x0 = if open { -1.0 } else { 1.0 };
    vec![
        (x0, 1.0),
        (x - half, 1.0),
        (x, 2.5 - gap),
        (x + half, 1.0),
        (5.0, 1.0),
        (5.0, 4.0),
        (3.5, 4.0),
        TAB[0],
        TAB[1],
        (2.5, 4.0),
        (x0, 4.0),
    ]
}

/// **Far-side peninsulas at and around the gap, across the window.**
/// Tip `x` from beyond one station to beyond the other (between and on
/// the screen's samples), `gap` from clear of the setback to `0.01`;
/// ring and outer cycle, pocket and boss, both verbs. The peninsula lies
/// across the tip edge's line, so a built body removes the prism closed
/// form exactly.
#[test]
fn far_side_peninsulas_build_or_refuse_typed() {
    let mut built = 0;
    let mut refused = 0;
    for open in [false, true] {
        for convex in [true, false] {
            let z = if convex { 2.0 } else { 1.0 };
            for x in [2.45, 2.5, 2.52, 2.5625, 3.0, 3.0625, 3.48, 3.5, 3.55] {
                for gap in [0.2, 0.101, 0.099, 0.09, 0.08, 0.05, 0.01] {
                    let body = outline_body(&far_side_at(x, gap, 0.0375, open), convex);
                    let e = edge(&body, [3.5, 2.5, z], [2.5, 2.5, z]);
                    for verb in [Verb::Chamfer, Verb::Fillet] {
                        let removed = if convex {
                            verb.section()
                        } else {
                            -verb.section()
                        };
                        let what = format!("open {open} convex {convex} x {x} gap {gap} {verb:?}");
                        let o = run(&body, &[e], verb, removed, &what);
                        sound(&o, &what);
                        match o {
                            Outcome::Built => built += 1,
                            _ => refused += 1,
                        }
                        // Between samples, past the setback's sampled
                        // reach, the far side is clear and must build.
                        if x == 3.0625 && gap >= 0.09 {
                            assert_eq!(o, Outcome::Built, "{what}: clear far side builds");
                        }
                    }
                }
            }
        }
    }
    eprintln!("PROBE far side: built {built}, refused {refused}");
}

/// **A wide far-side plateau parallel to the edge**, its top edge a
/// segment across the line spanning past both stations: no vertex near
/// the strip, an edge along it.
#[test]
fn a_far_side_plateau_along_the_line_builds_or_refuses_typed() {
    for open in [false, true] {
        for convex in [true, false] {
            let z = if convex { 2.0 } else { 1.0 };
            for gap in [0.2, 0.12, 0.09, 0.05] {
                let x0 = if open { -1.0 } else { 1.0 };
                let outline = [
                    (x0, 1.0),
                    (2.0, 1.0),
                    (2.2, 2.5 - gap),
                    (3.8, 2.5 - gap),
                    (4.0, 1.0),
                    (5.0, 1.0),
                    (5.0, 4.0),
                    (3.5, 4.0),
                    TAB[0],
                    TAB[1],
                    (2.5, 4.0),
                    (x0, 4.0),
                ];
                let body = outline_body(&outline, convex);
                let e = edge(&body, [3.5, 2.5, z], [2.5, 2.5, z]);
                for verb in [Verb::Chamfer, Verb::Fillet] {
                    let removed = if convex {
                        verb.section()
                    } else {
                        -verb.section()
                    };
                    let what = format!("plateau open {open} convex {convex} gap {gap} {verb:?}");
                    let o = run(&body, &[e], verb, removed, &what);
                    sound(&o, &what);
                    if gap >= 0.12 {
                        assert_eq!(o, Outcome::Built, "{what}: clear plateau builds");
                    }
                }
            }
        }
    }
}

/// The groove fixture of `band_planar_cut_off_meters` (copied): the
/// tabbed pocket beside a through half-cylinder groove along `x` at
/// `y = 4.6`, its mouth `y = 4.4` a cylinder–plane link's edge on the
/// top face; `wall` spliced into the pocket's `y = 4` wall.
fn grooved(wall: &[((f64, f64), f64)], open: bool) -> Body<f64> {
    let x0 = if open { -1.0 } else { 1.0 };
    let outline: Vec<(Point2<f64>, f64)> = [
        (x0, 1.0),
        (5.0, 1.0),
        (5.0, 4.0),
        (3.5, 4.0),
        TAB[0],
        TAB[1],
        (2.5, 4.0),
    ]
    .into_iter()
    .map(|p| (p, 0.0))
    .chain(wall.iter().copied())
    .chain([((x0, 4.0), 0.0)])
    .map(|((x, y), bulge)| (Point2::new(x, y), bulge))
    .collect();
    let pocket = realized(
        topo::boolean::BooleanOp::Subtract,
        &block(6.0, 5.0, 2.0, tol()),
        &sweep::test_support::prism_at(outline, 1.0, 2.0, tol()),
        tol(),
    );
    let rod = prism_on(
        sketch_from_axes(
            Point3::new(-0.1, 4.6, 2.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            tol(),
        ),
        vec![(Point2::new(-0.2, 0.0), 1.0), (Point2::new(0.2, 0.0), 1.0)],
        6.2,
        tol(),
    );
    let body = realized(topo::boolean::BooleanOp::Subtract, &pocket, &rod, tol());
    validate_geometric(&body, tol()).expect("the grooved pocket is tier-3 valid");
    body
}

fn mouth_only(body: &Body<f64>) -> Result<(), BlendError> {
    let e = edge(body, [0.0, 4.4, 2.0], [6.0, 4.4, 2.0]);
    Verb::Fillet.run(body, &[e]).map(|out| {
        validate_geometric(&out.body, tol()).expect("tier 3");
        validate_pseudomanifold(&out.body, &ContactRecords::default(), tol()).expect("tier 3′");
    })
}

fn report(r: &Result<(), BlendError>, what: &str) -> Outcome {
    let o = match r {
        Ok(()) => Outcome::Built,
        Err(e) => Outcome::Refused(variant(e), gap_of(e)),
    };
    sound(&o, what);
    o
}

/// **A ruled link's planar support: an outer-cycle spike at and about
/// the setback.** No strip meter covers the ruled link, so only the
/// reach (`screened` for the vertex, the cell meter for its walls) can
/// see the spike. Inside by more than `3ε` it must refuse; outside by
/// more than `3ε` it must build; in band, escalate or refuse — never
/// build a body whose spike the band cuts.
#[test]
fn a_ruled_links_outer_spike_about_the_setback() {
    let eps = tol().eps();
    let setback = 0.08_f64.sqrt() - 0.2;
    let line = 4.4 - setback;
    for x in [1.875, 0.06, 2.4] {
        for dy in [
            -0.01,
            -1e-4,
            -3.0 * eps,
            -eps,
            0.0,
            eps,
            3.0 * eps,
            1e-4,
            0.01,
        ] {
            let tip = line + dy;
            let wall = [
                ((x + 0.025, 4.0), 0.0),
                ((x, tip), 0.0),
                ((x - 0.025, 4.0), 0.0),
            ];
            let what = format!("ruled outer spike x {x} tip − (mouth − setback) = {dy:e}");
            let o = report(&mouth_only(&grooved(&wall, true)), &what);
            if dy >= 1e-4 {
                assert!(
                    matches!(o, Outcome::Refused(..)),
                    "{what}: a vertex inside the strip refuses"
                );
            }
            if dy <= -1e-4 {
                assert_eq!(
                    o,
                    Outcome::Built,
                    "{what}: a spike clear of the strip builds"
                );
            }
        }
    }
}

/// **A ruled link's planar support: an outer-cycle arc bulging into the
/// strip with no vertex in it** — the edge-only feature `screened`
/// cannot see; the review's ring arc, on the outer cycle.
#[test]
fn a_ruled_links_outer_arc_in_the_strip() {
    let setback = 0.08_f64.sqrt() - 0.2;
    for (apex, must_refuse) in [(4.35, true), (4.4 - setback + 1e-3, true), (4.3, false)] {
        // bulge = tan(θ/4) for sagitta s over half-chord c: s/c.
        let (c, s) = (0.125_f64, apex - 4.0);
        let bulge = s / c;
        let wall = [((2.0, 4.0), bulge), ((1.75, 4.0), 0.0)];
        let what = format!("ruled outer arc apex {apex}");
        let o = report(&mouth_only(&grooved(&wall, true)), &what);
        if must_refuse {
            assert!(matches!(o, Outcome::Refused(..)), "{what}: refuses");
        } else {
            assert_eq!(o, Outcome::Built, "{what}: clear arc builds");
        }
    }
}

/// **The plane–plane ring spike at and about `D`, near each station.**
/// The PR's spike sits mid-edge; near a station the window end and the
/// station skip are the nearest sides.
#[test]
fn a_ring_spike_near_a_station_about_the_setback() {
    let eps = tol().eps();
    for (x, wall_x) in [(3.47, 3.5), (2.53, 2.5)] {
        for dy in [-0.01, -3.0 * eps, 3.0 * eps, 0.01, 0.05] {
            let tip = 2.5 + D - dy;
            // A spike cut into the tab's side wall at `wall_x`, its tip
            // at `(x, tip)`.
            let (a, b) = if wall_x > 3.0 {
                ((3.5, tip + 0.3), (3.5, tip + 0.25))
            } else {
                ((2.5, tip + 0.25), (2.5, tip + 0.3))
            };
            let mut outline = vec![(1.0, 1.0), (5.0, 1.0), (5.0, 4.0), (3.5, 4.0)];
            if wall_x > 3.0 {
                outline.extend([a, (x, tip), b, TAB[0], TAB[1]]);
            } else {
                outline.extend([TAB[0], TAB[1], a, (x, tip), b]);
            }
            outline.extend([(2.5, 4.0), (1.0, 4.0)]);
            let body = outline_body(&outline, true);
            let e = edge(&body, [3.5, 2.5, 2.0], [2.5, 2.5, 2.0]);
            for verb in [Verb::Chamfer, Verb::Fillet] {
                let what = format!("ring spike x {x} depth D − {dy:e} {verb:?}");
                let o = run(&body, &[e], verb, verb.section(), &what);
                sound(&o, &what);
                if dy >= 0.01 {
                    assert!(
                        matches!(o, Outcome::Refused(..)),
                        "{what}: in strip refuses"
                    );
                }
            }
        }
    }
}

/// **The two-bands-of-one-request excuse under oblique poses** (RED on
/// 7781e428e: the 0.7-about-x, 0.7-about-z and 0.3-about-y poses build
/// on the merge base 8cd31ca1d and refuse `FaceClearance` here). The one
/// test that needs the `others` excuse (`blend_per_shell_carry`'s sealed
/// void + island, filleted in one request) reads it over axis-aligned
/// edges, where a face box's cell straddles an edge's line by no more
/// than the pad. Posed obliquely, a box cell of a support's strip
/// straddles the line by up to its own width, and the region the excuse
/// reads is now one-sided (set back by `2·pad` only), so a cell the
/// two-sided tube held whole may no longer be excused.
#[test]
fn the_sealed_island_request_builds_at_every_pose() {
    use crate::common::cavity::brick;
    use crate::common::oracles::rounded_box_volume;
    let sub = |a: &Body<f64>, b: &Body<f64>| {
        let a = sweep::test_support::finished("a", a.clone(), tol());
        let b = sweep::test_support::finished("b", b.clone(), tol());
        topo::subtract(&a, &b, tol())
            .unwrap()
            .body()
            .expect("material")
            .body
            .clone()
            .into_body()
    };
    let uni = |a: &Body<f64>, b: &Body<f64>| {
        let a = sweep::test_support::finished("a", a.clone(), tol());
        let b = sweep::test_support::finished("b", b.clone(), tol());
        topo::union(&a, &b, tol())
            .unwrap()
            .body()
            .expect("material")
            .body
            .clone()
            .into_body()
    };
    let sealed = sub(
        &brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0)),
        &brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0)),
    );
    let body = uni(
        &sealed,
        &brick(Point3::new(1.05, 1.05, 1.05), Point3::new(2.95, 2.95, 2.95)),
    );
    let corners = |lo: f64, hi: f64| {
        crate::common::cavity::edges_with_corners(&body, |p| {
            p.to_array()
                .iter()
                .all(|c| (c - lo).abs() < 1e-9 || (c - hi).abs() < 1e-9)
        })
    };
    let mut req = corners(1.0, 3.0);
    req.extend(corners(1.05, 2.95));
    let want = 64.0 - rounded_box_volume(1.5, 0.25) + rounded_box_volume(1.4, 0.25);
    let mut failures = Vec::new();
    for (name, map) in crate::common::poses::poses() {
        let posed = topo::transform_rigid(&body, &map, tol()).unwrap();
        let r = sweep::blend::build::fillet_edges(
            &sweep::test_support::at_rest(&posed, tol()),
            &req,
            0.25,
            tol(),
        );
        match r {
            Ok(out) => {
                validate_geometric(&out.body, tol())
                    .unwrap_or_else(|e| panic!("{name}: tier 3 {e:?}"));
                let v = crate::band_planar_cut_off::volume(&out.body);
                eprintln!(
                    "PROBE sealed island pose {name}: Built, V − want = {:e}",
                    v - want
                );
                assert!((v - want).abs() < 1e-9, "{name}: V {v} vs {want}");
            }
            Err(e) => {
                let e = e.error;
                eprintln!(
                    "PROBE sealed island pose {name}: {:?} {}",
                    variant(&e),
                    gap_of(&e)
                );
                // The (1,2,3) pose refuses on the merge base too: not this PR's.
                if !name.starts_with("1.1 about") {
                    failures.push(format!("{name}: {e}"));
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "posed sealed island refused: {failures:#?}"
    );
}

/// **The PR's far-side rows under oblique poses**: the one-sided bound
/// and `screened` read an oblique edge.
#[test]
fn far_side_rows_build_at_every_pose() {
    let mut failures = Vec::new();
    for open in [false, true] {
        for convex in [true, false] {
            let z = if convex { 2.0 } else { 1.0 };
            let body = outline_body(&far_side_at(3.0625, 0.09, 0.0375, open), convex);
            let e = edge(&body, [3.5, 2.5, z], [2.5, 2.5, z]);
            for (name, map) in crate::common::poses::poses() {
                let posed = topo::transform_rigid(&body, &map, tol()).unwrap();
                for verb in [Verb::Chamfer, Verb::Fillet] {
                    let removed = if convex {
                        verb.section()
                    } else {
                        -verb.section()
                    };
                    let what = format!("posed {name} open {open} convex {convex} {verb:?}");
                    let o = run(&posed, &[e], verb, removed, &what);
                    sound(&o, &what);
                    if o != Outcome::Built {
                        failures.push(format!("{what}: {o:?}"));
                    }
                }
            }
        }
    }
    assert!(failures.is_empty(), "posed far-side rows: {failures:#?}");
}
