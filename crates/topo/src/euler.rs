//! Euler operators — the sanctioned construction path for topology (D1).
//!
//! This module implements the make-direction operators [`Body::mvfs`],
//! [`Body::mev`], and [`Body::mef`] (Mäntylä ch. 9 semantics, ch. 11
//! surgeries re-derived under our orientation convention), plus the
//! addressing helper [`Body::find_half_edge`]. The ring/genus operators
//! (`kemr`, `mekr`, `kfmrh`) are M1 PR 3; the kill-direction duals are
//! PR 4; the raw-insertion builder retreats behind these operators at
//! PR 5.
//!
//! # Operator contracts (uniform across all ops)
//!
//! - **Atomic.** Every precondition is validated *before the first
//!   mutation*; on `Err` the body is untouched. Failures are typed
//!   ([`EulerOpError`], closed enum) — never panics (D9). Checks run in
//!   the documented order per op, and the first failure is returned.
//! - **Tier-1-valid input assumed.** The operators are specified on
//!   euler-valid bodies (what [`crate::validate`] accepts). Corrupt
//!   input is tolerated only to the D9 extent — no panic, no hang (the
//!   bounded walks guarantee that), and a typed error where corruption
//!   is cheaply detectable — but the *output* on corrupt input carries
//!   no validity promise.
//! - **Deterministic minting order** (D9 + lineage replay): each op's
//!   doc comment states the exact arena-insertion order of everything it
//!   mints. Two bodies built by identical operator sequences mint
//!   identical key sequences.
//! - **D5 provenance:** every topology entity minted by a call records
//!   that call — the operator and its argument keys — as a typed
//!   [`Provenance`] variant ([`Provenance::Mvfs`] / [`Provenance::Mev`] /
//!   [`Provenance::Mef`]).
//! - **Debug postconditions** (D1's ratified clause): under
//!   `cfg(debug_assertions)`, each successful op asserts that the arena
//!   count deltas match its Euler vector and that the whole body still
//!   passes tier-1 [`crate::validate::validate`]. A firing postcondition
//!   is a kernel bug by definition (the per-call instance of the ch. 9
//!   soundness theorem failing against our transcription), never an
//!   input failure — this is the one legitimate panic site in the crate.
//!
//! # Geometry policy at M1
//!
//! Real geometry attaches at M2; the ops keep the geometry arenas
//! coherent with placeholders:
//!
//! - `mvfs`/`mev` insert the given [`Point3`] as a new point (only
//!   vertex-creating operators carry coordinates — Mäntylä ch. 11).
//!   `mvfs` also mints a placeholder surface for its face and `mev` a
//!   placeholder curve for its edge, both anchored at the given point.
//! - `mef` mints a placeholder curve anchored at the coordinates of
//!   `start(he1)`'s point (the lone vertex's point for
//!   [`MefSite::Lone`]) — deterministic and documented, no geometric
//!   meaning. The new face **shares** the old face's `SurfaceKey`: a
//!   face split is two regions of one surface, so sharing is
//!   semantically right, not a shortcut.
//! - The new face joins the old face's shell (membership plus
//!   back-pointer).
//!
//! # Orientation: how the book's surgeries were adapted
//!
//! Mäntylä's GWB orients face loops **clockwise** viewed from outside;
//! we ratified counterclockwise (interior-left — see [`crate::entity`]).
//! Every orbit-order-sensitive detail below was re-derived from our
//! convention rather than transcribed:
//!
//! - The fan run of [`MevSite::Fan`] is defined along **our** orbit step
//!   `next(mate(·))`, which walks **clockwise** viewed from outside
//!   (derived and pinned in PR 1). The pointer surgery is combinatorially
//!   the same as `lmev`'s; the *geometric reading* of "from `he1` to
//!   `he2`" is mirrored relative to the book's figures.
//! - **Edge direction for `mev` deviates from Mäntylä**: our `he_plus`
//!   (the intrinsic direction, [`crate::Edge`]) runs **old vertex → new
//!   vertex**; `lmev` orients the new edge new → old. Chosen for
//!   readability ("the edge grows away from where you applied it");
//!   everything downstream reads direction off `he_plus`, so only this
//!   one site had to pick.
//! - Edge direction for `mef` keeps Mäntylä's association:
//!   `start(he1) → start(he2)` is `he_plus`, and **`he1`'s side becomes
//!   the new face's outer loop** — both orientation-neutral, so there
//!   was nothing to mirror.
//!
//! # Example: skeletal body → segment → digon pillow
//!
//! One `mvfs`, one `mev`, one `mef` build the minimal closed solid (two
//! vertices, two edges, two faces — the digon pillow):
//!
//! ```
//! use geom_core::Point3;
//! use topo::{Body, MefSite, MevSite};
//!
//! # fn run() -> Result<(), topo::EulerOpError> {
//! let mut body = Body::<f64>::new();
//! // The skeletal body: one face whose outer loop is a lone vertex.
//! let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0))?;
//! // Grow the lone vertex into a segment edge v → w.
//! let seg = body.mev(
//!     MevSite::Lone { r#loop: seed.r#loop },
//!     Point3::new(1.0, 0.0, 0.0),
//! )?;
//! // Split the loop with a second v–w edge: the segment closes into a
//! // two-edge, two-face pillow — the smallest closed manifold body.
//! let split = body.mef(MefSite::Chords {
//!     he1: seg.he_plus,
//!     he2: seg.he_minus,
//! })?;
//! assert_eq!(topo::validate(&body), Ok(()));
//! assert_eq!(body.vertices().count(), 2);
//! assert_eq!(body.edges().count(), 2);
//! assert_eq!(body.faces().count(), 2);
//! // The new face's outer loop is he1's side, per the documented
//! // association.
//! assert_eq!(
//!     body.get_half_edge(seg.he_plus).unwrap().parent_loop,
//!     split.r#loop,
//! );
//! # Ok(()) }
//! # run().unwrap();
//! ```

use core::fmt;

use geom_core::{Point3, Real};

use crate::body::Body;
use crate::entity::{
    Edge, EdgeKey, EntityId, Face, FaceKey, GeomRef, HalfEdge, HalfEdgeKey, Loop, LoopBoundary,
    LoopKey, Shell, ShellKey, Solid, SolidKey, Vertex, VertexKey,
};
use crate::geometry::{CurveGeom, CurveKey, PointKey, SurfaceGeom, SurfaceKey};
use crate::provenance::Provenance;

/// Where [`Body::mev`] acts: the site addressing for "make edge,
/// vertex".
///
/// The ratified typed-`Empty` loop state ([`LoopBoundary::Empty`]) means
/// the degenerate configurations are addressed **explicitly**, not
/// through placeholder half-edges: GWB's uniform "two half-edge
/// pointers" API silently accepts an empty loop's placeholder half-edge;
/// here the lone-vertex case is its own variant and a half-edge argument
/// always names a real edge segment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MevSite {
    /// Split the edge fan of the vertex both half-edges start at.
    ///
    /// `he1` and `he2` must start at the same vertex `v`. The
    /// **contiguous run of `v`'s orbit from `he1` (inclusive) to `he2`
    /// (exclusive)** is reassigned to start at the new vertex; the new
    /// edge then joins `v` to the new vertex.
    ///
    /// **Run direction (ratified here, once):** the run is walked with
    /// the orbit step `next(mate(·))` — **clockwise around `v` viewed
    /// from outside** under our counterclockwise-loops convention
    /// (see [`crate::entity`]; the step itself is
    /// [`Body::vertex_orbit`]'s). Choosing the other direction would
    /// move the complementary set whenever `v` has valence ≥ 3 with
    /// `he1`/`he2` non-adjacent in the orbit; the valence-4 fan-split
    /// test pins this choice.
    ///
    /// `he1 == he2` means an **empty run**: nothing is reassigned and
    /// the new edge becomes a *strut* — a dangling edge from `v` to the
    /// new vertex, traversed twice by `he1`'s loop, spliced in
    /// immediately before `he1`.
    Fan {
        /// First half-edge of the run (inclusive); must start at the
        /// same vertex as `he2`.
        he1: HalfEdgeKey,
        /// End of the run (exclusive); must start at the same vertex as
        /// `he1`. The new edge's far end attaches here: the cycle
        /// position immediately before `he2` receives the new minus
        /// half.
        he2: HalfEdgeKey,
    },
    /// Grow a lone vertex: the loop must be [`LoopBoundary::Empty`],
    /// holding vertex `v`. The result is a *segment*: a single edge from
    /// `v` to the new vertex, and the loop becomes a two-half-edge
    /// cycle (`v`'s `emanating` goes from `None` to the new plus half).
    ///
    /// This is the state `mvfs` seeds; `mvfs` followed by `mev(Lone)`
    /// is the canonical opening of every construction.
    Lone {
        /// The empty loop to grow.
        r#loop: LoopKey,
    },
}

/// Where [`Body::mef`] acts: the site addressing for "make edge, face".
///
/// Same design as [`MevSite`]: the degenerate lone-vertex case is a
/// typed variant, not a placeholder half-edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MefSite {
    /// Split a loop by joining `start(he1)` to `start(he2)` with a new
    /// edge. Both half-edges must belong to the **same loop**.
    ///
    /// The cycle is divided so that **`he1`'s side — the half-edges from
    /// `he1` (inclusive) to `he2` (exclusive) in `next` order — becomes
    /// the NEW face's outer loop** (Mäntylä's association; it is
    /// orientation-neutral, so it survives our mirrored convention
    /// unchanged). `he2`'s side stays in the old loop, which keeps its
    /// outer/ring designation on the old face.
    ///
    /// `he1 == he2` means an empty run: the new face is a one-edge
    /// **circular (self-loop) face** at `start(he1)` — its outer loop is
    /// the single new minus half, and the new plus half is spliced into
    /// the old loop immediately before `he1`.
    Chords {
        /// First half-edge of the side that becomes the new face's
        /// outer loop (inclusive). The new edge starts at `start(he1)`.
        he1: HalfEdgeKey,
        /// End of the moved side (exclusive); stays in the old loop.
        /// The new edge ends at `start(he2)`.
        he2: HalfEdgeKey,
    },
    /// Split an empty loop: the loop must be [`LoopBoundary::Empty`],
    /// holding lone vertex `v`. The result is ch. 9's "circular edge
    /// from a lone vertex" (Fig. 9.8b): one self-loop edge at `v` whose
    /// two halves are two one-half-edge loops — the old loop keeps the
    /// plus half, the new face's outer loop gets the minus half — and a
    /// new face.
    Lone {
        /// The empty loop to split.
        r#loop: LoopKey,
    },
}

/// Every key minted by one [`Body::mvfs`] call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MvfsCreated {
    /// The new solid.
    pub solid: SolidKey,
    /// The new (only) shell of the solid.
    pub shell: ShellKey,
    /// The new (only) face of the shell.
    pub face: FaceKey,
    /// The face's outer loop: [`LoopBoundary::Empty`], holding `vertex`.
    pub r#loop: LoopKey,
    /// The new lone vertex (`emanating: None`).
    pub vertex: VertexKey,
    /// The new point carrying the given coordinates.
    pub point: PointKey,
    /// The face's placeholder surface (M1 geometry policy).
    pub surface: SurfaceKey,
}

/// Every key minted by one [`Body::mev`] call.
///
/// Direction convention (deviation from Mäntylä's `lmev`, documented in
/// the [module docs](self)): `he_plus` runs **old vertex → new vertex**.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MevCreated {
    /// The new vertex (`emanating` = `he_minus`).
    pub vertex: VertexKey,
    /// The new edge joining the old vertex to `vertex`.
    pub edge: EdgeKey,
    /// The plus half: starts at the **old** vertex, ends at `vertex`
    /// (the edge's intrinsic direction — old → new). Lands in `he1`'s
    /// loop ([`MevSite::Fan`]) or the grown loop ([`MevSite::Lone`]),
    /// spliced immediately before `he1` (before `he_minus` for a strut;
    /// the sole predecessor position for `Lone`).
    pub he_plus: HalfEdgeKey,
    /// The minus half: starts at `vertex`, ends at the old vertex.
    /// Lands in `he2`'s loop, spliced immediately before `he2`
    /// (`Fan`) or as the plus half's cycle partner (`Lone`).
    pub he_minus: HalfEdgeKey,
    /// The new point carrying the given coordinates.
    pub point: PointKey,
    /// The edge's placeholder curve (M1 geometry policy; anchored at the
    /// given point).
    pub curve: CurveKey,
}

/// Every key minted by one [`Body::mef`] call.
///
/// Direction convention (Mäntylä's, kept): `he_plus` runs
/// `start(he1) → start(he2)` and lands in the **old** loop; `he_minus`
/// lands in the new face's outer loop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MefCreated {
    /// The new face (outer loop = `r#loop`; shares the old face's
    /// surface; joins the old face's shell).
    pub face: FaceKey,
    /// The new face's outer loop: `he1`'s side of the split plus
    /// `he_minus` (its `Cycle::first`).
    pub r#loop: LoopKey,
    /// The new edge joining `start(he1)` to `start(he2)`.
    pub edge: EdgeKey,
    /// The plus half: `start(he1) → start(he2)`, in the **old** loop
    /// (which it re-anchors as `Cycle::first`).
    pub he_plus: HalfEdgeKey,
    /// The minus half: `start(he2) → start(he1)`, in the new loop.
    pub he_minus: HalfEdgeKey,
    /// The edge's placeholder curve (M1 geometry policy; anchored at
    /// `start(he1)`'s coordinates).
    pub curve: CurveKey,
}

/// A failed Euler-operator precondition. Closed enum (D3 style); the
/// body is untouched whenever one of these is returned (the operators
/// are atomic).
///
/// The `Fan`/`Loop` "broken" variants are reachable only on
/// tier-1-invalid input: the operators assume euler-valid bodies and
/// surface cheap-to-detect corruption as typed errors instead of
/// producing garbage (never a panic or a hang, per D9).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EulerOpError {
    /// An argument key, or a key the operator must follow to do its
    /// work (a `prev` link, a spine parent, a start vertex), does not
    /// resolve.
    StaleKey {
        /// The unresolvable reference, wrapped with its kind.
        key: EntityId,
    },
    /// A geometry key the operator must read (the anchor point for
    /// `mef`'s placeholder curve) does not resolve.
    StaleGeometry {
        /// The unresolvable geometry reference.
        key: GeomRef,
    },
    /// [`MevSite::Fan`]'s half-edges start at different vertices — there
    /// is no shared fan to split.
    FanStartMismatch {
        /// The first half-edge.
        he1: HalfEdgeKey,
        /// The second half-edge, starting elsewhere.
        he2: HalfEdgeKey,
    },
    /// The clockwise orbit walk from `he1` failed to close, or closed
    /// without visiting `he2` (despite the matching start vertex) —
    /// tier-1-invalid input.
    FanOrbitBroken {
        /// The half-edge the orbit was walked from.
        he1: HalfEdgeKey,
        /// The half-edge the walk never reached.
        he2: HalfEdgeKey,
    },
    /// [`MefSite::Chords`]'s half-edges belong to different loops; `mef`
    /// splits one loop (joining two loops of a face is `mekr`, PR 3;
    /// joining two faces' loops is not an Euler op at all).
    NotSameLoop {
        /// The first half-edge.
        he1: HalfEdgeKey,
        /// The second half-edge, in a different loop.
        he2: HalfEdgeKey,
    },
    /// The cycle walk from `he1` failed to close, or closed without
    /// visiting `he2` (despite the matching parent loop) —
    /// tier-1-invalid input.
    LoopCycleBroken {
        /// The loop whose cycle is broken.
        r#loop: LoopKey,
    },
    /// A `Lone` site named a loop that is not [`LoopBoundary::Empty`] —
    /// the lone-vertex operators only apply to empty loops.
    LoopNotEmpty {
        /// The non-empty loop.
        r#loop: LoopKey,
    },
    /// [`MefSite::Chords`]'s shared parent loop is [`LoopBoundary::Empty`]
    /// — half-edges claiming an empty parent are tier-1-invalid input
    /// (an empty loop reaches no half-edges).
    LoopNotCycle {
        /// The empty loop claimed as parent.
        r#loop: LoopKey,
    },
}

impl fmt::Display for EulerOpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StaleKey { key } => {
                write!(f, "euler op requires {key}, which does not resolve")
            }
            Self::StaleGeometry { key } => {
                write!(f, "euler op requires {key}, which does not resolve")
            }
            Self::FanStartMismatch { he1, he2 } => write!(
                f,
                "mev fan: half-edges {he1:?} and {he2:?} start at different \
                 vertices"
            ),
            Self::FanOrbitBroken { he1, he2 } => write!(
                f,
                "mev fan: the clockwise vertex orbit from {he1:?} never \
                 reaches {he2:?} (malformed body)"
            ),
            Self::NotSameLoop { he1, he2 } => write!(
                f,
                "mef: half-edges {he1:?} and {he2:?} belong to different loops"
            ),
            Self::LoopCycleBroken { r#loop } => write!(
                f,
                "mef: loop {loop:?}'s cycle walk never reaches the second \
                 half-edge (malformed body)",
                loop = r#loop
            ),
            Self::LoopNotEmpty { r#loop } => write!(
                f,
                "lone-vertex site: loop {loop:?} is not an empty loop",
                loop = r#loop
            ),
            Self::LoopNotCycle { r#loop } => write!(
                f,
                "mef: the half-edges' parent loop {loop:?} is an empty loop \
                 (malformed body)",
                loop = r#loop
            ),
        }
    }
}

impl std::error::Error for EulerOpError {}

/// The seven topology-arena lengths, captured for the debug
/// postcondition's Euler-vector check.
#[cfg(debug_assertions)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ArenaCounts {
    solids: usize,
    shells: usize,
    faces: usize,
    loops: usize,
    half_edges: usize,
    edges: usize,
    vertices: usize,
}

#[cfg(debug_assertions)]
impl ArenaCounts {
    /// The counts shifted by an op's Euler vector
    /// `(Δsolids, Δshells, Δfaces, Δloops, Δhalf-edges, Δedges, Δvertices)`.
    fn plus(self, delta: (usize, usize, usize, usize, usize, usize, usize)) -> Self {
        Self {
            solids: self.solids + delta.0,
            shells: self.shells + delta.1,
            faces: self.faces + delta.2,
            loops: self.loops + delta.3,
            half_edges: self.half_edges + delta.4,
            edges: self.edges + delta.5,
            vertices: self.vertices + delta.6,
        }
    }
}

impl<T: Real> Body<T> {
    /// MVFS — *make vertex, face, solid*: the initialization operator.
    ///
    /// Creates the skeletal body from scratch: a new solid with one
    /// shell, one face whose outer loop is an **empty loop**
    /// ([`LoopBoundary::Empty`]) holding one lone vertex at `point`.
    /// This is the boundary-model form of the skeletal plane model
    /// (Mäntylä §9.2.2) — the start state of every Euler construction.
    ///
    /// Euler vector: `(v +1, e 0, f +1, h 0, r 0, s +1)` — arena deltas
    /// +1 solid, +1 shell, +1 face, +1 loop, +1 vertex. (The shell is
    /// our entity, absent in GWB, where a "solid" is one connected
    /// boundary; multi-shell solids arrive at M3.)
    ///
    /// **Minting order** (D9, exact): point, surface, vertex, solid,
    /// shell, loop, face. The placeholder surface is anchored at
    /// `point`.
    ///
    /// # Errors
    ///
    /// None today — `mvfs` has no preconditions (it consumes nothing).
    /// The `Result` keeps the operator signatures uniform.
    pub fn mvfs(&mut self, point: Point3<T>) -> Result<MvfsCreated, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        let point_key = self.add_point(point);
        let surface = self.add_surface(SurfaceGeom::Placeholder { anchor: point });
        let vertex = self.add_vertex(
            Vertex {
                point: point_key,
                emanating: None,
            },
            Provenance::Mvfs,
        );
        let solid = self.add_solid(Solid { shells: vec![] }, Provenance::Mvfs);
        let shell = self.add_shell(
            Shell {
                faces: vec![],
                solid,
            },
            Provenance::Mvfs,
        );
        let r#loop = self.add_loop(
            Loop {
                boundary: LoopBoundary::Empty { vertex },
                // Provisional: the face does not exist yet (the loop ↔
                // face references are mutually cyclic); patched below.
                face: FaceKey::default(),
            },
            Provenance::Mvfs,
        );
        let face = self.add_face(
            Face {
                surface,
                outer: r#loop,
                rings: vec![],
                shell,
            },
            Provenance::Mvfs,
        );
        // Close the cyclic references. Every patched key was minted five
        // lines up; the lookups cannot fail.
        if let Some(l) = self.get_loop_mut(r#loop) {
            l.face = face;
        }
        if let Some(s) = self.get_shell_mut(shell) {
            s.faces.push(face);
        }
        if let Some(s) = self.get_solid_mut(solid) {
            s.shells.push(shell);
        }

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(before, (1, 1, 1, 1, 0, 0, 1), "mvfs");
        Ok(MvfsCreated {
            solid,
            shell,
            face,
            r#loop,
            vertex,
            point: point_key,
            surface,
        })
    }

    /// MEV — *make edge, vertex*: split a vertex's edge fan (or grow a
    /// lone vertex) with a new vertex at `point` joined to the old
    /// vertex by a new edge.
    ///
    /// Site semantics, run direction (clockwise), and the degenerate
    /// strut/segment cases: [`MevSite`]. Direction convention:
    /// `he_plus` runs **old → new** (deviation from Mäntylä's `lmev`,
    /// see the [module docs](self)).
    ///
    /// Euler vector: `(v +1, e +1, f 0, h 0, r 0, s 0)` — arena deltas
    /// +1 vertex, +1 edge, +2 half-edges.
    ///
    /// **Minting order** (D9, exact): point, curve (placeholder,
    /// anchored at `point`), vertex, edge, `he_plus`, `he_minus`.
    ///
    /// **Emanating rule** (deterministic, unconditional): after `mev`,
    /// the old vertex's `emanating` is `he_plus` and the new vertex's is
    /// `he_minus` — whether or not the fan move stripped the old
    /// vertex's previous anchor. (Which half-edge `emanating` names is
    /// documented as arbitrary; the unconditional overwrite keeps the
    /// rule branch-free and replay-deterministic.)
    ///
    /// # Surgery (Fan, `he1 != he2`)
    ///
    /// Everything in the clockwise orbit run `[he1 .. he2)` is
    /// reassigned to start at the new vertex `w`; `he_plus` is spliced
    /// immediately before `he1` (in `he1`'s loop), `he_minus`
    /// immediately before `he2` (in `he2`'s loop):
    ///
    /// ```text
    ///        before                          after
    ///     \  |  /                        \  |  /
    ///      \ | /  ← run [he1..he2)        \ | /
    ///        v                              w  ← new vertex
    ///       / \                        minus↓↑plus  ← new edge
    ///    he2   (rest of fan)                v
    ///                                      / \
    ///                                   he2   (rest of fan)
    /// ```
    ///
    /// For a strut (`he1 == he2`, empty run) both new halves land
    /// before `he1`, plus first: `… → he_plus → he_minus → he1 → …`
    /// (`v → w → v`, the edge traversed twice by one loop).
    ///
    /// # Precondition check order
    ///
    /// `Fan`: `he1` resolves, `he2` resolves ([`EulerOpError::StaleKey`]);
    /// equal start vertices ([`EulerOpError::FanStartMismatch`]); the
    /// start vertex resolves (`StaleKey`); the orbit from `he1` reaches
    /// `he2` ([`EulerOpError::FanOrbitBroken`]); both `prev` links
    /// resolve (`StaleKey`). `Lone`: the loop resolves (`StaleKey`); it
    /// is empty ([`EulerOpError::LoopNotEmpty`]); its vertex resolves
    /// (`StaleKey`).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn mev(&mut self, site: MevSite, point: Point3<T>) -> Result<MevCreated, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        let created = match site {
            MevSite::Fan { he1, he2 } => self.mev_fan(site, he1, he2, point),
            MevSite::Lone { r#loop } => self.mev_lone(site, r#loop, point),
        }?;

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(before, (0, 0, 0, 0, 2, 1, 1), "mev");
        Ok(created)
    }

    /// MEF — *make edge, face*: split a loop (or an empty loop) with a
    /// new edge, creating a new face.
    ///
    /// Site semantics, the `he1`-side-becomes-new-face association, and
    /// the degenerate self-loop/circular cases: [`MefSite`]. Direction
    /// convention: `he_plus` runs `start(he1) → start(he2)` and stays in
    /// the old loop (Mäntylä's association, kept).
    ///
    /// Euler vector: `(v 0, e +1, f +1, h 0, r 0, s 0)` — arena deltas
    /// +1 edge, +1 face, +1 loop, +2 half-edges.
    ///
    /// **Minting order** (D9, exact): curve (placeholder, anchored at
    /// `start(he1)`'s coordinates — the lone vertex's for
    /// [`MefSite::Lone`]), edge, loop, face, `he_plus`, `he_minus`.
    ///
    /// The new face shares the old face's surface and joins its shell
    /// (M1 geometry policy, module docs). `mef` does **not** reclassify
    /// the old face's rings (they all stay on the old face — Mäntylä
    /// p. 192; the `ring_move` helper is PR 3), and it touches
    /// `emanating` only in the `Lone` case (the lone vertex gains
    /// `he_plus`); in the `Chords` case every vertex keeps its anchor
    /// (no half-edge changes its start vertex).
    ///
    /// Both loops are re-anchored deterministically: the old loop's
    /// `Cycle::first` becomes `he_plus` (its previous first may have
    /// moved to the new loop), the new loop's is `he_minus`.
    ///
    /// # Surgery (Chords, `he1 != he2`)
    ///
    /// The run `[he1 .. he2)` in `next` order moves to the new loop;
    /// `he_minus` is spliced before `he1`, `he_plus` before `he2`, and
    /// the cycle splits in two:
    ///
    /// ```text
    ///          before                            after
    ///    ┌── he1 ──────┐                  ┌── he1 ──────┐
    ///    │             ↓                  │             ↓
    ///    │  (one loop) │            he_minus  NEW LOOP  │
    ///    ↑             │                  ↑             │
    ///    └────── he2 ──┘                  └─────────────┘
    ///                                     ┌── he2 ──────┐
    ///                                     │             ↓
    ///                                he_plus   OLD LOOP │
    ///                                     ↑             │
    ///                                     └─────────────┘
    /// ```
    ///
    /// For `he1 == he2` (empty run) the new loop is the single
    /// self-cycled `he_minus`, and `he_plus` is spliced before `he1` in
    /// the old loop — the one-edge circular face. For [`MefSite::Lone`]
    /// both halves are self-cycled one-half-edge loops at the lone
    /// vertex: the old loop keeps `he_plus`, the new loop `he_minus`.
    ///
    /// # Precondition check order
    ///
    /// `Chords`: `he1` resolves, `he2` resolves
    /// ([`EulerOpError::StaleKey`]); same parent loop
    /// ([`EulerOpError::NotSameLoop`]); the loop resolves (`StaleKey`);
    /// it is a cycle ([`EulerOpError::LoopNotCycle`]); the cycle walk
    /// from `he1` reaches `he2` ([`EulerOpError::LoopCycleBroken`]);
    /// both `prev` links resolve (`StaleKey`); the loop's face and the
    /// face's shell resolve (`StaleKey`); `start(he1)` and its point
    /// resolve (`StaleKey` / [`EulerOpError::StaleGeometry`]). `Lone`:
    /// the loop resolves; it is empty ([`EulerOpError::LoopNotEmpty`]);
    /// its vertex and point resolve; its face and shell resolve.
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn mef(&mut self, site: MefSite) -> Result<MefCreated, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        let created = match site {
            MefSite::Chords { he1, he2 } => self.mef_chords(site, he1, he2),
            MefSite::Lone { r#loop } => self.mef_lone(site, r#loop),
        }?;

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(before, (0, 0, 1, 1, 2, 1, 0), "mef");
        Ok(created)
    }

    /// Finds the half-edge running `from → to` in `face`, or `None`.
    ///
    /// The searching counterpart of GWB's `fhe` (Program 11.9): tests
    /// and hand construction address half-edges by
    /// `(face, start vertex, end vertex)`; operators take half-edge keys
    /// directly (arenas make keys stable handles, so GWB's id-scan layer
    /// is dropped).
    ///
    /// **Deterministic scan order** (D9): the face's outer loop first,
    /// then its rings in list order; within each loop, cycle (`next`)
    /// order starting at the loop's `Cycle::first`. The first match
    /// wins. Empty loops hold no half-edges and are skipped; on a
    /// malformed body, loops whose keys or cycles do not resolve are
    /// skipped too (total, never panics), so `None` means "not found or
    /// not reachable".
    pub fn find_half_edge(
        &self,
        face: FaceKey,
        from: VertexKey,
        to: VertexKey,
    ) -> Option<HalfEdgeKey> {
        let face_data = self.get_face(face)?;
        for loop_key in core::iter::once(face_data.outer).chain(face_data.rings.iter().copied()) {
            let Some(loop_data) = self.get_loop(loop_key) else {
                continue;
            };
            let LoopBoundary::Cycle { first } = loop_data.boundary else {
                continue;
            };
            let Some(cycle) = self.loop_cycle(first) else {
                continue;
            };
            for he in cycle {
                let Some(he_data) = self.get_half_edge(he) else {
                    continue;
                };
                if he_data.start == from && self.half_edge_end(he) == Some(to) {
                    return Some(he);
                }
            }
        }
        None
    }

    // ------------------------------------------------------------------
    // mev implementation
    // ------------------------------------------------------------------

    /// [`MevSite::Fan`] — precondition block, then the fan surgery.
    fn mev_fan(
        &mut self,
        site: MevSite,
        he1: HalfEdgeKey,
        he2: HalfEdgeKey,
        point: Point3<T>,
    ) -> Result<MevCreated, EulerOpError> {
        // ---- Preconditions: no mutation until every check passes. ----
        let he1_data = self.resolve_half_edge(he1)?;
        let (v, he1_prev, he1_loop) = (he1_data.start, he1_data.prev, he1_data.parent_loop);
        let he2_data = self.resolve_half_edge(he2)?;
        let (he2_start, he2_prev, he2_loop) = (he2_data.start, he2_data.prev, he2_data.parent_loop);
        if he2_start != v {
            return Err(EulerOpError::FanStartMismatch { he1, he2 });
        }
        if !self.vertices.contains_key(v) {
            // The op rewrites v's emanating; a dangling start vertex is
            // tier-1-invalid input caught here.
            return Err(EulerOpError::StaleKey {
                key: EntityId::Vertex(v),
            });
        }
        // The clockwise run [he1 .. he2): members of the next(mate(·))
        // orbit walk. The walk is bounded (D9) and resolves every member
        // it returns.
        let run: Vec<HalfEdgeKey> = if he1 == he2 {
            Vec::new() // strut: empty run, no walk needed
        } else {
            let orbit = self
                .vertex_orbit(he1)
                .ok_or(EulerOpError::FanOrbitBroken { he1, he2 })?;
            let position = orbit
                .iter()
                .position(|&he| he == he2)
                .ok_or(EulerOpError::FanOrbitBroken { he1, he2 })?;
            orbit[..position].to_vec()
        };
        // The splice writes through both prev links; validate them now
        // so the mutation below cannot fail midway (atomicity).
        for prev in [he1_prev, he2_prev] {
            if !self.half_edges.contains_key(prev) {
                return Err(EulerOpError::StaleKey {
                    key: EntityId::HalfEdge(prev),
                });
            }
        }

        // ---- Mutation (infallible from here on). ----
        // Minting order (documented on `mev`): point, curve, vertex,
        // edge, he_plus, he_minus.
        let provenance = Provenance::Mev { site };
        let point_key = self.add_point(point);
        let curve = self.add_curve(CurveGeom::Placeholder { anchor: point });
        let w = self.add_vertex(
            Vertex {
                point: point_key,
                emanating: None, // patched below
            },
            provenance.clone(),
        );
        let edge = self.mint_edge(curve, &provenance);
        let (he_plus, he_minus) = self.mint_halves(
            edge,
            // he_plus: old vertex → new vertex (our deviation from
            // lmev's new → old), spliced into he1's loop.
            (v, he1_loop),
            // he_minus: new vertex → old vertex, spliced into he2's loop.
            (w, he2_loop),
            &provenance,
        );

        // Splice. Derived (module docs) rather than transcribed; the two
        // cases are the sequential "insert before he1, then before he2"
        // with the strut's second insertion landing between the first
        // and he1.
        if he1 == he2 {
            // Strut: … → prev → he_plus → he_minus → he1 → …
            self.link_half_edges(he1_prev, he_plus);
            self.link_half_edges(he_plus, he_minus);
            self.link_half_edges(he_minus, he1);
        } else {
            // … → prev(he1) → he_plus → he1 → …  (in he1's loop)
            // … → prev(he2) → he_minus → he2 → … (in he2's loop)
            self.link_half_edges(he1_prev, he_plus);
            self.link_half_edges(he_plus, he1);
            self.link_half_edges(he2_prev, he_minus);
            self.link_half_edges(he_minus, he2);
        }
        // Reassign the clockwise run to the new vertex.
        for &moved in &run {
            if let Some(he) = self.get_half_edge_mut(moved) {
                he.start = w;
            }
        }
        // Emanating rule (documented on `mev`): unconditional.
        if let Some(vertex) = self.get_vertex_mut(v) {
            vertex.emanating = Some(he_plus);
        }
        if let Some(vertex) = self.get_vertex_mut(w) {
            vertex.emanating = Some(he_minus);
        }

        Ok(MevCreated {
            vertex: w,
            edge,
            he_plus,
            he_minus,
            point: point_key,
            curve,
        })
    }

    /// [`MevSite::Lone`] — precondition block, then the segment surgery.
    fn mev_lone(
        &mut self,
        site: MevSite,
        loop_key: LoopKey,
        point: Point3<T>,
    ) -> Result<MevCreated, EulerOpError> {
        // ---- Preconditions. ----
        let loop_data = self.get_loop(loop_key).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(loop_key),
        })?;
        let LoopBoundary::Empty { vertex: v } = loop_data.boundary else {
            return Err(EulerOpError::LoopNotEmpty { r#loop: loop_key });
        };
        if !self.vertices.contains_key(v) {
            return Err(EulerOpError::StaleKey {
                key: EntityId::Vertex(v),
            });
        }

        // ---- Mutation (infallible from here on). ----
        let provenance = Provenance::Mev { site };
        let point_key = self.add_point(point);
        let curve = self.add_curve(CurveGeom::Placeholder { anchor: point });
        let w = self.add_vertex(
            Vertex {
                point: point_key,
                emanating: None, // patched below
            },
            provenance.clone(),
        );
        let edge = self.mint_edge(curve, &provenance);
        let (he_plus, he_minus) =
            self.mint_halves(edge, (v, loop_key), (w, loop_key), &provenance);
        // The two halves form the whole cycle: v → w → v.
        self.link_half_edges(he_plus, he_minus);
        self.link_half_edges(he_minus, he_plus);
        if let Some(l) = self.get_loop_mut(loop_key) {
            l.boundary = LoopBoundary::Cycle { first: he_plus };
        }
        if let Some(vertex) = self.get_vertex_mut(v) {
            vertex.emanating = Some(he_plus);
        }
        if let Some(vertex) = self.get_vertex_mut(w) {
            vertex.emanating = Some(he_minus);
        }

        Ok(MevCreated {
            vertex: w,
            edge,
            he_plus,
            he_minus,
            point: point_key,
            curve,
        })
    }

    // ------------------------------------------------------------------
    // mef implementation
    // ------------------------------------------------------------------

    /// [`MefSite::Chords`] — precondition block, then the loop-split
    /// surgery.
    fn mef_chords(
        &mut self,
        site: MefSite,
        he1: HalfEdgeKey,
        he2: HalfEdgeKey,
    ) -> Result<MefCreated, EulerOpError> {
        // ---- Preconditions. ----
        let he1_data = self.resolve_half_edge(he1)?;
        let (u1, he1_prev, loop_key) = (he1_data.start, he1_data.prev, he1_data.parent_loop);
        let he2_data = self.resolve_half_edge(he2)?;
        let (u2, he2_prev) = (he2_data.start, he2_data.prev);
        if he2_data.parent_loop != loop_key {
            return Err(EulerOpError::NotSameLoop { he1, he2 });
        }
        let loop_data = self.get_loop(loop_key).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(loop_key),
        })?;
        if matches!(loop_data.boundary, LoopBoundary::Empty { .. }) {
            return Err(EulerOpError::LoopNotCycle { r#loop: loop_key });
        }
        let face_key = loop_data.face;
        // The run [he1 .. he2) in next order — he1's side of the split.
        // Walked from he1 itself (not the loop's first), bounded (D9).
        let run: Vec<HalfEdgeKey> = if he1 == he2 {
            Vec::new() // circular one-edge face: empty run
        } else {
            let cycle = self
                .loop_cycle(he1)
                .ok_or(EulerOpError::LoopCycleBroken { r#loop: loop_key })?;
            let position = cycle
                .iter()
                .position(|&he| he == he2)
                .ok_or(EulerOpError::LoopCycleBroken { r#loop: loop_key })?;
            cycle[..position].to_vec()
        };
        for prev in [he1_prev, he2_prev] {
            if !self.half_edges.contains_key(prev) {
                return Err(EulerOpError::StaleKey {
                    key: EntityId::HalfEdge(prev),
                });
            }
        }
        let face_data = self.get_face(face_key).ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(face_key),
        })?;
        let (surface, shell_key) = (face_data.surface, face_data.shell);
        if !self.shells.contains_key(shell_key) {
            return Err(EulerOpError::StaleKey {
                key: EntityId::Shell(shell_key),
            });
        }
        let anchor = self.resolve_vertex_point(u1)?;

        // ---- Mutation (infallible from here on). ----
        // Minting order (documented on `mef`): curve, edge, loop, face,
        // he_plus, he_minus.
        let provenance = Provenance::Mef { site };
        let curve = self.add_curve(CurveGeom::Placeholder { anchor });
        let edge = self.mint_edge(curve, &provenance);
        let (new_loop, new_face) = self.mint_loop_and_face(surface, shell_key, &provenance);
        let (he_plus, he_minus) = self.mint_halves(
            edge,
            // he_plus: start(he1) → start(he2), in the OLD loop.
            (u1, loop_key),
            // he_minus: start(he2) → start(he1), in the NEW loop.
            (u2, new_loop),
            &provenance,
        );

        // Splice (derivation in the module docs — Mäntylä's tail swap,
        // re-derived): he_minus closes he1's side into the new loop,
        // he_plus closes he2's side into the old loop.
        if he1 == he2 {
            // Circular one-edge face: the new loop is he_minus alone.
            self.link_half_edges(he_minus, he_minus);
            self.link_half_edges(he1_prev, he_plus);
            self.link_half_edges(he_plus, he1);
        } else {
            // New loop: … → prev(he2) → he_minus → he1 → … (he1's side)
            // Old loop: … → prev(he1) → he_plus → he2 → … (he2's side)
            self.link_half_edges(he2_prev, he_minus);
            self.link_half_edges(he_minus, he1);
            self.link_half_edges(he1_prev, he_plus);
            self.link_half_edges(he_plus, he2);
        }
        // Move he1's side into the new loop.
        for &moved in &run {
            if let Some(he) = self.get_half_edge_mut(moved) {
                he.parent_loop = new_loop;
            }
        }
        // Re-anchor both loops deterministically (the old loop's first
        // may have migrated to the new loop).
        if let Some(l) = self.get_loop_mut(loop_key) {
            l.boundary = LoopBoundary::Cycle { first: he_plus };
        }
        if let Some(l) = self.get_loop_mut(new_loop) {
            l.boundary = LoopBoundary::Cycle { first: he_minus };
        }

        Ok(MefCreated {
            face: new_face,
            r#loop: new_loop,
            edge,
            he_plus,
            he_minus,
            curve,
        })
    }

    /// [`MefSite::Lone`] — precondition block, then the circular-edge
    /// surgery ("circular edge from a lone vertex", Mäntylä Fig. 9.8b).
    fn mef_lone(&mut self, site: MefSite, loop_key: LoopKey) -> Result<MefCreated, EulerOpError> {
        // ---- Preconditions. ----
        let loop_data = self.get_loop(loop_key).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(loop_key),
        })?;
        let LoopBoundary::Empty { vertex: v } = loop_data.boundary else {
            return Err(EulerOpError::LoopNotEmpty { r#loop: loop_key });
        };
        let face_key = loop_data.face;
        if !self.vertices.contains_key(v) {
            return Err(EulerOpError::StaleKey {
                key: EntityId::Vertex(v),
            });
        }
        let anchor = self.resolve_vertex_point(v)?;
        let face_data = self.get_face(face_key).ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(face_key),
        })?;
        let (surface, shell_key) = (face_data.surface, face_data.shell);
        if !self.shells.contains_key(shell_key) {
            return Err(EulerOpError::StaleKey {
                key: EntityId::Shell(shell_key),
            });
        }

        // ---- Mutation (infallible from here on). ----
        // Same minting order as Chords: curve, edge, loop, face,
        // he_plus, he_minus.
        let provenance = Provenance::Mef { site };
        let curve = self.add_curve(CurveGeom::Placeholder { anchor });
        let edge = self.mint_edge(curve, &provenance);
        let (new_loop, new_face) = self.mint_loop_and_face(surface, shell_key, &provenance);
        let (he_plus, he_minus) =
            self.mint_halves(edge, (v, loop_key), (v, new_loop), &provenance);
        // Both halves are one-half-edge loops at v: the old loop keeps
        // he_plus, the new face's outer loop gets he_minus (the same
        // association as Chords — he1's "side" is the new loop).
        self.link_half_edges(he_plus, he_plus);
        self.link_half_edges(he_minus, he_minus);
        if let Some(l) = self.get_loop_mut(loop_key) {
            l.boundary = LoopBoundary::Cycle { first: he_plus };
        }
        if let Some(l) = self.get_loop_mut(new_loop) {
            l.boundary = LoopBoundary::Cycle { first: he_minus };
        }
        // The lone vertex gains its first half-edge (the only case where
        // mef touches emanating).
        if let Some(vertex) = self.get_vertex_mut(v) {
            vertex.emanating = Some(he_plus);
        }

        Ok(MefCreated {
            face: new_face,
            r#loop: new_loop,
            edge,
            he_plus,
            he_minus,
            curve,
        })
    }

    // ------------------------------------------------------------------
    // Shared internals
    // ------------------------------------------------------------------

    /// Resolves a half-edge argument, copying out its fields
    /// ([`EulerOpError::StaleKey`] if it does not resolve).
    fn resolve_half_edge(&self, he: HalfEdgeKey) -> Result<HalfEdge, EulerOpError> {
        self.get_half_edge(he)
            .cloned()
            .ok_or(EulerOpError::StaleKey {
                key: EntityId::HalfEdge(he),
            })
    }

    /// Resolves a vertex's point coordinates (for `mef`'s placeholder
    /// curve anchor): [`EulerOpError::StaleKey`] on the vertex,
    /// [`EulerOpError::StaleGeometry`] on the point.
    fn resolve_vertex_point(&self, vertex: VertexKey) -> Result<Point3<T>, EulerOpError> {
        let vertex_data = self.get_vertex(vertex).ok_or(EulerOpError::StaleKey {
            key: EntityId::Vertex(vertex),
        })?;
        self.get_point(vertex_data.point)
            .copied()
            .ok_or(EulerOpError::StaleGeometry {
                key: GeomRef::Point(vertex_data.point),
            })
    }

    /// Mints an edge with provisional half-edge slots (the halves are
    /// minted next by [`Body::mint_halves`], which patches the slots).
    fn mint_edge(&mut self, curve: CurveKey, provenance: &Provenance) -> EdgeKey {
        self.add_edge(
            Edge {
                // Provisional: the halves do not exist yet; patched by
                // mint_halves.
                he_plus: HalfEdgeKey::default(),
                he_minus: HalfEdgeKey::default(),
                curve,
            },
            provenance.clone(),
        )
    }

    /// Mints an edge's two halves (`he_plus` first, then `he_minus` —
    /// part of every op's documented minting order) and wires the
    /// edge ↔ half-edge bijection. Each half is described by its
    /// `(start vertex, parent loop)` pair; `next`/`prev` are left
    /// provisional (null keys) for the caller's splice.
    fn mint_halves(
        &mut self,
        edge: EdgeKey,
        plus: (VertexKey, LoopKey),
        minus: (VertexKey, LoopKey),
        provenance: &Provenance,
    ) -> (HalfEdgeKey, HalfEdgeKey) {
        let half = |(start, parent_loop): (VertexKey, LoopKey)| HalfEdge {
            edge,
            start,
            parent_loop,
            next: HalfEdgeKey::default(), // provisional; caller splices
            prev: HalfEdgeKey::default(), // provisional; caller splices
        };
        let he_plus = self.add_half_edge(half(plus), provenance.clone());
        let he_minus = self.add_half_edge(half(minus), provenance.clone());
        if let Some(e) = self.get_edge_mut(edge) {
            e.he_plus = he_plus;
            e.he_minus = he_minus;
        }
        (he_plus, he_minus)
    }

    /// Mints `mef`'s new loop and face (in that order — part of `mef`'s
    /// documented minting order) and joins the new face to the old
    /// face's shell. The loop's boundary anchor is provisional; the
    /// caller re-anchors it after the splice.
    fn mint_loop_and_face(
        &mut self,
        surface: SurfaceKey,
        shell: ShellKey,
        provenance: &Provenance,
    ) -> (LoopKey, FaceKey) {
        let new_loop = self.add_loop(
            Loop {
                // Provisional in both fields: the face is minted next
                // (cyclic reference), and the boundary's first half-edge
                // is spliced by the caller.
                boundary: LoopBoundary::Cycle {
                    first: HalfEdgeKey::default(),
                },
                face: FaceKey::default(),
            },
            provenance.clone(),
        );
        let new_face = self.add_face(
            Face {
                surface, // shared with the old face (M1 geometry policy)
                outer: new_loop,
                rings: vec![],
                shell,
            },
            provenance.clone(),
        );
        if let Some(l) = self.get_loop_mut(new_loop) {
            l.face = new_face;
        }
        if let Some(s) = self.get_shell_mut(shell) {
            s.faces.push(new_face);
        }
        (new_loop, new_face)
    }

    /// Writes the mutual `next`/`prev` link `a → b`. Both keys were
    /// pre-validated (or freshly minted) by the caller; the lookups
    /// cannot fail on the operator paths.
    fn link_half_edges(&mut self, a: HalfEdgeKey, b: HalfEdgeKey) {
        if let Some(he) = self.get_half_edge_mut(a) {
            he.next = b;
        }
        if let Some(he) = self.get_half_edge_mut(b) {
            he.prev = a;
        }
    }

    /// Captures the topology-arena lengths for the debug postcondition.
    #[cfg(debug_assertions)]
    fn arena_counts(&self) -> ArenaCounts {
        ArenaCounts {
            solids: self.solids.len(),
            shells: self.shells.len(),
            faces: self.faces.len(),
            loops: self.loops.len(),
            half_edges: self.half_edges.len(),
            edges: self.edges.len(),
            vertices: self.vertices.len(),
        }
    }

    /// D1's ratified postcondition-assert clause: after a successful
    /// operator, the arena deltas must match the op's Euler vector and
    /// the body must be tier-1 valid. A failure here is a kernel bug (a
    /// per-call violation of the ch. 9 soundness theorem by our
    /// transcription), never an input failure — the one legitimate
    /// panic site (debug builds only).
    #[cfg(debug_assertions)]
    fn assert_euler_postcondition(
        &self,
        before: ArenaCounts,
        delta: (usize, usize, usize, usize, usize, usize, usize),
        op: &str,
    ) {
        debug_assert_eq!(
            self.arena_counts(),
            before.plus(delta),
            "{op} postcondition: arena deltas do not match the Euler vector \
             (kernel bug)",
        );
        debug_assert_eq!(
            crate::validate::validate(self),
            Ok(()),
            "{op} postcondition: result is not tier-1 valid (kernel bug)",
        );
    }
}
