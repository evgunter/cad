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
//!    / conventional mapped-curve for edges inside one surface, the
//!    boundary iso-curve rung for NURBS wall seams (M7-3). Pcurve
//!    caches re-mint through [`topo::mint_pcurves`] (every curved
//!    chart that mints natively mints here — cylinder, cone, sphere,
//!    torus, described non-rational NURBS; plane faces stay
//!    derive-on-demand, exactly what a natively built body carries).
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
//! unsupported entity types, units outside the subset, and geometry
//! the adoption ladder cannot explain. No panics; no silent guesses;
//! no lenient re-interpretation.
//!
//! # NURBS faces (M7-3)
//!
//! Both `B_SPLINE_SURFACE_WITH_KNOTS` arms import — the non-rational
//! simple record and the `RATIONAL_B_SPLINE_SURFACE` complex
//! instance — retiring the old named M7 frontier by the S9 flip its
//! refusal text predicted. Wall–wall seams adopt through the
//! **IsoCurve rung** (the carrier bitwise-matched against the
//! adjacent walls' `boundary_iso_u` columns, certified through the
//! iso residual lane), cap rims through the conventional rung's
//! Nurbs-adjacency exemption (`PlacedSegment` for line rims — the
//! native loft's own description class, which is what the NURBS
//! chart's pcurve mint accepts; `RevolvedPoint` for arc rims on
//! rational walls, which mint nothing — the native rational body's
//! own state). Multi-ring NURBS faces keep refusing typed at the
//! curved-face ring gate (no NURBS chart inversion — stage-1
//! recognition territory, named there), as do spline sub-types the
//! writer never emits (knots-implied `QUASI_UNIFORM_CURVE` and kin).
//!
//! **Coverage is bounded by what the writer can emit** (#207): the
//! loft/sweep skin's chord-length fit drifts unit weights on any
//! curved-path sweep or non-uniformly spaced loft, so those bodies
//! refuse at BUILD time and no file of them exists to import. The
//! round-trippable class today is uniformly-spaced lofts: polyline
//! profiles (non-rational, full tier 3) and arc-bearing profiles
//! (rational walls — the typed tier-3 limitation below).
//!
//! # The wild (M7-4; `docs/M7-4-SPEC.md`)
//!
//! The subset above is what the writer emits. What translators emit
//! is a *dialect* of it, and reading files nobody here authored is
//! the only way to learn which differences are real. `tests/wild/`
//! holds thirteen license-verified foreign files (see the crate's
//! `NOTICE`); five widenings came out of them, none of which relaxes
//! what the reader is willing to BELIEVE:
//!
//! - **Lexical** ([`parse`]): a string literal folded across a raw
//!   newline at column 72 splices back into one word; CRLF; comments
//!   inside entity records. `\X2\` control directives stay refused.
//! - **Units** ([`units`], [`entities`]): `CONVERSION_BASED_UNIT`
//!   resolves through the conversion expression THE FILE states —
//!   inch and degree both, never a table of what an inch is — and the
//!   units that govern are the ones the geometry's own context names,
//!   so a mass-property unit cluster and Open CASCADE's dimensionless
//!   per-pcurve `GEOMETRIC_REPRESENTATION_CONTEXT(2)` are simply not
//!   consulted. A file's angle scale reaches the one angle in the
//!   subset, a cone's `semi_angle`.
//! - **Directions and vectors**: a `VECTOR` may have any positive
//!   magnitude and a `DIRECTION` need not be normalized (both are
//!   ratios per ISO 10303-42); an `AXIS2_PLACEMENT_3D` may leave its
//!   axis or reference direction unset, and the schema's own
//!   `build_axes` defaults are read rather than guessed. Fields that
//!   ARE stated unit and perpendicular are still adopted bit for bit.
//! - **Assemblies**: one rigid `ITEM_DEFINED_TRANSFORMATION` covering
//!   all of a file's content places the body through
//!   [`topo::transform_rigid`]. Per-component placement, and any
//!   transformation stated as an operator (which can mirror or
//!   scale), refuse typed.
//! - **Edge sense**: `EDGE_CURVE` `same_sense` `.F.` composes into the
//!   half-edge direction; no carrier is ever reversed.
//!
//! Two things the wild states that this reader still refuses, both
//! named at the point of refusal: a curved face carrying rings —
//! Open CASCADE's seamless periodic band — because `topo` has no
//! volume construction for one and the body would not be tier-3
//! valid; and edges the D7 ladder cannot certify, which is the same
//! refusal it has always been.

mod adopt;
mod assemble;
mod chart;
mod entities;
mod error;
mod geometry;
mod normalize;
mod parse;
mod units;

pub use error::{AdoptionAttempt, AdoptionCandidate, StepImportError};

use topo::Body;

/// A boundary-graph census: what a region contributes to the body.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FaceCensus {
    /// Faces.
    pub faces: usize,
    /// Edges.
    pub edges: usize,
    /// Vertices.
    pub vertices: usize,
}

/// Which normalization was applied (each one a named, bounded case —
/// never an open licence to re-mint).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NormalizationKind {
    /// A **closed face with no edges**: a whole sphere arrives as one
    /// `ADVANCED_FACE` bounded by a `VERTEX_LOOP` (Open CASCADE drops
    /// the seam and both degenerate pole edges on export). The kernel's
    /// half-edge structure cannot represent an edge-free closed face,
    /// so the locus is adopted whole and the kernel mints its own
    /// canonical splitting of it — the same 2 faces / 2 edges / 2
    /// vertices a natively revolved ball carries.
    EdgeFreeSphere,
    /// A **cone with a degenerate apex**: Open CASCADE never splits a
    /// periodic face, so a full cone's lateral side arrives as one
    /// `ADVANCED_FACE` whose seam generator ends at the apex — a
    /// vertex with a single incident edge, which `topo`'s tier-2
    /// validity calls construction scaffolding, because in a finished
    /// solid that is what it is. Re-minted as the kernel's own two
    /// lateral half-faces, joined by a second generator half a turn
    /// round the cone's axis.
    DegenerateApexCone,
    /// A **whole torus in one face**: the file's single face wraps the
    /// full period in BOTH chart directions (the fundamental-polygon
    /// square, two curves each used twice). The topology closes, but
    /// the face is not a chart iso-rectangle and its closed-form
    /// divergence contribution comes back with the wrong sign.
    /// Re-minted as the kernel's own two half-faces — but only after
    /// the face's **winding** is read out of its loop's cyclic order
    /// (the fundamental polygon's flag multiset is reversal-invariant,
    /// so the order is the only place the winding lives) and checked
    /// against its `same_sense`. A torus whose two disagree describes
    /// an inside-out ring and REFUSES typed: re-tessellating it
    /// right-side-out would launder the inversion, and import returns
    /// certified bodies — the kernel's tier-3 curved sense gate
    /// (check 6, M6-6) refuses the inside-out face adoption would
    /// build, so the refusal fires pre-body instead.
    FullPeriodTorus,
}

/// A **reported structure normalization** (D7 stage-3 repair, in its
/// letter): the file's locus is fully explained and adopted, but its
/// boundary-graph tessellation is not representable, so the kernel
/// re-minted the tessellation and says so — as data, never silently.
///
/// Volume and validity are exact as always; what changed is only how
/// the same surface is cut into faces, edges, and vertices. The census
/// pair is the mapping a reader needs to reconcile the file's counts
/// with the imported body's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StructureNormalization {
    /// The `ADVANCED_FACE` entity instance the file states.
    pub face: u64,
    /// What was re-minted.
    pub kind: NormalizationKind,
    /// The census the file states for that region.
    pub file_census: FaceCensus,
    /// The census the kernel minted in its place.
    pub kernel_census: FaceCensus,
}

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
        /// tier-valid at rest, **to the tier its native twin
        /// certifies to** (M7-3, the rational arm's honest
        /// conditioning): a body whose native construction is tier-3
        /// valid imports tier-3 valid; a body whose native twin
        /// refuses tier 3 typed — a rational-walled loft, whose
        /// volume quadrature names the banked rational lane —
        /// imports with tiers 1/2 valid and the SAME typed tier-3
        /// refusal (pinned by test). Nothing imports into a state
        /// its native twin does not occupy.
        body: Body<f64>,
        /// The import's input tolerance ε_in (meters): the override if
        /// given, else the file's declared uncertainty.
        eps_in: f64,
        /// The structure normalizations the adoption applied, in
        /// resolution order — empty for a file whose boundary graph the
        /// kernel represents as stated (every own-corpus file).
        normalizations: Vec<StructureNormalization>,
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

    /// The structure normalizations this import applied (empty for a
    /// wireframe, which has no boundary graph to re-mint).
    pub fn normalizations(&self) -> &[StructureNormalization] {
        match self {
            Self::Solid { normalizations, .. } => normalizations,
            Self::Wireframe { .. } => &[],
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
            // The assembly's placement, through the kernel's own door
            // (M7-4 Leg D): `transform_rigid` re-checks rigidity with
            // decided predicates and re-certifies every carrier
            // against the mapped geometry, so a placed body is as
            // first-class as an unplaced one — and a map this reader
            // let through that the kernel will not becomes a typed
            // refusal, never a silently skewed body.
            let body = match model.placement {
                None => body,
                Some(map) => topo::transform_rigid(&body, &map)
                    .map_err(|source| StepImportError::Placement { source })?,
            };
            Ok(StepImport::Solid {
                body,
                eps_in,
                normalizations: model.normalizations.clone(),
            })
        }
        entities::Shape::Wireframe(ref curves) => Ok(StepImport::Wireframe {
            curves: curves.clone(),
            eps_in,
        }),
    }
}
