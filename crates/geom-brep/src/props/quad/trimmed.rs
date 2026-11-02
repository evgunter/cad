//! **The trimmed-region lane** ([`trimmed_patch_face_rounds`]): Green's
//! theorem on the trim loop's own chart image — its chords, lunes and
//! pads, and the chord polygon's exact and composite rules.

use geom_core::Bounds;
use geom_core::interval::certification::Certification;
use geom_core::interval::{Interval, certified_hi, cross, dot};
use geom_core::spline::algebra;
use geom_core::spline::{
    KnotVector, ParamRange, RingRefinement, constant_weight, restrict_segment,
};
use geom_core::{Band, Decide, InfSpeed, Margin, Sign};

use super::net::{
    Collapse, PatchGrid, PatchNet, RVec3, certified_box, grid_vec, knots_inside,
    newton_cotes_weights, norm_hi, normal_partials, refused_vec, rv_add, widen,
};
use super::run::{Accept, FaceCutBounds, LaneRun, RoundWindow, Run, face_extent};
use crate::offset_meters::mig;
use crate::props::bracket::{lo_or_refuse, mid, pt};
use crate::props::{PropsCheck, PropsError};

/// Weight of one chord cell of the trimmed lane's exact arm, by the
/// chart's bidegree `(p_u, p_v)`, read at face entry. A cell is one nested
/// Newton–Cotes pass of `(3p_u + 3p_v)·(3p_v)` chart evaluations, so its
/// cost follows the bidegree and not the chart's shape: a flat and a
/// curved bilinear chart cost the same (0.20 ms a cell, measured), a
/// bicubic-in-v one fifty times that. Measured on this box in release
/// (a timing probe, reverted), rounds 5 and 6 of one arc trim, twice:
/// the dearest of the four readings, over a rational composite cell's
/// 9.1 µs (the unit's ten). The readings vary by up to 1.6× run to run,
/// so the dearest is taken, as for every other lane. The exact arm
/// runs only inside its window, `p_u + p_v ≤ 4`.
///
/// | `(p_u, p_v)` | ms / cell | weight |
/// |---|---|---|
/// | (1, 1) | 0.471 | 520 |
/// | (2, 1) | 1.100 | 1 210 |
/// | (3, 1) | 2.971 | 3 300 |
/// | (1, 2) | 3.905 | 4 300 |
/// | (2, 2) | 6.064 | 6 700 |
/// | (1, 3) | 10.59 | 11 700 |
const fn w_trimmed(p_u: usize, p_v: usize) -> usize {
    match (p_u, p_v) {
        (1, 1) => 520,
        (2, 1) => 1_210,
        (3, 1) => 3_300,
        (1, 2) => 4_300,
        (2, 2) => 6_700,
        _ => 11_700,
    }
}

/// Weight of one piece of the trimmed lane's composite arm, by the
/// chart's bidegree: a piece is one inner Newton–Cotes column of `f`
/// at its midpoint, one of `f_uu` over it, and one read of `f_u` and
/// `f_v`. Measured as [`w_trimmed`] was, in release on this box (a
/// timing probe, reverted), rounds 5 and 6 of the iso triangle on the
/// curved chart of each bidegree (one interior knot each way), twice:
/// the dearest of the four readings. The same probe re-read the exact
/// arm's cells at 0.56–0.94 of that table's times, so a reading here is
/// taken to that table's scale by the factor 1.5, a little above the
/// readings' median ratio `1/0.70`, at 1 104 weight a millisecond. A
/// bidegree the table does not list takes `64·(p_u + 1)·(p_v + 1)²`,
/// never below the dearest listed weight:
/// the formula is above every listed weight at that weight's own
/// bidegree, so it is meant to dominate a measured reading the table
/// lacks, and the floor keeps an unlisted bidegree from costing less
/// than a listed one it outranks.
///
/// | `(p_u, p_v)` | ms / piece | weight |
/// |---|---|---|
/// | (4, 1) | 0.424 | 700 |
/// | (5, 1) | 0.427 | 710 |
/// | (3, 2) | 0.616 | 1 020 |
/// | (4, 2) | 0.982 | 1 630 |
/// | (2, 3) | 1.070 | 1 780 |
/// | (3, 3) | 1.580 | 2 620 |
/// | (4, 3) | 2.295 | 3 810 |
/// | (1, 4) | 1.833 | 3 040 |
/// | (2, 4) | 2.351 | 3 900 |
/// | (3, 4) | 3.378 | 5 610 |
/// | (4, 4) | 4.207 | 6 980 |
const fn w_trimmed_composite(p_u: usize, p_v: usize) -> usize {
    match (p_u, p_v) {
        (4, 1) => 700,
        (5, 1) => 710,
        (3, 2) => 1_020,
        (4, 2) => 1_630,
        (2, 3) => 1_780,
        (3, 3) => 2_620,
        (4, 3) => 3_810,
        (1, 4) => 3_040,
        (2, 4) => 3_900,
        (3, 4) => 5_610,
        (4, 4) => 6_980,
        _ => {
            let formula = 64 * (p_u + 1) * (p_v + 1) * (p_v + 1);
            if formula > 6_980 { formula } else { 6_980 }
        }
    }
}

/// Pieces per `General` image at round 0, doubling per round — the
/// lever of the lune pad, which is `O(h²)` in the piece length.
const IMAGE_INIT_PIECES: usize = 8;
/// Bisections a piece whose monotonicity is not definite is refined by
/// before the lane refuses it (fixed, D9 — never data-dependent). Each
/// bisection halves the chord, so a piece that is a graph over its
/// chord anywhere becomes one within a few levels; a cusp or a fold
/// never does, and refusing is the honest answer for it.
const MONOTONE_BISECTIONS: usize = 4;
/// Cells per axis of the trimmed lane's AREA rule under ONE chord: its
/// `u` span, and its column in `v` ([`chord_area_cuts`]).
///
/// The rule under each chord is the rectangle lane's own cell rule
/// ([`chord_polygon_area`]), so its pad is `½h_u·sup|∂_u g| +
/// `½h_v·sup|∂_v g|` per cell and the resolution is what sets the
/// area's honest width, exactly as it does there. A lower count was
/// tried and is what a curved chart cannot afford: at 16 the pad on an
/// ordinary bidegree-(2,2) chart is the size of the face's own area,
/// `area.lo()` goes negative and the extent gate answers
/// `DegenerateFace`.
const CHORD_AREA_CELLS: usize = 64;

/// A chart-space point bracket (`(u, v)`).
type RPt2 = (Interval, Interval);

/// The bracketed control net of one trim edge's **`General` chart
/// image**, on the carrier's own parameter.
///
/// Non-rational by construction: `general_image_lane` interpolates its
/// foot samples with unit weights, and the piece boxes below are the
/// POLYNOMIAL convex-hull fact. A net whose weights are not one
/// constant ([`constant_weight`]) refuses typed rather than reading the
/// hull of a quotient.
#[derive(Clone, Debug)]
pub struct TrimPiece {
    knots: KnotVector,
    control: Vec<RPt2>,
    weights: Vec<f64>,
}

impl TrimPiece {
    /// The image with knot vector `knots`, bracketed control polygon
    /// `control` (one `(u, v)` pair per point) and `weights` (all
    /// `1.0`, or the door refuses).
    ///
    /// # Errors
    ///
    /// [`PropsError::QuadratureUnsupported`] when `control` is not
    /// `knots`' count, or `weights` not one per point.
    pub fn new(
        knots: KnotVector,
        control: Vec<RPt2>,
        weights: Vec<f64>,
    ) -> Result<Self, PropsError> {
        if control.len() != knots.control_count() || weights.len() != control.len() {
            return Err(PropsError::QuadratureUnsupported {
                what: "a trim image whose control polygon is not its knot vector's count, \
                       or whose weights are not one per control point",
            });
        }
        Ok(Self {
            knots,
            control,
            weights,
        })
    }

    /// The image's knot vector.
    #[must_use]
    pub fn knots(&self) -> &KnotVector {
        &self.knots
    }

    /// The bracketed control polygon.
    #[must_use]
    pub fn control(&self) -> &[RPt2] {
        &self.control
    }

    /// The image's weights.
    #[must_use]
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }
}

/// One trim-loop EDGE of the trimmed lane, fully bracketed by the
/// caller (key-free, like [`TrimEdgeQ`](super::cylinder::TrimEdgeQ)).
///
/// An iso image is one exact chord and carries no `piece`: its
/// endpoints are structure (`p0 → p0 + pd`), so there is no arc to
/// bound and no box to pad. A `General` image carries its net, and the
/// engine subdivides it per round — the chords are the lane's lever,
/// so they are not the caller's to fix.
#[derive(Clone, Debug)]
pub struct TrimChord {
    /// The traversal-start chart point.
    pub a: RPt2,
    /// The traversal-end chart point.
    pub b: RPt2,
    /// The `General` image, when the edge carries one.
    pub piece: Option<TrimPiece>,
    /// Whether the image's parameter INCREASES along the traversal —
    /// read only when `piece` is `Some` (an iso chord states its own
    /// traversal in `a`/`b`).
    pub forward: bool,
    /// The stored certificate's envelope: metres, through the map.
    ///
    /// The edge's metric LENGTH, which the envelope pad multiplies, is
    /// NOT a caller's field: the door already holds the chord (and, for
    /// a `General`, its whole control polygon) and the chart, so it
    /// derives the length itself ([`polygon_variation`] crossed by the
    /// chart's metric rate). A lane that took a caller's perimeter is
    /// `work/quadtail/quad-face-extent-trusts-caller-perimeter.md` (#1368,
    /// open), and this one does not open a second instance of it.
    pub env: Interval,
}

/// Midpoint of a bracket pair — the chord polygon's vertex.
///
/// The polygon is built on f64 vertices deliberately: every structural
/// decision below (which knot cell a sub-chord lies in, where it
/// crosses a `v`-knot) is exact structure, and the distance from the
/// bracket midpoint to the true image endpoint is repaid by the VERTEX
/// PAD, never assumed away.
fn vertex(p: RPt2) -> (f64, f64) {
    (mid(p.0), mid(p.1))
}

/// The half-width of a chart-point bracket, as a chart-space radius
/// (the 1-norm over-bounds the 2-norm; only an upper bound is read).
fn vertex_slack(p: RPt2) -> f64 {
    0.5 * (p.0.width() + p.1.width())
}

/// Bisection of a bracketed Bézier control polygon: the two halves at
/// `λ = ½`, each again a Bézier polygon of the same degree whose first
/// and last points are the arc's endpoints there and whose hull
/// contains its half of the arc. Each channel's row is restricted to
/// `[0, ½]` and `[½, 1]` of its segment on `[0, 1]`
/// ([`restrict_segment`]), so both halves are certified enclosures of
/// the same arc rather than re-approximations of it.
fn bezier_bisect(block: &[RPt2]) -> (Vec<RPt2>, Vec<RPt2>) {
    #[cfg(test)]
    tests::BISECTED.with(|b| b.borrow_mut().push(block.to_vec()));
    let half = |channel: &[Interval], ends: (f64, f64)| restrict_segment(channel, (0.0, 1.0), ends);
    let xs: Vec<Interval> = block.iter().map(|q| q.0).collect();
    let ys: Vec<Interval> = block.iter().map(|q| q.1).collect();
    let zip = |x: Vec<Interval>, y: Vec<Interval>| x.into_iter().zip(y).collect::<Vec<RPt2>>();
    (
        zip(half(&xs, (0.0, 0.5)), half(&ys, (0.0, 0.5))),
        zip(half(&xs, (0.5, 1.0)), half(&ys, (0.5, 1.0))),
    )
}

/// Why [`bezier_blocks`] refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BlockRefusal {
    /// The weights are not one constant: the blocks' hulls would not be
    /// the polynomial convex-hull fact.
    Rational,
    /// A zero degree, or a refusing knot algebra.
    Malformed,
}

/// **Bézier extraction of a bracketed image net** at its own knots and
/// `m − 1` uniform cuts: every interior break is raised to
/// multiplicity `degree`, so the refined polygon splits into per-span
/// blocks whose first and last points ARE the arc's endpoints there
/// (the clamped-end fact) and whose hull is the span's box (the
/// convex-hull property). Knot insertion is exact in ℝ, so a block is
/// a statement about the same arc.
///
/// The net's weights must be one constant ([`constant_weight`]): that
/// makes each axis its own polynomial channel below, and the blocks'
/// hulls the polynomial convex-hull fact. A rational net refuses
/// ([`BlockRefusal::Rational`]), as does a malformed one: a zero
/// degree, or a knot algebra that refuses ([`BlockRefusal::Malformed`]).
fn bezier_blocks(img: &TrimPiece, m: usize) -> Result<Vec<Vec<RPt2>>, BlockRefusal> {
    let kv = &img.knots;
    let p = kv.degree();
    if p == 0 {
        return Err(BlockRefusal::Malformed);
    }
    if constant_weight(&img.weights).is_none() {
        return Err(BlockRefusal::Rational);
    }
    let mut breaks: Vec<f64> = kv.interior_knots().map(|(k, _)| k).collect();
    // A uniform cut near a knot is dropped and the knot stands: the
    // narrow span between them would carry the insertion's rounding,
    // and the block list a box of next to no width.
    breaks.extend(algebra::domain_grid_points(kv, m));
    breaks.sort_by(f64::total_cmp);
    breaks.dedup();
    let mut add: Vec<f64> = Vec::new();
    for b in &breaks {
        let mult = kv.multiplicity_of(*b).map_or(0, |(m, _)| m);
        for _ in mult..p {
            add.push(*b);
        }
    }
    // A constant weight makes the chart net its own polynomial form, so
    // each axis refines as a channel in the certification ring.
    let refinement = RingRefinement::new(kv, &add).map_err(|_| BlockRefusal::Malformed)?;
    let cu: Vec<Interval> = img.control.iter().map(|q| q.0).collect();
    let cv: Vec<Interval> = img.control.iter().map(|q| q.1).collect();
    let ctl: Vec<RPt2> = refinement
        .refine(&cu)
        .into_iter()
        .zip(refinement.refine(&cv))
        .collect();
    if ctl.len() < p + 1 || !(ctl.len() - 1).is_multiple_of(p) {
        return Err(BlockRefusal::Malformed);
    }
    Ok((0..(ctl.len() - 1) / p)
        .map(|k| ctl[k * p..=k * p + p].to_vec())
        .collect())
}

/// The axis-aligned chart box of a bracketed control block — the
/// convex-hull property, read per axis.
fn block_box(block: &[RPt2]) -> (Interval, Interval) {
    let mut bu = block[0].0;
    let mut bv = block[0].1;
    for q in &block[1..] {
        bu = Interval::hull(bu, q.0);
        bv = Interval::hull(bv, q.1);
    }
    (bu, bv)
}

/// Sound LOWER bound on the Euclidean norm of a bracketed 3-vector:
/// `|v|² = Σ vᵢ² ≥ Σ (min|vᵢ|)²`, and a component whose bracket
/// straddles zero contributes nothing — so a hull that has lost the
/// direction answers `0`, which is the refusing direction wherever
/// this is read.
fn norm_lo(v: RVec3) -> f64 {
    let comp = |x: Interval| Interval::point(mig(x));
    let root = (comp(v[0]).sqr() + comp(v[1]).sqr() + comp(v[2]).sqr()).sqrt();
    if !root.is_certified() { 0.0 } else { root.lo() }
}

/// One piece of the chord polygon: its chord, and — for a `General`
/// arc — the lune between arc and chord, as the region the integrand
/// sups are read over and the chart AREA that region's lune occupies.
struct TrimCell {
    /// Which [`TrimChord`] of the loop this cell came from — the
    /// envelope pad is stated per EDGE and this lane's chords are the
    /// edges' pieces.
    edge: usize,
    a: (f64, f64),
    b: (f64, f64),
    /// The axis-aligned chart hull of the piece's control polygon —
    /// where `sup|f|` and `sup g` are read. `None` for an exact chord
    /// (an iso image, or a degree-1 block, which IS its chord).
    hull: Option<(Interval, Interval)>,
    /// The chart area of the lune, `0` for an exact chord.
    ///
    /// **The box is the chord's own frame's, not the axes'.** The lune
    /// lies inside the control polygon's hull, and that hull's
    /// OBLIQUE box — the extent along the chord by the extent across
    /// it — is `h × sagitta = O(h³)` for a smooth arc, so the sum over
    /// `L/h` pieces is `O(L³κ/m²)`, second order in the round's lever.
    /// The AXIS-ALIGNED box is `O(h²)` on a chord that is neither
    /// horizontal nor vertical, whose sum is first order and never
    /// leaves the round window.
    lune_area: f64,
    /// The cell's own chart-space VARIATION, `(Σ|Δu|, Σ|Δv|)` over its
    /// control polygon — an upper bound on the arc's, by the
    /// variation-diminishing property. Crossed to metres by the
    /// chart's own rate it gives the cell's metric length, which is
    /// what the envelope pad and the extent gate's perimeter are built
    /// from. The door computes it here rather than taking a caller's
    /// number: `work/quadtail/quad-face-extent-trusts-caller-perimeter.md`
    /// (#1368) is the class a trusted perimeter lands in.
    var: (f64, f64),
    /// The chart-space radius of the wider of the two endpoint
    /// brackets — the vertex pad's lever.
    slack: f64,
}

/// The total variation of a bracketed control polygon per axis, taken
/// outward off the brackets: an upper bound on the arc's own, by the
/// variation-diminishing property of the B-spline basis.
fn polygon_variation(block: &[RPt2]) -> (f64, f64) {
    let mut du = 0.0f64;
    let mut dv = 0.0f64;
    for w in block.windows(2) {
        du += (w[1].0 - w[0].0).mag();
        dv += (w[1].1 - w[0].1).mag();
    }
    (du, dv)
}

/// The lune's chart area for one bracketed control block over its own
/// chord: the extent ALONG the chord times the extent ACROSS it, both
/// taken outward off the brackets. Exactly zero for a two-point block
/// — a degree-1 arc IS its chord, so there is no lune at all, which is
/// every image this head's producer mints (`PXN_IMAGE_DEGREE = 1`).
fn lune_area(block: &[RPt2], a: (f64, f64), b: (f64, f64)) -> f64 {
    let (cu, cv) = (b.0 - a.0, b.1 - a.1);
    let len = (cu.powi(2) + cv.powi(2)).sqrt();
    if block.len() <= 2 {
        return 0.0;
    }
    if len <= 0.0 || !len.is_finite() {
        // No chord direction to resolve the hull in: fall back to the
        // axis-aligned box, which contains the oblique one.
        let (bu, bv) = block_box(block);
        return bu.width() * bv.width();
    }
    let (du, dv) = (pt(cu / len), pt(cv / len));
    let project = |k: fn(RPt2, Interval, Interval) -> Interval| -> f64 {
        let mut acc = k(block[0], du, dv);
        for q in &block[1..] {
            acc = Interval::hull(acc, k(*q, du, dv));
        }
        acc.width()
    };
    let along = project(|q, du, dv| q.0 * du + q.1 * dv);
    let across = project(|q, du, dv| q.1 * du - q.0 * dv);
    along * across
}

/// The typed refusal a piece that is not a graph over its chord earns,
/// named once so the row and the site cannot drift apart.
const TRIM_NOT_MONOTONE: &str = "a trim piece's monotonicity over its own chord is \
                                 UNRESOLVED at this lane's resolution — the control \
                                 polygon still advances backwards along the chord after \
                                 the fixed bisection depth, which a cusp or a feature \
                                 below the round's lever both produce and which the lune \
                                 bound's |w| ≤ 1 premise does not cover; the lane refuses \
                                 here rather than padding";

/// The typed refusal a caller's chord list that does not CLOSE earns.
///
/// Green's theorem is a statement about a closed curve; an open walk
/// has no winding number and the lane's answer for one would be a
/// certified number for no region at all. The rectangle certificate
/// refuses the analogous shape (its shoelace must equal ±the
/// rectangle's area), and this is that guard for a polygon whose
/// vertices are brackets: consecutive chords must meet inside their
/// own brackets, and the in-bracket disagreement is PADDED rather
/// than assumed away.
const TRIM_OPEN_WALK: &str = "the trim loop's chord walk does not close — some chord's end \
                              lies outside the bracket of the next chord's start, and \
                              Green's theorem has no winding number for an open curve; a \
                              loop assembled from one face's half-edges always closes, so \
                              this is a caller handing the door a walk it did not walk";

/// The typed refusal a chart outside the INNER rule's node window
/// earns. The outer rule has a composite arm past its own window; the
/// inner one has none.
const TRIM_NC_WINDOW: &str = "trimmed lane's inner Newton–Cotes window — the chord integrand \
                              has v-degree 3·p_v − 1 and the exact closed rule's INTERVAL \
                              count tops out at 12 (13 nodes), so the inner rule certifies \
                              p_v ≤ 4; the composite arm is the OUTER rule's only";

/// **`sign` of the chart-space monotonicity of one piece over its own
/// chord**, decided under `props_trim_piece_monotone`.
///
/// The margin is the control polygon's LEAST advance along the unit
/// chord direction — a chart-space span, a convexity fact on the
/// derivative polygon (the Bézier differences are the derivative's own
/// coefficients up to the positive factor `p/h`, so their signs decide
/// `⟨P′, c⟩` without evaluating it) — crossed to metres by the chart's
/// metric rate along that same direction, bounded BELOW over the piece
/// box. A hull that has lost the direction answers rate `0`, hence
/// margin `0`, hence refine-then-refuse: every rounding here runs
/// toward refusing.
///
/// **The margin carries the ROUND's lever as well as the image's**, and
/// a reader has to know it: the span is the REFINED block's advance,
/// so it halves with every uniform cut and every bisection, and it
/// scales with the face. Measured: the same smooth parabolic arc
/// certifies at every round on a 10 µm face, refuses at round 4 on a
/// 1 µm face, and refuses at every round on a 0.1 µm face. That is the
/// lane's resolution speaking, which is why the refusal names an
/// unresolved margin rather than a cusp. Nothing on today's corpus is
/// near the band; the audit row and the K roster entry say the same.
fn piece_monotone<T: Decide>(
    block: &[RPt2],
    a: (f64, f64),
    b: (f64, f64),
    su: Option<&PatchGrid>,
    sv: Option<&PatchGrid>,
    band: Band,
) -> Result<Sign, PropsError> {
    let (cu, cv) = (b.0 - a.0, b.1 - a.1);
    let len = (cu.powi(2) + cv.powi(2)).sqrt();
    if len <= 0.0 || !len.is_finite() {
        // A chord of zero length has no direction to be monotone over.
        return Ok(Sign::Zero);
    }
    let (du, dv) = (pt(cu / len), pt(cv / len));
    let mut span = f64::INFINITY;
    for w in block.windows(2) {
        span = span.min(lo_or_refuse(
            (w[1].0 - w[0].0) * du + (w[1].1 - w[0].1) * dv,
        ));
    }
    let (bu, bv) = block_box(block);
    let (gu, gv) = match certified_box(bu, bv) {
        Some(over) => (grid_vec(su, over.0, over.1), grid_vec(sv, over.0, over.1)),
        None => (refused_vec(), refused_vec()),
    };
    let rate = norm_lo(core::array::from_fn(|k| gu[k] * du + gv[k] * dv));
    // The INF door by signature (`predicate.rs`'s rate pair): this is a
    // "definitely apart" claim — the piece advances along its chord by
    // at least this much — and `rate` is a certified LOWER bound, so
    // under-stating it can only fail to certify, never certify a
    // sliver.
    super::run::classify_len::<T>(
        "props_trim_piece_monotone",
        Margin::metered(span, InfSpeed::new(rate)),
        band,
        PropsCheck::Inventory,
    )
}

/// The round's chord polygon: one exact chord per iso edge, and per
/// `General` edge the Bézier blocks of its image at this round's
/// lever, each decided monotone over its own chord and bisected while
/// the verdict is short of definite.
fn trim_cells<T: Decide>(
    chords: &[TrimChord],
    m: usize,
    su: Option<&PatchGrid>,
    sv: Option<&PatchGrid>,
    band: Band,
) -> Result<Vec<TrimCell>, PropsError> {
    let mut out: Vec<TrimCell> = Vec::new();
    for (edge, c) in chords.iter().enumerate() {
        let Some(img) = &c.piece else {
            out.push(TrimCell {
                edge,
                a: vertex(c.a),
                b: vertex(c.b),
                hull: None,
                lune_area: 0.0,
                var: polygon_variation(&[c.a, c.b]),
                slack: vertex_slack(c.a).max(vertex_slack(c.b)),
            });
            continue;
        };
        let blocks = bezier_blocks(img, m).map_err(|refusal| match refusal {
            BlockRefusal::Rational => PropsError::QuadratureUnsupported {
                what: "a RATIONAL General trim image reached the trimmed lane — the piece \
                       boxes and the monotonicity row are the polynomial convex-hull fact, \
                       and no shipped producer mints one (general_image_lane interpolates \
                       with unit weights)",
            },
            BlockRefusal::Malformed => PropsError::QuadratureUnsupported {
                what: "a General trim image whose knot algebra refused its Bézier \
                       extraction — a malformed stored net",
            },
        })?;
        // The traversal, not the storage: a half-edge walked backwards
        // presents its image's pieces in reverse and each piece's own
        // polygon reversed, so the polygon below is one closed walk.
        let mut blocks: Vec<Vec<RPt2>> = blocks;
        if !c.forward {
            blocks.reverse();
            for b in &mut blocks {
                b.reverse();
            }
        }
        // The polygon's SHARED vertices are the caller's `a`/`b`, not
        // the extraction's clamped ends: the two are the same point up
        // to the refinement's rounding, and hulling them together
        // enlarges a hull that already contains the arc (so the
        // convex-hull fact survives) while giving the closure check one
        // bracket per vertex instead of two readings of it.
        let hull2 =
            |x: RPt2, y: RPt2| -> RPt2 { (Interval::hull(x.0, y.0), Interval::hull(x.1, y.1)) };
        if let Some(first) = blocks.first_mut() {
            first[0] = hull2(first[0], c.a);
        }
        if let Some(last) = blocks.last_mut() {
            let n = last.len() - 1;
            last[n] = hull2(last[n], c.b);
        }
        let mut stack: Vec<(Vec<RPt2>, usize)> =
            blocks.into_iter().rev().map(|b| (b, 0usize)).collect();
        while let Some((block, depth)) = stack.pop() {
            let a = vertex(block[0]);
            let b = vertex(block[block.len() - 1]);
            // An ESCALATION propagates, as it does from every other
            // quadrature lane: an in-band margin is the funnel's own
            // answer, and bisection cannot improve it — the span it
            // meters HALVES at every level, so a margin that sits in
            // the band arrives at the depth limit still in it. What
            // bisection DOES resolve is a definite verdict: a control
            // polygon can step backwards along the chord of the WHOLE
            // piece while every half is a graph over its own chord
            // (a curvature artefact, not a fold), and each half's
            // differences are new numbers rather than the same ones
            // scaled.
            match piece_monotone::<T>(&block, a, b, su, sv, band)? {
                Sign::Positive => {}
                // NO ROW PINS THIS LADDER, and the reason is worth
                // knowing: the round has ALREADY split the image into
                // `IMAGE_INIT_PIECES` blocks before the row sees one, and
                // on every fixture built for this lane that split alone
                // resolves a cubic's backward control step — removing
                // the ladder entirely reds nothing (row Q5b passes
                // either way). It is kept because the two resolutions
                // are different knobs: the round's lever is the image's
                // arc length and this one is the block's own, and a
                // feature sharp enough to survive eight uniform cuts is
                // exactly the case that has no other recourse.
                Sign::Zero | Sign::Negative if depth < MONOTONE_BISECTIONS => {
                    let (l, r) = bezier_bisect(&block);
                    stack.push((r, depth + 1));
                    stack.push((l, depth + 1));
                    continue;
                }
                Sign::Zero | Sign::Negative => {
                    return Err(PropsError::QuadratureUnsupported {
                        what: TRIM_NOT_MONOTONE,
                    });
                }
            }
            let area = lune_area(&block, a, b);
            out.push(TrimCell {
                edge,
                a,
                b,
                hull: (area > 0.0).then(|| block_box(&block)),
                lune_area: area,
                var: polygon_variation(&block),
                slack: vertex_slack(block[0]).max(vertex_slack(block[block.len() - 1])),
            });
        }
    }
    Ok(out)
}

/// The distinct breakpoints of a knot vector, ascending — the cell
/// boundaries of every rule below.
fn breakpoints(kv: &KnotVector) -> Vec<f64> {
    let mut b: Vec<f64> = kv.knots().to_vec();
    b.sort_by(f64::total_cmp);
    b.dedup();
    b
}

/// The `[lo, hi]` cell of a breakpoint list containing `t` (clamped to
/// the ends): the cell whose polynomial the rules evaluate.
fn cell_of(breaks: &[f64], t: f64) -> (f64, f64) {
    for w in breaks.windows(2) {
        if w[1] > w[0] && t < w[1] {
            return (w[0], w[1]);
        }
    }
    let n = breaks.len();
    (breaks[n - 2], breaks[n - 1])
}

/// The chart integrand `f = ⟨S, S_u×S_v⟩` at one collapse pair.
fn flux_at(
    s: &PatchGrid,
    su: Option<&PatchGrid>,
    sv: Option<&PatchGrid>,
    cu: Collapse<'_>,
    cv: Collapse<'_>,
) -> Interval {
    dot(
        &s.vec(cu, cv),
        &cross(&grid_vec(su, cu, cv), &grid_vec(sv, cu, cv)),
    )
}

/// The area integrand `g = |S_u×S_v|` at one collapse pair.
fn area_at(
    su: Option<&PatchGrid>,
    sv: Option<&PatchGrid>,
    cu: Collapse<'_>,
    cv: Collapse<'_>,
) -> Interval {
    let c = cross(&grid_vec(su, cu, cv), &grid_vec(sv, cu, cv));
    (c[0].sqr() + c[1].sqr() + c[2].sqr()).sqrt()
}

/// The area integrand's CELL reading over one chart rectangle: the
/// hull of `g = |S_u×S_v|` and upper bounds on its two partials, all
/// three read over the SAME extent the caller is about to pad with.
///
/// This is [`area_midpoint_taylor`](super::area::area_midpoint_taylor)'s `AreaCell` closure, written for
/// a lane whose cells are not a tensor grid: the rectangle lane hands
/// its rule an `AreaBox` and the rule calls back; the trimmed lane's
/// outer variable is a chord parameter and its inner extent depends on
/// the outer node, so the two cannot share the loop. They share this
/// reading, which is the part the discipline lives in —
/// `|∂_d |c|| ≤ |∂_d c|` is the Lipschitz constant, and a hull over a
/// larger extent is a LOOSER one, never a wrong one.
fn area_cell(
    su: Option<&PatchGrid>,
    sv: Option<&PatchGrid>,
    suu: Option<&PatchGrid>,
    suv: Option<&PatchGrid>,
    svv: Option<&PatchGrid>,
    u: Collapse<'_>,
    v: Collapse<'_>,
) -> (Interval, f64, f64) {
    let h_su = grid_vec(su, u, v);
    let h_sv = grid_vec(sv, u, v);
    let c = cross(&h_su, &h_sv);
    let (d_u, d_v) = normal_partials(
        &h_su,
        &h_sv,
        &grid_vec(suu, u, v),
        &grid_vec(suv, u, v),
        &grid_vec(svv, u, v),
    );
    (
        (c[0].sqr() + c[1].sqr() + c[2].sqr()).sqrt(),
        norm_hi(d_u),
        norm_hi(d_v),
    )
}

/// One chord `v = ℓ(u)` of the polygon, as both outer rules read it: a
/// chord with `u_a = u_b` carries no `du` and is no chord here.
struct Chord {
    a: (f64, f64),
    b: (f64, f64),
    slope: Interval,
}

/// How a chord's cuts meet a crossing of a `v`-knot, whose `u` is a
/// certified bracket.
#[derive(Clone, Copy)]
enum CrossingCut {
    /// One cut at the bracket's midpoint; the triangle the definite cut
    /// leaves between the knot and `ℓ` is the SLIVER pad's.
    Midpoint,
    /// Cuts at both ends of the bracket, which then lies between them as
    /// a GAP: outside it `ℓ` provably stays in one span.
    Bracket,
}

/// A chord's cut list, ascending, and what its crossings left to pay.
struct ChordCuts {
    cuts: Vec<f64>,
    /// `Σ w²·|ℓ′|` over the midpoint cuts.
    sliver: f64,
    /// The crossing brackets cut at both ends.
    gaps: Vec<(f64, f64)>,
}

impl Chord {
    fn of(c: &TrimCell) -> Option<Self> {
        let ((ua, va), (ub, vb)) = (c.a, c.b);
        (ua != ub).then(|| Self {
            a: c.a,
            b: c.b,
            slope: (pt(vb) - pt(va)) / (pt(ub) - pt(ua)),
        })
    }

    fn ell(&self, u: Interval) -> Interval {
        pt(self.a.1) + (u - pt(self.a.0)) * self.slope
    }

    /// `∫_{u_a}^{u_b}` from the integral over the ascending cuts.
    fn oriented(&self, ascending: Interval) -> Interval {
        if self.b.0 > self.a.0 {
            ascending
        } else {
            -ascending
        }
    }

    /// **The cuts**: the chord's own ends, the chart's `u`-knots, `n − 1`
    /// evenly spaced points, and its `v`-knot crossings under `policy`.
    /// `None` when a crossing's bracket does not certify under
    /// [`CrossingCut::Bracket`].
    ///
    /// A cut is what makes either rule's argument true: between cuts the
    /// chart is one polynomial in `u` AND `ℓ(u)` stays in one `v` span
    /// (up to a midpoint cut's sliver), so `G_f(u, ℓ(u))` is one
    /// polynomial of degree `3p_u + 3p_v − 1`.
    ///
    /// EVERY knot and crossing cut is mandatory, and nothing may drop
    /// one: a dropped cut leaves a piece straddling a knot by the width
    /// of whatever guard dropped it, with no term paying for it. That is
    /// the invariant [`knot_aligned_cuts`] states in capitals for its own
    /// mandatory set; this list applies none of that function's SLIVER
    /// guard. A hairline piece is the safe outcome, and a piece of zero
    /// width is removed by the dedup.
    fn cuts(
        &self,
        ubreaks: &[f64],
        vbreaks: &[f64],
        n: usize,
        policy: CrossingCut,
    ) -> Option<ChordCuts> {
        let ((ua, va), (ub, vb)) = (self.a, self.b);
        let (lo, hi) = (ua.min(ub), ua.max(ub));
        let mut cuts = vec![lo, hi];
        let mut sliver = 0.0f64;
        let mut gaps = Vec::new();
        cuts.extend(knots_inside(ubreaks, lo, hi));
        for j in 1..n {
            #[allow(clippy::cast_precision_loss)]
            let t = lo + (hi - lo) * (j as f64 / n as f64);
            if t > lo && t < hi {
                cuts.push(t);
            }
        }
        if vb != va {
            let du = pt(ub) - pt(ua);
            let dv = pt(vb) - pt(va);
            for k in vbreaks.iter().copied() {
                if k <= va.min(vb) || k >= va.max(vb) {
                    continue;
                }
                let us = pt(ua) + (pt(k) - pt(va)) * du / dv;
                match policy {
                    CrossingCut::Midpoint => {
                        let m = mid(us);
                        if m > lo && m < hi {
                            cuts.push(m);
                            // The mismatch is the triangle `{u ∈ bracket,
                            // v between the knot and ℓ(u)}`, of chart area
                            // at most `½·w²·|ℓ′|` on each side of the cut.
                            let w = us.width();
                            sliver += w.powi(2) * (dv / du).mag();
                        }
                    }
                    CrossingCut::Bracket => {
                        if !us.is_certified() {
                            return None;
                        }
                        let (g0, g1) = (us.lo().max(lo), us.hi().min(hi));
                        if g0 <= g1 {
                            cuts.extend([g0, g1]);
                            gaps.push((g0, g1));
                        }
                    }
                }
            }
        }
        cuts.sort_by(f64::total_cmp);
        cuts.dedup();
        Some(ChordCuts { cuts, sliver, gaps })
    }
}

/// The `v` cells one piece's column crosses: the complete spans `below`
/// the located cell `[v_k, v_{k+1}]`, each `(lo, hi, locator)`, and the
/// located cell's own start and locator.
struct Column {
    below: Vec<(f64, f64, f64)>,
    vk: f64,
    mid_v: f64,
}

impl Column {
    fn under(vbreaks: &[f64], (vk, vk1): (f64, f64)) -> Self {
        Self {
            below: vbreaks
                .windows(2)
                .filter(|b| b[1] > b[0] && b[1] <= vk)
                .map(|b| (b[0], b[1], b[0].midpoint(b[1])))
                .collect(),
            vk,
            mid_v: vk.midpoint(vk1),
        }
    }
}

/// `∫_{v₀}^{top} g(s) ds` for an integrand read through a `v` collapse,
/// by the inner closed Newton–Cotes rule `wv`: the column's complete
/// spans, then `[v_k, top]` on the located cell's polynomial. Exact per
/// span for a polynomial of degree `≤ m_v` there (`f` and its
/// `u`-partials have `v`-degree `3p_v − 1`), so at a point parameter the
/// enclosure's width is the ring's, and at an enclosed one it holds the
/// integral for every parameter and top in their brackets.
fn column_integral(
    wv: &[Interval],
    column: &Column,
    top: Interval,
    integrand: &dyn Fn(Collapse<'_>) -> Interval,
) -> Interval {
    let mv = wv.len() - 1;
    let mut inner = Interval::zero();
    let mut leg = |lo: Interval, hi: Interval, locator: f64| {
        let sc = hi - lo;
        let mut row = Interval::zero();
        for (k, wv_k) in wv.iter().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let v_k = lo + sc * (pt(k as f64) / pt(mv as f64));
            let cv = Collapse::AtSpan {
                mid: locator,
                t: &v_k,
            };
            row = row + *wv_k * integrand(cv);
        }
        inner = inner + sc * row;
    };
    for (c0, c1, cm) in &column.below {
        leg(pt(*c0), pt(*c1), *cm);
    }
    leg(pt(column.vk), top, column.mid_v);
    inner
}

/// The grids the composite rule's remainder reads beyond the
/// integrand's: the chart's second partials, and the two third partials
/// `f_uu` needs, derived here so that every `None` is [`PatchGrid`]'s
/// own "identically zero away from knots" and never "not built".
pub(super) struct Curvature<'g> {
    suu: Option<&'g PatchGrid>,
    suv: Option<&'g PatchGrid>,
    svv: Option<&'g PatchGrid>,
    suuu: Option<PatchGrid>,
    suuv: Option<PatchGrid>,
}

impl<'g> Curvature<'g> {
    pub(super) fn of(
        suu: Option<&'g PatchGrid>,
        suv: Option<&'g PatchGrid>,
        svv: Option<&'g PatchGrid>,
    ) -> Self {
        Self {
            suu,
            suv,
            svv,
            suuu: suu.and_then(PatchGrid::deriv_u),
            suuv: suu.and_then(PatchGrid::deriv_v),
        }
    }
}

/// **The trimmed lane's outer `u` rule**, chosen once at face entry by
/// whether the exact rule's weights exist for the chord integrand's
/// degree `3p_u + 3p_v − 1`.
pub(super) enum OuterRule<'g> {
    /// Closed Newton–Cotes weights of that order: exact on a piece.
    Exact(Vec<Interval>),
    /// The composite midpoint rule with its certified remainder.
    Composite(Box<Curvature<'g>>),
}

impl OuterRule<'_> {
    /// Weight of one cell of this rule, in [`FACE_WORK_WALL`](super::run::FACE_WORK_WALL)'s unit.
    fn weight(&self, p_u: usize, p_v: usize) -> usize {
        match self {
            Self::Exact(_) => w_trimmed(p_u, p_v),
            Self::Composite(_) => w_trimmed_composite(p_u, p_v),
        }
    }

    /// The pieces an iso chord is integrated in at a round of `pieces`
    /// blocks per `General` image: one exact chord, or as many as an
    /// image has blocks.
    fn iso_pieces(&self, pieces: usize) -> usize {
        match self {
            Self::Exact(_) => 1,
            Self::Composite(_) => pieces,
        }
    }

    /// The work of round `round` on a chart of bidegree `(p_u, p_v)`
    /// whose loop has `images` `General` images and `isos` iso chords:
    /// a round's blocks per image, and an iso chord's pieces, at the
    /// rule's weight.
    pub(super) fn round_cost(
        &self,
        (p_u, p_v): (usize, usize),
        images: usize,
        isos: usize,
        round: usize,
    ) -> usize {
        let pieces = IMAGE_INIT_PIECES << round;
        self.weight(p_u, p_v) * (images * pieces + isos * self.iso_pieces(pieces))
    }

    fn crossing_cut(&self) -> CrossingCut {
        match self {
            Self::Exact(_) => CrossingCut::Midpoint,
            Self::Composite(_) => CrossingCut::Bracket,
        }
    }
}

/// Enclosures of the chord integrand's first partials at one collapse
/// pair: `f_u = ⟨S, ∂_u(S_u×S_v)⟩` and `f_v = ⟨S, ∂_v(S_u×S_v)⟩` (the
/// `⟨S_u, S_u×S_v⟩` and `⟨S_v, S_u×S_v⟩` terms vanish).
fn flux_slopes(
    s: &PatchGrid,
    su: Option<&PatchGrid>,
    sv: Option<&PatchGrid>,
    k: &Curvature<'_>,
    u: Collapse<'_>,
    v: Collapse<'_>,
) -> (Interval, Interval) {
    let hs = s.vec(u, v);
    let (d_u, d_v) = normal_partials(
        &grid_vec(su, u, v),
        &grid_vec(sv, u, v),
        &grid_vec(k.suu, u, v),
        &grid_vec(k.suv, u, v),
        &grid_vec(k.svv, u, v),
    );
    (dot(&hs, &d_u), dot(&hs, &d_v))
}

/// Enclosure of `f_uu = ⟨S_u, S_uu×S_v⟩ + ⟨S, S_uuu×S_v + 2·S_uu×S_uv +
/// S_u×S_uuv⟩` at one collapse pair (`⟨S_u, S_u×S_uv⟩` vanishes).
fn flux_uu(
    s: &PatchGrid,
    su: Option<&PatchGrid>,
    sv: Option<&PatchGrid>,
    k: &Curvature<'_>,
    u: Collapse<'_>,
    v: Collapse<'_>,
) -> Interval {
    let (hs, hu, hv) = (s.vec(u, v), grid_vec(su, u, v), grid_vec(sv, u, v));
    let huu = grid_vec(k.suu, u, v);
    let twice = cross(&huu, &grid_vec(k.suv, u, v)).map(|c| c + c);
    dot(&hu, &cross(&huu, &hv))
        + dot(
            &hs,
            &rv_add(
                rv_add(cross(&grid_vec(k.suuu.as_ref(), u, v), &hv), twice),
                cross(&hu, &grid_vec(k.suuv.as_ref(), u, v)),
            ),
        )
}

/// What [`chord_polygon_flux`] answers.
struct ChordFlux {
    /// The polygon's flux, the composite rule's column pads included.
    flux: Interval,
    /// `Σ w²·|ℓ′|` over the exact rule's midpoint crossing cuts.
    sliver: f64,
    /// The summed width of the composite rule's remainders, the part of
    /// `flux`'s width a finer round removes.
    removable: f64,
}

/// **The chord polygon's flux** — `∫∫ f·w_chord` by Green's theorem on
/// `G_f(u, v) = ∫_{v₀}^{v} f(u, s) ds`, one chord at a time
/// (`∫∫ f·w = −∮ G_f du`), each cut ([`Chord::cuts`]) so that a piece
/// sees one chart cell, and the inner `v` integral by the exact closed
/// Newton–Cotes rule ([`column_integral`]). `pieces(cell)` is how many
/// even pieces a cell's chord is cut into before its mandatory cuts.
///
/// **The outer rule is `rule`'s.**
///
/// - [`OuterRule::Exact`]: the closed Newton–Cotes rule of order
///   `3p_u + 3p_v − 1` integrates a piece's polynomial EXACTLY, so the
///   enclosure width is the nodes' and weights' ring rounding and
///   nothing else. Crossings are cut at their bracket's midpoint, the
///   sliver returned for the caller's pad.
/// - [`OuterRule::Composite`]: the midpoint rule on a piece `[p, q]`,
///
///   ```text
///   ∫_p^q φ = h·φ(m) + (h³/24)·φ''(η),   φ'' = G_uu + 2ℓ′·f_u + ℓ′²·f_v,
///   ```
///
///   for some `η` in the piece, `φ(u) = G_f(u, ℓ(u))` — the midpoint
///   rule's Peano kernel is non-negative. The remainder is that formula
///   over enclosures of its terms on the box `[p, q] × ℓ([p, q])`, each
///   read on the located cells' polynomials at the piece's brackets
///   (`G_uu = ∫_{v₀}^{ℓ} f_uu` by the same inner rule), so `φ''`'s
///   enclosure narrows with the piece and the remainder's width with
///   it, at `O(h⁴)` a piece. Crossings are cut at both bracket ends, so
///   outside a bracket `ℓ` provably stays in one span. A piece inside a
///   bracket, or one too narrow to locate its span, takes the
///   first-order bound `h·|ℓ − v₀|·sup|f|` on its column instead.
///
/// Nodes read through [`Collapse::AtSpan`], the piece's own midpoint
/// pinning the cell, exactly as [`patch_flux_exact`](super::patch::patch_flux_exact) does.
#[allow(clippy::too_many_arguments)]
fn chord_polygon_flux(
    s: &PatchGrid,
    su: Option<&PatchGrid>,
    sv: Option<&PatchGrid>,
    rule: &OuterRule<'_>,
    kv_u: &KnotVector,
    kv_v: &KnotVector,
    cells: &[TrimCell],
    pieces: &dyn Fn(&TrimCell) -> usize,
    wv: &[Interval],
) -> ChordFlux {
    let refused = ChordFlux {
        flux: Interval::refused(),
        sliver: 0.0,
        removable: 0.0,
    };
    let ubreaks = breakpoints(kv_u);
    let vbreaks = breakpoints(kv_v);
    let mut total = Interval::zero();
    let mut sliver = 0.0f64;
    let mut removable = 0.0f64;
    // The composite rule's first-order column bounds. **No row can go
    // red on this term**: a piece falls to it only inside a crossing's
    // `u` bracket or below the resolution that locates its span, and the
    // bracket is about one ulp wide, because the chord's `f64` ends make
    // `Δv` exact and only the division rounds — so the pad is `~h·ulp`
    // and dropping it moves no enclosure past its own rounding. It is
    // the composite twin of the exact rule's SLIVER, unguarded for the
    // same reason.
    let mut column_pad = 0.0f64;
    for c in cells {
        let Some(chord) = Chord::of(c) else {
            continue;
        };
        let Some(cut) = chord.cuts(&ubreaks, &vbreaks, pieces(c), rule.crossing_cut()) else {
            return refused;
        };
        sliver += cut.sliver;
        let mut acc = Interval::zero();
        for w in cut.cuts.windows(2) {
            let (p, q) = (w[0], w[1]);
            match rule {
                OuterRule::Exact(wu) => {
                    acc = acc + exact_piece(s, su, sv, wu, wv, &vbreaks, &chord, p, q);
                }
                OuterRule::Composite(k) => {
                    match composite_piece(s, su, sv, k, wv, &vbreaks, &chord, &cut.gaps, p, q) {
                        Some(Piece::Rule { value, remainder }) => {
                            removable += remainder.width();
                            acc = acc + value + remainder;
                        }
                        Some(Piece::Column(pad)) => column_pad += pad,
                        None => return refused,
                    }
                }
            }
        }
        total = total + chord.oriented(acc);
    }
    ChordFlux {
        flux: widen(-total, column_pad),
        sliver,
        removable,
    }
}

/// One piece by the exact rule: `∫_p^q G_f(u, ℓ(u)) du`.
#[allow(clippy::too_many_arguments)]
fn exact_piece(
    s: &PatchGrid,
    su: Option<&PatchGrid>,
    sv: Option<&PatchGrid>,
    wu: &[Interval],
    wv: &[Interval],
    vbreaks: &[f64],
    chord: &Chord,
    p: f64,
    q: f64,
) -> Interval {
    let mu = wu.len() - 1;
    let mid_u = p.midpoint(q);
    let column = Column::under(vbreaks, cell_of(vbreaks, mid(chord.ell(pt(mid_u)))));
    let scale_u = pt(q) - pt(p);
    let mut acc = Interval::zero();
    for (j, wu_j) in wu.iter().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        let u_j = pt(p) + scale_u * (pt(j as f64) / pt(mu as f64));
        let cu = Collapse::AtSpan {
            mid: mid_u,
            t: &u_j,
        };
        acc = acc
            + *wu_j
                * column_integral(wv, &column, chord.ell(u_j), &|cv| {
                    flux_at(s, su, sv, cu, cv)
                });
    }
    scale_u * acc
}

/// A composite piece's reading.
enum Piece {
    /// `h·φ(m)` and the remainder `(h³/24)·φ''`.
    Rule {
        value: Interval,
        remainder: Interval,
    },
    /// The first-order column bound, a pad.
    Column(f64),
}

/// One piece by the composite rule ([`chord_polygon_flux`]'s docs);
/// `None` where a window does not certify.
#[allow(clippy::too_many_arguments)]
fn composite_piece(
    s: &PatchGrid,
    su: Option<&PatchGrid>,
    sv: Option<&PatchGrid>,
    k: &Curvature<'_>,
    wv: &[Interval],
    vbreaks: &[f64],
    chord: &Chord,
    gaps: &[(f64, f64)],
    p: f64,
    q: f64,
) -> Option<Piece> {
    let v0 = pt(vbreaks[0]);
    let (pi, qi) = (pt(p), pt(q));
    let h = qi - pi;
    let band = Interval::hull(chord.ell(pi), chord.ell(qi));
    let m = p.midpoint(q);
    let in_gap = gaps.iter().any(|&(g0, g1)| p >= g0 && q <= g1);
    let located = (p < m && m < q && !in_gap)
        .then(|| {
            let lm = chord.ell(pt(m));
            let (vk, vk1) = cell_of(vbreaks, mid(lm));
            (lm.lo() >= vk && lm.hi() <= vk1).then_some((vk, vk1))
        })
        .flatten();
    let Some(cell) = located else {
        let over_u = Collapse::Over(ParamRange::certified(Interval::hull(pi, qi))?);
        let col = Collapse::Over(ParamRange::certified(Interval::hull(v0, band))?);
        let f = flux_at(s, su, sv, over_u, col);
        return Some(Piece::Column(h.hi() * ((band - v0) * f).mag()));
    };
    let column = Column::under(vbreaks, cell);
    let mi = (pi + qi) * pt(0.5);
    let cu = Collapse::AtSpan { mid: m, t: &mi };
    let value = h * column_integral(wv, &column, chord.ell(mi), &|cv| flux_at(s, su, sv, cu, cv));
    // `φ''` over the piece, every term read on the located cells'
    // polynomials at the piece's own brackets, so its enclosure narrows
    // with the piece.
    let piece = Interval::hull(pi, qi);
    let cu_piece = Collapse::AtSpan { mid: m, t: &piece };
    let g_uu = column_integral(wv, &column, band, &|cv| flux_uu(s, su, sv, k, cu_piece, cv));
    let cv_band = Collapse::AtSpan {
        mid: column.mid_v,
        t: &band,
    };
    let (fu, fv) = flux_slopes(s, su, sv, k, cu_piece, cv_band);
    let slope = chord.slope;
    let phi2 = g_uu + pt(2.0) * slope * fu + slope.sqr() * fv;
    Some(Piece::Rule {
        value,
        remainder: h.sqr() * h / pt(24.0) * phi2,
    })
}

/// **The cut list under one chord** in one direction: the range's ends,
/// the chart's knots inside it, and the range's own
/// [`CHORD_AREA_CELLS`]-piece grid, a grid point within
/// [`algebra::grid_clearance`] of a knot dropped. Uniform over the
/// RANGE, not per knot span: a chord's column runs from the chart's
/// lowest `v` and may cross every span beneath it, and a per-span count
/// there would multiply the pass by the spans crossed
/// (`work/quad/the-trimmed-lanes-chord-area-is-laid-per-range-not-per-span.md`).
fn chord_area_cuts(lo: f64, hi: f64, knots: &[f64]) -> Vec<f64> {
    let mut cuts = vec![lo, hi];
    cuts.extend(knots_inside(knots, lo, hi));
    cuts.sort_by(f64::total_cmp);
    cuts.dedup();
    let mandatory = cuts.clone();
    cuts.extend(algebra::range_grid_points(
        lo,
        hi,
        CHORD_AREA_CELLS,
        &mandatory,
    ));
    cuts.sort_by(f64::total_cmp);
    cuts.dedup();
    cuts
}

/// **The chord polygon's area**, the same Green decomposition with
/// `G_g(u, v) = ∫_{v₀}^{v} g` — `g = |S_u×S_v|` is not polynomial, so
/// it cannot be the exact rule the flux takes.
///
/// **What runs instead is [`area_midpoint_taylor`](super::area::area_midpoint_taylor)'s OWN cell rule**,
/// per sub-chord: `ℓ` is affine, so `∫_p^q G_g(u, ℓ(u)) du` is the
/// integral of `g` over the trapezoid under the sub-chord, and that
/// trapezoid has exactly the area of the rectangle
/// `[p,q] × [v₀, ℓ(u_m)]` — the midpoint is exact for an affine
/// boundary. So the rule sums the rectangle's cells with the rectangle
/// lane's own `½h_u·G_u + ½h_v·G_v` pad and pays a separate, second
/// order term for the trapezoid-versus-rectangle mismatch.
///
/// **It is a COPY of that rule, not a call of it, and this is where
/// the discipline was once lost.** The rule there runs over a tensor
/// grid with a callback; here the v extent depends on the sub-chord,
/// so the loops cannot be shared — what they DO share is
/// [`area_cell`], every hull read over exactly the cell it pads. A
/// first version wrote a NESTED rule instead (an outer midpoint over a
/// column integral) and read one hull over the whole trim box for all
/// of it. Both halves of that were wrong in the same direction: the
/// whole-box hull does not shrink with the cell, and a column
/// integral's Lipschitz constant carries the column's HEIGHT, so the
/// outer pad was `O(h_u · height)` where the cell rule's is `O(h)`.
/// On an ordinary bidegree-(2,2) chart that drove `area.lo()` below
/// zero and the extent gate answered `DegenerateFace` for a face of
/// area ≈ 0.6. Track R's consolidation (#723) owns unifying the two;
/// this unit owes the reading.
///
/// First order and fixed resolution, exactly as the rectangle's area
/// is: the area is a DENOMINATOR (the convergence meter's lever) and a
/// GAUGE (the extent gate), never the meter itself. **Its width is
/// first order in the CHORD COUNT, not in the lune pad** — measured:
/// on a fixture with no lune at all (a degree-1 image, where every
/// block is its own chord) the area width halves per round, which is
/// the per-chord cell rule's own `O(h)`.
#[allow(clippy::too_many_arguments)]
fn chord_polygon_area(
    su: Option<&PatchGrid>,
    sv: Option<&PatchGrid>,
    suu: Option<&PatchGrid>,
    suv: Option<&PatchGrid>,
    svv: Option<&PatchGrid>,
    kv_u: &KnotVector,
    kv_v: &KnotVector,
    cells: &[TrimCell],
) -> Interval {
    let ku = breakpoints(kv_u);
    let kvb = breakpoints(kv_v);
    let v0 = kvb[0];
    let mut total = Interval::zero();
    for c in cells {
        let ((ua, va), (ub, vb)) = (c.a, c.b);
        if ua == ub {
            continue;
        }
        // `spanning`, not `min`/`max`: those drop a NaN end and would
        // mint the chord's window from the other end alone.
        let Some(span_u) = ParamRange::spanning(ua, ub) else {
            return Interval::refused();
        };
        let slope = (vb - va) / (ub - ua);
        let ell = |u: f64| va + (u - ua) * slope;
        let mut chord = Interval::zero();
        for w in chord_area_cuts(span_u.start(), span_u.end(), &ku).windows(2) {
            let (p, q) = (w[0], w[1]);
            // Adjacent cuts of a sorted list are ordered; a pair that is
            // not (an infinite end cuts to NaN) names no region, and the
            // area refuses.
            let Some(sub) = ParamRange::new(p, q) else {
                return Interval::refused();
            };
            let hu = q - p;
            let um = p.midpoint(q);
            let top = ell(um);
            let Some(column) = ParamRange::spanning(v0, top) else {
                return Interval::refused();
            };
            // The COLUMN under this sub-chord's midpoint, cell by cell
            // — and each cell's pad is the rectangle lane's own,
            // `½h_u·G_u + ½h_v·G_v`, with both hulls read over the
            // 2-D cell. There is no outer Lipschitz constant here to
            // bound or to mis-read: the `(q − p)` factor below turns
            // this column into the rectangle `[p,q] × [v₀, ℓ(u_m)]`,
            // and the cell rule pads that rectangle directly.
            let mut col = Interval::zero();
            for wc in chord_area_cuts(column.start(), column.end(), &kvb).windows(2) {
                let (c0, c1) = (wc[0], wc[1]);
                let Some(cell_v) = ParamRange::new(c0, c1) else {
                    return Interval::refused();
                };
                let hv = c1 - c0;
                let vm = c0.midpoint(c1);
                let (cell_g, cell_gu, cell_gv) = area_cell(
                    su,
                    sv,
                    suu,
                    suv,
                    svv,
                    Collapse::Over(sub),
                    Collapse::Over(cell_v),
                );
                let mean = widen(
                    area_at(su, sv, Collapse::At(um), Collapse::At(vm)),
                    0.5 * hu * cell_gu + 0.5 * hv * cell_gv,
                )
                .clamped_to(lo_or_refuse(cell_g), certified_hi(cell_g));
                col = col + (pt(c1) - pt(c0)) * mean;
            }
            if top < v0 {
                col = -col;
            }
            // **The trapezoid the chord really bounds, against the
            // rectangle just integrated.** `ℓ` is affine, so the two
            // have EXACTLY the same area and the difference sees only
            // `g`'s oscillation over the band `ℓ` sweeps:
            // `|∫_p^q ∫_{ℓ(u_m)}^{ℓ(u)} g| ≤ |ℓ′|·h_u²·width(g)/8`,
            // the constant term of `g` integrating to zero against
            // `∫(u − u_m) du`.
            let Some(band) = ParamRange::spanning(ell(p), ell(q)) else {
                return Interval::refused();
            };
            let (band_g, _, _) = area_cell(
                su,
                sv,
                suu,
                suv,
                svv,
                Collapse::Over(sub),
                Collapse::Over(band),
            );
            let mismatch = 0.125 * slope.abs() * hu.powi(2) * band_g.width();
            chord = chord + widen((pt(q) - pt(p)) * col, mismatch);
        }
        total = total + if ub > ua { chord } else { -chord };
    }
    -total
}

/// The chart box every global hull below is taken over: the hull of
/// every chord endpoint bracket and every stored image net. An OUTER
/// enclosure of the trim region, so a hull over it contains every
/// cell's own.
fn trim_box(chords: &[TrimChord]) -> (Interval, Interval) {
    let mut bu = Interval::refused();
    let mut bv = Interval::refused();
    let mut seeded = false;
    let mut take = |p: RPt2| {
        if seeded {
            bu = Interval::hull(bu, p.0);
            bv = Interval::hull(bv, p.1);
        } else {
            bu = p.0;
            bv = p.1;
            seeded = true;
        }
    };
    for c in chords {
        take(c.a);
        take(c.b);
        if let Some(img) = &c.piece {
            for p in &img.control {
                take(*p);
            }
        }
    }
    (bu, bv)
}

/// The traversal's signed chart area (the shoelace) — whose SIGN is
/// the S10 winding, exactly as the rectangle lane reads it: the area
/// is `|w|`-weighted and the flux is `w`-weighted, so Green's own
/// signed answer IS the flux and the area takes this sign.
fn polygon_winding(cells: &[TrimCell]) -> f64 {
    let mut acc = 0.0f64;
    for c in cells {
        acc += c.a.0 * c.b.1 - c.b.0 * c.a.1;
    }
    acc
}

/// **The trimmed-region quadrature** (TRIM-2): certified flux and area
/// of a described NURBS face whose trim region is what its loop's
/// chart image bounds, rather than an axis-aligned rectangle.
///
/// Three certified steps, each a widening: the image's CHORDS (its own
/// knots, then uniform at the round's lever), the LUNES between arc
/// and chord (`A(B_k)·sup|f|` on the piece box, under the
/// `props_trim_piece_monotone` row), and the chord polygon by Green's
/// `G_f`: its inner `v` rule exact, its outer `u` rule exact while
/// `p_u + p_v ≤ 4` and the composite midpoint rule with its certified
/// remainder past that (both [`chord_polygon_flux`]'s, by
/// [`OuterRule`]), the round's lever cutting an iso
/// chord into as many pieces as a `General` image has blocks. Both arms
/// are one engine: the cells, lunes, pads, area and stop are shared,
/// and the composite arm's remainders join the lunes as the width a
/// later round removes. The envelope pad is the rectangle lane's,
/// unchanged in kind.
///
/// Direction of every rounding: boxes are outer enclosures, sups are
/// hulls, the monotone rate is bounded BELOW, a definite verdict short
/// of `Positive` refines and then refuses rather than padding (and an
/// in-band one escalates), the `v`-knot crossing's definite cut
/// carries its own sliver pad, the refinement's rounded `λ` is paid
/// for in [`bezier_blocks`], and the walk's in-bracket closure gap is
/// paid at `|Δu|·sup|G_f|`. No path runs from an imprecise input to a
/// narrower enclosure.
///
/// **One clause of that paragraph no row can go red on**: the
/// Newton–Cotes weights are built as exact `i128` fractions and
/// bracketed by ONE outward ring division
/// ([`newton_cotes_weights`]), and a mutant that pushes the rounded
/// quotient instead moves every enclosure here by less than its own
/// ring width — there is no fixture at which a one-ulp weight bias is
/// visible, because the rule it belongs to is exact and its answer's
/// width IS that rounding. Recorded rather than claimed guarded.
///
/// # Errors
///
/// [`PropsError`] — `accept`'s own refusal, an escalated funnel
/// decision, a degenerate face, and the typed refusals of the rational
/// chart, the rational image, the non-monotone piece and the inner
/// Newton–Cotes window.
#[allow(clippy::too_many_lines)] // one engine, kept whole like the cylinder lane
pub fn trimmed_patch_face_rounds<T: Decide>(
    net: &PatchNet<'_>,
    chords: &[TrimChord],
    band: Band,
    window: RoundWindow,
    accept: &mut Accept<'_>,
) -> Result<LaneRun, PropsError> {
    if chords.len() < 3 {
        return Err(PropsError::QuadratureUnsupported {
            what: "a trim loop of fewer than three chart images — a region with no \
                   interior for Green's theorem to bound",
        });
    }
    if net.is_rational() {
        return Err(PropsError::QuadratureUnsupported {
            what: "a RATIONAL chart reached the trimmed lane — the chord integrand is the \
                   polynomial `⟨S, S_u×S_v⟩`, and a quotient's is not polynomial in the \
                   chord parameter, so the exact Newton–Cotes argument does not hold; the \
                   rational chart carries IsoArc rims only on this head",
        });
    }
    let (kv_u, kv_v) = (net.knots_u(), net.knots_v());
    let (p_u, p_v) = (kv_u.degree(), kv_v.degree());
    let Some(wv) = (3 * p_v).checked_sub(1).and_then(newton_cotes_weights) else {
        return Err(PropsError::QuadratureUnsupported {
            what: TRIM_NC_WINDOW,
        });
    };
    let s = PatchGrid::base(&net.positions());
    let su = s.deriv_u();
    let sv = s.deriv_v();
    let suu = su.as_ref().and_then(PatchGrid::deriv_u);
    let suv = su.as_ref().and_then(PatchGrid::deriv_v);
    let svv = sv.as_ref().and_then(PatchGrid::deriv_v);
    let rule = match (3 * p_u + 3 * p_v)
        .checked_sub(1)
        .and_then(newton_cotes_weights)
    {
        Some(wu) => OuterRule::Exact(wu),
        None => OuterRule::Composite(Box::new(Curvature::of(
            suu.as_ref(),
            suv.as_ref(),
            svv.as_ref(),
        ))),
    };
    let (tb_u, tb_v) = trim_box(chords);
    let (s_hull, h_su, h_sv) = match certified_box(tb_u, tb_v) {
        Some(over) => (
            s.vec(over.0, over.1),
            grid_vec(su.as_ref(), over.0, over.1),
            grid_vec(sv.as_ref(), over.0, over.1),
        ),
        None => (refused_vec(), refused_vec(), refused_vec()),
    };
    let p_bound = s_hull[0].mag() + s_hull[1].mag() + s_hull[2].mag();
    let normal = cross(&h_su, &h_sv);
    let sup_f = dot(&s_hull, &normal).mag();
    // Whole-box `sup|f|` and `sup g` are the PADS' levers only — the
    // vertex connectors and the sliver; the gap's is the column's,
    // below. The area RULE reads its own per-cell hulls ([`area_cell`]):
    // a whole-box hull inside a fixed-resolution midpoint rule is a pad
    // that does not shrink with the cell, which is how an ordinary
    // curved chart came out `DegenerateFace`.
    let sup_g = (normal[0].sqr() + normal[1].sqr() + normal[2].sqr()).sqrt();
    // The chart's own metric RATE, bounded above over the trim box:
    // the door crosses each cell's chart variation to metres with it
    // rather than taking a caller's length (`TrimChord`'s docs, and
    // #1368 for what a trusted perimeter costs). An UPPER bound is the
    // refusing direction in both consumers — it widens the envelope
    // pad, and it shrinks `area/perimeter` at the extent gate.
    let (rate_u, rate_v) = (norm_hi(h_su), norm_hi(h_sv));
    // `|G_f| ≤ sup|f|·(v extent)` and `|G_g| ≤ sup g·(v extent)`: the
    // lever the closure gap is paid at, `G` being what Green integrates
    // along `du`. The sups are over the COLUMN `G` integrates, the trim
    // box's `u` extent from the domain's `v₀` up, not over the trim box.
    let v_reach = (kv_v.domain().1 - kv_v.domain().0).abs();
    let (col_s, col_su, col_sv) =
        match certified_box(tb_u, Interval::hull(pt(kv_v.domain().0), tb_v)) {
            Some(over) => (
                s.vec(over.0, over.1),
                grid_vec(su.as_ref(), over.0, over.1),
                grid_vec(sv.as_ref(), over.0, over.1),
            ),
            None => (refused_vec(), refused_vec(), refused_vec()),
        };
    let col_normal = cross(&col_su, &col_sv);
    let col_f = dot(&col_s, &col_normal).mag();
    let col_g = (col_normal[0].sqr() + col_normal[1].sqr() + col_normal[2].sqr())
        .sqrt()
        .mag();
    // A round's cells are its iso chords and, per `General` image, the
    // round's blocks: the schedule's count, before a monotone bisection
    // adds any. `cost` does not count the pieces a chord's knot and
    // crossing cuts add, in either rule (the exact rule's cells have
    // always been counted so).
    let images = chords.iter().filter(|c| c.piece.is_some()).count();
    let cost = |round: usize| rule.round_cost((p_u, p_v), images, chords.len() - images, round);
    let mut run = Run::enter(window, &cost);
    let mut round = run.first();
    let mut pieces = IMAGE_INIT_PIECES << round;
    // **The AREA is taken once, at the ENTERING round's chords, and
    // that is a cost decision rather than a fact about the width.**
    // Measured: its width is first order in the CHORD COUNT — on a
    // fixture with no lune at all it halves per round — so a later
    // round's area IS tighter, and what a caller gets back is round
    // `window.first`'s whatever round the flux converges at. Per-cell
    // hulls ([`area_cell`]) moved the constant, not the order. Why it
    // is not re-taken at the converging round: the rule is a nested
    // 16×16 pass PER CHORD and the chord count doubles every round, so
    // "one more call" at the last round is two orders of magnitude of
    // work, not one call's worth.
    let mut area_at_first: Option<Interval> = None;
    loop {
        let cells = trim_cells::<T>(chords, pieces, su.as_ref(), sv.as_ref(), band)?;
        if cells.len() < 3 {
            return Err(PropsError::QuadratureUnsupported {
                what: "a trim loop of fewer than three chart images — a region with no \
                       interior for Green's theorem to bound",
            });
        }
        // **The walk must close.** Each chord's end has to meet the
        // next chord's start inside their brackets; the disagreement
        // that survives INSIDE them is real (two pcurves read one
        // shared vertex) and is paid below at `|Δu|·sup|G_f|`, which is
        // exactly what the closing segment would contribute.
        let mut gap_u = 0.0f64;
        for i in 0..cells.len() {
            let end = cells[i].b;
            let next = &cells[(i + 1) % cells.len()];
            let (gu, gv) = (end.0 - next.a.0, end.1 - next.a.1);
            let d = (gu.powi(2) + gv.powi(2)).sqrt();
            if d.is_nan() || d > cells[i].slack + next.slack {
                return Err(PropsError::QuadratureUnsupported {
                    what: TRIM_OPEN_WALK,
                });
            }
            gap_u += gu.abs();
        }
        // The envelope pad's lever and the extent gate's perimeter,
        // both derived here from the cells the door built.
        let mut edge_len = vec![0.0f64; chords.len()];
        for c in &cells {
            edge_len[c.edge] += c.var.0 * rate_u + c.var.1 * rate_v;
        }
        let perimeter: f64 = edge_len.iter().sum();
        let boundary_defect: f64 = chords
            .iter()
            .zip(&edge_len)
            .map(|(c, l)| l * c.env.mag())
            .sum();
        let mut lune_f = 0.0f64;
        let mut lune_g = 0.0f64;
        let mut vertex_pad = 0.0f64;
        for c in &cells {
            let chord_len = ((c.b.0 - c.a.0).powi(2) + (c.b.1 - c.a.1).powi(2)).sqrt();
            // The chord polygon's vertices are the endpoint brackets'
            // MIDPOINTS, so the closed walk is exact structure; the
            // connectors from a midpoint to the true endpoint sweep at
            // most `chord·slack` of chart area, which is what this pays.
            vertex_pad += chord_len * c.slack;
            let Some((bu, bv)) = c.hull else {
                continue;
            };
            let a = c.lune_area;
            let (hs, hc) = match certified_box(bu, bv) {
                Some(ov) => (
                    s.vec(ov.0, ov.1),
                    cross(
                        &grid_vec(su.as_ref(), ov.0, ov.1),
                        &grid_vec(sv.as_ref(), ov.0, ov.1),
                    ),
                ),
                None => (refused_vec(), refused_vec()),
            };
            lune_f += a * dot(&hs, &hc).mag();
            lune_g += a * (hc[0].sqr() + hc[1].sqr() + hc[2].sqr()).sqrt().mag();
        }
        let winding = polygon_winding(&cells);
        let per_cell = |c: &TrimCell| {
            if chords[c.edge].piece.is_none() {
                rule.iso_pieces(pieces)
            } else {
                1
            }
        };
        let ChordFlux {
            flux: flux_raw,
            sliver,
            removable: remainder,
        } = chord_polygon_flux(
            &s,
            su.as_ref(),
            sv.as_ref(),
            &rule,
            kv_u,
            kv_v,
            &cells,
            &per_cell,
            &wv,
        );
        // **Two of the terms below and one inside `chord_polygon_flux`
        // are not independently guarded, and saying so is cheaper than
        // implying they are.** The GAP term is guarded where its lever
        // is the column's (Q17: a chart whose integrand vanishes on the
        // trim box); where it is not, inside the closure check's
        // admitted region (`|Δ| ≤ the two brackets' slacks`) the VERTEX
        // pad bounds a displacement of the same size, so one covers the
        // other. They are kept apart because they bound different
        // things in general — the vertex pad pays for a connector from
        // a bracket's midpoint to a point inside THAT bracket, the gap
        // for a disagreement between two different brackets' midpoints
        // — and neither subsumes the other for arbitrary slacks.
        // Halving the SLIVER constant reds nothing: the `v`-crossing
        // bracket is thin at `f64` (measured `1.49e-30` on Q7's chart),
        // so the term is structurally right and numerically invisible
        // until the interval lane instantiates the door. The composite
        // rule's crossing-bracket COLUMN bound is unguarded for the same
        // reason (its site says so).
        //
        // §1(iii)'s constant, written whole: the mismatch sliver is
        // `½·w²·|ℓ′|` on EACH side of the definite cut and the
        // integrand over it is bounded by `sup|f|`, so the pad is
        // `w²·|ℓ′|·sup|f|` — `chord_cuts` accumulates `w²·|ℓ′|` and
        // this multiplies it. An earlier spelling halved it; still ≥
        // the triangle it bounds, but not the ratified letter.
        let pad_f = lune_f
            + (vertex_pad + sliver) * sup_f
            + gap_u * col_f * v_reach
            + boundary_defect * p_bound;
        let flux = widen(flux_raw, pad_f);
        let area = *area_at_first.get_or_insert_with(|| {
            let raw = chord_polygon_area(
                su.as_ref(),
                sv.as_ref(),
                suu.as_ref(),
                suv.as_ref(),
                svv.as_ref(),
                kv_u,
                kv_v,
                &cells,
            );
            let signed = if winding < 0.0 { -raw } else { raw };
            widen(
                signed,
                lune_g
                    + (vertex_pad + sliver) * sup_g.mag()
                    + gap_u * col_g * v_reach
                    + boundary_defect,
            )
        });
        let bounds = FaceCutBounds { flux, area };
        // The lunes, widening the flux both ways, and the composite
        // arm's remainders are the part of its width a later round
        // removes; the vertex, sliver, gap and envelope pads are not.
        // The perimeter the extent gate levers by is the DOOR's,
        // derived from the cells above — not a caller's field. A lane
        // that takes the caller's is
        // `work/quadtail/quad-face-extent-trusts-caller-perimeter.md`
        // (#1368, open), and this one deliberately does not join it.
        let (stop, accepted) = run.ask(round, bounds, 2.0 * lune_f + remainder, accept, || {
            face_extent::<T>(area, perimeter, band)
        })?;
        if let Some(out) = run.exit(round, stop, accepted, bounds) {
            return Ok(out);
        }
        round += 1;
        pieces *= 2;
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::super::patch::nurbs_patch_face_rounds;
    use super::super::run::LaneEnd;
    use super::*;
    use crate::props::quad::test_target::{self, bounds_of, encloses, met, patch, to_target};
    use geom_core::Bounds;
    use geom_core::Tol;

    // ---------------------------------------------------------------
    // TRIM-2 PR-1 — the trimmed-region lane (Q1–Q7)
    //
    // Every chart here is hand-built and key-free, and every truth is
    // a closed form: the flat `z = c` bilinear patch has `S_u×S_v ≡ ẑ`,
    // so `f = ⟨S, S_u×S_v⟩ ≡ c` and `g ≡ 1` — which turns "the flux of
    // a curved trim region" into "c times its chart area", a number a
    // reader can check by hand.
    // ---------------------------------------------------------------

    /// The flat chart `S(u, v) = (u, v, c)` on `[0,1]²`, bilinear.
    fn flat_chart(c: f64) -> (KnotVector, KnotVector, Vec<RVec3>, Vec<f64>) {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let mut control = Vec::new();
        for i in 0..2 {
            for j in 0..2 {
                control.push([pt(f64::from(i)), pt(f64::from(j)), pt(c)]);
            }
        }
        (kv.clone(), kv, control, vec![1.0; 4])
    }

    /// One exact chord of a trim polygon (an iso image's class):
    /// endpoints by structure, no arc, no lune.
    fn iso(a: (f64, f64), b: (f64, f64)) -> TrimChord {
        TrimChord {
            a: (pt(a.0), pt(a.1)),
            b: (pt(b.0), pt(b.1)),
            piece: None,
            forward: true,
            env: pt(0.0),
        }
    }

    /// A `General` chord: the single-span Bézier image through
    /// `points`, walked forward.
    fn general(points: &[(f64, f64)], env: f64) -> TrimChord {
        let p = points.len() - 1;
        let mut knots = vec![0.0; p + 1];
        knots.extend(std::iter::repeat_n(1.0, p + 1));
        let (a, b) = (points[0], points[points.len() - 1]);
        TrimChord {
            a: (pt(a.0), pt(a.1)),
            b: (pt(b.0), pt(b.1)),
            piece: Some(
                TrimPiece::new(
                    KnotVector::clamped(knots, p).unwrap(),
                    points.iter().map(|q| (pt(q.0), pt(q.1))).collect(),
                    vec![1.0; points.len()],
                )
                .unwrap(),
            ),
            forward: true,
            env: pt(env),
        }
    }

    /// `piece` with its first control point moved to `p`, minted again.
    fn first_point_at(piece: &TrimPiece, p: RPt2) -> TrimPiece {
        let mut control = piece.control().to_vec();
        control[0] = p;
        TrimPiece::new(piece.knots().clone(), control, piece.weights().to_vec()).unwrap()
    }

    /// A CURVED chart of bidegree `(pu, pv)` with one interior knot per
    /// direction, off the round's uniform piece grid, and a net that is
    /// linear-precision in NEITHER direction — so `g = |S_u×S_v|` is a
    /// real function with real Lipschitz constants and the area rule is
    /// no longer inert.
    ///
    /// Adopted from the t2q-r2 reviewer probe `curved_chart` that first
    /// executed the whole-box-hull refusal; the shape and the constants
    /// are that probe's.
    fn curved_chart(pu: usize, pv: usize) -> (KnotVector, KnotVector, Vec<RVec3>, Vec<f64>) {
        let kv = |p: usize, k: f64| {
            let mut v = vec![0.0; p + 1];
            v.push(k);
            v.extend(std::iter::repeat_n(1.0, p + 1));
            KnotVector::clamped(v, p).unwrap()
        };
        let (ku, kvv) = (kv(pu, 0.4), kv(pv, 0.3));
        let (nu, nv) = (ku.control_count(), kvv.control_count());
        let mut control = Vec::new();
        for i in 0..nu {
            for j in 0..nv {
                #[allow(clippy::cast_precision_loss)]
                let (fi, fj) = (i as f64 / (nu - 1) as f64, j as f64 / (nv - 1) as f64);
                let x = fi + 0.07 * (3.0 * fi + fj).sin();
                let y = fj + 0.05 * (2.0 * fi - 5.0 * fj).cos();
                let z = 0.4 + 0.3 * (4.0 * fi + 2.0 * fj).sin() * (fj * 3.0).cos();
                control.push([pt(x), pt(y), pt(z)]);
            }
        }
        (ku, kvv, control, vec![1.0; nu * nv])
    }

    /// The rectangle certificate on the same chart — the independent
    /// lane every curved-chart row below is measured against.
    fn rectangle(net: &PatchNet<'_>) -> FaceCutBounds {
        nurbs_patch_face_rounds::<f64>(
            net,
            (0.0, 1.0, 0.0, 1.0),
            4.0,
            0.0,
            Band::linear(Tol::witness()).unwrap(),
            RoundWindow::SCHEDULE,
            &mut to_target(Tol::witness().get().eps),
        )
        .map(|o| bounds_of(&o))
        .expect("the rectangle certificate answers its own fixture")
    }

    fn overlaps(a: Interval, b: Interval) -> bool {
        a.lo() <= b.hi() && b.lo() <= a.hi()
    }

    /// The lane over the whole schedule, at the run's own ε and band.
    fn trimmed(
        net: &PatchNet<'_>,
        chords: &[TrimChord],
        window: RoundWindow,
    ) -> Result<LaneRun, PropsError> {
        trimmed_patch_face_rounds::<f64>(
            net,
            chords,
            Band::linear(Tol::witness()).unwrap(),
            window,
            &mut to_target(Tol::witness().get().eps),
        )
    }

    /// **Q1 — a flat patch with a DIAGONAL `General` trim.**
    ///
    /// The loop `(0,0) → (1,0) → (1,1) → (0,0)` closes with a
    /// degree-1 `General` image, which is the class this head's
    /// producer actually mints (`PXN_IMAGE_DEGREE = 1`): the region is
    /// half the unit square, so `flux = c/2` and `area = 1/2`
    /// exactly. The rectangle lane cannot answer it at all — it reads
    /// endpoints and calls a diagonal a refusal — so this row is the
    /// whole point of the door.
    #[test]
    fn q1_a_flat_patch_with_a_diagonal_general_trim() {
        let c = 3.0;
        let (ku, kvv, control, w) = flat_chart(c);
        let net = patch(&ku, &kvv, &control, &w);
        let chords = vec![
            iso((0.0, 0.0), (1.0, 0.0)),
            iso((1.0, 0.0), (1.0, 1.0)),
            general(&[(1.0, 1.0), (0.0, 0.0)], 0.0),
        ];
        let out =
            trimmed(&net, &chords, RoundWindow::SCHEDULE).expect("the diagonal trim certifies");
        let LaneRun {
            bounds: b,
            end: LaneEnd::Accepted(_),
            ..
        } = out
        else {
            panic!("an exactly-integrated chord polygon converges at round 0: {out:?}")
        };
        encloses(b.flux, c * 0.5, "Q1 flux");
        encloses(b.area, 0.5, "Q1 area");
        // The chord polygon is integrated EXACTLY and a degree-1 image
        // IS its chord, so the only width here is outward rounding — the
        // target is four decades above it at every ε the matrix draws.
        let target = test_target::REPORTING_RESOLUTION_FACTOR * Tol::witness().get().eps;
        assert!(
            b.flux.width() < target * 3.0 * b.area.lo(),
            "Q1: width {:e} is not inside the reporting target's own bound",
            b.flux.width()
        );
    }

    /// A flat chart (`z = 1`) trimmed by a regular `n`-gon of curved
    /// degree-2 `General` arcs about `(0.5, 0.5)`, radius 0.35, each
    /// arc's control point `b` off its chord's midpoint, outward (PR
    /// 5589's second reviewer's probe), and its closed-form area: the
    /// polygon plus `n` parabolic segments of `chord·b/3` each. On the
    /// flat chart at height 1 the flux is the area.
    fn n_gon_of_arcs(n: usize, b: f64) -> (Vec<TrimChord>, f64) {
        let pts: Vec<(f64, f64)> = (0..n)
            .map(|k| {
                #[allow(clippy::cast_precision_loss)]
                let t = core::f64::consts::TAU * k as f64 / n as f64;
                (0.5 + 0.35 * t.cos(), 0.5 + 0.35 * t.sin())
            })
            .collect();
        let chords = (0..n)
            .map(|k| {
                let (a, c) = (pts[k], pts[(k + 1) % n]);
                let m = ((a.0 + c.0) / 2.0, (a.1 + c.1) / 2.0);
                let (dx, dy) = (m.0 - 0.5, m.1 - 0.5);
                let l = dx.hypot(dy);
                general(&[a, (m.0 + b * dx / l, m.1 + b * dy / l), c], 0.0)
            })
            .collect();
        #[allow(clippy::cast_precision_loss)]
        let nf = n as f64;
        let polygon = 0.5 * nf * 0.35 * 0.35 * (core::f64::consts::TAU / nf).sin();
        let chord = 2.0 * 0.35 * (core::f64::consts::PI / nf).sin();
        (chords, polygon + nf * chord * b / 3.0)
    }

    /// The trimmed lane on `chords` over the flat chart at height 1,
    /// against the default ε's target whatever the run's ε.
    fn n_gon_run(chords: &[TrimChord]) -> LaneRun {
        let (ku, kvv, control, w) = flat_chart(1.0);
        trimmed_patch_face_rounds::<f64>(
            &patch(&ku, &kvv, &control, &w),
            chords,
            Band::linear(Tol::witness()).unwrap(),
            RoundWindow::SCHEDULE,
            &mut to_target(1e-9),
        )
        .expect("the n-gon of arcs has an enclosure")
    }

    /// **A trimmed cell is priced by its chart's bidegree.** A flat
    /// bilinear chart's cell costs about a fourteenth of a biquadratic
    /// one's, so the triangle, square and hexagon of curved arcs on it
    /// (PR 5589's second reviewer's witnesses, which a single
    /// biquadratic weight stopped at rounds 5, 5 and 4) certify at
    /// round 6, where the round caps certified them, holding their
    /// closed forms.
    #[test]
    fn n_gons_of_arcs_on_a_flat_chart_certify_at_their_round() {
        for n in [3, 4, 6] {
            let (chords, truth) = n_gon_of_arcs(n, 0.016);
            let run = n_gon_run(&chords);
            assert!(
                matches!(run.end, LaneEnd::Accepted(_)) && run.round == 6,
                "the {n}-gon certifies at round 6: {run:?}"
            );
            encloses(run.bounds.flux, truth, &format!("{n}-gon flux"));
        }
    }

    /// **The wall bounds work, not rounds** (`FACE_WORK_WALL`'s doc): a
    /// face costlier per round than the calibrating one gets fewer
    /// rounds, and stops at the wall with a sound bracket. Thirty-two
    /// curved arcs at `b = 0.03` spend round 6's work by round 5, and
    /// the bracket round 5 hands back holds the closed form.
    #[test]
    fn a_32_gon_of_arcs_stops_at_the_wall_holding_its_closed_form() {
        let (chords, truth) = n_gon_of_arcs(32, 0.03);
        let run = n_gon_run(&chords);
        assert_eq!((run.end, run.rounds), (LaneEnd::Wall, 6), "{run:?}");
        encloses(run.bounds.flux, truth, "32-gon flux");
    }

    /// **The trimmed lane's envelope is a floor.** A wide envelope on
    /// the arc chord puts a pad under every round's width that no round
    /// removes and that the lunes fall under, so the lane stops at its
    /// floor rather than refining toward a target it cannot reach — at
    /// whichever window it is entered, since a round's stop reads that
    /// round alone.
    #[test]
    fn the_trimmed_lanes_envelope_is_a_floor() {
        let (ku, kvv, control, w) = flat_chart(1.0);
        let net = patch(&ku, &kvv, &control, &w);
        let chords = vec![
            iso((0.0, 0.0), (1.0, 0.0)),
            general(&[(1.0, 0.0), (0.7, 0.7), (0.0, 1.0)], 1e-2),
            iso((0.0, 1.0), (0.0, 0.0)),
        ];
        let whole = trimmed(&net, &chords, RoundWindow::SCHEDULE)
            .expect("the enveloped arc has an enclosure");
        assert_eq!(
            whole.end,
            LaneEnd::Floor,
            "the envelope is a floor: {whole:?}"
        );
        let at = trimmed(&net, &chords, RoundWindow::at(whole.rounds - 1))
            .expect("the same round, windowed");
        assert_eq!(
            (at.end, at.rounds),
            (LaneEnd::Floor, whole.rounds),
            "a window at the floor's round stops there too: {at:?}"
        );
    }

    /// **Q2 — a flat patch with an ARC trim, and its lune pad.**
    ///
    /// The quarter-disc-shaped region closed by a degree-2 `General`
    /// image (a parabola, so the arc is genuinely off its chord): the
    /// chart area is the triangle plus the quadratic Bézier's own
    /// bulge, `⅔` of its control triangle's area (Archimedes). The pad
    /// this costs is the LUNE pad, and it must be second order in the
    /// round's lever — the oblique box's whole reason.
    #[test]
    fn q2_an_arc_trim_pads_its_lunes_and_the_pad_is_second_order() {
        let c = 1.0;
        let (ku, kvv, control, w) = flat_chart(c);
        let net = patch(&ku, &kvv, &control, &w);
        let arc = [(1.0, 0.0), (0.7, 0.7), (0.0, 1.0)];
        let chords = vec![
            iso((0.0, 0.0), (1.0, 0.0)),
            general(&arc, 0.0),
            iso((0.0, 1.0), (0.0, 0.0)),
        ];
        // ½ (the chord triangle) + ⅔·(the control triangle) — the
        // parabolic segment's area, which is what the lune bounds.
        let tri = 0.5
            * (arc[0].0 * (arc[1].1 - arc[2].1)
                + arc[1].0 * (arc[2].1 - arc[0].1)
                + arc[2].0 * (arc[0].1 - arc[1].1))
                .abs();
        let truth = 0.5 + 2.0 / 3.0 * tri;
        let mut widths = Vec::new();
        for round in 0..2 {
            let out = trimmed(&net, &chords, RoundWindow::at(round))
                .unwrap_or_else(|e| panic!("Q2 round {round}: {e:?}"));
            let b = bounds_of(&out);
            encloses(b.flux, c * truth, &format!("Q2 round {round} flux"));
            encloses(b.area, truth, &format!("Q2 round {round} area"));
            widths.push(b.flux.width());
        }
        assert!(
            widths[0] > 1e-6,
            "Q2: an arc off its chord must COST a lune pad, got width {:e}",
            widths[0]
        );
        assert!(
            widths[0] / widths[1] >= 3.0,
            "Q2: the lune pad is O(h³) per piece, so doubling the lever must shrink the \
             width by ~4× — got {:e} then {:e} (ratio {})",
            widths[0],
            widths[1],
            widths[0] / widths[1]
        );
    }

    /// **Q3 — an all-iso rectangle loop through the NEW door agrees
    /// with the rectangle certificate.**
    ///
    /// The dispatch keeps every loft/sweep wall on today's lane
    /// (§8.1), so nothing routes here in production; the row exists
    /// because two engines that answer the same face must answer it
    /// the same, and a trapezoid sum that disagreed with the exact
    /// per-span rule would be caught nowhere else.
    #[test]
    fn q3_the_rectangle_loop_agrees_with_the_rectangle_certificate() {
        let c = 2.0;
        let (ku, kvv, control, w) = flat_chart(c);
        let net = patch(&ku, &kvv, &control, &w);
        let chords = vec![
            iso((0.0, 0.0), (1.0, 0.0)),
            iso((1.0, 0.0), (1.0, 1.0)),
            iso((1.0, 1.0), (0.0, 1.0)),
            iso((0.0, 1.0), (0.0, 0.0)),
        ];
        let mine = bounds_of(
            &trimmed(&net, &chords, RoundWindow::SCHEDULE)
                .expect("the rectangle loop certifies through the trimmed door"),
        );
        let theirs = nurbs_patch_face_rounds::<f64>(
            &net,
            (0.0, 1.0, 0.0, 1.0),
            4.0,
            0.0,
            Band::linear(Tol::witness()).unwrap(),
            RoundWindow::SCHEDULE,
            &mut to_target(Tol::witness().get().eps),
        )
        .map(|o| bounds_of(&o))
        .expect("the rectangle certificate answers its own fixture");
        encloses(mine.flux, c, "Q3 flux");
        encloses(mine.area, 1.0, "Q3 area");
        assert!(
            mine.flux.lo() <= theirs.flux.hi() && theirs.flux.lo() <= mine.flux.hi(),
            "Q3: the two lanes' flux enclosures do not overlap: {:?} vs {:?}",
            mine.flux,
            theirs.flux
        );
        assert!(
            mine.area.lo() <= theirs.area.hi() && theirs.area.lo() <= mine.area.hi(),
            "Q3: the two lanes' area enclosures do not overlap: {:?} vs {:?}",
            mine.area,
            theirs.area
        );
    }

    /// **Q4 — the planted envelope.**
    ///
    /// The certificate's envelope says the TRUE trim edge is within
    /// `env` metres of the stored image, so the lane owes the region
    /// that displacement can reach. Displacing Q1's diagonal outward
    /// by `env` grows the region's area by `L·env` to first order —
    /// exactly the `boundary_defect` this lane pads with, and `c·L·env`
    /// of flux against a pad of `L·env·p_bound` with
    /// `p_bound = sup|u| + sup|v| + |c| = 2 + c`. At `c = 10` the full
    /// pad covers the displaced truth and HALF of it does not, which
    /// is the mutant this row is written for.
    #[test]
    fn q4_the_planted_envelope_covers_the_displacement_it_states() {
        let c = 10.0;
        let env = 1.0e-4;
        let (ku, kvv, control, w) = flat_chart(c);
        let net = patch(&ku, &kvv, &control, &w);
        let square = |e: f64| {
            vec![
                iso((0.0, 0.0), (1.0, 0.0)),
                iso((1.0, 0.0), (1.0, 1.0)),
                general(&[(1.0, 1.0), (0.0, 0.0)], e),
            ]
        };
        let read = |e: f64| {
            bounds_of(
                &trimmed(&net, &square(e), RoundWindow::SCHEDULE)
                    .unwrap_or_else(|err| panic!("Q4 env {e:e}: {err:?}")),
            )
        };
        let bare = read(0.0);
        let padded = read(env);
        let half = read(env * 0.5);
        // The pad enters at FULL strength, both channels. The length it
        // multiplies is the DOOR's, not the chord's Euclidean one: the
        // door crosses the chord's chart VARIATION to metres per axis
        // (`|Δu|·sup|S_u| + |Δv|·sup|S_v|` = `1 + 1` here), which
        // over-bounds the metric length `√2` — the refusing direction
        // for the envelope pad and for the extent gate alike, and the
        // reason the door does not take a caller's number (#1368).
        let door_len = 2.0;
        let grew_flux = padded.flux.width() - bare.flux.width();
        let grew_area = padded.area.width() - bare.area.width();
        let want_flux = 2.0 * door_len * env * (2.0 + c);
        let want_area = 2.0 * door_len * env;
        assert!(
            (grew_flux - want_flux).abs() <= 1e-9 * want_flux,
            "Q4: the flux pad is L·env·p_bound each side — wanted {want_flux:e}, grew {grew_flux:e}"
        );
        assert!(
            (grew_area - want_area).abs() <= 1e-9 * want_area,
            "Q4: the area pad is L·env each side — wanted {want_area:e}, grew {grew_area:e}"
        );
        // And it covers the displacement it states, where half does not.
        let displaced = c * (0.5 + 2.0_f64.sqrt() * env);
        encloses(padded.flux, displaced, "Q4 displaced flux");
        assert!(
            half.flux.hi() < displaced,
            "Q4: HALF the stated envelope must not reach the displacement the whole one \
             states — {:?} still covers {displaced}",
            half.flux
        );
    }

    /// **Q5 — a fold that survives every bisection refuses typed,
    /// naming its row.**
    ///
    /// The image runs OUT along its chord and back: control points
    /// `(1,1) → (2,2) → (0,0)` on the diagonal, so the arc reaches
    /// `(1.5, 1.5)` and returns. The whole piece's control polygon
    /// steps backwards along its chord, and so does every half's at
    /// every depth — which is what distinguishes a real fold from the
    /// curvature artefact the next row pins.
    ///
    /// **Q5's first fixture did not pin this.** It was
    /// `(0,1) → (0,2) → (0,0)`, collinear on a VERTICAL chord: the
    /// flux rule skips a chord with `u_a = u_b` outright, and the
    /// monotone row answered by zero advance rather than by a fold.
    /// The t2q-r1 and t2q-r2 reviewers found the same thing
    /// independently; this fixture is the row its name says.
    #[test]
    fn q5_a_fold_over_its_chord_refuses_typed() {
        let (ku, kvv, control, w) = flat_chart(1.0);
        let net = patch(&ku, &kvv, &control, &w);
        let chords = vec![
            iso((0.0, 0.0), (1.0, 0.0)),
            iso((1.0, 0.0), (1.0, 1.0)),
            general(&[(1.0, 1.0), (2.0, 2.0), (0.0, 0.0)], 0.0),
        ];
        match trimmed(&net, &chords, RoundWindow::SCHEDULE) {
            Err(PropsError::QuadratureUnsupported { what }) => {
                assert!(
                    what.contains("monotonicity") && what.contains("UNRESOLVED"),
                    "Q5: the refusal names the row that decided it and what it decides: {what}"
                );
            }
            other => panic!("Q5: a fold has no certified answer, got {other:?}"),
        }
    }

    /// **Q5b — a piece that only LOOKS folded converges.**
    ///
    /// The cubic through `(1,1) → (0.2,0.2) → (0.8,0.8) → (0,0)` has a
    /// backward control step, so `piece_monotone` answers `Negative` on
    /// the whole piece — but `v′(t) < 0` throughout, so every half IS a
    /// graph over its own chord and the bisection resolves it. Without
    /// that ladder the lane would refuse an ordinary curve; Q5 above
    /// only pins the refusal, and a refusal is cheap to get right by
    /// refusing everything.
    ///
    /// Adopted from the t2q-r2 reviewer probe P10.
    #[test]
    fn q5b_a_piece_that_only_looks_folded_resolves_by_bisection() {
        let c = 3.0;
        let (ku, kvv, control, w) = flat_chart(c);
        let net = patch(&ku, &kvv, &control, &w);
        let chords = vec![
            iso((0.0, 0.0), (1.0, 0.0)),
            iso((1.0, 0.0), (1.0, 1.0)),
            general(&[(1.0, 1.0), (0.2, 0.2), (0.8, 0.8), (0.0, 0.0)], 0.0),
        ];
        let out = trimmed(&net, &chords, RoundWindow::SCHEDULE)
            .expect("bisection resolves a monotone piece with a backward control step");
        let b = bounds_of(&out);
        encloses(b.flux, c * 0.5, "Q5b flux");
        encloses(b.area, 0.5, "Q5b area");
    }

    std::thread_local! {
        /// Every block [`super::bezier_bisect`] has halved on this thread.
        pub(super) static BISECTED: core::cell::RefCell<Vec<Vec<RPt2>>> =
            const { core::cell::RefCell::new(Vec::new()) };
    }

    /// **The bisection's halves are the retired door's to the bit.**
    /// Every block the Q5 and Q5b fixtures bisect, at every depth (four
    /// blocks; on 2026-10-20 they were every block the props suite
    /// bisected), is
    /// halved by [`restrict_segment`] on `[0, 1]`; each channel's halves
    /// are those of the arithmetic `restrict_bernstein` ran
    /// (`geom_core::test_support::bernstein_restriction_as_retired`),
    /// ends and certification bit for bit.
    #[test]
    fn the_bisection_halves_are_the_retired_doors_to_the_bit() {
        BISECTED.with(|b| b.borrow_mut().clear());
        let (ku, kvv, control, w) = flat_chart(1.0);
        let net = patch(&ku, &kvv, &control, &w);
        let fold = vec![
            iso((0.0, 0.0), (1.0, 0.0)),
            iso((1.0, 0.0), (1.0, 1.0)),
            general(&[(1.0, 1.0), (2.0, 2.0), (0.0, 0.0)], 0.0),
        ];
        let _ = trimmed(&net, &fold, RoundWindow::SCHEDULE);
        let (ku, kvv, control, w) = flat_chart(3.0);
        let net = patch(&ku, &kvv, &control, &w);
        let looks_folded = vec![
            iso((0.0, 0.0), (1.0, 0.0)),
            iso((1.0, 0.0), (1.0, 1.0)),
            general(&[(1.0, 1.0), (0.2, 0.2), (0.8, 0.8), (0.0, 0.0)], 0.0),
        ];
        let _ = trimmed(&net, &looks_folded, RoundWindow::SCHEDULE);
        let blocks = BISECTED.with(|b| b.borrow().clone());
        assert_eq!(blocks.len(), 4, "the fixtures bisect four blocks");
        let bits = |row: &[Interval]| -> Vec<(u64, u64, bool)> {
            row.iter()
                .map(|x| (x.lo().to_bits(), x.hi().to_bits(), x.is_certified()))
                .collect()
        };
        for (n, block) in blocks.iter().enumerate() {
            let (l, r) = super::bezier_bisect(block);
            for (axis, pick) in [("u", 0usize), ("v", 1usize)] {
                let channel: Vec<Interval> = block
                    .iter()
                    .map(|q| if pick == 0 { q.0 } else { q.1 })
                    .collect();
                let got = |h: &[RPt2]| -> Vec<Interval> {
                    h.iter()
                        .map(|q| if pick == 0 { q.0 } else { q.1 })
                        .collect()
                };
                for ((a, b), half) in [((0.0, 0.5), got(&l)), ((0.5, 1.0), got(&r))] {
                    let want = geom_core::test_support::bernstein_restriction_as_retired(
                        &channel,
                        (pt(a), pt(b)),
                    )
                    .expect("a half of [0, 1]");
                    assert_eq!(bits(&half), bits(&want), "block {n} {axis} on [{a}, {b}]");
                }
            }
        }
    }

    /// **Q6 — past the exact outer rule's window the composite arm
    /// answers; past the inner rule's window the lane refuses typed.**
    ///
    /// The chord integrand's `u`-degree is `3p_u + 3p_v − 1` and the
    /// exact closed rule's interval count tops out at 12, so a
    /// degree-`(3, 2)` chart takes the composite outer rule. On the
    /// flat chart `(u, v, 1)` (`f = 1`) over half the square it holds
    /// `1/2`. The inner rule stays exact and needs `3p_v − 1 ≤ 12`, so a
    /// degree-`(1, 5)` chart refuses at the window it names.
    #[test]
    fn q6_a_chart_past_the_newton_cotes_window_refuses_typed() {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let kvv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let mut control = Vec::new();
        for u in [0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0] {
            for v in [0.0, 0.5, 1.0] {
                control.push([pt(u), pt(v), pt(1.0)]);
            }
        }
        let w = vec![1.0; 12];
        let chords = vec![
            iso((0.0, 0.0), (1.0, 0.0)),
            iso((1.0, 0.0), (1.0, 1.0)),
            general(&[(1.0, 1.0), (0.0, 0.0)], 0.0),
        ];
        let b = met(trimmed(
            &patch(&ku, &kvv, &control, &w),
            &chords,
            RoundWindow::SCHEDULE,
        ))
        .expect("Q6: degree (3, 2) takes the composite arm");
        encloses(b.flux, 0.5, "Q6 (3, 2) flux");
        encloses(b.area, 0.5, "Q6 (3, 2) area");

        let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let mut knots = vec![0.0; 6];
        knots.extend([1.0; 6]);
        let kvv = KnotVector::clamped(knots, 5).unwrap();
        let mut control = Vec::new();
        for u in [0.0, 1.0] {
            for j in 0..6 {
                control.push([pt(u), pt(f64::from(j) / 5.0), pt(1.0)]);
            }
        }
        let w = vec![1.0; 12];
        match trimmed(
            &patch(&ku, &kvv, &control, &w),
            &chords,
            RoundWindow::SCHEDULE,
        ) {
            Err(PropsError::QuadratureUnsupported { what }) => {
                assert!(
                    what == TRIM_NC_WINDOW,
                    "Q6: the refusal names its own window: {what}"
                );
            }
            other => panic!("Q6: degree (1, 5) is past the inner rule, got {other:?}"),
        }
    }

    /// **Q7 — interior knots the chord crosses.**
    ///
    /// Q1's loop on a chart with one interior knot per direction and a
    /// NON-UNIFORM net, so each direction's two spans carry genuinely
    /// different polynomials: `S(u, v) = (x(u), w(v), c)` with `x` and
    /// `w` the piecewise-linear maps through `(0, ½, 1)` on knots
    /// `(0, 0.4, 1)` and `(0, 0.3, 1)`. The integrand
    /// `f = c·x′(u)·w′(v)` therefore JUMPS at both, and the diagonal
    /// crosses the `v` knot at `u = 0.3`.
    ///
    /// **Three things about this fixture are load-bearing, and two of
    /// them were measured rather than reasoned.** A UNIFORM net makes
    /// the surface globally linear and every rule exact whether or not
    /// the sub-chord is split. And NEITHER knot may sit on the round's
    /// own uniform piece grid: at `v = ½` the crossing lands exactly on
    /// a boundary of the `IMAGE_INIT_PIECES = 8` subdivision, no
    /// sub-chord straddles it, and a lane that had dropped the crossing
    /// cut entirely still answers correctly — as does one that dropped
    /// the `u`-knot cut while the `u` knot sat at `0.25`. `0.3` and
    /// `0.4` are off that grid; `0.25` and `0.5` are on it.
    #[test]
    fn q7_a_chord_crossing_interior_knots_is_split_at_them() {
        let c = 3.0;
        let (uk, vk) = (0.4, 0.3);
        let ku = KnotVector::clamped(vec![0.0, 0.0, uk, 1.0, 1.0], 1).unwrap();
        let kvv = KnotVector::clamped(vec![0.0, 0.0, vk, 1.0, 1.0], 1).unwrap();
        let mut control = Vec::new();
        for u in [0.0, 0.5, 1.0] {
            for v in [0.0, 0.5, 1.0] {
                control.push([pt(u), pt(v), pt(c)]);
            }
        }
        let w = vec![1.0; 9];
        let chords = vec![
            iso((0.0, 0.0), (1.0, 0.0)),
            iso((1.0, 0.0), (1.0, 1.0)),
            general(&[(1.0, 1.0), (0.0, 0.0)], 0.0),
        ];
        // `f = c·x′·w′` and `g = |x′·w′|` over the triangle
        // `{0 < v < u < 1}`, where `∫∫ = ∫₀¹ x′(u)·w(u) du` because
        // `w(0) = 0`. `x′` is constant and `w` affine on each of the
        // three stretches `[0, vk]`, `[vk, uk]`, `[uk, 1]`, so the
        // trapezoid below is the closed form and not a quadrature.
        let xp = |u: f64| if u < uk { 0.5 / uk } else { 0.5 / (1.0 - uk) };
        let wv = |v: f64| {
            if v < vk {
                0.5 / vk * v
            } else {
                0.5 + 0.5 / (1.0 - vk) * (v - vk)
            }
        };
        let seg = |a: f64, b: f64| xp(0.5 * (a + b)) * 0.5 * (wv(a) + wv(b)) * (b - a);
        let g_int = seg(0.0, vk) + seg(vk, uk) + seg(uk, 1.0);
        let out = trimmed(
            &patch(&ku, &kvv, &control, &w),
            &chords,
            RoundWindow::SCHEDULE,
        )
        .expect("the knot-crossing diagonal certifies");
        let b = bounds_of(&out);
        encloses(b.flux, c * g_int, "Q7 flux");
        encloses(b.area, g_int, "Q7 area");
    }

    /// **A constant weight is the polynomial lanes'.** The curved chart
    /// with every weight one constant (`2`, `3`, `0.1`) is the polynomial
    /// patch on its net ([`constant_weight`]), so the rectangle lane and
    /// the trimmed lane each answer it as the unit-weight chart, to the
    /// bit (neither reads a weight on its polynomial arm); a chart whose
    /// weights vary is rational, and the trimmed lane refuses it by name.
    #[test]
    fn a_constant_weight_chart_is_measured_as_its_polynomial() {
        let (ku, kvv, control, unit) = curved_chart(2, 2);
        let bits = |b: FaceCutBounds| {
            [b.flux.lo(), b.flux.hi(), b.area.lo(), b.area.hi()].map(f64::to_bits)
        };
        let loop_ = vec![
            iso((0.0, 0.0), (1.0, 0.0)),
            iso((1.0, 0.0), (1.0, 1.0)),
            general(&[(1.0, 1.0), (0.0, 0.0)], 0.0),
        ];
        let run = |w: &[f64]| {
            trimmed(
                &patch(&ku, &kvv, &control, w),
                &loop_,
                RoundWindow::SCHEDULE,
            )
        };
        let rect_unit = bits(rectangle(&patch(&ku, &kvv, &control, &unit)));
        let trim_unit = bits(bounds_of(&run(&unit).expect("the unit chart")));
        for c in [2.0, 3.0, 0.1] {
            let w = vec![c; unit.len()];
            assert_eq!(
                bits(rectangle(&patch(&ku, &kvv, &control, &w))),
                rect_unit,
                "the rectangle lane on weights {c}"
            );
            assert_eq!(
                bits(bounds_of(&run(&w).expect("a constant chart is polynomial"))),
                trim_unit,
                "the trimmed lane on weights {c}"
            );
        }
        let mut varying = unit.clone();
        varying[4] = 2.0;
        match run(&varying) {
            Err(PropsError::QuadratureUnsupported { what }) => {
                assert!(what.contains("RATIONAL chart"), "{what}");
            }
            other => panic!("a varying weight vector is rational: {other:?}"),
        }
    }

    /// **Q8 — a CURVED chart's area agrees with the rectangle lane's.**
    ///
    /// The headline row of the fix pass. Q1–Q7 all live on the flat
    /// chart, where `g = |S_u×S_v| ≡ 1` and every Lipschitz constant is
    /// zero — the area machinery is inert there, and a version of it
    /// that read ONE hull over the whole trim box and padded every
    /// 16×16 cell with it passed all seven. On an ordinary curved chart
    /// that hull was `[0.33, 5.56]` against `g(½,½) = 0.457`, the
    /// region's area came out `[−0.549, 1.742]`, and `area.lo() < 0`
    /// drove the extent gate to `Err(DegenerateFace)` for a face of
    /// area ≈ 0.6.
    ///
    /// The oracle is the independent lane: the two complementary
    /// regions of the diagonal must have areas summing to the whole
    /// patch's, and each region's own enclosure must be a positive
    /// extent the gate accepts. Adopted from the t2q-r2 reviewer probes
    /// P5/P11, whose curved chart this is.
    #[test]
    fn q8_a_curved_chart_area_agrees_with_the_rectangle_lane() {
        for (pu, pv) in [(1usize, 2usize), (2, 2), (3, 2), (3, 3)] {
            let (ku, kvv, control, w) = curved_chart(pu, pv);
            let net = patch(&ku, &kvv, &control, &w);
            let rect = rectangle(&net);
            // Both triangles of the diagonal, the General walked each
            // way — `second` is the orientation the whole-box hull
            // refused.
            let first = vec![
                iso((0.0, 0.0), (1.0, 0.0)),
                iso((1.0, 0.0), (1.0, 1.0)),
                general(&[(1.0, 1.0), (0.0, 0.0)], 0.0),
            ];
            let second = vec![
                general(&[(0.0, 0.0), (1.0, 1.0)], 0.0),
                iso((1.0, 1.0), (0.0, 1.0)),
                iso((0.0, 1.0), (0.0, 0.0)),
            ];
            let mut sum_a = Interval::zero();
            let mut sum_f = Interval::zero();
            for (label, chords) in [("first", &first), ("second", &second)] {
                let out = trimmed(&net, chords, RoundWindow::SCHEDULE).unwrap_or_else(|e| {
                    panic!(
                        "Q8 ({pu},{pv}) {label}: a curved chart's half-patch is an \
                                ordinary face, got {e:?}"
                    )
                });
                let b = bounds_of(&out);
                assert!(
                    b.area.lo() > 0.0,
                    "Q8 ({pu},{pv}) {label}: the area must certify a POSITIVE extent, got {:?}",
                    b.area
                );
                sum_a = sum_a + b.area;
                sum_f = sum_f + b.flux;
            }
            assert!(
                overlaps(sum_a, rect.area),
                "Q8 ({pu},{pv}): the two halves' areas {sum_a:?} must sum to the rectangle \
                 lane's {:?} on the same chart",
                rect.area
            );
            assert!(
                overlaps(sum_f, rect.flux),
                "Q8 ({pu},{pv}): the two halves' fluxes {sum_f:?} must sum to the rectangle \
                 lane's {:?}",
                rect.flux
            );
            // And the area rule is no longer inert: a curved chart's
            // enclosure has to be narrower than the whole-box hull it
            // used to be padded with.
            assert!(
                sum_a.width() < rect.area.mag(),
                "Q8 ({pu},{pv}): the area pad is still whole-patch sized: {sum_a:?}"
            );
        }
    }

    /// **Q9 — the chord rules' ORDERS, pinned by the ANSWER.**
    ///
    /// The trimmed lane runs two closed Newton–Cotes rules: the OUTER
    /// one along the chord at order `3p_u + 3p_v − 1`, and the INNER
    /// one across the patch at `3p_v − 1`. Both survived every earlier
    /// row because every fixture's chord integrand had degree ≤ 2
    /// (Q1–Q3 are flat, so `h` is affine; Q7's `f` is piecewise
    /// constant; E1's chart is a bilinear surface restated).
    ///
    /// **Which order this fixture pins, and at what cut** — measured
    /// on `curved_chart(2, 2)` with the three-chord loop below, taking
    /// round 0's midpoint against the `1e-12` window asserted here:
    ///
    /// | rule cut | round-0 midpoint | from `EXACT` | this row |
    /// | --- | --- | --- | --- |
    /// | none (shipped) | `3.2542626001979752e-1` | `2.3e-17` | passes |
    /// | OUTER, 2 short | `3.2542626001979757e-1` | `0`, and round 2 bit-identical | **passes** |
    /// | INNER, 2 short | `3.2542430502718228e-1` | `1.95e-6` | reds |
    /// | OUTER, 4 short | `3.2542626002142860e-1` | `1.63e-12` | reds |
    /// | OUTER, 6 short | `3.2542623695793438e-1` | `2.31e-8` | reds |
    ///
    /// So what this fixture classifies at two counts short is the
    /// **inner** order; the outer one it catches only from four, and
    /// there by `1.63e-12` against a `1e-12` window — a hair, not a
    /// classification. The chord integrand's `u`-degree is what decides
    /// that, and this chart's image is degree 1. Reaching `h`-degree 11
    /// takes a chart whose image is not, which brings a lune pad along
    /// to blur the comparison: a different fixture rather than a wider
    /// one, and it is scheduled with the rest at
    /// `work/quad/q9-refinement-invariance-does-not-classify-the-rule-order`.
    ///
    /// **The oracle is a golden plus a consistency check, not a
    /// structural instrument**, and that is the residue the filed row
    /// carries. `EXACT` is the shipped rule's own output; what keeps it
    /// from being purely circular is that a genuinely different
    /// quadrature reproduces it — the outer-two-short rule to
    /// `5.5e-17`, and the outer-four-short rule exactly, at round 2.
    ///
    /// **Why the refinement half cannot classify on its own, measured.**
    /// This row used to gate `|m₀ − m₂|` at a quarter ulp, and that
    /// separated the shipped order (which answered the two rounds BIT
    /// for bit) from the inner-two-short mutant (one ulp apart). Both
    /// numbers were the arithmetic's rounding and not the rule's error:
    /// that mutant is refinement-invariant too — its answer is wrong by
    /// `1.95e-6` at BOTH rounds — so the old gate was separating an
    /// exact zero from one ulp of luck. Certification arithmetic pads only where an
    /// operation is inexact now, so an enclosure is no longer symmetric
    /// about the round-to-nearest value and its midpoint carries that
    /// asymmetry: the shipped order reads two ulps apart (`1.11e-16` on
    /// `0.325`) and the mutant one, which ranks them BACKWARDS.
    /// Re-derived rather than widened
    /// (`memories/output-stability-as-justification.md`): the
    /// classification moves onto the quantity that carries the signal,
    /// and the refinement half stays as the consistency claim it can
    /// support — which is not nothing, since the outer-four-short
    /// mutant reds on it as well as on the answer.
    #[test]
    fn q9_the_inner_rules_order_is_pinned_at_two_short_and_the_outer_at_four() {
        let (ku, kvv, control, w) = curved_chart(2, 2);
        let net = patch(&ku, &kvv, &control, &w);
        let chords = vec![
            iso((0.0, 0.0), (1.0, 0.0)),
            iso((1.0, 0.0), (1.0, 1.0)),
            general(&[(1.0, 1.0), (0.0, 0.0)], 0.0),
        ];
        let read = |round: usize| {
            bounds_of(
                &trimmed(&net, &chords, RoundWindow::at(round))
                    .unwrap_or_else(|e| panic!("Q9 round {round}: {e:?}")),
            )
        };
        let (r0, r2) = (read(0), read(2));
        // **The classification.** The shipped order's answer, pinned
        // at a window seven orders above the arithmetic's own width
        // and six below the two-counts-short mutant's error (the doc
        // above carries both measurements): a rule short of the degree
        // integrates a DIFFERENT number, and that is the difference
        // this row exists to see.
        const EXACT: f64 = 3.254_262_600_197_975e-1;
        const WINDOW: f64 = 1e-12;
        let (m0, m2) = (mid(r0.flux), mid(r2.flux));
        for (round, m) in [(0, m0), (2, m2)] {
            assert!(
                (m - EXACT).abs() <= WINDOW,
                "Q9: round {round} integrates {m:e}, which is {:e} from the exact rule's \
                 {EXACT:e} — past the {WINDOW:e} window, so the rule is short of the \
                 degree rather than exact",
                (m - EXACT).abs()
            );
        }
        // Refinement invariance, at the width the arithmetic supports:
        // an enclosure is padded only where an operation was inexact,
        // so the two rounds' midpoints sit inside each other's bracket
        // rather than on top of each other.
        let widths = r0.flux.width().max(r2.flux.width());
        assert!(
            (m0 - m2).abs() <= widths,
            "Q9: an EXACT rule answers the same integral at every chord count — round 0 \
             {m0:e} and round 2 {m2:e} differ by {:e}, past the wider enclosure's own \
             {widths:e}",
            (m0 - m2).abs()
        );
        assert!(
            overlaps(r0.flux, r2.flux),
            "Q9: round 0 {:?} and round 2 {:?} must also overlap as enclosures",
            r0.flux,
            r2.flux
        );
        // …and it is exact to outward rounding, not merely consistent: a
        // composite short of the degree would agree only to its own
        // error. **Short of the degree, not short of the order**: a
        // closed Newton–Cotes rule on an EVEN interval count is exact
        // one degree past its order, so a single count off either rule
        // still integrates this integrand exactly and reds nothing.
        // `mv − 2` is the mutant this row kills outright; `mu` has to
        // lose four before this fixture sees it at all (the doc's
        // table). The spec's counts are the first that are exact for
        // every parity.
        assert!(
            r0.flux.width() < 1e-9 * r0.flux.mag().max(1.0),
            "Q9: the exact lane's width is the nodes' and weights' outward rounding, got {:e}",
            r0.flux.width()
        );
    }

    /// **Q10 — the VERTEX pad, on the fat brackets that need it.**
    ///
    /// Every other row calls the door with `pt(..)` endpoints, so
    /// `vertex_slack` is exactly zero and the pad is unexercised: the
    /// PR body once claimed CI's interval rows were its evidence, and
    /// they are not — `m8_4_intersection_iso` is `Body<f64>` throughout.
    /// This row is the engine's interval-lane shape written directly:
    /// one shared vertex
    /// whose bracket is fat and whose MIDPOINT is off the true corner,
    /// which is exactly what the pad pays for.
    ///
    /// Adopted from the t2q-r2 reviewer probe P4.
    #[test]
    fn q10_a_fat_shared_vertex_is_paid_by_the_vertex_pad() {
        let c = 3.0;
        let d = 1.0e-4;
        let (ku, kvv, control, w) = flat_chart(c);
        let net = patch(&ku, &kvv, &control, &w);
        let fat = (
            Interval::from_bounds(1.0 - 2.0 * d, 1.0),
            Interval::from_bounds(1.0, 1.0 + 2.0 * d),
        );
        let mut g = general(&[(1.0, 1.0), (0.0, 0.0)], 0.0);
        g.a = fat;
        g.piece = g.piece.as_ref().map(|piece| first_point_at(piece, fat));
        let mut rim = iso((1.0, 0.0), (1.0, 1.0));
        rim.b = fat;
        let chords = vec![iso((0.0, 0.0), (1.0, 0.0)), rim, g];
        let out =
            trimmed(&net, &chords, RoundWindow::SCHEDULE).unwrap_or_else(|e| panic!("Q10: {e:?}"));
        let b = bounds_of(&out);
        // The true corner is (1,1) and the bracket's midpoint is not;
        // the truth has to stay inside.
        encloses(b.flux, c * 0.5, "Q10 flux");
        encloses(b.area, 0.5, "Q10 area");
        let want = 2.0 * 2.0_f64.sqrt() * (2.0 * d) * c;
        assert!(
            b.flux.width() >= want * 0.99,
            "Q10: the connector pad (chord · slack · sup|f|, each side) is missing from the \
             width: {:e} < {:e}",
            b.flux.width(),
            want
        );
    }

    /// **Q11 — a walk that does not close refuses typed.**
    ///
    /// Green's theorem is a statement about a closed curve, and an open
    /// walk has no winding number: the lane used to answer one anyway.
    /// The rectangle certificate refuses the analogous shape (its
    /// shoelace must equal ±the rectangle's area); this is that guard
    /// for a polygon whose vertices are brackets. Both reviewers found
    /// it independently — a `General` whose start disagreed with the
    /// previous iso's end by 0.05 got `Ok(Converged)` with a flux
    /// enclosing neither plausible closing.
    ///
    /// The second half of the row is the one that matters more: an
    /// in-bracket disagreement (two pcurves reading one shared vertex)
    /// must be PAID, not refused — that is the shipped assembler's own
    /// case.
    ///
    /// Adopted from the t2q-r2 reviewer probe P1 and the t2q-r1 probe
    /// P10.
    #[test]
    fn q11_an_open_walk_refuses_and_an_in_bracket_gap_is_paid() {
        let c = 3.0;
        let (ku, kvv, control, w) = flat_chart(c);
        let net = patch(&ku, &kvv, &control, &w);
        let open = vec![
            iso((0.0, 0.0), (1.0, 0.0)),
            iso((1.0, 0.0), (1.0, 1.0)),
            general(&[(0.95, 1.0), (0.0, 0.0)], 0.0),
        ];
        match trimmed(&net, &open, RoundWindow::SCHEDULE) {
            Err(PropsError::QuadratureUnsupported { what }) => {
                assert!(
                    what.contains("does not close"),
                    "Q11: the refusal names the premise it guards: {what}"
                );
            }
            other => panic!("Q11: Green's theorem has no answer for an open walk: {other:?}"),
        }
        // In-bracket: the two sides of one shared vertex are DIFFERENT
        // brackets, each containing the other's reading — two pcurves
        // reading one vertex. The shipped assembler hulls them into one
        // bracket before the door (`topo`'s `quad_lane::trimmed_face`),
        // so a shipped face's gap is `0`; the door pays it for a caller
        // that does not. Their midpoints differ by `0.375·d` along `u` (`1 − d/2` and
        // `1 − 7d/8`), so the walk does not close exactly and the lane
        // PAYS the gap rather than refusing it.
        let d = 1.0e-3;
        let mut rim = iso((1.0, 0.0), (1.0, 1.0));
        rim.b = (Interval::from_bounds(1.0 - d, 1.0), pt(1.0));
        let mut g = general(&[(1.0 - d, 1.0), (0.0, 0.0)], 0.0);
        g.a = (
            Interval::from_bounds(1.0 - 1.5 * d, 1.0 - 0.25 * d),
            pt(1.0),
        );
        g.piece = g.piece.as_ref().map(|piece| first_point_at(piece, g.a));
        let closed = vec![iso((0.0, 0.0), (1.0, 0.0)), rim, g];
        let b = bounds_of(
            &trimmed(&net, &closed, RoundWindow::SCHEDULE)
                .unwrap_or_else(|e| panic!("Q11: an in-bracket gap is paid, not refused: {e:?}")),
        );
        // The truth is the polygon the caller MEANT — the corner at
        // (1,1) — and the gap pad `|Δu|·sup|G_f|` is what reaches it
        // from the walk that was actually handed over.
        encloses(b.flux, c * 0.5, "Q11 in-bracket flux");
    }

    /// The bidegree-`(3, 2)` Bézier chart `S = (3u, 2v, z(u, v))` on the
    /// unit square, `z` the Bernstein patch of the height net `zs` (rows
    /// in `u`), and the closed form of its flux over the triangle
    /// `(0,0) → (1,0) → (1,1)`. The chord integrand is
    /// `f = 6(z − u·z_u − v·z_v)`, so `z`'s monomial `u^a v^b` integrates
    /// to `6(1 − a − b)/((b + 1)(a + b + 2))`.
    fn cubic_by_quadratic(
        zs: [[f64; 3]; 4],
    ) -> (KnotVector, KnotVector, Vec<RVec3>, Vec<f64>, f64) {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let kvv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let mut control = Vec::new();
        for (i, row) in zs.iter().enumerate() {
            for (j, z) in row.iter().enumerate() {
                #[allow(clippy::cast_precision_loss)]
                control.push([pt(i as f64), pt(j as f64), pt(*z)]);
            }
        }
        let binom = |n: usize, k: usize| -> f64 {
            (0..k).fold(1.0, |acc, m| {
                #[allow(clippy::cast_precision_loss)]
                let r = acc * (n - m) as f64 / (m + 1) as f64;
                r
            })
        };
        // The coefficient of `t^k` in `B_i^n(t)`.
        let mono = |n: usize, i: usize, k: usize| -> f64 {
            if k < i {
                return 0.0;
            }
            let sign = if (k - i).is_multiple_of(2) { 1.0 } else { -1.0 };
            sign * binom(n, i) * binom(n - i, k - i)
        };
        let mut truth = 0.0;
        for a in 0..=3 {
            for b in 0..=2 {
                let mut c = 0.0;
                for (i, row) in zs.iter().enumerate() {
                    for (j, z) in row.iter().enumerate() {
                        c += z * mono(3, i, a) * mono(2, j, b);
                    }
                }
                #[allow(clippy::cast_precision_loss)]
                let (af, bf) = (a as f64, b as f64);
                truth += 6.0 * c * (1.0 - af - bf) / ((bf + 1.0) * (af + bf + 2.0));
            }
        }
        (ku, kvv, control, vec![1.0; 12], truth)
    }

    /// The height net of the composite rows: curved enough that the
    /// midpoint rule's own error is far above the enclosure's other
    /// widths at the first rounds.
    const CUBIC_HEIGHTS: [[f64; 3]; 4] = [
        [0.0, 0.5, 0.25],
        [1.0, -0.5, 0.75],
        [-0.25, 1.0, 0.5],
        [0.5, 0.0, -0.5],
    ];

    fn the_triangle() -> Vec<TrimChord> {
        vec![
            iso((0.0, 0.0), (1.0, 0.0)),
            iso((1.0, 0.0), (1.0, 1.0)),
            iso((1.0, 1.0), (0.0, 0.0)),
        ]
    }

    /// **Q14 — the composite arm holds a closed form at every round,
    /// and narrows at third order.**
    ///
    /// The degree-`(3, 2)` chart [`cubic_by_quadratic`] is past the
    /// exact outer rule's window. Its diagonal chord is the one the rule
    /// integrates (`G_f = 0` along `v = 0`, and the vertical chord has
    /// no `du`), cut into `8·2^r` pieces at round `r`. Each round's
    /// enclosure holds the closed form, and each narrows the last by
    /// about eight: the remainder `h³/24·φ''` is a correction whose own
    /// width is `O(h⁴)` a piece, over `1/h` pieces.
    #[test]
    fn q14_the_composite_arm_holds_a_closed_form_at_every_round() {
        let (ku, kvv, control, w, truth) = cubic_by_quadratic(CUBIC_HEIGHTS);
        let chords = the_triangle();
        let mut widths = Vec::new();
        for round in 0..3 {
            let run = trimmed(
                &patch(&ku, &kvv, &control, &w),
                &chords,
                RoundWindow::at(round),
            )
            .expect("Q14: the composite arm answers");
            encloses(run.bounds.flux, truth, &format!("Q14 round {round} flux"));
            widths.push(run.bounds.flux.width());
        }
        for pair in widths.windows(2) {
            assert!(
                pair[1] < 0.15 * pair[0] && pair[1] > 0.1 * pair[0],
                "Q14: a round narrows the last by about eight (third order): {widths:?}"
            );
        }
        let b = met(trimmed(
            &patch(&ku, &kvv, &control, &w),
            &chords,
            RoundWindow::SCHEDULE,
        ))
        .expect("Q14: the composite arm reaches the target");
        encloses(b.flux, truth, "Q14 converged flux");
    }

    /// **Q15 — a composite face that never meets its caller stops at the
    /// work wall, holding its closed form.** A caller that accepts no
    /// round runs the composite arm until the next round would pass
    /// `FACE_WORK_WALL`; the round it hands back is the narrowest, still
    /// an enclosure, which the door hands back as its bracket.
    #[test]
    fn q15_a_composite_face_short_of_its_caller_stops_at_the_wall() {
        let (ku, kvv, control, w, truth) = cubic_by_quadratic(CUBIC_HEIGHTS);
        let run = trimmed_patch_face_rounds::<f64>(
            &patch(&ku, &kvv, &control, &w),
            &the_triangle(),
            Band::linear(Tol::witness()).unwrap(),
            RoundWindow::SCHEDULE,
            &mut |_, _| Ok(false),
        )
        .expect("Q15: the composite arm answers");
        assert_eq!(
            run.end,
            LaneEnd::Wall,
            "Q15: the wall ends the run: {run:?}"
        );
        encloses(run.bounds.flux, truth, "Q15 flux at the wall");
    }

    /// An independent reference for the trimmed lane's flux: the chart
    /// evaluated by its own Cox–de Boor recursion, the region clipped to
    /// each knot cell (where the integrand is one polynomial), each
    /// piece fanned into triangles and each triangle integrated by a
    /// Duffy-mapped 14 × 14 Gauss–Legendre rule on nodes found here.
    /// Nothing in it is the lane's: no `PatchGrid`, no Green form, no
    /// Newton–Cotes.
    mod reference {
        use super::RVec3;

        /// `N_{i,p}(t)` and its derivative, by the recursion with
        /// `0/0 = 0` and half-open degree-0 supports.
        fn basis(knots: &[f64], i: usize, p: usize, t: f64) -> f64 {
            if p == 0 {
                return if knots[i] <= t && t < knots[i + 1] {
                    1.0
                } else {
                    0.0
                };
            }
            let left = knots[i + p] - knots[i];
            let right = knots[i + p + 1] - knots[i + 1];
            let a = if left > 0.0 {
                (t - knots[i]) / left * basis(knots, i, p - 1, t)
            } else {
                0.0
            };
            let b = if right > 0.0 {
                (knots[i + p + 1] - t) / right * basis(knots, i + 1, p - 1, t)
            } else {
                0.0
            };
            a + b
        }

        fn basis_d(knots: &[f64], i: usize, p: usize, t: f64) -> f64 {
            #[allow(clippy::cast_precision_loss)]
            let pf = p as f64;
            let left = knots[i + p] - knots[i];
            let right = knots[i + p + 1] - knots[i + 1];
            let a = if left > 0.0 {
                pf / left * basis(knots, i, p - 1, t)
            } else {
                0.0
            };
            let b = if right > 0.0 {
                pf / right * basis(knots, i + 1, p - 1, t)
            } else {
                0.0
            };
            a - b
        }

        /// A tensor B-spline chart, row-major in `u`.
        pub(super) struct Chart {
            pub(super) ku: Vec<f64>,
            pub(super) kv: Vec<f64>,
            pub(super) pu: usize,
            pub(super) pv: usize,
            pub(super) control: Vec<[f64; 3]>,
        }

        impl Chart {
            /// `f = ⟨S, S_u×S_v⟩` at an interior point of a knot cell.
            fn f(&self, u: f64, v: f64) -> f64 {
                let (nu, nv) = (self.ku.len() - self.pu - 1, self.kv.len() - self.pv - 1);
                let (mut s, mut su, mut sv) = ([0.0; 3], [0.0; 3], [0.0; 3]);
                for i in 0..nu {
                    let (bu, du) = (
                        basis(&self.ku, i, self.pu, u),
                        basis_d(&self.ku, i, self.pu, u),
                    );
                    for j in 0..nv {
                        let (bv, dv) = (
                            basis(&self.kv, j, self.pv, v),
                            basis_d(&self.kv, j, self.pv, v),
                        );
                        let c = self.control[i * nv + j];
                        for k in 0..3 {
                            s[k] += bu * bv * c[k];
                            su[k] += du * bv * c[k];
                            sv[k] += bu * dv * c[k];
                        }
                    }
                }
                let n = [
                    su[1] * sv[2] - su[2] * sv[1],
                    su[2] * sv[0] - su[0] * sv[2],
                    su[0] * sv[1] - su[1] * sv[0],
                ];
                s[0] * n[0] + s[1] * n[1] + s[2] * n[2]
            }

            /// The lane's net, bracketed.
            pub(super) fn net(&self) -> Vec<RVec3> {
                self.control.iter().map(|c| c.map(super::pt)).collect()
            }
        }

        /// Gauss–Legendre nodes and weights on `[0, 1]`, by Newton on
        /// `P_n` from the Chebyshev-like guess.
        fn gauss(n: usize) -> Vec<(f64, f64)> {
            let mut out = Vec::new();
            #[allow(clippy::cast_precision_loss)]
            let nf = n as f64;
            for i in 1..=n {
                #[allow(clippy::cast_precision_loss)]
                let mut x = (core::f64::consts::PI * (i as f64 - 0.25) / (nf + 0.5)).cos();
                let mut dp = 0.0;
                for _ in 0..100 {
                    let (mut p0, mut p1) = (1.0, x);
                    for k in 2..=n {
                        #[allow(clippy::cast_precision_loss)]
                        let kf = k as f64;
                        let p2 = ((2.0 * kf - 1.0) * x * p1 - (kf - 1.0) * p0) / kf;
                        p0 = p1;
                        p1 = p2;
                    }
                    dp = nf * (x * p1 - p0) / (x * x - 1.0);
                    let dx = p1 / dp;
                    x -= dx;
                    if dx.abs() < 1e-16 {
                        break;
                    }
                }
                let w = 2.0 / ((1.0 - x * x) * dp * dp);
                out.push((0.5 * (x + 1.0), 0.5 * w));
            }
            out
        }

        /// The convex polygon `poly` clipped to `[u0, u1] × [v0, v1]`.
        fn clip(poly: &[(f64, f64)], (u0, u1, v0, v1): (f64, f64, f64, f64)) -> Vec<(f64, f64)> {
            let mut out = poly.to_vec();
            let edges: [(usize, f64, bool); 4] =
                [(0, u0, true), (0, u1, false), (1, v0, true), (1, v1, false)];
            for (axis, at, keep_above) in edges {
                let coord = |p: (f64, f64)| if axis == 0 { p.0 } else { p.1 };
                let inside = |p: (f64, f64)| {
                    if keep_above {
                        coord(p) >= at
                    } else {
                        coord(p) <= at
                    }
                };
                let input = core::mem::take(&mut out);
                for (k, &a) in input.iter().enumerate() {
                    let b = input[(k + 1) % input.len()];
                    if inside(a) {
                        out.push(a);
                    }
                    if inside(a) != inside(b) {
                        let t = (at - coord(a)) / (coord(b) - coord(a));
                        out.push((a.0 + t * (b.0 - a.0), a.1 + t * (b.1 - a.1)));
                    }
                }
                if out.is_empty() {
                    break;
                }
            }
            out
        }

        /// `∫∫ f` over the convex, counter-clockwise polygon `poly`.
        pub(super) fn flux(chart: &Chart, poly: &[(f64, f64)]) -> f64 {
            let rule = gauss(14);
            let cells = |k: &[f64]| -> Vec<(f64, f64)> {
                k.windows(2)
                    .filter(|w| w[1] > w[0])
                    .map(|w| (w[0], w[1]))
                    .collect()
            };
            let mut total = 0.0;
            for (u0, u1) in cells(&chart.ku) {
                for (v0, v1) in cells(&chart.kv) {
                    let piece = clip(poly, (u0, u1, v0, v1));
                    for k in 1..piece.len().saturating_sub(1) {
                        let (a, b, c) = (piece[0], piece[k], piece[k + 1]);
                        // `x = a + s(b − a) + s·t(c − b)`, Jacobian `s·det`.
                        let det = (b.0 - a.0) * (c.1 - b.1) - (b.1 - a.1) * (c.0 - b.0);
                        for &(s, ws) in &rule {
                            for &(t, wt) in &rule {
                                let x = (
                                    a.0 + s * (b.0 - a.0) + s * t * (c.0 - b.0),
                                    a.1 + s * (b.1 - a.1) + s * t * (c.1 - b.1),
                                );
                                total += ws * wt * s * det * chart.f(x.0, x.1);
                            }
                        }
                    }
                }
            }
            total
        }
    }

    /// A clamped knot vector of degree `p` on `[0, 1]` with `interior`.
    fn clamped(p: usize, interior: &[f64]) -> Vec<f64> {
        let mut k = vec![0.0; p + 1];
        k.extend_from_slice(interior);
        k.extend(std::iter::repeat_n(1.0, p + 1));
        k
    }

    /// A chart of bidegree `(pu, pv)` with interior knots at `¼, ½, ¾` in
    /// `u` and `⅛, ⅜, ½, ⅞` in `v`: the polynomial surface
    /// `(u + ½u², v, 24uv + ½)`, its net read off its blossoms, plus a
    /// height of `¼·((3i + 5j + ij) mod 7 − 2)/8` per control point, so
    /// a different polynomial on each cell. The polynomial part is what
    /// makes `S_uu×S_uv` and `f_uu` below the chord large against the
    /// remainder's own width, so every term of `φ''` shows in the
    /// enclosure by round 3.
    fn piecewise_chart(pu: usize, pv: usize) -> reference::Chart {
        let ku = clamped(pu, &[0.25, 0.5, 0.75]);
        let kv = clamped(pv, &[0.125, 0.375, 0.5, 0.875]);
        let (nu, nv) = (ku.len() - pu - 1, kv.len() - pv - 1);
        // The blossom of `t` and of `t²` at a control point's knots.
        #[allow(clippy::cast_precision_loss)]
        let linear =
            |k: &[f64], p: usize, i: usize| k[i + 1..=i + p].iter().sum::<f64>() / p as f64;
        let square = |k: &[f64], p: usize, i: usize| {
            let args = &k[i + 1..=i + p];
            let mut pairs = 0.0;
            for (n, a) in args.iter().enumerate() {
                for b in &args[n + 1..] {
                    pairs += a * b;
                }
            }
            #[allow(clippy::cast_precision_loss)]
            let count = (p * (p - 1) / 2) as f64;
            pairs / count
        };
        let mut control = Vec::new();
        for i in 0..nu {
            for j in 0..nv {
                let (gu, gv) = (linear(&ku, pu, i), linear(&kv, pv, j));
                let z =
                    (f64::from(u32::try_from((3 * i + 5 * j + i * j) % 7).unwrap()) - 2.0) * 0.125;
                control.push([
                    gu + 0.5 * square(&ku, pu, i),
                    gv,
                    24.0 * gu * gv + 0.5 + 0.25 * z,
                ]);
            }
        }
        reference::Chart {
            ku,
            kv,
            pu,
            pv,
            control,
        }
    }

    /// **Q16 — the composite arm holds an independent reference where
    /// its chords cross knots of both directions at varied slopes.**
    ///
    /// Bidegrees `(3, 3)` and `(4, 3)`, interior knots in both
    /// directions ([`piecewise_chart`]), and two convex regions whose
    /// chords have slopes far from `0` and `±1` and cross `v`-knots away
    /// from the `u`-knots: a quadrilateral, and a steep triangle
    /// (`ℓ′ ≈ ±4.5`). Each runs as iso chords and with its closing edge
    /// a `General` image, rounds 0–3, against [`reference::flux`]. The
    /// reference is exact to the rounding of a 14-point rule on each
    /// knot cell's polynomial; the enclosures are far wider.
    #[test]
    fn q16_the_composite_arm_holds_an_independent_reference_across_knots() {
        let regions: [(&str, Vec<(f64, f64)>); 2] = [
            (
                "quadrilateral",
                vec![(0.05, 0.03), (0.93, 0.2), (0.8, 0.97), (0.1, 0.7)],
            ),
            ("steep triangle", vec![(0.2, 0.05), (0.6, 0.1), (0.4, 0.98)]),
        ];
        for (pu, pv) in [(3usize, 3usize), (4, 3)] {
            let chart = piecewise_chart(pu, pv);
            let ku = KnotVector::clamped(chart.ku.clone(), pu).unwrap();
            let kvv = KnotVector::clamped(chart.kv.clone(), pv).unwrap();
            let net = chart.net();
            let w = vec![1.0; net.len()];
            for (name, poly) in &regions {
                let truth = reference::flux(&chart, poly);
                let n = poly.len();
                for closing in ["iso", "General"] {
                    let mut chords: Vec<TrimChord> =
                        (0..n).map(|k| iso(poly[k], poly[(k + 1) % n])).collect();
                    if closing == "General" {
                        chords[n - 1] = general(&[poly[n - 1], poly[0]], 0.0);
                    }
                    let mut rounds = Vec::new();
                    let run = trimmed_patch_face_rounds::<f64>(
                        &patch(&ku, &kvv, &net, &w),
                        &chords,
                        Band::linear(Tol::witness()).unwrap(),
                        RoundWindow { first: 0, last: 3 },
                        &mut |b, _| {
                            rounds.push(b.flux);
                            Ok(false)
                        },
                    );
                    let label = format!("Q16 ({pu},{pv}) {name}, {closing} closing chord");
                    run.unwrap_or_else(|e| panic!("{label}: the composite arm answers: {e:?}"));
                    assert_eq!(rounds.len(), 4, "{label}: rounds 0–3 ran");
                    for (r, flux) in rounds.iter().enumerate() {
                        encloses(*flux, truth, &format!("{label}, round {r}"));
                    }
                }
            }
        }
    }

    /// **Q17 — the closure gap is paid over the column `G` integrates,
    /// not over the trim box.**
    ///
    /// Q11's in-bracket gap, on a chart whose integrand vanishes on the
    /// trim box and is large below it: `S = (u, v, z(v))` with `z` the
    /// quadratic spline on knots `(0, ½, 1)` and heights `(400, 0, 0, 0)`,
    /// so `f = z − v·z′` is `0` on the upper span, where the triangle
    /// `(0.1, 0.6) → (0.9, 0.6) → (0.9, 0.9)` lies, and the flux it
    /// bounds is `0`. The walk's closing chord starts `0.375·d` along `u`
    /// from where the rim ends, so Green's sum differs from `0` by that
    /// gap times `|G_f| = |∫₀^{0.9} f| ≈ 133`, about `5e-4` at
    /// `d = 10⁻⁵`. A pad levered by the trim box's `sup|f| = 0` misses
    /// `0` (measured: reverting the pad to the trim box reds this row).
    #[test]
    fn q17_the_closure_gap_is_paid_over_the_column_below_the_trim() {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let kvv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let mut control = Vec::new();
        for u in [0.0, 1.0] {
            for (y, z) in [(0.0, 400.0), (0.25, 0.0), (0.75, 0.0), (1.0, 0.0)] {
                control.push([pt(u), pt(y), pt(z)]);
            }
        }
        let w = vec![1.0; 8];
        let d = 1.0e-5;
        let mut rim = iso((0.9, 0.6), (0.9, 0.9));
        rim.b = (Interval::from_bounds(0.9 - d, 0.9), pt(0.9));
        let mut g = general(&[(0.9 - d, 0.9), (0.1, 0.6)], 0.0);
        g.a = (
            Interval::from_bounds(0.9 - 1.5 * d, 0.9 - 0.25 * d),
            pt(0.9),
        );
        g.piece.as_mut().unwrap().control[0] = g.a;
        let chords = vec![iso((0.1, 0.6), (0.9, 0.6)), rim, g];
        let run = trimmed(&patch(&ku, &kvv, &control, &w), &chords, RoundWindow::at(0))
            .unwrap_or_else(|e| panic!("Q17: an in-bracket gap is paid, not refused: {e:?}"));
        encloses(run.bounds.flux, 0.0, "Q17 flux");
    }

    /// **Q13 — an in-band monotone verdict ESCALATES.**
    ///
    /// Bisection halves the very span `props_trim_piece_monotone`
    /// meters, so a margin that sits in the ambiguity band arrives at
    /// the depth limit still in it: refining it is a four-level detour
    /// to a foregone refusal, and the refusal would misreport an
    /// undecided margin as a decided fold. The funnel's own answer for
    /// an in-band margin is an escalation, which is what every other
    /// quadrature lane returns.
    ///
    /// The fixture puts the first Bézier block's backward step inside
    /// `(ε, K·ε)`: the image is split into `IMAGE_INIT_PIECES = 8`, so a
    /// control step of `40ε` on the whole image is `5ε` on the block.
    /// Adopted from the t2q-r2 reviewer probe P2b.
    #[test]
    fn q13_an_in_band_monotone_verdict_escalates() {
        let eps = Tol::witness().get().eps;
        let (ku, kvv, control, w) = flat_chart(1.0);
        let net = patch(&ku, &kvv, &control, &w);
        let chords = vec![
            iso((0.0, 0.0), (1.0, 0.0)),
            iso((1.0, 0.0), (1.0, 1.0)),
            iso((1.0, 1.0), (0.0, 1.0)),
            general(&[(0.0, 1.0), (0.0, 1.0 + 40.0 * eps), (0.0, 0.0)], 0.0),
        ];
        match trimmed(&net, &chords, RoundWindow::SCHEDULE) {
            Err(PropsError::Escalated { cause, .. }) => {
                assert!(
                    cause
                        .predicate
                        .is_some_and(|p| p == "props_trim_piece_monotone"),
                    "Q13: the escalation names the row that could not decide: {cause:?}"
                );
            }
            other => panic!(
                "Q13: an in-band monotone margin is the funnel's to report, not a refusal \
                 to mint: {other:?}"
            ),
        }
    }

    /// **Q12 — a chart knot an ulp from the chord's end is still a
    /// cut.**
    ///
    /// `chord_cuts` used to apply [`knot_aligned_cuts`]' SLIVER guard
    /// to its own MANDATORY set, which is the opposite of the rule that
    /// function states in capitals: a dropped knot cut leaves the
    /// sub-chord straddling a knot by the guard's own width, the
    /// Newton–Cotes exactness argument fails on it, and no term pays.
    /// The guard's width is the same order as the outward rounding the
    /// lane calls its whole enclosure.
    ///
    /// The fixture puts the chart's `u` knot four ulps of the chord's
    /// span above the chord's start, inside the old `8·ulp` guard, and
    /// kinks the net across it so the two spans carry different
    /// polynomials.
    #[test]
    fn q12_a_knot_inside_the_old_sliver_guard_is_still_cut() {
        let c = 3.0;
        let uk = 4.0 * f64::EPSILON;
        let ku = KnotVector::clamped(vec![0.0, 0.0, uk, 1.0, 1.0], 1).unwrap();
        let kvv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        // x(u) jumps rate at `uk`: 0 → 0.5 over [0, uk], then 0.5 → 1.
        let mut control = Vec::new();
        for x in [0.0, 0.5, 1.0] {
            for v in [0.0, 1.0] {
                control.push([pt(x), pt(v), pt(c)]);
            }
        }
        let w = vec![1.0; 6];
        let chords = vec![
            iso((0.0, 0.0), (1.0, 0.0)),
            iso((1.0, 0.0), (1.0, 1.0)),
            general(&[(1.0, 1.0), (0.0, 0.0)], 0.0),
        ];
        // `f = c·x′(u)` and `g = |x′|`; over the triangle
        // `{0 < v < u < 1}`, `∫∫ = ∫₀¹ x′(u)·u du`.
        let g_int = (0.5 / uk) * 0.5 * uk.powi(2) + (0.5 / (1.0 - uk)) * 0.5 * (1.0 - uk.powi(2));
        let b = bounds_of(
            &trimmed(
                &patch(&ku, &kvv, &control, &w),
                &chords,
                RoundWindow::SCHEDULE,
            )
            .unwrap_or_else(|e| panic!("Q12: {e:?}")),
        );
        encloses(b.flux, c * g_int, "Q12 flux");
        encloses(b.area, g_int, "Q12 area");
    }

    /// `bezier_blocks`' uniform breaks are the DOMAIN's `m`ths, dropped
    /// within `GRID_CLEARANCE` of the spacing of a knot: a knot one ulp
    /// above `1/4` suppresses the break at `1/4` (four blocks, not five),
    /// while `1/2` and `3/4` stand. The block COUNT carries the pin:
    /// the starts' brackets (~1e-15 wide, the λ-rounding pad) cannot
    /// tell `near` from `1/4`, so they only confirm the other breaks
    /// sit where the grid puts them.
    #[test]
    fn bezier_blocks_drops_a_uniform_break_within_the_clearance_of_a_knot() {
        let near = f64::from_bits(0.25f64.to_bits() + 1);
        let kv = KnotVector::clamped(vec![0.0, 0.0, near, 1.0, 1.0], 1).unwrap();
        let img = TrimPiece::new(
            kv,
            [0.0, near, 1.0].iter().map(|t| (pt(*t), pt(0.0))).collect(),
            vec![1.0; 3],
        )
        .unwrap();
        let blocks = bezier_blocks(&img, 4).unwrap();
        assert_eq!(blocks.len(), 4, "{blocks:?}");
        for (block, start) in blocks.iter().zip([0.0, near, 0.5, 0.75]) {
            let u = block[0].0;
            assert!(u.lo() <= start && start <= u.hi(), "{u:?} vs {start}");
            assert!(u.hi() - u.lo() < 1e-12, "{u:?}");
        }
    }

    /// `bezier_blocks` holds its constant-weight invariant itself: an
    /// image whose weights vary refuses as `Rational`, whoever calls it,
    /// and one whose weights are one constant (`2`, `3`, `0.1`) is the
    /// unit image, block for block. A weight list or a control polygon
    /// of the wrong length never becomes an image at all.
    #[test]
    fn bezier_blocks_refuses_a_rational_image_by_name() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let control: Vec<RPt2> = [0.0, 0.5, 1.0].iter().map(|t| (pt(*t), pt(0.0))).collect();
        let img = |weights: Vec<f64>| TrimPiece::new(kv.clone(), control.clone(), weights);
        assert_eq!(
            bezier_blocks(&img(vec![1.0, 0.5, 1.0]).unwrap(), 4).err(),
            Some(BlockRefusal::Rational)
        );
        assert!(bezier_blocks(&img(vec![1.0; 3]).unwrap(), 4).is_ok());
        assert!(img(vec![1.0; 2]).is_err(), "a short weight list");
        assert!(
            TrimPiece::new(kv.clone(), control[..2].to_vec(), vec![1.0; 2]).is_err(),
            "a short control polygon"
        );
        let bits = |blocks: &[Vec<RPt2>]| -> Vec<[u64; 4]> {
            blocks
                .iter()
                .flatten()
                .map(|q| [q.0.lo(), q.0.hi(), q.1.lo(), q.1.hi()].map(f64::to_bits))
                .collect()
        };
        let unit = bits(&bezier_blocks(&img(vec![1.0; 3]).unwrap(), 4).expect("unit"));
        for c in [2.0, 3.0, 0.1] {
            let blocks = bezier_blocks(&img(vec![c; 3]).unwrap(), 4).expect("a constant weight");
            assert_eq!(bits(&blocks), unit, "weights {c}: the unit image's blocks");
        }
    }

    /// `bezier_blocks`' breaks are the DISTINCT interior knots: a
    /// double knot at `1/2` on a degree-3 image is one break, raised
    /// by ONE inserted copy to multiplicity 3, and the quarters grid
    /// adds `1/4` and `3/4` — four blocks of `degree + 1` points. A
    /// break that reached the insertion loop twice would insert two
    /// copies and take `1/2` past the degree; the distinct knot read
    /// and the dedup each prevent that alone, so the row goes red only
    /// when both are gone.
    #[test]
    fn bezier_blocks_breaks_once_at_a_repeated_knot() {
        let kv =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        // The Greville abscissae: an identity image, so each block
        // starts at its break.
        let greville = [0.0, 1.0 / 6.0, 1.0 / 3.0, 2.0 / 3.0, 5.0 / 6.0, 1.0];
        let img = TrimPiece::new(
            kv,
            greville.iter().map(|t| (pt(*t), pt(0.0))).collect(),
            vec![1.0; 6],
        )
        .unwrap();
        let blocks = bezier_blocks(&img, 4).unwrap();
        assert_eq!(blocks.len(), 4, "{blocks:?}");
        for (block, start) in blocks.iter().zip([0.0, 0.25, 0.5, 0.75]) {
            assert_eq!(block.len(), 4, "{block:?}");
            let u = block[0].0;
            assert!(u.lo() <= start && start <= u.hi(), "{u:?} vs {start}");
        }
    }

    /// `[0, 0]` decorated `Trv`: `sqrt([−1, 0])`, a bracket whose ends
    /// are an exact point and which does not certify. A producer that
    /// read its bracket instead of minting through `certified` would
    /// see an ordinary point.
    fn trv_zero() -> Interval {
        let x = Interval::from_bounds(-1.0, 0.0).sqrt();
        assert!(
            !x.is_certified() && x.lo() == 0.0 && x.hi() == 0.0,
            "fixture: {x:?}"
        );
        x
    }

    /// **A Trv bracket driven through `piece_monotone` does not certify
    /// a monotone piece.** On the flat chart every window's derivative
    /// hull is the same exact constant, so the control row certifies the
    /// straight chord. With one `Trv` coordinate the block box refuses
    /// (`Certification::hull` refuses an uncertified operand, so a box
    /// over several points is certified or NaI, never `Trv`), no window
    /// is minted, and the rate is the refused one.
    #[test]
    fn a_trv_block_does_not_certify_a_piece_monotone() {
        let (ku, kvv, control, w) = flat_chart(3.0);
        let net = patch(&ku, &kvv, &control, &w);
        let s = PatchGrid::base(&net.positions());
        let (su, sv) = (s.deriv_u(), s.deriv_v());
        let band = Band::linear(Tol::witness()).unwrap();
        // The `Trv` rides on the LAST point, so the first step's advance
        // is certified and the row reads the rate, not the span.
        let block = |end: Interval| vec![(pt(0.0), pt(0.5)), (pt(0.5), pt(0.5)), (pt(1.0), end)];
        let control_row = piece_monotone::<f64>(
            &block(pt(0.5)),
            (0.0, 0.5),
            (1.0, 0.5),
            su.as_ref(),
            sv.as_ref(),
            band,
        );
        assert!(
            matches!(control_row, Ok(Sign::Positive)),
            "CONTROL: a straight chord on the flat chart is monotone: {control_row:?}"
        );
        let trv = trv_zero() + pt(0.5);
        let got = piece_monotone::<f64>(
            &block(trv),
            (0.0, 0.5),
            (1.0, 0.5),
            su.as_ref(),
            sv.as_ref(),
            band,
        );
        assert!(
            !matches!(got, Ok(Sign::Positive)),
            "a Trv block certified a monotone piece: {got:?}"
        );
    }

    /// **A Trv bracket driven through the trimmed lane does not
    /// converge.** One chord endpoint is a `Trv` `[0, 0]`, Q1's loop
    /// otherwise. The trim box over the endpoints is NaI (a hull refuses
    /// an uncertified operand), so it mints no window and the pads it
    /// bounds refuse; the lane answers no converged bound.
    #[test]
    fn a_trv_trim_endpoint_does_not_converge_the_trimmed_lane() {
        let c = 3.0;
        let (ku, kvv, control, w) = flat_chart(c);
        let net = patch(&ku, &kvv, &control, &w);
        let trv = trv_zero();
        let mut first = iso((0.0, 0.0), (1.0, 0.0));
        first.a = (trv, pt(0.0));
        let chords = vec![
            first,
            iso((1.0, 0.0), (1.0, 1.0)),
            general(&[(1.0, 1.0), (0.5, 0.5), (0.0, 0.0)], 1e-3),
        ];
        let got = trimmed(&net, &chords, RoundWindow::SCHEDULE);
        assert!(
            !matches!(
                got,
                Ok(LaneRun {
                    end: LaneEnd::Accepted(_),
                    ..
                })
            ),
            "a Trv trim endpoint converged: {got:?}"
        );
    }
}
