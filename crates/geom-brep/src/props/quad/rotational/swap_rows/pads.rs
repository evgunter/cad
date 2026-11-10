//! **The lane's honesty pads** (module docs): a pcurve off its carrier,
//! inside its envelope, which the map residual pays for; and carriers
//! off the cylinder, so the carriers' `A⃗` the chart term reads is not
//! the face's, and the miss grows with the lever from the point the flux
//! is about ([`chart_term_pad`]).

use super::*;
use crate::props::quad::run::{FaceCutBounds, LaneStop};

/// Accepts the round the lane stops refining at, or one whose flux
/// width is at most twice `face`'s chart-term pad plus `slack`: the pads
/// are no round's to remove, so a row whose pads pass the reporting
/// target reads to its floor.
fn to_pad(
    face: &RotationalFaceQ,
    slack: f64,
) -> impl FnMut(&FaceCutBounds, LaneStop) -> Result<bool, PropsError> {
    let pad = pad_of(face);
    move |b, stop| Ok(stop == LaneStop::Floor || b.flux.width() <= 2.0 * pad + slack)
}

/// `face`'s chart-term pad, read as the lane reads it.
fn pad_of(face: &RotationalFaceQ) -> f64 {
    let entries: Vec<Vec<EdgeEntry>> = face
        .loops
        .iter()
        .map(|edges| {
            edges
                .iter()
                .map(|e| edge_entry(e, face.chart).unwrap())
                .collect()
        })
        .collect();
    chart_term_pad(face, &entries, Band::linear(Tol::witness()).unwrap())
}

/// A unit-cylinder rectangle `u ∈ [0, θ]`, `v ∈ [0, h]` (frame: the axis
/// `z`, the chart origin at the frame's) whose bottom rim's carrier is
/// the circle of radius `R = 1 + δ` and whose rulings' carriers are the
/// lines from `R·(cos u, sin u, 0)` to `(cos u, sin u, h)`: each carrier's
/// radial projection is its edge of the rectangle exactly, and each is
/// within `δ` of it (`env = 1.01·δ`). The top rim is harmonic and on the
/// cylinder. The flux is about the point `lever` below the chart origin
/// on the axis, its chart term the carriers' `A⃗`, as the lane assembles
/// it. Returns the face and its truth, the rectangle's own flux `θ·h`
/// (its vector area has no axial part, so the lever adds nothing).
fn off_surface_face(delta: f64, lever: f64) -> (RotationalFaceQ, f64) {
    let (theta, h) = (1.0_f64, 1.0_f64);
    let r_big = 1.0 + delta;
    let env = pt(delta * 1.01);
    let (c, s) = (theta.cos(), theta.sin());
    let bottom = EdgeTermQ::Projected(ProjectedEdgeQ {
        carrier: FramedQ::Conic {
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
    let ruling = |u: f64, (c, s): (f64, f64), forward: bool| {
        EdgeTermQ::Projected(ProjectedEdgeQ {
            carrier: FramedQ::Line {
                point: [pt(r_big * c), pt(r_big * s), Interval::zero()],
                dir: [pt(-delta * c), pt(-delta * s), pt(h)],
                cuts: vec![0.0, 1.0],
            },
            v_off: Interval::zero(),
            v_sign: 1.0,
            forward,
            ends: [(pt(u), pt(0.0)), (pt(u), pt(h))],
            env,
        })
    };
    let top = EdgeTermQ::Harmonic(TrimEdgeQ {
        u: chan(0.0, 1.0),
        v: chan(h, 0.0),
        t0: pt(0.0),
        t1: pt(theta),
        forward: false,
        trig0: trig_at_limit(pt(0.0)),
        trig1: Some(trig_at_limit(pt(theta))),
        env: Interval::zero(),
    });
    // The carriers' vector area's axial part, `½(R² − 1)·θ`: the bottom
    // arc's `½R²θ`, the top's `−½θ`, the lines' none.
    let va_z = 0.5 * delta * (2.0 + delta) * theta;
    let line_length = (delta * delta + h * h).sqrt();
    let ribbon = (env * pt(r_big * theta + 2.0 * line_length)).hi();
    let face = RotationalFaceQ {
        chart: RotationalChartQ::Cylinder { radius: pt(1.0) },
        chart_term: ChartTermQ {
            closed: pt(lever) * pt(va_z),
            lever: [Interval::zero(), Interval::zero(), pt(lever)],
            rationals: Vec::new(),
            ribbon,
        },
        loops: vec![vec![
            bottom,
            ruling(theta, (c, s), true),
            top,
            ruling(0.0, (1.0, 0.0), false),
        ]],
    };
    (face, theta * h)
}

/// **A carrier off the cylinder is padded at its lever**: the face of
/// [`off_surface_face`] misses its own region's flux by `lever·δ·θ`,
/// which the map-residual pad (`r·length·env`) stops covering once the
/// lever passes `r`; the chart-term pad holds it at every `(δ, lever)`.
#[test]
fn the_chart_term_of_an_off_surface_carrier_is_padded() {
    let band = Band::linear(Tol::witness()).unwrap();
    for delta in [1e-9, 1e-6] {
        for lever in [0.0, 1.0, 10.0, 1e3] {
            let (face, truth) = off_surface_face(delta, lever);
            let got = met(rotational_cut_face_rounds::<f64>(
                &face,
                band,
                RoundWindow::SCHEDULE,
                &mut to_pad(&face, 1e-11),
            ))
            .unwrap();
            encloses(got.flux, truth, &format!("δ {delta:e}, lever {lever:e}"));
        }
    }
}

/// **The chart-term pad is what holds it**: at `δ = 1e-6` a thousand
/// metres out, the miss `lever·δ·θ` passes the enclosure's half-width
/// less the pad, so with the pad removed the truth escapes.
#[test]
fn without_the_chart_term_pad_an_off_surface_carrier_escapes() {
    let band = Band::linear(Tol::witness()).unwrap();
    let (face, truth) = off_surface_face(1e-6, 1e3);
    let got = met(rotational_cut_face_rounds::<f64>(
        &face,
        band,
        RoundWindow::SCHEDULE,
        &mut to_pad(&face, 1e-11),
    ))
    .unwrap();
    let pad = pad_of(&face);
    let miss = (mid(got.flux) - truth).abs();
    eprintln!(
        "miss {miss:e}, half-width {:e}, pad {pad:e}",
        0.5 * got.flux.width()
    );
    assert!(
        miss > 0.5 * got.flux.width() - pad,
        "the miss {miss:e} is inside the enclosure without its pad {pad:e}: {:?}",
        got.flux
    );
}

/// The rim `v = vc` over `us` as a rational quadratic on the cylinder of
/// radius `r·(1 + delta)` about the same axis: a carrier `r·delta` off
/// the wall everywhere, whose radial projection is the exact rim.
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

/// The rectangle on the chart at `o`, about the world origin, its bottom
/// rim's carrier `r·delta` off the wall ([`rim_off`]), its chart term
/// built as `quad_lane::cut_face_lane` builds it: the closed part the
/// other edges' exact `A⃗` (the rectangle's less its exact bottom rim's,
/// [`face`]), the off carrier's read by the lane. The truth is the face
/// its pcurves bound, the exact rectangle (the carrier's image is the
/// exact rim).
fn far_face_off_wall(delta: f64, o: Point3<f64>) -> (RotationalFaceQ, f64) {
    let chart = Chart::tilted(o, 0.8);
    let about = Point3::origin();
    let (us, vs) = ((0.3, 1.4), (0.2, 1.1));
    let (flux, _, va) = rectangle_truth(chart, us, vs, about, 1.0);
    let mut loops = [rectangle(chart, us, vs, &[0])];
    loops[0][0].swap = Some(rim_off(chart, vs.0, us, delta));
    let (_, swapped) = face(chart, &loops, va, about, Deck::AsMinted);
    (swapped, flux)
}

/// The far face's run at `delta` and `o`, its truth inside it.
fn far_face_holds(delta: f64, o: Point3<f64>, label: &str) {
    let (face, truth) = far_face_off_wall(delta, o);
    let band = Band::linear(Tol::witness()).unwrap();
    let got = met(rotational_cut_face_rounds::<f64>(
        &face,
        band,
        RoundWindow::SCHEDULE,
        &mut to_pad(&face, 1e-9),
    ))
    .unwrap();
    encloses(got.flux, truth, label);
}

/// **A rim carrier on the wall holds the closed form a thousand metres
/// out**, built as the lane assembles it: the control for
/// [`a_carrier_off_the_wall_holds_far_out`].
#[test]
fn a_carrier_on_the_wall_holds_far_out() {
    far_face_holds(0.0, Point3::new(1e3, -1e3, 5e2), "on the wall, far out");
}

/// How far off the wall the off-wall rows set their rim's carrier, as a
/// share of the chart's radius `0.8`: half of ε in metres (`5e-10` at ε
/// `1e-9`), inside the band the image mints in at every ε row.
fn off_the_wall() -> f64 {
    0.5 * Tol::witness().eps() / 0.8
}

/// **A rim carrier half of ε off the wall ([`off_the_wall`]) holds near
/// the chart's own origin**, where the lever is short.
#[test]
fn a_carrier_off_the_wall_holds_near_the_origin() {
    far_face_holds(
        off_the_wall(),
        Point3::new(0.4, -0.2, 0.3),
        "off the wall, near",
    );
}

/// **A rim carrier half of ε off the wall ([`off_the_wall`], `5e-10` m at
/// ε `1e-9`, its envelope says so) holds a thousand metres out**: the
/// chart term reads the carrier's
/// `A⃗` at the lever `|o − c| ≈ 1.5e3`, and the pad holds the miss
/// (`~lever·length·env`, `7.7e-8` before it).
#[test]
fn a_carrier_off_the_wall_holds_far_out() {
    far_face_holds(
        off_the_wall(),
        Point3::new(1e3, -1e3, 5e2),
        "off the wall, far out",
    );
}

/// **A pcurve off its carrier is paid for by the map residual**: the
/// rectangle's bottom rim read at the level `v₀ + δ`, its envelope
/// `1.01·δ`, while the face (its carriers' and its truth) is the
/// rectangle at `v₀`. The region term misses `r²·Δu·δ`, which only the
/// residual pad (`r·length·env`) pays; about the chart's own origin the
/// chart term's lever is zero.
#[test]
fn a_pcurve_off_its_carrier_is_paid_for_by_the_map_residual() {
    let chart = Chart::tilted(Point3::new(0.4, -0.2, 0.3), 0.8);
    let (us, vs) = ((0.3, 1.4), (0.2, 1.1));
    let (flux, area, _) = rectangle_truth(chart, us, vs, chart.o, 1.0);
    let band = Band::linear(Tol::witness()).unwrap();
    for delta in [1e-7, 1e-5] {
        let mut loops: Vec<EdgeTermQ> = rectangle(chart, us, vs, &[])
            .into_iter()
            .map(|e| EdgeTermQ::Harmonic(e.harmonic))
            .collect();
        if let EdgeTermQ::Harmonic(bottom) = &mut loops[0] {
            bottom.v.c0 = pt(vs.0 + delta);
            bottom.env = pt(1.01 * delta);
        }
        let face = RotationalFaceQ {
            chart: RotationalChartQ::Cylinder {
                radius: pt(chart.r),
            },
            chart_term: ChartTermQ {
                closed: Interval::zero(),
                lever: [Interval::zero(); 3],
                rationals: Vec::new(),
                ribbon: 0.0,
            },
            loops: vec![loops],
        };
        let got = met(rotational_cut_face_rounds::<f64>(
            &face,
            band,
            RoundWindow::SCHEDULE,
            &mut to_pad(&face, 1e-11),
        ))
        .unwrap();
        encloses(got.flux, flux, &format!("δ {delta:e}: flux"));
        encloses(got.area, area, &format!("δ {delta:e}: area"));
    }
}
