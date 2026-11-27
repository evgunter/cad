//! **The pole form, the wrap tip**: how a face meets a chart singularity
//! of its surface, a sphere's pole or a cone's apex.
//!
//! A chart singularity in a face's closure is a vertex of it. Where no
//! edge a construction or a file states reaches it, it is the tip of
//! one **wrap edge**: an edge on the surface's own meridian, which the
//! face's loop walks out to the singular point and back (DESIGN
//! `[topology]`'s seam clause). A full revolve's wall seam already is
//! that edge (`sweep`'s `revolve::full::build_wire`); `step-import`
//! mints it where a file states the point some other way (its
//! edge-free sphere and its `normalize` pass).
//!
//! This module is the description those doors share: the meridian
//! carriers ([`sphere_meridian`], [`cone_generator`]). It says what the
//! wrap edge's carrier is, not where a door places it round the axis.

use geom_core::{Point3, Real, RigidMotion, Vec3};

use crate::Curve3;

/// The sphere's meridian at azimuth `d` (a unit direction perpendicular
/// to `axis`): the circle through both poles with normal `axis × d` and
/// reference direction `axis`, so `θ = 0` is the pole `center +
/// axis·radius`, `θ = π/2` the equator point `center + d·radius`, and
/// `θ = π` the other pole.
pub fn sphere_meridian<T: Real>(
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    d: Vec3<T>,
) -> Curve3<T> {
    Curve3::circle(center, axis.cross(d), radius, axis)
}

/// The cone's generator in `frame`'s `w`–`u` plane: the line through the
/// apex `frame.origin()` whose parameter is the surface's own slant `v`,
/// `apex + (w·cos α + u·sin α)·v` ([`RigidMotion::swing`]), so `v > 0`
/// runs on the nappe opening along the axis `w` and `v < 0` on the
/// other, at the azimuth `u`. A caller holding an axis and an azimuth
/// squares the azimuth about the axis first
/// ([`RigidMotion::squared_about`]).
pub fn cone_generator<T: Real>(frame: RigidMotion<T>, half_angle: T) -> Curve3<T> {
    let (s, c) = half_angle.sin_cos();
    Curve3::line(frame.origin(), frame.swing(c, s))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    use crate::Surface;

    /// Each carrier runs on its surface through the singular point, at
    /// the azimuth it names.
    #[test]
    fn the_meridian_carriers_run_through_the_singularity_at_their_azimuth() {
        let (center, radius) = (Point3::new(1.0, -2.0, 0.5), 3.0);
        let axis = Vec3::new(0.0, 0.6, 0.8);
        let u_ref = Vec3::new(1.0, 0.0, 0.0);
        let sphere = Surface::Sphere {
            frame: crate::Frame::new(center, axis, u_ref),
            radius,
        };
        for (u, d) in [(0.0, u_ref), (core::f64::consts::PI, -u_ref)] {
            let m = sphere_meridian(center, radius, axis, d);
            assert!(
                (m.eval(0.0) - (center + axis * radius)).norm() < 1e-15,
                "{u}: θ = 0"
            );
            let at = m.eval(core::f64::consts::FRAC_PI_2);
            assert!((at - (center + d * radius)).norm() < 1e-14, "{u}: θ = π/2");
            assert!(
                (m.eval(core::f64::consts::PI) - (center - axis * radius)).norm() < 1e-14,
                "{u}: θ = π"
            );
            assert!((sphere.eval(u, 0.0) - at).norm() < 1e-14, "{u}: azimuth");
        }

        let (apex, alpha) = (Point3::new(0.0, 0.0, 2.0), 0.4);
        let axis = Vec3::new(0.0, 0.0, -1.0);
        let frame = crate::Frame::new(apex, axis, u_ref);
        let cone = Surface::Cone {
            frame,
            half_angle: alpha,
        };
        for (u, d) in [(0.0, u_ref), (core::f64::consts::PI, -u_ref)] {
            let g = cone_generator(
                geom_core::RigidMotion::squared_about(apex, frame.axis_unit(), d),
                alpha,
            );
            for v in [-1.5, 0.0, 0.7] {
                assert!((g.eval(v) - cone.eval(u, v)).norm() < 1e-14, "{u}: v = {v}");
            }
        }
    }

    /// **A generator at an azimuth read near the axis, far along it,
    /// opens at its half-angle**: the azimuth of a point `50 m` along a
    /// tilted axis and `1e-5 m` off it, read as the importer reads one
    /// (the point's offset from the axis, normalized), stands off square
    /// to the axis by the axial distance's rounding over the radius,
    /// millions of ulps. Squared about the axis
    /// ([`RigidMotion::squared_about`]), the generator swings in one
    /// frame's plane: unit to its rounding, and `α` off the axis to a few
    /// ulps at every azimuth tried. Red under the generator swung toward
    /// the raw azimuth, `w·cos α + d·sin α`, whose angle to the axis is
    /// off by the azimuth's tilt.
    #[test]
    fn a_generator_read_near_the_axis_far_along_it_opens_at_its_half_angle() {
        let apex = Point3::new(0.3, -1.2, 0.7);
        let axis = crate::stated_direction(Vec3::new(2.0, -1.0, 2.0) / 3.0);
        let across = Vec3::new(1.0, 2.0, 0.0) / 5.0_f64.sqrt();
        let alpha = 0.35_f64;
        let mut tilted = 0;
        for k in 0..64 {
            let t = f64::from(k) * 0.098;
            let ring = across * t.cos() + axis.get().cross(across) * t.sin();
            let p = apex + axis.get() * 50.0 + ring * 1e-5;
            let d = crate::Line {
                origin: apex,
                dir: axis,
            }
            .offset(p)
            .normalize();
            tilted += usize::from(d.dot(axis.get()).abs() > 1e3 * f64::EPSILON);
            let Curve3::Line(g) = cone_generator(RigidMotion::squared_about(apex, axis, d), alpha)
            else {
                unreachable!("a generator is a line")
            };
            assert!(
                geom_core::UnitVec3::stated(g.dir.get()).is_some(),
                "{k}: unit"
            );
            let off = (g.dir.get().dot(axis.get()) - alpha.cos()).abs();
            assert!(off <= 4.0 * f64::EPSILON, "{k}: {off:e} off cos α");
        }
        assert!(tilted > 0, "some azimuth stands off square to the axis");
    }
}
