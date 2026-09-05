//! **The chart-boundary description and its outside test** — a face's
//! trimmed region described in a chart the CONSUMER names, and the
//! certified predicate *"this metred rectangle holds no point of the
//! face"*.
//!
//! [`ChartBound`] is the description: one closed chord polygon per
//! loop (outer first, then rings, D9 order), each edge either a
//! [`ChartEdge::Segment`] — the image is a straight chart segment BY
//! STRUCTURE — or a [`ChartEdge::Envelope`], which carries a certified
//! box around the image instead. It is minted by
//! [`crate::pcurves::chart_boundary`], which walks the loops; this
//! module owns the value, its metring and the test.
//!
//! # The certified statement
//!
//! For a loop `L` let `P_L` be its closed chord polygon and `E_L` the
//! union of its envelope boxes. The face's trimmed region, in this
//! chart and on the walk's branch, lies in
//! `(closure(int P_outer) ∪ E_outer)` minus
//! `⋃_rings (int P_ring \ E_ring)`. The argument is that the true
//! boundary differs from `P_L` only inside `E_L`: a lune between an
//! arc and its chord is bounded by a closed curve lying in the arc's
//! convex box, hence lies in that box.
//!
//! # Direction of every rounding
//!
//! [`MetredBound::certifies_outside`] is the only consumer of that
//! statement, and **every rounding keeps the cell**. That invariant is
//! stated at the method and is what makes the description usable by a
//! subdivision driver whose looseness must run one way: a cell is
//! dropped only on a definite verdict at `≥ K·ε`, so `Sign::Zero`, an
//! in-band margin, poison and a ring copy that was never emitted all
//! read as "not certified".

use geom_brep::ChartWindow;
use geom_core::{Band, Decide, Margin, Point2, Real, Sign, Vec2};

use crate::ray_parity::{self, ParityRows};
use crate::validate::decide;

/// The K rows the outside test meters through. Distinct from
/// `chart_region.rs`'s and from `containment.rs`'s: a chart-boundary
/// cell margin is its own population (`ray_parity`'s module docs), and
/// this value is a roster entry in `docs/K-REPORT.md`.
const ROWS: ParityRows = ParityRows {
    segment: "chart_bound_segment",
    boundary: "chart_bound_boundary",
    side: "chart_bound_side",
    advance: "chart_bound_advance",
};

/// The separating-axis row: every one of the five candidate axes per
/// edge decides under this one name, because they are one question
/// (metres of clear space between a cell and a boundary edge) asked
/// five ways.
const GAP: &str = "chart_bound_gap";

/// One boundary edge's chart image, in loop direction (entry `a` →
/// exit `b`).
#[derive(Clone, Debug)]
pub enum ChartEdge<T: Real> {
    /// The image is the straight chart segment `a → b` BY STRUCTURE —
    /// an `IsoLine`, an `IsoArc` (whose UV image is the segment
    /// `p0 → p0 + pd`), or a `Harmonic` whose four trigonometric
    /// channels are exact-structural zeros. No measurement decides
    /// this: a numerically-almost-zero channel is an `Envelope`,
    /// deliberately.
    Segment {
        /// The entry vertex.
        a: Point2<T>,
        /// The exit vertex.
        b: Point2<T>,
    },
    /// Anything else: the chord `a → b` plus a certified box around
    /// the true image.
    Envelope {
        /// The entry vertex.
        a: Point2<T>,
        /// The exit vertex.
        b: Point2<T>,
        /// The image over the WHOLE span as ONE enclosure — each
        /// coordinate is the bracket, so at the interval scalar this
        /// pair IS the box. At a point scalar the span hull is poison
        /// and the box is poison, which certifies nothing.
        image: Point2<T>,
        /// Metres of residual the stored certificate allows between
        /// the carrier and this image; [`ChartBound::metred`] widens
        /// the box by it. Zero for a closed-form image, which is
        /// exact in its family.
        slack: T,
    },
}

impl<T: Real> ChartEdge<T> {
    /// The entry vertex — the polygon vertex this edge contributes.
    pub fn a(&self) -> Point2<T> {
        match *self {
            ChartEdge::Segment { a, .. } | ChartEdge::Envelope { a, .. } => a,
        }
    }

    /// The exit vertex.
    pub fn b(&self) -> Point2<T> {
        match *self {
            ChartEdge::Segment { b, .. } | ChartEdge::Envelope { b, .. } => b,
        }
    }

    /// The chart box this edge's image is known to lie in: the chord's
    /// own box, hulled with the envelope box where there is one.
    fn window(&self) -> ChartWindow<T> {
        let (a, b) = (self.a(), self.b());
        let chord = ChartWindow {
            u_min: a.x.min(b.x),
            u_max: a.x.max(b.x),
            v_min: a.y.min(b.y),
            v_max: a.y.max(b.y),
        };
        match *self {
            ChartEdge::Segment { .. } => chord,
            ChartEdge::Envelope { image, .. } => chord.hull(ChartWindow {
                u_min: image.x,
                u_max: image.x,
                v_min: image.y,
                v_max: image.y,
            }),
        }
    }

    /// This edge shifted by `du` along the chart's first parameter —
    /// the whole-period lift a ring copy is.
    fn shifted(&self, du: T) -> Self {
        let sh = |p: Point2<T>| Point2::new(p.x + du, p.y);
        match *self {
            ChartEdge::Segment { a, b } => ChartEdge::Segment { a: sh(a), b: sh(b) },
            ChartEdge::Envelope { a, b, image, slack } => ChartEdge::Envelope {
                a: sh(a),
                b: sh(b),
                image: sh(image),
                slack,
            },
        }
    }
}

/// One loop's closed chord polygon.
#[derive(Clone, Debug)]
pub struct ChartLoop<T: Real> {
    /// The edges in loop direction; the polygon's vertices are their
    /// entry points.
    pub edges: Vec<ChartEdge<T>>,
    /// Is this a ring (a hole) rather than the outer loop? Ring copies
    /// of one ring all carry `true`.
    pub ring: bool,
}

impl<T: Real> ChartLoop<T> {
    /// The loop's own chart box, over its vertices and envelope boxes.
    fn window(&self) -> Option<ChartWindow<T>> {
        self.edges
            .iter()
            .map(ChartEdge::window)
            .reduce(ChartWindow::hull)
    }

    fn shifted(&self, du: T) -> Self {
        Self {
            edges: self.edges.iter().map(|e| e.shifted(du)).collect(),
            ring: self.ring,
        }
    }
}

/// A face's boundary in one chart, on the loop walk's own branch.
///
/// The branch is a real number, never folded into `[0, τ)`: an
/// extruded arc's face is `[0, θ]` and a negative-angle revolve's band
/// is `[θ, 0]`, and the description keeps whichever the walk pinned.
#[derive(Clone, Debug)]
pub struct ChartBound<T: Real> {
    /// The outer loop first, then every ring copy (D9 order).
    pub loops: Vec<ChartLoop<T>>,
    /// The OUTER loop's box, over its vertices and envelope boxes.
    /// Rings lie in it by construction.
    pub hull: ChartWindow<T>,
    /// The chart's first-parameter period, decided by surface KIND:
    /// `τ` on the azimuth charts, the knot-domain length on a spline
    /// chart that closes, `None` on a plane.
    pub period: Option<T>,
}

/// The whole-period shifts a ring is lifted by on a periodic chart.
///
/// Every shift is emitted, with no test that the lift lands near the
/// outer loop. A lift is a GENUINE representation of the same ring —
/// on a chart of period `p` the points `(u, v)` and `(u + kp, v)` are
/// one point of the model — so emitting one that turns out to be far
/// away costs a few edges in the separating-axis stage and changes no
/// verdict, while the test that would prune it is a comparison of
/// brackets, which `geom_core::Bounds`' scope rule keeps out of code
/// that also decides.
const RING_SHIFTS: [f64; 3] = [-1.0, 0.0, 1.0];

impl<T: Real> ChartBound<T> {
    /// Assembles the description from a walked outer loop and its
    /// walked rings, lifting each ring by every whole-period shift in
    /// [`RING_SHIFTS`] on a periodic chart.
    ///
    /// Each copy is a genuine lift of the same ring, so parity over
    /// all copies is exact.
    pub fn assembled(outer: ChartLoop<T>, rings: Vec<ChartLoop<T>>, period: Option<T>) -> Self {
        let hull = outer.window().unwrap_or(ChartWindow {
            // An outer loop with no edges bounds nothing; the inverted
            // box contains nothing and meets no ring.
            u_min: T::one(),
            u_max: -T::one(),
            v_min: T::one(),
            v_max: -T::one(),
        });
        let mut loops = vec![outer];
        for ring in rings {
            match period {
                None => loops.push(ring),
                Some(p) => {
                    for k in RING_SHIFTS.iter().copied() {
                        loops.push(ring.shifted(p * T::from_f64(k)));
                    }
                }
            }
        }
        Self {
            loops,
            hull,
            period,
        }
    }
}

impl<T: Real> ChartBound<T> {
    /// The description with every chart quantity scaled to **metres**
    /// by the chart's arms `(a_u, a_v)` — metres per chart unit — and
    /// every envelope box widened by its own `slack`.
    ///
    /// The consumer meters once per window and passes EXACT arms
    /// (plane `(1, 1)`, cylinder `(r, 1)`); every margin the test then
    /// decides is a length, which is what the linear band governs.
    pub fn metred(&self, arms: (T, T)) -> MetredBound<T> {
        let (au, av) = arms;
        let scale = |p: Point2<T>| Point2::new(p.x * au, p.y * av);
        MetredBound {
            loops: self
                .loops
                .iter()
                .map(|lp| MetredLoop {
                    ring: lp.ring,
                    edges: lp
                        .edges
                        .iter()
                        .map(|e| match *e {
                            ChartEdge::Segment { a, b } => MetredEdge::Segment {
                                a: scale(a),
                                b: scale(b),
                            },
                            ChartEdge::Envelope { a, b, image, slack } => {
                                let m = scale(image);
                                let (ma, mb) = (scale(a), scale(b));
                                // The chord's own box is folded in, so
                                // the single box test of stage (1)
                                // answers for the whole edge without
                                // relying on the image enclosure to
                                // contain its endpoints.
                                MetredEdge::Envelope {
                                    a: ma,
                                    image: ChartWindow {
                                        u_min: (m.x - slack).min(ma.x).min(mb.x),
                                        u_max: (m.x + slack).max(ma.x).max(mb.x),
                                        v_min: (m.y - slack).min(ma.y).min(mb.y),
                                        v_max: (m.y + slack).max(ma.y).max(mb.y),
                                    },
                                }
                            }
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}

/// One metred boundary edge — every coordinate in metres.
#[derive(Clone, Debug)]
enum MetredEdge<T: Real> {
    Segment {
        a: Point2<T>,
        b: Point2<T>,
    },
    /// The chord and the certified image together, as ONE box: the
    /// chord lies in it by construction (`metred` folds it in), so the
    /// box test of stage (1) answers for the whole edge.
    Envelope {
        a: Point2<T>,
        image: ChartWindow<T>,
    },
}

impl<T: Real> MetredEdge<T> {
    fn a(&self) -> Point2<T> {
        match *self {
            MetredEdge::Segment { a, .. } | MetredEdge::Envelope { a, .. } => a,
        }
    }
}

#[derive(Clone, Debug)]
struct MetredLoop<T: Real> {
    edges: Vec<MetredEdge<T>>,
    ring: bool,
}

/// A [`ChartBound`] scaled to metres — the value the outside test
/// decides against.
#[derive(Clone, Debug)]
pub struct MetredBound<T: Real> {
    loops: Vec<MetredLoop<T>>,
}

/// A metred chart rectangle: the cell the consumer is asking about.
/// Its four bounds are exact `f64` structure (a subdivision cell's
/// corners are the driver's own arithmetic), and a POINT is this with
/// zero extent.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MetredRect {
    /// Lower first-parameter bound, metres.
    pub u_min: f64,
    /// Upper first-parameter bound, metres.
    pub u_max: f64,
    /// Lower second-parameter bound, metres.
    pub v_min: f64,
    /// Upper second-parameter bound, metres.
    pub v_max: f64,
}

impl MetredRect {
    /// The rectangle `[u0, u1] × [v0, v1]`.
    pub fn new(u0: f64, u1: f64, v0: f64, v1: f64) -> Self {
        Self {
            u_min: u0,
            u_max: u1,
            v_min: v0,
            v_max: v1,
        }
    }

    /// A single metred point, as the zero-extent rectangle.
    pub fn point(u: f64, v: f64) -> Self {
        Self::new(u, u, v, v)
    }

    /// The four corners, in a fixed order (D9).
    fn corners(self) -> [(f64, f64); 4] {
        [
            (self.u_min, self.v_min),
            (self.u_max, self.v_min),
            (self.u_min, self.v_max),
            (self.u_max, self.v_max),
        ]
    }
}

impl<T: Decide> MetredBound<T> {
    /// **Does the boundary certify that `rect` holds no point of the
    /// face?**
    ///
    /// INVARIANT — *every rounding keeps the cell*. `true` is a
    /// certificate; `false` is "not certified", never "inside". A
    /// definite sign at `≥ K·ε` on every row is what a `true` costs,
    /// so `Sign::Zero`, an in-band margin, poison, an envelope box
    /// that could not be separated and a ring copy that was never
    /// emitted all answer `false`. A consumer may therefore DROP the
    /// cell on `true` and must KEEP it on `false`; there is no path
    /// from an imprecise input to a drop.
    ///
    /// Two stages. **(1)** No boundary edge meets `rect`: a
    /// five-axis separating-axis test per segment (the rectangle's
    /// four sides and the segment's own normal), the box test alone
    /// for an envelope, since the chord lies in the box. **(2)** With
    /// (1) holding, `rect` meets no boundary, so its centre decides
    /// for all of it: a ray-parity walk at the centre answers outside
    /// when the outer polygon says `Out`, or when some ring copy says
    /// `In` (the centre is in a hole).
    pub fn certifies_outside(&self, rect: MetredRect, band: Band) -> bool {
        for lp in &self.loops {
            for e in &lp.edges {
                if !edge_clears(e, rect, band) {
                    return false;
                }
            }
        }
        let q = Point2::new(
            T::from_f64(0.5 * (rect.u_min + rect.u_max)),
            T::from_f64(0.5 * (rect.v_min + rect.v_max)),
        );
        for lp in &self.loops {
            let verts: Vec<Point2<T>> = lp.edges.iter().map(MetredEdge::a).collect();
            if verts.len() < 3 {
                // A polygon of fewer than three vertices bounds no
                // region; it can neither put the centre outside nor
                // put it in a hole.
                continue;
            }
            let Some(inside) = parity(&verts, q, band) else {
                continue;
            };
            if lp.ring {
                if inside {
                    return true; // in a hole ⇒ off the face
                }
            } else if !inside {
                return true;
            }
        }
        false
    }
}

/// Stage (1) for one edge: is `rect` definitely clear of it?
fn edge_clears<T: Decide>(edge: &MetredEdge<T>, rect: MetredRect, band: Band) -> bool {
    match *edge {
        MetredEdge::Segment { a, b } => {
            let hull = ChartWindow {
                u_min: a.x.min(b.x),
                u_max: a.x.max(b.x),
                v_min: a.y.min(b.y),
                v_max: a.y.max(b.y),
            };
            box_separates(&hull, rect, band) || normal_separates(a, b, rect, band)
        }
        // The chord lies in the box, so the box test alone answers for
        // the whole edge.
        MetredEdge::Envelope { image, .. } => box_separates(&image, rect, band),
    }
}

/// The rectangle's own four side axes: a definite positive gap on any
/// one of them separates.
fn box_separates<T: Decide>(w: &ChartWindow<T>, rect: MetredRect, band: Band) -> bool {
    [
        T::from_f64(rect.u_min) - w.u_max,
        w.u_min - T::from_f64(rect.u_max),
        T::from_f64(rect.v_min) - w.v_max,
        w.v_min - T::from_f64(rect.v_max),
    ]
    .into_iter()
    .any(|gap| matches!(decide(GAP, Margin::of(gap), band), Ok(Sign::Positive)))
}

/// The fifth axis: the segment's own normal. All four corners of
/// `rect` strictly on one side of the segment's line separates.
///
/// The signed offset is an area over the normal's length — the
/// perpendicular distance the corner stands off the line — through
/// [`Margin::over_lever`]. A degenerate segment divides by zero,
/// poisons, and separates nothing, which is the safe direction.
fn normal_separates<T: Decide>(a: Point2<T>, b: Point2<T>, rect: MetredRect, band: Band) -> bool {
    let e = b - a;
    let n = Vec2::new(-e.y, e.x);
    let lever = n.norm();
    let (mut all_positive, mut all_negative) = (true, true);
    for (cu, cv) in rect.corners() {
        let w = Vec2::new(T::from_f64(cu) - a.x, T::from_f64(cv) - a.y);
        match decide(GAP, Margin::over_lever(n.dot(w), lever), band) {
            Ok(Sign::Positive) => all_negative = false,
            Ok(Sign::Negative) => all_positive = false,
            _ => {
                all_positive = false;
                all_negative = false;
            }
        }
    }
    all_positive || all_negative
}

/// Stage (2) for one polygon: the ray-parity verdict at `q`, or `None`
/// when the walk cannot answer (on the boundary, both schedule members
/// grazing, or an escalated margin). The schedule is the fixed pair
/// `(+u, +v)`; the first definite verdict counts.
fn parity<T: Decide>(verts: &[Point2<T>], q: Point2<T>, band: Band) -> Option<bool> {
    if !matches!(ray_parity::on_boundary(verts, q, &ROWS, band), Ok(false)) {
        return None;
    }
    let (one, zero) = (T::one(), T::zero());
    for (d, side) in [
        (Vec2::new(one, zero), Vec2::new(zero, one)),
        (Vec2::new(zero, one), Vec2::new(-one, zero)),
    ] {
        match ray_parity::ray_verdict(verts, q, d, side, &ROWS, band) {
            Ok(Some(inside)) => return Some(inside),
            Ok(None) => {}
            Err(_) => return None,
        }
    }
    None
}
