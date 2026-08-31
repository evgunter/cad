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
//!    tolerance, the decision `Band`, and the f64-first constructors
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
pub use crate::authoring::{p2, p3, real, v2, v3, validated};
// `Band` is here because `fillet_edges` (group 3) takes one: a prelude
// operation whose arguments are not prelude-constructible is a rung the
// user cannot start from. The recipe is `Band::linear(tol)` — the run's
// tolerance ε as the coincidence threshold, K·ε as the escalation
// threshold — which is what every kernel operation builds internally.
// `Tol` is here for the same reason one rung down: it is the first
// argument of every authoring call that decides anything, and a
// prelude that cannot name it is a prelude you cannot author from.
// `Tol::witness()` is the one line a program writes before modelling
// — see `crate::tolerance`.
pub use geom_core::{
    Affine3, Band, BandError, Mat3, Point2, Point3, Real, Tol, Tolerance, Vec2, Vec3,
};
// The D6 quantity layer: value types, unit constants
// (`25.0 * MM`), and the display formatter. NAME DISCIPLINE: this
// `Length` is the API-boundary quantity newtype; the kernel-internal
// classify-seam margin type is `geom_core::predicate::Margin<T>`
// (renamed from `Length<T>`), which has never been prelude
// surface and must not become it. Scope: the prelude carries the
// value types + the six unit constants + the formatter; the unit
// TABLE itself and the prefix data (`UNITS`, `unit_by_symbol`,
// `MILLI`, `CENTI`) stay one module hop away at `pncad::quantity`,
// per the corpus-measured prelude rule (module docs above).
pub use quantity::{
    Angle, AngleUnit, CM, Count, DEG, FmtQuantityError, IN, Length, LengthUnit, M, MM, PI, RAD,
    fmt_angle, fmt_length,
};

// --- 2. Profile authoring -------------------------------------
// NAMEABLE, NOT MINTABLE:
// `ProfileLoop` and `ProfileVertex` stay here because read-back hands
// them back, `ProfileError` payloads point into them, and `validated`
// takes a `Vec<ProfileLoop>` — a prelude user must be able to name what
// the ladder passes around. What left is the raw MINTING tier:
// `ProfileLoop::new`/`polygon` now live on `profile::RawLoop`, which is
// kernel vocabulary and is re-exported by neither this prelude nor
// `crate::profile`. Loops are authored through the lattice below.
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
    ArcLen, ArcSide, Bulge, Center, ClosedLoop, LineTarget, Open, PartialPath, PathError,
    PathNoCornerReason, Radius, Start, Sweep, TangentArcTarget, Via, circle, circle_split,
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
pub use sweep::{
    ExtrudeError, Extruded, Extrusion, LoftError, Lofted, Revolution, RevolveAxis, RevolveError,
    Revolved, TubeError, TubeWindow, extrude, loft_body, revolve, sweep_body, tube_along_arc,
    tube_along_arc_hollow,
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
pub use topo::{
    Body, BooleanBody, BooleanDeclarations, BooleanError, BooleanOp, BooleanResult,
    BooleanResultKind, ContactRecords, Curve3, EdgeDescription, EdgeKey, FaceKey, Operand,
    PlaneRelation, Surface, TransformError, VertexKey, intersect, intersect_with, subtract,
    subtract_with, transform_rigid, union, union_with,
};

// --- 5. The validation ladder ---------------------------------
pub use topo::{
    ValidationError, validate, validate_closed, validate_geometric, validate_pseudomanifold,
};

// --- 6. Mass properties ---------------------------------------
pub use topo::{MassProperties, MassPropsError, PropsQuadLane, mass_properties};

// --- 7. Tessellation and export -------------------------------
pub use mesh::{Mesh, TessellateError, tessellate};
pub use step_export::{StepExportError, StepOptions, step_string, write_step};
pub use step_import::{ImportOptions, StepImportError, import_step};
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
    ALL_SURFACE_KINDS, CONTACT_RECOURSE, CapEnd, Cmp, ContactClass, ContactRefusal, ContactVerdict,
    CurveKind, CurveKindSet, DanglingRef, DeclareError, DeclaredContact, Denotation, EntityKind,
    FIT_DEFERRAL, FlushEvidence, FlushFinding, FlushRung, GeomPred, InterrogateError, MeridianEnd,
    NamePat, NameTable, OpGroup, Pose, ProfileEdgeRef, ProfileVertexRef, ReadbackError, RimSupport,
    RolePath, RoleSeg, SEL_DATUM_DISTANCE, SegPat, SegTag, SelectRefusal, Selector, Side,
    SplitHalf, SurfaceKindSet, TagPat, all_bodies, all_edges, all_faces, all_vertices, declare,
    declare_all, declare_node, denotation, edge_frame, edge_name, face_frame, face_name,
    find_flush_candidates, select, select_where, vertex_position,
};
