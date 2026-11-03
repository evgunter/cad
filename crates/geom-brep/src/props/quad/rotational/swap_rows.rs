//! **Carrier-swap rows**: a cylinder face whose loops are harmonic,
//! with one boundary carrier rewritten as a NURBS that represents it
//! exactly — a ruling as a degree-3 line, a rim arc as a rational
//! quadratic — so that edge mints a projected image. The lane's
//! enclosures over the swapped face must contain the face's closed-form
//! flux and area, which the harmonic face's own enclosures contain too.
//!
//! Each row is a containment check against exact arithmetic, never a
//! width: a swapped edge whose term were wrong by any amount the
//! enclosure does not cover fails it. A complementary pair of faces is
//! no oracle here, since a chord bounds both with opposite traversals
//! and a consistent error in its term cancels from their sum.

use super::*;
use crate::pcurve_cache::{PcurveCache, chart_pcurve_over};
use crate::props::quad::test_target::{encloses, met, to_target};
use crate::props::rational_rim::rational_edge_q;
use geom::{Curve3, NurbsCurve3, Surface};
use geom_core::spline::{KnotVector, Knots};
use geom_core::{Point3, Tol, Vec3};

/// A cylinder chart `S(u, v) = o + r·(cos u·û + sin u·v̂) + v·â`.
#[derive(Clone, Copy)]
struct Chart {
    o: Point3<f64>,
    axis: Vec3<f64>,
    u_ref: Vec3<f64>,
    r: f64,
}

impl Chart {
    fn tilted(o: Point3<f64>, r: f64) -> Self {
        let axis = Vec3::new(0.2, -0.3, 1.0);
        let axis = axis / axis.norm();
        let u_ref = axis.cross(Vec3::new(1.0, 0.0, 0.0));
        Self {
            o,
            axis,
            u_ref: u_ref / u_ref.norm(),
            r,
        }
    }

    fn v_ref(self) -> Vec3<f64> {
        self.axis.cross(self.u_ref)
    }

    fn radial(self, u: f64) -> Vec3<f64> {
        self.u_ref * u.cos() + self.v_ref() * u.sin()
    }

    fn at(self, u: f64, v: f64) -> Point3<f64> {
        self.o + self.radial(u) * self.r + self.axis * v
    }

    fn surface(self) -> Surface<f64> {
        Surface::Cylinder {
            origin: self.o,
            axis: self.axis,
            radius: self.r,
            u_ref: self.u_ref,
        }
    }

    /// `∫∫ n dA` over the chart rectangle `[u0, u1] × [v0, v1]`.
    fn vector_area(self, (u0, u1): (f64, f64), (v0, v1): (f64, f64)) -> Vec3<f64> {
        (self.u_ref * (u1.sin() - u0.sin()) + self.v_ref() * (u0.cos() - u1.cos()))
            * (self.r * (v1 - v0))
    }
}

/// One edge of a fixture loop: its harmonic channels over its interval,
/// its traversal, and — where it is swapped — the exact NURBS of its
/// carrier, parametrized on `[0, 1]` in the harmonic edge's own
/// direction.
struct Edge {
    harmonic: TrimEdgeQ,
    swap: Option<NurbsCurve3<f64>>,
}

fn chan(c0: f64, cl: f64) -> HarmChan {
    HarmChan {
        c0: pt(c0),
        ca: Interval::zero(),
        cb: Interval::zero(),
        cl: pt(cl),
    }
}

fn harmonic(u: HarmChan, v: HarmChan, (t0, t1): (f64, f64), forward: bool) -> TrimEdgeQ {
    TrimEdgeQ {
        u,
        v,
        t0: pt(t0),
        t1: pt(t1),
        forward,
        trig0: trig_at_limit(pt(t0)),
        trig1: Some(trig_at_limit(pt(t1))),
        env: Interval::zero(),
    }
}

use super::super::trig::trig_at_limit;

fn net(knots: Vec<f64>, degree: usize, control: Vec<Point3<f64>>, w: Vec<f64>) -> NurbsCurve3<f64> {
    NurbsCurve3::new(
        Knots::from(KnotVector::clamped(knots, degree).unwrap()),
        control,
        w,
    )
    .unwrap()
}

/// The ruling `u = uc` from `va` to `vb` as a degree-3 line.
fn ruling_net(chart: Chart, uc: f64, (va, vb): (f64, f64)) -> NurbsCurve3<f64> {
    net(
        vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
        3,
        (0..4)
            .map(|i| chart.at(uc, va + (vb - va) * f64::from(i) / 3.0))
            .collect(),
        vec![1.0; 4],
    )
}

/// The rim `v = vc` from `ua` to `ub` as a rational quadratic, one
/// segment per `segments`.
fn rim_net(chart: Chart, vc: f64, (ua, ub): (f64, f64), segments: usize) -> NurbsCurve3<f64> {
    #[allow(clippy::cast_precision_loss)]
    let n = segments as f64;
    let half = (ub - ua) / (2.0 * n);
    let mut control = vec![chart.at(ua, vc)];
    let mut weights = vec![1.0];
    let mut knots = vec![0.0; 3];
    for k in 0..segments {
        #[allow(clippy::cast_precision_loss)]
        let a = ua + (ub - ua) * (k as f64) / n;
        let m = a + half;
        control.push(chart.o + chart.radial(m) * (chart.r / half.cos()) + chart.axis * vc);
        weights.push(half.cos());
        control.push(chart.at(a + 2.0 * half, vc));
        weights.push(1.0);
        if k + 1 < segments {
            #[allow(clippy::cast_precision_loss)]
            knots.extend([(k + 1) as f64 / n; 2]);
        }
    }
    knots.extend([1.0; 3]);
    net(knots, 2, control, weights)
}

/// The rectangle `[u0, u1] × [v0, v1]` counterclockwise in the chart
/// (bottom, right, top, left), with the edges `swap` names rewritten:
/// `0`/`2` the bottom/top rims, `1`/`3` the right/left rulings.
fn rectangle(
    chart: Chart,
    (u0, u1): (f64, f64),
    (v0, v1): (f64, f64),
    swap: &[usize],
) -> Vec<Edge> {
    let segments = if u1 - u0 > 2.0 { 2 } else { 1 };
    let edges = [
        (
            harmonic(chan(0.0, 1.0), chan(v0, 0.0), (u0, u1), true),
            rim_net(chart, v0, (u0, u1), segments),
        ),
        (
            harmonic(chan(u1, 0.0), chan(0.0, 1.0), (v0, v1), true),
            ruling_net(chart, u1, (v0, v1)),
        ),
        (
            harmonic(chan(0.0, 1.0), chan(v1, 0.0), (u0, u1), false),
            rim_net(chart, v1, (u0, u1), segments),
        ),
        (
            harmonic(chan(u0, 0.0), chan(0.0, 1.0), (v0, v1), false),
            ruling_net(chart, u0, (v0, v1)),
        ),
    ];
    edges
        .into_iter()
        .enumerate()
        .map(|(i, (harmonic, n))| Edge {
            harmonic,
            swap: swap.contains(&i).then_some(n),
        })
        .collect()
}

/// `½∫(P − o)×P′` of an edge's exact carrier, traversal folded in, by
/// the curved rims' rule at a resolution far past the rows' widths — its
/// own row (`rational_rim`) holds it to 40-digit truths. The closed part
/// of a row's chart term is the face's exact `A⃗` less these.
fn carrier_term(chart: Chart, n: &NurbsCurve3<f64>, forward: bool) -> RationalEdgeQ {
    let iv = Interval::from_certified;
    rational_edge_q(
        &n.map_scalar(iv),
        pt(0.0),
        pt(1.0),
        forward,
        Point3::new(iv(chart.o.x), iv(chart.o.y), iv(chart.o.z)),
    )
    .unwrap()
}

/// How a row reads its swapped images: as minted, or mirrored in `v`
/// (the deck sign `−1`), its harmonic edges then read in the mirrored
/// chart too.
#[derive(Clone, Copy, PartialEq)]
enum Deck {
    AsMinted,
    Mirrored,
}

/// The face of `loops` on `chart`, its chart term about `about`, every
/// swapped edge projected, and the closed-form chart term `A⃗` the
/// harmonic face carries.
fn face(
    chart: Chart,
    loops: &[Vec<Edge>],
    vector_area: Vec3<f64>,
    about: Point3<f64>,
    deck: Deck,
) -> (RotationalFaceQ, RotationalFaceQ) {
    let band = Band::linear(Tol::witness()).unwrap();
    let surface = chart.surface();
    let lever = chart.o - about;
    let lever = [pt(lever.x), pt(lever.y), pt(lever.z)];
    let mirror = |e: TrimEdgeQ| match deck {
        Deck::AsMinted => e,
        Deck::Mirrored => TrimEdgeQ {
            v: HarmChan {
                c0: -e.v.c0,
                ca: -e.v.ca,
                cb: -e.v.cb,
                cl: -e.v.cl,
            },
            ..e
        },
    };
    let dot = |a: [Interval; 3], v: Vec3<f64>| a[0] * pt(v.x) + a[1] * pt(v.y) + a[2] * pt(v.z);
    let mut rationals = Vec::new();
    let mut swapped = Vec::new();
    for edges in loops {
        let mut out = Vec::new();
        for e in edges {
            let Some(n) = &e.swap else {
                out.push(EdgeTermQ::Harmonic(mirror(e.harmonic)));
                continue;
            };
            let carrier = Curve3::Nurbs(std::sync::Arc::new(n.clone()));
            let image = chart_pcurve_over(&carrier, 0.0, 1.0, &surface, band).unwrap();
            let env = PcurveCache::certify_projected(
                image.projected().unwrap().clone(),
                0.0,
                1.0,
                &carrier,
                &surface,
                band,
                Some(crate::FittedLane::<f64>::certified()),
            )
            .unwrap()
            .certificate()
            .envelope;
            let image = match deck {
                Deck::AsMinted => image,
                Deck::Mirrored => image.mirror_v().unwrap(),
            };
            let projected = image.projected().unwrap();
            let at = |t: f64| {
                let p = image.eval(t);
                (pt(p.x), pt(p.y))
            };
            out.push(EdgeTermQ::Projected(
                ProjectedEdgeQ::of_image(
                    projected,
                    (pt(0.0), pt(1.0)),
                    e.harmonic.forward,
                    [at(0.0), at(1.0)],
                    pt(env),
                    pt,
                )
                .unwrap(),
            ));
            rationals.push(carrier_term(chart, n, e.harmonic.forward));
        }
        swapped.push(out);
    }
    // The swapped carriers' exact terms, read at a resolution whose width
    // is far below the rows', taken off the closed part.
    let swapped_va = rationals.iter().fold([Interval::zero(); 3], |acc, e| {
        let (va, _) = rational_vector_area_split(e, 4096);
        [0, 1, 2].map(|k| acc[k] + va[k])
    });
    let whole = dot(lever, vector_area);
    let closed = whole - (0..3).fold(Interval::zero(), |a, k| a + lever[k] * swapped_va[k]);
    let cylinder = RotationalChartQ::Cylinder {
        radius: pt(chart.r),
    };
    let harmonic = RotationalFaceQ {
        chart: cylinder,
        chart_term: ChartTermQ {
            closed: whole,
            lever: [Interval::zero(); 3],
            rationals: Vec::new(),
        },
        loops: loops
            .iter()
            .map(|edges| {
                edges
                    .iter()
                    .map(|e| EdgeTermQ::Harmonic(mirror(e.harmonic)))
                    .collect()
            })
            .collect(),
    };
    let swapped = RotationalFaceQ {
        chart: cylinder,
        chart_term: ChartTermQ {
            closed,
            lever,
            rationals,
        },
        loops: swapped,
    };
    (harmonic, swapped)
}

/// Both faces' enclosures hold the closed forms `flux`, `area`.
fn holds(
    label: &str,
    (harmonic, swapped): &(RotationalFaceQ, RotationalFaceQ),
    flux: f64,
    area: f64,
) {
    let band = Band::linear(Tol::witness()).unwrap();
    for (which, face) in [("harmonic", harmonic), ("swapped", swapped)] {
        let got = met(rotational_cut_face_rounds::<f64>(
            face,
            band,
            RoundWindow::SCHEDULE,
            &mut to_target(1e-9),
        ))
        .unwrap_or_else(|e| panic!("{label} ({which}): the lane refused: {e:?}"));
        encloses(got.flux, flux, &format!("{label} ({which}) flux"));
        encloses(got.area, area, &format!("{label} ({which}) area"));
    }
}

/// The closed forms of the rectangle `[u0, u1] × [v0, v1]` about
/// `about`: `r²·A + (o − about)·A⃗` and `r·|A|`, `A` signed by `sign`.
fn rectangle_truth(
    chart: Chart,
    us: (f64, f64),
    vs: (f64, f64),
    about: Point3<f64>,
    sign: f64,
) -> (f64, f64, Vec3<f64>) {
    let a = (us.1 - us.0) * (vs.1 - vs.0) * sign;
    let va = chart.vector_area(us, vs);
    (
        chart.r * chart.r * a + (chart.o - about).dot(va),
        chart.r * a.abs(),
        va,
    )
}

/// **A swapped ruling, rim, seam-crossing rim, a rim past a quarter
/// turn, and every edge at once hold the closed forms**, on a tilted
/// chart, about a point off it.
#[test]
fn a_swapped_edge_holds_the_closed_forms() {
    let chart = Chart::tilted(Point3::new(0.4, -0.2, 0.3), 0.8);
    let about = Point3::new(0.1, 0.5, -0.2);
    for (label, us, vs, swap) in [
        ("a ruling", (0.3, 1.4), (0.2, 1.1), vec![1]),
        ("a rim", (0.3, 1.4), (0.2, 1.1), vec![0]),
        ("a rim across the seam", (-0.5, 0.6), (0.2, 1.1), vec![2]),
        (
            "a rim past a quarter turn",
            (0.2, 2.6),
            (-0.4, 0.7),
            vec![0, 2],
        ),
        ("every edge", (0.3, 1.4), (0.2, 1.1), vec![0, 1, 2, 3]),
    ] {
        let (flux, area, va) = rectangle_truth(chart, us, vs, about, 1.0);
        let loops = [rectangle(chart, us, vs, &swap)];
        holds(
            label,
            &face(chart, &loops, va, about, Deck::AsMinted),
            flux,
            area,
        );
    }
}

/// **A mirrored image holds the closed form**: the face read in the
/// chart mirrored in `v` (the deck sign `−1` on its swapped edges, `−v`
/// on its harmonic ones) is wound the other way, so its signed area
/// is `−A`; its chart term is zero, about the chart's own origin.
#[test]
fn a_mirrored_image_holds_the_closed_form() {
    let chart = Chart::tilted(Point3::new(0.4, -0.2, 0.3), 0.8);
    let (us, vs) = ((0.3, 1.4), (0.2, 1.1));
    let a = (us.1 - us.0) * (vs.1 - vs.0);
    let loops = [rectangle(chart, us, vs, &[1, 2])];
    holds(
        "mirrored",
        &face(chart, &loops, Vec3::zero(), chart.o, Deck::Mirrored),
        -chart.r * chart.r * a,
        chart.r * a,
    );
}

/// **A chord loop that winds the cylinder holds the band's closed
/// form**: the band between two rims, its bottom rim three projected
/// arcs of a third of a turn each, its top one harmonic.
#[test]
fn a_winding_chord_loop_holds_the_band() {
    let chart = Chart::tilted(Point3::new(0.4, -0.2, 0.3), 0.8);
    let tau = core::f64::consts::TAU;
    let (v0, v1) = (0.1, 0.9);
    let third = tau / 3.0;
    let bottom: Vec<Edge> = (0..3)
        .map(|k| {
            let ua = third * f64::from(k);
            Edge {
                harmonic: harmonic(chan(0.0, 1.0), chan(v0, 0.0), (ua, ua + third), true),
                swap: Some(rim_net(chart, v0, (ua, ua + third), 1)),
            }
        })
        .collect();
    let top = vec![Edge {
        harmonic: harmonic(chan(0.0, 1.0), chan(v1, 0.0), (0.0, tau), false),
        swap: None,
    }];
    let about = Point3::new(0.1, 0.5, -0.2);
    let a = tau * (v1 - v0);
    holds(
        "a winding chord loop",
        &face(chart, &[bottom, top], Vec3::zero(), about, Deck::AsMinted),
        chart.r * chart.r * a,
        chart.r * a,
    );
}

/// **A projected ring in a harmonic face holds the difference**: the
/// outer rectangle harmonic, a ring inside it every edge projected and
/// wound against it.
#[test]
fn a_projected_ring_holds_the_difference() {
    let chart = Chart::tilted(Point3::new(0.4, -0.2, 0.3), 0.8);
    let about = Point3::new(0.1, 0.5, -0.2);
    let (outer_u, outer_v) = ((0.1, 2.0), (-0.3, 1.2));
    let (inner_u, inner_v) = ((0.6, 1.3), (0.1, 0.6));
    let outer = rectangle(chart, outer_u, outer_v, &[]);
    // The ring: the same rectangle walked the other way.
    let ring: Vec<Edge> = rectangle(chart, inner_u, inner_v, &[0, 1, 2, 3])
        .into_iter()
        .rev()
        .map(|e| Edge {
            harmonic: TrimEdgeQ {
                forward: !e.harmonic.forward,
                ..e.harmonic
            },
            swap: e.swap,
        })
        .collect();
    let (f_out, a_out, va_out) = rectangle_truth(chart, outer_u, outer_v, about, 1.0);
    let (f_in, a_in, va_in) = rectangle_truth(chart, inner_u, inner_v, about, 1.0);
    holds(
        "a projected ring",
        &face(chart, &[outer, ring], va_out - va_in, about, Deck::AsMinted),
        f_out - f_in,
        a_out - a_in,
    );
}

/// **The face a thousand metres out holds its closed form**: the
/// swapped-ruling face with the chart's origin at `1e3` and the flux
/// about the world origin, so the chart term is `o·A⃗` at that lever.
#[test]
fn a_far_face_holds_its_closed_form() {
    let chart = Chart::tilted(Point3::new(1e3, -1e3, 5e2), 0.8);
    let about = Point3::origin();
    let (us, vs) = ((0.3, 1.4), (0.2, 1.1));
    let (flux, area, va) = rectangle_truth(chart, us, vs, about, 1.0);
    let loops = [rectangle(chart, us, vs, &[1, 2])];
    holds(
        "a thousand metres out",
        &face(chart, &loops, va, about, Deck::AsMinted),
        flux,
        area,
    );
}

/// **A projected circle holds the closed form**: the rectangle's bottom
/// rim read as a circle in the chart's frame (`FramedQ::Circle`, cut at
/// a break inside it), the rest harmonic, about the chart's own origin.
#[test]
fn a_projected_circle_holds_the_closed_form() {
    let chart = Chart::tilted(Point3::new(0.4, -0.2, 0.3), 0.8);
    let (us, vs) = ((0.3, 2.4), (0.2, 1.1));
    let (flux, area, _) = rectangle_truth(chart, us, vs, chart.o, 1.0);
    let r = pt(chart.r);
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
        ends: [(pt(us.0), pt(vs.0)), (pt(us.1), pt(vs.0))],
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
    .unwrap();
    encloses(got.flux, flux, "a projected circle: flux");
    encloses(got.area, area, "a projected circle: area");
}

/// **A projected line holds the closed form**: the rectangle's right
/// ruling read as a line in the chart's frame (`FramedQ::Line`, cut at a
/// break), the rest harmonic, about the chart's own origin.
#[test]
fn a_projected_line_holds_the_closed_form() {
    let chart = Chart::tilted(Point3::new(0.4, -0.2, 0.3), 0.8);
    let (us, vs) = ((0.3, 1.4), (0.2, 1.1));
    let (flux, area, _) = rectangle_truth(chart, us, vs, chart.o, 1.0);
    let r = chart.r;
    let line = EdgeTermQ::Projected(ProjectedEdgeQ {
        carrier: FramedQ::Line {
            point: [pt(r * us.1.cos()), pt(r * us.1.sin()), Interval::zero()],
            dir: [Interval::zero(), Interval::zero(), Interval::one()],
            cuts: vec![vs.0, 0.7, vs.1],
        },
        v_off: Interval::zero(),
        v_sign: 1.0,
        forward: true,
        ends: [(pt(us.1), pt(vs.0)), (pt(us.1), pt(vs.1))],
        env: Interval::zero(),
    });
    let mut loops: Vec<EdgeTermQ> = rectangle(chart, us, vs, &[])
        .into_iter()
        .map(|e| EdgeTermQ::Harmonic(e.harmonic))
        .collect();
    loops[1] = line;
    let face = RotationalFaceQ {
        chart: RotationalChartQ::Cylinder { radius: pt(r) },
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
    .unwrap();
    encloses(got.flux, flux, "a projected line: flux");
    encloses(got.area, area, "a projected line: area");
}

#[cfg(test)]
mod probes_r2;
