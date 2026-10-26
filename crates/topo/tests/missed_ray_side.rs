//! **A body thin everywhere is sided far away from it.** A ray from a
//! point that crosses no face leaves the point on the at-infinity side,
//! read off a ray aimed at a planar face from beyond the body, off its
//! closest crossing (`side_reader::side_at_infinity`). The selection's
//! signed volume, whose margin `V/A` is `t/(2 + 4t)` for a slab `t`
//! thick, in band for every slab from `10ε` (the thinnest the prism door
//! builds, its span margin) to `20ε`, would not read it.
//!
//! Every pose scales with the run's ε, so each row runs at every ε.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use common::brick;
use geom_core::{Band, Bounds, Decide, Dual64, Point3, Tol};
use topo::{
    AtRestBody, AtRestOutcome, AtRestPolicy, Body, BooleanError, BooleanResult, Operand,
    SolidContainment, ValidationError, point_in_solid,
};

/// The slab thicknesses, in ε: the in-band window of `V/A`, its ends
/// included.
const WINDOW: [f64; 4] = [10.0, 12.0, 15.0, 20.0];

/// Points a metre or more off a unit-square slab at `z ∈ [0, t]`: above,
/// below, beside at mid-thickness, and off a corner.
fn far_points(t: f64) -> [Point3<f64>; 4] {
    [
        Point3::new(0.5, 0.5, 3.0),
        Point3::new(0.3, 0.6, -2.0),
        Point3::new(4.0, 0.5, t / 2.0),
        Point3::new(5.5, 5.5, 5.5),
    ]
}

fn slab(t: f64, tol: Tol) -> Body<f64> {
    brick((0.0, 1.0), (0.0, 1.0), (0.0, t), tol)
}

fn at_rest(body: Body<f64>, tol: Tol) -> AtRestBody<f64> {
    AtRestBody::validate(body, tol).expect("the operand is at rest")
}

fn far_brick(tol: Tol) -> AtRestBody<f64> {
    at_rest(brick((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), tol), tol)
}

/// `body`'s signed volume over its area lies in the band, so the volume
/// cannot read the side at infinity and the aimed rays have to.
fn volume_in_band(what: &str, body: &Body<f64>) {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let m = topo::mass_properties(
        &topo::test_support::finished("the measured body", (body).clone(), tol),
        tol,
        topo::test_support::pinned_resolution(tol),
    )
    .expect("the body measures");
    let va = (m.enclosure.volume / m.enclosure.surface_area).abs();
    assert!(
        va > band.zero() && va < band.escalate(),
        "{what}: |V/A| = {va:e} lies in the band"
    );
}

/// `body` answers `want` at every point of `points`.
fn sides(what: &str, body: &Body<f64>, points: &[Point3<f64>], want: SolidContainment) {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    for &q in points {
        assert_eq!(
            point_in_solid(body, q, band, tol),
            Ok(want),
            "{what}: {q:?} reads {want:?}"
        );
    }
}

/// **A thin slab reads `Out` far from it, and its complement `In`.**
/// The reverted slab bounds the slab's complement, so the same points
/// lie in its material: the aimed ray reads the face's own outward
/// normal, and no orientation is assumed.
#[test]
fn a_thin_slab_reads_out_far_away_and_its_complement_in() {
    let tol = Tol::witness();
    for m in WINDOW {
        let t = m * tol.eps();
        let body = slab(t, tol);
        volume_in_band(&format!("slab {m}ε"), &body);
        sides(
            &format!("slab {m}ε"),
            &body,
            &far_points(t),
            SolidContainment::Out,
        );
        sides(
            &format!("reverted slab {m}ε"),
            &body.revert().unwrap(),
            &far_points(t),
            SolidContainment::In,
        );
    }
}

/// **A thin slab is a legal operand against a brick far away.** The
/// containment fallback sides each shell by a witness of it read
/// against the other operand, and every witness of the brick lies far
/// from the slab. `slab ∩ brick` is empty in both orders, and
/// `brick ∖ slab` is the brick. `slab ∪ brick` is two solids: the pieces
/// sort reads the slab's role off a ray aimed at it.
#[test]
fn a_thin_slab_is_a_legal_operand_against_a_brick_far_away_and_its_union_with_it_is_two_solids() {
    let tol = Tol::witness();
    let far = far_brick(tol);
    for m in WINDOW {
        let t = m * tol.eps();
        let s = at_rest(slab(t, tol), tol);
        for (order, a, b) in [("slab ∩ brick", &s, &far), ("brick ∩ slab", &far, &s)] {
            assert!(
                matches!(topo::intersect(a, b, tol), Ok(BooleanResult::Empty)),
                "{m}ε: {order} is empty"
            );
        }
        let kept = topo::subtract(&far, &s, tol)
            .unwrap_or_else(|e| panic!("{m}ε: brick ∖ slab builds, got {e:?}"));
        let kept = kept.body().expect("brick ∖ slab is not empty");
        assert_eq!(
            (kept.body.solids().count(), kept.body.faces().count()),
            (1, 6),
            "{m}ε: brick ∖ slab is the brick"
        );
        let joined = topo::union(&s, &far, tol)
            .unwrap_or_else(|e| panic!("{m}ε: slab ∪ brick builds, got {e:?}"));
        let joined = joined.body().expect("slab ∪ brick is not empty");
        assert_eq!(
            joined.body.solids().count(),
            2,
            "{m}ε: slab ∪ brick is two solids"
        );
    }
}

/// **A thin-walled hollow (a void) reads `Out` far from it and in its
/// cavity**, and `In` at both once reverted. The whole body's `V/A` is
/// its wall's half-thickness, in band, while each shell's role decides;
/// so `hollow ∪ brick` builds, two solids. The window is narrower
/// here: at `10ε` the core's inset rounds the wall under the band at
/// ε = 1e-6, and the subtract refuses that coincidence; at `20ε` the
/// wall's `V/A` rounds just past the band, and the volume decides.
#[test]
fn a_thin_walled_hollow_reads_out_far_away_and_in_its_cavity() {
    let tol = Tol::witness();
    let far = far_brick(tol);
    for m in [12.0, 15.0, 18.0] {
        let d = m * tol.eps();
        let unit = at_rest(brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol), tol);
        let core = at_rest(brick((d, 1.0 - d), (d, 1.0 - d), (d, 1.0 - d), tol), tol);
        let hollow = topo::subtract(&unit, &core, tol)
            .unwrap_or_else(|e| panic!("{m}ε: the hollow builds, got {e:?}"));
        let hollow = &hollow.body().expect("the hollow is not empty").body;
        let shells: Vec<usize> = hollow.solids().map(|(_, s)| s.shells.len()).collect();
        assert_eq!(
            shells,
            [2],
            "{m}ε: one solid, its outer shell and its cavity"
        );
        let points = [
            Point3::new(0.5, 0.5, 3.0),
            Point3::new(5.5, 5.5, 5.5),
            Point3::new(0.5, 0.5, 0.5),
        ];
        volume_in_band(&format!("hollow {m}ε"), hollow);
        sides(
            &format!("hollow {m}ε"),
            hollow,
            &points,
            SolidContainment::Out,
        );
        sides(
            &format!("reverted hollow {m}ε"),
            &hollow.revert().unwrap(),
            &points,
            SolidContainment::In,
        );
        let hollow = at_rest((**hollow).clone(), tol);
        let joined = topo::union(&hollow, &far, tol)
            .unwrap_or_else(|e| panic!("{m}ε: hollow ∪ brick builds, got {e:?}"));
        let joined = &joined.body().expect("not empty").body;
        assert_eq!(
            joined.solids().count(),
            2,
            "{m}ε: hollow ∪ brick is two solids"
        );
        let wall = 1.0 - (1.0 - 2.0 * d).powi(3);
        let v =
            topo::mass_properties(joined, tol, topo::test_support::pinned_resolution(tol)).unwrap();
        assert!(
            (v.enclosure.volume - (wall + 1.0)).abs() <= v.enclosure.volume_pad + 1e-12,
            "{m}ε: hollow ∪ brick holds the wall and the brick: {} ± {} vs {}",
            v.enclosure.volume,
            v.enclosure.volume_pad,
            wall + 1.0
        );
    }
}

/// **Two thin slabs side by side in one body read `Out` between and
/// beyond them**, and `In` once reverted: a selection of two solids,
/// each thin, including points in their common mid-plane.
#[test]
fn two_thin_slabs_in_one_body_read_their_sides() {
    let tol = Tol::witness();
    for m in WINDOW {
        let t = m * tol.eps();
        let mut body = slab(t, tol);
        let beside = brick((2.0, 3.0), (0.0, 1.0), (0.0, t), tol);
        topo::graft_disjoint(&mut body, &beside).expect("the slabs are disjoint");
        volume_in_band(&format!("two slabs {m}ε"), &body);
        let points = [
            Point3::new(1.5, 0.5, 1.0),
            Point3::new(1.5, 0.5, t / 2.0),
            Point3::new(0.4, 0.7, -3.0),
            Point3::new(6.0, 0.5, t / 2.0),
        ];
        sides(
            &format!("two slabs {m}ε"),
            &body,
            &points,
            SolidContainment::Out,
        );
        sides(
            &format!("two reverted slabs {m}ε"),
            &body.revert().unwrap(),
            &points,
            SolidContainment::In,
        );
    }
}

/// Every op in both orders between `rev` and each of `others` refuses
/// `InsideOutOperand`, naming `rev`'s operand and its one solid inside
/// out (check 7's `NegativeVolume`).
fn refused_as_inside_out<T: Decide + Bounds + AtRestPolicy>(
    what: &str,
    rev: &AtRestBody<T>,
    others: &[(&str, AtRestBody<T>)],
) {
    type Op<T> = fn(&AtRestBody<T>, &AtRestBody<T>, Tol) -> Result<BooleanResult<T>, BooleanError>;
    let tol = Tol::witness();
    let ops: [(&str, Op<T>); 3] = [
        ("∪", topo::union::<T>),
        ("∩", topo::intersect::<T>),
        ("∖", topo::subtract::<T>),
    ];
    for (place, other) in others {
        for (name, op) in ops {
            for (a, b, inside_out) in [(rev, other, Operand::A), (other, rev, Operand::B)] {
                match op(a, b, tol) {
                    Err(BooleanError::InsideOutOperand { operand, errors }) => {
                        assert_eq!(operand, inside_out, "{what} {place} {name}: the operand");
                        assert!(
                            matches!(errors[..], [ValidationError::NegativeVolume { .. }]),
                            "{what} {place} {name}: {errors:?}"
                        );
                    }
                    other => panic!(
                        "{what} {place} {name} ({inside_out:?} inside-out): want \
                         InsideOutOperand, got {:?}",
                        other.map(|_| ())
                    ),
                }
            }
        }
    }
}

/// **An inside-out slab never finishes at rest.** Its `V/A` is in band,
/// but check 7 reads its orientation off a ray aimed at one of its faces,
/// which crosses into its material first: the at-rest gate refuses it,
/// naming its solid inside out.
#[test]
fn an_inside_out_slab_refuses_at_rest() {
    let tol = Tol::witness();
    for m in WINDOW {
        let rev = slab(m * tol.eps(), tol).revert().unwrap();
        let solid = rev.solids().next().expect("one solid").0;
        assert_eq!(
            AtRestBody::validate(rev, tol).map(|_| ()),
            Err(vec![ValidationError::NegativeVolume { solid }]),
            "{m}ε: the at-rest gate refuses the inside-out slab"
        );
    }
}

/// **At a dual, the operand gate refuses it.** A dual's policy runs no
/// at-rest gate, so the inside-out slab reaches the Boolean with no
/// verdict; the operand gate reads every solid of such an operand with
/// check 7's reader, and refuses it in every op and both orders.
#[test]
fn an_inside_out_slab_is_refused_as_an_operand_at_a_dual() {
    let tol = Tol::witness();
    let kept =
        |b: Body<Dual64>| Dual64::gate_at_rest_kept(b, tol).expect("a dual gate runs nothing");
    let others = [
        (
            "far",
            kept(brick::<Dual64>((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), tol)),
        ),
        (
            "across",
            kept(brick::<Dual64>((0.5, 1.5), (0.5, 1.5), (-1.0, 1.0), tol)),
        ),
    ];
    for m in WINDOW {
        let t = m * tol.eps();
        let rev = kept(
            brick::<Dual64>((0.0, 1.0), (0.0, 1.0), (0.0, t), tol)
                .revert()
                .unwrap(),
        );
        assert_eq!(rev.outcome(), AtRestOutcome::NotRunAtThisScalar);
        refused_as_inside_out(&format!("dual {m}ε"), &rev, &others);
    }
}
