//! STEP (ISO 10303-21 / AP214) import of the analytic subset the
//! kernel exports — D7's "import is adoption, not admission" made
//! executable for the first slice: the corpus `step-export` writes
//! (M7-1).
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
//! 5. **The shared at-rest gate** (M7-7, #260 ruling (a);
//!    [`import_step`]): the body is handed to
//!    `topo::validate_geometric` — the kernel's own at-rest validator,
//!    tiers 1–3, the same function on the same body a native
//!    construction's caller runs — and only a body it passes ships as
//!    [`StepImport::Solid`]. Steps 1–4 certify each edge's
//!    description; this certifies the BODY, which is what "import is
//!    adoption" has to mean if it means anything.
//!
//!    Asked once per `MANIFOLD_SOLID_BREP` **on that solid's own
//!    body**, and once on the assembled body. Per solid because
//!    several of the gate's invariants are whole-body sums (check 7's
//!    +V is the boundary flux over every shell), so an inside-out
//!    solid can be cancelled by a right-side-out neighbour and the
//!    aggregate reads Zero, which is exempt — "every imported solid
//!    passes the gate" is only true if each solid is a subject. The
//!    refusal names which one. There is exactly one place in this
//!    crate that calls the validator (`gate`), and it is
//!    unconditional there: no body kind is exempt, no verdict class is
//!    filtered (an escalated verdict refuses like any other —
//!    escalate-never-guess).
//!
//!    This is D9 engineering convention 2 applied to the door #260
//!    found open: import cannot hold an idea of validity that differs
//!    from the kernel's, because it has no validation code of its own
//!    to drift. Files that describe bodies the kernel refuses at rest
//!    refuse at import, typed, naming the failing check and its
//!    entities ([`StepImportError::TierInvalid`]) — a statement about
//!    the FILE's geometry, never the kernel-bug voice.
//!
//!    **Scope, named**: the gate is tier 3, and 3′ on an empty contact
//!    record is strictly stronger (it runs the coincidence census).
//!    Imports declare no contacts, so an imported assembly whose parts
//!    TOUCH is checked less than its native twin, whose pipeline
//!    carries declarations. Import-side declared contacts are banked
//!    with the M8 contact program (D7 step 4).
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
//! instance. Wall–wall seams adopt through the
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
//! round-trippable class today is uniformly-spaced lofts with polyline
//! profiles (non-rational, full tier 3). Arc-bearing profiles export
//! and read, and their rational walls now have a volume quadrature
//! that converges through interior knots — but it is a composite on a
//! fixed round budget, so a large or strongly curved rational wall can
//! still exhaust that budget and the at-rest gate below refuses it,
//! carrying the measured width.
//!
//! # The wild (M7-4)
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
//! What the wild states that this reader still refuses is named at
//! the point of refusal: a curved face carrying a genuine interior
//! ring — `topo` has no volume construction for one, so the body
//! would not be tier-3 valid — and edges the D7 ladder cannot
//! certify, the same refusal it has always been. Open CASCADE's
//! seamless periodic band NORMALIZES since
//! M7-5: cylinder and torus bands take the seam re-mint
//! ([`NormalizationKind::SeamlessPeriodicBand`]); band shapes on
//! other charts keep a typed refusal naming that re-mint as the
//! recourse.

mod adopt;
mod assemble;
mod chart;
#[cfg(test)]
mod cr_r1_probes;
mod entities;
mod error;
mod geometry;
mod normalize;
mod parse;
mod recognize;
mod recognize_curve;
mod signed_zero;
mod units;

pub use error::{AdoptionAttempt, AdoptionCandidate, StepImportError};

use geom_core::Tol;
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

/// **One materialized assembly instance** — the A7 record shape
/// (`docs/ASSEMBLY-DESIGN.md`), kept so a flattened import can later
/// be re-adopted as an assembly document **without re-parsing the
/// STEP file**.
///
/// An AP214 assembly states N occurrences of M component
/// representations; import materializes each occurrence as its own
/// solid (A2 — the multi-solid body IS the evaluation product), and
/// one of these records travels with each. The association the record
/// carries is the one a body graph would rebuild:
/// `component → instances → solid indices`.
///
/// Every field names a real record in the file, or is `None` because
/// the file states no assembly. A file with no assembly vocabulary
/// still gets one record per solid — `component` is the
/// representation the solid resolved under, the placement is the
/// identity, and there is no occurrence to name — so a consumer never
/// has to ask whether the record exists.
#[derive(Clone, Copy, Debug)]
pub struct PlacedInstance {
    /// Index of this instance's solid in the shipped body, in
    /// [`topo::Body::solids`] order — the bridge from this record back
    /// to the geometry it describes.
    pub index: usize,
    /// The `MANIFOLD_SOLID_BREP` this instance is a copy of. Repeats
    /// across the records of one component's several occurrences: that
    /// repetition IS the instancing.
    pub solid: u64,
    /// The shape representation that names `solid` — the component
    /// representation of the assembly, or the representation the solid
    /// resolved under in a file that places nothing.
    pub component: u64,
    /// The `NEXT_ASSEMBLY_USAGE_OCCURRENCE` this instance is, where
    /// the file links one (through
    /// `CONTEXT_DEPENDENT_SHAPE_REPRESENTATION` and
    /// `PRODUCT_DEFINITION_SHAPE`). This is the occurrence identity a
    /// re-adoption would hang an instance node on.
    pub occurrence: Option<u64>,
    /// The `REPRESENTATION_RELATIONSHIP` complex that states this
    /// occurrence's placement.
    pub relationship: Option<u64>,
    /// The `ITEM_DEFINED_TRANSFORMATION` the placement was read from.
    pub transform: Option<u64>,
    /// The rigid map actually applied to this copy, through
    /// [`topo::transform_rigid`]. `None` is the identity — the file
    /// stated a placement that is the identity at ε_in, or stated
    /// none; either way nothing moved, and recording the map as
    /// applied (rather than as stated) is what makes the record
    /// re-adoptable without re-deciding anything.
    pub placement: Option<geom_core::Affine3<f64>>,
}

/// The analytic kinds D7 stage-1 surface recognition can promote a
/// NURBS patch to. Cone, sphere, and torus recognition are banked
/// (unimplemented; such patches stay NURBS).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PromotedKind {
    /// A plane.
    Plane,
    /// A cylinder.
    Cylinder,
}

impl core::fmt::Display for PromotedKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Plane => "plane",
            Self::Cylinder => "cylinder",
        })
    }
}

/// Which normalization was applied (each one a named, bounded case —
/// never an open licence to re-mint).
#[derive(Clone, Copy, Debug, PartialEq)]
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
    /// A **seamless periodic band** (M7-5): a cylinder or torus
    /// lateral face stated as its two full-period rim bounds with NO
    /// seam generator between them (Open CASCADE never splits a
    /// periodic face on export). The kernel's face model has one outer
    /// loop plus rings, and a curved face with a ring has no volume
    /// construction (`RingOnCurvedFace`), so the band cannot adopt as
    /// stated. Re-minted as the kernel's own shape for the same locus:
    /// ONE single-loop face whose loop walks one rim, the minted seam
    /// generator (the surface's u_ref ruling for a cylinder, its u_ref
    /// meridian arc for a torus), the other rim, and the generator
    /// again reversed — the seam edge used twice, exactly what a
    /// natively revolved wall carries. Where a rim has no vertex at
    /// the u_ref azimuth it is split there first, and the split
    /// propagates to every face sharing that rim. The face's winding
    /// is DERIVED (each rim's chart-u direction against `same_sense`);
    /// an orientation-inverted cylinder band refuses typed pre-body,
    /// and a torus band's winding × sense pair selects which of the
    /// two v-intervals between its rims the face covers.
    SeamlessPeriodicBand,
    /// A **NURBS surface promoted to an analytic kind** (D7 stage 1,
    /// ruling #256): the patch's residual against the fitted analytic
    /// surface CERTIFIED at ε_in, so the face is adopted on the
    /// analytic chart — D3's exactness benefits restored to the
    /// imported body (analytic pcurve lanes, exact tier-3 volume,
    /// curved sense arms). The boundary graph is untouched: the
    /// census pair on the record is the identity map, and the
    /// geometric motion is bounded by the recorded residual (~0 for
    /// an exact emission). Reported, never silent — this is the one
    /// normalization that changes a surface's DESCRIPTION rather than
    /// its tessellation.
    SurfacePromotion {
        /// The kind that certified.
        to: PromotedKind,
        /// The certified residual sup (meters): the patch's worst
        /// deviation from the promoted surface over the certification
        /// domain (control-net bound or fixed sampled grid — the
        /// recognizer's dual track).
        residual: f64,
    },
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StructureNormalization {
    /// The `ADVANCED_FACE` entity instance the file states.
    pub face: u64,
    /// What was re-minted.
    pub kind: NormalizationKind,
    /// The census the file states for that region.
    pub file_census: FaceCensus,
    /// The census as THIS mint left the region — a mint-event value,
    /// not an at-rest one: a later normalization may split an edge the
    /// region shares (a band's rim shared with a neighbouring band)
    /// and the earlier record is not revised. The records' deltas sum
    /// to the body's totals exactly; per-face at-rest counts are the
    /// body's to answer.
    pub kernel_census: FaceCensus,
}

/// The analytic kinds D7 stage-1 CURVE recognition can promote a NURBS
/// carrier to (#327). Line-as-degree-1, ellipse, helix, and open
/// (partial) circular arcs are named exclusions with filed follow-ups;
/// such carriers stay NURBS.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PromotedCurveKind {
    /// A full, closed circle.
    Circle,
}

impl core::fmt::Display for PromotedCurveKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Circle => "circle",
        })
    }
}

/// A **reported curve promotion** (D7 stage 1, #327): the file stated a
/// NURBS carrier whose residual against an analytic curve CERTIFIED at
/// ε_in, so the edge adopted on the analytic carrier — D3's exactness
/// benefits restored (analytic pcurve lanes, the `MappedCurve` rungs of
/// the adoption ladder, exact re-export).
///
/// **Why this is not a [`StructureNormalization`]** (the small design
/// elaboration #327 asked for, argued): that record is FACE-keyed and
/// carries a census pair, because a structure normalization's whole
/// subject is a re-minted boundary graph — how many faces, edges and
/// vertices a region became. A curve promotion re-mints NOTHING: the
/// boundary graph is untouched, so a census pair would be the identity
/// map twice over on a quantity the record does not even name (a curve
/// has no face census), and the key is a CURVE entity, not a face. The
/// honest parallel is a separate, smaller record: entity id, kind,
/// residual. It carries no census by design, and its absence is the
/// statement that nothing was re-tessellated.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CurvePromotion {
    /// The curve entity instance the file states (the `EDGE_CURVE`'s
    /// carrier — e.g. dm1's `#684`, not the `#685` edge that uses it:
    /// one carrier may serve several edges, and the promotion is a
    /// property of the carrier).
    pub curve: u64,
    /// The kind that certified.
    pub kind: PromotedCurveKind,
    /// The certified residual sup (METERS): the carrier's worst
    /// deviation from the promoted curve over its whole domain. Read
    /// off ring-coefficient hulls — exact, whole-domain, no sampling
    /// (`recognize_curve`'s module docs carry the derivation and the
    /// meters² → meters conversions).
    pub residual: f64,
}

/// Per-call import options (D7's ε_in override door).
#[derive(Clone, Debug, Default)]
pub struct ImportOptions {
    /// Overrides the file's declared `UNCERTAINTY_MEASURE_WITH_UNIT`
    /// as the import's input tolerance ε_in. `None` (default) reads
    /// the file's declaration. Must be finite and strictly positive
    /// when given ([`StepImportError::InvalidEpsOverride`]).
    pub eps_in: Option<f64>,
    /// **The import-side declaration channel** (M9-2, D7 step 4's
    /// residue): contact declarations the adopting CALLER attaches to
    /// this import, resolved against the assembled body and certified
    /// by the SAME tier-3′ gate a native declared-contact body runs —
    /// there is no import-only validity path (the #276/#260 one-gate
    /// ruling). A file has no arena keys to declare with, so the
    /// channel is POSITION-anchored; an anchor that does not resolve
    /// is [`StepImportError::DeclarationUnresolved`], never dropped.
    pub declared_contacts: Vec<ImportContact>,
}

/// One position-anchored import declaration (module docs at
/// [`ImportOptions::declared_contacts`]).
///
/// The kernel-side currency these resolve INTO is
/// [`topo::ContactRecords`] — the boolean 3′ records, same type, no
/// adapter. The vertex-rest anchor is the shipped arm (the touching-
/// assembly class the corpus holds); face-granularity anchors extend
/// here when their first import consumer lands, through the same
/// resolve-or-refuse posture.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImportContact {
    /// Two vertices of the assembled body coincide at `at` and are
    /// declared at REST: resolves to the exactly-two vertices within
    /// ε_in of the anchor — the FILE's tolerance, not the kernel band
    /// — and mints their `VvContact`.
    VertexRest {
        /// The anchor position (model metres).
        at: [f64; 3],
    },
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
        /// The adopted body — first-class: Euler-built, certified, and
        /// **tier-valid at rest, checked** (M7-7, #260 ruling (a)):
        /// `topo::validate_geometric` passed on THIS body, and on each
        /// of its solids taken alone, before it was handed out — so
        /// the promise is a measurement rather than a claim, and it is
        /// the promise a native body's caller makes with the same
        /// call. (Per solid as well as whole, because the gate's +V
        /// invariant is a flux SUM: on a multi-solid body an
        /// inside-out solid can hide behind a right-side-out one.)
        ///
        /// A body whose native twin refuses tier 3 (a rational-walled
        /// loft whose volume quadrature exhausts its round budget, say)
        /// does not arrive here at all; the gate
        /// hands back its verdicts as
        /// [`StepImportError::TierInvalid`]. Nothing imports into a
        /// state its native twin does not occupy — and nothing imports
        /// into a state the kernel will not certify.
        body: Body<f64>,
        /// The import's input tolerance ε_in (meters): the override if
        /// given, else the file's declared uncertainty.
        eps_in: f64,
        /// The structure normalizations the adoption applied, in
        /// resolution order — empty for a file whose boundary graph the
        /// kernel represents as stated (every own-corpus file).
        normalizations: Vec<StructureNormalization>,
        /// The **curve promotions** this import applied (#327), by
        /// ascending curve entity id — empty for a file whose carriers
        /// are all stated analytically or all stay NURBS.
        curve_promotions: Vec<CurvePromotion>,
        /// **The assembly record** ([`PlacedInstance`], A7): one entry
        /// per solid of `body`, in `body.solids()` order, saying what
        /// the file said about it — which component representation it
        /// came from, which `MANIFOLD_SOLID_BREP`, which occurrence
        /// and transform stated it, and the rigid map applied.
        ///
        /// Flattening is the correct evaluation product (A2: N placed
        /// instances ARE one non-connected body), but flattening is
        /// not forgetting: this is the structure a later
        /// import-as-assembly-document door needs, and it is cheap
        /// here and expensive to retrofit, so it is kept whether or
        /// not the file states an assembly at all.
        instances: Vec<PlacedInstance>,
    },
    /// The file carried a `GEOMETRIC_CURVE_SET` wireframe and no
    /// solid: the reconstructed carriers, exact. **No body is
    /// claimed** — a wireframe has no faces, shells, or solids to
    /// adopt, so there is nothing a `Body` could honestly represent; a
    /// disposition, not a skip.
    Wireframe {
        /// The curve-set's carriers in file order. Exact (control
        /// points / knots / weights bitwise) for every carrier that
        /// stays as stated — and D7 stage-1 CURVE recognition runs on
        /// this lane too, so a carrier that CERTIFIES is handed back
        /// as its analytic kind, with the promotion recorded in
        /// `curve_promotions` below. Recognition is a property of the
        /// carrier, not of what a body later does with it; a wireframe
        /// whose promotions were silent would be the one place in this
        /// crate where a description changed without being reported.
        curves: Vec<geom::Curve3<f64>>,
        /// As [`StepImport::Solid::eps_in`].
        eps_in: f64,
        /// The curve promotions applied to `curves` (#327), by
        /// ascending curve entity id.
        curve_promotions: Vec<CurvePromotion>,
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

    /// The curve promotions this import applied (#327). Both lanes
    /// report: recognition fires at the sole `NurbsCurve3`
    /// construction site, which the `GEOMETRIC_CURVE_SET` lane reaches
    /// as well as the solid lane.
    pub fn curve_promotions(&self) -> &[CurvePromotion] {
        match self {
            Self::Solid {
                curve_promotions, ..
            }
            | Self::Wireframe {
                curve_promotions, ..
            } => curve_promotions,
        }
    }

    /// The assembly record ([`PlacedInstance`], A7) — empty for a
    /// wireframe, which has no solid to instance.
    pub fn instances(&self) -> &[PlacedInstance] {
        match self {
            Self::Solid { instances, .. } => instances,
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
/// cover, topology that does not assemble, geometry the D7 adoption
/// ladder cannot certify, or a body the kernel's shared at-rest gate
/// refuses ([`StepImportError::TierInvalid`]). Files written by
/// `step_export::step_string` from finished kernel bodies import
/// cleanly.
pub fn import_step(
    text: &str,
    options: &ImportOptions,
    tol: Tol,
) -> Result<StepImport, StepImportError> {
    if let Some(eps) = options.eps_in
        && !(eps.is_finite() && eps > 0.0)
    {
        return Err(StepImportError::InvalidEpsOverride { value: eps });
    }
    let file = parse::parse_file(text)?;
    let model = entities::resolve(&file, options.eps_in)?;
    let eps_in = options.eps_in.unwrap_or(model.uncertainty_m);
    match model.shape {
        entities::Shape::Solids(ref solids) => {
            // **Materialization** (M8 instancing). The model says what
            // to make: one entry per placed INSTANCE, each naming a
            // solid and the frame that copy sits in. N occurrences of
            // one component representation are N entries over the same
            // solid index, and each is built into a body of its OWN —
            // fresh topology, fresh arena keys, no structure shared
            // between copies. Sharing was never on the table: a
            // `SolidSpec`'s maps are keyed by the file's entity ids,
            // so two copies assembled into one arena would collide id
            // for id.
            //
            // Each instance then goes through the kernel's own
            // placement door (M7-4 Leg D, unchanged): `transform_rigid`
            // re-checks rigidity with decided predicates and
            // RE-CERTIFIES every carrier against the mapped geometry,
            // so a placed copy is as first-class as an unplaced one —
            // and a map this reader let through that the kernel will
            // not becomes a typed refusal, never a silently skewed
            // body. N instances is N re-certifications, paid per copy
            // because each copy is a different body.
            //
            // The copies meet in one arena through
            // `topo::graft_disjoint` — the disjoint half of the
            // boolean pipeline's combine door, which transplants a
            // body's solid under a solid of its own with fresh keys in
            // deterministic slot order. Nothing is fused; the shipped
            // body is entity for entity the union of the bodies gated
            // below, not a re-derivation of them.
            let mut body = topo::Body::new();
            let mut record = Vec::with_capacity(model.instances.len());
            for (index, instance) in model.instances.iter().enumerate() {
                let spec = &solids[instance.solid];
                let one = assemble::build_one_solid(spec, tol)?;
                let one = match instance.placed {
                    Some(entities::Placed {
                        map: Some(map),
                        transform,
                        ..
                    }) => topo::transform_rigid(&one, &map, tol)
                        .map_err(|source| StepImportError::Placement { transform, source })?,
                    _ => one,
                };
                // The per-solid subject of the shared gate (below),
                // asked about the PLACED copy — the body that ships.
                // With one instance the per-solid and aggregate
                // subjects are the same body, so this call is skipped
                // as an identity, never as an exemption.
                if model.instances.len() > 1 {
                    gate(&one, Some(spec.id), tol)?;
                }
                topo::graft_disjoint(&mut body, &one, tol).map_err(|source| {
                    StepImportError::Instance {
                        solid: spec.id,
                        source: Box::new(source),
                    }
                })?;
                // The A7 record, minted where the instance is: `index`
                // is the graft order, and the graft appends one solid
                // per call, so it IS the shipped body's `solids()`
                // order (pinned by `the_assembly_record_indexes_the_
                // shipped_solids`).
                record.push(PlacedInstance {
                    index,
                    solid: spec.id,
                    component: instance.component,
                    occurrence: instance.placed.and_then(|p| p.occurrence),
                    relationship: instance.placed.map(|p| p.relationship),
                    transform: instance.placed.map(|p| p.transform),
                    placement: instance.placed.and_then(|p| p.map),
                });
            }
            // **The shared at-rest validation gate** (M7-7, the #260
            // ruling (a) + D9 engineering convention 2). Every
            // imported solid is held to the kernel's invariants by the
            // SAME function a native body's caller runs at rest —
            // `topo::validate_geometric`, tiers 1–3 — reached only
            // through `gate` below, which adds no opinion of its own:
            // no kind predicate selects which bodies are asked (the
            // band re-mint's backstop was that opinion, and it
            // dissolved here — bands were only special because
            // ordinary solids skipped the gate), and no verdict filter
            // decides which failures matter (an escalated verdict is a
            // refusal, not a pass: escalate-never-guess). Adoption
            // certifies each EDGE's intensional description; this
            // certifies the BODY, which is what `StepImport::Solid`
            // promises at rest.
            //
            // Asked twice, for two different subjects. Several of the
            // gate's invariants are WHOLE-BODY sums — check 7's +V is
            // the boundary flux summed over every shell — so a solid
            // stated inside-out cancels against a right-side-out
            // neighbour and the aggregate reads Zero, which is exempt.
            // "Every imported solid passes the gate" therefore has to
            // mean each INSTANCE's own body, which is exactly the body
            // the materialization loop above already holds, and the
            // refusal names which `MANIFOLD_SOLID_BREP` it came from.
            // The aggregate pass stays: it is the subject that owns
            // the cross-solid structure (shared arena integrity, edges
            // across shells) no per-solid view can see.
            //
            // With one instance the two subjects are the same body, so
            // the per-solid call would re-run the aggregate call on
            // identical geometry — skipped as an identity, never as an
            // exemption.
            //
            // The per-solid subject is the PLACED copy (M8): the body
            // that ships is the union of exactly these, so gating them
            // before placement would gate something else. It costs
            // nothing in verdicts — `transform_rigid` admits only
            // det = +1 maps and re-certifies every carrier against the
            // mapped geometry, so no tier-3 verdict is a function of
            // the placement — and it costs nothing in honesty.
            //
            // **The 3′ form, with the import-side declaration
            // channel resolved** (M9-2, D7 step 4 executed): the
            // aggregate body is held to `validate_pseudomanifold` —
            // the SAME gate a native declared-contact body runs,
            // against exactly the records the adopting caller's
            // declarations resolve to (empty when none were given).
            // Consequence, stated plainly: an imported assembly whose
            // parts TOUCH now refuses UNDECLARED at this gate — the
            // census discovers the touch and F1 forbids blessing it —
            // and certifies WITH the declaration. The
            // per-solid gates above stay tier 3: contact is an
            // aggregate-body fact, and the aggregate census sweeps
            // every entity of every instance.
            let records = resolve_declarations(&body, &options.declared_contacts, eps_in)?;
            gate3(&body, &records, tol)?;
            Ok(StepImport::Solid {
                body,
                eps_in,
                normalizations: model.normalizations.clone(),
                curve_promotions: model.curve_promotions.clone(),
                instances: record,
            })
        }
        entities::Shape::Wireframe(ref curves) => Ok(StepImport::Wireframe {
            curves: curves.clone(),
            eps_in,
            curve_promotions: model.curve_promotions.clone(),
        }),
    }
}

/// The reader's ONLY contact with the kernel's at-rest validator: pass
/// a body, get the verdicts back as a typed refusal naming the subject
/// (`solid` = the `MANIFOLD_SOLID_BREP` asked about alone, `None` = the
/// assembled body). Nothing is filtered, nothing is reworded, no
/// verdict class is privileged — the whole point is that import has no
/// validation logic that could drift from the kernel's (D9 engineering
/// convention 2). If this function ever grows a condition, the gate has
/// grown an opinion.
fn gate(body: &topo::Body<f64>, solid: Option<u64>, tol: Tol) -> Result<(), StepImportError> {
    topo::validate_geometric(body, tol)
        .map_err(|errors| StepImportError::TierInvalid { solid, errors })
}

/// The aggregate subject's gate: the tier-3′ form over the resolved
/// declaration records — the same function a native declared-contact
/// body's caller runs, with the same no-opinion contract as [`gate`].
fn gate3(
    body: &topo::Body<f64>,
    records: &topo::ContactRecords,
    tol: Tol,
) -> Result<(), StepImportError> {
    topo::validate_pseudomanifold(body, records, tol).map_err(|errors| {
        StepImportError::TierInvalid {
            solid: None,
            errors,
        }
    })
}

/// Resolves the position-anchored import declarations against the
/// assembled body into the kernel's contact-record currency
/// ([`ImportContact`] docs). Resolution compares a rounded-f64 sum of
/// squared coordinate differences against ε_in² — the import's own
/// input tolerance, because anchors are FILE-side data and resolve at
/// the file's tolerance, not the kernel band. An anchor with anything
/// other than exactly two coincident vertices refuses typed.
fn resolve_declarations(
    body: &topo::Body<f64>,
    contacts: &[ImportContact],
    eps_in: f64,
) -> Result<topo::ContactRecords, StepImportError> {
    let mut records = topo::ContactRecords::default();
    for c in contacts {
        match *c {
            ImportContact::VertexRest { at } => {
                let candidates = body
                    .vertices()
                    .map(|(vk, v)| (vk, body.get_point(v.point).copied()));
                records.vv.push(vertex_rest_contact(candidates, at, eps_in)?);
            }
        }
    }
    Ok(records)
}

/// One vertex-rest anchor resolved against the body's vertices, each
/// paired with its position: the exactly-two coincidences within ε_in
/// of `at`, as the kernel's `VvContact`.
///
/// A `None` position is a vertex whose point key does not resolve in
/// the body that produced it. That is a corrupt-body state, and no
/// caller here can prove it away: the aggregate body reaches this
/// resolution before any gate has run on it, and the per-solid gate
/// above sees only the pre-graft copies and only when more than one
/// instance ships. Passing over such a vertex would silently
/// understate the census — a resolvable anchor would report as
/// `DeclarationUnresolved` with the wrong `found`, a three-way
/// coincidence would resolve as exactly two — so the census refuses
/// with [`StepImportError::VertexWithoutPoint`] instead.
fn vertex_rest_contact(
    candidates: impl Iterator<Item = (topo::VertexKey, Option<geom_core::Point3<f64>>)>,
    at: [f64; 3],
    eps_in: f64,
) -> Result<topo::VvContact, StepImportError> {
    let mut hits = Vec::new();
    for (vk, position) in candidates {
        let Some(p) = position else {
            return Err(StepImportError::VertexWithoutPoint { anchor: at });
        };
        let d2 = (p.x - at[0]).powi(2) + (p.y - at[1]).powi(2) + (p.z - at[2]).powi(2);
        if d2 <= eps_in.powi(2) {
            hits.push(vk);
        }
    }
    let [a, b] = hits[..] else {
        return Err(StepImportError::DeclarationUnresolved {
            at,
            found: hits.len(),
        });
    };
    Ok(topo::VvContact { a, b })
}

#[cfg(test)]
mod declaration_tests {
    use super::{ImportContact, StepImportError, resolve_declarations, vertex_rest_contact};
    use geom_core::Point3;

    /// Two lone vertices at one position, in one body: the smallest
    /// thing a vertex-rest anchor can resolve against. `mvfs` mints a
    /// solid of its own per call, so two calls put two vertices in one
    /// arena with no edge between them — the shape a touching assembly
    /// presents to the resolver.
    fn two_coincident_vertices(at: Point3<f64>) -> topo::Body<f64> {
        let mut body = topo::Body::new();
        body.mvfs(at).expect("mvfs mints a lone vertex");
        body.mvfs(at).expect("mvfs mints a second lone vertex");
        body
    }

    /// The resolvable case, end to end through the walk: an anchor on
    /// two coincident vertices mints their `VvContact`.
    #[test]
    fn a_vertex_rest_anchor_resolves_two_coincident_vertices() {
        let body = two_coincident_vertices(Point3::new(1.0, 1.0, 1.0));
        let keys: Vec<_> = body.vertices().map(|(vk, _)| vk).collect();
        let records = resolve_declarations(
            &body,
            &[ImportContact::VertexRest { at: [1.0, 1.0, 1.0] }],
            1e-9,
        )
        .expect("two coincident vertices resolve");
        assert_eq!(records.vv.len(), 1, "one declaration, one record");
        assert_eq!(
            [records.vv[0].a, records.vv[0].b],
            [keys[0], keys[1]],
            "the record names the two vertices at the anchor"
        );
    }

    /// **The planted witness.** A vertex whose point key does not
    /// resolve is announced, not passed over. The plant is at the
    /// position seam rather than in the arena because a dangling point
    /// key is unconstructible through `topo`'s public doors — nothing
    /// outside that crate can write a `Vertex::point` — so the corrupt
    /// state is presented to the resolver exactly as the walk would
    /// present it: a real body's real vertex keys, one of them with no
    /// position.
    ///
    /// The refusal is the whole point: the vertex with no position is
    /// one of the anchor's two coincidences, so passing over it would
    /// have reported `DeclarationUnresolved { found: 1 }` — an
    /// honest-looking refusal naming the wrong fault, on a declaration
    /// that is in fact resolvable.
    #[test]
    fn a_dangling_point_key_refuses_rather_than_miscounting() {
        let at = Point3::new(1.0, 1.0, 1.0);
        let body = two_coincident_vertices(at);
        let keys: Vec<_> = body.vertices().map(|(vk, _)| vk).collect();
        let planted = [(keys[0], None), (keys[1], Some(at))];
        match vertex_rest_contact(planted.into_iter(), [1.0, 1.0, 1.0], 1e-9) {
            Err(StepImportError::VertexWithoutPoint { anchor }) => {
                assert_eq!(anchor, [1.0, 1.0, 1.0], "the refusal names the anchor");
            }
            Err(StepImportError::DeclarationUnresolved { found, .. }) => panic!(
                "the dangling key was passed over: the census read {found} coincidences at an \
                 anchor that has two"
            ),
            other => panic!("expected the corrupt-body refusal, got {other:?}"),
        }
    }
}
