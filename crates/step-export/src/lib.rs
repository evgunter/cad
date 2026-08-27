//! STEP (ISO 10303-21 / AP214) export of finished kernel bodies — the
//! in-house analytic-subset writer (M4 PR 7, discharging the F6
//! decision, 2026-07-23: in-house subset writer, adopt nothing at
//! runtime — ruststep and truck-stepio stay dev-only parse-back
//! oracles).
//!
//! # What this is, and what it is not
//!
//! An EXPORT writer only: a finished [`topo::Body`] (certified
//! analytic geometry — planar since M4, curved since M5 PR 13 —
//! tier-2-valid at rest) is serialized to a Part 21 exchange
//! file whose data section is the AP214 advanced-B-rep product
//! structure. Import is M7 and deliberately absent. The F6 spike
//! established that no adoptable crate writes conformant STEP
//! (ruststep has no writer at all; truck-stepio's writer ships
//! preamble/entity defects unfixable through its API), so this writer
//! is in-house and the external crates appear as **dev-dependency
//! parse-back oracles in the tests only**.
//!
//! # Conformance decisions (the truck-stepio defect list, in reverse)
//!
//! - **Schema**: `AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }` (AP214
//!   ed. 2), and the data section's `APPLICATION_PROTOCOL_DEFINITION`
//!   says `'automotive_design', 2000` — the FILE_SCHEMA names the AP
//!   the data section actually uses (truck's names a resource schema
//!   over an AP214 body — the disqualifying defect class). AP214 over
//!   AP203 because AP203 ed. 1 conformance drags the config-control
//!   person/organization/approval apparatus into every file; AP214's
//!   product skeleton is the minimal one every importer accepts, and
//!   the spike's AP214 prototype was reconstructed intact by an
//!   independent importer and by FreeCAD/OCC.
//! - **Units**: `SI_UNIT($, .METRE.)` — unprefixed metres, because
//!   kernel-internal geometry IS metres by D6/D4 ¶4. No hardcoded
//!   millimetre lie (truck defect 3).
//! - **Uncertainty**: `UNCERTAINTY_MEASURE_WITH_UNIT` wraps the value
//!   in the `LENGTH_MEASURE(...)` select (truck emits the bare real —
//!   a Part 21 typing error), and the value is the run's ambient
//!   tolerance ε (D4 ¶1) unless overridden in [`StepOptions`].
//! - **Topology**: `ADVANCED_FACE` under
//!   `ADVANCED_BREP_SHAPE_REPRESENTATION` with `FACE_OUTER_BOUND` for
//!   outer loops (truck emits `FACE_SURFACE`/`FACE_BOUND`, violating
//!   the ABSR WHERE rules). Rings emit as `FACE_BOUND`.
//! - **Product structure**: real `PRODUCT` chain with a caller-visible
//!   product name ([`StepOptions::product_name`]), not a hardcoded
//!   empty string.
//!
//! # The analytic subset (M5 PR 13: the curved subset)
//!
//! The writer's printers are the two closed matches in `writer.rs`,
//! one per kernel geometry enum. **Every arm is an exact native AP214
//! entity** — the whole point of writing this in-house was to export
//! analytic geometry *as* analytic geometry, never as a B-spline
//! approximation, which is the carrier story truck-stepio could not
//! serve:
//!
//! | kernel `Surface` | AP214 entity | exact? |
//! |---|---|---|
//! | `Plane` | `PLANE` | yes, identity |
//! | `Cylinder` | `CYLINDRICAL_SURFACE` | yes, identity |
//! | `Cone` | `CONICAL_SURFACE` (apex placement, `radius = 0`) | yes as a LOCUS; `v` differs by the fixed factor cos α (STEP's `v` is axial, the kernel's is slant arc length) — invisible without pcurves |
//! | `Sphere` | `SPHERICAL_SURFACE` | yes, identity |
//! | `Torus` | `TOROIDAL_SURFACE` | yes, identity |
//! | `Nurbs` | — | refuses (see below) |
//!
//! | kernel `Curve3` | AP214 entity | exact? |
//! |---|---|---|
//! | `Line` | `LINE` (unit `VECTOR`) | yes, identity |
//! | `Circle` | `CIRCLE` | yes, identity |
//! | `Ellipse` | `ELLIPSE` | yes, identity |
//! | `Nurbs` | `B_SPLINE_CURVE_WITH_KNOTS`, or the `RATIONAL_B_SPLINE_CURVE` complex instance when any weight ≠ 1 | yes, structure for structure |
//!
//! "Identity" is meant literally: each kernel frame `(origin, axis,
//! u_ref)` is ISO 10303-42's `axis2_placement_3d` field for field, and
//! each variant's evaluation formula is the schema's formula for the
//! matching entity, so the two parameterizations agree — not merely the
//! point sets. Nothing is renormalized, reordered, or offset.
//!
//! **Conics do not take the rational-quadratic road.** The NURBS Book
//! §7.3–7.4 form (shape factor `k = w₀w₂/w₁²`, arcs ≥ 180° needing
//! infinite control points) is representationally exact and is the
//! kernel's declared export/tessellation form for conics — but AP214
//! *has* `CIRCLE` and `ELLIPSE`, so using it here would be an equally
//! exact, strictly worse encoding: it discards the axes and centre
//! every reader consumes, reparameterizes the curve for nothing, and
//! trades a five-float record for a control/weight/knot triple. The
//! native entity wins wherever it exists, which for the kernel's conic
//! rungs is everywhere.
//!
//! **What still refuses**, typed and named, never silently degraded:
//! the mvfs "no description yet" NURBS placeholder
//! ([`StepExportError::UnsupportedSurface`]) — a mid-surgery fact,
//! never exportable. A DESCRIBED NURBS surface exports natively as
//! `B_SPLINE_SURFACE_WITH_KNOTS` since M6-3 (the loft walls; the
//! rational complex instance for weighted nets), and a described
//! NURBS carrier as `B_SPLINE_CURVE_WITH_KNOTS` (the loft seams).
//!
//! # Export only — the round trip is not closed
//!
//! Said plainly, because the curved subset makes the asymmetry
//! bigger: this crate WRITES STEP and cannot READ it. There is no
//! importer in the kernel at all; STEP import is M7. A file written
//! here is consumed by other CAD systems (the FreeCAD/OCC acceptance
//! below), never by this kernel — so "round trip" in these docs always
//! means *someone else's* reader reconstructing our geometry, and a
//! shape that leaves through this writer does not come back.
//!
//! # Orientation mapping (cites the ratified conventions, adds none)
//!
//! - A face's outward normal is `topo::Face::sense_sign()` times its
//!   stored surface normal (M5 S10; before S10 the stored normal
//!   simply WAS the outward normal — the M1 interior-left
//!   ratification, restated in `geom_brep::enters`). STEP has a field
//!   that means exactly this, so the mapping is an identity and not a
//!   conversion: `ADVANCED_FACE`'s `same_sense` is `Face::sense`,
//!   emitted `.T.`/`.F.`. The surface's `AXIS2_PLACEMENT_3D` axis
//!   stays the stored **chart** normal (with `ref_direction` the
//!   stored `u_ref`, ⊥ normal by the same convention) — correct
//!   precisely *because* `same_sense` carries the flip: the reversal
//!   is stated once, in the field the schema provides for it, and the
//!   exported surface stays the true surface. Since M5 S11 the sweep
//!   constructors mint `sense: false` on walls whose material lies
//!   against the chart normal — concave extrude walls, revolve's
//!   bore/cone/sphere/torus bands, and the under-side annulus that
//!   rides with them. Every such wall is curved or rides on a curved
//!   body, so before M5 PR 13 no `.F.` could reach the emitter; with
//!   the curved arms landed it does, and the planar goldens are
//!   nevertheless byte-unchanged (no planar-only body at rest mints a
//!   reversed face).
//! - An edge's certified carrier parameter runs `start(he_plus) →
//!   end(he_plus)` (the M2 forward contract on `Edge::he_plus`), so
//!   every `EDGE_CURVE` has `same_sense = .T.` with its vertices taken
//!   from `he_plus`; an `ORIENTED_EDGE` is `.T.` exactly when the
//!   loop's half-edge is the edge's plus half.
//! - Outer loops as stored run counterclockwise about the outward
//!   normal and rings clockwise (interior-left), which is precisely
//!   STEP's convention for `FACE_OUTER_BOUND`/`FACE_BOUND` with
//!   orientation `.T.` — the stored direction is emitted unchanged.
//!   This holds for either face sense and the flag is never flipped:
//!   the schema *composes* a bound's orientation with the owning
//!   face's `same_sense`, so with the sense already emitted above, a
//!   second flip here would double-count it. Shell volumes are read
//!   from these same windings and are sense-invariant by derivation
//!   for the same reason (`volume.rs` module docs).
//!
//! # Solids, shells, and voids
//!
//! Each shell exports as one `CLOSED_SHELL` under its own
//! `MANIFOLD_SOLID_BREP`. For a **multi-shell solid** (an M3 boolean
//! Assembly/Voided result) the writer classifies each shell by the
//! sign of its divergence-theorem volume over the planar subset,
//! computed in f64 (closed forms per segment; rounded accumulation in
//! general, exact on dyadic inputs — the sign read is a plain
//! comparison, not a Q1 trilean, and `volume.rs`'s module docs carry
//! the headroom argument for why that is safe on any buildable shell).
//! Outward-normal convention: a positive shell bounds material from
//! outside, a negative shell is a void cavity wall. Positive shells
//! become independent `MANIFOLD_SOLID_BREP`s (the disjoint-assembly
//! case); a negative shell is **refused** with
//! [`StepExportError::VoidShellUnsupported`] — `BREP_WITH_VOIDS`
//! needs a void-to-containing-shell association the kernel does not
//! yet record, and guessing it would be a silent lie. Voided bodies
//! wait for that designation (M5+); the refusal is pinned by test.
//! Single-shell solids are deliberately never volume-checked: tier-2
//! validity plus the +V invariant own a lone shell's orientation, and
//! the classification gate exists only to tell a multi-shell solid's
//! shells apart.
//!
//! The classifier's closed forms are **planar only** and did not grow
//! at M5 PR 13, so it is now narrower than the emitter: a MULTI-shell
//! solid carrying curved geometry refuses
//! ([`StepExportError::CurvedShellClassification`]) even though every
//! one of its faces has a printer. The reduction the classifier
//! performs is a planarity identity with no closed-form curved
//! counterpart, and its output is a material-vs-void sign — the one
//! place an approximation would be a silent lie rather than a
//! roundoff. Single-shell curved solids — every curved body at rest
//! today bar S12's two-stub `boss ∖ plate` complement — never reach it
//! and export normally.
//!
//! # Determinism (D9)
//!
//! Same body value + same options ⇒ **byte-identical** file. Entity
//! ids are allocated in a single fixed traversal (solids → shells →
//! faces → outer-then-rings → loop cycle order; vertex/edge entities
//! minted at first encounter), hash maps are used for lookup only —
//! no hash iteration anywhere on the write path — and every real is
//! printed by the shortest round-trip formatter (`real.rs`): the
//! printed token parses back to the identical f64 bits. There is no
//! wall-clock anywhere: the header timestamp is
//! [`StepOptions::timestamp`], a fixed string.
//!
//! # Fail loud (D4/D9)
//!
//! Every unrepresentable input is a typed [`StepExportError`]: no
//! panics, no lossy fallbacks, no NaN smuggling (`NaN`/`±∞` refuse at
//! the float printer with the offending context named). Mid-surgery
//! bodies (null scaffolding, empty loops) refuse rather than emit
//! half-topology.
//!
//! # External acceptance
//!
//! The parse-back oracles prove syntax and reconstructed topology; the
//! acceptance gate for "real CAD systems read our files" is external
//! (the admesh pattern, STEP-shaped): `scripts/check_step.sh` runs a
//! headless FreeCAD/OCC import over the committed fixtures in
//! `tests/fixtures/` and asserts validity, entity counts, and volume.

use std::collections::HashMap;
use std::fmt;

use geom_core::Tol;
use topo::{Body, EdgeKey, FaceKey, LoopKey, ShellKey};

mod real;
mod volume;
mod writer;

pub use real::fmt_real;

/// Typed export failure (closed enum, D4 ¶3). Every variant names the
/// offending entity or value — the writer never panics and never
/// silently degrades output.
#[derive(Debug)]
pub enum StepExportError {
    /// A real value on the write path (coordinate, direction component,
    /// uncertainty) is NaN or ±∞ — unrepresentable as a Part 21 real.
    NonFiniteReal {
        /// The offending value.
        value: f64,
        /// Which quantity was being printed (static description).
        context: &'static str,
    },
    /// A header/product string contains characters outside Part 21's
    /// basic alphabet (0x20..=0x7E) — the writer refuses rather than
    /// emit an encoding it cannot promise importers read back.
    UnrepresentableString {
        /// Which string field was being quoted (static description).
        context: &'static str,
    },
    /// A face's surface has no printer. Since M5 PR 13 every
    /// ELEMENTARY_SURFACE prints (plane, cylinder, cone, sphere,
    /// torus) and since M6-3 described NURBS surfaces print as
    /// `B_SPLINE_SURFACE_WITH_KNOTS`, so the one live case is the
    /// mvfs "no description yet" NURBS placeholder — joined by the
    /// approximating surface, which refuses rather than print its fit
    /// as though it were the described geometry (the file has no place
    /// to carry the certificate that makes the fit honest). Typed
    /// refusal, never a B-spline approximation of an analytic surface.
    UnsupportedSurface {
        /// The face whose surface is out of subset.
        face: FaceKey,
        /// The surface variant's name.
        kind: &'static str,
    },
    /// An edge's certified carrier has no printer. Since M5 PR 13
    /// every `Curve3` kind prints (line, circle, ellipse, NURBS), so
    /// the one live case is the "no description yet" NURBS carrier
    /// placeholder — the curve-side twin of the surface placeholder,
    /// a mid-surgery fact rather than a subset boundary.
    UnsupportedCurve {
        /// The edge whose carrier is out of subset.
        edge: EdgeKey,
        /// The carrier variant's name.
        kind: &'static str,
    },
    /// An edge is M3 null-edge scaffolding (no carrier by type): the
    /// body is mid-surgery — tier 2 refuses such bodies at rest, and
    /// so does export.
    NullScaffoldEdge {
        /// The scaffolding edge.
        edge: EdgeKey,
    },
    /// A loop is the empty-loop degenerate state (a lone vertex, no
    /// edges) — legal only mid-construction; a finished solid's face
    /// bounds never contain one, and STEP's advanced B-rep has no
    /// place for it.
    EmptyLoop {
        /// The empty loop.
        loop_: LoopKey,
    },
    /// The body contains no exportable shell (no solids, or solids
    /// with no shells) — an empty DATA section would be a conformance
    /// defect, so the writer refuses.
    NothingToExport,
    /// A multi-shell solid contains a shell whose signed volume is
    /// definitely negative — a void cavity wall (outward normals point
    /// into the cavity). `BREP_WITH_VOIDS` needs the kernel's
    /// void-to-outer-shell designation, which does not exist yet
    /// (M5+); exporting the void as a solid would be silently wrong,
    /// so the writer refuses.
    VoidShellUnsupported {
        /// The void shell.
        shell: ShellKey,
        /// Its computed signed volume (m³, negative).
        volume: f64,
    },
    /// A multi-shell solid contains a shell whose signed volume is
    /// zero or non-finite — the outward/void classification has no
    /// honest answer.
    ShellVolumeIndeterminate {
        /// The unclassifiable shell.
        shell: ShellKey,
        /// The computed signed volume (m³).
        volume: f64,
    },
    /// A **multi-shell** solid contains a shell whose geometry leaves
    /// the outward/void classifier's closed forms (planes bounded by
    /// lines). The emitter itself handles the curved subset — this is
    /// the classifier alone: the divergence-theorem reduction in
    /// `volume.rs` is exact per planar face and has no curved-face
    /// counterpart yet, and guessing a shell's sign would silently
    /// decide material-vs-void. Single-shell solids never reach it, so
    /// curved bodies export normally; only a curved solid with two or
    /// more shells refuses.
    CurvedShellClassification {
        /// The shell that could not be classified.
        shell: ShellKey,
        /// The face whose geometry left the closed forms.
        face: FaceKey,
        /// What that face carries (surface or carrier variant name).
        kind: &'static str,
    },
    /// An explicit [`StepOptions::uncertainty_m`] is not finite and
    /// strictly positive.
    InvalidUncertainty {
        /// The rejected value.
        value: f64,
    },
    /// A key held by the body fails to resolve, or the half-edge
    /// structure is incoherent — corrupt input, surfaced rather than
    /// trusted (the structural validators own the diagnosis; this is
    /// the fail-loud surface).
    Corrupt {
        /// What failed to resolve (static description).
        what: &'static str,
    },
    /// An I/O failure from the output sink.
    Io(std::io::Error),
}

impl fmt::Display for StepExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteReal { value, context } => write!(
                f,
                "step export: {context} is {value}, which has no Part 21 real representation"
            ),
            Self::UnrepresentableString { context } => write!(
                f,
                "step export: {context} contains characters outside Part 21's basic \
                 alphabet (0x20..=0x7E)"
            ),
            Self::UnsupportedSurface { face, kind } => write!(
                f,
                "step export: face {face:?}'s surface ({kind}) has no printer in the \
                 analytic subset (every elementary surface prints; NURBS faces do not)"
            ),
            Self::UnsupportedCurve { edge, kind } => write!(
                f,
                "step export: edge {edge:?}'s carrier ({kind}) has no printer in the \
                 analytic subset (every described carrier prints; a placeholder does not)"
            ),
            Self::NullScaffoldEdge { edge } => write!(
                f,
                "step export: edge {edge:?} is null-edge scaffolding — the body is \
                 mid-surgery (tier 2 refuses null entities at rest)"
            ),
            Self::EmptyLoop { loop_ } => write!(
                f,
                "step export: loop {loop_:?} is an empty loop (lone vertex) — not part \
                 of any finished solid's boundary"
            ),
            Self::NothingToExport => {
                f.write_str("step export: the body contains no exportable shell")
            }
            Self::VoidShellUnsupported { shell, volume } => write!(
                f,
                "step export: shell {shell:?} of a multi-shell solid has negative signed \
                 volume {volume} m\u{b3} — a void cavity; BREP_WITH_VOIDS export needs the \
                 kernel's void-shell designation (future work), refusing rather than \
                 exporting a void as material"
            ),
            Self::ShellVolumeIndeterminate { shell, volume } => write!(
                f,
                "step export: shell {shell:?} of a multi-shell solid has signed volume \
                 {volume} m\u{b3} — outward/void classification is indeterminate"
            ),
            Self::CurvedShellClassification { shell, face, kind } => write!(
                f,
                "step export: shell {shell:?} of a multi-shell solid carries face \
                 {face:?} ({kind}) — the outward/void classifier's divergence-theorem \
                 closed forms cover planes bounded by lines only, and a guessed sign \
                 would silently decide material vs void; refusing (the emitter itself \
                 exports this geometry — single-shell curved solids are unaffected)"
            ),
            Self::InvalidUncertainty { value } => write!(
                f,
                "step export: uncertainty override {value} is not finite and strictly \
                 positive"
            ),
            Self::Corrupt { what } => write!(f, "step export: corrupt body ({what})"),
            Self::Io(e) => write!(f, "step export: I/O failure: {e}"),
        }
    }
}

impl std::error::Error for StepExportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

/// Export options: everything that lands in the file besides the body.
///
/// Determinism (D9): the writer is a pure function of `(body, options)`
/// — equal inputs give byte-identical files. In particular the header
/// timestamp is **this fixed string**, never a wall-clock read; callers
/// wanting a real timestamp supply one (ISO 8601, e.g.
/// `2026-07-23T12:00:00`).
#[derive(Clone, Debug, PartialEq)]
pub struct StepOptions {
    /// The product name: `PRODUCT` id/name, the `MANIFOLD_SOLID_BREP`
    /// name, and the header's file name (with `.step` appended).
    /// Part 21 basic alphabet only (apostrophes and backslashes are
    /// escaped for you).
    pub product_name: String,
    /// The header `FILE_NAME` time_stamp field, verbatim. Fixed epoch
    /// placeholder by default (determinism over false freshness).
    pub timestamp: String,
    /// The header author field (empty by default).
    pub author: String,
    /// The header organization field (empty by default).
    pub organization: String,
    /// The header originating_system field (empty by default).
    pub originating_system: String,
    /// The `UNCERTAINTY_MEASURE_WITH_UNIT` length value in meters:
    /// `Some(ε)` explicit (must be finite and > 0), `None` reads the
    /// run's ambient tolerance `geom_core::Tolerance::get().eps` at
    /// write time (D4 ¶1 — the one ε the body was built under).
    pub uncertainty_m: Option<f64>,
}

impl Default for StepOptions {
    fn default() -> Self {
        Self {
            product_name: "part".to_owned(),
            timestamp: "1970-01-01T00:00:00".to_owned(),
            author: String::new(),
            organization: String::new(),
            originating_system: String::new(),
            uncertainty_m: None,
        }
    }
}

/// Serializes `body` as an AP214 Part 21 exchange file (crate docs for
/// the mapping, subset, and conformance decisions) and returns it as a
/// string.
///
/// # Errors
///
/// [`StepExportError`] — out-of-subset geometry, mid-surgery bodies,
/// non-finite values, void shells, corrupt structure. Finished planar
/// bodies from the public construction APIs export cleanly.
pub fn step_string(
    body: &Body<f64>,
    options: &StepOptions,
    tol: Tol,
) -> Result<String, StepExportError> {
    writer::write_document(body, options, tol)
}

/// [`step_string`], written to an [`std::io::Write`] sink.
///
/// # Errors
///
/// As [`step_string`], plus [`StepExportError::Io`] from the sink.
pub fn write_step<W: std::io::Write>(
    body: &Body<f64>,
    options: &StepOptions,
    sink: &mut W,
    tol: Tol,
) -> Result<(), StepExportError> {
    let document = step_string(body, options, tol)?;
    sink.write_all(document.as_bytes())
        .map_err(StepExportError::Io)
}

/// Quotes `s` as a Part 21 string literal: surrounding apostrophes,
/// `'` doubled, `\` doubled; any character outside the basic alphabet
/// (0x20..=0x7E) is a typed refusal.
fn quoted(s: &str, context: &'static str) -> Result<String, StepExportError> {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('\'');
    for ch in s.chars() {
        match ch {
            '\'' => out.push_str("''"),
            '\\' => out.push_str("\\\\"),
            ' '..='~' => out.push(ch),
            _ => return Err(StepExportError::UnrepresentableString { context }),
        }
    }
    out.push('\'');
    Ok(out)
}

/// Lookup-only id caches for entities shared across the walk (vertices
/// and edges are referenced by every adjacent face). Hash maps are
/// never iterated — output order is the traversal's (D9).
#[derive(Default)]
struct SharedIds {
    vertex_points: HashMap<topo::VertexKey, u64>,
    edge_curves: HashMap<EdgeKey, u64>,
}
