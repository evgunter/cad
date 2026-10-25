//! **A spline edge of a planar loop, read on its own carrier** — the
//! boundary reading and the ray crossing count the in-plane walk
//! ([`super::containment`]) runs beside its conic and spiric rows, both
//! [`super::pieced_arc`]'s readings over the pieces below.
//!
//! **The pieces** are windows of the edge's parameter span, one knot
//! span at most each: the edge's span cut at the knots, then halved.
//! A piece's control net is its Bézier net, read off the spline's own
//! B-spline coefficients by the blossom
//! ([`geom_core::spline::pieces::span_row`], on the weighted points and
//! the weights apart), so a piece is restricted exactly at any scalar
//! and no piece reads a refined knot vector or another piece's net.
//!
//! **The net bounds its piece.** With positive weights the rational
//! Bézier piece lies in the convex hull of its control points `c_k`, so
//! in the ball about their bounding box's centre reaching the farthest
//! of them; and `c_0` is the piece's start point, a point of the arc.
//!
//! **The net crosses a line no fewer times than its piece.** Along a
//! line of unit normal `n` through `q`, the offset of the piece is
//! `s(t) = Σ B_k w_k s_k / Σ B_k w_k` with `s_k = (c_k − q)·n`: its sign
//! is the sign of the polynomial `Σ B_k w_k s_k`, whose roots in the
//! piece number at most the sign changes of `w_k s_k`, and so of `s_k`
//! (Descartes' rule in the Bernstein basis; variation diminishing). So
//! where every `s_k` has one sign the piece misses the line; and where
//! the `s_k` are strictly monotone, `s − c` has at most one sign change
//! for every `c`, so `s` is monotone across the piece and it crosses
//! the line exactly once if its two ends lie on opposite sides, and not
//! at all if on one.

use geom_core::spline::pieces::span_row;
use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};

use crate::splitting::pieced_arc::{PieceBall, PiecedArc, Ray};
use crate::validate::decide;

/// The rows of a ray's crossing of a spline arc — the walk's alone.
const CROSS_SIDE: &str = "point_in_arc_loop_spline_side";
const CROSS_TURN: &str = "point_in_arc_loop_spline_turn";
const CROSS_ADVANCE: &str = "point_in_arc_loop_spline_advance";

/// One knot span's B-spline coefficients: its `p + 1` weighted points
/// `w_i·P_i` and weights, and the index of the first.
struct SpanNet<T: geom_core::Real> {
    first: usize,
    weighted: Vec<Vec3<T>>,
    weights: Vec<T>,
}

/// An arc of a spline.
pub(crate) struct SplineArc<T: geom_core::Real> {
    /// The spline's knot values.
    knots: Vec<T>,
    /// Each knot span the arc runs over, in order, with the part of the
    /// arc's window it holds.
    spans: Vec<(SpanNet<T>, (T, T))>,
    ends: [Point3<T>; 2],
}

/// A window `[a, b]` of one of the arc's spans.
#[derive(Clone, Copy)]
pub(crate) struct Piece<T: geom_core::Real> {
    span: usize,
    a: T,
    b: T,
    depth: u32,
}

impl<T: Decide> SplineArc<T> {
    /// The arc `(t0, t1)` of `curve`, or `None` where a parameter
    /// locates in no span, or the end's span comes before the start's.
    pub(crate) fn new(curve: &geom::NurbsCurve3<T>, (t0, t1): (T, T)) -> Option<Self> {
        let knots = curve.knot_values();
        let p = knots.degree();
        let s0 = knots.locate(t0)?.1.span().index();
        let s1 = knots.locate(t1)?.0.span().index();
        if s1 < s0 {
            return None;
        }
        let values = knots.values();
        let spans = (s0..=s1)
            .filter(|&s| knots.span(s).is_some())
            .map(|s| {
                let first = s - p;
                let window = first..=s;
                let net = SpanNet {
                    first,
                    weighted: curve.control()[window.clone()]
                        .iter()
                        .zip(&curve.weights()[window.clone()])
                        .map(|(c, &w)| (*c - Point3::origin()) * w)
                        .collect(),
                    weights: curve.weights()[window].to_vec(),
                };
                let lo = if s == s0 { t0 } else { values[s] };
                let hi = if s == s1 { t1 } else { values[s + 1] };
                (net, (lo, hi))
            })
            .collect();
        Some(Self {
            knots: values.to_vec(),
            spans,
            ends: [curve.eval(t0), curve.eval(t1)],
        })
    }

    /// A piece's Bézier control points: `c_k = (Σ w P)_k / (Σ w)_k`.
    fn net(&self, p: &Piece<T>) -> Vec<Point3<T>> {
        let (span, _) = &self.spans[p.span];
        let window = (p.a, p.b);
        let (Some(weighted), Some(weights)) = (
            span_row(&span.weighted, &self.knots, span.first, window),
            span_row(&span.weights, &self.knots, span.first, window),
        ) else {
            unreachable!("a span's net reads knots of its own vector");
        };
        weighted
            .into_iter()
            .zip(weights)
            .map(|(v, w)| Point3::origin() + v * (T::one() / w))
            .collect()
    }
}

impl<T: Decide> PiecedArc<T> for SplineArc<T> {
    type Piece = Piece<T>;

    fn ends(&self) -> [Point3<T>; 2] {
        self.ends
    }

    /// One piece per knot span the arc runs over.
    fn roots(&self) -> Vec<Piece<T>> {
        self.spans
            .iter()
            .enumerate()
            .map(|(span, &(_, (a, b)))| Piece {
                span,
                a,
                b,
                depth: 0,
            })
            .collect()
    }

    fn depth(piece: &Piece<T>) -> u32 {
        piece.depth
    }

    fn halves(&self, p: Piece<T>) -> [Piece<T>; 2] {
        let m = geom::mid_param(p.a, p.b);
        let depth = p.depth + 1;
        [Piece { b: m, depth, ..p }, Piece { a: m, depth, ..p }]
    }

    /// The ball about the net's bounding box's centre, reaching the
    /// farthest control point, and the piece's start point `c_0`.
    fn ball(&self, p: &Piece<T>) -> PieceBall<T> {
        let net = self.net(p);
        let (lo, hi) = net
            .iter()
            .fold((net[0], net[0]), |(lo, hi), &c| (lo.min(c), hi.max(c)));
        let center = lo + (hi - lo) * T::from_f64(0.5);
        let reach = net
            .iter()
            .fold(T::zero(), |r, c| r.max((*c - center).norm()));
        PieceBall {
            center,
            reach,
            on: net[0],
        }
    }

    /// Settled when every control point lies definitely on one side of
    /// the ray line, or definitely behind `q` along `d`: no crossing.
    /// Or when the control points' offsets `s_k` from the ray line are
    /// definitely strictly monotone with both ends' definitely signed
    /// (the module docs): same signs, no crossing; opposite, exactly
    /// one, counted when every control point lies definitely ahead of
    /// `q` along `d`.
    fn cross(&self, p: &Piece<T>, Ray { q, d, side }: Ray<T>, band: Band) -> Option<usize> {
        let net = self.net(p);
        let offsets: Vec<T> = net.iter().map(|c| (*c - q).dot(side)).collect();
        let along = || net.iter().map(|c| (*c - q).dot(d));
        let offset = || offsets.iter().copied();
        if all(CROSS_SIDE, offset(), Sign::Positive, band)
            || all(CROSS_SIDE, offset(), Sign::Negative, band)
            || all(CROSS_ADVANCE, along(), Sign::Negative, band)
        {
            return Some(0);
        }
        let ends = [offsets[0], offsets[offsets.len() - 1]]
            .map(|s| decide(CROSS_SIDE, Margin::of(s), band));
        let [
            Ok(a @ (Sign::Positive | Sign::Negative)),
            Ok(z @ (Sign::Positive | Sign::Negative)),
        ] = ends
        else {
            return None;
        };
        let steps = || offsets.windows(2).map(|w| w[1] - w[0]);
        let monotone = all(CROSS_TURN, steps(), Sign::Positive, band)
            || all(CROSS_TURN, steps(), Sign::Negative, band);
        if !monotone {
            return None;
        }
        if a == z {
            return Some(0);
        }
        all(CROSS_ADVANCE, along(), Sign::Positive, band).then_some(1)
    }
}

/// Whether every margin of `xs` decides `sign` on `row`, read in order
/// to the first that does not.
fn all<T: Decide>(
    row: &'static str,
    xs: impl IntoIterator<Item = T>,
    sign: Sign,
    band: Band,
) -> bool {
    xs.into_iter()
        .all(|x| decide(row, Margin::of(x), band) == Ok(sign))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::splitting::pieced_arc::{ArcHit, ArcRows};
    use geom_core::{Interval, Tol};

    const ROWS: ArcRows = ArcRows {
        end: "test_spline_end",
        clear: "test_spline_clear",
        on: "test_spline_on",
        leaf: "test_spline_leaf",
    };

    /// The plane the curve lies in, `z = 0.7`.
    const Z: f64 = 0.7;

    /// A rational cubic over three knot spans, `t ∈ [0, 1]` with interior
    /// knots `0.4` and `0.7`, in the plane `z = 0.7`: two bumps over the
    /// `x` axis, peaking near `y = 1.89` and `2.4` with a dip to `1.52`
    /// between them, above `y = 0` but at its ends; `x` grows along it.
    fn wave() -> geom::NurbsCurve3<f64> {
        let p = |x, y| Point3::new(x, y, Z);
        geom::NurbsCurve3::new(
            geom_core::spline::KnotVector::clamped(
                vec![0.0, 0.0, 0.0, 0.0, 0.4, 0.7, 1.0, 1.0, 1.0, 1.0],
                3,
            )
            .unwrap(),
            vec![
                p(0.0, 0.0),
                p(1.0, 2.5),
                p(2.0, -0.6),
                p(3.0, 3.0),
                p(4.0, 1.5),
                p(5.0, 0.0),
            ],
            vec![1.0, 2.0, 0.7, 1.5, 1.0, 1.0],
        )
        .unwrap()
    }

    fn arc<T: Decide>((t0, t1): (f64, f64)) -> SplineArc<T> {
        let f = T::from_f64;
        SplineArc::new(&wave().map_scalar(f), (f(t0), f(t1))).expect("a forward span")
    }

    fn at(t: f64) -> (f64, f64) {
        let p = wave().eval(t);
        (p.x, p.y)
    }

    /// **The arc closed by its chord, read by the row, against the
    /// region sampled densely** — the whole arc, and a window of it that
    /// starts and ends inside spans and runs across both knots. Every
    /// grid point further than `1e-3` from the sampled boundary is
    /// definitely off the arc, and its ray along a fixed generic
    /// direction crosses the arc a certified number of times whose
    /// parity with the chord's is the sampled region. Most of the grid
    /// lies inside the ball holding the control net. On the arc it is
    /// on; at an end, the end; between the band's two thresholds off the
    /// arc, it refuses.
    fn the_row_reads_the_arcs_region<T: Decide>(lane: &str) {
        let band = Band::linear(Tol::witness()).unwrap();
        let f = T::from_f64;
        for (t0, t1) in [(0.0, 1.0), (0.1, 0.9)] {
            let k = arc::<T>((t0, t1));
            let n = 20_000;
            let mut boundary: Vec<(f64, f64)> = (0..=n)
                .map(|i| at(t0 + (t1 - t0) * f64::from(i) / f64::from(n)))
                .collect();
            boundary.push(at(t0));
            let truth = |(x, y): (f64, f64)| -> (bool, f64) {
                let mut inside = false;
                let mut gap = f64::INFINITY;
                for w in boundary.windows(2) {
                    let ((ax, ay), (bx, by)) = (w[0], w[1]);
                    if (ay > y) != (by > y) && x < ax + (y - ay) / (by - ay) * (bx - ax) {
                        inside = !inside;
                    }
                    let (dx, dy) = (bx - ax, by - ay);
                    let s = (((x - ax) * dx + (y - ay) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
                    gap = gap.min((x - ax - s * dx).hypot(y - ay - s * dy));
                }
                (inside, gap)
            };
            let (a, b) = (at(t0), at(t1));
            let angle = 0.7_f64;
            let (dx, dy) = (angle.cos(), angle.sin());
            let point = |(x, y): (f64, f64)| Point3::new(f(x), f(y), f(Z));
            let d = Vec3::new(f(dx), f(dy), f(0.0));
            let side = Vec3::new(f(0.0), f(0.0), f(1.0)).cross(d);
            let (mut asked, mut inside) = (0, 0);
            for i in 0..=30 {
                for j in 0..=24 {
                    let q = (
                        -0.5 + 6.0 * f64::from(i) / 30.0,
                        -0.5 + 3.0 * f64::from(j) / 24.0,
                    );
                    let (want, gap) = truth(q);
                    if gap < 1e-3 {
                        continue;
                    }
                    asked += 1;
                    inside += usize::from(want);
                    assert_eq!(
                        k.contact(point(q), ROWS, band),
                        Ok(ArcHit::Off),
                        "{lane} {t0}–{t1} {q:?}: off the arc"
                    );
                    // The chord's crossing, in closed form.
                    let cross = |(px, py): (f64, f64)| (px - q.0) * dy - (py - q.1) * dx;
                    let chord = if (cross(a) > 0.0) == (cross(b) > 0.0) {
                        0
                    } else {
                        let s = cross(a) / (cross(a) - cross(b));
                        let (hx, hy) = (a.0 + s * (b.0 - a.0), a.1 + s * (b.1 - a.1));
                        usize::from((hx - q.0) * dx + (hy - q.1) * dy > 0.0)
                    };
                    let Some(count) = k.crossings(point(q), d, side, band) else {
                        panic!("{lane} {t0}–{t1} {q:?}: the ray was abandoned");
                    };
                    assert_eq!(
                        (count + chord) % 2 == 1,
                        want,
                        "{lane} {t0}–{t1} {q:?}: {count} arc crossings"
                    );
                }
            }
            assert!(
                asked >= 500 && inside >= 40 && asked - inside >= 300,
                "{lane} {t0}–{t1}: not vacuous ({asked} asked, {inside} inside)"
            );
            let mid = point(at(0.5 * (t0 + t1) + 0.013));
            assert!(
                matches!(k.contact(mid, ROWS, band), Ok(ArcHit::On(_))),
                "{lane} {t0}–{t1}: on the arc"
            );
            assert_eq!(
                k.contact(point(b), ROWS, band),
                Ok(ArcHit::End),
                "{lane} {t0}–{t1}: at its end"
            );
            // Off a point of the arc along its normal, in the band.
            let t = 0.5 * (t0 + t1);
            let (on, tangent) = (at(t), wave().deriv(t).normalize());
            let off = 0.5 * (band.zero() + band.escalate());
            let in_band = (on.0 - tangent.y * off, on.1 + tangent.x * off);
            assert!(
                k.contact(point(in_band), ROWS, band).is_err(),
                "{lane} {t0}–{t1}: in the band of the arc, the reading refuses"
            );
        }
    }

    #[test]
    fn the_row_reads_the_arcs_region_at_f64() {
        the_row_reads_the_arcs_region::<f64>("f64");
    }

    #[test]
    fn the_row_reads_the_arcs_region_at_interval() {
        the_row_reads_the_arcs_region::<Interval>("Interval");
    }

    /// **The count is the number of crossings, not just its parity.** A
    /// ray along `+x` at height `h` from left of the wave crosses it as
    /// often as `y(t) − h` changes sign along it, sampled: four times
    /// through both bumps, twice through the taller one alone, never
    /// above both; from under the dip, once.
    fn the_count_is_exact<T: Decide>(lane: &str) {
        let band = Band::linear(Tol::witness()).unwrap();
        let f = T::from_f64;
        let k = arc::<T>((0.0, 1.0));
        let d = Vec3::new(f(1.0), f(0.0), f(0.0));
        let side = Vec3::new(f(0.0), f(0.0), f(1.0)).cross(d);
        let samples: Vec<(f64, f64)> = (0..=20_000).map(|i| at(f64::from(i) / 2e4)).collect();
        let mut seen = Vec::new();
        for (x, h) in [
            (-1.0, 1.7),
            (-1.0, 2.0),
            (-1.0, 3.0),
            (2.0, 1.0),
            (6.0, 1.0),
        ] {
            let want = samples
                .windows(2)
                .filter(|w| (w[0].1 > h) != (w[1].1 > h) && w[1].0 > x)
                .count();
            let q = Point3::new(f(x), f(h), f(Z));
            assert_eq!(
                k.crossings(q, d, side, band),
                Some(want),
                "{lane} from ({x}, {h})"
            );
            seen.push(want);
        }
        assert_eq!(seen, [4, 2, 0, 1, 0], "{lane}: the rows reach every count");
    }

    #[test]
    fn the_count_is_exact_at_f64() {
        the_count_is_exact::<f64>("f64");
    }

    #[test]
    fn the_count_is_exact_at_interval() {
        the_count_is_exact::<Interval>("Interval");
    }
    fn review_knot_windows<T: Decide>(lane: &str) {
        let band = Band::linear(Tol::witness()).unwrap();
        let f = T::from_f64;
        for (t0, t1) in [(0.0, 0.4), (0.4, 0.7), (0.0, 0.7), (0.4, 1.0), (0.7, 1.0)] {
            let k = arc::<T>((t0, t1));
            let n = 20_000;
            let mut boundary: Vec<(f64, f64)> = (0..=n)
                .map(|i| at(t0 + (t1 - t0) * f64::from(i) / f64::from(n)))
                .collect();
            boundary.push(at(t0));
            let truth = |(x, y): (f64, f64)| -> (bool, f64) {
                let mut inside = false;
                let mut gap = f64::INFINITY;
                for w in boundary.windows(2) {
                    let ((ax, ay), (bx, by)) = (w[0], w[1]);
                    if (ay > y) != (by > y) && x < ax + (y - ay) / (by - ay) * (bx - ax) {
                        inside = !inside;
                    }
                    let (dx, dy) = (bx - ax, by - ay);
                    let s = (((x - ax) * dx + (y - ay) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
                    gap = gap.min((x - ax - s * dx).hypot(y - ay - s * dy));
                }
                (inside, gap)
            };
            let (a, b) = (at(t0), at(t1));
            let angle = 0.7_f64;
            let (dx, dy) = (angle.cos(), angle.sin());
            let point = |(x, y): (f64, f64)| Point3::new(f(x), f(y), f(Z));
            let d = Vec3::new(f(dx), f(dy), f(0.0));
            let side = Vec3::new(f(0.0), f(0.0), f(1.0)).cross(d);
            let (mut asked, mut inside) = (0, 0);
            for i in 0..=30 {
                for j in 0..=24 {
                    let q = (
                        -0.5 + 6.0 * f64::from(i) / 30.0,
                        -0.5 + 3.0 * f64::from(j) / 24.0,
                    );
                    let (want, gap) = truth(q);
                    if gap < 1e-3 {
                        continue;
                    }
                    asked += 1;
                    inside += usize::from(want);
                    assert_eq!(
                        k.contact(point(q), ROWS, band),
                        Ok(ArcHit::Off),
                        "{lane} {t0}–{t1} {q:?}: off the arc"
                    );
                    // The chord's crossing, in closed form.
                    let cross = |(px, py): (f64, f64)| (px - q.0) * dy - (py - q.1) * dx;
                    let chord = if (cross(a) > 0.0) == (cross(b) > 0.0) {
                        0
                    } else {
                        let s = cross(a) / (cross(a) - cross(b));
                        let (hx, hy) = (a.0 + s * (b.0 - a.0), a.1 + s * (b.1 - a.1));
                        usize::from((hx - q.0) * dx + (hy - q.1) * dy > 0.0)
                    };
                    let Some(count) = k.crossings(point(q), d, side, band) else {
                        panic!("{lane} {t0}–{t1} {q:?}: the ray was abandoned");
                    };
                    assert_eq!(
                        (count + chord) % 2 == 1,
                        want,
                        "{lane} {t0}–{t1} {q:?}: {count} arc crossings"
                    );
                }
            }
            assert!(
                asked >= 500,
                "{lane} {t0}–{t1}: not vacuous ({asked} asked, {inside} inside)"
            );
            let mid = point(at(0.5 * (t0 + t1) + 0.013));
            assert!(
                matches!(k.contact(mid, ROWS, band), Ok(ArcHit::On(_))),
                "{lane} {t0}–{t1}: on the arc"
            );
            assert_eq!(
                k.contact(point(b), ROWS, band),
                Ok(ArcHit::End),
                "{lane} {t0}–{t1}: at its end"
            );
            // Off a point of the arc along its normal, in the band.
            let t = 0.5 * (t0 + t1);
            let (on, tangent) = (at(t), wave().deriv(t).normalize());
            let off = 0.5 * (band.zero() + band.escalate());
            let in_band = (on.0 - tangent.y * off, on.1 + tangent.x * off);
            assert!(
                k.contact(point(in_band), ROWS, band).is_err(),
                "{lane} {t0}–{t1}: in the band of the arc, the reading refuses"
            );
        }
    }

    #[test]
    fn review_knot_windows_f64() {
        review_knot_windows::<f64>("f64");
    }
    #[test]
    fn review_knot_windows_interval() {
        review_knot_windows::<Interval>("Interval");
    }

    fn review_on_curve<T: Decide>(lane: &str) {
        let band = Band::linear(Tol::witness()).unwrap();
        let f = T::from_f64;
        let k = arc::<T>((0.0, 1.0));
        let (mut on, mut esc, mut other) = (0, 0, Vec::new());
        for i in 1..2000 {
            let t = f64::from(i) / 2000.0 + 1.234e-5;
            let (x, y) = at(t);
            match k.contact(Point3::new(f(x), f(y), f(Z)), ROWS, band) {
                Ok(ArcHit::On(_)) | Ok(ArcHit::End) => on += 1,
                Err(crate::splitting::pieced_arc::ArcRefusal::Escalated(_)) => esc += 1,
                o => other.push(format!("{t}: {o:?}")),
            }
        }
        println!("{lane}: on {on} escalated {esc} other {}", other.len());
        for o in other.iter().take(5) {
            println!("  {o}");
        }
        assert!(
            esc == 0 && other.is_empty(),
            "{lane}: on {on} escalated {esc} other {other:?}"
        );
    }
    #[test]
    fn review_on_curve_f64() {
        review_on_curve::<f64>("f64");
    }
    #[test]
    fn review_on_curve_interval() {
        review_on_curve::<Interval>("Interval");
    }

    fn review_fuzz<T: Decide>(lane: &str) {
        let band = Band::linear(Tol::witness()).unwrap();
        let f = T::from_f64;
        let mut seed: u64 = 0x2545F4914F6CDD1D;
        let mut rnd = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed >> 11) as f64 / (1u64 << 53) as f64
        };
        let (mut asked, mut abandoned, mut wrong, mut refused) = (0, 0, Vec::new(), 0);
        for case in 0..150 {
            let p = 1 + (rnd() * 4.0) as usize;
            let spans = 1 + (rnd() * 4.0) as usize;
            let n = p + spans;
            let mut knots = vec![0.0; p + 1];
            let mut inner: Vec<f64> = (1..spans).map(|_| 0.05 + 0.9 * rnd()).collect();
            inner.sort_by(|a, b| a.partial_cmp(b).unwrap());
            if case % 3 == 0 && !inner.is_empty() {
                let v = inner[0];
                inner.push(v);
                inner.sort_by(|a, b| a.partial_cmp(b).unwrap());
            }
            knots.extend(inner.iter().copied());
            knots.extend(vec![1.0; p + 1]);
            let ncp = knots.len() - p - 1;
            let _ = n;
            let ctrl: Vec<Point3<f64>> = (0..ncp)
                .map(|_| Point3::new(4.0 * rnd() - 2.0, 4.0 * rnd() - 2.0, Z))
                .collect();
            let w: Vec<f64> = (0..ncp)
                .map(|_| {
                    if case % 2 == 0 {
                        1.0
                    } else {
                        0.1 + 6.0 * rnd() * rnd()
                    }
                })
                .collect();
            let Ok(kv) = geom_core::spline::KnotVector::clamped(knots.clone(), p) else {
                continue;
            };
            let Ok(c) = geom::NurbsCurve3::new(kv, ctrl, w) else {
                continue;
            };
            let t0 = if case % 4 == 0 { 0.0 } else { 0.3 * rnd() };
            let t1 = if case % 5 == 0 && !inner.is_empty() {
                inner[inner.len() - 1]
            } else {
                1.0 - 0.3 * rnd()
            };
            let Some(k) = SplineArc::new(&c.map_scalar(f), (f(t0), f(t1))) else {
                panic!("{lane} case {case}: new refused")
            };
            let m = 40_000;
            let pts: Vec<(f64, f64)> = (0..=m)
                .map(|i| {
                    let q = c.eval(t0 + (t1 - t0) * f64::from(i) / f64::from(m));
                    (q.x, q.y)
                })
                .collect();
            for _ in 0..40 {
                let q = (5.0 * rnd() - 2.5, 5.0 * rnd() - 2.5);
                let a = 6.283185 * rnd();
                let (dx, dy) = (a.cos(), a.sin());
                let s = |(x, y): (f64, f64)| (x - q.0) * dy * -1.0 + (y - q.1) * dx;
                let g = |(x, y): (f64, f64)| (x - q.0) * dx + (y - q.1) * dy;
                let gap = pts
                    .windows(2)
                    .map(|w| {
                        let (ax, ay) = w[0];
                        let (bx, by) = w[1];
                        let (ex, ey) = (bx - ax, by - ay);
                        let l = ex * ex + ey * ey;
                        let tt = if l > 0.0 {
                            (((q.0 - ax) * ex + (q.1 - ay) * ey) / l).clamp(0.0, 1.0)
                        } else {
                            0.0
                        };
                        (q.0 - ax - tt * ex).hypot(q.1 - ay - tt * ey)
                    })
                    .fold(f64::INFINITY, f64::min);
                if gap < 1e-3 {
                    continue;
                }
                // minimum |s| at a near-line sample ahead, to skip near-grazes
                let sv: Vec<f64> = pts.iter().map(|&p| s(p)).collect();
                let fragile = (1..sv.len() - 1).any(|i| {
                    (sv[i] - sv[i - 1]) * (sv[i + 1] - sv[i]) <= 0.0
                        && sv[i].abs() < 1e-3
                        && g(pts[i]) > -1e-3
                }) || pts
                    .windows(2)
                    .any(|w| (s(w[0]) > 0.0) != (s(w[1]) > 0.0) && g(w[0]).abs() < 1e-3)
                    || sv[0].abs() < 1e-3
                    || sv[sv.len() - 1].abs() < 1e-3;
                let near = if fragile { 0.0 } else { 1.0 };
                let want = pts
                    .windows(2)
                    .filter(|w| (s(w[0]) > 0.0) != (s(w[1]) > 0.0) && g(w[0]) > 0.0)
                    .count();
                let qq = Point3::new(f(q.0), f(q.1), f(Z));
                match k.contact(qq, ROWS, band) {
                    Ok(ArcHit::Off) => {}
                    _ => {
                        refused += 1;
                        continue;
                    }
                }
                let d = Vec3::new(f(dx), f(dy), f(0.0));
                let side = Vec3::new(f(0.0), f(0.0), f(1.0)).cross(d);
                asked += 1;
                match k.crossings(qq, d, side, band) {
                    None => abandoned += 1,
                    Some(got) if got != want && near > 1e-4 => wrong.push(format!("case {case} p{p} {knots:?} t {t0}..{t1} q {q:?} a {a}: got {got} want {want} near {near}")),
                    Some(_) => {}
                }
            }
        }
        println!(
            "{lane}: asked {asked} abandoned {abandoned} refused {refused} wrong {}",
            wrong.len()
        );
        for w in wrong.iter().take(8) {
            println!("  {w}");
        }
        assert!(wrong.is_empty(), "{lane}: {} wrong", wrong.len());
    }
    #[test]
    fn review_fuzz_f64() {
        review_fuzz::<f64>("f64");
    }
    #[test]
    fn review_fuzz_interval() {
        review_fuzz::<Interval>("Interval");
    }

    fn review_near<T: Decide>(lane: &str) {
        let band = Band::linear(Tol::witness()).unwrap();
        let f = T::from_f64;
        let k = arc::<T>((0.0, 1.0));
        let mut bad = Vec::new();
        let mut tally = std::collections::BTreeMap::new();
        for i in 1..1500 {
            let t = f64::from(i) / 1500.0 + 3.1e-5;
            let (x, y) = at(t);
            let nrm = wave().deriv(t).normalize();
            for (j, frac) in [
                0.2,
                0.9,
                1.5,
                0.5 * (band.zero() + band.escalate()) / band.zero(),
                0.95 * band.escalate() / band.zero(),
            ]
            .iter()
            .enumerate()
            {
                for sgn in [-1.0, 1.0] {
                    let dl = sgn * frac * band.zero();
                    let q = Point3::new(f(x - nrm.y * dl), f(y + nrm.x * dl), f(Z));
                    let got = k.contact(q, ROWS, band);
                    let key = format!(
                        "{j} {}",
                        match got {
                            Ok(ArcHit::Off) => "Off",
                            Ok(ArcHit::On(_)) => "On",
                            Ok(ArcHit::End) => "End",
                            Err(_) => "Err",
                        }
                    );
                    *tally.entry(key).or_insert(0) += 1;
                    if got == Ok(ArcHit::Off) {
                        bad.push(format!("t {t} dl {dl}"));
                    }
                }
            }
        }
        println!("{lane}: {tally:?} bad {}", bad.len());
        assert!(bad.is_empty(), "{lane}: {:?}", &bad[..bad.len().min(5)]);
    }
    #[test]
    fn review_near_f64() {
        review_near::<f64>("f64");
    }
    #[test]
    fn review_near_interval() {
        review_near::<Interval>("Interval");
    }
}
