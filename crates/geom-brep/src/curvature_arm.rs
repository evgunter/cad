//! **The curvature arms' home**: a lever read off a carrier's own
//! curvature (`crates/geom-core/README.md`, "Float rounding": a
//! verdict's lever arm is a read of the consumed extent, a curvature arm
//! `1/κ_rel`, or the session box). Each reader here takes the carrier it
//! reads, so a call site hands geometry and never a radius it computed.
//! `geom_core::Lever`'s bare curvature doors are referenced in this file
//! alone, which `scripts/gates/lever-doors.sh` holds.

use geom::{Circle, Curve3, Surface};
use geom_core::spline::SpanLocate;
use geom_core::{Lever, Point3, Real, poison};

/// The local curvature lever arm of `s` at `p` (`implicit`'s module
/// docs): the chart's own length scale, `f64::MAX` for a plane (the practical
/// `min` identity — see those docs for why not `+∞`), poison for
/// [`Surface::Nurbs`].
///
/// Per kind: sphere/cylinder — the radius; cone — the radial distance
/// ρ of `p` from the axis; torus — the tube radius `r`. It is the scale
/// the implicit forms are normalized by (`|∇F| − 1` is the elevation
/// over it for all four), which is how its readers use it: to meter a
/// dimensionless residual or a relative curvature as a length at the
/// chart's own scale (`face_normal`'s on-chart certificate, the
/// dihedral fold, the contact and SSI jet checks). Each value is a
/// principal radius of curvature at `p` (the torus's along its
/// meridian), so it is minted as a curvature arm
/// ([`Lever::curvature_radius`]).
///
/// **It is NOT a bound on the curvature on a fat torus.** On a ring
/// with `R < 2r` the inner equator bends at `1/(R − r)`, harder than
/// the tube's `1/r`. A reader that charges a SAGITTA against the
/// tightest bend wants [`crate::min_radius_of_curvature`] instead, and the two
/// must not be swapped: a smaller value here LOOSENS the readers above
/// (their margins shrink toward Zero), while a smaller radius of
/// curvature tightens a sagitta charge.
pub fn curvature_lever_arm<T: Real>(s: &Surface<T>, p: Point3<T>) -> Lever<T> {
    match *s {
        Surface::Cone { apex, axis, .. } => {
            let (_, w) = crate::implicit::axial_radial(p, apex, axis);
            Lever::curvature_radius(w.norm())
        }
        _ => uniform_curvature_arm(s),
    }
}

/// **A circle's curvature arm**: its radius, the arm of a turn of its
/// plane or along it.
#[must_use]
pub fn circle_curvature_arm<T: Real>(circle: &Circle<T>) -> Lever<T> {
    Lever::curvature_radius(circle.radius)
}

/// **`arm`, no longer than `carrier`'s radius of curvature at `t`**:
/// the lesser of the two, `κ = |C′ × C″| / |C′|³` read off the carrier's
/// own derivatives there. A straight carrier (`κ = 0`) reads `arm`.
#[must_use]
pub fn within_carrier_bend<T: SpanLocate>(arm: Lever<T>, carrier: &Curve3<T>, t: T) -> Lever<T> {
    let (d1, d2) = (carrier.deriv(t), carrier.deriv2(t));
    arm.within_curvature(d1.cross(d2).norm() / d1.norm().powi(3))
}

/// **A surface's curvature arm where it is the same at every point**
/// ([`curvature_lever_arm`] with no point to read at): a sphere's or a
/// cylinder's radius, a torus's or a spindle's tube radius, `f64::MAX`
/// for a plane. Poison for a cone, whose arm is its parallel's radius
/// through the point read, and for a spline.
#[must_use]
pub fn uniform_curvature_arm<T: Real>(s: &Surface<T>) -> Lever<T> {
    Lever::curvature_radius(match *s {
        Surface::Plane { .. } => T::from_f64(f64::MAX),
        Surface::Sphere { radius, .. } | Surface::Cylinder { radius, .. } => radius,
        Surface::Torus { minor_radius, .. } | Surface::Spindle { minor_radius, .. } => minor_radius,
        // A spline has no implicit form, and its stand-in no closed arm
        // to lend.
        Surface::Cone { .. } | Surface::Nurbs(_) | Surface::Approx(_) => poison(),
    })
}

/// **The radius of `s`'s parallel through `p`**: `p`'s distance from the
/// surface's own axis ([`Surface::axis_line`]), the arm of a turn about
/// that axis at `p`. On a cone it is [`curvature_lever_arm`]; on a torus
/// it is the scale its azimuthal sections bend at, which the tube radius
/// does not bound. Poison for a surface with no axis.
#[must_use]
pub fn parallel_curvature_arm<T: Real>(s: &Surface<T>, p: Point3<T>) -> Lever<T> {
    Lever::curvature_radius(s.axis_line().map_or_else(poison, |axis| axis.distance(p)))
}
