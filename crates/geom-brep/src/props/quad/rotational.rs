//! **The rotational-chart Green lane** ([`rotational_cut_face_rounds`]):
//! the divergence-theorem contributions of a face on a chart of
//! revolution whose trim loops are curved, and the loop windings and
//! ring readings the face rests on. The chart table
//! ([`RotationalChartQ`]) holds the cylinder and the sphere.
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
//! The form is read along the closed **image chain**: each loop's
//! images, and at each junction the connector between the edge before's
//! end and the edge after's start, which the closure accepts up to its
//! slack apart ([`connector_terms`]).
//!
//! # The sphere
//!
//! On the sphere `S(u, v) = o + r·(cos v·(û·cos u + v̂·sin u) + â·sin v)`,
//! `v` the latitude, `dA = r²·cos v du dv` and the normal is radial, so
//! `F(v) = r³·sin v` and `G(v) = r²·sin v`, with no anchor: `sin v` is
//! `z/r` at the point, which every name of it shares (the twin
//! `(u + π, π − v)` included). The form `−G du` is singular at the poles,
//! where `du` is, so the chain's form misses the region by whole poles'
//! terms: `|Ω| = s_f·A₀ + 2πr²·q`, `A₀ = −Σ∮G du` over the chain, `s_f`
//! the face's sense (`+1` where its normal is the outward one), and `q`
//! the poles the region holds as the chain reads them, with
//! `q ≡ W (mod 2)`, `W` the face's total azimuth winding. A loop on a
//! sphere bounds a region on each side, its face on its left under the
//! face's normal, so the sense is what says which: the loops alone
//! cannot. Where a pole is a vertex of the face, as every pole in a
//! face's closure is (DESIGN `[topology.tiers]`), the junction there is
//! the chart's pole edge and its connector carries the pole's share,
//! whatever whole turns the closure took off it; `q` absorbs the rest.
//! `|Ω|` lies in `(0, 4πr²]`, a window two of `q`'s steps wide, so it
//! fixes `q`: per loop, the area outside it is read in `[0, 4πr²)`
//! ([`sphere_outside`]), the face's is the sphere less those, and the
//! reading is decided ([`require_sphere_sides`]). The flux about the
//! centre is then `r·s_f·|Ω|`.
//!
//! # The torus
//!
//! On the ring torus `S(u, v) = o + (R + r·cos v)·(û·cos u + v̂·sin u) +
//! â·r·sin v`, `v` the tube angle, `0 < r < R`, `dA = r·ρ du dv` with
//! `ρ = R + r·cos v`, and the flux density about the centre is
//! `R·cos v + r`, so
//!
//! ```text
//! F(v) = r·((R² + r²)·sin v + ½Rr·sin v·cos v) + κ_F·v,   κ_F = 3/2·R·r²
//! G(v) = r²·sin v + κ_G·v,                                  κ_G = R·r
//! ```
//!
//! Each is a periodic part, read off `(cos v, sin v)`, plus a slope
//! times `v` itself. On a projected chord the frame point gives the
//! periodic part in the ring operations and one root,
//! `(cos v, sin v) = (ρ − R, z)/|(ρ − R, z)|`, but `v` is an `atan2`,
//! which certification arithmetic does not evaluate (C9). The lane reads
//! it as `v(t) = v(t₀) + ∫v′`, `v′ = ((ρ − R)·z′ − z·ρ′)/((ρ − R)² +
//! z²)` algebraic, so `∫v·u′` is an iterated integral of algebraic
//! functions ([`crate::props::taylor::primitive_each`]); `v(t₀)` is the
//! image's own level at the carrier interval's start, read by its
//! evaluator at the lane scalar and crossed in through
//! `Interval::from_certified`, the door every edge's ends already take
//! into the closure and the connectors. A rim reads `F(V)` at its level,
//! and a spiric's exact image `v = v₀ + σ·t` reads `v` as a polynomial
//! in its own parameter.
//!
//! Both chart coordinates are angles, so `v du` is not a global form on
//! the torus, and `F(v + 2π) = F(v) + κ_F·2π`. The lane reads each loop
//! on the chart's continuous lift: every edge on its own lift (its level
//! from its own start), carried by the whole turns `o` the chain's level
//! sits above it, `−o·κ·2π·Δu` ([`LoopLift`]), the chain gaining each
//! edge's branch turns and each junction's whole level turns as the
//! closure takes them off. A loop that winds the tube `W_v` times
//! crosses the seam its lift opens: cut along a path from it to its
//! partner, the region's lifted boundary carries that path twice, a
//! turn apart, `κ·2π·(u_B − u_A)`, which per loop is `W_v·κ·2π·u₀` at the
//! azimuth `u₀` its chain starts at, independent of where that is
//! (moving the start along the loop changes the chain's integral by just
//! as much the other way). A whole turn of a level or of an azimuth
//! anywhere in this moves the area by a whole torus `4π²Rr` and the flux
//! by the whole torus's `6π²Rr²` together. A loop that winds nothing
//! bounds a region on each side, the face's under its sense `s_f`, so
//! the face is read off its loops' signed area under the sense, carried
//! into `(0, 4π²Rr]` by whole tori ([`torus_face`]): the face is a region
//! of the torus, and the window holds the whole torus, so a face bounded
//! by its chart's wrap loop reads whole. The reading holds every whole
//! count of tori its enclosure admits, so it decides nothing and a later
//! round narrows it. The loops that wind the torus are at most two, wound
//! against each other ([`loop_windings`]), and its rings are read against
//! the face as the cylinder's are ([`require_holes_wound_against`]: every
//! loop that winds nothing but the first is a hole, and with a winding
//! pair every one is). Under a flipped bit the lane certifies the complement, as on
//! the sphere.
//!
//! `v du` because it is a global 1-form on the chart (the azimuth is
//! periodic, its differential is not), so the form reads a loop that
//! winds the chart — a seamless band's rim — and a loop whose images
//! sit in different branches of the chart alike; `u dv` reads neither.
//! On the cylinder the loops' own traversal signs carry `s_f`, a ring's
//! opposite to its face's, so no separate orientation classification is
//! needed; what is decided is that the stored loops say so (every
//! junction closes, a whole number of turns apart in the azimuth; each
//! loop winds at most once and the face's loops none together, which
//! makes the form independent of the anchor level `v_a`; every loop that
//! does not wind, other than a non-band face's own, is wound against its
//! face). The sphere reads `s_f` off the face's sense bit (below), which
//! the loops cannot carry, and nothing here checks the bit against them:
//! under a flipped bit the lane certifies the complement, a valid face
//! of its own, and tier 3 refuses the body that holds it.
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
//!   The sphere's harmonic images are rims and meridians, read so; any
//!   other refuses.
//! - **Projected** ([`ProjectedEdgeQ`]): the chart's inverse applied to
//!   the carrier written in the chart's frame (`Pcurve::Projected`), a
//!   rational net ([`FramedQ::Net`]), a circle's or an ellipse's harmonic
//!   form ([`FramedQ::Conic`]) or a line ([`FramedQ::Line`]).
//!   At the frame point `(x, y, z)` the azimuth's rate is
//!   `u′ = (x·y′ − y·x′)/(x² + y²)`, which reads no branch, and the
//!   cylinder's level is `v = v_off + v_sign·z` (the image's deck map).
//!   On a net `P = B/w`, `u′ = (B_x·B_y′ − B_y·B_x′)/(B_x² + B_y²)` (the
//!   weight cancels) and `z = B_z/w`. Both read through the curved rims'
//!   Taylor rule ([`crate::props::taylor`]; a net span by span,
//!   [`crate::props::rational_rim`]), with every span, and the conic's
//!   interval, cut at the image's sector breaks. On the cylinder those
//!   cuts guard no soundness: a break is where the image's evaluator
//!   changes branch, and the rule's integrand reads no branch, so it is
//!   as smooth across a break as anywhere; its one singularity, the
//!   chart's axis (`x² + y² = 0`), is caught where the reciprocal's
//!   enclosure refuses, at any cut. The cuts align the rule's pieces with
//!   the image's, and nothing reads them otherwise. The edge's ends are the
//!   image's own, read at the lane scalar; its turn `Δu = ∫u′` is the
//!   rule's, and the closure reads the two against each other, so an
//!   image whose evaluator reads a branch its integral does not is
//!   caught rather than integrated through. Each span or interval is
//!   read in the pieces its own turn needs ([`span_schedule`]),
//!   doubling per round, so a chord of many short spans reads one piece
//!   a span and a span by the axis as many as its turn needs.
//!
//! # The harmonic integrand
//!
//! Trigonometric values enter only as **enclosures propagated from the
//! edge endpoints**: the caller supplies `(cos t₀, sin t₀)` brackets at
//! the limits the lane integrates between
//! ([`trig_at_limit`](crate::props::trig::trig_at_limit), rotated from the
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
//! Each pad is added symmetrically. Their premise is the pcurve
//! certificate's: each image tracks its carrier within its envelope `e`
//! (metres through the map) at every parameter, and the face's true
//! trim boundary lies within metric distance `e` of its images, the
//! chart's metric `ds² = r²du² + dv²` on the cylinder,
//! `r²·(cos²v du² + dv²)` on the sphere and `ρ²du² + r²dv²` on the
//! torus.
//!
//! - **Map residual**: the chart-area defect between the images' region
//!   and the true one is at most `Σ (metric edge length)·e`, and the flux
//!   density about the chart origin is at most `r` per unit area on the
//!   cylinder and the sphere and `R + r` on the torus, so that times it
//!   in flux. Beside each junction's connector lies the
//!   region between it and the true vertex, `2·g·(e_i + e_j + g)`
//!   ([`connector_terms`]).
//! - **Endpoint parameter brackets**: a non-thin `t₀/t₁` bracket of a
//!   harmonic edge contributes `mag(f)·width` at each end. A projected
//!   edge's carrier interval is exact.
//! - **The chart term**: `A⃗` is the carriers' vector area, and the face
//!   is the region its pcurves bound, so `o·A⃗` is padded by `|o|` times
//!   the ribbons between them — each carrier to its image,
//!   `½·e·(L_carrier + L_image)`, and each image to the true boundary,
//!   `e·L_image`, plus the junctions' connectors ([`chart_term_pad`]).
//!   The map-residual pad's `r` is the flux density about the chart
//!   origin; this one is the lever's, and grows with the face's distance
//!   from the point the flux is about.
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
use crate::props::PropsError;
use crate::props::bracket::{AbsEnclosure, exact_point, mid, pt};
use crate::props::rational_rim::{
    RationalEdgeQ, RationalJets, rational_integrals_each, rational_primitive_each,
    rational_rate_bound, rational_span_pieces, rational_vector_area_each,
};
use crate::props::spiric::{SpiricEdgeQ, WallImageQ, spiric_length_bound};
use crate::props::taylor::{
    Integrand, Jet, Primitive, W_SPIRIC, composite_each, pieces_where, primitive_each,
    rate_bound_over, trig_jets, trig_of, w_rational,
};
use crate::props::trig::{limit, rotate, spread_cos, spread_sin, trig_at, trig_over};

/// Weight of one harmonic piece of a cylinder edge, in [`FACE_WORK_WALL`](super::run::FACE_WORK_WALL)'s unit.
const W_CYLINDER: usize = 3;

/// The pieces a harmonic edge reads at `round`: [`EDGE_INIT_PIECES`],
/// doubling per round.
fn round_pieces(round: usize) -> usize {
    EDGE_INIT_PIECES << round
}

/// The work of round `round` on a face of `edges` harmonic trim edges.
#[cfg(test)]
pub(super) fn round_cost(edges: usize, round: usize) -> usize {
    W_CYLINDER * edges * round_pieces(round)
}

/// The work of round `round` on `face`: a harmonic edge's pieces at
/// [`W_CYLINDER`], a projected carrier's at its piece weight
/// ([`framed_weight`]) and a rational chart term's at a rational rim's,
/// each projected or rational carrier at the pieces its [`Schedule`]
/// reads, and a spiric edge at [`W_SPIRIC`] a round piece, its chart
/// term counted with it.
fn face_cost(
    face: &RotationalFaceQ,
    entries: &[Vec<EdgeEntry>],
    terms: &[Schedule],
    round: usize,
) -> usize {
    let edges: usize = face
        .loops
        .iter()
        .flatten()
        .zip(entries.iter().flatten())
        .map(|(e, entry)| match e {
            EdgeTermQ::Harmonic(_) => W_CYLINDER * round_pieces(round),
            EdgeTermQ::Projected(p) => framed_weight(&p.carrier) * entry.schedule.pieces(round),
            EdgeTermQ::Spiric(_) => W_SPIRIC * round_pieces(round),
        })
        .sum();
    let rationals: usize = face
        .chart_term
        .rationals
        .iter()
        .zip(terms)
        .map(|(e, s)| w_rational(e.degree()) * s.pieces(round))
        .sum();
    edges + rationals
}

/// One piece's weight on a projected carrier: a net's at a rational
/// rim's ([`w_rational`]: the same piece forms, read by Horner), a
/// conic's at a spiric's ([`W_SPIRIC`]: the same trig jets and one
/// reciprocal), and a line's at a degree-1 rim's.
fn framed_weight(carrier: &FramedQ) -> usize {
    match carrier {
        FramedQ::Net(net) => w_rational(net.degree()),
        FramedQ::Conic { .. } => W_SPIRIC,
        FramedQ::Line { .. } => w_rational(1),
    }
}

/// **A carrier's schedule**: its pieces per span or cut interval at
/// round 0, each read off that span's own geometry ([`span_schedule`],
/// [`term_schedule`]), doubling per round. A span's count is a function
/// of the span alone, so a boundary reads the same pieces however it is
/// cut into edges.
#[derive(Clone, Debug, Default)]
struct Schedule(Vec<usize>);

impl Schedule {
    /// The pieces span `i` reads at `round`.
    fn at(&self, round: usize) -> impl Fn(usize) -> usize + '_ {
        move |i| self.0[i] << round
    }

    /// Every span's pieces at `round`, together.
    fn pieces(&self, round: usize) -> usize {
        self.0.iter().sum::<usize>() << round
    }
}

/// The most pieces a span or cut interval is read in by the entry: its
/// schedule's round 0 ([`Schedule`]) and its turn ([`entry_turn`]).
const MOST_SPAN_PIECES: usize = 1024;

/// **The azimuth one round-0 piece may turn through**: a sixteenth of a
/// turn, what a full-turn harmonic rim's round-0 piece turns
/// ([`EDGE_INIT_PIECES`]).
#[allow(clippy::cast_precision_loss)]
const PIECE_TURN: f64 = core::f64::consts::TAU / EDGE_INIT_PIECES as f64;

/// **A projected edge's schedule**: per span or cut interval, the fewest
/// pieces ([`pieces_where`], at most [`MOST_SPAN_PIECES`]) on each of
/// which the azimuth turns through at most [`PIECE_TURN`], read as
/// `sup|u′|·h` over the piece. The azimuth rate's reciprocal reads the
/// chart's axis or a pole: a piece that sweeps past it reads no rate
/// (and fits nowhere), and one that turns fast by it reads a wide
/// remainder, so a span by the axis takes the pieces its turn needs, and
/// a short span far from it one.
///
/// On the torus the level is an angle too, and a piece fits only where
/// the tube angle turns through at most [`PIECE_TURN`] as well: a
/// meridian's azimuth stands still while its level sweeps the tube.
///
/// **No piece a round reads turns through more than [`PIECE_TURN`]**,
/// its pieces only halving round by round, unless its span is still not
/// read so at [`MOST_SPAN_PIECES`], where the rounds read it as they can
/// and refuse what they cannot.
fn span_schedule(p: &ProjectedEdgeQ, chart: RotationalChartQ) -> Schedule {
    let fits = |at: &FrameJets, h: f64| {
        at.du.0[0].mag() * h <= PIECE_TURN
            && match chart {
                RotationalChartQ::Torus { major, .. } => {
                    tube_jets(major, at).rate.0[0].mag() * h <= PIECE_TURN
                }
                _ => true,
            }
    };
    Schedule(match &p.carrier {
        FramedQ::Net(net) => {
            rational_span_pieces(net, MOST_SPAN_PIECES, |j, h| fits(&net_rate(j), h))
        }
        FramedQ::Conic { cuts, .. } | FramedQ::Line { cuts, .. } => cuts
            .windows(2)
            .map(|w| {
                pieces_where(
                    (w[0], w[1]),
                    MOST_SPAN_PIECES,
                    |_, _| (),
                    |(), t, h| fits(&analytic_rate(&p.carrier, t), h),
                )
            })
            .collect(),
    })
}

/// **The length one round-0 piece of a rational chart term may span**:
/// a sixteenth of the chart's widest circumference, what a full-turn harmonic
/// rim's round-0 piece spans ([`EDGE_INIT_PIECES`]).
fn piece_reach(chart: RotationalChartQ) -> f64 {
    let radius = match chart {
        RotationalChartQ::Cylinder { radius } | RotationalChartQ::Sphere { radius, .. } => radius,
        RotationalChartQ::Torus { major, minor, .. } => major + minor,
    };
    radius.hi() * PIECE_TURN
}

/// **A rational chart term's schedule**: per span, the fewest pieces on
/// each of which the carrier `B/w` certifies and its length,
/// `Σ|B′w − Bw′|/w²` read over the piece, is at most `reach`.
fn term_schedule(e: &RationalEdgeQ, reach: f64) -> Schedule {
    Schedule(rational_span_pieces(e, MOST_SPAN_PIECES, |j, h| {
        let (b, db) = (j.b, j.db);
        let (w0, dw) = (j.w.0[0], j.w.0[1]);
        let speed = (0..3).fold(Interval::zero(), |acc, c| {
            acc + ((db[c].0[0] * w0 - b[c].0[0] * dw) / w0.sqr()).abs_enclosure()
        });
        speed.is_certified() && speed.hi() * h <= reach
    }))
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
    /// [`trig_at_limit`](crate::props::trig::trig_at_limit)).
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
    /// A circle or an ellipse `centre + a·cos t + b·sin t`, read over `cuts`: the
    /// edge's carrier interval, its ends and the image's sector breaks
    /// inside it, increasing, each an exact parameter.
    Conic {
        /// The centre.
        centre: RVec3,
        /// The `cos t` coefficient.
        a: RVec3,
        /// The `sin t` coefficient.
        b: RVec3,
        /// The interval's ends and the breaks between them.
        cuts: Vec<f64>,
    },
    /// A line `point + dir·t`, read over `cuts` as a conic is.
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
    /// A spiric's exact image on its own torus.
    Spiric(SpiricWallQ),
}

/// One spiric edge of a torus wall, bracketed by the caller: its carrier
/// about the torus centre, its exact lifted image (`u = u₀ + σ·atan2(f,
/// d)`, `v = v₀ + σ·t`, [`crate::SpiricImage::Wall`] on the carrier's own
/// torus), and the image's chart points at the carrier interval's start
/// and its end, read by its evaluator.
#[derive(Clone, Copy, Debug)]
pub struct SpiricWallQ {
    /// The carrier, its centre about the torus centre.
    pub edge: SpiricEdgeQ,
    /// The image.
    pub image: WallImageQ,
    /// The image's chart points `(u, v)` at the carrier interval's start
    /// and its end, in that order whatever the traversal.
    pub ends: [(Interval, Interval); 2],
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
    /// A sphere of radius `r`, `v` the latitude: `F = r³·sin v`,
    /// `G = r²·sin v`, and the poles' terms off the face's winding and
    /// its sense (module docs, the sphere).
    Sphere {
        /// `r`.
        radius: Interval,
        /// Whether the face's normal is the sphere's outward one (its
        /// sense bit).
        outward: bool,
    },
    /// A ring torus of major radius `R` and minor radius `r < R`, `v` the
    /// tube angle: `F = r·((R² + r²)·sin v + ½Rr·sin v·cos v + 3/2·Rr·v)`,
    /// `G = r·(R·v + r·sin v)`, the face's region read off its windings
    /// and its sense (module docs, the torus).
    Torus {
        /// `R`.
        major: Interval,
        /// `r`.
        minor: Interval,
        /// Whether the face's normal is the torus's outward one (its sense
        /// bit).
        outward: bool,
    },
}

impl RotationalChartQ {
    /// The metres one radian of azimuth moves a point of the chart at
    /// `level`: what meters a junction's azimuth gap and an edge's chart
    /// length.
    fn azimuth_lever(self, level: Interval) -> Interval {
        match self {
            Self::Cylinder { radius } => radius,
            Self::Sphere { radius, .. } => radius * trig_of(level).0.abs_enclosure(),
            Self::Torus { major, minor, .. } => major + minor * trig_of(level).0,
        }
    }

    /// The metres one unit of level moves a point of the chart.
    fn level_lever(self) -> Interval {
        match self {
            Self::Cylinder { .. } => Interval::one(),
            Self::Sphere { radius, .. } => radius,
            Self::Torus { minor, .. } => minor,
        }
    }

    /// Whether the level is periodic: the torus's tube angle.
    fn level_periodic(self) -> bool {
        matches!(self, Self::Torus { .. })
    }

    /// The flux density about the chart origin per unit of metric area,
    /// from above: the radius on the cylinder and the sphere, whose
    /// normal is radial, and on the torus `|R·cos v + r| ≤ R + r`.
    fn flux_density(self) -> Interval {
        match self {
            Self::Cylinder { radius } | Self::Sphere { radius, .. } => radius,
            Self::Torus { major, minor, .. } => major + minor,
        }
    }

    /// An upper bound on a projected image's metric speed at a frame
    /// point read over a piece: on the cylinder `r·|u′| + |z′|`, on the
    /// sphere the speed of the carrier's radial projection,
    /// `r·|p × p′|/|p|²` (the weight cancels), its cross product read in
    /// `L1`, which exceeds the norm.
    fn projected_speed(self, at: &FrameJets) -> Interval {
        match self {
            Self::Cylinder { radius } => {
                chart_speed(radius, Interval::one(), at.du.0[0], at.height().0[1])
            }
            Self::Sphere { radius, .. } => {
                let (p, d) = (at.p.map(|j| j.0[0]), at.dp.map(|j| j.0[0]));
                let cross = (p[1] * d[2] - p[2] * d[1]).abs_enclosure()
                    + (p[2] * d[0] - p[0] * d[2]).abs_enclosure()
                    + (p[0] * d[1] - p[1] * d[0]).abs_enclosure();
                radius * cross / (p[0].sqr() + p[1].sqr() + p[2].sqr())
            }
            Self::Torus { major, minor, .. } => {
                let tube = tube_jets(major, at);
                chart_speed(
                    major + minor * tube.cos.0[0],
                    minor,
                    at.du.0[0],
                    tube.rate.0[0],
                )
            }
        }
    }

    /// The map-residual pad `[flux, area]` of a metric area `area` (m²)
    /// the images may move: that area at the flux density's sup.
    fn residual(self, area: Interval) -> [f64; 2] {
        let area = area.mag();
        [(pt(area) * self.flux_density()).mag(), area]
    }
}

/// The cylinder's `[F, G] = [r²·(v − v_a), r·(v − v_a)]` at a level.
fn cylinder_forms(radius: Interval, level: Interval, anchor: f64) -> [Interval; 2] {
    let g = radius * (level - pt(anchor));
    [radius * g, g]
}

/// The cylinder's `[F, G]` as jets at a projected image's frame height
/// `z = p_z/w` under its deck map's level `v_off + v_sign·z`.
fn cylinder_projected_forms(
    radius: Interval,
    p: &ProjectedEdgeQ,
    at: &FrameJets,
    anchor: f64,
) -> [Jet; 2] {
    let g = at
        .height()
        .scale(pt(p.v_sign))
        .add(Jet::cst(p.v_off - pt(anchor)))
        .scale(radius);
    [g.scale(radius), g]
}

/// The sphere's `G = r²·sin v` at a level.
fn sphere_g(radius: Interval, level: Interval) -> Interval {
    radius.sqr() * trig_of(level).1
}

/// The sphere's `G` as a jet at a projected image's frame point,
/// `sin v = p_z/|p|`, which every name of the point shares (the deck
/// map is checked at the image, [`ProjectedEdgeQ::of_image`]).
fn sphere_projected_g(radius: Interval, at: &FrameJets) -> Jet {
    let [x, y, z] = at.p;
    z.mul(x.sqr().add(y.sqr()).add(z.sqr()).sqrt().recip())
        .scale(radius.sqr())
}

/// The sphere's area, `4πr²`.
fn sphere_area(radius: Interval) -> Interval {
    pt(2.0) * crate::props::bracket::tau_bracket() * radius.sqr()
}

/// The torus's `[F, G]` at a level (the closed-form lane's
/// antiderivatives): a rim's terms, and a connector's.
fn torus_forms(major: Interval, minor: Interval, level: Interval) -> [Interval; 2] {
    let (c, s) = trig_of(level);
    let [kf, kg] = torus_slopes(major, minor);
    [
        minor * ((major.sqr() + minor.sqr()) * s + pt(0.5) * major * minor * s * c) + kf * level,
        minor.sqr() * s + kg * level,
    ]
}

/// The slopes `[κ_F, κ_G] = [3/2·R·r², R·r]` of the torus's `[F, G]` in
/// the tube angle past their periodic parts: what one whole turn of the
/// level adds to each, over `2π` (module docs, the torus).
fn torus_slopes(major: Interval, minor: Interval) -> [Interval; 2] {
    [pt(1.5) * major * minor.sqr(), major * minor]
}

/// The torus's area `4π²Rr` and its flux about its centre under the
/// outward normal, `6π²Rr²`: what one whole turn of a level or of an
/// azimuth adds, through the slopes ([`torus_slopes`]), to a winding
/// loop's `[−∮F du, −∮G du]`.
fn torus_whole(major: Interval, minor: Interval) -> [Interval; 2] {
    let tau = crate::props::bracket::tau_bracket();
    torus_slopes(major, minor).map(|k| k * tau.sqr())
}

/// The tube angle of a projected image's frame point, as jets: its
/// cosine and sine, and its rate. In homogeneous coordinates the point's
/// meridian offset from the tube's core is `(X, Z) = (√(B_x² + B_y²) −
/// R·w, B_z)`, `w` times the frame's `(ρ − R, z)`, so `v = atan2(Z, X)`
/// (the weight, positive, cancels) and
/// `v′ = (X·Z′ − Z·X′)/(X² + Z²)`, `X′ = (B_x·B_x′ + B_y·B_y′)/√(B_x² +
/// B_y²) − R·w′`. Every name of the point shares the cosine and the sine
/// (the deck map is checked at the image,
/// [`ProjectedEdgeQ::of_image`]); the rate is every name's.
struct TubeJets {
    cos: Jet,
    sin: Jet,
    rate: Jet,
}

fn tube_jets(major: Interval, at: &FrameJets) -> TubeJets {
    let [bx, by, bz] = at.p;
    let [dx, dy, dz] = at.dp;
    let s = bx.sqr().add(by.sqr()).sqrt();
    let x = s.sub(at.w.scale(major));
    let dx_ = bx
        .mul(dx)
        .add(by.mul(dy))
        .mul(s.recip())
        .sub(at.dw.scale(major));
    let m2 = x.sqr().add(bz.sqr());
    let inv_m = m2.sqrt().recip();
    TubeJets {
        cos: x.mul(inv_m),
        sin: bz.mul(inv_m),
        rate: x.mul(dz).sub(bz.mul(dx_)).mul(m2.recip()),
    }
}

/// A projected image's frame point over a piece or at a node, as the
/// chart table reads it: the azimuth's rate `u′`, and the carrier's
/// homogeneous point `p`, its derivative `p′`, its weight `w` and the
/// weight's derivative `w′` (a conic's or a line's weight is `1`).
struct FrameJets {
    du: Jet,
    p: [Jet; 3],
    dp: [Jet; 3],
    w: Jet,
    dw: Jet,
}

impl FrameJets {
    /// The frame height `z = p_z/w`.
    fn height(&self) -> Jet {
        self.p[2].mul(self.w.recip())
    }
}

/// The chart term `o·A⃗` of a face (module docs): the closed-form edges'
/// part, bracketed by the caller, and each rational carrier's vector
/// area about the chart origin, read through `lever` (the chart origin
/// less the point the flux is about) per round. `A⃗` is the carriers',
/// and the face is the region on the chart its pcurves bound, so the
/// term is padded by the ribbon between the two at `|lever|`
/// ([`chart_term_pad`]); `ribbon` is the carriers' half of it, which
/// the caller, who holds the carriers, reads.
#[derive(Clone, Debug)]
pub struct ChartTermQ {
    /// The closed-form edges' `o·A⃗`.
    pub closed: Interval,
    /// The chart origin less the point the flux is about.
    pub lever: RVec3,
    /// The rational carriers, about the chart origin.
    pub rationals: Vec<RationalEdgeQ>,
    /// The spiric carriers, their centres about the chart origin.
    pub spirics: Vec<SpiricEdgeQ>,
    /// `Σ env·L` over the face's edges, `env` an edge's pcurve
    /// certificate envelope and `L` an upper bound on its carrier's
    /// length (m²): what the carriers' speed adds to the ribbon between
    /// them and their images.
    pub ribbon: f64,
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
        let zero = |x: Interval| exact_point(x) == Some(0.0);
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

/// **The chart's speed bound** at the rates `u′`, `v′`:
/// `√(a²u′² + l²v′²) ≤ a·|u′| + l·|v′|`, `a` the chart's azimuth lever
/// and `l` its level lever — what bounds a harmonic edge's metric length
/// and a projected cylinder edge's.
fn chart_speed(lever: Interval, level: Interval, du: Interval, dv: Interval) -> Interval {
    lever * du.abs_enclosure() + level * dv.abs_enclosure()
}

/// The harmonic channel `c`'s enclosure over the whole of `e`'s span.
fn harmonic_over(e: &TrimEdgeQ, c: HarmChan) -> Interval {
    let (a, b) = e.limits();
    let (c_all, s_all) = trig_over(e.trig0, 0.0, b - a);
    c.eval(c_all, s_all, Interval::from_bounds(a, b))
}

/// A certified upper bound on a harmonic edge's METRIC length (metres):
/// `sup chart_speed · span`, over the whole span at once, the azimuth
/// lever read over the edge's levels.
fn harmonic_metric_length(e: &TrimEdgeQ, chart: RotationalChartQ) -> f64 {
    let (a, b) = e.limits();
    let speed = chart_speed(
        chart.azimuth_lever(harmonic_over(e, e.v)),
        chart.level_lever(),
        harmonic_over(e, e.u.deriv()),
        harmonic_over(e, e.v.deriv()),
    );
    (speed * pt(b - a)).mag()
}

/// **A harmonic sphere or torus edge's terms**, `σ·∫ −forms(v) du` and
/// its turn `σ·Δu`: a rim (`v = V`, `u` linear) reads `−forms(V)·Δu`
/// exactly, and a meridian (`u` constant) nothing; the endpoint-bracket
/// pad is the rim's `|forms(V)·u′|` times its brackets' widths. Those two
/// shapes are the harmonic images these charts read, and any other
/// refuses at the edge's entry ([`edge_entry`]).
///
/// That pad is sound by slack, and no row isolates it: the part of the
/// true boundary between a bracket's midpoint and the true end lies on
/// the junction's corner loop, whose leg along the boundary the end's
/// slack already counts (the bracket's half-width times the speed,
/// [`end_slack`]), so the corner pad ([`connector_terms`]) covers that
/// region before this pad is read. It is kept as the rim's own reading of
/// its bracket, as the cylinder's harmonic rule keeps its.
fn rim_terms<const K: usize>(
    e: &TrimEdgeQ,
    forms: impl Fn(Interval) -> [Interval; K],
) -> ([Interval; K], Interval) {
    if e.u.is_flat(false) {
        return ([Interval::zero(); K], Interval::zero());
    }
    let (a, b) = e.limits();
    let sign = |x: Interval| if e.forward { x } else { -x };
    let widths = e.t0.width() + e.t1.width();
    let terms = forms(e.v.c0).map(|f| {
        let pad = (f * e.u.cl).mag() * widths;
        sign(-f * e.u.cl * pt(b - a) + Interval::from_bounds(-pad, pad))
    });
    (terms, sign(e.u.cl * pt(b - a)))
}

/// Whether a harmonic image is one the sphere's and the torus's arms
/// read: a meridian, or a rim.
fn iso_harmonic_shape(e: &TrimEdgeQ) -> bool {
    e.u.is_flat(false) || (e.u.is_flat(true) && e.v.is_flat(false))
}

// ---------------------------------------------------------------------
// Projected edges (module docs: the curved rims' rule over the frame)
// ---------------------------------------------------------------------

/// The azimuth's rate `u′ = (x·y′ − y·x′)/(x² + y²)` at a frame point
/// `p` moving at `p′`, which reads no branch; on a net, in homogeneous
/// coordinates, where the weight cancels from it.
fn azimuth_rate(p: &[Jet; 3], dp: &[Jet; 3]) -> Jet {
    p[0].mul(dp[1])
        .sub(p[1].mul(dp[0]))
        .mul(p[0].sqr().add(p[1].sqr()).recip())
}

/// The frame point of a projected net in homogeneous coordinates.
fn net_rate(j: &RationalJets) -> FrameJets {
    FrameJets {
        du: azimuth_rate(&j.b, &j.db),
        p: j.b,
        dp: j.db,
        w: j.w,
        dw: j.dw,
    }
}

/// The frame point of a projected conic at the trig jets `c`, `s` of
/// its parameter.
fn conic_rate(centre: RVec3, a: RVec3, b: RVec3, c: Jet, s: Jet) -> FrameJets {
    let p = [0, 1, 2].map(|k| Jet::cst(centre[k]).add(c.scale(a[k])).add(s.scale(b[k])));
    let dp = [0, 1, 2].map(|k| c.scale(b[k]).sub(s.scale(a[k])));
    unit_weight(p, dp)
}

/// The frame point of a projected line at the jet `t` of its parameter.
fn line_rate(point: RVec3, dir: RVec3, t: Jet) -> FrameJets {
    let p = [0, 1, 2].map(|k| Jet::cst(point[k]).add(t.scale(dir[k])));
    unit_weight(p, dir.map(Jet::cst))
}

/// A frame point of weight `1` and its azimuth's rate.
fn unit_weight(p: [Jet; 3], dp: [Jet; 3]) -> FrameJets {
    FrameJets {
        du: azimuth_rate(&p, &dp),
        p,
        dp,
        w: Jet::cst(Interval::one()),
        dw: Jet::cst(Interval::zero()),
    }
}

/// The frame point of a projected conic or line at `t`, a conic's
/// through the trig jets of `t`. A net has none here: its spans are read
/// through [`rational_integrals_each`], and its reading is refused.
fn analytic_rate(carrier: &FramedQ, t: Interval) -> FrameJets {
    match carrier {
        FramedQ::Conic { centre, a, b, .. } => {
            let (_, cj, sj) = trig_jets(t);
            conic_rate(*centre, *a, *b, cj, sj)
        }
        FramedQ::Line { point, dir, .. } => line_rate(*point, *dir, Jet::var(t)),
        FramedQ::Net(_) => {
            let refused = Jet::cst(Interval::refused());
            FrameJets {
                du: refused,
                p: [refused; 3],
                dp: [refused; 3],
                w: refused,
                dw: refused,
            }
        }
    }
}

/// A conic's or a line's cut intervals, as the rule reads intervals.
fn cut_intervals(cuts: &[f64]) -> impl Iterator<Item = (f64, f64, ())> + '_ {
    cuts.windows(2).map(|w| (w[0], w[1], ()))
}

/// `∫ g(frame point) dt` of a projected edge in its carrier's own order
/// at `pieces(i)` pieces on span or cut interval `i`, with the part of each width a
/// later round removes: a net span by span, a conic or a line over its
/// cuts.
fn framed_integrals<const N: usize>(
    carrier: &FramedQ,
    pieces: impl Fn(usize) -> usize,
    g: impl Fn(&FrameJets) -> [Jet; N],
) -> ([Interval; N], [f64; N]) {
    match carrier {
        FramedQ::Net(net) => rational_integrals_each(net, pieces, |j| g(&net_rate(j))),
        FramedQ::Conic { cuts, .. } | FramedQ::Line { cuts, .. } => composite_each(
            cut_intervals(cuts)
                .enumerate()
                .map(|(i, (a, b, ()))| (a, b, (), pieces(i))),
            |(), _, _| (),
            |(), t| g(&analytic_rate(carrier, t)),
        ),
    }
}

/// `σ·∫ g(frame point) dt` of a projected edge at `pieces(i)` pieces on
/// span or cut interval `i`, `σ` its traversal, and the part of each width a later
/// round removes.
fn projected_integrals<const N: usize>(
    e: &ProjectedEdgeQ,
    pieces: impl Fn(usize) -> usize,
    g: impl Fn(&FrameJets) -> [Jet; N],
) -> ([Interval; N], [f64; N]) {
    let (terms, removable) = framed_integrals(&e.carrier, pieces, g);
    (if e.forward { terms } else { terms.map(|t| -t) }, removable)
}

/// Pieces per span or cut interval at which a projected edge's metric
/// length is read, before the rounds: an upper bound at any count, which
/// pads every round's enclosure ([`RotationalChartQ::residual`],
/// [`chart_term_pad`]), so it is read finer than a round-0 piece.
const PROJECTED_ENTRY_PIECES: usize = 4;

/// A projected edge's turns `[Δu, Δv]`, in its carrier's order, read at
/// its schedule's round 0 ([`span_schedule`]) and at four times as many
/// pieces again while either enclosure is wider than an eighth of a
/// turn, every span at most [`MOST_SPAN_PIECES`]: a chord that passes
/// near the chart's axis (a sphere's pole) turns fast there, and its turn
/// at the first count may not tell whole turns apart ([`branch_turns`]).
/// A reading still that wide at the cap is the one [`branch_turns`]
/// refuses. `Δv` is read on the torus alone, whose level is an angle;
/// elsewhere it is `0`.
fn entry_turn(p: &ProjectedEdgeQ, schedule: &Schedule, chart: RotationalChartQ) -> [Interval; 2] {
    let eighth = core::f64::consts::TAU / 8.0;
    let mut scale = 1;
    loop {
        let pieces = |i: usize| (schedule.0[i] * scale).min(MOST_SPAN_PIECES);
        let (turns, _) = framed_integrals(&p.carrier, pieces, |at| {
            [
                at.du,
                match chart {
                    RotationalChartQ::Torus { major, .. } => tube_jets(major, at).rate,
                    _ => Jet::cst(Interval::zero()),
                },
            ]
        });
        let capped = schedule.0.iter().all(|&n| n * scale >= MOST_SPAN_PIECES);
        let told = |t: Interval| t.is_certified() && t.width() < eighth;
        if capped || turns.iter().all(|&t| told(t)) {
            return turns;
        }
        scale *= 4;
    }
}

/// A spiric edge's turn `Δu`, read at [`PROJECTED_ENTRY_PIECES`] and at
/// four times as many pieces again while it is wider than an eighth of a
/// turn, up to [`MOST_SPAN_PIECES`], as a projected edge's is
/// ([`entry_turn`]): an oval near the tube's hole turns fast where its
/// cut passes the inner equator.
fn refined_turn(read: impl Fn(usize) -> Interval) -> Interval {
    let eighth = core::f64::consts::TAU / 8.0;
    let mut pieces = PROJECTED_ENTRY_PIECES;
    loop {
        let turn = read(pieces);
        if pieces >= MOST_SPAN_PIECES || (turn.is_certified() && turn.width() < eighth) {
            return turn;
        }
        pieces *= 4;
    }
}

/// A projected torus edge's `[−∫F du, −∫G du]` in its carrier's
/// order at `pieces(i)` pieces on span or cut interval `i`. `F` and `G`
/// are their periodic parts, read off
/// the tube angle's cosine and sine at the frame point, plus their slopes
/// times the level `v` ([`torus_slopes`]), which no frame point names: it
/// is the image's own level at the carrier interval's start (its
/// evaluator's, crossed in by the caller) plus `∫v′`, the rule's running
/// primitive ([`primitive_each`]).
fn torus_projected(
    e: &ProjectedEdgeQ,
    major: Interval,
    minor: Interval,
    pieces: impl Fn(usize) -> usize,
) -> Primitive<2> {
    let [kf, kg] = torus_slopes(major, minor);
    let neg = -Interval::one();
    let g = |at: &FrameJets| {
        let tube = tube_jets(major, at);
        let fp = tube
            .sin
            .scale(minor * (major.sqr() + minor.sqr()))
            .add(tube.sin.mul(tube.cos).scale(pt(0.5) * major * minor.sqr()));
        let gp = tube.sin.scale(minor.sqr());
        Integrand {
            own: [fp.mul(at.du).scale(neg), gp.mul(at.du).scale(neg)],
            by: [at.du.scale(-kf), at.du.scale(-kg)],
            rate: tube.rate,
        }
    };
    let start = e.ends[0].1;
    match &e.carrier {
        FramedQ::Net(net) => rational_primitive_each(net, pieces, start, |j| g(&net_rate(j))),
        FramedQ::Conic { cuts, .. } | FramedQ::Line { cuts, .. } => primitive_each(
            cut_intervals(cuts)
                .enumerate()
                .map(|(i, (a, b, ()))| (a, b, (), pieces(i))),
            start,
            |(), _, _| (),
            |(), t| g(&analytic_rate(&e.carrier, t)),
        ),
    }
}

/// A certified upper bound on a projected edge's metric length (metres),
/// `Σ h·sup speed` over its pieces ([`RotationalChartQ::projected_speed`]);
/// `NaN` where a piece reads no bound.
fn projected_metric_length(e: &ProjectedEdgeQ, chart: RotationalChartQ) -> f64 {
    let rate = |at: &FrameJets| {
        let speed = chart.projected_speed(at);
        if speed.is_certified() {
            speed.hi()
        } else {
            f64::NAN
        }
    };
    match &e.carrier {
        FramedQ::Net(net) => {
            rational_rate_bound(net, PROJECTED_ENTRY_PIECES, |j| rate(&net_rate(j)))
        }
        FramedQ::Conic { cuts, .. } | FramedQ::Line { cuts, .. } => rate_bound_over(
            cut_intervals(cuts),
            PROJECTED_ENTRY_PIECES,
            |(), _, _| (),
            |(), t| rate(&analytic_rate(&e.carrier, t)),
        ),
    }
}

// ---------------------------------------------------------------------
// One edge, whatever its term
// ---------------------------------------------------------------------

/// What the lane reads of each edge once, before the rounds: its metric
/// length, and a projected or a spiric edge's turns `[Δu, Δv]` in its
/// carrier's order (`Δv` read on the torus alone).
struct EdgeEntry {
    length: f64,
    turn: Option<[Interval; 2]>,
    /// A projected edge's pieces per span; empty for a harmonic or a
    /// spiric edge.
    schedule: Schedule,
}

/// What the lane reads of `e` before the rounds.
fn edge_entry(e: &EdgeTermQ, chart: RotationalChartQ) -> Result<EdgeEntry, PropsError> {
    Ok(match e {
        EdgeTermQ::Harmonic(h) => {
            if !matches!(chart, RotationalChartQ::Cylinder { .. }) && !iso_harmonic_shape(h) {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a harmonic sphere or torus image that is neither a rim nor a meridian",
                });
            }
            EdgeEntry {
                length: harmonic_metric_length(h, chart),
                turn: None,
                schedule: Schedule::default(),
            }
        }
        EdgeTermQ::Projected(p) => {
            let length = projected_metric_length(p, chart);
            let schedule = span_schedule(p, chart);
            let turn = entry_turn(p, &schedule, chart);
            if !(length.is_finite() && turn.iter().all(|t| t.is_certified())) {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a projected edge whose chart speed or turn reads no enclosure — its \
                           carrier reaches the chart's axis",
                });
            }
            EdgeEntry {
                length,
                turn: Some(turn),
                schedule,
            }
        }
        EdgeTermQ::Spiric(w) => {
            let RotationalChartQ::Torus { major, minor, .. } = chart else {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a spiric's wall image on a chart other than its torus",
                });
            };
            let length = spiric_length_bound(&w.edge, crate::props::spiric::LENGTH_PIECES);
            let du = refined_turn(|pieces| {
                crate::props::spiric::wall_terms(&w.edge, &w.image, (major, minor), pieces).0[2]
            });
            let dv = pt(w.image.sense) * (w.edge.t1 - w.edge.t0);
            let (Some(length), true) = (length, du.is_certified()) else {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a spiric wall edge whose span's rate window has no certified \
                           positive f",
                });
            };
            EdgeEntry {
                length,
                turn: Some([du, dv]),
                schedule: Schedule::default(),
            }
        }
    })
}

/// One cylinder edge's `[−∫F du, −∫G du]` at `round`, traversal and
/// pads folded in, and the part of its flux width a later round removes.
fn cylinder_edge_terms(
    e: &EdgeTermQ,
    entry: &EdgeEntry,
    radius: Interval,
    anchor: f64,
    round: usize,
) -> ([Interval; 2], f64) {
    let chart = RotationalChartQ::Cylinder { radius };
    let (terms, removable) = match e {
        EdgeTermQ::Harmonic(h) => {
            let (level, rem) = harmonic_edge_integral(h, anchor, round_pieces(round));
            (
                [radius.sqr() * level, radius * level],
                rem * radius.sqr().mag(),
            )
        }
        EdgeTermQ::Projected(p) => {
            let (terms, [rf, _]) = projected_integrals(p, entry.schedule.at(round), |at| {
                let [f, g] = cylinder_projected_forms(radius, p, at, anchor);
                [
                    f.mul(at.du).scale(-Interval::one()),
                    g.mul(at.du).scale(-Interval::one()),
                ]
            });
            (terms, rf)
        }
        EdgeTermQ::Spiric(_) => ([Interval::refused(); 2], f64::INFINITY),
    };
    let pad = chart.residual(pt(entry.length) * edge_env(e));
    (
        [0, 1].map(|k| terms[k] + Interval::from_bounds(-pad[k], pad[k])),
        removable,
    )
}

/// One sphere edge's `−∫G du` at `round`, traversal and the
/// map-residual pad folded in, and the part of its flux width a later
/// round removes. The sphere reads no `F`: about the centre `F = r·G`,
/// and its flux is `r·s_f·|Ω|` ([`sphere_face_area`]).
fn sphere_edge_area(
    e: &EdgeTermQ,
    entry: &EdgeEntry,
    radius: Interval,
    round: usize,
) -> (Interval, f64) {
    let (area, removable) = match e {
        EdgeTermQ::Harmonic(h) => (rim_terms(h, |v| [sphere_g(radius, v)]).0[0], 0.0),
        EdgeTermQ::Projected(p) => {
            let ([g], [rg]) = projected_integrals(p, entry.schedule.at(round), |at| {
                [sphere_projected_g(radius, at)
                    .mul(at.du)
                    .scale(-Interval::one())]
            });
            (g, rg * radius.mag())
        }
        EdgeTermQ::Spiric(_) => (Interval::refused(), f64::INFINITY),
    };
    let pad = RotationalChartQ::Sphere {
        radius,
        outward: true,
    }
    .residual(pt(entry.length) * edge_env(e))[1];
    (area + Interval::from_bounds(-pad, pad), removable)
}

/// One torus edge's `[−∫F du, −∫G du]` at `round`, traversal and
/// the map-residual pad folded in, read on the edge's own lift: a rim's
/// level, a spiric's `v₀ + σ·t`, and a projected image's level from its
/// own start ([`torus_projected`]); the lane carries each onto the
/// chain's ([`LoopLift`]). And the part of its flux width a later round
/// removes.
fn torus_edge_terms(
    e: &EdgeTermQ,
    entry: &EdgeEntry,
    (major, minor): (Interval, Interval),
    round: usize,
) -> ([Interval; 2], f64) {
    let sign = |forward: bool, t: [Interval; 2]| if forward { t } else { t.map(|x| -x) };
    let (terms, removable) = match e {
        EdgeTermQ::Harmonic(h) => (rim_terms(h, |v| torus_forms(major, minor, v)).0, 0.0),
        EdgeTermQ::Projected(p) => {
            let run = torus_projected(p, major, minor, entry.schedule.at(round));
            (sign(p.forward, run.total), run.removable[0])
        }
        EdgeTermQ::Spiric(w) => {
            let ([f, g, _], rem) = crate::props::spiric::wall_terms(
                &w.edge,
                &w.image,
                (major, minor),
                round_pieces(round),
            );
            (sign(w.edge.forward, [f, g]), rem)
        }
    };
    let pad = RotationalChartQ::Torus {
        major,
        minor,
        outward: true,
    }
    .residual(pt(entry.length) * edge_env(e));
    (
        [0, 1].map(|k| terms[k] + Interval::from_bounds(-pad[k], pad[k])),
        removable,
    )
}

/// One junction of a loop: the edge before and its entry, the edge
/// after and its entry.
struct Junction<'a> {
    before: (&'a EdgeTermQ, &'a EdgeEntry),
    after: (&'a EdgeTermQ, &'a EdgeEntry),
}

impl Junction<'_> {
    /// The two chart ends, the edge before's end and the edge after's
    /// start.
    fn ends(&self) -> [(Interval, Interval); 2] {
        [
            chart_end(self.before.0, true),
            chart_end(self.after.0, false),
        ]
    }

    /// The slack the closure accepts the two ends apart by, its
    /// [`crate::props::closure::VERTEX_SLACK`]`·ε` aside: each end's
    /// [`end_slack`].
    fn slack(&self) -> f64 {
        end_slack(self.before.0, self.before.1, true) + end_slack(self.after.0, self.after.1, false)
    }

    /// The gap the closure accepts at the junction, its vertex slack
    /// included ([`crate::props::closure::interval_loop_turns`]): what
    /// the connector's pads read.
    fn gap(&self, band: Band) -> Interval {
        pt(self.slack() + crate::props::closure::VERTEX_SLACK * band.zero())
    }

    /// The levels the connector runs between: the hull of the two ends'
    /// levels, the edge after's start carried by the whole turns of the
    /// level nearest the edge before's end where the level is periodic
    /// (the torus's tube angle), as the closure takes them off
    /// ([`crate::props::closure::nearest_turns`]).
    fn levels(&self, chart: RotationalChartQ) -> Interval {
        let [(_, v1), (_, v0)] = self.ends();
        if chart.level_periodic() {
            Interval::hull(v1, v0 + level_turns(v1 - v0))
        } else {
            Interval::hull(v1, v0)
        }
    }

    /// The metric area beside the connector, between it and the true
    /// vertex, that no image's strip covers ([`connector_terms`]).
    fn corner(&self, band: Band) -> Interval {
        let g = self.gap(band);
        pt(2.0) * g * (pt(edge_env(self.before.0).mag()) + pt(edge_env(self.after.0).mag()) + g)
    }
}

/// A loop's junctions, in traversal order, each edge to the next.
fn junctions<'a>(
    edges: &'a [EdgeTermQ],
    entries: &'a [EdgeEntry],
) -> impl Iterator<Item = Junction<'a>> + 'a {
    let n = edges.len();
    (0..n).map(move |i| Junction {
        before: (&edges[i], &entries[i]),
        after: (&edges[(i + 1) % n], &entries[(i + 1) % n]),
    })
}

/// **The junctions' connectors** of each loop, the chart's forms `forms`
/// of a level (`[F, G]` on the cylinder, `G` on the sphere) summed over
/// the loop's junctions with their pads at the densities `density`.
///
/// The closure accepts a junction whose two images' ends sit apart
/// within its slack, so the image chain is closed on the chart only by a
/// connector between them: from the edge before's end to the edge
/// after's start, a chart displacement of `−(Δu − m·τ)` in the azimuth,
/// `m` the closure's whole turns ([`junction_gap`]), at levels between
/// the two ends'. Its term is `forms(v)·(Δu − m·τ)` for some such level
/// `v`, read over their hull. At a sphere's pole, where the azimuth lever
/// vanishes and the two ends' azimuths differ by the face's angle there,
/// the connector is the pole's own edge of the chart and its term the
/// pole's share. Reading the forms over the hull rather than at either
/// end matters only to second order, `|F′|·|Δv|·|Δu|`, the box the
/// connector spans, which the corner pad covers by the same bound; no
/// row isolates it.
///
/// **The corner pad.** Between the connector and the true vertex `V`
/// lies a closed curve: from the edge before's image end to its true
/// boundary point at the same parameter (within its envelope `e_i`),
/// along the true boundary to `V` (within the end's parameter bracket
/// times its speed), on to the edge after's in the same way, and back
/// along the connector. Each leg is within the end slack it was read
/// with, so its length `P` is at most twice the junction's gap `g` (the
/// two slacks and the closure's `3ε`). By the isoperimetric inequality
/// the area it bounds, weighted by winding, is at most `P²/4π` on the
/// flat cylinder chart, and on the sphere, for the side of at most half
/// the sphere, at most `P²/2π` (`P² ≥ A·(4π − A/r²)`): `2g²/π` at most,
/// which the pad `2·g·(e_i + e_j + g)` exceeds by `π`. No leg need be
/// straight, and the other side of the sphere is a whole sphere away,
/// which the poles' count reads ([`sphere_outside`]). The strips along
/// the images are the map-residual pad's.
fn connector_terms<const K: usize>(
    face: &RotationalFaceQ,
    entries: &[Vec<EdgeEntry>],
    band: Band,
    forms: impl Fn(usize, usize, Interval) -> [Interval; K],
    density: [Interval; K],
) -> Vec<[Interval; K]> {
    face.loops
        .iter()
        .zip(entries)
        .enumerate()
        .map(|(l, (edges, entries))| {
            let mut i = 0;
            junctions(edges, entries).fold([Interval::zero(); K], |acc, j| {
                let [(u1, _), (u0, _)] = j.ends();
                let du = junction_gap(u1 - u0);
                let forms = forms(l, i, j.levels(face.chart));
                i += 1;
                let corner = j.corner(band);
                core::array::from_fn(|k| {
                    let pad = (corner * density[k]).mag();
                    acc[k] + forms[k] * du + Interval::from_bounds(-pad, pad)
                })
            })
        })
        .collect()
}

/// A junction's azimuth gap `Δu` (the edge before's end less the edge
/// after's start) less the whole turns the closure takes off it
/// ([`crate::props::closure::nearest_turns`]): what is left for its
/// connector.
fn junction_gap(delta: Interval) -> Interval {
    delta - level_turns(delta)
}

/// The whole turns nearest a junction's gap `delta`, in radians
/// ([`crate::props::closure::nearest_turns`]).
fn level_turns(delta: Interval) -> Interval {
    pt(crate::props::closure::nearest_turns(delta)) * crate::props::bracket::tau_bracket()
}

/// An edge's pcurve certificate envelope.
fn edge_env(e: &EdgeTermQ) -> Interval {
    match e {
        EdgeTermQ::Harmonic(h) => h.env,
        EdgeTermQ::Projected(p) => p.env,
        EdgeTermQ::Spiric(w) => w.image.env,
    }
}

/// Whether an edge is traversed in its carrier's order.
fn traversal(e: &EdgeTermQ) -> bool {
    match e {
        EdgeTermQ::Harmonic(h) => h.forward,
        EdgeTermQ::Projected(p) => p.forward,
        EdgeTermQ::Spiric(w) => w.edge.forward,
    }
}

/// **The chart term's pad** (module docs, honesty pads): `|lever|` times
/// how far the carriers' vector area may sit from the face's.
///
/// The face is the region on the chart its true boundary bounds, within
/// metric distance `e` of each pcurve image, and `A⃗` is read off the
/// carriers, each within `e` of its image at every parameter. Two
/// ribbons separate them:
///
/// - **carrier to image**: with `d = a − b` the carrier less its image,
///   the homotopy `b + λ·d` sweeps the vector-area difference
///   `∫(b′ + ½d′)×d = ½∫(a′ + b′)×d`, at most `½·e·(L_a + L_b)`, `L_a`
///   the carrier's length (the caller's [`ChartTermQ::ribbon`]) and `L_b`
///   the image's metric length ([`EdgeEntry::length`]);
/// - **image to the true boundary**: both on the chart, so the
///   difference is the vector area of the region between them, at most
///   its area `e·L_b`, the map-residual pad's own area.
///
/// Each junction, where the two images' ends meet within the slack the
/// closure accepted (`g`, [`end_slack`] at each end), adds the connector
/// that closes the image chain on the chart: a ribbon of length `g` and
/// width at most `e + g` to the carriers' shared vertex, and the region
/// beside it, `2·g·(e_i + e_j + g)` together.
///
/// `|lever|` is read as the sum of its components' magnitudes, which
/// exceeds the norm.
fn chart_term_pad(face: &RotationalFaceQ, entries: &[Vec<EdgeEntry>], band: Band) -> f64 {
    let lever = face
        .chart_term
        .lever
        .iter()
        .fold(Interval::zero(), |a, l| a + pt(l.mag()));
    let mut ribbon = pt(0.5) * pt(face.chart_term.ribbon);
    for (edges, entries) in face.loops.iter().zip(entries) {
        for j in junctions(edges, entries) {
            let (edge, entry) = j.before;
            ribbon =
                ribbon + pt(1.5) * pt(entry.length) * pt(edge_env(edge).mag()) + j.corner(band);
        }
    }
    certified_hi(lever * ribbon)
}

/// The face's chart term at `round`, each rational at its schedule
/// ([`term_schedule`]) and each spiric at a round's pieces, and the part
/// of its width a later round removes: each carrier's vector area
/// through the lever.
fn chart_term_at(term: &ChartTermQ, schedules: &[Schedule], round: usize) -> (Interval, f64) {
    let (mut flux, mut removable) = (term.closed, 0.0);
    for (e, s) in term.rationals.iter().zip(schedules) {
        let (va, rem) = rational_vector_area_each(e, s.at(round));
        flux = (0..3).fold(flux, |a, k| a + term.lever[k] * va[k]);
        removable += (0..3).map(|k| term.lever[k].mag() * rem[k]).sum::<f64>();
    }
    for e in &term.spirics {
        let (va, rem) = crate::props::spiric::vector_area_split(e, round_pieces(round));
        flux = (0..3).fold(flux, |a, k| a + term.lever[k] * va[k]);
        // The term carries its moment `J` as `n·r·J`.
        removable += (0..3)
            .map(|k| (term.lever[k] * e.normal[k]).mag())
            .sum::<f64>()
            * e.minor.mag()
            * rem;
    }
    (flux, removable)
}

// ---------------------------------------------------------------------
// The lane
// ---------------------------------------------------------------------

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
/// ([`PropsError::LoopsWindChart`]), a torus that is not a ring torus, a
/// ring wound with its face at the round the call ends
/// ([`PropsError::RingWoundWithFace`]), one wholly inside the band, or
/// one still straddling it there (an accepted round whose ring straddles
/// refines on, [`Run::ringed`]).
pub fn rotational_cut_face_rounds<T: Decide>(
    face: &RotationalFaceQ,
    band: Band,
    window: RoundWindow,
    accept: &mut Accept<'_>,
) -> Result<LaneRun, PropsError> {
    let chart = face.chart;
    if let RotationalChartQ::Torus { major, minor, .. } = chart {
        let floor = (major - minor).lo().min(minor.lo());
        if !(major.is_certified() && minor.is_certified()) || floor.is_nan() || floor <= 0.0 {
            return Err(PropsError::QuadratureUnsupported {
                what: "a torus face whose radii are refused brackets or not a ring torus's \
                       (0 < r < R)",
            });
        }
    }
    let entries: Vec<Vec<EdgeEntry>> = face
        .loops
        .iter()
        .map(|edges| edges.iter().map(|e| edge_entry(e, chart)).collect())
        .collect::<Result<_, _>>()?;
    let turns = loop_windings::<T>(face, &entries, band)?;
    let term_pad = chart_term_pad(face, &entries, band);
    if !term_pad.is_finite() {
        return Err(PropsError::QuadratureUnsupported {
            what: "a rotational face whose chart-term pad reads no bound",
        });
    }
    let reading = match chart {
        RotationalChartQ::Cylinder { radius } => {
            let anchor = level_anchor(&face.loops)?;
            Reading::Cylinder {
                radius,
                anchor,
                connectors: connector_terms(
                    face,
                    &entries,
                    band,
                    |_, _, v| cylinder_forms(radius, v, anchor),
                    [radius, Interval::one()],
                ),
            }
        }
        RotationalChartQ::Sphere { radius, outward } => Reading::Sphere {
            radius,
            outward,
            connectors: connector_terms(
                face,
                &entries,
                band,
                |_, _, v| [sphere_g(radius, v)],
                [Interval::one()],
            ),
        },
        RotationalChartQ::Torus {
            major,
            minor,
            outward,
        } => {
            let lifts = loop_lifts(face, &entries)?;
            let turn = torus_slopes(major, minor).map(|k| k * crate::props::bracket::tau_bracket());
            let connectors = connector_terms(
                face,
                &entries,
                band,
                |l, j, v| {
                    let up = pt(lift_turns(lifts[l].ends[j]));
                    let forms = torus_forms(major, minor, v);
                    [0, 1].map(|k| forms[k] + up * turn[k])
                },
                [major + minor, Interval::one()],
            );
            Reading::Torus {
                radii: (major, minor),
                outward,
                lifts,
                connectors,
            }
        }
    };
    let reach = piece_reach(chart);
    let term_schedules: Vec<Schedule> = face
        .chart_term
        .rationals
        .iter()
        .map(|e| term_schedule(e, reach))
        .collect();
    let cost = |round: usize| face_cost(face, &entries, &term_schedules, round);
    let mut run = Run::enter(window, &cost);
    let mut round = run.first();
    loop {
        let mut removable = 0.0_f64;
        let (term, term_rem) = chart_term_at(&face.chart_term, &term_schedules, round);
        let term = term + Interval::from_bounds(-term_pad, term_pad);
        // Per loop at this resolution, its connectors included: the
        // cylinder's and the torus's [−∮F du, −∮G du], the sphere's
        // −∮G du.
        let (flux, signed, area, areas) = match &reading {
            Reading::Cylinder {
                radius,
                anchor,
                connectors,
            } => {
                let per_loop: Vec<[Interval; 2]> = face
                    .loops
                    .iter()
                    .zip(&entries)
                    .zip(connectors)
                    .map(|((edges, entries), &connectors)| {
                        edges
                            .iter()
                            .zip(entries)
                            .fold(connectors, |acc, (e, entry)| {
                                let (terms, rem) =
                                    cylinder_edge_terms(e, entry, *radius, *anchor, round);
                                removable += rem;
                                [acc[0] + terms[0], acc[1] + terms[1]]
                            })
                    })
                    .collect();
                let flux = per_loop.iter().fold(term, |f, l| f + l[0]);
                let signed = per_loop.iter().fold(Interval::zero(), |a, l| a + l[1]);
                let areas: Vec<Interval> = per_loop.iter().map(|l| l[1]).collect();
                (flux, signed, signed, areas)
            }
            Reading::Sphere {
                radius,
                outward,
                connectors,
            } => {
                let areas: Vec<Interval> = face
                    .loops
                    .iter()
                    .zip(&entries)
                    .zip(connectors)
                    .map(|((edges, entries), &[connectors])| {
                        edges
                            .iter()
                            .zip(entries)
                            .fold(connectors, |acc, (e, entry)| {
                                let (g, rem) = sphere_edge_area(e, entry, *radius, round);
                                removable += rem;
                                acc + g
                            })
                    })
                    .collect();
                let area = sphere_face_area(*radius, *outward, &areas, &turns);
                let signed = if *outward { area } else { -area };
                (term + *radius * signed, signed, area, areas)
            }
            Reading::Torus {
                radii,
                outward,
                lifts,
                connectors,
            } => {
                let per_loop: Vec<[Interval; 2]> = face
                    .loops
                    .iter()
                    .zip(&entries)
                    .zip(connectors)
                    .zip(lifts)
                    .zip(&turns)
                    .map(|((((edges, entries), &connectors), lift), w)| {
                        torus_loop(
                            *radii,
                            (edges, entries),
                            (connectors, lift, w[1]),
                            round,
                            &mut removable,
                        )
                    })
                    .collect();
                let reading = torus_face(*radii, *outward, &per_loop);
                let signed = if *outward {
                    reading.area
                } else {
                    -reading.area
                };
                let areas = per_loop.iter().map(|l| l[1]).collect();
                (term + reading.flux, signed, reading.area, areas)
            }
        };
        // The area's magnitude: the face's own, signed on the cylinder by
        // its loops' traversal, and on the sphere and the torus by
        // nothing, read where a loop wound with its face makes it
        // negative.
        let bounds = FaceCutBounds {
            flux,
            area: area.abs_enclosure(),
        };
        let (stop, accepted) = run.ask(round, bounds, removable + term_rem, accept, || {
            let perim: f64 = entries.iter().flatten().map(|e| e.length).sum();
            face_extent::<T>(bounds.area, perim, band)
        })?;
        if let Some(out) = run.ringed(round, stop, accepted, bounds, || match &reading {
            Reading::Cylinder { .. } => {
                let winds: Vec<bool> = turns.iter().map(|&[k, _]| k != 0).collect();
                require_holes_wound_against(&entries, &areas, &winds, signed, band)
            }
            Reading::Sphere {
                radius, outward, ..
            } => {
                let sphere = sphere_area(*radius);
                let outsides: Vec<Interval> = areas
                    .iter()
                    .zip(&turns)
                    .map(|(&l, &[k, _])| sphere_outside(*radius, *outward, l, k))
                    .collect();
                let lengths: Vec<Interval> = entries.iter().map(|e| loop_length(e)).collect();
                require_sphere_sides(sphere, &outsides, &lengths, area, band)
            }
            Reading::Torus { .. } => {
                let winds: Vec<bool> = turns.iter().map(|&k| k != [0, 0]).collect();
                require_holes_wound_against(&entries, &areas, &winds, signed, band)
            }
        })? {
            return Ok(out);
        }
        round += 1;
    }
}

/// What the lane reads per chart, before the rounds: the chart's
/// numbers and each loop's connectors ([`connector_terms`]).
enum Reading {
    /// The cylinder, about the anchor level ([`level_anchor`]).
    Cylinder {
        radius: Interval,
        anchor: f64,
        connectors: Vec<[Interval; 2]>,
    },
    /// The sphere, under its face's sense.
    Sphere {
        radius: Interval,
        outward: bool,
        connectors: Vec<[Interval; 1]>,
    },
    /// The torus `(R, r)`, under its face's sense, each loop's chain on
    /// the chart's lift.
    Torus {
        radii: (Interval, Interval),
        outward: bool,
        lifts: Vec<LoopLift>,
        connectors: Vec<[Interval; 2]>,
    },
}

/// A whole number of turns as an `f64`, exact for any count a loop
/// reaches.
#[allow(clippy::cast_precision_loss)]
fn lift_turns(k: i64) -> f64 {
    k as f64
}

/// **One torus loop's `[−∮F du, −∮G du]` on the chart's lift** (module
/// docs, the torus): each edge's terms on its own lift carried onto the
/// chain's, `−o·κτ·Δu` for its anchor's `o` whole turns
/// ([`LoopLift`]), its junctions' connectors, and for a loop that winds
/// the tube `W_v` times, `W_v·κτ·u₀` at its chain's start `u₀`: the
/// seam the chain crosses, read where it starts.
fn torus_loop(
    radii: (Interval, Interval),
    (edges, entries): (&[EdgeTermQ], &[EdgeEntry]),
    (connectors, lift, winds): ([Interval; 2], &LoopLift, i64),
    round: usize,
    removable: &mut f64,
) -> [Interval; 2] {
    let turn = torus_slopes(radii.0, radii.1).map(|k| k * crate::props::bracket::tau_bracket());
    let mut acc = connectors;
    for (((e, entry), &o), &du) in edges
        .iter()
        .zip(entries)
        .zip(&lift.anchors)
        .zip(&lift.turns)
    {
        let (t, rem) = torus_edge_terms(e, entry, radii, round);
        *removable += rem;
        let o = pt(lift_turns(o));
        acc = [0, 1].map(|k| acc[k] + t[k] - o * turn[k] * du);
    }
    if winds != 0 {
        let w = pt(lift_turns(winds));
        acc = [0, 1].map(|k| acc[k] + w * turn[k] * lift.start);
    }
    acc
}

/// A torus face's reading off its loops' `[−∮F du, −∮G du]`: its area
/// `|Ω|` and its flux about the centre.
struct TorusFace {
    area: Interval,
    flux: Interval,
}

/// **A torus face's region from its loops** (module docs, the torus):
/// its loops' signed area read under the face's sense, `X = s_f·ΣA`, is
/// the face's area up to whole tori, `|Ω| = X + N·4π²Rr` for the one `N`
/// that puts it in `(0, 4π²Rr]`, and its flux about the centre is
/// `ΣF + s_f·N·6π²Rr²`, the whole torus's flux at the same count. The
/// reading decides nothing: it holds every `N` whose `X + N·4π²Rr` meets
/// that window, the area clipped to it, so a round whose `X` is wide
/// reads wide and a later round narrows it, and a round that tells `N`
/// reads it alone. The window holds the whole torus, so a face bounded by
/// its chart's wrap loop reads whole.
fn torus_face(radii: (Interval, Interval), outward: bool, per_loop: &[[Interval; 2]]) -> TorusFace {
    let [flux_whole, area_whole] = torus_whole(radii.0, radii.1);
    let sense = pt(if outward { 1.0 } else { -1.0 });
    let flux = per_loop.iter().fold(Interval::zero(), |f, l| f + l[0]);
    let x = sense * per_loop.iter().fold(Interval::zero(), |a, l| a + l[1]);
    let refused = TorusFace {
        area: Interval::refused(),
        flux: Interval::refused(),
    };
    if !(x.is_certified() && area_whole.is_certified() && area_whole.lo() > 0.0) {
        return refused;
    }
    // Every `N` with `X + N·A` meeting `(0, A]`: `−X/A < N ≤ 1 − X/A`,
    // over-counted at the ends of `A`'s bracket, then each end moved in
    // past the counts that miss the window. They run unbroken, and `F` is
    // affine in `N`.
    let meets = |n: f64| {
        let area = x + pt(n) * area_whole;
        area.hi() > 0.0 && area.lo() <= area_whole.hi()
    };
    let mut lo = (-x.hi() / area_whole.lo())
        .min(-x.hi() / area_whole.hi())
        .floor();
    let mut hi = (1.0 - x.lo() / area_whole.lo())
        .max(1.0 - x.lo() / area_whole.hi())
        .ceil();
    if !(lo.is_finite() && hi.is_finite()) {
        return refused;
    }
    while lo <= hi && !meets(lo) {
        lo += 1.0;
    }
    while hi >= lo && !meets(hi) {
        hi -= 1.0;
    }
    if lo > hi {
        return refused;
    }
    // Two counts or more read the whole window: the lowest's value
    // reaches below `0` and the highest's above `A`, one torus apart.
    let window = Interval::from_bounds(0.0, area_whole.hi());
    TorusFace {
        area: if lo == hi {
            (x + pt(lo) * area_whole).meet(window)
        } else {
            window
        },
        flux: Interval::hull(
            flux + sense * pt(lo) * flux_whole,
            flux + sense * pt(hi) * flux_whole,
        ),
    }
}

/// **A sphere loop's side away from its face**: the metric area to the
/// loop's right under the face's normal, the region the loop alone
/// leaves outside the face (module docs, the sphere): `n·4πr² − X`, `X =
/// s_f·A₀ + 2πr²·W` its Green form `A₀` read with its winding `W`, and
/// `n` the whole number of spheres that leaves it in `[0, 4πr²)` at `X`'s
/// midpoint. The loops' decision ([`require_sphere_sides`]) reads where
/// the midpoint's `n` is not every point's.
fn sphere_outside(radius: Interval, outward: bool, area: Interval, turns: i64) -> Interval {
    let sphere = sphere_area(radius);
    #[allow(clippy::cast_precision_loss)]
    let x = (if outward { area } else { -area }) + pt(0.5) * sphere * pt(turns as f64);
    let n = (mid(x) / mid(sphere)).floor() + 1.0;
    pt(n) * sphere - x
}

/// **A sphere face's area** `|Ω| = 4πr² − Σ` its loops'
/// [`sphere_outside`] areas: the region left of every loop under the
/// face's normal.
fn sphere_face_area(
    radius: Interval,
    outward: bool,
    areas: &[Interval],
    turns: &[[i64; 2]],
) -> Interval {
    areas
        .iter()
        .zip(turns)
        .fold(sphere_area(radius), |a, (&l, &[k, _])| {
            a - sphere_outside(radius, outward, l, k)
        })
}

/// A loop's metric length from above.
fn loop_length(entries: &[EdgeEntry]) -> Interval {
    pt(entries.iter().map(|e| e.length).sum::<f64>())
}

/// **Every loop of a sphere face leaves area outside it, and the face
/// some inside them all**, decided at the interval scalar through the
/// closed forms' mean width `2·A/P`
/// ([`crate::props::curved::require_holes_wound_against`]'s predicates):
/// each loop's outside ([`sphere_outside`]) as a ring's hole
/// (`props_ring_winding`), and the face's own area `face` as its whole
/// (`props_chart_area_side`), each against its boundary's length. A
/// loop's outside read in `(0, whole)` and the face's area positive say
/// the whole spheres each was read with are the true ones. One whose
/// enclosure straddles a chart's multiple escalates, and a later round
/// narrows it ([`super::run::Run::ringed`]); a face whose area reads
/// negative has a loop wound with it ([`PropsError::RingWoundWithFace`]),
/// and one of no area is degenerate.
fn require_sphere_sides(
    whole: Interval,
    outsides: &[Interval],
    lengths: &[Interval],
    face: Interval,
    band: Band,
) -> Result<(), PropsError> {
    let total = lengths.iter().fold(Interval::zero(), |l, &x| l + x);
    let mut holes = Vec::with_capacity(2 * outsides.len());
    for (&outside, &length) in outsides.iter().zip(lengths) {
        holes.push((-outside, length));
        holes.push((outside - whole, length));
    }
    crate::props::curved::require_holes_wound_against(
        "props_chart_area_side",
        (face, total),
        &holes,
        band,
    )
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
        EdgeTermQ::Spiric(w) => w.ends[usize::from(end == w.edge.forward)],
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
        EdgeTermQ::Spiric(w) => (w.image.env, 0.0),
    };
    (1.5 * certified_hi(env) + bracket) * (1.0 + 1e-12)
}

/// **Every junction of every loop closes on the chart, and the loops
/// wind it as a face's boundary does** — the periodic-chart closure
/// decision ([`crate::props::closure`]) on the lane's chart images.
///
/// Each junction's gap is read off the two ends' chart images
/// ([`chart_end`]): the azimuth metered at the chart's lever over the
/// connector's levels ([`Junction::levels`]), the level at its own, and
/// the slack the two ends' [`end_slack`]
/// ([`crate::props::closure::interval_loop_turns`]); the level is
/// periodic on the torus alone. A projected or a spiric edge's ends are
/// its image's evaluator's and its turns the rule's, so it adds the
/// whole turns by which the two disagree ([`edge_branch`]), and the
/// loop's turns count as the integral reads them.
///
/// Returns, per loop, its turns `[W_u, W_v]`. On the cylinder and the
/// torus the loops wind as a face's boundary does
/// ([`crate::props::closure::require_boundary_windings`]). On the sphere
/// any azimuth turns are read: the poles' terms take the face's winding
/// with its sense, whatever the junctions at a pole took off
/// ([`sphere_outside`]).
fn loop_windings<T: Decide>(
    face: &RotationalFaceQ,
    entries: &[Vec<EdgeEntry>],
    band: Band,
) -> Result<Vec<[i64; 2]>, PropsError> {
    let chart = face.chart;
    let mut windings = Vec::with_capacity(face.loops.len());
    for (edges, entries) in face.loops.iter().zip(entries) {
        let readings = junctions(edges, entries).map(|j| {
            let [(u1, v1), (u0, v0)] = j.ends();
            crate::props::closure::IntervalJunction {
                delta: [u1 - u0, v1 - v0],
                lever: [chart.azimuth_lever(j.levels(chart)), chart.level_lever()],
                slack: j.slack(),
            }
        });
        let mut k = crate::props::closure::interval_loop_turns::<T, 2>(
            readings,
            [true, chart.level_periodic()],
            band,
        )?;
        for (e, entry) in edges.iter().zip(entries) {
            let b = edge_branch(e, entry, chart)?;
            k = [0, 1].map(|c| k[c].saturating_add(b[c]));
        }
        windings.push(k);
    }
    if !matches!(chart, RotationalChartQ::Sphere { .. }) {
        crate::props::closure::require_boundary_windings(&windings)?;
    }
    Ok(windings)
}

/// **The whole turns `[k_u, k_v]` an edge's turns make past its ends'
/// differences**, in its traversal ([`branch_turns`]): a projected or a
/// spiric edge's, the level's on the torus alone; a harmonic edge's ends
/// and terms read one channel, and make none.
fn edge_branch(
    e: &EdgeTermQ,
    entry: &EdgeEntry,
    chart: RotationalChartQ,
) -> Result<[i64; 2], PropsError> {
    let (Some([du, dv]), ends) = (
        entry.turn,
        match e {
            EdgeTermQ::Harmonic(_) => return Ok([0, 0]),
            EdgeTermQ::Projected(p) => p.ends,
            EdgeTermQ::Spiric(w) => w.ends,
        },
    ) else {
        return Ok([0, 0]);
    };
    let ku = branch_turns(ends[1].0 - ends[0].0, du)?;
    let kv = if chart.level_periodic() {
        branch_turns(ends[1].1 - ends[0].1, dv)?
    } else {
        0
    };
    Ok(if traversal(e) { [ku, kv] } else { [-ku, -kv] })
}

/// **The whole turns a turn makes past its ends' difference**, in its
/// carrier's order: `(Δ − (x₁ − x₀))/2π`, the ends read by the image's
/// evaluator, which places each on its piece's branch, and the turn `Δ`
/// by the rule ([`entry_turn`]). The two differ by whole turns where the
/// image's branches meet as its sectors say, so the enclosure must lie
/// within a quarter turn of one whole number; anything else is an image
/// whose evaluator and integral disagree, and refuses, as does a turn
/// [`entry_turn`]'s finest reading leaves wider than that.
fn branch_turns(ends: Interval, turn: Interval) -> Result<i64, PropsError> {
    let tau = crate::props::bracket::tau_bracket();
    let gap = (turn - ends) / tau;
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

/// **One torus loop's chain on the chart's continuous lift** (module
/// docs, the torus): per edge in traversal order, the whole turns `o` by
/// which the chain's level sits above the edge's own lift where the
/// edge's terms anchor it (its carrier interval's start), and per
/// junction those at the edge before's end; per edge its turn `Δu`, its
/// ends' azimuths apart plus its branch turns, which the evaluator's
/// ends read thin where the rule's turn is wide on a coarse round; and
/// the azimuth where the chain starts, the first edge's traversal start.
/// The chain starts on
/// the first edge's own lift; past each edge it gains the edge's branch
/// turns ([`edge_branch`]), and past each junction the whole turns the
/// closure took off its level gap, so it ends `W_v` turns up.
struct LoopLift {
    anchors: Vec<i64>,
    ends: Vec<i64>,
    turns: Vec<Interval>,
    start: Interval,
}

fn loop_lifts(
    face: &RotationalFaceQ,
    entries: &[Vec<EdgeEntry>],
) -> Result<Vec<LoopLift>, PropsError> {
    face.loops
        .iter()
        .zip(entries)
        .map(|(edges, entries)| {
            let (mut anchors, mut ends, mut turns) = (Vec::new(), Vec::new(), Vec::new());
            let mut c = 0i64;
            for j in junctions(edges, entries) {
                let (e, entry) = j.before;
                let [bu, b] = edge_branch(e, entry, face.chart)?;
                turns.push(
                    chart_end(e, true).0 - chart_end(e, false).0
                        + pt(lift_turns(bu)) * crate::props::bracket::tau_bracket(),
                );
                anchors.push(if traversal(e) { c } else { c + b });
                ends.push(c + b);
                let [(_, v1), (_, v0)] = j.ends();
                // Finite: the closure decided every junction.
                #[allow(clippy::cast_possible_truncation)]
                let m = crate::props::closure::nearest_turns(v1 - v0) as i64;
                c = c.saturating_add(b).saturating_add(m);
            }
            let start = edges
                .first()
                .map_or_else(Interval::refused, |e| chart_end(e, false).0);
            Ok(LoopLift {
                anchors,
                ends,
                turns,
                start,
            })
        })
        .collect()
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
        Ok(geom::bracket_mid((lo, hi)))
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
    areas: &[Interval],
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
        .zip(areas)
        .zip(winds)
        .enumerate()
        .filter(|&(i, (_, &winds))| !winds && (band_face || i > 0))
        .map(|(_, ((edges, &ring), _))| (ring, length(edges)))
        .collect();
    crate::props::curved::require_holes_wound_against("props_chart_area_side", whole, &holes, band)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod swap_rows;

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

    /// The face of harmonic `loops` on the unit cylinder, its chart term
    /// zero.
    fn unit_face(loops: &[&[TrimEdgeQ]]) -> RotationalFaceQ {
        RotationalFaceQ {
            chart: RotationalChartQ::Cylinder { radius: pt(1.0) },
            chart_term: ChartTermQ {
                closed: Interval::zero(),
                lever: [Interval::zero(); 3],
                rationals: Vec::new(),
                spirics: Vec::new(),
                ribbon: 0.0,
            },
            loops: loops
                .iter()
                .map(|edges| edges.iter().copied().map(EdgeTermQ::Harmonic).collect())
                .collect(),
        }
    }

    fn chan(c0: f64, ca: f64, cb: f64, cl: f64) -> HarmChan {
        HarmChan {
            c0: pt(c0),
            ca: pt(ca),
            cb: pt(cb),
            cl: pt(cl),
        }
    }

    /// A rim `v = level` over `u ∈ us` and a ruling `u = at` over
    /// `v ∈ vs`, each read with the envelope `env`.
    fn rim_q(level: f64, us: (f64, f64), forward: bool, env: f64) -> TrimEdgeQ {
        TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, 1.0),
            v: chan(level, 0.0, 0.0, 0.0),
            t0: pt(us.0),
            t1: pt(us.1),
            forward,
            trig0: crate::props::trig::trig_at_limit(pt(us.0)),
            trig1: Some(crate::props::trig::trig_at_limit(pt(us.1))),
            env: pt(env),
        }
    }

    fn ruling_q(at: f64, vs: (f64, f64), forward: bool, env: f64) -> TrimEdgeQ {
        TrimEdgeQ {
            u: chan(at, 0.0, 0.0, 0.0),
            v: chan(0.0, 0.0, 0.0, 1.0),
            ..rim_q(0.0, vs, forward, env)
        }
    }

    /// **A junction's connector is in the region term**: a tall
    /// rectangle `[0, 1] × [0, 100]` whose top is four rims, each image
    /// short of its vertices by `δ` (within its envelope `δ`, so the
    /// closure accepts every junction). The image chain leaves `8δ` of
    /// the top open at a level `50` above the anchor, `400δ` of area and
    /// flux, past the map-residual pad's `202δ`: only the connectors
    /// close it, and the enclosures hold the closed forms.
    #[test]
    fn a_tall_face_whose_images_stop_short_holds_its_closed_form() {
        let (h, d) = (100.0, 1e-4);
        let mut edges = vec![
            rim_q(0.0, (0.0, 1.0), true, d),
            ruling_q(1.0, (0.0, h), true, d),
        ];
        for k in 0..4 {
            let (a, b) = (0.75 - 0.25 * f64::from(k), 1.0 - 0.25 * f64::from(k));
            edges.push(rim_q(h, (a + d, b - d), false, d));
        }
        edges.push(ruling_q(0.0, (0.0, h), false, d));
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let got = rotational_cut_face_rounds::<f64>(
            &unit_face(&[&edges]),
            band,
            RoundWindow::SCHEDULE,
            &mut to_target(1e-9),
        )
        .expect("the tall face measures")
        .bounds;
        test_target::encloses(got.flux, h, "the tall face's flux");
        test_target::encloses(got.area, h, "the tall face's area");
    }

    /// **The connectors' corners are padded**: the unit square, each side
    /// ten pieces whose images are moved out by `e` and cover the middle
    /// half of their piece, each within its envelope. The image chain
    /// closed by its forty connectors bounds about `1 + 4e`, and the
    /// map-residual pad, read along the images, pays only `2.8e` of the
    /// `4e` strip between the chain and the square: the rest lies beside
    /// the connectors, which only the connectors' own pad holds.
    #[test]
    fn images_short_of_their_pieces_and_moved_out_hold_the_square() {
        let n = 10;
        let e = 0.25 / f64::from(n);
        let env = 1.42 * e;
        let piece = |k: u32| {
            (
                f64::from(k) / f64::from(n) + e,
                f64::from(k + 1) / f64::from(n) - e,
            )
        };
        let flip = |(a, b): (f64, f64)| (1.0 - b, 1.0 - a);
        let mut edges = Vec::new();
        edges.extend((0..n).map(|k| rim_q(-e, piece(k), true, env)));
        edges.extend((0..n).map(|k| ruling_q(1.0 + e, piece(k), true, env)));
        edges.extend((0..n).map(|k| rim_q(1.0 + e, flip(piece(k)), false, env)));
        edges.extend((0..n).map(|k| ruling_q(-e, flip(piece(k)), false, env)));
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let got = rotational_cut_face_rounds::<f64>(
            &unit_face(&[&edges]),
            band,
            RoundWindow::SCHEDULE,
            &mut to_target(1e-9),
        )
        .expect("the square measures")
        .bounds;
        test_target::encloses(got.flux, 1.0, "the square's flux");
        test_target::encloses(got.area, 1.0, "the square's area");
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
        let out = met(rotational_cut_face_rounds::<f64>(
            &unit_face(&[&edges]),
            band,
            RoundWindow::SCHEDULE,
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
                a + cylinder_edge_terms(e, &entry, pt(1.0), anchor, 0).0[0]
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
            let at0 = rotational_cut_face_rounds::<f64>(
                &unit_face(&loops),
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
            let whole = met(rotational_cut_face_rounds::<f64>(
                &unit_face(&loops),
                band,
                RoundWindow::SCHEDULE,
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
        let read = |window| match rotational_cut_face_rounds::<f64>(
            &unit_face(&[&outer, &ring]),
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
        let out = met(rotational_cut_face_rounds::<f64>(
            &unit_face(&[&outer, &ring]),
            band,
            RoundWindow::SCHEDULE,
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
            let got = rotational_cut_face_rounds::<f64>(
                &unit_face(&loops),
                band,
                RoundWindow::at(round),
                &mut to_target(eps),
            );
            assert!(
                matches!(got, Err(PropsError::RingWoundWithFace)),
                "round {round}: {got:?}"
            );
        }
        let got = met(rotational_cut_face_rounds::<f64>(
            &unit_face(&loops),
            band,
            RoundWindow::SCHEDULE,
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
        let out = met(rotational_cut_face_rounds::<f64>(
            &unit_face(&[&outer, &ring]),
            band,
            RoundWindow::SCHEDULE,
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
            let out = met(rotational_cut_face_rounds::<f64>(
                &unit_face(&[&[bottom], &[top]]),
                band,
                RoundWindow::SCHEDULE,
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
                met(rotational_cut_face_rounds::<f64>(
                    &unit_face(loops),
                    band,
                    RoundWindow::SCHEDULE,
                    &mut to_target(eps),
                )),
            ),
            (
                "Interval",
                met(rotational_cut_face_rounds::<Interval>(
                    &unit_face(loops),
                    band,
                    RoundWindow::SCHEDULE,
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
        let run = rotational_cut_face_rounds::<f64>(
            &unit_face(&[&[e]]),
            band,
            RoundWindow::SCHEDULE,
            &mut to_target(eps),
        )
        .expect("the hopeless loop has an enclosure");
        assert_eq!(
            (run.end, run.round, run.rounds),
            (LaneEnd::Floor, 1, 2),
            "the endpoint pad, and the connector its bracket's slack opens at the junction, \
             are a floor round 1 sits on: {run:?}"
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
            match rotational_cut_face_rounds::<f64>(
                &unit_face(&[&edges]),
                band,
                RoundWindow::SCHEDULE,
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
