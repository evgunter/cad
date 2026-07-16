//! The structural validation harness: [`validate`] and
//! [`ValidationError`].
//!
//! M1 PR 1 validates **tier-1 structural coherence** of the half-edge
//! representation — the referential and combinatorial soundness of the
//! arena store, checkable without any geometry evaluation. Tier 1
//! accepts every Euler-reachable state, construction scaffolding
//! included (empty loops, struts, self-loop edges, laminae): those are
//! mandatory intermediates, and it is the operators (PR 2+) and tier 2
//! (PR 5's `validate_closed`) that layer "finished solid" rules on top.
//! Watertightness in the half-edge representation is *structural* and
//! therefore already tier 1: every edge has exactly two antiparallel
//! half-edges (passes 3–4) and every vertex orbit is a single cycle
//! (pass 6).
//!
//! The check set (each check names its error variants):
//!
//! 1. **Reference resolution.** Every topology key held by an entity
//!    resolves in its arena ([`ValidationError::DanglingTopology`]), and
//!    every geometry key resolves — a face's surface, an edge's curve, a
//!    vertex's point ([`ValidationError::DanglingGeometry`]).
//! 2. **Half-edge chain coherence.** `next`/`prev` are mutual inverses
//!    ([`ValidationError::NextPrevMismatch`]); every
//!    [`LoopBoundary::Cycle`] closes within the arena bound
//!    ([`ValidationError::LoopCycleOverrun`]); every half-edge reached
//!    by a loop's cycle points back to that loop
//!    ([`ValidationError::ParentLoopMismatch`]); every half-edge is
//!    reached by its own parent's cycle
//!    ([`ValidationError::UnreachableHalfEdge`] — also fired when the
//!    claimed parent is an `Empty` loop, which reaches nothing).
//! 3. **Edge ↔ half-edge bijection.** An edge's halves are distinct
//!    ([`ValidationError::EdgeHalvesIdentical`]), both point back via
//!    `.edge` ([`ValidationError::EdgeSlotBackpointerMismatch`]), and
//!    every half-edge is claimed by exactly one edge slot overall
//!    ([`ValidationError::HalfEdgeUnclaimed`] /
//!    [`ValidationError::HalfEdgeMultiplyClaimed`]).
//! 4. **Antiparallelism.** The two halves traverse the edge in opposite
//!    directions: `end(he_plus) = start(he_minus)` and vice versa
//!    ([`ValidationError::EdgeNotAntiparallel`]).
//! 5. **Vertex anchoring.** `emanating = Some(he)` ⇒ `he` starts at the
//!    vertex ([`ValidationError::EmanatingStartMismatch`]) and the
//!    vertex is no empty loop's lone vertex
//!    ([`ValidationError::EmptyLoopVertexWithEmanating`]);
//!    `emanating = None` ⇒ no half-edge starts at the vertex
//!    ([`ValidationError::LoneVertexWithIncidence`]) and the vertex is
//!    the lone vertex of *exactly one* empty loop — zero is
//!    [`ValidationError::OrphanEntity`], two or more is
//!    [`ValidationError::MultiplyOwned`]. (This restates M0's
//!    orphan-vertex rule in half-edge terms: every vertex is anchored by
//!    incident half-edges XOR by one empty loop.)
//! 6. **Vertex-orbit closure (manifoldness).** The bounded orbit walk
//!    from `emanating` returns to its start
//!    ([`ValidationError::VertexOrbitOverrun`] on non-closure) and
//!    visits exactly the half-edges starting at the vertex — no foreign
//!    members ([`ValidationError::OrbitForeignMember`]), no split orbits
//!    ([`ValidationError::SplitVertexOrbit`], the classic non-manifold
//!    "bowtie" catch).
//! 7. **Ownership and back-pointers.** `outer ∉ rings`
//!    ([`ValidationError::OuterListedAsRing`]); every shell, face, and
//!    loop is owned exactly once by its parent kind, counting
//!    multiplicity (zero is [`ValidationError::OrphanEntity`], two or
//!    more [`ValidationError::MultiplyOwned`] — duplicate rings surface
//!    here); the upward back-pointers `loop.face`, `face.shell`,
//!    `shell.solid` match the actual owner
//!    ([`ValidationError::BackPointerMismatch`]).
//! 8. **Orphan geometry.** Every point, curve, and surface is referenced
//!    by at least one entity ([`ValidationError::OrphanGeometry`]) — an
//!    **error**, not a warning: bodies are values built by operators,
//!    and nothing should leak.
//!
//! The harness is deliberately a plain function plus an error enum,
//! **not a trait**: there is exactly one notion of body validity per
//! milestone, and speculative abstraction would only blur it.
//!
//! # Deliberately deferred to PR 5
//!
//! Named so they are not mistaken for oversights: **tier 2**
//! (`validate_closed` — no empty loops, no struts on finished solids),
//! **Euler–Poincaré counting** per shell (`v − e + f − r = 2(1 − h)`),
//! and the **bidirectional D5 provenance check** (kill-side operators,
//! PR 4, are what first make provenance leaks reachable). The E–P pass
//! is also what closes the **shell-partition vs. edge-adjacency
//! coherence** gap: tier 1 checks the ownership tree but never that an
//! edge's two faces sit in the *same shell* — a cube with one side face
//! moved to a second solid+shell (membership and back-pointers
//! self-consistent) passes every pass here, and it is the per-shell
//! count that fails on the split shell. Do not read passes 3–4 + 6 as a
//! complete per-shell watertightness story until PR 5 lands. Geometric
//! validation (D4 ¶2 residual certification) starts at M2.
//!
//! # All failures, not the first
//!
//! [`validate`] collects **every** failure before returning: a validator
//! that stops at the first defect is a bad debugging tool, and fail-loud
//! (D4) means report everything. The `Err` vector is never empty.
//!
//! # Cascade discipline
//!
//! One defect must not drown the report in echoes, so downstream checks
//! are *gated* on their prerequisites, and every gate is chosen so that
//! skipping is only possible when an earlier pass already reported the
//! cause: a walk that hits a stale link or a broken mate stays silent
//! (pass 1/3 reported it); reachability is only judged against loops
//! whose cycle closed; antiparallelism needs both halves distinct and
//! both ends derivable; orbit checks need a resolving `emanating` that
//! starts at its vertex; back-pointer comparisons need an unambiguous
//! (exactly-one) owner and a resolving stored key. Genuinely independent
//! facts still report independently — a corruption that breaks two
//! invariants yields two errors.
//!
//! # Deterministic report order (D9)
//!
//! Errors arrive in a fixed, documented order — eight passes, each
//! walking its arenas in slot-index order, checking an entity's
//! references in field-declaration order:
//!
//! 1. reference resolution, walking solids → shells → faces → loops →
//!    half-edges → edges → vertices;
//! 2. chain coherence: next/prev inverses (half-edges), then cycle walks
//!    (loops; members in walk order), then unreachable half-edges
//!    (half-edges);
//! 3. bijection: per-edge checks (halves identical, then plus/minus
//!    back-pointers), then per-half-edge claim counts;
//! 4. antiparallelism, sweeping edges;
//! 5. vertex anchoring, sweeping vertices;
//! 6. orbit closure, sweeping vertices (foreign members in walk order);
//! 7. ownership/back-pointers: outer-vs-rings (faces), then shells,
//!    faces, loops;
//! 8. orphan geometry, sweeping points → curves → surfaces.

use core::fmt;

use geom_core::Real;
use slotmap::{Key, SecondaryMap};

use crate::body::{Body, Walk};
use crate::entity::{
    EdgeKey, EntityId, FaceKey, GeomRef, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey,
};

/// A structural defect found by [`validate`]. Closed enum, D3 style:
/// later PRs add variants (Euler–Poincaré, tier-2 closure, provenance)
/// as compiler-guided extensions — every match site is forced to say
/// what it does with the new failure kinds, which is exactly the
/// loudness the house style wants.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationError {
    /// An entity holds a topology key that does not resolve in its arena.
    /// Reported once per occurrence (a parent listing the same dangling
    /// key twice yields two errors).
    DanglingTopology {
        /// The entity holding the dangling reference.
        from: EntityId,
        /// The dangling key (wrapped with its kind; it resolves to
        /// nothing).
        to: EntityId,
    },
    /// An entity holds a geometry key that does not resolve in its arena.
    DanglingGeometry {
        /// The entity holding the dangling reference.
        from: EntityId,
        /// The dangling geometry key.
        to: GeomRef,
    },
    /// `next(half_edge).prev != half_edge` — the doubly-linked cycle is
    /// torn between this half-edge and its successor.
    NextPrevMismatch {
        /// The half-edge whose successor does not point back.
        half_edge: HalfEdgeKey,
    },
    /// Walking `next` from the loop's `Cycle::first` failed to return to
    /// it within the arena bound (the walk wandered into a cycle not
    /// containing its start — always accompanied by the
    /// [`NextPrevMismatch`](Self::NextPrevMismatch) that made `next`
    /// non-injective).
    LoopCycleOverrun {
        /// The loop whose cycle does not close.
        loop_: LoopKey,
    },
    /// A half-edge reached by a loop's cycle has `parent_loop` pointing
    /// elsewhere.
    ParentLoopMismatch {
        /// The half-edge with the wrong back-pointer.
        half_edge: HalfEdgeKey,
        /// The loop whose cycle actually reaches it.
        owner: LoopKey,
    },
    /// A half-edge that its own `parent_loop`'s boundary does not reach:
    /// the parent's cycle closed without it, or the parent is an
    /// [`LoopBoundary::Empty`] loop (which reaches nothing).
    UnreachableHalfEdge {
        /// The unreachable half-edge.
        half_edge: HalfEdgeKey,
    },
    /// An edge whose two half-edge slots hold the same key — an edge
    /// must have two distinct halves.
    EdgeHalvesIdentical {
        /// The degenerate edge.
        edge: EdgeKey,
    },
    /// An edge slot's half-edge does not point back to the edge via
    /// `.edge`.
    EdgeSlotBackpointerMismatch {
        /// The claiming edge.
        edge: EdgeKey,
        /// The claimed half-edge whose `.edge` points elsewhere.
        half_edge: HalfEdgeKey,
    },
    /// A half-edge no edge slot claims (every half-edge must be exactly
    /// one edge's plus or minus half).
    HalfEdgeUnclaimed {
        /// The unclaimed half-edge.
        half_edge: HalfEdgeKey,
    },
    /// A half-edge claimed by more than one edge slot, counting
    /// multiplicity (an edge claiming it in both slots reports 2).
    HalfEdgeMultiplyClaimed {
        /// The multiply-claimed half-edge.
        half_edge: HalfEdgeKey,
        /// How many edge slots claim it (≥ 2).
        claims: usize,
    },
    /// An edge whose halves do not traverse it in opposite directions:
    /// `end(he_plus) != start(he_minus)` or
    /// `end(he_minus) != start(he_plus)`.
    EdgeNotAntiparallel {
        /// The misoriented edge.
        edge: EdgeKey,
    },
    /// A vertex's `emanating` half-edge does not start at the vertex.
    EmanatingStartMismatch {
        /// The vertex.
        vertex: VertexKey,
        /// Its `emanating` half-edge (which starts elsewhere).
        emanating: HalfEdgeKey,
    },
    /// A vertex with an `emanating` half-edge that is also the lone
    /// vertex of one or more empty loops — a lone vertex must have no
    /// half-edges (`emanating: None`).
    EmptyLoopVertexWithEmanating {
        /// The doubly-anchored vertex.
        vertex: VertexKey,
        /// How many empty loops hold it (≥ 1).
        empty_loops: usize,
    },
    /// A vertex with `emanating: None` at which half-edges nevertheless
    /// start — a lone vertex must have none.
    LoneVertexWithIncidence {
        /// The vertex claiming to be lone.
        vertex: VertexKey,
        /// How many half-edges start at it (≥ 1).
        incident: usize,
    },
    /// Walking the vertex orbit (`next(mate(he))`) from `emanating`
    /// failed to return to it within the arena bound (always accompanied
    /// by the chain/bijection error that broke the orbit permutation).
    VertexOrbitOverrun {
        /// The vertex whose orbit does not close.
        vertex: VertexKey,
    },
    /// The vertex orbit visited a half-edge that does not start at the
    /// vertex — the orbit permutation is crossing between vertices.
    OrbitForeignMember {
        /// The vertex whose orbit was walked.
        vertex: VertexKey,
        /// The visited half-edge that starts elsewhere.
        half_edge: HalfEdgeKey,
    },
    /// The vertex orbit closed but did not visit every half-edge
    /// starting at the vertex: the incident half-edges fall into more
    /// than one orbit — a non-manifold vertex (the "bowtie" case).
    SplitVertexOrbit {
        /// The non-manifold vertex.
        vertex: VertexKey,
        /// How many half-edges the closed orbit visited.
        orbit: usize,
        /// How many half-edges start at the vertex (> `orbit`).
        incident: usize,
    },
    /// A face listing its outer loop among its rings — `outer` is
    /// excluded from `rings` by construction (see [`crate::Face`]).
    OuterListedAsRing {
        /// The offending face.
        face: FaceKey,
    },
    /// A spine back-pointer (`loop.face`, `face.shell`, `shell.solid`)
    /// that does not match the entity's actual (unique) owner.
    BackPointerMismatch {
        /// The entity with the wrong back-pointer.
        child: EntityId,
        /// What the back-pointer stores.
        stored: EntityId,
        /// The parent that actually owns the child.
        owner: EntityId,
    },
    /// A spine entity nothing anchors: a shell in no solid, a face in no
    /// shell, a loop in no face (outer or ring), or a vertex at which no
    /// half-edge starts and which no empty loop holds.
    OrphanEntity {
        /// The unreferenced entity.
        entity: EntityId,
    },
    /// A shell, face, or loop referenced more than once by its parent
    /// kind (the containment spine is a tree of ownership down to
    /// loops), or an empty-loop vertex held by more than one empty loop.
    /// Multiplicity counts: one parent listing a child twice reports 2.
    MultiplyOwned {
        /// The multiply-referenced child.
        child: EntityId,
        /// How many owning references were found (≥ 2).
        owners: usize,
    },
    /// A geometry-arena entry no entity references. An error, not a
    /// warning: bodies are values built by operators, and nothing should
    /// leak (see the [module docs](self)).
    OrphanGeometry {
        /// The unreferenced geometry entry.
        geometry: GeomRef,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DanglingTopology { from, to } => {
                write!(f, "{from} references {to}, which does not resolve")
            }
            Self::DanglingGeometry { from, to } => {
                write!(f, "{from} references {to}, which does not resolve")
            }
            Self::NextPrevMismatch { half_edge } => write!(
                f,
                "half-edge {half_edge:?}'s next half-edge does not point back \
                 to it via prev"
            ),
            Self::LoopCycleOverrun { loop_ } => write!(
                f,
                "loop {loop_:?}'s cycle does not return to its first half-edge \
                 within the arena bound"
            ),
            Self::ParentLoopMismatch { half_edge, owner } => write!(
                f,
                "half-edge {half_edge:?} is in loop {owner:?}'s cycle but its \
                 parent_loop points elsewhere"
            ),
            Self::UnreachableHalfEdge { half_edge } => write!(
                f,
                "half-edge {half_edge:?} is not reached by its parent loop's \
                 boundary"
            ),
            Self::EdgeHalvesIdentical { edge } => write!(
                f,
                "edge {edge:?}'s two half-edge slots hold the same half-edge"
            ),
            Self::EdgeSlotBackpointerMismatch { edge, half_edge } => write!(
                f,
                "edge {edge:?} claims half-edge {half_edge:?}, which points at \
                 a different edge"
            ),
            Self::HalfEdgeUnclaimed { half_edge } => {
                write!(f, "half-edge {half_edge:?} is claimed by no edge slot")
            }
            Self::HalfEdgeMultiplyClaimed { half_edge, claims } => write!(
                f,
                "half-edge {half_edge:?} is claimed by {claims} edge slots \
                 (exactly one required)"
            ),
            Self::EdgeNotAntiparallel { edge } => write!(
                f,
                "edge {edge:?}'s half-edges do not traverse it in opposite \
                 directions"
            ),
            Self::EmanatingStartMismatch { vertex, emanating } => write!(
                f,
                "vertex {vertex:?}'s emanating half-edge {emanating:?} does \
                 not start at it"
            ),
            Self::EmptyLoopVertexWithEmanating {
                vertex,
                empty_loops,
            } => write!(
                f,
                "vertex {vertex:?} has an emanating half-edge but is the lone \
                 vertex of {empty_loops} empty loop(s)"
            ),
            Self::LoneVertexWithIncidence { vertex, incident } => write!(
                f,
                "vertex {vertex:?} has no emanating half-edge but {incident} \
                 half-edge(s) start at it"
            ),
            Self::VertexOrbitOverrun { vertex } => write!(
                f,
                "vertex {vertex:?}'s orbit does not return to its emanating \
                 half-edge within the arena bound"
            ),
            Self::OrbitForeignMember { vertex, half_edge } => write!(
                f,
                "vertex {vertex:?}'s orbit visits half-edge {half_edge:?}, \
                 which starts at a different vertex"
            ),
            Self::SplitVertexOrbit {
                vertex,
                orbit,
                incident,
            } => write!(
                f,
                "vertex {vertex:?} is non-manifold: its orbit closes over \
                 {orbit} half-edge(s) but {incident} start at it"
            ),
            Self::OuterListedAsRing { face } => {
                write!(f, "face {face:?} lists its outer loop among its rings")
            }
            Self::BackPointerMismatch {
                child,
                stored,
                owner,
            } => write!(f, "{child} points back at {stored} but is owned by {owner}"),
            Self::OrphanEntity { entity } => {
                write!(f, "{entity} is anchored by no parent entity")
            }
            Self::MultiplyOwned { child, owners } => write!(
                f,
                "{child} has {owners} owning references (exactly one required)"
            ),
            Self::OrphanGeometry { geometry } => {
                write!(f, "{geometry} is referenced by no entity")
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Counts one resolved reference to `key` (dangling keys never reach
/// here). `SecondaryMap` rather than a hash map: typed per key kind and
/// deterministic to sweep, per the crate's D9 iteration invariant.
fn count_ref<K: Key>(counts: &mut SecondaryMap<K, usize>, key: K) {
    let n = counts.get(key).copied().unwrap_or(0);
    counts.insert(key, n + 1);
}

/// Counts one owning reference to `key` and remembers the owner. Only
/// the count is meaningful when it ends up ≠ 1; the remembered owner is
/// consulted for back-pointer comparison exactly when the count is 1
/// (in which case it is *the* owner).
fn count_owner<K: Key, O: Copy>(counts: &mut SecondaryMap<K, (usize, O)>, key: K, owner: O) {
    let n = counts.get(key).map_or(0, |(n, _)| *n);
    counts.insert(key, (n + 1, owner));
}

/// Validates a body's tier-1 structural coherence, collecting **all**
/// failures.
///
/// The check set, the report order, the cascade discipline, and the
/// PR 5 deferrals are documented in the [module docs](self). The empty
/// body validates vacuously.
///
/// # Errors
///
/// A non-empty vector of every [`ValidationError`] found, in the
/// documented deterministic order.
pub fn validate<T: Real>(body: &Body<T>) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    // Reference/ownership counters, filled by pass 1 and consumed by the
    // later passes. Only keys that resolve are counted; dangling keys are
    // reported in pass 1 and count toward nothing.
    let mut shell_owners: SecondaryMap<_, (usize, _)> = SecondaryMap::new();
    let mut face_owners: SecondaryMap<_, (usize, _)> = SecondaryMap::new();
    let mut loop_owners: SecondaryMap<_, (usize, _)> = SecondaryMap::new();
    let mut vertex_incidence: SecondaryMap<VertexKey, usize> = SecondaryMap::new();
    let mut vertex_empty_loops: SecondaryMap<VertexKey, usize> = SecondaryMap::new();
    let mut point_refs: SecondaryMap<_, usize> = SecondaryMap::new();
    let mut curve_refs: SecondaryMap<_, usize> = SecondaryMap::new();
    let mut surface_refs: SecondaryMap<_, usize> = SecondaryMap::new();

    // ------------------------------------------------------------------
    // Pass 1: reference resolution, solids → shells → faces → loops →
    // half-edges → edges → vertices; within an entity, field-declaration
    // order.
    // ------------------------------------------------------------------
    for (solid_key, solid) in body.solids.iter() {
        for &shell in &solid.shells {
            if body.shells.contains_key(shell) {
                count_owner(&mut shell_owners, shell, solid_key);
            } else {
                errors.push(ValidationError::DanglingTopology {
                    from: EntityId::Solid(solid_key),
                    to: EntityId::Shell(shell),
                });
            }
        }
    }
    for (shell_key, shell) in body.shells.iter() {
        for &face in &shell.faces {
            if body.faces.contains_key(face) {
                count_owner(&mut face_owners, face, shell_key);
            } else {
                errors.push(ValidationError::DanglingTopology {
                    from: EntityId::Shell(shell_key),
                    to: EntityId::Face(face),
                });
            }
        }
        if !body.solids.contains_key(shell.solid) {
            errors.push(ValidationError::DanglingTopology {
                from: EntityId::Shell(shell_key),
                to: EntityId::Solid(shell.solid),
            });
        }
    }
    for (face_key, face) in body.faces.iter() {
        if body.surfaces.contains_key(face.surface) {
            count_ref(&mut surface_refs, face.surface);
        } else {
            errors.push(ValidationError::DanglingGeometry {
                from: EntityId::Face(face_key),
                to: GeomRef::Surface(face.surface),
            });
        }
        for &loop_ in core::iter::once(&face.outer).chain(&face.rings) {
            if body.loops.contains_key(loop_) {
                count_owner(&mut loop_owners, loop_, face_key);
            } else {
                errors.push(ValidationError::DanglingTopology {
                    from: EntityId::Face(face_key),
                    to: EntityId::Loop(loop_),
                });
            }
        }
        if !body.shells.contains_key(face.shell) {
            errors.push(ValidationError::DanglingTopology {
                from: EntityId::Face(face_key),
                to: EntityId::Shell(face.shell),
            });
        }
    }
    for (loop_key, loop_) in body.loops.iter() {
        match loop_.boundary {
            LoopBoundary::Empty { vertex } => {
                if body.vertices.contains_key(vertex) {
                    count_ref(&mut vertex_empty_loops, vertex);
                } else {
                    errors.push(ValidationError::DanglingTopology {
                        from: EntityId::Loop(loop_key),
                        to: EntityId::Vertex(vertex),
                    });
                }
            }
            LoopBoundary::Cycle { first } => {
                if !body.half_edges.contains_key(first) {
                    errors.push(ValidationError::DanglingTopology {
                        from: EntityId::Loop(loop_key),
                        to: EntityId::HalfEdge(first),
                    });
                }
            }
        }
        if !body.faces.contains_key(loop_.face) {
            errors.push(ValidationError::DanglingTopology {
                from: EntityId::Loop(loop_key),
                to: EntityId::Face(loop_.face),
            });
        }
    }
    for (he_key, he) in body.half_edges.iter() {
        if !body.edges.contains_key(he.edge) {
            errors.push(ValidationError::DanglingTopology {
                from: EntityId::HalfEdge(he_key),
                to: EntityId::Edge(he.edge),
            });
        }
        if body.vertices.contains_key(he.start) {
            count_ref(&mut vertex_incidence, he.start);
        } else {
            errors.push(ValidationError::DanglingTopology {
                from: EntityId::HalfEdge(he_key),
                to: EntityId::Vertex(he.start),
            });
        }
        if !body.loops.contains_key(he.parent_loop) {
            errors.push(ValidationError::DanglingTopology {
                from: EntityId::HalfEdge(he_key),
                to: EntityId::Loop(he.parent_loop),
            });
        }
        for link in [he.next, he.prev] {
            if !body.half_edges.contains_key(link) {
                errors.push(ValidationError::DanglingTopology {
                    from: EntityId::HalfEdge(he_key),
                    to: EntityId::HalfEdge(link),
                });
            }
        }
    }
    for (edge_key, edge) in body.edges.iter() {
        for slot in [edge.he_plus, edge.he_minus] {
            if !body.half_edges.contains_key(slot) {
                errors.push(ValidationError::DanglingTopology {
                    from: EntityId::Edge(edge_key),
                    to: EntityId::HalfEdge(slot),
                });
            }
        }
        if body.curves.contains_key(edge.curve) {
            count_ref(&mut curve_refs, edge.curve);
        } else {
            errors.push(ValidationError::DanglingGeometry {
                from: EntityId::Edge(edge_key),
                to: GeomRef::Curve(edge.curve),
            });
        }
    }
    for (vertex_key, vertex) in body.vertices.iter() {
        if body.points.contains_key(vertex.point) {
            count_ref(&mut point_refs, vertex.point);
        } else {
            errors.push(ValidationError::DanglingGeometry {
                from: EntityId::Vertex(vertex_key),
                to: GeomRef::Point(vertex.point),
            });
        }
        if let Some(emanating) = vertex.emanating
            && !body.half_edges.contains_key(emanating)
        {
            errors.push(ValidationError::DanglingTopology {
                from: EntityId::Vertex(vertex_key),
                to: EntityId::HalfEdge(emanating),
            });
        }
    }

    // ------------------------------------------------------------------
    // Pass 2: half-edge chain coherence — next/prev inverses, cycle
    // walks, reachability.
    // ------------------------------------------------------------------
    for (he_key, he) in body.half_edges.iter() {
        if let Some(next) = body.half_edges.get(he.next)
            && next.prev != he_key
        {
            errors.push(ValidationError::NextPrevMismatch { half_edge: he_key });
        }
    }
    // Which loops' cycles closed, and which half-edges were reached by
    // their own parent's cycle (gates for the reachability check).
    let mut loop_closed: SecondaryMap<LoopKey, ()> = SecondaryMap::new();
    let mut reached_by_parent: SecondaryMap<HalfEdgeKey, ()> = SecondaryMap::new();
    for (loop_key, loop_) in body.loops.iter() {
        let LoopBoundary::Cycle { first } = loop_.boundary else {
            continue;
        };
        match body.loop_walk(first) {
            Walk::Closed(members) => {
                loop_closed.insert(loop_key, ());
                for member in members {
                    // Walk members always resolve (the walk checked).
                    let Some(he) = body.half_edges.get(member) else {
                        continue;
                    };
                    if he.parent_loop == loop_key {
                        reached_by_parent.insert(member, ());
                    } else {
                        errors.push(ValidationError::ParentLoopMismatch {
                            half_edge: member,
                            owner: loop_key,
                        });
                    }
                }
            }
            // Broken: a stale link mid-cycle — pass 1 reported it.
            Walk::Broken => {}
            Walk::Overrun => {
                errors.push(ValidationError::LoopCycleOverrun { loop_: loop_key });
            }
        }
    }
    for (he_key, he) in body.half_edges.iter() {
        let Some(parent) = body.loops.get(he.parent_loop) else {
            continue; // dangling parent: pass 1 reported it
        };
        let unreachable = match parent.boundary {
            // An empty loop reaches no half-edge at all.
            LoopBoundary::Empty { .. } => true,
            // Only judge against cycles that actually closed; a broken or
            // overrun parent already carries its own report.
            LoopBoundary::Cycle { .. } => {
                loop_closed.contains_key(he.parent_loop) && !reached_by_parent.contains_key(he_key)
            }
        };
        if unreachable {
            errors.push(ValidationError::UnreachableHalfEdge { half_edge: he_key });
        }
    }

    // ------------------------------------------------------------------
    // Pass 3: edge ↔ half-edge bijection.
    // ------------------------------------------------------------------
    let mut he_claims: SecondaryMap<HalfEdgeKey, usize> = SecondaryMap::new();
    for (edge_key, edge) in body.edges.iter() {
        if edge.he_plus == edge.he_minus {
            errors.push(ValidationError::EdgeHalvesIdentical { edge: edge_key });
        }
        for slot in [edge.he_plus, edge.he_minus] {
            let Some(he) = body.half_edges.get(slot) else {
                continue; // dangling slot: pass 1 reported it
            };
            count_ref(&mut he_claims, slot);
            if he.edge != edge_key {
                errors.push(ValidationError::EdgeSlotBackpointerMismatch {
                    edge: edge_key,
                    half_edge: slot,
                });
            }
        }
    }
    for (he_key, _) in body.half_edges.iter() {
        match he_claims.get(he_key).copied().unwrap_or(0) {
            0 => errors.push(ValidationError::HalfEdgeUnclaimed { half_edge: he_key }),
            1 => {}
            claims => errors.push(ValidationError::HalfEdgeMultiplyClaimed {
                half_edge: he_key,
                claims,
            }),
        }
    }

    // ------------------------------------------------------------------
    // Pass 4: antiparallelism. Gated on distinct, resolving halves whose
    // ends are derivable (everything else was reported above).
    // ------------------------------------------------------------------
    for (edge_key, edge) in body.edges.iter() {
        if edge.he_plus == edge.he_minus {
            continue;
        }
        let (Some(plus), Some(minus)) = (
            body.half_edges.get(edge.he_plus),
            body.half_edges.get(edge.he_minus),
        ) else {
            continue;
        };
        let (Some(plus_next), Some(minus_next)) = (
            body.half_edges.get(plus.next),
            body.half_edges.get(minus.next),
        ) else {
            continue;
        };
        if plus_next.start != minus.start || minus_next.start != plus.start {
            errors.push(ValidationError::EdgeNotAntiparallel { edge: edge_key });
        }
    }

    // ------------------------------------------------------------------
    // Pass 5: vertex anchoring (see the module docs, check 5).
    // ------------------------------------------------------------------
    for (vertex_key, vertex) in body.vertices.iter() {
        let incident = vertex_incidence.get(vertex_key).copied().unwrap_or(0);
        let empty_loops = vertex_empty_loops.get(vertex_key).copied().unwrap_or(0);
        match vertex.emanating {
            Some(emanating) => {
                if let Some(he) = body.half_edges.get(emanating)
                    && he.start != vertex_key
                {
                    errors.push(ValidationError::EmanatingStartMismatch {
                        vertex: vertex_key,
                        emanating,
                    });
                }
                if empty_loops > 0 {
                    errors.push(ValidationError::EmptyLoopVertexWithEmanating {
                        vertex: vertex_key,
                        empty_loops,
                    });
                }
            }
            None => {
                if incident > 0 {
                    errors.push(ValidationError::LoneVertexWithIncidence {
                        vertex: vertex_key,
                        incident,
                    });
                }
                if incident == 0 && empty_loops == 0 {
                    errors.push(ValidationError::OrphanEntity {
                        entity: EntityId::Vertex(vertex_key),
                    });
                }
                if empty_loops >= 2 {
                    errors.push(ValidationError::MultiplyOwned {
                        child: EntityId::Vertex(vertex_key),
                        owners: empty_loops,
                    });
                }
            }
        }
    }

    // ------------------------------------------------------------------
    // Pass 6: vertex-orbit closure (manifoldness). Gated on a resolving
    // emanating half-edge that starts at the vertex.
    // ------------------------------------------------------------------
    for (vertex_key, vertex) in body.vertices.iter() {
        let Some(emanating) = vertex.emanating else {
            continue;
        };
        let Some(he) = body.half_edges.get(emanating) else {
            continue; // dangling: pass 1 reported it
        };
        if he.start != vertex_key {
            continue; // pass 5 reported the mismatch
        }
        match body.orbit_walk(emanating) {
            // Broken: a stale link or broken mate — passes 1/3 reported
            // the cause.
            Walk::Broken => {}
            Walk::Overrun => {
                errors.push(ValidationError::VertexOrbitOverrun { vertex: vertex_key });
            }
            Walk::Closed(members) => {
                let orbit = members.len();
                let mut foreign = false;
                for member in members {
                    let Some(member_he) = body.half_edges.get(member) else {
                        continue; // walk members always resolve
                    };
                    if member_he.start != vertex_key {
                        foreign = true;
                        errors.push(ValidationError::OrbitForeignMember {
                            vertex: vertex_key,
                            half_edge: member,
                        });
                    }
                }
                let incident = vertex_incidence.get(vertex_key).copied().unwrap_or(0);
                // With foreign members the two counts measure different
                // sets; the foreign reports carry the failure.
                if !foreign && orbit != incident {
                    errors.push(ValidationError::SplitVertexOrbit {
                        vertex: vertex_key,
                        orbit,
                        incident,
                    });
                }
            }
        }
    }

    // ------------------------------------------------------------------
    // Pass 7: ownership and back-pointers. Back-pointer comparison is
    // gated on an unambiguous owner (count exactly 1) and a resolving
    // stored key (a dangling one was reported in pass 1).
    // ------------------------------------------------------------------
    for (face_key, face) in body.faces.iter() {
        if face.rings.contains(&face.outer) {
            errors.push(ValidationError::OuterListedAsRing { face: face_key });
        }
    }
    for (shell_key, shell) in body.shells.iter() {
        match shell_owners.get(shell_key).copied() {
            None => errors.push(ValidationError::OrphanEntity {
                entity: EntityId::Shell(shell_key),
            }),
            Some((1, owner)) => {
                if body.solids.contains_key(shell.solid) && shell.solid != owner {
                    errors.push(ValidationError::BackPointerMismatch {
                        child: EntityId::Shell(shell_key),
                        stored: EntityId::Solid(shell.solid),
                        owner: EntityId::Solid(owner),
                    });
                }
            }
            Some((owners, _)) => errors.push(ValidationError::MultiplyOwned {
                child: EntityId::Shell(shell_key),
                owners,
            }),
        }
    }
    for (face_key, face) in body.faces.iter() {
        match face_owners.get(face_key).copied() {
            None => errors.push(ValidationError::OrphanEntity {
                entity: EntityId::Face(face_key),
            }),
            Some((1, owner)) => {
                if body.shells.contains_key(face.shell) && face.shell != owner {
                    errors.push(ValidationError::BackPointerMismatch {
                        child: EntityId::Face(face_key),
                        stored: EntityId::Shell(face.shell),
                        owner: EntityId::Shell(owner),
                    });
                }
            }
            Some((owners, _)) => errors.push(ValidationError::MultiplyOwned {
                child: EntityId::Face(face_key),
                owners,
            }),
        }
    }
    for (loop_key, loop_) in body.loops.iter() {
        match loop_owners.get(loop_key).copied() {
            None => errors.push(ValidationError::OrphanEntity {
                entity: EntityId::Loop(loop_key),
            }),
            Some((1, owner)) => {
                if body.faces.contains_key(loop_.face) && loop_.face != owner {
                    errors.push(ValidationError::BackPointerMismatch {
                        child: EntityId::Loop(loop_key),
                        stored: EntityId::Face(loop_.face),
                        owner: EntityId::Face(owner),
                    });
                }
            }
            Some((owners, _)) => errors.push(ValidationError::MultiplyOwned {
                child: EntityId::Loop(loop_key),
                owners,
            }),
        }
    }

    // ------------------------------------------------------------------
    // Pass 8: orphan geometry, points → curves → surfaces.
    // ------------------------------------------------------------------
    for (key, _) in body.points.iter() {
        if point_refs.get(key).copied().unwrap_or(0) == 0 {
            errors.push(ValidationError::OrphanGeometry {
                geometry: GeomRef::Point(key),
            });
        }
    }
    for (key, _) in body.curves.iter() {
        if curve_refs.get(key).copied().unwrap_or(0) == 0 {
            errors.push(ValidationError::OrphanGeometry {
                geometry: GeomRef::Curve(key),
            });
        }
    }
    for (key, _) in body.surfaces.iter() {
        if surface_refs.get(key).copied().unwrap_or(0) == 0 {
            errors.push(ValidationError::OrphanGeometry {
                geometry: GeomRef::Surface(key),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Point3;
    use proptest::prelude::*;

    use super::*;
    use crate::body::Body;
    use crate::entity::{Face, Loop, Shell, Solid, Vertex};
    use crate::fixtures::{mvfs_state, ngon_pillow, pillow, prism, prov};
    use crate::geometry::SurfaceGeom;

    fn anchor() -> Point3<f64> {
        Point3::origin()
    }

    /// Adds a face bounded by one empty loop holding `vertex` to `shell`
    /// (the mvfs shape grafted onto an existing body), returning the new
    /// loop key. Used to manufacture empty-loop/vertex interactions.
    fn add_empty_loop_face(
        body: &mut Body<f64>,
        shell: crate::entity::ShellKey,
        vertex: VertexKey,
    ) -> LoopKey {
        let surface = body.add_surface(SurfaceGeom::Placeholder { anchor: anchor() });
        let lp = body.add_loop(
            Loop {
                boundary: LoopBoundary::Empty { vertex },
                face: FaceKey::default(),
            },
            prov(),
        );
        let f = body.add_face(
            Face {
                surface,
                outer: lp,
                rings: vec![],
                shell,
            },
            prov(),
        );
        body.get_loop_mut(lp).unwrap().face = f;
        body.get_shell_mut(shell).unwrap().faces.push(f);
        lp
    }

    // ------------------------------------------------------------------
    // Well-formed fixtures validate cleanly.
    // ------------------------------------------------------------------

    #[test]
    fn empty_body_validates_vacuously() {
        assert_eq!(validate(&Body::<f64>::new()), Ok(()));
    }

    #[test]
    fn digon_pillow_validates_cleanly() {
        // The minimal closed body: 2 vertices, 2 edges, 2 faces —
        // Euler–Poincaré v − e + f = 2 − 2 + 2 = 2 = 2(s − h) + r with
        // s = 1, h = r = 0 (a sphere). Replaces M0's single-face tiny().
        let t = pillow();
        assert_eq!(validate(&t.body), Ok(()));
        assert_eq!(t.body.vertices().count(), 2);
        assert_eq!(t.body.edges().count(), 2);
        assert_eq!(t.body.faces().count(), 2);
        assert_eq!(t.body.half_edges().count(), 4);
    }

    #[test]
    fn self_loop_digon_validates_cleanly() {
        // n = 1: one vertex, one self-loop edge whose two halves live in
        // different faces' one-half-edge loops. A legal (tier-1 and
        // tier-2) closed manifold body: v − e + f = 1 − 1 + 2 = 2.
        let t = ngon_pillow(1);
        assert_eq!(validate(&t.body), Ok(()));
        assert_eq!(t.body.vertices().count(), 1);
        assert_eq!(t.body.edges().count(), 1);
        assert_eq!(t.body.faces().count(), 2);
    }

    #[test]
    fn mvfs_state_validates_cleanly() {
        // The skeletal mvfs state (empty outer loop + lone vertex) is
        // tier-1-legal BY DESIGN: it is the state every Euler
        // construction starts from. Tier 2 (PR 5) bans it on finished
        // solids; tier 1 must accept it.
        let t = mvfs_state();
        assert_eq!(validate(&t.body), Ok(()));
    }

    #[test]
    fn prism_validates_cleanly() {
        let t = prism(4);
        assert_eq!(validate(&t.body), Ok(()));
        // v = 2n, e = 3n, f = n + 2: v − e + f = 8 − 12 + 6 = 2.
        assert_eq!(t.body.vertices().count(), 8);
        assert_eq!(t.body.edges().count(), 12);
        assert_eq!(t.body.faces().count(), 6);
    }

    // ------------------------------------------------------------------
    // Orbit-step direction: the ratified derivation, tested against a
    // hand-computed prism vertex (see the entity module docs).
    // ------------------------------------------------------------------

    #[test]
    fn orbit_step_next_mate_walks_clockwise_from_outside() {
        // At top-rim vertex t[i] of the prism (indices counterclockwise
        // viewed from above = from outside above the cap), the emanating
        // half-edges are: ht[i] (toward t[i+1]), s3[i] (down toward
        // u[i]), s2[i-1] (toward t[i-1]). Viewed from outside,
        // "toward t[i+1]" → "down" → "toward t[i-1]" is CLOCKWISE — and
        // that is exactly the next(mate(·)) order. (GWB states its orbit
        // idiom for the mirrored clockwise-loop convention; this test is
        // the transcription guard.)
        let t = prism(4);
        let i = 1;
        assert_eq!(
            t.body.vertex_orbit(t.ht[i]),
            Some(vec![t.ht[i], t.s3[i], t.s2[i - 1]])
        );
        // Bottom-rim vertex u[i]: s0[i] (toward u[i+1]), hb[i-1]
        // (toward u[i-1]... as start of the bottom cap's walk), s1[i-1]
        // (up toward t[i]).
        assert_eq!(
            t.body.vertex_orbit(t.s0[i]),
            Some(vec![t.s0[i], t.hb[i - 1], t.s1[i - 1]])
        );
        // The inverse step mate(prev(·)) walks the SAME orbit
        // counterclockwise: its first step from ht[i] is the CW orbit's
        // last member.
        let prev = t.body.get_half_edge(t.ht[i]).unwrap().prev;
        assert_eq!(prev, t.ht[i - 1]);
        assert_eq!(t.body.mate(prev), Some(t.s2[i - 1]));
    }

    #[test]
    fn orbit_steps_are_mutual_inverses_and_preserve_start() {
        let t = prism(3);
        for (he_key, he) in t.body.half_edges() {
            // cw(he) = next(mate(he)) starts at the same vertex...
            let mate = t.body.mate(he_key).unwrap();
            let cw = t.body.get_half_edge(mate).unwrap().next;
            assert_eq!(t.body.get_half_edge(cw).unwrap().start, he.start);
            // ...and ccw(cw(he)) = mate(prev(cw(he))) returns to he.
            let cw_prev = t.body.get_half_edge(cw).unwrap().prev;
            assert_eq!(t.body.mate(cw_prev), Some(he_key));
        }
    }

    // ------------------------------------------------------------------
    // One test per error variant, each asserting the EXACT error vector —
    // the corruption is surgical, so every reported error is reasoned
    // about in the test. Where a corruption necessarily breaks several
    // invariants at once (e.g. an overrun requires a torn next/prev
    // pair), the full expected vector is spelled out in documented pass
    // order — these double as report-order tests.
    //
    // Malformed bodies are built through the public raw API (including
    // the _mut patching accessors) plus direct pub(crate) arena access
    // for removals, which is exactly what unit tests are for.
    // ------------------------------------------------------------------

    #[test]
    fn dangling_topology_is_reported() {
        let mut t = pillow();
        // Mint a key that can never resolve again: insert, then remove
        // (the slot's version is bumped; even reuse cannot revive it).
        let dead = t.body.add_half_edge(
            crate::entity::HalfEdge {
                edge: t.edges[0],
                start: t.vertices[0],
                parent_loop: t.loop_a,
                next: t.hes_a[0],
                prev: t.hes_a[0],
            },
            prov(),
        );
        t.body.half_edges.remove(dead);
        t.body.get_half_edge_mut(t.hes_a[1]).unwrap().next = dead;
        // Loop A's walk breaks (silent: this very dangling is the cause),
        // e1's antiparallelism is underivable (skipped), v0's orbit
        // breaks (silent). Exactly one error.
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::DanglingTopology {
                from: EntityId::HalfEdge(t.hes_a[1]),
                to: EntityId::HalfEdge(dead),
            }])
        );
    }

    #[test]
    fn dangling_empty_loop_vertex_is_reported() {
        let mut t = mvfs_state();
        t.body.vertices.remove(t.vertex);
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::DanglingTopology {
                    from: EntityId::Loop(t.lone_loop),
                    to: EntityId::Vertex(t.vertex),
                },
                ValidationError::OrphanGeometry {
                    geometry: GeomRef::Point(t.point),
                },
            ])
        );
    }

    #[test]
    fn dangling_geometry_is_reported() {
        let mut t = pillow();
        // Repoint an existing vertex at a removed point: the vertex's
        // reference dangles, and the abandoned point becomes an orphan.
        let dead = t.body.add_point(anchor());
        t.body.points.remove(dead);
        t.body.get_vertex_mut(t.vertices[0]).unwrap().point = dead;
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::DanglingGeometry {
                    from: EntityId::Vertex(t.vertices[0]),
                    to: GeomRef::Point(dead),
                },
                ValidationError::OrphanGeometry {
                    geometry: GeomRef::Point(t.points[0]),
                },
            ])
        );
    }

    #[test]
    fn dangling_back_pointer_reports_once() {
        // A dangling back-pointer is a pass-1 error only: pass 7's
        // mismatch comparison is gated on a RESOLVING stored key, so the
        // defect is reported exactly once.
        let mut t = mvfs_state();
        let dead = t.body.add_shell(
            Shell {
                faces: vec![],
                solid: t.solid,
            },
            prov(),
        );
        t.body.shells.remove(dead);
        t.body.get_face_mut(t.face).unwrap().shell = dead;
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::DanglingTopology {
                from: EntityId::Face(t.face),
                to: EntityId::Shell(dead),
            }])
        );
    }

    #[test]
    fn next_prev_mismatch_is_reported() {
        let mut t = pillow();
        // Tear prev only: a1's true predecessor (via next) is a0, so the
        // check fires for a0 ("my successor does not point back") and
        // for nothing else — next itself is intact, so cycles and orbits
        // still close.
        let a1 = t.hes_a[1];
        t.body.get_half_edge_mut(a1).unwrap().prev = a1;
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::NextPrevMismatch {
                half_edge: t.hes_a[0],
            }])
        );
    }

    #[test]
    fn loop_cycle_overrun_is_reported() {
        let mut t = pillow();
        // Point a0's next into loop B's cycle. A pure overrun is
        // impossible: while next/prev stay mutual inverses next is a
        // permutation and every walk closes — so the expected vector
        // necessarily pairs the overrun with the NextPrevMismatch that
        // tore the permutation, plus the orbit overrun at v1 whose orbit
        // permutation is equally torn. Documented pass order: 2a, 2b, 6.
        t.body.get_half_edge_mut(t.hes_a[0]).unwrap().next = t.hes_b[0];
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::NextPrevMismatch {
                    half_edge: t.hes_a[0],
                },
                ValidationError::LoopCycleOverrun { loop_: t.loop_a },
                ValidationError::VertexOrbitOverrun {
                    vertex: t.vertices[1],
                },
            ])
        );
    }

    #[test]
    fn parent_loop_mismatch_is_reported() {
        let mut t = pillow();
        // b0 claims loop A as parent while sitting in loop B's cycle:
        // loop B's walk reports the mismatch, and b0 is simultaneously
        // unreachable from its claimed parent (whose cycle closed
        // without it) — two true statements, two errors.
        t.body.get_half_edge_mut(t.hes_b[0]).unwrap().parent_loop = t.loop_a;
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::ParentLoopMismatch {
                    half_edge: t.hes_b[0],
                    owner: t.loop_b,
                },
                ValidationError::UnreachableHalfEdge {
                    half_edge: t.hes_b[0],
                },
            ])
        );
    }

    #[test]
    fn half_edge_claiming_an_empty_parent_is_unreachable() {
        let mut t = pillow();
        // A fresh lone vertex in an empty loop (legal), then a1 claims
        // that empty loop as its parent: an empty loop reaches nothing.
        let p2 = t.body.add_point(anchor());
        let v2 = t.body.add_vertex(
            Vertex {
                point: p2,
                emanating: None,
            },
            prov(),
        );
        let empty = add_empty_loop_face(&mut t.body, t.shell, v2);
        t.body.get_half_edge_mut(t.hes_a[1]).unwrap().parent_loop = empty;
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::ParentLoopMismatch {
                    half_edge: t.hes_a[1],
                    owner: t.loop_a,
                },
                ValidationError::UnreachableHalfEdge {
                    half_edge: t.hes_a[1],
                },
            ])
        );
    }

    #[test]
    fn edge_halves_identical_is_reported() {
        let mut t = pillow();
        // e0 claims a0 in both slots: a0 is now claimed twice overall
        // and b0 (e0's real minus half) by nobody. Antiparallelism is
        // gated on distinct halves (skipped); both orbits break at the
        // mate of an unclaimed half-edge (silent — the claim errors are
        // the cause). Pass order: 3 per-edge, then 3 claim counts in
        // half-edge slot order.
        t.body.get_edge_mut(t.edges[0]).unwrap().he_minus = t.hes_a[0];
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::EdgeHalvesIdentical { edge: t.edges[0] },
                ValidationError::HalfEdgeMultiplyClaimed {
                    half_edge: t.hes_a[0],
                    claims: 2,
                },
                ValidationError::HalfEdgeUnclaimed {
                    half_edge: t.hes_b[0],
                },
            ])
        );
    }

    #[test]
    fn edge_slot_backpointer_mismatch_is_reported() {
        let mut t = pillow();
        // e0's minus slot claims b1 (whose .edge is e1): back-pointer
        // mismatch; b0 goes unclaimed, b1 doubly claimed; and e0's
        // halves (a0, b1) both run v0 → v1, so antiparallelism genuinely
        // fails too. Orbits break at b0's missing mate (silent).
        t.body.get_edge_mut(t.edges[0]).unwrap().he_minus = t.hes_b[1];
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::EdgeSlotBackpointerMismatch {
                    edge: t.edges[0],
                    half_edge: t.hes_b[1],
                },
                ValidationError::HalfEdgeUnclaimed {
                    half_edge: t.hes_b[0],
                },
                ValidationError::HalfEdgeMultiplyClaimed {
                    half_edge: t.hes_b[1],
                    claims: 2,
                },
                ValidationError::EdgeNotAntiparallel { edge: t.edges[0] },
            ])
        );
    }

    #[test]
    fn edge_not_antiparallel_is_reported() {
        let mut t = pillow();
        // Swap loop B's start vertices: both edges' halves now run the
        // same way (parallel, not antiparallel), and both orbits close
        // over a foreign member — the b half-edge that now starts at the
        // OTHER vertex. Incidence counts are unchanged (2 per vertex),
        // so no anchoring errors. Pass order: 4 (edges), then 6
        // (vertices, walk order).
        let v0 = t.vertices[0];
        let v1 = t.vertices[1];
        t.body.get_half_edge_mut(t.hes_b[0]).unwrap().start = v0;
        t.body.get_half_edge_mut(t.hes_b[1]).unwrap().start = v1;
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::EdgeNotAntiparallel { edge: t.edges[0] },
                ValidationError::EdgeNotAntiparallel { edge: t.edges[1] },
                ValidationError::OrbitForeignMember {
                    vertex: v0,
                    half_edge: t.hes_b[1],
                },
                ValidationError::OrbitForeignMember {
                    vertex: v1,
                    half_edge: t.hes_b[0],
                },
            ])
        );
    }

    #[test]
    fn emanating_start_mismatch_is_reported() {
        let mut t = pillow();
        // v0's emanating points at a half-edge starting at v1. The orbit
        // check is gated on a matching start (skipped for v0), so the
        // mismatch is the single report.
        t.body.get_vertex_mut(t.vertices[0]).unwrap().emanating = Some(t.hes_a[1]);
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::EmanatingStartMismatch {
                vertex: t.vertices[0],
                emanating: t.hes_a[1],
            }])
        );
    }

    #[test]
    fn empty_loop_vertex_with_emanating_is_reported() {
        let mut t = pillow();
        // An empty loop claiming v0, which has half-edges: a lone vertex
        // must have none.
        add_empty_loop_face(&mut t.body, t.shell, t.vertices[0]);
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::EmptyLoopVertexWithEmanating {
                vertex: t.vertices[0],
                empty_loops: 1,
            }])
        );
    }

    #[test]
    fn lone_vertex_with_incidence_is_reported() {
        let mut t = pillow();
        // v0 claims to be lone (emanating: None) while two half-edges
        // start at it. The orbit check needs an emanating (skipped).
        t.body.get_vertex_mut(t.vertices[0]).unwrap().emanating = None;
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::LoneVertexWithIncidence {
                vertex: t.vertices[0],
                incident: 2,
            }])
        );
    }

    #[test]
    fn orphan_vertex_is_reported() {
        let mut t = pillow();
        // A vertex no half-edge starts at and no empty loop holds — the
        // M0 orphan-vertex rule restated in half-edge terms. Its point
        // is NOT an orphan: geometry referenced by an orphan entity is
        // still referenced.
        let p = t.body.add_point(anchor());
        let v = t.body.add_vertex(
            Vertex {
                point: p,
                emanating: None,
            },
            prov(),
        );
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::OrphanEntity {
                entity: EntityId::Vertex(v),
            }])
        );
    }

    #[test]
    fn orphaned_empty_loop_vertex_is_reported() {
        let mut t = mvfs_state();
        // Repoint the empty loop at a fresh lone vertex: the original
        // lone vertex loses its only anchor (no half-edges start at it
        // and no empty loop holds it any more) — the mvfs-state
        // counterpart of orphaning a vertex.
        let p2 = t.body.add_point(anchor());
        let v2 = t.body.add_vertex(
            Vertex {
                point: p2,
                emanating: None,
            },
            prov(),
        );
        t.body.get_loop_mut(t.lone_loop).unwrap().boundary = LoopBoundary::Empty { vertex: v2 };
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::OrphanEntity {
                entity: EntityId::Vertex(t.vertex),
            }])
        );
    }

    #[test]
    fn vertex_in_two_empty_loops_is_multiply_owned() {
        let mut t = mvfs_state();
        // A second empty loop claiming the same lone vertex: empty-loop
        // ownership of a vertex is exclusive.
        add_empty_loop_face(&mut t.body, t.shell, t.vertex);
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::MultiplyOwned {
                child: EntityId::Vertex(t.vertex),
                owners: 2,
            }])
        );
    }

    #[test]
    fn split_vertex_orbit_is_reported() {
        // The classic non-manifold "bowtie": glue a second, self-loop
        // digon pillow onto v0. Every local invariant holds — cycles
        // close, mates pair, antiparallelism holds, emanating matches —
        // but v0's incident half-edges fall into TWO orbits, and the
        // orbit-closure check is exactly what catches it.
        let mut t = pillow();
        let v0 = t.vertices[0];
        let curve = t
            .body
            .add_curve(crate::geometry::CurveGeom::Placeholder { anchor: anchor() });
        let e2 = t.body.add_edge(
            crate::entity::Edge {
                he_plus: HalfEdgeKey::default(),
                he_minus: HalfEdgeKey::default(),
                curve,
            },
            prov(),
        );
        let self_loop_face = |body: &mut Body<f64>| {
            let he = body.add_half_edge(
                crate::entity::HalfEdge {
                    edge: e2,
                    start: v0,
                    parent_loop: LoopKey::default(),
                    next: HalfEdgeKey::default(),
                    prev: HalfEdgeKey::default(),
                },
                prov(),
            );
            let surface = body.add_surface(SurfaceGeom::Placeholder { anchor: anchor() });
            let lp = body.add_loop(
                Loop {
                    boundary: LoopBoundary::Cycle { first: he },
                    face: FaceKey::default(),
                },
                prov(),
            );
            let f = body.add_face(
                Face {
                    surface,
                    outer: lp,
                    rings: vec![],
                    shell: t.shell,
                },
                prov(),
            );
            body.get_loop_mut(lp).unwrap().face = f;
            body.get_shell_mut(t.shell).unwrap().faces.push(f);
            let h = body.get_half_edge_mut(he).unwrap();
            h.parent_loop = lp;
            h.next = he;
            h.prev = he;
            he
        };
        let ap = self_loop_face(&mut t.body);
        let bp = self_loop_face(&mut t.body);
        let e = t.body.get_edge_mut(e2).unwrap();
        e.he_plus = ap;
        e.he_minus = bp;
        // v0 now has 4 incident half-edges but its orbit (from the
        // pillow's a0) closes over 2.
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::SplitVertexOrbit {
                vertex: v0,
                orbit: 2,
                incident: 4,
            }])
        );
    }

    #[test]
    fn outer_listed_as_ring_is_reported() {
        let mut t = pillow();
        // outer ∈ rings is both the designation error and a double
        // ownership (the loop is counted once as outer, once as ring) —
        // two true statements, two errors, in pass-7 order.
        t.body.get_face_mut(t.face_a).unwrap().rings = vec![t.loop_a];
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::OuterListedAsRing { face: t.face_a },
                ValidationError::MultiplyOwned {
                    child: EntityId::Loop(t.loop_a),
                    owners: 2,
                },
            ])
        );
    }

    #[test]
    fn loop_back_pointer_mismatch_is_reported() {
        let mut t = pillow();
        t.body.get_loop_mut(t.loop_b).unwrap().face = t.face_a;
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::BackPointerMismatch {
                child: EntityId::Loop(t.loop_b),
                stored: EntityId::Face(t.face_a),
                owner: EntityId::Face(t.face_b),
            }])
        );
    }

    #[test]
    fn face_and_shell_back_pointer_mismatches_are_reported() {
        let mut t = pillow();
        // A second (empty but owned) shell+solid to point at.
        let solid2 = t.body.add_solid(Solid { shells: vec![] }, prov());
        let shell2 = t.body.add_shell(
            Shell {
                faces: vec![],
                solid: solid2,
            },
            prov(),
        );
        t.body.get_solid_mut(solid2).unwrap().shells.push(shell2);
        t.body.get_shell_mut(t.shell).unwrap().solid = solid2;
        t.body.get_face_mut(t.face_b).unwrap().shell = shell2;
        // Pass 7 order: shells before faces.
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::BackPointerMismatch {
                    child: EntityId::Shell(t.shell),
                    stored: EntityId::Solid(solid2),
                    owner: EntityId::Solid(t.solid),
                },
                ValidationError::BackPointerMismatch {
                    child: EntityId::Face(t.face_b),
                    stored: EntityId::Shell(shell2),
                    owner: EntityId::Shell(t.shell),
                },
            ])
        );
    }

    #[test]
    fn orphan_shell_is_reported() {
        let mut t = pillow();
        let sh2 = t.body.add_shell(
            Shell {
                faces: vec![],
                solid: t.solid,
            },
            prov(),
        );
        // sh2 is not in any solid's shell list (ownership counts, not
        // back-pointers, define anchoring — and with zero owners the
        // back-pointer comparison is skipped).
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::OrphanEntity {
                entity: EntityId::Shell(sh2),
            }])
        );
    }

    #[test]
    fn multiply_owned_shell_is_reported() {
        let mut t = pillow();
        let sh = t.shell;
        t.body.get_solid_mut(t.solid).unwrap().shells.push(sh);
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::MultiplyOwned {
                child: EntityId::Shell(sh),
                owners: 2,
            }])
        );
    }

    #[test]
    fn orphan_geometry_is_reported_for_all_three_arenas() {
        let mut t = pillow();
        let p = t.body.add_point(anchor());
        let c = t
            .body
            .add_curve(crate::geometry::CurveGeom::Placeholder { anchor: anchor() });
        let s = t
            .body
            .add_surface(SurfaceGeom::Placeholder { anchor: anchor() });
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::OrphanGeometry {
                    geometry: GeomRef::Point(p),
                },
                ValidationError::OrphanGeometry {
                    geometry: GeomRef::Curve(c),
                },
                ValidationError::OrphanGeometry {
                    geometry: GeomRef::Surface(s),
                },
            ])
        );
    }

    #[test]
    fn errors_display_without_panicking() {
        // One instance of every variant (closed enum — a new variant
        // must be added here, which the exhaustive-Display match already
        // forces at compile time).
        let t = pillow();
        let he = t.hes_a[0];
        let v = t.vertices[0];
        let e = t.edges[0];
        let all: Vec<ValidationError> = vec![
            ValidationError::DanglingTopology {
                from: EntityId::Solid(t.solid),
                to: EntityId::Shell(t.shell),
            },
            ValidationError::DanglingGeometry {
                from: EntityId::Vertex(v),
                to: GeomRef::Point(t.points[0]),
            },
            ValidationError::NextPrevMismatch { half_edge: he },
            ValidationError::LoopCycleOverrun { loop_: t.loop_a },
            ValidationError::ParentLoopMismatch {
                half_edge: he,
                owner: t.loop_a,
            },
            ValidationError::UnreachableHalfEdge { half_edge: he },
            ValidationError::EdgeHalvesIdentical { edge: e },
            ValidationError::EdgeSlotBackpointerMismatch {
                edge: e,
                half_edge: he,
            },
            ValidationError::HalfEdgeUnclaimed { half_edge: he },
            ValidationError::HalfEdgeMultiplyClaimed {
                half_edge: he,
                claims: 2,
            },
            ValidationError::EdgeNotAntiparallel { edge: e },
            ValidationError::EmanatingStartMismatch {
                vertex: v,
                emanating: he,
            },
            ValidationError::EmptyLoopVertexWithEmanating {
                vertex: v,
                empty_loops: 1,
            },
            ValidationError::LoneVertexWithIncidence {
                vertex: v,
                incident: 2,
            },
            ValidationError::VertexOrbitOverrun { vertex: v },
            ValidationError::OrbitForeignMember {
                vertex: v,
                half_edge: he,
            },
            ValidationError::SplitVertexOrbit {
                vertex: v,
                orbit: 2,
                incident: 4,
            },
            ValidationError::OuterListedAsRing { face: t.face_a },
            ValidationError::BackPointerMismatch {
                child: EntityId::Loop(t.loop_a),
                stored: EntityId::Face(t.face_a),
                owner: EntityId::Face(t.face_b),
            },
            ValidationError::OrphanEntity {
                entity: EntityId::Vertex(v),
            },
            ValidationError::MultiplyOwned {
                child: EntityId::Shell(t.shell),
                owners: 2,
            },
            ValidationError::OrphanGeometry {
                geometry: GeomRef::Point(t.points[0]),
            },
        ];
        for err in &all {
            // Display and Error are wired up; content is human-oriented.
            assert!(!err.to_string().is_empty());
            let _: &dyn std::error::Error = err;
        }
    }

    // ------------------------------------------------------------------
    // Property tests: both fixture families validate cleanly at every
    // size, and clones stay independent (mutating the clone leaves the
    // original clean). Strategies are deterministic (proptest's seeded
    // RNG; no ambient state).
    // ------------------------------------------------------------------

    proptest! {
        #[test]
        fn ngon_pillows_validate_cleanly(n in 1usize..=8) {
            let t = ngon_pillow(n);
            prop_assert_eq!(validate(&t.body), Ok(()));
            prop_assert_eq!(t.body.vertices().count(), n);
            prop_assert_eq!(t.body.edges().count(), n);
            prop_assert_eq!(t.body.faces().count(), 2);
            // Clone independence, propertywise.
            let mut cloned = t.body.clone();
            prop_assert_eq!(validate(&cloned), Ok(()));
            cloned.add_point(anchor()); // now malformed (orphan point)
            prop_assert!(validate(&cloned).is_err());
            prop_assert_eq!(validate(&t.body), Ok(()));
        }

        #[test]
        fn prisms_validate_cleanly(n in 2usize..=8) {
            let t = prism(n);
            prop_assert_eq!(validate(&t.body), Ok(()));
            prop_assert_eq!(t.body.vertices().count(), 2 * n);
            prop_assert_eq!(t.body.edges().count(), 3 * n);
            prop_assert_eq!(t.body.faces().count(), n + 2);
            prop_assert_eq!(t.body.half_edges().count(), 6 * n);
            // Every vertex has valence 3: its orbit closes over 3
            // half-edges (nontrivial orbits are the point of this
            // family).
            for (_, vertex) in t.body.vertices() {
                let orbit = t.body.vertex_orbit(vertex.emanating.unwrap()).unwrap();
                prop_assert_eq!(orbit.len(), 3);
            }
            // Clone independence.
            let mut cloned = t.body.clone();
            prop_assert_eq!(validate(&cloned), Ok(()));
            cloned.add_point(anchor());
            prop_assert!(validate(&cloned).is_err());
            prop_assert_eq!(validate(&t.body), Ok(()));
        }
    }
}
