//! **The collapsed conventional meter at the INTERVAL scalar**
//! (PCURVE P-1a fix pass).
//!
//! The meter is generic over `T`, and the fix pass changed what it
//! MEASURES on the conventional arm, not merely how the bits fall.
//! A quantity change in a generic meter deserves a row at more than
//! one scalar: an enclosure lane can widen where an `f64` lane is
//! exact, and the `sec α` re-baseline is a claim about the geometry
//! that should survive the widening.
//!
//! The file's NAME is load-bearing, and honestly so: `ci-filter.py`'s
//! `_forces_interval` pins the interval compile-mode lane whenever a
//! changed file's basename contains "interval", so adding genuine
//! interval-lane content here is what draws that lane on this PR
//! rather than leaving it to the sampler.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{EdgeCurve, EdgeCurveSpec, EdgeGeometry};
use geom_core::{Band, Bounds, Interval, Point3, Real, Vec3};

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

/// The rows' own band, fixed rather than the run's — a row about the
/// meter must not become a row about the matrix point it drew.
fn band() -> Band {
    Band::new(1.0e-9, 1.0e-8).expect("the row's own band")
}

fn table(
    surfs: Vec<Surface<Interval>>,
) -> (
    Vec<geom_brep::SurfaceKey>,
    impl Fn(geom_brep::SurfaceKey) -> Option<Surface<Interval>>,
) {
    let mut map: slotmap::SlotMap<geom_brep::SurfaceKey, Surface<Interval>> =
        slotmap::SlotMap::with_key();
    let keys: Vec<geom_brep::SurfaceKey> = surfs.into_iter().map(|s| map.insert(s)).collect();
    (keys, move |k| map.get(k).cloned())
}

/// A cylinder seam certifies at the interval scalar through the
/// collapsed meter — the arm the fix pass rewrote, on the enclosure
/// lane, at a drift well inside the band.
#[test]
fn a_cylinder_seam_certifies_through_the_collapsed_meter_at_intervals() {
    let r = 2.0;
    let d = 2.5e-10;
    let (keys, lookup) = table(vec![Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
        radius: iv(r),
        u_ref: Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
    }]);
    let carrier = Curve3::Line {
        origin: Point3::new(iv(r + d), iv(0.0), iv(0.0)),
        dir: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
    };
    let (t0, t1) = (iv(0.0), iv(3.0));
    let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));
    let certified = EdgeCurve::certify(
        EdgeCurveSpec {
            description: EdgeGeometry::Seam { surface: keys[0] },
            carrier,
            param_start: t0,
            param_end: t1,
        },
        p0,
        p1,
        &lookup,
        band(),
    )
    .expect("the seam certifies at the interval scalar");
    // The enclosure must CONTAIN the drift it was built from: an
    // interval meter that certified while excluding the true residual
    // would be unsound, and that is the property worth a row here.
    //
    // The comparand is the REPRESENTABLE drift `fl(r + d) − r`, not
    // the nominal `d`. The fixture's carrier is placed at the `f64`
    // `r + d`, so the geometry's true offset is that rounding's
    // result — about 2.1e-17 m above `d` here. Asserting against the
    // nominal value fails, and it fails for the right reason: the
    // enclosure is sound and the expectation was not. (It did fail,
    // the first time this row ran.)
    let representable = (r + d) - r;
    let m = certified.certificate().max_residual;
    assert!(
        m.lo() <= representable && representable <= m.hi(),
        "the certified residual enclosure {m:?} must contain the constructed drift \
         {representable:e}"
    );
}

/// The **cone re-baseline** survives the widening: the collapsed
/// meter's enclosure on a cone seam must contain `d·sec α`, the
/// radial chord, and must NOT sit at the perpendicular distance `d`
/// the pre-collapse arm metered. If an interval lane could still
/// certify at `d`, the `sec α` finding would be an `f64` artifact
/// rather than the geometry it is.
#[test]
fn the_cone_rebaseline_survives_the_interval_widening() {
    let alpha = core::f64::consts::FRAC_PI_6;
    let d = 2.5e-10;
    let (s, c) = alpha.sin_cos();
    let (keys, lookup) = table(vec![Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
        half_angle: iv(alpha),
        u_ref: Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
    }]);
    let carrier = Curve3::Line {
        origin: Point3::new(iv(c * d), iv(0.0), iv(-s * d)),
        dir: Vec3::new(iv(s), iv(0.0), iv(c)),
    };
    let (t0, t1) = (iv(0.5), iv(2.0));
    let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));
    let certified = EdgeCurve::certify(
        EdgeCurveSpec {
            description: EdgeGeometry::Seam { surface: keys[0] },
            carrier,
            param_start: t0,
            param_end: t1,
        },
        p0,
        p1,
        &lookup,
        band(),
    )
    .expect("a 2.5e-10 drift is inside the band even after the sec α re-baseline");
    let m = certified.certificate().max_residual;
    let chord = d / c;
    assert!(
        m.hi() >= chord * (1.0 - 1e-6),
        "the collapsed meter's enclosure {m:?} must reach the radial chord {chord:e} \
         — if it stops at the perpendicular distance {d:e}, the sec α re-baseline is \
         an f64 artifact and the disposition is wrong"
    );
}
