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
    "crates/geom/src/surfaces.rs",
    "crates/geom/src/curves.rs",
    "crates/geom/src/azimuth.rs",
];

use std::collections::BTreeSet;

use super::{
    ChartWindings, ChartWindow, Derivation, EnvelopeTerm, EnvelopeTerms, Pcurve, PcurveCache,
    PcurveCertifyError, PcurveCheck, Winding, carrier_harmonic, chart_image_harmonic,
    derive_harmonic, incidence, orthonormal_chart, periodic_envelope,
};
use geom::{Curve3, Surface};
use geom_core::predicate::Band;
use geom_core::tolerance::Tol;
use geom_core::{Bounds, Interval, Point2, Point3, Real, Vec2, Vec3};
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
/// derivation selects. Two classes lie on the chart but are not the
/// class their derivation selects (`SmallCircle`,
/// `CylinderMeridianCircle`: their radial motion is the admitted
/// drift), two do not lie on it at all (`AxisCircle`,
/// `CylinderLeaningLine`: they are there to reach the zero-winding
/// arms), and the near-pole meridian's derivation reads its meridian
/// plane off an `a_r` a few ulps long, so its image is off by that
/// conditioning.
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
                    major: radius / theta.cos(),
                    minor: radius,
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
            let (chart, radius, v0) = cylinder(s);
            let rho = band().escalate() * s.range(0.02, 0.98);
            let start = r * s.range(-1.0, 1.0) + axis * s.range(-1.0, 1.0);
            (
                chart,
                circle(
                    at + r * radius + axis * v0,
                    tang * sign(s),
                    rho,
                    if start.norm() > 0.1 {
                        start.normalize()
                    } else {
                        r
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
            // A small circle about a point of the chart, in a random
            // plane: its radial amplitude is real but its winding, an
            // area metered at the chart's lever, reads Zero or in band.
            let rho = band().zero() * 10f64.powf(s.range(1.5, 4.4));
            let (chart, point) = match s.below(4) {
                0 => {
                    let (chart, radius, v0) = cylinder(s);
                    (chart, at + r * radius + axis * v0)
                }
                1 => {
                    let (chart, half_angle) = cone(s);
                    let v = sign(s) * s.range(0.2, 3.0);
                    (
                        chart,
                        at + (axis * half_angle.cos() + r * half_angle.sin()) * v,
                    )
                }
                2 => {
                    let (chart, radius) = sphere(s);
                    let v = s.range(-1.3, 1.3);
                    (chart, at + (r * v.cos() + axis * v.sin()) * radius)
                }
                _ => {
                    let (chart, major, minor) = torus(s);
                    let v = s.range(-PI, PI);
                    (
                        chart,
                        at + r * (major + minor * v.cos()) + axis * (minor * v.sin()),
                    )
                }
            };
            let (n, e) = frame(s);
            (chart, circle(point, n, rho, e))
        }
        Class::AxisCircle => {
            // From inside the band (the winding, metered at the circle's
            // own radius on a rim or a torus parallel, reads Zero) to
            // well clear of it; the plane tilted off ⊥ by an angle the
            // axial decision still reads Zero at the radius drawn, so a
            // turning class's two radial coefficients disagree
            // (`Orientation`).
            let rho = band().zero() * 10f64.powf(s.range(-1.5, 4.0));
            let theta = (band().zero() / rho).min(1.0) * s.range(0.0, 0.9);
            let normal = (axis + tang * theta).normalize();
            let chart = match s.below(4) {
                0 => cylinder(s).0,
                1 => cone(s).0,
                2 => sphere(s).0,
                _ => torus(s).0,
            };
            (
                chart,
                circle(
                    at + axis * s.range(-3.0, 3.0),
                    normal * sign(s),
                    rho,
                    (r - normal * r.dot(normal)).normalize(),
                ),
            )
        }
        Class::CylinderLeaningLine => {
            let (chart, radius, v0) = cylinder(s);
            (
                chart,
                Curve3::Line {
                    origin: at + r * radius + axis * v0,
                    dir: axis * s.range(-3.0, 3.0) + r * s.range(-1.0, 1.0),
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
            // its two radial coefficients disagree (what `Orientation`
            // meters on a turning class).
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
                    major,
                    minor,
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
            major,
            minor,
            u_ref,
        } => {
            let axis = (axis + nudge(s, 3)).normalize();
            let u_ref = (u_ref - axis * u_ref.dot(axis)).normalize();
            Curve3::Ellipse {
                center: center + nudge(s, 1),
                axis,
                major: scaled(s, major),
                minor: scaled(s, minor),
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

/// The chart's windings the derivation decided, as check 1 reads them
/// off a stored image.
fn windings_of(derivation: Derivation) -> ChartWindings {
    let (u, v) = match derivation {
        Derivation::Plane => (Winding::Zero, None),
        Derivation::CylinderMoving { beta } | Derivation::ConeRim { beta, .. } => (beta, None),
        Derivation::CylinderMeridian | Derivation::ConeRuling { .. } => (Winding::Zero, None),
        Derivation::SphereParallel { beta } | Derivation::TorusParallel { beta } => {
            (beta, Some(Winding::Zero))
        }
        Derivation::SphereMeridian { sigma, .. } | Derivation::TorusMeridian { sigma } => {
            (Winding::Zero, Some(sigma))
        }
        Derivation::ConeSection => {
            unreachable!("windings_of: the sweep draws no ellipse on a cone")
        }
    };
    ChartWindings { u, v }
}

/// The arm a derivation selected, as the coverage census names it: the
/// variant, split where the arm's own terms differ (a moving class's
/// zero winding, a ruling's anchor, a meridian's pole start).
fn arm(derivation: Derivation) -> &'static str {
    let moving = |beta: Winding, turning: &'static str, still: &'static str| match beta {
        Winding::Zero => still,
        Winding::Pos | Winding::Neg => turning,
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
        Derivation::Plane | Derivation::ConeSection => unreachable!("the sweep draws neither"),
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
///
/// Left out too: `Orientation` on a turning sphere parallel. Its axial
/// gate reads the plane's tilt at the circle's radius `ρ` (`θ ≲ ε/ρ`),
/// and its winding reads the circle's area at the sphere's radius
/// (`ρ ≳ √(εR)`), so the orientation defect `ρ·(1 − cos θ) ≲ ε²/ρ` is
/// under the rounding of every row that reaches the arm.
fn arm_terms() -> Vec<(&'static str, Vec<EnvelopeTerm>)> {
    use EnvelopeTerm::{Centre, Drift, Line, Orientation, Radius, Tilt};
    let cases: [(&str, &[EnvelopeTerm]); 14] = [
        ("cylinder moving", &[Centre, Radius, Orientation]),
        (
            "cylinder moving, zero winding",
            &[Line, Centre, Radius, Drift],
        ),
        ("cylinder meridian", &[Line, Radius, Drift]),
        ("cone ruling, by direction", &[Radius, Line]),
        ("cone ruling, anchored", &[Radius, Line]),
        ("cone rim", &[Centre, Tilt, Radius, Orientation]),
        ("cone rim, zero winding", &[Centre, Tilt, Radius, Drift]),
        ("sphere parallel", &[Centre, Tilt, Radius]),
        (
            "sphere parallel, zero winding",
            &[Centre, Tilt, Radius, Drift],
        ),
        ("sphere meridian", &[Centre, Radius, Orientation]),
        (
            "sphere meridian, pole start",
            &[Centre, Radius, Drift, Orientation],
        ),
        ("torus parallel", &[Centre, Tilt, Radius, Orientation]),
        (
            "torus parallel, zero winding",
            &[Centre, Tilt, Radius, Drift],
        ),
        ("torus meridian", &[Centre, Radius, Tilt, Orientation]),
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
    let Pcurve::Harmonic { p0, pa, pb, pl } = *p else {
        unreachable!("the sweep stores harmonic images")
    };
    let f = Interval::from_f64;
    let v = |q: Vec2<f64>| Vec2::new(f(q.x), f(q.y));
    Pcurve::Harmonic {
        p0: Point2::new(f(p0.x), f(p0.y)),
        pa: v(pa),
        pb: v(pb),
        pl: v(pl),
    }
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

/// Draw one row of `class` and assert the lemma on it: the envelope at
/// `f64` is not under the sampled sup, and the envelope at `Interval`
/// (over the row's lifted inputs) is not under it either; a renamed
/// exact image costs nothing.
fn row(s: &mut fuzz::Rng, class: Class, trial: usize) -> Option<Row> {
    let (chart, exact) = on_chart(s, class);
    // 0: the carrier moves; 1: the image moves; 2: both; 3: neither,
    // the image only renamed (a whole period, the twin); 4: the chart's
    // frame moves.
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
    // The stored image is what the mint derives on the chart as stored.
    let (derived, _) = derive_harmonic(&carrier, &surface, band()).ok()?;
    let (_, derivation) = derive_harmonic(&carrier, &orthonormal_chart(&surface), band()).ok()?;
    // An ellipse a move left on a cone images as a cone section, which
    // its own lane certifies.
    let Pcurve::Harmonic { p0, pa, pb, pl } = derived else {
        return None;
    };
    let drift = if matches!(mode, 1 | 2) {
        magnitude(s)
    } else {
        0.0
    };
    let pick = s.below(9);
    let mut slot = 0;
    let mut jitter = |v: f64| {
        slot += 1;
        if pick == 0 || pick == slot {
            v + drift * s.range(-1.0, 1.0)
        } else {
            v
        }
    };
    let mut stored = Pcurve::Harmonic {
        p0: Point2::new(jitter(p0.x), jitter(p0.y)),
        pa: Vec2::new(jitter(pa.x), jitter(pa.y)),
        pb: Vec2::new(jitter(pb.x), jitter(pb.y)),
        pl: Vec2::new(jitter(pl.x), jitter(pl.y)),
    };
    let k = s.range(-1.5, 1.5).round();
    stored = stored.shift_branch(k, TAU);
    if matches!(surface, Surface::Sphere { .. }) && s.below(2) == 0 {
        stored = stored.map_affine(
            |p| Point2::new(p.x + PI, PI - p.y),
            |v| Vec2::new(v.x, -v.y),
        );
    }
    let form = carrier_harmonic(&carrier).unwrap();
    let context = || {
        format!(
            "trial {trial} ({class:?}, {derivation:?}, mode {mode}, δ = {delta:e}, \
             drift = {drift:e}, k = {k})"
        )
    };
    let terms = periodic_envelope(&stored, &carrier, form, &surface, (t0, t1), reach, band())
        .unwrap_or_else(|e| panic!("{}: the envelope refused at f64: {e}", context()));
    let envelope = terms.total();
    // A NaN envelope decides `Invalid`, which refuses (a circle past a
    // sphere's pole has no polar angle): sound, and no bound to compare.
    if envelope.is_nan() {
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
    if mode == 3 && images_exactly(class) {
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
        &lift_harmonic(&stored),
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
    Some(Row {
        arm: arm(derivation),
        terms: terms.0,
        noise,
    })
}

/// **The lemma, swept** (a counterexample search: the seed varies). For
/// a carrier moved off its chart, a stored image moved off the one
/// derived from it (by a whole period, or onto a sphere's involution
/// twin, as well as by drift), and a chart whose frame is off its
/// convention, check 4's envelope is never below the sampled sup of the
/// stored image's residual, at `f64` or at `Interval`. Each row moves
/// one of the four alone, or two, so a term is met with nothing else to
/// cover for it. The failure prints the terms.
#[test]
fn the_envelope_dominates_the_stored_images_residual_on_every_periodic_arm() {
    let mut s = fuzz::start("pcurve_cache::envelope_lemma::dominates");
    let mut checked = 0usize;
    for trial in 0..fuzz::scaled(700) {
        let class = CLASSES[s.below(CLASSES.len())];
        checked += usize::from(row(&mut s, class, trial).is_some());
    }
    println!("[fuzz] envelope_lemma::dominates: {checked} rows checked");
}

/// **Every arm's every term carries some row** (a coverage claim: the
/// seed is pinned, so this row cannot pass on a lucky draw or fail on
/// an unlucky one). On a pinned stream of rows of every class, each
/// row's dominance is asserted (as in the sweep above), and for every
/// arm `derive_harmonic` selects, each of its terms is over the row's
/// noise in at least one row. So a table that drops or under-states any
/// term is met by a row where that term is load-bearing, at `f64` and
/// at `Interval`, whatever seed the counterexample search draws.
#[test]
fn every_arm_and_term_is_load_bearing_in_a_pinned_sweep() {
    let mut s = fuzz::pinned(
        "pcurve_cache::envelope_lemma::coverage",
        0x5ca1_ab1e_c0ff_ee42,
    );
    let mut seen: BTreeSet<(&'static str, usize)> = BTreeSet::new();
    let mut trial = 0usize;
    for class in CLASSES {
        for _ in 0..if class == Class::AxisCircle { 640 } else { 160 } {
            trial += 1;
            let Some(row) = row(&mut s, class, trial) else {
                continue;
            };
            for term in EnvelopeTerm::ALL {
                if row.terms[term.slot()] > row.noise {
                    seen.insert((row.arm, term.slot()));
                }
            }
        }
    }
    // On an arm only circles reach, `Orientation` is second order in an
    // in-band tilt of the circle's plane (`ρ·(1 − cos θ)` with `θ ≲ ε/ρ`,
    // so `≲ ε²/ρ`): at a band this narrow that is under the f64 rounding
    // every row is read to, at any radius the arm admits. The sphere
    // parallel's is under it at every ε row (`arm_terms`).
    let second_order = |arm: &str, t: EnvelopeTerm| {
        band().zero() < 1e-10
            && t == EnvelopeTerm::Orientation
            && matches!(
                arm,
                "cone rim" | "sphere meridian, pole start" | "torus parallel"
            )
    };
    let missing: Vec<(&str, EnvelopeTerm)> = arm_terms()
        .into_iter()
        .flat_map(|(arm, terms)| terms.into_iter().map(move |t| (arm, t)))
        .filter(|&(arm, t)| !seen.contains(&(arm, t.slot())) && !second_order(arm, t))
        .collect();
    assert!(
        missing.is_empty(),
        "no row carried these (arm, term) pairs over the noise: {missing:?}"
    );
}

/// **A corrupted stored image refuses at `Interval` as at `f64`**
/// (PR 3812's review R1, P3; static rows). Check 3 does not run over a
/// box, so fidelity is what refuses a stored image that is not the
/// carrier's; every corruption here refuses, at both scalars, and every
/// legitimate renaming (a whole period, a sphere's involution twin)
/// certifies at both. A branch past `MAX_BRANCH_PERIODS` refuses by
/// type.
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
    let window = |w: f64| ChartWindow {
        u_min: -w,
        u_max: w,
        v_min: -w,
        v_max: w,
    };
    let (t0, t1) = (0.2, 2.9);
    // A move a thousand times the band's zero: definitely over it at
    // every ε row, and far inside every class decision's margin.
    let off = 1e3 * band().zero();
    let mut wrong = Vec::new();
    for (surface, carrier, name) in &fixtures {
        let sphere = name.starts_with("sphere");
        let angular_v = sphere || name.starts_with("torus");
        let (image, _) = derive_harmonic(carrier, surface, band()).unwrap();
        let Pcurve::Harmonic { p0, pa, pb, pl } = image else {
            unreachable!()
        };
        let h = |p0: Point2<f64>, pl: Vec2<f64>| Pcurve::Harmonic { p0, pa, pb, pl };
        // `Ok(())` certifies; `Err(true)` refuses by the branch's type;
        // `Err(false)` refuses as a residual.
        let corruptions: Vec<(&str, Pcurve<f64>, Result<(), bool>)> = vec![
            ("exact", image.clone(), Ok(())),
            ("u + τ", h(Point2::new(p0.x + TAU, p0.y), pl), Ok(())),
            ("u − 3τ", h(Point2::new(p0.x - 3.0 * TAU, p0.y), pl), Ok(())),
            (
                "u + 7τ, past MAX_BRANCH_PERIODS",
                h(Point2::new(p0.x + 7.0 * TAU, p0.y), pl),
                Err(true),
            ),
            ("u + π", h(Point2::new(p0.x + PI, p0.y), pl), Err(false)),
            (
                "u + τ/2 + 1e-9",
                h(Point2::new(p0.x + PI + 1e-9, p0.y), pl),
                Err(false),
            ),
            (
                "u + 1e3·ε",
                h(Point2::new(p0.x + off, p0.y), pl),
                Err(false),
            ),
            (
                "v + 1e3·ε",
                h(Point2::new(p0.x, p0.y + off), pl),
                Err(false),
            ),
            (
                "v + τ",
                h(Point2::new(p0.x, p0.y + TAU), pl),
                if angular_v { Ok(()) } else { Err(false) },
            ),
            (
                "slope u flipped",
                h(p0, Vec2::new(-pl.x, pl.y)),
                if pl.x == 0.0 { Ok(()) } else { Err(false) },
            ),
            (
                "slope v flipped",
                h(p0, Vec2::new(pl.x, -pl.y)),
                if pl.y == 0.0 { Ok(()) } else { Err(false) },
            ),
            (
                "u + π, v kept (half a twin)",
                h(Point2::new(p0.x + PI, p0.y), pl),
                Err(false),
            ),
            (
                "the involution twin",
                image.map_affine(
                    |p| Point2::new(p.x + PI, PI - p.y),
                    |v| Vec2::new(v.x, -v.y),
                ),
                if sphere { Ok(()) } else { Err(false) },
            ),
            (
                "the twin, a period on",
                image.map_affine(
                    |p| Point2::new(p.x + PI + TAU, PI - p.y),
                    |v| Vec2::new(v.x, -v.y),
                ),
                if sphere { Ok(()) } else { Err(false) },
            ),
        ];
        for (what, stored, want) in corruptions {
            let read = |r: Result<(), PcurveCertifyError>| match r {
                Ok(()) => Ok(()),
                Err(PcurveCertifyError::BranchOutOfReach) => Err(true),
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
                Err(other) => panic!("{name} / {what}: an unexpected refusal {other:?}"),
            };
            let at_f64 = read(
                PcurveCache::certify(
                    stored.clone(),
                    t0,
                    t1,
                    carrier,
                    surface,
                    window(100.0),
                    band(),
                )
                .map(|_| ()),
            );
            let lift = Interval::from_f64;
            let at_iv = read(
                PcurveCache::certify(
                    lift_harmonic(&stored),
                    lift(t0),
                    lift(t1),
                    &carrier.map_scalar(lift),
                    &surface.map_scalar(lift),
                    ChartWindow {
                        u_min: lift(-100.0),
                        u_max: lift(100.0),
                        v_min: lift(-100.0),
                        v_max: lift(100.0),
                    },
                    band(),
                )
                .map(|_| ()),
            );
            if at_f64 != want || at_iv != want {
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
        let (derived, derivation) = derive_harmonic(&carrier, &surface, band())
            .unwrap_or_else(|e| panic!("trial {trial} ({class:?}): an exact carrier refused: {e}"));
        let noise = 1e-12 * scale(&surface, &carrier, reach);
        let image = chart_image_harmonic(&derived, &surface, windings_of(derivation))
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
