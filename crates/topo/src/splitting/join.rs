//! Joining of null edges (ch. 14 §14.7, `splitconnect`): pair loose
//! null-edge halves into section-polygon chains, connect them with
//! real on-plane edges, and close each chain into a 2-loop **null
//! face** — recorded as F9 data
//! ([`NullFacePair::Split`](crate::null::NullFacePair)) with the
//! above/below roles determined by **vertex-key membership**, never by
//! `floops` list position or he1/he2 slot convention.
//!
//! The chord `mef`/`mekr` insertion, the ring re-homing and the
//! null-edge retirement this sweep drives are NOT here: they are
//! [`crate::chord_join`], one implementation shared with the boolean
//! lane's joining (smell scan S5). What is here is the split lane's
//! **sweep** — the order it visits null edges in, the loose-end
//! pairing, and the role resolution and area certification it layers
//! on the core's side-agnostic outcome.
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
//! # What this lane adds to the core's `cut`
//!
//! When [`ChordJoiner::cut_core`] reports the polygon COMPLETE, the
//! 2-loop null face comes back with its roles unresolved. This lane
//! resolves them by membership of the minted above-copy vertex set,
//! certifies the polygon's area definitely-positive
//! (**`split_section_area`**, margin 2·A/P) — a zero-area section
//! polygon is the one-sided tangency residue PR 2's adjudication
//! record promised to refuse here, typed
//! [`SplitJoinError::DegenerateSection`] — and writes the F9 record.

use geom_core::{Band, Decide, Margin, Point3, Sign};
use slotmap::SecondaryMap;

use super::order;
use super::{SplitPlane, SplitReduction};
use crate::body::Body;
use crate::chord_join::{
    ChordJoiner, CutOutcome, FragmentRows, JoinLane, SectionCtx, SplitJoinError, corrupt_edge,
    corrupt_face, corrupt_he, corrupt_loop, vertex_point,
};
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::null::{CurveGeom, NullFacePair};
use crate::validate::decide;
use geom_core::Tol;

/// One completed section polygon: the null face and its role loops
/// (mirrors the body's [`NullFacePair`] record; carried separately so
/// the finish step consumes explicit keys in completion order).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CompletedSection {
    /// The null face (two coincident loops).
    pub face: FaceKey,
    /// The loop through the minted above copies.
    pub above_loop: LoopKey,
    /// The loop through the original below-side vertices.
    pub below_loop: LoopKey,
}

/// The joining sweep (module docs). Mutates `red.body` in place;
/// returns the completed section polygons in completion order, with
/// the F9 `NullFacePair::Split` records set on the body, and the
/// chord-mef fragment rows the core logged.
///
/// # Errors
///
/// [`SplitJoinError`] — the body may be left mid-surgery on `Err`
/// (callers operate on a scratch clone; the public ops discard it).
pub(super) fn split_connect<T: Decide>(
    red: &mut SplitReduction<T>,
    band: Band,
    tol: Tol,
) -> Result<(Vec<CompletedSection>, FragmentRows), SplitJoinError> {
    let exact = order::exact_band().map_err(SplitJoinError::Band)?;

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
        section: SectionCtx {
            plane: red.plane,
            plane_key: None,
        },
    };

    for idx in sorted {
        let record = red.null_edges[idx];
        let edge = red
            .body
            .get_edge(record.edge)
            .ok_or_else(|| corrupt_edge(record.edge))?
            .clone();
        // Data order: the up half (starting at below_end) first — the
        // book's he1-first as data (module docs).
        let (up, down) = {
            let plus_start = red
                .body
                .get_half_edge(edge.he_plus)
                .ok_or_else(|| corrupt_he(edge.he_plus))?
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
                let Sweep {
                    joiner, section, ..
                } = &mut st;
                joiner.join(&mut red.body, end, half, JoinLane::Split(section), tol)?;
                joined[slot] = true;
                // Retire the consumed end's edge if its other half is
                // no longer loose.
                let end_edge = he_edge(&red.body, end)?;
                let mate = red.body.mate(end).ok_or_else(|| corrupt_he(end))?;
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
    let fragments = st.joiner.take_fragments();
    Ok((st.completed, fragments))
}

/// The edge of a half-edge.
fn he_edge<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Result<EdgeKey, SplitJoinError> {
    Ok(body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.edge)
}

/// The face owning a half-edge's loop.
fn he_face<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Result<FaceKey, SplitJoinError> {
    let l = body
        .get_half_edge(he)
        .ok_or_else(|| corrupt_he(he))?
        .parent_loop;
    Ok(body.get_loop(l).ok_or_else(|| corrupt_loop(l))?.face)
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
    /// The section-geometry context (M5 PR 5: the conic chord lane).
    section: SectionCtx<T>,
}

impl<T: Decide> Sweep<T> {
    /// Is `he` currently a loose end?
    fn is_loose(&self, he: HalfEdgeKey) -> bool {
        self.ends.contains(&he)
    }

    /// The up/down sense of a null-edge half — data from the vertex
    /// key (`start ∈ above_set` ⇒ down half).
    fn is_down<B: Decide>(&self, body: &Body<B>, he: HalfEdgeKey) -> Result<bool, SplitJoinError> {
        let start = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.start;
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

impl<T: Decide> Sweep<T> {
    /// `cut` with the split lane's role resolution, area certification,
    /// and F9 record-keeping layered on the shared core.
    fn cut(&mut self, body: &mut Body<T>, edge: EdgeKey) -> Result<(), SplitJoinError> {
        match self.joiner.cut_core(body, edge)? {
            CutOutcome::Merged => Ok(()),
            CutOutcome::Completed { face, ring } => {
                let outer_loop = body.get_face(face).ok_or_else(|| corrupt_face(face))?.outer;
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
    ///
    /// **Conic boundary edges (M5 PR 5)**: the vertex shoelace below is
    /// exact for straight chords and stays BIT-IDENTICAL for all-planar
    /// sections; each circle/ellipse section edge then adds its exact
    /// closed-form segment excess
    /// `[(c − O)×(B − A)]·n̂ + s_a·s_b·(axis·n̂)·Δt − [(A − O)×(B − O)]·n̂`
    /// (the eccentric-anomaly parameterization's constant areal rate —
    /// the same fact as `loop_vector_area`'s ellipse arm), and its
    /// perimeter contribution is raised from the chord to the
    /// conservative arc-length bound `s_a·|Δt|` (a larger perimeter
    /// only shrinks the mean-width margin — refuses more, never less).
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
        // The conic excess pass (adds nothing for all-planar loops).
        let LoopBoundary::Cycle { first } = body
            .get_loop(below_loop)
            .ok_or_else(|| corrupt_loop(below_loop))?
            .boundary
        else {
            // The key RESOLVED; the shape is wrong. A completed section
            // polygon's below loop is a cycle by construction, so an
            // `Empty` one is this lane's own invariant, not a dangling
            // reference — and `Corrupt` is corruption only.
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "a completed section polygon's below loop holds a lone vertex \
                       instead of a cycle",
            });
        };
        for he in body.loop_cycle(first).ok_or_else(|| corrupt_he(first))? {
            let he_data = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?;
            let edge = body
                .get_edge(he_data.edge)
                .ok_or_else(|| corrupt_edge(he_data.edge))?;
            let Some(CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
                continue;
            };
            let (c_e, axis_e, sa, sb) = match *curve.carrier() {
                geom::Curve3::Circle {
                    center,
                    axis,
                    radius,
                    ..
                } => (center, axis, radius, radius),
                geom::Curve3::Ellipse {
                    center,
                    axis,
                    major,
                    minor,
                    ..
                } => (center, axis, major, minor),
                geom::Curve3::Line { .. } | geom::Curve3::Nurbs(_) => continue,
            };
            let (t0, t1) = curve.params();
            let span = t1 - t0;
            let forward = edge.he_plus == he;
            let a_pt = vertex_point(body, he_data.start)?;
            let b_pt = vertex_point(body, body.half_edge_end(he).ok_or_else(|| corrupt_he(he))?)?;
            let dt_signed = if forward { span } else { T::zero() - span };
            let a = a_pt - origin;
            let b = b_pt - origin;
            let excess = (c_e - origin).cross(b_pt - a_pt).dot(self.plane.normal)
                + sa * sb * axis_e.dot(self.plane.normal) * dt_signed
                - a.cross(b).dot(self.plane.normal);
            twice_area = twice_area + excess;
            perimeter = perimeter + (sa * span.abs() - (b - a).norm());
        }
        // `twice_area` IS 2A (shoelace), so dividing by the full
        // perimeter yields the documented margin 2·|A|/P — the mean
        // width in meters — the dimensional argument stated once at
        // `Margin::over_lever`'s door, and the factor itself held as
        // a contract by this module's head docs and by
        // `docs/predicate-dimension-audit.md`'s row for this
        // predicate. `chart_region_area` asks the same question two
        // dimensions down through the same door; the accumulators
        // stay separate, for the reasons written at that site.
        let margin = Margin::over_lever(twice_area.abs(), perimeter);
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
    let loop_data = body.get_loop(l).ok_or_else(|| corrupt_loop(l))?;
    let LoopBoundary::Cycle { first } = loop_data.boundary else {
        // Resolved, but shaped wrong: an `Empty` loop has no starts to
        // walk. That is a caller invariant, not a corrupt arena.
        return Err(SplitJoinError::SectionInvariant {
            face: loop_data.face,
            what: "a loop asked for its start vertices holds a lone vertex instead of \
                   a cycle",
        });
    };
    let mut out = Vec::new();
    for he in body.loop_cycle(first).ok_or_else(|| corrupt_he(first))? {
        out.push(body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.start);
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
