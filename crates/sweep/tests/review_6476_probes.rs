//! Reviewer evidence for PR 6476 (spline faces onto ∖ and ∩): placements
//! the census does not hold.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use sweep::test_support::{finished, loft_prism};
use topo::{AtRestBody, BooleanError, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished(
        "brick",
        topo::test_support::brick_with_fresh_keys::<f64>(x, y, z, tol()),
        tol(),
    )
}

/// **A slab resting flush on the loft prism's top cap, wider than the
/// prism, meets other doors under ∩ and ∖ than under ∪.** ∪ reaches the
/// crossings path (the on-sectors Eq. 15.3 reclassifies per op) and
/// refuses at the spline face pair; ∩ and ∖ find no crossing and refuse
/// at the extent re-gate. Both refuse; the claim "each op meets the same
/// doors as ∪ and refuses where ∪ does" (`reduce::boolean_arm_exists`'s
/// docs, the census's module docs and its per-run assertion) does not
/// hold here, so the census would go red on this placement.
#[test]
fn a_flush_slab_on_the_prisms_cap_refuses_at_other_doors_under_subtract_and_intersect() {
    let t = tol();
    let prism = finished("prism", loft_prism(t), t);
    let slab = brick((-3.0, 3.0), (-3.0, 3.0), (2.0, 3.0));
    for (op, got) in [
        ("O∪B", topo::union(&prism, &slab, t)),
        ("B∪O", topo::union(&slab, &prism, t)),
    ] {
        assert!(
            matches!(got, Err(BooleanError::CurvedBooleanUnsupported { .. })),
            "{op}: {got:?}"
        );
    }
    for (op, got) in [
        ("O∩B", topo::intersect(&prism, &slab, t)),
        ("B∩O", topo::intersect(&slab, &prism, t)),
        ("O∖B", topo::subtract(&prism, &slab, t)),
        ("B∖O", topo::subtract(&slab, &prism, t)),
    ] {
        assert!(
            matches!(got, Err(BooleanError::NurbsExtentUnsupported { .. })),
            "{op}: {got:?}"
        );
    }
}

/// **A cavity whose spline walls carry the wrong `sense` passes the gate
/// and measures the true volume.** `around ∖ prism` builds a cavity
/// with sense-false NURBS walls; flipping every one back (what a
/// reversal that ignored `sense` on spline faces would build) is
/// admitted at rest and its volume reads the same 135. So neither the
/// at-rest gate nor the census's volume oracle sees a wrong `sense` on
/// a spline face; with `point_in_solid` refusing NURBS, the census's
/// point sample does not either.
#[test]
fn a_cavity_with_its_spline_walls_unreversed_passes_the_gate_at_the_true_volume() {
    let t = tol();
    let prism = finished("prism", loft_prism(t), t);
    let around = brick((-3.0, 3.0), (-3.0, 3.0), (-1.0, 3.0));
    let Ok(BooleanResult::Body(cav)) = topo::subtract(&around, &prism, t) else {
        panic!("around ∖ prism builds");
    };
    let v_right = topo::test_support::volume_at("right", &cav.body, t, 1e-6).volume;
    let cav: topo::Body<f64> = (*cav.body).clone();
    let nurbs: Vec<_> = cav
        .faces()
        .filter(|(_, f)| matches!(cav.get_surface(f.surface).unwrap(), geom::Surface::Nurbs(_)))
        .map(|(k, f)| (k, f.sense))
        .collect();
    assert_eq!(nurbs.len(), 4);
    assert!(
        nurbs.iter().all(|&(_, s)| !s),
        "the cavity's walls are sense-false"
    );
    let mut wrong = cav.clone();
    for &(k, _) in &nurbs {
        wrong = wrong.flipped_face_sense_for_tests(k).unwrap();
    }
    let at_rest = AtRestBody::validate(wrong, t);
    let gate = format!("{:?}", at_rest.as_ref().map(|_| ()));
    let v_wrong = at_rest
        .map(|b| topo::test_support::volume_at("wrong", &b, t, 1e-6).volume)
        .ok();
    println!("right V = {v_right}; wrong-sense gate {gate}, V = {v_wrong:?}");
    assert!((v_right - 135.0).abs() < 1e-6);
    // The defect this row records: the gate admits it, at the same volume.
    assert_eq!(gate, "Ok(())", "the gate admits the wrong-sense cavity");
    assert!((v_wrong.unwrap() - 135.0).abs() < 1e-6, "{v_wrong:?}");
}
