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
//! window, a sphere's latitude and a spindle sheet's tube angle in the
//! latitude's. So a point's coordinates are
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
//! | torus, spindle | azimuth of `(x, y)` | tube angle of `(ρ − R, h)` |
//!
//! The table is spelled once, in [`ChartChannels::read`], which decides
//! nothing: each angle leaves as the direction it is read off, and the
//! caller names it. [`ChartInverse::point`] names it in its window and
//! decides the cone's nappe on `h`; `geom_brep`'s projected image (C4)
//! names it on the branch its piece's sector decided at derivation
//! ([`Azimuth::about`]), its nappe fixed per image.
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

use crate::Frame;
use crate::azimuth::{Azimuth, Window};
use crate::curves::Line;
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
    frame: Frame<T>,
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
    /// A spindle sheet; `v` the tube angle, closed on `[−v*, v*]` and read
    /// about zero ([`ChartAngle::window`]).
    Spindle {
        /// The signed major radius R.
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
        let frame = surface.frame().ok_or(NoClosedInverse {
            kind: surface.kind(),
        })?;
        let arm = match *surface {
            Surface::Plane { .. } => ChartArm::Plane,
            Surface::Cylinder { radius, .. } => ChartArm::Cylinder { radius },
            Surface::Cone { half_angle, .. } => ChartArm::Cone { half_angle },
            Surface::Sphere { radius, .. } => ChartArm::Sphere { radius },
            Surface::Torus {
                major_radius,
                minor_radius,
                ..
            } => ChartArm::Torus {
                major: major_radius,
                minor: minor_radius,
            },
            Surface::Spindle {
                major_radius,
                minor_radius,
                ..
            } => ChartArm::Spindle {
                major: major_radius,
                minor: minor_radius,
            },
            Surface::Nurbs(_) | Surface::Approx(_) => unreachable!("a net has no frame"),
        };
        Ok(Self { frame, arm })
    }

    /// The arm, with its kind's data.
    pub fn arm(&self) -> ChartArm<T> {
        self.arm
    }

    /// The surface's placement ([`Surface::frame`]).
    pub fn frame(&self) -> &Frame<T> {
        &self.frame
    }
}

/// Which angle a chart channel names (module docs' table): the
/// azimuth, a sphere's latitude, a torus's tube angle, or a spindle
/// sheet's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartAngle {
    /// The azimuth of `(x, y)`, or of `−(x, y)` on a cone's mirror nappe.
    Azimuth,
    /// A sphere's latitude, the angle of `(ρ, h)`.
    Latitude,
    /// A torus's tube angle, the angle of `(ρ − R, h)`.
    Tube,
    /// A spindle sheet's tube angle, the angle of `(ρ − R, h)` with `R`
    /// signed.
    SheetTube,
}

impl ChartAngle {
    /// The window a point reading names this angle in (C4): the
    /// latitude's for a sphere's latitude, the angle window for the
    /// azimuth and a torus's tube angle. `None` for a spindle sheet's
    /// tube angle, which is named about zero ([`Azimuth::near`]): the
    /// sheet's `v` is closed on `[−v*, v*]`, `v* ≤ π`, and the one jump
    /// of a read about zero, at `±π`, meets the sheet only at a horn's
    /// cusp.
    pub fn window(self) -> Option<Window> {
        match self {
            Self::Azimuth | Self::Tube => Some(Window::Angle),
            Self::Latitude => Some(Window::Latitude),
            Self::SheetTube => None,
        }
    }
}

/// **The chart's channels**: the one spelling of the module docs'
/// table, the arithmetic [`ChartInverse::point`] and `geom_brep`'s
/// projected image run on frame-local components `(x, y, h)`. Sites
/// that still spell a channel apart are filed under `work/pcert/`
/// (`a-cone-points-v-is-read-off-its-height-in-props-and-replace-face`,
/// `derive-class-spells-a-parallels-latitude-and-a-rims-slant-on-its-own`,
/// `the-winding-count-reads-chart-channels-by-its-own-atan2`,
/// `sphere-and-torus-levels-are-read-as-sin-cos-pairs-by-their-own-spelling`,
/// and the image's own box forms in
/// `the-projected-box-forms-spell-the-point-reading-apart`). It decides nothing: each
/// angle leaves as the direction it is read off ([`Azimuth`]), and the
/// caller names it. [`ChartInverse::point`] names it in its window
/// ([`Azimuth::principal_in`]); `geom_brep`'s projected image `ψ`
/// (C4) names it on the branch its piece's sector decided at derivation
/// ([`Azimuth::about`]), so the image evaluates at every scalar with no
/// band and stays one continuous branch across its piece.
///
/// The arm carries its scalars as the channels read them (a cone's
/// `sin α` and `cos α`, a torus's `R`), so a caller holding them copied
/// (the projected image stores its chart's) reads through the same
/// arithmetic, and the cone's nappe is the caller's: decided on `h` by
/// [`ChartInverse::point`], fixed per image by the projected image.
#[derive(Clone, Copy, Debug)]
pub enum ChartChannels<T> {
    /// `u = x`, `v = y`.
    Plane,
    /// `u` the azimuth, `v = h`.
    Cylinder,
    /// `u` the azimuth of `σ·(x, y)`, `v = h·cos α + σ·ρ·sin α`.
    Cone {
        /// `sin α`.
        sin: T,
        /// `cos α`.
        cos: T,
        /// The mirror nappe, `σ = −1`; the opening one, `σ = 1`, when
        /// false.
        mirror: bool,
    },
    /// `u` the azimuth, `v` the latitude.
    Sphere,
    /// `u` the azimuth, `v` the tube angle.
    Torus {
        /// The major radius `R`.
        major: T,
    },
    /// `u` the azimuth, `v` the sheet's tube angle.
    Spindle {
        /// The signed major radius `R`.
        major: T,
    },
}

/// What [`ChartChannels::read`] answers: the coordinates, and `ρ` on
/// the arms with an azimuth (the radius the azimuth is read at).
#[derive(Clone, Copy, Debug)]
pub struct ChannelRead<T: Real> {
    /// The chart coordinates.
    pub uv: Point2<T>,
    /// `ρ = |(x, y)|`, `None` on the plane.
    pub rho: Option<T>,
}

impl<T: Real> ChartChannels<T> {
    /// The chart coordinates of the frame-local point `(x, y, h)`, each
    /// angle named by `name` off the direction it is read from, the
    /// azimuth first (type docs).
    pub fn read(
        self,
        (x, y, h): (T, T, T),
        mut name: impl FnMut(ChartAngle, Azimuth<T>) -> T,
    ) -> ChannelRead<T> {
        let radial = Azimuth::of(y, x);
        let rho = || radial.length();
        let (u, v, rho) = match self {
            Self::Plane => (x, y, None),
            Self::Cylinder => (name(ChartAngle::Azimuth, radial), h, Some(rho())),
            Self::Cone { sin, cos, mirror } => {
                let rho = rho();
                // On the mirror nappe the radial term runs against
                // `radial(u)`, so the azimuth is read off `−(x, y)`.
                let (direction, sigma) = if mirror {
                    (Azimuth::of(T::zero() - y, T::zero() - x), -T::one())
                } else {
                    (radial, T::one())
                };
                (
                    name(ChartAngle::Azimuth, direction),
                    h * cos + sigma * rho * sin,
                    Some(rho),
                )
            }
            Self::Sphere => {
                let rho = rho();
                let u = name(ChartAngle::Azimuth, radial);
                (
                    u,
                    name(ChartAngle::Latitude, Azimuth::of(h, rho)),
                    Some(rho),
                )
            }
            Self::Torus { major } => {
                let rho = rho();
                let u = name(ChartAngle::Azimuth, radial);
                (
                    u,
                    name(ChartAngle::Tube, Azimuth::tube(h, rho, major)),
                    Some(rho),
                )
            }
            Self::Spindle { major } => {
                let rho = rho();
                let u = name(ChartAngle::Azimuth, radial);
                let v = name(ChartAngle::SheetTube, Azimuth::tube(h, rho, major));
                (u, v, Some(rho))
            }
        };
        ChannelRead {
            uv: Point2::new(u, v),
            rho,
        }
    }
}

impl<T: Decide> ChartInverse<T> {
    /// The chart coordinates of `p`, its angles read at `band` (module
    /// docs): [`ChartChannels::read`] on `p`'s frame-local components,
    /// each angle named in its window, the cone's nappe decided on `h`.
    pub fn point(&self, p: Point3<T>, band: Band) -> ChartPoint<T> {
        let (x, y, h) = self.frame.local(p);
        let off_as =
            |name, length: T| !matches!(decide(name, Margin::of(length), band), Ok(Sign::Positive));
        // The cone's nappe: the nearer of the two generators in the
        // point's half-plane pair, the `+radial` one where `h > 0`, the
        // mirror one where `h < 0`. Within the band of `h = 0` both
        // pass within it, so neither is the point's.
        let mut nappe_decided = true;
        let channels = match self.arm {
            ChartArm::Plane => ChartChannels::Plane,
            ChartArm::Cylinder { .. } => ChartChannels::Cylinder,
            ChartArm::Cone { half_angle } => {
                let (sin, cos) = half_angle.sin_cos();
                let nappe = decide("chart_point_cone_nappe", Margin::of(h), band);
                nappe_decided = matches!(nappe, Ok(Sign::Positive | Sign::Negative));
                ChartChannels::Cone {
                    sin,
                    cos,
                    mirror: matches!(nappe, Ok(Sign::Negative)),
                }
            }
            ChartArm::Sphere { .. } => ChartChannels::Sphere,
            ChartArm::Torus { major, .. } => ChartChannels::Torus { major },
            ChartArm::Spindle { major, .. } => ChartChannels::Spindle { major },
        };
        let mut tube = None;
        let read = channels.read((x, y, h), |angle, direction| {
            if matches!(angle, ChartAngle::Tube | ChartAngle::SheetTube) {
                tube = Some(direction);
            }
            match angle.window() {
                Some(window) => direction.principal_in(window, band),
                None => direction.near(T::zero()),
            }
        });
        let on_axis = || {
            read.rho
                .is_some_and(|rho| off_as("chart_point_on_axis", rho))
        };
        let on_tube_circle =
            || tube.is_some_and(|tube| off_as("chart_point_on_tube_circle", tube.length()));
        let singular = !nappe_decided || on_axis() || on_tube_circle();
        ChartPoint {
            uv: read.uv,
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

    /// **`p`'s foot on this surface, in closed form and on no chart**:
    /// the point of the surface nearest `p`, read off the surface's
    /// geometric fields alone. It reads no `u_ref` and no chart frame, so a
    /// surface charted on another frame, or about its flipped axis, has
    /// the same foot to the bit, with one exception: a `p` exactly on a
    /// cone's apex plane, where `axis · (p − apex)` rounds to a zero whose
    /// sign does not follow the axis. There the nappe is the stored axis's,
    /// and the flipped axis names the other.
    ///
    /// | kind | foot |
    /// |---|---|
    /// | plane | `p` less its offset along the normal ([`Line::square_foot`]) |
    /// | cylinder | the axis's foot `q` ([`Line::foot`]), then `radius` along `p − q` |
    /// | cone | on the generator in `p`'s half-plane, on the nappe of `p`'s side of the apex plane by the raw sign of `axis · (p − apex)`, a zero's sign bit included, `apex + g·(w·g)` |
    /// | sphere | `radius` along `p − center` |
    /// | torus | the tube circle's point `m` along `p − q`, then `minor_radius` along `p − m` |
    ///
    /// The cone's nappe is not decided: within the band of the apex plane
    /// a point is as near one nappe as the other, and a reader that pairs
    /// this foot with chart coordinates reads the chart's own point there,
    /// where the chart names the nappe ([`ChartPoint::singular`]).
    ///
    /// `None` where the direction a foot is read along may be the zero
    /// vector ([`Vec3::checked_direction`]): `p` at a sphere's centre, on
    /// a cylinder's, cone's or torus's axis, or on a torus's tube circle,
    /// where every point of a circle, or the whole sphere, is a foot
    /// alike. At `f64` that is the exact zero vector, so a `p` there only
    /// to rounding reads one of those feet; at the interval scalar it is a
    /// length whose enclosure holds zero.
    ///
    /// `None` also for a spindle sheet, whose meridian circles overlap
    /// across the axis, and for a spline net or a fitted stand-in, none of
    /// which has a foot in closed form.
    #[must_use]
    pub fn foot(&self, p: Point3<T>) -> Option<Point3<T>> {
        let along = |w: Vec3<T>| w.checked_direction().ok().map(|(d, _)| d);
        // `p`'s foot on the axis and the radial direction out to `p`. The
        // offset is rejected from the axis twice: once off `p`, and once
        // more off the offset itself, whose rounding leaves a part along
        // the axis no longer than `p`'s last place. Beside the axis that
        // part is not small against the offset, and a radial read with
        // it would tilt a cone's generator, or lift a torus's tube
        // circle off its plane, by as much. Both rejections are
        // subtractive, not [`Vec3::reject_from`]'s triple product, which
        // amplifies an axis carrying width at the interval scalar
        // ([`Line::offset`]).
        let about = |origin: Point3<T>, axis: Vec3<T>| {
            let line = Line { origin, dir: axis };
            let (t, off) = line.param_and_offset(p);
            (line.at(t), along(off - off.project_onto(axis)))
        };
        match *self {
            Self::Plane { origin, normal, .. } => Some(
                Line {
                    origin,
                    dir: normal,
                }
                .square_foot(p),
            ),
            Self::Cylinder {
                origin,
                axis,
                radius,
                ..
            } => {
                let (q, r) = about(origin, axis);
                r.map(|r| q + r * radius)
            }
            Self::Cone {
                apex,
                axis,
                half_angle,
                ..
            } => {
                let w = p - apex;
                about(apex, axis).1.map(|r| {
                    let (sin, cos) = half_angle.sin_cos();
                    let g = axis * cos.copysign(axis.dot(w)) + r * sin;
                    apex + g * w.dot(g)
                })
            }
            Self::Sphere { center, radius, .. } => along(p - center).map(|d| center + d * radius),
            Self::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                ..
            } => about(center, axis).1.and_then(|r| {
                let m = center + r * major_radius;
                along(p - m).map(|d| m + d * minor_radius)
            }),
            Self::Spindle { .. } | Self::Nurbs(_) | Self::Approx(_) => None,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Vec3;

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// A lemon, an apple, a wide apple (`v* > π − 1`) and one near the
    /// horn, as `(R, r)`.
    const SHEETS: [(f64, f64); 4] = [(-0.1, 0.9), (0.4, 1.0), (0.8, 1.0), (0.999, 1.0)];

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
        // A lemon (`v* ≈ 1.459`) and apples from `v* ≈ 1.982` to within
        // `0.045` of the horn: the sheet's foot is read by the signed
        // major radius. On a sheet the walk's `|v| ≤ 1.42` is stretched
        // onto `0.97·v*`, so an apple's samples run past `π/2` to near
        // its apexes, and a wide one's past `1 − π`.
        for (major_radius, minor_radius) in SHEETS {
            all.push(Surface::Spindle {
                center: Point3::new(0.2, -0.1, 0.3),
                axis: Vec3::new(0.0, 0.6, 0.8),
                major_radius,
                minor_radius,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            });
        }
        for surface in &all {
            let stretch = match *surface {
                Surface::Spindle {
                    major_radius,
                    minor_radius,
                    ..
                } => 0.97 * crate::sheet_apex(major_radius, minor_radius).v / 1.4,
                _ => 1.0,
            };
            for i in 0..48 {
                let u = -3.1 + 0.13 * f64::from(i);
                let v = (-1.4 + 0.06 * f64::from(i)) * stretch;
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

    /// Points within the band of where `surface`'s chart is singular:
    /// beside its axis, a cone's apex plane and a torus's tube circle, each
    /// `5e-10` or `5e-11` off it at a spread of places along it.
    fn beside_singularities(surface: &Surface<f64>) -> Vec<Point3<f64>> {
        let Some(axis) = surface.axis_line() else {
            return Vec::new();
        };
        let a = axis.dir.normalize();
        let (e1, e2) = a.orthonormal_basis();
        let mut out = Vec::new();
        for k in 0..12 {
            let phi = f64::from(k) * 0.53;
            let radial = e1 * phi.cos() + e2 * phi.sin();
            for off in [5e-10, -5e-11, 0.0] {
                // Beside the axis, at heights along it.
                if off != 0.0 {
                    out.push(axis.origin + a * (f64::from(k) * 0.3 - 1.6) + radial * off.abs());
                }
                match *surface {
                    // Beside the apex plane, out along it.
                    Surface::Cone { apex, .. } => {
                        out.push(apex + radial * (0.4 + f64::from(k) * 0.2) + a * off);
                    }
                    // Beside the tube circle, along its plane and across it.
                    Surface::Torus {
                        center,
                        major_radius,
                        ..
                    } if off != 0.0 => {
                        let m = center + radial * major_radius;
                        out.push(m + radial * off);
                        out.push(m + a * off);
                    }
                    _ => {}
                }
            }
        }
        out
    }

    /// **The foot is the chart's foot, on no chart.** Over a cloud of
    /// points round each analytic kind, and points within the band of
    /// where its chart is singular ([`beside_singularities`]):
    /// - the foot is no farther from `p` than the chart's round trip
    ///   `eval(chart_point(p))`;
    /// - where the chart reads `p` past its singularities, `p − foot`
    ///   lies along the surface's normal there (the oracle: the nearest
    ///   point is where the normal points at `p`) and the foot is the
    ///   round trip to rounding. Within the band of one, the chart names
    ///   a foot by convention (a cone's nappe, an azimuth beside an axis)
    ///   that the closed form need not share, and the foot's direction
    ///   is read off a difference no longer than the band, so only its
    ///   nearness is asked;
    /// - on each frame of [`crate::test_support::CHART_FRAMES`], the axis
    ///   flipped on one, the foot is the same to the bit.
    #[test]
    fn the_foot_is_the_charts_foot_on_no_chart() {
        for surface in surfaces() {
            let others: Vec<Surface<f64>> = crate::test_support::CHART_FRAMES[1..]
                .iter()
                .map(|&(t, flip)| crate::test_support::reframed(&surface, t, flip))
                .collect();
            let cloud = (0..500).map(|i| {
                let k = f64::from(i);
                Point3::new(
                    2.5 * (k * 0.731).sin(),
                    2.5 * (k * 1.917).sin(),
                    2.5 * (k * 0.389).cos(),
                )
            });
            for p in cloud.chain(beside_singularities(&surface)) {
                let foot = surface
                    .foot(p)
                    .expect("a point off the zero direction has a foot");
                let chart = surface.chart_point(p, band()).unwrap();
                let round_trip = surface.eval(chart.uv.x, chart.uv.y);
                if !chart.singular {
                    assert!((foot - round_trip).norm() < 1e-14, "{surface:?} at {p:?}");
                }
                assert!(
                    (p - foot).norm() <= (p - round_trip).norm() + 1e-14,
                    "{surface:?} at {p:?}: the foot is the nearer: {:e} vs {:e} singular {}",
                    (p - foot).norm(),
                    (p - round_trip).norm(),
                    chart.singular
                );
                if !chart.singular {
                    let n = surface.normal(chart.uv.x, chart.uv.y);
                    let off = p - foot;
                    assert!(
                        off.cross(n).norm() <= 1e-13 * (1.0 + off.norm()),
                        "{surface:?} at {p:?}: p lies off the foot along the normal"
                    );
                }
                // Exactly on a cone's apex plane the nappe is the stored
                // axis's, so the flipped frame names the other (`foot`'s
                // docs).
                let on_apex_plane = match surface {
                    Surface::Cone { apex, axis, .. } => axis.dot(p - apex) == 0.0,
                    _ => false,
                };
                let frames = crate::test_support::CHART_FRAMES[1..].iter();
                for (other, &(_, flip)) in others.iter().zip(frames) {
                    if on_apex_plane && flip {
                        continue;
                    }
                    let again = other.foot(p).unwrap();
                    assert_eq!(
                        (again.x.to_bits(), again.y.to_bits(), again.z.to_bits()),
                        (foot.x.to_bits(), foot.y.to_bits(), foot.z.to_bits()),
                        "{surface:?} reframed as {other:?} at {p:?}"
                    );
                }
            }
        }
    }

    /// **Where the direction a foot is read along vanishes, there is no
    /// foot**: a sphere's centre, a point of a cylinder's, cone's or
    /// torus's axis, a point of a torus's tube circle; and a spindle sheet
    /// has none in closed form.
    #[test]
    fn a_point_every_point_of_a_circle_is_nearest_has_no_foot() {
        let (o, z, x) = (
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        );
        let surfaces = [
            Surface::Cylinder {
                origin: o,
                axis: z,
                radius: 0.7,
                u_ref: x,
            },
            Surface::Cone {
                apex: o,
                axis: z,
                half_angle: 0.4,
                u_ref: x,
            },
            Surface::Sphere {
                center: o,
                radius: 0.7,
                axis: z,
                u_ref: x,
            },
            Surface::Torus {
                center: o,
                axis: z,
                major_radius: 2.0,
                minor_radius: 0.5,
                u_ref: x,
            },
        ];
        for s in &surfaces {
            assert!(s.foot(o).is_none(), "{s:?} at its centre");
            if !matches!(s, Surface::Sphere { .. }) {
                assert!(s.foot(o + z * 1.5).is_none(), "{s:?} on its axis");
            }
            assert!(
                s.foot(o + z * 1.5 + x * 1e-3).is_some(),
                "{s:?} off its axis"
            );
        }
        let torus = &surfaces[3];
        assert!(torus.foot(o + x * 2.0).is_none(), "on the tube circle");
        assert!(torus.foot(o + x * 2.0 + z * 1e-3).is_some());
        let sheet = Surface::Spindle {
            center: o,
            axis: z,
            major_radius: 0.4,
            minor_radius: 1.0,
            u_ref: x,
        };
        assert!(sheet.foot(o + z * 0.2 + x * 0.5).is_none());
        let plane = Surface::Plane {
            origin: o,
            normal: z,
            u_ref: x,
        };
        assert!(plane.foot(o).is_some(), "a plane has a foot everywhere");
    }

    /// **At the interval scalar, a point whose direction's enclosure
    /// holds zero has no foot**, on every arm that reads one: on a skew
    /// axis lifted to `Interval`, a point on the axis is on it only to the
    /// axis's enclosure, so the radial offset straddles zero, and the foot
    /// is refused rather than read along a direction that is not one. So
    /// for the cylinder's, cone's and torus's axes and the torus's tube
    /// circle, and for an enclosure of a point that holds the cone's apex
    /// or the sphere's centre (each also an exact zero vector at its own
    /// point); a point a millimetre off each has a foot.
    #[test]
    fn an_enclosure_that_holds_the_zero_direction_has_no_foot() {
        use geom_core::Interval;
        let iv = Interval::from_f64;
        let axis = Vec3::new(1.0, 2.0, 3.0).normalize();
        let u_ref = axis.cross(Vec3::new(0.0, 0.0, 1.0)).normalize();
        let o = Point3::new(0.3, 0.1, 0.2);
        let (a, u) = (axis.map(iv), u_ref.map(iv));
        let lifted = |p: Point3<f64>| p.map(iv);
        // An enclosure of `p` a femtometre wide each way.
        let about_p = |p: Point3<f64>| p.map(|x| Interval::from_bounds(x - 1e-15, x + 1e-15));
        let off = u_ref.cross(axis) * 1e-3;
        let on_axis = o + axis * 1.5;
        let tube = o + u_ref * 2.0;
        let rows: [(Surface<Interval>, Point3<f64>, &str); 7] = [
            (
                Surface::Cylinder {
                    origin: lifted(o),
                    axis: a,
                    radius: iv(0.7),
                    u_ref: u,
                },
                on_axis,
                "the cylinder's axis",
            ),
            (
                Surface::Cone {
                    apex: lifted(o),
                    axis: a,
                    half_angle: iv(0.4),
                    u_ref: u,
                },
                on_axis,
                "the cone's axis",
            ),
            (
                Surface::Cone {
                    apex: lifted(o),
                    axis: a,
                    half_angle: iv(0.4),
                    u_ref: u,
                },
                o,
                "the cone's apex",
            ),
            (
                Surface::Sphere {
                    center: lifted(o),
                    radius: iv(0.7),
                    axis: a,
                    u_ref: u,
                },
                o,
                "the sphere's centre",
            ),
            (
                Surface::Torus {
                    center: lifted(o),
                    axis: a,
                    major_radius: iv(2.0),
                    minor_radius: iv(0.5),
                    u_ref: u,
                },
                on_axis,
                "the torus's axis",
            ),
            (
                Surface::Torus {
                    center: lifted(o),
                    axis: a,
                    major_radius: iv(2.0),
                    minor_radius: iv(0.5),
                    u_ref: u,
                },
                tube,
                "the torus's tube circle",
            ),
            (
                Surface::Cylinder {
                    origin: lifted(o),
                    axis: a,
                    radius: iv(0.7),
                    u_ref: u,
                },
                o + axis * -0.8,
                "the cylinder's axis, below its origin",
            ),
        ];
        // The apex and the centre are points, and a point there is the
        // exact zero vector; an enclosure holding one straddles it.
        let points = ["the cone's apex", "the sphere's centre"];
        for (surface, p, what) in rows {
            let at = if points.contains(&what) {
                about_p(p)
            } else {
                lifted(p)
            };
            assert!(surface.foot(at).is_none(), "on {what}");
            assert!(surface.foot(lifted(p + off)).is_some(), "off {what}");
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
        // A spindle sheet's two apexes, at the ends `±v*` of its window
        // (`cos v* = −R/r`), lie on its axis.
        for (major_radius, minor_radius) in SHEETS {
            let sheet = Surface::Spindle {
                center: Point3::new(0.2, -0.1, 0.3),
                axis: Vec3::new(0.0, 0.6, 0.8),
                major_radius,
                minor_radius,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            let v_star = crate::sheet_apex(major_radius, minor_radius).v;
            for v in [v_star, -v_star] {
                let apex = sheet.eval(0.3, v);
                assert!(
                    sheet.chart_point(apex, band()).unwrap().singular,
                    "R = {major_radius}: the apex at v = {v} reads singular"
                );
            }
            let near = sheet.eval(0.3, v_star - 1e-3);
            assert!(
                !sheet.chart_point(near, band()).unwrap().singular,
                "R = {major_radius}: a point inside the window reads its foot"
            );
        }
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
