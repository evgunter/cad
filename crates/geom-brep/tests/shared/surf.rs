//! The elementary surfaces the suites of this crate build in ONE
//! frame — centred at the origin, about `+z`, with `u_ref = +x` — and
//! the slotmap arena that hands them to a door as keys.
//!
//! The frame is the whole content of these three constructors: every
//! copy they replace passed the same centre, the same axis and the same
//! `u_ref`, and differed only in the radii, which stay the caller's
//! (they are each suite's own fixture scale).
//!
//! **What this module does NOT absorb.** A surface in a DIFFERENT frame
//! stays where it is used and reads as its own fixture:
//! `review_m6_3_chart_probes.rs`'s sphere at `(0.3, -0.2, 1.1)`,
//! `pcurve_conic.rs`'s cylinder at `(1, 2, 3)` and
//! `offset_mint.rs`'s `cyl`/`torus`, which are placed on that suite's
//! own tilted `t_center()`/`t_axis()`/`t_uref()` frame. Nor does it
//! hold the NURBS carriers — those are `shared::fixture`.

use geom::Surface;
use geom_brep::SurfaceKey;
use geom_core::Real;

use crate::shared::point::{p3, v3};

/// The sphere of radius `radius` centred at the origin, polar axis
/// `+z`, `u_ref = +x`.
pub(crate) fn sphere<T: Real>(radius: f64) -> Surface<T> {
    Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: T::from_f64(radius),
        axis: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The cylinder of radius `radius` about `+z` through the origin,
/// `u_ref = +x`.
pub(crate) fn cylinder<T: Real>(radius: f64) -> Surface<T> {
    Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: T::from_f64(radius),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The torus with the given major and minor radii, centred at the
/// origin about `+z`, `u_ref = +x`.
pub(crate) fn torus<T: Real>(major: f64, minor: f64) -> Surface<T> {
    Surface::Torus {
        center: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        major_radius: T::from_f64(major),
        minor_radius: T::from_f64(minor),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// A surface table and its resolver: the keys, in the order the
/// surfaces were given, and the injected lookup the `geom-brep` doors
/// take (keys never resolve inside `geom-brep`).
pub(crate) fn table<T: Real>(
    surfs: Vec<Surface<T>>,
) -> (Vec<SurfaceKey>, impl Fn(SurfaceKey) -> Option<Surface<T>>) {
    let mut map: slotmap::SlotMap<SurfaceKey, Surface<T>> = slotmap::SlotMap::with_key();
    let keys: Vec<SurfaceKey> = surfs.into_iter().map(|s| map.insert(s)).collect();
    (keys, move |k| map.get(k).cloned())
}
