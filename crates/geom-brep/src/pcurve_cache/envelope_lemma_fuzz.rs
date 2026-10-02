//! Check 4's per-arm lemma, swept: on every periodic chart class the
//! derivation covers, the incidence terms plus the fidelity terms
//! dominate the true sup of `|S(P(t)) − C(t)|`, and on an exact carrier
//! the closed-form tables compose back to the carrier. In a file of its
//! own so the per-file gate can skip the sweep without skipping
//! `pcurve_cache`'s deterministic rows.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/pcurve_cache.rs",
    "crates/geom/src/surfaces.rs",
    "crates/geom/src/curves.rs",
    "crates/geom/src/azimuth.rs",
];

use super::{
    ChartWindings, Derivation, EnvelopeTerm, EnvelopeTerms, Pcurve, Winding, carrier_harmonic,
    chart_image_harmonic, derive_harmonic, incidence, periodic_envelope,
};
use geom::{Curve3, Surface};
use geom_core::predicate::Band;
use geom_core::tolerance::Tol;
use geom_core::{Point2, Point3, Vec2, Vec3};
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

/// A circle carrier from a centre, a unit axis and a unit `u_ref ⊥ axis`.
fn circle(center: Point3<f64>, axis: Vec3<f64>, radius: f64, u_ref: Vec3<f64>) -> Curve3<f64> {
    Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    }
}

/// One random chart and one carrier of a covered class on it, exact.
fn on_chart(s: &mut fuzz::Rng) -> (Surface<f64>, Curve3<f64>, &'static str) {
    let (axis, u_ref) = frame(s);
    let at = Point3::new(s.range(-3.0, 3.0), s.range(-3.0, 3.0), s.range(-3.0, 3.0));
    let alpha = s.range(-PI, PI);
    let r = rad(axis, u_ref, alpha);
    let tang = axis.cross(r);
    match s.below(9) {
        0..=2 => {
            let radius = s.range(0.2, 4.0);
            let chart = Surface::Cylinder {
                origin: at,
                axis,
                radius,
                u_ref,
            };
            let v0 = s.range(-3.0, 3.0);
            match s.below(3) {
                0 => (
                    chart,
                    circle(at + axis * v0, axis * sign(s), radius, r),
                    "cylinder rim",
                ),
                1 => {
                    // A plane section tilted by θ about `tang`: the
                    // ellipse whose minor axis is the radius along it.
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
                        "cylinder section",
                    )
                }
                _ => (
                    chart,
                    Curve3::Line {
                        origin: at + r * radius + axis * v0,
                        dir: axis * s.range(-3.0, 3.0),
                    },
                    "cylinder ruling",
                ),
            }
        }
        3 | 4 => {
            let half_angle = s.range(0.1, 1.4);
            let chart = Surface::Cone {
                apex: at,
                axis,
                half_angle,
                u_ref,
            };
            let generator = axis * half_angle.cos() + r * half_angle.sin();
            let v0 = sign(s) * s.range(0.2, 3.0);
            if s.below(2) == 0 {
                let center = at + axis * (v0 * half_angle.cos());
                let radius = v0.abs() * half_angle.sin();
                // A mirror-nappe rim's chart azimuth is the spatial one
                // plus π, so its `u_ref` is the spatial direction.
                let spatial = if v0 > 0.0 { r } else { -r };
                (
                    chart,
                    circle(center, axis * sign(s), radius, spatial),
                    "cone rim",
                )
            } else {
                (
                    chart,
                    Curve3::Line {
                        origin: at + generator * v0,
                        dir: generator * (sign(s) * s.range(0.2, 3.0)),
                    },
                    "cone ruling",
                )
            }
        }
        5 | 6 => {
            let radius = s.range(0.2, 4.0);
            let chart = Surface::Sphere {
                center: at,
                radius,
                axis,
                u_ref,
            };
            if s.below(2) == 0 {
                let v0 = s.range(-1.3, 1.3);
                (
                    chart,
                    circle(
                        at + axis * (radius * v0.sin()),
                        axis * sign(s),
                        radius * v0.cos(),
                        r,
                    ),
                    "sphere parallel",
                )
            } else {
                // Started at a pole, or clear of one: between the two,
                // the derivation reads the meridian plane off an `a_r`
                // a few ulps long, and so does every term built on it.
                let delta = if s.below(4) == 0 {
                    sign(s) * FRAC_PI_2
                } else {
                    s.range(-1.4, 1.4)
                };
                let start = r * delta.cos() + axis * delta.sin();
                (
                    chart,
                    circle(at, tang * sign(s), radius, start),
                    "sphere meridian",
                )
            }
        }
        _ => {
            let minor = s.range(0.2, 1.5);
            let major = minor + s.range(0.2, 3.0);
            let chart = Surface::Torus {
                center: at,
                axis,
                major_radius: major,
                minor_radius: minor,
                u_ref,
            };
            if s.below(2) == 0 {
                let v0 = s.range(-PI, PI);
                (
                    chart,
                    circle(
                        at + axis * (minor * v0.sin()),
                        axis * sign(s),
                        major + minor * v0.cos(),
                        r,
                    ),
                    "torus parallel",
                )
            } else {
                let delta = s.range(-PI, PI);
                let start = r * delta.cos() + axis * delta.sin();
                (
                    chart,
                    circle(at + r * major, tang * sign(s), minor, start),
                    "torus meridian",
                )
            }
        }
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
            Curve3::Circle {
                center: center + nudge(s, 1),
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

/// A size of move: inside the band half the time, where every class
/// decision still reads the exact class and the terms it admits are
/// the ones in play, and anywhere up to a millimetre otherwise.
fn magnitude(s: &mut fuzz::Rng) -> f64 {
    if s.below(2) == 0 {
        Tol::witness().eps() * s.range(0.05, 0.95)
    } else {
        10f64.powf(s.range(-9.0, -3.0))
    }
}

/// The sampled sup of `|S(P(t)) − C(t)|` over `[t0, t1]` — a lower
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

/// **The lemma, swept**: for a carrier moved off its chart and a stored
/// image moved off the one derived from it — by a whole period, or onto
/// a sphere's involution twin, as well as by drift — check 4's envelope
/// is never below the sampled sup of the stored image's residual. Each
/// row moves the carrier alone, the image alone, or both, so a term is
/// met with nothing else to cover for it.
///
/// A table that under-states a term (a dropped norm, a lost factor, a
/// wrong sign in an orientation) lets the residual through it on some
/// draw; the failure prints the terms.
#[test]
fn the_envelope_dominates_the_stored_images_residual_on_every_periodic_arm() {
    let mut s = fuzz::start("pcurve_cache::envelope_lemma::dominates");
    let mut checked = 0usize;
    for trial in 0..fuzz::scaled(500) {
        let (surface, exact, class) = on_chart(&mut s);
        // 0: the carrier moves; 1: the image moves; 2: both; 3:
        // neither, the image only renamed (a whole period, the twin).
        let mode = s.below(4);
        let delta = magnitude(&mut s);
        let carrier = if mode == 1 || mode == 3 {
            exact
        } else {
            perturbed(&mut s, &exact, delta)
        };
        let (t0, t1) = span(&mut s);
        let reach = t0.abs().max(t1.abs());
        let Ok((derived, _)) = derive_harmonic(&carrier, &surface, band()) else {
            continue;
        };
        let Pcurve::Harmonic { p0, pa, pb, pl } = derived else {
            unreachable!("derive_harmonic answers harmonic images")
        };
        let drift = if mode == 0 || mode == 3 {
            0.0
        } else {
            magnitude(&mut s)
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
        let terms = periodic_envelope(&stored, &carrier, form, &surface, (t0, t1), reach, band())
            .unwrap_or_else(|e| panic!("trial {trial} ({class}): the envelope refused: {e}"));
        let envelope = terms.total();
        let sup = sampled_sup(&stored, &surface, &carrier, t0, t1);
        let noise = 64.0 * f64::EPSILON * scale(&surface, &carrier, reach);
        assert!(
            envelope >= sup - noise,
            "trial {trial} ({class}, mode {mode}, δ = {delta:e}, drift = {drift:e}, k = {k}): \
             envelope {envelope:e} under the sampled sup {sup:e}; terms {:?} — {}",
            EnvelopeTerm::ALL
                .iter()
                .map(|&t| (t, terms.0[t as usize]))
                .collect::<Vec<_>>(),
            fuzz::replay()
        );
        if mode == 3 {
            assert!(
                envelope <= noise,
                "trial {trial} ({class}, k = {k}): a renamed exact image costs {envelope:e}; \
                 terms {:?} — {}",
                terms.0,
                fuzz::replay()
            );
        }
        checked += 1;
    }
    println!("[fuzz] envelope_lemma::dominates: {checked} rows checked");
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
        let (surface, carrier, class) = on_chart(&mut s);
        let (t0, t1) = span(&mut s);
        let reach = t0.abs().max(t1.abs());
        let (derived, derivation) = derive_harmonic(&carrier, &surface, band())
            .unwrap_or_else(|e| panic!("trial {trial} ({class}): an exact carrier refused: {e}"));
        let noise = 1e-12 * scale(&surface, &carrier, reach);
        let image = chart_image_harmonic(&derived, &surface, windings_of(derivation))
            .unwrap_or_else(|| panic!("trial {trial} ({class}): no closed-form image"));
        let form = carrier_harmonic(&carrier).unwrap();
        let gaps = [
            (image.c - form.c).norm(),
            (image.a - form.a).norm(),
            (image.b - form.b).norm(),
            (image.l - form.l).norm(),
        ];
        assert!(
            gaps.iter().all(|&g| g <= noise),
            "trial {trial} ({class}, {derivation:?}): S∘P_d is not the carrier, coefficient \
             gaps {gaps:?} — {}",
            fuzz::replay()
        );
        let mut terms = EnvelopeTerms::new();
        incidence(form, &surface, derivation, reach, &mut terms);
        assert!(
            terms.total() <= noise,
            "trial {trial} ({class}, {derivation:?}): an exact carrier's incidence is {:e}, terms \
             {:?} — {}",
            terms.total(),
            terms.0,
            fuzz::replay()
        );
    }
}
