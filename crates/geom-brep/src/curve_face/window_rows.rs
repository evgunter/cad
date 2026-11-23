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
/// wall and on a sphere, a circle's first-harmonic extreme on a sphere,
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
        1.0,
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
        1.0,
        b,
    )
    .unwrap_or_else(|r| panic!("the walk read past its refusals: {r:?}"));
    assert!(got.roots.is_empty(), "no root certified: {:?}", got.roots);
    assert_eq!(got.refused.len(), 2, "two windows: {:?}", got.refused);
    for (want, &((lo, hi), _)) in [-FRAC_PI_3, FRAC_PI_3].iter().zip(&got.refused) {
        assert!(
            lo <= *want && *want <= hi,
            "the window {:?} holds {want}",
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
/// (W3, `clip`): a window starting a hair past the span's end, which the
/// band cannot call apart from it, is held to the span as its last
/// `Kε` of arc, never an inverted window; one past by a clear gap is
/// dropped. Red where the clip keeps the inverted bounds.
#[test]
fn a_window_past_an_end_in_band_is_held_to_the_end() {
    let c = hanging(1.0, -0.5);
    let span = Span::new(&c, (0.0, 1.0));
    let b = band(1e-9);
    let past = (1.0 + 2e-9, 1.5);
    assert!(
        !super::misses(&span, past, b),
        "a gap in band keeps the window"
    );
    let (lo, hi) = super::clip(&span, past, b);
    assert!(
        lo < hi && hi <= 1.0 && 1.0 - lo <= 2.0 * b.escalate(),
        "{:?}",
        (lo, hi)
    );
    assert!(super::misses(&span, (1.1, 1.5), b), "a clear gap drops it");
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

/// **No found meeting lies in a window** (W1, `finish`): a meeting the
/// band cannot read outside a window is taken into it, and one apart from
/// every window is found. Red where a meeting inside a window is kept.
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
    let got = super::cross::finish(
        &span,
        Pose::Apart,
        vec![meeting(0.5), meeting(0.9)],
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
