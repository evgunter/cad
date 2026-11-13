//! **The carrier ladder read through the Boolean's doors**: the rows of
//! `geom_brep::carrier_eq` and `geom_brep::plane_eq` that need a `Body`
//! or one of the Boolean's own types, beside the face-pair door
//! ([`carrier_pair`](super::carrier_pair)) that reads the ladder over
//! two faces.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::carrier_eq::fixtures::{OrientationPose, orientation_in_zero_band, torus};
use geom_brep::{
    CarrierEqError, CarrierRelation, CarrierVerdict, ConsumedExtent, PlaneRung, carrier_eq,
    oriented_plane_eq,
};
use geom_core::{Indeterminate, MarginDiag, Point3, Tol, Vec3};

use super::carrier_pair::{carrier_pair_decided, carrier_pair_relation};
use super::{BooleanDecision, BooleanError, Operand, off_ring_torus};

fn band() -> geom_core::Band {
    geom_core::Band::linear(Tol::witness()).unwrap()
}

/// **Two torus faces are one carrier exactly where their pair's pose
/// decides them one surface** ([`geom_brep::tori_coincide`]), whatever
/// extent the faces span, read at the face-pair door
/// ([`carrier_pair_decided`]). The poses are the ring
/// of `R = 2`, `r = 0.5` tilted so its far side swings `0.3ε`, slid
/// `0.6ε` along and across its axis, its tube in the band, and its
/// tube a thousand escalation widths wider, whose faces' corners
/// witness it. Each pair is two opposed patches of one chart window,
/// a quarter turn and a smaller one.
#[test]
fn the_ladder_reads_two_tori_as_their_pose_does() {
    use crate::boxes::tests::torus_wall;
    let b = band();
    let e = b.zero();
    let surface = |(center, axis, major, minor): (Point3<f64>, Vec3<f64>, f64, f64)| {
        let axis = axis.normalize();
        geom::Surface::Torus {
            center,
            axis,
            major_radius: major,
            minor_radius: minor,
            u_ref: axis.orthonormal_basis().0,
        }
    };
    let ring = (Point3::origin(), Vec3::unit_z(), 2.0, 0.5);
    for (what, other, one) in [
        (
            "tilted",
            (Point3::origin(), Vec3::new(0.148 * e, 0.0, 1.0), 2.0, 0.5),
            Some(true),
        ),
        (
            "slid",
            (Point3::new(0.6 * e, 0.0, 0.6 * e), Vec3::unit_z(), 2.0, 0.5),
            Some(true),
        ),
        (
            "in band",
            (
                Point3::origin(),
                Vec3::unit_z(),
                2.0,
                0.5 + 0.5 * (e + b.escalate()),
            ),
            None,
        ),
        (
            "apart",
            (
                Point3::origin(),
                Vec3::unit_z(),
                2.0,
                0.5 + 1000.0 * b.escalate(),
            ),
            Some(false),
        ),
    ] {
        let pose = match geom_brep::TorusTorusPose::decide(&surface(ring), &surface(other), b)
            .expect("two tori")
        {
            Err(geom_brep::TorusPoseRefusal::Coincident) => Some(true),
            Err(geom_brep::TorusPoseRefusal::Undecided { .. }) => None,
            Ok(_) => Some(false),
            Err(refusal) => panic!("{what}: the pose refused {refusal:?}"),
        };
        assert_eq!(pose, one, "{what}: the pose");
        for us in [(0.3, 1.9), (0.3, 0.9)] {
            let wall = |(center, axis, major, minor): (Point3<f64>, Vec3<f64>, f64, f64)| {
                let axis = axis.normalize();
                let u_ref = axis.orthonormal_basis().0;
                torus_wall(center, axis, u_ref, major, minor, us, (-0.7, 0.8))
            };
            let (a, fa) = wall(ring);
            let (mut o, fo) = wall(other);
            let sense = a.get_face(fa).unwrap().sense;
            o.set_face_sense(fo, !sense).unwrap();
            let read = match carrier_pair_decided(&a, fa, &o, fo, b) {
                Ok(Ok(CarrierVerdict::One {
                    relation: CarrierRelation::SameOpposite,
                    ..
                })) => Some(true),
                Ok(Ok(CarrierVerdict::Apart(_))) => Some(false),
                Ok(Err(CarrierEqError::Undecided { .. })) => None,
                other => panic!("{what}: the door read {other:?}"),
            };
            assert_eq!(read, pose, "{what}: the door over {us:?}");
        }
    }
}

/// **A torus off its ring convention ends at the Boolean as the
/// pierce normal ends it** (`boolean::off_ring_torus`), never as
/// poison: the face it names refused in band as the half's
/// escalation ([`BooleanDecision::Torus`]) and decided as
/// [`BooleanError::DegenerateTorus`], each ending in the
/// half's lever, on the operand and face of the torus the ladder
/// named.
#[test]
fn a_torus_off_its_ring_ends_in_its_halfs_lever_at_the_boolean() {
    use geom_brep::TorusConvention::{Ring, Tube};
    let b = band();
    let mid = 0.5 * (b.zero() + b.escalate());
    let key = |n: u64| crate::entity::FaceKey::from(slotmap::KeyData::from_ffi((1 << 32) | n));
    let ring = torus([0.0; 3], [0.0, 0.0, 1.0], 2.0, 0.5, true);
    let tube = "reshape the torus so its tube is clearly thicker than the tolerance";
    let ring_lever = "make the tube radius clearly smaller than the ring radius";
    for (what, major, minor, half, in_band, lever) in [
        ("tube in band", 2.0, mid, Tube, true, tube),
        ("tube negative", 2.0, -0.5, Tube, false, tube),
        ("ring in band", 0.5 + mid, 0.5, Ring, true, ring_lever),
        ("horn", 0.5, 0.5, Ring, false, ring_lever),
        ("spindle", 0.5, 2.0, Ring, false, ring_lever),
    ] {
        let off = torus([0.0; 3], [0.0, 0.0, 1.0], major, minor, true);
        for (side, (x, y), named) in [
            (
                geom_brep::PairSide::Second,
                (ring, off),
                (Operand::B, key(2)),
            ),
            (
                geom_brep::PairSide::First,
                (off, ring),
                (Operand::A, key(1)),
            ),
        ] {
            let what = format!("{what}, {side:?}");
            let Err(CarrierEqError::OffRing { torus, refusal }) =
                carrier_eq(&x, &y, &ConsumedExtent::arm(1.0), b)
            else {
                panic!("{what}: the ladder refuses the convention");
            };
            assert_eq!(torus, side, "{what}");
            let err = off_ring_torus(torus, refusal, [(Operand::A, key(1)), (Operand::B, key(2))]);
            match (&err, in_band) {
                (
                    BooleanError::Escalated {
                        decision: BooleanDecision::Torus(got),
                        ..
                    },
                    true,
                ) => assert_eq!(*got, half, "{what}"),
                (
                    BooleanError::DegenerateTorus {
                        convention,
                        operand,
                        face,
                        ..
                    },
                    false,
                ) => {
                    assert_eq!(*convention, half, "{what}");
                    assert_eq!(
                        (*operand, *face),
                        named,
                        "{what}: the face the ladder named"
                    );
                }
                _ => panic!("{what}: the half's refusal, got {err:?}"),
            }
            let text = err.render(geom_brep::recourse::Reading::Build);
            assert!(
                text.contains(lever) && !text.contains("not finite"),
                "{what}: the half's lever: {text}"
            );
        }
    }
}

/// **The curved ladder's material side is the faces' `outward`
/// bits.** One cylinder face against itself, against its reverted
/// body, and against a twin with its sense flipped, each pair both
/// ways: the reverted and the flipped pair read opposed.
#[test]
fn the_curved_ladder_reads_a_reverted_face_as_opposed() {
    use CarrierRelation::{SameOpposite, SameOriented};
    let mut body = crate::Body::<f64>::new();
    let (face, _) = crate::test_support_fixtures::unit_cyl_sheet(
        &mut body,
        None,
        (0.0, 1.0),
        (0.0, 1.0),
        true,
        Tol::witness(),
    );
    let reverted = body.revert().unwrap();
    let mut flipped = body.clone();
    flipped.set_face_sense(face, false).unwrap();
    for (name, other, rung) in [
        ("itself", &body, SameOriented),
        ("its reverted body", &reverted, SameOpposite),
        ("its sense flipped", &flipped, SameOpposite),
    ] {
        for (x, y) in [(&body, other), (other, &body)] {
            assert_eq!(
                carrier_pair_relation(x, face, y, face, band())
                    .unwrap()
                    .unwrap(),
                rung,
                "the curved ladder, a face against {name}"
            );
        }
    }
}

/// **An orientation decided zero ends as its untagged margin does.**
/// Over the zero-band poses `geom_brep::plane_eq`'s own row pins (the
/// gate's rejection, tagged Zero, on the log), the tag is the gate's
/// alone: the Boolean's ending renders the same as on the untagged
/// margin.
#[test]
fn an_orientation_decided_zero_ends_as_its_untagged_margin() {
    let band = band();
    for OrientationPose {
        what,
        up,
        other,
        extent,
        cos_arm,
    } in orientation_in_zero_band(band)
    {
        let Err(CarrierEqError::Escalated {
            rung: PlaneRung::Orientation,
            diag,
        }) = oriented_plane_eq(&up, &other, &extent, band)
        else {
            panic!("{what}: the orientation reads zero");
        };
        let untagged = Indeterminate {
            margin: MarginDiag::value(cos_arm),
            ..diag
        };
        let text = |d| BooleanError::plane_identity(PlaneRung::Orientation, d);
        assert_eq!(
            text(diag).render(geom_brep::recourse::Reading::Build),
            text(untagged).render(geom_brep::recourse::Reading::Build),
            "{what}: the ending reads the margin, not the gate's tag"
        );
    }
}
