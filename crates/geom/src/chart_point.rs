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
//! Each arm is the closed-form inverse of [`Surface::eval`] composed
//! with the projection onto the surface: a point off the surface reads
//! the coordinates of its foot. With `w = p − anchor` and `(x, y, h)`
//! its components along `u_ref`, `v_ref = axis × u_ref` and the axis
//! (the plane's normal), `ρ = |(x, y)|`:
//!
//! | arm | `u` | `v` |
//! |---|---|---|
//! | plane | `x` | `y` |
//! | cylinder | azimuth of `(x, y)` | `h` |
//! | cone | azimuth of `σ·(x, y)` | `h·cos α + σ·ρ·sin α` |
//! | sphere | azimuth of `(x, y)` | latitude of `(ρ, h)` |
//! | torus | azimuth of `(x, y)` | tube angle of `(ρ − R, h)` |
//!
//! A cone's `v` is the point's foot on the nearer generator of its
//! half-plane pair, `σ` the sign of `h`: on the mirror nappe (`σ = −1`)
//! the radial term runs against `radial(u)`, so the azimuth is read off
//! `−(x, y)`.
//!
//! **Where an angle names nothing.** A point within the band of the
//! axis (a sphere's pole, a cone's apex), or of a torus's tube circle,
//! has an angle that every value names alike, and a point within the
//! band of a cone's apex plane (`h` undecided) lies within it of both
//! its generators, whose azimuths are half a turn apart; the reading answers the
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
    /// The chart coordinates of the point's foot on the surface.
    pub uv: Point2<T>,
    /// Whether the point's coordinates are not decided past the band:
    /// it lies within the band of the chart's axis, of a torus's tube
    /// circle, or of a cone's apex plane, where more than one `u` names
    /// a foot (module docs). On a sphere this is the band of its poles;
    /// on a cone, of its apex and, for a wide cone, the ring about it
    /// the apex plane's band cuts.
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
    arm: ChartArm<T>,
}

/// The arm of a [`ChartInverse`]: its surface kind with that kind's
/// data past the frame. A consumer serving fewer kinds matches on this
/// answer rather than on the [`Surface`] again.
#[derive(Clone, Copy, Debug)]
pub enum ChartArm<T> {
    /// A plane; `v` along `normal × u_ref`.
    Plane,
    /// A cylinder of the radius; `v` axial metres.
    Cylinder {
        /// The radius.
        radius: T,
    },
    /// A cone of the half-angle α; `v` slant metres from the apex.
    Cone {
        /// The half-angle α.
        half_angle: T,
    },
    /// A sphere of the radius; `v` latitude.
    Sphere {
        /// The radius.
        radius: T,
    },
    /// A torus; `v` the tube angle.
    Torus {
        /// The major radius R.
        major: T,
        /// The minor radius r.
        minor: T,
    },
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
            } => (origin, normal, u_ref, ChartArm::Plane),
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => (origin, axis, u_ref, ChartArm::Cylinder { radius }),
            Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => (apex, axis, u_ref, ChartArm::Cone { half_angle }),
            Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => (center, axis, u_ref, ChartArm::Sphere { radius }),
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => (
                center,
                axis,
                u_ref,
                ChartArm::Torus {
                    major: major_radius,
                    minor: minor_radius,
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

    /// The arm, with its kind's data.
    pub fn arm(&self) -> ChartArm<T> {
        self.arm
    }

    /// The point the frame is anchored at (origin, apex or center).
    pub fn anchor(&self) -> Point3<T> {
        self.anchor
    }

    /// The axis (the plane's normal).
    pub fn axis(&self) -> Vec3<T> {
        self.axis
    }

    /// The seam direction, `u = 0`.
    pub fn u_ref(&self) -> Vec3<T> {
        self.u_ref
    }

    /// `axis × u_ref`, the frame convention.
    pub fn v_ref(&self) -> Vec3<T> {
        self.v_ref
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
            ChartArm::Plane => (x, y, false),
            ChartArm::Cylinder { .. } => (
                Azimuth::of(y, x).principal(band),
                h,
                off("chart_point_on_axis", rho),
            ),
            ChartArm::Cone { half_angle } => {
                let (sin_half_angle, cos_half_angle) = half_angle.sin_cos();
                // The nearer of the two generators in the point's
                // half-plane pair: the `+radial` one where `h > 0`, the
                // mirror one where `h < 0`. Within the band of `h = 0`
                // both pass within it, so neither is the point's.
                let nappe = decide("chart_point_cone_nappe", Margin::of(h), band);
                let (radial, sigma) = match nappe {
                    Ok(Sign::Negative) => (Azimuth::of(T::zero() - y, T::zero() - x), -T::one()),
                    Ok(Sign::Positive | Sign::Zero) | Err(_) => (Azimuth::of(y, x), T::one()),
                };
                (
                    radial.principal(band),
                    h * cos_half_angle + sigma * rho * sin_half_angle,
                    !matches!(nappe, Ok(Sign::Positive | Sign::Negative))
                        || off("chart_point_on_axis", rho),
                )
            }
            ChartArm::Sphere { .. } => (
                Azimuth::of(y, x).principal(band),
                Azimuth::of(h, rho).principal_in(Window::Latitude, band),
                off("chart_point_on_axis", rho),
            ),
            ChartArm::Torus { major, .. } => {
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

    /// The coordinates `chart_point` reads, compared modulo a turn in
    /// each periodic channel.
    fn same(surface: &Surface<f64>, a: Point2<f64>, b: Point2<f64>, tol: f64) -> bool {
        let tau = core::f64::consts::TAU;
        let wrap = |d: f64| d - tau * (d / tau).round();
        let (pu, pv) = match surface {
            Surface::Plane { .. } => (false, false),
            Surface::Torus { .. } => (true, true),
            _ => (true, false),
        };
        let du = if pu { wrap(a.x - b.x) } else { a.x - b.x };
        let dv = if pv { wrap(a.y - b.y) } else { a.y - b.y };
        du.abs() <= tol && dv.abs() <= tol
    }

    /// **Each arm inverts `eval`, and reads a point off the surface as
    /// its foot**: a point evaluated at `(u, v)` reads back `(u, v)`
    /// modulo a turn, and so does that point moved along the normal by
    /// `±δ`, on both nappes of a narrow and of a wide cone and at every
    /// angle round the window; the angles read lie in their windows.
    #[test]
    fn each_arm_reads_a_point_as_its_foot() {
        let mut all = surfaces();
        for alpha in [1.2, 1.4] {
            all.push(Surface::Cone {
                apex: Point3::new(0.2, -0.1, 0.3),
                axis: Vec3::new(0.0, 0.6, 0.8),
                half_angle: alpha,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            });
        }
        for surface in &all {
            for i in 0..48 {
                let u = -3.1 + 0.13 * f64::from(i);
                let v = -1.4 + 0.06 * f64::from(i);
                let on = surface.eval(u, v);
                let n = surface.normal(u, v);
                for d in [0.0, 1e-6, -1e-6, 1e-3] {
                    let p = on + n * d;
                    let read = surface.chart_point(p, band()).unwrap();
                    assert!(
                        same(surface, read.uv, Point2::new(u, v), 1e-9),
                        "{surface:?} at ({u}, {v}) moved {d} along the normal read {:?}",
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
    }

    /// A sphere's poles and a cone's apex read as singular, and a point
    /// a hair off the axis past the band does not. On a wide cone a
    /// mirror-nappe point whose `h` lies in the band reads singular
    /// (both generators pass within the band of it, half a turn apart),
    /// and one past the band reads its own foot.
    #[test]
    fn poles_the_apex_and_a_wide_cones_apex_plane_are_singular() {
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
        for alpha in [1.2, 1.4] {
            let wide = Surface::Cone {
                apex: Point3::new(0.2, -0.1, 0.3),
                axis: Vec3::new(0.0, 0.6, 0.8),
                half_angle: alpha,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            let in_band = wide.chart_point(wide.eval(0.7, -2e-8), band()).unwrap();
            assert!(in_band.singular, "α {alpha}: an apex-plane point in band");
            let past = wide.eval(0.7, -1e-6);
            let read = wide.chart_point(past, band()).unwrap();
            assert!(!read.singular, "α {alpha}: past the band");
            assert!(
                (wide.eval(read.uv.x, read.uv.y) - past).norm() < 1e-15,
                "α {alpha}: {:?} is not the point's own preimage",
                read.uv
            );
        }
    }
}
