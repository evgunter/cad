//! Review probes for PR 4485 (not for merge).

use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;
use geom_core::spline::algebra::refine_plan_homogeneous;
use geom_core::spline::{KnotVector, TensorCoeffs};

/// `TensorCoeffs::refine` takes plan lists related to the pair by count
/// alone: plans built from ANOTHER vector of the same control count pass
/// the extent guard, and the answer pairs a net differenced with that
/// vector's Boehm steps against that vector's refinement.
#[test]
fn refine_accepts_plans_of_another_vector() {
    let ka = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kb = KnotVector::clamped(vec![0.0, 0.0, 0.0, 2.0, 2.0, 2.0], 2).unwrap();
    let line = [0.0, 0.0, 1.0]; // f(u) = u^2 on ka
    let t = TensorCoeffs::from_fn(&ka, &ka, |i, _| Interval::point(line[i]));
    let plans_b = refine_plan_homogeneous(&kb, &[1.0]).unwrap();
    let r = t.refine(&plans_b, &[]);
    // Not refused: every entry certified ...
    for i in 0..r.net().nu() {
        for j in 0..r.net().nv() {
            assert!(r.net().get(i, j).is_certified());
        }
    }
    // ... and the pair now claims kb's domain, which the source never had.
    assert_eq!(t.knots_u().domain(), (0.0, 1.0));
    assert_eq!(r.knots_u().domain(), (0.0, 2.0));
}
