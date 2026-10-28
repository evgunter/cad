//! Check 4's per-arm lemma, swept: on every periodic chart class the
//! derivation covers, the frame, incidence and fidelity terms dominate
//! the true sup of `|S(P(t)) − C(t)|`, at `f64` and at the certifying
//! scalar `Interval`; every arm's every term is load-bearing in some row
//! of a pinned sweep, so a table that under-states one goes red with no
//! luck of the seed; a stored image corrupted in any one respect
//! refuses at `Interval` as at `f64`; and on an exact carrier the
//! closed-form tables compose back to the carrier. In a file of its own
//! so the per-file gate can skip the sweeps without skipping
//! `pcurve_cache`'s deterministic rows.
//!
//! Since check 3 left the parameter box (C4), this file is the only
//! thing that pins the lemma on
//! [`super::EnvelopeStatement::MapResidualClosedForm`] at the scalar
//! that certifies.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/pcurve_cache.rs",
    "crates/geom-brep/src/pcurve_cache/projected.rs",
    "crates/geom/src/surfaces.rs",
    "crates/geom/src/curves.rs",
    "crates/geom/src/azimuth.rs",
];

use std::collections::{BTreeMap, BTreeSet};

mod projected;
mod projected_searches;

use super::{
    AxisMap, Channel, ChartMap, Derivation, EnvelopeTerm, EnvelopeTerms, FocalImage, IncidenceRead,
    Pcurve, PcurveCache, PcurveCertifyError, PcurveCheck, Period, angle_rate, carrier_harmonic,
    channels_of, chart_image_harmonic, chart_pcurve, derive_harmonic, focal_section_envelope,
    incidence, orthonormal_chart, periodic_envelope,
};
use geom::{Curve3, Surface};
use geom_core::k_stats::decide;
use geom_core::predicate::Band;
use geom_core::tolerance::Tol;
use geom_core::{Bounds, Interval, Margin, Point3, Real, Sign, Vec3};
use std::f64::consts::{FRAC_PI_2, PI, TAU};
use test_utils::fuzz;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn unit(s: &mut fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(s.range(-1.0, 1.0), s.range(-1.0, 1.0), s.range(-1.0, 1.0));
        if v.norm() > 0.2 {
            return v.normalize();
        }
    }
}

/// A random right-handed chart frame `(axis, u_ref)`, unit to rounding.
fn frame(s: &mut fuzz::Rng) -> (Vec3<f64>, Vec3<f64>) {
    let axis = unit(s);
    let u_ref = axis.cross(unit(s)).normalize();
    (axis, u_ref)
}

fn rad(axis: Vec3<f64>, u_ref: Vec3<f64>, u: f64) -> Vec3<f64> {
    u_ref * u.cos() + axis.cross(u_ref) * u.sin()
}

fn sign(s: &mut fuzz::Rng) -> f64 {
    if s.below(2) == 0 { 1.0 } else { -1.0 }
}

fn circle(center: Point3<f64>, axis: Vec3<f64>, radius: f64, u_ref: Vec3<f64>) -> Curve3<f64> {
    Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    }
}

/// A size of move, from all three regimes the decisions read: inside
/// the band's zero half (every class decision reads the exact class),
/// inside `(ε, Kε)` where the D9 tie-breaks select (a radial amplitude
/// or a winding in the band takes the static form, and the term it
/// admits carries the whole residual), and up to a millimetre.
fn magnitude(s: &mut fuzz::Rng) -> f64 {
    let b = band();
    match s.below(3) {
        0 => b.zero() * s.range(0.05, 0.95),
        1 => b.zero() + (b.escalate() - b.zero()) * s.range(0.02, 0.98),
        _ => 10f64.powf(s.range(-8.0, -3.0)),
    }
}

/// The carrier classes the sweep draws: every arm `derive_harmonic`
/// selects, each class it has a tie-break for, and the slivers between
/// classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Class {
    CylinderRim,
    CylinderSection,
    CylinderRuling,
    /// A circle in a plane holding the axis, its radius in the band: the
    /// radial amplitude decides Zero or in band and the meridian form is
    /// taken, so its whole radial motion is the meridian's `Drift`.
    CylinderMeridianCircle,
    ConeRim,
    ConeRuling,
    /// A ruling anchored at the apex (its anchor's height in the band),
    /// so the nappe is read off the direction.
    ConeRulingAtApex,
    SphereParallel,
    SphereMeridian,
    SphereMeridianAtPole,
    /// A meridian started `1e-12`–`1e-6` rad off a pole.
    SphereMeridianNearPole,
    TorusParallel,
    TorusMeridian,
    /// A circle on the chart whose radius is small enough that its
    /// winding decides Zero (or in band) against the chart's lever.
    SmallCircle,
    /// A small circle ⊥ the axis and centred on it, at any height: the
    /// rim and parallel classes' zero-winding arm (the winding is an
    /// area, `ρ²`, metered at the chart's lever).
    AxisCircle,
    /// A line leaning off a cylinder's axis: a radial part that moves
    /// with no winding, so the moving class's zero-winding arm with its
    /// `Line` term.
    CylinderLeaningLine,
}

const CLASSES: [Class; 16] = [
    Class::CylinderRim,
    Class::CylinderSection,
    Class::CylinderRuling,
    Class::CylinderMeridianCircle,
    Class::ConeRim,
    Class::ConeRuling,
    Class::ConeRulingAtApex,
    Class::SphereParallel,
    Class::SphereMeridian,
    Class::SphereMeridianAtPole,
    Class::SphereMeridianNearPole,
    Class::TorusParallel,
    Class::TorusMeridian,
    Class::SmallCircle,
    Class::AxisCircle,
    Class::CylinderLeaningLine,
];

/// Whether a carrier of `class` is imaged to round-off by the arm its
/// derivation selects. Four classes lie on the chart within its zero
/// band, so the mint's incidence read admits them, but not exactly on
/// the class their derivation selects: `SmallCircle` and
/// `CylinderMeridianCircle` (their radial motion is the admitted
/// drift), `AxisCircle` (its plane tilted inside the band) and
/// `CylinderLeaningLine` (its lean along the wall); they are there to
/// reach the zero-winding arms. The near-pole meridian's derivation
/// reads its meridian plane off an `a_r` a few ulps long, so its image
/// is off by that conditioning.
fn images_exactly(class: Class) -> bool {
    !matches!(
        class,
        Class::SmallCircle
            | Class::CylinderMeridianCircle
            | Class::SphereMeridianNearPole
            | Class::AxisCircle
            | Class::CylinderLeaningLine
    )
}

/// One random chart and one carrier of `class` on it, exact.
fn on_chart(s: &mut fuzz::Rng, class: Class) -> (Surface<f64>, Curve3<f64>) {
    let (axis, u_ref) = frame(s);
    let at = Point3::new(s.range(-3.0, 3.0), s.range(-3.0, 3.0), s.range(-3.0, 3.0));
    let alpha = s.range(-PI, PI);
    let r = rad(axis, u_ref, alpha);
    let tang = axis.cross(r);
    let cylinder = |s: &mut fuzz::Rng| {
        let radius = s.range(0.2, 4.0);
        (
            Surface::Cylinder {
                origin: at,
                axis,
                radius,
                u_ref,
            },
            radius,
            s.range(-3.0, 3.0),
        )
    };
    let cone = |s: &mut fuzz::Rng| {
        let half_angle = s.range(0.1, 1.4);
        (
            Surface::Cone {
                apex: at,
                axis,
                half_angle,
                u_ref,
            },
            half_angle,
        )
    };
    let sphere = |s: &mut fuzz::Rng| {
        let radius = s.range(0.2, 4.0);
        (
            Surface::Sphere {
                center: at,
                radius,
                axis,
                u_ref,
            },
            radius,
        )
    };
    let torus = |s: &mut fuzz::Rng| {
        let minor = s.range(0.2, 1.5);
        let major = minor + s.range(0.2, 3.0);
        (
            Surface::Torus {
                center: at,
                axis,
                major_radius: major,
                minor_radius: minor,
                u_ref,
            },
            major,
            minor,
        )
    };
    let meridian_start = |delta: f64| r * delta.cos() + axis * delta.sin();
    match class {
        Class::CylinderRim => {
            let (chart, radius, v0) = cylinder(s);
            (chart, circle(at + axis * v0, axis * sign(s), radius, r))
        }
        Class::CylinderSection => {
            // A plane section tilted by θ about `tang`: the ellipse whose
            // minor axis is the radius along it.
            let (chart, radius, v0) = cylinder(s);
            let theta = s.range(0.05, 1.2);
            let normal = axis * theta.cos() + r * theta.sin();
            let major_dir = normal.cross(tang).normalize();
            (
                chart,
                Curve3::Ellipse {
                    center: at + axis * v0,
                    axis: normal,
                    semi_u: radius / theta.cos(),
                    semi_v: radius,
                    u_ref: major_dir,
                },
            )
        }
        Class::CylinderRuling => {
            let (chart, radius, v0) = cylinder(s);
            (
                chart,
                Curve3::Line {
                    origin: at + r * radius + axis * v0,
                    dir: axis * s.range(-3.0, 3.0),
                },
            )
        }
        Class::CylinderMeridianCircle => {
            // In the plane tangent to the wall, which holds the axis: its
            // radial amplitude is its radius, in the band, and it leaves
            // the wall by its sagitta `ρ²/2R`, far inside it.
            let (chart, radius, v0) = cylinder(s);
            let rho = band().escalate() * s.range(0.02, 0.98);
            let start = tang * s.range(-1.0, 1.0) + axis * s.range(-1.0, 1.0);
            (
                chart,
                circle(
                    at + r * radius + axis * v0,
                    r * sign(s),
                    rho,
                    if start.norm() > 0.1 {
                        start.normalize()
                    } else {
                        tang
                    },
                ),
            )
        }
        Class::ConeRim => {
            let (chart, half_angle) = cone(s);
            let v0 = sign(s) * s.range(0.2, 3.0);
            let center = at + axis * (v0 * half_angle.cos());
            let radius = v0.abs() * half_angle.sin();
            // A mirror-nappe rim's chart azimuth is the spatial one plus
            // π, so its `u_ref` is the spatial direction.
            let spatial = if v0 > 0.0 { r } else { -r };
            (chart, circle(center, axis * sign(s), radius, spatial))
        }
        Class::ConeRuling | Class::ConeRulingAtApex => {
            let (chart, half_angle) = cone(s);
            let generator = axis * half_angle.cos() + r * half_angle.sin();
            let v0 = if class == Class::ConeRuling {
                sign(s) * s.range(0.2, 3.0)
            } else {
                sign(s) * band().zero() * s.range(0.0, 0.9)
            };
            (
                chart,
                Curve3::Line {
                    origin: at + generator * v0,
                    dir: generator * (sign(s) * s.range(0.2, 3.0)),
                },
            )
        }
        Class::SphereParallel => {
            let (chart, radius) = sphere(s);
            let v0 = s.range(-1.3, 1.3);
            (
                chart,
                circle(
                    at + axis * (radius * v0.sin()),
                    axis * sign(s),
                    radius * v0.cos(),
                    r,
                ),
            )
        }
        Class::SphereMeridian | Class::SphereMeridianAtPole | Class::SphereMeridianNearPole => {
            let (chart, radius) = sphere(s);
            let delta = match class {
                Class::SphereMeridian => s.range(-1.4, 1.4),
                Class::SphereMeridianAtPole => sign(s) * FRAC_PI_2,
                _ => sign(s) * (FRAC_PI_2 - 10f64.powf(s.range(-12.0, -6.0))),
            };
            (
                chart,
                circle(at, tang * sign(s), radius, meridian_start(delta)),
            )
        }
        Class::TorusParallel => {
            let (chart, major, minor) = torus(s);
            let v0 = s.range(-PI, PI);
            (
                chart,
                circle(
                    at + axis * (minor * v0.sin()),
                    axis * sign(s),
                    major + minor * v0.cos(),
                    r,
                ),
            )
        }
        Class::TorusMeridian => {
            let (chart, major, minor) = torus(s);
            let delta = s.range(-PI, PI);
            (
                chart,
                circle(at + r * major, tang * sign(s), minor, meridian_start(delta)),
            )
        }
        Class::SmallCircle => {
            // A small circle in the plane tangent to the chart at one of
            // its points: its radial amplitude is real but its winding,
            // an area metered at the chart's lever, reads Zero or in
            // band. Its radius keeps its sagitta `κ·ρ²/2` (κ the chart's
            // largest curvature there) inside half the zero band.
            let (chart, point, normal, kappa) = match s.below(4) {
                0 => {
                    let (chart, radius, v0) = cylinder(s);
                    (chart, at + r * radius + axis * v0, r, 1.0 / radius)
                }
                1 => {
                    let (chart, half_angle) = cone(s);
                    let v = sign(s) * s.range(0.2, 3.0);
                    let (sin, cos) = half_angle.sin_cos();
                    (
                        chart,
                        at + (axis * cos + r * sin) * v,
                        r * cos - axis * sin,
                        cos / (v.abs() * sin),
                    )
                }
                2 => {
                    let (chart, radius) = sphere(s);
                    let v = s.range(-1.3, 1.3);
                    let out = r * v.cos() + axis * v.sin();
                    (chart, at + out * radius, out, 1.0 / radius)
                }
                _ => {
                    let (chart, major, minor) = torus(s);
                    let v = s.range(-PI, PI);
                    let out = r * v.cos() + axis * v.sin();
                    let kappa = (1.0 / minor).max(1.0 / (major - minor));
                    (chart, at + r * major + out * minor, out, kappa)
                }
            };
            let rho = (band().zero() / kappa).sqrt() * s.range(0.05, 0.95);
            let u = unit(s);
            let e = (u - normal * u.dot(normal)).normalize();
            (chart, circle(point, normal * sign(s), rho, e))
        }
        Class::AxisCircle => {
            // A circle ⊥ the axis and centred on it, on the chart: a cone's
            // rim at its own height, a sphere's polar circle, a near-horn
            // torus's inner equator (the parallel nearest the axis). From
            // inside the band (the winding, metered at the circle's own
            // radius, reads Zero) to well clear of it; the plane tilted
            // off ⊥ by an angle the axial decision still reads Zero at
            // the radius drawn, which moves it off the chart by less than
            // the zero band, so a turning class's two radial coefficients
            // disagree (its `Radius` pair). A cylinder holds no circle
            // about its axis but its rims: its zero-winding arm is
            // `SmallCircle`'s and `CylinderLeaningLine`'s.
            let rho = band().zero() * 10f64.powf(s.range(-1.5, 4.0));
            let theta = (band().zero() / rho).min(1.0) * s.range(0.0, 0.9);
            let normal = (axis + tang * theta).normalize();
            let (chart, height) = match s.below(3) {
                0 => {
                    // A rim is one only clear of the apex's band, where
                    // the apex plane meets the cone at the apex alone: a
                    // radius the drawn cone would set within that band
                    // takes a cone narrow enough to set it clear.
                    let (_, drawn) = cone(s);
                    let clear = band().escalate() * 10f64.powf(s.range(0.5, 6.0));
                    let half_angle = if rho / drawn.tan() < clear {
                        (rho / clear).atan()
                    } else {
                        drawn
                    };
                    let chart = Surface::Cone {
                        apex: at,
                        axis,
                        half_angle,
                        u_ref,
                    };
                    (chart, sign(s) * rho / half_angle.tan())
                }
                1 => {
                    let (chart, radius) = sphere(s);
                    (chart, sign(s) * (radius * radius - rho * rho).sqrt())
                }
                _ => {
                    let minor = s.range(0.2, 1.5);
                    let chart = Surface::Torus {
                        center: at,
                        axis,
                        major_radius: minor + rho,
                        minor_radius: minor,
                        u_ref,
                    };
                    (chart, 0.0)
                }
            };
            (
                chart,
                circle(
                    at + axis * height,
                    normal * sign(s),
                    rho,
                    (r - normal * r.dot(normal)).normalize(),
                ),
            )
        }
        Class::CylinderLeaningLine => {
            // Leaning along the wall, not out of it: over any span drawn
            // (`|t| ≤ 3π`) it leaves the wall by `(t·b)²/2R`, inside half
            // the zero band, while its radial part moves.
            let (chart, radius, v0) = cylinder(s);
            let lean = (radius * band().zero()).sqrt() / (3.0 * PI) * s.range(0.05, 0.95);
            (
                chart,
                Curve3::Line {
                    origin: at + r * radius + axis * v0,
                    dir: axis * s.range(-3.0, 3.0) + tang * (sign(s) * lean),
                },
            )
        }
    }
}

/// The chart with its frame moved off the convention by up to `delta`,
/// one respect at a time or all three: the axis lengthened, `u_ref`
/// lengthened, `u_ref` tilted toward the axis. What `Frame` meters.
fn frame_moved(s: &mut fuzz::Rng, surface: &Surface<f64>, delta: f64) -> Surface<f64> {
    let which = s.below(4);
    let off = |s: &mut fuzz::Rng, axis: Vec3<f64>, u_ref: Vec3<f64>| {
        let mut nudge = |field: usize| {
            if which == 0 || which == field {
                delta * s.range(-1.0, 1.0)
            } else {
                0.0
            }
        };
        let (long_axis, long_u, tilt) = (nudge(1), nudge(2), nudge(3));
        (
            axis * (1.0 + long_axis),
            u_ref * (1.0 + long_u) + axis * tilt,
        )
    };
    match *surface {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => {
            let (axis, u_ref) = off(s, axis, u_ref);
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            }
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => {
            let (axis, u_ref) = off(s, axis, u_ref);
            Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            }
        }
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => {
            let (axis, u_ref) = off(s, axis, u_ref);
            Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            }
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => {
            let (axis, u_ref) = off(s, axis, u_ref);
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            }
        }
        _ => unreachable!("on_chart builds periodic charts only"),
    }
}

/// `carrier` moved off its chart by up to `delta`, its kind kept: one
/// field at a time (centre, size, plane tilt, line direction) or all
/// of them, so each incidence term is met alone as well as mixed.
fn perturbed(s: &mut fuzz::Rng, carrier: &Curve3<f64>, delta: f64) -> Curve3<f64> {
    let which = s.below(4);
    let nudge = |s: &mut fuzz::Rng, field: usize| {
        if which == 0 || which == field {
            unit(s) * (delta * s.unit())
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    };
    let scaled = |s: &mut fuzz::Rng, v: f64| {
        if which == 0 || which == 2 {
            v * (1.0 + delta * s.range(-1.0, 1.0))
        } else {
            v
        }
    };
    match *carrier {
        Curve3::Line { origin, dir } => Curve3::Line {
            origin: origin + nudge(s, 1),
            dir: dir + nudge(s, 3) * dir.norm(),
        },
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => {
            let axis = (axis + nudge(s, 3)).normalize();
            let u_ref = (u_ref - axis * u_ref.dot(axis)).normalize();
            let center = center + nudge(s, 1);
            // Half the size moves stretch the circle into an ellipse, so
            // its two radial coefficients disagree (what a turning
            // class's `Radius` pair meters).
            if (which == 0 || which == 2) && s.below(2) == 0 {
                let (major, minor) = (scaled(s, radius), scaled(s, radius));
                let (major, minor, u_ref) = if major >= minor {
                    (major, minor, u_ref)
                } else {
                    (minor, major, axis.cross(u_ref))
                };
                return Curve3::Ellipse {
                    center,
                    axis,
                    semi_u: major,
                    semi_v: minor,
                    u_ref,
                };
            }
            Curve3::Circle {
                center,
                axis,
                radius: scaled(s, radius),
                u_ref,
            }
        }
        Curve3::Ellipse {
            center,
            axis,
            semi_u,
            semi_v,
            u_ref,
        } => {
            let axis = (axis + nudge(s, 3)).normalize();
            let u_ref = (u_ref - axis * u_ref.dot(axis)).normalize();
            Curve3::Ellipse {
                center: center + nudge(s, 1),
                axis,
                semi_u: scaled(s, semi_u),
                semi_v: scaled(s, semi_v),
                u_ref,
            }
        }
        Curve3::Spiric { .. } | Curve3::Nurbs(_) => unreachable!("on_chart builds neither"),
    }
}

/// The sampled sup of `|S(P(t)) − C(t)|` over `[t0, t1]`: a lower
/// estimate of the true sup, the oracle every row compares against.
fn sampled_sup(p: &Pcurve<f64>, s: &Surface<f64>, c: &Curve3<f64>, t0: f64, t1: f64) -> f64 {
    (0..=4096)
        .map(|k| {
            let t = t0 + (t1 - t0) * (f64::from(k) / 4096.0);
            let q = p.eval(t);
            s.eval(q.x, q.y).distance(c.eval(t))
        })
        .fold(0.0, f64::max)
}

/// The arm a derivation selected, as the coverage census names it: the
/// variant, split where the arm's own terms differ (a moving class's
/// zero winding, a ruling's anchor, a meridian's pole start).
fn arm(derivation: Derivation) -> &'static str {
    let moving = |beta: i32, turning: &'static str, still: &'static str| match beta {
        0 => still,
        _ => turning,
    };
    match derivation {
        Derivation::CylinderMoving { beta } => {
            moving(beta, "cylinder moving", "cylinder moving, zero winding")
        }
        Derivation::CylinderMeridian => "cylinder meridian",
        Derivation::ConeRuling { anchored: true, .. } => "cone ruling, anchored",
        Derivation::ConeRuling {
            anchored: false, ..
        } => "cone ruling, by direction",
        Derivation::ConeRim { beta, .. } => moving(beta, "cone rim", "cone rim, zero winding"),
        Derivation::SphereParallel { beta } => {
            moving(beta, "sphere parallel", "sphere parallel, zero winding")
        }
        Derivation::SphereMeridian { pole: false, .. } => "sphere meridian",
        Derivation::SphereMeridian { pole: true, .. } => "sphere meridian, pole start",
        Derivation::TorusParallel { beta } => {
            moving(beta, "torus parallel", "torus parallel, zero winding")
        }
        Derivation::TorusMeridian { .. } => "torus meridian",
        Derivation::Plane | Derivation::FocalSection => unreachable!("the sweep draws neither"),
    }
}

/// Every arm and the terms it adds, as `incidence` spells them, plus
/// the three every arm carries (`Frame`, `FidelityU`, `FidelityV`).
///
/// Left out: `Line` on an arm only circles and ellipses reach (a
/// turning cylinder, every rim, parallel and meridian). Their harmonic
/// form has `l = 0` identically and a line's has `a = b = 0`, so no
/// carrier the arm admits has a linear term; the term is defence, zero
/// on every input that reaches it, and a table that drops it is not a
/// table that under-states anything.
fn arm_terms() -> Vec<(&'static str, Vec<EnvelopeTerm>)> {
    use EnvelopeTerm::{Centre, Drift, Line, Radius};
    let cases: [(&str, &[EnvelopeTerm]); 14] = [
        ("cylinder moving", &[Centre, Radius]),
        (
            "cylinder moving, zero winding",
            &[Line, Centre, Radius, Drift],
        ),
        ("cylinder meridian", &[Line, Radius, Drift]),
        ("cone ruling, by direction", &[Radius, Line]),
        ("cone ruling, anchored", &[Radius, Line]),
        ("cone rim", &[Centre, Radius]),
        ("cone rim, zero winding", &[Centre, Radius, Drift]),
        ("sphere parallel", &[Centre, Radius]),
        ("sphere parallel, zero winding", &[Centre, Radius, Drift]),
        ("sphere meridian", &[Centre, Radius]),
        ("sphere meridian, pole start", &[Centre, Radius]),
        ("torus parallel", &[Radius]),
        ("torus parallel, zero winding", &[Centre, Radius, Drift]),
        ("torus meridian", &[Centre, Radius]),
    ];
    cases
        .into_iter()
        .map(|(arm, own)| {
            let mut terms = own.to_vec();
            terms.extend([
                EnvelopeTerm::Frame,
                EnvelopeTerm::FidelityU,
                EnvelopeTerm::FidelityV,
            ]);
            (arm, terms)
        })
        .collect()
}

/// A random carrier span: a circle's within one turn, a line's anywhere.
fn span(s: &mut fuzz::Rng) -> (f64, f64) {
    let t0 = s.range(-PI, PI);
    (t0, t0 + s.range(0.1, TAU - 0.1))
}

/// The dimensions a row's round-off scales with.
fn scale(surface: &Surface<f64>, carrier: &Curve3<f64>, reach: f64) -> f64 {
    let c = carrier_harmonic(carrier).unwrap();
    let chart = match *surface {
        Surface::Cylinder { origin, radius, .. } => origin.distance(Point3::origin()) + radius,
        Surface::Cone { apex, .. } => apex.distance(Point3::origin()),
        Surface::Sphere { center, radius, .. } => center.distance(Point3::origin()) + radius,
        Surface::Torus {
            center,
            major_radius,
            minor_radius,
            ..
        } => center.distance(Point3::origin()) + major_radius + minor_radius,
        _ => unreachable!("on_chart builds periodic charts only"),
    };
    chart + c.c.distance(Point3::origin()) + c.a.norm() + c.b.norm() + c.l.norm() * reach + 1.0
}

fn lift_harmonic(p: &Pcurve<f64>) -> Pcurve<Interval> {
    let Pcurve::Harmonic { u, v } = *p else {
        unreachable!("the sweep stores harmonic images")
    };
    let f = Interval::from_f64;
    let lift = |c: Channel<f64>| match c {
        Channel::Angle { phase, winding } => Channel::Angle {
            phase: f(phase),
            winding,
        },
        Channel::Free { c, a, b, l } => Channel::Free {
            c: f(c),
            a: f(a),
            b: f(b),
            l: f(l),
        },
    };
    Pcurve::Harmonic {
        u: lift(u),
        v: lift(v),
    }
}

/// `image` with each channel's numbers passed through `jitter`, in the
/// order `u` then `v`: an angle's phase (its windings are structure, so
/// no jitter reaches them), and a free channel's `c, a, b, l`.
fn jittered(image: &Pcurve<f64>, mut jitter: impl FnMut(f64) -> f64) -> Pcurve<f64> {
    let Pcurve::Harmonic { u, v } = *image else {
        unreachable!("the sweep stores harmonic images")
    };
    let mut channel = |c: Channel<f64>| match c {
        Channel::Angle { phase, winding } => Channel::Angle {
            phase: jitter(phase),
            winding,
        },
        Channel::Free { c, a, b, l } => Channel::Free {
            c: jitter(c),
            a: jitter(a),
            b: jitter(b),
            l: jitter(l),
        },
    };
    let u = channel(u);
    let v = channel(v);
    Pcurve::Harmonic { u, v }
}

/// `image` translated by `du`, `dv`, with each channel's slope negated
/// where `flip` asks. An angle's windings are negated through
/// [`Channel::mapped`], by the reflection about its own phase, which
/// keeps the phase; a free channel's linear coefficient is a number,
/// negated as one.
fn restated(image: &Pcurve<f64>, du: f64, dv: f64, flip: [bool; 2]) -> Pcurve<f64> {
    let Pcurve::Harmonic { u, v } = image.translate(du, dv) else {
        unreachable!("the rows store harmonic images")
    };
    let mv = |c: Channel<f64>, f: bool| match c {
        _ if !f => c,
        Channel::Angle { phase, .. } => c
            .mapped(AxisMap::reflect_about(2.0 * phase))
            .expect("a derived winding has a negation"),
        Channel::Free { c, a, b, l } => Channel::Free { c, a, b, l: -l },
    };
    Pcurve::Harmonic {
        u: mv(u, flip[0]),
        v: mv(v, flip[1]),
    }
}

/// `image`'s twin on a sphere chart ([`ChartMap::sphere_involution`]),
/// then `shift` along `u`: `(u + π + shift, π − v)`.
fn twin(image: &Pcurve<f64>, shift: f64) -> Pcurve<f64> {
    image
        .map_affine(ChartMap::sphere_involution())
        .expect("a derived row has a twin")
        .translate(shift, 0.0)
}

/// One row the sweeps read: the arm the derivation selected and each
/// term's value at `f64`, after the row's dominance was asserted at
/// `f64` and at `Interval`. `None` where the derivation refused (a
/// draw whose class decision escalates is not a row).
struct Row {
    arm: &'static str,
    terms: [f64; EnvelopeTerm::ALL.len()],
    noise: f64,
}

/// **What the draws came to**: the rows checked, and every draw that
/// gave none, counted by its class and why. A draw the mint refuses is
/// a reading of the carrier (moved off the chart, or past the band), so
/// it is no row of the lemma's; it is counted, never dropped unseen,
/// and each sweep holds a floor on the rows it checked.
#[derive(Default)]
struct Tally {
    rows: usize,
    none: BTreeMap<(String, String), usize>,
    /// Rows renamed a whole period over: those the C4 claim was checked
    /// on, and those it is vacuous on (a circle whose turn is in band).
    lifted: usize,
    vacuous: usize,
}

impl Tally {
    fn drop(&mut self, class: Class, why: &str) {
        *self
            .none
            .entry((format!("{class:?}"), why.to_owned()))
            .or_default() += 1;
    }

    /// Why the mint refused a draw: its incidence read, definite or in
    /// band, or a refusal of another kind.
    fn refused(&mut self, class: Class, e: &PcurveCertifyError) {
        let why = match IncidenceRead::of(e) {
            Some(read) if read.is_definite() => "the mint reads it off the chart",
            Some(_) => "the mint reads it in band",
            None => {
                let variant = format!("{e:?}");
                let variant = variant.split([' ', '{', '(']).next().unwrap_or_default();
                return self.drop(class, &format!("the mint refuses it: {variant}"));
            }
        };
        self.drop(class, why);
    }

    /// The sweep's report, and its floors: at least `floor` of `trials`
    /// draws checked as rows, and at least `lift_floor` of the rows
    /// renamed a whole period over checked against C4.
    fn hold(&self, name: &str, trials: usize, floor: f64, lift_floor: f64) {
        println!(
            "[fuzz] {name}: {} of {trials} draws checked as rows; none from {:?}; \
             {} of {} whole-period renamings checked against C4",
            self.rows,
            self.none,
            self.lifted,
            self.lifted + self.vacuous
        );
        assert!(
            self.rows as f64 >= floor * trials as f64,
            "{name}: only {} of {trials} draws checked as rows (floor {floor}); none from {:?} — {}",
            self.rows,
            self.none,
            fuzz::replay()
        );
        assert!(
            self.lifted as f64 >= lift_floor * (self.lifted + self.vacuous) as f64
                && self.lifted > 0,
            "{name}: only {} of {} whole-period renamings checked against C4 (floor {lift_floor}) — {}",
            self.lifted,
            self.lifted + self.vacuous,
            fuzz::replay()
        );
    }
}

/// Draw one row of `class` and assert the lemma on it: the envelope at
/// `f64` is not under the sampled sup, and the envelope at `Interval`
/// (over the row's lifted inputs) is not under it either; a renamed
/// exact image costs nothing.
fn row(s: &mut fuzz::Rng, class: Class, trial: usize, tally: &mut Tally) -> Option<Row> {
    let (chart, exact) = on_chart(s, class);
    // 0: the carrier moves; 1: the image moves; 2: both; 3: neither,
    // the image only renamed (its sphere twin, or a whole period over,
    // which is no row); 4: the chart's frame moves.
    let mode = s.below(5);
    let delta = magnitude(s);
    let surface = if mode == 4 {
        frame_moved(s, &chart, delta)
    } else {
        chart
    };
    let carrier = if matches!(mode, 0 | 2) {
        perturbed(s, &exact, delta)
    } else {
        exact
    };
    let (t0, t1) = span(s);
    let reach = t0.abs().max(t1.abs());
    // The stored image is what the mint derives on the chart as stored,
    // placed over the span it is certified on (C4).
    let derived = derive_harmonic(
        &carrier,
        &surface,
        super::Placement::over((t0, t1), &carrier, &surface),
        band(),
    );
    let derivation = derive_harmonic(
        &carrier,
        &orthonormal_chart(&surface),
        super::Placement::own(&carrier, &surface),
        band(),
    );
    let ((derived, _), (_, derivation)) = match (derived, derivation) {
        (Ok(d), Ok(o)) => (d, o),
        (Err(e), _) | (_, Err(e)) => {
            tally.refused(class, &e);
            return None;
        }
    };
    // An ellipse a move left on a cone images as a cone section, which
    // its own lane certifies.
    if derived.harmonic_form().is_none() {
        tally.drop(class, "a cone section, its own lane's");
        return None;
    }
    let drift = if matches!(mode, 1 | 2) {
        magnitude(s)
    } else {
        0.0
    };
    let pick = s.below(9);
    let mut slot = 0;
    let jitter = |v: f64| {
        slot += 1;
        if pick == 0 || pick == slot {
            v + drift * s.range(-1.0, 1.0)
        } else {
            v
        }
    };
    let mut stored = jittered(&derived, jitter);
    // A whole turn of the window either way, or none.
    let k = [-1, 0, 1][s.below(3)];
    stored = stored.shift_branch(k, Period::new(TAU));
    if matches!(surface, Surface::Sphere { .. }) && s.below(2) == 0 {
        stored = twin(&stored, 0.0);
    }
    let form = carrier_harmonic(&carrier).unwrap();
    let context = || {
        format!(
            "trial {trial} ({class:?}, {derivation:?}, mode {mode}, δ = {delta:e}, \
             drift = {drift:e}, k = {k})"
        )
    };
    let terms = periodic_envelope(
        channels_of(&stored),
        &carrier,
        form,
        &surface,
        (t0, t1),
        reach,
        band(),
    )
    .unwrap_or_else(|e| panic!("{}: the envelope refused at f64: {e}", context()));
    let envelope = terms.total();
    // A NaN envelope decides `Invalid`, which refuses (a circle past a
    // sphere's pole has no polar angle): sound, and no bound to compare.
    if envelope.is_nan() {
        tally.drop(class, "an envelope that decides Invalid");
        return None;
    }
    let sup = sampled_sup(&stored, &surface, &carrier, t0, t1);
    let noise = 64.0 * f64::EPSILON * scale(&surface, &carrier, reach);
    assert!(
        envelope >= sup - noise,
        "{}: envelope {envelope:e} under the sampled sup {sup:e}; terms {:?} — {}",
        context(),
        EnvelopeTerm::ALL
            .iter()
            .map(|&t| (t, terms.0[t.slot()]))
            .collect::<Vec<_>>(),
        fuzz::replay()
    );
    // A row renamed by a whole period is a lifted copy, no row (C4): its
    // azimuth's fidelity term reads the period as its offset and refuses
    // alone, whatever else the draw moved. An exact image renamed by its
    // sphere's twin costs nothing.
    //
    // Vacuous where the term's own lever reads a whole period Zero: a
    // copy one period over sits `π` past half a period, which the term
    // levers at the azimuth's arm over the row's chart box, so where
    // `π·arm` decides Zero the copy is the same point set to within the
    // tolerance and no term can see it; at the door check 2 refuses such
    // a circle's span before check 4
    // (`work/pcert/a-lifted-copy-of-a-circle-whose-turn-is-within-the-band-certifies.md`).
    let turn_in_band =
        super::chart_sup_arms_at(&surface, &stored.chart_box(t0, t1)).is_ok_and(|(arm_u, _)| {
            matches!(
                decide(
                    "pcurve_envelope_term",
                    Margin::of(arm_u.to_meters(PI)),
                    band()
                ),
                Ok(Sign::Zero)
            )
        });
    if k != 0 && turn_in_band {
        tally.vacuous += 1;
    } else if k != 0 {
        tally.lifted += 1;
        assert!(
            !matches!(
                decide(
                    "pcurve_envelope_term",
                    Margin::of(terms.0[EnvelopeTerm::FidelityU.slot()]),
                    band()
                ),
                Ok(Sign::Zero)
            ),
            "{}: a row a whole period off is no row, yet its FidelityU term {:e} is in the \
             band — {}",
            context(),
            terms.0[EnvelopeTerm::FidelityU.slot()],
            fuzz::replay()
        );
    } else if mode == 3 && images_exactly(class) {
        assert!(
            envelope <= noise,
            "{}: a renamed exact image costs {envelope:e}; terms {:?} — {}",
            context(),
            terms.0,
            fuzz::replay()
        );
    }
    // The certifying scalar: the same row's inputs lifted. The
    // derivation's decisions may land elsewhere at `Interval` (a draw in
    // the band), and a refusal is the sound answer there; a bound is
    // never under the sampled sup.
    let lift = Interval::from_f64;
    if let Ok(at_iv) = periodic_envelope(
        channels_of(&lift_harmonic(&stored)),
        &carrier.map_scalar(lift),
        carrier_harmonic(&carrier.map_scalar(lift)).unwrap(),
        &surface.map_scalar(lift),
        (lift(t0), lift(t1)),
        lift(reach),
        band(),
    ) {
        let hi = at_iv.total().hi();
        // A NaN envelope decides `Invalid` and refuses: sound, and no
        // bound to compare.
        assert!(
            hi.is_nan() || hi >= sup - noise,
            "{}: the Interval envelope {hi:e} is under the sampled sup {sup:e}; terms {:?} — {}",
            context(),
            at_iv.0,
            fuzz::replay()
        );
    }
    tally.rows += 1;
    Some(Row {
        arm: arm(derivation),
        terms: terms.0,
        noise,
    })
}

/// **The lemma, swept** (a counterexample search: the seed varies). For
/// a carrier moved off its chart, a stored image moved off the one
/// derived from it (by drift, onto a sphere's involution twin, or a
/// whole period over, which its azimuth term refuses alone), and a
/// chart whose frame is off its convention, check 4's envelope is never
/// below the sampled sup of the stored image's residual, at `f64` or at
/// `Interval`. Each row moves
/// one of the four alone, or two, so a term is met with nothing else to
/// cover for it. The failure prints the terms.
#[test]
fn the_envelope_dominates_the_stored_images_residual_on_every_periodic_arm() {
    let mut s = fuzz::start("pcurve_cache::envelope_lemma::dominates");
    let mut tally = Tally::default();
    let trials = fuzz::scaled(700);
    for trial in 0..trials {
        let class = CLASSES[s.below(CLASSES.len())];
        row(&mut s, class, trial, &mut tally);
    }
    tally.hold("envelope_lemma::dominates", trials, 0.6, 0.9);
}

/// **The lemma, swept where no circle reaches** (a counterexample
/// search: the seed varies). Two sites the circle sweep above cannot
/// load: a sphere meridian's incidence over pairs no circle has (the
/// pole start's orientation is second order on a circle), read off
/// `incidence` against its image `r·(cos v·d̂ + sin v·e)`, `v = δ + σt`,
/// written out here; and a cone rim's stored image moved up to a radian
/// in azimuth and its own slant along the ruling, where the channels'
/// legs meet at an acute angle. Each total is never under the sampled
/// sup, at `f64` or at `Interval`.
#[test]
fn the_envelope_dominates_where_no_circle_reaches() {
    let mut s = fuzz::start("pcurve_cache::envelope_lemma::off_circle");
    let z = band().zero();
    let f = Interval::from_f64;
    let lift3 = |v: Vec3<f64>| Vec3::new(f(v.x), f(v.y), f(v.z));
    let unit = |s: &mut fuzz::Rng| {
        Vec3::new(s.range(-1.0, 1.0), s.range(-1.0, 1.0), s.range(-1.0, 1.0)).normalize()
    };
    for trial in 0..fuzz::scaled(200) {
        // The meridian: `e` the axis, `d̂` the azimuth read, `a` within
        // the band of a pole start (or `d̂`'s own direction), `b` a
        // quarter turn off it, both moved up to the band.
        let r = s.range(0.5, 2.0);
        let e = unit(&mut s);
        let d = unit(&mut s);
        let d = (d - e * d.dot(e)).normalize();
        let pole = s.below(2) == 0;
        let sigma: i32 = if s.below(2) == 0 { 1 } else { -1 };
        let delta = if pole {
            FRAC_PI_2 * if s.below(2) == 0 { 1.0 } else { -1.0 }
        } else {
            s.range(-1.2, 1.2)
        };
        let moved = |s: &mut fuzz::Rng| unit(s) * (z * s.range(0.0, 0.9));
        let a0 = (d * delta.cos() + e * delta.sin()) * r;
        let b0 = (e * delta.cos() - d * delta.sin()) * (r * angle_rate::<f64>(sigma));
        let (a, b) = (a0 + moved(&mut s), b0 + moved(&mut s));
        // The derivation's own reading of `a` decides the pole; a draw it
        // would read the other way is not this arm's.
        let a_r = a - e * a.dot(e);
        if pole != (a_r.norm() < z) {
            continue;
        }
        let d_read = if pole {
            (b - e * b.dot(e)).normalize()
        } else {
            a_r.normalize()
        };
        let v0 = a.dot(e).atan2(a_r.norm());
        let sphere = Surface::Sphere {
            center: Point3::origin(),
            radius: r,
            axis: e,
            u_ref: d_read,
        };
        let form = super::Harmonic3 {
            c: Point3::origin(),
            a,
            b,
            l: Vec3::zero(),
        };
        let derivation = Derivation::SphereMeridian {
            pole,
            sigma,
            far: false,
        };
        let sup = (0..=4096)
            .map(|i| {
                let t = -PI + TAU * f64::from(i) / 4096.0;
                let v = v0 + angle_rate::<f64>(sigma) * t;
                let image = (d_read * v.cos() + e * v.sin()) * r;
                (image - (a * t.cos() + b * t.sin())).norm()
            })
            .fold(0.0, f64::max);
        let mut terms = EnvelopeTerms::new();
        incidence(form, &sphere, derivation, PI, &mut terms);
        let mut terms_i = EnvelopeTerms::new();
        let form_i = super::Harmonic3 {
            c: Point3::new(f(0.0), f(0.0), f(0.0)),
            a: lift3(a),
            b: lift3(b),
            l: lift3(Vec3::zero()),
        };
        incidence(
            form_i,
            &sphere.map_scalar(f),
            derivation,
            f(PI),
            &mut terms_i,
        );
        let noise = 64.0 * f64::EPSILON * (r + 1.0);
        assert!(
            terms.total() >= sup - noise && terms_i.total().hi() >= sup - noise,
            "trial {trial}: meridian (pole {pole}) incidence {:e} / {:e} under the sampled sup {sup:e} — {}",
            terms.total(),
            terms_i.total().hi(),
            fuzz::replay()
        );

        // The cone: a rim at slant `v`, its stored image a long way off.
        let alpha = s.range(0.2, 1.3);
        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: alpha,
            u_ref: Vec3::unit_x(),
        };
        let v = z * 10f64.powf(s.range(2.0, 4.0));
        let carrier = Curve3::Circle {
            center: Point3::new(0.0, 0.0, v * alpha.cos()),
            axis: Vec3::unit_z(),
            radius: v * alpha.sin(),
            u_ref: Vec3::unit_x(),
        };
        let Ok(image @ Pcurve::Harmonic { .. }) = chart_pcurve(&carrier, &cone, band()) else {
            continue;
        };
        let du = s.range(-1.0, 1.0);
        let dv = v * s.range(-0.5, 1.0);
        let stored = restated(&image, du, dv, [false, false]);
        let sup = sampled_sup(&stored, &cone, &carrier, 0.0, TAU);
        let form = carrier_harmonic(&carrier).unwrap();
        let Ok(at_f64) = periodic_envelope(
            channels_of(&stored),
            &carrier,
            form,
            &cone,
            (0.0, TAU),
            TAU,
            band(),
        ) else {
            continue;
        };
        let carrier_i = carrier.map_scalar(f);
        let at_iv = periodic_envelope(
            channels_of(&lift_harmonic(&stored)),
            &carrier_i,
            carrier_harmonic(&carrier_i).unwrap(),
            &cone.map_scalar(f),
            (f(0.0), f(TAU)),
            f(TAU),
            band(),
        )
        .map(|t| t.total().hi());
        let noise = 64.0 * f64::EPSILON * (v + 1.0);
        assert!(
            at_f64.total() >= sup - noise && at_iv.as_ref().is_ok_and(|&hi| hi >= sup - noise),
            "trial {trial}: cone (α {alpha}, v {v:e}) envelope {:e} / {at_iv:?} under the sampled sup {sup:e} — {}",
            at_f64.total(),
            fuzz::replay()
        );
    }
}

/// **Every arm's every term carries some row** (a coverage claim: the
/// seed is pinned, so this row cannot pass on a lucky draw or fail on
/// an unlucky one). On a pinned stream of rows of every class, each
/// row's dominance is asserted (as in the sweep above), and for every
/// arm `derive_harmonic` selects, each of its terms is over the row's
/// noise in at least one row. What it checks is that much: every term
/// is exercised by a row where it is not negligible. It does not assert
/// that dropping the term there breaks dominance.
#[test]
fn every_arm_and_term_is_load_bearing_in_a_pinned_sweep() {
    let mut s = fuzz::pinned(
        "pcurve_cache::envelope_lemma::coverage",
        0x5ca1_ab1e_c0ff_ee42,
    );
    let mut seen: BTreeSet<(&'static str, usize)> = BTreeSet::new();
    let mut tally = Tally::default();
    let mut trial = 0usize;
    for class in CLASSES {
        for _ in 0..if class == Class::AxisCircle { 640 } else { 160 } {
            trial += 1;
            let Some(row) = row(&mut s, class, trial, &mut tally) else {
                continue;
            };
            for term in EnvelopeTerm::ALL {
                if row.terms[term.slot()] > row.noise {
                    seen.insert((row.arm, term.slot()));
                }
            }
        }
    }
    // The pairs the stream reaches only through a move, written down
    // (`zero_winding_axis_circle`).
    for (row, _) in axis_charts().map(|(chart, _)| zero_winding_axis_circle(&chart, 4.0)) {
        for term in EnvelopeTerm::ALL {
            if row.terms[term.slot()] > row.noise {
                seen.insert((row.arm, term.slot()));
            }
        }
    }
    tally.hold("envelope_lemma::coverage", trial, 0.6, 0.9);
    let missing: Vec<(&str, EnvelopeTerm)> = arm_terms()
        .into_iter()
        .flat_map(|(arm, terms)| terms.into_iter().map(move |t| (arm, t)))
        .filter(|&(arm, t)| !seen.contains(&(arm, t.slot())))
        .collect();
    assert!(
        missing.is_empty(),
        "no row carried these (arm, term) pairs over the noise: {missing:?}"
    );
}

/// **A zero-winding axis circle with its centre off the axis**, written
/// down (implementer discipline §8, shape 2): a circle inside the band
/// about a chart's axis (its winding reads Zero at its own radius), its
/// plane ⊥ the axis, its centre `floors` noise floors off the axis. On a
/// cone it is the zero-winding rim, on a torus the zero-winding parallel.
/// The row's dominance is asserted at `f64` and at `Interval`, as the
/// sweep's are. Returned with the row: its `Centre` term's lower end at
/// `Interval`, which the row that pins the witness reads.
fn zero_winding_axis_circle(chart: &Surface<f64>, floors: f64) -> (Row, f64) {
    let (rho, reach) = (axis_rho(), TAU);
    // On the chart: the cone's rim at its own height, the near-horn
    // torus's inner equator.
    let h = match *chart {
        Surface::Cone { half_angle, .. } => rho / half_angle.tan(),
        Surface::Torus { .. } => 0.0,
        _ => unreachable!("zero_winding_axis_circle: a cone or a torus"),
    };
    let on_axis = circle(
        Point3::new(0.0, 0.0, h),
        Vec3::unit_z(),
        rho,
        Vec3::unit_x(),
    );
    let offset = floors * 64.0 * f64::EPSILON * scale(chart, &on_axis, reach);
    let carrier = circle(
        Point3::new(offset, 0.0, h),
        Vec3::unit_z(),
        rho,
        Vec3::unit_x(),
    );
    let (stored, _) = derive_harmonic(
        &carrier,
        chart,
        super::Placement::own(&carrier, chart),
        band(),
    )
    .unwrap_or_else(|e| panic!("{floors} floors: the rim derives: {e}"));
    let (_, derivation) = derive_harmonic(
        &carrier,
        &orthonormal_chart(chart),
        super::Placement::own(&carrier, chart),
        band(),
    )
    .unwrap_or_else(|e| panic!("{floors} floors: the rim derives: {e}"));
    let noise = 64.0 * f64::EPSILON * scale(chart, &carrier, reach);
    let terms = periodic_envelope(
        channels_of(&stored),
        &carrier,
        carrier_harmonic(&carrier).unwrap(),
        chart,
        (0.0, TAU),
        reach,
        band(),
    )
    .unwrap_or_else(|e| panic!("{floors} floors: the envelope refused at f64: {e}"));
    let sup = sampled_sup(&stored, chart, &carrier, 0.0, TAU);
    assert!(
        terms.total() >= sup - noise,
        "{floors} floors: envelope {:e} under the sampled sup {sup:e}",
        terms.total()
    );
    let lift = Interval::from_f64;
    let carrier_i = carrier.map_scalar(lift);
    let at_iv = periodic_envelope(
        channels_of(&lift_harmonic(&stored)),
        &carrier_i,
        carrier_harmonic(&carrier_i).unwrap(),
        &chart.map_scalar(lift),
        (lift(0.0), lift(TAU)),
        lift(reach),
        band(),
    )
    .unwrap_or_else(|e| panic!("{floors} floors: the envelope refused at Interval: {e}"));
    assert!(
        at_iv.total().hi() >= sup - noise,
        "{floors} floors: the Interval envelope {:e} under the sampled sup {sup:e}",
        at_iv.total().hi()
    );
    (
        Row {
            arm: arm(derivation),
            terms: terms.0,
            noise,
        },
        at_iv.0[EnvelopeTerm::Centre.slot()].lo(),
    )
}

/// **A zero-winding cone rim's and torus parallel's `Centre` are
/// load-bearing at every ε row.** The pinned sweep's stream reached
/// these pairs only through a move that carried an axis circle's centre
/// off the axis, in three or four rows, and at ε 1e-12 a reshuffled
/// stream lost the cone's; the witnesses are written down instead
/// ([`zero_winding_axis_circle`]), at two, four and eight noise
/// floors: at ε 1e-12 a floor is a few hundredths of the band, so the
/// offset and the radius together keep the winding inside it.
#[test]
fn a_zero_winding_axis_circles_centre_off_the_axis_is_load_bearing() {
    for ((chart, want), floors) in axis_charts()
        .into_iter()
        .flat_map(|c| [2.0, 4.0, 8.0].map(move |f| (c.clone(), f)))
    {
        let (row, centre_iv) = zero_winding_axis_circle(&chart, floors);
        assert_eq!(row.arm, want, "{floors} floors");
        assert!(
            centre_iv > row.noise,
            "{want}, {floors} floors: Centre's lower end {centre_iv:e} is not over the noise {:e} at Interval",
            row.noise
        );
        let centre = row.terms[EnvelopeTerm::Centre.slot()];
        assert!(
            centre > row.noise,
            "{want}, {floors} floors: Centre {centre:e} is not over the noise {:e}",
            row.noise
        );
    }
}

/// **A zero-winding parallel near a sphere's pole**: a circle in band
/// about the axis whose parallel's radius `ρ = √(R² − h²)` is read where
/// `h ≈ R`, so the polar angle the image reads, `asin(h/R)`, is
/// conditioned by `R/ρ`. Returns the envelope's total at `f64`, its upper
/// end at `Interval`, the sampled sup and the noise floor.
fn zero_winding_near_the_pole(
    chart: &Surface<f64>,
    carrier: &Curve3<f64>,
    (t0, t1): (f64, f64),
) -> (f64, f64, f64, f64) {
    let reach = t0.abs().max(t1.abs());
    let (stored, derivation) = derive_harmonic(
        carrier,
        chart,
        super::Placement::own(carrier, chart),
        band(),
    )
    .unwrap_or_else(|e| panic!("the parallel derives: {e}"));
    assert!(
        matches!(derivation, Derivation::SphereParallel { beta: 0 }),
        "{derivation:?}"
    );
    let form = carrier_harmonic(carrier).unwrap();
    let terms = periodic_envelope(
        channels_of(&stored),
        carrier,
        form,
        chart,
        (t0, t1),
        reach,
        band(),
    )
    .unwrap_or_else(|e| panic!("the envelope refused at f64: {e}"));
    let lift = Interval::from_f64;
    let carrier_i = carrier.map_scalar(lift);
    let at_iv = periodic_envelope(
        channels_of(&lift_harmonic(&stored)),
        &carrier_i,
        carrier_harmonic(&carrier_i).unwrap(),
        &chart.map_scalar(lift),
        (lift(t0), lift(t1)),
        lift(reach),
        band(),
    )
    .unwrap_or_else(|e| panic!("the envelope refused at Interval: {e}"));
    (
        terms.total(),
        at_iv.total().hi(),
        sampled_sup(&stored, chart, carrier, t0, t1),
        64.0 * f64::EPSILON * scale(chart, carrier, reach),
    )
}

/// **The zero-winding parallel's envelope holds near the pole**, at
/// `f64` and at `Interval`: the pinned coverage sweep's trial 2273, a
/// circle of radius `3.3e-11` about the axis of a sphere of radius `0.64`
/// where the parallel's radius is `1.05e-8`, written down on the sphere
/// exactly, and the same circle on the unit sphere where the parallel's
/// radius is `1e-6`. Near the pole `asin(h/R)` is conditioned by `R/ρ`,
/// so the image's parallel lies off `√(R² − h²)` by up to `ε·R²/ρ`; the
/// sphere parallel's `Radius` reads the radius the image turns at
/// (`polar_read_radius`).
#[test]
fn a_zero_winding_parallel_near_the_pole_is_bounded() {
    let rows = [(0.6400109500871758, 1.0536712127723509e-8), (1.0, 1e-6)].map(|(r, rho)| {
        (
            format!("a parallel of radius {rho:e} on a sphere of radius {r}"),
            Surface::Sphere {
                center: Point3::origin(),
                radius: r,
                axis: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            circle(
                Point3::new(0.0, 0.0, (r * r - rho * rho).sqrt()),
                Vec3::unit_z(),
                3.3246718371187426e-11,
                Vec3::unit_x(),
            ),
            (0.0, TAU),
        )
    });
    for (what, chart, carrier, span) in rows {
        let (at_f64, at_iv, sup, noise) = zero_winding_near_the_pole(&chart, &carrier, span);
        assert!(
            at_f64 >= sup - noise,
            "{what}: envelope {at_f64:e} under the sampled sup {sup:e}"
        );
        assert!(
            at_iv >= sup - noise,
            "{what}: the Interval envelope {at_iv:e} under the sampled sup {sup:e}"
        );
    }
}

/// **A lifted copy of a circle whose turn is within the band is not
/// seen by check 4, and never reaches it**
/// (`work/pcert/a-lifted-copy-of-a-circle-whose-turn-is-within-the-band-certifies.md`):
/// the pinned coverage sweep's trial 2244 written down, a cone's
/// zero-winding rim of radius a tenth of the band's zero, set clear of
/// the apex, its image renamed a whole period over. The copy is the same
/// point set to within the tolerance, so `FidelityU`, the offset past half
/// a period levered at the rim's radius, reads `π·ρ`, decided Zero. At the
/// door the rim's whole span is a length within the band, so check 2
/// refuses it (`IntervalNotForward`) before check 4, at `f64` and at
/// `Interval`, the lifted copy and the exact image alike. A rim ten
/// thousand times wider (a turning one) passes check 2 and refuses its
/// copy on `FidelityU`, as C4 asks.
#[test]
fn a_lifted_copy_of_a_circle_whose_turn_is_within_the_band_never_reaches_check_4() {
    let rim = |rho: f64| {
        let half_angle = (rho / (1e3 * band().escalate())).atan();
        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle,
            u_ref: Vec3::unit_x(),
        };
        let carrier = circle(
            Point3::new(0.0, 0.0, rho / half_angle.tan()),
            Vec3::unit_z(),
            rho,
            Vec3::unit_x(),
        );
        let (derived, derivation) = derive_harmonic(
            &carrier,
            &cone,
            super::Placement::own(&carrier, &cone),
            band(),
        )
        .unwrap_or_else(|e| panic!("ρ {rho:e}: the rim derives: {e}"));
        (cone, carrier, derived, derivation)
    };
    let certify = |image: &Pcurve<f64>, carrier: &Curve3<f64>, cone: &Surface<f64>| {
        let f = Interval::from_f64;
        [
            PcurveCache::certify(image.clone(), 0.0, TAU, carrier, cone, band()).map(|_| ()),
            PcurveCache::certify(
                lift_harmonic(image),
                f(0.0),
                f(TAU),
                &carrier.map_scalar(f),
                &cone.map_scalar(f),
                band(),
            )
            .map(|_| ()),
        ]
    };
    let small = 0.1 * band().zero();
    let (cone, carrier, derived, derivation) = rim(small);
    assert!(
        matches!(derivation, Derivation::ConeRim { beta: 0, .. }),
        "a rim of radius {small:e} winds zero: {derivation:?}"
    );
    let lifted = derived.shift_branch(-1, Period::new(TAU));
    let terms = periodic_envelope(
        channels_of(&lifted),
        &carrier,
        carrier_harmonic(&carrier).unwrap(),
        &cone,
        (0.0, TAU),
        TAU,
        band(),
    )
    .unwrap();
    let fidelity = terms.0[EnvelopeTerm::FidelityU.slot()];
    assert!(
        matches!(
            decide("pcurve_envelope_term", Margin::of(fidelity), band()),
            Ok(Sign::Zero)
        ),
        "the lift of a rim of radius {small:e} reads {fidelity:e}, within the zero band"
    );
    for image in [&derived, &lifted] {
        for got in certify(image, &carrier, &cone) {
            assert!(
                matches!(got, Err(PcurveCertifyError::IntervalNotForward)),
                "a rim of radius {small:e} is refused at check 2: {got:?}"
            );
        }
    }
    let wide = 1e3 * band().zero();
    let (cone, carrier, derived, _) = rim(wide);
    let lifted = derived.shift_branch(-1, Period::new(TAU));
    for got in certify(&lifted, &carrier, &cone) {
        assert!(
            matches!(
                got,
                Err(PcurveCertifyError::ResidualExceeded {
                    check: PcurveCheck::EnvelopeTerm(EnvelopeTerm::FidelityU),
                    ..
                })
            ),
            "the lifted copy of a rim of radius {wide:e} refuses on FidelityU: {got:?}"
        );
    }
}

/// **The incidence terms read the carrier's own departure from its
/// chart**, written down on carriers the mint images: each moved off the
/// unit cylinder by less than the zero band, so its incidence read is on,
/// and the one term that reads that move carries it, to a few ulps of
/// the unit data, at `f64` and inside the `Interval` enclosure:
///
/// - a rim whose centre sits `d` off the axis: `Centre` is `d`;
/// - a ruling `d` outside the wall: `Radius` is `d`;
/// - a ruling leaning radially by `d` per metre over `[0, 1]`: `Line` is
///   `d` (its radial rate over the span's reach).
#[test]
fn the_incidence_terms_read_the_carriers_departure() {
    let d = 0.3 * band().zero();
    let cyl = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let rows = [
        (
            EnvelopeTerm::Centre,
            circle(
                Point3::new(d, 0.0, 0.5),
                Vec3::unit_z(),
                1.0,
                Vec3::unit_x(),
            ),
            (0.0, PI),
        ),
        (
            EnvelopeTerm::Radius,
            Curve3::Line {
                origin: Point3::new(1.0 + d, 0.0, 0.0),
                dir: Vec3::unit_z(),
            },
            (0.0, 1.0),
        ),
        (
            EnvelopeTerm::Line,
            Curve3::Line {
                origin: Point3::new(1.0, 0.0, 0.0),
                dir: Vec3::new(d, 0.0, 1.0),
            },
            (0.0, 1.0),
        ),
    ];
    let lift = Interval::from_f64;
    for (term, carrier, (t0, t1)) in rows {
        let reach = t0.abs().max(t1.abs());
        let (stored, _) = derive_harmonic(
            &carrier,
            &cyl,
            super::Placement::over((t0, t1), &carrier, &cyl),
            band(),
        )
        .unwrap_or_else(|e| panic!("{term:?}: the moved carrier images: {e}"));
        let at_f64 = periodic_envelope(
            channels_of(&stored),
            &carrier,
            carrier_harmonic(&carrier).unwrap(),
            &cyl,
            (t0, t1),
            reach,
            band(),
        )
        .unwrap();
        // The data are of unit size, so the move itself rounds by a few
        // ulps of 1.
        let ulps = 8.0 * f64::EPSILON;
        let read = at_f64.0[term.slot()];
        assert!(
            (read - d).abs() <= ulps,
            "{term:?} reads {read:e} for a move of {d:e}"
        );
        let carrier_i = carrier.map_scalar(lift);
        let at_iv = periodic_envelope(
            channels_of(&lift_harmonic(&stored)),
            &carrier_i,
            carrier_harmonic(&carrier_i).unwrap(),
            &cyl.map_scalar(lift),
            (lift(t0), lift(t1)),
            lift(reach),
            band(),
        )
        .unwrap();
        let enclosure = at_iv.0[term.slot()];
        assert!(
            enclosure.lo() <= d + ulps && enclosure.hi() >= d - ulps,
            "{term:?} at Interval encloses the move {d:e}: {enclosure:?}"
        );
    }
}

/// The radius of [`zero_winding_axis_circle`]'s circle: a quarter of the
/// band's zero at the run's ε, so its winding, metered at its own
/// radius, reads Zero at every ε row.
fn axis_rho() -> f64 {
    0.25 * band().zero()
}

/// The charts [`zero_winding_axis_circle`] is drawn on, with the arm each
/// selects: a cone narrow enough that a rim of [`axis_rho`] sits clear
/// of the apex's sliver (its height `ρ/tan α` a hundred radii up, two
/// and a half times the band's escalation), and
/// a near-horn torus whose inner equator is that rim.
fn axis_charts() -> [(Surface<f64>, &'static str); 2] {
    [
        (
            Surface::Cone {
                apex: Point3::origin(),
                axis: Vec3::unit_z(),
                half_angle: 0.01,
                u_ref: Vec3::unit_x(),
            },
            "cone rim, zero winding",
        ),
        (
            Surface::Torus {
                center: Point3::origin(),
                axis: Vec3::unit_z(),
                major_radius: 1.5 + axis_rho(),
                minor_radius: 1.5,
                u_ref: Vec3::unit_x(),
            },
            "torus parallel, zero winding",
        ),
    ]
}

/// **A corrupted stored image refuses at `Interval` as at `f64`**
/// (PR 3812's review R1, P3; static rows). Check 3 does not run over a
/// box, so fidelity is what refuses a stored image that is not the
/// carrier's; every corruption here refuses, at both scalars, and so
/// does every whole-period renaming, a lifted copy and no row (C4),
/// however many periods out; a sphere's involution twin, the same placed
/// image's other name, certifies at both.
#[test]
fn a_corrupted_stored_image_refuses_at_interval_as_at_f64() {
    let fixtures: Vec<(Surface<f64>, Curve3<f64>, &str)> = vec![
        (
            Surface::Cylinder {
                origin: Point3::origin(),
                axis: Vec3::unit_z(),
                radius: 0.5,
                u_ref: Vec3::unit_x(),
            },
            circle(
                Point3::new(0.0, 0.0, 0.3),
                Vec3::unit_z(),
                0.5,
                Vec3::unit_y(),
            ),
            "cylinder rim",
        ),
        (
            Surface::Sphere {
                center: Point3::origin(),
                radius: 1.0,
                axis: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            circle(Point3::origin(), Vec3::unit_y(), 1.0, Vec3::unit_x()),
            "sphere meridian",
        ),
        (
            Surface::Sphere {
                center: Point3::origin(),
                radius: 1.0,
                axis: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            circle(
                Point3::new(0.0, 0.0, 0.6),
                Vec3::unit_z(),
                0.8,
                Vec3::unit_x(),
            ),
            "sphere parallel",
        ),
        (
            Surface::Torus {
                center: Point3::origin(),
                axis: Vec3::unit_z(),
                major_radius: 2.0,
                minor_radius: 0.5,
                u_ref: Vec3::unit_x(),
            },
            circle(
                Point3::new(2.0, 0.0, 0.0),
                -Vec3::unit_y(),
                0.5,
                Vec3::unit_x(),
            ),
            "torus meridian",
        ),
        (
            Surface::Cone {
                apex: Point3::origin(),
                axis: Vec3::unit_z(),
                half_angle: 0.4,
                u_ref: Vec3::unit_x(),
            },
            circle(
                Point3::new(0.0, 0.0, -1.0),
                Vec3::unit_z(),
                0.4f64.tan(),
                -Vec3::unit_x(),
            ),
            "cone rim (lower nappe)",
        ),
    ];
    let (t0, t1) = (0.2, 2.9);
    // A move a thousand times the band's zero: definitely over it at
    // every ε row, and far inside every class decision's margin.
    let off = 1e3 * band().zero();
    let mut wrong = Vec::new();
    for (surface, carrier, name) in &fixtures {
        let sphere = name.starts_with("sphere");
        let angular_v = sphere || name.starts_with("torus");
        let (image, _) = derive_harmonic(
            carrier,
            surface,
            super::Placement::over((t0, t1), carrier, surface),
            band(),
        )
        .unwrap();
        let Pcurve::Harmonic {
            u: u_channel,
            v: v_channel,
        } = image
        else {
            unreachable!("the fixtures derive harmonic images")
        };
        let h = |du: f64, dv: f64| restated(&image, du, dv, [false, false]);
        // A slope negated: a winding's is structure, refused as the
        // windings' mismatch (`Err(true)`); a free channel's linear
        // coefficient is a number, refused as a residual (`Err(false)`).
        let flipped = |c: Channel<f64>, structural: bool| {
            let still = match c {
                Channel::Angle { winding, .. } => winding == 0,
                Channel::Free { l, .. } => l == 0.0,
            };
            match (still, structural) {
                (true, _) => Ok(()),
                (false, true) => Err(true),
                (false, false) => Err(false),
            }
        };
        // Off a sphere the twin map is no renaming: it negates a winding
        // polar channel (a torus meridian's), a structural refusal, and
        // reflects a free one, a residual.
        let off_sphere_twin = if v_channel.winding().is_some_and(|w| w != 0) {
            Err(true)
        } else {
            Err(false)
        };
        // `Ok(())` certifies; `Err(false)` refuses as a residual.
        let corruptions: Vec<(&str, Pcurve<f64>, Result<(), bool>)> = vec![
            ("exact", image.clone(), Ok(())),
            // A whole-period rename is a lifted copy, the row with a deck
            // element, and no row of its own (C4): refused as a residual.
            ("u + τ", h(TAU, 0.0), Err(false)),
            ("u − 3τ", h(-3.0 * TAU, 0.0), Err(false)),
            ("u + 7τ", h(7.0 * TAU, 0.0), Err(false)),
            ("u + π", h(PI, 0.0), Err(false)),
            ("u + τ/2 + 1e-9", h(PI + 1e-9, 0.0), Err(false)),
            ("u + 1e3·ε", h(off, 0.0), Err(false)),
            ("v + 1e3·ε", h(0.0, off), Err(false)),
            ("v + τ", h(0.0, TAU), Err(false)),
            (
                "slope u flipped",
                restated(&image, 0.0, 0.0, [true, false]),
                flipped(u_channel, true),
            ),
            // On a sphere meridian a polar winding negated is the
            // twin's, so the row reads as the twin with its azimuth a
            // half turn off, a residual.
            (
                "slope v flipped",
                restated(&image, 0.0, 0.0, [false, true]),
                flipped(v_channel, angular_v && !sphere),
            ),
            ("u + π, v kept (half a twin)", h(PI, 0.0), Err(false)),
            (
                "the involution twin",
                twin(&image, 0.0),
                if sphere { Ok(()) } else { off_sphere_twin },
            ),
            // The twin a whole period on is a lifted copy of the twin,
            // no row (C4).
            (
                "the twin, a period on",
                twin(&image, TAU),
                if sphere { Err(false) } else { off_sphere_twin },
            ),
        ];
        for (what, stored, want) in corruptions {
            let read = |r: Result<(), PcurveCertifyError>| match r {
                Ok(()) => Ok(()),
                // At f64 check 3's cross-check may refuse first.
                Err(
                    PcurveCertifyError::ResidualExceeded {
                        check:
                            PcurveCheck::EnvelopeTerm(_)
                            | PcurveCheck::Envelope
                            | PcurveCheck::MapResidual,
                        ..
                    }
                    | PcurveCertifyError::Escalated {
                        check:
                            PcurveCheck::EnvelopeTerm(_)
                            | PcurveCheck::Envelope
                            | PcurveCheck::MapResidual,
                        ..
                    },
                ) => Err(false),
                Err(PcurveCertifyError::WindingMismatch { .. }) => Err(true),
                Err(other) => panic!("{name} / {what}: an unexpected refusal {other:?}"),
            };
            let at_f64 = read(
                PcurveCache::certify(stored.clone(), t0, t1, carrier, surface, band()).map(|_| ()),
            );
            let lift = Interval::from_f64;
            let at_iv = read(
                PcurveCache::certify(
                    lift_harmonic(&stored),
                    lift(t0),
                    lift(t1),
                    &carrier.map_scalar(lift),
                    &surface.map_scalar(lift),
                    band(),
                )
                .map(|_| ()),
            );
            // At f64 check 3's cross-check runs before check 4, so a
            // winding mismatch may refuse first as the schedule's
            // residual.
            let f64_reads = at_f64 == want || (want == Err(true) && at_f64 == Err(false));
            if !f64_reads || at_iv != want {
                wrong.push(format!(
                    "{name} / {what}: want {want:?}, f64 {at_f64:?}, Interval {at_iv:?}"
                ));
            }
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// **The tables, composed**: on a carrier lying exactly on its chart,
/// the closed-form chart image of the derived pcurve
/// (`chart_image_harmonic ∘ derive_harmonic`) is the carrier's own
/// harmonic form, and every incidence term is round-off. A mutated
/// image table, derivation arm or incidence term breaks one of the two
/// on some draw.
#[test]
fn the_closed_form_tables_compose_back_to_the_carrier() {
    let mut s = fuzz::start("pcurve_cache::envelope_lemma::compose");
    for trial in 0..fuzz::scaled(400) {
        let class = CLASSES[s.below(CLASSES.len())];
        if !images_exactly(class) {
            continue;
        }
        let (surface, carrier) = on_chart(&mut s, class);
        let (t0, t1) = span(&mut s);
        let reach = t0.abs().max(t1.abs());
        let (derived, derivation) = derive_harmonic(
            &carrier,
            &surface,
            super::Placement::own(&carrier, &surface),
            band(),
        )
        .unwrap_or_else(|e| panic!("trial {trial} ({class:?}): an exact carrier refused: {e}"));
        let noise = 1e-12 * scale(&surface, &carrier, reach);
        let image = chart_image_harmonic(&derived, &surface)
            .unwrap_or_else(|| panic!("trial {trial} ({class:?}): no closed-form image"));
        let form = carrier_harmonic(&carrier).unwrap();
        let gaps = [
            (image.c - form.c).norm(),
            (image.a - form.a).norm(),
            (image.b - form.b).norm(),
            (image.l - form.l).norm(),
        ];
        assert!(
            gaps.iter().all(|&g| g <= noise),
            "trial {trial} ({class:?}, {derivation:?}): S∘P_d is not the carrier, coefficient \
             gaps {gaps:?} — {}",
            fuzz::replay()
        );
        let mut terms = EnvelopeTerms::new();
        incidence(form, &surface, derivation, reach, &mut terms);
        assert!(
            terms.total() <= noise,
            "trial {trial} ({class:?}, {derivation:?}): an exact carrier's incidence is {:e}, \
             terms {:?} — {}",
            terms.total(),
            terms.0,
            fuzz::replay()
        );
    }
}

/// A random tilted plane section of a random cone, and the cone.
fn cone_section(s: &mut fuzz::Rng) -> Option<(Surface<f64>, Curve3<f64>)> {
    let (axis, u_ref) = frame(s);
    let apex = Point3::new(s.range(-3.0, 3.0), s.range(-3.0, 3.0), s.range(-3.0, 3.0));
    let half_angle = s.range(0.15, 1.3);
    let cone = Surface::Cone {
        apex,
        axis,
        half_angle,
        u_ref,
    };
    // A tilt under the generator's keeps the section an ellipse.
    let tilt = s.range(0.02, 0.8) * (FRAC_PI_2 - half_angle);
    let side = axis.cross(u_ref);
    let normal = axis * tilt.cos() + (u_ref * s.range(-1.0, 1.0) + side).normalize() * tilt.sin();
    let normal = normal.normalize();
    let plane = Surface::Plane {
        origin: apex + axis * (s.range(0.5, 3.0) * sign(s)),
        normal,
        u_ref: unit(s).cross(normal).normalize(),
    };
    let reach = crate::Reach::Measured {
        at: apex,
        lever: 4.0,
    };
    match crate::intersect::plane_cone_section(&plane, &cone, &reach, band()) {
        Ok(crate::intersect::PlaneConeSection::TiltedEllipse(e)) => Some((cone, e)),
        _ => None,
    }
}

/// A random ring torus and one of its Villarceau circles, on either
/// family and traversal, from any start.
fn villarceau(s: &mut fuzz::Rng) -> (Surface<f64>, Curve3<f64>) {
    let (axis, u_ref) = frame(s);
    let center = Point3::new(s.range(-3.0, 3.0), s.range(-3.0, 3.0), s.range(-3.0, 3.0));
    let major = s.range(0.5, 3.0);
    let minor = major * s.range(0.1, 0.85);
    let phi = s.range(-PI, PI);
    let d = rad(axis, u_ref, phi);
    let tilt = (minor / major).asin();
    let lean = d.cross(axis) * tilt.cos() + axis * (sign(s) * tilt.sin());
    let psi = s.range(-PI, PI);
    (
        Surface::Torus {
            center,
            axis,
            major_radius: major,
            minor_radius: minor,
            u_ref,
        },
        circle(
            center + d * minor,
            d.cross(lean) * sign(s),
            major,
            d * psi.cos() + lean * psi.sin(),
        ),
    )
}

/// **The focal-section lemma, swept** (a counterexample search: the
/// seed varies). On a cone section and a Villarceau circle alike, for a
/// carrier moved off its chart, a stored image with any of its eight
/// numbers moved, and a chart whose frame is off its convention, the
/// envelope `focal_section_envelope` states is never below the sampled
/// sup of the stored image's residual, at `f64` or at `Interval`; and
/// an exact row's envelope is rounding-scale.
#[test]
fn the_focal_section_envelope_dominates_its_residual_on_both_instances() {
    let mut s = fuzz::start("pcurve_cache::envelope_lemma::focal_section");
    let mut checked = [0usize; 2];
    for trial in 0..fuzz::scaled(300) {
        let torus = s.below(2) == 0;
        let Some((chart, exact)) = (if torus {
            Some(villarceau(&mut s))
        } else {
            cone_section(&mut s)
        }) else {
            continue;
        };
        let Ok(derived) = chart_pcurve(&exact, &chart, band()) else {
            panic!(
                "trial {trial}: an exact {} does not derive — {}",
                if torus {
                    "Villarceau circle"
                } else {
                    "cone section"
                },
                fuzz::replay()
            );
        };
        let Pcurve::FocalSection(image) = derived else {
            panic!(
                "trial {trial}: {derived:?} is no focal section — {}",
                fuzz::replay()
            );
        };
        // 0: the carrier moves; 1: the image moves; 2: the frame moves;
        // 3: nothing moves.
        let mode = s.below(4);
        let delta = magnitude(&mut s);
        let carrier = if mode == 0 {
            perturbed(&mut s, &exact, delta)
        } else {
            exact.clone()
        };
        let surface = if mode == 2 {
            frame_moved(&mut s, &chart, delta)
        } else {
            chart
        };
        let pick = s.below(9);
        let mut slot = 0;
        let mut jitter = |v: f64| {
            slot += 1;
            if mode == 1 && (pick == 0 || pick == slot) {
                v + delta * s.range(-1.0, 1.0)
            } else {
                v
            }
        };
        let stored_image = FocalImage {
            u0: jitter(image.u0),
            t0: jitter(image.t0),
            v0: jitter(image.v0),
            va: jitter(image.va),
            vb: jitter(image.vb),
            vl: jitter(image.vl),
            beta: jitter(image.beta),
            sense: jitter(image.sense),
        };
        let stored = Pcurve::FocalSection(stored_image);
        let (t0, t1) = span(&mut s);
        let reach = t0.abs().max(t1.abs());
        let v_sup = stored.chart_box(t0, t1).v_reach();
        let form = carrier_harmonic(&carrier).unwrap();
        let envelope = focal_section_envelope(&stored_image, form, &surface, v_sup, reach);
        let sup = sampled_sup(&stored, &surface, &carrier, t0, t1);
        let noise = 64.0 * f64::EPSILON * scale(&surface, &carrier, reach);
        let context = || {
            format!(
                "trial {trial} (torus {torus}, mode {mode}, δ = {delta:e}, pick {pick}, \
                 span [{t0}, {t1}])"
            )
        };
        assert!(
            envelope >= sup - noise,
            "{}: envelope {envelope:e} under the sampled sup {sup:e} — {}",
            context(),
            fuzz::replay()
        );
        if mode == 3 {
            assert!(
                envelope <= noise,
                "{}: an exact row costs {envelope:e} — {}",
                context(),
                fuzz::replay()
            );
        }
        let lift = Interval::from_f64;
        let lifted_image = FocalImage {
            u0: lift(stored_image.u0),
            t0: lift(stored_image.t0),
            v0: lift(stored_image.v0),
            va: lift(stored_image.va),
            vb: lift(stored_image.vb),
            vl: lift(stored_image.vl),
            beta: lift(stored_image.beta),
            sense: lift(stored_image.sense),
        };
        let lifted = Pcurve::FocalSection(lifted_image);
        let at_iv = focal_section_envelope(
            &lifted_image,
            carrier_harmonic(&carrier.map_scalar(lift)).unwrap(),
            &surface.map_scalar(lift),
            lifted.chart_box(lift(t0), lift(t1)).v_reach(),
            lift(reach),
        )
        .hi();
        assert!(
            envelope.is_finite() && at_iv >= sup - noise,
            "{}: the Interval envelope {at_iv:e} is under the sampled sup {sup:e} — {}",
            context(),
            fuzz::replay()
        );
        checked[usize::from(torus)] += 1;
    }
    println!(
        "[fuzz] envelope_lemma::focal_section: {} cone and {} torus rows checked",
        checked[0], checked[1]
    );
}

/// **The meridional-phase remainder is load-bearing** (a static row).
/// A Villarceau circle's image with `β` moved off `−r/(R + √(R² − r²))`
/// splits as `H + (r + R·e)·cos E·ρ̂(u)` with `H` an exact ellipse; offered
/// against THAT ellipse as its carrier, every coefficient difference is
/// rounding and the whole residual is the remainder, which reaches
/// `|r + R·e|` at the vertex. An envelope without the phase term reads
/// rounding there.
#[test]
fn the_phase_remainder_carries_a_carrier_that_is_the_harmonic_part() {
    let (big, minor) = (2.0_f64, 0.7_f64);
    let (axis, u_ref) = (Vec3::unit_z(), Vec3::unit_x());
    let surface = Surface::Torus {
        center: Point3::origin(),
        axis,
        major_radius: big,
        minor_radius: minor,
        u_ref,
    };
    let tilt = (minor / big).asin();
    let lean = Vec3::unit_y() * tilt.cos() + axis * tilt.sin();
    let exact = circle(
        Point3::new(minor, 0.0, 0.0),
        Vec3::unit_x().cross(lean),
        big,
        u_ref,
    );
    let derived = chart_pcurve(&exact, &surface, band()).unwrap();
    let Pcurve::FocalSection(image) = derived else {
        panic!("{derived:?} is no focal section")
    };
    assert_eq!(image.t0, 0.0, "the vertex is the carrier's start");
    let beta = image.beta * 0.999;
    let stored_image = FocalImage { beta, ..image };
    let stored = Pcurve::FocalSection(stored_image);
    // `H` at the stored `β`: `O − R·e·d0 + cos t·R·d0 + sin t·(R·q·d1 + vl·r·n̂)`.
    let bb = beta * beta;
    let (e, q) = (2.0 * beta / (1.0 + bb), (1.0 - bb) / (1.0 + bb));
    let d0 = rad(axis, u_ref, image.u0);
    let d1 = axis.cross(d0) * image.sense;
    let (x, y) = (d0 * big, d1 * (big * q) + axis * (image.vl * minor));
    let carrier = Curve3::Ellipse {
        center: Point3::origin() - d0 * (big * e),
        axis: x.cross(y).normalize(),
        semi_u: x.norm(),
        semi_v: y.norm(),
        u_ref: x.normalize(),
    };
    let (t0, t1) = (-0.5, 0.5);
    let envelope = focal_section_envelope(
        &stored_image,
        carrier_harmonic(&carrier).unwrap(),
        &surface,
        stored.chart_box(t0, t1).v_reach(),
        0.5,
    );
    let sup = sampled_sup(&stored, &surface, &carrier, t0, t1);
    let phase = (minor + big * e).abs();
    assert!(
        sup > 0.9 * phase,
        "the residual is the remainder: {sup:e} against {phase:e}"
    );
    let noise = 64.0 * f64::EPSILON * scale(&surface, &carrier, 0.5);
    assert!(
        envelope >= sup - noise,
        "envelope {envelope:e} under the sampled sup {sup:e}"
    );
}

/// **A Villarceau circle moved off its class is refused typed, and never
/// read off the torus while it is within the band** (a counterexample
/// search, the seed varies; the reviewers' probe on PR 4227). Each draw
/// moves a Villarceau circle of a random torus by a size from the band's
/// three regimes along one of six directions — the centre's radial, the
/// axis, the equator's tangent, a tilt about the centre's radial, the
/// radius, a turn about the vertex's tangent — and the torus arm's
/// answer is checked: an image it mints and certifies has an envelope
/// over the dense residual, and `CarrierOffChart` is never the answer
/// for a circle whose dense distance from the torus is within `ε`.
#[test]
fn a_moved_villarceau_circle_is_refused_typed_and_never_off_within_the_band() {
    let mut s = fuzz::start("pcurve_cache::envelope_lemma::moved_villarceau");
    let b = band();
    let mut answers = [0usize; 4];
    for trial in 0..fuzz::scaled(300) {
        let (surface, carrier) = villarceau(&mut s);
        let Surface::Torus {
            center: tc,
            axis,
            major_radius,
            minor_radius,
            ..
        } = surface
        else {
            unreachable!("villarceau draws a torus")
        };
        let Curve3::Circle {
            center,
            axis: n,
            radius,
            u_ref,
        } = carrier
        else {
            unreachable!("villarceau draws a circle")
        };
        let delta = match s.below(3) {
            0 => b.zero() * s.range(0.1, 0.9),
            1 => b.escalate() * s.range(1.5, 30.0),
            _ => 10f64.powf(s.range(-7.0, -3.0)),
        };
        let which = s.below(6);
        let w = center - tc;
        let d = (w - axis * w.dot(axis)).normalize();
        let moved = match which {
            0 => circle(center + d * delta, n, radius, u_ref),
            1 => circle(center + axis * delta, n, radius, u_ref),
            2 => circle(center + axis.cross(d) * delta, n, radius, u_ref),
            3 => {
                let n2 = (n + n.cross(d) * delta).normalize();
                circle(center, n2, radius, (u_ref - n2 * u_ref.dot(n2)).normalize())
            }
            4 => circle(center, n, radius + delta, u_ref),
            _ => {
                let tangent = n.cross(d);
                let n2 = (n + d * delta).normalize();
                let toward = n2.cross(tangent).normalize();
                let c2 = center + d * radius - toward * (radius * d.dot(toward).signum());
                circle(c2, n2, radius, (u_ref - n2 * u_ref.dot(n2)).normalize())
            }
        };
        let dense = (0..2048)
            .map(|i| {
                let q = moved.eval(f64::from(i) * TAU / 2048.0) - tc;
                let h = q.dot(axis);
                let rho = (q - axis * h).norm();
                ((rho - major_radius).hypot(h) - minor_radius).abs()
            })
            .fold(0.0, f64::max);
        let context =
            || format!("trial {trial}, move {which} by {delta:e}, dense distance {dense:e}");
        match chart_pcurve(&moved, &surface, b) {
            Ok(image) => {
                answers[0] += 1;
                if let Ok(cache) =
                    PcurveCache::certify(image.clone(), 0.0, TAU, &moved, &surface, b)
                {
                    let sup = sampled_sup(&image, &surface, &moved, 0.0, TAU);
                    let noise = 64.0 * f64::EPSILON * scale(&surface, &moved, TAU);
                    assert!(
                        cache.certificate().envelope >= sup - noise,
                        "{}: envelope {:e} under the residual {sup:e} — {}",
                        context(),
                        cache.certificate().envelope,
                        fuzz::replay()
                    );
                }
            }
            Err(PcurveCertifyError::CarrierOffChart { .. }) => {
                answers[1] += 1;
                assert!(
                    dense > b.zero(),
                    "{}: within the band, read off the torus — {}",
                    context(),
                    fuzz::replay()
                );
            }
            // On the torus but none of its circles: its general lane is
            // not written yet (D3).
            Err(PcurveCertifyError::NoGeneralLane {
                gap: crate::LaneGap::Unwritten,
                ..
            }) => answers[2] += 1,
            Err(PcurveCertifyError::Escalated { .. }) => answers[3] += 1,
            Err(other) => panic!(
                "{}: refused untyped for this arm: {other} — {}",
                context(),
                fuzz::replay()
            ),
        }
    }
    println!(
        "[fuzz] envelope_lemma::moved_villarceau: {} minted, {} off, {} no lane yet, {} escalated",
        answers[0], answers[1], answers[2], answers[3]
    );
}

/// **Each envelope term the sweep leaves slack for is load-bearing in a
/// row of its own** (static rows; the reviewers' probes on PR 4227):
/// against a carrier equal to the lemma's own harmonic part `H` (or the
/// exact carrier), the term is the whole residual, so an envelope that
/// drops or under-states it reads under the sampled sup.
mod focal_section_term_rows {
    use super::super::Harmonic3;
    use super::*;

    fn sup_against(p: &Pcurve<f64>, s: &Surface<f64>, h: Harmonic3<f64>, t0: f64, t1: f64) -> f64 {
        (0..=8192)
            .map(|k| {
                let t = t0 + (t1 - t0) * (f64::from(k) / 8192.0);
                let q = p.eval(t);
                let c = h.c + h.a * t.cos() + h.b * t.sin() + h.l * t;
                s.eval(q.x, q.y).distance(c)
            })
            .fold(0.0, f64::max)
    }

    fn check(name: &str, p: &Pcurve<f64>, s: &Surface<f64>, h: Harmonic3<f64>, t0: f64, t1: f64) {
        let Pcurve::FocalSection(image) = *p else {
            unreachable!("the rows store focal sections")
        };
        let v_sup = p.chart_box(t0, t1).v_reach();
        let env = focal_section_envelope(&image, h, s, v_sup, t0.abs().max(t1.abs()));
        let sup = sup_against(p, s, h, t0, t1);
        assert!(
            env >= sup - 1e-13,
            "{name}: envelope {env:e} under sampled sup {sup:e}"
        );
    }

    fn torus(big: f64, r: f64) -> Surface<f64> {
        Surface::Torus {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            major_radius: big,
            minor_radius: r,
            u_ref: Vec3::unit_x(),
        }
    }

    /// The lemma's torus `H` at the stored fields (its docs' formula).
    fn torus_h(i: &FocalImage<f64>, big: f64, r: f64) -> Harmonic3<f64> {
        let n = Vec3::unit_z();
        let d0 = rad(n, Vec3::unit_x(), i.u0);
        let d1 = n.cross(d0) * i.sense;
        let bb = i.beta * i.beta;
        let (e, q) = (2.0 * i.beta / (1.0 + bb), (1.0 - bb) / (1.0 + bb));
        let v_r = i.v0 + i.vl * i.t0;
        let x = d0 * big + n * (r * v_r.sin());
        let y = d1 * (big * q) + n * (r * i.vl * v_r.cos());
        let (st, ct) = i.t0.sin_cos();
        Harmonic3 {
            c: Point3::origin() - d0 * (big * e),
            a: x * ct - y * st,
            b: x * st + y * ct,
            l: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    fn image_of(p: &Pcurve<f64>) -> FocalImage<f64> {
        let Pcurve::FocalSection(image) = *p else {
            unreachable!("the rows store focal sections")
        };
        image
    }

    fn villarceau_fields(big: f64, r: f64, t0: f64) -> Pcurve<f64> {
        Pcurve::FocalSection(FocalImage {
            u0: 0.0,
            t0,
            v0: -t0,
            va: 0.0,
            vb: 0.0,
            vl: 1.0,
            beta: -r / (big + (big * big - r * r).sqrt()),
            sense: 1.0,
        })
    }

    fn with(p: &Pcurve<f64>, f: impl Fn(&mut FocalImage<f64>)) -> Pcurve<f64> {
        let mut i = image_of(p);
        f(&mut i);
        Pcurve::FocalSection(i)
    }

    /// The Villarceau circle of `torus(big, r)` centred `r` along `+x`,
    /// started `t0` before its outer-equator vertex.
    fn villarceau_carrier(big: f64, r: f64, t0: f64) -> Curve3<f64> {
        let tilt = (r / big).asin();
        let lean = Vec3::unit_y() * tilt.cos() + Vec3::unit_z() * tilt.sin();
        let n = Vec3::unit_x().cross(lean);
        let d = Vec3::unit_x();
        circle(
            Point3::new(r, 0.0, 0.0),
            n,
            big,
            d * (-t0).cos() + n.cross(d) * (-t0).sin(),
        )
    }

    /// `r·|sin v_r|` half of the meridional-phase remainder.
    #[test]
    fn phase_sine_half() {
        let (big, r) = (2.0, 0.7);
        let s = torus(big, r);
        let p = with(&villarceau_fields(big, r, 0.0), |i| i.v0 += 0.05);
        let h = torus_h(&image_of(&p), big, r);
        check("phase_sine_half", &p, &s, h, -1.5, 1.5);
    }

    /// The `+1` on the tube drift (H reads `vl` for its sign).
    #[test]
    fn tube_drift_plus_one() {
        let (big, r) = (2.0, 0.7);
        let s = torus(big, r);
        let p = with(&villarceau_fields(big, r, 0.0), |i| i.vl = 1.0 + 1e-3);
        let h = torus_h(&image_of(&p), big, r);
        check(
            "tube_drift_plus_one",
            &p,
            &s,
            h,
            FRAC_PI_2 - 0.01,
            FRAC_PI_2 + 0.01,
        );
    }

    /// `E_max = reach + |t0|`: a vertex far from a short span.
    #[test]
    fn e_max_reads_t0() {
        let (big, r) = (2.0, 0.7);
        let s = torus(big, r);
        let carrier = villarceau_carrier(big, r, 3.0);
        let derived = chart_pcurve(&carrier, &s, band()).unwrap();
        let t0 = image_of(&derived).t0;
        assert!((t0 - 3.0).abs() < 1e-12, "vertex at {t0}");
        let p = with(&derived, |i| {
            i.vl *= 1.0 + 1e-3;
            i.v0 = -i.vl * i.t0;
        });
        check(
            "e_max_reads_t0",
            &p,
            &s,
            carrier_harmonic(&carrier).unwrap(),
            -0.05,
            0.05,
        );
    }

    /// The torus sense-drift lever is `R + r`, not `R`.
    #[test]
    fn torus_sense_lever_is_outer_radius() {
        let (big, r) = (1.0, 0.8);
        let s = torus(big, r);
        let carrier = villarceau_carrier(big, r, 0.0);
        let derived = chart_pcurve(&carrier, &s, band()).unwrap();
        let p = with(&derived, |i| i.sense *= 1.0 + 1e-4);
        let k = 3.0 * TAU;
        check(
            "torus_sense_lever_is_outer_radius",
            &p,
            &s,
            carrier_harmonic(&carrier).unwrap(),
            k - 0.1,
            k + 0.1,
        );
    }

    /// The cone remainder's `e·v0·sin t0` (a stored `t0 ≠ 0`).
    #[test]
    fn cone_remainder_reads_sin_t0() {
        let (alpha, t0) = (0.5_f64, 0.5_f64);
        let (sin_a, cos_a) = alpha.sin_cos();
        let n = Vec3::unit_z();
        let s = Surface::Cone {
            apex: Point3::origin(),
            axis: n,
            half_angle: alpha,
            u_ref: Vec3::unit_x(),
        };
        let (beta, v0) = (0.3_f64, 2.0_f64);
        let bb = beta * beta;
        let (e, q) = (2.0 * beta / (1.0 + bb), (1.0 - bb) / (1.0 + bb));
        let (st, ct) = t0.sin_cos();
        let (va, vb) = (-e * v0 * ct, 0.0);
        let p = Pcurve::FocalSection(FocalImage {
            u0: 0.0,
            t0,
            v0,
            va,
            vb,
            vl: 0.0,
            beta,
            sense: 1.0,
        });
        let d0 = Vec3::unit_x();
        let d1 = n.cross(d0);
        let lever = sin_a * v0;
        let (x, y) = (d0 * lever, d1 * (lever * q));
        let h = Harmonic3 {
            c: Point3::origin() + n * (cos_a * v0) - d0 * (lever * e),
            a: n * (cos_a * va) + x * ct - y * st,
            b: n * (cos_a * vb) + x * st + y * ct,
            l: Vec3::new(0.0, 0.0, 0.0),
        };
        check("cone_remainder_reads_sin_t0", &p, &s, h, 1.0, 2.0);
    }

    /// The sense drift's `π` on a cone: a near-parabolic section
    /// (`β = 0.95`) whose short span sits where the true anomaly runs
    /// furthest ahead of the eccentric one, with `sense` off by `1e-6`.
    /// There the azimuth's drift is `|ν|`, well past `E_max + 1`.
    #[test]
    fn the_sense_drift_reads_pi_on_a_cone() {
        let alpha = 0.5_f64;
        let n = Vec3::unit_z();
        let s = Surface::Cone {
            apex: Point3::origin(),
            axis: n,
            half_angle: alpha,
            u_ref: Vec3::unit_x(),
        };
        let (beta, v0) = (0.95_f64, 2.0_f64);
        let bb = beta * beta;
        let e = 2.0 * beta / (1.0 + bb);
        let exact = Pcurve::FocalSection(FocalImage {
            u0: 0.0,
            t0: 0.0,
            v0,
            va: -e * v0,
            vb: 0.0,
            vl: 0.0,
            beta,
            sense: 1.0,
        });
        // The carrier is the exact image's own map, an ellipse: its
        // harmonic form read off three of its points.
        let at = |t: f64| {
            let q = exact.eval(t);
            s.eval(q.x, q.y)
        };
        let (p0, p1, p2) = (at(0.0), at(FRAC_PI_2), at(PI));
        let c = Point3::origin() + ((p0 - Point3::origin()) + (p2 - Point3::origin())) * 0.5;
        let h = Harmonic3 {
            c,
            a: (p0 - p2) * 0.5,
            b: p1 - c,
            l: Vec3::new(0.0, 0.0, 0.0),
        };
        let p = with(&exact, |i| i.sense = 1.0 + 1e-6);
        check(
            "the_sense_drift_reads_pi_on_a_cone",
            &p,
            &s,
            h,
            -1.57,
            -1.56,
        );
    }
}
