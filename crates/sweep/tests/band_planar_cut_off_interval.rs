//! The plane–plane band's cut-off at the CERTIFIED scalar — the
//! `Interval` replay of `band_planar_cut_off`'s box rows: one edge, and
//! the three edges of one corner, each chamfered and filleted. Every row
//! carves, is tier-3 valid with naming total, and its volume enclosure
//! brackets the closed form narrowly enough to be a claim. The fillet's
//! perpendicular end faces are decided `Zero` by `fillet3_cap_transverse`
//! at the certified scalar, read off the funnel's log. A sheared box's
//! oblique end faces put a fillet's foot half a turn from its end
//! ellipse's `u_ref`; those rows build at `Interval` as at `f64`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::interval::iv;
use geom_core::k_stats::Bracket;
use geom_core::{Bounds, Decide, Interval, Point3, Sign, Tol};
use sweep::blend::BlendError;
use sweep::blend::build::{chamfer_edges, fillet_edges};
use sweep::test_support::{assert_naming_totality, block};
use topo::{
    Body, ContactRecords, EdgeKey, mass_properties, query, validate_geometric,
    validate_pseudomanifold,
};

const D: f64 = 0.1;

/// The box edge between two points, either way round.
fn edge<T: Bounds>(body: &Body<T>, a: [f64; 3], b: [f64; 3]) -> EdgeKey {
    let near = |p: &Point3<T>, q: [f64; 3]| {
        [p.x, p.y, p.z]
            .iter()
            .zip(q)
            .all(|(c, w)| c.lo() <= w + 1e-12 && w - 1e-12 <= c.hi())
    };
    query::all_edges(body)
        .into_iter()
        .find(|&e| {
            let he = body.get_edge(e).unwrap().he_plus;
            let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            let (s, t) = (
                p(body.get_half_edge(he).unwrap().start),
                p(body.half_edge_end(he).unwrap()),
            );
            (near(&s, a) && near(&t, b)) || (near(&s, b) && near(&t, a))
        })
        .expect("a box edge between the two points")
}

#[test]
fn the_box_rows_carve_at_the_certified_scalar_and_bracket_their_closed_forms() {
    let tol = Tol::witness();
    let body = sweep::test_support::finished("body", block::<Interval>(2.0, 1.5, 1.0, tol), tol);
    let front = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let up = edge(&body, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let across = edge(&body, [0.0, 0.0, 1.0], [0.0, 1.5, 1.0]);
    let v0 = mass_properties(&body, tol, topo::test_support::pinned_resolution(tol))
        .expect("interval props")
        .met()
        .expect("the pinned resolution is met")
        .volume;
    let chamfer = (D * D / 2.0, 2.0 / 3.0 * D.powi(3));
    let fillet = (
        (1.0 - PI / 4.0) * D * D,
        (2.0 - 7.0 * PI / 12.0) * D.powi(3),
    );
    let rows: [(&str, Vec<EdgeKey>, f64, usize); 2] = [
        ("one edge", vec![front], 2.0, 0),
        ("three edges of one corner", vec![front, up, across], 4.5, 1),
    ];
    for (what, edges, length, corners) in rows {
        for (round, (section, corner)) in [(false, chamfer), (true, fillet)] {
            let bracket = Bracket::open();
            let out = if round {
                fillet_edges(&body, &edges, iv(D), tol)
            } else {
                chamfer_edges(&body, &edges, iv(D), tol)
            }
            .unwrap_or_else(|e| panic!("{what} (round {round}): carves at Interval, got {e:?}"));
            let ends = bracket
                .finish()
                .verdicts
                .into_iter()
                .filter(|v| v.predicate == "fillet3_cap_transverse")
                .collect::<Vec<_>>();
            if round {
                assert!(
                    !ends.is_empty() && ends.iter().all(|v| v.sign == Sign::Zero),
                    "{what}: every fillet end face is perpendicular at Interval: {ends:?}"
                );
            }
            validate_geometric(&out.body, tol)
                .unwrap_or_else(|e| panic!("{what} (round {round}): tier 3, got {e:?}"));
            assert_naming_totality(&body, &out, &edges, what);
            let p = topo::mass_properties(
                &topo::test_support::finished(
                    "the measured body",
                    topo::Body::clone(&out.body),
                    tol,
                ),
                tol,
                topo::test_support::pinned_resolution(tol),
            )
            .expect("interval props");
            assert_eq!(
                p.enclosure.volume_pad, 0.0,
                "{what}: a closed-form inventory"
            );
            let removed = v0 - p.enclosure.volume;
            let truth = section * length - corner * corners as f64;
            assert!(
                removed.lo() <= truth + 1e-15 && truth - 1e-15 <= removed.hi(),
                "{what} (round {round}): ΔV {removed:?} brackets {truth}"
            );
            assert!(
                removed.hi() - removed.lo() < 1e-9,
                "{what} (round {round}): the enclosure is a claim: {removed:?}"
            );
        }
    }
}

/// The tabbed pocket or boss at `T`: a `6 × 5 × 2` block less the
/// prism over `outline` from `z = 1` (convex), or a `6 × 5 × 1` block
/// with it from `z = 0.5` (concave).
fn tabbed<T: geom_core::Decide + Bounds + topo::AtRestPolicy>(
    convex: bool,
    outline: &[(f64, f64)],
    tol: Tol,
) -> Body<T> {
    use sweep::test_support::{finished, prism_at};
    use topo::boolean::{BooleanInputs, BooleanOp, SweepStrategy, boolean_op_with};
    let outline = outline
        .iter()
        .map(|&(x, y)| {
            (
                geom_core::Point2::new(T::from_f64(x), T::from_f64(y)),
                T::zero(),
            )
        })
        .collect();
    let (op, h, z0) = if convex {
        (BooleanOp::Subtract, 2.0, 1.0)
    } else {
        (BooleanOp::Union, 1.0, 0.5)
    };
    boolean_op_with(
        op,
        &finished("the block", block::<T>(6.0, 5.0, h, tol), tol),
        &finished(
            "the tabbed prism",
            prism_at(outline, T::from_f64(z0), T::from_f64(2.0), tol),
            tol,
        ),
        &BooleanInputs::default(),
        SweepStrategy::Realized,
        tol,
    )
    .unwrap_or_else(|e| panic!("the tabbed fixture builds, got {e}"))
    .body()
    .expect("the tabbed fixture is a body")
    .body
    .clone()
    .into_body()
}

/// **A feature across a requested edge's line is clear at the certified
/// scalar** — the `Interval` replay of
/// `band_planar_cut_off_meters::a_feature_across_a_requested_edges_line_is_clear_of_its_strip`:
/// a peninsula of the support `0.09` across the tab's tip, on its ring
/// and on its outer cycle, pocket and boss, both verbs. Every row
/// carves, is tier-3 valid with naming total, and brackets `±` the
/// prism section over the tip's unit length.
#[test]
fn a_feature_across_a_requested_edges_line_is_clear_at_the_certified_scalar() {
    let tol = Tol::witness();
    for open in [false, true] {
        let x0 = if open { -1.0 } else { 1.0 };
        let outline = [
            (x0, 1.0),
            (3.025, 1.0),
            (3.0625, 2.41),
            (3.1, 1.0),
            (5.0, 1.0),
            (5.0, 4.0),
            (3.5, 4.0),
            (3.5, 2.5),
            (2.5, 2.5),
            (2.5, 4.0),
            (x0, 4.0),
        ];
        for (convex, z) in [(true, 2.0), (false, 1.0)] {
            let what = format!("open {open}, convex {convex}");
            let body = sweep::test_support::finished(
                "body",
                tabbed::<Interval>(convex, &outline, tol),
                tol,
            );
            let tip = edge(&body, [3.5, 2.5, z], [2.5, 2.5, z]);
            let v0 = mass_properties(&body, tol, topo::test_support::pinned_resolution(tol))
                .expect("interval props")
                .met()
                .expect("the pinned resolution is met")
                .volume;
            for (round, section) in [(false, D * D / 2.0), (true, (1.0 - PI / 4.0) * D * D)] {
                let out = if round {
                    fillet_edges(&body, &[tip], iv(D), tol)
                } else {
                    chamfer_edges(&body, &[tip], iv(D), tol)
                }
                .unwrap_or_else(|e| {
                    panic!("{what} (round {round}): carves at Interval, got {e:?}")
                });
                validate_geometric(&out.body, tol)
                    .unwrap_or_else(|e| panic!("{what} (round {round}): tier 3, got {e:?}"));
                assert_naming_totality(&body, &out, &[tip], &what);
                let removed = v0
                    - topo::mass_properties(
                        &topo::test_support::finished(
                            "the measured body",
                            topo::Body::clone(&out.body),
                            tol,
                        ),
                        tol,
                        topo::test_support::pinned_resolution(tol),
                    )
                    .expect("interval props")
                    .met()
                    .expect("interval props")
                    .volume;
                let truth = if convex { section } else { -section };
                assert!(
                    removed.lo() <= truth + 1e-15 && truth - 1e-15 <= removed.hi(),
                    "{what} (round {round}): ΔV {removed:?} brackets {truth}"
                );
                assert!(
                    removed.hi() - removed.lo() < 1e-9,
                    "{what} (round {round}): the enclosure is a claim: {removed:?}"
                );
            }
        }
    }
}

/// The sheared box `parallelepiped_at(s)` at `T`, as itself (convex) or
/// as a void in a block (concave, its top edge a pocket's ceiling edge),
/// and its top edge out of `(s, s, 1)`, whose end faces both lean.
fn sheared<T: Decide + Bounds + topo::AtRestPolicy>(
    s: f64,
    convex: bool,
    tol: Tol,
) -> (topo::AtRestBody<T>, EdgeKey) {
    use sweep::test_support::{brick, finished};
    use topo::boolean::{BooleanInputs, BooleanOp, SweepStrategy, boolean_op_with};
    let leaning = crate::common::operands::parallelepiped_at::<T>(s);
    let body = if convex {
        leaning
    } else {
        boolean_op_with(
            BooleanOp::Subtract,
            &finished(
                "the block",
                brick::<T>((-1.0, 4.0), (-1.0, 3.0), (-1.0, 2.0), tol),
                tol,
            ),
            &finished("the sheared box", leaning, tol),
            &BooleanInputs::default(),
            SweepStrategy::Realized,
            tol,
        )
        .unwrap_or_else(|e| panic!("the void builds, got {e}"))
        .body()
        .expect("the void is a body")
        .body
        .clone()
        .into_body()
    };
    let body = finished("the sheared fixture", body, tol);
    let top = edge(&body, [s, s, 1.0], [2.0 + s, s, 1.0]);
    (body, top)
}

/// Tier 3′, past one refusal: below `ε = 1e-9` the curve-pair meter
/// cannot settle a fillet joint's meeting count
/// (`work/cpair/curve-pair-polish-meters-refuse-shallow-meetings.md`),
/// so there, and only there, that typed refusal is all it may say.
fn tier_3_prime(body: &Body<Interval>, tol: Tol, what: &str) {
    let Err(findings) = validate_pseudomanifold(body, &ContactRecords::default(), tol) else {
        return;
    };
    let unsettled = |e: &topo::ValidationError| {
        matches!(
            e,
            topo::ValidationError::CensusUnsupported {
                cause: topo::CensusUnsupportedCause::ContactLane(
                    topo::ContactRefusal::NotCertifiable { .. }
                ),
                ..
            }
        )
    };
    assert!(
        tol.eps() < 1e-9 && findings.iter().all(unsettled),
        "{what}: tier 3′, got {findings:?}"
    );
    println!(
        "{what}: tier 3′ unsettled at ε {} (cpair's meter): {findings:?}",
        tol.eps()
    );
}

/// **A fillet foot on `atan2`'s cut carves at the certified scalar as
/// at `f64`**: the sheared box's top edge over a ladder of shears of
/// both signs, convex and concave, both verbs. At a positive shear the
/// fillet's foot on the top rim is the end ellipse's major vertex, half
/// a turn from `u_ref`, where a parameter read from the seam straddles
/// `atan2`'s cut: those six rows are the witnesses. The negative-shear
/// rows put the foot on `u_ref` itself, and a chamfer's cut-off reads
/// no arc; they are the ladder's controls. Every row builds at both
/// scalars, tiers 3 and 3′ and naming total at `Interval`, its `ΔV`
/// enclosure bracketing the `f64` body's within the props pad and its
/// midpoint within `midpoint_tol`. At `ε = 1e-12` a control's
/// `Interval` build may escalate its pcurve envelope instead, a witness
/// never, and tier 3′ may meet cpair's meter ([`tier_3_prime`]).
#[test]
fn a_fillet_foot_on_atan2s_cut_at_an_oblique_end_carves_at_the_certified_scalar() {
    use crate::band_planar_cut_off::{midpoint_tol, pad_ceiling};
    let tol = Tol::witness();
    for s in [-0.6, -0.3, -0.1, 0.1, 0.3, 0.6] {
        for convex in [true, false] {
            let (fbody, fedge) = sheared::<f64>(s, convex, tol);
            let (ibody, iedge) = sheared::<Interval>(s, convex, tol);
            let fv0 = mass_properties(&fbody, tol, topo::test_support::pinned_resolution(tol))
                .expect("props")
                .met()
                .expect("the pinned resolution is met")
                .volume;
            let iv0 = mass_properties(&ibody, tol, topo::test_support::pinned_resolution(tol))
                .expect("interval props");
            for round in [false, true] {
                let witness = round && s > 0.0;
                let what = format!("shear {s}, convex {convex}, round {round}");
                let fout = if round {
                    fillet_edges(&fbody, &[fedge], D, tol)
                } else {
                    chamfer_edges(&fbody, &[fedge], D, tol)
                }
                .unwrap_or_else(|e| panic!("{what}: carves at f64, got {e:?}"));
                let want = fv0
                    - topo::mass_properties(
                        &topo::test_support::finished(
                            "the measured body",
                            topo::Body::clone(&fout.body),
                            tol,
                        ),
                        tol,
                        topo::test_support::pinned_resolution(tol),
                    )
                    .expect("props")
                    .met()
                    .expect("props")
                    .volume;
                let out = match if round {
                    fillet_edges(&ibody, &[iedge], iv(D), tol)
                } else {
                    chamfer_edges(&ibody, &[iedge], iv(D), tol)
                } {
                    Ok(out) => out,
                    // At the finest ε a re-minted pcurve's envelope
                    // enclosure is wider than the band, and escalates.
                    Err(e)
                        if !witness
                            && tol.eps() <= 1e-12
                            && matches!(
                                &e.error,
                                BlendError::Certify {
                                    source: topo::PcurveMintError::Certify {
                                        error: geom_brep::PcurveCertifyError::Escalated { .. },
                                        ..
                                    },
                                    ..
                                }
                            ) =>
                    {
                        test_utils::vacuity::stood_down(
                            &what,
                            &format!("a control escalates at ε {}: {:?}", tol.eps(), e.error),
                        );
                        continue;
                    }
                    Err(e) => panic!("{what}: carves at Interval, got {e:?}"),
                };
                validate_geometric(&out.body, tol)
                    .unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
                tier_3_prime(&out.body, tol, &what);
                assert_naming_totality(&ibody, &out, &[iedge], &what);
                let p = topo::mass_properties(
                    &topo::test_support::finished(
                        "the measured body",
                        topo::Body::clone(&out.body),
                        tol,
                    ),
                    tol,
                    topo::test_support::pinned_resolution(tol),
                )
                .expect("interval props");
                let removed = iv0.enclosure.volume - p.enclosure.volume;
                let pad = iv0.enclosure.volume_pad.hi() + p.enclosure.volume_pad.hi();
                assert!(
                    removed.lo() - pad <= want && want <= removed.hi() + pad,
                    "{what}: ΔV {removed:?} ± {pad} brackets the f64 body's {want}"
                );
                let mid = 0.5 * (removed.lo() + removed.hi());
                assert!(
                    (mid - want).abs() < midpoint_tol(),
                    "{what}: ΔV's midpoint {mid} against the f64 body's {want}"
                );
                assert!(
                    removed.hi() - removed.lo() + 2.0 * pad < 2.0 * pad_ceiling(),
                    "{what}: the enclosure is a claim: {removed:?} ± {pad}"
                );
            }
        }
    }
}
