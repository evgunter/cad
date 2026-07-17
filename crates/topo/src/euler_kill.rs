//! Kill-direction Euler duals — [`Body::kvfs`], [`Body::kev`],
//! [`Body::kef`] — and the ring-promotion inverse [`Body::mfkrh`]
//! (M1 PR 4).
//!
//! These complete the ten-operator catalog (Mäntylä ch. 9): every
//! make-direction operator now has its exact inverse in-tree, which is
//! what makes the make/kill roundtrip property tests possible (this PR's
//! second half) and what M3's booleans and splitting will consume. The
//! semantics are **forced**: each operator is the exact inverse of its
//! dual, derived from the duals' documented surgeries rather than
//! designed fresh. They share the operator contracts of [`crate::euler`]
//! (atomic typed-error preconditions, deterministic minting, debug
//! postconditions) and the kill-side duties of [`crate::euler_ring`]:
//!
//! - **Kill hygiene.** A killed entity's arena slot AND its D5
//!   provenance entry are removed together; killed keys in the result
//!   structs are **dead** — returned for the caller's records only.
//! - **Geometry hygiene.** Orphaned curves/surfaces/points are reaped
//!   ([`Body::remove_curve_if_orphaned`] /
//!   [`Body::remove_surface_if_orphaned`] /
//!   [`Body::remove_point_if_orphaned`] — deterministic full-arena
//!   scans). With M1's per-entity minting the killed vertex's point and
//!   the killed edge's curve are always orphaned in practice, but the
//!   scan is the rule; the killed face's surface is usually *shared*
//!   (every `mef` face shares its parent's), so `kef`/`kvfs` usually
//!   reap no surface.
//! - **Survivor re-anchoring**, unconditional (the PR 3 pattern): a
//!   surviving loop's `Cycle::first` and a surviving vertex's
//!   `emanating` are overwritten by documented deterministic rules,
//!   never patched only-when-dangling. Which member they name is
//!   documented as arbitrary ([`crate::LoopBoundary::Cycle`],
//!   [`crate::Vertex::emanating`]), so a rule that does not restore the
//!   pre-make anchor is still an exact inverse *of the body*; the
//!   isomorphism oracle used by the roundtrip tests normalizes these
//!   anchors accordingly.
//!
//! Kills and lineage follow PR 3's per-arena replay semantics
//! ([`crate::euler_ring`] module docs): identical histories replay
//! deep-identically; a balanced make∘kill pair leaves survivor keys
//! untouched; a kill∘make pair re-mints in recycled slots with bumped
//! generations, per-arena.
//!
//! # `kvfs` — inverse of `mvfs`
//!
//! [`Body::kvfs`] destroys a solid that is EXACTLY the skeletal `mvfs`
//! state: one shell, one face, no rings, an [`Empty`] outer loop holding
//! the lone vertex. Anything else is a typed error naming the failing
//! arity precisely. Euler vector `(v −1, e 0, f −1, h 0, r 0, s −1)`.
//!
//! # `kev` — inverse of `mev` (the fan merge)
//!
//! [`Body::kev`] kills `he`'s edge and **`end(he)`** — the far vertex —
//! merging that vertex's remaining fan back onto `start(he)`. "The given
//! half-edge's side is the affected thing", uniform with `mef`/`kemr`:
//! the vertex `he` *points at* dies. Euler vector
//! `(v −1, e −1, f 0, h 0, r 0, s 0)`.
//!
//! Derivation (exact inverse of the `mev` fan surgery, [`crate::MevSite`]):
//! `mev(Fan { he1, he2 })` at `v` minted `he_plus`/`he_minus`, spliced
//! them immediately before `he1`/`he2`, and reassigned the clockwise
//! orbit run `[he1 .. he2)` to the new vertex `w`. With `he = he_plus`
//! and `m = mate(he) = he_minus`, the post-state satisfies
//! `next(he) = he1` and `next(m) = he2`, and `w`'s clockwise orbit is
//! `[m, run…]` (the first orbit step from `m` is
//! `next(mate(m)) = next(he) = he1`). So the inverse is forced:
//!
//! - **Fan merge**: every half-edge of `w`'s orbit except `m` — walked
//!   clockwise from `m`, i.e. `vertex_orbit(m)[1..]` — is reassigned to
//!   start at `v` (this is the run move, reversed; the whole remaining
//!   fan moves, there is no direction choice left to make).
//! - **Unsplice**: `link(prev(he), next(he))` and
//!   `link(prev(m), next(m))` — exactly undoing `mev`'s four link
//!   writes. The two half-edges may lie in one loop or two (mev splices
//!   across two loops when `he1`/`he2` do); the unsplice is loop-count
//!   preserving either way.
//!
//! ```text
//!         before                          after
//!      \  |  /                        \  |  /
//!       \ | /  ← fan (orbit of m       \ | /
//!         w       minus m itself)        v   ← merged fan
//!    m ↓  ↑ he ← killed edge            / \
//!         v                          he2   (rest of v's fan)
//!        / \
//!     he2   (rest of v's fan)
//! ```
//!
//! Degenerate cases, each the inverse of a `mev` site case:
//!
//! - **Strut kill** (`next(he) = m`): `w` has no other edges (its orbit
//!   is exactly `[m]` — `next(mate(m)) = next(he) = m` closes it), so
//!   the fan is empty and the kill is pure removal:
//!   `… → prev(he) → he → m → x → …` becomes `… → prev(he) → x → …`.
//!   Inverse of `MevSite::Fan { he1 == he2 }`.
//! - **Segment kill** (the loop is the 2-cycle `[he, m]`): removing both
//!   halves leaves the loop with no members — it becomes
//!   [`Empty`] at `v` (the survivor, matching `MevSite::Lone`'s
//!   old vertex; `Lone` grew exactly this loop). Inverse of
//!   `MevSite::Lone`.
//! - The mirror adjacency (`next(m) = he`) is `v`-valence-1: `v`'s only
//!   edge was the killed one and the whole fan migrates to it. Handled
//!   by the same unsplice, not a separate surgery case — but note it is
//!   the one `kev` site that undoes no single `mev` (see the re-make
//!   taxonomy below).
//!
//! Re-anchoring rules (unconditional): the loop of `he` re-anchors at
//! the first survivor after `he` in `next` order (`next(he)`, or
//! `next(m)` when `next(he) = m`); the loop of `m` — when distinct — at
//! `next(m)`; when both halves share one loop the `next(m)`-side write
//! is last and wins (deterministic). `v.emanating` becomes the first
//! merged-fan member (= `next(he)`, which starts at `w` pre-kill), else
//! the strut case's `next(m)` (which starts at `v`), else `None`
//! (segment kill — `v` is lone again).
//!
//! # `kef` — inverse of `mef` (the loop splice)
//!
//! [`Body::kef`] kills `he`'s edge and **the face of `he`'s loop** —
//! `he`'s side dies, the exact inverse of `mef`'s "`he1`'s side becomes
//! the new face" (a `mef` is undone by `kef(created.he_minus)`, the half
//! it placed in the new loop). The dying loop's remnant merges into the
//! mate's loop. The edge must border two DISTINCT faces and the dying
//! face must be RING-FREE (move rings off with [`Body::ring_move`]
//! first, mirroring `kfmrh` — or kill the mate's side if that one is
//! bare). Euler vector `(v 0, e −1, f −1, h 0, r 0, s 0)`.
//!
//! Derivation (exact inverse of the `mef` tail swap,
//! [`crate::MefSite`]): `mef(Chords { he1, he2 })` spliced
//! `he_minus` before `he1` and `he_plus` before `he2`, moving
//! `[he1 .. he2)` into the new loop. With `he = he_minus` and
//! `m = he_plus`, the post-state satisfies `next(he) = he1`,
//! `prev(he) = ` old `prev(he2)`, `next(m) = he2`, `prev(m) = ` old
//! `prev(he1)`. Undoing the four link writes gives the general splice:
//! `link(prev(m), next(he))` and `link(prev(he), next(m))` — the dying
//! loop's remnant is stitched into the mate's loop across the gap the
//! killed halves leave:
//!
//! ```text
//!          before                            after
//!    ┌── next(he) ──┐  DYING LOOP
//!    │              ↓  (he's side)     ┌── next(he) ──┐
//!    he ← killed    │                  │              ↓
//!    ↑              │                  │  (one loop)  │
//!    └── prev(he) ──┘                  ↑              │
//!    ┌── next(m) ───┐  MATE'S LOOP     └─ prev(he) →  │
//!    │              ↓  (survives)         next(m) ····┘
//!    m ← killed     │
//!    ↑              │
//!    └── prev(m) ───┘
//! ```
//!
//! Degenerate cases, each the inverse of a `mef` site case (`he` is
//! always the half in the dying loop):
//!
//! - **Dying loop is `[he]` alone** (`next(he) = he`): the remnant is
//!   empty; the mate is simply unspliced from its loop
//!   (`link(prev(m), next(m))`). Inverse of
//!   `MefSite::Chords { he1 == he2 }` applied at `created.he_minus` —
//!   the one-edge circular face dies.
//! - **Mate's loop is `[m]` alone** (`next(m) = m`, which forces the
//!   edge to be a self-loop): the remnant closes into itself
//!   (`link(prev(he), next(he))`) and becomes the surviving loop's
//!   whole cycle. This is `kef` on the OTHER half of the same
//!   configuration (killing the big side of a circular-edge split);
//!   the one-op re-make is a `Chords { he1 == he2 }` from the surviving
//!   side — exact up to isomorphism iff the surviving singleton is the
//!   outer of a ring-free face (the only shape `mef` itself creates —
//!   face identities swap, which the oracle does not track). When the
//!   survivor is a ring, or its face carries rings, the re-split lands
//!   the big loop on the wrong side of the ring distribution and no
//!   single op restores the original (see the taxonomy below).
//! - **Both alone** (`[he]` and `[m]`, a self-loop edge): the surviving
//!   loop becomes [`Empty`] at the one vertex, `emanating`
//!   `None`. Inverse of `MefSite::Lone`.
//!
//! `kef` allows `start(he) == end(he)`: self-loop edges are exactly its
//! `Lone`/circular-inverse territory (the *kev* precondition is the one
//! that demands distinct endpoints).
//!
//! Re-anchoring rules (unconditional): the surviving loop re-anchors at
//! `next(m)` when it survives the kill, else `next(he)` (both dead only
//! in the `Lone` inverse, which anchors the [`Empty`] state
//! instead). Emanating: `start(he)` gets `next(m)` — which starts at
//! `start(he)` by antiparallelism — falling back to `next(he)`, else
//! `None`; `start(m)` symmetrically gets `next(he)` falling back to
//! `next(m)`, else `None`; when the endpoints coincide the
//! `start(m)`-side write is last and wins (kemr's precedent).
//!
//! # `mfkrh` — inverse of `kfmrh`
//!
//! [`Body::mfkrh`] promotes a ring — [`Cycle`](crate::LoopBoundary::Cycle)
//! OR [`Empty`] — to the outer loop of a NEW face in the same
//! shell. Empty-outer faces hereby become **operator-reachable** (as
//! PR 3 predicted): promoting an empty ring yields a face whose outer
//! loop is an empty loop, the `mvfs`-face shape inside a larger body.
//! Euler vector `(v 0, e 0, f +1, h −1, r −1, s 0)`.
//!
//! The new face mints a fresh `Placeholder` surface. **Exact key or
//! surface restoration is NOT promised** for a `kfmrh ∘ mfkrh` pair:
//! `kfmrh` may have reaped the original surface (or, more commonly, the
//! killed face *shared* a surviving surface, so there is nothing to
//! restore a reference to), and `mfkrh` has no parent face to share
//! from — minting is the only sound choice. The surface anchor is
//! deterministic under replay (D9): the promoted ring's lone vertex's
//! point (empty ring) or the point of `start(Cycle::first)` (cycle
//! ring). `Cycle::first` is a representation-internal anchor, so the
//! ballast coordinates are history-dependent — they carry no geometric
//! meaning (M1 placeholder policy), and the isomorphism oracle ignores
//! surface payloads for exactly this reason.
//!
//! # The make ↔ kill inverse map (all five pairs, for the record)
//!
//! | make | exact inverse | degenerate cases |
//! |---|---|---|
//! | `mvfs` | `kvfs(created.solid)` | — |
//! | `mev(site)` | `kev(created.he_plus)` | `Fan{he1==he2}` ↔ strut kill; `Lone` ↔ segment kill |
//! | `mef(site)` | `kef(created.he_minus)` | `Chords{he1==he2}` ↔ dying-loop-alone; `Lone` ↔ both-alone |
//! | `mekr(site)` | `kemr(created.he_plus, created.he_minus)` | per-site, see [`crate::euler_ring`] |
//! | `kfmrh(f1, f2)` | `mfkrh(result.ring)` | empty-outer `f2` ↔ empty-ring promotion |
//!
//! and in the kill∘make direction the re-make sites are derived from the
//! pre-kill neighborhood (`kev` ↔ `mev(Fan{next(he), next(mate(he))})`
//! etc.). The make∘kill direction is exact for every site; the
//! kill∘make direction is exact for every site EXCEPT two subcases with
//! no single-op re-make (the roundtrip property tests skip exactly
//! these — precise statement and proof sketch in the seqgen
//! test-support module):
//!
//! - `kev` from the valence-1 side of an edge whose far vertex carries
//!   a fan (the mirror adjacency): the full-fan `mev` run is
//!   inexpressible under the ratified empty-run convention, and the
//!   strut re-make puts the fan at the wrong coordinates (given
//!   distinct vertex coordinates — coordinate-coincident endpoints
//!   would collapse the distinction, inside the oracle's documented
//!   twin blind spot).
//! - `kef` mate-alone where the surviving singleton loop is a ring or
//!   its face carries rings (see the degenerate-case list above); the
//!   bare-outer subcase IS one-op re-makeable and is exercised, not
//!   skipped.
//!
//! # Example: grow and ungrow
//!
//! ```
//! use geom_core::Point3;
//! use topo::{Body, MevSite};
//!
//! # fn run() -> Result<(), topo::EulerOpError> {
//! let mut body = Body::<f64>::new();
//! let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0))?;
//! let seg = body.mev(
//!     MevSite::Lone { r#loop: seed.r#loop },
//!     Point3::new(1.0, 0.0, 0.0),
//! )?;
//! // kev(he_plus) kills end(he_plus) — the far vertex — and the edge:
//! // the loop is empty again, holding the seed vertex.
//! let kill = body.kev(seg.he_plus)?;
//! assert_eq!(kill.killed_vertex, seg.vertex);
//! assert_eq!(
//!     body.get_loop(seed.r#loop).unwrap().boundary,
//!     topo::LoopBoundary::Empty { vertex: seed.vertex },
//! );
//! // And kvfs takes the skeletal remnant back to nothing.
//! let end = body.kvfs(seed.solid)?;
//! assert_eq!(end.killed_vertex, seed.vertex);
//! assert_eq!(body.solids().count(), 0);
//! assert_eq!(body.vertices().count(), 0);
//! assert_eq!(body.points().count(), 0);
//! # Ok(()) }
//! # run().unwrap();
//! ```
//!
//! [`Empty`]: crate::LoopBoundary::Empty

use geom_core::Real;

use crate::body::Body;
use crate::entity::{
    EdgeKey, EntityId, Face, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, ShellKey, SolidKey,
    VertexKey,
};
use crate::euler::EulerOpError;
use crate::geometry::{CurveKey, PointKey, SurfaceGeom, SurfaceKey};
use crate::provenance::Provenance;

/// The outcome of one [`Body::kvfs`] call: five dead topology keys plus
/// the reaped geometry.
///
/// Every `killed_*` key is **dead** — it no longer resolves; the keys
/// are returned for the caller's records (e.g. an operation log), not
/// for lookup.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KvfsResult {
    /// The killed solid (dead key).
    pub killed_solid: SolidKey,
    /// The killed shell (dead key).
    pub killed_shell: ShellKey,
    /// The killed face (dead key).
    pub killed_face: FaceKey,
    /// The killed (empty) outer loop (dead key).
    pub killed_loop: LoopKey,
    /// The killed lone vertex (dead key).
    pub killed_vertex: VertexKey,
    /// The face's surface (dead key), if killing the face orphaned it
    /// and it was removed; `None` if another face still references it
    /// (possible only through surface sharing, which no M1 construction
    /// of a *skeletal* solid produces — the scan is the rule).
    pub killed_surface: Option<SurfaceKey>,
    /// The vertex's point (dead key), if killing the vertex orphaned it
    /// and it was removed; `None` if another vertex still references it
    /// (never, with M1's per-vertex minting — the scan is the rule).
    pub killed_point: Option<PointKey>,
}

/// The outcome of one [`Body::kev`] call: the killed edge complex, the
/// killed far vertex, and the reaped geometry.
///
/// Every `killed_*` key is **dead** (see [`KvfsResult`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KevResult {
    /// The killed edge (dead key).
    pub killed_edge: EdgeKey,
    /// The killed edge's plus half (dead key). Slot association is the
    /// edge's, not the argument's: this is the argument or its mate
    /// according to which the edge claimed as [`crate::Edge::he_plus`].
    pub killed_he_plus: HalfEdgeKey,
    /// The killed edge's minus half (dead key).
    pub killed_he_minus: HalfEdgeKey,
    /// The killed far vertex (dead key). Recorded as the mate's start
    /// vertex — equal to `end(he)` on tier-1-valid input (antiparallel
    /// mates); on corrupt input it reports what was actually killed.
    pub killed_vertex: VertexKey,
    /// The killed edge's curve (dead key), iff orphaned and removed.
    pub killed_curve: Option<CurveKey>,
    /// The killed vertex's point (dead key), iff orphaned and removed.
    pub killed_point: Option<PointKey>,
}

/// The outcome of one [`Body::kef`] call: the killed edge complex, the
/// killed face and its outer loop, and the reaped geometry.
///
/// Every `killed_*` key is **dead** (see [`KvfsResult`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KefResult {
    /// The killed edge (dead key).
    pub killed_edge: EdgeKey,
    /// The killed edge's plus half (dead key). Slot association is the
    /// edge's, as in [`KevResult`].
    pub killed_he_plus: HalfEdgeKey,
    /// The killed edge's minus half (dead key).
    pub killed_he_minus: HalfEdgeKey,
    /// The killed face — the face of the argument half-edge's loop
    /// (dead key).
    pub killed_face: FaceKey,
    /// The killed face's outer loop — the argument half-edge's loop,
    /// whose remnant merged into the mate's loop (dead key).
    pub killed_loop: LoopKey,
    /// The killed edge's curve (dead key), iff orphaned and removed.
    pub killed_curve: Option<CurveKey>,
    /// The killed face's surface (dead key), iff orphaned and removed;
    /// `None` whenever another face still shares it (the common case —
    /// every `mef` face shares its parent's surface).
    pub killed_surface: Option<SurfaceKey>,
}

/// Every key minted by one [`Body::mfkrh`] call. Nothing is killed: the
/// promoted ring survives as the new face's outer loop, keeping its key
/// and its D5 birth record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MfkrhCreated {
    /// The new face (outer loop = the promoted ring; same shell as the
    /// ring's former face).
    pub face: FaceKey,
    /// The new face's fresh placeholder surface (see the
    /// [module docs](self) on why sharing cannot be restored).
    pub surface: SurfaceKey,
}

impl<T: Real> Body<T> {
    /// KVFS — *kill vertex, face, solid*: the inverse of [`Body::mvfs`].
    /// Destroys a solid in EXACTLY the skeletal state `mvfs` creates:
    /// one shell, one face with no rings, an
    /// [`LoopBoundary::Empty`] outer loop holding one lone vertex.
    ///
    /// Euler vector: `(v −1, e 0, f −1, h 0, r 0, s −1)` — arena deltas
    /// −1 solid, −1 shell, −1 face, −1 loop, −1 vertex.
    ///
    /// **Minting order**: nothing is minted. **Kill order** (D9, exact,
    /// the reverse of `mvfs`'s spine minting): face, loop, shell, solid,
    /// vertex (each with its provenance entry), then the face's surface
    /// iff orphaned and the vertex's point iff orphaned (geometry
    /// hygiene, module docs).
    ///
    /// # Precondition check order
    ///
    /// The solid resolves ([`EulerOpError::StaleKey`]); it has exactly
    /// one shell ([`EulerOpError::SolidNotSingleShell`]); the shell
    /// resolves (`StaleKey`); it has exactly one face
    /// ([`EulerOpError::ShellNotSingleFace`]); the face resolves
    /// (`StaleKey`); it has no rings ([`EulerOpError::FaceHasRings`]);
    /// its outer loop resolves (`StaleKey`); the loop is empty
    /// ([`EulerOpError::LoopNotEmpty`]); the lone vertex resolves
    /// (`StaleKey`). (A lone vertex with `emanating: Some` — or one held
    /// by a second empty loop — is tier-1-invalid input and is not
    /// re-checked here; the debug postcondition would report the
    /// resulting garbage.)
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn kvfs(&mut self, solid: SolidKey) -> Result<KvfsResult, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        // ---- Preconditions: no mutation until every check passes. ----
        let solid_data = self.get_solid(solid).ok_or(EulerOpError::StaleKey {
            key: EntityId::Solid(solid),
        })?;
        let [shell] = solid_data.shells[..] else {
            return Err(EulerOpError::SolidNotSingleShell {
                solid,
                shells: solid_data.shells.len(),
            });
        };
        let shell_data = self.get_shell(shell).ok_or(EulerOpError::StaleKey {
            key: EntityId::Shell(shell),
        })?;
        let [face] = shell_data.faces[..] else {
            return Err(EulerOpError::ShellNotSingleFace {
                shell,
                faces: shell_data.faces.len(),
            });
        };
        let face_data = self.get_face(face).cloned().ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(face),
        })?;
        if !face_data.rings.is_empty() {
            return Err(EulerOpError::FaceHasRings { face });
        }
        let loop_key = face_data.outer;
        let loop_data = self.get_loop(loop_key).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(loop_key),
        })?;
        let LoopBoundary::Empty { vertex } = loop_data.boundary else {
            return Err(EulerOpError::LoopNotEmpty { r#loop: loop_key });
        };
        let vertex_data = self.get_vertex(vertex).ok_or(EulerOpError::StaleKey {
            key: EntityId::Vertex(vertex),
        })?;
        let point = vertex_data.point;

        // ---- Mutation (infallible from here on). ----
        // Kill order (documented above): face, loop, shell, solid,
        // vertex, then orphaned geometry.
        self.faces.remove(face);
        self.face_provenance.remove(face);
        self.loops.remove(loop_key);
        self.loop_provenance.remove(loop_key);
        self.shells.remove(shell);
        self.shell_provenance.remove(shell);
        self.solids.remove(solid);
        self.solid_provenance.remove(solid);
        self.vertices.remove(vertex);
        self.vertex_provenance.remove(vertex);
        let killed_surface = self
            .remove_surface_if_orphaned(face_data.surface)
            .then_some(face_data.surface);
        let killed_point = self.remove_point_if_orphaned(point).then_some(point);

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(before, (-1, -1, -1, -1, 0, 0, -1), "kvfs");
        Ok(KvfsResult {
            killed_solid: solid,
            killed_shell: shell,
            killed_face: face,
            killed_loop: loop_key,
            killed_vertex: vertex,
            killed_surface,
            killed_point,
        })
    }

    /// KEV — *kill edge, vertex*: the inverse of [`Body::mev`]. Kills
    /// `he`'s edge and **`end(he)`** (the far vertex), merging that
    /// vertex's remaining edge fan back onto `start(he)`.
    ///
    /// Which vertex dies is the argument's choice: `kev(he)` kills the
    /// vertex `he` points at; to kill the other endpoint, pass the mate.
    /// Full derivation, surgery diagram, and the degenerate strut/
    /// segment cases (each the inverse of a [`crate::MevSite`] case):
    /// [module docs](self).
    ///
    /// Euler vector: `(v −1, e −1, f 0, h 0, r 0, s 0)` — arena deltas
    /// −2 half-edges, −1 edge, −1 vertex.
    ///
    /// **Minting order**: nothing is minted. **Kill order** (D9, exact):
    /// `he`, its mate, the edge, the far vertex (each with its
    /// provenance entry), then the edge's curve iff orphaned and the
    /// vertex's point iff orphaned.
    ///
    /// **Re-anchoring** (unconditional, module docs): affected loops
    /// re-anchor at the first survivor after the killed half in `next`
    /// order; the survivor vertex's `emanating` becomes the first
    /// merged-fan member, else the strut case's `next(mate)`, else
    /// `None` (segment kill — the loop is [`LoopBoundary::Empty`] at the
    /// survivor again).
    ///
    /// # Precondition check order
    ///
    /// `he` resolves ([`EulerOpError::StaleKey`]); its edge resolves
    /// (`StaleKey`) and claims it
    /// ([`EulerOpError::UnclaimedHalfEdge`]); the mate resolves
    /// (`StaleKey`); the endpoints are distinct
    /// ([`EulerOpError::SelfLoopEdge`]); both endpoint vertices resolve
    /// (`StaleKey`); both parent loops resolve (`StaleKey`) and are
    /// cycles ([`EulerOpError::LoopNotCycle`]); the far vertex's orbit
    /// closes ([`EulerOpError::OrbitBroken`] — tier-1-invalid input);
    /// the four splice links (`prev`/`next` of both halves) resolve
    /// (`StaleKey`).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn kev(&mut self, he: HalfEdgeKey) -> Result<KevResult, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        // ---- Preconditions: no mutation until every check passes. ----
        let he_data = self.resolve_half_edge(he)?;
        let edge = he_data.edge;
        let edge_data = self.get_edge(edge).cloned().ok_or(EulerOpError::StaleKey {
            key: EntityId::Edge(edge),
        })?;
        let m = if edge_data.he_plus == he {
            edge_data.he_minus
        } else if edge_data.he_minus == he {
            edge_data.he_plus
        } else {
            return Err(EulerOpError::UnclaimedHalfEdge { he, edge });
        };
        let m_data = self.resolve_half_edge(m)?;
        let v = he_data.start; // survives
        let w = m_data.start; // dies (= end(he))
        if v == w {
            return Err(EulerOpError::SelfLoopEdge { edge, vertex: v });
        }
        if !self.vertices.contains_key(v) {
            return Err(EulerOpError::StaleKey {
                key: EntityId::Vertex(v),
            });
        }
        let w_point = self
            .get_vertex(w)
            .ok_or(EulerOpError::StaleKey {
                key: EntityId::Vertex(w),
            })?
            .point;
        let l1 = he_data.parent_loop;
        let l2 = m_data.parent_loop;
        for loop_key in [l1, l2] {
            let loop_data = self.get_loop(loop_key).ok_or(EulerOpError::StaleKey {
                key: EntityId::Loop(loop_key),
            })?;
            if matches!(loop_data.boundary, LoopBoundary::Empty { .. }) {
                return Err(EulerOpError::LoopNotCycle { r#loop: loop_key });
            }
        }
        // w's whole fan, walked clockwise from the doomed mate: the
        // members after m are the survivors to merge onto v (bounded
        // walk, D9).
        let orbit_w = self
            .vertex_orbit(m)
            .ok_or(EulerOpError::OrbitBroken { he: m })?;
        let fan: Vec<HalfEdgeKey> = orbit_w[1..].to_vec();
        // The unsplice writes through all four neighbor links; validate
        // them now so the mutation below cannot fail midway (atomicity).
        let (a, b) = (he_data.prev, he_data.next);
        let (c, d) = (m_data.prev, m_data.next);
        for link in [a, b, c, d] {
            if !self.half_edges.contains_key(link) {
                return Err(EulerOpError::StaleKey {
                    key: EntityId::HalfEdge(link),
                });
            }
        }
        // ---- Mutation (infallible from here on). ----
        // Fan merge: everything starting at w except the doomed mate now
        // starts at v (the run move, reversed — module docs).
        for &moved in &fan {
            if let Some(half_edge) = self.get_half_edge_mut(moved) {
                half_edge.start = v;
            }
        }
        // Unsplice (derived as mev's exact inverse — module docs). The
        // adjacency cases collapse the two link writes into one.
        let segment_kill = b == m && d == he; // the loop was [he, m]
        if segment_kill {
            // The 2-cycle loop empties: Empty at the survivor v — the
            // inverse of MevSite::Lone.
            if let Some(loop_data) = self.get_loop_mut(l1) {
                loop_data.boundary = LoopBoundary::Empty { vertex: v };
            }
        } else if b == m {
            // Strut shape … a → he → m → d …: one write bridges both.
            self.link_half_edges(a, d);
            if let Some(loop_data) = self.get_loop_mut(l1) {
                loop_data.boundary = LoopBoundary::Cycle { first: d };
            }
        } else if d == he {
            // Mirror adjacency … c → m → he → b ….
            self.link_half_edges(c, b);
            if let Some(loop_data) = self.get_loop_mut(l1) {
                loop_data.boundary = LoopBoundary::Cycle { first: b };
            }
        } else {
            // General: unsplice each half from its own loop (one loop or
            // two). Re-anchor both; when l1 == l2 the second write wins
            // (deterministic).
            self.link_half_edges(a, b);
            self.link_half_edges(c, d);
            if let Some(loop_data) = self.get_loop_mut(l1) {
                loop_data.boundary = LoopBoundary::Cycle { first: b };
            }
            if let Some(loop_data) = self.get_loop_mut(l2) {
                loop_data.boundary = LoopBoundary::Cycle { first: d };
            }
        }
        // Emanating rule (unconditional, module docs): first merged-fan
        // member (which is next(he) — the clockwise orbit step from m is
        // next(mate(m)) = next(he)), else the strut's next(m), else None.
        let v_anchor = if segment_kill {
            None
        } else if let Some(&first) = fan.first() {
            Some(first)
        } else {
            Some(d)
        };
        if let Some(vertex) = self.get_vertex_mut(v) {
            vertex.emanating = v_anchor;
        }
        // Kills, each with its provenance entry (kill order documented
        // above).
        self.half_edges.remove(he);
        self.half_edge_provenance.remove(he);
        self.half_edges.remove(m);
        self.half_edge_provenance.remove(m);
        self.edges.remove(edge);
        self.edge_provenance.remove(edge);
        self.vertices.remove(w);
        self.vertex_provenance.remove(w);
        let killed_curve = self
            .remove_curve_if_orphaned(edge_data.curve)
            .then_some(edge_data.curve);
        let killed_point = self.remove_point_if_orphaned(w_point).then_some(w_point);

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(before, (0, 0, 0, 0, -2, -1, -1), "kev");
        Ok(KevResult {
            killed_edge: edge,
            killed_he_plus: edge_data.he_plus,
            killed_he_minus: edge_data.he_minus,
            killed_vertex: w,
            killed_curve,
            killed_point,
        })
    }

    /// KEF — *kill edge, face*: the inverse of [`Body::mef`]. Kills
    /// `he`'s edge and **the face of `he`'s loop** (`he`'s side dies);
    /// the dying loop's remnant merges into the mate's loop.
    ///
    /// Which face dies is the argument's choice: a `mef` is undone by
    /// `kef(created.he_minus)`; passing the mate kills the other side.
    /// Full derivation, surgery diagram, and the degenerate circular/
    /// lone cases (each the inverse of a [`crate::MefSite`] case):
    /// [module docs](self). Self-loop edges (`start == end`) are legal
    /// here — they are the `Lone`-inverse territory.
    ///
    /// Euler vector: `(v 0, e −1, f −1, h 0, r 0, s 0)` — arena deltas
    /// −2 half-edges, −1 edge, −1 face, −1 loop.
    ///
    /// **Minting order**: nothing is minted. **Kill order** (D9, exact):
    /// `he`, its mate, the edge, the dying loop, the dying face (each
    /// with its provenance entry), then the edge's curve iff orphaned
    /// and the face's surface iff orphaned (usually shared, hence kept —
    /// module docs).
    ///
    /// **Re-anchoring** (unconditional, module docs): the surviving loop
    /// re-anchors at `next(mate)` when that survives, else `next(he)`;
    /// both endpoint vertices' `emanating` are rewritten to the derived
    /// survivors (or `None` in the `Lone` inverse, where the surviving
    /// loop becomes [`LoopBoundary::Empty`]).
    ///
    /// # Precondition check order
    ///
    /// `he` resolves ([`EulerOpError::StaleKey`]); its edge resolves
    /// (`StaleKey`) and claims it
    /// ([`EulerOpError::UnclaimedHalfEdge`]); the mate resolves
    /// (`StaleKey`); the two halves lie in distinct loops
    /// ([`EulerOpError::SameLoop`] — the same-loop configuration is
    /// [`Body::kemr`]'s) of distinct faces ([`EulerOpError::SameFace`] —
    /// two loops of one face is what [`Body::kfmrh`] on adjacent faces
    /// leaves behind; kill such an edge with [`Body::kev`], or via
    /// [`Body::mfkrh`]-then-`kef` for the self-loop variant); both
    /// loops resolve (`StaleKey`) and are cycles
    /// ([`EulerOpError::LoopNotCycle`]); both faces resolve
    /// (`StaleKey`); the dying face is ring-free
    /// ([`EulerOpError::FaceHasRings`]); its shell resolves
    /// (`StaleKey`); the dying loop's cycle closes
    /// ([`EulerOpError::LoopCycleBroken`]); the mate's `prev`/`next`
    /// resolve (`StaleKey`); both endpoint vertices resolve
    /// (`StaleKey`).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn kef(&mut self, he: HalfEdgeKey) -> Result<KefResult, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        // ---- Preconditions: no mutation until every check passes. ----
        let he_data = self.resolve_half_edge(he)?;
        let edge = he_data.edge;
        let edge_data = self.get_edge(edge).cloned().ok_or(EulerOpError::StaleKey {
            key: EntityId::Edge(edge),
        })?;
        let m = if edge_data.he_plus == he {
            edge_data.he_minus
        } else if edge_data.he_minus == he {
            edge_data.he_plus
        } else {
            return Err(EulerOpError::UnclaimedHalfEdge { he, edge });
        };
        let m_data = self.resolve_half_edge(m)?;
        let l1 = he_data.parent_loop; // dies with its face
        let l2 = m_data.parent_loop; // survives, absorbs the remnant
        if l1 == l2 {
            return Err(EulerOpError::SameLoop { r#loop: l1 });
        }
        let l1_data = self.get_loop(l1).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(l1),
        })?;
        if matches!(l1_data.boundary, LoopBoundary::Empty { .. }) {
            return Err(EulerOpError::LoopNotCycle { r#loop: l1 });
        }
        let f1 = l1_data.face; // dies
        let l2_data = self.get_loop(l2).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(l2),
        })?;
        if matches!(l2_data.boundary, LoopBoundary::Empty { .. }) {
            return Err(EulerOpError::LoopNotCycle { r#loop: l2 });
        }
        if l2_data.face == f1 {
            return Err(EulerOpError::SameFace { face: f1 });
        }
        let f1_data = self.get_face(f1).cloned().ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(f1),
        })?;
        if !f1_data.rings.is_empty() {
            return Err(EulerOpError::FaceHasRings { face: f1 });
        }
        let shell = f1_data.shell;
        if !self.shells.contains_key(shell) {
            return Err(EulerOpError::StaleKey {
                key: EntityId::Shell(shell),
            });
        }
        // The dying loop's full cycle (bounded, D9): everything after he
        // is the remnant that moves to the mate's loop. The walk resolves
        // every member, so prev(he)/next(he) are already validated.
        let cycle = self
            .loop_cycle(he)
            .ok_or(EulerOpError::LoopCycleBroken { r#loop: l1 })?;
        let remnant: Vec<HalfEdgeKey> = cycle[1..].to_vec();
        let (a, b) = (he_data.prev, he_data.next);
        let (c, d) = (m_data.prev, m_data.next);
        for link in [c, d] {
            if !self.half_edges.contains_key(link) {
                return Err(EulerOpError::StaleKey {
                    key: EntityId::HalfEdge(link),
                });
            }
        }
        let u = he_data.start;
        let w = m_data.start; // may equal u (self-loop edge)
        for vertex in [u, w] {
            if !self.vertices.contains_key(vertex) {
                return Err(EulerOpError::StaleKey {
                    key: EntityId::Vertex(vertex),
                });
            }
        }

        // ---- Mutation (infallible from here on). ----
        // The remnant joins the mate's loop.
        for &moved in &remnant {
            if let Some(half_edge) = self.get_half_edge_mut(moved) {
                half_edge.parent_loop = l2;
            }
        }
        // Splice (derived as mef's exact inverse — module docs diagram).
        let he_alone = b == he; // dying loop was [he]
        let m_alone = d == m; // mate's loop was [m]
        match (he_alone, m_alone) {
            (true, true) => {
                // The Lone inverse: a self-loop edge whose halves were
                // both one-half-edge loops. The surviving loop empties.
                if let Some(loop_data) = self.get_loop_mut(l2) {
                    loop_data.boundary = LoopBoundary::Empty { vertex: w };
                }
            }
            (true, false) => {
                // Empty remnant: just unsplice the mate from its loop.
                self.link_half_edges(c, d);
                if let Some(loop_data) = self.get_loop_mut(l2) {
                    loop_data.boundary = LoopBoundary::Cycle { first: d };
                }
            }
            (false, true) => {
                // The mate was alone: the remnant closes into itself and
                // becomes the surviving loop's whole cycle.
                self.link_half_edges(a, b);
                if let Some(loop_data) = self.get_loop_mut(l2) {
                    loop_data.boundary = LoopBoundary::Cycle { first: b };
                }
            }
            (false, false) => {
                // General: stitch the remnant across the mate's gap.
                self.link_half_edges(c, b);
                self.link_half_edges(a, d);
                if let Some(loop_data) = self.get_loop_mut(l2) {
                    loop_data.boundary = LoopBoundary::Cycle { first: d };
                }
            }
        }
        // Emanating rule (unconditional, module docs): next(m) starts at
        // u and next(he) starts at w by antiparallelism; fall back
        // across, else None. When u == w the w-side write is last and
        // wins (kemr's precedent).
        let survivor = |candidate: HalfEdgeKey, fallback: HalfEdgeKey| {
            if candidate != he && candidate != m {
                Some(candidate)
            } else if fallback != he && fallback != m {
                Some(fallback)
            } else {
                None
            }
        };
        let u_anchor = survivor(d, b);
        let w_anchor = survivor(b, d);
        if let Some(vertex) = self.get_vertex_mut(u) {
            vertex.emanating = u_anchor;
        }
        if let Some(vertex) = self.get_vertex_mut(w) {
            vertex.emanating = w_anchor;
        }
        // Kills, each with its provenance entry (kill order documented
        // above), then the shell's face list and orphaned geometry.
        self.half_edges.remove(he);
        self.half_edge_provenance.remove(he);
        self.half_edges.remove(m);
        self.half_edge_provenance.remove(m);
        self.edges.remove(edge);
        self.edge_provenance.remove(edge);
        self.loops.remove(l1);
        self.loop_provenance.remove(l1);
        self.faces.remove(f1);
        self.face_provenance.remove(f1);
        if let Some(shell_data) = self.get_shell_mut(shell) {
            shell_data.faces.retain(|&face| face != f1);
        }
        let killed_curve = self
            .remove_curve_if_orphaned(edge_data.curve)
            .then_some(edge_data.curve);
        let killed_surface = self
            .remove_surface_if_orphaned(f1_data.surface)
            .then_some(f1_data.surface);

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(before, (0, 0, -1, -1, -2, -1, 0), "kef");
        Ok(KefResult {
            killed_edge: edge,
            killed_he_plus: edge_data.he_plus,
            killed_he_minus: edge_data.he_minus,
            killed_face: f1,
            killed_loop: l1,
            killed_curve,
            killed_surface,
        })
    }

    /// MFKRH — *make face, kill ring–hole*: the inverse of
    /// [`Body::kfmrh`]. Promotes a ring (cycle or empty) to the outer
    /// loop of a NEW face in the same shell.
    ///
    /// The promoted loop survives with its key and D5 birth record; the
    /// new face mints a fresh placeholder surface (exact restoration of
    /// `kfmrh`'s killed face/surface is impossible and NOT promised —
    /// [module docs](self), which also document the deterministic
    /// surface-anchor rule). Promoting an [`LoopBoundary::Empty`] ring
    /// yields an **empty-outer face** — the `mvfs`-face shape, now
    /// operator-reachable inside a larger body.
    ///
    /// Euler vector: `(v 0, e 0, f +1, h −1, r −1, s 0)` — arena delta
    /// +1 face (the "−1 ring" is the surviving loop's promotion, not a
    /// kill; genus is derived, not stored).
    ///
    /// **Minting order** (D9, exact): surface, face. Nothing is killed.
    /// The new face is appended to the shell's face list; the ring
    /// leaves its former face's ring list (`retain`, order-preserving
    /// for the others).
    ///
    /// # Precondition check order
    ///
    /// The ring resolves ([`EulerOpError::StaleKey`]); its face resolves
    /// (`StaleKey`); it is not that face's outer loop
    /// ([`EulerOpError::RingIsOuter`]); the face's shell resolves
    /// (`StaleKey`); the surface-anchor chain resolves — the lone
    /// vertex (empty ring) or `Cycle::first` and its start vertex
    /// (cycle ring), plus the vertex's point (`StaleKey` /
    /// [`EulerOpError::StaleGeometry`]).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn mfkrh(&mut self, ring: LoopKey) -> Result<MfkrhCreated, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        // ---- Preconditions: no mutation until every check passes. ----
        let ring_data = self.get_loop(ring).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(ring),
        })?;
        let old_face = ring_data.face;
        let boundary = ring_data.boundary;
        let old_face_data = self.get_face(old_face).ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(old_face),
        })?;
        if old_face_data.outer == ring {
            return Err(EulerOpError::RingIsOuter { r#loop: ring });
        }
        let shell = old_face_data.shell;
        if !self.shells.contains_key(shell) {
            return Err(EulerOpError::StaleKey {
                key: EntityId::Shell(shell),
            });
        }
        // The surface-anchor rule (module docs): the ring's lone
        // vertex's point, or start(Cycle::first)'s point.
        let anchor_vertex = match boundary {
            LoopBoundary::Empty { vertex } => vertex,
            LoopBoundary::Cycle { first } => self.resolve_half_edge(first)?.start,
        };
        let anchor = self.resolve_vertex_point(anchor_vertex)?;

        // ---- Mutation (infallible from here on). ----
        // Minting order (documented above): surface, face.
        let surface = self.add_surface(SurfaceGeom::Placeholder { anchor });
        let face = self.add_face(
            Face {
                surface,
                outer: ring,
                rings: vec![],
                shell,
            },
            Provenance::Mfkrh { ring },
        );
        if let Some(old) = self.get_face_mut(old_face) {
            old.rings.retain(|&l| l != ring);
        }
        if let Some(loop_data) = self.get_loop_mut(ring) {
            loop_data.face = face;
        }
        if let Some(shell_data) = self.get_shell_mut(shell) {
            shell_data.faces.push(face);
        }

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(before, (0, 0, 1, 0, 0, 0, 0), "mfkrh");
        Ok(MfkrhCreated { face, surface })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Point3;

    use super::*;
    use crate::entity::{Edge, HalfEdge, Loop, Shell, Vertex};
    use crate::euler::{MefCreated, MefSite, MevCreated, MevSite, MvfsCreated};
    use crate::fixtures::{deep_snapshot, ops_cube, ops_holed_box, prov};
    use crate::iso::{canonical_form, isomorphic};
    use crate::validate::validate;

    fn p(x: f64) -> Point3<f64> {
        Point3::new(x, 0.0, 0.0)
    }

    /// Runs `op` on `body`, asserts it fails with exactly `expected`,
    /// and asserts the body is DEEPLY untouched (kill-direction
    /// atomicity needs more than counts).
    fn assert_err_deep_unchanged(
        body: &mut Body<f64>,
        expected: &EulerOpError,
        op: impl FnOnce(&mut Body<f64>) -> EulerOpError,
    ) {
        let before = deep_snapshot(body);
        let err = op(body);
        assert_eq!(&err, expected);
        assert_eq!(deep_snapshot(body), before, "body changed on Err");
    }

    /// All ten arena lengths, for delta checks.
    fn counts(body: &Body<f64>) -> [usize; 10] {
        [
            body.solids().count(),
            body.shells().count(),
            body.faces().count(),
            body.loops().count(),
            body.half_edges().count(),
            body.edges().count(),
            body.vertices().count(),
            body.points().count(),
            body.curves().count(),
            body.surfaces().count(),
        ]
    }

    /// mvfs + mev(Lone): the segment body.
    fn segment() -> (Body<f64>, MvfsCreated, MevCreated) {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0)).unwrap();
        let seg = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(1.0),
            )
            .unwrap();
        (body, seed, seg)
    }

    /// Segment + one strut at the far vertex: cycle
    /// `[seg+, strut+, strut−, seg−]`.
    fn strutted() -> (Body<f64>, MvfsCreated, MevCreated, MevCreated) {
        let (mut body, seed, seg) = segment();
        let strut = body
            .mev(
                MevSite::Fan {
                    he1: seg.he_minus,
                    he2: seg.he_minus,
                },
                p(2.0),
            )
            .unwrap();
        (body, seed, seg, strut)
    }

    // ------------------------------------------------------------------
    // kvfs
    // ------------------------------------------------------------------

    #[test]
    fn kvfs_inverts_mvfs_to_the_empty_body() {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(Point3::new(1.0, 2.0, 3.0)).unwrap();
        let result = body.kvfs(seed.solid).unwrap();
        assert_eq!(validate(&body), Ok(()));

        // E–P vector (−1, 0, −1, 0, 0, −1): everything is gone.
        assert_eq!(counts(&body), [0; 10]);
        // Killed keys are exactly mvfs's mints, and they are dead.
        assert_eq!(result.killed_solid, seed.solid);
        assert_eq!(result.killed_shell, seed.shell);
        assert_eq!(result.killed_face, seed.face);
        assert_eq!(result.killed_loop, seed.r#loop);
        assert_eq!(result.killed_vertex, seed.vertex);
        assert_eq!(result.killed_surface, Some(seed.surface));
        assert_eq!(result.killed_point, Some(seed.point));
        assert!(body.get_solid(seed.solid).is_none());
        assert!(body.get_vertex(seed.vertex).is_none());
        // Kill hygiene: provenance entries died with their entities.
        for id in [
            EntityId::Solid(seed.solid),
            EntityId::Shell(seed.shell),
            EntityId::Face(seed.face),
            EntityId::Loop(seed.r#loop),
            EntityId::Vertex(seed.vertex),
        ] {
            assert_eq!(body.provenance(id), None, "for {id}");
        }
    }

    #[test]
    fn kvfs_roundtrips_with_mvfs_beside_another_solid() {
        // mvfs ∘ kvfs in a body that also holds an unrelated solid: the
        // bystander is untouched (deep) and the pair restores the
        // canonical form.
        let (mut body, _, _) = segment();
        let before = canonical_form(&body);
        let seed = body.mvfs(p(9.0)).unwrap();
        body.kvfs(seed.solid).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(canonical_form(&body), before);
    }

    #[test]
    fn kvfs_precondition_errors_are_atomic() {
        // Stale solid.
        let (mut body, seed, seg) = segment();
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::StaleKey {
                key: EntityId::Solid(SolidKey::default()),
            },
            |b| b.kvfs(SolidKey::default()).unwrap_err(),
        );
        // Grown solid: the outer loop is a cycle, not empty.
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::LoopNotEmpty {
                r#loop: seed.r#loop,
            },
            |b| b.kvfs(seed.solid).unwrap_err(),
        );
        // Two faces: kill the edge back and split instead.
        body.kev(seg.he_plus).unwrap();
        body.mef(MefSite::Lone {
            r#loop: seed.r#loop,
        })
        .unwrap();
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::ShellNotSingleFace {
                shell: seed.shell,
                faces: 2,
            },
            |b| b.kvfs(seed.solid).unwrap_err(),
        );
    }

    #[test]
    fn kvfs_rejects_extra_shells_and_rings() {
        // Extra shell (raw-built; multi-shell solids arrive at M3).
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0)).unwrap();
        let extra = body.add_shell(
            Shell {
                faces: vec![],
                solid: seed.solid,
            },
            prov(),
        );
        body.get_solid_mut(seed.solid).unwrap().shells.push(extra);
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::SolidNotSingleShell {
                solid: seed.solid,
                shells: 2,
            },
            |b| b.kvfs(seed.solid).unwrap_err(),
        );
        // A ring on the single face (raw-built empty ring).
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0)).unwrap();
        let point = body.add_point(p(1.0));
        let vertex = body.add_vertex(
            Vertex {
                point,
                emanating: None,
            },
            prov(),
        );
        let ring = body.add_loop(
            Loop {
                boundary: LoopBoundary::Empty { vertex },
                face: seed.face,
            },
            prov(),
        );
        body.get_face_mut(seed.face).unwrap().rings.push(ring);
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::FaceHasRings { face: seed.face },
            |b| b.kvfs(seed.solid).unwrap_err(),
        );
    }

    // ------------------------------------------------------------------
    // kev: segment / strut / general fan merge / two loops / mirror
    // ------------------------------------------------------------------

    #[test]
    fn kev_segment_kill_inverts_lone_mev() {
        let (mut body, seed, seg) = segment();
        let skeletal = {
            let mut fresh = Body::<f64>::new();
            fresh.mvfs(p(0.0)).unwrap();
            canonical_form(&fresh)
        };
        let before = counts(&body);
        let result = body.kev(seg.he_plus).unwrap();
        assert_eq!(validate(&body), Ok(()));

        // E–P vector (−1, −1, 0, 0, 0, 0): −1 vertex, −1 edge, −2 halves
        // (plus the reaped point and curve).
        let after = counts(&body);
        assert_eq!(
            after,
            [
                before[0],
                before[1],
                before[2],
                before[3],
                before[4] - 2,
                before[5] - 1,
                before[6] - 1,
                before[7] - 1,
                before[8] - 1,
                before[9],
            ]
        );
        // The far vertex died; the loop is empty at the survivor; the
        // survivor is lone again (emanating None).
        assert_eq!(result.killed_vertex, seg.vertex);
        assert_eq!(result.killed_edge, seg.edge);
        assert_eq!(result.killed_he_plus, seg.he_plus);
        assert_eq!(result.killed_he_minus, seg.he_minus);
        assert_eq!(result.killed_curve, Some(seg.curve));
        assert_eq!(result.killed_point, Some(seg.point));
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Empty {
                vertex: seed.vertex
            }
        );
        assert_eq!(body.get_vertex(seed.vertex).unwrap().emanating, None);
        // Kill hygiene: dead keys, dead provenance.
        for id in [
            EntityId::Vertex(seg.vertex),
            EntityId::Edge(seg.edge),
            EntityId::HalfEdge(seg.he_plus),
            EntityId::HalfEdge(seg.he_minus),
        ] {
            assert_eq!(body.provenance(id), None, "for {id}");
        }
        assert!(body.get_edge(seg.edge).is_none());
        // The remnant is exactly the skeletal mvfs body.
        assert_eq!(canonical_form(&body), skeletal);
        // Hand kill∘make roundtrip: mev(Lone) at the same coordinates
        // restores the segment body up to isomorphism.
        let with_segment = {
            let (fresh, _, _) = segment();
            canonical_form(&fresh)
        };
        body.mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p(1.0),
        )
        .unwrap();
        assert_eq!(canonical_form(&body), with_segment);
    }

    #[test]
    fn kev_kills_the_vertex_the_argument_points_at() {
        // The direction pin: kev(he_minus) kills end(he_minus) — the OLD
        // vertex — leaving the new one as the lone survivor.
        let (mut body, seed, seg) = segment();
        let result = body.kev(seg.he_minus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(result.killed_vertex, seed.vertex);
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Empty { vertex: seg.vertex }
        );
        assert!(body.get_vertex(seed.vertex).is_none());
        assert!(body.get_vertex(seg.vertex).is_some());
    }

    #[test]
    fn kev_strut_kill_is_pure_removal() {
        let (mut body, seed, seg, strut) = strutted();
        let before_strut = {
            let (fresh, _, _) = segment();
            canonical_form(&fresh)
        };
        let result = body.kev(strut.he_plus).unwrap();
        assert_eq!(validate(&body), Ok(()));

        assert_eq!(result.killed_vertex, strut.vertex);
        // The loop is the plain segment cycle again, re-anchored at the
        // first survivor after the killed pair (next(mate) =
        // seg.he_minus).
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Cycle {
                first: seg.he_minus
            }
        );
        assert_eq!(
            body.loop_cycle(seg.he_plus),
            Some(vec![seg.he_plus, seg.he_minus])
        );
        // Emanating rule, strut case: the survivor anchors at next(mate)
        // (which starts at it).
        assert_eq!(
            body.get_vertex(seg.vertex).unwrap().emanating,
            Some(seg.he_minus)
        );
        assert_eq!(body.vertex_orbit(seg.he_minus), Some(vec![seg.he_minus]));
        // make ∘ kill: the strut mev is exactly undone.
        assert_eq!(canonical_form(&body), before_strut);
    }

    /// PR 2's valence-4 star: central vertex `v` with clockwise orbit
    /// `[a+, b+, c+, d+]`.
    fn four_spoke_star() -> (Body<f64>, MvfsCreated, [MevCreated; 4]) {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0)).unwrap();
        let a = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(1.0),
            )
            .unwrap();
        let strut_at = |body: &mut Body<f64>, x: f64| {
            body.mev(
                MevSite::Fan {
                    he1: a.he_plus,
                    he2: a.he_plus,
                },
                p(x),
            )
            .unwrap()
        };
        let b = strut_at(&mut body, 2.0);
        let c = strut_at(&mut body, 3.0);
        let d = strut_at(&mut body, 4.0);
        assert_eq!(
            body.vertex_orbit(a.he_plus),
            Some(vec![a.he_plus, b.he_plus, c.he_plus, d.he_plus])
        );
        (body, seed, [a, b, c, d])
    }

    #[test]
    fn kev_general_case_merges_the_fan_back() {
        // Inverse of THE direction-pinning mev test: split the valence-4
        // fan, then kev the new edge — the moved run {b+, c+} must come
        // back to v, and the orbit must be the original four spokes.
        let (mut body, seed, [a, b, c, d]) = four_spoke_star();
        let before = canonical_form(&body);
        let split = body
            .mev(
                MevSite::Fan {
                    he1: b.he_plus,
                    he2: d.he_plus,
                },
                p(5.0),
            )
            .unwrap();
        let result = body.kev(split.he_plus).unwrap();
        assert_eq!(validate(&body), Ok(()));

        let v = seed.vertex;
        assert_eq!(result.killed_vertex, split.vertex);
        let start = |body: &Body<f64>, he| body.get_half_edge(he).unwrap().start;
        for spoke in [a.he_plus, b.he_plus, c.he_plus, d.he_plus] {
            assert_eq!(start(&body, spoke), v);
        }
        // Emanating rule, fan case: the first merged-fan member — the
        // clockwise-first half the killed plus half pointed at (b+).
        assert_eq!(body.get_vertex(v).unwrap().emanating, Some(b.he_plus));
        // Orbit closes over exactly the four spokes.
        let orbit = body.vertex_orbit(a.he_plus).unwrap();
        assert_eq!(orbit.len(), 4);
        // make ∘ kill: exact canonical restoration.
        assert_eq!(canonical_form(&body), before);
    }

    #[test]
    fn kev_unsplices_across_two_loops() {
        // Inverse of PR 2's two-loop fan mev (the digon pillow's seed
        // vertex): he_plus and he_minus land in different loops; kev
        // must unsplice each from its own loop.
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0)).unwrap();
        let seg = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(1.0),
            )
            .unwrap();
        let split = body
            .mef(MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            })
            .unwrap();
        let before = canonical_form(&body);
        let fan = body
            .mev(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: split.he_plus,
                },
                p(2.0),
            )
            .unwrap();
        // The new halves landed in different loops (pinned by PR 2's
        // test); now undo.
        let result = body.kev(fan.he_plus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(result.killed_vertex, fan.vertex);
        assert_eq!(
            body.get_half_edge(seg.he_plus).unwrap().start,
            seed.vertex,
            "the moved run returned to the old vertex"
        );
        assert_eq!(canonical_form(&body), before);
    }

    #[test]
    fn kev_mirror_case_merges_fan_onto_the_valence_one_survivor() {
        // kev from the tip side: he = strut.he_minus starts at the
        // valence-1 tip and points at the fan-carrying vertex. The fan
        // migrates to the tip. (Documented asymmetry: THIS kill has no
        // single-mev re-make — the full-fan run is mev-inexpressible —
        // but the result must still be tier-1 valid.)
        let (mut body, seed, seg, strut) = strutted();
        let result = body.kev(strut.he_minus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(result.killed_vertex, seg.vertex);
        // The whole old fan of seg.vertex (seg.he_minus) now starts at
        // the strut tip.
        assert_eq!(
            body.get_half_edge(seg.he_minus).unwrap().start,
            strut.vertex
        );
        assert_eq!(
            body.loop_cycle(seg.he_plus),
            Some(vec![seg.he_plus, seg.he_minus])
        );
        // The segment now runs seed.vertex → strut.vertex.
        assert_eq!(body.get_half_edge(seg.he_plus).unwrap().start, seed.vertex);
        assert_eq!(body.half_edge_end(seg.he_plus), Some(strut.vertex));
    }

    #[test]
    fn kev_rejects_self_loops_and_stale_keys() {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0)).unwrap();
        let circ = body
            .mef(MefSite::Lone {
                r#loop: seed.r#loop,
            })
            .unwrap();
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::SelfLoopEdge {
                edge: circ.edge,
                vertex: seed.vertex,
            },
            |b| b.kev(circ.he_plus).unwrap_err(),
        );
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::StaleKey {
                key: EntityId::HalfEdge(HalfEdgeKey::default()),
            },
            |b| b.kev(HalfEdgeKey::default()).unwrap_err(),
        );
    }

    #[test]
    fn kev_rejects_a_corrupt_edge_bijection() {
        // Raw corruption: the edge no longer claims the argument half.
        let (mut body, _, seg, strut) = strutted();
        body.get_edge_mut(seg.edge).unwrap().he_plus = strut.he_plus;
        body.get_edge_mut(seg.edge).unwrap().he_minus = strut.he_minus;
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::UnclaimedHalfEdge {
                he: seg.he_plus,
                edge: seg.edge,
            },
            |b| b.kev(seg.he_plus).unwrap_err(),
        );
    }

    #[test]
    fn kev_rejects_corrupt_loops_and_orbits() {
        // LoopNotCycle: a half-edge whose parent loop claims to be
        // empty (tier-1-invalid raw corruption).
        let (mut body, seed, seg, _strut) = strutted();
        body.get_loop_mut(seed.r#loop).unwrap().boundary = LoopBoundary::Empty {
            vertex: seed.vertex,
        };
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::LoopNotCycle {
                r#loop: seed.r#loop,
            },
            |b| b.kev(seg.he_plus).unwrap_err(),
        );
        // OrbitBroken: the far vertex's orbit walk hits a corrupt mate
        // bijection mid-fan (raw corruption two steps away from the
        // argument).
        let (mut body, _seed, seg, strut) = strutted();
        body.get_edge_mut(strut.edge).unwrap().he_plus = strut.he_minus;
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::OrbitBroken { he: seg.he_minus },
            |b| b.kev(seg.he_plus).unwrap_err(),
        );
    }

    // ------------------------------------------------------------------
    // kef: general / argument side / circular / mate-alone / lone
    // ------------------------------------------------------------------

    /// The digon pillow built through the operators, plus its keys.
    fn ops_pillow() -> (Body<f64>, MvfsCreated, MevCreated, MefCreated) {
        let (mut body, seed, seg) = segment();
        let split = body
            .mef(MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            })
            .unwrap();
        (body, seed, seg, split)
    }

    #[test]
    fn kef_general_case_inverts_mef() {
        // Split one pillow face with a second chord edge, then kef the
        // half in the NEW loop: exact undo.
        let (mut body, _seed, seg, split) = ops_pillow();
        let before = canonical_form(&body);
        let before_counts = counts(&body);
        // Split the new face (loop [he_minus, seg.he_plus]) between its
        // two vertices.
        let cut = body
            .mef(MefSite::Chords {
                he1: split.he_minus,
                he2: seg.he_plus,
            })
            .unwrap();
        let result = body.kef(cut.he_minus).unwrap();
        assert_eq!(validate(&body), Ok(()));

        // E–P vector (0, −1, −1, 0, 0, 0) relative to the pre-mef state.
        assert_eq!(counts(&body), before_counts);
        assert_eq!(result.killed_face, cut.face);
        assert_eq!(result.killed_loop, cut.r#loop);
        assert_eq!(result.killed_edge, cut.edge);
        assert_eq!(result.killed_curve, Some(cut.curve));
        // The new face shared the split face's surface, so nothing is
        // reaped.
        assert_eq!(result.killed_surface, None);
        for id in [
            EntityId::Face(cut.face),
            EntityId::Loop(cut.r#loop),
            EntityId::Edge(cut.edge),
            EntityId::HalfEdge(cut.he_plus),
            EntityId::HalfEdge(cut.he_minus),
        ] {
            assert_eq!(body.provenance(id), None, "for {id}");
        }
        assert_eq!(canonical_form(&body), before);
    }

    #[test]
    fn kef_kills_the_argument_side() {
        // Same construction, but kef(he_plus): the OLD face (he_plus's
        // side) dies instead, and the new face survives. Pins "he's side
        // dies".
        let (mut body, _seed, seg, split) = ops_pillow();
        let old_face = body
            .get_loop(body.get_half_edge(split.he_minus).unwrap().parent_loop)
            .unwrap()
            .face;
        let cut = body
            .mef(MefSite::Chords {
                he1: split.he_minus,
                he2: seg.he_plus,
            })
            .unwrap();
        assert_eq!(old_face, split.face, "the mef split the new pillow face");
        let result = body.kef(cut.he_plus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(result.killed_face, split.face);
        assert!(body.get_face(split.face).is_none());
        assert!(body.get_face(cut.face).is_some());
    }

    #[test]
    fn kef_circular_face_inverse() {
        // mef(Chords{x, x}) makes the one-edge circular face; kef on the
        // self-cycled minus half is its exact inverse.
        let (mut body, _seed, seg) = segment();
        let before = canonical_form(&body);
        let circ = body
            .mef(MefSite::Chords {
                he1: seg.he_minus,
                he2: seg.he_minus,
            })
            .unwrap();
        let result = body.kef(circ.he_minus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(result.killed_face, circ.face);
        assert_eq!(canonical_form(&body), before);
    }

    #[test]
    fn kef_mate_alone_kills_the_big_side() {
        // Same circular-face configuration, but kef(he_plus): the BIG
        // side (the old face) dies and its remnant becomes the surviving
        // circular face's loop. (Documented asymmetry: no single-mef
        // re-make exists for this kill; validity is the contract.)
        let (mut body, seed, seg) = segment();
        let circ = body
            .mef(MefSite::Chords {
                he1: seg.he_minus,
                he2: seg.he_minus,
            })
            .unwrap();
        let old_face = body.get_loop(seed.r#loop).unwrap().face;
        let result = body.kef(circ.he_plus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(result.killed_face, old_face);
        assert_eq!(result.killed_loop, seed.r#loop);
        // The segment remnant survives under the (former) circular
        // face's loop.
        assert_eq!(
            body.get_half_edge(seg.he_plus).unwrap().parent_loop,
            circ.r#loop
        );
        assert_eq!(
            body.loop_cycle(seg.he_plus),
            Some(vec![seg.he_plus, seg.he_minus])
        );
    }

    #[test]
    fn kef_lone_inverse_restores_the_empty_loop() {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0)).unwrap();
        let before = canonical_form(&body);
        let before_counts = counts(&body);
        let circ = body
            .mef(MefSite::Lone {
                r#loop: seed.r#loop,
            })
            .unwrap();
        let result = body.kef(circ.he_minus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(counts(&body), before_counts);
        assert_eq!(result.killed_face, circ.face);
        assert_eq!(result.killed_loop, circ.r#loop);
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Empty {
                vertex: seed.vertex
            }
        );
        assert_eq!(body.get_vertex(seed.vertex).unwrap().emanating, None);
        assert_eq!(canonical_form(&body), before);
    }

    #[test]
    fn kef_requires_two_loops_and_two_faces() {
        // Both halves in one loop: kemr's configuration.
        let (mut body, seed, seg) = segment();
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::SameLoop {
                r#loop: seed.r#loop,
            },
            |b| b.kef(seg.he_plus).unwrap_err(),
        );
        // Two loops of ONE face (raw-built): SameFace.
        let mut body = Body::<f64>::new();
        let point = body.add_point(p(0.0));
        let vertex = body.add_vertex(
            Vertex {
                point,
                emanating: None,
            },
            prov(),
        );
        let solid = body.add_solid(crate::entity::Solid { shells: vec![] }, prov());
        let shell = body.add_shell(
            Shell {
                faces: vec![],
                solid,
            },
            prov(),
        );
        body.get_solid_mut(solid).unwrap().shells.push(shell);
        let curve = body.add_curve(crate::geometry::CurveGeom::Placeholder { anchor: p(0.0) });
        let edge = body.add_edge(
            Edge {
                he_plus: HalfEdgeKey::default(),
                he_minus: HalfEdgeKey::default(),
                curve,
            },
            prov(),
        );
        let surface = body.add_surface(SurfaceGeom::Placeholder { anchor: p(0.0) });
        let one_half_loop = |body: &mut Body<f64>| {
            let l = body.add_loop(
                Loop {
                    boundary: LoopBoundary::Cycle {
                        first: HalfEdgeKey::default(),
                    },
                    face: FaceKey::default(),
                },
                prov(),
            );
            let he = body.add_half_edge(
                HalfEdge {
                    edge,
                    start: vertex,
                    parent_loop: l,
                    next: HalfEdgeKey::default(),
                    prev: HalfEdgeKey::default(),
                },
                prov(),
            );
            body.get_half_edge_mut(he).unwrap().next = he;
            body.get_half_edge_mut(he).unwrap().prev = he;
            body.get_loop_mut(l).unwrap().boundary = LoopBoundary::Cycle { first: he };
            (l, he)
        };
        let (outer, he1) = one_half_loop(&mut body);
        let (ring, he2) = one_half_loop(&mut body);
        let edge_data = body.get_edge_mut(edge).unwrap();
        edge_data.he_plus = he1;
        edge_data.he_minus = he2;
        let face = body.add_face(
            Face {
                surface,
                outer,
                rings: vec![ring],
                shell,
            },
            prov(),
        );
        body.get_loop_mut(outer).unwrap().face = face;
        body.get_loop_mut(ring).unwrap().face = face;
        body.get_shell_mut(shell).unwrap().faces.push(face);
        body.get_vertex_mut(vertex).unwrap().emanating = Some(he1);
        assert_err_deep_unchanged(&mut body, &EulerOpError::SameFace { face }, |b| {
            b.kef(he1).unwrap_err()
        });
    }

    #[test]
    fn kef_rejects_a_ringed_dying_face_but_accepts_the_mate() {
        // Plant an empty ring on one pillow face: kef from that side is
        // FaceHasRings; kef on the mate kills the bare other side.
        let (mut body, _seed, seg, split) = ops_pillow();
        // seg.he_plus lives in the new face's loop after the mef; strut
        // + kemr plants a ring there.
        let strut = body
            .mev(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: seg.he_plus,
                },
                p(2.0),
            )
            .unwrap();
        body.kemr(strut.he_plus, strut.he_minus).unwrap();
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::FaceHasRings { face: split.face },
            |b| b.kef(seg.he_plus).unwrap_err(),
        );
        // The mate's side (the old face) is ring-free: killable.
        let result = body.kef(seg.he_minus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_ne!(result.killed_face, split.face);
    }

    // ------------------------------------------------------------------
    // mfkrh
    // ------------------------------------------------------------------

    /// Pillow with one empty ring planted on the new face (strut +
    /// kemr): the §9.3 hole-anchor idiom.
    fn pillow_with_empty_ring() -> (Body<f64>, MefCreated, crate::euler_ring::KemrResult) {
        let (mut body, _seed, seg, split) = ops_pillow();
        let strut = body
            .mev(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: seg.he_plus,
                },
                p(2.0),
            )
            .unwrap();
        let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
        (body, split, kill)
    }

    #[test]
    fn mfkrh_promotes_an_empty_ring_to_an_empty_outer_face() {
        // THE operator-reachable Empty-outer face (predicted by PR 3's
        // log): promoting an empty ring yields a face whose outer loop
        // is an empty loop — and the body validates.
        let (mut body, split, kill) = pillow_with_empty_ring();
        let before_counts = counts(&body);
        let created = body.mfkrh(kill.ring).unwrap();
        assert_eq!(validate(&body), Ok(()));

        // E–P vector (0, 0, +1, −1, −1, 0): +1 face, +1 surface, all
        // else unchanged.
        let after = counts(&body);
        assert_eq!(
            after,
            [
                before_counts[0],
                before_counts[1],
                before_counts[2] + 1,
                before_counts[3],
                before_counts[4],
                before_counts[5],
                before_counts[6],
                before_counts[7],
                before_counts[8],
                before_counts[9] + 1,
            ]
        );
        // The promoted loop survives with its key, now the outer loop of
        // the new face.
        let new_face = body.get_face(created.face).unwrap();
        assert_eq!(new_face.outer, kill.ring);
        assert!(new_face.rings.is_empty());
        assert_eq!(new_face.surface, created.surface);
        assert_eq!(body.get_loop(kill.ring).unwrap().face, created.face);
        assert!(matches!(
            body.get_loop(kill.ring).unwrap().boundary,
            LoopBoundary::Empty { .. }
        ));
        // It left the old face's ring list.
        assert!(body.get_face(split.face).unwrap().rings.is_empty());
        // D5: the new face records the promotion; the loop keeps its
        // kemr birth record (promotion is not a re-birth).
        assert_eq!(
            body.provenance(EntityId::Face(created.face)),
            Some(&Provenance::Mfkrh { ring: kill.ring })
        );
        assert!(matches!(
            body.provenance(EntityId::Loop(kill.ring)),
            Some(&Provenance::Kemr { .. })
        ));
    }

    #[test]
    fn mfkrh_then_kfmrh_roundtrips() {
        let (mut body, split, kill) = pillow_with_empty_ring();
        let before = canonical_form(&body);
        let created = body.mfkrh(kill.ring).unwrap();
        let plug = body.kfmrh(split.face, created.face).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(plug.ring, kill.ring);
        // The fresh surface died with the face it was minted for.
        assert_eq!(plug.killed_surface, Some(created.surface));
        assert_eq!(canonical_form(&body), before);
    }

    #[test]
    fn mfkrh_unplugs_the_holed_box() {
        // kfmrh ∘ mfkrh on the genus-1 acceptance body: promoting the
        // demoted membrane loop back to a face takes genus 1 → 0, and
        // re-killing it restores the canonical form.
        let t = ops_holed_box();
        let mut body = t.body;
        let before = canonical_form(&body);
        let bottom_face = t.box_mefs[0].face;
        let created = body.mfkrh(t.plug.ring).unwrap();
        assert_eq!(validate(&body), Ok(()));
        // Euler–Poincaré at genus 0 with one ring left (the hole's top
        // rim): v − e + f − r = 16 − 24 + 11 − 1 = 2 = 2(1 − 0).
        let rings: usize = body.faces().map(|(_, face)| face.rings.len()).sum();
        assert_eq!(rings, 1);
        assert_eq!(body.faces().count(), 11);
        let plug = body.kfmrh(bottom_face, created.face).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(plug.ring, t.plug.ring);
        assert_eq!(canonical_form(&body), before);
    }

    #[test]
    fn mfkrh_rejects_outer_loops_and_stale_keys() {
        let (mut body, _split, kill) = pillow_with_empty_ring();
        let outer = {
            let ring_face = body.get_loop(kill.ring).unwrap().face;
            body.get_face(ring_face).unwrap().outer
        };
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::RingIsOuter { r#loop: outer },
            |b| b.mfkrh(outer).unwrap_err(),
        );
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::StaleKey {
                key: EntityId::Loop(LoopKey::default()),
            },
            |b| b.mfkrh(LoopKey::default()).unwrap_err(),
        );
    }

    #[test]
    fn mfkrh_rejects_a_stale_anchor_point() {
        // PR 3 noted the StaleGeometry arm becomes testable once point
        // removal exists; it does now. Rip the ring vertex's point out
        // through the raw arenas: the surface-anchor chain must fail
        // loudly and atomically.
        let (mut body, _split, kill) = pillow_with_empty_ring();
        let LoopBoundary::Empty { vertex } = body.get_loop(kill.ring).unwrap().boundary else {
            panic!("the planted ring is empty");
        };
        let point = body.get_vertex(vertex).unwrap().point;
        body.points.remove(point);
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::StaleGeometry {
                key: crate::entity::GeomRef::Point(point),
            },
            |b| b.mfkrh(kill.ring).unwrap_err(),
        );
    }

    // ------------------------------------------------------------------
    // The cube teardown: kill all 12 edges via kef/kev in a derived
    // order back to the skeletal state, then kvfs — empty body.
    // ------------------------------------------------------------------

    #[test]
    fn cube_tears_down_to_the_empty_body() {
        let t = ops_cube();
        let mut body = t.body;
        // Undo the five mefs in reverse: each kef(created.he_minus)
        // kills the face that mef made.
        for mef in t.mefs.iter().rev() {
            let result = body.kef(mef.he_minus).unwrap();
            assert_eq!(validate(&body), Ok(()));
            assert_eq!(result.killed_face, mef.face);
        }
        // Undo the seven mevs in reverse: each kev(created.he_plus)
        // kills the vertex that mev made.
        for mev in t.mevs.iter().rev() {
            let result = body.kev(mev.he_plus).unwrap();
            assert_eq!(validate(&body), Ok(()));
            assert_eq!(result.killed_vertex, mev.vertex);
        }
        // Skeletal again: kvfs finishes the job.
        assert_eq!(
            body.get_loop(t.seed.r#loop).unwrap().boundary,
            LoopBoundary::Empty {
                vertex: t.seed.vertex
            }
        );
        body.kvfs(t.seed.solid).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(counts(&body), [0; 10]);
    }

    // ------------------------------------------------------------------
    // Display coverage for the new error variants
    // ------------------------------------------------------------------

    #[test]
    fn new_error_variants_display() {
        let errors = [
            EulerOpError::UnclaimedHalfEdge {
                he: HalfEdgeKey::default(),
                edge: EdgeKey::default(),
            },
            EulerOpError::SelfLoopEdge {
                edge: EdgeKey::default(),
                vertex: VertexKey::default(),
            },
            EulerOpError::OrbitBroken {
                he: HalfEdgeKey::default(),
            },
            EulerOpError::SolidNotSingleShell {
                solid: SolidKey::default(),
                shells: 2,
            },
            EulerOpError::ShellNotSingleFace {
                shell: ShellKey::default(),
                faces: 3,
            },
        ];
        for err in &errors {
            assert!(!err.to_string().is_empty());
            let _: &dyn std::error::Error = err;
        }
    }

    #[test]
    fn isomorphic_smoke_check_for_the_module() {
        // Two identical grow-shrink histories agree deeply, not just
        // canonically (replay determinism extends to the kill ops).
        let run = || {
            let (mut body, _seed, seg, split) = ops_pillow();
            let cut = body
                .mef(MefSite::Chords {
                    he1: split.he_minus,
                    he2: seg.he_plus,
                })
                .unwrap();
            body.kef(cut.he_minus).unwrap();
            body.kev(seg.he_plus).unwrap(); // pillow → circular-edge body
            body
        };
        let a = run();
        let b = run();
        assert_eq!(deep_snapshot(&a), deep_snapshot(&b));
        assert!(isomorphic(&a, &b));
    }
}
