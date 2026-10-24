//! **A point's chart coordinates**: the one reading of one carrier
//! against a point that answers `(u, v)` (D3, "One reading per
//! question"). [`ChartInverse::of`] is the kind-level description, one
//! exhaustive match from a [`Surface`] kind to its arm, refusing the
//! kinds with no closed inversion typed; [`ChartInverse::point`] is the
//! arm's method. [`Surface::chart_point`] composes the two for a caller
//! holding both operands.
//!
//! Every angle is read off the geometry as an [`Azimuth`] in its
//! [`Window`] (C4): the azimuth and a torus's tube angle in the angle
//! window, a sphere's latitude in its own. So a point's coordinates are
//! named in the same windows as every placed pcurve image, and the band
//! the reading takes is the band its cut-side decisions are read at.
//!
//! Each arm is the closed-form inverse of [`Surface::eval`], with
//! `w = p − anchor` and `(x, y, h)` its components along `u_ref`,
//! `v_ref = axis × u_ref` and the axis (the plane's normal):
//!
//! | arm | `u` | `v` |
//! |---|---|---|
//! | plane | `x` | `y` |
//! | cylinder | azimuth of `(x, y)` | `h` |
//! | cone | azimuth of `±(x, y)` | `h / cos α` |
//! | sphere | azimuth of `(x, y)` | latitude of `(ρ, h)` |
//! | torus | azimuth of `(x, y)` | tube angle of `(ρ − R, h)` |
//!
//! with `ρ = |(x, y)|`. A cone's mirror nappe (`h` decided negative)
//! carries the radial term against `radial(u)`, so its azimuth is read
//! off `−(x, y)`.
//!
//! **Where an angle names nothing.** A point within the band of the
//! axis (a sphere's pole, a cone's apex), or of a torus's tube circle,
//! has an angle that every value names alike; the reading answers the
//! value [`Azimuth::principal`] reads there and says so
//! ([`ChartPoint::singular`]). A consumer that needs a unique preimage
//! refuses on that answer; one that only needs a preimage reads `uv`.

use geom_core::k_stats::decide;
use geom_core::predicate::Band;
use geom_core::{Decide, Margin, Point2, Point3, Real, Sign, Vec3};

use crate::azimuth::{Azimuth, Window};
use crate::surfaces::{Surface, SurfaceKind};

/// A surface kind with no closed-form chart inversion: a spline net,
/// or a fitted stand-in whose chart is one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NoClosedInverse {
    /// The kind refused.
    pub kind: SurfaceKind,
}

impl core::fmt::Display for NoClosedInverse {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "a {:?} surface has no closed-form chart inversion, so a point has no chart \
             coordinates read off it",
            self.kind
        )
    }
}

impl core::error::Error for NoClosedInverse {}

/// A point read into a surface's chart ([`ChartInverse::point`]).
#[derive(Clone, Copy, Debug)]
pub struct ChartPoint<T: Real> {
    /// The chart coordinates; a preimage of the point's foot on the
    /// surface when the point lies on it.
    pub uv: Point2<T>,
    /// Whether one of the point's angles is not decided past the band:
    /// the point lies within the band of the chart's axis or of a
    /// torus's tube circle, where every value of that angle names it
    /// alike (module docs). A point on the surface is singular exactly
    /// at a sphere's pole or a cone's apex.
    pub singular: bool,
}

/// A surface's chart inversion: the kind-level description of
/// [`Surface::chart_point`] (module docs).
#[derive(Clone, Copy, Debug)]
pub struct ChartInverse<T: Real> {
    anchor: Point3<T>,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    v_ref: Vec3<T>,
    arm: Arm<T>,
}

/// The arm of a [`ChartInverse`], with the one datum past the frame
/// that it reads.
#[derive(Clone, Copy, Debug)]
enum Arm<T> {
    Plane,
    Cylinder,
    Cone { cos_half_angle: T },
    Sphere,
    Torus { major: T },
}

impl<T: Real> ChartInverse<T> {
    /// The inversion of `surface`'s chart.
    ///
    /// # Errors
    ///
    /// [`NoClosedInverse`] for a spline net or a fitted stand-in.
    pub fn of(surface: &Surface<T>) -> Result<Self, NoClosedInverse> {
        let (anchor, axis, u_ref, arm) = match *surface {
            Surface::Plane {
                origin,
                normal,
                u_ref,
            } => (origin, normal, u_ref, Arm::Plane),
            Surface::Cylinder {
                origin,
                axis,
                u_ref,
                ..
            } => (origin, axis, u_ref, Arm::Cylinder),
            Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => (
                apex,
                axis,
                u_ref,
                Arm::Cone {
                    cos_half_angle: half_angle.cos(),
                },
            ),
            Surface::Sphere {
                center,
                axis,
                u_ref,
                ..
            } => (center, axis, u_ref, Arm::Sphere),
            Surface::Torus {
                center,
                axis,
                major_radius,
                u_ref,
                ..
            } => (
                center,
                axis,
                u_ref,
                Arm::Torus {
                    major: major_radius,
                },
            ),
            Surface::Nurbs(_) | Surface::Approx(_) => {
                return Err(NoClosedInverse {
                    kind: surface.kind(),
                });
            }
        };
        Ok(Self {
            anchor,
            axis,
            u_ref,
            v_ref: axis.cross(u_ref),
            arm,
        })
    }
}

impl<T: Decide> ChartInverse<T> {
    /// The chart coordinates of `p`, its angles read at `band` (module
    /// docs).
    pub fn point(&self, p: Point3<T>, band: Band) -> ChartPoint<T> {
        let w = p - self.anchor;
        let (x, y, h) = (w.dot(self.u_ref), w.dot(self.v_ref), w.dot(self.axis));
        let rho = Azimuth::of(y, x).length();
        let off =
            |name, length: T| !matches!(decide(name, Margin::of(length), band), Ok(Sign::Positive));
        let (u, v, singular) = match self.arm {
            Arm::Plane => (x, y, false),
            Arm::Cylinder => (
                Azimuth::of(y, x).principal(band),
                h,
                off("chart_point_on_axis", rho),
            ),
            Arm::Cone { cos_half_angle } => {
                let radial = match decide("chart_point_cone_nappe", Margin::of(h), band) {
                    Ok(Sign::Negative) => Azimuth::of(T::zero() - y, T::zero() - x),
                    Ok(Sign::Positive | Sign::Zero) | Err(_) => Azimuth::of(y, x),
                };
                (
                    radial.principal(band),
                    h / cos_half_angle,
                    off("chart_point_on_axis", rho),
                )
            }
            Arm::Sphere => (
                Azimuth::of(y, x).principal(band),
                Azimuth::of(h, rho).principal_in(Window::Latitude, band),
                off("chart_point_on_axis", rho),
            ),
            Arm::Torus { major } => {
                let tube = Azimuth::of(h, rho - major);
                (
                    Azimuth::of(y, x).principal(band),
                    tube.principal(band),
                    off("chart_point_on_axis", rho)
                        || off("chart_point_on_tube_circle", tube.length()),
                )
            }
        };
        ChartPoint {
            uv: Point2::new(u, v),
            singular,
        }
    }
}

impl<T: Decide> Surface<T> {
    /// **A point's chart coordinates** on this surface, its angles read
    /// at `band`: [`ChartInverse::of`] then [`ChartInverse::point`].
    ///
    /// # Errors
    ///
    /// [`NoClosedInverse`] for a spline net or a fitted stand-in.
    pub fn chart_point(&self, p: Point3<T>, band: Band) -> Result<ChartPoint<T>, NoClosedInverse> {
        ChartInverse::of(self).map(|inverse| inverse.point(p, band))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    fn surfaces() -> Vec<Surface<f64>> {
        let axis = Vec3::new(0.0, 0.6, 0.8);
        let u_ref = Vec3::new(1.0, 0.0, 0.0);
        let o = Point3::new(0.2, -0.1, 0.3);
        vec![
            Surface::Plane {
                origin: o,
                normal: axis,
                u_ref,
            },
            Surface::Cylinder {
                origin: o,
                axis,
                radius: 0.7,
                u_ref,
            },
            Surface::Cone {
                apex: o,
                axis,
                half_angle: 0.4,
                u_ref,
            },
            Surface::Sphere {
                center: o,
                radius: 0.7,
                axis,
                u_ref,
            },
            Surface::Torus {
                center: o,
                axis,
                major_radius: 2.0,
                minor_radius: 0.5,
                u_ref,
            },
        ]
    }

    /// **Each arm inverts `eval`**: a point evaluated at `(u, v)` reads
    /// back a `(u', v')` that evaluates to the same point, on both
    /// nappes of the cone and at every angle round the window, and the
    /// angles it reads lie in their windows.
    #[test]
    fn each_arm_inverts_eval() {
        for surface in &surfaces() {
            for i in 0..48 {
                let u = -3.1 + 0.13 * f64::from(i);
                let v = -1.4 + 0.06 * f64::from(i);
                let p = surface.eval(u, v);
                let read = surface.chart_point(p, band()).unwrap();
                let back = surface.eval(read.uv.x, read.uv.y);
                assert!(
                    (back - p).norm() < 1e-12,
                    "{surface:?} at ({u}, {v}) read {:?}, which is {back:?} not {p:?}",
                    read.uv
                );
                assert!(
                    !read.singular,
                    "{surface:?} at ({u}, {v}) is no singularity"
                );
                if !matches!(surface, Surface::Plane { .. }) {
                    assert!(
                        read.uv.x > -1.0 - 1e-12
                            && read.uv.x <= core::f64::consts::TAU - 1.0 + 1e-12,
                        "{surface:?} u {} outside the angle window",
                        read.uv.x
                    );
                }
            }
        }
    }

    /// A sphere's poles and a cone's apex read as singular, and a point
    /// a hair off the axis past the band does not.
    #[test]
    fn poles_and_the_apex_are_singular() {
        let surfaces = surfaces();
        let (cone, sphere) = (&surfaces[2], &surfaces[3]);
        for (surface, p) in [
            (cone, cone.eval(0.3, 0.0)),
            (sphere, sphere.eval(0.3, core::f64::consts::FRAC_PI_2)),
            (sphere, sphere.eval(0.3, -core::f64::consts::FRAC_PI_2)),
        ] {
            assert!(
                surface.chart_point(p, band()).unwrap().singular,
                "{surface:?} at {p:?}"
            );
        }
        let near = sphere.eval(0.3, core::f64::consts::FRAC_PI_2 - 1e-6);
        assert!(!sphere.chart_point(near, band()).unwrap().singular);
    }
}
