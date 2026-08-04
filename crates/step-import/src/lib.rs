//! STEP (ISO 10303-21 / AP214) import of the analytic subset the
//! kernel exports — D7's "import is adoption, not admission" made
//! executable for the first slice: the corpus `step-export` writes
//! (M7-1; `docs/M7-1-SPEC.md`).
//!
//! # What this is
//!
//! A reader for exactly the exchange structure the in-house writer
//! emits: HEADER + DATA Part 21 sections, simple and complex entity
//! instances, the AP214 advanced-B-rep product structure
//! (`ADVANCED_BREP_SHAPE_REPRESENTATION` over `MANIFOLD_SOLID_BREP`s)
//! and the curve-set wireframe form
//! (`GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION`). The
//! result of a solid import is a first-class [`topo::Body`]: topology
//! rebuilt through the public Euler-operator vocabulary, geometry
//! adopted per D7 (intensional descriptions reconstructed and
//! certified by the kernel's own gates — never trusted extensional
//! data), caches re-minted by the kernel's own machinery.
//!
//! # Adoption, not admission (D7, the shape of the pipeline)
//!
//! 1. **Substrate** ([`parse`]): Part 21 tokenization and entity
//!    resolution. Reals parse with `str::parse::<f64>` — the writer's
//!    printer round-trips to identical bits, so downstream exact `==`
//!    comparisons are legitimate on own-corpus files.
//! 2. **Entity mapping** ([`entities`], [`geometry`]): each AP214
//!    record maps to its kernel twin **field for field** (the export
//!    direction is an identity, so import is too — no re-derivation,
//!    no normalization that moves bits). The one non-identity is the
//!    cone: STEP's `v` is axial, the kernel's is slant arc length; the
//!    writer's apex placement (`radius = 0.0`) is recognized and the
//!    surface fields adopted verbatim (no trim parameters cross the
//!    wire in this subset, so the fixed cos α factor has nothing to
//!    act on yet).
//! 3. **Topology assembly** ([`assemble`]): the shell's half-edge
//!    structure is realized through `topo`'s public Euler operators
//!    (`mvfs`/`mev`/`mef`, the ring vocabulary, scaffold struts killed
//!    by their inverse ops) — the sanctioned construction path, D1.
//! 4. **Adoption** ([`adopt`]): surfaces attach (bitwise-identical
//!    records share one kernel surface key, restoring the writer-side
//!    key sharing), `same_sense` lands as [`topo::Face::sense`]
//!    honored, not healed, and every edge's intensional description is
//!    **rebuilt and certified** by the kernel's own gates: intersection
//!    / tangent-intersection for edges between distinct surfaces, seam
//!    / conventional mapped-curve for edges inside one surface. Pcurve
//!    caches re-mint through [`topo::mint_pcurves`] (cylinder charts
//!    mint; other charts are derive-on-demand, exactly what a natively
//!    built body carries).
//!
//! # Two tolerances (D7)
//!
//! ε_in — the per-import *input* tolerance — defaults from the file's
//! `UNCERTAINTY_MEASURE_WITH_UNIT` and is overridable per call
//! ([`ImportOptions::eps_in`]); it is carried on the result
//! ([`StepImport::eps_in`]). This unit *records* it: own-corpus files
//! declare the kernel's own ε and adopt exactly, so the healing ladder
//! that consumes ε_in is M7-2+. Certification of everything built
//! runs at the kernel's ambient ε like native geometry.
//!
//! # Fail loud (D4 ¶5)
//!
//! Every refusal is a typed [`StepImportError`] naming the offending
//! entity id / line: malformed syntax, dangling references,
//! unsupported entity types (`B_SPLINE_SURFACE_WITH_KNOTS` is the
//! named M7 frontier), units outside the subset, and geometry the
//! adoption ladder cannot explain. No panics; no silent guesses; no
//! lenient re-interpretation.

mod adopt;
mod assemble;
mod entities;
mod error;
mod geometry;
mod parse;

pub use error::{AdoptionAttempt, AdoptionCandidate, StepImportError};

use topo::Body;

/// Per-call import options (D7's ε_in override door).
#[derive(Clone, Debug, Default)]
pub struct ImportOptions {
    /// Overrides the file's declared `UNCERTAINTY_MEASURE_WITH_UNIT`
    /// as the import's input tolerance ε_in. `None` (default) reads
    /// the file's declaration. Must be finite and strictly positive
    /// when given ([`StepImportError::InvalidEpsOverride`]).
    pub eps_in: Option<f64>,
}

/// A successful import: what the file's shape representation supports.
///
/// The variant sizes differ by design (a whole body vs a curve list);
/// boxing the body would put an indirection in every consumer of the
/// common case for no benefit at an import-sized allocation rate.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum StepImport {
    /// The file carried `MANIFOLD_SOLID_BREP`s: a first-class kernel
    /// body, one kernel solid (with one shell) per `MANIFOLD_SOLID_BREP`
    /// — the writer emits one MSB per shell, so solid grouping is not
    /// recoverable from the file and each MSB adopts as its own solid
    /// (matching what independent readers report for e.g. the
    /// kiss assembly).
    Solid {
        /// The adopted body — first-class: Euler-built, certified,
        /// tier-valid at rest.
        body: Body<f64>,
        /// The import's input tolerance ε_in (meters): the override if
        /// given, else the file's declared uncertainty.
        eps_in: f64,
    },
    /// The file carried a `GEOMETRIC_CURVE_SET` wireframe and no
    /// solid: the reconstructed carriers, exact. **No body is
    /// claimed** — a wireframe has no faces, shells, or solids to
    /// adopt, so there is nothing a `Body` could honestly represent; a
    /// disposition, not a skip.
    Wireframe {
        /// The curve-set's carriers in file order, exact
        /// (control points / knots / weights bitwise).
        curves: Vec<geom_curves::Curve3<f64>>,
        /// As [`StepImport::Solid::eps_in`].
        eps_in: f64,
    },
}

impl StepImport {
    /// The import's input tolerance ε_in (meters) — D7's separate
    /// interpretation tolerance, carried on every result.
    pub fn eps_in(&self) -> f64 {
        match self {
            Self::Solid { eps_in, .. } | Self::Wireframe { eps_in, .. } => *eps_in,
        }
    }
}

/// Imports a Part 21 exchange file (crate docs for subset and
/// pipeline).
///
/// # Errors
///
/// [`StepImportError`] — malformed syntax, dangling references,
/// entities outside the exported subset, units the subset does not
/// cover, topology that does not assemble, or geometry the D7
/// adoption ladder cannot certify. Files written by
/// `step_export::step_string` from finished kernel bodies import
/// cleanly.
pub fn import_step(text: &str, options: &ImportOptions) -> Result<StepImport, StepImportError> {
    if let Some(eps) = options.eps_in
        && !(eps.is_finite() && eps > 0.0)
    {
        return Err(StepImportError::InvalidEpsOverride { value: eps });
    }
    let file = parse::parse_file(text)?;
    let model = entities::resolve(&file)?;
    let eps_in = options.eps_in.unwrap_or(model.uncertainty_m);
    match model.shape {
        entities::Shape::Solids(ref solids) => {
            let body = assemble::build_body(solids, &model)?;
            Ok(StepImport::Solid { body, eps_in })
        }
        entities::Shape::Wireframe(ref curves) => Ok(StepImport::Wireframe {
            curves: curves.clone(),
            eps_in,
        }),
    }
}
