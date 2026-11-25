//! **The pole form**: how a face meets a chart singularity of its
//! surface, a sphere's pole or a cone's apex.
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

use geom_core::{Point3, Real, Vec3};

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

/// The cone's generator at azimuth `d` (a unit direction perpendicular
/// to `axis`): the line through the apex whose parameter is the
/// surface's own slant `v`, `apex + (axis·cos α + d·sin α)·v`, so
/// `v > 0` runs on the nappe opening along `axis` and `v < 0` on the
/// other.
pub fn cone_generator<T: Real>(axis: crate::Line<T>, half_angle: T, d: Vec3<T>) -> Curve3<T> {
    let crate::Line {
        origin: apex,
        dir: axis,
    } = axis;
    let (s, c) = half_angle.sin_cos();
    Curve3::line(apex, axis * c + d * s)
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
        let cone = Surface::Cone {
            frame: crate::Frame::new(apex, axis, u_ref),
            half_angle: alpha,
        };
        for (u, d) in [(0.0, u_ref), (core::f64::consts::PI, -u_ref)] {
            let g = cone_generator(
                crate::Line {
                    origin: apex,
                    dir: axis,
                },
                alpha,
                d,
            );
            for v in [-1.5, 0.0, 0.7] {
                assert!((g.eval(v) - cone.eval(u, v)).norm() < 1e-14, "{u}: v = {v}");
            }
        }
    }
}
