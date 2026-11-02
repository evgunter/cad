//! **The split serves finished bodies only.** Its doors (`split`,
//! `split_reduce`, `plane_section`, `vertex_sides`) take the scalar's
//! finished body, so an operand its finish refuses never reaches them:
//! at a certifying scalar tier 3 refuses an inside-out operand (check 7
//! per solid, check 10 per shell); at a dual the finish is tier 2, which
//! refuses scaffolding.
//!
//! The inside-out wedge is a prism over a clockwise profile, closed and
//! tier-2 clean with its faces pointing inward (volume −0.2349). Taken,
//! it came back as two inside-out halves of −0.1175 each, each the
//! complement of the half it bounds.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{Decide, Dual64, Interval, Point3, Tol, Vec3};
use topo::{
    AtRestPolicy, Body, Replay, ShellKey, SolidKey, SplitPart, SplitPlane, SplitReduceError,
    ValidationError, classify_neighborhood, mass_properties, mass_properties_structural, split,
    split_reduce,
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

/// `body` finished at a dual: tier 2 alone.
fn replay(body: Body<Dual64>) -> Replay<Dual64> {
    topo::finish(body, Tol::witness()).expect("tier 2 passes")
}

/// **At a certifying scalar the inside-out wedge does not finish**: the
/// at-rest gate refuses it on check 7 alone, so no split door is handed
/// it.
#[test]
fn the_inside_out_wedge_does_not_finish_at_f64_or_interval() {
    let tol = Tol::witness();
    // Raw door: the measured body is an inside-out body, which tier 3 refuses (NegativeVolume).
    let volume = topo::test_support::measure_unvalidated(
        &wedge::<f64>(false),
        tol,
        topo::test_support::pinned_resolution(tol),
    )
    .unwrap()
    .met()
    .expect("the pinned resolution is met")
    .volume;
    assert!(
        (volume + 0.5 * 0.5 * 110f64.to_radians().sin()).abs() < 1e-12,
        "the clockwise wedge encloses minus its area times its height: {volume}"
    );
    for (scalar, errors) in [
        ("f64", topo::finish(wedge::<f64>(false), tol).map(|_| ())),
        (
            "Interval",
            topo::finish(wedge::<Interval>(false), tol).map(|_| ()),
        ),
    ] {
        let errors = errors.expect_err("an inside-out body is not a finished body");
        assert!(
            errors.len() == 1 && matches!(errors[0], ValidationError::NegativeVolume { .. }),
            "{scalar}: the gate refuses the one solid inside-out, got {errors:?}"
        );
    }
}

/// An outward 2×2×2 brick and the inside-out wedge beside it, as two
/// solids, every face described: the body's total is positive.
fn brick_beside_inside_out_wedge() -> Body<f64> {
    let tol = Tol::witness();
    let mut parts = Body::<f64>::new();
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
        topo::test_support::measure_unvalidated(
            &parts,
            tol,
            topo::test_support::pinned_resolution(tol)
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume
            > 0.0,
        "the body's total is positive, so it hides the part's sign"
    );
    parts
}

/// **A small inside-out part beside a larger ordinary one**, in one
/// body: its total volume is positive, so a reading of the merged solid
/// the split cuts would see nothing. The finish reads check 7 per solid
/// and names the wedge's solid. (The dual row this replaces read the
/// same check at the split's door; a dual's finish is tier 2, so the row
/// lives at the base scalar.)
#[test]
fn an_inside_out_part_beside_an_ordinary_one_does_not_finish() {
    let tol = Tol::witness();
    let parts = brick_beside_inside_out_wedge();
    let wedge_solid: Vec<SolidKey> = parts
        .solids()
        .map(|(k, _)| k)
        .filter(|&k| parts.faces_of_solid(k).map(|f| f.len()) == Some(5))
        .collect();
    assert_eq!(wedge_solid.len(), 1, "one solid is the five-faced wedge");
    let errors = topo::finish(parts, tol).expect_err("an inside-out part is not finished");
    assert!(
        errors.contains(&ValidationError::NegativeVolume {
            solid: wedge_solid[0]
        }),
        "the refusal names the wedge's solid: {errors:?}"
    );
}

/// **The same two parts as two shells of ONE solid**: the solid's total
/// is positive too, so check 7 passes it, and the wedge's shell is a
/// `Void` standing outside the brick's `Outer`, bounding winding −1.
/// Tier 3 refuses it on check 10 (`ShellWinding`), naming the wedge's
/// shell. Red without that read: `split` at z = 0.75 answered two sides,
/// each carrying the wedge's inside-out half as a stray void.
#[test]
fn an_inside_out_shell_under_one_solid_does_not_finish_at_f64() {
    let tol = Tol::witness();
    let one_solid = brick_beside_inside_out_wedge().with_solids_merged_for_tests();
    let solids: Vec<SolidKey> = one_solid.solids().map(|(k, _)| k).collect();
    assert_eq!(solids.len(), 1, "the merge leaves one solid");
    let wedge_shell: Vec<ShellKey> = one_solid
        .shells_of_solid(solids[0])
        .unwrap()
        .iter()
        .copied()
        .filter(|&sh| one_solid.get_shell(sh).map(|s| s.faces.len()) == Some(5))
        .collect();
    assert_eq!(wedge_shell.len(), 1, "one shell is the five-faced wedge");
    let errors = topo::finish(one_solid, tol)
        .map(|_| ())
        .expect_err("an inside-out shell is not a finished body");
    assert!(
        matches!(
            &errors[..],
            [ValidationError::ShellWinding { solid, shell, winding: 0, bounded: -1 }]
                if *solid == solids[0] && *shell == wedge_shell[0]
        ),
        "the gate refuses the wedge's shell on check 10, got {errors:?}"
    );
}

/// **A body carrying null edges does not finish at a dual, on tier 2's
/// findings**: the reduction's own scratch body, finished again. The
/// dual's finish is tier 2, which names each null edge.
#[test]
fn a_reduced_body_does_not_finish_at_a_dual_on_its_null_edges() {
    let tol = Tol::witness();
    let red = split_reduce(&replay(wedge::<Dual64>(true)), &mid_height::<Dual64>(), tol)
        .expect("the outward wedge reduces");
    assert!(!red.null_edges.is_empty(), "the reduction mints null edges");
    let errors = topo::finish(red.body, tol).expect_err("null edges are tier 2's scaffolding");
    assert!(
        errors
            .iter()
            .all(|e| matches!(e, ValidationError::NullEdgeAtRest { .. })),
        "tier 2's findings are the null edges, got {errors:?}"
    );
}

/// **The public neighbourhood read refuses a null edge by name**: over
/// the reduction's scratch body, an ON vertex the reduction minted a
/// null edge at reads [`SplitReduceError::NullEdgeAtVertex`], naming the
/// vertex and one of the reduction's null edges.
#[test]
fn the_neighbourhood_read_names_a_null_edge_at_its_vertex() {
    let tol = Tol::witness();
    let red = split_reduce(&replay(wedge::<Dual64>(true)), &mid_height::<Dual64>(), tol)
        .expect("the outward wedge reduces");
    let record = red
        .null_edges
        .first()
        .expect("the reduction mints null edges");
    let band = geom_core::Band::linear(tol).unwrap();
    match classify_neighborhood(&red.body, &red.plane, &red.sides, record.at_vertex, band) {
        Err(SplitReduceError::NullEdgeAtVertex { vertex, edge }) => {
            assert_eq!(
                vertex, record.at_vertex,
                "the refusal names the vertex asked about"
            );
            assert!(
                red.null_edges.iter().any(|r| r.edge == edge),
                "the refusal names one of the reduction's null edges, got {edge:?}"
            );
        }
        other => panic!("want NullEdgeAtVertex, got {:?}", other.map(|_| ())),
    }
}

/// **The counterclockwise control splits into two outward halves**, each
/// half the wedge's volume, at `f64` and at a dual.
#[test]
fn the_outward_wedge_splits_into_two_positive_halves() {
    let tol = Tol::witness();
    let half = 0.25 * 0.5 * 110f64.to_radians().sin();
    let halves = |r: topo::SplitResult<f64>| {
        [r.above, r.below].map(|part| match part {
            SplitPart::Body(b) => {
                mass_properties(&b, tol, topo::test_support::pinned_resolution(tol))
                    .unwrap()
                    .met()
                    .expect("the pinned resolution is met")
                    .volume
            }
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
    let at_dual = split(&replay(wedge::<Dual64>(true)), &mid_height(), tol)
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
