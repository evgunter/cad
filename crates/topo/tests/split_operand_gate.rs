//! **The split serves finished bodies only.** Its doors (`split`,
//! `split_reduce`, `plane_section`, `vertex_sides`) take an
//! `AtRestBody`, so an operand tier 3 refuses never reaches them at a
//! certifying scalar. At a dual, whose policy runs no at-rest gate, the
//! door reads what the type promises itself, per solid and before a
//! several-solid operand is read as one: tier 2, then check 7.
//!
//! The inside-out wedge is a prism over a clockwise profile, closed and
//! tier-2 clean with its faces pointing inward (volume −0.2349). Taken,
//! it came back as two inside-out halves of −0.1175 each, each the
//! complement of the half it bounds.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{Decide, Dual64, Interval, Point3, Tol, Vec3};
use topo::{
    AtRestBody, AtRestOutcome, AtRestPolicy, Body, SectionError, SolidKey, SplitError, SplitPart,
    SplitPlane, SplitReduceError, ValidationError, mass_properties, mass_properties_structural,
    plane_section, split, split_reduce, vertex_sides,
};

/// The triangle (0,0), 80°, 190° on the unit circle, counterclockwise
/// when `ccw`.
fn wedge_profile(ccw: bool) -> [(f64, f64); 3] {
    let at = |deg: f64| (deg.to_radians().cos(), deg.to_radians().sin());
    if ccw {
        [(0.0, 0.0), at(80.0), at(190.0)]
    } else {
        [(0.0, 0.0), at(190.0), at(80.0)]
    }
}

/// The wedge prism over z ∈ (0.5, 1).
fn wedge<T: Decide + AtRestPolicy>(ccw: bool) -> Body<T> {
    common::prism_z::<T>(&wedge_profile(ccw), 0.5, 1.0, Tol::witness()).body
}

/// The plane z = 0.75, normal +z: through the wedge's mid-height.
fn mid_height<T: Decide>() -> SplitPlane<T> {
    common::split_plane(
        Point3::new(T::from_f64(0.0), T::from_f64(0.0), T::from_f64(0.75)),
        Vec3::new(T::from_f64(0.0), T::from_f64(0.0), T::from_f64(1.0)),
        Tol::witness(),
    )
}

/// `body` through a dual's at-rest gate, which runs nothing: the
/// operand carries no verdict.
fn unverdicted(body: Body<Dual64>) -> AtRestBody<Dual64> {
    let kept = Dual64::gate_at_rest_kept(body, Tol::witness()).expect("a dual gate runs nothing");
    assert_eq!(kept.outcome(), AtRestOutcome::NotRunAtThisScalar);
    kept
}

/// Every split door's refusal of `operand`, by door name: the
/// reduction-stage refusal each carries.
fn every_door(operand: &AtRestBody<Dual64>) -> Vec<(&'static str, Option<SplitReduceError>)> {
    let tol = Tol::witness();
    let plane = mid_height::<Dual64>();
    vec![
        (
            "split",
            match split(operand, &plane, tol) {
                Err(SplitError::Reduce(e)) => Some(e),
                _ => None,
            },
        ),
        ("split_reduce", split_reduce(operand, &plane, tol).err()),
        (
            "plane_section",
            match plane_section(operand, &plane, tol) {
                Err(SectionError::Split(SplitError::Reduce(e))) => Some(e),
                _ => None,
            },
        ),
        ("vertex_sides", vertex_sides(operand, &plane, tol).err()),
    ]
}

/// **At a certifying scalar the inside-out wedge does not finish**: the
/// at-rest gate refuses it on check 7 alone, so no split door is handed
/// it.
#[test]
fn the_inside_out_wedge_does_not_finish_at_f64_or_interval() {
    let tol = Tol::witness();
    let volume = mass_properties(&wedge::<f64>(false), tol).unwrap().volume;
    assert!(
        (volume + 0.5 * 0.5 * 110f64.to_radians().sin()).abs() < 1e-12,
        "the clockwise wedge encloses minus its area times its height: {volume}"
    );
    for (scalar, errors) in [
        (
            "f64",
            f64::gate_at_rest_kept(wedge::<f64>(false), tol).map(|_| ()),
        ),
        (
            "Interval",
            Interval::gate_at_rest_kept(wedge::<Interval>(false), tol).map(|_| ()),
        ),
    ] {
        let errors = errors.expect_err("an inside-out body is not a finished body");
        assert!(
            errors.len() == 1 && matches!(errors[0], ValidationError::NegativeVolume { .. }),
            "{scalar}: the gate refuses the one solid inside-out, got {errors:?}"
        );
    }
}

/// **At a dual every split door reads check 7 itself** and refuses the
/// wedge as the inside-out operand it is, naming its solid. Red without
/// that read: `split` answered two halves of −0.1175 each.
#[test]
fn every_split_door_refuses_an_inside_out_operand_at_a_dual() {
    let operand = unverdicted(wedge::<Dual64>(false));
    let (solid, _) = operand.solids().next().expect("the wedge is one solid");
    for (door, refusal) in every_door(&operand) {
        match refusal {
            Some(SplitReduceError::InsideOutOperand { solid: named }) => {
                assert_eq!(named, solid, "{door}: the refusal names the wedge's solid");
            }
            other => panic!("{door}: want InsideOutOperand, got {other:?}"),
        }
    }
}

/// **A small inside-out part beside a larger ordinary one**, in one
/// body: its total volume is positive, so a reading of the merged solid
/// the split cuts sees nothing. The door reads check 7 per solid before
/// the merge, and names the wedge's solid. Red if it read the merge.
#[test]
fn an_inside_out_part_beside_an_ordinary_one_refuses_at_a_dual() {
    let tol = Tol::witness();
    let mut parts = Body::<Dual64>::new();
    for (profile, z) in [
        (
            [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)].to_vec(),
            (0.0, 2.0),
        ),
        (
            wedge_profile(false)
                .iter()
                .map(|&(x, y)| (x + 5.0, y))
                .collect(),
            (0.5, 1.0),
        ),
    ] {
        common::prism_ops(
            &mut parts,
            &profile,
            z,
            common::identity_map,
            common::FaceGeometry::Certified,
            tol,
        );
    }
    common::describe_as_intersections(&mut parts, tol);
    assert!(
        mass_properties_structural(&parts, tol)
            .unwrap()
            .volume
            .value
            > 0.0,
        "the body's total is positive, so it hides the part's sign"
    );
    let wedge_solid: Vec<SolidKey> = parts
        .solids()
        .map(|(k, _)| k)
        .filter(|&k| parts.faces_of_solid(k).map(|f| f.len()) == Some(5))
        .collect();
    assert_eq!(wedge_solid.len(), 1, "one solid is the five-faced wedge");
    let operand = unverdicted(parts);
    match split(&operand, &mid_height(), tol) {
        Err(SplitError::Reduce(SplitReduceError::InsideOutOperand { solid })) => {
            assert_eq!(solid, wedge_solid[0], "the refusal names the wedge's solid");
        }
        other => panic!("want InsideOutOperand, got {:?}", other.map(|_| ())),
    }
}

/// **A body carrying null edges refuses at every door with tier 2's
/// findings**: the reduction's own scratch body, handed back in at a
/// dual. The door's tier-2 read names each null edge, so no stage past
/// it reads one.
#[test]
fn a_reduced_body_refuses_at_every_door_with_its_null_edges_at_a_dual() {
    let tol = Tol::witness();
    let red = split_reduce(
        &unverdicted(wedge::<Dual64>(true)),
        &mid_height::<Dual64>(),
        tol,
    )
    .expect("the outward wedge reduces");
    assert!(!red.null_edges.is_empty(), "the reduction mints null edges");
    let operand = unverdicted(red.body);
    for (door, refusal) in every_door(&operand) {
        match refusal {
            Some(SplitReduceError::ScaffoldingOperand { errors }) => assert!(
                !errors.is_empty()
                    && errors
                        .iter()
                        .all(|e| matches!(e, ValidationError::NullEdgeAtRest { .. })),
                "{door}: tier 2's findings are the null edges, got {errors:?}"
            ),
            other => panic!("{door}: want ScaffoldingOperand, got {other:?}"),
        }
    }
}

/// **The counterclockwise control splits into two outward halves**, each
/// half the wedge's volume, finished at `f64` and unverdicted at a dual.
#[test]
fn the_outward_wedge_splits_into_two_positive_halves() {
    let tol = Tol::witness();
    let half = 0.25 * 0.5 * 110f64.to_radians().sin();
    let halves = |r: topo::SplitResult<f64>| {
        [r.above, r.below].map(|part| match part {
            SplitPart::Body(b) => mass_properties(&b, tol).unwrap().volume,
            SplitPart::Empty => panic!("the mid-height plane leaves material on both sides"),
        })
    };
    let operand = common::finished("the outward wedge", wedge::<f64>(true), tol);
    for v in halves(split(&operand, &mid_height(), tol).expect("the outward wedge splits")) {
        assert!(
            (v - half).abs() < 1e-12,
            "f64: half volume {v}, want {half}"
        );
    }
    let at_dual = split(&unverdicted(wedge::<Dual64>(true)), &mid_height(), tol)
        .expect("the outward wedge splits at a dual");
    for part in [at_dual.above, at_dual.below] {
        let SplitPart::Body(b) = part else {
            panic!("the mid-height plane leaves material on both sides");
        };
        let v = mass_properties_structural(&b, tol).unwrap().volume.value;
        assert!(
            (v - half).abs() < 1e-12,
            "dual: half volume {v}, want {half}"
        );
    }
}
