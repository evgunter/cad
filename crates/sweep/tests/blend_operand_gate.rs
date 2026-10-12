//! **The blend doors serve finished bodies only.** `fillet_edges` and
//! `chamfer_edges` take an `AtRestBody`, so an operand tier 3 refuses
//! never reaches them at a certifying scalar
//! (`topo`'s `split_operand_gate` pins that the inside-out wedge does
//! not finish at `f64` or `Interval`). At a dual, whose policy runs no
//! at-rest gate, each door reads orientation itself.
//!
//! The inside-out wedge is a prism over a clockwise profile, closed and
//! tier-2 clean with its faces pointing inward (volume −0.2349). Taken
//! whole at 0.02, the fillet came back `Ok` at −0.23355 and the chamfer
//! at −0.23331: each answered for the complement and shipped a body
//! tier 3 refuses.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Decide, Dual64, Tol};
use sweep::blend::{BlendError, BlendRefusal, Blended, chamfer_edges, fillet_edges};
use sweep::test_support::at_rest;
use topo::{
    AtRestBody, AtRestOutcome, AtRestPolicy, Body, ValidationError, mass_properties,
    mass_properties_structural, test_support::prism_z, validate_geometric,
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
    prism_z::<T>(&wedge_profile(ccw), 0.5, 1.0, Tol::witness()).body
}

/// The blend size every row requests.
const SIZE: f64 = 0.02;

/// Both doors over every edge of `operand`, by verb name.
#[allow(clippy::type_complexity)]
fn both_doors<T: Decide + geom_core::Bounds + AtRestPolicy>(
    operand: &AtRestBody<T>,
) -> [(&'static str, Result<Blended<T>, BlendRefusal>); 2] {
    let edges: Vec<_> = operand.edges().map(|(k, _)| k).collect();
    assert_eq!(edges.len(), 9, "a triangular prism has nine edges");
    let tol = Tol::witness();
    [
        (
            "fillet",
            fillet_edges(operand, &edges, T::from_f64(SIZE), tol),
        ),
        (
            "chamfer",
            chamfer_edges(operand, &edges, T::from_f64(SIZE), tol),
        ),
    ]
}

/// **At a dual both doors read check 7 themselves** and refuse the
/// clockwise wedge as the inside-out operand it is, naming its solid.
/// Red without that read: both returned `Ok` with negative volume.
#[test]
fn both_blend_doors_refuse_an_inside_out_operand_at_a_dual() {
    let operand = Dual64::gate_at_rest_kept(wedge::<Dual64>(false), Tol::witness())
        .expect("a dual gate runs nothing");
    assert_eq!(operand.outcome(), AtRestOutcome::NotRunAtThisScalar);
    let (solid, _) = operand.solids().next().expect("the wedge is one solid");
    for (verb, result) in both_doors(&operand) {
        match result.map_err(|r| r.error) {
            Err(BlendError::InsideOutOperand { errors }) => assert_eq!(
                errors,
                vec![ValidationError::NegativeVolume { solid }],
                "{verb}: the refusal names the wedge's solid"
            ),
            other => panic!(
                "{verb}: want InsideOutOperand on check 7, got {:?}",
                other.map(|b| b.body.solids().count())
            ),
        }
    }
}

/// **The counterclockwise wedge blends outward at both doors and both
/// scalars** — the control, and the refusal's recourse ("build it with
/// its faces pointing outward") followed. At `f64` the result is
/// finished and encloses the clockwise wedge's measured volumes with
/// their sign turned (fillet +0.23355, chamfer +0.23331), within half a
/// unit of those readings' fifth decimal; at the dual the door's own
/// read passes it, and the blended volume's value channel agrees with
/// `f64`'s.
#[test]
fn the_counterclockwise_wedge_blends_outward_at_both_doors() {
    let tol = Tol::witness();
    let measured = |verb: &str| match verb {
        "fillet" => 0.23355,
        "chamfer" => 0.23331,
        other => panic!("no measured volume for {other}"),
    };
    let at_f64 = both_doors(&at_rest(&wedge::<f64>(true), tol));
    let dual =
        Dual64::gate_at_rest_kept(wedge::<Dual64>(true), tol).expect("a dual gate runs nothing");
    let at_dual = both_doors(&dual);
    for ((verb, f), (_, d)) in at_f64.into_iter().zip(at_dual) {
        let f = f.unwrap_or_else(|r| panic!("{verb} at f64: {r}")).body;
        let d = d.unwrap_or_else(|r| panic!("{verb} at the dual: {r}")).body;
        validate_geometric(&f, tol)
            .unwrap_or_else(|e| panic!("{verb}: the result is finished: {e:?}"));
        let volume = mass_properties(&f, tol).unwrap().volume;
        assert!(
            (volume - measured(verb)).abs() <= 5e-6,
            "{verb}: the blended wedge encloses {}, got {volume}",
            measured(verb)
        );
        let dual_volume = mass_properties_structural(&d, tol).unwrap().volume.value;
        assert!(
            (dual_volume - volume).abs() < 1e-9,
            "{verb}: the dual's value channel {dual_volume} is f64's {volume}"
        );
    }
}
