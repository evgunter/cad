//! **A glued pair is read through its record** (`[coincidence.booleans]`:
//! nothing is decided twice). The glue door decides a pair once, on its
//! margin over both faces, and records it; these rows pin what the
//! consuming sites read of such a pair, and the n-ary fold step that is
//! given one.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::carrier_eq;
use super::carrier_pair::face_carrier;
use super::{Glue, GluedPairs, Operand, glue, verify_declared_contacts};
use crate::AtRestBody;
use crate::contact::GlueClass;
use crate::test_support::finished;
use crate::test_support_fixtures::{
    FaceGeometry, brick, describe_as_intersections, prism_ops, prism_z,
};
use geom_core::{Band, Margin, Point3, Tol, Vec3};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// The glue door over `a` and `b`, as a boolean runs it: the pairs it
/// decided, and the index the consuming sites read them through.
fn door(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> (Glue<f64>, GluedPairs<f64>) {
    let none = Glue::none();
    let decls = glue::decided_declarations(a, b, &none, band())
        .unwrap()
        .into_owned();
    let verified = verify_declared_contacts(a, b, &decls, band()).unwrap();
    let declared = GluedPairs::build(&decls, verified, a, b, Default::default(), band()).unwrap();
    (decls, declared)
}

/// **A pair the door glued reads, at its consuming site, the relation
/// the door recorded, in both operand orders, and the face-pair door
/// over the extent it was decided on agrees.** A brick resting on a
/// cube (`Rest`) and two cubes side by side (continuations, and `Rest`
/// across their shared wall): every one-carrier pair the door glued
/// reads its class's relation from the record
/// ([`GluedPairs::glued_relation`]) whichever operand is asked first,
/// and its carriers read over the pair's extent decide the same relation
/// on a Zero.
#[test]
fn a_glued_pair_reads_its_record_in_both_orders() {
    let cube = || {
        finished(
            "the cube",
            brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()),
            tol(),
        )
    };
    let scenes = [
        (
            "a brick on a cube",
            cube(),
            finished(
                "the brick",
                brick((0.25, 0.75), (0.25, 0.75), (1.0, 2.0), tol()),
                tol(),
            ),
        ),
        (
            "two cubes side by side",
            cube(),
            finished(
                "the neighbour",
                brick((1.0, 2.0), (0.0, 1.0), (0.0, 1.0), tol()),
                tol(),
            ),
        ),
    ];
    let mut classes = std::collections::BTreeSet::new();
    for (what, a, b) in &scenes {
        let (decls, declared) = door(a, b);
        for pair in decls.pairs.iter().filter(|p| p.class.is_one_carrier()) {
            classes.insert(pair.class);
            let (ca, cb) = (
                face_carrier(a, pair.a).unwrap(),
                face_carrier(b, pair.b).unwrap(),
            );
            for (c1, c2, o1, f1, o2, f2) in [
                (ca, cb, Operand::A, pair.a, Operand::B, pair.b),
                (cb, ca, Operand::B, pair.b, Operand::A, pair.a),
            ] {
                let extent = declared.consumed(o1, f1, o2, f2).unwrap();
                let (rel, margin) = carrier_eq::pair_door_reading(&c1, &c2, &extent, band())
                    .unwrap_or_else(|e| panic!("{what}, {o1:?} first: {e:?}"));
                assert_eq!(
                    (
                        declared.glued_relation(o1, f1, o2, f2),
                        GlueClass::of_senses(rel),
                        margin.is_some()
                    ),
                    (pair.class.carrier_relation(), Some(pair.class), true),
                    "{what}, {o1:?} first: the consumer reads the door's record, and the \
                     carriers agree on a Zero"
                );
            }
        }
    }
    assert_eq!(
        classes,
        [GlueClass::Rest, GlueClass::Continuation].into(),
        "both one-carrier classes are read"
    );
}

/// A point turned by `angle` about the line through `c` along the unit
/// `u` (Rodrigues).
fn turned(p: [f64; 3], c: [f64; 3], u: [f64; 3], angle: f64) -> Point3<f64> {
    let v = Vec3::new(p[0] - c[0], p[1] - c[1], p[2] - c[2]);
    let u = Vec3::new(u[0], u[1], u[2]);
    let (s, k) = angle.sin_cos();
    let r = v * k + u.cross(v) * s + u * (u.dot(v) * (1.0 - k));
    Point3::new(c[0] + r.x, c[1] + r.y, c[2] + r.z)
}

/// **A glued sector whose tilt at its arm is in band.** A prism on a
/// unit equilateral triangle rests on a prism on that triangle scaled by
/// 1.05 about its centroid, so each of the upper prism's bottom corners
/// pierces the lower prism's top, and its two bounds run inside that top
/// to the next corner: the sector's arm is the unit side. The upper
/// prism is turned about the bisector-square line through the centroid
/// by `1.1·ε` over that arm. Over the ball the door reads the pair on
/// (radius ≈ 0.69) the tilt is Zero and the pair glues `Rest`, its
/// bounds each read Zero off the top (`cos 30°` of the tilt, or none),
/// and the normals' cross levered at the unit arm is in band: the lump
/// the corner takes is the glued pair's, which reads no tilt of its own.
#[test]
fn a_glued_sector_with_an_in_band_tilt_at_its_arm_lumps_as_glued() {
    let h = 3f64.sqrt() / 2.0;
    let tri = [(0.0, 0.0), (1.0, 0.0), (0.5, h)];
    let c = [0.5, h / 3.0, 0.0];
    let scaled: Vec<(f64, f64)> = tri
        .iter()
        .map(|&(x, y)| (c[0] + 1.05 * (x - c[0]), c[1] + 1.05 * (y - c[1])))
        .collect();
    let lower = finished(
        "the lower prism",
        prism_z(&scaled, -1.0, 0.0, tol()).body,
        tol(),
    );
    // Square to the bisector at the first corner, in the plane.
    let axis = [-0.5, h, 0.0];
    let theta = 1.1 * band().zero();
    let mut body = crate::body::Body::<f64>::new();
    prism_ops(
        &mut body,
        &tri,
        (0.0, 1.0),
        |x, y, z| turned([x, y, z], c, axis, theta),
        FaceGeometry::Certified,
        tol(),
    );
    describe_as_intersections(&mut body, tol());
    let upper = finished("the tilted prism", body, tol());

    let (decls, _) = door(&lower, &upper);
    let glued = decls
        .pairs
        .iter()
        .find(|p| p.class == GlueClass::Rest)
        .expect("the door glues the top against the tilted bottom, Rest");
    let (na, nb) = match (
        face_carrier(&lower, glued.a).unwrap(),
        face_carrier(&upper, glued.b).unwrap(),
    ) {
        (
            carrier_eq::CarrierDesc::Plane { normal: na, .. },
            carrier_eq::CarrierDesc::Plane { normal: nb, .. },
        ) => (na, nb),
        other => panic!("two planes: {other:?}"),
    };
    assert!(
        crate::validate::decide(
            "bool_sector_coplanar",
            Margin::levered(na.cross(nb).norm(), 1.0),
            band()
        )
        .is_err(),
        "the tilt levered at the sector's unit arm is in band"
    );
    let out = super::union(&lower, &upper, tol());
    assert!(
        out.is_ok(),
        "the union lumps each corner as the glued pair: {:?}",
        out.err()
    );
}

/// **A fold step given two Zeros whose ends stand in band reads the
/// third member's vertices off the merged face by value, and refuses
/// there**: no ladder meets the ends' gap. Three unit blocks in a row
/// along x: the first's top flat at `z = 1`, the other two's tops one
/// plane falling seven tenths of ε per unit from `(1, 1)`. The outer two
/// never meet, so they are never judged; the first two tops glue on a
/// Zero over both (the fall over their ball is under ε), and the last
/// two are one plane. The first and last tops stand in band over the
/// three blocks' ball. The second step joins the third block to the
/// accumulation whose top is the first two merged on the flat plane,
/// given the second pair's continuation, and the third block's far top
/// corners read `2s` off that plane: `VertexOnFace` in band.
///
/// Falling, not rising: a rising top moves the first judgement's merged
/// result past `vol(A ∪ B) ≤ vol(A) + vol(B)`
/// (`reachhold/a-settled-declared-coincidence-crosses-a-tight-volume-bound`),
/// before any step runs.
#[test]
fn a_fold_step_given_two_zeros_whose_ends_stand_in_band_refuses_at_a_vertex() {
    let s = -0.7 * band().zero();
    // A block over `x ∈ [x0, x0 + 1]`, `y ∈ [0, 1]`, its top through
    // `(x, top(x))`: the profile in the x–z plane, extruded along y.
    let block = |x0: f64, top: &dyn Fn(f64) -> f64| {
        let profile = [
            (x0, 0.0),
            (x0 + 1.0, 0.0),
            (x0 + 1.0, top(x0 + 1.0)),
            (x0, top(x0)),
        ];
        let mut body = crate::body::Body::<f64>::new();
        prism_ops(
            &mut body,
            &profile,
            (-1.0, 0.0),
            |u, v, w| Point3::new(u, -w, v),
            FaceGeometry::Certified,
            tol(),
        );
        describe_as_intersections(&mut body, tol());
        finished("a block", body, tol())
    };
    let falling = |x: f64| 1.0 + s * (x - 1.0);
    let blocks = [
        block(0.0, &|_| 1.0),
        block(1.0, &falling),
        block(2.0, &falling),
    ];
    let flat = Vec3::new(0.0, 0.0, 1.0);
    let sloped = Vec3::new(-s, 0.0, 1.0).normalize();
    let ball = 0.5 * 10f64.sqrt();
    assert!(
        crate::validate::decide(
            "bool_plane_reach",
            Margin::of(0.5 * s.abs() + Margin::levered((flat - sloped).norm(), ball).value()),
            band()
        )
        .is_err(),
        "the ends' tops stand in band of each other over the three blocks' ball"
    );
    let inputs: Vec<crate::MemberInputs> = (0..3)
        .map(|rank| crate::MemberInputs {
            rank,
            ..crate::MemberInputs::default()
        })
        .collect();
    let members: Vec<crate::NaryMember<'_, f64>> = blocks
        .iter()
        .zip(&inputs)
        .map(|(body, inputs)| crate::NaryMember { body, inputs })
        .collect();
    let run = crate::nary_boolean(
        crate::NaryOp::Union,
        &members,
        crate::SweepStrategy::Realized,
        tol(),
    );
    assert_eq!(
        run.judgements
            .iter()
            .map(|j| (j.a, j.b))
            .collect::<Vec<_>>(),
        vec![(0, 1), (1, 2)],
        "the neighbours alone are judged: {:?}",
        run.end
    );
    let crate::NaryEnd::Refused(crate::NaryRefusal::Step { member: 2, error }) = &run.end else {
        panic!("the second step refuses: {:?}", run.end);
    };
    let super::BooleanError::Escalated {
        decision:
            super::BooleanDecision::Coincidence(
                super::Coincide::VertexOnFace,
                super::DeclarationRead::Spent(GlueClass::Continuation),
            ),
        diag,
    } = **error
    else {
        panic!("a vertex on the merged top, in band: {error:?}");
    };
    assert_eq!(
        diag.predicate,
        Some("bool_vertex_face_side"),
        "the far corners' side of the flat top: {diag:?}"
    );
}
