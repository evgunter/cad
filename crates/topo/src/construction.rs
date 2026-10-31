//! **Topo's construction tier** (DESIGN D4 ¶1 (iv); Ev on PR 5838): the
//! Euler operators, the edge and face description setters and the row
//! and joint doors, as one trait over [`Body`]. It is the kernel's
//! internal API: every curve-and-face pair these doors read is one a
//! kernel crate composed, so a miss there is the kernel's defect.
//!
//! The doors are trait methods, not inherent ones, so the curated
//! façade (`pncad::topo`) can leave them out: an inherent method
//! travels with `Body` through any re-export that names the type.
//! A kernel crate brings the trait into scope (`use
//! topo::construction::Construct;`). Each method is documented here and
//! forwards to the crate-private inherent door of the same name.

use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, PcurveCache};
use geom_core::{Band, Decide, Point3, Real, Tol};

use crate::boolean::{BooleanError, EdgeJoin};
use crate::{
    Body, CurveKey, EdgeKey, EulerOpError, FaceKey, HalfEdgeKey, JointElement, LoopKey, ShellKey,
    SolidKey, SurfaceKey, VertexKey,
};

// The construction tier's own vocabulary: the sites, surfaces and
// results its doors take and return, which nothing outside the tier
// names. The doors' refusal and its payloads (`EulerOpError`,
// `BadArgument`, `RechartDoor`) stay at the crate root.
pub use crate::attach::Rechart;
pub use crate::euler::{FaceSurface, MefCreated, MefSite, MevCreated, MevSite, MvfsCreated};
pub use crate::euler_kill::{KefResult, KevResult, KvfsResult, MergedMember, MfkrhCreated};
pub use crate::euler_ring::{KemrResult, KfmrhResult, MekrResult, MekrSite};
pub use crate::null::{NewVertexSide, NullEdge, NullFacePair};
pub use crate::split::SplitEdgeCreated;
pub use crate::surgery::Surgery;

/// **The construction tier, the kernel's internal API** (module docs):
/// each method forwards to the `Body` door of the same name.
pub trait Construct<T: geom_core::Real> {
    /// MVFS — *make vertex, face, solid*: the initialization operator.
    ///
    /// Creates the skeletal body from scratch: a new solid with one
    /// shell, one face whose outer loop is an **empty loop**
    /// ([`LoopBoundary::Empty`](crate::LoopBoundary::Empty)) holding one lone vertex at `point`.
    /// This is the boundary-model form of the skeletal plane model
    /// (Mäntylä §9.2.2) — the start state of every Euler construction.
    ///
    /// Euler vector: `(v +1, e 0, f +1, h 0, r 0, s +1)` — arena deltas
    /// +1 solid, +1 shell, +1 face, +1 loop, +1 vertex. (The shell is
    /// our entity, absent in GWB, where a "solid" is one connected
    /// boundary; a solid here may hold several, and does whenever a
    /// boolean leaves a void shell — [`crate::Solid`].)
    ///
    /// **Minting order** (D9, exact): point, surface, vertex, solid,
    /// shell, loop, face. The seed face's surface is the
    /// [`Surface::Nurbs`](geom::Surface::Nurbs) representable-unimplemented placeholder —
    /// the honest "no description yet" state (module docs, geometry
    /// policy): a construction attaches the real surface via
    /// [`Body::set_face_surface`](crate::construction::Construct::set_face_surface) once it exists; a body reaching rest
    /// with it fails tier 3.
    ///
    /// `sense` is the seed face's [`crate::Face::sense`], stated by the
    /// caller and provisional until it charts the face
    /// ([`Body::resolve_face_surface`]).
    ///
    /// # Errors
    ///
    /// None today — `mvfs` has no preconditions (it consumes nothing).
    /// The `Result` keeps the operator signatures uniform.
    fn mvfs(&mut self, point: Point3<T>, sense: bool) -> Result<MvfsCreated, EulerOpError>
    where
        T: Decide;
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
    /// **Geometry** (module docs, M2 policy): `curve` is certified
    /// against the endpoints **old vertex's point → `point`** (the
    /// `he_plus` forward order — `he_plus` runs old → new) before any
    /// mutation; failure is [`EulerOpError::Certification`], body
    /// untouched. Chord-line sugar: [`Body::mev_line`](crate::construction::Construct::mev_line).
    ///
    /// **Pcurve rows** ([`crate::pcurves`]): a face the new halves join
    /// whose rows are COMPLETE is not left half-minted. The loops the
    /// halves join are re-minted with them, before any mutation — the
    /// rows the minting pass would store — and its other loops keep
    /// theirs; or, where the closed-form lane cannot mint the face as
    /// the surgery leaves it, it stores nothing; on a spline chart the
    /// op refuses [`EulerOpError::PcurveMint`]. A face whose only gaps
    /// are on loops a null edge holds open is taken the same way, and a
    /// loop the null edge still holds keeps what it had, its new halves
    /// rowless; on a spline chart that face is left as found. A face
    /// storing no row stays rowless, and one half-minted any other way
    /// is left as found (`crate::pcurves::site_rows` carries the rule). The cost is one
    /// walk and one certification per half-edge of the rewired loops,
    /// one presence read per half-edge of the face's other loops, and on
    /// a face missing a row three lookups per half-edge for which loops
    /// a null edge holds open.
    ///
    /// **The moved run's carriers are re-certified, never
    /// re-described.** At a fan site the run `[he1 .. he2)` is
    /// re-based onto the new vertex `w`, and each of those edges keeps
    /// the curve it was certified with. So the gate asks whether that
    /// curve still describes the edge `w` gives it, against the
    /// endpoints of the re-based edge, and **refuses**
    /// [`EulerOpError::RebasedCarrier`] naming the edge where THIS
    /// MOVE is what makes the answer no — body untouched, like every
    /// other precondition. A carrier that already missed its own
    /// endpoint before the call is carried rather than refused: it
    /// names a defect this operation did not create. Re-describing a
    /// run is [`Body::set_edge_curve`](crate::construction::Construct::set_edge_curve)'s decision, with the caller's own
    /// spec; the gate's own docs carry the argument for why an
    /// operator re-certifies exactly rather than re-fitting. A **null
    /// edge** the run moves one end of (one of its halves in the run,
    /// the other not) is refused [`EulerOpError::RebasedNullEdge`]; one
    /// whose two halves are both in the run moves whole and is carried.
    /// The one-half refusal stands even where `point` is the old
    /// vertex's own: the gate does not ask whether `point` is
    /// that point, and its docs (the crate-internal
    /// `Body::certify_rebased_run`) say why. A fan split that moves
    /// nothing is [`Body::mev_null`](crate::construction::Construct::mev_null), which copies the old point; it
    /// leaves a null edge, and describing that edge is a second call,
    /// [`Body::set_edge_curve`](crate::construction::Construct::set_edge_curve), which can fail on its own and leave
    /// the null edge in place. The two calls are the no-move split, not
    /// one atomic door.
    ///
    /// **The variant family.** `mev` takes the new edge's spec;
    /// [`Body::mev_line`](crate::construction::Construct::mev_line) derives the chord; [`Body::mev_null`](crate::construction::Construct::mev_null) takes no
    /// geometry and moves nothing. The kill side mirrors it: [`Body::kev`](crate::construction::Construct::kev)
    /// is keys-only and refuses every merge with a certified member,
    /// one that moves no point included, and [`Body::kev_describing`](crate::construction::Construct::kev_describing)
    /// takes a band and the merged fan's re-descriptions, as this door
    /// takes its spec and band — `kev_describing(he, &[], tol)` is the
    /// merge that moves nothing, or moves within band.
    ///
    /// **Minting order** (D9, exact): point, curve (the certified
    /// [`EdgeCurve`](geom_brep::EdgeCurve)), vertex, edge, `he_plus`, `he_minus`.
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
    /// `Fan`: `he1` resolves, `he2` resolves ([`BadArgument::Stale`](crate::BadArgument::Stale));
    /// equal start vertices ([`EulerOpError::FanStartMismatch`]).
    /// `Lone`: the loop resolves ([`BadArgument::Stale`](crate::BadArgument::Stale)); it is empty
    /// ([`EulerOpError::LoopNotEmpty`]). Then, for both sites, the
    /// geometry gate: `curve` certifies ([`EulerOpError::Certification`];
    /// an interval that runs no clear length forward or past a full turn
    /// is the caller's, [`geom_brep::CertifyError::StatedInterval`]);
    /// and at a `Fan` site, per edge of the moved run in run order: the
    /// edge is not a null edge the run moves one end of
    /// ([`EulerOpError::RebasedNullEdge`]), and its carrier re-certifies
    /// against the endpoints the move gives it
    /// ([`EulerOpError::RebasedCarrier`]). The first edge of the run to
    /// fail names the refusal. Last, the pcurve rows, for both sites:
    /// where the site mint selects a face the new halves join, its row
    /// plan is minted ([`EulerOpError::PcurveMint`]).
    ///
    /// # Panics
    ///
    /// Where a record the plan reads past the arguments does not resolve
    /// or a walk of it does not close: the orbit walk from `he1` (which
    /// must reach `he2` and walk back by `mate(prev(·))`), the `prev`
    /// links, the start vertex and its point, the moved run's edges and
    /// their curves, the loops, faces and surfaces the rows read. Every
    /// public door keeps the body tier-1-valid, where all of these hold.
    ///
    /// Also on a kernel bug: an edge the body stores, re-minted or
    /// restated on its own carrier and interval, whose span or winding
    /// reads certainly refused at the run's band.
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    fn mev(
        &mut self,
        site: MevSite,
        point: Point3<T>,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<MevCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    /// [`Body::mev`](crate::construction::Construct::mev) with the chord-line spec derived from the site:
    /// the new edge's carrier is the straight chord from the old
    /// vertex's point to `point`
    /// ([`EdgeCurveSpec::line_between`] — the caller asserts the locus
    /// *is* that chord; module docs, geometry policy). Coincident
    /// endpoints give a chord of no length, refused typed
    /// ([`geom_brep::CertifyError::StatedInterval`]).
    ///
    /// # Panics
    ///
    /// On a kernel bug: an edge the body stores, re-minted or restated on its
    /// own carrier and interval, whose span or winding reads certainly
    /// refused at the run's band.
    ///
    /// # Errors
    ///
    /// As [`Body::mev`](crate::construction::Construct::mev).
    fn mev_line(
        &mut self,
        site: MevSite,
        point: Point3<T>,
        tol: Tol,
    ) -> Result<MevCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    /// MEV, null-edge form — *make (null) edge, vertex*: the zero-length
    /// `lmev` idiom of ch. 14/15 (`separ1`/`separ2`), minting a new
    /// vertex **on the site's old vertex's point** (the same
    /// `PointKey` — structural coincidence, no comparison; D1 tier 3′
    /// reads two vertices on one point as structural sharing) joined by a
    /// **null edge** whose curve entry is [`CurveGeom::NullScaffold`](crate::CurveGeom::NullScaffold)
    /// carrying the F9 side attribute (`new_side` declares which side
    /// the new vertex faces; the old vertex takes the other).
    ///
    /// Serves ch. 14 `splitclassify` null-edge insertion and ch. 15
    /// null-edge pairs (M3 PRs 2 and 4). Site semantics — fan split,
    /// strut (`he1 == he2`, ch. 15's dangling null edge), lone — and
    /// the surgery are exactly [`Body::mev`](crate::construction::Construct::mev)'s; only the geometry lane
    /// differs, and BOTH of `mev`'s geometry gates are skipped rather
    /// than passed:
    ///
    /// - the new edge's own certification, because there is no carrier
    ///   to certify, by type (the ratified F9 shape, module docs);
    /// - the re-basing gate over a fan site's moved run
    ///   ([`Body::certify_rebased_run`]), because the new vertex's
    ///   point is the old one's, so no re-based edge's
    ///   endpoint moves and every certificate is the one it had. A
    ///   structural coincidence, not a comparison — which is why this
    ///   door needs no `Tol`.
    ///
    /// Tier 1 accepts the result; tier 2 refuses it at rest
    /// ([`crate::ValidationError::NullEdgeAtRest`]).
    ///
    /// **Pcurve rows** ([`crate::pcurves`]): the null edge has no
    /// carrier to derive its halves' rows from, so the loop it joins is
    /// held open, missing those two, and the site mint cannot walk it.
    /// The door that releases the loop mints it whole, with the rows of
    /// any half an operator added to it meanwhile: an Euler operator
    /// that rewires the loop out from under the edge — the boolean's and
    /// the splitting lane's joins, whose chord `mef`s cut the section's
    /// null halves off the faces they cross — the edge's first
    /// description ([`Body::set_edge_curve`](crate::construction::Construct::set_edge_curve), or a
    /// [`Body::kev_describing`](crate::construction::Construct::kev_describing) that lists it), or the band kill of the
    /// edge ([`Body::kev_describing`](crate::construction::Construct::kev_describing), [`Body::kef_minting`](crate::construction::Construct::kef_minting),
    /// [`Body::kemr_minting`](crate::construction::Construct::kemr_minting)); on a spline chart each leaves the face as
    /// found. The keys-only kills take no band to mint it with, and
    /// refuse [`crate::pcurves::SiteRowRefusal::KeysOnly`] where the
    /// loop they release would miss a row.
    ///
    /// Euler vector: `(v +1, e +1, f 0, h 0, r 0, s 0)` — identical to
    /// `mev` (a null edge is an edge).
    ///
    /// **Minting order** (D9, exact — deviates from `mev`'s): no point,
    /// then **vertex**, curve entry (the F9 attribute names the new vertex,
    /// so the vertex must exist first), edge, `he_plus`, `he_minus`.
    /// Emanating rule and splice positions: as [`Body::mev`](crate::construction::Construct::mev).
    ///
    /// # Precondition check order
    ///
    /// [`Body::mev`](crate::construction::Construct::mev)'s site list, and nothing after it.
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    fn mev_null(
        &mut self,
        site: MevSite,
        new_side: NewVertexSide,
    ) -> Result<MevCreated, EulerOpError>
    where
        T: geom_core::Decide;
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
    /// **Geometry** (module docs, M2 policy): `curve` is certified
    /// against the endpoints `start(he1)`'s point → `start(he2)`'s
    /// point (the `he_plus` forward order) before any mutation;
    /// `surface` supplies the new face's surface per [`FaceSurface`]
    /// (`Inherit` keeps the M1 face-split semantics — two regions of
    /// one surface). Chord-line sugar: [`Body::mef_chord`](crate::construction::Construct::mef_chord).
    ///
    /// **Sense** ([`crate::Face::sense`]): `mef` passes
    /// [`ParentSide::With`](crate::euler::ParentSide::With), derived on the old face's chart and stated
    /// on any other ([`Body::resolve_face_surface`]).
    ///
    /// **Minting order** (D9, exact): surface (only for
    /// [`FaceSurface::New`]), curve (the certified [`EdgeCurve`](geom_brep::EdgeCurve)),
    /// edge, loop, face, `he_plus`, `he_minus`.
    ///
    /// The new face joins the old face's shell
    /// (membership plus back-pointer). `mef` does **not** reclassify
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
    /// **Pcurve rows** ([`crate::pcurves`]): the moved run's stored
    /// rows are curves stated in the OLD face's chart. Where the new
    /// face is on the same chart — [`FaceSurface::Inherit`], a
    /// [`FaceSurface::Shared`] naming the old key or one sharing its
    /// payload ([`Body::same_chart`]) — they stand; under any other
    /// surface the run's rows are DROPPED ([`Body::drop_rows`]). The
    /// old face's remaining rows are untouched either way. The two
    /// halves this op mints get their rows at the site, as
    /// [`Body::mev`](crate::construction::Construct::mev)'s do: when the site mint selects the old face, it
    /// is re-minted with `he_plus` in it, and the new face is minted
    /// with `he_minus` and the run walked in the new face's chart —
    /// whichever chart that is, so a run whose rows were dropped is
    /// derived there again — on the terms [`Body::mev`](crate::construction::Construct::mev) states. On a
    /// spline chart other than the old face's, the new face keeps the
    /// drop and is left unminted. A face that takes the old face's
    /// last null edge off it is re-minted whatever the old face missed:
    /// its loops the cut rewires leave complete, and a ring it keeps
    /// keeps what it had.
    ///
    /// **A chord bounding a face minted on another key describes the
    /// edge it is, or is refused.** Where `surface` is not the old
    /// face's key, the chord's two faces wear the old face's key and the
    /// new face's, and `curve`'s description is asked the adjacency
    /// question [`Body::set_edge_curve`](crate::construction::Construct::set_edge_curve) asks
    /// ([`EulerOpError::DescriptionNotAdjacent`], naming no edge: the
    /// chord has no key yet).
    ///
    /// **What it does not ask:** where the new face keeps the old
    /// face's key, whether the chord's description names it. The
    /// boolean pipeline's chord joins rely on that: they mint the chord
    /// with the description of the face glued along it afterwards
    /// (`work/restate/minting-doors-take-a-callers-description-unasked-where-the-new-face-keeps-the-key`).
    ///
    /// **A face minted on another chart is vouched for by the edges it
    /// takes, or refused** ([`RechartDoor::Mef`](crate::RechartDoor::Mef)). Where `surface` is
    /// not the old face's key, the run's edges and the chord change the
    /// key one of their faces wears, and this keys-only door asks
    /// [`Body::vouch_move`]'s questions of them. A `New` key is one no
    /// description names, so onto a `New` chart off the old face's
    /// payload only scaffold and null edges pass. The lever is the chart the face is minted on: mint it
    /// on a key its certified edges name, or on the old face's and move
    /// it with [`Body::set_face_surfaces_describing`](crate::construction::Construct::set_face_surfaces_describing).
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
    /// `Chords`: `he1` resolves, `he2` resolves ([`BadArgument::Stale`](crate::BadArgument::Stale));
    /// same parent loop ([`EulerOpError::NotSameLoop`]). `Lone`: the loop
    /// resolves ([`BadArgument::Stale`](crate::BadArgument::Stale)); it is empty
    /// ([`EulerOpError::LoopNotEmpty`]). Then, for both sites, the
    /// geometry gates: a [`FaceSurface::Shared`] key resolves
    /// ([`BadArgument::StaleGeometry`](crate::BadArgument::StaleGeometry)), a stated sense agrees with the
    /// derived one on the old face's chart
    /// ([`EulerOpError::SenseContradictsChart`]), where the new face's
    /// key is not the old one's `curve`'s description is
    /// adjacency-coherent with the two
    /// ([`EulerOpError::DescriptionNotAdjacent`], `edge: None`), and
    /// `curve` certifies ([`EulerOpError::Certification`]) — the order
    /// [`Body::set_edge_curve`](crate::construction::Construct::set_edge_curve) asks them in. Then, where the new face's key
    /// is not the old one's, no edge of the run is stranded
    /// ([`EulerOpError::RechartStrandsDescriptions`], every one named,
    /// in run order), then every certified edge of the run and a
    /// certified chord name it ([`EulerOpError::RechartUnvouched`], the
    /// same). Last, the pcurve rows, as [`Body::mev`](crate::construction::Construct::mev) states them
    /// ([`EulerOpError::PcurveMint`]).
    ///
    /// # Panics
    ///
    /// Where a record the plan reads past the arguments does not resolve
    /// or a walk of it does not close: the loop (which must be a cycle,
    /// and whose walk from `he1` must reach `he2` through members that
    /// claim it), the `prev` links, the face and its shell, the start
    /// vertices and their points, the surfaces and loops the rows read.
    /// Every public door keeps the body tier-1-valid, where all of these
    /// hold.
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    fn mef(
        &mut self,
        site: MefSite,
        curve: EdgeCurveSpec<T>,
        surface: FaceSurface<T>,
        tol: Tol,
    ) -> Result<MefCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    /// [`Body::mef`](crate::construction::Construct::mef) with derived scaffolding geometry and
    /// [`FaceSurface::Inherit`] — the polyhedral/migration sugar
    /// (module docs, geometry policy):
    ///
    /// - distinct end vertices ⇒ the chord line between their points
    ///   ([`EdgeCurveSpec::line_between`]);
    /// - a self-loop site (`Lone`, `Chords` with `he1 == he2`, or both
    ///   halves starting at one vertex) ⇒ the canonical scaffolding
    ///   circle at the shared point
    ///   ([`EdgeCurveSpec::self_loop_circle_at`]).
    ///
    /// The dispatch is **structural** (key equality, never a scalar
    /// comparison); two *distinct* vertices at coincident coordinates
    /// still take the chord path, and their chord of no length is
    /// refused typed ([`geom_brep::CertifyError::StatedInterval`]).
    ///
    /// # Errors
    ///
    /// As [`Body::mef`](crate::construction::Construct::mef).
    fn mef_chord(&mut self, site: MefSite, tol: Tol) -> Result<MefCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    /// MEKR — *make edge, kill ring*: join a ring of a face back into
    /// another loop of the same face with a new edge; the ring's loop
    /// key dies. The exact inverse of [`Body::kemr`](crate::construction::Construct::kemr).
    ///
    /// Site semantics (one variant per boundary-shape combination):
    /// [`MekrSite`]. Direction convention (module docs): `he_plus` runs
    /// **target anchor vertex → ring anchor vertex**; both halves land
    /// in the target loop, which is re-anchored at `he_plus`
    /// (unconditional). Emanating: `u → he_plus`, `w → he_minus`
    /// (unconditional, `mev`'s rule).
    ///
    /// Euler vector: `(v 0, e +1, f 0, h 0, r −1, s 0)` — arena deltas
    /// −1 loop, +2 half-edges, +1 edge.
    ///
    /// **Pcurve rows**: as [`Body::mev`](crate::construction::Construct::mev)'s — the face, when its rows
    /// were complete, has its merged loop re-minted with both halves in
    /// it before any mutation, on the terms `mev` states.
    ///
    /// **Minting order** (D9, exact): curve (placeholder, anchored at the
    /// target anchor vertex's coordinates), edge, `he_plus`, `he_minus`.
    /// **Kill order**: the ring loop (with its provenance entry and every
    /// null-face record naming it, as a null-face record lives only while
    /// its face holds both loops it names — [`crate::null`]), after the
    /// splice.
    ///
    /// # Precondition check order
    ///
    /// The listing below is **`Cycles`-canonical**; the loop-key-
    /// addressed sites (`EmptyTarget`, `BothEmpty`) necessarily resolve
    /// the target loop (and check its boundary shape) *before* the
    /// distinctness check. Each site's own order is fixed and
    /// deterministic (the first failure wins).
    ///
    /// Anchors resolve ([`BadArgument::Stale`](crate::BadArgument::Stale)); the two loops are
    /// distinct ([`EulerOpError::SameLoop`]); an argument loop has the
    /// site's required boundary shape ([`EulerOpError::LoopNotEmpty`]);
    /// both loops belong to one face ([`EulerOpError::NotSameFace`]);
    /// the ring is not that face's outer loop
    /// ([`EulerOpError::RingIsOuter`]). Then the geometry gate: `curve`
    /// certifies against the anchors' points, u → w in the `he_plus`
    /// forward order ([`EulerOpError::Certification`]; `crate::euler`
    /// module docs, M2 geometry policy). Last, the pcurve rows, as
    /// [`Body::mev`](crate::construction::Construct::mev) states them: the face's row plan is minted
    /// ([`EulerOpError::PcurveMint`]). Chord sugar: [`Body::mekr_chord`](crate::construction::Construct::mekr_chord).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    ///
    /// # Panics
    ///
    /// Where the body is not tier-1-valid around the two loops: a
    /// half-edge claims an empty or dead loop, a record they name does
    /// not resolve, a cycle walk does not close or is not exactly the
    /// half-edges that claim its loop, a face but the ring's lists the
    /// ring, or two empty loops share one lone vertex. Every public door
    /// keeps the body tier-1-valid, so only a kernel bug reaches these.
    fn mekr(
        &mut self,
        site: MekrSite,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<MekrResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    /// [`Body::mekr`](crate::construction::Construct::mekr) with derived scaffolding geometry — the
    /// polyhedral/migration sugar (see `crate::euler`'s geometry
    /// policy): the chord line between the two anchor vertices' points
    /// (`EdgeCurveSpec::line_between`), or the canonical scaffolding
    /// circle when both anchors are one vertex (structural key
    /// equality — `EdgeCurveSpec::self_loop_circle_at`).
    ///
    /// # Errors
    ///
    /// As [`Body::mekr`](crate::construction::Construct::mekr).
    fn mekr_chord(&mut self, site: MekrSite, tol: Tol) -> Result<MekrResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    /// MFKRH — *make face, kill ring–hole*: the inverse of
    /// [`Body::kfmrh`](crate::construction::Construct::kfmrh). Promotes a ring (cycle or empty) to the outer
    /// loop of a NEW face in the same shell.
    ///
    /// **This op is also the book's cross-shell `lmfkrh`** (M3 PR 1 —
    /// the inverse *motion* of cross-shell [`Body::kfmrh`](crate::construction::Construct::kfmrh), serving
    /// ch. 14 `splitfinish` / ch. 15 `setopfinish` section-face
    /// promotion, M3 PRs 3 and 5): promoting a ring whose cycle is
    /// detached from its face's component **splits the shell's surface
    /// into two connected components** while the single shell entity
    /// remains — the M2-deferred multi-shell transient, tier-1 legal
    /// (component-aware E–P) and tier-2 refused until
    /// [`Body::movefac`](crate::construction::Construct::movefac) distributes the components into real shells.
    /// The shell-level split is deliberately movefac's job, not this
    /// op's: `mfkrh` cannot re-home the component without walking it,
    /// and pass 10 (edge-adjacency shell coherence) requires whole
    /// components to move together. For the split/boolean pipeline's
    /// section faces — where the promoted ring geometrically coincides
    /// with the remaining loop — pass [`FaceSurface::Inherit`] (same
    /// surface key as the demoting face; the pipeline re-plates the
    /// section plane immediately after) or `Shared` with the section
    /// plane's key.
    ///
    /// The promoted loop survives with its key and D5 birth record; the
    /// new face's surface comes from the [`FaceSurface`] spec (M2
    /// geometry policy, `crate::euler` module docs): `Inherit` shares
    /// the demoting face's surface, `New` mints (pass
    /// `Surface::Nurbs` for the honest "no description yet" state —
    /// exact restoration of `kfmrh`'s killed surface is impossible and
    /// NOT promised, [module docs](self)), `Shared` reuses an existing
    /// key.
    ///
    /// **Sense** ([`crate::Face::sense`]): `mfkrh` passes
    /// [`ParentSide::Against`](crate::euler::ParentSide::Against), derived on the demoting face's chart
    /// and stated on any other ([`Body::resolve_face_surface`]).
    /// Promoting an [`LoopBoundary::Empty`](crate::LoopBoundary::Empty) ring yields an
    /// **empty-outer face** — the `mvfs`-face shape, now
    /// operator-reachable inside a larger body.
    ///
    /// **Pcurve rows** ([`crate::pcurves`]): the promoted ring's stored
    /// rows are a curve stated in the DEMOTING face's chart. A spec on
    /// that chart ([`Body::same_chart`]) keeps them; any other surface
    /// DROPS them ([`Body::drop_rows`] states why), over the members the
    /// plan proves are the ring's ([`Body::whole_cycle`]). Where they do
    /// not stand on the new face — dropped, or
    /// missing — and the demoting face's rows were complete on an
    /// analytic chart, this door refuses
    /// [`crate::pcurves::SiteRowRefusal::KeysOnly`] before anything
    /// moves; [`Body::mfkrh_minting`](crate::construction::Construct::mfkrh_minting) takes a band and mints the new
    /// face with the ring walked in its chart. It is two doors for the
    /// kill family's reason: `mfkrh` decides nothing against ε, and the
    /// band is read only where that re-mint runs. A spline chart keeps
    /// the drop at either door.
    ///
    /// **A face minted on another chart is vouched for by the ring's
    /// edges, or refused** ([`RechartDoor::Mfkrh`](crate::RechartDoor::Mfkrh)), at every door of
    /// the family: every edge of the ring is asked
    /// [`Body::vouch_move`]'s questions against the new face's key. The
    /// lever is the chart the face is minted on: mint it on a key the
    /// ring's certified edges name, or on the demoting face's and move
    /// it with [`Body::set_face_surfaces_describing`](crate::construction::Construct::set_face_surfaces_describing).
    ///
    /// Euler vector: `(v 0, e 0, f +1, h −1, r −1, s 0)` — arena delta
    /// +1 face (the "−1 ring" is the surviving loop's promotion, not a
    /// kill; genus is derived, not stored).
    ///
    /// **Minting order** (D9, exact): surface (only for
    /// [`FaceSurface::New`]), face. Nothing is killed. The new face is
    /// appended to the shell's face list; the ring leaves its former face's
    /// ring list (`retain`, order-preserving for the others), and every
    /// null-face record naming it is dropped, as a null-face record lives
    /// only while its face holds both loops it names ([`crate::null`]).
    ///
    /// # Precondition check order
    ///
    /// The ring resolves ([`BadArgument::Stale`](crate::BadArgument::Stale));
    /// it is not its face's outer loop ([`EulerOpError::RingIsOuter`]);
    /// a [`FaceSurface::Shared`] key resolves
    /// ([`BadArgument::StaleGeometry`](crate::BadArgument::StaleGeometry));
    /// a stated sense agrees with the derived one on the demoting face's
    /// chart ([`EulerOpError::SenseContradictsChart`]); then, where the
    /// new face's key is not the demoting face's, no edge of the ring is
    /// stranded ([`EulerOpError::RechartStrandsDescriptions`], every one
    /// named, in cycle order) and every certified one names that key
    /// ([`EulerOpError::RechartUnvouched`], the same); then the site
    /// mint's plan ([`Body::plan_moved_rows`]'s errors): where the new
    /// face would be minted, [`EulerOpError::PcurveMint`] naming the
    /// demoting face (`KeysOnly` at this door).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    ///
    /// # Panics
    ///
    /// Where a record the ring reaches does not resolve, or its walk does
    /// not close on the half-edges that claim it ([`Body::whole_cycle`]):
    /// every public door keeps the body tier-1-valid, where neither
    /// occurs.
    fn mfkrh(
        &mut self,
        ring: LoopKey,
        surface: FaceSurface<T>,
    ) -> Result<MfkrhCreated, EulerOpError>
    where
        T: Decide;
    /// [`Body::mfkrh`](crate::construction::Construct::mfkrh) with a band: where `mfkrh` refuses
    /// [`crate::pcurves::SiteRowRefusal::KeysOnly`] — the promoted
    /// ring's rows do not stand on the new face, and the demoting
    /// face's rows were complete — this door mints the new face at
    /// `tol`'s band, the ring walked in its chart
    /// ([`Body::plan_moved_rows`]); everywhere else it is `mfkrh`.
    ///
    /// # Errors
    ///
    /// As [`Body::mfkrh`](crate::construction::Construct::mfkrh), except the `KeysOnly` refusal, and the site
    /// mint's plan in its place ([`Body::plan_moved_rows`]'s errors).
    fn mfkrh_minting(
        &mut self,
        ring: LoopKey,
        surface: FaceSurface<T>,
        tol: Tol,
    ) -> Result<MfkrhCreated, EulerOpError>
    where
        T: Decide;
    /// [`Body::mfkrh`](crate::construction::Construct::mfkrh) with [`FaceSurface::New`] holding the
    /// `Surface::Nurbs` placeholder — the new face is born with the
    /// honest "no description yet" surface (`crate::euler`'s geometry
    /// policy; replace it via [`Body::set_face_surface`](crate::construction::Construct::set_face_surface) before rest).
    /// Mirrors the M1 fresh-surface semantics for the migrated suites
    /// and for promotions whose real surface is not yet known.
    ///
    /// The placeholder is a fresh payload, so it is never the demoting
    /// face's chart: `mfkrh` writes `sense`, the caller's provisional
    /// bit, as stated, and drops the promoted ring's rows — a spline
    /// chart, so none is owed there and this keys-only door never
    /// refuses for one. The caller states the honest bit, and mints the
    /// rows, when it gives the face a real surface.
    ///
    /// **The new face is chartless.** No description names the
    /// placeholder's fresh key and no certificate vouches on it, so of
    /// [`Body::mfkrh`](crate::construction::Construct::mfkrh)'s questions ([`Body::vouch_move`]) it asks only
    /// the strand, as [`RechartDoor::MfkrhPlug`](crate::RechartDoor::MfkrhPlug): a ring with an edge
    /// whose description would name no key its faces wear is refused.
    /// Its caller picks no chart, so the lever that refusal names is the
    /// door: promote the ring with [`Body::mfkrh`](crate::construction::Construct::mfkrh) onto a chart those
    /// edges name.
    ///
    /// # Errors
    ///
    /// As [`Body::mfkrh`](crate::construction::Construct::mfkrh), the re-chart refusals naming
    /// [`RechartDoor::MfkrhPlug`](crate::RechartDoor::MfkrhPlug).
    fn mfkrh_plug(&mut self, ring: LoopKey, sense: bool) -> Result<MfkrhCreated, EulerOpError>
    where
        T: Decide;
    /// KEV — *kill edge, vertex*: the inverse of [`Body::mev`](crate::construction::Construct::mev). Kills
    /// `he`'s edge and **`end(he)`** (the far vertex), merging that
    /// vertex's remaining edge fan back onto `start(he)`.
    ///
    /// Which vertex dies is the argument's choice: `kev(he)` kills the
    /// vertex `he` points at; to kill the other endpoint, pass the mate.
    /// Full derivation, surgery diagram, and the degenerate strut/
    /// segment cases (each the inverse of a [`crate::construction::MevSite`] case):
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
    /// `None` (segment kill — the loop is [`LoopBoundary::Empty`](crate::LoopBoundary::Empty) at the
    /// survivor again).
    ///
    /// # The merged fan
    ///
    /// The far vertex's surviving edges — the **merged members** — are
    /// re-based onto `start(he)`, each keeping the curve it was
    /// certified with, which pins one end of its carrier to the dying
    /// vertex's point. This door takes no band, so it certifies none of
    /// them: it refuses **every** merge with a certified member
    /// ([`EulerOpError::MergeRebasesCarriers`], naming each in orbit
    /// order), a merge of two vertices at one point included, since
    /// whether the survivor's point is the dying vertex's is the question
    /// the re-basing gate does not ask (the crate-internal
    /// `Body::certify_rebased_run`'s docs). It carries what structure
    /// answers: an empty merged fan, a killed null edge, and a null
    /// member with both halves in the fan; a null member with one half
    /// in it refuses [`EulerOpError::RebasedNullEdge`].
    ///
    /// A killed null edge moves nothing because its two vertices hold
    /// one point: [`Body::mev_null`](crate::construction::Construct::mev_null) mints them so, and both re-basing
    /// gates refuse to move one end of one. The doors that move
    /// vertices (`replace_face`, `offset_*`, through
    /// `Body::move_vertices`) take tier-2-valid bodies, which hold no
    /// null edge.
    ///
    /// A merge that moves nothing, or moves its members within band,
    /// goes through `kev_describing(he, &[], tol)`
    /// ([`Body::kev_describing`](crate::construction::Construct::kev_describing)), and a merge that re-describes its
    /// members hands their specs to that door;
    /// [`Body::kev_merged_members`](crate::construction::Construct::kev_merged_members) reads the members and the endpoints
    /// the merge gives each.
    ///
    /// # Precondition check order
    ///
    /// `he` resolves ([`BadArgument::Stale`](crate::BadArgument::Stale));
    /// the endpoints are distinct ([`EulerOpError::SelfLoopEdge`]). Then,
    /// where the merged fan is not empty and the killed edge is not a
    /// null edge, per merged member in orbit order, it is not a null
    /// edge the merge moves one end of
    /// ([`EulerOpError::RebasedNullEdge`]); last, no member is certified
    /// ([`EulerOpError::MergeRebasesCarriers`], naming all of them).
    /// Before the merged fan, where the unsplice crosses each killed half
    /// whole and the killed edge is not a null edge, no side stores the
    /// two elements its bridged joint would sum through the half's turn
    /// ([`EulerOpError::PcurveMint`] with
    /// [`crate::pcurves::SiteRowRefusal::KeysOnly`]: this door takes no
    /// band to decide the turn by, so it refuses rather than leave the
    /// face half-minted; [`Body::kev_describing`](crate::construction::Construct::kev_describing) decides it). Last,
    /// where the killed edge is a null edge, no loop the kill releases
    /// is left missing a row on a face the site mint selects
    /// ([`Body::plan_released_rows`]: [`EulerOpError::PcurveMint`] with
    /// `KeysOnly` naming the face; [`Body::kev_describing`](crate::construction::Construct::kev_describing) mints it).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    ///
    /// # Panics
    ///
    /// Where a record `he` reaches does not resolve, the mate its edge
    /// gives is not its other half, either half claims an empty loop,
    /// the far vertex's orbit does not close at it, an anchor the kill
    /// writes does not hold, or a record the kill keeps names one it
    /// removes: every public door keeps the body tier-1-valid, where none
    /// of these occurs.
    fn kev(&mut self, he: HalfEdgeKey) -> Result<KevResult, EulerOpError>
    where
        T: Decide;
    /// KEV with the merged fan's re-descriptions: [`Body::kev`](crate::construction::Construct::kev)'s kill,
    /// taking a band and, for any merged member whose stored carrier
    /// does not describe it at the merged endpoints, the spec that
    /// does.
    ///
    /// The shape of [`Body::mev`](crate::construction::Construct::mev) on the kill side: the site (`he`),
    /// the geometry the kill writes (`redescriptions`) and the band
    /// (`tol`). Topology and descriptions are written together, so no
    /// state between them is ever observable, and nothing is written
    /// until everything has certified.
    ///
    /// - **A listed member** is certified with its spec against the
    ///   endpoints the merge WILL give it — the surviving vertex's point
    ///   at every half in the merged fan, its current point elsewhere —
    ///   through the attachment gate [`Body::set_edge_curve`](crate::construction::Construct::set_edge_curve) certifies
    ///   through, adjacency coherence included, and its curve is
    ///   replaced by fresh insertion (the old one reaped iff orphaned).
    ///   The kill moves its end and this door its carrier, so the rows
    ///   its halves store would span the interval the end moved from:
    ///   each face its halves are on is re-minted, through the planner
    ///   `set_edge_curve` runs for a null edge's first description, over
    ///   the face as the kill leaves it — the killed halves gone, and
    ///   every listed member's halves under the curve this door
    ///   installs. A face on a spline chart is left as found, its rows
    ///   tier 3's to report.
    /// - **An unlisted member** keeps its carrier and passes the
    ///   re-basing gate [`Body::mev`](crate::construction::Construct::mev)'s fan site passes, under this
    ///   band: its stored description re-certified against the merged
    ///   endpoints, a pre-existing staleness carried and a null edge
    ///   the merge moves one end of refused. Where the killed edge is a
    ///   null edge nothing moves and the gate is not asked ([`Body::kev`](crate::construction::Construct::kev)
    ///   says why).
    /// - **A loop a killed null edge releases** has what it missed while
    ///   the edge held it open minted, over each face it is on as the
    ///   kill leaves it ([`Body::plan_released_rows`]), where
    ///   [`Body::kev`](crate::construction::Construct::kev) refuses `KeysOnly`.
    ///
    /// An empty list is the kill for a caller whose members all pass
    /// where they land — a merge of two vertices at one point, or a band
    /// apart — and a non-empty one is the kill for a caller that
    /// re-describes what it merges: `split_edge`'s inverse re-attaching
    /// the parent's spec, a blend handing a member the chord it spans.
    /// [`Body::kev_merged_members`](crate::construction::Construct::kev_merged_members) reads the members and the endpoints
    /// the merge gives each, which is what a caller states its specs
    /// against.
    ///
    /// Euler vector, kill order and re-anchoring: [`Body::kev`](crate::construction::Construct::kev)'s. Then,
    /// per listed member in list order: its certified curve is inserted
    /// and its old curve reaped iff orphaned, through the write
    /// [`Body::set_edge_curve`](crate::construction::Construct::set_edge_curve) makes. [`KevResult`] reports what the
    /// kill kills and not these: a re-described member survives, its
    /// new curve is its edge's `curve`, and the replaced curve is reaped
    /// exactly as `set_edge_curve` reaps one, which reports no reaped
    /// key either.
    ///
    /// # Precondition check order
    ///
    /// [`Body::kev`](crate::construction::Construct::kev)'s structural list (through the adjacent arms' one
    /// loop).
    /// Then per entry of `redescriptions`, in list order: the edge is a
    /// merged member ([`EulerOpError::NotMergedMember`]); it was not
    /// listed before ([`EulerOpError::DuplicateRedescription`]); its
    /// spec is adjacency-coherent
    /// ([`EulerOpError::DescriptionNotAdjacent`], or
    /// [`BadArgument::Stale`](crate::BadArgument::Stale) for a key the
    /// spec names that does not resolve); the spec certifies against
    /// the merged endpoints ([`EulerOpError::RebasedCarrier`] naming the
    /// edge). Then,
    /// where the merged fan is not empty, the killed edge is not a null
    /// edge and a member is left unlisted, the unlisted members pass the
    /// re-basing gate in orbit order ([`EulerOpError::RebasedNullEdge`]
    /// / [`EulerOpError::RebasedCarrier`] naming the first that fails).
    /// So an empty merged fan with an empty list asks nothing past the
    /// structural list but the released loops' rows (last), where the
    /// two doors part: one refuses, this one mints. Then, where the
    /// unsplice is general and the killed edge is not a null edge (each
    /// killed half is crossed whole), `tol` builds a band
    /// ([`EulerOpError::Certification`] with
    /// [`geom_brep::CertifyError::Band`]) and each killed half's turn is
    /// decided at it ([`EulerOpError::KillTurnUndecided`] naming the
    /// first half, `he` before its mate, whose turn is not decided). Then,
    /// where a member is listed, the site mint over the faces its halves
    /// are on is planned ([`EulerOpError::PcurveMint`], as
    /// [`Body::set_edge_curve`](crate::construction::Construct::set_edge_curve)'s). Last, where the killed edge is a
    /// null edge, the site mint over every other face its halves are on
    /// that a loop it releases leaves with a gap
    /// ([`Body::plan_released_rows`]: [`EulerOpError::Certification`]
    /// where `tol` builds no band).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    ///
    /// # Panics
    ///
    /// As [`Body::kev`](crate::construction::Construct::kev), and where a record a merged member reaches
    /// does not resolve: every public door keeps the body tier-1-valid.
    ///
    /// Also on a kernel bug: an edge the body stores, re-minted or
    /// restated on its own carrier and interval, whose span or winding
    /// reads certainly refused at the run's band.
    fn kev_describing(
        &mut self,
        he: HalfEdgeKey,
        redescriptions: &[(EdgeKey, EdgeCurveSpec<T>)],
        tol: Tol,
    ) -> Result<KevResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    /// The merged members of `kev(he)` — every edge the kill re-bases
    /// onto `start(he)`, once each, in the dying vertex's clockwise
    /// orbit order from the killed edge — each with the endpoints the
    /// merge gives it. Read from the plan both kill doors run and the
    /// endpoint reading their gates certify against, so a spec stated
    /// between these endpoints is one [`Body::kev_describing`](crate::construction::Construct::kev_describing)
    /// certifies at them. Pure.
    ///
    /// Empty where the merged fan is: a strut or segment kill merges
    /// nothing.
    ///
    /// # Errors
    ///
    /// [`Body::kev`](crate::construction::Construct::kev)'s structural list. Nothing the merge's geometry
    /// decides is asked.
    ///
    /// # Panics
    ///
    /// As [`Body::kev`](crate::construction::Construct::kev), and where the surviving vertex's point or a
    /// member's other endpoint does not resolve: every public door keeps
    /// the body tier-1-valid, where a vertex's point resolves.
    fn kev_merged_members(&self, he: HalfEdgeKey) -> Result<Vec<MergedMember<T>>, EulerOpError>
    where
        T: Decide;
    /// KEF — *kill edge, face*: the inverse of [`Body::mef`](crate::construction::Construct::mef). Kills
    /// `he`'s edge and **the face of `he`'s loop** (`he`'s side dies);
    /// the dying loop's remnant merges into the mate's loop.
    ///
    /// Which face dies is the argument's choice: a `mef` is undone by
    /// `kef(created.he_minus)`; passing the mate kills the other side.
    /// Full derivation, surgery diagram, and the degenerate circular/
    /// lone cases (each the inverse of a [`crate::construction::MefSite`] case):
    /// [module docs](self). Self-loop edges (`start == end`) are legal
    /// here — they are the `Lone`-inverse territory.
    ///
    /// Euler vector: `(v 0, e −1, f −1, h 0, r 0, s 0)` — arena deltas
    /// −2 half-edges, −1 edge, −1 face, −1 loop.
    ///
    /// **Minting order**: nothing is minted. **Kill order** (D9, exact):
    /// `he`, its mate, the edge, the dying loop, the dying face (each with
    /// its provenance entry; the face with its null-face record, the loop
    /// with every null-face record naming it, as a null-face record lives
    /// only while its face holds both loops it names — [`crate::null`]),
    /// then the edge's curve iff orphaned and the face's surface iff
    /// orphaned (usually shared, hence kept — module docs).
    ///
    /// **Re-anchoring** (unconditional, module docs): the surviving loop
    /// re-anchors at `next(mate)` when that survives, else `next(he)`;
    /// both endpoint vertices' `emanating` are rewritten to the derived
    /// survivors (or `None` in the `Lone` inverse, where the surviving
    /// loop becomes [`LoopBoundary::Empty`](crate::LoopBoundary::Empty)).
    ///
    /// **Pcurve rows** ([`crate::pcurves`]): the remnant's stored rows
    /// are curves stated in the DYING face's chart. Where the surviving
    /// face is on the same chart — one key, or two sharing one
    /// payload ([`Body::same_chart`]) — they stand; on any
    /// other chart the remnant's rows are DROPPED ([`Body::drop_rows`]).
    /// Where they do not stand — dropped, or missing — and the
    /// surviving face's rows were complete on an analytic chart, the
    /// surviving face is owed a re-mint this keys-only kill takes no
    /// band for, so it refuses
    /// [`crate::pcurves::SiteRowRefusal::KeysOnly`] before anything
    /// moves rather than leave that face half-minted — the shape of
    /// [`Body::kev`](crate::construction::Construct::kev)'s refusal where a carrier would go stale.
    /// [`Body::kef_minting`](crate::construction::Construct::kef_minting) is the same kill with a band: it walks the
    /// surviving loop as the splice leaves it (its own members from its
    /// new anchor, then the remnant) in the surviving face's chart. On
    /// a spline chart, or a surviving face that was unminted or
    /// half-minted, the drop is the whole answer at either door and the
    /// surviving loop's own rows are untouched. Where the remnant's rows
    /// stand and the killed edge is a null edge, the kill can still
    /// release the surviving loop missing rows it missed while held
    /// open ([`Body::plan_released_rows`]): this door refuses `KeysOnly`
    /// there too, and [`Body::kef_minting`](crate::construction::Construct::kef_minting) mints them.
    /// Which is why the surviving face RESOLVES in the plan phase
    /// below, and the chart is decided there: it is what the mutation
    /// phase acts on, and a mutation phase reads nothing it has not
    /// proven.
    ///
    /// **Keys-only.** Where the surviving face wears another key, the
    /// remnant moves onto a chart its edges' descriptions may not name.
    /// This door asks [`Body::mef`](crate::construction::Construct::mef)'s and [`Body::ring_move`](crate::construction::Construct::ring_move)'s
    /// questions of that move ([`Body::vouch_move`], as
    /// [`RechartDoor::Kef`](crate::RechartDoor::Kef)): it refuses an edge the move would leave
    /// described against a key neither of its faces wears, then a
    /// certified edge landing on a chart its description does not name,
    /// unless the two keys share one payload or the surviving face
    /// wears the "no chart yet" placeholder. [`Body::kef_describing`](crate::construction::Construct::kef_describing)
    /// takes the move with its re-descriptions under a band.
    ///
    /// # Precondition check order
    ///
    /// `he` resolves ([`BadArgument::Stale`](crate::BadArgument::Stale));
    /// the two halves lie in distinct loops ([`EulerOpError::SameLoop`] —
    /// the same-loop configuration is [`Body::kemr`](crate::construction::Construct::kemr)'s); the two loops
    /// lie on distinct faces ([`EulerOpError::SameFace`] — two loops of
    /// one face is what [`Body::kfmrh`](crate::construction::Construct::kfmrh) on adjacent faces leaves behind;
    /// kill such an edge with [`Body::kev`](crate::construction::Construct::kev), or via
    /// [`Body::mfkrh`](crate::construction::Construct::mfkrh)-then-`kef` for the self-loop variant); the dying
    /// face is ring-free ([`EulerOpError::FaceHasRings`]); then the
    /// move's keys ([`EulerOpError::RechartStrandsDescriptions`], then
    /// [`EulerOpError::RechartUnvouched`], each naming every edge, in
    /// remnant order); then the site
    /// mint's plan ([`Body::plan_moved_rows`]'s errors, or, where the
    /// remnant's rows stand, [`Body::plan_released_rows`]'): where the
    /// surviving face would be re-minted, [`EulerOpError::PcurveMint`]
    /// naming it (`KeysOnly` at this door).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    ///
    /// # Panics
    ///
    /// Where a record `he` reaches does not resolve, the mate its edge
    /// gives is not its other half, either half claims an empty loop,
    /// the dying loop's cycle does not close on the half-edges that
    /// claim it, an anchor the kill writes does not hold, or a record the
    /// kill keeps names one it removes: every public door keeps the body
    /// tier-1-valid, where none of these occurs.
    fn kef(&mut self, he: HalfEdgeKey) -> Result<KefResult, EulerOpError>
    where
        T: Decide;
    /// **The describing kill**: [`Body::kef_minting`](crate::construction::Construct::kef_minting)'s kill with the
    /// re-descriptions of the edges it moves, under `tol`'s band — the
    /// twin [`Body::kef`](crate::construction::Construct::kef)'s refusals name, as [`Body::kev_describing`](crate::construction::Construct::kev_describing)
    /// is [`Body::kev`](crate::construction::Construct::kev)'s. Each listed description is stated on the
    /// adjacency the kill leaves, a key the dying face wears standing
    /// for the surviving face's ([`Body::kef_carried_redescriptions`](crate::construction::Construct::kef_carried_redescriptions)
    /// states the stored ones there), and certified on the edge's own
    /// carrier and interval: a kill moves no carrier, so the stored
    /// carrier, interval and pcurve rows stand. It re-describes nothing
    /// it is not handed.
    ///
    /// # Precondition check order
    ///
    /// [`Body::kef`](crate::construction::Construct::kef)'s structural list; then the listed re-descriptions,
    /// the strands and the residuals, in [`Body::vouch_described_move`]'s
    /// order: per entry, the edge resolves
    /// ([`BadArgument::Stale`](crate::BadArgument::Stale)), was not
    /// listed before ([`EulerOpError::DuplicateRedescription`]), has a
    /// half in the remnant ([`EulerOpError::NotMovedEdge`]), is not a
    /// null edge ([`EulerOpError::NullScaffoldCurve`]), is
    /// adjacency-coherent after the kill
    /// ([`EulerOpError::DescriptionNotAdjacent`]) and certifies
    /// ([`EulerOpError::RechartFalsifies`]; `tol` builds the band,
    /// [`EulerOpError::Certification`]); then no unlisted edge strands
    /// ([`EulerOpError::RechartUndescribed`]); then, onto another chart
    /// that is not chartless, every certified moved edge no description
    /// names there is vouched for by residual on a plane
    /// ([`EulerOpError::RechartOffBoundary`],
    /// [`EulerOpError::RechartBoundaryEscalated`]) and refused on a
    /// curved chart ([`EulerOpError::RechartUnvouched`]). Then
    /// [`Body::kef_minting`](crate::construction::Construct::kef_minting)'s site mint.
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    ///
    /// # Panics
    ///
    /// As [`Body::kef`](crate::construction::Construct::kef).
    ///
    /// Also on a kernel bug: an edge the body stores, re-minted or
    /// restated on its own carrier and interval, whose span or winding
    /// reads certainly refused at the run's band.
    fn kef_describing(
        &mut self,
        he: HalfEdgeKey,
        redescriptions: &[(EdgeKey, EdgeDescriptionSpec<T>)],
        tol: Tol,
    ) -> Result<KefResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    /// [`Body::kef`](crate::construction::Construct::kef) with a band: where `kef` refuses
    /// [`crate::pcurves::SiteRowRefusal::KeysOnly`] — the remnant's
    /// rows do not stand on a complete surviving face, or a killed null
    /// edge releases the surviving loop with a gap — this door re-mints
    /// the surviving face at `tol`'s band, the surviving loop walked in
    /// its chart ([`Body::plan_moved_rows`],
    /// [`Body::plan_released_rows`]); everywhere else it is `kef`.
    ///
    /// # Errors
    ///
    /// As [`Body::kef`](crate::construction::Construct::kef), except the `KeysOnly` refusal, and the site
    /// mint's plan in its place; and it does not ask the move's keys.
    /// Its production callers (the boolean's seam zip, the coplanar
    /// merge, the chord join's sliver cut, the blend) rely on that
    /// move, and [`Body::kef_describing`](crate::construction::Construct::kef_describing) absorbs it once each states
    /// its re-descriptions
    /// (`work/restate/kef-and-kfmrh-across-keys-want-a-describing-door-or-reordered-callers`).
    fn kef_minting(&mut self, he: HalfEdgeKey, tol: Tol) -> Result<KefResult, EulerOpError>
    where
        T: Decide;
    /// **The re-descriptions [`Body::kef_describing`](crate::construction::Construct::kef_describing) would need to
    /// carry `kef(he)`'s move**: every edge the move strands whose stored
    /// description is coherent once the dying face's key stands for the
    /// surviving face's, with that description restated there, the
    /// stored kind travelling verbatim; a chart image whose chart's
    /// payload changes is left for the door to re-derive. In remnant
    /// order. Nothing is certified here. An edge the move leaves
    /// incoherent however its keys are read is not listed, so the caller
    /// states it or the door refuses it. Pure.
    ///
    /// # Errors
    ///
    /// [`Body::kef`](crate::construction::Construct::kef)'s structural list through
    /// [`EulerOpError::FaceHasRings`].
    fn kef_carried_redescriptions(
        &self,
        he: HalfEdgeKey,
    ) -> Result<Vec<(EdgeKey, EdgeDescriptionSpec<T>)>, EulerOpError>
    where
        T: Decide;
    /// KVFS — *kill vertex, face, solid*: the inverse of [`Body::mvfs`](crate::construction::Construct::mvfs).
    /// Destroys a solid in EXACTLY the skeletal state `mvfs` creates:
    /// one shell, one face with no rings, an
    /// [`LoopBoundary::Empty`](crate::LoopBoundary::Empty) outer loop holding one lone vertex.
    ///
    /// Euler vector: `(v −1, e 0, f −1, h 0, r 0, s −1)` — arena deltas
    /// −1 solid, −1 shell, −1 face, −1 loop, −1 vertex.
    ///
    /// **Minting order**: nothing is minted. **Kill order** (D9, exact, the
    /// reverse of `mvfs`'s spine minting): face, loop, shell, solid, vertex
    /// (each with its provenance entry; the face with its null-face record,
    /// the loop with every null-face record naming it, as a null-face
    /// record lives only while its face holds both loops it names —
    /// [`crate::null`]), then the face's surface iff orphaned and the
    /// vertex's point iff orphaned (geometry hygiene, module docs).
    ///
    /// # Precondition check order
    ///
    /// The solid resolves ([`BadArgument::Stale`](crate::BadArgument::Stale)); it has exactly one
    /// shell ([`EulerOpError::SolidNotSingleShell`]); that shell has
    /// exactly one face ([`EulerOpError::ShellNotSingleFace`]); the face
    /// has no rings ([`EulerOpError::FaceHasRings`]); its outer loop is
    /// empty ([`EulerOpError::LoopNotEmpty`]).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    ///
    /// # Panics
    ///
    /// Where a record the solid reaches does not resolve, or a record the
    /// kill keeps names one it removes (a half-edge claiming the loop or
    /// starting at the vertex, another loop `Empty` at the vertex, another
    /// record naming or listing the loop, face, shell or solid): every
    /// public door keeps the body tier-1-valid, where ownership is
    /// exclusive and the skeletal solid's records are named only by each
    /// other.
    fn kvfs(&mut self, solid: SolidKey) -> Result<KvfsResult, EulerOpError>
    where
        T: Decide;
    /// KEMR — *kill edge, make ring*: remove an edge occurring twice in
    /// one loop, splitting the loop's cycle into two components; `he1`'s
    /// side becomes a new **ring** of the same face.
    ///
    /// The ratified side association, the empty-anchor rules, and the
    /// deterministic re-anchoring rules: [module docs](self). `he1` and
    /// `he2` must be the two halves of one edge and lie in the same
    /// loop.
    ///
    /// Euler vector: `(v 0, e −1, f 0, h 0, r +1, s 0)` — arena deltas
    /// +1 loop, −2 half-edges, −1 edge.
    ///
    /// **Minting order** (D9, exact): the ring loop — the only entity
    /// minted. **Kill order** (D9, exact): `he1`, `he2`, the edge (each
    /// with its provenance entry), then the edge's curve iff orphaned
    /// ([`Body::remove_curve_if_orphaned`]).
    ///
    /// # Precondition check order
    ///
    /// `he1` resolves, then `he2` (`BadArgument::Stale`); they are the
    /// two halves of one edge ([`BadArgument::NotMates`](crate::BadArgument::NotMates)); same parent
    /// loop ([`EulerOpError::NotSameLoop`]). Last, where the edge is a
    /// null edge, the face is not left with a loop the kill releases
    /// missing a row on a face the site mint selects
    /// ([`Body::plan_released_rows`]: [`EulerOpError::PcurveMint`] with
    /// [`crate::pcurves::SiteRowRefusal::KeysOnly`] naming the face —
    /// this door takes no band to mint it; [`Body::kemr_minting`](crate::construction::Construct::kemr_minting) does).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    ///
    /// # Panics
    ///
    /// Where the body is not tier-1-valid around the edge: a record the
    /// edge, its halves or their loop name does not resolve, the loop's
    /// cycle from `he1` does not close through `he2`, a re-anchoring
    /// write does not hold, both sides are empty at one vertex, or a
    /// record the kill keeps still names a killed one. Every public door
    /// keeps the body tier-1-valid, so only a kernel bug reaches these.
    fn kemr(&mut self, he1: HalfEdgeKey, he2: HalfEdgeKey) -> Result<KemrResult, EulerOpError>
    where
        T: Decide;
    /// [`Body::kemr`](crate::construction::Construct::kemr) with a band: where `kemr` refuses
    /// [`crate::pcurves::SiteRowRefusal::KeysOnly`] — the killed edge is
    /// a null edge, and the loop's two sides, as the kill leaves them,
    /// run through no null edge and miss rows the loop missed while it
    /// was held open — the face is re-minted at `tol`'s band over those
    /// sides ([`Body::plan_released_rows`]); everywhere else it is
    /// `kemr`.
    ///
    /// # Errors
    ///
    /// As [`Body::kemr`](crate::construction::Construct::kemr), except the `KeysOnly` refusal, and the site
    /// mint's plan in its place ([`Body::plan_released_rows`]').
    fn kemr_minting(
        &mut self,
        he1: HalfEdgeKey,
        he2: HalfEdgeKey,
        tol: Tol,
    ) -> Result<KemrResult, EulerOpError>
    where
        T: Decide;
    /// KFMRH — *kill face, make ring–hole*: the connected sum. `f2`'s
    /// outer loop becomes a ring of `f1`; `f2` dies.
    ///
    /// Two forms, selected by where the faces live (M3 PR 1 lifted the
    /// M1 cross-shell deferral — the ch. 12 glue / ch. 15 seam-zip call
    /// site now exists):
    ///
    /// - **Same shell** (the M1 form): genus surgery — a handle (or a
    ///   component merge within the shell, the PR 4 transient).
    /// - **Cross-shell, same solid** (M3): shell **fusion** — `f2`'s
    ///   shell's surviving faces are re-homed into `f1`'s shell and
    ///   `f2`'s shell dies. Per-component E–P reading: the two
    ///   incidence components join through the demoted ring (connected
    ///   sum — genera add, χ₁+χ₂−2). Serves ch. 15 `setopfinish`'s
    ///   seam zip and ch. 12 glue (M3 PR 5). Fusing across **solids**
    ///   stays a typed error ([`EulerOpError::CrossSolid`]): combining
    ///   bodies is the boolean pipeline's combine step, not an Euler
    ///   surgery.
    ///
    /// The no-rings-on-`f2` rule: [module docs](self). `f2`'s outer may
    /// be `Empty` or `Cycle` — both legal. No half-edge, vertex, or
    /// edge is touched.
    ///
    /// Euler vector, same-shell form: `(v 0, e 0, f −1, h +1, r +1,
    /// s 0)` — arena delta −1 face (the "+1 ring" is the surviving
    /// loop's reclassification, not a mint; genus is derived, not
    /// stored).
    ///
    /// The fusion form's is `(v 0, e 0, f −1, h 0, r +1, s −1)`, `s`
    /// being the SHELL count (the solid count is unchanged — GWB's
    /// §9.2.4 "KFSMR" reading), and its arena delta is −1 face and
    /// −1 shell. **The shell term carries this form's surgery instead
    /// of the genus term**: eq. 9.2 with `Δf = −1`, `Δr = +1` and
    /// `Δs = −1` forces `Δh = 0`, which is the connected sum reading
    /// above — two components' genera add, so no handle is made. A
    /// caller that expects the same-shell `h +1` here is off by a
    /// handle.
    ///
    /// **Pcurve rows** ([`crate::pcurves`]): the demoted loop's stored
    /// rows are a curve stated in `f2`'s chart, so they survive this op
    /// only when `f1` is on the same CHART ([`Body::same_chart`]: one
    /// key, or two keys sharing one payload). When it is not, they are
    /// DROPPED ([`Body::drop_rows`]). Where they do not stand on `f1`
    /// — dropped, or missing — and `f1`'s rows were complete on an
    /// analytic chart, `f1` is owed a re-mint this keys-only kill takes
    /// no band for, so it refuses
    /// [`crate::pcurves::SiteRowRefusal::KeysOnly`] before anything
    /// moves rather than leave `f1` half-minted — the shape of
    /// [`Body::kev`](crate::construction::Construct::kev)'s refusal where a carrier would go stale.
    /// [`Body::kfmrh_minting`](crate::construction::Construct::kfmrh_minting) is the same kill with a band, and makes
    /// that move whole. On a spline chart, or an `f1` that was unminted
    /// or half-minted, the drop is the whole answer at either door.
    ///
    /// **Keys-only.** Where `f1` wears another key than `f2`, the
    /// demoted loop moves onto a chart its edges' descriptions may not
    /// name. This door asks [`Body::ring_move`](crate::construction::Construct::ring_move)'s questions of that move
    /// ([`Body::vouch_move`], as [`RechartDoor::Kfmrh`](crate::RechartDoor::Kfmrh)), on
    /// [`Body::kef`](crate::construction::Construct::kef)'s terms; [`Body::kfmrh_describing`](crate::construction::Construct::kfmrh_describing) takes the move
    /// with its re-descriptions under a band.
    ///
    /// **Minting order**: nothing is minted (the loop survives with its D5
    /// birth record — no provenance changes for survivors; re-homed faces
    /// keep their birth records — re-homing is not a re-birth). **Kill
    /// order** (D9, exact): cross-shell form first re-homes `f2`'s shell's
    /// surviving faces (appended to `f1`'s shell's list in their surviving
    /// order) and kills the shell (with its provenance); then, both forms,
    /// the face `f2` (with its provenance and its null-face record) and
    /// every null-face record naming `f2`'s demoted outer loop, as a
    /// null-face record lives only while its face holds both loops it names
    /// ([`crate::null`]), then `f2`'s surface iff orphaned
    /// ([`Body::remove_surface_if_orphaned`]).
    ///
    /// # Precondition check order
    ///
    /// `f1` resolves, then `f2` ([`BadArgument::Stale`](crate::BadArgument::Stale)); they are
    /// distinct ([`EulerOpError::SameFace`]); cross-shell only: one
    /// solid ([`EulerOpError::CrossSolid`]); `f2` has no rings
    /// ([`EulerOpError::FaceHasRings`]); then the move's keys
    /// ([`EulerOpError::RechartStrandsDescriptions`], then
    /// [`EulerOpError::RechartUnvouched`], each naming every edge, in
    /// the demoted loop's cycle order); then the site mint's plan
    /// ([`Body::plan_moved_rows`]'s errors): where `f1` would be
    /// re-minted, [`EulerOpError::PcurveMint`] naming `f1` (`KeysOnly`
    /// at this door).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    ///
    /// # Panics
    ///
    /// Where the body is not tier-1-valid around the faces: a record
    /// they name does not resolve, the demoted loop's walk is not
    /// exactly the half-edges that claim it, or a record the kill keeps
    /// still names `f2` or, in the fusion form, its shell. Every public
    /// door keeps the body tier-1-valid, so only a kernel bug reaches
    /// these.
    fn kfmrh(&mut self, f1: FaceKey, f2: FaceKey) -> Result<KfmrhResult, EulerOpError>
    where
        T: Decide;
    /// **The describing kill**: [`Body::kfmrh_minting`](crate::construction::Construct::kfmrh_minting)'s kill with the
    /// re-descriptions of the demoted loop's edges, under `tol`'s band
    /// — the twin [`Body::kfmrh`](crate::construction::Construct::kfmrh)'s refusals name, as
    /// [`Body::kef_describing`](crate::construction::Construct::kef_describing) is [`Body::kef`](crate::construction::Construct::kef)'s, and on its terms:
    /// each listed description is stated on the adjacency the kill
    /// leaves, a key `f2` wears standing for `f1`'s
    /// ([`Body::kfmrh_carried_redescriptions`](crate::construction::Construct::kfmrh_carried_redescriptions) states the stored ones
    /// there), certified on the edge's own carrier and interval, and
    /// nothing unlisted is re-described.
    ///
    /// # Precondition check order
    ///
    /// [`Body::kfmrh`](crate::construction::Construct::kfmrh)'s structural list; then
    /// [`Body::kef_describing`](crate::construction::Construct::kef_describing)'s listed-edge, strand and residual
    /// checks, an edge with a half in the demoted loop counting as
    /// moved ([`EulerOpError::NotMovedEdge`] otherwise); then
    /// [`Body::kfmrh_minting`](crate::construction::Construct::kfmrh_minting)'s site mint.
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    ///
    /// # Panics
    ///
    /// As [`Body::kfmrh`](crate::construction::Construct::kfmrh).
    ///
    /// Also on a kernel bug: an edge the body stores, re-minted or
    /// restated on its own carrier and interval, whose span or winding
    /// reads certainly refused at the run's band.
    fn kfmrh_describing(
        &mut self,
        f1: FaceKey,
        f2: FaceKey,
        redescriptions: &[(EdgeKey, geom_brep::EdgeDescriptionSpec<T>)],
        tol: Tol,
    ) -> Result<KfmrhResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    /// [`Body::kfmrh`](crate::construction::Construct::kfmrh) with a band: where `kfmrh` refuses
    /// [`crate::pcurves::SiteRowRefusal::KeysOnly`] — the demoted
    /// loop's rows do not stand on a complete `f1` — this door re-mints
    /// `f1` at `tol`'s band, the demoted loop walked in its chart
    /// ([`Body::plan_moved_rows`]); everywhere else it is `kfmrh`.
    ///
    /// # Errors
    ///
    /// As [`Body::kfmrh`](crate::construction::Construct::kfmrh), except the `KeysOnly` refusal, and the site
    /// mint's plan in its place ([`Body::plan_moved_rows`]'s errors);
    /// and it does not ask the move's keys. Its production caller, the
    /// boolean's seam zip, relies on that move, and
    /// [`Body::kfmrh_describing`](crate::construction::Construct::kfmrh_describing) absorbs it once the zip states its
    /// re-descriptions
    /// (`work/restate/kef-and-kfmrh-across-keys-want-a-describing-door-or-reordered-callers`).
    fn kfmrh_minting(
        &mut self,
        f1: FaceKey,
        f2: FaceKey,
        tol: Tol,
    ) -> Result<KfmrhResult, EulerOpError>
    where
        T: Decide;
    /// **The re-descriptions [`Body::kfmrh_describing`](crate::construction::Construct::kfmrh_describing) would need to
    /// carry `kfmrh(f1, f2)`'s move**: [`Body::kef_carried_redescriptions`](crate::construction::Construct::kef_carried_redescriptions)'
    /// reading for the demoted loop, in its cycle order. Pure.
    ///
    /// # Errors
    ///
    /// [`Body::kfmrh`](crate::construction::Construct::kfmrh)'s structural list: `f1` resolves, then `f2`
    /// ([`BadArgument::Stale`](crate::BadArgument::Stale)); they are distinct
    /// ([`EulerOpError::SameFace`]); they share a solid
    /// ([`EulerOpError::CrossSolid`]); `f2` has no rings
    /// ([`EulerOpError::FaceHasRings`]).
    fn kfmrh_carried_redescriptions(
        &self,
        f1: FaceKey,
        f2: FaceKey,
    ) -> Result<Vec<(EdgeKey, geom_brep::EdgeDescriptionSpec<T>)>, EulerOpError>
    where
        T: Decide;
    /// Reparents a ring from its face to `to_face` (same shell).
    /// **NOT an Euler operator** — see the [module docs](self): the
    /// Euler vector is unchanged, nothing is created or killed, and no
    /// provenance changes (D5 records are birth records; reparenting is
    /// not a re-birth). It is the non-Euler addendum to `mef`/`kfmrh`
    /// (GWB's `lringmv`), needed because `mef` deliberately does not
    /// reclassify the split face's rings and `kfmrh` requires `f2`
    /// ring-free.
    ///
    /// **This op is also ch. 14's `laringmv`** (M3 PR 1 — ring
    /// re-homing after splits change containment; serves ch. 14
    /// `splitconnect` joining, M3 PR 3). Division of labor, ratified
    /// with the M3 plan: the *containment decision* — which face a
    /// ring now geometrically belongs to — is the **caller's**, made
    /// in the splitting/boolean pipeline through PR 2's trilean
    /// point-in-loop classification machinery. `ring_move` takes the
    /// target face explicitly and validates only structural legality
    /// (the ring's current owner, target in the same shell); it never
    /// runs a geometric containment check itself, so a wrong target is
    /// the caller's bug, surfaced by the designation-sensitive
    /// consumers (tier-3 region checks, mass properties), not here.
    ///
    /// Reparenting: the ring leaves its face's `rings`, joins
    /// `to_face.rings` (appended — deterministic order), and its `face`
    /// back-pointer is repointed. Every null-face record naming the ring is
    /// dropped, as a null-face record lives only while its face holds both
    /// loops it names ([`crate::null`]). Moving a ring to its own face is a
    /// documented no-op (`Ok(())`, body untouched — the rings order is NOT
    /// perturbed, keeping replay byte-stable, and a record naming the ring
    /// stands).
    ///
    /// **Pcurve rows** ([`crate::pcurves`]): this door mints and kills
    /// no half-edge, and it still writes the map — a re-parenting is
    /// pure of the ARENAS, not of the pcurve map. A row is a curve
    /// stated in a FACE's chart, so the ring's rows survive the move
    /// only when `to_face` is on the same CHART as the ring's old face
    /// ([`Body::same_chart`]); when it is not they are DROPPED
    /// ([`Body::drop_rows`]). Where they do not stand on `to_face` —
    /// dropped, or missing — and `to_face`'s rows were complete on an
    /// analytic chart, this keys-only door refuses
    /// [`crate::pcurves::SiteRowRefusal::KeysOnly`] before anything
    /// moves, as [`Body::kfmrh`](crate::construction::Construct::kfmrh) does; [`Body::ring_move_minting`](crate::construction::Construct::ring_move_minting)
    /// takes a band and re-mints `to_face` with the ring walked in its
    /// chart. Neither face's other loops are touched, the face the ring
    /// leaves needs nothing beyond the record drop, and the same-face
    /// no-op moves nothing.
    ///
    /// **A ring moved onto a face on another chart is vouched for by
    /// its edges, or refused** ([`RechartDoor::RingMove`](crate::RechartDoor::RingMove)), at both
    /// doors: every edge of the ring is asked [`Body::vouch_move`]'s
    /// questions against `to_face`'s key. The containment decision
    /// stays the caller's; this is the keys' half of it. The lever is
    /// the face the ring moves onto: one on a chart its edges name.
    ///
    /// # Tier-1 preservation (the demotion claim's least obvious case)
    ///
    /// `ring_move` re-glues the per-shell component partition (a face
    /// glues all its loops — validator pass 11), so unlike the Euler
    /// operators its tier-1 preservation is not a per-surgery
    /// Euler-vector fact. It holds anyway, by the separating-curve
    /// argument (independently derived by the PR 5 review): validator
    /// passes 3/4/6 force every component to be a closed oriented
    /// surface, and on a genus-0 component every ring cycle is a
    /// separating (Jordan) curve — so a cross-component move
    /// re-partitions the complex into pieces that are again closed
    /// surfaces with `χ = 2(1 − g)`, `g ≥ 0`; a NON-separating ring
    /// forces its component's genus ≥ 1 before the move, and a move can
    /// merge at most two components, so no move manufactures odd χ or
    /// negative genus. Exercised continuously by the seqgen fuzz lane
    /// (`ring_move` is a candidate mutator) and by the promoted review
    /// sweeps (`tests/review_m1_pr5.rs`: every `ring_move` to depth 2
    /// over adversarial fixtures, including the non-separating-ring
    /// pillow-torus).
    ///
    /// # Precondition check order
    ///
    /// The ring resolves ([`BadArgument::Stale`](crate::BadArgument::Stale)); it is not its face's
    /// outer loop ([`EulerOpError::RingIsOuter`]); `to_face` resolves
    /// (`BadArgument::Stale`); both faces lie in one shell
    /// ([`EulerOpError::CrossShell`]); then, where `to_face`'s key is
    /// not the ring's face's, no edge of the ring is stranded
    /// ([`EulerOpError::RechartStrandsDescriptions`], every one named,
    /// in cycle order) and every certified one names that key
    /// ([`EulerOpError::RechartUnvouched`], the same); then the site
    /// mint's plan ([`Body::plan_moved_rows`]'s errors): where `to_face`
    /// would be re-minted, `KeysOnly` at this door.
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    ///
    /// # Panics
    ///
    /// Where the ring's face does not resolve, or the ring's walk is not
    /// exactly the half-edges that claim it: every public door keeps the
    /// body tier-1-valid, where neither happens.
    fn ring_move(&mut self, ring: LoopKey, to_face: FaceKey) -> Result<(), EulerOpError>
    where
        T: Decide;
    /// [`Body::ring_move`](crate::construction::Construct::ring_move) with a band: where `ring_move` refuses
    /// [`crate::pcurves::SiteRowRefusal::KeysOnly`] — the ring's rows
    /// do not stand on a complete `to_face` — this door re-mints
    /// `to_face` at `tol`'s band, the ring walked in its chart
    /// ([`Body::plan_moved_rows`]); everywhere else it is `ring_move`.
    ///
    /// # Errors
    ///
    /// As [`Body::ring_move`](crate::construction::Construct::ring_move), except the `KeysOnly` refusal, and the
    /// site mint's plan in its place ([`Body::plan_moved_rows`]'s
    /// errors).
    fn ring_move_minting(
        &mut self,
        ring: LoopKey,
        to_face: FaceKey,
        tol: Tol,
    ) -> Result<(), EulerOpError>
    where
        T: Decide;
    /// Splits `edge` at carrier parameter `t`, minting a new vertex at
    /// `carrier(t)` and a new edge so that the parent covers `[t₀, t]`
    /// and the new edge `[t, t₁]` of the unchanged carrier (type docs
    /// for the direction conventions; module docs for the ch. 14/15
    /// consumers).
    ///
    /// Euler vector: `(v +1, e +1, f 0, h 0, r 0, s 0)` — `mev`'s
    /// vector (an edge split IS a mev, applied mid-edge).
    ///
    /// **Interiority is trilean** (Q1): the two sub-spans `t − t₀` and
    /// `t₁ − t` are metered into meters exactly like the certification
    /// span gate (`·radius` for circle carriers) and each must classify
    /// **definitely positive** through the
    /// `split_edge_param_interior` predicate — at/outside an endpoint
    /// refuses ([`EulerOpError::SplitParamNotInterior`]), in-band
    /// escalates ([`EulerOpError::SplitParamEscalated`]; a poisoned
    /// margin, e.g. an unmeterable carrier, escalates the same way).
    ///
    /// **Surgery** (derived from the interior-left rule, replacing the
    /// book's `lmev(e->he1, e->he2->nxt, p)`): with parent halves
    /// `hp = he_plus` (u → v) and `hm = he_minus` (v → u), the new
    /// plus half is spliced immediately **after `hp`** (in `hp`'s
    /// loop) and the new minus half immediately **before `hm`** (in
    /// `hm`'s loop); `hm`'s start is rebound to the new vertex `w`.
    /// `hp` then derives its end as `w` — the parent becomes the
    /// first child with no further pointer changes. Degenerate sites
    /// fall out of the same two splices: a strut
    /// (`next(hp) == hm`) becomes the two-edge strut
    /// `… hp → n⁺ → n⁻ → hm …`, and self-loop parents (u = v,
    /// including one-half-edge circular loops) split into a two-edge
    /// digon — the second splice reads `prev(hm)` **after** the first
    /// splice precisely so these coincidence cases land correctly.
    ///
    /// **Emanating rules** (deterministic): `w.emanating = n⁺` (the
    /// second child's plus half — the analogue of `mev`'s new-vertex
    /// rule); `v.emanating` is rebound to `n⁻` iff it was `hm` (which
    /// no longer starts at v); all other anchors are untouched.
    ///
    /// **Minting order** (D9, exact): point, first-child curve,
    /// second-child curve, vertex, edge, `he_plus`, `he_minus`. **Kill
    /// order**: the parent's curve iff orphaned, after the parent edge
    /// is repointed to the first-child curve.
    ///
    /// # Precondition check order
    ///
    /// `edge` resolves ([`crate::BadArgument::Stale`]); its curve entry
    /// is certified ([`EulerOpError::NullScaffoldCurve`] — null
    /// scaffolding has nothing to split); the interiority trilean
    /// (above); a tangent child's description by the must-carry rule
    /// over the child ([`EulerOpError::SplitSmoothEscalated`],
    /// [`EulerOpError::SplitSmoothBendEscalated`],
    /// [`EulerOpError::SplitSmoothRefuted`]); both child specs certify
    /// ([`EulerOpError::Certification`] — endpoints
    /// `start(hp) → carrier(t)` and `carrier(t) → end(hp)`, he_plus
    /// forward order on each child, through the scalar's edge door).
    ///
    /// **Pcurve rows** ([`crate::pcurves`]): a parent half-edge's
    /// stored chart row passes to both children as each child's own
    /// row. C4 places a row over its own edge's span in the chart's
    /// principal window, so a closed-form or projected child is derived
    /// over its sub-interval exactly as the mint derives it (and is the
    /// mint's row, byte for byte); a fitted or general row, which has
    /// no angular placement, is the parent's restricted to the
    /// sub-interval. Either is certified through the door that states
    /// it (`restate`: `PcurveCache::certify` for an exact row,
    /// `certify_projected`, `certify_fitted` or `certify_general` for
    /// the rest, the last three through the scalar's
    /// [`crate::AtRestPolicy::fitted_lane`]). Both children are
    /// certified in the plan phase
    /// ([`crate::pcurves::split_cache`]), so a face this op touches is
    /// never left half-minted and a refusal
    /// ([`EulerOpError::PcurveSplit`]) arrives with the body
    /// untouched. So do the joints, decided as every joint is: the one
    /// between the two children at the split point, the way the half
    /// runs, and, where a child was placed again, the parent's joints
    /// into and out of it against their neighbours' rows (children
    /// placed apart can sit whole turns from the parent's image). Where
    /// one is undecided the op refuses
    /// ([`EulerOpError::SplitJointUndecided`]) rather than write an
    /// element nothing decided. A half-edge with no row keeps none: the op carries
    /// what is there, and minting what is missing is the producer's
    /// closing mint.
    ///
    /// The carry is exact on every chart; two readers of the split body are
    /// narrower, both stated at `split_cache`. On a SPLINE chart the
    /// recovery step the caveat below names, `mint_pcurves`, refuses on
    /// a described-NURBS wall's split: the iso derivation's rim arms
    /// map an edge's WHOLE interval onto the chart's whole `u` domain,
    /// which a sub-edge no longer spans (pre-existing, filed on ISO's
    /// slate; a split of such a wall no longer NEEDS that pass). And
    /// the trimmed quadrature lane refuses a carried `General` row's
    /// sub-range typed (filed on QUAD's slate).
    ///
    /// # Tier-3 caveat (review F2)
    ///
    /// Splitting a circle rim of an iso-rectangle patch (e.g. a
    /// plane×cylinder intersection rim on an extruded disc) leaves the
    /// curved wall's loop no longer an iso-rectangle: mass properties
    /// / tier 3 then refuse **typed** (`NotIsoRectangle`) on a body
    /// that was tier-3 before the split. Honest and loud, but the
    /// split knocks the body out of the props inventory until the
    /// patch machinery generalizes (pinned in
    /// `review_m3_pr1_sweep.rs::split_circle_carrier_intersection_edge`).
    ///
    /// # Panics
    ///
    /// On a kernel bug: an edge the body stores, re-minted or restated on its
    /// own carrier and interval, whose span or winding reads certainly
    /// refused at the run's band.
    ///
    /// # Errors
    ///
    /// The first failing precondition above, a tangent child the
    /// must-carry rule leaves in band or reads at a corner among them;
    /// the body is untouched on `Err`.
    fn split_edge(
        &mut self,
        edge: EdgeKey,
        t: T,
        tol: Tol,
    ) -> Result<SplitEdgeCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    /// **The join** (`docs/DESIGN.md`, maximal edges): every joinable
    /// vertex ([`joinable_vertices`](crate::joinable_vertices)) of the body, one at a time in
    /// vertex-arena order until none is left, killed and its two edges
    /// made one. The door every finisher ends with, so no body reaches
    /// rest holding a vertex the predicate would join
    /// (`ValidationError::JoinableVertexAtRest`). Returns the joins in
    /// the order made, a later one's `gone` or `kept` possibly an
    /// earlier one's `kept`: a door carrying records or names keyed by
    /// the body's cells reads each join as `vertex → kept`,
    /// `gone → kept`, and `conventional → kept` where it is set.
    ///
    /// Atomic: the joins run on a clone, which is adopted only once every
    /// join is made and, where the body carried pcurve rows, they are
    /// re-derived whole, so no row is left keyed by a killed half-edge.
    /// On any refusal the body is as found. The clone is under one
    /// surgery scope ([`crate::surgery`]) for every kill, so the door
    /// derives tier 1 once, before it adopts.
    ///
    /// # Panics
    ///
    /// On a kernel bug: an edge the body stores, re-minted or restated on its
    /// own carrier and interval, whose span or winding reads certainly
    /// refused at the run's band.
    ///
    /// # Errors
    ///
    /// [`BooleanError::JoinUndecided`], the kill's own refusal
    /// ([`BooleanError::Euler`]), a carrier the joined edge cannot be
    /// restated on ([`BooleanError::JoinCarrierUnsupported`]), the
    /// description's, or the re-mint's ([`BooleanError::Pcurves`]).
    fn join_edges(&mut self, band: Band, tol: Tol) -> Result<Vec<EdgeJoin>, BooleanError>
    where
        T: Decide + crate::props::AtRestPolicy;
    /// [`Body::join_edges`](crate::construction::Construct::join_edges) over the vertices `within` holds, on a body
    /// the caller has staged: the join of a door that owes no write
    /// outside the entities its call writes (the offset doors, and the
    /// blend over the shells it carved), or of one that joins its own
    /// fresh output in place (the shell). Not staged itself —
    /// a refusal past the first kill leaves `self` part-joined, for the
    /// caller to discard with its staging — and no reading outside
    /// `within` is taken, so a vertex the call does not hold can neither
    /// be joined nor refuse it. The rows the kills moved are re-derived
    /// over the joined edges' faces alone: the killed edges' rows are
    /// dropped and [`crate::pcurves::mint_pcurves_of`] runs on the faces
    /// each kept edge bounds, where the body carries rows at all.
    ///
    /// The kills run under one surgery scope ([`crate::surgery`]), so a
    /// run of joins derives tier 1 once, at its close; a run that joins
    /// nothing writes nothing and sweeps nothing.
    ///
    /// # Panics
    ///
    /// On a kernel bug: an edge the body stores, re-minted or restated on its
    /// own carrier and interval, whose span or winding reads certainly
    /// refused at the run's band.
    ///
    /// # Errors
    ///
    /// As [`Body::join_edges`](crate::construction::Construct::join_edges), for the vertices `within` holds.
    fn join_edges_within(
        &mut self,
        band: Band,
        tol: Tol,
        within: &dyn Fn(VertexKey) -> bool,
    ) -> Result<Vec<EdgeJoin>, BooleanError>
    where
        T: Decide + crate::props::AtRestPolicy;
    /// Partitions `shell`'s faces into connected components of the
    /// incidence complex (the validator's pass-11 relation: a face
    /// glues all its loops; a cycle loop glues across each edge via
    /// mate; an empty-loop face is its own dartless component) and
    /// re-homes every component after the first into a **new shell of
    /// the same solid**.
    ///
    /// Returns the component → shell map, in component order:
    /// `result[0]` is always `shell` (which keeps the first
    /// component), `result[i]` for i ≥ 1 are the minted shells. A
    /// connected shell returns `vec![shell]` with the body untouched
    /// (deterministic no-op, like `ring_move`'s).
    ///
    /// The minted shells stay under the one solid, so a shell that falls
    /// apart into separate pieces of material leaves construction state:
    /// several `Outer` shells under one solid, which tier 3's check 10
    /// refuses until the verb that cut it sorts its result
    /// ([`crate::pieces`]).
    ///
    /// **Determinism (D9)**: components are seeded in the shell's
    /// face-list order; the worklist expands loops in (outer, rings)
    /// list order and cycles in `next` order; each new shell's face
    /// list preserves the original list's relative order; new shells
    /// are appended to the solid's shell list in component order.
    /// **Minting order** (exact): the new shells, in component order —
    /// nothing else is minted or killed.
    ///
    /// Tier-1 preservation: components move **whole**, so every edge's
    /// two faces stay in one shell (pass 10) and each new shell's
    /// complex is exactly one component with its old per-component
    /// Euler–Poincaré count (pass 11) — the partition is re-labeled,
    /// never re-cut.
    ///
    /// # Errors
    ///
    /// [`BadArgument::Stale`](crate::euler::BadArgument::Stale) if `shell` does not resolve. All checks
    /// precede any mutation (atomic).
    ///
    /// # Panics
    ///
    /// Where the walk reaches a record that does not resolve, a face that
    /// is not the shell's (either direction), a loop its face does not
    /// own (either direction), or a cycle that is not its loop's whole
    /// cycle: on a tier-1-valid body ownership is mutual and a loop's
    /// `next` cycle is exactly the half-edges that claim it.
    fn movefac(&mut self, shell: ShellKey) -> Result<Vec<ShellKey>, EulerOpError>
    where
        T: Decide;
    /// Moves `shells` — some, not all, of ONE solid's shells — into a
    /// **new solid** and returns it. Not an Euler operator: it
    /// re-partitions ownership, like [`Body::movefac`](crate::construction::Construct::movefac) one level up.
    /// The moved shells keep their keys and their faces; what changes
    /// is their `solid` back-pointer and the two solids' shell lists.
    ///
    /// **Determinism (D9)**: the new solid's shell list is `shells` in
    /// the order given; the source solid keeps its remaining shells in
    /// their relative order. **Minting order** (exact): the one solid,
    /// recorded as [`Provenance::MoveShells`](crate::Provenance::MoveShells) naming the source — nothing
    /// else is minted or killed.
    ///
    /// Tier-1 preservation: shells move **whole**, so every edge's two
    /// faces stay in one shell and every shell's complex is untouched;
    /// both solids keep at least one shell (pass 9's floor).
    ///
    /// **Ownership, not material coherence.** Nothing here reads which
    /// shell is an outer boundary and which a cavity: moving a lone
    /// void mints a solid with no outer shell, and tier 3 accepts it
    /// (`shell5_r2_probes::r2_the_new_door_mints_a_solid_with_no_outer_shell`).
    /// A caller owns the pairing it moves.
    ///
    /// # Errors
    ///
    /// All checks precede any mutation (atomic).
    /// [`EulerOpError::NoShellsNamed`] on an empty list;
    /// [`EulerOpError::ShellRepeated`] when a shell is named twice;
    /// [`BadArgument::Stale`](crate::euler::BadArgument::Stale) if a shell does not resolve;
    /// [`EulerOpError::ShellsAcrossSolids`] if the shells are not all in
    /// one solid; [`EulerOpError::SolidWouldEmpty`] if the list is every
    /// shell of that solid.
    ///
    /// # Panics
    ///
    /// Where a shell's solid does not resolve or does not list it: on a
    /// tier-1-valid body ownership is mutual.
    fn move_shells_to_new_solid(&mut self, shells: &[ShellKey]) -> Result<SolidKey, EulerOpError>
    where
        T: Decide;
    /// Replaces `edge`'s curve geometry with a freshly certified
    /// attachment of `curve`, returning the new curve key. The
    /// certification endpoints are the edge's own vertices' points in
    /// `he_plus` forward order; intrinsic/seam descriptions must also
    /// be **adjacency-coherent** (module docs). The old curve is
    /// removed iff no other edge references it.
    ///
    /// **Every description re-mints the faces its halves are on.** A row
    /// is stated in a face's chart over its edge's carrier and interval,
    /// and this door installs a new carrier, so it keeps no row of the
    /// old one; a null edge's first carrier is the first from which its
    /// halves' rows can be derived ([`Body::mev_null`](crate::construction::Construct::mev_null) returns a minted
    /// face missing them). Before the door mutates, each face the
    /// halves are on that the site mint selects — a minted face whose
    /// every loop walks, and whose only gaps are on loops a null edge
    /// holds open or which no null edge is left on once this one is
    /// described — has every loop that no null edge holds open walked and
    /// completed, through the site mint the Euler operators run
    /// ([`crate::pcurves`]' `site_rows`), the edge's halves under the
    /// curve the door installs: the loop leaves complete — the rows of
    /// halves an operator added while it was held open included, and on
    /// a face no null edge is left on every row it missed — or the face
    /// rowless where the closed-form lane cannot mint it. A loop another
    /// null edge still runs through is left as found, for that edge to
    /// release. A face it finds half-minted is left as found, and so is a
    /// certified edge's face on a spline chart; a row kept there is the
    /// tier-3 pcurve pass's, which re-derives each stored row's agreement
    /// from the edge's current carrier.
    ///
    /// # Errors
    ///
    /// [`BadArgument::Stale`](crate::euler::BadArgument::Stale) if `edge` does not
    /// resolve; [`EulerOpError::DescriptionNotAdjacent`] on a
    /// description whose surfaces are not the edge's faces' surfaces;
    /// [`EulerOpError::Certification`] on a failed gate, the scalar's
    /// edge door ([`crate::AtRestPolicy::attach_edge`]);
    /// [`EulerOpError::PcurveMint`] where a face is
    /// re-minted and a half-edge of it does not resolve.
    /// The body is untouched on `Err`.
    fn set_edge_curve(
        &mut self,
        edge: EdgeKey,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<CurveKey, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    /// Replaces `face`'s surface per the [`FaceSurface`] spec,
    /// returning the face's (possibly unchanged) surface key.
    /// `Inherit` is a no-op (the current surface); `New` mints; `Shared`
    /// reuses an existing key. The old surface is removed iff nothing
    /// else references it (faces or edge descriptions).
    ///
    /// **An edge described against the surface the face leaves is
    /// refused, never stranded.** A description names its surfaces by
    /// key — an `Intersection`'s operands, a `Chart`'s chart — so a
    /// swap leaves every description that named the face's old key
    /// naming a surface the face no longer wears. Where such a
    /// description is adjacency-coherent now and the swap would make it
    /// not (tier 3's `DescriptionNotAdjacent` at rest), the door
    /// refuses before mutating, naming every such edge. That question
    /// is exact key adjacency, so the door takes no band;
    /// [`Body::set_face_surfaces_describing`](crate::construction::Construct::set_face_surfaces_describing) takes a band and the
    /// re-descriptions.
    ///
    /// **A moved face's boundary is vouched for by its edges'
    /// certificates, or the swap is refused.** A certified edge is
    /// certified on the keys its description names: its samples lie on
    /// each within the band, and its ends on its vertices. So where the
    /// face moves onto another chart, every certified edge on it must
    /// name the key the face wears after the swap; the door refuses
    /// before mutating, naming every one that does not
    /// ([`EulerOpError::RechartUnvouched`]), on every chart kind. A
    /// `New` key is one no description names. A move onto
    /// [`Body::same_chart`]'s one payload is not asked, since a
    /// certificate is a function of the payload it was taken on.
    ///
    /// The question is [`Body::unvouched`]'s, which every re-chart door
    /// asks: scaffold and null edges, and an empty loop's lone vertex,
    /// carry no certificate and are not asked. This door reads keys
    /// alone; [`Body::set_face_surfaces_describing`](crate::construction::Construct::set_face_surfaces_describing) also reads residuals
    /// against a plane, and neither reads them on a curved chart.
    ///
    /// **The face's pcurve rows are not a cache this door may keep.** A
    /// row is a curve stated in a face's CHART
    /// ([`crate::pcurves`]), so re-charting the face in place makes
    /// every one of its rows a statement about a surface the face is no
    /// longer on — the loop-re-parenting doors' defect with the two
    /// sides swapped, and it takes their answer: a swap onto the same
    /// chart carries every row untouched, and a swap onto a different
    /// one drops the rows of the face's loops, proven whole in the plan
    /// ([`Body::face_cycles`]), deriving nothing. A caller that wants the face's rows on its new chart
    /// runs [`crate::pcurves::mint_pcurves`]. Leaving them was silent
    /// wherever the new surface does not mint — tier 3's pcurve pass
    /// skips such a face — so what the drop removes is a wrong row no
    /// reader could be warned about.
    ///
    /// **Sense** ([`crate::Face::sense`]): the face stands as its own
    /// parent and the door passes [`ParentSide::With`](crate::euler::ParentSide::With), so the bit is
    /// kept on the face's own chart and stated on any other
    /// ([`Body::resolve_face_surface`]).
    ///
    /// # Errors
    ///
    /// [`BadArgument::Stale`](crate::euler::BadArgument::Stale) if `face` does not resolve;
    /// [`BadArgument::StaleGeometry`](crate::euler::BadArgument::StaleGeometry) if a `Shared` key
    /// does not resolve; [`EulerOpError::SenseContradictsChart`] if a
    /// spec on the face's own chart states the other bit; then
    /// [`EulerOpError::RechartStrandsDescriptions`], then
    /// [`EulerOpError::RechartUnvouched`]. The body is untouched on
    /// `Err`.
    fn set_face_surface(
        &mut self,
        face: FaceKey,
        surface: FaceSurface<T>,
    ) -> Result<SurfaceKey, EulerOpError>
    where
        T: Decide;
    #[cfg(any(test, feature = "test-support", feature = "sweep-testing"))]
    #[doc(hidden)]
    /// **Failure-injection door** (test builds only: this crate's own
    /// tests, `test-support` and `sweep-testing`):
    /// [`Body::set_face_surface`](crate::construction::Construct::set_face_surface) with its refusals of a swap it cannot
    /// vouch for taken out, so a swap may leave edges described against
    /// the surface the face leaves — the state tier 3 reports at rest as
    /// `DescriptionNotAdjacent` — or move a face onto a chart no
    /// certified edge of it names. Every other precondition and every
    /// write is the real door's.
    ///
    /// It is for a row that builds such a body on purpose, and only
    /// there: a swap [`Body::set_face_surfaces_describing`](crate::construction::Construct::set_face_surfaces_describing) takes goes
    /// through it, with
    /// [`Body::carried_redescriptions`] where the stored descriptions
    /// are what the row wants. Every call carries a one-line `// Lifts`
    /// comment naming the refusal it takes out and why that state is
    /// the row's premise; the reasons live there, beside the code they
    /// govern, and nowhere else.
    ///
    /// # Errors
    ///
    /// [`Body::set_face_surface`](crate::construction::Construct::set_face_surface)'s, but for
    /// [`EulerOpError::RechartStrandsDescriptions`] and
    /// [`EulerOpError::RechartUnvouched`].
    fn set_face_surface_unvouched_for_tests(
        &mut self,
        face: FaceKey,
        surface: FaceSurface<T>,
    ) -> Result<SurfaceKey, EulerOpError>
    where
        T: Decide;
    /// Re-charts faces with the re-descriptions of the edges the move
    /// would strand: [`Body::set_face_surface`](crate::construction::Construct::set_face_surface)'s swap for every face of
    /// every [`Rechart`], taking a band and, for each edge the swap
    /// would strand, the spec that describes it on the moved charts.
    /// Returns each chart's key, in `charts` order: the minted key of a
    /// [`Rechart::new`] chart, the given one of a [`Rechart::shared`].
    ///
    /// The describing sibling of the keys-only surface setter, as
    /// [`Body::kev_describing`](crate::construction::Construct::kev_describing) is [`Body::kev`](crate::construction::Construct::kev)'s. It takes every chart
    /// that door takes — a fresh surface ([`FaceSurface::New`]) or one
    /// the body holds ([`FaceSurface::Shared`]) — so every swap that
    /// door refuses has a way through here. Faces, descriptions and the
    /// band arrive together, and nothing is written until everything
    /// has certified.
    ///
    /// It moves several charts in one call because an edge between two
    /// moving charts certifies on neither pair of mixed charts. Two
    /// single-chart calls reach the same body only by describing such
    /// an edge as a scaffold in the first, and that intermediate is a
    /// state tier 3 refuses at rest (`ScaffoldAtRest`); one call never
    /// shows it.
    ///
    /// **Nothing is re-described by default.** An edge the move would
    /// strand — [`Body::set_face_surface`](crate::construction::Construct::set_face_surface)'s question, asked of every
    /// moved face — is refused unless it is listed
    /// ([`EulerOpError::RechartUndescribed`]), however its stored
    /// description would read on the moved charts.
    /// [`Body::carried_redescriptions`] states those stored descriptions
    /// on the moved charts, for a caller that chooses them.
    ///
    /// **A key an edge's face wore stands, in that edge's listed spec,
    /// for the chart the face moves onto** — unless one of the edge's
    /// faces still wears it after the move, when it stands for itself.
    /// A new chart has no key before it is minted, so a spec names the
    /// chart it is stated in by the key the face wears now; where both
    /// faces wore the key and move apart, the `he_plus` side's chart is
    /// the one named. A listed spec is certified against its current
    /// endpoints and the moved charts, adjacency coherence against its
    /// faces' moved charts included, and its curve is replaced by fresh
    /// insertion. Any certified edge may be listed; a null edge may not
    /// (its first description is [`Body::set_edge_curve`](crate::construction::Construct::set_edge_curve)'s, which mints
    /// its rows).
    ///
    /// **What it certifies:** every listed description, on every chart
    /// kind; and that every moved face's boundary is vouched for on its
    /// new chart ([`Body::unvouched`], the question the keys-only doors
    /// ask). A certified edge is vouched for where its description —
    /// the listed one, or the stored one — names the chart its face
    /// moves onto; where none does and the chart is a plane, by its
    /// residuals: its ends and interior certification samples within
    /// the band of the plane, tier 3's planar residual checks asked
    /// before the move. A curved chart's residuals are not read, here
    /// or by tier 3 at rest
    /// (`work/restfront/validate-tier3-curved-boundary-containment`,
    /// #638), so onto a curved chart an edge no description names
    /// there is refused, as the keys-only door refuses it. Scaffold and
    /// null edges, and an empty loop's lone vertex, carry no
    /// certificate and are not asked.
    ///
    /// Each face's sense is [`FaceSurface`]'s rule, per face; its
    /// pcurve rows are [`Body::set_face_surface`](crate::construction::Construct::set_face_surface)'s (kept on the same
    /// chart, dropped on another); a re-described edge's rows on a face
    /// that keeps its chart are kept as found, for the tier-3 pcurve
    /// pass to re-certify against the new carrier, where
    /// [`Body::set_edge_curve`](crate::construction::Construct::set_edge_curve) re-mints that face
    /// (`work/restate/set-face-surfaces-describing-keeps-a-moved-edges-rows-on-a-kept-chart`).
    ///
    /// Minting order (D9): the new charts' surfaces in `charts` order,
    /// then the listed edges' curves in list order.
    ///
    /// # Precondition check order
    ///
    /// `tol` builds a band ([`EulerOpError::Certification`]). Per chart
    /// in order, per face in order: the face resolves
    /// ([`BadArgument::Stale`](crate::euler::BadArgument::Stale)), was not listed before
    /// ([`EulerOpError::FaceMovedTwice`]), a shared chart's key resolves
    /// ([`BadArgument::StaleGeometry`](crate::euler::BadArgument::StaleGeometry)) and the face
    /// states a sense its chart admits
    /// ([`EulerOpError::SenseContradictsChart`]). Per listed edge in
    /// order: it resolves (`BadArgument::Stale`), was not listed
    /// before ([`EulerOpError::DuplicateRedescription`]), is not a null
    /// edge ([`EulerOpError::NullScaffoldCurve`]), its spec is
    /// adjacency-coherent on the moved charts
    /// ([`EulerOpError::DescriptionNotAdjacent`]) and certifies at its
    /// endpoints through the scalar's edge door
    /// ([`EulerOpError::RechartFalsifies`]). Then no unlisted
    /// edge is stranded ([`EulerOpError::RechartUndescribed`], every one
    /// named). Then per moved face in order onto a chart that is not its
    /// old one, per loop (outer, then rings) and half-edge in cycle
    /// order, each certified edge no description names there: on a
    /// plane, the half's start vertex, its end, then its edge's
    /// interior samples lie on it
    /// ([`EulerOpError::RechartOffBoundary`] /
    /// [`EulerOpError::RechartBoundaryEscalated`]); on a curved chart,
    /// it is refused ([`EulerOpError::RechartUnvouched`], every one on
    /// the face named).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    fn set_face_surfaces_describing(
        &mut self,
        charts: Vec<Rechart<T>>,
        redescriptions: &[(EdgeKey, EdgeCurveSpec<T>)],
        tol: Tol,
    ) -> Result<Vec<SurfaceKey>, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    /// Sets `face`'s orientation sense ([`crate::Face::sense`]) in
    /// place, on the chart it already has.
    ///
    /// A face minted or re-charted states its bit with its chart
    /// instead ([`FaceSurface`]; [`Body::resolve_face_surface`] owns
    /// the rule), so this door is for a bit learned while the chart
    /// stands still. Callers must keep the two encodings of
    /// orientation coherent — the bit and the loop winding — and the
    /// obligation is the CALLER'S, because at rest it is only partly
    /// checkable: tier 3's check 6 falsifies a planar disagreement
    /// whose loop rides `Line`, `Circle` and `Ellipse` carriers, and
    /// passes over one whose loop rides a spiric or NURBS carrier (the
    /// residue named at the arm's banner), so a planar face bounded so
    /// can carry an inverted bit through this door and certify. The
    /// test-only hand-flip door [`Body::flipped_face_sense_for_tests`]
    /// is the deliberate exception to the coherence rule; it is not the
    /// only way to break it.
    ///
    /// Not an Euler operator (no topology changes) and not a numeric
    /// decision (a `bool` is written, nothing compared); tier 1 is
    /// trivially preserved.
    ///
    /// # Errors
    ///
    /// [`BadArgument::Stale`](crate::euler::BadArgument::Stale) if `face` does not
    /// resolve. The body is untouched on `Err`.
    fn set_face_sense(&mut self, face: FaceKey, sense: bool) -> Result<(), EulerOpError>
    where
        T: Decide;
    /// **A stored edge restated under a new description**: `edge` keeps
    /// its own stored carrier and interval, verbatim, and only its
    /// description becomes `description`. The re-description an
    /// operation outside `topo` makes of an edge it minted, once the
    /// surfaces that describe it exist, goes through here rather than
    /// [`Body::set_edge_curve`](crate::construction::Construct::set_edge_curve): the caller hands no carrier, interval
    /// or origin, and the door certifies the stored edge as the kernel's.
    ///
    /// # Errors
    ///
    /// [`BadArgument::Stale`](crate::euler::BadArgument::Stale) if `edge`
    /// does not resolve; [`EulerOpError::NullScaffoldCurve`] on an edge
    /// with no certified curve; then [`Body::set_edge_curve`](crate::construction::Construct::set_edge_curve)'s.
    ///
    /// # Panics
    ///
    /// On a kernel bug: a stored edge whose own span or winding reads
    /// certainly refused at the run's band. That check reads the carrier,
    /// the interval and the band, never the description. The carrier and
    /// interval are the stored edge's, which `topo` certified when it
    /// stored them at `Band::linear(tol)` of the run's one [`Tol`], so
    /// they read certainly refused here only where the kernel stored an
    /// edge its own gate refuses, whatever `description` says.
    fn restate_stored_edge(
        &mut self,
        edge: EdgeKey,
        description: EdgeDescriptionSpec<T>,
        tol: Tol,
    ) -> Result<CurveKey, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    /// Re-states `edge` where it rests ([`crate::rest_locus`]), keeping
    /// its carrier, its parameter interval and the pushforward that
    /// scaffolded it, as its authority record. Where it rests is read
    /// off its faces, so no caller chooses a chart, and a pair is stated
    /// only where the must-carry rule, asked here over the edge
    /// ([`geom_brep::reask_pair`]), reads the join under-determined: a
    /// corner is stated as the pair's `Intersection` and a determinate
    /// tangency as its `TangentIntersection`. A door that asked the rule
    /// already gets the same answer again; one that did not (a same-key
    /// lane, a revolve's rims) cannot state a pair the rule refutes.
    ///
    /// This is D2's **conventional split**, at rest: the edge is between
    /// two real faces now, so it is described where it rests and not as
    /// the scaffolding the mint left (D3's transience fence; the
    /// scaffolding door is for edges whose surfaces do not exist yet).
    /// An edge already described keeps its description.
    ///
    /// **The carrier is RESTATED, never rebuilt.** Re-deriving it from
    /// the endpoints recomputes the direction and the interval from a
    /// sum that need not be bitwise what the edge was minted with,
    /// which silently moves geometry in a pass whose whole contract is
    /// that only the DESCRIPTION moves.
    ///
    /// One home for a rule the sweep lanes read: `extrude`'s strut
    /// join, `revolve`'s join lanes and `swept`'s scaffold-retirement
    /// pass. It lives here, beside [`Body::set_edge_curve`](crate::construction::Construct::set_edge_curve), because
    /// that is the certified mutation it performs.
    ///
    /// # Panics
    ///
    /// On a kernel bug: an edge the body stores, re-minted or restated on its
    /// own carrier and interval, whose span or winding reads certainly
    /// refused at the run's band.
    ///
    /// # Errors
    ///
    /// [`BadArgument::Stale`](crate::euler::BadArgument::Stale) if `edge` does not
    /// resolve; [`EulerOpError::NullScaffoldCurve`] on an edge whose curve
    /// carries no certified geometry to re-state;
    /// [`EulerOpError::RestJoinUndecided`] and
    /// [`EulerOpError::RestJoinMixed`] where the rule over a pair is in
    /// band or mixed; and whatever [`Body::set_edge_curve`](crate::construction::Construct::set_edge_curve) raises on the
    /// re-attachment.
    fn describe_at_rest(&mut self, edge: EdgeKey, tol: Tol) -> Result<(), EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    /// Marks `face` as a null face with the given F9 loop-role
    /// attribute (replacing any existing mark — the record is data the
    /// splitting/boolean pipeline maintains as it builds the pair).
    ///
    /// Preconditions: the face resolves, both role loops resolve, they
    /// are distinct, and both are the face's own — its outer loop or
    /// one of its rings. A null face is one face's two coincident loops
    /// (module docs), so a record naming another face's loop describes
    /// none, and the door refuses it before it writes. The type cannot
    /// say it instead: the record names its loops by explicit key
    /// (F9), not by outer-vs-ring designation.
    ///
    /// Every op that removes a named loop, or moves it off its face,
    /// drops the record rather than refusing (module docs), so no op
    /// leaves a record naming a loop its face does not hold, and tier 1
    /// checks ownership too
    /// ([`crate::ValidationError::StaleNullFaceOwnership`]). Tier 2
    /// refuses marked faces at rest
    /// ([`crate::ValidationError::NullFaceAtRest`]).
    ///
    /// # Errors
    ///
    /// [`crate::BadArgument::Stale`] if the face or a role loop does not
    /// resolve; [`EulerOpError::SameLoop`] if the two role loops are
    /// one loop; [`EulerOpError::NullPairForeignLoop`] naming the first
    /// role loop, in declaration order, that is not the face's own. The
    /// body is untouched on `Err`.
    fn set_null_face_pair(&mut self, face: FaceKey, pair: NullFacePair) -> Result<(), EulerOpError>
    where
        T: Real;
    /// Removes `face`'s null-face mark, returning it (or `None` if the
    /// face was not marked — total, like the accessor: clearing is the
    /// consuming pipeline's bookkeeping, not a checked surgery).
    fn clear_null_face_pair(&mut self, face: FaceKey) -> Option<NullFacePair>
    where
        T: Real;
    /// Attaches a **certified** pcurve cache to `half_edge`, returning
    /// the row it replaced.
    ///
    /// The argument's type is the guarantee: a [`PcurveCache`] cannot
    /// be built except through certification, so no uncertified pcurve
    /// can enter a body through this door (or any other). What this
    /// door cannot check is *coherence with this body* — that the cache
    /// was certified against THIS half-edge's carrier, THIS face's
    /// surface, and a branch consistent with its loop. That is the
    /// tier-3 pcurve pass's job ([`crate::pcurves::validate_pcurves`]),
    /// which re-derives all three and never consults the stored
    /// certificate.
    ///
    /// The ordinary producer is [`crate::pcurves::mint_pcurves`], which
    /// the splitting lane runs on every side it mints.
    fn attach_pcurve(
        &mut self,
        half_edge: HalfEdgeKey,
        cache: PcurveCache<T>,
    ) -> Option<PcurveCache<T>>
    where
        T: Real;
    /// Removes and returns `half_edge`'s stored pcurve cache —
    /// [`Body::attach_pcurve`](crate::construction::Construct::attach_pcurve)'s inverse (same trust posture: the
    /// tier-3 pcurve pass owns coherence, and a curved face left
    /// missing a row fails it loudly). Consumers of caches refuse
    /// typed on absence; nothing re-derives a branch silently.
    fn detach_pcurve(&mut self, half_edge: HalfEdgeKey) -> Option<PcurveCache<T>>
    where
        T: Real;
    /// Attaches the element of the joint into `half_edge`, returning the
    /// one it replaced — [`Body::attach_pcurve`](crate::construction::Construct::attach_pcurve)'s companion, under the
    /// same trust posture: the tier-3 pcurve pass re-decides every joint
    /// ([`crate::pcurves::validate_pcurves`]). The element is stored in
    /// its canonical form ([`JointElement::canonical`]: a reset's
    /// azimuth periods dropped).
    fn attach_joint(
        &mut self,
        half_edge: HalfEdgeKey,
        element: JointElement,
    ) -> Option<JointElement>
    where
        T: Real;
    /// Removes and returns the element of the joint into `half_edge` —
    /// [`Body::attach_joint`](crate::construction::Construct::attach_joint)'s inverse.
    fn detach_joint(&mut self, half_edge: HalfEdgeKey) -> Option<JointElement>
    where
        T: Real;
    #[must_use = "a scope that is opened and immediately dropped silences nothing"]
    /// **Opens a surgery scope on this body** — the door's declaration
    /// that it, and not the operators it runs, owns the tier-1
    /// postcondition.
    ///
    /// The returned guard derefs to the body, so the door's operator
    /// sequence is written through it unchanged. Close it with
    /// [`Surgery::sweep_and_close`] on the success path; an `Err`
    /// returned mid-sequence drops the guard instead, which closes
    /// the scope and sweeps nothing.
    ///
    /// See the [module docs](self) for what the scope claims and for
    /// the per-operator scalpel that recovers the failing operator's
    /// name.
    fn begin_surgery(&mut self) -> Surgery<'_, T>
    where
        T: Real;
}

impl<T: geom_core::Real> Construct<T> for Body<T> {
    fn mvfs(&mut self, point: Point3<T>, sense: bool) -> Result<MvfsCreated, EulerOpError>
    where
        T: Decide,
    {
        Body::mvfs(self, point, sense)
    }

    fn mev(
        &mut self,
        site: MevSite,
        point: Point3<T>,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<MevCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::mev(self, site, point, curve, tol)
    }

    fn mev_line(
        &mut self,
        site: MevSite,
        point: Point3<T>,
        tol: Tol,
    ) -> Result<MevCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::mev_line(self, site, point, tol)
    }

    fn mev_null(
        &mut self,
        site: MevSite,
        new_side: NewVertexSide,
    ) -> Result<MevCreated, EulerOpError>
    where
        T: geom_core::Decide,
    {
        Body::mev_null(self, site, new_side)
    }

    fn mef(
        &mut self,
        site: MefSite,
        curve: EdgeCurveSpec<T>,
        surface: FaceSurface<T>,
        tol: Tol,
    ) -> Result<MefCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::mef(self, site, curve, surface, tol)
    }

    fn mef_chord(&mut self, site: MefSite, tol: Tol) -> Result<MefCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::mef_chord(self, site, tol)
    }

    fn mekr(
        &mut self,
        site: MekrSite,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<MekrResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::mekr(self, site, curve, tol)
    }

    fn mekr_chord(&mut self, site: MekrSite, tol: Tol) -> Result<MekrResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::mekr_chord(self, site, tol)
    }

    fn mfkrh(
        &mut self,
        ring: LoopKey,
        surface: FaceSurface<T>,
    ) -> Result<MfkrhCreated, EulerOpError>
    where
        T: Decide,
    {
        Body::mfkrh(self, ring, surface)
    }

    fn mfkrh_minting(
        &mut self,
        ring: LoopKey,
        surface: FaceSurface<T>,
        tol: Tol,
    ) -> Result<MfkrhCreated, EulerOpError>
    where
        T: Decide,
    {
        Body::mfkrh_minting(self, ring, surface, tol)
    }

    fn mfkrh_plug(&mut self, ring: LoopKey, sense: bool) -> Result<MfkrhCreated, EulerOpError>
    where
        T: Decide,
    {
        Body::mfkrh_plug(self, ring, sense)
    }

    fn kev(&mut self, he: HalfEdgeKey) -> Result<KevResult, EulerOpError>
    where
        T: Decide,
    {
        Body::kev(self, he)
    }

    fn kev_describing(
        &mut self,
        he: HalfEdgeKey,
        redescriptions: &[(EdgeKey, EdgeCurveSpec<T>)],
        tol: Tol,
    ) -> Result<KevResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::kev_describing(self, he, redescriptions, tol)
    }

    fn kev_merged_members(&self, he: HalfEdgeKey) -> Result<Vec<MergedMember<T>>, EulerOpError>
    where
        T: Decide,
    {
        Body::kev_merged_members(self, he)
    }

    fn kef(&mut self, he: HalfEdgeKey) -> Result<KefResult, EulerOpError>
    where
        T: Decide,
    {
        Body::kef(self, he)
    }

    fn kef_describing(
        &mut self,
        he: HalfEdgeKey,
        redescriptions: &[(EdgeKey, EdgeDescriptionSpec<T>)],
        tol: Tol,
    ) -> Result<KefResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::kef_describing(self, he, redescriptions, tol)
    }

    fn kef_minting(&mut self, he: HalfEdgeKey, tol: Tol) -> Result<KefResult, EulerOpError>
    where
        T: Decide,
    {
        Body::kef_minting(self, he, tol)
    }

    fn kef_carried_redescriptions(
        &self,
        he: HalfEdgeKey,
    ) -> Result<Vec<(EdgeKey, EdgeDescriptionSpec<T>)>, EulerOpError>
    where
        T: Decide,
    {
        Body::kef_carried_redescriptions(self, he)
    }

    fn kvfs(&mut self, solid: SolidKey) -> Result<KvfsResult, EulerOpError>
    where
        T: Decide,
    {
        Body::kvfs(self, solid)
    }

    fn kemr(&mut self, he1: HalfEdgeKey, he2: HalfEdgeKey) -> Result<KemrResult, EulerOpError>
    where
        T: Decide,
    {
        Body::kemr(self, he1, he2)
    }

    fn kemr_minting(
        &mut self,
        he1: HalfEdgeKey,
        he2: HalfEdgeKey,
        tol: Tol,
    ) -> Result<KemrResult, EulerOpError>
    where
        T: Decide,
    {
        Body::kemr_minting(self, he1, he2, tol)
    }

    fn kfmrh(&mut self, f1: FaceKey, f2: FaceKey) -> Result<KfmrhResult, EulerOpError>
    where
        T: Decide,
    {
        Body::kfmrh(self, f1, f2)
    }

    fn kfmrh_describing(
        &mut self,
        f1: FaceKey,
        f2: FaceKey,
        redescriptions: &[(EdgeKey, geom_brep::EdgeDescriptionSpec<T>)],
        tol: Tol,
    ) -> Result<KfmrhResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::kfmrh_describing(self, f1, f2, redescriptions, tol)
    }

    fn kfmrh_minting(
        &mut self,
        f1: FaceKey,
        f2: FaceKey,
        tol: Tol,
    ) -> Result<KfmrhResult, EulerOpError>
    where
        T: Decide,
    {
        Body::kfmrh_minting(self, f1, f2, tol)
    }

    fn kfmrh_carried_redescriptions(
        &self,
        f1: FaceKey,
        f2: FaceKey,
    ) -> Result<Vec<(EdgeKey, geom_brep::EdgeDescriptionSpec<T>)>, EulerOpError>
    where
        T: Decide,
    {
        Body::kfmrh_carried_redescriptions(self, f1, f2)
    }

    fn ring_move(&mut self, ring: LoopKey, to_face: FaceKey) -> Result<(), EulerOpError>
    where
        T: Decide,
    {
        Body::ring_move(self, ring, to_face)
    }

    fn ring_move_minting(
        &mut self,
        ring: LoopKey,
        to_face: FaceKey,
        tol: Tol,
    ) -> Result<(), EulerOpError>
    where
        T: Decide,
    {
        Body::ring_move_minting(self, ring, to_face, tol)
    }

    fn split_edge(
        &mut self,
        edge: EdgeKey,
        t: T,
        tol: Tol,
    ) -> Result<SplitEdgeCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::split_edge(self, edge, t, tol)
    }

    fn join_edges(&mut self, band: Band, tol: Tol) -> Result<Vec<EdgeJoin>, BooleanError>
    where
        T: Decide + crate::props::AtRestPolicy,
    {
        Body::join_edges(self, band, tol)
    }

    fn join_edges_within(
        &mut self,
        band: Band,
        tol: Tol,
        within: &dyn Fn(VertexKey) -> bool,
    ) -> Result<Vec<EdgeJoin>, BooleanError>
    where
        T: Decide + crate::props::AtRestPolicy,
    {
        Body::join_edges_within(self, band, tol, within)
    }

    fn movefac(&mut self, shell: ShellKey) -> Result<Vec<ShellKey>, EulerOpError>
    where
        T: Decide,
    {
        Body::movefac(self, shell)
    }

    fn move_shells_to_new_solid(&mut self, shells: &[ShellKey]) -> Result<SolidKey, EulerOpError>
    where
        T: Decide,
    {
        Body::move_shells_to_new_solid(self, shells)
    }

    fn set_edge_curve(
        &mut self,
        edge: EdgeKey,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<CurveKey, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::set_edge_curve(self, edge, curve, tol)
    }

    fn set_face_surface(
        &mut self,
        face: FaceKey,
        surface: FaceSurface<T>,
    ) -> Result<SurfaceKey, EulerOpError>
    where
        T: Decide,
    {
        Body::set_face_surface(self, face, surface)
    }

    #[cfg(any(test, feature = "test-support", feature = "sweep-testing"))]
    #[doc(hidden)]
    fn set_face_surface_unvouched_for_tests(
        &mut self,
        face: FaceKey,
        surface: FaceSurface<T>,
    ) -> Result<SurfaceKey, EulerOpError>
    where
        T: Decide,
    {
        Body::set_face_surface_unvouched_for_tests(self, face, surface)
    }

    fn set_face_surfaces_describing(
        &mut self,
        charts: Vec<Rechart<T>>,
        redescriptions: &[(EdgeKey, EdgeCurveSpec<T>)],
        tol: Tol,
    ) -> Result<Vec<SurfaceKey>, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::set_face_surfaces_describing(self, charts, redescriptions, tol)
    }

    fn set_face_sense(&mut self, face: FaceKey, sense: bool) -> Result<(), EulerOpError>
    where
        T: Decide,
    {
        Body::set_face_sense(self, face, sense)
    }

    fn restate_stored_edge(
        &mut self,
        edge: EdgeKey,
        description: EdgeDescriptionSpec<T>,
        tol: Tol,
    ) -> Result<CurveKey, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::restate_stored_edge(self, edge, description, tol)
    }

    fn describe_at_rest(&mut self, edge: EdgeKey, tol: Tol) -> Result<(), EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::describe_at_rest(self, edge, tol)
    }

    fn set_null_face_pair(&mut self, face: FaceKey, pair: NullFacePair) -> Result<(), EulerOpError>
    where
        T: Real,
    {
        Body::set_null_face_pair(self, face, pair)
    }

    fn clear_null_face_pair(&mut self, face: FaceKey) -> Option<NullFacePair>
    where
        T: Real,
    {
        Body::clear_null_face_pair(self, face)
    }

    fn attach_pcurve(
        &mut self,
        half_edge: HalfEdgeKey,
        cache: PcurveCache<T>,
    ) -> Option<PcurveCache<T>>
    where
        T: Real,
    {
        Body::attach_pcurve(self, half_edge, cache)
    }

    fn detach_pcurve(&mut self, half_edge: HalfEdgeKey) -> Option<PcurveCache<T>>
    where
        T: Real,
    {
        Body::detach_pcurve(self, half_edge)
    }

    fn attach_joint(
        &mut self,
        half_edge: HalfEdgeKey,
        element: JointElement,
    ) -> Option<JointElement>
    where
        T: Real,
    {
        Body::attach_joint(self, half_edge, element)
    }

    fn detach_joint(&mut self, half_edge: HalfEdgeKey) -> Option<JointElement>
    where
        T: Real,
    {
        Body::detach_joint(self, half_edge)
    }

    fn begin_surgery(&mut self) -> Surgery<'_, T>
    where
        T: Real,
    {
        Body::begin_surgery(self)
    }
}
