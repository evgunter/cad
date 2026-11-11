//! **Projected edges whose reading moves**: an image whose ends sit a
//! turn off its integral (the closure's [`branch_turns`]), a deck map
//! with a level offset, a chord whose azimuth and level both move, a
//! plane section, and the rule's integrals against 40-digit truths.

use super::*;
use crate::props::rational_rim::rational_edge_q_split;

/// **A projected end read a turn off holds the closed form**: the
/// image's evaluator lifts the edge's end onto the branch a whole turn
/// away (`u₁ ∓ τ`), as an image whose end lies past its sector's seam
/// does, and the closure counts that turn back ([`branch_turns`]) so the
/// face is the plain rectangle — the bottom rim forward a turn below,
/// and the top rim, traversed against its carrier, a turn above.
#[test]
fn a_projected_end_read_a_turn_off_holds_the_closed_form() {
    let chart = Chart::tilted(Point3::new(0.4, -0.2, 0.3), 0.8);
    let (us, vs) = ((0.3, 2.4), (0.2, 1.1));
    let tau = core::f64::consts::TAU;
    let bottom = rim_circle(
        chart,
        vs.0,
        (0.0, 1.0),
        vec![us.0, 1.6, us.1],
        true,
        [(us.0, vs.0), (us.1 - tau, vs.0)],
    );
    framed_holds(
        chart,
        us,
        vs,
        0,
        bottom,
        "the bottom rim's end a turn below",
    );
    let top = rim_circle(
        chart,
        vs.1,
        (0.0, 1.0),
        vec![us.0, 1.6, us.1],
        false,
        [(us.0, vs.1), (us.1 + tau, vs.1)],
    );
    framed_holds(chart, us, vs, 2, top, "the top rim's end a turn above");
}

/// **A projected edge whose ends and turn disagree by no whole number of
/// turns refuses**: the bottom rim's carrier runs a fraction of a turn
/// past the end its image reads, so its junctions close and its integral
/// does not, by a turn no branch accounts for — `0.4`, `0.5` and `0.35`
/// of a turn from the nearest whole one, each past the quarter turn
/// [`branch_turns`] admits.
#[test]
fn a_projected_edge_whose_ends_and_turn_disagree_refuses() {
    let chart = Chart::tilted(Point3::new(0.4, -0.2, 0.3), 0.8);
    let (us, vs) = ((0.3, 2.4), (0.2, 1.1));
    let tau = core::f64::consts::TAU;
    for off in [0.4, 0.5, 0.65] {
        // Cut every quarter radian, so the turn reads tight at the entry
        // resolution and only its disagreement with the ends refuses.
        let end = us.1 + off * tau;
        let n = ((end - us.0) / 0.25).ceil();
        let cuts: Vec<f64> = (0..=n as usize)
            .map(|i| us.0 + (end - us.0) * (i as f64 / n))
            .collect();
        let bottom = rim_circle(
            chart,
            vs.0,
            (0.0, 1.0),
            cuts,
            true,
            [(us.0, vs.0), (us.1, vs.0)],
        );
        match with_framed(chart, us, vs, 0, bottom) {
            Err(PropsError::QuadratureUnsupported { what }) => assert!(
                what.starts_with("a projected edge whose image's ends and turn disagree"),
                "{off}: {what}"
            ),
            other => panic!("a carrier {off} of a turn past its end: {other:?}"),
        }
    }
}

/// **A deck map's level offset holds the closed form**: the bottom rim
/// as a circle at frame height `z`, its level `v = v_off + v_sign·z`, for
/// either sign.
#[test]
fn a_level_offset_holds_the_closed_form() {
    let chart = Chart::tilted(Point3::new(0.4, -0.2, 0.3), 0.8);
    let (us, vs) = ((0.3, 2.4), (0.2, 1.1));
    for (v_off, v_sign) in [(0.3, 1.0), (-0.7, 1.0), (0.9, -1.0)] {
        let z = (vs.0 - v_off) * v_sign;
        let bottom = rim_circle(
            chart,
            z,
            (v_off, v_sign),
            vec![us.0, 1.6, us.1],
            true,
            [(us.0, vs.0), (us.1, vs.0)],
        );
        framed_holds(
            chart,
            us,
            vs,
            0,
            bottom,
            &format!("v_off {v_off}, v_sign {v_sign}"),
        );
    }
}

/// **A chord across the wall holds its region's area**: the chord from
/// `(u₀, v₀)` to `(u₁, v₁)` of the chart, a line in its frame
/// (`FramedQ::Line`) whose image moves in both azimuth and level, closes
/// the region over the rim `v = v₀` and the ruling `u = u₁`, traversed
/// against its carrier. The region's signed area is
/// `∫(v − v₀)·u′ dt = (v₁ − v₀)·(u₁ − ∫u dt)`, the azimuth read by
/// `atan2` and integrated by composite Simpson, not through `u′`.
#[test]
fn a_chord_across_the_wall_holds_its_area() {
    let chart = Chart::tilted(Point3::new(0.4, -0.2, 0.3), 0.8);
    let r = chart.r;
    let rows: [((f64, f64), (f64, f64)); 2] =
        [((0.3, 1.4), (0.2, 1.1)), ((-0.6, 1.9), (-0.4, 0.5))];
    for (us, vs) in rows {
        let p0 = [r * us.0.cos(), r * us.0.sin(), vs.0];
        let p1 = [r * us.1.cos(), r * us.1.sin(), vs.1];
        let chord = EdgeTermQ::Projected(ProjectedEdgeQ {
            carrier: FramedQ::Line {
                point: p0.map(pt),
                dir: [0, 1, 2].map(|k| pt(p1[k] - p0[k])),
                cuts: vec![0.0, 0.5, 1.0],
            },
            v_off: Interval::zero(),
            v_sign: 1.0,
            forward: false,
            ends: [(pt(us.0), pt(vs.0)), (pt(us.1), pt(vs.1))],
            env: Interval::zero(),
        });
        let rect = rectangle(chart, us, vs, &[]);
        let face = RotationalFaceQ {
            chart: RotationalChartQ::Cylinder { radius: pt(r) },
            chart_term: ChartTermQ {
                closed: Interval::zero(),
                lever: [Interval::zero(); 3],
                rationals: Vec::new(),
                ribbon: 0.0,
            },
            loops: vec![vec![
                EdgeTermQ::Harmonic(rect[0].harmonic),
                EdgeTermQ::Harmonic(rect[1].harmonic),
                chord,
            ]],
        };
        let u_at = |t: f64| {
            let (x, y) = (p0[0] + t * (p1[0] - p0[0]), p0[1] + t * (p1[1] - p0[1]));
            y.atan2(x)
        };
        let n = 4096_u32;
        let h = 1.0 / f64::from(n);
        let mut sum = u_at(0.0) + u_at(1.0);
        for i in 1..n {
            sum += u_at(h * f64::from(i)) * if i % 2 == 1 { 4.0 } else { 2.0 };
        }
        let a = (vs.1 - vs.0) * (us.1 - sum * h / 3.0);
        let band = Band::linear(Tol::witness()).unwrap();
        let got = met(rotational_cut_face_rounds::<f64>(
            &face,
            band,
            RoundWindow::SCHEDULE,
            &mut to_target(1e-9),
        ))
        .unwrap();
        encloses(got.flux, r * r * a, &format!("a chord over {us:?}: flux"));
        encloses(got.area, r * a.abs(), &format!("a chord over {us:?}: area"));
    }
}

/// The plane section `v = c + k·r·cos u` of the chart from `ua` to `ub`
/// as an exact rational quadratic (the rim's net, its level an affine
/// function of the frame's `x`), one segment per `segments`.
fn section_net(
    chart: Chart,
    (c, k): (f64, f64),
    (ua, ub): (f64, f64),
    segments: usize,
) -> NurbsCurve3<f64> {
    #[allow(clippy::cast_precision_loss)]
    let n = segments as f64;
    let half = (ub - ua) / (2.0 * n);
    let lvl = |x_frame: f64| c + k * x_frame;
    let mut control = vec![chart.at(ua, lvl(chart.r * ua.cos()))];
    let mut weights = vec![1.0];
    let mut knots = vec![0.0; 3];
    for s in 0..segments {
        #[allow(clippy::cast_precision_loss)]
        let a = ua + (ub - ua) * (s as f64) / n;
        let m = a + half;
        let rr = chart.r / half.cos();
        control.push(chart.o + chart.radial(m) * rr + chart.axis * lvl(rr * m.cos()));
        weights.push(half.cos());
        let e = a + 2.0 * half;
        control.push(chart.at(e, lvl(chart.r * e.cos())));
        weights.push(1.0);
        if s + 1 < segments {
            #[allow(clippy::cast_precision_loss)]
            knots.extend([(s + 1) as f64 / n; 2]);
        }
    }
    knots.extend([1.0; 3]);
    net(knots, 2, control, weights)
}

/// A face between the level `v0` and the plane section
/// `v = c + k·r·cos u` over `us`, the section swapped onto a net of
/// `segments`.
#[derive(Clone, Copy)]
struct Section {
    label: &'static str,
    us: (f64, f64),
    plane: (f64, f64),
    v0: f64,
    segments: usize,
}

impl Section {
    const fn new(
        label: &'static str,
        us: (f64, f64),
        plane: (f64, f64),
        v0: f64,
        segments: usize,
    ) -> Self {
        Self {
            label,
            us,
            plane,
            v0,
            segments,
        }
    }

    /// The face's harmonic and swapped readings about `about`, and its
    /// closed forms `(flux, area)` there: `r²·A + (o − about)·A⃗` and
    /// `r·|A|`, `A⃗` the harmonic channels' own.
    fn faces(
        self,
        chart: Chart,
        about: Point3<f64>,
    ) -> ((RotationalFaceQ, RotationalFaceQ), f64, f64) {
        let Self {
            us: (u0, u1),
            plane: (c, k),
            v0,
            ..
        } = self;
        let r = chart.r;
        let v_at = |u: f64| c + k * r * u.cos();
        let top_harmonic = TrimEdgeQ {
            u: chan(0.0, 1.0),
            v: HarmChan {
                c0: pt(c),
                ca: pt(k * r),
                cb: Interval::zero(),
                cl: Interval::zero(),
            },
            t0: pt(u0),
            t1: pt(u1),
            forward: false,
            trig0: trig_at_limit(pt(u0)),
            trig1: Some(trig_at_limit(pt(u1))),
            env: Interval::zero(),
        };
        let edges = vec![
            Edge {
                harmonic: harmonic(chan(0.0, 1.0), chan(v0, 0.0), (u0, u1), true),
                swap: None,
            },
            Edge {
                harmonic: harmonic(chan(u1, 0.0), chan(0.0, 1.0), (v0, v_at(u1)), true),
                swap: None,
            },
            Edge {
                harmonic: top_harmonic,
                swap: Some(section_net(chart, (c, k), (u0, u1), self.segments)),
            },
            Edge {
                harmonic: harmonic(chan(u0, 0.0), chan(0.0, 1.0), (v0, v_at(u0)), false),
                swap: None,
            },
        ];
        let va = edges
            .iter()
            .fold(Vec3::zero(), |s, e| s + channel_vector_area(chart, &e.harmonic));
        let a = (c - v0) * (u1 - u0) + k * r * (u1.sin() - u0.sin());
        let flux = r * r * a + (chart.o - about).dot(va);
        (
            face(chart, &[edges], va, about, Deck::AsMinted),
            flux,
            r * a.abs(),
        )
    }
}

/// **A swapped plane section, whose level and azimuth both move, holds
/// the closed form**: `F(v)·u′` read as a product neither of whose
/// factors is constant.
#[test]
fn a_swapped_plane_section_holds_the_closed_form() {
    let chart = Chart::tilted(Point3::new(0.4, -0.2, 0.3), 0.8);
    for section in [
        Section::new("short", (0.3, 1.4), (1.1, 0.6), 0.1, 1),
        Section::new("past a quarter turn", (0.2, 2.9), (1.4, 0.7), -0.2, 2),
        Section::new("across the seam", (-1.0, 0.8), (1.3, -0.5), 0.0, 2),
        Section::new("winding most of a turn", (-2.8, 2.8), (2.0, 0.9), 0.0, 3),
    ] {
        let (faces, flux, area) = section.faces(chart, chart.o);
        holds(section.label, &faces, flux, area);
    }
}

/// **A winding chord loop traversed against its carriers holds the
/// band**: the band's top rim three projected arcs walked backwards, its
/// bottom harmonic, about a point off the chart.
#[test]
fn a_reversed_winding_chord_loop_holds_the_band() {
    let chart = Chart::tilted(Point3::new(0.4, -0.2, 0.3), 0.8);
    let tau = core::f64::consts::TAU;
    let (v0, v1) = (0.1, 0.9);
    let third = tau / 3.0;
    let bottom = vec![Edge {
        harmonic: harmonic(chan(0.0, 1.0), chan(v0, 0.0), (0.0, tau), true),
        swap: None,
    }];
    let top: Vec<Edge> = (0..3)
        .rev()
        .map(|k| {
            let ua = third * f64::from(k);
            Edge {
                harmonic: harmonic(chan(0.0, 1.0), chan(v1, 0.0), (ua, ua + third), false),
                swap: Some(rim_net(chart, v1, (ua, ua + third), 1)),
            }
        })
        .collect();
    let about = Point3::new(0.1, 0.5, -0.2);
    let a = tau * (v1 - v0);
    holds(
        "a reversed winding chord loop",
        &face(chart, &[bottom, top], Vec3::zero(), about, Deck::AsMinted),
        chart.r * chart.r * a,
        chart.r * a,
    );
}

/// The chords of [`projected_integrals_hold_high_precision_truths`]:
/// (label, knots, degree, control points in the chart's frame, weights).
#[allow(clippy::type_complexity)]
fn chords() -> Vec<(&'static str, Vec<f64>, usize, Vec<[f64; 3]>, Vec<f64>)> {
    let polar = |deg: f64, rho: f64, z: f64| {
        let a = deg.to_radians();
        [rho * a.cos(), rho * a.sin(), z]
    };
    vec![
        (
            "a rational cubic past a quarter turn",
            vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
            3,
            vec![
                [1.0, 0.0, 0.0],
                [1.0, 1.2, 0.3],
                [-0.5, 1.3, 0.9],
                [-1.1, 0.2, 1.4],
            ],
            vec![1.0, 0.7, 1.3, 1.0],
        ),
        (
            "the same cubic turned across the seam",
            vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
            3,
            vec![
                polar(150.0, 1.0, 0.0),
                polar(200.0, 1.562_049_9, 0.3),
                polar(260.0, 1.392_838_8, 0.9),
                polar(320.0, 1.118_034_0, 1.4),
            ],
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

/// **The rule's integrals hold 40-digit truths**: `[−∫F du, −∫G du, ∫du]`
/// of each of [`chords`] at `r = 0.8`, `v = 0.3 − z`, anchor `0.25`, with
/// an extra break at `0.37`, at 8 and 128 pieces a span, against the
/// truths in `framed_truths.in`, which `framed_truths.py` beside it
/// writes (mpmath at 40 digits: de Boor on the same `f64` control
/// values, `u′` from the curve's own derivative, tanh-sinh per span).
// The truths are quoted at the reference's digits, rounded at the parse.
#[allow(clippy::excessive_precision)]
#[test]
fn projected_integrals_hold_high_precision_truths() {
    let truths: [[f64; 3]; 3] = include!("framed_truths.in");
    let radius = pt(0.8);
    for ((label, knots, degree, control, w), truth) in chords().into_iter().zip(truths) {
        let n = net(
            knots,
            degree,
            control
                .iter()
                .map(|p| Point3::new(p[0], p[1], p[2]))
                .collect(),
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
        for pieces in [1, 2, 3, 8, 128] {
            let (got, _) = projected_integrals(&e, |_| pieces, |at| {
                let [f, g] = cylinder_projected_forms(radius, &e, at, 0.25);
                let minus = -Interval::one();
                [f.mul(at.du).scale(minus), g.mul(at.du).scale(minus), at.du]
            });
            for k in 0..3 {
                encloses(got[k], truth[k], &format!("{label} @{pieces} [{k}]"));
            }
        }
    }
}

/// One round of `face` exactly, taken whatever its width.
fn at_round(face: &RotationalFaceQ, round: usize) -> FaceCutBounds {
    let band = Band::linear(Tol::witness()).unwrap();
    rotational_cut_face_rounds::<f64>(face, band, RoundWindow::at(round), &mut to_target(1e6))
        .unwrap_or_else(|e| panic!("round {round}: refused {e:?}"))
        .bounds
}

/// **Many-span sections hold at every round about a point off the
/// chart**, so the chart term's rationals are read through a nonzero
/// lever: rounds 0 to 3, each taken whatever its width.
#[test]
fn many_span_sections_hold_at_every_round_about_a_point_off_the_chart() {
    let chart = Chart::tilted(Point3::new(0.4, -0.2, 0.3), 0.8);
    let about = Point3::new(1.1, 0.5, -0.7);
    for (label, us, plane, v0) in [
        ("winding most of a turn", (-2.8, 2.8), (2.0, 0.9), 0.0),
        ("across the seam", (-1.0, 0.8), (1.3, -0.5), 0.0),
        ("past a quarter turn", (0.2, 2.9), (1.4, 0.7), -0.2),
        ("steep and nearly a turn", (-3.1, 3.1), (0.0, 3.0), -3.0),
    ] {
        for segments in [16, 23, 64] {
            let section = Section::new(label, us, plane, v0, segments);
            let ((_, swapped), flux, area) = section.faces(chart, about);
            for round in 0..4 {
                let got = at_round(&swapped, round);
                let tag = format!("{label}, {segments} segments, round {round}");
                encloses(got.flux, flux, &format!("{tag}: flux"));
                encloses(got.area, area, &format!("{tag}: area"));
            }
        }
    }
}

/// **A circle cut into many intervals holds at every round**: one of
/// its 17 intervals most of the arc (5.4 rad), and the arc cut evenly
/// into 40; each interval is read in the pieces its own turn and length
/// need ([`span_schedule`]).
#[test]
fn a_circle_of_many_cuts_holds_at_every_round() {
    let chart = Chart::tilted(Point3::new(0.4, -0.2, 0.3), 0.8);
    let (us, vs) = ((-2.8, 2.8), (0.2, 1.1));
    let uneven: Vec<f64> = (0..=16)
        .map(|i| us.0 + 0.2 * f64::from(i) / 16.0)
        .chain([us.1])
        .collect();
    let even: Vec<f64> = (0..=40)
        .map(|i| us.0 + (us.1 - us.0) * f64::from(i) / 40.0)
        .collect();
    let (flux, area, _) = rectangle_truth(chart, us, vs, chart.o, 1.0);
    for (label, cuts) in [("uneven", uneven), ("even", even)] {
        let circle = rim_circle(
            chart,
            vs.0,
            (0.0, 1.0),
            cuts,
            true,
            [(us.0, vs.0), (us.1, vs.0)],
        );
        let mut loops: Vec<EdgeTermQ> = rectangle(chart, us, vs, &[])
            .into_iter()
            .map(|e| EdgeTermQ::Harmonic(e.harmonic))
            .collect();
        loops[0] = circle;
        let face = RotationalFaceQ {
            chart: RotationalChartQ::Cylinder { radius: pt(chart.r) },
            chart_term: ChartTermQ {
                closed: Interval::zero(),
                lever: [Interval::zero(); 3],
                rationals: Vec::new(),
                ribbon: 0.0,
            },
            loops: vec![loops],
        };
        for round in 0..3 {
            let got = at_round(&face, round);
            encloses(got.flux, flux, &format!("circle {label} r{round}: flux"));
            encloses(got.area, area, &format!("circle {label} r{round}: area"));
        }
        framed_holds(chart, us, vs, 0, face.loops[0][0].clone(), &format!("circle {label} to target"));
    }
}

/// The plane section `v = c + k·r·cos u` over `[−2.8, 2.8]` on a chart
/// whose frame is the world's, as a projected edge over its carrier
/// interval `ts`.
fn frame_section(segments: usize, ts: (f64, f64)) -> ProjectedEdgeQ {
    let chart = Chart {
        o: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
        r: 0.8,
    };
    let n = section_net(chart, (2.0, 0.9), (-2.8, 2.8), segments);
    let q = rational_edge_q_split(
        &n.map_scalar(Interval::from_certified),
        (pt(ts.0), pt(ts.1)),
        true,
        Point3::new(pt(0.0), pt(0.0), pt(0.0)),
        &[],
    )
    .unwrap();
    ProjectedEdgeQ {
        carrier: FramedQ::Net(q),
        v_off: Interval::zero(),
        v_sign: 1.0,
        forward: true,
        ends: [(pt(0.0), pt(0.0)); 2],
        env: Interval::zero(),
    }
}

/// **A boundary reads the same pieces however it is cut into edges**: a
/// 20-span section read as one edge and as its two 10-span halves, cut
/// at a knot, schedules each span alike ([`span_schedule`]).
#[test]
fn a_chord_read_as_one_edge_or_two_schedules_each_span_alike() {
    let chart = RotationalChartQ::Cylinder { radius: pt(0.8) };
    let whole = span_schedule(&frame_section(20, (0.0, 1.0)), chart).0;
    let halves: Vec<usize> = [(0.0, 0.5), (0.5, 1.0)]
        .into_iter()
        .flat_map(|ts| span_schedule(&frame_section(20, ts), chart).0)
        .collect();
    assert_eq!(whole.len(), 20, "the section's spans");
    assert_eq!(whole, halves, "one edge against its two halves");
}

/// **A face's work is the pieces its rounds read**: a harmonic edge's
/// [`EDGE_INIT_PIECES`] and a 40-span chord's one piece a span (each
/// span a fraction of the chart's sixteenth), both doubling per round.
#[test]
fn a_faces_work_is_the_pieces_its_rounds_read() {
    let chart = RotationalChartQ::Cylinder { radius: pt(0.8) };
    let chord = EdgeTermQ::Projected(frame_section(40, (0.0, 1.0)));
    let rim = EdgeTermQ::Harmonic(harmonic(chan(0.0, 1.0), chan(0.0, 0.0), (0.0, 1.0), true));
    let face = RotationalFaceQ {
        chart,
        chart_term: ChartTermQ {
            closed: Interval::zero(),
            lever: [Interval::zero(); 3],
            rationals: Vec::new(),
            ribbon: 0.0,
        },
        loops: vec![vec![rim, chord]],
    };
    let entries = vec![
        face.loops[0]
            .iter()
            .map(|e| edge_entry(e, chart).unwrap())
            .collect::<Vec<_>>(),
    ];
    assert_eq!(entries[0][1].schedule.0, vec![1; 40], "one piece a span");
    for round in 0..4 {
        assert_eq!(
            face_cost(&face, &entries, &[], round),
            (W_CYLINDER * EDGE_INIT_PIECES + w_rational(2) * 40) << round,
            "round {round}"
        );
    }
}
