//! CERT-4 review lane R2 — the brief's e2e exercise, authored fresh.
//! A fused arc-fillet-arc tangency that is NOT the rocker eye (an
//! asymmetric two-carrier pocket with generic centres and an off-axis
//! entry), driven through the PUBLIC door at BOTH lanes; plus the
//! public-door check that a true tangency still classifies as an
//! exact fit at f64. Local-only; never pushed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use profile::{ArcSweep, Center, Open, SketchPlane, Start};

fn p2(x: f64, y: f64) -> geom_core::Point2<f64> {
    geom_core::Point2::new(x, y)
}

/// My own fused pocket: carriers about (-0.7, 0.2) and (0.8, -0.3)
/// through the entry (0.0, -1.5), fillet r = 0.4 — asymmetric, entry
/// at a generic carrier crossing, nothing axis-aligned. What a
/// consumer sees at f64: a validated three-vertex loop.
#[test]
fn an_asymmetric_fused_pocket_replays_at_f64() {
    let built = Open
        .arc_fillet_arc(
            Center {
                c: p2(-0.7, 0.2),
                winding: ArcSweep::Ccw,
                p: p2(0.0, -1.5),
            },
            0.4f64,
            Center {
                c: p2(0.8, -0.3),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        )
        .expect("the fused pocket constructs at f64");
    let lp = built.loop_;
    profile::Profile::new(SketchPlane::xy(), vec![lp.clone()])
        .validate(Tol::witness())
        .expect("and validates");
    assert!(lp.vertices().len() >= 3, "a pocket, not a degenerate loop");
}

/// The same authored numbers at `Interval`: the pocket replays, every
/// enclosure is hairline (the issue-1191 acceptance shape, on MY
/// numbers rather than the unit's), and each encloses the f64 lane.
#[cfg(feature = "interval")]
#[test]
fn the_asymmetric_fused_pocket_replays_hairline_at_interval() {
    use geom_core::{Bounds, Interval, Real};
    let ip2 = |x: f64, y: f64| {
        geom_core::Point2::new(Interval::from_f64(x), Interval::from_f64(y))
    };
    let iv = Open
        .arc_fillet_arc(
            Center {
                c: ip2(-0.7, 0.2),
                winding: ArcSweep::Ccw,
                p: ip2(0.0, -1.5),
            },
            Interval::from_f64(0.4),
            Center {
                c: ip2(0.8, -0.3),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        )
        .expect("the fused pocket constructs at Interval");
    let f = Open
        .arc_fillet_arc(
            Center {
                c: p2(-0.7, 0.2),
                winding: ArcSweep::Ccw,
                p: p2(0.0, -1.5),
            },
            0.4f64,
            Center {
                c: p2(0.8, -0.3),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        )
        .expect("the f64 twin");
    assert_eq!(iv.loop_.vertices().len(), f.loop_.vertices().len());
    for (k, (a, b)) in f
        .loop_
        .vertices()
        .iter()
        .zip(iv.loop_.vertices())
        .enumerate()
    {
        for (what, exact, enc) in [
            ("x", a.pos().x, b.pos().x),
            ("y", a.pos().y, b.pos().y),
            ("bulge", a.bulge(), b.bulge()),
        ] {
            let w = enc.hi() - enc.lo();
            assert!(
                w <= 1e-12,
                "vertex {k} {what}: {w:e} wide — period-width, not input-width"
            );
            assert!(
                enc.lo() <= exact && exact <= enc.hi(),
                "vertex {k} {what}: enclosure [{}, {}] excludes f64 {exact}",
                enc.lo(),
                enc.hi()
            );
        }
    }
}

/// **Claim 2's public-door half at f64**: a TRUE tangency (the r = 1
/// knife-edge of the interval suite's line x arc fixture) classifies
/// as an EXACT FIT through the public builder — fit Zero, the trimmed
/// straight piece suppressed — rather than a near-miss.
#[test]
fn a_true_tangency_classifies_as_an_exact_fit_through_the_public_door() {
    let built = Open
        .at(p2(0.0, 2.0))
        .line_to(p2(0.0, 0.0), Tol::witness())
        .expect("the straight run")
        .toward(1.0, 0.0, Tol::witness())
        .expect("the incoming ray runs +x")
        .fillet_arc(
            1.0f64,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        )
        .expect("the knife-edge fit constructs at f64: the margin is bit-zero");
    let lp = built.loop_;
    profile::Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("and validates as a closed loop");
}
