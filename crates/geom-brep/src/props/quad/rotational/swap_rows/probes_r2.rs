//! Reviewer R2's probes on PR 6478 (frozen head e12e3c8d00).

use super::*;
use crate::props::rational_rim::rational_edge_q_split;

/// The rim `v = vc` over `us`, written as a rational quadratic on the
/// cylinder of radius `r·(1 + delta)` about the same axis: a carrier off
/// the wall by `r·delta` everywhere, whose radial projection onto the wall
/// is the exact rim.
fn rim_off(chart: Chart, vc: f64, us: (f64, f64), delta: f64) -> NurbsCurve3<f64> {
    rim_net(
        Chart {
            r: chart.r * (1.0 + delta),
            ..chart
        },
        vc,
        us,
        1,
    )
}

/// The far face, its bottom rim's carrier `r·delta` off the wall, its
/// chart term's closed part built the way `quad_lane::cut_face_lane`
/// builds it: the closed-form (harmonic) edges' exact `A⃗` at the lever,
/// the rational carrier's through `rationals`. The truth is the face the
/// pcurves bound, which is the exact rectangle (the image of the carrier
/// is the exact rim), so the module's own honesty model ("the true trim
/// boundary lies within metric distance `e` of the pcurve") requires the
/// enclosure to hold the rectangle's closed form.
fn far_face_off_wall(delta: f64, o: Point3<f64>) -> ((RotationalFaceQ, RotationalFaceQ), f64, f64, f64) {
    let chart = Chart::tilted(o, 0.8);
    let about = Point3::origin();
    let (us, vs) = ((0.3, 1.4), (0.2, 1.1));
    let (flux, area, va) = rectangle_truth(chart, us, vs, about, 1.0);
    let mut loops = [rectangle(chart, us, vs, &[0])];
    loops[0][0].swap = Some(rim_off(chart, vs.0, us, delta));
    let mut faces = face(chart, &loops, va, about, Deck::AsMinted);
    // `face` subtracts the CARRIER's own vector area from the closed part,
    // which cancels whatever the lane reads for it. `cut_face_lane` instead
    // forms the closed part from the other edges, which here are the exact
    // rectangle's less its exact bottom rim.
    let lever = chart.o - about;
    let lever = [pt(lever.x), pt(lever.y), pt(lever.z)];
    let exact_rim = carrier_term(chart, &rim_net(chart, vs.0, us, 1), true);
    let (va_rim, _) = rational_vector_area_split(&exact_rim, 4096);
    let whole = lever[0] * pt(va.x) + lever[1] * pt(va.y) + lever[2] * pt(va.z);
    faces.1.chart_term.closed = whole - (0..3).fold(Interval::zero(), |a, k| a + lever[k] * va_rim[k]);
    let env = match &faces.1.loops[0][0] {
        EdgeTermQ::Projected(p) => p.env.hi(),
        EdgeTermQ::Harmonic(_) => unreachable!(),
    };
    (faces, flux, area, env)
}

fn swapped_flux(face: &RotationalFaceQ) -> Interval {
    let band = Band::linear(Tol::witness()).unwrap();
    met(rotational_cut_face_rounds::<f64>(
        face,
        band,
        RoundWindow::SCHEDULE,
        &mut to_target(1e-9),
    ))
    .unwrap()
    .flux
}

/// Control: the carrier ON the wall, built as `cut_face_lane` builds it,
/// holds the closed form a thousand metres out.
#[test]
fn r2_control_a_carrier_on_the_wall_holds_far_out() {
    let ((_, swapped), flux, _, env) = far_face_off_wall(0.0, Point3::new(1e3, -1e3, 5e2));
    let got = swapped_flux(&swapped);
    eprintln!("on the wall: env {env:e}, flux {got:?}, truth {flux}");
    encloses(got, flux, "on the wall, far out");
}

/// Control: the carrier `5e-10` m off the wall, about the chart's own
/// neighbourhood (lever ~0.5), holds: the residual pad covers it there.
#[test]
fn r2_control_a_carrier_off_the_wall_holds_near_the_origin() {
    let ((_, swapped), flux, _, env) = far_face_off_wall(6.25e-10, Point3::new(0.4, -0.2, 0.3));
    let got = swapped_flux(&swapped);
    eprintln!("near: env {env:e}, flux {got:?}, truth {flux}, miss {:e}", (mid(got) - flux).abs());
    encloses(got, flux, "off the wall, near");
}

/// **Claim 4 falsified**: a rim carrier `5e-10` m off the wall (inside
/// ε = 1e-9, its certified envelope says so) on a face a thousand metres
/// out. The chart term reads the carrier's vector area at the lever
/// `|o − c| ≈ 1.5e3`, unpadded, so the bracket misses the face its own
/// pcurves bound by ~lever·length·env, far past its width.
#[test]
fn r2_a_carrier_off_the_wall_escapes_the_bracket_far_out() {
    let ((_, swapped), flux, _, env) = far_face_off_wall(6.25e-10, Point3::new(1e3, -1e3, 5e2));
    let got = swapped_flux(&swapped);
    eprintln!(
        "far: env {env:e}, flux {got:?} (width {:e}), truth {flux}, miss {:e}",
        got.width(),
        (mid(got) - flux).abs()
    );
    encloses(got, flux, "off the wall, far out");
}

/// **The projected-circle row with its end read a turn off**: the image's
/// evaluator lifts the edge's end onto the branch a whole turn below
/// (`u₁ − τ`), as an image whose end lies past its sector's seam does.
/// The closure must count that turn back through `branch_turns`, so the
/// face is the plain rectangle. No PR row has a nonzero `branch_turns`:
/// with its result discarded, every swap row still passes.
#[test]
fn r2_a_projected_end_read_a_turn_off_holds_the_closed_form() {
    let chart = Chart::tilted(Point3::new(0.4, -0.2, 0.3), 0.8);
    let (us, vs) = ((0.3, 2.4), (0.2, 1.1));
    let (flux, area, _) = rectangle_truth(chart, us, vs, chart.o, 1.0);
    let r = pt(chart.r);
    let tau = core::f64::consts::TAU;
    let circle = EdgeTermQ::Projected(ProjectedEdgeQ {
        carrier: FramedQ::Circle {
            centre: [Interval::zero(), Interval::zero(), pt(vs.0)],
            a: [r, Interval::zero(), Interval::zero()],
            b: [Interval::zero(), r, Interval::zero()],
            cuts: vec![us.0, 1.6, us.1],
        },
        v_off: Interval::zero(),
        v_sign: 1.0,
        forward: true,
        ends: [(pt(us.0), pt(vs.0)), (pt(us.1) - pt(tau), pt(vs.0))],
        env: Interval::zero(),
    });
    let mut loops: Vec<EdgeTermQ> = rectangle(chart, us, vs, &[])
        .into_iter()
        .map(|e| EdgeTermQ::Harmonic(e.harmonic))
        .collect();
    loops[0] = circle;
    let face = RotationalFaceQ {
        chart: RotationalChartQ::Cylinder { radius: r },
        chart_term: ChartTermQ {
            closed: Interval::zero(),
            lever: [Interval::zero(); 3],
            rationals: Vec::new(),
        },
        loops: vec![loops],
    };
    let band = Band::linear(Tol::witness()).unwrap();
    let got = met(rotational_cut_face_rounds::<f64>(
        &face,
        band,
        RoundWindow::SCHEDULE,
        &mut to_target(1e-9),
    ))
    .unwrap_or_else(|e| panic!("refused: {e:?}"));
    encloses(got.flux, flux, "an end a turn off: flux");
    encloses(got.area, area, "an end a turn off: area");
}

/// The chords of [`r2_projected_integrals_hold_high_precision_truths`]:
/// (knots, degree, control points in the chart's frame, weights).
#[allow(clippy::type_complexity)]
fn r2_chords() -> Vec<(&'static str, Vec<f64>, usize, Vec<[f64; 3]>, Vec<f64>)> {
    let polar = |deg: f64, rho: f64, z: f64| {
        let a = deg.to_radians();
        [rho * a.cos(), rho * a.sin(), z]
    };
    vec![
        (
            "a rational cubic past a quarter turn",
            vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
            3,
            vec![[1.0, 0.0, 0.0], [1.0, 1.2, 0.3], [-0.5, 1.3, 0.9], [-1.1, 0.2, 1.4]],
            vec![1.0, 0.7, 1.3, 1.0],
        ),
        (
            "the same cubic turned across the seam",
            vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
            3,
            vec![polar(150.0, 1.0, 0.0), polar(200.0, 1.5620499, 0.3), polar(260.0, 1.3928388, 0.9), polar(320.0, 1.1180340, 1.4)],
            vec![1.0, 0.7, 1.3, 1.0],
        ),
        (
            "a rational quadratic spiral winding 1.3 turns",
            vec![0.0, 0.0, 0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0, 1.0],
            2,
            vec![
                polar(0.0, 1.0, 0.0),
                polar(100.0, 1.3, 0.4),
                polar(220.0, 1.2, 0.8),
                polar(340.0, 1.25, 1.2),
                polar(470.0, 1.0, 1.6),
            ],
            vec![1.0, 0.8, 1.1, 0.9, 1.0],
        ),
    ]
}

/// Prints `[−∫F du, −∫G du, ∫du]` of each chord at r = 0.8,
/// v = 0.3 − z, anchor 0.25, at 8 and 128 pieces a span, with an extra
/// break at 0.37, for an independent mpmath truth
/// (`r2_mpmath_truths.py` on this branch); asserts those truths.
#[test]
fn r2_projected_integrals_hold_high_precision_truths() {
    let truths: [[f64; 3]; 3] = include!("r2_truths.in");
    let chart = RotationalChartQ::Cylinder { radius: pt(0.8) };
    for ((label, knots, degree, control, w), truth) in r2_chords().into_iter().zip(truths) {
        let n = net(
            knots,
            degree,
            control.iter().map(|p| Point3::new(p[0], p[1], p[2])).collect(),
            w,
        );
        let q = rational_edge_q_split(
            &n.map_scalar(Interval::from_certified),
            (pt(0.0), pt(1.0)),
            true,
            Point3::new(pt(0.0), pt(0.0), pt(0.0)),
            &[0.37],
        )
        .unwrap();
        let e = ProjectedEdgeQ {
            carrier: FramedQ::Net(q),
            v_off: pt(0.3),
            v_sign: -1.0,
            forward: true,
            ends: [(pt(0.0), pt(0.0)); 2],
            env: Interval::zero(),
        };
        for pieces in [8, 128] {
            let (got, _) = projected_integrals(&e, chart, 0.25, pieces);
            eprintln!("{label} @{pieces}: {got:?}");
            for k in (0..3).filter(|_| truth != [0.0; 3]) {
                encloses(got[k], truth[k], &format!("{label} @{pieces} [{k}]"));
            }
        }
        let len = projected_metric_length(&e, pt(0.8));
        eprintln!("{label}: metric length bound {len}");
    }
}
