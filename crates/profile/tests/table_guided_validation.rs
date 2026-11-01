//! **A table validated guided by its own `f64` record**
//! (`Profile::validate_guided`, the loft's door for sections held in a
//! lane scalar): at `f64` it is the plain validation, and at `Dual64`
//! its value channel is the `f64` canonical form while the arcs keep
//! their derivative.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Dual64, Point2, Real, Tol};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop, SketchPlane};

/// A square, its right side an arc of bulge `b`, written clockwise so
/// that validation reverses it.
fn square<T: Real>(b: T) -> Vec<ProfileLoop<T>> {
    let p = |x: f64, y: f64| Point2::new(T::from_f64(x), T::from_f64(y));
    vec![bulge_loop(vec![
        (p(-1.0, 1.0), T::zero()),
        (p(1.0, 1.0), b),
        (p(1.0, -1.0), T::zero()),
        (p(-1.0, -1.0), T::zero()),
    ])]
}

#[test]
fn a_tables_guided_validation_is_its_plain_validation_at_f64() {
    let tol = Tol::witness();
    let profile = Profile::new(SketchPlane::xy(), square(0.2));
    let plain = profile.validate(tol).unwrap();
    let (recorded, record) = profile.validate_recording(tol).unwrap();
    let guided = profile.validate_guided(tol, &record).unwrap();
    assert_eq!(format!("{guided:?}"), format!("{plain:?}"));
    assert_eq!(format!("{recorded:?}"), format!("{plain:?}"));
}

#[test]
fn a_tables_guided_validation_at_dual_keeps_the_record_and_the_derivative() {
    let tol = Tol::witness();
    let (nominal, record) = Profile::new(SketchPlane::xy(), square(0.2))
        .validate_recording(tol)
        .unwrap();
    let guided = Profile::new(SketchPlane::<Dual64>::xy(), square(Dual64::variable(0.2)))
        .validate_guided(tol, &record)
        .unwrap();
    let values = |v: Vec<f64>| v.into_iter().map(f64::to_bits).collect::<Vec<_>>();
    let starts = |segs: &[profile::ValidatedSegment<f64>]| -> Vec<f64> {
        segs.iter().flat_map(|s| [s.start.x, s.start.y]).collect()
    };
    let lane: Vec<f64> = guided.loops()[0]
        .segments()
        .iter()
        .flat_map(|s| [s.start.x.value, s.start.y.value])
        .collect();
    assert_eq!(values(lane), values(starts(nominal.loops()[0].segments())));
    let sweeps: Vec<f64> = guided.loops()[0]
        .segments()
        .iter()
        .filter_map(|s| match s.kind {
            profile::SegmentKind::Arc { arc, .. } => Some(arc.sweep.deriv),
            profile::SegmentKind::Line => None,
        })
        .collect();
    assert_eq!(sweeps.len(), 1, "one arc");
    assert!(
        sweeps[0] != 0.0,
        "the arc's sweep carries the bulge's derivative"
    );
}
