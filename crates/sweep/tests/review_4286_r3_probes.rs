//! Third-review probes for PR 4286 (not for merge): the split regions
//! of fix pass 2 — `screened` reads the one-sided bound, and the
//! cross-chain excuse reads the two-sided region only where `screened`
//! found it vacant of support vertices but the request's.
//!
//! Every row asserts the PR's contract: a request either builds a body
//! that passes tier 3, tier 3′, one genus-0 shell, naming totality and
//! ΔV at the closed form (`carve`), or refuses with a typed clearance
//! refusal. Never `SurgeryInvariant`, `BodyNotIntact`, a panic, or a
//! wrong body. Each row prints its outcome so the report can quote it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::panic::{AssertUnwindSafe, catch_unwind};

use geom_core::{Point2, Point3};
use sweep::blend::BlendError;
use sweep::test_support::{block, realized};
use topo::{Body, ContactRecords, EdgeKey, validate_geometric, validate_pseudomanifold};

use crate::band_planar_cut_off::{Verb, carve, edge, tol};

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

fn detail(e: &BlendError) -> String {
    match e {
        BlendError::FaceClearanceUncertified {
            gap,
            margin,
            cross_chain,
            ..
        } => format!(
            "gap {:?} margin {:?} cross {cross_chain}",
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
        other => format!("{other:?}"),
    }
}

fn run(body: &Body<f64>, edges: &[EdgeKey], verb: Verb, removed: f64, what: &str) -> Outcome {
    match verb.run(body, edges) {
        Err(e) => Outcome::Refused(variant(&e), detail(&e)),
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

/// The outline as a pocket (`convex`) of a `6 × 5 × 2` block from
/// `z = 1`, or as a boss on a `6 × 5 × 1` block to `z = 1.5`.
fn outline_body(outline: &[((f64, f64), f64)], convex: bool) -> Body<f64> {
    let outline: Vec<(Point2<f64>, f64)> = outline
        .iter()
        .map(|&((x, y), b)| (Point2::new(x, y), b))
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

/// The tabbed outline, its far wall `far` (from `(x0, 1)` on, in
/// order), and a second tab `[1.3, 1.8] × [3, 4]` whose tip is a chain
/// apart from the first when `tab2`.
fn tabbed(far: &[((f64, f64), f64)], open: bool, tab2: bool) -> Vec<((f64, f64), f64)> {
    let x0 = if open { -1.0 } else { 1.0 };
    let mut o = vec![((x0, 1.0), 0.0)];
    o.extend_from_slice(far);
    o.extend([
        ((5.0, 1.0), 0.0),
        ((5.0, 4.0), 0.0),
        ((3.5, 4.0), 0.0),
        ((3.5, 2.5), 0.0),
        ((2.5, 2.5), 0.0),
        ((2.5, 4.0), 0.0),
    ]);
    if tab2 {
        o.extend([
            ((1.8, 4.0), 0.0),
            ((1.8, 3.0), 0.0),
            ((1.3, 3.0), 0.0),
            ((1.3, 4.0), 0.0),
        ]);
    }
    o.push(((x0, 4.0), 0.0));
    o
}

fn plain(pts: &[(f64, f64)]) -> Vec<((f64, f64), f64)> {
    pts.iter().map(|&p| (p, 0.0)).collect()
}

fn sign(convex: bool) -> f64 {
    if convex { 1.0 } else { -1.0 }
}

/// **Claim 3: a far-side support edge across the two-sided region with
/// no vertex in it, on a cross-chain request.** The far plateau's top
/// is flat at `y = 2.5 − base` from `x = 2.2` to `3.8`, with a
/// semicircular bump of radius `0.03` centred under `x = 3.0625`
/// (midway between the tip's samples): the bump's apex stands `apex`
/// from the tip's line, inside the tube of radius `D`, and its two end
/// vertices stand `base = apex + 0.03` from it, outside the tube once
/// `base > D`. So `screened` finds the two-sided region vacant and the
/// excuse reads two-sided. The sampled screen reads `≈ 0.1353 − 0.03`
/// off its nearest samples. Requested alone and with a second tab's
/// tip (a chain apart, so the request is cross-chain).
#[test]
fn a_far_side_bump_with_no_vertex_in_the_tube_builds_or_refuses_typed() {
    for open in [false, true] {
        for convex in [true, false] {
            let z = if convex { 2.0 } else { 1.0 };
            for apex in [0.095, 0.09, 0.08] {
                for bulge in [-1.0, 1.0] {
                    let base = apex + 0.03;
                    let y = 2.5 - base;
                    let far = [
                        ((2.2, 1.0), 0.0),
                        ((2.2, y), 0.0),
                        ((3.0325, y), bulge),
                        ((3.0925, y), 0.0),
                        ((3.8, y), 0.0),
                        ((3.8, 1.0), 0.0),
                    ];
                    for tab2 in [false, true] {
                        let body = outline_body(&tabbed(&far, open, tab2), convex);
                        let mut edges = vec![edge(&body, [3.5, 2.5, z], [2.5, 2.5, z])];
                        let mut len = 1.0;
                        if tab2 {
                            edges.push(edge(&body, [1.8, 3.0, z], [1.3, 3.0, z]));
                            len += 0.5;
                        }
                        for verb in [Verb::Chamfer, Verb::Fillet] {
                            let what = format!(
                                "bump open {open} convex {convex} apex {apex} bulge {bulge} \
                                 tab2 {tab2} {verb:?}"
                            );
                            let o = run(
                                &body,
                                &edges,
                                verb,
                                sign(convex) * verb.section() * len,
                                &what,
                            );
                            sound(&o, &what);
                        }
                    }
                }
            }
        }
    }
}

/// **The cross-chain plateau**: the tip and the far plateau's own top
/// edge requested together, `gap` apart across the slot between them.
/// The bands lie on opposite sides of the slot at every gap, so the
/// body removes (adds) the two prism closed forms, `1 + 1.6` long.
#[test]
fn a_cross_chain_plateau_builds_or_refuses_typed() {
    for open in [false, true] {
        for convex in [true, false] {
            let z = if convex { 2.0 } else { 1.0 };
            for gap in [0.3, 0.2, 0.12, 0.1, 0.09, 0.05, 0.02] {
                let y = 2.5 - gap;
                let far = plain(&[(2.2, 1.0), (2.2, y), (3.8, y), (3.8, 1.0)]);
                let body = outline_body(&tabbed(&far, open, false), convex);
                let edges = [
                    edge(&body, [3.5, 2.5, z], [2.5, 2.5, z]),
                    edge(&body, [2.2, y, z], [3.8, y, z]),
                ];
                for verb in [Verb::Chamfer, Verb::Fillet] {
                    let what =
                        format!("cross plateau open {open} convex {convex} gap {gap} {verb:?}");
                    let o = run(
                        &body,
                        &edges,
                        verb,
                        sign(convex) * verb.section() * 2.6,
                        &what,
                    );
                    sound(&o, &what);
                }
            }
        }
    }
}

/// **The occupied branch on a cross-chain request**: the review's
/// far-side peninsula (tip `0.09` across the tip's line at
/// `x = 3.0625`), so `screened` finds a vertex across and the excuse
/// reads one-sided, with a second tab's tip requested beside it.
#[test]
fn the_occupied_branch_on_a_cross_chain_request() {
    for open in [false, true] {
        for convex in [true, false] {
            let z = if convex { 2.0 } else { 1.0 };
            for gap in [0.2, 0.09, 0.05] {
                let far = plain(&[(3.025, 1.0), (3.0625, 2.5 - gap), (3.1, 1.0)]);
                let body = outline_body(&tabbed(&far, open, true), convex);
                let edges = [
                    edge(&body, [3.5, 2.5, z], [2.5, 2.5, z]),
                    edge(&body, [1.8, 3.0, z], [1.3, 3.0, z]),
                ];
                for verb in [Verb::Chamfer, Verb::Fillet] {
                    let what = format!("occupied open {open} convex {convex} gap {gap} {verb:?}");
                    let o = run(
                        &body,
                        &edges,
                        verb,
                        sign(convex) * verb.section() * 1.5,
                        &what,
                    );
                    sound(&o, &what);
                    if gap >= 0.09 {
                        assert_eq!(o, Outcome::Built, "{what}: clear far side builds");
                    }
                }
            }
        }
    }
}

/// The bump and the occupied cross-chain rows at every pose of
/// `common::poses::poses()`: built bodies pass tier 3 and hold V at
/// the unposed body's ΔV; otherwise a typed clearance refusal.
#[test]
fn the_cross_chain_rows_at_every_pose() {
    let bump = |convex| {
        let y = 2.5 - 0.12;
        let far = [
            ((2.2, 1.0), 0.0),
            ((2.2, y), 0.0),
            ((3.0325, y), -1.0),
            ((3.0925, y), 0.0),
            ((3.8, y), 0.0),
            ((3.8, 1.0), 0.0),
        ];
        outline_body(&tabbed(&far, false, true), convex)
    };
    let occupied = |convex| {
        let far = plain(&[(3.025, 1.0), (3.0625, 2.41), (3.1, 1.0)]);
        outline_body(&tabbed(&far, false, true), convex)
    };
    for (name, make) in [
        ("bump", &bump as &dyn Fn(bool) -> Body<f64>),
        ("occupied", &occupied),
    ] {
        for convex in [true, false] {
            let z = if convex { 2.0 } else { 1.0 };
            let body = make(convex);
            let edges = [
                edge(&body, [3.5, 2.5, z], [2.5, 2.5, z]),
                edge(&body, [1.8, 3.0, z], [1.3, 3.0, z]),
            ];
            let v0 = crate::band_planar_cut_off::volume(&body);
            for verb in [Verb::Chamfer, Verb::Fillet] {
                let want = v0 - sign(convex) * verb.section() * 1.5;
                for (pose, map) in crate::common::poses::poses() {
                    let posed = topo::transform_rigid(&body, &map, tol()).unwrap();
                    let what = format!("{name} convex {convex} {verb:?} pose {pose}");
                    match verb.run(&posed, &edges) {
                        Ok(out) => {
                            validate_geometric(&out.body, tol())
                                .unwrap_or_else(|e| panic!("{what}: tier 3 {e:?}"));
                            let v = crate::band_planar_cut_off::volume(&out.body);
                            eprintln!("PROBE {what}: Built, V − want = {:e}", v - want);
                            assert!((v - want).abs() < 1e-9, "{what}: V {v} vs {want}");
                        }
                        Err(e) => sound(&Outcome::Refused(variant(&e), detail(&e)), &what),
                    }
                }
            }
        }
    }
}
