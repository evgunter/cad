//! Joining of null edges (ch. 14 §14.7, `splitconnect`): pair loose
//! null-edge halves into section-polygon chains, connect them with
//! real on-plane edges, and close each chain into a 2-loop **null
//! face** — recorded as F9 data
//! ([`NullFacePair::Split`](crate::null::NullFacePair)) with the
//! above/below roles determined by **vertex-key membership**, never by
//! `floops` list position or he1/he2 slot convention.
//!
//! # The sweep (Programs 14.9/14.10, re-derived)
//!
//! Null edges are processed in the total lexicographic order of their
//! (coincident-copy) points ([`super::order`] — the ε-banded book sort
//! engineered out). Each edge offers its two halves in a fixed data
//! order (**up half first** — the half starting at `below_end`; the
//! book's "he1 first" as data, not slot). A half either consumes a
//! **loose end** — a registered half in the *same face* with the
//! *opposite* up/down sense — or becomes one (growable typed
//! collection; the book's `ends[30]` is gone).
//!
//! `join(h1 = old end, h2 = new half)` connects the two null edges
//! with up to two real chord edges (head↔head and tail↔tail — each
//! connecting edge's endpoints lie on ONE side, pinned by test):
//!
//! - same loop ⇒ `mef(Chords { he1: h1, he2: next(h2) })` (the book's
//!   `lmef(h1, h2->nxt)`; our [`MefSite::Chords`] documents the same
//!   run association, so the argument pair ports literally — and the
//!   mirror test pins the outcome, not the citation), guarded by
//!   `prev(prev(h1)) != h2` (adjacent ⇒ the chord already exists);
//! - different loops ⇒ `mekr` with the **ring chosen structurally**
//!   (the loop that is not the face's outer; the book's fixed
//!   `lmekr(h1, h2->nxt)` argument order assumes GWB's list layout —
//!   ours is explicit outer/ring data);
//! - then the second chord `mef(Chords { he1: h2, he2: next(h1) })`
//!   guarded by `next(next(h1)) != h2`; if the first `mef` split a
//!   face that still owns rings, the rings are re-homed by trilean
//!   containment ([`super::containment`]) + [`Body::ring_move`] — the
//!   `laringmv` step (lkemr/ring-placement mirror site).
//!
//! `cut(edge)` retires a fully-joined null edge: halves in different
//! loops ⇒ `kef` (merge the two sliver faces — the killed side must be
//! a join-minted sliver, asserted); same loop ⇒ the polygon is
//! COMPLETE: `kemr` leaves the 2-loop null face, roles resolved by
//! membership of the minted above-copy vertex set, and the polygon's
//! area is certified definitely-positive (**`split_section_area`**,
//! margin 2·A/P) — a zero-area section polygon is the one-sided
//! tangency residue PR 2's adjudication record promised to refuse
//! here, typed [`SplitJoinError::DegenerateSection`].

use geom_core::{Band, Decide, Indeterminate, Point3, Sign};
use slotmap::SecondaryMap;

use super::containment::{LoopContainment, PointInLoopError, point_in_loop};
use super::order;
use super::{SplitPlane, SplitReduction};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::euler::{EulerOpError, MefSite};
use crate::euler_ring::MekrSite;
use crate::null::NullFacePair;
use crate::validate::decide;

/// One completed section polygon: the null face and its role loops
/// (mirrors the body's [`NullFacePair`] record; carried separately so
/// the finish step consumes explicit keys in completion order).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompletedSection {
    /// The null face (two coincident loops).
    pub face: FaceKey,
    /// The loop through the minted above copies.
    pub above_loop: LoopKey,
    /// The loop through the original below-side vertices.
    pub below_loop: LoopKey,
}

/// Typed failure of the joining step.
#[derive(Debug)]
pub enum SplitJoinError {
    /// The exact-order comparator escalated (interval lane only).
    OrderEscalated {
        /// Diagnostics (named predicate inside).
        diag: Indeterminate,
    },
    /// The section-area or containment machinery escalated.
    Escalated {
        /// The site face (the null face or the ring's face).
        face: FaceKey,
        /// Diagnostics.
        diag: Indeterminate,
    },
    /// A completed section polygon bounds zero area — the zero-area
    /// residue (rule (b) adjudication record): no degenerate body is
    /// ever emitted.
    ///
    /// Per RUN this fires iff the pinched pieces lie on the NEGATIVE
    /// side of the run's plane normal (the below side, where the
    /// ch. 14 insertion mints no vertex copies). Since M3 PR 6a (D7)
    /// the public [`super::split`] consumes this refusal as the pinch
    /// trigger and reruns under the mirrored plane — where the
    /// pinched fans are ABOVE runs and mint their copies — so op
    /// success is orientation-independent; the error still surfaces
    /// from [`super::split`] when BOTH orientations refuse (a genuine
    /// both-sided zero-area residue) and from the join lane directly
    /// (e.g. [`super::plane_section`], which has no sides to swap).
    DegenerateSection {
        /// The completed null face.
        face: FaceKey,
    },
    /// Ring re-homing could not decide (escalation or exhaustion).
    RingHoming(PointInLoopError),
    /// A ring's representative landed ON the divided face's outer
    /// loop — containment is ambiguous (ill-conditioned operand).
    RingHomingAmbiguous {
        /// The undecidable ring.
        ring: LoopKey,
    },
    /// Loose ends survived the sweep — the null-edge set does not
    /// close into section polygons (kernel bug or corrupt reduction).
    UnpairedLooseEnds {
        /// How many halves remained.
        count: usize,
    },
    /// A section loop mixed above copies with below-side vertices —
    /// the joining invariant (heads join heads, tails join tails)
    /// failed (kernel bug, loudly).
    SectionLoopMixed {
        /// The offending null face.
        face: FaceKey,
    },
    /// `cut` found neither side of an interior null edge to be a
    /// join-minted sliver face (kernel bug, loudly).
    CutInvariant {
        /// The null edge being retired.
        edge: EdgeKey,
    },
    /// A traversal failed mid-join (corrupt body).
    Corrupt,
    /// An underlying Euler operation refused.
    Euler(EulerOpError),
}

impl From<EulerOpError> for SplitJoinError {
    fn from(e: EulerOpError) -> Self {
        Self::Euler(e)
    }
}

impl From<PointInLoopError> for SplitJoinError {
    fn from(e: PointInLoopError) -> Self {
        Self::RingHoming(e)
    }
}

impl core::fmt::Display for SplitJoinError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OrderEscalated { diag } => {
                write!(f, "split join: lexicographic order escalated: {diag}")
            }
            Self::Escalated { face, diag } => {
                write!(f, "split join: escalated at face {face:?}: {diag}")
            }
            Self::DegenerateSection { face } => write!(
                f,
                "split join: section polygon at {face:?} bounds zero area — one-sided \
                 tangency: the degenerate side has no real material (refused, never emitted)"
            ),
            Self::RingHoming(e) => write!(f, "split join: ring re-homing: {e}"),
            Self::RingHomingAmbiguous { ring } => write!(
                f,
                "split join: ring {ring:?} sits ON the divided face's outer loop — \
                 containment undecidable (ill-conditioned operand)"
            ),
            Self::UnpairedLooseEnds { count } => write!(
                f,
                "split join: {count} loose null-edge halves survived the sweep (kernel bug)"
            ),
            Self::SectionLoopMixed { face } => write!(
                f,
                "split join: null face {face:?} has a side-mixed section loop (kernel bug)"
            ),
            Self::CutInvariant { edge } => write!(
                f,
                "split join: neither face flanking null edge {edge:?} is a sliver (kernel bug)"
            ),
            Self::Corrupt => write!(f, "split join: traversal failed (corrupt body)"),
            Self::Euler(e) => write!(f, "split join: euler operation refused: {e}"),
        }
    }
}

impl std::error::Error for SplitJoinError {}

/// The joining sweep (module docs). Mutates `red.body` in place;
/// returns the completed section polygons in completion order, with
/// the F9 `NullFacePair::Split` records set on the body.
///
/// # Errors
///
/// [`SplitJoinError`] — the body may be left mid-surgery on `Err`
/// (callers operate on a scratch clone; the public ops discard it).
pub(super) fn split_connect<T: Decide>(
    red: &mut SplitReduction<T>,
    band: Band,
) -> Result<Vec<CompletedSection>, SplitJoinError> {
    let exact = order::exact_band().map_err(|_| SplitJoinError::Corrupt)?;

    // The minted above-copy set (role resolution is key membership).
    let mut above_set: SecondaryMap<VertexKey, ()> = SecondaryMap::new();
    for r in &red.null_edges {
        above_set.insert(r.attr.above_end, ());
    }

    // Sort points: each null edge's coincident-copy position.
    let mut points = Vec::with_capacity(red.null_edges.len());
    for r in &red.null_edges {
        points.push(vertex_point(&red.body, r.attr.below_end)?);
    }
    let sorted = order::sort_indices_by_point(&points, &red.plane, band, exact)
        .map_err(|diag| SplitJoinError::OrderEscalated { diag })?;

    let mut st = Sweep {
        ends: Vec::new(),
        joiner: ChordJoiner::new(band),
        completed: Vec::new(),
        above_set,
        plane: red.plane,
        band,
    };

    for idx in sorted {
        let record = red.null_edges[idx];
        let edge = red
            .body
            .get_edge(record.edge)
            .ok_or(SplitJoinError::Corrupt)?
            .clone();
        // Data order: the up half (starting at below_end) first — the
        // book's he1-first as data (module docs).
        let (up, down) = {
            let plus_start = red
                .body
                .get_half_edge(edge.he_plus)
                .ok_or(SplitJoinError::Corrupt)?
                .start;
            if plus_start == record.attr.below_end {
                (edge.he_plus, edge.he_minus)
            } else {
                (edge.he_minus, edge.he_plus)
            }
        };
        let mut joined = [false, false];
        for (slot, half) in [(0, up), (1, down)] {
            if let Some(end) = st.take_neighbor(&red.body, half)? {
                st.joiner.join(&mut red.body, end, half)?;
                joined[slot] = true;
                // Retire the consumed end's edge if its other half is
                // no longer loose.
                let end_edge = he_edge(&red.body, end)?;
                let mate = red.body.mate(end).ok_or(SplitJoinError::Corrupt)?;
                if !st.is_loose(mate) {
                    st.cut(&mut red.body, end_edge)?;
                }
            }
        }
        if joined[0] && joined[1] {
            st.cut(&mut red.body, record.edge)?;
        }
    }

    if !st.ends.is_empty() {
        return Err(SplitJoinError::UnpairedLooseEnds {
            count: st.ends.len(),
        });
    }
    Ok(st.completed)
}

/// The point of a vertex.
fn vertex_point<T: Decide>(body: &Body<T>, v: VertexKey) -> Result<Point3<T>, SplitJoinError> {
    let vertex = body.get_vertex(v).ok_or(SplitJoinError::Corrupt)?;
    body.get_point(vertex.point)
        .copied()
        .ok_or(SplitJoinError::Corrupt)
}

/// The edge of a half-edge.
fn he_edge<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Result<EdgeKey, SplitJoinError> {
    Ok(body.get_half_edge(he).ok_or(SplitJoinError::Corrupt)?.edge)
}

/// The face owning a half-edge's loop.
fn he_face<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Result<FaceKey, SplitJoinError> {
    let l = body
        .get_half_edge(he)
        .ok_or(SplitJoinError::Corrupt)?
        .parent_loop;
    Ok(body.get_loop(l).ok_or(SplitJoinError::Corrupt)?.face)
}

/// The outcome of retiring a fully-joined null edge (`cut`):
/// either the section polygon completed (a 2-loop null face remains,
/// roles unresolved — the caller resolves them from its own role data)
/// or two in-progress slivers were merged.
#[derive(Clone, Copy, Debug)]
pub(crate) enum CutOutcome {
    /// The kemr completion: `face` is the 2-loop null face, `ring` the
    /// loop kemr demoted (the caller must not assume which side it is).
    Completed {
        /// The completed 2-loop null face.
        face: FaceKey,
        /// The loop kemr left as the ring.
        ring: LoopKey,
    },
    /// An interior null edge: kef merged two slivers.
    Merged,
}

/// The reusable chord-join core (ch. 14 Program 14.10's `join`/`cut`
/// mechanics), shared between the split sweep and the boolean joining
/// (M3 PR 5 — "the ch. 14 join reused"): chord `mef`/`mekr` insertion,
/// ring re-homing (`laringmv`), and null-edge retirement. Role
/// resolution and any side-specific certification stay with the
/// callers — the core is side-agnostic.
pub(crate) struct ChordJoiner {
    /// Faces minted by `join`'s mefs — the sliver (section-polygon-in-
    /// progress) faces `cut` may kill.
    slivers: SecondaryMap<FaceKey, ()>,
    /// The run band (ring re-homing containment).
    band: Band,
}

/// The sweep state.
struct Sweep<T: Decide> {
    /// Loose ends, in registration order (growable — no `ends[30]`).
    ends: Vec<HalfEdgeKey>,
    /// The shared chord-join core.
    joiner: ChordJoiner,
    /// Completed polygons, in completion order.
    completed: Vec<CompletedSection>,
    /// Minted above copies (role membership).
    above_set: SecondaryMap<VertexKey, ()>,
    /// The split plane (section-area certification).
    plane: SplitPlane<T>,
    /// The run band.
    band: Band,
}

impl ChordJoiner {
    /// A fresh core.
    pub(crate) fn new(band: Band) -> Self {
        Self {
            slivers: SecondaryMap::new(),
            band,
        }
    }
}

impl<T: Decide> Sweep<T> {
    /// Is `he` currently a loose end?
    fn is_loose(&self, he: HalfEdgeKey) -> bool {
        self.ends.contains(&he)
    }

    /// The up/down sense of a null-edge half — data from the vertex
    /// key (`start ∈ above_set` ⇒ down half).
    fn is_down<B: Decide>(&self, body: &Body<B>, he: HalfEdgeKey) -> Result<bool, SplitJoinError> {
        let start = body.get_half_edge(he).ok_or(SplitJoinError::Corrupt)?.start;
        Ok(self.above_set.contains_key(start))
    }

    /// `canjoin`: scan the loose ends for a topological neighbor of
    /// `half` (same face, opposite sense); consume and return it, or
    /// register `half` as a new loose end and return `None`.
    fn take_neighbor(
        &mut self,
        body: &Body<T>,
        half: HalfEdgeKey,
    ) -> Result<Option<HalfEdgeKey>, SplitJoinError> {
        let face = he_face(body, half)?;
        let down = self.is_down(body, half)?;
        for i in 0..self.ends.len() {
            let end = self.ends[i];
            if he_face(body, end)? == face && self.is_down(body, end)? != down {
                self.ends.remove(i);
                return Ok(Some(end));
            }
        }
        self.ends.push(half);
        Ok(None)
    }
}

impl ChordJoiner {
    /// `join` (module docs): connect the old loose end `h1` and the
    /// new half `h2` with up to two chord edges; the minted chord
    /// edges come back (the boolean joining records their germ — M3
    /// PR 5).
    pub(crate) fn join<T: Decide>(
        &mut self,
        body: &mut Body<T>,
        h1: HalfEdgeKey,
        h2: HalfEdgeKey,
    ) -> Result<Vec<EdgeKey>, SplitJoinError> {
        let corrupt = || SplitJoinError::Corrupt;
        let l1 = body.get_half_edge(h1).ok_or_else(corrupt)?.parent_loop;
        let l2 = body.get_half_edge(h2).ok_or_else(corrupt)?.parent_loop;
        let oldf = body.get_loop(l1).ok_or_else(corrupt)?.face;
        let next = |body: &Body<T>, he: HalfEdgeKey| -> Result<HalfEdgeKey, SplitJoinError> {
            Ok(body.get_half_edge(he).ok_or(SplitJoinError::Corrupt)?.next)
        };
        let prev = |body: &Body<T>, he: HalfEdgeKey| -> Result<HalfEdgeKey, SplitJoinError> {
            Ok(body.get_half_edge(he).ok_or(SplitJoinError::Corrupt)?.prev)
        };

        let mut chords = Vec::new();
        let mut newf = None;
        if l1 == l2 {
            if prev(body, prev(body, h1)?)? != h2 {
                let created = body.mef_chord(MefSite::Chords {
                    he1: h1,
                    he2: next(body, h2)?,
                })?;
                self.slivers.insert(created.face, ());
                chords.push(created.edge);
                newf = Some(created.face);
            }
        } else {
            // Structural ring choice (module docs): kill the loop that
            // is not the face's outer; if both are rings, keep the
            // book's order (kill h2's loop).
            let outer = body.get_face(oldf).ok_or_else(corrupt)?.outer;
            let (target, ring) = if l2 == outer {
                (next(body, h2)?, h1)
            } else {
                (h1, next(body, h2)?)
            };
            let made = body.mekr_chord(MekrSite::Cycles { target, ring })?;
            chords.push(made.edge);
        }
        // Second-chord guard: when the two halves are already adjacent
        // the second mef is skipped (the chord already exists).
        if next(body, next(body, h1)?)? != h2 {
            let created = body.mef_chord(MefSite::Chords {
                he1: h2,
                he2: next(body, h1)?,
            })?;
            self.slivers.insert(created.face, ());
            chords.push(created.edge);
        }
        // Ring re-homing (`laringmv`, the lkemr/ring-placement mirror
        // site): whenever the FIRST mef divided a face that still owns
        // rings. PR 3 shipped this call *inside* the second-chord
        // guard (Program 14.10's literal placement) and flagged the
        // skip window as a watch item; the re-homing need depends only
        // on the first mef having divided the face, not on whether the
        // second chord was minted, so the call now runs unconditionally
        // on `newf` (M3 PR 5 — the boolean joining reaches face-
        // dividing joins with unrelated rings present). The book never
        // re-homes after the second mef alone (its face is the sliver
        // between the two chords, which bounds no ring-holding region);
        // that placement is kept.
        if let Some(newf) = newf {
            self.rehome_rings(body, oldf, newf)?;
        }
        Ok(chords)
    }

    /// `laringmv(oldf, newf)`: move every ring of `oldf` that no
    /// longer lies inside `oldf`'s outer loop into `newf` — decided by
    /// the trilean containment predicate, never a raw comparison.
    fn rehome_rings<T: Decide>(
        &mut self,
        body: &mut Body<T>,
        oldf: FaceKey,
        newf: FaceKey,
    ) -> Result<(), SplitJoinError> {
        let corrupt = || SplitJoinError::Corrupt;
        let face = body.get_face(oldf).ok_or_else(corrupt)?;
        if face.rings.is_empty() {
            return Ok(());
        }
        let outer = face.outer;
        let rings = face.rings.clone();
        let normal = face_plane_normal(body, oldf)?;
        for ring in rings {
            let rep = ring_representative(body, ring)?;
            match point_in_loop(body, outer, normal, rep, self.band)? {
                LoopContainment::In => {}
                LoopContainment::Out => body.ring_move(ring, newf)?,
                LoopContainment::OnBoundary => {
                    return Err(SplitJoinError::RingHomingAmbiguous { ring });
                }
            }
        }
        Ok(())
    }

    /// `cut` (module docs): retire a fully-joined null edge. The
    /// completion outcome comes back unresolved — the caller assigns
    /// roles from its own side data.
    pub(crate) fn cut_core<T: Decide>(
        &mut self,
        body: &mut Body<T>,
        edge: EdgeKey,
    ) -> Result<CutOutcome, SplitJoinError> {
        let corrupt = || SplitJoinError::Corrupt;
        let edge_data = body.get_edge(edge).ok_or_else(corrupt)?.clone();
        let loop_of = |body: &Body<T>, he: HalfEdgeKey| -> Result<LoopKey, SplitJoinError> {
            Ok(body
                .get_half_edge(he)
                .ok_or(SplitJoinError::Corrupt)?
                .parent_loop)
        };
        let l_plus = loop_of(body, edge_data.he_plus)?;
        let l_minus = loop_of(body, edge_data.he_minus)?;
        if l_plus == l_minus {
            // The last null edge of a section polygon: kemr leaves the
            // 2-loop null face.
            let face = body.get_loop(l_plus).ok_or_else(corrupt)?.face;
            let result = body.kemr(edge_data.he_plus, edge_data.he_minus)?;
            Ok(CutOutcome::Completed {
                face,
                ring: result.ring,
            })
        } else {
            // Interior null edge: kef merges the two slivers. Kill a
            // sliver side (never a real face), deterministically
            // preferring he_plus's side.
            let f_plus = body.get_loop(l_plus).ok_or_else(corrupt)?.face;
            let f_minus = body.get_loop(l_minus).ok_or_else(corrupt)?.face;
            let victim = if self.slivers.contains_key(f_plus) {
                edge_data.he_plus
            } else if self.slivers.contains_key(f_minus) {
                edge_data.he_minus
            } else {
                return Err(SplitJoinError::CutInvariant { edge });
            };
            let killed = body.kef(victim)?;
            self.slivers.remove(killed.killed_face);
            Ok(CutOutcome::Merged)
        }
    }
}

impl<T: Decide> Sweep<T> {
    /// `cut` with the split lane's role resolution, area certification,
    /// and F9 record-keeping layered on the shared core.
    fn cut(&mut self, body: &mut Body<T>, edge: EdgeKey) -> Result<(), SplitJoinError> {
        let corrupt = || SplitJoinError::Corrupt;
        match self.joiner.cut_core(body, edge)? {
            CutOutcome::Merged => Ok(()),
            CutOutcome::Completed { face, ring } => {
                let outer_loop = body.get_face(face).ok_or_else(corrupt)?.outer;
                let (above_loop, below_loop) = self.resolve_roles(body, face, outer_loop, ring)?;
                self.certify_section_area(body, face, below_loop)?;
                body.set_null_face_pair(
                    face,
                    NullFacePair::Split {
                        above_loop,
                        below_loop,
                    },
                )?;
                self.completed.push(CompletedSection {
                    face,
                    above_loop,
                    below_loop,
                });
                Ok(())
            }
        }
    }

    /// Resolve which of the null face's two loops is the above loop —
    /// by membership of the minted-copy vertex set, uniform across the
    /// loop (mixed ⇒ typed kernel-bug error).
    fn resolve_roles(
        &self,
        body: &Body<T>,
        face: FaceKey,
        outer: LoopKey,
        ring: LoopKey,
    ) -> Result<(LoopKey, LoopKey), SplitJoinError> {
        let classify = |l: LoopKey| -> Result<bool, SplitJoinError> {
            let starts = loop_starts(body, l)?;
            let above = starts
                .iter()
                .filter(|v| self.above_set.contains_key(**v))
                .count();
            if above == starts.len() {
                Ok(true)
            } else if above == 0 {
                Ok(false)
            } else {
                Err(SplitJoinError::SectionLoopMixed { face })
            }
        };
        match (classify(outer)?, classify(ring)?) {
            (true, false) => Ok((outer, ring)),
            (false, true) => Ok((ring, outer)),
            _ => Err(SplitJoinError::SectionLoopMixed { face }),
        }
    }

    /// Certify the completed polygon's area definitely positive
    /// (margin 2·|A|/P — mean width in meters, profile's
    /// `loop_orientation` lever-arm story); Zero ⇒ the degenerate
    /// one-sided-tangency section, refused typed.
    fn certify_section_area(
        &self,
        body: &Body<T>,
        face: FaceKey,
        below_loop: LoopKey,
    ) -> Result<(), SplitJoinError> {
        let points = loop_points_of(body, below_loop)?;
        let origin = points[0];
        let mut twice_area = T::zero();
        let mut perimeter = T::zero();
        for i in 0..points.len() {
            let a = points[i] - origin;
            let b = points[(i + 1) % points.len()] - origin;
            twice_area = twice_area + a.cross(b).dot(self.plane.normal);
            perimeter = perimeter + (b - a).norm();
        }
        let margin = twice_area.abs() / (perimeter * T::from_f64(0.5));
        match decide("split_section_area", margin, self.band) {
            Ok(Sign::Positive) => Ok(()),
            Ok(_) => Err(SplitJoinError::DegenerateSection { face }),
            Err(diag) => Err(SplitJoinError::Escalated { face, diag }),
        }
    }
}

/// The start vertices of a cycle loop.
pub(crate) fn loop_starts<T: Decide>(
    body: &Body<T>,
    l: LoopKey,
) -> Result<Vec<VertexKey>, SplitJoinError> {
    let corrupt = || SplitJoinError::Corrupt;
    let LoopBoundary::Cycle { first } = body.get_loop(l).ok_or_else(corrupt)?.boundary else {
        return Err(corrupt());
    };
    let mut out = Vec::new();
    for he in body.loop_cycle(first).ok_or_else(corrupt)? {
        out.push(body.get_half_edge(he).ok_or_else(corrupt)?.start);
    }
    Ok(out)
}

/// The start points of a cycle loop, in cycle order.
pub(super) fn loop_points_of<T: Decide>(
    body: &Body<T>,
    l: LoopKey,
) -> Result<Vec<Point3<T>>, SplitJoinError> {
    let starts = loop_starts(body, l)?;
    starts.into_iter().map(|v| vertex_point(body, v)).collect()
}

/// The face's plane normal (F5-gated: always a `Plane`).
fn face_plane_normal<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<geom_core::Vec3<T>, SplitJoinError> {
    let f = body.get_face(face).ok_or(SplitJoinError::Corrupt)?;
    match body.get_surface(f.surface) {
        Some(geom_surfaces::Surface::Plane { normal, .. }) => Ok(*normal),
        _ => Err(SplitJoinError::Corrupt),
    }
}

/// A representative point of a ring (its anchor vertex).
fn ring_representative<T: Decide>(
    body: &Body<T>,
    ring: LoopKey,
) -> Result<Point3<T>, SplitJoinError> {
    let corrupt = || SplitJoinError::Corrupt;
    let v = match body.get_loop(ring).ok_or_else(corrupt)?.boundary {
        LoopBoundary::Cycle { first } => body.get_half_edge(first).ok_or_else(corrupt)?.start,
        LoopBoundary::Empty { vertex } => vertex,
    };
    vertex_point(body, v)
}
