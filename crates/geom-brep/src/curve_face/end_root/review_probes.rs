//! Review probes (PR 7463, R1).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stderr)]
use super::*;
use geom_core::{Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}
fn pipe() -> Surface<f64> {
    Surface::Cylinder {
        frame: geom::Frame::new(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        radius: 1.0,
    }
}
fn bend() -> Surface<f64> {
    super::super::conic_oracle::torus(Point3::new(3.0, 0.0, 2.0), Vec3::new(0.0, 1.0, 0.0), 3.0, 1.0)
}
fn ruling(theta: f64) -> Curve3<f64> {
    Curve3::Line(geom::Line {
        origin: Point3::new(theta.cos(), theta.sin(), 0.0),
        dir: Vec3::new(0.0, 0.0, 1.0),
    })
}
fn residual(c: &Curve3<f64>, s: &Surface<f64>, t: f64) -> f64 {
    crate::implicit_residual(s, c.eval(t))
}

/// `whole` claims "no root of the span but the end's"; a circle's run
/// is capped at a quarter turn, so a span past a quarter turn with a
/// second carrier root past it must not read `whole`.
#[test]
fn probe_whole_on_an_arc_longer_than_a_quarter_turn() {
    // Circle in y = 0 about (-0.5, 0, 2), radius 1.5, leaving (1, 0, 2)
    // along -z: tangent to the pipe's wall there; it meets x = -1 at
    // phi = acos(-1/3) = 1.9106 and ends at 2.2, off the carrier.
    let arc = Curve3::circle(
        Point3::new(-0.5, 0.0, 2.0),
        Vec3::new(0.0, 1.0, 0.0),
        1.5,
        Vec3::new(1.0, 0.0, 0.0),
    );
    let span = (0.0, 2.2);
    let second = (-1.0_f64 / 3.0).acos();
    eprintln!(
        "p(0)={:?} p(second)={:?} r(second)={:e} r(2.2)={:e}",
        arc.eval(0.0),
        arc.eval(second),
        residual(&arc, &pipe(), second),
        residual(&arc, &pipe(), 2.2)
    );
    let w = end_window(&arc, span, SpanEnd::Start, &pipe(), band());
    eprintln!("window: {w:?}");
    let w = w.expect("a window");
    assert!(
        !(w.whole && second < span.1),
        "`whole` read on a span with a second root at {second} (window to {})",
        w.at
    );
}

/// A transverse end (no junction) is order 1: goes red if the walk
/// starts at j = 2 without deciding c1.
#[test]
fn probe_a_transverse_end_is_order_one() {
    let line = Curve3::Line(geom::Line {
        origin: Point3::new(0.0, 0.0, 1.0),
        dir: Vec3::new(1.0, 0.0, 0.0),
    });
    let w = end_window(&line, (0.0, 1.0), SpanEnd::End, &pipe(), band()).expect("window");
    eprintln!("transverse: {w:?}");
    assert_eq!(w.order, 1);
}

/// A ruling just off the neutral point with c2 decided nonzero but tiny:
/// the quartic tail shrinks the window to ~sqrt(3 delta), where the
/// zero band's charge on c0 beats c2 s^2: the spread must refuse.
/// Goes red if the charge is dropped.
#[test]
fn probe_a_tiny_decided_c2_refuses_on_its_spread() {
    let b = band();
    let delta = 6.0 * b.escalate();
    let got = end_window(&ruling(core::f64::consts::FRAC_PI_2 - delta), (0.0, 2.0), SpanEnd::End, &bend(), b);
    eprintln!("zero={:e} esc={:e} delta={delta:e}: {got:?}", b.zero(), b.escalate());
    assert!(matches!(got, Err(EndRootRefusal::Spread(_) | EndRootRefusal::Window(_))), "{got:?}");
}

/// Sweep the ruling's angle off the neutral point: every window the door
/// returns must hold no sign change of the residual on (spread, at],
/// sampled densely; print the windows.
#[test]
fn probe_sampled_windows_hold_no_root() {
    let b = band();
    for k in 0..40 {
        let theta = core::f64::consts::FRAC_PI_2 * (k as f64) / 40.0 + 1e-3;
        for (span, end) in [((0.0, 2.0), SpanEnd::End), ((-2.0, 2.0), SpanEnd::End), ((-6.0, 2.0), SpanEnd::End)] {
            let c = ruling(theta);
            let Ok(w) = end_window(&c, span, end, &bend(), b) else { continue };
            let (lo, hi) = if w.whole { (span.0, 2.0) } else { (w.at, 2.0) };
            // Sample from the far end to 1e-3 short of the end.
            let n = 4000;
            let mut prev = residual(&c, &bend(), lo);
            for i in 1..n {
                let t = lo + (hi - 1e-3 - lo) * (i as f64) / (n as f64);
                let r = residual(&c, &bend(), t);
                assert!(prev.signum() == r.signum() || r == 0.0, "theta {theta} span {span:?}: sign change at {t} inside window {w:?}");
                prev = r;
            }
        }
    }
}

#[test]
fn probe_scan_delta() {
    let b = band();
    for k in 0..30 {
        let delta = 1e-8 * 10f64.powf(k as f64 / 5.0);
        let got = end_window(&ruling(core::f64::consts::FRAC_PI_2 - delta), (0.0, 2.0), SpanEnd::End, &bend(), b);
        eprintln!("SCAN delta={delta:.3e}: {got:?}");
    }
}
