//! Reviewer R1's probes on PR 6478 (not for merge).

use super::*;
use geom_core::Tol;

/// A unit-cylinder rectangle `u ∈ [0, θ]`, `v ∈ [0, H]` whose bottom rim
/// carrier is the circle of radius `R = 1 + δ` (radial projection: the
/// rim `v = 0` exactly) and whose rulings' carriers are the lines from
/// `R·(cos u, sin u, 0)` to `(cos u, sin u, H)` (radial projection: the
/// ruling exactly). Every carrier is within `δ` of its image: `env = δ`.
/// The top rim is harmonic and on the cylinder. The chart term reads the
/// carriers' vector area (the PR's convention), about a point `L` below
/// the chart origin on the axis. Returns (flux enclosure, truth, lever error).
fn off_surface_face(delta: f64, lever: f64) -> (Interval, f64, f64) {
    let (theta, h) = (1.0_f64, 1.0_f64);
    let r_big = 1.0 + delta;
    let env = pt(delta * 1.01);
    let (c, s) = (theta.cos(), theta.sin());
    let bottom = EdgeTermQ::Projected(ProjectedEdgeQ {
        carrier: FramedQ::Circle {
            centre: [Interval::zero(); 3],
            a: [pt(r_big), Interval::zero(), Interval::zero()],
            b: [Interval::zero(), pt(r_big), Interval::zero()],
            cuts: vec![0.0, theta],
        },
        v_off: Interval::zero(),
        v_sign: 1.0,
        forward: true,
        ends: [(pt(0.0), pt(0.0)), (pt(theta), pt(0.0))],
        env,
    });
    let right = EdgeTermQ::Projected(ProjectedEdgeQ {
        carrier: FramedQ::Line {
            point: [pt(r_big * c), pt(r_big * s), Interval::zero()],
            dir: [pt(-delta * c), pt(-delta * s), pt(h)],
            cuts: vec![0.0, 1.0],
        },
        v_off: Interval::zero(),
        v_sign: 1.0,
        forward: true,
        ends: [(pt(theta), pt(0.0)), (pt(theta), pt(h))],
        env,
    });
    let top = EdgeTermQ::Harmonic(TrimEdgeQ {
        u: HarmChan {
            c0: Interval::zero(),
            ca: Interval::zero(),
            cb: Interval::zero(),
            cl: Interval::one(),
        },
        v: HarmChan {
            c0: pt(h),
            ca: Interval::zero(),
            cb: Interval::zero(),
            cl: Interval::zero(),
        },
        t0: pt(0.0),
        t1: pt(theta),
        forward: false,
        trig0: super::super::trig::trig_at_limit(pt(0.0)),
        trig1: Some(super::super::trig::trig_at_limit(pt(theta))),
        env: Interval::zero(),
    });
    let left = EdgeTermQ::Projected(ProjectedEdgeQ {
        carrier: FramedQ::Line {
            point: [pt(r_big), Interval::zero(), Interval::zero()],
            dir: [pt(-delta), Interval::zero(), pt(h)],
            cuts: vec![0.0, 1.0],
        },
        v_off: Interval::zero(),
        v_sign: 1.0,
        forward: false,
        ends: [(pt(0.0), pt(0.0)), (pt(0.0), pt(h))],
        env,
    });
    // The carriers' vector area, z part: ½(R² − 1)·θ (closed loop of
    // carriers; derived by hand: bottom ½R²θ, top −½θ, lines 0 in z).
    let va_z = 0.5 * delta * (2.0 + delta) * theta;
    let closed = pt(lever) * pt(va_z);
    let face = RotationalFaceQ {
        chart: RotationalChartQ::Cylinder { radius: pt(1.0) },
        chart_term: ChartTermQ {
            closed,
            lever: [Interval::zero(); 3],
            rationals: Vec::new(),
        },
        loops: vec![vec![bottom, right, top, left]],
    };
    let band = Band::linear(Tol::witness()).unwrap();
    let run = rotational_cut_face_rounds::<f64>(
        &face,
        band,
        RoundWindow::SCHEDULE,
        &mut |b: &FaceCutBounds, _| Ok(b.flux.width() < 1e-11),
    )
    .unwrap();
    // The image region is the chart rectangle exactly: flux about the
    // chart origin r²·θ·H, and its vector area H(sin θ, 1 − cos θ, 0)
    // has no axial part, so the lever adds nothing.
    (run.bounds.flux, theta * h, lever * va_z)
}

/// **The chart term reads the carriers' `A⃗`, unpadded**: a face whose
/// carriers sit `δ` off the cylinder (inside their envelope) misses its
/// own image region's flux by `L·δ·θ`, which the map-residual pad
/// (`r·length·env`) does not cover once the axial lever `L` passes `r`.
#[test]
fn r1_the_chart_term_of_an_off_surface_carrier_is_padded() {
    let mut escaped = Vec::new();
    for delta in [1e-9, 1e-6] {
        for lever in [0.0, 1.0, 10.0, 1e3] {
            let (flux, truth, err) = off_surface_face(delta, lever);
            let inside = flux.lo() <= truth && truth <= flux.hi();
            eprintln!(
                "δ {delta:e} L {lever:e}: flux [{:.15e}, {:.15e}] width {:e}, truth {truth}, carrier-term error {err:e}, contains {inside}",
                flux.lo(),
                flux.hi(),
                flux.width()
            );
            if !inside {
                escaped.push((delta, lever));
            }
        }
    }
    assert!(escaped.is_empty(), "truth escapes at (δ, L) = {escaped:?}");
}
