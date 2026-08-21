//! M5 PR 11 Interval enclosure row: the certified quadrature lane at
//! the interval scalar — the tiltedcut halves' brackets
//! (`volume ± volume_pad` around the Interval value's own enclosure)
//! must contain the closed form πr²H/2, and tier 3 passes.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Bounds, Interval, Point2, Point3, Tolerance, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{Body, validate_geometric};
use geom_core::Tol;

const R: f64 = 0.5;
const H: f64 = 1.0;
const PHI: f64 = 0.3;

fn i(x: f64) -> Interval {
    geom_core::Real::from_f64(x)
}

fn halves() -> (Body<Interval>, Body<Interval>) {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(i(-R), i(0.0)), i(1.0)),
        ProfileVertex::new(Point2::new(i(R), i(0.0)), i(1.0)),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let cylinder = extrude(&profile, Extrusion::Distance(i(H))).unwrap().body;
    let plane = SplitPlane {
        origin: Point3::new(i(0.0), i(0.0), i(H / 2.0)),
        normal: Vec3::new(i(PHI.sin()), i(0.0), i(PHI.cos())),
    };
    let result = split(&cylinder, &plane).unwrap();
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("both sides carry material");
    };
    (above.clone(), below.clone())
}

/// The Interval-lane certified bracket contains πr²H/2 and tier 3
/// passes end to end (check 7 consumes the bounds at Interval too).
#[test]
fn interval_lane_quadrature_brackets_the_closed_form() {
    let (above, below) = halves();
    let half_exact = core::f64::consts::PI * R * R * H / 2.0;
    for (label, body) in [("above", &above), ("below", &below)] {
        let m = topo::mass_properties(body)
            .unwrap_or_else(|e| panic!("{label}: interval quadrature computes: {e:?}"));
        let (lo, hi) = (m.volume.lo() - m.volume_pad, m.volume.hi() + m.volume_pad);
        assert!(
            lo <= half_exact && half_exact <= hi,
            "{label}: interval bracket [{lo}, {hi}] must contain {half_exact}"
        );
        assert!(
            hi - lo < 1e-3 * half_exact,
            "{label}: bracket width {} is not useful",
            hi - lo
        );
        if let Err(errs) = validate_geometric(body) {
            panic!("{label}: tier 3 at Interval: {errs:?}");
        }
    }
}
