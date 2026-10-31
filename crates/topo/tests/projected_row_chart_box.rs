//! **A projected row's chart box**: what `Pcurve::chart_box` reads of
//! a spline carrier's projected image on a plane chart, against what
//! the image's interval `eval` reads over the same span.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::spline::Knots;
use std::sync::Arc;
use topo::construction::Construct;

use geom::{Curve3, NurbsCurve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
use geom_core::spline::KnotVector;
use geom_core::{Band, Bounds, Interval, Point3, Real, SpanLocate, Tol, Vec3};
use topo::{Body, LoopBoundary};

use crate::common;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn base_plane() -> Surface<Interval> {
    Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
        u_ref: Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
    }
}

/// The prism's face surface, cloned.
fn face_chart(body: &Body<Interval>, face: topo::FaceKey) -> Surface<Interval> {
    let key = body.get_face(face).unwrap().surface;
    body.get_surface(key).unwrap().clone()
}

/// Swaps the carrier of `face`'s first outer edge for a degree-1
/// spline net along the same chord, under an `Intersection`
/// description, the edge reading the net over `along` (its parameter
/// range on the net's `[0, 1]`): `(0, 1)` is the chord itself, and a
/// strict sub-range extends the net past the edge's ends along the
/// chord. Returns the new carrier.
fn swap_first_edge_for_a_chord_net(
    body: &mut Body<Interval>,
    face: topo::FaceKey,
    along: (f64, f64),
) -> Curve3<Interval> {
    let edge_key = {
        let lp = body.get_face(face).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(lp).unwrap().boundary else {
            panic!("the outer loop is a cycle")
        };
        body.get_half_edge(first).unwrap().edge
    };
    let edge = body.get_edge(edge_key).unwrap().clone();
    let face_of = |body: &Body<Interval>, he| {
        let lp = body.get_half_edge(he).unwrap().parent_loop;
        body.get_loop(lp).unwrap().face
    };
    let s1 = body.get_face(face_of(body, edge.he_plus)).unwrap().surface;
    let s2 = body.get_face(face_of(body, edge.he_minus)).unwrap().surface;
    let start = body.get_half_edge(edge.he_plus).unwrap().start;
    let end = body.half_edge_end(edge.he_plus).unwrap();
    let q0 = *body
        .get_point(body.get_vertex(start).unwrap().point)
        .unwrap();
    let q1 = *body.get_point(body.get_vertex(end).unwrap().point).unwrap();
    // The net's ends, so that the net reads `q0` at `along.0` and `q1`
    // at `along.1`.
    let (t0, t1) = along;
    let d = (q1 - q0) / iv(t1 - t0);
    let (n0, n1) = (q0 - d * iv(t0), q0 + d * iv(1.0 - t0));
    let knots = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let spline = NurbsCurve3::new(
        Knots::from(knots).lift(geom_core::Real::from_f64),
        vec![n0, n1],
        vec![Real::one(); 2],
    )
    .expect("the chord as a degree-1 spline");
    let carrier = Curve3::Nurbs(Arc::new(spline));
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            pair: SurfacePair::new(s1, s2),
            selector: geom_brep::CurveSelector::Witness(q0.lerp(q1, iv(0.5))),
        },
        carrier: carrier.clone(),
        param_start: iv(t0),
        param_end: iv(t1),
    };
    body.set_edge_curve(edge_key, spec, Tol::witness())
        .expect("a rung-3 carrier under an Intersection description certifies");
    carrier
}

/// **A projected row's box against its `eval`**: why a reader of a
/// projected row's chart extent reads `Pcurve::chart_box` rather than
/// `Pcurve::eval` over the walked interval, as the closed forms do
/// (`work/nurbs/nurbs-curve-interval-eval-loses-the-partition-of-unity-over-a-wide-t.md`).
///
/// The spline's interval evaluation encloses each basis function on its
/// own, so its sums lose the partition of unity. On a degree-1 chord `u = 0.5`, `v ∈ [−0.5, 0.5]` on a plane chart:
/// - over the whole edge the denominator `(1 − t) + t` encloses
///   `[0, 2]`, and both channels are the entire line, decorated `Trv`.
///   That is not NaI (`Real::is_poison` is false for it), and the
///   numerator is finite;
/// - over `[0.1, 0.9]`, `u` reads `[0.056, 4.5]` (true `0.5`) and `v`
///   reads `[−2, 2]` (true `[−0.4, 0.4]`).
///
/// The box over the whole net is the image's exact hull. Over a strict
/// sub-range it reads the whole pieces the sub-range meets: over
/// `[0.1, 0.9]` it is still `v ∈ [−0.5, 0.5]`, against the true
/// `[−0.4, 0.4]`.
///
/// A red here means re-measure and decide, not retire the box: a half
/// fix that skips the division on a polynomial net turns the whole-edge
/// reading finite while `u` still reads `[0, 1]` over the edge, where
/// the box reads `0.5` exactly.
#[test]
fn a_projected_rows_eval_over_its_edge_is_entire_and_its_box_is_exact() {
    let knots = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let chord = NurbsCurve3::new(
        Knots::from(knots).lift(Real::from_f64),
        vec![
            Point3::new(iv(0.5), iv(-0.5), iv(0.0)),
            Point3::new(iv(0.5), iv(0.5), iv(0.0)),
        ],
        vec![Real::one(); 2],
    )
    .unwrap();
    let image = geom_brep::chart_pcurve(&Curve3::Nurbs(Arc::new(chord)), &base_plane(), band())
        .expect("a chord on the plane has a projected image");
    let width = |x: Interval| x.hi() - x.lo();
    // `Trv` in `repr_bits`' decoration channel.
    const TRV: u8 = 1;

    // The cause, pinned: the entire line at `Trv`, not NaI and not a
    // failed locate's NaN, in each channel.
    let whole = image.eval(iv(0.0).enclosure_hull(iv(1.0)));
    for (what, x) in [("u", whole.x), ("v", whole.y)] {
        assert!(
            x.lo() == f64::NEG_INFINITY
                && x.hi() == f64::INFINITY
                && x.repr_bits().2 == TRV
                && !x.is_poison(),
            "eval's {what} over the whole edge is no longer the entire line at Trv: {x:?}"
        );
    }
    // Inside the edge both channels stay finite and loose. The true `v`
    // width is 0.8 and the true `u` width is 0; each bound is 2.5× the
    // true `v` width, against measured widths of 4.44 (u) and 4.0 (v),
    // so the row goes red only when eval inside an edge tightens by
    // about half or more.
    let inner = image.eval(iv(0.1).enclosure_hull(iv(0.9)));
    assert!(
        width(inner.x).is_finite() && width(inner.x) >= 2.0,
        "eval's u over [0.1, 0.9] is now within 2.5× of the true v width 0.8: {:?}",
        inner.x
    );
    assert!(
        width(inner.y).is_finite() && width(inner.y) >= 2.0,
        "eval's v over [0.1, 0.9] is now within 2.5× of its true width 0.8: {:?}",
        inner.y
    );

    // The box over the whole net is exact, and over a strict sub-range
    // it reads the whole piece.
    for (t0, t1) in [(0.0, 1.0), (0.1, 0.9)] {
        let hull = image.chart_box(iv(t0), iv(t1));
        assert!(
            hull.u_min.lo() == 0.5 && hull.u_max.hi() == 0.5,
            "the box's u over [{t0}, {t1}] is not the chord's 0.5: {hull:?}"
        );
        assert!(
            hull.v_min.lo() == -0.5 && hull.v_max.hi() == 0.5,
            "the box's v over [{t0}, {t1}] is not the net's [-0.5, 0.5]: {hull:?}"
        );
    }
}

/// **A projected edge on a strict sub-range of its net**, where `eval`
/// over the edge is FINITE and still several times too wide, and the
/// box, which reads the whole net, is the tighter enclosure. On a unit
/// prism the first edge's carrier becomes a degree-1 net that reads the
/// edge over `[1/16, 15/16]`, so the net overhangs each end by `1/14` of
/// the edge. Along the chord from the edge's start, `eval` reads
/// `[−3.5, 4.5]` edge lengths and the box `[−1/14, 15/14]`.
#[test]
fn a_projected_edge_on_part_of_its_net_reads_the_tighter_box() {
    let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let built = common::prism::<Interval>(&square, 1.0, geom_core::Tol::witness());
    let mut body = built.body;
    let face = built.bottom_face;
    let chart = face_chart(&body, face);
    let along = (1.0 / 16.0, 15.0 / 16.0);
    let carrier = swap_first_edge_for_a_chord_net(&mut body, face, along);

    let projected = geom_brep::chart_pcurve(&carrier, &chart, band()).expect("a projected image");
    let mid = |x: Interval| 0.5 * (x.lo() + x.hi());
    let (pa, pb) = (projected.eval(iv(along.0)), projected.eval(iv(along.1)));
    let (a, b) = ((mid(pa.x), mid(pa.y)), (mid(pb.x), mid(pb.y)));
    let d = (b.0 - a.0, b.1 - a.1);
    let along_chord = |u: f64, v: f64| (u - a.0) * d.0 + (v - a.1) * d.1;

    let evaluated = projected.eval(iv(along.0).enclosure_hull(iv(along.1)));
    assert!(
        evaluated.x.hi().is_finite() && evaluated.y.hi().is_finite(),
        "eval over the edge is finite: {evaluated:?}"
    );
    let boxed = projected.chart_box(iv(along.0), iv(along.1));
    let reach_eval = along_chord(evaluated.x.hi(), evaluated.y.hi())
        .max(along_chord(evaluated.x.lo(), evaluated.y.lo()));
    let reach_box = along_chord(boxed.u_max.hi(), boxed.v_max.hi())
        .max(along_chord(boxed.u_min.lo(), boxed.v_min.lo()));
    assert!(
        reach_eval >= 4.0 && reach_box <= 1.1,
        "along the chord eval reaches {reach_eval} edge lengths and the box {reach_box}: \
         {evaluated:?} / {boxed:?}"
    );
}

/// **The cell door beside a spline edge.** The unit prism's bottom face
/// with its first edge's carrier swapped for a degree-1 net along the
/// same chord: the face is still the unit square, and the spline edge is
/// read as the ball its control net lies in, and past it piece by piece
/// (`cell_region`'s ball row). Every cell of a grid over the square and
/// its surround the door decides agrees with the square.
///
/// The decided counts are pinned: a door that read the spline edge in
/// any other floor than its control net's ball and then its pieces
/// decides other cells, and the count goes red. A door that called every
/// spline edge clear is a different failure, and
/// [`a_cell_crossing_only_the_spline_edge_is_undecided`] stands against
/// it.
#[test]
fn the_cell_door_reads_a_spline_edge_as_its_ball() {
    use topo::cell_region::{CellRegion, cell_region};
    let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let built = common::prism::<Interval>(&square, 1.0, geom_core::Tol::witness());
    let mut body = built.body;
    let face = built.bottom_face;
    swap_first_edge_for_a_chord_net(&mut body, face, (0.0, 1.0));
    let chart = face_chart(&body, face).map_scalar(|x| 0.5 * (x.lo() + x.hi()));
    let n = 24;
    // Offset from the square's edges, so cells straddle them.
    let (lo, hi) = (-0.52, 1.48);
    let step = |i: usize| lo + (hi - lo) * (i as f64) / (n as f64);
    let (mut inside, mut outside, mut wrong) = (0, 0, 0);
    for i in 0..n {
        for j in 0..n {
            let (cu, cv) = ((step(i), step(i + 1)), (step(j), step(j + 1)));
            let verdict = cell_region(&body, face, cu, cv, band());
            let (mut any_in, mut any_out) = (false, false);
            for p in 0..5 {
                for q in 0..5 {
                    let u = cu.0 + (0.1 + 0.2 * p as f64) * (cu.1 - cu.0);
                    let v = cv.0 + (0.1 + 0.2 * q as f64) * (cv.1 - cv.0);
                    let x = chart.eval(u, v);
                    let skin = 1e-7;
                    let margin = x.x.min(1.0 - x.x).min(x.y).min(1.0 - x.y);
                    if margin > skin {
                        any_in = true;
                    } else if margin < -skin {
                        any_out = true;
                    }
                }
            }
            match verdict {
                CellRegion::Inside => {
                    inside += 1;
                    wrong += usize::from(any_out);
                }
                CellRegion::Outside => {
                    outside += 1;
                    wrong += usize::from(any_in);
                }
                CellRegion::Undecided => {}
            }
        }
    }
    eprintln!("spline-edged square: inside={inside} outside={outside} wrong={wrong}");
    assert_eq!(wrong, 0, "a decided cell contradicts the square");
    assert_eq!(
        (inside, outside),
        (121, 407),
        "the cells the door decides beside the spline edge, pinned"
    );
}

/// **A cell crossing only the spline edge keeps it.** The prism of
/// [`the_cell_door_reads_a_spline_edge_as_its_ball`], its first edge
/// swapped for the chord net; the cell is the rectangle `1.3` deep
/// across the edge (from `1.25` outside it to `0.05` inside the square)
/// and `0.4` along it, about the edge's middle. Its centre, `0.6`
/// outside the edge, lies outside the ball holding the net, and every
/// other edge and every vertex lies past the cell's reach about it, so
/// only the spline edge's floor keeps the cell: a
/// floor that called the edge clear lets the door place the centre,
/// outside the square, and read the cell `Outside` though it reaches
/// into the face.
#[test]
fn a_cell_crossing_only_the_spline_edge_is_undecided() {
    use topo::cell_region::{CellRegion, cell_region};
    let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let built = common::prism::<Interval>(&square, 1.0, geom_core::Tol::witness());
    let mut body = built.body;
    let face = built.bottom_face;
    let net = swap_first_edge_for_a_chord_net(&mut body, face, (0.0, 1.0));
    let mid = |p: Point3<Interval>| p.map(|x| 0.5 * (x.lo() + x.hi()));
    let (q0, q1) = (mid(net.eval(iv(0.0))), mid(net.eval(iv(1.0))));
    let m = q0.lerp(q1, 0.5);
    let along = (q1 - q0).normalize();
    let out = (m - Point3::new(0.5, 0.5, m.z)).normalize();
    let chart = face_chart(&body, face).map_scalar(|x| 0.5 * (x.lo() + x.hi()));
    let c0 = chart.eval(0.0, 0.0);
    let (eu, ev) = (chart.eval(1.0, 0.0) - c0, chart.eval(0.0, 1.0) - c0);
    let uv = |p: Point3<f64>| ((p - c0).dot(eu) / eu.dot(eu), (p - c0).dot(ev) / ev.dot(ev));
    let corners = [(-0.05, -0.2), (-0.05, 0.2), (1.25, -0.2), (1.25, 0.2)]
        .map(|(o, a)| uv(m + out * o + along * a));
    let span = |pick: fn((f64, f64)) -> f64| {
        let all = corners.map(pick);
        (
            all.iter().copied().fold(f64::INFINITY, f64::min),
            all.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        )
    };
    let (cu, cv) = (span(|c| c.0), span(|c| c.1));
    let got = cell_region(&body, face, cu, cv, band());
    assert!(
        matches!(got, CellRegion::Undecided),
        "the cell reaches 0.05 into the face across the spline edge: {got:?}"
    );
}
