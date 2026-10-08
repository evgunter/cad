//! The signed winding of a planar boundary around a normal — the
//! `bool_ring_run_winding` predicate, stated once for the three `topo`
//! sites that ask it: the merge's role assigner (`merge_faces`'
//! `merged_outline_ring`), which ASSIGNS outer/ring roles by it over a
//! stored loop; tier 3's check 6 planar arm (`validate`), which
//! FALSIFIES a stored role and sense by it; and the boolean join's ring
//! lane (`boolean::join::ring_run_ccw`), which asks it of an OPEN run
//! closed by the chord the join will mint, to pick an island's new
//! outer boundary. One home
//! is what keeps them from answering about different carrier sets, by
//! different claims, or by different arithmetic.
//!
//! # Dimension (audit F4, `docs/predicate-dimension-audit.md`)
//!
//! The canonical statement for all three sites. The margin is the
//! plane's Newell functional — twice the enclosed signed area, an AREA
//! (m²) — and ε is a point deviation (D4), so the decided margin
//! divides it by the region's boundary PERIMETER `P`. `2A/P` is the
//! region's MEAN WIDTH — exactly the deviation the winding sign is
//! about: the distance the boundary would have to move to sweep the
//! enclosed region away. A margin above ε says "this boundary encloses
//! material no ε-scale point perturbation can unwind"; one below it
//! says the region is thinner than the model's own resolution.
//! Precedents: `validate`'s `positive_volume` (V/A) and the splitter's
//! `split_section_area` (2|A|/P, the same mean width).
//!
//! `P` is the closed region's own boundary: each half-edge contributes
//! its arc length (below), and an open run's closing curve is one more
//! edge of the region, read by the same per-edge terms.
//!
//! # The carriers this answers about
//!
//! Line, Circle and Ellipse, and a null-edge scaffold, which is its
//! zero-length chord. A boundary carrying a NURBS or spiric edge has no
//! winding here — the remainder this home has not built: a chord
//! winding says nothing about a fitted carrier's region. A closed form
//! exists for a NON-rational B-spline (`geom_brep`'s
//! `props::loop_vector_area` integrates it exactly per knot span); a
//! rational spline and the spiric have none.
//!
//! For the conic carriers the enclosed vector area decomposes EXACTLY,
//! per edge — a substitution, not an approximation:
//!
//! ```text
//!   2·A⃗ = Σ_edges       (p_prev − p₀) × (p − p₀)      [chord Newell]
//!        + Σ_conic-edges axis · sa·sb · (Δ − sin Δ)    [bulge]
//! ```
//!
//! A circular arc of radius `R` spanning signed angle `Δ` cuts off a
//! circular segment of area `R²(Δ − sin Δ)/2` between itself and its
//! chord; twice that is `R²(Δ − sin Δ)`, which is the cross-sum's own
//! `2A` convention, and it is ODD in `Δ` — so it carries the traversal
//! sign the winding question is about. The ellipse is the circle's
//! affine image, which scales every area by `major·minor/R²`, giving
//! `sa·sb`. The chord term is untouched for every edge, so the bulge is
//! a CORRECTION on a chord polygon and a mixed Line+Circle cycle needs
//! no case split beyond the per-edge carrier match.
//!
//! The perimeter lever moves with the area: a conic edge contributes
//! `|Δ|·max(|sa|, |sb|)` — the circle's exact arc length, the ellipse's
//! upper bound; an over-large `P` understates the width, i.e. escalates
//! rather than decides.
//!
//! A LINE-ONLY cycle is decided with exactly the chord arithmetic and
//! accumulation order: the correction block is structurally skipped,
//! not zero-added into a reordered sum.
//!
//! A cycle whose perimeter is exactly zero — every vertex coincident —
//! divides `0/0`, poisons, and escalates: a loop with no extent has no
//! winding to report.
//!
//! # Orientation (S10)
//!
//! The margin multiplies two differently-sourced signs and needs
//! exactly ONE of them threaded. `normal` must be the face's OUTWARD
//! normal: the caller folds the sense into the chart normal exactly
//! once, through [`crate::face_normal`]'s door, and the sum here is
//! left alone. The sum is built from the STORED traversal order, which
//! `revert` reverses in the same breath as it flips the sense bit, so
//! it changes sign on its own. Threading the sense onto both factors
//! would cancel (the classic double-count); threading neither would
//! make "counterclockwise about the outward normal" mean "about the
//! chart normal", the opposite statement on a reversed face.

use geom::Curve3;
use geom_core::{Decide, Decided, Indeterminate, Margin, Point3, Real, Sign, Vec3};

use crate::body::Body;
use crate::entity::{EntityId, HalfEdgeKey, LoopKey, VertexKey};
use crate::live::linked;

/// The winding decision's predicate name: the K-stats key its margin is
/// recorded under, and the name an escalation of it carries.
pub(crate) const WINDING_PREDICATE: &str = "bool_ring_run_winding";

/// [`Body::planar_run_winding_decided`]'s refusal: the run's walk from
/// its first half came back round to it without reaching its last, so
/// the two are not one cycle's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RunMissesEnd;

/// What [`Body::planar_loop_winding_decided`] reads of a loop, `W` the
/// winding when one is read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopWinding<W> {
    /// An empty loop: a lone vertex, which bounds no area.
    Empty,
    /// A cycle carrying a NURBS or spiric edge, which the kernel does
    /// not wind (module docs).
    Unsupported,
    /// The winding of a cycle the kernel reads.
    Wound(W),
}

/// **The conic term of one traversed edge** ([`ConicFrame`],
/// [`chord_bulge`]): `(axis · sa·sb · (Δ − sin Δ), |Δ|·max(|sa|, |sb|))`, the vector area between the
/// arc and its chord (the cross-sum's `2A` convention, odd in the
/// signed span `Δ`) and the edge's boundary length (exact for a
/// circle, an upper bound for an ellipse: `|Δ|` times the larger
/// semi-axis MAGNITUDE, whichever field stores it and with whatever
/// sign — an ellipse stored with `minor > major`, or with a negative
/// `major` and its `u_ref` flipped, certifies at mint, and this term is
/// read in flight (the merge's role assigner, the boolean join's
/// `ring_run_ccw`) before any at-rest check refuses it; `|Δ|·major`
/// would then be a lower bound, overstating the metered width). The
/// bulge needs no magnitude: `sa·sb` is the signed determinant of the
/// affine image, so a negative semi-axis mirrors the traversal and the
/// area's sign with it. A circle's lever is its `radius` as stored:
/// certification meters the span as `(t₁ − t₀)·r` and refuses one
/// that is not definitely positive, and a circle with `r < 0` over a
/// reversed interval is the one shape that passes that and reads a
/// negative lever here — check 1 refuses it at rest
/// (`UnrepresentableCurveDatum`). `forward` is whether the
/// traversal runs with increasing carrier parameter — the edge's plus
/// half; `(t0, t1)` the carrier interval the edge spans. `None` for
/// every carrier that is not a conic: its term is its chord's, which
/// the caller owns.
pub(crate) fn conic_segment_term<T: Real>(
    (carrier, (t0, t1)): (&Curve3<T>, (T, T)),
    forward: bool,
) -> Option<(Vec3<T>, T)> {
    let conic = ConicFrame::of(carrier)?;
    let span = if forward { t1 - t0 } else { t0 - t1 };
    Some((
        conic.axis * (conic.sa * conic.sb * chord_bulge(span)),
        span.abs() * conic.reach,
    ))
}

/// **A conic carrier's frame** — the one reading of a circle or an
/// ellipse as `P(θ) = center + a·cos θ + b·sin θ`, `a = u_ref·sa`,
/// `b = (axis × u_ref)·sb`: a circle's `sa = sb = radius`, an
/// ellipse's `(major, minor)`, as stored, signs included.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ConicFrame<T: Real> {
    /// The conic's centre.
    pub(crate) center: Point3<T>,
    /// The unit normal of its plane.
    pub(crate) axis: Vec3<T>,
    /// The `θ = 0` direction.
    pub(crate) u_ref: Vec3<T>,
    /// The semi-axis along `u_ref`.
    pub(crate) sa: T,
    /// The semi-axis along `axis × u_ref`.
    pub(crate) sb: T,
    /// The arc-length lever per radian ([`conic_segment_term`]): a
    /// circle's radius itself, an ellipse's larger semi-axis magnitude.
    pub(crate) reach: T,
}

impl<T: Real> ConicFrame<T> {
    /// The frame of `carrier`; `None` for a line, a spiric or a NURBS.
    pub(crate) fn of(carrier: &geom::Curve3<T>) -> Option<Self> {
        // The circle's lever is its radius itself, not `radius.max(radius)`:
        // the same value, but at a symbolic scalar a `max` node is opaque
        // where the radius is not.
        let (center, axis, u_ref, sa, sb, reach) = match *carrier {
            geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => (center, axis, u_ref, radius, radius, radius),
            geom::Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } => (
                center,
                axis,
                u_ref,
                major,
                minor,
                major.abs().max(minor.abs()),
            ),
            geom::Curve3::Line { .. } | geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => {
                return None;
            }
        };
        Some(Self {
            center,
            axis,
            u_ref,
            sa,
            sb,
            reach,
        })
    }

    /// The `θ = 0` semi-axis vector, `u_ref·sa`.
    pub(crate) fn a(&self) -> Vec3<T> {
        self.u_ref * self.sa
    }

    /// The `θ = π/2` semi-axis vector, `(axis × u_ref)·sb`.
    pub(crate) fn b(&self) -> Vec3<T> {
        self.axis.cross(self.u_ref) * self.sb
    }
}

/// **The bulge of an arc over its chord** — the one statement of it:
/// `Δ − sin Δ` for the signed span `Δ`. Twice the area between a unit
/// circle's arc and its chord; an arc of the conic `c + a·cos θ +
/// b·sin θ` cuts off `(a × b)·(Δ − sin Δ)` of twice-area, for any
/// signed `Δ` and any `a`, `b` (module docs).
pub(crate) fn chord_bulge<T: Real>(span: T) -> T {
    span - span.sin()
}

/// A curve as one traversal reads it: its carrier and the interval it
/// spans, and whether the traversal runs with the carrier's parameter.
type Traversed<'a, T> = ((&'a Curve3<T>, (T, T)), bool);

/// The curve that closes an open run from the run's end back to its
/// start: the straight chord, or a curve the caller has (the boolean
/// join's ring lane hands over the chord it will mint). A closing conic
/// is one more edge of the region: its bulge joins the area and its arc
/// length the lever, by [`conic_segment_term`] as every run edge's do.
#[derive(Debug, Clone, Copy)]
pub(crate) enum RunClosing<'a, T: Real> {
    /// The straight chord.
    Straight,
    /// The carrier and its interval, already running from the run's end
    /// to its start with increasing parameter.
    Curve(&'a Curve3<T>, (T, T)),
}

impl<'a, T: Real> RunClosing<'a, T> {
    /// The closing a chord spec states: `None` is the straight chord.
    pub(crate) fn of(spec: Option<&'a geom_brep::EdgeCurveSpec<T>>) -> Self {
        match spec {
            None => RunClosing::Straight,
            Some(s) => RunClosing::Curve(&s.carrier, (s.param_start, s.param_end)),
        }
    }
}

/// How a winding's traversed halves close into the region it is read
/// of.
#[derive(Debug, Clone, Copy)]
enum Closing<'a, T: Real> {
    /// A stored cycle: the last half ends where the first starts.
    Cycle,
    /// An open run, closed from the last half's end back to the first
    /// half's start.
    Run(RunClosing<'a, T>),
}

/// What the sum reads of one traversed half: its start point, and its
/// certified curve as traversed (`None` for a null-edge scaffold, which
/// is its chord).
type Step<'a, T> = (Point3<T>, Option<Traversed<'a, T>>);

impl<T: Decide> Body<T> {
    /// The signed winding of cycle loop `l` around `normal` (module
    /// docs): `Positive` is counterclockwise about the normal
    /// (interior-left), `Negative` clockwise, `Zero` no signed area,
    /// `Err` an in-band margin.
    ///
    /// `None` — no predicate is asked — for an empty loop (a
    /// lone-vertex ring bounds no area) and for a cycle carrying a
    /// NURBS or spiric edge (the honest remainder). There is no
    /// narrower reach to ask for: the assigner, the checker and the
    /// join's ring lane answer on the one carrier set, which is what
    /// keeps them from disagreeing about which loops have a winding.
    pub(crate) fn planar_loop_winding(
        &self,
        l: LoopKey,
        normal: Vec3<T>,
        band: geom_core::Band,
    ) -> Option<Result<Sign, Indeterminate>> {
        match self.planar_loop_winding_decided(l, normal, band) {
            LoopWinding::Empty | LoopWinding::Unsupported => None,
            LoopWinding::Wound(w) => Some(w.map(|d| d.sign)),
        }
    }

    /// [`Body::planar_loop_winding`], keeping the margin the sign was
    /// decided on (a caller that refuses a zero winding quotes it,
    /// [`geom_core::k_stats::decide_reported`]) and telling an empty
    /// loop from a cycle it does not wind.
    ///
    /// # Panics
    ///
    /// Where `l`, a key every caller read out of the body, or a record
    /// the walk reads from it does not resolve, the cycle does not close,
    /// or a member's edge does not claim it (D2 row 4).
    #[track_caller]
    pub(crate) fn planar_loop_winding_decided(
        &self,
        l: LoopKey,
        normal: Vec3<T>,
        band: geom_core::Band,
    ) -> LoopWinding<Result<Decided, Indeterminate>> {
        let crate::entity::LoopBoundary::Cycle { first } =
            crate::live::proven(&self.loops, l, EntityId::Loop).boundary
        else {
            return LoopWinding::Empty;
        };
        let walk = self.winding_walk(first, EntityId::Loop(l), "first", |_, next| next == first);
        let Some(cycle) = walk else {
            unreachable!("a cycle walk ends where it closes")
        };
        match self.winding_of_halves(&cycle, Closing::Cycle, normal, band) {
            Some(w) => LoopWinding::Wound(w),
            None => LoopWinding::Unsupported,
        }
    }

    /// The signed winding around `normal` of the open run `h1 → h2` —
    /// the `next`-order arc from `h1` through `h2` — closed by `closing`
    /// from `h2`'s end back to `h1`'s start: the region the boolean
    /// join's ring lane walls off as an island's new face
    /// (`boolean::join::ring_run_ccw`). The sum, the carrier set and the
    /// claim are [`Body::planar_loop_winding_decided`]'s; the closing
    /// curve is the one thing a run adds. `Ok(None)` for a run or a
    /// closing curve carrying a NURBS or spiric carrier; [`RunMissesEnd`]
    /// when `next` comes back to `h1` without reaching `h2`.
    ///
    /// # Panics
    ///
    /// As [`Body::planar_loop_winding_decided`], `h1` being a key the
    /// caller read out of the body.
    #[track_caller]
    pub(crate) fn planar_run_winding_decided(
        &self,
        (h1, h2): (HalfEdgeKey, HalfEdgeKey),
        closing: RunClosing<'_, T>,
        normal: Vec3<T>,
        band: geom_core::Band,
    ) -> Result<Option<Result<Decided, Indeterminate>>, RunMissesEnd> {
        crate::live::proven(&self.half_edges, h1, EntityId::HalfEdge);
        let run = self
            .winding_walk(h1, EntityId::HalfEdge(h1), "next", |he, _| he == h2)
            .ok_or(RunMissesEnd)?;
        Ok(self.winding_of_halves(&run, Closing::Run(closing), normal, band))
    }

    /// The halves from `first` (`holder`'s field `field`) in `next`
    /// order through the first `he` with `last(he, next)`, each a link of
    /// the one before. `None` when the walk comes back to `first` first.
    ///
    /// # Panics
    ///
    /// Where a link does not resolve, or the walk does not come back to
    /// `first` within the arena's length (D2 row 4).
    #[track_caller]
    fn winding_walk(
        &self,
        first: HalfEdgeKey,
        holder: EntityId,
        field: &'static str,
        last: impl Fn(HalfEdgeKey, HalfEdgeKey) -> bool,
    ) -> Option<Vec<HalfEdgeKey>> {
        let cap = self.half_edges.len();
        let mut walk = Vec::new();
        let (mut he, mut holder, mut field) = (first, holder, field);
        loop {
            let next = linked(&self.half_edges, he, EntityId::HalfEdge, holder, field).next;
            walk.push(he);
            if last(he, next) {
                return Some(walk);
            }
            if next == first {
                return None;
            }
            assert!(
                walk.len() < cap,
                "the next walk from {} does not come back to it within the arena's length: {}",
                EntityId::HalfEdge(first),
                crate::body::WALKS_CLOSE
            );
            (he, holder, field) = (next, EntityId::HalfEdge(he), "next");
        }
    }

    /// The point of vertex `v`, `holder`'s field `field`.
    #[track_caller]
    fn winding_point(&self, v: VertexKey, holder: EntityId, field: &'static str) -> Point3<T> {
        self.linked_vertex_point(v, holder, field)
    }

    /// **The winding sum** (module docs) over `halves` in order, closed
    /// as `closing` says — the one statement of it. `None` when a
    /// half carries a NURBS or spiric edge.
    ///
    /// A null-edge scaffold is wound as its chord: it states no carrier,
    /// and `Body::mev_null` mints its far vertex on a copy of its near
    /// vertex's point, so the chord is the whole of the zero-length edge
    /// (the chord joiner reads it the same way — zero-length, ON). The
    /// boolean join's ring run starts and ends on one.
    fn winding_of_halves(
        &self,
        halves: &[HalfEdgeKey],
        closing: Closing<'_, T>,
        normal: Vec3<T>,
        band: geom_core::Band,
    ) -> Option<Result<Decided, Indeterminate>> {
        let mut walked: Vec<Step<'_, T>> = Vec::with_capacity(halves.len());
        // Whether some carrier is a conic: the one fact about the
        // carrier set the sum reads (the correction block below).
        let mut any_conic = false;
        let conic_kind = |carrier: &Curve3<T>| match carrier {
            geom::Curve3::Line { .. } => Some(false),
            geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } => Some(true),
            geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => None,
        };
        let closing_curve = match closing {
            Closing::Run(RunClosing::Curve(carrier, params)) => Some(((carrier, params), true)),
            Closing::Cycle | Closing::Run(RunClosing::Straight) => None,
        };
        if let Some(((carrier, _), _)) = closing_curve {
            let conic = conic_kind(carrier)?;
            any_conic |= conic;
        }
        for &he in halves {
            let hd = crate::live::proven(&self.half_edges, he, EntityId::HalfEdge);
            let edge = linked(
                &self.edges,
                hd.edge,
                EntityId::Edge,
                EntityId::HalfEdge(he),
                "edge",
            );
            let claim = edge.claim(he).unwrap_or_else(|| {
                unreachable!(
                    "{}'s edge {} does not claim it in either slot: on a tier-1-valid body \
                     an edge claims the two half-edges that name it",
                    EntityId::HalfEdge(he),
                    EntityId::Edge(hd.edge)
                )
            });
            let curve = self.edge_curve_linked(hd.edge, edge).certified();
            if let Some(curve) = curve {
                let conic = conic_kind(curve.carrier())?;
                any_conic |= conic;
            }
            walked.push((
                self.winding_point(hd.start, EntityId::HalfEdge(he), "start"),
                curve.map(|c| ((c.carrier(), c.params()), claim.plus)),
            ));
        }
        let (Some(&(p0, _)), Some(&last)) = (walked.first(), halves.last()) else {
            unreachable!("a walk holds its first half")
        };
        // Where the last half ends: the first half's start on a cycle,
        // the run's own end on an open run.
        let end = match closing {
            Closing::Cycle => p0,
            Closing::Run(_) => {
                let next = crate::live::proven(&self.half_edges, last, EntityId::HalfEdge).next;
                let end = linked(
                    &self.half_edges,
                    next,
                    EntityId::HalfEdge,
                    EntityId::HalfEdge(last),
                    "next",
                );
                self.winding_point(end.start, EntityId::HalfEdge(next), "start")
            }
        };
        let conic = |step: &Step<'_, T>| step.1.and_then(|(c, fwd)| conic_segment_term(c, fwd));
        let open = matches!(closing, Closing::Run(_));
        let mut newell = Vec3::new(T::zero(), T::zero(), T::zero());
        // The F4 metering lever: the boundary's own length, accumulated
        // with the area — a chord per straight edge, an arc length per
        // conic, the closing curve included.
        let mut perimeter = T::zero();
        for (i, step) in walked.iter().enumerate() {
            let p = step.0;
            let next = walked.get(i + 1).map_or(end, |w| w.0);
            if i + 1 < walked.len() || open {
                newell = newell + (p - p0).cross(next - p0);
            }
            perimeter = perimeter
                + match conic(step) {
                    Some((_, len)) => len,
                    None => (next - p).norm(),
                };
        }
        let closing_term = closing_curve.and_then(|(c, fwd)| conic_segment_term(c, fwd));
        if open {
            perimeter = perimeter
                + match closing_term {
                    Some((_, len)) => len,
                    None => (end - p0).norm(),
                };
        }
        // The conic correction (module docs), added only when some
        // carrier is a conic: a line-only cycle keeps the chord sum.
        // Summed on its own and added once, so the chord sum is never
        // re-associated.
        if any_conic {
            let mut bulge = Vec3::new(T::zero(), T::zero(), T::zero());
            for (b, _) in walked.iter().filter_map(conic).chain(closing_term) {
                bulge = bulge + b;
            }
            newell = newell + bulge;
        }
        Some(crate::validate::decide_reported(
            WINDING_PREDICATE,
            Margin::over_lever(normal.dot(newell), perimeter),
            band,
        ))
    }
}
