//! **A vertex whose three edges mix convexity refuses only what it
//! cannot build.** The ODD edge is the one that rounds unlike the other
//! two, the PAIR. Requested alone, any one edge there is cut off as at a
//! uniform vertex; the pair alone meets in the mitre, which at their
//! shared face's reflex sector is a join over the wedge the two bands'
//! extensions overlap in — `d³/3` for a chamfer and `(5/3 − π/2)·r³` for
//! a fillet at right angles, removed past a convex pair and filled past
//! a concave one. The odd edge requested with one other edge there
//! chamfers, the other strip cut off in the odd strip's plane
//! (`band_mixed_corner_odd_plus_one`), and refuses the fillet's
//! run-out (`UnbuiltEnd::RunOut`); all three build the mixed corner
//! (`band_mixed_corner`).
//!
//! The section a band takes from (or adds to) a right dihedral is
//! `d²/2` for the chamfer and `r²(1 − π/4)` for the fillet
//! (`Verb::section`), so a band cut off square at both ends changes the
//! volume by the section times its edge's length.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Bounds, Interval, Point3};
use sweep::blend::build::{chamfer_edges, fillet_edges};
use sweep::blend::{BlendError, FILLET3_MIXED_CORNER_RECOURSE, UnbuiltEnd};
use sweep::test_support::{assert_naming_totality, corners, finished, prism_at, realized};
use topo::boolean::BooleanOp;
use topo::{Body, SolidContainment, VertexKey, point_in_solid, validate_geometric};

use crate::band_planar_cut_off::{D, Verb, carve, edge, tol};
use crate::band_planar_mitre::overlap;
use crate::common::interval::iv;

/// **The L-bar**: an L-shaped section, legs 3 and 2 long and 0.5 thick,
/// extruded 1 up. Its inside edge at (0.5, 0.5) is concave and the two
/// rims it meets at each end are convex.
fn l_bar<T: geom_core::Decide + topo::AtRestPolicy>() -> Body<T> {
    prism_at(
        corners(&[
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 0.5),
            (0.5, 0.5),
            (0.5, 2.0),
            (0.0, 2.0),
        ]),
        T::from_f64(0.0),
        T::from_f64(1.0),
        tol(),
    )
}

/// **A plate with a boss**: a 6 × 4 × 0.5 plate and a 0.5 × 3 boss on it
/// standing to z = 2, its low side at `x = inset` (flush with the
/// plate's `x = 0` side at `inset = 0`). Each vertex where the boss
/// meets the plate mixes convexity.
pub(crate) fn plate_boss(inset: f64) -> Body<f64> {
    let plate = prism_at(
        corners(&[(0.0, 0.0), (6.0, 0.0), (6.0, 4.0), (0.0, 4.0)]),
        0.0,
        0.5,
        tol(),
    );
    let boss = prism_at(
        corners(&[
            (inset, 0.5),
            (inset + 0.5, 0.5),
            (inset + 0.5, 3.5),
            (inset, 3.5),
        ]),
        0.25,
        1.75,
        tol(),
    );
    realized(BooleanOp::Union, &plate, &boss, tol())
}

/// **A curved link alone at a mixed vertex refuses as at a uniform
/// one**, both verbs: the arc root of a quarter-cylinder boss on a
/// plate ends at vertices where the boss's convex vertical edges meet
/// the concave root, and it refuses exactly as the same arc does at a
/// lone quarter-cylinder's uniform corner — its own arm's refusal,
/// never the mixed corner's.
#[test]
fn a_curved_link_alone_at_a_mixed_vertex_refuses_as_at_a_uniform_one() {
    let bulge = (core::f64::consts::PI / 8.0).tan();
    let quarter = || {
        vec![
            (geom_core::Point2::new(1.0, 0.5), 0.0),
            (geom_core::Point2::new(2.0, 0.5), bulge),
            (geom_core::Point2::new(1.0, 1.5), 0.0),
        ]
    };
    let plate = prism_at(
        corners(&[(0.0, 0.0), (6.0, 0.0), (6.0, 4.0), (0.0, 4.0)]),
        0.0,
        0.5,
        tol(),
    );
    let boss = prism_at(quarter(), 0.25, 1.75, tol());
    let body = realized(BooleanOp::Union, &plate, &boss, tol());
    let arc = edge(&body, [2.0, 0.5, 0.5], [1.0, 1.5, 0.5]);
    let lone = prism_at(quarter(), 0.0, 1.5, tol());
    let lone_arc = edge(&lone, [2.0, 0.5, 0.0], [1.0, 1.5, 0.0]);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let mixed = verb.run(&body, &[arc]).map(|_| ()).unwrap_err();
        let uniform = verb.run(&lone, &[lone_arc]).map(|_| ()).unwrap_err();
        assert_eq!(
            core::mem::discriminant(&mixed),
            core::mem::discriminant(&uniform),
            "{verb:?}: as at a uniform vertex: {mixed:?} vs {uniform:?}"
        );
        assert!(
            !mixed
                .render(geom_brep::recourse::Reading::Build)
                .contains(FILLET3_MIXED_CORNER_RECOURSE),
            "{verb:?}: not the mixed corner's recourse: {mixed:?}"
        );
    }
}

pub(crate) fn vertex_at(body: &Body<f64>, q: [f64; 3]) -> VertexKey {
    body.vertices()
        .find(|(_, v)| {
            let p = body.get_point(v.point).unwrap();
            (p.x - q[0]).abs() + (p.y - q[1]).abs() + (p.z - q[2]).abs() < 1e-12
        })
        .map(|(k, _)| k)
        .unwrap_or_else(|| panic!("a vertex at {q:?}"))
}

/// **The L-bar's inside edge alone**, both verbs: its band is cut off at
/// the two end faces, at each end a vertex whose two rims are convex,
/// and fills `section · 1`.
#[test]
fn the_l_bars_inside_edge_alone_is_cut_off_at_both_ends() {
    let body = l_bar::<f64>();
    let inside = edge(&body, [0.5, 0.5, 0.0], [0.5, 0.5, 1.0]);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        carve(
            &body,
            &[inside],
            verb,
            -verb.section(),
            "the L-bar's inside edge",
        );
    }
}

/// **The L-bar's inside edge at the certified scalar**, both verbs:
/// carved, tier-3 valid, naming total, and the volume enclosure
/// brackets the closed form narrowly enough to be a claim.
#[test]
fn the_l_bars_inside_edge_carves_at_the_certified_scalar() {
    let tol = tol();
    let body = finished("the L-bar", l_bar::<Interval>(), tol);
    let inside = edge(&body, [0.5, 0.5, 0.0], [0.5, 0.5, 1.0]);
    let volume = |b: &Body<Interval>| {
        let p = topo::mass_properties(
            &finished("the measured body", Body::clone(b), tol),
            tol,
            topo::test_support::pinned_resolution(tol),
        )
        .expect("interval props");
        (p.enclosure.volume, p.enclosure.volume_pad)
    };
    let (v0, pad0) = volume(&body);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let out = match verb {
            Verb::Chamfer => chamfer_edges(&body, &[inside], iv(D), tol),
            Verb::Fillet => fillet_edges(&body, &[inside], iv(D), tol),
        }
        .unwrap_or_else(|e| panic!("{verb:?}: carves at Interval, got {e:?}"));
        validate_geometric(&out.body, tol).unwrap_or_else(|e| panic!("{verb:?}: tier 3, {e:?}"));
        assert_naming_totality(
            &body,
            &out,
            &[inside],
            "the L-bar's inside edge at Interval",
        );
        let (v1, pad1) = volume(&out.body);
        let added = v1 - v0;
        let pad = pad0 + pad1;
        let truth = verb.section();
        assert!(
            added.lo() - pad <= truth && truth <= added.hi() + pad,
            "{verb:?}: ΔV {added:?} ± {pad} brackets {truth}"
        );
        assert!(
            added.hi() - added.lo() + 2.0 * pad < 1e-5,
            "{verb:?}: the enclosure is a claim: {added:?} ± {pad}"
        );
    }
}

/// **The inset boss's vertical edge alone**, both verbs: convex, cut off
/// at the plate's top where both root edges are concave and at the
/// boss's top, removing `section · 1.5`.
#[test]
fn the_inset_bosss_vertical_edge_alone_is_cut_off_at_the_plate() {
    let body = plate_boss(1.0);
    let up = edge(&body, [1.0, 0.5, 0.5], [1.0, 0.5, 2.0]);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        carve(
            &body,
            &[up],
            verb,
            verb.section() * 1.5,
            "the inset vertical",
        );
    }
}

/// **The flush pair**: the plate's top edge along `x = 0` and the boss's
/// vertical edge above it, both convex, the concave root edge between
/// them unrequested. They turn on the shared `x = 0` face, whose sector
/// at the vertex V is reflex, so the mitre joins over the wedge the two
/// bands' extensions overlap in inside the plate: `ΔV = −(section · L +
/// wedge)`, `L = 0.5 + 1.5` the two edges' lengths, each band cut off
/// square at its far end. The point V + (d/2)(ŷ − ẑ) + ε·x̂ lies in that
/// wedge, so it is outside the result.
#[test]
fn the_flush_pair_meets_in_a_mitre_that_removes_the_wedge() {
    let body = plate_boss(0.0);
    let rim = edge(&body, [0.0, 0.0, 0.5], [0.0, 0.5, 0.5]);
    let up = edge(&body, [0.0, 0.5, 0.5], [0.0, 0.5, 2.0]);
    let band = Band::linear(tol()).expect("the witness band");
    let probe = Point3::new(D / 100.0, 0.5 + D / 2.0, 0.5 - D / 2.0);
    assert_eq!(
        point_in_solid(&body, probe, band, tol()).expect("classifies"),
        SolidContainment::In,
        "the probe is material before the blend"
    );
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let out = carve(
            &body,
            &[rim, up],
            verb,
            verb.section() * 2.0 + overlap(verb),
            "the flush pair",
        );
        let rec = out.naming.as_ref().expect("births");
        assert_eq!(rec.mitres.len(), 1, "{verb:?}: one mitre");
        assert_eq!(
            point_in_solid(&out.body, probe, band, tol()).expect("classifies"),
            SolidContainment::Out,
            "{verb:?}: the wedge is removed"
        );
    }
}

/// **A flush pair whose feet stand apart refuses at its overrun**, both
/// verbs: skew the boss's front face so the vertical edge's dihedral is
/// no longer square, and the two bands' feet on the root edge part. The
/// far band's cut-off past the mitre then lands its foot inside a face,
/// the cut-off's typed refusal, at the turn's vertex.
#[test]
fn a_flush_pair_whose_feet_stand_apart_refuses_at_its_overrun() {
    let plate = prism_at(
        corners(&[(0.0, 0.0), (6.0, 0.0), (6.0, 4.0), (0.0, 4.0)]),
        0.0,
        0.5,
        tol(),
    );
    let boss = prism_at(
        corners(&[(0.0, 0.5), (0.5, 0.6), (0.5, 3.5), (0.0, 3.5)]),
        0.25,
        1.75,
        tol(),
    );
    let body = realized(BooleanOp::Union, &plate, &boss, tol());
    let rim = edge(&body, [0.0, 0.0, 0.5], [0.0, 0.5, 0.5]);
    let up = edge(&body, [0.0, 0.5, 0.5], [0.0, 0.5, 2.0]);
    let turn = vertex_at(&body, [0.0, 0.5, 0.5]);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        match verb.run(&body, &[rim, up]) {
            Err(BlendError::UnbuiltEnd(UnbuiltEnd::FootInsideAFace { vertex })) => assert!(
                vertex == turn,
                "{verb:?}: at the turn, a foot inside a face: {vertex:?}"
            ),
            got => panic!(
                "{verb:?}: the overrun refuses typed, got {:?}",
                got.map(|_| ())
            ),
        }
    }
}

/// **The inset boss's root rim**, both verbs: four concave edges, each
/// two meeting at a corner whose vertical edge is unrequested and
/// convex, over the plate's top face, reflex there. Each mitre fills
/// the mirror wedge: `ΔV = +(section · 7 + 4 · wedge)`.
///
/// Two of those root edges alone refuse at their far ends, not at the
/// corner they share: there each is cut off by a face of the boss, and
/// its foot on the plate's top lands inside that face, past the boss.
#[test]
fn the_inset_root_rim_fills_the_mirror_wedge_at_each_corner() {
    let body = plate_boss(1.0);
    let front = edge(&body, [1.0, 0.5, 0.5], [1.5, 0.5, 0.5]);
    let side = edge(&body, [1.0, 0.5, 0.5], [1.0, 3.5, 0.5]);
    let back = edge(&body, [1.0, 3.5, 0.5], [1.5, 3.5, 0.5]);
    let far = edge(&body, [1.5, 0.5, 0.5], [1.5, 3.5, 0.5]);
    let far_ends = [
        vertex_at(&body, [1.5, 0.5, 0.5]),
        vertex_at(&body, [1.0, 3.5, 0.5]),
    ];
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let out = carve(
            &body,
            &[front, side, back, far],
            verb,
            -(verb.section() * 7.0 + 4.0 * overlap(verb)),
            "the inset root rim",
        );
        let rec = out.naming.as_ref().expect("births");
        assert_eq!(rec.mitres.len(), 4, "{verb:?}: four mitres");
        match verb.run(&body, &[front, side]) {
            Err(BlendError::UnbuiltEnd(UnbuiltEnd::FootInsideAFace { vertex })) => assert!(
                far_ends.contains(&vertex),
                "{verb:?}: at a far end, a foot inside a face: {vertex:?}"
            ),
            got => panic!(
                "{verb:?}: two root edges refuse at a far foot, got {:?}",
                got.map(|_| ())
            ),
        }
    }
}

/// **The odd edge with one other edge of its corner refuses**: the
/// fillet at both fixtures, and the chamfer where the face the other two
/// share leans off square to the odd edge
/// (`band_mixed_corner::leaning`): the fillet's run-out and the
/// chamfer's `ObliqueMixedCorner` at the corner, each with the mixed
/// corner's rule for its recourse. The
/// flush odd edge is the concave root, the inset one the convex
/// vertical; square, the chamfer builds
/// ([`the_odd_edge_with_one_other_chamfers_at_the_flush_fixture`]).
#[test]
fn the_odd_edge_with_another_edge_of_its_corner_refuses() {
    let flush = plate_boss(0.0);
    let inset = plate_boss(1.0);
    let f = |a, b| edge(&flush, a, b);
    let i = |a, b| edge(&inset, a, b);
    let rows = [
        (
            "flush root and vertical",
            &flush,
            [0.0, 0.5, 0.5],
            vec![
                f([0.0, 0.5, 0.5], [0.5, 0.5, 0.5]),
                f([0.0, 0.5, 0.5], [0.0, 0.5, 2.0]),
            ],
        ),
        (
            "inset vertical and root",
            &inset,
            [1.0, 0.5, 0.5],
            vec![
                i([1.0, 0.5, 0.5], [1.0, 0.5, 2.0]),
                i([1.0, 0.5, 0.5], [1.5, 0.5, 0.5]),
            ],
        ),
    ];
    let leaning = crate::band_mixed_corner::leaning();
    let oblique = crate::band_mixed_corner::leaning_odd_and_one(&leaning).to_vec();
    let rows = rows
        .into_iter()
        .map(|(what, body, at, edges)| (what, body, at, edges, Verb::Fillet))
        .chain([(
            "oblique root and vertical",
            &leaning,
            [0.5, 0.0, 0.0],
            oblique,
            Verb::Chamfer,
        )]);
    for (what, body, at, edges, verb) in rows {
        let corner = vertex_at(body, at);
        let err = verb.run(body, &edges).map(|_| ()).unwrap_err();
        assert!(
            match verb {
                Verb::Fillet => matches!(
                    err,
                    BlendError::UnbuiltEnd(UnbuiltEnd::RunOut { vertex, odd: Some(_), .. })
                        if vertex == corner
                ),
                Verb::Chamfer => matches!(
                    err,
                    BlendError::ObliqueMixedCorner { vertex, .. } if vertex == corner
                ),
            },
            "{what} ({verb:?}): refuses at the corner, got {err:?}"
        );
        let shown = err.render(geom_brep::recourse::Reading::Build);
        assert!(
            shown.contains(FILLET3_MIXED_CORNER_RECOURSE) && !shown.contains("feather"),
            "{what} ({verb:?}): {shown}"
        );
    }
}

/// **The odd edge with one other edge of its corner chamfers**
/// (`band_mixed_corner_odd_plus_one`): at the flush fixture's corner
/// `(0, 0.5, 0.5)` the root (odd, `L = 0.5`) with the vertical
/// (`Y = 1.5`), the pair strip cut off in the odd strip's plane, which
/// adds `(s²/2)(L − Y) + (2/3)s³`. The root's far end is an inset
/// corner, where a concave root alone would land its foot inside the
/// plate, so the boss's other two roots are requested with it: they
/// meet it and each other in the inset corners' mitres, each filling
/// its wedge `s³/3`, and the last is cut off alone in the flush side.
#[test]
fn the_odd_edge_with_one_other_chamfers_at_the_flush_fixture() {
    let flush = plate_boss(0.0);
    let f = |a, b| edge(&flush, a, b);
    let edges = [
        f([0.0, 0.5, 0.5], [0.5, 0.5, 0.5]),
        f([0.0, 0.5, 0.5], [0.0, 0.5, 2.0]),
        f([0.5, 0.5, 0.5], [0.5, 3.5, 0.5]),
        f([0.0, 3.5, 0.5], [0.5, 3.5, 0.5]),
    ];
    let added = D * D / 2.0 * (0.5 - 1.5 + 3.0 + 0.5) + (2.0 / 3.0 + 2.0 / 3.0) * D.powi(3);
    carve(
        &flush,
        &edges,
        Verb::Chamfer,
        -added,
        "flush root and vertical, with the other roots",
    );
}
