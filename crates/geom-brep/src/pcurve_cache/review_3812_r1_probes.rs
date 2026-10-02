//! PCERT reviewer R1's adversarial probes for PR 3812 (check 4 restated
//! as incidence + fidelity). Each probe reports how many rows it ran and
//! how many broke, and asserts none did.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::{
    ChartWindow, Pcurve, PcurveCache, carrier_harmonic, derive_harmonic, periodic_envelope,
    whole_periods,
};
use geom::{Curve3, Surface};
use geom_core::predicate::Band;
use geom_core::predicate::Margin;
use geom_core::tolerance::Tol;
use geom_core::{Bounds, Interval, Point2, Point3, Real, Vec2, Vec3};
use std::f64::consts::{FRAC_PI_2, PI, TAU};
use test_utils::fuzz;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn eps() -> f64 {
    Tol::witness().eps()
}

fn unit(s: &mut fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(s.range(-1.0, 1.0), s.range(-1.0, 1.0), s.range(-1.0, 1.0));
        if v.norm() > 0.2 {
            return v.normalize();
        }
    }
}

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

fn dense_sup(p: &Pcurve<f64>, s: &Surface<f64>, c: &Curve3<f64>, t0: f64, t1: f64) -> f64 {
    (0..=8192)
        .map(|k| {
            let t = t0 + (t1 - t0) * (f64::from(k) / 8192.0);
            let q = p.eval(t);
            s.eval(q.x, q.y).distance(c.eval(t))
        })
        .fold(0.0, f64::max)
}

/// Every covered class, including the sliver arms the shipped sweep
/// never draws: a ruling anchored AT the apex (nappe read off the
/// direction), a sphere meridian started a hair off the pole, and a
/// torus meridian with its centre near the axis plane.
fn on_chart(s: &mut fuzz::Rng, at: Point3<f64>) -> (Surface<f64>, Curve3<f64>, &'static str) {
    let (axis, u_ref) = frame(s);
    let alpha = s.range(-PI, PI);
    let r = rad(axis, u_ref, alpha);
    let tang = axis.cross(r);
    match s.below(8) {
        0 => {
            let radius = s.range(0.2, 4.0);
            let v0 = s.range(-3.0, 3.0);
            (
                Surface::Cylinder {
                    origin: at,
                    axis,
                    radius,
                    u_ref,
                },
                circle(at + axis * v0, axis * sign(s), radius, r),
                "cylinder rim",
            )
        }
        1 => {
            let radius = s.range(0.2, 4.0);
            let v0 = s.range(-3.0, 3.0);
            (
                Surface::Cylinder {
                    origin: at,
                    axis,
                    radius,
                    u_ref,
                },
                Curve3::Line {
                    origin: at + r * radius + axis * v0,
                    dir: axis * s.range(-3.0, 3.0),
                },
                "cylinder ruling",
            )
        }
        2 => {
            // A ruling anchored at (or a hair off) the apex: h0 is in
            // the band, so the nappe is read off the direction.
            let half_angle = s.range(0.1, 1.4);
            let generator = axis * half_angle.cos() + r * half_angle.sin();
            let h0 = eps() * s.range(-0.9, 0.9) * f64::from(u8::from(s.below(2) == 0));
            (
                Surface::Cone {
                    apex: at,
                    axis,
                    half_angle,
                    u_ref,
                },
                Curve3::Line {
                    origin: at + generator * h0,
                    dir: generator * (sign(s) * s.range(0.2, 3.0)),
                },
                "cone ruling at apex",
            )
        }
        3 => {
            let half_angle = s.range(0.1, 1.4);
            let v0 = sign(s) * s.range(0.2, 3.0);
            let center = at + axis * (v0 * half_angle.cos());
            let radius = v0.abs() * half_angle.sin();
            let spatial = if v0 > 0.0 { r } else { -r };
            (
                Surface::Cone {
                    apex: at,
                    axis,
                    half_angle,
                    u_ref,
                },
                circle(center, axis * sign(s), radius, spatial),
                "cone rim",
            )
        }
        4 => {
            let radius = s.range(0.2, 4.0);
            // A hair off the pole: cos δ is 1e-12 .. 1e-6.
            let delta = sign(s) * (FRAC_PI_2 - 10f64.powf(s.range(-12.0, -6.0)));
            let start = r * delta.cos() + axis * delta.sin();
            (
                Surface::Sphere {
                    center: at,
                    radius,
                    axis,
                    u_ref,
                },
                circle(at, tang * sign(s), radius, start),
                "sphere meridian near pole",
            )
        }
        5 => {
            let radius = s.range(0.2, 4.0);
            // A parallel a hair off the pole: its radius is tiny.
            let v0 = sign(s) * (FRAC_PI_2 - 10f64.powf(s.range(-7.0, -2.0)));
            (
                Surface::Sphere {
                    center: at,
                    radius,
                    axis,
                    u_ref,
                },
                circle(
                    at + axis * (radius * v0.sin()),
                    axis * sign(s),
                    radius * v0.cos(),
                    r,
                ),
                "sphere parallel near pole",
            )
        }
        6 => {
            let minor = s.range(0.2, 1.5);
            let major = minor + s.range(0.2, 3.0);
            let v0 = s.range(-PI, PI);
            (
                Surface::Torus {
                    center: at,
                    axis,
                    major_radius: major,
                    minor_radius: minor,
                    u_ref,
                },
                circle(
                    at + axis * (minor * v0.sin()),
                    axis * sign(s),
                    major + minor * v0.cos(),
                    r,
                ),
                "torus parallel",
            )
        }
        _ => {
            let minor = s.range(0.2, 1.5);
            let major = minor + s.range(0.2, 3.0);
            let delta = s.range(-PI, PI);
            let start = r * delta.cos() + axis * delta.sin();
            (
                Surface::Torus {
                    center: at,
                    axis,
                    major_radius: major,
                    minor_radius: minor,
                    u_ref,
                },
                circle(at + r * major, tang * sign(s), minor, start),
                "torus meridian",
            )
        }
    }
}

/// Every field moved at once, by up to `delta`, plus the circle's own
/// phase (`u_ref` rotated in its plane) and its handedness reversed.
fn perturbed(s: &mut fuzz::Rng, carrier: &Curve3<f64>, delta: f64) -> Curve3<f64> {
    let nudge = |s: &mut fuzz::Rng| unit(s) * (delta * s.unit());
    match *carrier {
        Curve3::Line { origin, dir } => Curve3::Line {
            origin: origin + nudge(s),
            dir: dir + nudge(s) * dir.norm(),
        },
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => {
            let axis = (axis + nudge(s)).normalize();
            let u_ref = (u_ref - axis * u_ref.dot(axis)).normalize();
            let phase = s.range(-PI, PI);
            let u_ref = u_ref * phase.cos() + axis.cross(u_ref) * phase.sin();
            Curve3::Circle {
                center: center + nudge(s),
                axis,
                radius: radius * (1.0 + delta * s.range(-1.0, 1.0)),
                u_ref,
            }
        }
        _ => unreachable!(),
    }
}

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
        _ => unreachable!(),
    };
    chart + c.c.distance(Point3::origin()) + c.a.norm() + c.b.norm() + c.l.norm() * reach + 1.0
}

/// P1: the lemma at LARGE moves (1e-3 .. 0.3, all fields at once) and on
/// the sliver arms — wherever the derivation still answers, the envelope
/// must dominate the dense residual of the derived image AND of a stored
/// image jittered in every slot.
#[test]
fn r1_envelope_dominates_large_and_sliver_moves() {
    let mut s = fuzz::start("review_3812_r1::large_and_sliver");
    let (mut ran, mut broke) = (0usize, Vec::new());
    for trial in 0..fuzz::scaled(4000) {
        let at = Point3::new(s.range(-3.0, 3.0), s.range(-3.0, 3.0), s.range(-3.0, 3.0));
        let (surface, exact, class) = on_chart(&mut s, at);
        let delta = match s.below(3) {
            0 => 0.0,
            1 => eps() * s.range(0.05, 3.0),
            _ => 10f64.powf(s.range(-3.0, -0.5)),
        };
        let carrier = perturbed(&mut s, &exact, delta);
        let t0 = s.range(-PI, PI);
        let (t0, t1) = (t0, t0 + s.range(0.1, TAU - 0.1));
        let reach = t0.abs().max(t1.abs());
        let Ok((derived, _)) = derive_harmonic(&carrier, &surface, band()) else {
            continue;
        };
        let Pcurve::Harmonic { p0, pa, pb, pl } = derived else {
            continue;
        };
        let drift = if s.below(2) == 0 {
            0.0
        } else {
            10f64.powf(s.range(-10.0, -2.0))
        };
        let mut j = |v: f64| v + drift * s.range(-1.0, 1.0);
        let stored = Pcurve::Harmonic {
            p0: Point2::new(j(p0.x), j(p0.y)),
            pa: Vec2::new(j(pa.x), j(pa.y)),
            pb: Vec2::new(j(pb.x), j(pb.y)),
            pl: Vec2::new(j(pl.x), j(pl.y)),
        };
        let form = carrier_harmonic(&carrier).unwrap();
        let Ok(terms) =
            periodic_envelope(&stored, &carrier, form, &surface, (t0, t1), reach, band())
        else {
            continue;
        };
        let sup = dense_sup(&stored, &surface, &carrier, t0, t1);
        let noise = 64.0 * f64::EPSILON * scale(&surface, &carrier, reach);
        // The certifying scalar: the same inputs, lifted.
        let f = Interval::from_f64;
        let ci = carrier.map_scalar(f);
        let Ok(ti) = periodic_envelope(
            &lift(&stored),
            &ci,
            carrier_harmonic(&ci).unwrap(),
            &surface.map_scalar(f),
            (f(t0), f(t1)),
            f(reach),
            band(),
        ) else {
            continue;
        };
        let iv = ti.total();
        if iv.hi().is_nan() || terms.total().is_nan() {
            continue;
        }
        let envelope = iv.hi();
        ran += 1;
        if !(envelope >= sup - noise) {
            broke.push(format!(
                "trial {trial} {class} δ={delta:e} drift={drift:e}: env {envelope:e} < sup {sup:e} terms {:?}",
                terms.0
            ));
        }
    }
    println!("[r1] large_and_sliver: {ran} rows, {} broke", broke.len());
    for b in broke.iter().take(20) {
        println!("  {b}");
    }
    assert!(
        broke.is_empty(),
        "{} rows under-bounded — {}",
        broke.len(),
        fuzz::replay()
    );
}

/// P2: a non-unit chart axis at a far placement. The lemma's frame
/// premise ("`axis` unit") is not metered inside check 4; `Surface::eval`
/// uses the axis as stored. Does the envelope still dominate?
#[test]
fn r1_envelope_against_a_non_unit_axis_far_out() {
    let mut worst: f64 = 0.0;
    for &(eta, far) in &[
        (1e-15, 1e4),
        (1e-13, 1e4),
        (1e-12, 1e3),
        (1e-10, 1e2),
        (1e-9, 10.0),
    ] {
        let axis = Vec3::new(0.0, 0.0, 1.0 + eta);
        let surface = Surface::Cylinder {
            origin: Point3::new(far, -far, far),
            axis,
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        // The carrier lies EXACTLY on the surface as `eval` reads it.
        let v0 = 0.5 * far;
        let rim_center = surface.eval(0.0, v0) - Vec3::unit_x();
        let carrier = circle(rim_center, Vec3::unit_z(), 1.0, Vec3::unit_x());
        let Ok((image, _)) = derive_harmonic(&carrier, &surface, band()) else {
            println!("[r1] η={eta:e} far={far:e}: derivation refused");
            continue;
        };
        let form = carrier_harmonic(&carrier).unwrap();
        let env = periodic_envelope(&image, &carrier, form, &surface, (0.0, PI), PI, band())
            .map(|t| t.total());
        let sup = dense_sup(&image, &surface, &carrier, 0.0, PI);
        println!("[r1] η={eta:e} far={far:e}: envelope {env:?}, dense sup {sup:e}");
        if let Ok(e) = env {
            worst = worst.max(sup - e);
        }
    }
    println!("[r1] non-unit axis: worst (sup − envelope) = {worst:e}");
}

fn lift_window() -> ChartWindow<Interval> {
    let f = Interval::from_f64;
    ChartWindow {
        u_min: f(-100.0),
        u_max: f(100.0),
        v_min: f(-100.0),
        v_max: f(100.0),
    }
}

fn lift(p: &Pcurve<f64>) -> Pcurve<Interval> {
    let Pcurve::Harmonic { p0, pa, pb, pl } = *p else {
        unreachable!()
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

/// P3: claim 2 at the exact witness, where check 3 no longer runs. A
/// corrupted stored row must refuse at `Interval` exactly as at f64.
#[test]
fn r1_corrupted_rows_refuse_at_interval() {
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
    let mut wrong = Vec::new();
    for (surface, carrier, name) in &fixtures {
        let (image, _) = derive_harmonic(carrier, surface, band()).unwrap();
        let Pcurve::Harmonic { p0, pa, pb, pl } = image else {
            unreachable!()
        };
        let h = |p0: Point2<f64>, pl: Vec2<f64>| Pcurve::Harmonic { p0, pa, pb, pl };
        let corruptions: Vec<(&str, Pcurve<f64>, bool)> = vec![
            ("exact", image.clone(), true),
            ("u + τ", h(Point2::new(p0.x + TAU, p0.y), pl), true),
            ("u − 3τ", h(Point2::new(p0.x - 3.0 * TAU, p0.y), pl), true),
            (
                "u + 7τ (past MAX_BRANCH)",
                h(Point2::new(p0.x + 7.0 * TAU, p0.y), pl),
                false,
            ),
            ("u + π", h(Point2::new(p0.x + PI, p0.y), pl), false),
            (
                "u + τ/2 + 1e-9",
                h(Point2::new(p0.x + PI + 1e-9, p0.y), pl),
                false,
            ),
            ("u + 1e-6", h(Point2::new(p0.x + 1e-6, p0.y), pl), false),
            ("v + 1e-6", h(Point2::new(p0.x, p0.y + 1e-6), pl), false),
            (
                "v + τ",
                h(Point2::new(p0.x, p0.y + TAU), pl),
                !name.starts_with("cyl") && !name.starts_with("cone"),
            ),
            (
                "slope u flipped",
                h(p0, Vec2::new(-pl.x, pl.y)),
                pl.x == 0.0,
            ),
            (
                "slope v flipped",
                h(p0, Vec2::new(pl.x, -pl.y)),
                pl.y == 0.0,
            ),
            (
                "u + π, v kept (half a twin)",
                h(Point2::new(p0.x + PI, p0.y), pl),
                false,
            ),
            (
                "full twin",
                image.map_affine(
                    |p| Point2::new(p.x + PI, PI - p.y),
                    |v| Vec2::new(v.x, -v.y),
                ),
                name.starts_with("sphere"),
            ),
        ];
        for (what, stored, should_pass) in corruptions {
            let at_f64 = PcurveCache::certify(
                stored.clone(),
                t0,
                t1,
                carrier,
                surface,
                ChartWindow {
                    u_min: -100.0,
                    u_max: 100.0,
                    v_min: -100.0,
                    v_max: 100.0,
                },
                band(),
            );
            let at_iv = PcurveCache::certify(
                lift(&stored),
                Interval::from_f64(t0),
                Interval::from_f64(t1),
                &carrier.map_scalar(Interval::from_f64),
                &surface.map_scalar(Interval::from_f64),
                lift_window(),
                band(),
            );
            let line = format!(
                "{name} / {what}: f64 {} · Interval {}",
                at_f64
                    .as_ref()
                    .map_or_else(|e| format!("ERR {e:?}"), |_| "ok".into()),
                at_iv.as_ref().map_or_else(
                    |e| format!("ERR {e:?}"),
                    |c| format!("ok samples={}", c.certificate().samples)
                ),
            );
            println!("[r1] {line}");
            if at_iv.is_ok() != should_pass || at_f64.is_ok() != at_iv.is_ok() {
                wrong.push(line);
            }
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// P4: `whole_periods` at and around the half-period marks, at a point
/// and over a box straddling one.
#[test]
fn r1_whole_periods_near_a_half_period() {
    let meter = |g: f64| Margin::levered(g, 1.0);
    for gap in [
        PI - 1e-6,
        PI - 2e-9,
        PI,
        PI + 2e-9,
        PI + 1e-6,
        -PI + 1e-7,
        4.4 * TAU,
        4.6 * TAU,
        -4.6 * TAU,
    ] {
        let k = whole_periods("r1", gap, TAU, meter, band());
        println!("[r1] f64 gap {gap:.12}: {k:?}");
    }
    let imeter = |g: Interval| Margin::levered(g, Interval::from_f64(1.0));
    for (lo, hi) in [
        (PI - 1e-3, PI + 1e-3),
        (PI - 1e-3, PI - 1e-4),
        (0.5 * TAU - 1e-12, 0.5 * TAU + 1e-12),
        (2.9 * TAU, 3.4 * TAU),
    ] {
        let g = Interval::from_bounds(lo, hi);
        let k = whole_periods("r1", g, <Interval as Real>::tau(), imeter, band());
        println!("[r1] interval gap [{lo:.6}, {hi:.6}]: {k:?}");
    }
    let _ = <Interval as Real>::tau();
    let _: f64 = Real::from_f64(0.0);
}

/// P5: a near-pole parallel whose plane sits a hair PAST the pole
/// (`h > r` by less than the band): the radius term takes `√(r² − h²)`
/// of a negative. What does certify answer at f64 and at Interval?
#[test]
fn r1_parallel_past_the_pole() {
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    for (h, rho) in [
        (1.0 + 3e-10, 2e-5),
        (1.0 + 3e-10, 0.0),
        (1.0 - 1e-12, 1.4e-6),
        (1.0 + 5e-10, 3e-5),
    ] {
        let carrier = circle(
            Point3::new(0.0, 0.0, h),
            Vec3::unit_z(),
            rho.max(1e-12),
            Vec3::unit_x(),
        );
        let derived = derive_harmonic(&carrier, &sphere, band());
        println!(
            "[r1] h={h} rho={rho}: derive {:?}",
            derived.as_ref().map(|d| d.1)
        );
        let Ok((image, _)) = derived else { continue };
        let form = carrier_harmonic(&carrier).unwrap();
        let terms = periodic_envelope(&image, &carrier, form, &sphere, (0.0, PI), PI, band());
        println!("[r1]   terms {:?}", terms.as_ref().map(|t| t.0));
        let f = PcurveCache::certify(
            image.clone(),
            0.0,
            PI,
            &carrier,
            &sphere,
            ChartWindow {
                u_min: -100.0,
                u_max: 100.0,
                v_min: -100.0,
                v_max: 100.0,
            },
            band(),
        );
        println!(
            "[r1]   f64 certify: {:?}",
            f.as_ref().map(|c| c.certificate().envelope)
        );
        let iv = PcurveCache::certify(
            lift(&image),
            Interval::from_f64(0.0),
            Interval::from_f64(PI),
            &carrier.map_scalar(Interval::from_f64),
            &sphere.map_scalar(Interval::from_f64),
            lift_window(),
            band(),
        );
        println!(
            "[r1]   Interval certify: {:?}",
            iv.as_ref().map(|c| c.certificate().envelope)
        );
    }
}

/// P6: the exact involution twin at Interval — which decision drops it?
#[test]
fn r1_twin_at_interval() {
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let carrier = circle(Point3::origin(), Vec3::unit_y(), 1.0, Vec3::unit_x());
    let (image, _) = derive_harmonic(&carrier, &sphere, band()).unwrap();
    let twin = image.map_affine(
        |p| Point2::new(p.x + PI, PI - p.y),
        |v| Vec2::new(v.x, -v.y),
    );
    let si = sphere.map_scalar(Interval::from_f64);
    let ci = carrier.map_scalar(Interval::from_f64);
    let form = carrier_harmonic(&ci).unwrap();
    let (di, _) = derive_harmonic(&ci, &si, band()).unwrap();
    let (Pcurve::Harmonic { p0: tp, .. }, Pcurve::Harmonic { p0: dp, .. }) = (lift(&twin), di)
    else {
        unreachable!()
    };
    let off = (tp.x - dp.x).reduce_periodic_centred(<Interval as Real>::tau());
    println!("[r1] twin offset reduced at Interval: {off:?}");
    let terms = periodic_envelope(
        &lift(&twin),
        &ci,
        form,
        &si,
        (Interval::from_f64(0.2), Interval::from_f64(2.9)),
        Interval::from_f64(2.9),
        band(),
    );
    println!("[r1] twin terms at Interval: {:?}", terms.map(|t| t.0));
}
