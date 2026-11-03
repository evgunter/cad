//! **The rotational-chart Green lane** ([`rotational_cut_face_rounds`]):
//! the divergence-theorem contributions of a face on a chart of
//! revolution whose trim loops are curved, and the loop windings and
//! ring readings the face rests on. The chart table
//! ([`RotationalChartQ`]) holds the cylinder.
//!
//! # Formulation
//!
//! On a chart of revolution, `u` the azimuth about its axis, the flux
//! density about the chart's own origin and the area density depend on
//! the level `v` alone. With `F` and `G` their antiderivatives in `v`, a
//! face whose trim region is Ω (bounded by its loops, the outer one and
//! its rings, each traversed with the interior on its left about the
//! outward normal) has
//!
//! ```text
//! A     = −Σ_loops ∮ G(v) du   (signed chart area; A = s_f·|Ω|)
//! area  = |A|
//! flux  = −Σ_loops ∮ F(v) du + o·A⃗
//! ```
//!
//! with the chart term `o·A⃗` (`o` the chart origin less the point the
//! flux is about, `A⃗` the exact vector area of the face's carriers,
//! summed over the loops). On the cylinder
//! `S(u, v) = o + û·r·cos u + v̂·r·sin u + â·v` the normal is radial and
//! `dA = r du dv`, so `F(v) = r²·(v − v_a)` and `G(v) = r·(v − v_a)`.
//!
//! `v du` because it is a global 1-form on the chart (the azimuth is
//! periodic, its differential is not), so the form reads a loop that
//! winds the chart — a seamless band's rim — and a loop whose images
//! sit in different branches of the chart alike; `u dv` reads neither.
//! The loops' own traversal signs carry `s_f`, a ring's opposite to its
//! face's, so no separate orientation classification is needed; what is
//! decided is that the stored loops say so (every junction closes, a
//! whole number of turns apart in the azimuth; each loop winds at most
//! once and the face's loops none together, which makes the form
//! independent of the anchor level `v_a`; every loop that does not wind,
//! other than a non-band face's own, is wound against its face).
//!
//! # The edge terms
//!
//! The boundary is the face's **pcurves**, one [`EdgeTermQ`] per edge,
//! each contributing `σ·∫ −F(v(t))·u′(t) dt` (and `G`'s) over its stored
//! carrier interval, `σ` its traversal:
//!
//! - **Harmonic** ([`TrimEdgeQ`]): a minted cylinder pcurve
//!   `u, v(t) = c₀ + c_a·cos t + c_b·sin t + c_l·t`, by the composite
//!   midpoint rule below. A rim (`v` constant, `u` linear) gives
//!   `−F(V)·Δu` and a ruling (`u` constant) nothing, both read exactly.
//! - **Projected** ([`ProjectedEdgeQ`]): the chart's inverse applied to
//!   the carrier written in the chart's frame (`Pcurve::Projected`), a
//!   rational net ([`FramedQ::Net`]), a circle ([`FramedQ::Circle`]) or a
//!   line ([`FramedQ::Line`]).
//!   At the frame point `(x, y, z)` the azimuth's rate is
//!   `u′ = (x·y′ − y·x′)/(x² + y²)`, which reads no branch, and the
//!   cylinder's level is `v = v_off + v_sign·z` (the image's deck map).
//!   On a net `P = B/w`, `u′ = (B_x·B_y′ − B_y·B_x′)/(B_x² + B_y²)` (the
//!   weight cancels) and `z = B_z/w`. Both read through the curved rims'
//!   Taylor rule ([`crate::props::taylor`]; a net span by span,
//!   [`crate::props::rational_rim`]), with every span, and the circle's
//!   interval, cut at the image's sector breaks. The edge's ends are the
//!   image's own, read at the lane scalar; its turn `Δu = ∫u′` is the
//!   rule's, and the closure reads the two against each other, so an
//!   image whose evaluator reads a branch its integral does not is
//!   caught rather than integrated through.
//!
//! # The harmonic integrand
//!
//! Trigonometric values enter only as **enclosures propagated from the
//! edge endpoints**: the caller supplies `(cos t₀, sin t₀)` brackets at
//! the limits the lane integrates between
//! ([`trig_at_limit`](super::trig::trig_at_limit), rotated from the
//! exact `(1, 0)`), and the engine advances them by exact rotation with
//! polynomial alternating-series bounds on the step's `(cos, sin)`
//! (valid for |step| ≤ 1.5; every step here is ≤ 1). Sound and
//! transcendental-free.
//!
//! Per piece `[p, q]`, `h = q − p`, `m` the midpoint (Taylor with
//! integral remainder; `∫(t−m)² dt = h³/12`):
//!
//! ```text
//! ∫_p^q f  ∈  h·f(m) + F₂·h³/24,   F₂ ⊇ f'' over [p, q]
//! ```
//!
//! `f(m)` is a *thin* enclosure (rotation to the midpoint), `F₂` a hull
//! enclosure — the remainder is the enclosure width itself, so the
//! bracket is sound at every refinement level and only *tightens* as
//! pieces shrink. Refinement doubles the piece count.
//!
//! # Honesty pads (both directions accounted)
//!
//! - **Map residual**: each pcurve tracks its carrier within its
//!   certificate envelope `e` (metres through the map). The cylinder
//!   metric is `ds² = r²du² + dv²`, so the true trim boundary lies
//!   within metric distance `e` of the pcurve and the chart-area defect
//!   is ≤ Σ (metric edge length)·e, `r²/r` of it in flux per unit area —
//!   added symmetrically.
//! - **Endpoint parameter brackets**: a non-thin `t₀/t₁` bracket of a
//!   harmonic edge contributes `mag(f)·width` at each end. A projected
//!   edge's carrier interval is exact.
//!
//! The closed-form part of `o·A⃗` rides at the caller's scalar exactly as
//! the closed-form lanes do (f64 rounding of closed forms is the
//! documented slack there, unchanged); a rational carrier's part is the
//! curved rims' rule, per round.

use geom_core::interval::certification::Certification;
use geom_core::interval::{Interval, certified_hi};
use geom_core::{Band, Bounds, Decide};

use super::RVec3;
use super::run::{Accept, EDGE_INIT_PIECES, FaceCutBounds, LaneRun, RoundWindow, Run, face_extent};
use super::trig::{limit, rotate, spread_cos, spread_sin, trig_at, trig_over};
use crate::props::PropsError;
use crate::props::bracket::{AbsEnclosure, mid, pt};
use crate::props::rational_rim::{
    RationalEdgeQ, rational_integrals, rational_rate_bound, rational_vector_area_split,
};
use crate::props::spiric::{W_SPIRIC, w_rational};
use crate::props::taylor::{Jet, composite, composite_by, node, trig_jets, trig_range};

/// Weight of one harmonic piece of a cylinder edge, in [`FACE_WORK_WALL`](super::run::FACE_WORK_WALL)'s unit.
const W_CYLINDER: usize = 3;

/// The work of round `round` on a face of `edges` harmonic trim edges.
#[cfg(test)]
pub(super) fn round_cost(edges: usize, round: usize) -> usize {
    W_CYLINDER * edges * (EDGE_INIT_PIECES << round)
}

/// The work of round `round` on `face`: a harmonic edge's pieces at
/// [`W_CYLINDER`], a projected net's and a rational chart term's at a
/// rational rim's weight per span ([`w_rational`]: the same piece forms,
/// read by Horner), a projected circle's at a spiric's per interval
/// ([`W_SPIRIC`]: the same trig jets and one reciprocal), and a projected
/// line's at a degree-1 rim's per interval.
fn face_cost(face: &RotationalFaceQ, round: usize) -> usize {
    let rational = |e: &RationalEdgeQ| w_rational(e.degree()) * e.span_count();
    let edges: usize = face
        .loops
        .iter()
        .flatten()
        .map(|e| match e {
            EdgeTermQ::Harmonic(_) => W_CYLINDER,
            EdgeTermQ::Projected(p) => match &p.carrier {
                FramedQ::Net(net) => rational(net),
                FramedQ::Circle { cuts, .. } => W_SPIRIC * cuts.len().saturating_sub(1),
                FramedQ::Line { cuts, .. } => w_rational(1) * cuts.len().saturating_sub(1),
            },
        })
        .sum();
    let terms: usize = face.chart_term.rationals.iter().map(rational).sum();
    (edges + terms) * (EDGE_INIT_PIECES << round)
}

/// One harmonic chart channel `c₀ + c_a·cos t + c_b·sin t + c_l·t`
/// with ring-bracketed coefficients.
#[derive(Clone, Copy, Debug)]
pub struct HarmChan {
    /// The constant coefficient.
    pub c0: Interval,
    /// The `cos t` coefficient.
    pub ca: Interval,
    /// The `sin t` coefficient.
    pub cb: Interval,
    /// The linear-in-`t` coefficient.
    pub cl: Interval,
}

/// One harmonic trim-loop edge of the lane, fully bracketed by the
/// caller (key-free, like [`crate::props::LoopEdge`]).
#[derive(Clone, Copy, Debug)]
pub struct TrimEdgeQ {
    /// The azimuth channel `u(t)`.
    pub u: HarmChan,
    /// The height channel `v(t)`.
    pub v: HarmChan,
    /// Bracket of the certified interval start.
    pub t0: Interval,
    /// Bracket of the certified interval end.
    pub t1: Interval,
    /// Loop traversal direction (`he_plus`-forward or reversed).
    pub forward: bool,
    /// Enclosure of `(cos a, sin a)` at the start limit `a`
    /// ([`TrimEdgeQ::limits`]; the at-rest caller forms it by
    /// [`trig_at_limit`](super::trig::trig_at_limit)).
    pub trig0: (Interval, Interval),
    /// Enclosure of `(cos a, sin a)` at the end limit, formed as `trig0`
    /// is. Where the loop's junctions are read ([`chart_end`]), the end
    /// is this bracket rather than the start's carried through the span,
    /// which on a steep conic widens it by the conic's tilt. `None` reads
    /// the end through the span.
    pub trig1: Option<(Interval, Interval)>,
    /// The pcurve certificate envelope (metres through the map) — the
    /// map-residual honesty pad's per-edge input.
    pub env: Interval,
}

impl TrimEdgeQ {
    /// The integration limits every reading of the edge takes: the
    /// midpoints of its parameter brackets, whose widths the
    /// endpoint-bracket pad repays.
    fn limits(&self) -> (f64, f64) {
        (limit(self.t0), limit(self.t1))
    }
}

/// The carrier of a projected edge in its chart's frame (origin at the
/// chart's origin, `z` along its axis).
#[derive(Clone, Debug)]
pub enum FramedQ {
    /// A spline net's Bézier spans about the frame's origin over the
    /// edge's carrier interval, cut at the image's sector breaks
    /// ([`crate::props::rational_rim::rational_edge_q_split`], built
    /// forward: the edge's traversal is [`ProjectedEdgeQ::forward`]).
    Net(RationalEdgeQ),
    /// A circle `centre + a·cos t + b·sin t`, read over `cuts`: the
    /// edge's carrier interval, its ends and the image's sector breaks
    /// inside it, increasing, each an exact parameter.
    Circle {
        /// The centre.
        centre: RVec3,
        /// The `cos t` coefficient.
        a: RVec3,
        /// The `sin t` coefficient.
        b: RVec3,
        /// The interval's ends and the breaks between them.
        cuts: Vec<f64>,
    },
    /// A line `point + dir·t`, read over `cuts` as a circle is.
    Line {
        /// The point at `t = 0`.
        point: RVec3,
        /// The direction, per unit of the parameter.
        dir: RVec3,
        /// The interval's ends and the breaks between them.
        cuts: Vec<f64>,
    },
}

/// One projected trim-loop edge of the lane (module docs), bracketed by
/// the caller.
#[derive(Clone, Debug)]
pub struct ProjectedEdgeQ {
    /// The carrier in the chart's frame.
    pub carrier: FramedQ,
    /// The deck map's level offset `v_off`.
    pub v_off: Interval,
    /// The deck map's level sign `v_sign`, exactly `±1`.
    pub v_sign: f64,
    /// Loop traversal direction.
    pub forward: bool,
    /// The image's chart points `(u, v)` at the carrier interval's start
    /// and its end, in that order whatever the traversal.
    pub ends: [(Interval, Interval); 2],
    /// The pcurve certificate envelope (metres through the map).
    pub env: Interval,
}

/// One trim-loop edge of the lane: how its Green-form terms are read
/// (module docs).
#[derive(Clone, Debug)]
pub enum EdgeTermQ {
    /// A harmonic pcurve.
    Harmonic(TrimEdgeQ),
    /// A projected pcurve.
    Projected(ProjectedEdgeQ),
}

/// The chart a rotational face's Green form runs on: the per-chart
/// `F`/`G` table (module docs).
#[derive(Clone, Copy, Debug)]
pub enum RotationalChartQ {
    /// A cylinder of radius `r`: `F = r²·(v − v_a)`, `G = r·(v − v_a)`.
    Cylinder {
        /// `r`.
        radius: Interval,
    },
}

impl RotationalChartQ {
    /// The metres one radian of azimuth moves a point of the chart: what
    /// meters a junction's azimuth gap and an edge's chart length.
    fn azimuth_lever(self) -> Interval {
        match self {
            Self::Cylinder { radius } => radius,
        }
    }

    /// `[F, G]` as jets, at a projected image's frame height `z` under
    /// its deck map's level `v_off + v_sign·z`.
    fn projected_forms(self, v_off: Interval, v_sign: f64, z: Jet, anchor: f64) -> [Jet; 2] {
        match self {
            Self::Cylinder { radius } => {
                let g = z
                    .scale(pt(v_sign))
                    .add(Jet::cst(v_off - pt(anchor)))
                    .scale(radius);
                [g.scale(radius), g]
            }
        }
    }

    /// `[−∫F du, −∫G du]` from the level integral `−∫(v − v_a) du`, on a
    /// chart whose `F` and `G` are that integral's multiples: what the
    /// harmonic rule reads.
    fn of_level(self, level: Interval) -> [Interval; 2] {
        match self {
            Self::Cylinder { radius } => [radius.sqr() * level, radius * level],
        }
    }

    /// The factor the harmonic rule's level width is read at in flux.
    fn level_flux_scale(self) -> f64 {
        match self {
            Self::Cylinder { radius } => radius.sqr().mag(),
        }
    }

    /// The map-residual pad `[flux, area]` of an edge of metric length
    /// `length` whose image is within `env` of its carrier: the chart
    /// area it may move, `length·env`, at the flux density's sup.
    fn residual(self, length: f64, env: Interval) -> [f64; 2] {
        match self {
            Self::Cylinder { radius } => {
                let area = (pt(length) * env).mag();
                [(pt(area) * radius).mag(), area]
            }
        }
    }
}

/// The chart term `o·A⃗` of a face (module docs): the closed-form edges'
/// part, bracketed by the caller, and each rational carrier's vector
/// area about the chart origin, read through `lever` (the chart origin
/// less the point the flux is about) per round.
#[derive(Clone, Debug)]
pub struct ChartTermQ {
    /// The closed-form edges' `o·A⃗`.
    pub closed: Interval,
    /// The chart origin less the point the flux is about.
    pub lever: RVec3,
    /// The rational carriers, about the chart origin.
    pub rationals: Vec<RationalEdgeQ>,
}

/// One face of the lane, bracketed by the caller: its chart, its chart
/// term and its loops, the outer first.
#[derive(Clone, Debug)]
pub struct RotationalFaceQ {
    /// The chart.
    pub chart: RotationalChartQ,
    /// The chart term `o·A⃗`.
    pub chart_term: ChartTermQ,
    /// Every loop of the face, each in traversal order.
    pub loops: Vec<Vec<EdgeTermQ>>,
}

// ---------------------------------------------------------------------
// Harmonic channels
// ---------------------------------------------------------------------

impl HarmChan {
    /// The channel's derivative as a channel:
    /// `d/dt (c₀ + c_a c + c_b s + c_l t) = c_l + c_b c − c_a s`.
    fn deriv(self) -> Self {
        Self {
            c0: self.cl,
            ca: self.cb,
            cb: -self.ca,
            cl: Interval::zero(),
        }
    }

    /// Enclosure of the channel over trig enclosures `(c, s)` and a
    /// `t` bracket (fixed association order, D9).
    fn eval(self, c: Interval, s: Interval, t: Interval) -> Interval {
        self.c0 + self.ca * c + self.cb * s + self.cl * t
    }

    /// Whether the channel's harmonic part is exactly zero, and so its
    /// linear one too when `linear` is false.
    fn is_flat(self, linear: bool) -> bool {
        let zero = |x: Interval| x.is_certified() && x.lo() == 0.0 && x.hi() == 0.0;
        zero(self.ca) && zero(self.cb) && (linear || zero(self.cl))
    }
}

// ---------------------------------------------------------------------
// The harmonic rule (module docs: h·f(m) + F₂·h³/24 per piece)
// ---------------------------------------------------------------------

/// `σ·∫_{a}^{b} (anchor − v(t))·u'(t) dt` for one harmonic edge at
/// `pieces` resolution — its share of `−∮ (v − anchor) du` — plus the
/// edge's endpoint-bracket pad; and the summed width of its remainder
/// terms `F₂·h³/24`, the part of its width a later round removes. A rim
/// and a ruling are read exactly (module docs).
fn harmonic_edge_integral(e: &TrimEdgeQ, anchor: f64, pieces: usize) -> (Interval, f64) {
    let (a, b) = e.limits();
    let span = b - a;
    if !(span.is_finite() && span >= 0.0) || pieces == 0 {
        return (Interval::refused(), f64::INFINITY);
    }
    let p = HarmChan {
        c0: pt(anchor) - e.v.c0,
        ca: -e.v.ca,
        cb: -e.v.cb,
        cl: -e.v.cl,
    };
    let dq = e.u.deriv();
    let (total, removable) = if e.u.is_flat(false) {
        (Interval::zero(), 0.0)
    } else if e.u.is_flat(true) && e.v.is_flat(false) {
        (p.c0 * e.u.cl * pt(span), 0.0)
    } else {
        harmonic_rule(e, p, pieces)
    };
    // Endpoint-bracket pad: |f| near the ends times the t-bracket
    // widths (module docs).
    let (c_all, s_all) = trig_over(e.trig0, 0.0, span);
    let t_all = Interval::from_bounds(a, b);
    let f_mag = (p.eval(c_all, s_all, t_all) * dq.eval(c_all, s_all, t_all)).mag();
    let wt = e.t0.width() + e.t1.width();
    let pad = f_mag * wt;
    let total = total + Interval::from_bounds(-pad, pad);
    (if e.forward { total } else { -total }, removable)
}

/// The composite rule of [`harmonic_edge_integral`] for `f = p·u′`.
fn harmonic_rule(e: &TrimEdgeQ, p: HarmChan, pieces: usize) -> (Interval, f64) {
    let (a, b) = e.limits();
    let span = b - a;
    let dp = p.deriv();
    let dq = e.u.deriv();
    // f = p·q'; f'' = p''·q' + 2·p'·q'' + p·q'''.
    let ddp = dp.deriv();
    let ddq = dq.deriv();
    let dddq = ddq.deriv();
    let mut total = Interval::zero();
    let mut removable = 0.0_f64;
    #[allow(clippy::cast_precision_loss)]
    let h = span / pieces as f64;
    let h3_24 = pt(h) * pt(h).sqr() / pt(24.0);
    // Per-piece midpoint rotation via `trig_at` (double-angle
    // composition): the base bracket passes through exactly ONE
    // rotation per piece, so its width never compounds across pieces
    // (a cursor march would amplify it by ~e^span).
    // Per-piece spread: midpoint ± h/2 (the interval rotation of
    // `trig_over`, midpoint-anchored).
    // `h` is finite by the caller's `span`/`pieces` guard, so — as in
    // `trig_over` — the `.max`/`.min` here operate on refusal-free ring
    // arithmetic and are not the `clamped_to` hazard.
    let h2 = h * 0.5;
    let (spread_c, spread_s) = (spread_cos(h2), spread_sin(h2));
    for i in 0..pieces {
        #[allow(clippy::cast_precision_loss)]
        let lo = a + span * (i as f64 / pieces as f64);
        let m = lo + h * 0.5;
        // Thin midpoint value.
        let (cm, sm) = trig_at(e.trig0, m - a);
        let fm = p.eval(cm, sm, pt(m)) * dq.eval(cm, sm, pt(m));
        // f'' hull over the piece (midpoint spread by ± h/2).
        let (cr, sr) = rotate(cm, sm, spread_c, spread_s);
        let tr = Interval::from_bounds(lo, lo + h);
        let f2 = ddp.eval(cr, sr, tr) * dq.eval(cr, sr, tr)
            + pt(2.0) * dp.eval(cr, sr, tr) * ddq.eval(cr, sr, tr)
            + p.eval(cr, sr, tr) * dddq.eval(cr, sr, tr);
        let remainder = f2 * h3_24;
        removable += remainder.width();
        total = total + pt(h) * fm + remainder;
    }
    (total, removable)
}

/// A certified upper bound on a harmonic edge's METRIC length (metres):
/// `∫ √(r²u'² + v'²) dt ≤ (r·sup|u'| + sup|v'|)·span`.
fn harmonic_metric_length(e: &TrimEdgeQ, lever: Interval) -> f64 {
    let (a, b) = e.limits();
    let span = b - a;
    let (c_all, s_all) = trig_over(e.trig0, 0.0, span);
    let t_all = Interval::from_bounds(a, b);
    let du = e.u.deriv();
    let dv = e.v.deriv();
    ((lever * du.eval(c_all, s_all, t_all).abs_enclosure()
        + dv.eval(c_all, s_all, t_all).abs_enclosure())
        * pt(span))
    .mag()
}

// ---------------------------------------------------------------------
// Projected edges (module docs: the curved rims' rule over the frame)
// ---------------------------------------------------------------------

/// The frame point's jets `(x, y, z)` and the azimuth's rate's numerator
/// and denominator, `x·y′ − y·x′` and `x² + y²`, of a projected net in
/// homogeneous coordinates (the weight cancels from `u′`).
fn net_rate(b: [Jet; 3], db: [Jet; 3], w: Jet) -> (Jet, Jet) {
    let du = b[0]
        .mul(db[1])
        .sub(b[1].mul(db[0]))
        .mul(b[0].sqr().add(b[1].sqr()).recip());
    (du, b[2].mul(w.recip()))
}

/// The jets of `u′` and `z` of a projected circle at the trig jets `c`,
/// `s` of its parameter.
fn circle_rate(centre: RVec3, a: RVec3, b: RVec3, c: Jet, s: Jet) -> (Jet, Jet) {
    let at = |k: usize| Jet::cst(centre[k]).add(c.scale(a[k])).add(s.scale(b[k]));
    let slope = |k: usize| c.scale(b[k]).sub(s.scale(a[k]));
    let (x, y) = (at(0), at(1));
    let du = x
        .mul(slope(1))
        .sub(y.mul(slope(0)))
        .mul(x.sqr().add(y.sqr()).recip());
    (du, at(2))
}

/// The jets of `u′` and `z` of a projected line at the jet `t` of its
/// parameter.
fn line_rate(point: RVec3, dir: RVec3, t: Jet) -> (Jet, Jet) {
    let at = |k: usize| Jet::cst(point[k]).add(t.scale(dir[k]));
    let (x, y) = (at(0), at(1));
    let du = x
        .scale(dir[1])
        .sub(y.scale(dir[0]))
        .mul(x.sqr().add(y.sqr()).recip());
    (du, at(2))
}

/// `[−∫F du, −∫G du, ∫du]` of a projected edge in its carrier's own
/// order at `pieces` per span or cut interval, and the part of each
/// width a later round removes.
fn projected_integrals(
    e: &ProjectedEdgeQ,
    chart: RotationalChartQ,
    anchor: f64,
    pieces: usize,
) -> ([Interval; 3], [f64; 3]) {
    let terms = |du: Jet, z: Jet| {
        let [f, g] = chart.projected_forms(e.v_off, e.v_sign, z, anchor);
        let minus = -Interval::one();
        [f.mul(du).scale(minus), g.mul(du).scale(minus), du]
    };
    match &e.carrier {
        FramedQ::Net(net) => rational_integrals(net, pieces, |b, db, w| {
            let (du, z) = net_rate(b, db, w);
            terms(du, z)
        }),
        FramedQ::Circle { centre, a, b, cuts } => {
            let mut total = [Interval::zero(); 3];
            let mut removable = [0.0_f64; 3];
            for w in cuts.windows(2) {
                let (part, rem) = composite(pt(w[0]), pt(w[1]), pieces, |_, c, s| {
                    let (du, z) = circle_rate(*centre, *a, *b, c, s);
                    terms(du, z)
                });
                for k in 0..3 {
                    total[k] = total[k] + part[k];
                    removable[k] += rem[k];
                }
            }
            (total, removable)
        }
        FramedQ::Line { point, dir, cuts } => {
            let mut total = [Interval::zero(); 3];
            let mut removable = [0.0_f64; 3];
            for w in cuts.windows(2) {
                let (part, rem) = composite_by(
                    pt(w[0]),
                    pt(w[1]),
                    pieces,
                    |_, _| (),
                    |(), t| {
                        let (du, z) = line_rate(*point, *dir, Jet::var(t));
                        terms(du, z)
                    },
                );
                for k in 0..3 {
                    total[k] = total[k] + part[k];
                    removable[k] += rem[k];
                }
            }
            (total, removable)
        }
    }
}

/// Pieces per span or cut interval at which a projected edge's metric
/// length and its turn are read once, before the rounds: the length is
/// an upper bound at any count, and the turn need only tell whole turns
/// apart ([`loop_windings`]).
const PROJECTED_ENTRY_PIECES: usize = 4;

/// A certified upper bound on a projected edge's metric length (metres),
/// `Σ h·(lever·sup|u′| + sup|z′|)` over its pieces; `NaN` where a piece
/// reads no bound.
fn projected_metric_length(e: &ProjectedEdgeQ, lever: Interval) -> f64 {
    let rate = |du: Jet, z: Jet| {
        let speed = lever * du.0[0].abs_enclosure() + z.0[1].abs_enclosure();
        if speed.is_certified() {
            speed.hi()
        } else {
            f64::NAN
        }
    };
    match &e.carrier {
        FramedQ::Net(net) => rational_rate_bound(net, PROJECTED_ENTRY_PIECES, |b, db, w| {
            let (du, z) = net_rate(b, db, w);
            rate(du, z)
        }),
        FramedQ::Circle { centre, a, b, cuts } => {
            let mut total = 0.0_f64;
            for w in cuts.windows(2) {
                for i in 0..PROJECTED_ENTRY_PIECES {
                    let (p, q) = (
                        node(w[0], w[1], PROJECTED_ENTRY_PIECES, i),
                        node(w[0], w[1], PROJECTED_ENTRY_PIECES, i + 1),
                    );
                    let over = Interval::from_bounds(p, q);
                    let (c, s) = trig_range(p, q);
                    let (_, cj, sj) = trig_jets(over, c, s);
                    let (du, z) = circle_rate(*centre, *a, *b, cj, sj);
                    total += (pt(rate(du, z)) * (pt(q) - pt(p))).hi();
                }
            }
            total
        }
        FramedQ::Line { point, dir, cuts } => {
            let mut total = 0.0_f64;
            for w in cuts.windows(2) {
                for i in 0..PROJECTED_ENTRY_PIECES {
                    let (p, q) = (
                        node(w[0], w[1], PROJECTED_ENTRY_PIECES, i),
                        node(w[0], w[1], PROJECTED_ENTRY_PIECES, i + 1),
                    );
                    let (du, z) = line_rate(*point, *dir, Jet::var(Interval::from_bounds(p, q)));
                    total += (pt(rate(du, z)) * (pt(q) - pt(p))).hi();
                }
            }
            total
        }
    }
}

// ---------------------------------------------------------------------
// One edge, whatever its term
// ---------------------------------------------------------------------

/// What the lane reads of each edge once, before the rounds: its metric
/// length, and a projected edge's turn `Δu` in its carrier's order.
struct EdgeEntry {
    length: f64,
    turn: Option<Interval>,
}

/// What the lane reads of `e` before the rounds.
fn edge_entry(e: &EdgeTermQ, chart: RotationalChartQ) -> Result<EdgeEntry, PropsError> {
    let lever = chart.azimuth_lever();
    Ok(match e {
        EdgeTermQ::Harmonic(h) => EdgeEntry {
            length: harmonic_metric_length(h, lever),
            turn: None,
        },
        EdgeTermQ::Projected(p) => {
            let length = projected_metric_length(p, lever);
            let ([_, _, turn], _) = projected_integrals(p, chart, 0.0, PROJECTED_ENTRY_PIECES);
            if !(length.is_finite() && turn.is_certified()) {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a projected edge whose chart speed or turn reads no enclosure — its \
                           carrier reaches the chart's axis",
                });
            }
            EdgeEntry {
                length,
                turn: Some(turn),
            }
        }
    })
}

/// One edge's `[−∫F du, −∫G du]` at `pieces`, traversal and pads folded
/// in, and the part of its flux width a later round removes.
fn edge_terms(
    e: &EdgeTermQ,
    entry: &EdgeEntry,
    chart: RotationalChartQ,
    anchor: f64,
    pieces: usize,
) -> ([Interval; 2], f64) {
    let (terms, removable, env) = match e {
        EdgeTermQ::Harmonic(h) => {
            let (level, rem) = harmonic_edge_integral(h, anchor, pieces);
            (chart.of_level(level), rem * chart.level_flux_scale(), h.env)
        }
        EdgeTermQ::Projected(p) => {
            let ([f, g, _], [rf, _, _]) = projected_integrals(p, chart, anchor, pieces);
            let terms = if p.forward { [f, g] } else { [-f, -g] };
            (terms, rf, p.env)
        }
    };
    let pad = chart.residual(entry.length, env);
    (
        [0, 1].map(|k| terms[k] + Interval::from_bounds(-pad[k], pad[k])),
        removable,
    )
}

/// The face's chart term at `pieces`, and the part of its width a later
/// round removes.
fn chart_term_at(term: &ChartTermQ, pieces: usize) -> (Interval, f64) {
    term.rationals
        .iter()
        .fold((term.closed, 0.0), |(acc, removable), e| {
            let (va, rem) = rational_vector_area_split(e, pieces);
            let flux = (0..3).fold(acc, |a, k| a + term.lever[k] * va[k]);
            let rem: f64 = (0..3).map(|k| term.lever[k].mag() * rem[k]).sum();
            (flux, removable + rem)
        })
}

// ---------------------------------------------------------------------
// The lane
// ---------------------------------------------------------------------

/// The flux and area enclosures of a **cylinder** face whose loops are
/// all harmonic: [`rotational_cut_face_rounds`] on the cylinder chart,
/// with `o_dot_va` its whole chart term, over the full schedule — the
/// harmonic rows' spelling.
#[cfg(test)]
fn cylinder_cut_face<T: Decide>(
    radius: Interval,
    o_dot_va: Interval,
    loops: &[&[TrimEdgeQ]],
    band: Band,
    accept: &mut Accept<'_>,
) -> Result<LaneRun, PropsError> {
    cylinder_cut_face_rounds::<T>(radius, o_dot_va, loops, band, RoundWindow::SCHEDULE, accept)
}

/// [`cylinder_cut_face`] over a [`RoundWindow`].
#[cfg(test)]
fn cylinder_cut_face_rounds<T: Decide>(
    radius: Interval,
    o_dot_va: Interval,
    loops: &[&[TrimEdgeQ]],
    band: Band,
    window: RoundWindow,
    accept: &mut Accept<'_>,
) -> Result<LaneRun, PropsError> {
    let face = RotationalFaceQ {
        chart: RotationalChartQ::Cylinder { radius },
        chart_term: ChartTermQ {
            closed: o_dot_va,
            lever: [Interval::zero(); 3],
            rationals: Vec::new(),
        },
        loops: loops
            .iter()
            .map(|edges| edges.iter().copied().map(EdgeTermQ::Harmonic).collect())
            .collect(),
    };
    rotational_cut_face_rounds::<T>(&face, band, window, accept)
}

/// The flux and area enclosures of a face on a chart of revolution with
/// a curved trim (module docs: the Green form, the edge terms, the
/// rules, the honesty pads), over every loop of the face, the outer
/// first: the Green form is a sum over edges, so a ring contributes its
/// own `−∮ G du` with its own winding. Rounds run over `window` until
/// `accept` takes one or the lane stops
/// ([`LaneStop`](super::run::LaneStop)); the pieces of a schedule
/// compose ([`RoundWindow`]).
///
/// `pub` because its consumer is `topo`'s `props::quad_lane`, across
/// the crate boundary; there is no narrower visibility that reaches
/// it.
///
/// # Errors
///
/// [`PropsError`] — `accept`'s own refusal, an escalated funnel
/// decision, a degenerate face, a projected edge whose chart speed reads
/// no enclosure, a loop that winds the chart by no whole number of
/// turns, or loops that wind it together
/// ([`PropsError::LoopsWindChart`]), a ring wound with its face at the
/// round the call ends ([`PropsError::RingWoundWithFace`]), one wholly
/// inside the band, or one still straddling it there (an accepted round
/// whose ring straddles refines on, [`Run::ringed`]).
pub fn rotational_cut_face_rounds<T: Decide>(
    face: &RotationalFaceQ,
    band: Band,
    window: RoundWindow,
    accept: &mut Accept<'_>,
) -> Result<LaneRun, PropsError> {
    let chart = face.chart;
    let entries: Vec<Vec<EdgeEntry>> = face
        .loops
        .iter()
        .map(|edges| edges.iter().map(|e| edge_entry(e, chart)).collect())
        .collect::<Result<_, _>>()?;
    let winds = loop_windings::<T>(face, &entries, band)?;
    let anchor = level_anchor(&face.loops)?;
    let cost = |round: usize| face_cost(face, round);
    let mut run = Run::enter(window, &cost);
    let mut round = run.first();
    let mut pieces = EDGE_INIT_PIECES << round;
    loop {
        // Per loop: [−∮F du, −∮G du] at this resolution.
        let mut removable = 0.0_f64;
        let per_loop: Vec<[Interval; 2]> = face
            .loops
            .iter()
            .zip(&entries)
            .map(|(edges, entries)| {
                edges
                    .iter()
                    .zip(entries)
                    .fold([Interval::zero(); 2], |acc, (e, entry)| {
                        let (terms, rem) = edge_terms(e, entry, chart, anchor, pieces);
                        removable += rem;
                        [acc[0] + terms[0], acc[1] + terms[1]]
                    })
            })
            .collect();
        let (term, term_rem) = chart_term_at(&face.chart_term, pieces);
        let (flux, signed) = per_loop
            .iter()
            .fold((term, Interval::zero()), |(f, a), l| (f + l[0], a + l[1]));
        let area = signed.abs_enclosure();
        let bounds = FaceCutBounds { flux, area };
        let (stop, accepted) = run.ask(round, bounds, removable + term_rem, accept, || {
            let perim: f64 = entries.iter().flatten().map(|e| e.length).sum();
            face_extent::<T>(area, perim, band)
        })?;
        if let Some(out) = run.ringed(round, stop, accepted, bounds, || {
            require_holes_wound_against(&entries, &per_loop, &winds, signed, band)
        })? {
            return Ok(out);
        }
        round += 1;
        pieces *= 2;
    }
}

/// An edge's chart point `(u, v)` where its traversal starts (`end`
/// false) or ends — a harmonic edge's at its [`TrimEdgeQ::limits`].
fn chart_end(e: &EdgeTermQ, end: bool) -> (Interval, Interval) {
    match e {
        EdgeTermQ::Harmonic(e) => {
            let (a, b) = e.limits();
            let (trig, t) = if end == e.forward {
                (e.trig1.unwrap_or_else(|| trig_at(e.trig0, b - a)), b)
            } else {
                (e.trig0, a)
            };
            (
                e.u.eval(trig.0, trig.1, pt(t)),
                e.v.eval(trig.0, trig.1, pt(t)),
            )
        }
        EdgeTermQ::Projected(p) => p.ends[usize::from(end == p.forward)],
    }
}

/// How far an edge's chart point at one end of its traversal may sit
/// from the image of the vertex it stands for, in the chart metric
/// `lever·|Δu| + |Δv|`: the pcurve's certificate envelope (a distance in
/// metres, which that metric exceeds by at most `√2`, taken as `3/2`),
/// plus, on a harmonic edge, the end's parameter bracket half-width
/// times the edge's speed bound in that metric. The trig brackets need
/// no term: [`chart_end`]'s enclosures carry them.
fn end_slack(e: &EdgeTermQ, entry: &EdgeEntry, end: bool) -> f64 {
    let (env, bracket) = match e {
        EdgeTermQ::Harmonic(e) => {
            let (a, b) = e.limits();
            let bracket = if end == e.forward { e.t1 } else { e.t0 };
            let span = b - a;
            let speed = if span > 0.0 { entry.length / span } else { 0.0 };
            (e.env, speed * bracket.width() * 0.5)
        }
        EdgeTermQ::Projected(p) => (p.env, 0.0),
    };
    (1.5 * certified_hi(env) + bracket) * (1.0 + 1e-12)
}

/// **Every junction of every loop closes on the chart, and the loops
/// wind it as a face's boundary does** — the periodic-chart closure
/// decision ([`crate::props::closure`]) on the lane's chart images.
///
/// Each junction's gap is read off the two ends' chart images
/// ([`chart_end`]): the azimuth metered at the chart's lever, the level
/// bare, and the slack the two ends' [`end_slack`]
/// ([`crate::props::closure::interval_loop_turns`]). A projected edge's
/// ends are its image's evaluator's and its turn `Δu` the rule's, so it
/// adds the whole turns by which the two disagree ([`branch_turns`]), and
/// the loop's turns count as the integral reads them.
///
/// Returns, per loop, whether it winds.
fn loop_windings<T: Decide>(
    face: &RotationalFaceQ,
    entries: &[Vec<EdgeEntry>],
    band: Band,
) -> Result<Vec<bool>, PropsError> {
    let lever = face.chart.azimuth_lever();
    let mut windings = Vec::with_capacity(face.loops.len());
    for (edges, entries) in face.loops.iter().zip(entries) {
        let n = edges.len();
        let junctions = (0..n).map(|i| {
            let (e, next) = (&edges[i], &edges[(i + 1) % n]);
            let (u1, v1) = chart_end(e, true);
            let (u0, v0) = chart_end(next, false);
            crate::props::closure::IntervalJunction {
                delta: [u1 - u0, v1 - v0],
                lever: [lever, pt(1.0)],
                slack: end_slack(e, &entries[i], true)
                    + end_slack(next, &entries[(i + 1) % n], false),
            }
        });
        let [k, _] =
            crate::props::closure::interval_loop_turns::<T, 2>(junctions, [true, false], band)?;
        let mut k = k;
        for (e, entry) in edges.iter().zip(entries) {
            if let (EdgeTermQ::Projected(p), Some(turn)) = (e, entry.turn) {
                let turns = branch_turns(p, turn)?;
                k = k.saturating_add(if p.forward { turns } else { -turns });
            }
        }
        windings.push([k]);
    }
    crate::props::closure::require_boundary_windings(&windings)?;
    Ok(windings.iter().map(|&[k]| k != 0).collect())
}

/// **The whole turns a projected edge's turn makes past its ends'
/// difference**, in its carrier's order: `(Δu − (u₁ − u₀))/2π`, the ends
/// read by the image's
/// evaluator, which places each on its piece's branch, and the turn by
/// the rule. The two differ by whole turns where the image's branches
/// meet as its sectors say, so the enclosure must lie within a quarter
/// turn of one whole number, which the rule's coarsest reading meets by
/// far; anything else is an image whose evaluator and integral disagree,
/// and refuses.
fn branch_turns(p: &ProjectedEdgeQ, turn: Interval) -> Result<i64, PropsError> {
    let tau = crate::props::bracket::tau_bracket();
    let gap = (turn - (p.ends[1].0 - p.ends[0].0)) / tau;
    let k = mid(gap).round();
    let off = (gap - pt(k)).abs_enclosure();
    if !(off.is_certified() && off.hi() < 0.25) {
        return Err(PropsError::QuadratureUnsupported {
            what: "a projected edge whose image's ends and turn disagree by no whole number \
                   of turns",
        });
    }
    // `k` is within a quarter of a finite enclosure here; the cast
    // saturates.
    #[allow(clippy::cast_possible_truncation)]
    Ok(k as i64)
}

/// The level the Green form is taken about: the middle of the levels
/// the face's edges start at. Any level gives the same form once the
/// loops wind zero times together ([`loop_windings`]); one inside the
/// face keeps a short face far along the axis at the conditioning of
/// its own height rather than of its distance from the chart origin.
///
/// Read after [`loop_windings`], which has decided every edge's start
/// point (a refused one escalates there), so every level here is
/// finite. A face with no edge has none, and no extent.
///
/// # Errors
///
/// [`PropsError::DegenerateFace`] for a face whose loops hold no edge.
fn level_anchor(loops: &[Vec<EdgeTermQ>]) -> Result<f64, PropsError> {
    let (lo, hi) = loops
        .iter()
        .flatten()
        .map(|e| mid(chart_end(e, false).1))
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| {
            (lo.min(v), hi.max(v))
        });
    if lo <= hi {
        Ok(lo + (hi - lo) * 0.5)
    } else {
        Err(PropsError::DegenerateFace)
    }
}

/// **Every ring that does not wind the chart winds against its face**,
/// through the closed forms' own decision
/// ([`crate::props::curved::require_holes_wound_against`], the mean width
/// `2·A/P` of the face's whole and of each such ring, in m² over m) at
/// the interval scalar: a ring definitely opposite to the whole passes,
/// one definitely the same way refuses
/// ([`PropsError::RingWoundWithFace`]), and an enclosure that straddles
/// the band escalates rather than takes a side. A ring that winds is a
/// band's rim, whose own `∮` depends on the anchor, so it is no hole and
/// is passed over, as the closed form passes over it
/// (`curved::cylinder_holes`); on such a face every loop that does not
/// wind is a hole, the first one stored included. A face with one loop
/// decides nothing here.
fn require_holes_wound_against(
    entries: &[Vec<EdgeEntry>],
    per_loop: &[[Interval; 2]],
    winds: &[bool],
    signed: Interval,
    band: Band,
) -> Result<(), PropsError> {
    if entries.len() < 2 {
        return Ok(());
    }
    let length = |entries: &[EdgeEntry]| pt(entries.iter().map(|e| e.length).sum::<f64>());
    let whole = (
        signed,
        entries
            .iter()
            .fold(Interval::zero(), |l, edges| l + length(edges)),
    );
    // Without a winding loop the face's own loop is the first, and the
    // others are its holes; with one, the face is a band and every loop
    // that does not wind is a hole, wherever it is stored.
    let band_face = winds.iter().any(|&w| w);
    let holes: Vec<(Interval, Interval)> = entries
        .iter()
        .zip(per_loop)
        .zip(winds)
        .enumerate()
        .filter(|&(i, (_, &winds))| !winds && (band_face || i > 0))
        .map(|(_, ((edges, ring), _))| (ring[1], length(edges)))
        .collect();
    crate::props::curved::require_holes_wound_against("props_chart_area_side", whole, &holes, band)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod swap_rows;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod r1_probes;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::props::bracket::AbsEnclosure;
    use geom_core::Bounds;

    use super::super::run::{LaneEnd, LaneStop, mean_boundary_displacement};
    use crate::props::PropsCheck;
    use crate::props::quad::test_target::{self, met, to_target};
    use geom_core::Tol;

    fn chan(c0: f64, ca: f64, cb: f64, cl: f64) -> HarmChan {
        HarmChan {
            c0: pt(c0),
            ca: pt(ca),
            cb: pt(cb),
            cl: pt(cl),
        }
    }

    /// A full sinusoid band on a unit cylinder: bottom rim v = 0
    /// (u = t), top boundary v(u) = 2 + 0.5·sin u traversed backwards,
    /// closed by the two branches of the seam meridian. The exact
    /// signed UV area is 4π; the engine's converged enclosure must
    /// bracket it tightly, and a dense midpoint sum must sit inside
    /// the enclosure too (the reviewer's independent-oracle shape).
    #[test]
    fn sinusoid_band_area_brackets_the_closed_form() {
        let tau = core::f64::consts::TAU;
        let rim = TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, 1.0),
            v: chan(0.0, 0.0, 0.0, 0.0),
            t0: pt(0.0),
            t1: pt(tau),
            forward: true,
            trig0: (pt(1.0), pt(0.0)),
            trig1: None,
            env: pt(0.0),
        };
        let seam_up = TrimEdgeQ {
            u: chan(tau, 0.0, 0.0, 0.0),
            v: chan(0.0, 0.0, 0.0, 1.0),
            t0: pt(0.0),
            t1: pt(2.0),
            forward: true,
            trig0: (pt(1.0), pt(0.0)),
            trig1: None,
            env: pt(0.0),
        };
        // Top: parameterized by t = u, v = 2 + 0.5 sin t, traversed
        // REVERSED (decreasing u).
        let top = TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, 1.0),
            v: chan(2.0, 0.0, 0.5, 0.0),
            t0: pt(0.0),
            t1: pt(tau),
            forward: false,
            trig0: (pt(1.0), pt(0.0)),
            trig1: None,
            env: pt(0.0),
        };
        let seam_down = TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, 0.0),
            v: chan(0.0, 0.0, 0.0, 1.0),
            t0: pt(0.0),
            t1: pt(2.0),
            forward: false,
            trig0: (pt(1.0), pt(0.0)),
            trig1: None,
            env: pt(0.0),
        };
        let edges = [rim, seam_up, top, seam_down];
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let eps = Tol::witness().get().eps;
        let out = met(cylinder_cut_face::<f64>(
            pt(1.0),
            Interval::zero(),
            &[&edges],
            band,
            &mut to_target(eps),
        ))
        .expect("the band face converges");
        let exact = 2.0 * tau; // −∮ v du = 4π
        assert!(
            out.flux.lo() <= exact && exact <= out.flux.hi(),
            "flux {:?} must bracket {exact}",
            out.flux
        );
        assert!(
            out.area.lo() <= exact && exact <= out.area.hi(),
            "area {:?} must bracket {exact}",
            out.area
        );
        // Independent dense oracle: midpoint sum of Σ σ·∫u·v' dt, the
        // other Green form, equal to the lane's on a loop closed on the
        // chart.
        let mut acc = 0.0;
        let n = 200_000;
        for e in &edges {
            let (a, b) = (e.t0.lo(), e.t1.hi());
            let mut s = 0.0;
            for i in 0..n {
                let t = a + (b - a) * ((f64::from(i) + 0.5) / f64::from(n));
                let u =
                    e.u.c0.lo() + e.u.ca.lo() * t.cos() + e.u.cb.lo() * t.sin() + e.u.cl.lo() * t;
                let vp = e.v.cl.lo() + e.v.cb.lo() * t.cos() - e.v.ca.lo() * t.sin();
                s += u * vp;
            }
            s *= (b - a) / f64::from(n);
            acc += if e.forward { s } else { -s };
        }
        assert!(
            out.flux.lo() <= acc && acc <= out.flux.hi(),
            "dense oracle {acc} escapes the enclosure {:?}",
            out.flux
        );
        // The width cap SCALES FROM THE RESOLVED BAND (the multi-ε
        // lesson): the lane converges to
        // width(flux) ≤ REPORTING_RESOLUTION_FACTOR·ε · 3·lever — assert
        // against the target the lane actually used, not a fixed
        // number (at ε = 1e-6 a ~3e-2 width is a legitimately wider
        // certified enclosure, not a defect).
        let lever = (out.area.lo() + out.area.hi()) * 0.5;
        let width_cap = test_target::REPORTING_RESOLUTION_FACTOR * eps * 3.0 * lever * 1.000001;
        assert!(
            out.flux.width() <= width_cap,
            "width {} exceeds the lane's own converged target {width_cap} (eps {eps})",
            out.flux.width()
        );
    }

    /// The chart rectangle `u ∈ [0, 2]`, `v ∈ [0, 1]`, counter-clockwise,
    /// and the circle of radius `0.3` about `(1, 0.5)` as two half arcs,
    /// clockwise when `hole`, the second arc's azimuth `shift` off.
    fn ringed_rectangle(hole: bool, shift: f64) -> [Vec<TrimEdgeQ>; 2] {
        rectangle_with_circle(hole, shift, (0.3, 0.5))
    }

    /// [`ringed_rectangle`] with a circle of radius `ring` about
    /// `(1, level)`.
    fn rectangle_with_circle(
        hole: bool,
        shift: f64,
        (ring, level): (f64, f64),
    ) -> [Vec<TrimEdgeQ>; 2] {
        let line = |u: HarmChan, v: HarmChan, span: f64, forward: bool| TrimEdgeQ {
            u,
            v,
            t0: pt(0.0),
            t1: pt(span),
            forward,
            trig0: (pt(1.0), pt(0.0)),
            trig1: None,
            env: pt(0.0),
        };
        let outer = vec![
            line(
                chan(0.0, 0.0, 0.0, 1.0),
                chan(0.0, 0.0, 0.0, 0.0),
                2.0,
                true,
            ),
            line(
                chan(2.0, 0.0, 0.0, 0.0),
                chan(0.0, 0.0, 0.0, 1.0),
                1.0,
                true,
            ),
            line(
                chan(0.0, 0.0, 0.0, 1.0),
                chan(1.0, 0.0, 0.0, 0.0),
                2.0,
                false,
            ),
            line(
                chan(0.0, 0.0, 0.0, 0.0),
                chan(0.0, 0.0, 0.0, 1.0),
                1.0,
                false,
            ),
        ];
        let pi = core::f64::consts::PI;
        let arc = |from: f64, du: f64, trig0: (Interval, Interval)| TrimEdgeQ {
            u: chan(1.0 + du, ring, 0.0, 0.0),
            v: chan(level, 0.0, ring, 0.0),
            t0: pt(from),
            t1: pt(from + pi),
            forward: !hole,
            trig0,
            trig1: None,
            env: pt(0.0),
        };
        let mut ring = vec![
            arc(0.0, 0.0, (pt(1.0), pt(0.0))),
            arc(pi, shift, (pt(-1.0), pt(0.0))),
        ];
        if hole {
            ring.reverse();
        }
        [outer, ring]
    }

    /// **A ring the first converged round leaves undecided refines on.**
    /// A small hole far from the Green form's anchor level has a wide
    /// enclosure at round 0, where the face's flux already converges:
    /// its mean width straddles the band there (`props_ring_winding`),
    /// and a window ending at round 0 escalates, while the whole
    /// schedule refines until the ring is decided and measures
    /// `2 − π·r²`, and not the island's `2 + π·r²`.
    ///
    /// The rings are as large as round 0 still leaves straddling (about
    /// ±3e-5 off the anchor level, `2e-5` to `3e-5` here), so the
    /// converged flux enclosure, about `5e-5·r` wide, is narrower than
    /// the `2·π·r²` between the two answers at every ε; and their mean
    /// width, `r/2` (the length is the chart metric's `4·π·r` bound), is
    /// clear of the band's escalate edge `K·ε` wherever the row runs.
    ///
    /// The face's flux must converge at round 0, or the row would read
    /// the open branch's ring check instead, and that is asserted
    /// first, from round 0's own convergence reading. To a reporting
    /// target of ε `1e-9` at least: at ε `1e-12` the face first
    /// converges at round 1, where these rings are already decided.
    #[test]
    fn a_ring_undecided_at_the_first_converged_round_refines_on() {
        let tol = Tol::witness();
        let band = geom_core::Band::linear(tol).unwrap();
        let eps = tol.get().eps;
        let target = eps.max(1e-9);
        // Round 0's convergence reading, as the lane takes it.
        let converges_at_round_0 = |loops: &[&[TrimEdgeQ]]| {
            let chart = RotationalChartQ::Cylinder { radius: pt(1.0) };
            let loops: Vec<Vec<EdgeTermQ>> = loops
                .iter()
                .map(|edges| edges.iter().copied().map(EdgeTermQ::Harmonic).collect())
                .collect();
            let anchor = level_anchor(&loops).unwrap();
            let a_s = loops.iter().flatten().fold(Interval::zero(), |a, e| {
                let entry = edge_entry(e, chart).unwrap();
                a + edge_terms(e, &entry, chart, anchor, EDGE_INIT_PIECES).0[0]
            });
            let bounds = FaceCutBounds {
                flux: a_s,
                area: a_s.abs_enclosure(),
            };
            to_target(target)(&bounds, LaneStop::Refinable).unwrap()
        };
        let rings = [(3e-5, 0.2), (3e-5, 0.95), (2e-5, 0.95)]
            .into_iter()
            .filter(|&(r, _)| r / 2.0 >= 1.5 * tol.k() * eps)
            .collect::<Vec<_>>();
        assert!(!rings.is_empty(), "a row runs at ε {eps}");
        for (r, level) in rings {
            let [outer, ring] = rectangle_with_circle(true, 0.0, (r, level));
            let loops: [&[TrimEdgeQ]; 2] = [&outer, &ring];
            assert!(
                converges_at_round_0(&loops),
                "ring {r} at {level}: the face's flux converges at round 0"
            );
            let at0 = cylinder_cut_face_rounds::<f64>(
                pt(1.0),
                Interval::zero(),
                &loops,
                band,
                RoundWindow::at(0),
                &mut to_target(target),
            );
            assert!(
                matches!(
                    &at0,
                    Err(PropsError::Escalated { cause, .. })
                        if cause.predicate == Some("props_ring_winding")
                            && !cause.terminal_sliver
                ),
                "ring {r} at {level}: round 0's enclosure must straddle the band: {at0:?}"
            );
            let whole = met(cylinder_cut_face::<f64>(
                pt(1.0),
                Interval::zero(),
                &loops,
                band,
                &mut to_target(target),
            ))
            .unwrap_or_else(|e| {
                panic!(
                    "ring {r} at {level}: the schedule refused: {e_at}",
                    e_at = e.render(crate::recourse::Reading::AtRest)
                )
            });
            let pir2 = core::f64::consts::PI * r * r;
            let (hole, island) = (2.0 - pir2, 2.0 + pir2);
            assert!(
                whole.flux.lo() <= hole && hole <= whole.flux.hi() && whole.flux.hi() < island,
                "ring {r} at {level}: flux {:?} must bracket the hole's {hole} and not the island's {island}",
                whole.flux
            );
        }
    }

    /// **A ring wholly inside the band escalates at once**
    /// (`props_ring_winding`), at any window: its enclosure lies inside
    /// one open sliver band ([`geom_core::Indeterminate::terminal_sliver`]),
    /// so its true mean width is there and no round decides it, and the
    /// schedule is not spent on it. A ring of radius `(1 + K)·ε` at the
    /// anchor level has mean width `(1 + K)·ε/2`, inside `(ε, K·ε)` for
    /// any `K > 1`, and its round-0 enclosure is some 0.2% of that wide.
    /// Every window's escalation carries round 0's own enclosure: a later
    /// round's would be about 8× narrower per round.
    #[test]
    fn a_ring_wholly_in_the_band_escalates_at_once() {
        let tol = Tol::witness();
        let band = geom_core::Band::linear(tol).unwrap();
        let eps = tol.get().eps;
        let [outer, ring] = rectangle_with_circle(true, 0.0, ((1.0 + tol.k()) * eps, 0.5));
        let read = |window| match cylinder_cut_face_rounds::<f64>(
            pt(1.0),
            Interval::zero(),
            &[&outer, &ring],
            band,
            window,
            &mut to_target(eps),
        ) {
            Err(PropsError::Escalated { cause, .. })
                if cause.predicate == Some("props_ring_winding") && cause.terminal_sliver =>
            {
                cause.margin
            }
            got => panic!("{window:?}: a ring wholly in the band escalates: {got:?}"),
        };
        let round0 = read(RoundWindow::at(0));
        for window in [RoundWindow { first: 0, last: 2 }, RoundWindow::SCHEDULE] {
            assert_eq!(
                read(window),
                round0,
                "{window:?}: the escalation is round 0's"
            );
        }
    }

    /// **A ring is a loop of its own.** The rectangle less the circle
    /// measures `2 − 0.09π`, the ring's `−∮ v du` entering with its own
    /// clockwise winding.
    #[test]
    fn a_ring_subtracts_its_own_area() {
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let eps = Tol::witness().get().eps;
        let [outer, ring] = ringed_rectangle(true, 0.0);
        let out = met(cylinder_cut_face::<f64>(
            pt(1.0),
            Interval::zero(),
            &[&outer, &ring],
            band,
            &mut to_target(eps),
        ))
        .expect("the ringed rectangle converges");
        let exact = 2.0 - 0.09 * core::f64::consts::PI;
        assert!(
            out.flux.lo() <= exact && exact <= out.flux.hi(),
            "flux {:?} must bracket {exact}",
            out.flux
        );
        assert!(
            out.area.lo() <= exact && exact <= out.area.hi(),
            "area {:?} must bracket {exact}",
            out.area
        );
    }

    /// **A ring wound with its face refuses** (`props_ring_winding`)
    /// rather than adding its area to the face's — at every round a
    /// window can return, not only at convergence: a sign walk settles on
    /// an open round's enclosure, and a ring wound with its face puts the
    /// face's true flux outside it.
    #[test]
    fn a_ring_wound_with_its_face_refuses_at_every_round() {
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let eps = Tol::witness().get().eps;
        let [outer, ring] = ringed_rectangle(false, 0.0);
        let loops: [&[TrimEdgeQ]; 2] = [&outer, &ring];
        for round in 0..4 {
            let got = cylinder_cut_face_rounds::<f64>(
                pt(1.0),
                Interval::zero(),
                &loops,
                band,
                RoundWindow::at(round),
                &mut to_target(eps),
            );
            assert!(
                matches!(got, Err(PropsError::RingWoundWithFace)),
                "round {round}: {got:?}"
            );
        }
        let got = met(cylinder_cut_face::<f64>(
            pt(1.0),
            Interval::zero(),
            &loops,
            band,
            &mut to_target(eps),
        ));
        assert!(
            matches!(got, Err(PropsError::RingWoundWithFace)),
            "the whole schedule: {got:?}"
        );
    }

    /// **A loop stored a period off on the chart measures as the loop
    /// it stores**: the ring's second arc carried `2π` along the
    /// azimuth closes in space and not on the chart, and `−∮ v du` reads
    /// only each edge's own `du`, so the ringed rectangle measures
    /// `2 − 0.09π` exactly as the unshifted one does. A jump at a
    /// junction of a loop that does not wind is no defect.
    #[test]
    fn a_loop_a_period_off_on_the_chart_measures_as_stored() {
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let eps = Tol::witness().get().eps;
        let [outer, ring] = ringed_rectangle(true, core::f64::consts::TAU);
        let out = met(cylinder_cut_face::<f64>(
            pt(1.0),
            Interval::zero(),
            &[&outer, &ring],
            band,
            &mut to_target(eps),
        ))
        .expect("the shifted ring converges");
        let exact = 2.0 - 0.09 * core::f64::consts::PI;
        assert!(
            out.area.lo() <= exact && exact <= out.area.hi(),
            "area {:?} must bracket {exact}",
            out.area
        );
    }

    /// A seamless band of the unit cylinder: a rim circle at `v = lift`
    /// traversed `+u`, and an ellipse rim `v = lift + 2 + k·cos u` (the
    /// wall cut by an oblique plane) traversed `−u` — two loops of one
    /// edge each, each winding the cylinder once.
    fn ellipse_band(k: f64, lift: f64) -> [TrimEdgeQ; 2] {
        let tau = core::f64::consts::TAU;
        let rim = |v: HarmChan, forward| TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, 1.0),
            v,
            t0: pt(0.0),
            t1: pt(tau),
            forward,
            trig0: (pt(1.0), pt(0.0)),
            trig1: Some((pt(1.0), pt(0.0))),
            env: pt(0.0),
        };
        [
            rim(chan(lift, 0.0, 0.0, 0.0), true),
            rim(chan(lift + 2.0, k, 0.0, 0.0), false),
        ]
    }

    /// **A seamless band with an ellipse rim measures**: its loops each
    /// wind the cylinder once, against each other, and the band's area
    /// is `2π·2` whatever the cut's tilt `k` and wherever along the axis
    /// it sits (`lift`) — the oblique cut's mean height is its height on
    /// the axis. `∮ u dv` reads the ellipse rim as `−2πk` and the
    /// circle rim as nothing.
    #[test]
    fn a_seamless_band_with_an_ellipse_rim_measures() {
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let eps = Tol::witness().get().eps;
        let exact = 2.0 * core::f64::consts::TAU;
        for (k, lift) in [(0.0, 0.0), (0.4, 0.0), (-0.7, 0.0), (0.4, 1000.0)] {
            let [bottom, top] = ellipse_band(k, lift);
            let out = met(cylinder_cut_face::<f64>(
                pt(1.0),
                Interval::zero(),
                &[&[bottom], &[top]],
                band,
                &mut to_target(eps),
            ))
            .unwrap_or_else(|e| panic!("k {k}, lift {lift}: the band refuses: {e:?}"));
            for (what, x) in [("flux", out.flux), ("area", out.area)] {
                assert!(
                    x.lo() <= exact && exact <= x.hi(),
                    "k {k}, lift {lift}: {what} {x:?} must bracket {exact}"
                );
            }
        }
    }

    /// The cylinder lane over `loops` at radius 1, at both scalars the
    /// funnel decides at.
    fn at_both_scalars(
        loops: &[&[TrimEdgeQ]],
    ) -> [(&'static str, Result<FaceCutBounds, PropsError>); 2] {
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let eps = Tol::witness().get().eps;
        [
            (
                "f64",
                met(cylinder_cut_face::<f64>(
                    pt(1.0),
                    Interval::zero(),
                    loops,
                    band,
                    &mut to_target(eps),
                )),
            ),
            (
                "Interval",
                met(cylinder_cut_face::<Interval>(
                    pt(1.0),
                    Interval::zero(),
                    loops,
                    band,
                    &mut to_target(eps),
                )),
            ),
        ]
    }

    /// One whole-turn edge at level `v`, its azimuth running `rate` turns
    /// per turn of its parameter.
    fn turning(rate: f64, v: f64, forward: bool) -> TrimEdgeQ {
        TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, rate),
            v: chan(v, 0.0, 0.0, 0.0),
            t0: pt(0.0),
            t1: pt(core::f64::consts::TAU),
            forward,
            trig0: (pt(1.0), pt(0.0)),
            trig1: None,
            env: pt(0.0),
        }
    }

    /// **Loops that wind the cylinder together refuse**
    /// ([`PropsError::LoopsWindChart`]): a band's ellipse rim alone, and
    /// both rims run the same way, each close and wind once, and bound
    /// nothing; `−∮ (v − anchor) du` over them is a number that moves
    /// with the anchor.
    #[test]
    fn loops_that_wind_the_cylinder_together_refuse() {
        let [bottom, top] = ellipse_band(0.4, 0.0);
        let mut same_way = top;
        same_way.forward = true;
        for (what, loops) in [
            ("the ellipse rim alone", vec![&[top][..]]),
            ("both rims run +u", vec![&[bottom][..], &[same_way][..]]),
        ] {
            for (scalar, got) in at_both_scalars(&loops) {
                assert!(
                    matches!(got, Err(PropsError::LoopsWindChart)),
                    "{what} at {scalar}: {got:?}"
                );
            }
        }
    }

    /// **Four whole rims bound no single face**
    /// ([`PropsError::LoopsWindChart`]): wound `+ + − −` or `+ − − +` up
    /// the axis, each turns once and together they turn zero times, but a
    /// connected face has at most two winding loops.
    #[test]
    fn four_winding_rims_bound_no_single_face() {
        for senses in [[true, true, false, false], [true, false, false, true]] {
            let rims: Vec<TrimEdgeQ> = senses
                .iter()
                .enumerate()
                .map(|(i, &up)| turning(1.0, i as f64, up))
                .collect();
            let loops: Vec<&[TrimEdgeQ]> = rims.iter().map(core::slice::from_ref).collect();
            for (scalar, got) in at_both_scalars(&loops) {
                assert!(
                    matches!(got, Err(PropsError::LoopsWindChart)),
                    "{senses:?} at {scalar}: {got:?}"
                );
            }
        }
    }

    /// **A loop that winds twice is no face's boundary**
    /// ([`PropsError::LoopsWindChart`]): two rims whose azimuth runs two
    /// turns, opposite ways, close at their junctions and wind zero
    /// times together, but each crosses itself. Read as a band they would
    /// measure twice its `2π·2`.
    #[test]
    fn a_pair_of_loops_winding_twice_refuses() {
        let (a, b) = (turning(2.0, 0.0, true), turning(2.0, 2.0, false));
        for (scalar, got) in at_both_scalars(&[&[a], &[b]]) {
            assert!(
                matches!(got, Err(PropsError::LoopsWindChart)),
                "at {scalar}: {got:?}"
            );
        }
    }

    /// **A loop that stops short of a whole turn does not close**
    /// ([`PropsError::ChartLoopOpen`]): a rim running one and a half
    /// turns, or 0.4 of one, ends half a turn or 0.4 of one from where it
    /// starts — a gap of `π` or `0.8π` metres at radius 1, against a
    /// slack of `3ε`.
    #[test]
    fn a_loop_that_stops_short_of_a_whole_turn_does_not_close() {
        for (what, loops) in [
            (
                "a pair at 1.5 turns",
                vec![
                    vec![turning(1.5, 0.0, true)],
                    vec![turning(1.5, 2.0, false)],
                ],
            ),
            ("0.4 of a turn", vec![vec![turning(0.4, 0.0, true)]]),
        ] {
            let loops: Vec<&[TrimEdgeQ]> = loops.iter().map(Vec::as_slice).collect();
            for (scalar, got) in at_both_scalars(&loops) {
                assert!(
                    matches!(got, Err(PropsError::ChartLoopOpen)),
                    "{what} at {scalar}: {got:?}"
                );
            }
        }
    }

    /// **A loop open in the level does not close**
    /// ([`PropsError::ChartLoopOpen`]): the rectangle `[0, 2] × [0, 1]`
    /// with its top stored at `v = 6`, five metres above where its sides
    /// end. Its azimuth closes, so no winding reading sees it.
    #[test]
    fn a_rectangle_whose_top_is_off_its_sides_does_not_close() {
        let line = |u: HarmChan, v: HarmChan, span: f64, forward: bool| TrimEdgeQ {
            u,
            v,
            t0: pt(0.0),
            t1: pt(span),
            forward,
            trig0: (pt(1.0), pt(0.0)),
            trig1: None,
            env: pt(0.0),
        };
        let outer = [
            line(
                chan(0.0, 0.0, 0.0, 1.0),
                chan(0.0, 0.0, 0.0, 0.0),
                2.0,
                true,
            ),
            line(
                chan(2.0, 0.0, 0.0, 0.0),
                chan(0.0, 0.0, 0.0, 1.0),
                1.0,
                true,
            ),
            line(
                chan(0.0, 0.0, 0.0, 1.0),
                chan(6.0, 0.0, 0.0, 0.0),
                2.0,
                false,
            ),
            line(
                chan(0.0, 0.0, 0.0, 0.0),
                chan(0.0, 0.0, 0.0, 1.0),
                1.0,
                false,
            ),
        ];
        for (scalar, got) in at_both_scalars(&[&outer]) {
            assert!(
                matches!(got, Err(PropsError::ChartLoopOpen)),
                "at {scalar}: {got:?}"
            );
        }
    }

    /// The rectangle `[0, 2] × [0, 1]` whose sides end where `v` reads
    /// through `side_v` and whose top is stored at `1 + top_off`.
    fn rectangle_with(
        side_v: HarmChan,
        side_trig0: (Interval, Interval),
        top_off: f64,
    ) -> [TrimEdgeQ; 4] {
        let line = |u: HarmChan, v: HarmChan, span: f64, forward: bool| TrimEdgeQ {
            u,
            v,
            t0: pt(0.0),
            t1: pt(span),
            forward,
            trig0: (pt(1.0), pt(0.0)),
            trig1: None,
            env: pt(0.0),
        };
        let side = |u0: f64, forward: bool| TrimEdgeQ {
            u: chan(u0, 0.0, 0.0, 0.0),
            v: side_v,
            t0: pt(0.0),
            t1: pt(1.0),
            forward,
            trig0: side_trig0,
            trig1: None,
            env: pt(0.0),
        };
        [
            line(
                chan(0.0, 0.0, 0.0, 1.0),
                chan(0.0, 0.0, 0.0, 0.0),
                2.0,
                true,
            ),
            side(2.0, true),
            line(
                chan(0.0, 0.0, 0.0, 1.0),
                chan(1.0 + top_off, 0.0, 0.0, 0.0),
                2.0,
                false,
            ),
            side(0.0, false),
        ]
    }

    /// **A junction its brackets cannot decide is never measured**
    /// (review of PR 4839, the verifier's probe). Each side's level is
    /// read through a whole-circle trig bracket, `v = −a + a·cos t + c·t`
    /// with `v(1) = 1` exactly, so its end is known only to about `2a`;
    /// the top is stored `0.45·a` above it. Read off the gap's lower
    /// bound the junction looked closed and the face measured `2 + 2g`.
    /// The gap's upper bound is beyond the slack and its lower bound is
    /// not, so it escalates — at `a = 10³·ε` (the verifier's `450ε`
    /// opening) and `a = 1e-3`, at `f64` and at `Interval`.
    #[test]
    fn a_junction_its_brackets_cannot_decide_is_never_measured() {
        let whole = (
            Interval::from_bounds(-1.0, 1.0),
            Interval::from_bounds(-1.0, 1.0),
        );
        let eps = Tol::witness().get().eps;
        for a in [1e3 * eps, 1e-3] {
            let c = 1.0 - a * (1.0_f64.cos() - 1.0);
            let outer = rectangle_with(chan(-a, a, 0.0, c), whole, 0.45 * a);
            for (scalar, got) in at_both_scalars(&[&outer]) {
                assert!(
                    matches!(
                        got,
                        Err(PropsError::Escalated {
                            check: PropsCheck::Inventory,
                            ..
                        })
                    ),
                    "a {a:e} at {scalar}: an undecidable junction escalates: {got:?}"
                );
            }
        }
    }

    /// **A junction open by the vertex tolerance closes**
    /// ([`crate::props::closure::VERTEX_SLACK`]).
    /// The rectangle with its top stored `2ε` above its sides' ends, every
    /// bracket thin: the gap is the two carriers' ends about their
    /// vertex, which the vertex term allows, so the face measures. Without
    /// that term the excess is `2ε`, in the band, and it escalates.
    #[test]
    fn a_junction_open_by_the_vertex_tolerance_closes() {
        let eps = Tol::witness().get().eps;
        let outer = rectangle_with(chan(0.0, 0.0, 0.0, 1.0), (pt(1.0), pt(0.0)), 2.0 * eps);
        for (scalar, got) in at_both_scalars(&[&outer]) {
            let out = got.unwrap_or_else(|e| panic!("at {scalar}: the face refuses: {e:?}"));
            let exact = 2.0 * 2.0_f64.mul_add(eps, 1.0);
            assert!(
                out.area.lo() <= exact && exact <= out.area.hi(),
                "at {scalar}: area {:?} must bracket {exact}",
                out.area
            );
        }
    }

    /// **A band reads its holes by winding, in any loop order.** The band
    /// between the rim `v = 0` and the cut `v = 3 + 0.5·cos u`, with a
    /// circular hole of radius 0.4: wound against the face it measures
    /// `3·2π − 0.16π` stored first, middle or last; wound with it, it
    /// refuses ([`PropsError::RingWoundWithFace`]) wherever it is stored
    /// — read by position, a hole stored first is the face's own loop
    /// and is added.
    #[test]
    fn a_band_reads_its_holes_by_winding_in_any_loop_order() {
        let tau = core::f64::consts::TAU;
        let rim = |v: HarmChan, forward| TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, 1.0),
            v,
            t0: pt(0.0),
            t1: pt(tau),
            forward,
            trig0: (pt(1.0), pt(0.0)),
            trig1: None,
            env: pt(0.0),
        };
        let hole = |forward| TrimEdgeQ {
            u: chan(2.0, 0.4, 0.0, 0.0),
            v: chan(1.2, 0.0, 0.4, 0.0),
            t0: pt(0.0),
            t1: pt(tau),
            forward,
            trig0: (pt(1.0), pt(0.0)),
            trig1: None,
            env: pt(0.0),
        };
        let bottom = [rim(chan(0.0, 0.0, 0.0, 0.0), true)];
        let top = [rim(chan(3.0, 0.5, 0.0, 0.0), false)];
        let (against, with) = ([hole(false)], [hole(true)]);
        let exact = 3.0 * tau - core::f64::consts::PI * 0.16;
        for (order, (h, b, t)) in [(0, 1, 2), (1, 0, 2), (2, 1, 0)].into_iter().enumerate() {
            let place = |ring: &'static str| {
                let mut loops: [&[TrimEdgeQ]; 3] = [&[], &[], &[]];
                loops[b] = &bottom;
                loops[t] = &top;
                loops[h] = if ring == "against" { &against } else { &with };
                loops
            };
            for (scalar, got) in at_both_scalars(&place("against")) {
                let out = got.unwrap_or_else(|e| panic!("order {order} at {scalar}: {e:?}"));
                assert!(
                    out.area.lo() <= exact && exact <= out.area.hi(),
                    "order {order} at {scalar}: area {:?} must bracket {exact}",
                    out.area
                );
            }
            for (scalar, got) in at_both_scalars(&place("with")) {
                assert!(
                    matches!(got, Err(PropsError::RingWoundWithFace)),
                    "order {order} at {scalar}, the hole wound with the face: {got:?}"
                );
            }
        }
    }

    /// Endpoint ignorance that no refinement narrows stops the lane at
    /// its floor within three rounds: the endpoint pad, a whole radian of
    /// `|f|`, already dwarfs round 0's remainder, which falls 8× a round
    /// to under 2⁻¹⁰ of the width by round 2, so the lane never returns a
    /// silently wide bracket and never refines toward a target it cannot
    /// reach. The loop
    /// is the unit circle of the chart, `(u, v) = (cos t, sin t)`,
    /// closed; its start bracket is a whole radian wide.
    #[test]
    fn hopeless_endpoint_brackets_stop_at_the_floor() {
        let e = TrimEdgeQ {
            u: chan(0.0, 1.0, 0.0, 0.0),
            v: chan(0.0, 0.0, 1.0, 0.0),
            t0: Interval::from_bounds(-0.5, 0.5),
            t1: pt(core::f64::consts::TAU),
            forward: true,
            trig0: (pt(1.0), pt(0.0)),
            trig1: None,
            env: pt(0.0),
        };
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let eps = Tol::witness().get().eps;
        let run = cylinder_cut_face::<f64>(
            pt(1.0),
            Interval::zero(),
            &[&[e]],
            band,
            &mut to_target(eps),
        )
        .expect("the hopeless loop has an enclosure");
        assert_eq!(
            (run.end, run.round, run.rounds),
            (LaneEnd::Floor, 2, 3),
            "the endpoint pad is a floor round 2 sits on: {run:?}"
        );
    }

    /// Review MIN-2 pin (adopted): the endpoint-bracket pad's factor-2
    /// absorbs the trig0 PHASE-SHIFT term, not just the limit
    /// mismatch. The scan places the TRUE endpoints at the corners and
    /// edges of width-4e-3 brackets (trig0 anchored at the true t₀,
    /// integration limits at the bracket midpoints) and asserts the
    /// engine's enclosure contains a dense Kahan-summed oracle of the
    /// true integral at every placement.
    #[test]
    fn endpoint_bracket_phase_shift_corner_scan() {
        let w = 4e-3;
        // (fraction of w for true t0 off mid, same for t1): 4 corners,
        // 2 single-edge, center — 7 placements.
        let placements = [
            (-0.5, -0.5),
            (-0.5, 0.5),
            (0.5, -0.5),
            (0.5, 0.5),
            (-0.5, 0.0),
            (0.0, 0.5),
            (0.0, 0.0),
        ];
        // A strong linear channel against trig channels (the
        // phase-shift term is c_l-shaped).
        let u = chan(0.3, 0.0, 0.0, 1.0);
        let v = chan(2.0, 0.7, 0.4, 0.3);
        let (mid0, mid1) = (0.2, 5.9);
        for (d0, d1) in placements {
            let t0_true = mid0 + d0 * w;
            let t1_true = mid1 + d1 * w;
            let e = TrimEdgeQ {
                u,
                v,
                t0: Interval::from_bounds(mid0 - w * 0.5, mid0 + w * 0.5),
                t1: Interval::from_bounds(mid1 - w * 0.5, mid1 + w * 0.5),
                forward: true,
                trig0: (pt(t0_true.cos()), pt(t0_true.sin())),
                trig1: None,
                env: pt(0.0),
            };
            let (enclosure, _) = harmonic_edge_integral(&e, 0.0, 65_536);
            // Dense Kahan-summed midpoint oracle of the TRUE integral,
            // `∫ −v·u' dt`.
            let minus_v = chan(-2.0, -0.7, -0.4, -0.3);
            let truth = kahan_dense_oracle(&minus_v, &u, t0_true, t1_true, 400_000);
            assert!(
                enclosure.lo() <= truth && truth <= enclosure.hi(),
                "placement ({d0}, {d1}): truth {truth} escapes {enclosure:?} — the \
                 endpoint pad's two-term accounting failed"
            );
        }
    }

    /// Dense midpoint oracle of `∫ u·v' dt` with Kahan (compensated)
    /// summation — the reviewer's independent-integration shape.
    fn kahan_dense_oracle(u: &HarmChan, v: &HarmChan, a: f64, b: f64, n: u32) -> f64 {
        let (mut sum, mut c) = (0.0f64, 0.0f64);
        let h = (b - a) / f64::from(n);
        for i in 0..n {
            let t = a + h * (f64::from(i) + 0.5);
            let uu = u.c0.lo() + u.ca.lo() * t.cos() + u.cb.lo() * t.sin() + u.cl.lo() * t;
            let vp = v.cl.lo() + v.cb.lo() * t.cos() - v.ca.lo() * t.sin();
            let term = uu * vp * h - c;
            let s2 = sum + term;
            c = (s2 - sum) - term;
            sum = s2;
        }
        sum
    }

    /// Review probe family (adopted): three adversarial faces against
    /// dense Kahan-summed oracles — a near-pinching sinusoid band, a
    /// thin sliver band, and a mixed forward/reversed loop with large
    /// linear channels. Bound below truth anywhere = automatic MAJOR.
    #[test]
    fn adversarial_faces_contain_their_dense_oracles() {
        let tau = core::f64::consts::TAU;
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let eps = Tol::witness().get().eps;
        let seam = |u_at: f64, v_hi: f64, forward: bool| TrimEdgeQ {
            u: chan(u_at, 0.0, 0.0, 0.0),
            v: chan(0.0, 0.0, 0.0, 1.0),
            t0: pt(0.0),
            t1: pt(v_hi),
            forward,
            trig0: (pt(1.0), pt(0.0)),
            trig1: None,
            env: pt(0.0),
        };
        let rim = |forward: bool| TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, 1.0),
            v: chan(0.0, 0.0, 0.0, 0.0),
            t0: pt(0.0),
            t1: pt(tau),
            forward,
            trig0: (pt(1.0), pt(0.0)),
            trig1: None,
            env: pt(0.0),
        };
        // (a) near-pinching band: v(u) = 1 + 0.999·sin u (dips to 1e-3).
        let pinch_top = TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, 1.0),
            v: chan(1.0, 0.0, 0.999, 0.0),
            t0: pt(0.0),
            t1: pt(tau),
            forward: false,
            trig0: (pt(1.0), pt(0.0)),
            trig1: None,
            env: pt(0.0),
        };
        let face_a = vec![
            rim(true),
            seam(tau, 1.0, true),
            pinch_top,
            seam(0.0, 1.0, false),
        ];
        // (b) thin sliver: bottom v = 1 + 0.5·cos u, top 1e-3 above it.
        let sliver_bot = TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, 1.0),
            v: chan(1.0, 0.5, 0.0, 0.0),
            t0: pt(0.0),
            t1: pt(tau),
            forward: true,
            trig0: (pt(1.0), pt(0.0)),
            trig1: None,
            env: pt(0.0),
        };
        let sliver_top = TrimEdgeQ {
            v: chan(1.0 + 1e-3, 0.5, 0.0, 0.0),
            forward: false,
            ..sliver_bot
        };
        // Both seams run v = 1.5 + 1e-3·t over t ∈ [0, 1] (the sliver
        // gap at u = 0 ≡ 2π); traversal direction alone distinguishes.
        let sliver_seam = |u_at: f64, forward: bool| TrimEdgeQ {
            u: chan(u_at, 0.0, 0.0, 0.0),
            v: chan(1.5, 0.0, 0.0, 1e-3),
            t0: pt(0.0),
            t1: pt(1.0),
            forward,
            trig0: (pt(1.0), pt(0.0)),
            trig1: None,
            env: pt(0.0),
        };
        let face_b = vec![
            sliver_bot,
            sliver_seam(tau, true),
            sliver_top,
            sliver_seam(0.0, false),
        ];
        // (c) mixed traversal with large linear channels: a skewed
        // band whose top runs v = 3 + 0.8·t (t = u) reversed.
        let skew_top = TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, 1.0),
            v: chan(3.0, 0.4, 0.0, 0.8),
            t0: pt(0.0),
            t1: pt(tau),
            forward: false,
            trig0: (pt(1.0), pt(0.0)),
            trig1: None,
            env: pt(0.0),
        };
        let seam_c = |u_at: f64, v_hi: f64, forward: bool| TrimEdgeQ {
            u: chan(u_at, 0.0, 0.0, 0.0),
            v: chan(0.0, 0.0, 0.0, 1.0),
            t0: pt(0.0),
            t1: pt(v_hi),
            forward,
            trig0: (pt(1.0), pt(0.0)),
            trig1: None,
            env: pt(0.0),
        };
        let face_c = vec![
            rim(true),
            seam_c(tau, 3.4 + 0.8 * tau, true),
            skew_top,
            seam_c(0.0, 3.4, false),
        ];
        for (label, edges) in [("pinch", face_a), ("sliver", face_b), ("skew", face_c)] {
            // Independent truth: Σ σ·∫ u·v' dt, Kahan-summed — the
            // other Green form, equal to the lane's on a loop closed on
            // the chart.
            let mut truth = 0.0;
            for e in &edges {
                let s = kahan_dense_oracle(&e.u, &e.v, e.t0.lo(), e.t1.hi(), 400_000);
                truth += if e.forward { s } else { -s };
            }
            // Two honest outcomes, ε-dependent: an accepted enclosure
            // MUST contain the oracle; at tight ε (1e-12 drives the
            // target to ~1e-9 m while the sliver's ring-arithmetic floor
            // sits above it) the lane stops short at its own floor or
            // wall — with an enclosure that still contains the oracle and
            // really missed its target, never a silently wide bracket,
            // never a silent skip.
            let target = test_target::REPORTING_RESOLUTION_FACTOR * eps;
            match cylinder_cut_face::<f64>(
                pt(1.0),
                Interval::zero(),
                &[&edges],
                band,
                &mut to_target(eps),
            ) {
                Ok(run) => {
                    let out = run.bounds;
                    assert!(
                        out.flux.lo() <= truth && truth <= out.flux.hi(),
                        "{label}: dense oracle {truth} escapes the enclosure {:?} ({run:?})",
                        out.flux
                    );
                    if !matches!(run.end, LaneEnd::Accepted(_)) {
                        let width_len = mean_boundary_displacement(out.flux, out.area).unwrap();
                        assert!(
                            width_len > target,
                            "{label}: a lane stopped short must report an ACHIEVED width \
                             above its target, got {width_len} vs {target} ({run:?})"
                        );
                    }
                }
                Err(other) => panic!("{label}: outside the two honest arms: {other:?}"),
            }
        }
    }
}
