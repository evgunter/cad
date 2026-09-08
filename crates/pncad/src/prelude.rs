//! The curated common surface: `use pncad::prelude::*;`.
//!
//! The inventory is *measured*, not chosen by taste. It is what the
//! tour scenes, the STEP-export corpus, and the
//! document-layer corpus actually import — the authoring vocabulary a
//! model needs on the way from a coordinate table to an exported
//! solid. Everything the corpus reaches for less than corpus-wide
//! stays one module hop away (`pncad::topo::…`), which keeps the
//! glob import small enough to be honest about what it brings in.
//!
//! **What guards that, and what does not** (issue #667's Q6). SUFFICIENCY
//! is guarded: `pncad/tests/all.rs` authors the whole ladder — profile,
//! body, booleans, validate, mesh, export — through the prelude ALONE, so
//! a name dropped from this list that a journey still needs fails to
//! compile there. MINIMALITY is not.
//!
//! The shape of it follows the user journey the tour documents:
//!
//! 1. **Numbers and frames** — points, vectors, transforms,
//!    tolerance, the decision `Band` the refusals quote, and the
//!    f64-first constructors
//!    from [`crate::authoring`] so no literal needs `from_f64`.
//! 2. **Author a profile** — the PATHS lattice, loops, sketch planes.
//! 3. **Build a body** — extrude, revolve, loft/sweep, fillet.
//! 4. **Combine** — the Boolean operations and their declarations.
//! 5. **Validate** — tiers 1→2→3, the ladder as the journey.
//! 6. **Measure** — mass properties.
//! 7. **Tessellate and export** — mesh, STL, STEP.
//! 8. **Or drive the document layer** — `Doc`, `DocEdit`, `evaluate`.
//!
//! `BooleanOp` is ONE type: the kernel operation, which the recipe
//! node carries directly. The prelude and `pncad::document` name the
//! same enum, so which one a caller imports it through cannot change
//! what it means.

// --- 1. Numbers and frames ------------------------------------
pub use crate::authoring::{p2, p3, polygon, real, v2, v3, validated};
// `Band`/`BandError` are here because the verbs' typed refusals quote
// them: a caller that matches `BlendError::Band` (this prelude) or
// `ShellError::Band` (reachable as `pncad::topo::ShellError`, through
// the wholesale `topo` re-export) has to be able to name what the arm
// carries. No prelude operation
// TAKES a band — every kernel verb derives `Band::linear(tol)` from the
// tolerance witness at its own entry (ε as the coincidence threshold,
// K·ε as the escalation threshold), so a band is a thing the user reads
// out of a refusal, not a thing the user supplies. Constructing one
// directly (`Band::new`, `Band::angular_at`) is a geometry-layer move.
// `Tol` is here for a different reason one rung down: it is the first
// argument of every authoring call that decides anything, and a
// prelude that cannot name it is a prelude you cannot author from.
// `Tol::witness()` is the one line a program writes before modelling
// — see `crate::tolerance`.
//
// **`BandField` is NOT here, and that is a measured decision, not an
// oversight.** It is `BandError::InvalidValue`'s discriminant, so it
// looks exactly like the payloads groups 3, 5 and 9 carry beside
// their refusals. What makes it different is that the discriminant is
// CONSTANT at this boundary:
//
// Every kernel verb derives its band from `Band::linear(tol)` at its
// own entry (the paragraph above), which is `Band::new(ε, K·ε)`.
// `Tol`'s invariant is ε finite and strictly positive and K finite
// and greater than one, so `Band::new`'s `zero` check cannot fire —
// the only `InvalidValue` reachable from a prelude door is
// `field: Escalate`, the K·ε overflow residue that `Band::linear`'s
// own docs call unreachable for any physically meaningful tolerance.
// The other producer, `Band::angular_at`, has no caller anywhere in
// the workspace outside `geom_core`'s OWN tests — no kernel verb
// reaches it — and `Band::new` is called directly only in
// geometry-layer interiors and test fixtures. `geom_core`'s
// `band_tolerance` test is worth naming rather than waving at: it is
// the one place `angular_at`'s overflow residue is exercised, and it
// lands on `field: Escalate` too.
//
// So a curated `BandField` would publish a two-arm type whose only
// use here is comparison against a value it always has. CUR3 carried
// `DanglingRef` because its two arms are two different facts about
// the model and a caller branches on which; `BandField`'s two arms
// collapse to one reachable fact, and the field's name is already in
// the refusal's own `Display` prose. A caller who really does
// construct a band directly has left the prelude for the geometry
// layer by definition, and there `Band`, `BandError` and `BandField`
// sit at ONE root together (`pncad::geom_core`) — contract clause 1,
// one crate, one path.
//
// This flips if `Band::angular_at` ever acquires a kernel caller:
// `ε/lever_arm` can underflow to zero for a large enough arm, which
// makes `field: Zero` reachable and the discriminant real. Stated so
// the next curation pass re-measures rather than re-deriving.
//
// **`Indeterminate` IS here, and the count is the argument.** It is
// the two-tolerance escalation payload (D4 ¶1 addendum): the thing a
// refusal carries when the kernel could not certify a sign at the
// tolerance it was given. THIRTEEN prelude-curated refusals carry it
// — `BlendError`, `BooleanError`, `ContactRefusal`, `ExtrudeError`,
// `LoftError`, `MateFault`, `PathError`, `ProfileError`,
// `RevolveError`, `SelectRefusal`, `TubeError`, `UnitVec3Error`,
// `ValidationError` — against one carrier for the payload CUR3
// carried and one apiece for CUR4's four. A caller holding an
// `Escalated` arm out of any of them reads `band` off it to decide
// whether tightening ε would help, and could not name what it was
// holding without a module hop.
//
// **`MarginDiag` is NOT here, and that is measured, the `BandField`
// reading one payload over.** It is `Indeterminate::margin`'s type:
// the in-band value, the enclosure that straddles the boundary, or
// the fact that the margin was poisoned. Its own documentation says
// what settles it — the arms are "here for error messages and margin
// telemetry, not to be branched on", because recovering the margin to
// make the sign decision the classifier refused is exactly what the
// escalation contract forbids. The recourse is the same three levers
// whichever arm it is, and `COINCIDENCE_RECOURSE` states them once
// for all of them.
//
// The boundary measurement agrees: no consumer of these refusals
// reads the discriminant. Every one of the thirteen crosses into
// Python as a single tag plus the kernel's own prose (`escalated`,
// `in_band`, `pair_in_band`, `mate_indeterminate`), and no attribute
// on any bound exception carries a margin, an enclosure bound or a
// band. So a curated `MarginDiag` would publish a three-arm type
// whose arms nothing branches on, which is `BandField`'s situation
// with a different reason for it: there the discriminant was constant,
// here it varies and is not a decision.
//
// This flips if a door ever projects the escalation's own shape — a
// caller told "the enclosure straddles" can subdivide where one told
// "the margin is in band" can only widen ε. Stated so the next
// curation pass re-measures rather than re-deriving. Either way
// `Indeterminate`, `MarginDiag` and `Band` sit at ONE root together
// (`pncad::geom_core`), so the fallback is a module hop and never a
// second crate.
pub use geom_core::{
    Affine3, Band, BandError, Indeterminate, Mat3, Point2, Point3, Real, Tol, Tolerance, Vec2, Vec3,
};
// The D6 quantity layer: value types, unit constants
// (`25.0 * MM`), and the display formatter. NAME DISCIPLINE: this
// `Length` is the API-boundary quantity newtype; the kernel-internal
// classify-seam margin type is `geom_core::predicate::Margin<T>`
// (renamed from `Length<T>`), which has never been prelude
// surface and must not become it. Scope: the prelude carries the
// value types + the six unit constants + the formatter; the unit
// TABLE itself and the prefix data (`UNITS`, `unit_by_symbol`,
// `MILLI`, `CENTI`, `ONE`) stay one module hop away at
// `pncad::quantity`, per the corpus-measured prelude rule (module
// docs above).
// `WrittenLength`/`WrittenAngle` are value types by the same rule and
// ride here for the same reason `Length` does: they are what an
// authored quantity IS at this boundary — a magnitude plus the
// notation it was written in — and `Expr::written_length` is the door
// they open, which is how a library recipe records `300 mm` rather
// than `0.3` for a reader to interpret.
pub use quantity::{
    Angle, AngleUnit, CM, Count, DEG, FmtQuantityError, IN, Length, LengthUnit, M, MM, PI, RAD,
    WrittenAngle, WrittenLength, fmt_angle, fmt_length,
};

// --- 2. Profile authoring -------------------------------------
// NAMEABLE, NOT MINTABLE:
// `ProfileLoop` and `ProfileVertex` stay here because read-back hands
// them back, `ProfileError` payloads point into them, and `validated`
// takes a `Vec<ProfileLoop>` — a prelude user must be able to name what
// the ladder passes around. What left is the raw MINTING tier:
// `ProfileLoop::new`/`polygon` live on `profile::RawLoop`, which is a
// FIXTURE door behind that crate's `test-support` feature — absent from
// every shipped build, so there is nothing here to decline. Loops are
// authored through the lattice below, and a table that already exists
// crosses scalars through `ProfileLoop::map`.
pub use ::profile::{
    ArcSweep, FilletLegShape, Profile, ProfileError, ProfileLoop, ProfileVertex, SegmentKind,
    SketchPlane, ValidatedLoop, ValidatedProfile, bulge_from_center, bulge_from_via,
};
// The PATHS authoring algebra: `circle` (the one-step closed-carrier
// program form) and the
// §2c spec modes the one `arc_to(spec)` leg and the fused fillet
// family dispatch through (`Bulge`/`Via`/`Center`/`Radius`/`Sweep`/
// `ArcLen` + the `ArcSide` bit), with the states an arc arrival can
// leave the chain in.
// `ClosedLoop` and `circle_split` belong to this group because a
// closing verb RETURNS a `ClosedLoop`, so a prelude user could hold
// the value and not name its type; and `circle_split` is the
// declared-subdivision carrier the boss corpus authors with (the
// `bossplate` scene's three-arc rim IS one), so `circle` alone left
// half of the closed-carrier vocabulary a crate away.
pub use ::profile::{
    ArcLen, ArcSide, ArrivesTangent, Bulge, Center, ClosedLoop, ContinueTarget, CornerReason,
    CornerRefusal, CornerWindow, LineTarget, Open, PartialPath, PathError, PathNoCornerReason,
    Radius, Start, Sweep, TangentArcTarget, Via, circle, circle_split,
};

// --- 3. The four body operations ------------------------------
pub use sweep::blend::{BlendError, Filleted, fillet_edges};
// The fillet's ruled sibling shares its refusal vocabulary
// (`BlendError`, above): one verb, one edge-blend front door, the
// band the only difference.
pub use sweep::chamfer::{Chamfered, chamfer_edges};
// `BlendKind` names WHICH blend a shared refusal came from — the
// recipe layer's `Node::Chamfer` and `Node::Fillet` carry one kernel
// error type between them, so the discriminant has to cross with it.
// `BlendRefusal` is how it crosses at the kernel doors: the refusal
// both `fillet_edges` and `chamfer_edges` return, the verb attached
// once around the shared verb-neutral error.
pub use sweep::blend::{BlendKind, BlendRefusal};
// **The blend refusal's own payload vocabulary**, carried beside the
// refusal for the reason `DanglingRef` rides beside `ReadbackError`
// in group 9: a curated list owes MATCHABILITY, not just
// nameability, and `BlendError` is on this list while the types its
// arms carry were not.
//
// The reach is worse here than it was there. `DanglingRef` at least
// sat at `topo`'s root, so `pncad::topo::DanglingRef` named it;
// `sweep` re-exports NOTHING from `blend`, so before this line the
// only spelling was `pncad::sweep::blend::CornerConfig` — and
// `pncad::sweep::blend::battery::Convexity`, three submodules deep.
// Contract clause 1 was met (one crate, a longer path); what was not
// met is that a caller holding a prelude-curated `BlendError` could
// not branch on what its arms say.
//
// What each one decides, which is why matchability is worth the four
// names:
//
// - `CornerConfig` is `UnsupportedCorner`'s whole content (the OQ6
//   vocabulary, decided at #85). A valence-4 vertex says "request the
//   other edges"; a `SeamVertex` says "ask for the rim whole", a door
//   that EXISTS — and `CornerConfig::policy` is the map that keeps a
//   refusal from disagreeing with its own tag. Different recourses,
//   so a caller branches, which is exactly CUR3's test.
// - `RunOutPolicy` is that arm's second field, and `None` is a fact
//   of its own: a seam vertex is not a corner, so no run-out helps.
// - `BlendSite` is `Escalated`'s: link, joint, or the chain whole —
//   the payload half of the two-tolerance shape (D4 ¶1 addendum).
// - `Convexity` is `ConvexitySignFlip`'s: which way the chain's
//   material wedge turns. It is NOT one of the three the CUR3 bank
//   named — the struct-payload sweep found it — and leaving it out
//   would ship a `BlendError` matchable in three arms and not the
//   fourth.
//
// The Python half of the CUR3 treatment does NOT apply to these, and
// that is measured rather than skipped: `BlendError` projects no arms
// into Python at all (`node_error_tag` reads the VERB, giving one
// `fillet`/`chamfer` tag for the whole refusal), so there is no tag
// here to split and none to pin. That the blend door is unprojected
// is #1479's census row, not this list's business.
pub use sweep::blend::battery::Convexity;
pub use sweep::blend::{BlendSite, CornerConfig, RunOutPolicy};
// **`RevolvedKind` is `Revolved::kind`**, and it is the same claim one
// value over: the payloads above are what a carried REFUSAL says, this
// is what a carried RESULT says, and a curated list owes matchability
// on both.
//
// `revolve` hands back a `Revolved` — the key bundle a downstream
// pass addresses the new body by — and its `kind` field is the
// ratified case split (the revolve module docs), which is not a label
// on one shape but two disjoint sets of handles. A `Partial`
// revolution has wedge CAPS (`start_cap`, `end_cap`) and both
// meridian chains; a `Full` one has no caps at all, one seam chain,
// and — in the wire case, where the profile touches the axis — a
// second π-band whose walls, meridians and rims are four more fields
// that exist on no other arm. So "which faces did my revolve make,
// and which can I fillet" is answered by branching on this
// discriminant and by nothing else: the fields are not `Option`s on
// one struct that a caller could probe, they are arms.
//
// Every key kind the arms carry is already on this list (`FaceKey`,
// `EdgeKey`, group 4), so the rung under it closes here rather than
// stopping. And no Python tag moves: `Revolved` does not cross at all
// — Python speaks the document layer, where a revolve is a
// `Node.revolve` whose answer is a body — so there is nothing here to
// split or pin, the `BlendError` reading on a value rather than a
// refusal.
pub use sweep::{
    ExtrudeError, Extruded, Extrusion, LoftError, Lofted, Revolution, RevolveAxis, RevolveError,
    Revolved, RevolvedKind, TubeError, TubeWindow, extrude, loft_body, revolve, sweep_body,
    tube_along_arc, tube_along_arc_hollow,
};

// --- 4. Bodies and Booleans -----------------------------------
// `geom_brep::SurfaceKind` rides here on purpose: it is the payload
// of `BooleanError::CurvedBooleanUnsupported`, so any code that
// matches on a curved-Boolean refusal needs it in the same breath as
// the error itself — the one-dependency contract's closure over
// error payloads (crate docs, contract clause 1).
pub use geom_brep::SurfaceKind;
// `PlaneRelation` rides here because it is the verdict a
// `FlushFinding`'s evidence carries (SameOpposite = resting contact,
// SameOriented = flush walls), so code inspecting findings names it.
//
// **The KEY set is four, not three.** `VertexKey`, `EdgeKey` and
// `FaceKey` were here and `LoopKey` was not, and the missing one is
// named by a refusal this prelude already carries:
// `ValidationError::RingMeetsOuter` is `{ face: FaceKey, ring:
// LoopKey, contact: RingContact }`, and `RingContactEscalated` beside
// it carries a ring too. So the arm named three key types, two of
// which a prelude consumer could spell. The claim a key makes here is
// weaker than a payload enum's — a key is bound and passed on, never
// branched on — but the inconsistency was inside ONE group of this
// list rather than a rung below it, and it went unseen because
// `LoopKey` is minted by `slotmap::new_key_type!`: a scan of `pub
// struct` declarations cannot see a macro's output, so no payload
// sweep this façade has run could report it.
//
// **`EntityId` and `GeomRef` ride with the keys**, because they are
// what the same module means by "any entity" and "any geometry": the
// type-erased sums over those keys, which is the form a refusal
// reports a site in. `ReadbackError::Dangling`'s payload is one or
// the other, and `BlendError` names an `EntityId` DIRECTLY in three
// arms — so a caller matching a prelude-carried refusal was binding
// a value whose type sits in this very group and could not spell it.
// The four key kinds neither sum reaches by a curated name
// (`SolidKey`, `ShellKey`, `HalfEdgeKey`, and the three geometry
// keys) are bound out of an arm and never branched on, and they are
// one module hop away at `pncad::topo::…` — the rung this list stops
// at, deliberately, for the reason CUR3 and CUR4 both stopped one
// rung out: what a curated list owes is the DISCRIMINANT, and both
// sums' discriminants are matchable now.
//
// **This is the `topo` seat's vocabulary and not the document
// layer's**, which is the distinction the arena-key seal is actually
// drawn on. `pncad::topo::query` answers keys from a `Body` and this
// group carries the keys it answers with; what does not cross is
// `editor-core`'s `EntityRef`/`EntityKey`/`Entry` — the document
// layer's (body index, key) pair and its name-table entry, which are
// body-lineage-scoped against the evaluation that minted them and
// which `pncad`'s own guard forbids naming.
//
// **`MappedCurve` is NOT here, and that is measured — the third
// entry of the `BandField` / `MarginDiag` family and the first whose
// reason is the ARM rather than the payload.** It is
// `EdgeDescription::Scaffold`'s payload: the sketch pushforward a D3
// scaffolding description carries. Three measurements settle it.
//
// The arm is fenced to CONSTRUCTION and refused at rest. `Scaffold`
// is the transient door — a pushforward standing in for an edge whose
// surfaces do not exist yet — and tier 3 refuses one on any edge that
// has two adjacent faces (`ValidationError::ScaffoldAtRest`). This
// list carries the validation ladder in group 5, so the bodies a
// prelude consumer holds are the ones the ladder passed, and the arm
// names a state they do not occupy.
//
// Nothing here can state one either. The authoring form is
// `EdgeDescriptionSpec`, which no curated list names, so a façade
// caller can neither be handed a scaffold at rest nor write one. In
// the kernel the discriminant has consumers and every one of them is
// a re-MINT rather than a read — `transform`, the axial offset and
// `replace_face` each match the three arms to rebuild the same arm
// under a map — which is the construction half of the CUR3 rule, on
// the side of the boundary that owns the scaffolding.
//
// And the rung is uncarried WHOLE, not at one arm.
// `EdgeDescription::Chart` carries a `ChartCurve`, whose `pcurve`
// field is a four-arm `Pcurve` — a real discriminant a reader of an
// at-rest conventional edge would branch on — and neither is curated.
// Carrying `MappedCurve` alone would make this refusal's read-back
// matchable at the fenced arm and not at the one every at-rest
// conventional edge actually has, which is the inconsistency the
// `Convexity` carriage in group 3 closed, in reverse.
//
// This flips if a door ever hands a caller the description of a
// TRANSIENT edge — the Euler scaffolding surface, which this façade
// does not expose. When it does, the rung is carried WHOLE
// (`ChartCurve`, `Pcurve` and `MappedCurve` together), because the
// fenced arm is the last one a reader needs and not the first.
// Stated so the next curation pass re-measures rather than
// re-deriving.
pub use topo::{
    Body, BooleanBody, BooleanDeclarations, BooleanError, BooleanOp, BooleanResult,
    BooleanResultKind, ContactRecords, Curve3, EdgeDescription, EdgeKey, EntityId, FaceKey,
    GeomRef, LoopKey, Operand, PlaneRelation, Surface, TransformError, VertexKey, intersect,
    intersect_with, subtract, subtract_with, transform_rigid, union, union_with,
};

// --- 5. The validation ladder ---------------------------------
// The tier-3′ census vocabulary rides with `ValidationError` for the
// group-9 reason again, and here the list was already half-persuaded:
// `DeclaredContact` — the payload of `ValidationError::
// ContactContradicted` — has been curated (through `crate::select`)
// all along, so the surface already carried ONE payload of this
// refusal and left its siblings a module hop away. These four are
// that inconsistency closed, not a new policy:
//
// - `CensusContact` is `UndeclaredContact`'s: which coincidence the
//   census found. The arms are not one fact — `EdgeFacePierce` is
//   interpenetration and categorically undeclarable until the C6 era,
//   while `EdgeEdgeOverlap` is certifiable through the D3
//   reconstruction TODAY. A caller that cannot tell them apart cannot
//   tell "declare this" from "you cannot declare this".
// - `StaleDeclaration` is `StaleContactDeclaration`'s: which record
//   lost its witness, so which record to withdraw.
// - `RingContact` is `RingMeetsOuter`'s: vertex-on-vertex,
//   vertex-on-edge, or edge-along-edge.
// - `CensusSubject` is what `CensusUnsupported` and
//   `CensusLaneUnsupported` are ABOUT, and it is the sharpest of the
//   four because both of its payload types are already on this list.
//   `Entity(EntityId)` and `FacePair(FaceKey, FaceKey)` are not two
//   spellings of one site: an entity subject is one carrier outside
//   the certifiable inventory, and the recourse is that carrier's —
//   simplify it, or certify it through a supported lane. A face PAIR
//   is a candidate CONTACT, and its recourse is the declaration
//   protocol: declare the coincidence, or separate the two faces.
//   The pair is also unordered as a subject (its `PartialEq` says so
//   rather than deriving the arena's order), which is a fact a caller
//   resolving a refusal against its own records has to have. So the
//   discriminant is the whole of what these two arms say, and until
//   it was carried it was the ONLY thing between a caller and a
//   subject whose parts this list already spelled.
//
// THE RUNG BELOW IS CARRIED TOO, and where it stops is one further
// down. `CensusContact::ConformalPatch` carries a
// `topo::ContactFinding`, which rides in group 9 with the rest of the
// contact vocabulary; `DanglingRef`'s arms carry `EntityId` and
// `GeomRef`, which ride in group 4 with the keys they sum over. What
// this list still stops at is the rung under THOSE — a
// `ContactFinding` is a `DeclaredContact` plus a `ContactVerdict`,
// both already here, a `CensusSubject` is an `EntityId` or two
// `FaceKey`s, all three already here, and the sums' remaining key
// kinds are bound out of an arm and never branched on. The rule is
// unchanged and only
// the reach moved: a caller binds a payload and branches on the
// DISCRIMINANT, and every discriminant these refusals name is
// spellable from this list.
//
// As with the blend vocabulary above, no Python tag moves: the
// validate doors cross their failures as joined `Display` prose with
// a `door` and a `failure_count` and no per-arm tag at all, so there
// is nothing here to split or pin.
pub use topo::{
    CensusContact, CensusSubject, RingContact, StaleDeclaration, ValidationError, validate,
    validate_closed, validate_geometric, validate_pseudomanifold,
};

// --- 6. Mass properties ---------------------------------------
pub use topo::{MassProperties, MassPropsError, PropsQuadLane, mass_properties};

// --- 7. Tessellation and export -------------------------------
pub use mesh::{Mesh, TessellateError, tessellate};
pub use step_export::{StepExportError, StepOptions, step_string, write_step};
// **`PromotedKind` is `StepImportError::RecognitionAmbiguous`'s
// discriminant** — which analytic kind's estimator declined on a face
// that could not import without promotion (D7 stage 1, ruling #256).
// It is a `pub` field of a prelude-carried refusal, so the arm was
// matchable and the fact it reports was not.
//
// A caller branches on it because the two kinds have different next
// moves. `Plane` declining means the patch is not flat within ε_in
// and the recourse is the tolerance or the emitting CAD's own
// planarity; `Cylinder` declining means the axis estimator was
// ill-conditioned — collinear azimuth samples — and the recourse is a
// re-export with more of the patch, or the face left as NURBS. The
// refusal's own prose names the kind, which is exactly the
// `DanglingRef` situation this curation rung exists to close: a fact
// readable only out of a message is not matchable.
//
// **`ImportContact` is a different defect and is carried for a
// different reason: it is an INPUT, not a payload.**
// `ImportOptions::declared_contacts` is a `pub Vec<ImportContact>` —
// the import-side declaration channel (M9-2, D7 step 4), the position
// anchored declarations a caller attaches to an import so they are
// certified by the SAME tier-3′ gate a native declared-contact body
// runs. `ImportOptions` was on this list and `ImportContact` was not,
// which is not a matchability gap but a REACH one: a prelude caller
// could name the options struct, could set `eps_in`, and could not
// put a declaration in it, because filling a public field means
// spelling its element type. Every other door on this list is
// callable from it; this one was callable and not fillable, so the
// declaration channel had no prelude caller at all.
//
// The rung under both closes here rather than stopping.
// `PromotedKind`'s arms carry nothing, and an `ImportContact` is one
// arm carrying three `f64` — the anchor position — so a caller states
// a declaration in numbers it already holds.
//
// The Python halves differ, and the difference follows the carrier
// each time. `StepImportError` projects a tag, so `PromotedKind`
// projects its own beside it (`StepImportError.promoted_kind`);
// `ImportOptions` does not cross at all — Python's `import_step`
// takes only the text — so `ImportContact` has nothing to project
// until that argument does.
pub use step_import::{ImportContact, ImportOptions, PromotedKind, StepImportError, import_step};
// `StlError` is the writers' own refusal type — what `write_ascii` and
// `write_binary` return. The option errors beside it
// (`BinaryHeaderError`, `SolidNameError`) refuse at option
// CONSTRUCTION; a prelude that carries the writers but not the type
// they fail with leaves a caller unable to match on the failure it can
// actually get.
pub use stl::{
    AsciiOptions, BinaryHeader, BinaryHeaderError, BinaryOptions, SolidName, SolidNameError,
    StlError, write_ascii, write_binary,
};

// --- 8. The document layer ------------------------------------
// `parse_expr` is the expression TEXT door: the checking
// parser whose every reduction runs the Expr smart constructors;
// `unparse` is the same door outward, source text the parser reads
// back as the same tree.
// The v4 program vocabulary: the profile payload is the
// Expr-bearing `ProfileProgram`, curated through the ONE document
// surface (`crate::document`). `Datum` and `ParamEnv` ride here
// because a datum node is the frame a
// `GeomPred::DatumDistance` selection is written against, and
// `select_where` takes a `ParamEnv`, so both are needed to write a
// position filter at all.
// `ParamName` and `DocParam` ride here because they are what
// `DocEdit::SetDocParam` and `Expr::param` take, so a prelude user
// could previously hold the param-editing doors and not open them —
// the parametric flagship (`plate_param`, guide §3.2) imports both.
pub use crate::document::{
    CancelToken, Datum, Dimension, Doc, DocEdit, DocParam, EditError, EvalOptions, Evaluation,
    Expr, LoopProgram, Node, NodeError, ParamEnv, ParamName, ParseError, PatternKind, ProfileLift,
    ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId, RecordedProgramError,
    SlotId, StepArg, ValuePayload, apply, evaluate, parse_expr, unparse,
};
pub use editor_core::StableName;

// --- 9. Names: obtain them, inspect them, select them ---------
// `StableName` sits in group 8 with no door there to obtain or read a
// value of it: the naming table, the whole-body
// materializers and the key→name inversions all stayed one crate
// away, so a prelude user could hold the type and do nothing with it.
// These are that door, curated as one group in `crate::select` (whose
// module docs carry the worked examples).
// The detect/declare protocol rides in this group too: the
// findings vocabulary, the detector, and the declare sugar (the
// worked example is in `crate::select`'s module docs).
// `DanglingRef` rides beside `ReadbackError` for the same reason
// every carried refusal's payload does: it is what `Dangling`'s
// field IS, so without it the arm is matchable and its two lanes
// are not.
pub use crate::select::{
    ALL_SURFACE_KINDS, CONTACT_RECOURSE, CapEnd, Cmp, ContactClass, ContactFinding, ContactRefusal,
    ContactVerdict, CurveKind, CurveKindSet, DanglingRef, DeclareError, DeclaredContact,
    Denotation, EntityKind, FIT_DEFERRAL, FlushEvidence, FlushFinding, FlushRung, GeomPred,
    InterrogateError, MeridianEnd, NameOrigin, NamePat, NameTable, OpGroup, Pose, ProfileEdgeRef,
    ProfileVertexRef, ReadbackError, RimSupport, RolePath, RoleSeg, SEL_DATUM_DISTANCE, SegPat,
    SegTag, SelectRefusal, Selector, Side, SplitHalf, SurfaceKindSet, TagPat, all_bodies,
    all_edges, all_faces, all_vertices, attribute, declare, declare_all, declare_node, denotation,
    edge_frame, edge_name, face_carrier_kind, face_frame, face_name, find_flush_candidates, select,
    select_where, vertex_position,
};
// The KERNEL query seat (`topo::query`): the same selection
// vocabulary as a pure function of a `Body`, for the caller who holds
// arena keys and no document. Re-exported as the MODULE, not its
// items, because the two seats' materializers deliberately share
// names — `all_edges` above answers names from an evaluation,
// `query::all_edges` answers keys from a body — and a prelude must
// not make one shadow the other. The vocabulary types the doors speak
// (`CurveKind`, `CurveKindSet`, `SurfaceKindSet`, `SurfaceKind`) are
// already above, one definition re-exported upward.
pub use topo::query;
// The KERNEL flush seat (`topo::flush`): the same detect/declare
// protocol as a pure function of two `Body`s, for the caller who holds
// arena keys and no document. A MODULE for the reason `query` is one,
// and more sharply — all three door names collide with the document
// seat's above (`find_flush_candidates`, `declare`, `declare_all`),
// which answer names from an evaluation where `flush::` answers keys
// from a body. The finding vocabulary the doors speak
// (`ContactClass`, `FlushEvidence`, `FlushRung`, `PlaneRelation`) is
// already above, one definition re-exported upward: `FlushFinding` is
// literally the same type at both seats, over each seat's pair.
pub use topo::flush;
