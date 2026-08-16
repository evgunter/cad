//! The curated common surface: `use pncad::prelude::*;`.
//!
//! The inventory is *measured*, not chosen by taste. It is what the
//! eighteen tour scenes, the STEP-export corpus, and the
//! document-layer corpus actually import — the authoring vocabulary a
//! model needs on the way from a coordinate table to an exported
//! solid. Everything the corpus reaches for less than corpus-wide
//! stays one module hop away (`pncad::topo::…`), which keeps the
//! glob import small enough to be honest about what it brings in.
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
//! One deliberate omission: `BooleanOp` exists in both `topo` (the
//! kernel operation) and `editor_core` (the recipe node's operation).
//! The prelude carries the kernel's; document-layer code spells
//! `pncad::document::BooleanOp`. A prelude cannot re-export two
//! types under one name, and silently preferring one while shadowing
//! the other in a glob is exactly the kind of surprise a curated
//! surface exists to prevent.

// --- 1. Numbers and frames ------------------------------------
pub use crate::authoring::{p2, p3, real, v2, v3, validated};
// `Band` is here because `fillet_edges` (group 3) takes one: a prelude
// operation whose arguments are not prelude-constructible is a rung the
// user cannot start from. The recipe is `Band::linear()` — the run's
// tolerance ε as the coincidence threshold, K·ε as the escalation
// threshold — which is what every kernel operation builds internally.
pub use geom_core::{Affine3, Band, BandError, Mat3, Point2, Point3, Real, Tolerance, Vec2, Vec3};
// The D6 quantity layer (LIB-U8a): value types, unit constants
// (`25.0 * MM`), and the display formatter. NAME DISCIPLINE: this
// `Length` is the API-boundary quantity newtype; the kernel-internal
// classify-seam margin type is `geom_core::predicate::Margin<T>`
// (renamed from `Length<T>`, LB9), which has never been prelude
// surface and must not become it. Scope: the prelude carries the
// value types + the six unit constants + the formatter; the unit
// TABLE itself and the prefix data (`UNITS`, `unit_by_symbol`,
// `MILLI`, `CENTI`) stay one module hop away at `pncad::quantity`,
// per the corpus-measured prelude rule (module docs above).
pub use quantity::{
    Angle, AngleUnit, CM, Count, DEG, FmtQuantityError, IN, Length, LengthUnit, M, MM, RAD,
    fmt_angle, fmt_length,
};

// --- 2. Profile authoring -------------------------------------
// NAMEABLE, NOT MINTABLE (LIB-RETTAIL, Evan's ruling on #413):
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
// The PATHS authoring algebra (LIB-U2), with the LIB-G1 vocabulary
// growth: `circle` (the one-step closed-carrier program form) and the
// target traits the new arc binding modes dispatch through.
// `ClosedLoop` and `circle_split` joined this group in LIB-PYG1: a
// closing verb RETURNS a `ClosedLoop`, so a prelude user could hold
// the value and not name its type; and `circle_split` is the
// declared-subdivision carrier the boss corpus authors with (the
// `bossplate` scene's three-arc rim IS one), so `circle` alone left
// half of the closed-carrier vocabulary a crate away.
pub use ::profile::{
    ArcCenterTarget, ArcTarget, ArcViaTarget, ClosedLoop, LineTarget, Open, PartialPath, PathError,
    Start, TangentArcTarget, circle, circle_split,
};

// --- 3. The four body operations ------------------------------
pub use sweep::fillet::{FilletError, Filleted, fillet_edges};
pub use sweep::{
    ExtrudeError, Extruded, Extrusion, LoftError, Lofted, Revolution, RevolveAxis, RevolveError,
    Revolved, TubeError, TubeWindow, extrude, loft_body, revolve, sweep_body, tube_along_arc,
};

// --- 4. Bodies and Booleans -----------------------------------
// `geom_brep::SurfaceKind` rides here on purpose: it is the payload
// of `BooleanError::CurvedBooleanUnsupported`, so any code that
// matches on a curved-Boolean refusal needs it in the same breath as
// the error itself (see `crate::closure`).
pub use geom_brep::SurfaceKind;
// `PlaneRelation` rides here since LIB-SEL2: it is the verdict a
// `FlushFinding`'s evidence carries (SameOpposite = resting contact,
// SameOriented = flush walls), so code inspecting findings names it.
pub use topo::{
    Body, BooleanBody, BooleanDeclarations, BooleanError, BooleanOp, BooleanResult,
    BooleanResultKind, ContactRecords, Curve3, EdgeGeometry, EdgeKey, FaceKey, Operand,
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
pub use stl::{write_ascii, write_binary};

// --- 8. The document layer ------------------------------------
// `parse_expr` is the expression TEXT door (LIB-U8a): the checking
// parser whose every reduction runs the Expr smart constructors.
// The v4 program vocabulary (LIB-SWITCH): the profile payload is the
// Expr-bearing `ProfileProgram`, curated through the ONE document
// surface (LB13's `crate::document`). `Datum` and `ParamEnv` ride
// here since LIB-SEL1: a datum node is the frame a
// `GeomPred::DatumDistance` selection is written against (GS-Q6), and
// `select_where` takes a `ParamEnv`, so both are needed to write a
// position filter at all.
// `ParamName` and `DocParam` ride here since R1-PARAMS: they are what
// `DocEdit::SetDocParam` and `Expr::param` take, so a prelude user
// could previously hold the param-editing doors and not open them —
// the parametric flagship (`plate_param`, guide §3.2) imports both.
pub use crate::document::{
    CancelToken, Datum, Dimension, Doc, DocEdit, DocParam, EditError, EvalOptions, Evaluation,
    Expr, LoopProgram, Node, NodeError, ParamEnv, ParamName, ParseError, PatternKind,
    ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId, RecordedProgramError, SlotId,
    StepArg, ValuePayload, apply, evaluate, parse_expr,
};
pub use editor_core::StableName;

// --- 9. Names: obtain them, inspect them, select them ---------
// LIB-U7. `StableName` was in group 8 from the start, with no door to
// obtain or read a value of it: the naming table, the whole-body
// materializers and the key→name inversions all stayed one crate
// away, so a prelude user could hold the type and do nothing with it.
// These are that door, curated as one group in `crate::select` (whose
// module docs carry the worked examples).
// The LIB-SEL2 detect/declare protocol rides in this group too: the
// findings vocabulary, the detector, and the declare sugar (the
// worked example is in `crate::select`'s module docs).
pub use crate::select::{
    ALL_SURFACE_KINDS, CONTACT_RECOURSE, CapEnd, Cmp, ContactClass, ContactRefusal, ContactVerdict,
    CurveKind, CurveKindSet, DeclareError, DeclaredContact, Denotation, EntityKind, FIT_DEFERRAL,
    FlushEvidence, FlushFinding, FlushRung, GeomPred, InterrogateError, MeridianEnd, NamePat,
    NameTable, OpGroup, Pose, ProfileEdgeRef, ProfileVertexRef, ReadbackError, RimSupport,
    RolePath, RoleSeg, SEL_DATUM_DISTANCE, SegPat, SegTag, SelectRefusal, Selector, Side,
    SplitHalf, SurfaceKindSet, TagPat, all_bodies, all_edges, all_faces, all_vertices, declare,
    declare_all, declare_node, denotation, edge_frame, edge_name, face_frame, face_name,
    find_flush_candidates, select, select_where, vertex_position,
};
