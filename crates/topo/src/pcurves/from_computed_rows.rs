#![allow(clippy::unwrap_used, clippy::expect_used)]
use std::sync::Arc;

use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Real};

use super::{computed_foot, derive_chart_foot};
use crate::revalue_pin::{Revalued, S, revalued};

/// A chart foot enters as computed values: the fitted lane's foot of
/// a point the parameter moves, on a flat spline chart at
/// `Sym<f64>`, re-values to neither coordinate `p0` measured.
#[test]
fn a_chart_foot_enters_as_computed() {
    let found = revalued([0.25, 0.25 + 1.0e-3], |_, x| {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
            .map(|(u, v)| Point3::new(u, v, 0.0).map(S::from_f64))
            .to_vec();
        let chart = Surface::Nurbs(Arc::new(
            NurbsSurface::new(
                geom_core::spline::Knots::from(kv.clone()).lift(geom_core::Real::from_f64),
                geom_core::spline::Knots::from(kv).lift(geom_core::Real::from_f64),
                control,
                vec![geom_core::Real::one(); 4],
            )
            .unwrap(),
        ));
        let point = Point3::new(x, S::from_f64(0.5) * x, S::zero());
        let foot = derive_chart_foot(point, &chart)
            .unwrap()
            .expect("a spline chart at a certifying scalar measures a foot");
        let foot = computed_foot::<S>(foot);
        vec![foot.x, foot.y]
    });
    assert_eq!(
        found,
        Revalued {
            kept: 0,
            opaque: 2,
            laundered: 0,
        }
    );
}
