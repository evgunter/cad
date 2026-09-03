//! CERT-3 review lane R1 — e2e exercise through the public doors.
//!
//! My own revolve/restrict fixtures (not the unit's): oblique wide
//! axis, rotational placement, a stored-placement round trip through
//! nested `restrict`, and an angle near but not at zero — both lanes.
//! Local-only; never pushed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_3, TAU};

use geom_brep::MappedCurve;
use geom_core::{Affine3, Mat3, Point2, Point3, Vec3};

/// f64 lane: nested restrict composes placements twice; the sample must
/// agree with the direct evaluation to rounding, near zero angle too.
#[test]
fn r1_f64_nested_restrict_round_trip() {
    let place = Affine3::from_parts(
        Mat3::rotation_about(Vec3::new(1.0f64, 1.0, 0.0), FRAC_PI_3),
        Vec3::new(0.5, -1.5, 3.0),
    );
    let curve = MappedCurve::RevolvedPoint {
        point: Point2::new(2.0f64, -1.0),
        place,
        axis_origin: Point3::new(1.0, 2.0, 3.0),
        axis_dir: Vec3::new(2.0, 1.0, -2.0),
        angle: TAU,
    };
    // restrict twice: [0.3, 0.7] then [0.5, 1.0] of that = [0.5, 0.7].
    let r1 = curve.restrict(0.3, 0.7);
    let r2 = r1.restrict(0.5, 1.0);
    for s in [0.0, 0.25, 1.0] {
        let via = r2.eval(s);
        let direct = curve.eval(0.5 + 0.2 * s);
        let d = (via.x - direct.x)
            .abs()
            .max((via.y - direct.y).abs())
            .max((via.z - direct.z).abs());
        println!("f64 nested restrict s={s}: |via - direct| = {d:e}");
        assert!(d <= 1e-12, "nested restrict drifted {d:e} from direct eval");
    }
    // Near-zero angle at f64: eval(s)·small-angle stays near the start.
    let tiny = MappedCurve::RevolvedPoint {
        point: Point2::new(2.0f64, -1.0),
        place,
        axis_origin: Point3::new(1.0, 2.0, 3.0),
        axis_dir: Vec3::new(2.0, 1.0, -2.0),
        angle: 1.0e-8,
    };
    let p0 = tiny.eval(0.0);
    let p1 = tiny.eval(1.0);
    let d = (p1.x - p0.x)
        .abs()
        .max((p1.y - p0.y).abs())
        .max((p1.z - p0.z).abs());
    println!("f64 near-zero revolve: |eval(1) - eval(0)| = {d:e}");
    assert!(d <= 1.0e-7, "a 1e-8 revolve moved the point {d:e}");
    assert!(d > 0.0, "a 1e-8 revolve moved nothing at all — suspicious");
}

#[cfg(feature = "interval")]
mod interval_lane {
    use super::*;
    use geom_core::{Bounds, Interval, Real};

    fn iv(x: f64) -> Interval {
        Interval::from_f64(x)
    }

    fn w3(p: Point3<Interval>) -> f64 {
        let w = |e: Interval| e.hi() - e.lo();
        w(p.x).max(w(p.y)).max(w(p.z))
    }

    /// An oblique wide axis and a ROTATIONAL placement (the unit's
    /// fixture used a translation placement and a +z axis).
    fn rig(half: f64, angle: f64) -> MappedCurve<Interval> {
        let wd = |c: f64| Interval::from_bounds(c - half, c + half);
        MappedCurve::RevolvedPoint {
            point: Point2::new(iv(2.0), iv(-1.0)),
            place: Affine3::from_parts(
                Mat3::rotation_about(
                    Vec3::new(iv(1.0), iv(1.0), iv(0.0)),
                    iv(core::f64::consts::FRAC_PI_3),
                ),
                Vec3::new(iv(0.5), iv(-1.5), iv(3.0)),
            ),
            axis_origin: Point3::new(wd(1.0), wd(2.0), wd(3.0)),
            axis_dir: Vec3::new(wd(2.0), wd(1.0), wd(-2.0)),
            angle: iv(angle),
        }
    }

    /// Near-but-not-zero angle: the start sample must not feel the
    /// axis width, and the far sample only in proportion to the angle.
    #[test]
    fn r1_interval_near_zero_angle_consumer_view() {
        for half in [0.0, 1.0e-9] {
            let c = rig(half, 1.0e-8);
            let start = w3(c.eval(Interval::zero()));
            let end = w3(c.eval(Interval::one()));
            println!("half {half:e}: start {start:e}, end {end:e}");
            // Start: placement + rotation floor only — must be
            // independent of the axis width (compare across the loop).
            assert!(start <= 1.0e-13, "start sample width {start:e}");
            // End: the 1e-8 rotation may charge ~theta*(|q| + width)
            // but nothing like 2*width(axis).
            assert!(end <= 1.0e-13, "1e-8 revolve end width {end:e}");
        }
    }

    /// The stored-placement round trip: nested restricts, then compare
    /// the sub-curve's samples against the direct evaluation — the two
    /// enclosures must overlap (they describe one point), and the
    /// stored path must not balloon.
    #[test]
    fn r1_interval_stored_round_trip() {
        let c = rig(1.0e-9, TAU);
        let r1 = c.restrict(iv(0.3), iv(0.7));
        let r2 = r1.restrict(iv(0.5), iv(1.0));
        for s in [0.0f64, 0.5, 1.0] {
            let via = r2.eval(iv(s));
            let direct = c.eval(iv(0.5 + 0.2 * s));
            for (a, b, name) in [
                (via.x, direct.x, "x"),
                (via.y, direct.y, "y"),
                (via.z, direct.z, "z"),
            ] {
                assert!(
                    a.lo() <= b.hi() && b.lo() <= a.hi(),
                    "s={s}: {name} enclosures disjoint — [{:e},{:e}] vs [{:e},{:e}]",
                    a.lo(),
                    a.hi(),
                    b.lo(),
                    b.hi()
                );
            }
            let wv = w3(via);
            let wd = w3(direct);
            println!(
                "s={s}: via width {wv:e}, direct width {wd:e}, ratio {}",
                wv / wd
            );
        }
    }

    /// The caller/callee-split round trip the PR names as its sweep's
    /// blind spot, measured: restrict(s0 != 0) stores (I-R1)q, eval
    /// re-subtracts through (I-R2)q, and an R2·q pair cancels over the
    /// reals. Compare the split cost against the fused rotation.
    #[test]
    fn r1_split_anchor_cost_measured() {
        let c = rig(1.0e-9, TAU);
        let split = w3(c.restrict(iv(0.25), iv(0.5)).eval(Interval::zero()));
        let fused = w3(c.eval(iv(0.25)));
        println!(
            "quarter-turn: fused {fused:e}, split {split:e}, ratio {}",
            split / fused
        );
        // Both scale with angle*width(axis) ~ 1e-8..1e-9-scale; the
        // split may pay a small constant factor more, but if it paid a
        // constant floor or 2*width regardless of angle the ratio blows
        // up. Bound it loosely and report the measured numbers.
        assert!(
            split <= 8.0 * fused,
            "split anchor cost is {}x the fused cost",
            split / fused
        );
    }
}
