//! **Local windows** (`docs/GERM-CROSSING-U4-SPEC.md` §2, W1–W5): the
//! pose's graze and apex rungs, the walk read past what it refuses, and
//! the closed forms re-posed so no window rests on a cancelled digit.
//! Every oracle reads the stored data with its own arithmetic: the
//! carrier's true distance from the surface, a double-double root, a
//! series.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/curve_face.rs",
    "crates/geom-brep/src/curve_face/cross.rs",
    "crates/geom-brep/src/curve_face/pose.rs",
    "crates/geom-brep/src/half_angle.rs",
];

use core::f64::consts::{FRAC_PI_2, FRAC_PI_3, PI, TAU};

use geom::{Curve3, Frame, Surface};
use geom_core::{Band, Point3, Rounded, Vec3};

use super::conic_oracle::exact::Dd;
use super::conic_oracle::{distance, graze, torus};
use super::{CrossWhy, Pose, PoseRefusal, PoseRung, cross, pose};
use crate::curve_pair::Span;

fn band(eps: f64) -> Band {
    Band::new(eps, 10.0 * eps).unwrap()
}

fn plane_z() -> Surface<f64> {
    Surface::Plane {
        frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
    }
}

/// A circle of radius `r` in the plane `y = 0`, its lowest point at
/// `θ = π/2` standing `lift` above `z = 0`.
fn hanging(r: f64, lift: f64) -> Curve3<f64> {
    Curve3::circle(
        Point3::new(0.0, 0.0, r + lift),
        Vec3::unit_y(),
        r,
        Vec3::unit_x(),
    )
}

/// **A window that holds every meeting, and is the band's escalation
/// wider** (W1, W2, W5): a proper part of the span, on it, and every span
/// point outside it, sampled densely and at the window's own edges,
/// stands at least `Kε` from the surface by its true distance.
fn holds(label: &str, span: (f64, f64), window: (f64, f64), dist: impl Fn(f64) -> f64, b: Band) {
    let (lo, hi) = window;
    let (t0, t1) = span;
    assert!(
        t0 <= lo && lo < hi && hi <= t1,
        "{label}: the window {window:?} is on the span {span:?}"
    );
    assert!(
        hi - lo < 0.5 * (t1 - t0),
        "{label}: the window {window:?} is local, a proper part of {span:?}"
    );
    let k_eps = b.escalate();
    let width = hi - lo;
    let edges = [lo - 1e-9 * width, hi + 1e-9 * width];
    let dense = (0..=20_000).map(|i| t0 + (t1 - t0) * f64::from(i) / 20_000.0);
    for t in dense.chain(edges) {
        if t < t0 || t > t1 || (lo..=hi).contains(&t) {
            continue;
        }
        let d = dist(t).abs();
        assert!(
            d >= 0.99 * k_eps,
            "{label}: the point at {t} outside the window {window:?} is {d:e} from the surface, \
             inside Kε = {k_eps:e}"
        );
    }
}

/// The refusal's window and why, or a panic naming the answer.
fn refused(
    label: &str,
    got: Result<Pose<f64>, super::Undecided<f64, PoseRefusal>>,
) -> ((f64, f64), PoseRefusal) {
    match got {
        Err(u) => (u.window, u.why),
        Ok(p) => panic!("{label}: a window, got {p:?}"),
    }
}

/// **A graze in band on each graze rung answers a local window holding
/// every meeting** (§2.2's table): a circle's belly on a plane, a line on a
/// wall and on a sphere, a circle's first-harmonic extreme on a sphere
/// and on a cone,
/// each read in the escalation gap (three zero bands deep or high), and a
/// line's higher-order contact on a wall (`Kappa`). Red where the window
/// is widened to the span, or the reach read at `ε` for `Kε` (the points
/// just past the window then stand inside `Kε`).
#[test]
fn a_graze_in_band_on_each_rung_is_a_window_over_its_reach() {
    let eps = 1e-9;
    let b = band(eps);
    for lift in [-3.0 * eps, 3.0 * eps] {
        // The plane belly.
        let c = hanging(1.0, lift);
        let label = format!("belly, lift {lift:e}");
        let (window, why) = refused(&label, pose(&Span::new(&c, (0.0, TAU)), &plane_z(), b));
        assert!(
            matches!(
                why,
                PoseRefusal::InBand {
                    rung: PoseRung::BellyGraze,
                    ..
                }
            ),
            "{label}: {why:?}"
        );
        holds(&label, (0.0, TAU), window, |t| c.eval(t).z, b);

        // A line on a wall and on a sphere, `lift` off them along `y`.
        let l = Curve3::Line(geom::Line {
            origin: Point3::new(-2.0, 1.0 + lift, 0.0),
            dir: geom_core::UnitVec3::exact_x(),
        });
        let wall = Surface::Cylinder {
            frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
            radius: 1.0,
        };
        let sphere = Surface::Sphere {
            frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
            radius: 1.0,
        };
        for (name, s) in [("wall", &wall), ("sphere", &sphere)] {
            let label = format!("line × {name}, lift {lift:e}");
            let (window, why) = refused(&label, pose(&Span::new(&l, (0.0, 4.0)), s, b));
            assert!(
                matches!(
                    why,
                    PoseRefusal::InBand {
                        rung: PoseRung::Discriminant,
                        ..
                    }
                ),
                "{label}: {why:?}"
            );
            holds(&label, (0.0, 4.0), window, |t| distance(s, l.eval(t)), b);
        }

        // A circle inside a sphere, its far point `lift` from it.
        let c = Curve3::circle(
            Point3::new(0.3, 0.0, 0.0),
            Vec3::unit_z(),
            0.7 + lift,
            Vec3::unit_y(),
        );
        let label = format!("first harmonic, lift {lift:e}");
        let (window, why) = refused(&label, pose(&Span::new(&c, (-PI, PI)), &sphere, b));
        assert!(
            matches!(
                why,
                PoseRefusal::InBand {
                    rung: PoseRung::Extreme,
                    ..
                }
            ),
            "{label}: {why:?}"
        );
        holds(
            &label,
            (-PI, PI),
            window,
            |t| distance(&sphere, c.eval(t)),
            b,
        );

        // A circle square to a π/4 cone's axis, its far point `lift`
        // from the cone: the extreme's window is read on the quadric
        // form `F`, and holds the residual's `Kε` reach (`|F| ≤ |res|`
        // on a cone).
        let cone = Surface::Cone {
            frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
            half_angle: core::f64::consts::FRAC_PI_4,
        };
        let c = Curve3::circle(
            Point3::new(0.3, 0.0, 1.0),
            Vec3::unit_z(),
            0.7 + lift,
            Vec3::unit_y(),
        );
        let label = format!("cone's first harmonic, lift {lift:e}");
        let (window, why) = refused(&label, pose(&Span::new(&c, (-PI, PI)), &cone, b));
        assert!(
            matches!(
                why,
                PoseRefusal::InBand {
                    rung: PoseRung::Extreme,
                    ..
                }
            ),
            "{label}: {why:?}"
        );
        holds(
            &label,
            (-PI, PI),
            window,
            |t| crate::implicit_residual(&cone, c.eval(t)),
            b,
        );
    }
    // A line that touches a wall to second order (`pose_rows`' slope
    // 1e-4): its curvature in the zero band, its reach about 70 m of the
    // line either way at this band, read on a span past it.
    let slope = 1e-4_f64;
    let l = Curve3::Line(geom::Line {
        origin: Point3::new(1.0, 0.0, 0.0),
        dir: geom_core::UnitVec3::squared(Vec3::new(0.0, slope, (1.0 - slope * slope).sqrt())),
    });
    let wall = Surface::Cylinder {
        frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        radius: 1.0,
    };
    let b = Band::new(1e-6, 1e-5).unwrap();
    let (window, why) = refused("kappa", pose(&Span::new(&l, (-300.0, 300.0)), &wall, b));
    assert!(
        matches!(
            why,
            PoseRefusal::InBand {
                rung: PoseRung::Kappa,
                ..
            }
        ),
        "kappa: {why:?}"
    );
    holds(
        "kappa",
        (-300.0, 300.0),
        window,
        |t| distance(&wall, l.eval(t)),
        b,
    );
}

/// **A line through a cone's apex in band is a window over its roots'
/// spread** (§2.2's `Apex` row): a line passing three zero bands from
/// the apex, its two meetings with the double cone about it. Red where
/// the apex's refusal is the whole span.
#[test]
fn a_line_by_a_cones_apex_in_band_is_a_window_over_its_spread() {
    let eps = 1e-9;
    let b = band(eps);
    let cone = Surface::Cone {
        frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        half_angle: 0.5,
    };
    let l = Curve3::Line(geom::Line {
        origin: Point3::new(-3.0, 3.0 * eps, 0.0),
        dir: geom_core::UnitVec3::exact_x(),
    });
    let span = (0.0, 6.0);
    let (window, why) = refused("apex", pose(&Span::new(&l, span), &cone, b));
    assert!(
        matches!(
            why,
            PoseRefusal::InBand {
                rung: PoseRung::Apex | PoseRung::ApexSpread,
                ..
            }
        ),
        "apex: {why:?}"
    );
    // The double cone's meetings with the line, read off its form.
    let (sin, cos) = 0.5_f64.sin_cos();
    let form = |t: f64| {
        let p = l.eval(t);
        (p.x.powi(2) + p.y.powi(2)).sqrt() * cos - p.z.abs() * sin
    };
    for root in super::conic_oracle::crossings(form, span.0, span.1) {
        assert!(
            window.0 <= root && root <= window.1,
            "apex: the meeting at {root} lies outside the window {window:?}"
        );
    }
    assert!(
        window.1 - window.0 < 3.0,
        "apex: the window {window:?} is local"
    );
}

/// The series `asin(s) = s + s³/6 + 3s⁵/40 + 15s⁷/336 + 105s⁹/3456` in
/// double-double, for `s ≤ 0.01`, where its next term is below `1e-21·s`.
fn asin_dd(s: Dd) -> f64 {
    let s2 = s.mul(s);
    let mut term = s;
    let mut sum = s;
    // Each term over the last: `s²·(2k+1)²/((2k+2)(2k+3))`.
    for ratio in [1.0 / 6.0, 9.0 / 20.0, 25.0 / 42.0, 49.0 / 72.0] {
        term = term.mul(s2).mul(Dd::of(ratio));
        sum = sum.add(term);
    }
    sum.to_f64()
}

/// **A touch's in-band arc keeps its digits** (`pose::arc`'s harmonic
/// branch, `2·asin(√(z/2·bend))`): a circle of radius `R` touching a plane
/// at its lowest point, read at `ε = 1e-12`, so `z/bend = ε/R` runs from
/// `1.3e-4` down to `1.2e-16`. The half-arc against the series in
/// double-double, to a few ulps of the touch's parameter. Red with
/// `acos(1 − z/bend)` restored, whose subtraction keeps four of the
/// ratio's digits at `1e-12`.
#[test]
fn a_touchs_harmonic_arc_keeps_its_digits_at_every_ratio() {
    let eps = 1e-12;
    let b = band(eps);
    for k in -27..=13 {
        let r = 2.0_f64.powi(k);
        let c = hanging(r, 0.0);
        let label = format!("R = 2^{k}, z/bend {:e}", eps / r);
        let touch = match pose(&Span::new(&c, (0.0, PI)), &plane_z(), b) {
            Ok(Pose::Tangent(touch)) => touch,
            other => panic!("{label}: a touch, got {other:?}"),
        };
        let half = (touch.arc.1 - touch.arc.0) / 2.0;
        // `z/2·bend` is exact: `ε` over a power of two.
        let s = Dd::of(eps / (2.0 * r)).sqrt();
        let want = 2.0 * asin_dd(s);
        let slack = 4.0 * f64::EPSILON * (FRAC_PI_2 + want);
        assert!(
            (half - want).abs() <= slack,
            "{label}: the half-arc {half:e} against {want:e}, off {:e}",
            (half - want).abs()
        );
    }
}

/// **A circle crossing a plane just below its belly is placed by its
/// depth** (`pose::plane_conic`'s roots): a circle of radius 3 dipping `h`
/// below a plane, `h = 2⁻¹⁴` (6e-5) down to `2⁻³³` (1.2e-10, ten `Kε` at
/// `ε = 1e-12`). Each meeting's offset along the plane against the
/// double-double `√(h(6 − h))`, within the zero band. Red with
/// `acos(−d₀/R)` restored: `−d₀/R = 1 − h/3` rounds, which moves the
/// root by `u·R/√(2h/R)` of arc, several zero bands at the deepest.
#[test]
fn a_circle_crossing_a_plane_near_its_belly_is_placed_within_the_band() {
    let eps = 1e-12;
    let b = band(eps);
    for k in 14..=33 {
        let h = 2.0_f64.powi(-k);
        let c = hanging(3.0, -h);
        let label = format!("h = 2^-{k}");
        let got = cross(&Span::new(&c, (0.0, PI)), &plane_z(), b)
            .unwrap_or_else(|r| panic!("{label}: two crossings, got {r:?}"));
        assert_eq!(got.meetings.len(), 2, "{label}: {got:?}");
        let want = Dd::of(h).mul(Dd::of(6.0 - h)).sqrt().to_f64();
        for m in &got.meetings {
            let off = (m.point.x.abs() - want).abs();
            assert!(
                off <= eps,
                "{label}: the meeting at x = {:e} against {want:e}, off {off:e}",
                m.point.x
            );
        }
    }
}

/// The torus of the measured off-arc touch
/// (`a-circle-touching-a-torus-off-the-arc-escalates-the-arcs-side-decision`):
/// about `z`, radii 1 and 0.25, and the circle of radius 0.5 about
/// `(1.75, 0, 0)` in `y = 0`, `u_ref = −x`, touching the outer equator at
/// `θ = 0`.
fn measured_touch() -> (Curve3<f64>, Surface<f64>) {
    let ring = Curve3::circle(
        Point3::new(1.75, 0.0, 0.0),
        Vec3::unit_y(),
        0.5,
        Vec3::new(-1.0, 0.0, 0.0),
    );
    (ring, torus(Point3::origin(), Vec3::unit_z(), 1.0, 0.25))
}

/// **A touch off the arc refuses nothing on it** (the measured case): the
/// arc `[1, 2]`, half a metre from the touch, answers with no window, at
/// every row the case was measured at, through `cross` and the sweep's
/// wrapper alike; the arc `[−0.5, 0.5]`, which holds the touch, still
/// refuses. Red where a window that misses the arc is kept (the walk's
/// `off_arc`, or `Walked::on_span`'s drop).
#[test]
fn a_touch_off_the_arc_leaves_the_arc_answered() {
    let (ring, s) = measured_touch();
    for eps in [1e-6, 1e-9, 1e-12] {
        let b = band(eps);
        let got = cross(&Span::new(&ring, (1.0, 2.0)), &s, b);
        assert!(got.is_ok(), "ε {eps:e}: the arc off the touch, {got:?}");
        let wrapped = super::circle_torus::circle_torus_roots(&ring, 1.0, 2.0, &s, b);
        assert!(
            matches!(wrapped, Ok(Ok(_))),
            "ε {eps:e}: the wrapper on the arc off the touch, {wrapped:?}"
        );
        let held = cross(&Span::new(&ring, (-0.5, 0.5)), &s, b);
        assert!(
            held.is_err(),
            "ε {eps:e}: the arc through the touch, {held:?}"
        );
    }
}

/// The walk's frame for `F` on the turn about zero, in metres.
fn turn_frame() -> crate::half_angle::SubdivisionFrame<f64> {
    crate::half_angle::SubdivisionFrame {
        t0: -PI,
        t1: PI,
        speed_hi: 1.0,
        noise: 0.0,
        f_per_metre: 1.0,
        f_per_metre_hi: 1.0,
        residual_reach: None,
    }
}

/// `F = cos θ − 1/2`, roots at `±π/3`.
fn half_cosine() -> crate::half_angle::TrigPoly<f64> {
    let mut cos = [0.0; crate::half_angle::MAX_DEGREE + 1];
    cos[0] = -0.5;
    cos[1] = 1.0;
    crate::half_angle::TrigPoly {
        cos,
        sin: [0.0; crate::half_angle::MAX_DEGREE + 1],
        degree: 1,
    }
}

const WALK_ROWS: crate::half_angle::SubdivisionRows = *crate::rows::CONIC_QUADRIC_LADDER.verify();

/// **A turn with one refused piece and one decided crossing answers both,
/// with no fault**: `F = cos θ − ½`, its residual read in the band's gap
/// within `0.01` of `π/3`, so the walk refuses the piece about that root
/// and certifies the one at `−π/3`. An odd count read round the turn,
/// which the parity check would call impossible, is the window's. Red
/// where the parity check runs with the window open.
#[test]
fn a_walk_with_a_window_skips_its_parity_check() {
    let eps = 1e-9;
    let b = band(eps);
    let f = half_cosine();
    let residual = |t: f64| {
        if (t - FRAC_PI_3).abs() < 0.01 {
            3.0 * eps
        } else {
            t.cos() - 0.5
        }
    };
    let got = crate::half_angle::certified_subdivision_past(
        crate::half_angle::Terms::Harmonics(&f),
        &residual,
        &turn_frame(),
        &WALK_ROWS,
        None,
        b,
    )
    .unwrap_or_else(|r| panic!("the walk read past its refusal: {r:?}"));
    assert_eq!(got.roots.len(), 1, "one root decided: {:?}", got.roots);
    assert!((got.roots[0] + FRAC_PI_3).abs() < 1e-9, "{:?}", got.roots);
    assert_eq!(got.refused.len(), 1, "one window: {:?}", got.refused);
    let (lo, hi) = got.refused[0].0;
    assert!(
        lo <= FRAC_PI_3 && FRAC_PI_3 <= hi,
        "the window {:?} holds π/3",
        (lo, hi)
    );
    // The legacy entry stops at the refusal, as before.
    let legacy = crate::half_angle::certified_subdivision(
        crate::half_angle::Terms::Harmonics(&f),
        &residual,
        &turn_frame(),
        &WALK_ROWS,
        None,
        b,
    );
    assert!(legacy.is_err(), "the legacy walk refuses: {legacy:?}");
}

/// **A root whose slack the meter refuses is a window over the root ±
/// its slack**: `F = cos θ − ½` read through a meter whose residual
/// carries a rounding bound of `1e-6`, so each root's slack is about
/// `1.2e-6` rad, past the zero band. Each root's window holds it and is
/// a few slacks wide, far inside its piece (`π/8`). Red where the slack
/// window is widened to the piece or the span.
#[test]
fn a_refused_root_slack_is_a_window_over_the_slack() {
    let eps = 1e-9;
    let b = band(eps);
    let f = half_cosine();
    let residual = |t: f64| t.cos() - 0.5;
    let rounded = |t: f64| Rounded {
        value: t.cos() - 0.5,
        error: 1e-6,
    };
    let meter = crate::half_angle::RootSlack {
        row: "conic_quadric_sub_root_slack",
        residual: &rounded,
        f_per_metre_hi: 1.0,
    };
    let got = crate::half_angle::certified_subdivision_past(
        crate::half_angle::Terms::Harmonics(&f),
        &residual,
        &turn_frame(),
        &WALK_ROWS,
        Some(&meter),
        b,
    )
    .unwrap_or_else(|r| panic!("the walk read past its refusals: {r:?}"));
    assert!(got.roots.is_empty(), "no root certified: {:?}", got.roots);
    assert_eq!(got.refused.len(), 2, "two windows: {:?}", got.refused);
    for (want, &((lo, hi), _)) in [-FRAC_PI_3, FRAC_PI_3].iter().zip(&got.refused) {
        assert!(
            lo <= *want - 1e-6 && *want + 1e-6 <= hi,
            "the window {:?} holds {want} and its slack",
            (lo, hi)
        );
        assert!(
            hi - lo < 1e-5,
            "the window {:?} is the slack's, not the piece's",
            (lo, hi)
        );
    }
}

/// **A graze-window fuzz across the gap** (W1–W4 through `cross`): circles
/// grazing a tilted wall and a torus by depths drawn across `±10ε`, read
/// on random arcs. Every true crossing (`conic_oracle::exact`, double-
/// double) on the span outside every window is a found meeting, within
/// the zero band of arc; no found meeting lies inside a window; every
/// found meeting is a true crossing; the windows are ascending and apart.
/// Red where the walk drops a refused piece's window (its crossing is then
/// neither found nor windowed), or a found meeting is kept inside one.
#[test]
fn graze_windows_hold_every_crossing_the_answer_does_not_find() {
    use super::conic_oracle::exact;
    let mut rng = test_utils::fuzz::start("curve_face_graze_windows");
    let n = test_utils::fuzz::scaled(60);
    let mut windowed = 0usize;
    for eps in [1e-6, 1e-9, 1e-12] {
        let b = band(eps);
        for i in 0..n {
            let (s, on, normal, tangent_seed) = if i % 2 == 0 {
                let r = rng.log_uniform(0.1, 10.0);
                let wall = Surface::Cylinder {
                    frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
                    radius: r,
                };
                let phi = rng.range(0.0, TAU);
                let radial = Vec3::new(phi.cos(), phi.sin(), 0.0);
                (
                    wall,
                    Point3::origin() + radial * r,
                    radial,
                    Vec3::new(-phi.sin(), phi.cos(), 0.0),
                )
            } else {
                let s = torus(Point3::origin(), Vec3::unit_z(), 1.0, 0.25);
                let phi = rng.range(0.0, TAU);
                let radial = Vec3::new(phi.cos(), phi.sin(), 0.0);
                (
                    s,
                    Point3::origin() + radial * 1.25,
                    radial,
                    Vec3::new(-phi.sin(), phi.cos(), 0.0),
                )
            };
            let lift = rng.range(-10.0, 10.0) * eps;
            let g = rng.range(0.0, TAU);
            let tangent = (tangent_seed * g.cos() + Vec3::unit_z() * g.sin()).normalize();
            let rho = rng.log_uniform(0.05, 5.0);
            let c = graze(on, normal, tangent, (rho, lift, rng.range(0.0, TAU)));
            let truth = exact::truth(&c, &s);
            let (m, h) = (rng.range(-PI, PI), rng.range(0.05, PI));
            let span = (m - h, m + h);
            let label = format!(
                "ε {eps:e}, lift {lift:e}: {c:?} — {}",
                test_utils::fuzz::replay()
            );
            let near = |t: f64, u: f64| ((t - u + PI).rem_euclid(TAU) - PI).abs() * rho;
            let on_span = |t: f64| {
                let turn = (m - t).rem_euclid(TAU);
                turn.min(TAU - turn) <= h
            };
            let (found, windows) = match cross(&Span::new(&c, span), &s, b) {
                Ok(got) => (got.meetings, Vec::new()),
                Err(r) => (r.found, r.windows),
            };
            windowed += usize::from(!windows.is_empty());
            let inside = |t: f64| {
                windows.iter().any(|w| {
                    let lo = w.window.0 - eps / rho;
                    let hi = w.window.1 + eps / rho;
                    (0..3).any(|k| {
                        let t = t + TAU * f64::from(k - 1);
                        lo <= t && t <= hi
                    })
                })
            };
            for w in windows.windows(2) {
                assert!(
                    w[0].window.1 < w[1].window.0,
                    "{label}: windows apart, ascending"
                );
            }
            for w in &windows {
                assert!(
                    w.why.fault().is_none(),
                    "{label}: a fault on a graze, {:?}",
                    w.why
                );
            }
            let at = |m: &super::Meeting<f64>| match m.on_span {
                super::OnSpan::Inside { t } => t,
                super::OnSpan::End(crate::curve_pair::SpanEnd::Start) => span.0,
                super::OnSpan::End(crate::curve_pair::SpanEnd::Stop) => span.1,
            };
            for meeting in &found {
                let t = at(meeting);
                let off = truth
                    .roots
                    .iter()
                    .map(|&u| near(t, u))
                    .fold(f64::INFINITY, f64::min);
                let touch = matches!(meeting.kind, super::MeetingKind::Touch { .. });
                assert!(
                    touch || off <= 10.0 * eps,
                    "{label}: a found meeting at {t} is {off:e} m of arc from every crossing"
                );
                if let super::OnSpan::Inside { t } = meeting.on_span {
                    assert!(
                        !windows.iter().any(|w| w.window.0 < t && t < w.window.1),
                        "{label}: a found meeting at {t} lies inside a window"
                    );
                }
            }
            // A crossing within the escalation band of an end is the end's.
            let reach = b.escalate() / rho;
            let clear_of_ends = |t: f64| {
                [span.0, span.1]
                    .iter()
                    .all(|&e| near(t, e) > 2.0 * reach * rho)
            };
            for &u in truth
                .roots
                .iter()
                .filter(|&&u| on_span(u) && clear_of_ends(u))
            {
                if inside(u) {
                    continue;
                }
                let got = found
                    .iter()
                    .map(|m| near(at(m), u))
                    .fold(f64::INFINITY, f64::min);
                assert!(
                    got <= 10.0 * eps,
                    "{label}: the crossing at {u} outside every window is not found \
                     ({windows:?})"
                );
            }
        }
    }
    assert!(windowed > 0, "the fuzz read no window at all");
}

/// **Overlapping windows are one, a fault's why ahead** (W4): a method's
/// escalation over `[0.1, 0.3]` and a fault over `[0.2, 0.5]` read in
/// that order merge into `[0.1, 0.5]`, its why the fault; a window apart
/// by `0.2` stays apart. Red where the merge keeps the earlier why.
#[test]
fn overlapping_windows_merge_and_a_fault_is_never_merged_away() {
    use crate::curve_pair::MeetFault;
    let c = hanging(1.0, -0.5);
    let span = Span::new(&c, (0.0, 1.0));
    let b = band(1e-9);
    let diag = geom_core::k_stats::decide("curve_face_window", geom_core::Margin::of(5e-9), b)
        .expect_err("a reading in the gap");
    let door = super::Door::PlaneCrossing;
    let window = |lo: f64, hi: f64, fault: MeetFault| super::Undecided {
        window: (lo, hi),
        why: CrossWhy::Method { door, fault },
    };
    let merged = super::cross::merged(
        &span,
        vec![
            window(0.1, 0.3, MeetFault::Escalated(diag)),
            window(0.7, 0.8, MeetFault::Escalated(diag)),
            window(
                0.2,
                0.5,
                MeetFault::Contradiction {
                    readings: "a row's fault",
                },
            ),
        ],
        b,
    );
    assert_eq!(merged.len(), 2, "{merged:?}");
    assert_eq!(merged[0].window, (0.1, 0.5), "{merged:?}");
    assert!(
        merged[0].why.fault().is_some(),
        "the fault's why: {merged:?}"
    );
    assert_eq!(merged[1].window, (0.7, 0.8), "{merged:?}");
}

/// **A window past an end by a gap in band is the end's escalation**
/// (W3, `on_span`): a window starting a hair past the span's end, which
/// the band cannot call apart from it, is held to the span as its last
/// `Kε` of arc, never an inverted window; one past by a clear gap is
/// dropped. Red where the clip keeps the inverted bounds.
#[test]
fn a_window_past_an_end_in_band_is_held_to_the_end() {
    let c = hanging(1.0, -0.5);
    let span = Span::new(&c, (0.0, 1.0));
    let b = band(1e-9);
    let past = (1.0 + 2e-9, 1.5);
    let held = super::on_span(&span, past, b);
    let [(lo, hi)] = held[..] else {
        panic!("a gap in band keeps one window: {held:?}")
    };
    assert!(
        lo < hi && hi <= 1.0 && 1.0 - lo <= 2.0 * b.escalate(),
        "{:?}",
        (lo, hi)
    );
    assert!(
        super::on_span(&span, (1.1, 1.5), b).is_empty(),
        "a clear gap drops it"
    );
}

/// **A first harmonic's refused root slack is a window over each root,
/// and off the arc it refuses nothing**: a circle of radius `0.5` in
/// `z = 0` about `(0.5 + δ, 0, 0)`, crossing the unit sphere `δ = 1e-8`
/// deep at `θ = 0`, at `ε = 1e-12`. Its roots, `±2.8e-4` rad apart, are
/// too shallow for the root slack to fall inside the band. Over the turn
/// the answer is a window, local, holding both true crossings; over the
/// arc `[2, 4]`, a radian and more from them, `cross` and the sweep's
/// wrapper answer with no window. Red where the slack's window is the
/// whole span, where a window that misses the span is kept, or where a
/// wrapper reads every window rather than the first that meets the span.
#[test]
fn a_first_harmonics_refused_slack_is_local_and_refuses_nothing_off_the_arc() {
    let b = band(1e-12);
    let sphere = Surface::Sphere {
        frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        radius: 1.0,
    };
    let delta = 1e-8;
    let c = Curve3::circle(
        Point3::new(0.5 + delta, 0.0, 0.0),
        Vec3::unit_z(),
        0.5,
        Vec3::unit_x(),
    );
    // The crossings in double-double: `|p|² = 1` at `cos θ = (1 − d² −
    // ρ²)/(2dρ)`, read here as `θ ≈ ±√(2(1 − cos θ))` to first order.
    let d = 0.5 + delta;
    let one_minus = Dd::of(1.0).sub(Dd::of(d).add(Dd::of(0.5)).mul(Dd::of(d).add(Dd::of(0.5))));
    let theta = (2.0 * -one_minus.to_f64() / (2.0 * d * 0.5)).sqrt();
    match cross(&Span::new(&c, (-PI, PI)), &sphere, b) {
        Err(r) => {
            assert!(r.found.is_empty(), "{r:?}");
            for root in [-theta, theta] {
                assert!(
                    r.windows
                        .iter()
                        .any(|w| w.window.0 <= root && root <= w.window.1),
                    "the crossing at {root} lies in a window: {r:?}"
                );
            }
            for w in &r.windows {
                assert!(w.window.1 - w.window.0 < 0.1, "a local window: {r:?}");
            }
        }
        Ok(got) => panic!("the turn's slack refuses: {got:?}"),
    }
    let arc = cross(&Span::new(&c, (2.0, 4.0)), &sphere, b);
    assert!(
        matches!(&arc, Ok(got) if got.meetings.is_empty()),
        "the arc off the crossings: {arc:?}"
    );
    let wrapped = super::conic_quadric::conic_quadric_roots(&c, 2.0, 4.0, &sphere, b);
    assert!(
        matches!(wrapped, Ok(Ok(super::circle_roots::CircleRoots::Miss))),
        "the wrapper on the arc: {wrapped:?}"
    );
}

/// **No found meeting lies in a window** (W1, W2, `finish`): a meeting
/// inside a window, and one past its edge by a gap the band cannot read,
/// are taken into it, the window then reaching `Kε` of arc past the
/// latter; one apart from every window is found. Red where a meeting
/// inside a window is kept, or where the taken one's hull is not widened
/// by its escalation.
#[test]
fn a_meeting_inside_a_window_is_taken_into_it() {
    use crate::curve_pair::MeetFault;
    let c = hanging(1.0, -0.5);
    let span = Span::new(&c, (0.0, 1.0));
    let b = band(1e-9);
    let meeting = |t: f64| {
        (
            super::Meeting {
                point: c.eval(t),
                on_span: super::OnSpan::Inside { t },
                kind: super::MeetingKind::Cross,
                chart: None,
            },
            t,
        )
    };
    let window = super::Undecided {
        window: (0.4, 0.6),
        why: CrossWhy::Method {
            door: super::Door::PlaneCrossing,
            fault: MeetFault::Unsettled(crate::roots::Unsettled::Tangency),
        },
    };
    // Past the window's edge by five zero bands, in the band's gap: the
    // band cannot call it apart from the window.
    let edge = 0.6 + 5e-9;
    let got = super::cross::finish(
        &span,
        Pose::Apart,
        vec![meeting(0.5), meeting(edge), meeting(0.9)],
        vec![window],
        b,
    );
    let Err(r) = got else {
        panic!("a window stands: {got:?}")
    };
    assert_eq!(r.found.len(), 1, "{r:?}");
    assert!(
        matches!(r.found[0].on_span, super::OnSpan::Inside { t } if t == 0.9),
        "{r:?}"
    );
    assert_eq!(r.windows.len(), 1, "{r:?}");
    // The circle's speed is one: a metre of arc per radian.
    let (lo, hi) = r.windows[0].window;
    assert_eq!(lo, 0.4, "{r:?}");
    assert!(
        hi >= edge + 0.99 * b.escalate() && hi <= edge + 1.01 * b.escalate(),
        "the window {:?} reaches Kε past the taken meeting at {edge}",
        (lo, hi)
    );
}

/// **Windows merged keep the earlier one's why** (W4, `merged`): two
/// escalations read in turn, the later read starting first, are one
/// window whose why is the one starting first. Red where the merged why
/// is the first read.
#[test]
fn merged_windows_keep_the_why_of_the_one_starting_first() {
    use crate::curve_pair::MeetFault;
    let c = hanging(1.0, -0.5);
    let span = Span::new(&c, (0.0, 1.0));
    let b = band(1e-9);
    let diag = geom_core::k_stats::decide("curve_face_window", geom_core::Margin::of(5e-9), b)
        .expect_err("a reading in the gap");
    let read_first = super::Undecided {
        window: (0.4, 0.6),
        why: CrossWhy::Method {
            door: super::Door::PlaneCrossing,
            fault: MeetFault::Escalated(diag),
        },
    };
    let starts_first = super::Undecided {
        window: (0.1, 0.5),
        why: CrossWhy::Method {
            door: super::Door::PlaneCrossing,
            fault: MeetFault::Unsettled(crate::roots::Unsettled::Tangency),
        },
    };
    let merged = super::cross::merged(&span, vec![read_first, starts_first], b);
    assert_eq!(merged.len(), 1, "{merged:?}");
    assert_eq!(merged[0].window, (0.1, 0.6), "{merged:?}");
    assert!(
        matches!(
            merged[0].why,
            CrossWhy::Method {
                fault: MeetFault::Unsettled(_),
                ..
            }
        ),
        "the why of the window starting first: {merged:?}"
    );
}

/// **A line × cone root whose slack the meter refuses is a window over
/// the root ± its slack** (`line_quadric_cone_root_slack`): a line at
/// height 1 passing `δ = 1e-6` inside the cone of half-angle `0.5`, at
/// `ε = 1e-12`. Its two shallow roots are each in a local window, which
/// holds the double-double root `2 ± √(δ(2R − δ))`. Red where the slack's
/// window is the whole span.
#[test]
fn a_line_cones_refused_slack_is_a_window_over_each_root() {
    let b = band(1e-12);
    let cone = Surface::Cone {
        frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        half_angle: 0.5,
    };
    let r = 0.5_f64.tan();
    let y = r - 1e-6;
    let l = Curve3::Line(geom::Line {
        origin: Point3::new(-2.0, y, 1.0),
        dir: geom_core::UnitVec3::exact_x(),
    });
    let depth = Dd::of(r).sub(Dd::of(y));
    let half = depth.mul(Dd::of(2.0 * r).sub(depth)).sqrt().to_f64();
    let Err(got) = cross(&Span::new(&l, (0.0, 4.0)), &cone, b) else {
        panic!("the slack refuses")
    };
    for root in [2.0 - half, 2.0 + half] {
        assert!(
            got.windows.iter().any(|w| w.window.0 <= root
                && root <= w.window.1
                && w.window.1 - w.window.0 < 1e-6),
            "a local window holds the root at {root}: {got:?}"
        );
    }
}

/// **A line along a cone's generator, beside its apex in band, is
/// refused over the whole of what it lies near** (the apex window is
/// local only once `Lies` is excluded): a π/4 cone, the line
/// `(0, 3ε, 0) + s·(1, 0, 1 + tilt)` on `[−10, 10]`, tilted off the
/// generator by `tilt`. At these tilts (`1e-12` at `ε = 1e-9`; `1e-12`
/// to `1e-8` at `ε = 1e-6`, each `Lies` unexcluded) every point of the
/// span within `Kε` of the cone (by its form `ρ cos α − |z| sin α`, the
/// distance to the nearest generator) lies inside a window, through
/// `pose` and `cross` alike. That is the row's claim, not a general one: at
/// a tilt that excludes `Lies` the apex window is local, and near-cone
/// points off the crossings may stand outside it; what holds there is
/// that every crossing does. Red where the apex window is answered before
/// `Lies` is read: it is then `±2e-3` about the apex, with the rest of the
/// line within `ε` outside it.
#[test]
fn a_line_lying_near_a_cone_by_its_apex_is_refused_over_all_it_lies_near() {
    let cone = Surface::Cone {
        frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        half_angle: core::f64::consts::FRAC_PI_4,
    };
    let span = (-10.0, 10.0);
    for (eps, tilts) in [(1e-9, &[1e-12][..]), (1e-6, &[1e-12, 1e-10, 1e-8][..])] {
        let b = band(eps);
        for &tilt in tilts {
            let l = Curve3::Line(geom::Line {
                origin: Point3::new(0.0, 3.0 * eps, 0.0),
                dir: geom_core::UnitVec3::squared(Vec3::new(1.0, 0.0, 1.0 + tilt)),
            });
            let label = format!("ε {eps:e}, tilt {tilt:e}");
            let form = |t: f64| {
                let p = l.eval(t);
                let s = core::f64::consts::FRAC_1_SQRT_2;
                (p.x.powi(2) + p.y.powi(2)).sqrt() * s - p.z.abs() * s
            };
            let (window, why) = refused(&label, pose(&Span::new(&l, span), &cone, b));
            let near = |t: f64| form(t).abs() < b.escalate();
            let dense = (0..=200_000).map(|i| span.0 + (span.1 - span.0) * f64::from(i) / 2e5);
            for t in dense.clone().filter(|&t| near(t)) {
                assert!(
                    window.0 <= t && t <= window.1,
                    "{label}: the point at {t}, {:e} from the cone, lies outside the pose's \
                     window {window:?} ({why:?})",
                    form(t)
                );
            }
            let windows = match cross(&Span::new(&l, span), &cone, b) {
                Ok(got) => panic!("{label}: cross answered {got:?}"),
                Err(r) => r.windows,
            };
            for t in dense.filter(|&t| near(t)) {
                assert!(
                    windows.iter().any(|w| w.window.0 <= t && t <= w.window.1),
                    "{label}: the point at {t} lies outside every window of cross, {windows:?}"
                );
            }
        }
    }
}

/// Every true crossing of `c` with `s` on the span, clear of its ends,
/// is found or lies in a window, at any turn; every found meeting is a
/// true one. `roots` are the true crossings in `[0, 2π)`; `rho` the
/// carrier's radius.
fn every_crossing_answered(
    label: &str,
    c: &Curve3<f64>,
    s: &Surface<f64>,
    span: (f64, f64),
    roots: &[f64],
    rho: f64,
    b: Band,
) {
    let eps = b.zero();
    let near = |t: f64, u: f64| ((t - u + PI).rem_euclid(TAU) - PI).abs() * rho;
    let (m, h) = ((span.0 + span.1) / 2.0, (span.1 - span.0) / 2.0);
    let on_span = |t: f64| {
        let turn = (m - t).rem_euclid(TAU);
        turn.min(TAU - turn) <= h
    };
    let (found, windows) = match cross(&Span::new(c, span), s, b) {
        Ok(got) => (got.meetings, Vec::new()),
        Err(r) => (r.found, r.windows),
    };
    let at = |m: &super::Meeting<f64>| match m.on_span {
        super::OnSpan::Inside { t } => t,
        super::OnSpan::End(crate::curve_pair::SpanEnd::Start) => span.0,
        super::OnSpan::End(crate::curve_pair::SpanEnd::Stop) => span.1,
    };
    let inside = |t: f64| {
        windows.iter().any(|w| {
            (0..3).any(|k| {
                let t = t + TAU * f64::from(k - 1);
                w.window.0 - eps / rho <= t && t <= w.window.1 + eps / rho
            })
        })
    };
    for w in &windows {
        assert!(
            span.0 <= w.window.0 && w.window.0 < w.window.1 && w.window.1 <= span.1,
            "{label}: the window {:?} is on the span",
            w.window
        );
    }
    for meeting in &found {
        let t = at(meeting);
        let touch = matches!(meeting.kind, super::MeetingKind::Touch { .. });
        let off = roots
            .iter()
            .map(|&u| near(t, u))
            .fold(f64::INFINITY, f64::min);
        assert!(
            touch || off <= 10.0 * eps,
            "{label}: a found meeting at {t} is {off:e} m from every crossing"
        );
    }
    let reach = b.escalate();
    for &u in roots
        .iter()
        .filter(|&&u| on_span(u) && near(u, span.0) > 2.0 * reach && near(u, span.1) > 2.0 * reach)
    {
        if inside(u) {
            continue;
        }
        let got = found
            .iter()
            .map(|m| near(at(m), u))
            .fold(f64::INFINITY, f64::min);
        assert!(
            got <= 10.0 * eps,
            "{label}: the crossing at {u} is neither found nor in a window \
             (found {found:?}, windows {windows:?})"
        );
    }
}

/// A circle of radius `r` in `y = 0` about `center`, its point at
/// `θ = 0` at `center + r·u`.
fn ring(center: Point3<f64>, r: f64, u: Vec3<f64>) -> Curve3<f64> {
    Curve3::circle(center, Vec3::unit_y(), r, u)
}

/// **A window across the seam holds its crossings at either end**
/// (W3 at the turn): a circle dipping into each window source a few
/// zero bands deep at `θ = 0`, read on arcs that begin and end at (or a
/// hair inside) the seam — a plane's belly, a torus's walk, a wall's
/// ladder, a sphere's first-harmonic extreme, each dipped `3ε`, `30ε` and
/// a slack's depth. Each true crossing clear of the arc's ends is found
/// or lies in a window, at every `ε`. Red where a window crossing the
/// seam is clipped at the span's ends by its one copy (it then keeps
/// only `(2π − w, 2π)` and drops the crossing at `θ ≈ w`).
#[test]
fn a_window_across_the_seam_holds_its_crossings_at_either_end() {
    use super::conic_oracle::exact;
    let spans = [(0.0, TAU), (1e-5, TAU - 1e-5), (1e-7, TAU - 1e-7)];
    for eps in [1e-6, 1e-9, 1e-12] {
        let b = band(eps);
        for dip in [3.0 * eps, 30.0 * eps, 1e-8_f64.max(300.0 * eps)] {
            // A plane: the unit circle's belly `dip` under `z = 0`.
            let z0 = 1.0 - dip;
            let belly = ring(Point3::new(0.0, 0.0, z0), 1.0, Vec3::new(0.0, 0.0, -1.0));
            let w = 2.0 * ((1.0 - z0) / 2.0).sqrt().asin();
            // A torus (about z, radii 1 and 1/4), a wall (radius 1 about
            // z) and the unit sphere, each entered at `θ = 0` from
            // outside along −x.
            let torus = torus(Point3::origin(), Vec3::unit_z(), 1.0, 0.25);
            let wall = Surface::Cylinder {
                frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
                radius: 1.0,
            };
            let sphere = Surface::Sphere {
                frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
                radius: 1.0,
            };
            let in_x = Vec3::new(-1.0, 0.0, 0.0);
            let cases = [
                (
                    "torus",
                    ring(Point3::new(1.75 - dip, 0.0, 0.0), 0.5, in_x),
                    torus,
                ),
                (
                    "wall",
                    ring(Point3::new(1.5 - dip, 0.0, 0.0), 0.5, in_x),
                    wall,
                ),
                (
                    "sphere",
                    ring(Point3::new(1.5 - dip, 0.0, 0.0), 0.5, in_x),
                    sphere,
                ),
            ];
            for &span in &spans {
                let label = format!("plane, ε {eps:e}, dip {dip:e}, span {span:?}");
                every_crossing_answered(&label, &belly, &plane_z(), span, &[w, TAU - w], 1.0, b);
                for (name, c, s) in &cases {
                    let label = format!("{name}, ε {eps:e}, dip {dip:e}, span {span:?}");
                    let truth = exact::truth(c, s);
                    every_crossing_answered(&label, c, s, span, &truth.roots, 0.5, b);
                }
            }
        }
    }
}

/// **A touch on the arc near its start, the arc ending short of the
/// turn, is never a miss** (the measured touch turned to `θ = a`): the
/// circle × torus touch of [`measured_touch`] turned so it sits at
/// `θ = a ∈ {1e-3, 2e-3}`, read on `[0, 2π − 0.003]` at `ε = 1e-6`;
/// the touch's window runs back past `θ = 0` to the arc's far end. `cross`
/// answers a meeting there or refuses, and the sweep's wrapper never
/// answers `Miss`. Red where the window's copy is clipped at the span's
/// start alone and then judged against the far end (both answered a miss).
#[test]
fn a_touch_near_the_start_of_an_arc_short_of_the_turn_is_never_a_miss() {
    let (_, s) = measured_touch();
    let b = band(1e-6);
    for a in [1e-3_f64, 2e-3] {
        let ring = ring(
            Point3::new(1.75, 0.0, 0.0),
            0.5,
            Vec3::new(-a.cos(), 0.0, -a.sin()),
        );
        let label = format!("a = {a:e}");
        let span = (0.0, TAU - 0.003);
        match cross(&Span::new(&ring, span), &s, b) {
            Ok(got) => assert!(
                got.meetings
                    .iter()
                    .any(|m| (m.point - Point3::new(1.25, 0.0, 0.0)).norm() < 1e-3),
                "{label}: cross answered without the touch, {got:?}"
            ),
            Err(r) => assert!(
                r.windows.iter().any(|w| w.window.0 <= a && a <= w.window.1),
                "{label}: no window holds the touch, {r:?}"
            ),
        }
        let wrapped = super::circle_torus::circle_torus_roots(&ring, span.0, span.1, &s, b);
        assert!(
            !matches!(wrapped, Ok(Ok(crate::roots::Roots::Miss))),
            "{label}: the wrapper answered a miss, {wrapped:?}"
        );
    }
}

/// **A crossing in band of an end, just before it, is a window on the
/// span** (W3 for the placement's `End` window): the line `z = t` on
/// `[3ε, 1]` crosses `z = 0` at `t = 0`, `3ε` before the span's start,
/// which the band cannot call apart from it. The window the placement
/// answers, `(−Kε, Kε)` about the root, is held to the span: it starts
/// at `3ε` and holds the start. Red where the placement's window is left
/// unclipped.
#[test]
fn a_crossing_in_band_just_before_the_start_is_a_window_on_the_span() {
    for eps in [1e-6, 1e-9, 1e-12] {
        let b = band(eps);
        let l = Curve3::line(Point3::origin(), geom_core::UnitVec3::exact_z());
        let span = (3.0 * eps, 1.0);
        let label = format!("ε {eps:e}");
        let windows = match cross(&Span::new(&l, span), &plane_z(), b) {
            Ok(got) => {
                assert!(
                    got.meetings.iter().all(|m| matches!(
                        m.on_span,
                        super::OnSpan::End(crate::curve_pair::SpanEnd::Start)
                    )),
                    "{label}: {got:?}"
                );
                continue;
            }
            Err(r) => r.windows,
        };
        assert!(!windows.is_empty(), "{label}");
        for w in &windows {
            assert!(
                span.0 <= w.window.0 && w.window.0 < w.window.1 && w.window.1 <= span.1,
                "{label}: the window {:?} is on the span {span:?}",
                w.window
            );
        }
        assert!(
            windows.iter().any(|w| w.window.0 == span.0),
            "{label}: a window holds the start, {windows:?}"
        );
    }
}

/// **Every stretch the band cannot read lies in a window** (W1, W2,
/// through `cross`): each stretch of the span where `unread` (within `Kε`
/// of the surface by its residual), its edges bisected to the bit, lies in
/// a window, unless it holds a meeting `cross` found (a crossing's own
/// band, which the answer places). W2's `Kε` of arc about every meeting
/// then holds to first order by itself: the residual is `0` at a meeting
/// and `Kε` at the stretch's edge, and moves near the surface at most
/// `(1 + Kε/r)` metres per metre of arc (a torus's `(g² − r²)/2r` at
/// `g/r`; a wall's and a sphere's alike), so the arc between them is at
/// least `Kε/(1 + Kε/r)`: `Kε`, short by a relative `O(Kε/r)`. `_speed`
/// is the carrier's least, kept for the callers' reading.
fn every_unread_stretch_is_in_a_window(
    label: &str,
    c: &Curve3<f64>,
    s: &Surface<f64>,
    span: (f64, f64),
    _speed: f64,
    b: Band,
    unread: impl Fn(f64) -> bool,
) {
    let (found, windows) = match cross(&Span::new(c, span), s, b) {
        Ok(got) => (got.meetings, Vec::new()),
        Err(r) => (r.found, r.windows),
    };
    assert!(!windows.is_empty(), "{label}: answered {found:?}");
    let inside = |t: f64| windows.iter().any(|w| w.window.0 <= t && t <= w.window.1);
    let periodic = c.period().is_some();
    let met = |(lo, hi): (f64, f64)| {
        found.iter().any(|m| {
            let t = match m.on_span {
                super::OnSpan::Inside { t } => t,
                super::OnSpan::End(crate::curve_pair::SpanEnd::Start) => span.0,
                super::OnSpan::End(crate::curve_pair::SpanEnd::Stop) => span.1,
            };
            let turns: &[f64] = if periodic {
                &[-2.0, -1.0, 0.0, 1.0, 2.0]
            } else {
                &[0.0]
            };
            turns.iter().any(|&k| (lo..=hi).contains(&(t + TAU * k)))
        })
    };
    let n = 200_000u32;
    let at = |i: u32| span.0 + (span.1 - span.0) * f64::from(i) / f64::from(n);
    let edge = |(mut a, mut z): (f64, f64)| {
        let inner = unread(a);
        for _ in 0..80 {
            let m = geom::mid_param(a, z);
            if unread(m) == inner {
                a = m;
            } else {
                z = m;
            }
        }
        a
    };
    // The unread stretches, each edge bisected between the samples astride it.
    let mut stretches = Vec::new();
    let mut start = unread(span.0).then_some(span.0);
    for i in 0..n {
        let (a, z) = (at(i), at(i + 1));
        match (unread(a), unread(z), start) {
            (false, true, None) => start = Some(edge((z, a))),
            (true, false, Some(from)) => {
                stretches.push((from, edge((a, z))));
                start = None;
            }
            _ => {}
        }
    }
    if let Some(from) = start {
        stretches.push((from, span.1));
    }
    for (lo, hi) in stretches {
        if met((lo, hi)) {
            continue;
        }
        for t in [lo, geom::mid_param(lo, hi), hi] {
            assert!(
                inside(t),
                "{label}: the point at {t}, in the unread stretch ({lo}, {hi}), lies outside \
                 every window {windows:?}"
            );
        }
    }
}

/// [`every_unread_stretch_is_in_a_window`] by the surface's implicit
/// residual, which the band classifies.
fn every_unread_point_is_in_a_window(
    label: &str,
    c: &Curve3<f64>,
    s: &Surface<f64>,
    span: (f64, f64),
    speed: f64,
    b: Band,
) {
    let k_eps = b.escalate();
    every_unread_stretch_is_in_a_window(label, c, s, span, speed, b, |t| {
        crate::implicit_residual(s, c.eval(t)).abs() < k_eps
    });
}

/// **Each window source holds what the band cannot read** (W1, W2 by
/// oracles independent of the windows' own sums): a circle × torus touch
/// (the walk's refused piece) and a circle's belly and first-harmonic
/// extreme in band (the graze windows), each holding every stretch within
/// `Kε` of the surface by its residual, edges bisected; a line crossing both nappes `3ε`
/// from a cone's apex (the apex window), holding each true crossing and
/// `Kε` of arc about it; a circle crossing a
/// sphere `1e-8` deep (the first harmonic's refused slack) and a line
/// crossing a cone `1e-6` deep (the line × cone slack), each holding its
/// double-double root and `Kε` of arc about it. Red where any of them is
/// understated: a method window's `Kε` widening dropped, the apex's `+Kε`
/// dropped, or a slack dropped or shrunk to its root.
#[test]
fn each_window_source_holds_what_the_band_cannot_read() {
    let (ring, torus) = measured_touch();
    for eps in [1e-6, 1e-9, 1e-12] {
        every_unread_point_is_in_a_window(
            &format!("torus touch, ε {eps:e}"),
            &ring,
            &torus,
            (-0.5, 0.5),
            0.5,
            band(eps),
        );
    }
    let b = band(1e-9);
    let cone = Surface::Cone {
        frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        half_angle: 0.5,
    };
    let sphere = Surface::Sphere {
        frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        radius: 1.0,
    };
    // A refused root's window holds the true root `Kε` of arc inside its
    // edges, the root in double-double. The circle about `(d, 0, 0)` of
    // radius `ρ = ½` meets the unit sphere where
    // `1 − cos θ = ((d + ρ)² − 1)/(2dρ)`, so `θ = ±2 asin √(that/2)`.
    let b12 = band(1e-12);
    let d = 0.5 + 1e-8;
    let c = Curve3::circle(
        Point3::new(d, 0.0, 0.0),
        Vec3::unit_z(),
        0.5,
        Vec3::unit_x(),
    );
    let lift = Dd::of(d)
        .add(Dd::of(0.5))
        .mul(Dd::of(d).add(Dd::of(0.5)))
        .sub(Dd::of(1.0));
    let half_chord = lift.mul(Dd::of(1.0 / (4.0 * d * 0.5))).sqrt();
    let theta = 2.0 * asin_dd(half_chord);
    let root_held =
        |label: &str, windows: &[super::Undecided<f64, CrossWhy>], root: f64, reach: f64| {
            assert!(
                windows
                    .iter()
                    .any(|w| w.window.0 <= root - reach && root + reach <= w.window.1),
                "{label}: no window holds the root {root} and Kε of arc either side: {windows:?}"
            );
        };
    // A line steeper than the cone's generators passing `3ε` from its
    // apex, crossing both nappes beside it: each true crossing of the
    // double cone (its form, sampled and bisected) lies in a window with
    // `Kε` of arc either side.
    let dir = Vec3::new(1.3_f64.cos(), 0.0, 1.3_f64.sin());
    let steep = Curve3::Line(geom::Line {
        origin: Point3::new(0.0, 3e-9, 0.0) - dir * 3.0,
        dir: geom_core::UnitVec3::squared(dir),
    });
    let (sin, cos) = 0.5_f64.sin_cos();
    let form = |t: f64| {
        let p = steep.eval(t);
        (p.x.powi(2) + p.y.powi(2)).sqrt() * cos - p.z.abs() * sin
    };
    let roots = super::conic_oracle::crossings(form, 0.0, 6.0);
    assert!(
        !roots.is_empty(),
        "apex: the line crosses the cone beside its apex"
    );
    let Err(r) = cross(&Span::new(&steep, (0.0, 6.0)), &cone, b) else {
        panic!("apex: the apex refuses")
    };
    for root in roots {
        root_held("apex", &r.windows, root, b.escalate());
    }
    let Err(r) = cross(&Span::new(&c, (-PI, PI)), &sphere, b12) else {
        panic!("the first harmonic's slack refuses")
    };
    for root in [-theta, theta] {
        root_held(
            "first harmonic's slack",
            &r.windows,
            root,
            b12.escalate() / 0.5,
        );
    }
    let rc = 0.5_f64.tan();
    let y = rc - 1e-6;
    let l = Curve3::Line(geom::Line {
        origin: Point3::new(-2.0, y, 1.0),
        dir: geom_core::UnitVec3::exact_x(),
    });
    let depth = Dd::of(rc).sub(Dd::of(y));
    let half = depth.mul(Dd::of(2.0 * rc).sub(depth)).sqrt().to_f64();
    let Err(r) = cross(&Span::new(&l, (0.0, 4.0)), &cone, b12) else {
        panic!("the line × cone slack refuses")
    };
    for root in [2.0 - half, 2.0 + half] {
        root_held("line × cone slack", &r.windows, root, b12.escalate());
    }
    for lift in [-3e-9, 3e-9] {
        let c = hanging(1.0, lift);
        every_unread_point_is_in_a_window(
            &format!("belly, lift {lift:e}"),
            &c,
            &plane_z(),
            (0.0, TAU),
            1.0,
            b,
        );
        let c = Curve3::circle(
            Point3::new(0.3, 0.0, 0.0),
            Vec3::unit_z(),
            0.7 + lift,
            Vec3::unit_y(),
        );
        every_unread_point_is_in_a_window(
            &format!("first harmonic, lift {lift:e}"),
            &c,
            &sphere,
            (-PI, PI),
            0.7,
            b,
        );
    }
}

/// **A span starting just past a graze's reach, by a gap the band cannot
/// read, is refused** (`reaches`, a gap in band reads in band): the
/// unit circle's belly `3ε` under `z = 0`, read from `5ε` (the band's
/// gap) and `ε/2` (its zero band) of arc past the reach the pose answers
/// on the whole turn. Red where an overlap in either is read `Apart`: the
/// span is then answered with no meeting.
#[test]
fn a_span_just_past_a_grazes_reach_is_refused() {
    let eps = 1e-9;
    let b = band(eps);
    let c = hanging(1.0, -3.0 * eps);
    let (window, _) = refused("belly", pose(&Span::new(&c, (0.0, PI)), &plane_z(), b));
    for gap in [5.0 * eps, 0.5 * eps] {
        let span = (window.1 + gap, PI);
        let got = pose(&Span::new(&c, span), &plane_z(), b);
        assert!(
            got.is_err(),
            "a reach the band cannot call apart from the span by {gap:e}: {got:?}"
        );
        if gap > eps {
            assert!(
                matches!(got, Err(ref u) if matches!(u.why, PoseRefusal::InBand { .. })),
                "the gap's refusal is in band: {got:?}"
            );
        }
        assert!(
            cross(&Span::new(&c, span), &plane_z(), b).is_err(),
            "cross refuses it too, by {gap:e}"
        );
    }
}

/// **A walk's refused piece holds what the band cannot read, `Kε` of arc
/// wider** (W2, the walk's own windows): circles touching a torus's tube
/// (the circle × torus walk) and a unit wall (the conic ladder's walk) at
/// `θ = 0`, tilted off the touch plane so neither pose's graze rung reads
/// them, each `0` and `±3ε` off at `ε = 1e-9`. Each refusal is the door's
/// method's local window, and every stretch within `Kε` of the surface,
/// its edges bisected, lies in it. Red where a walk's refused piece is
/// dropped or narrowed past what the band cannot read. (The `Kε` a method
/// window is widened by adds nothing a meeting needs here, to a relative
/// `O(Kε/r)`: a refused piece ends at cuts the walk read past `Kε`, and
/// the residual moves at most `(1 + Kε/r)` metres per metre of arc near
/// the surface, so every meeting inside stands `Kε/(1 + Kε/r)` of arc
/// from them already.)
#[test]
fn a_walks_refused_piece_holds_what_the_band_cannot_read() {
    let eps = 1e-9;
    let b = band(eps);
    let t = torus(Point3::origin(), Vec3::unit_z(), 1.0, 0.25);
    let wall = Surface::Cylinder {
        frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        radius: 1.0,
    };
    let cases = [
        (
            "torus walk",
            &t,
            Point3::new(1.25, 0.0, 0.0),
            0.7_f64,
            0.3,
            super::Door::ConicTorus,
        ),
        (
            "wall ladder",
            &wall,
            Point3::new(1.0, 0.0, 0.0),
            0.3,
            0.5,
            super::Door::ConicQuadric,
        ),
    ];
    for (name, s, on, g, rho, door) in cases {
        for lift in [-3.0 * eps, 0.0, 3.0 * eps] {
            let tangent = (Vec3::unit_y() * g.cos() + Vec3::unit_z() * g.sin()).normalize();
            let c = graze(on, Vec3::unit_x(), tangent, (rho, lift, 0.4));
            let label = format!("{name}, lift {lift:e}");
            let span = (-1.0, 1.0);
            let Err(r) = cross(&Span::new(&c, span), s, b) else {
                panic!("{label}: the touch refuses")
            };
            assert!(
                r.windows.iter().all(
                    |w| matches!(w.why, CrossWhy::Method { door: d, .. } if d == door)
                        && w.window.1 - w.window.0 < 0.1
                ),
                "{label}: the method's local windows, {:?}",
                r.windows
            );
            every_unread_point_is_in_a_window(&label, &c, s, span, rho, b);
        }
    }
}

/// **A span of several turns holds every turn's crossings** (`on_span`,
/// copies by every whole period that reaches the span): the unit circle's
/// belly dipped `3ε`, `30ε` and `1e-8` under `z = 0` at `θ = 0`, read on
/// `[0, 4π]` and `[0, 6π]`, its window across the seam at each turn. Each
/// crossing `2πk ± w` clear of the span's ends lies in a window at its
/// own turn, or is found (a found crossing is reported once, within half
/// a turn of the span's middle), at three `ε`. A torus walk's touch, its
/// window the method's, holds what the band cannot read at every turn of
/// spans a radian short of two and three turns. (A graze in band on a
/// span past a turn is refused whole by the pose, whose window is one
/// piece or the span.) The walk's windows are local, one a turn. Red
/// where a window is copied only a turn either way, whether the copies at
/// `4π` and beyond are lost or the span is refused whole for want of them.
#[test]
fn a_span_of_several_turns_holds_every_turns_crossings() {
    for eps in [1e-6, 1e-9, 1e-12] {
        let b = band(eps);
        for dip in [3.0 * eps, 30.0 * eps, 1e-8_f64.max(300.0 * eps)] {
            let z0 = 1.0 - dip;
            let c = ring(Point3::new(0.0, 0.0, z0), 1.0, Vec3::new(0.0, 0.0, -1.0));
            let w = 2.0 * ((1.0 - z0) / 2.0).sqrt().asin();
            for turns in [2u32, 3] {
                let span = (0.0, TAU * f64::from(turns));
                // A torus walk's touch at `θ = 0`, its window the method's,
                // at every turn of a span started a radian before it.
                if dip < 10.0 * eps {
                    let t = torus(Point3::origin(), Vec3::unit_z(), 1.0, 0.25);
                    let tangent = (Vec3::unit_y() * 0.7_f64.cos() + Vec3::unit_z() * 0.7_f64.sin())
                        .normalize();
                    let touch = graze(
                        Point3::new(1.25, 0.0, 0.0),
                        Vec3::unit_x(),
                        tangent,
                        (0.3, 0.0, 0.4),
                    );
                    let long = (-1.0, TAU * f64::from(turns) - 1.0);
                    let Err(r) = cross(&Span::new(&touch, long), &t, b) else {
                        panic!("the walk's touch refuses over {turns} turns")
                    };
                    // At `ε = 1e-9` the walk answers the touch locally (at
                    // `1e-12` it escalates the turn whole).
                    assert!(
                        eps != 1e-9
                            || r.windows.len() == turns as usize
                                && r.windows.iter().all(|w| w.window.1 - w.window.0 < 0.1),
                        "a local window at each of {turns} turns: {:?}",
                        r.windows
                    );
                    every_unread_point_is_in_a_window(
                        &format!("torus walk over {turns} turns, ε {eps:e}"),
                        &touch,
                        &t,
                        long,
                        0.3,
                        b,
                    );
                }
                let label = format!("ε {eps:e}, dip {dip:e}, span {span:?}");
                let (found, windows) = match cross(&Span::new(&c, span), &plane_z(), b) {
                    Ok(got) => (got.meetings, Vec::new()),
                    Err(r) => (r.found, r.windows),
                };
                let at = |m: &super::Meeting<f64>| match m.on_span {
                    super::OnSpan::Inside { t } => t,
                    super::OnSpan::End(crate::curve_pair::SpanEnd::Start) => span.0,
                    super::OnSpan::End(crate::curve_pair::SpanEnd::Stop) => span.1,
                };
                for k in 0..=turns {
                    for root in [TAU * f64::from(k) - w, TAU * f64::from(k) + w] {
                        if root <= span.0 + 1e-3 || root >= span.1 - 1e-3 {
                            continue;
                        }
                        let held = windows
                            .iter()
                            .any(|x| x.window.0 - eps <= root && root <= x.window.1 + eps);
                        // A found crossing is reported once, within
                        // half a turn of the span's middle; a window is
                        // held at every turn it reaches.
                        let met = found.iter().any(|m| {
                            ((at(m) - root + PI).rem_euclid(TAU) - PI).abs() <= 10.0 * eps
                        });
                        assert!(
                            held || met,
                            "{label}: the crossing at {root} is neither found nor in a window \
                             (found {found:?}, windows {windows:?})"
                        );
                    }
                }
            }
        }
    }
}
