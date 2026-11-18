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
//! (`ADVANCED_BREP_SHAPE_REPRESENTATION` over `MANIFOLD_SOLID_BREP`s,
//! and `BREP_WITH_VOIDS` for a solid with cavities)
//! and the curve-set wireframe form
//! (`GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION`). The
//! result of a solid import is a first-class [`topo::Body`]: topology
//! rebuilt through the public Euler-operator vocabulary, geometry
//! adopted per D7: the file's statements are interpreted at its declared
//! coincidence distance, and every edge's intensional description is
//! certified by the kernel's own gates at the run's ε. Edges still carry
//! the file's parsed carrier as their cache into that certification (the
//! build that re-makes every cache by the kernel's own constructions is
//! the next stage of D7's pipeline).
//!
//! # Adoption, not admission (D7, the shape of the pipeline)
//!
//! 1. **Substrate** ([`parse`]): Part 21 tokenization and entity
//!    resolution. Reals parse with `str::parse::<f64>` — the writer's
//!    printer round-trips to identical bits, so downstream exact `==`
//!    comparisons are legitimate on own-corpus files.
//! 2. **Interpretation** ([`entities`], [`interpret`], [`geometry`]):
//!    each AP214 record is read as the kernel description it is evidence
//!    of, at the file's band ([`interpret::file_band`]). A placement is
//!    built from its ratio vectors by `build_axes`, kept bit for bit
//!    where the file states it rigid within its rounding, and its move
//!    reported beyond that ([`NormalizationKind::PlacementReMinted`]); a
//!    cone placed at its base circle is moved to its apex; a NURBS
//!    surface or carrier whose residual reads Zero is promoted; two
//!    surface records of one locus become one
//!    ([`NormalizationKind::OneRecord`]); a value no construction takes
//!    refuses ([`InterpretationRefusal`]).
//! 3. **Topology assembly** ([`assemble`]): the shell's half-edge
//!    structure is realized through `topo`'s public Euler operators
//!    (`mvfs`/`mev`/`mef`, the ring vocabulary, scaffold struts killed
//!    by their inverse ops) — the sanctioned construction path, D1.
//! 4. **The build and adoption** ([`build`], [`adopt`]): every edge's
//!    description is chosen off its faces' surfaces at the file band,
//!    and every cache is built from it by the kernel's own
//!    constructions — the carrier (section, tangent locus, a wall's iso
//!    or a cap's rational rim row, a surface's own generator through the
//!    edge's built ends, the segment between them), each vertex where
//!    the description fixes it (built carriers crossing a surface, a
//!    pole or an apex), else, where the description leaves it free along
//!    one built carrier, at the foot of its printed point on that
//!    carrier, each interval by the carrier's own inverse. No curve or
//!    point the file printed is a cache. Surfaces attach (bitwise-identical records
//!    share one kernel surface key, restoring the writer-side key
//!    sharing), `same_sense` lands as [`topo::Face::sense`] honored, not
//!    healed, and the one built description is **certified** by the
//!    kernel's own gates at ε; a smooth join the must-carry rule answers
//!    with the pair alone refuses not-yet-supported. Pcurve
//!    caches re-mint through [`topo::mint_pcurves`] (every curved
//!    chart that mints natively mints here — cylinder, cone, sphere,
//!    torus, described non-rational NURBS; plane faces stay
//!    derive-on-demand, exactly what a natively built body carries).
//!
//!    **A vertex the description fixes, and one it leaves free.** Where
//!    the description fixes a vertex — two or more built carriers that
//!    cross a surface around it, a sphere's pole, a cone's apex — the
//!    build constructs it, and an escalation there refuses. Where it
//!    leaves the vertex free along one locus (a seam's end on a rim, an
//!    edge split along one carrier, a whole torus's one vertex), every
//!    point of that locus is an equally valid model of the file's
//!    statements; the foot of the printed point on the kernel-built
//!    carrier (or the one surface) is the printed value choosing among
//!    them, read within the file's band, not a cache, and certification
//!    reads it at ε on every incident carrier. Both are reported on the
//!    ledger where they move ([`NormalizationKind::VertexBuilt`]), and a
//!    carrier the kernel built that is not the printed one likewise
//!    ([`NormalizationKind::CarrierBuilt`]).
//! 5. **The shared at-rest gate** (M7-7, #260 ruling (a);
//!    [`import_step`]): the body is handed to
//!    the kernel's own at-rest gate — the scalar's finish
//!    ([`topo::finish`]), tiers 1–3, the same function on the same body
//!    every native door's result goes through — and only a body it passes ships as
//!    [`StepImport::Solid`]. Steps 1–4 certify each edge's
//!    description; this certifies the BODY, which is what "import is
//!    adoption" has to mean if it means anything.
//!
//!    The finish (`gate`) is asked once on the assembled body, and,
//!    where [`topo::per_part_gate_owed`] says the file owes it — more
//!    than one instance — once per placed instance **on that solid's
//!    own body**, so the refusal names the `MANIFOLD_SOLID_BREP` it is
//!    about. Why each solid is asked is `docs/DESIGN.md` import step 4.
//!    The assembled body, finished, then meets the census against its
//!    records (`gate3`, the rest of tier 3′). The finish and that census
//!    are the only places this crate calls the validator, and neither
//!    holds an opinion: no body kind is exempt, no verdict class is
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
//!    **The aggregate gate is tier 3′** (D7 step 4): the coincidence
//!    census over the assembled body, each solid its own group, records
//!    every touch between two solids as a row of its own Zero and a
//!    record citing it (`topo::record_at_rest`), which ship on
//!    [`StepImport::Solid`]; a touch inside one solid, or two solids
//!    that overlap, refuse.
//!
//! # Two tolerances (D7)
//!
//! ε_in — the per-import *input* tolerance — defaults from the file's
//! `UNCERTAINTY_MEASURE_WITH_UNIT` and is overridable per call
//! ([`ImportOptions::eps_in`]); it is carried on the result
//! ([`StepImport::eps_in`]). It governs interpretation: a question
//! asked of the file's statements is a distance in metres read through
//! the kernel's one classification funnel at the file's band, `(e, K·e)`
//! for `e` the larger of ε_in and the run's ε ([`interpret::file_band`]),
//! with `[verdicts]`' three arms. A lane choice (which kind a carrier
//! is) takes its closed form at Zero; a structural act (one record, a
//! vertex at the seam) acts at Zero, keeps apart past the band, and
//! refuses in it, naming what would decide it. One pass still reads ε_in
//! as a raw distance, the C⁰-knot split
//! (`work/importgate/the-c0-knot-split-reads-eps-in-raw-not-at-the-file-band.md`).
//! What interpretation moves is on the result's move ledger
//! ([`StepImport::normalizations`]), never in the body. Certification
//! of everything built runs at the run's ε like native geometry.
//!
//! # Chart coherence: a measurement, not a gate
//!
//! [`ImportOptions::examine_chart_coherence`] asks
//! [`topo::examine_chart_coherence`] about the body that ships and
//! carries its report at [`StepImport::Solid::coherence`]. The
//! examination measures how far two of the FILE's own statements
//! about one chart coordinate disagree — a carrier's midpoint azimuth
//! against its own endpoint vertex's, two edges of one rim row, two
//! edges of one meridian column — each as a gap times a lever arm, in
//! metres, against the band. This is the door where defective source
//! coordinates actually arrive. Its recorded witness was a
//! pole-crossing meridian arc (`tests/fixtures/halfcap/`), whose carrier
//! midpoint sat a half-turn from its own endpoint; import now makes the
//! pole a vertex ([`NormalizationKind::ChartSingularityVertex`]), so no
//! committed file ships a finding.
//!
//! **Three things this channel is not.** It is not a gate: the at-rest
//! validator above has already run and passed by the time the
//! examination is asked, and a body with findings ships exactly as one
//! without them does — nothing in the import acceptance decision reads
//! it. It is not the ε_in lane: the band is the run's ambient ε, the
//! same tolerance everything else built here certifies at, and each
//! finding carries the band it was judged against. And it is not on by
//! default, because the examination has no shape door and would report
//! a lane boundary (a trimmed curved face's loops) to callers who
//! asked no question.
//!
//! # Fail loud (D4 ¶5)
//!
//! Every refusal is a typed [`StepImportError`] naming the offending
//! entity id / line: malformed syntax, dangling references,
//! unsupported entity types, units outside the subset, and geometry
//! the build cannot interpret at the file band or that does not certify. No panics; no silent guesses;
//! no lenient re-interpretation.
//!
//! # NURBS faces (M7-3)
//!
//! Both `B_SPLINE_SURFACE_WITH_KNOTS` arms import — the non-rational
//! simple record and the `RATIONAL_B_SPLINE_SURFACE` complex
//! instance. Wall–wall seams adopt through the
//! **IsoCurve rung** (the carrier bitwise-matched against the
//! adjacent walls' `boundary_iso_u` columns, certified through the
//! iso residual lane). A cap rim adopts at the `Intersection` rung
//! where the plane × NURBS certificate takes its carrier — a straight
//! rim, whose chart image on the wall is straight — and otherwise
//! through the conventional rung's Nurbs-adjacency exemption
//! (`PlacedSegment` for a line rim the certificate refuses — the
//! native loft's own description class, which is what the NURBS
//! chart's pcurve mint accepts; `RevolvedPoint` for arc rims, whose
//! degree-1 chart image the certificate's hull refuses, on rational
//! walls minting nothing — the native rational body's own state).
//! Multi-ring NURBS faces keep refusing typed at the curved-face ring
//! gate (no NURBS chart inversion — stage-1 recognition territory,
//! named there), as do spline sub-types the writer never emits
//! (knots-implied `QUASI_UNIFORM_CURVE` and kin).
//!
//! **Coverage is bounded by what the writer can emit** (#207): the
//! loft/sweep skin's chord-length fit drifts unit weights on any
//! curved-path sweep or non-uniformly spaced loft, so those bodies
//! refuse at BUILD time and no file of them exists to import. The
//! round-trippable class today is uniformly-spaced lofts with polyline
//! profiles (non-rational, full tier 3). Arc-bearing profiles export
//! and read, and their rational walls now have a volume quadrature
//! that converges through interior knots — but it is a composite under
//! a per-face work wall, so a large or strongly curved rational wall can
//! still stop short of a fine resolution. The at-rest gate below
//! decides each solid's volume SIGN and admits such a body, and the
//! import hands back the certificate the gate decided on: a caller
//! measures through it under its own predicate, and an unmet resolution
//! is a bracket, not a refusal of the file.
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
//!   `build_axes` defaults are read rather than guessed. A frame whose
//!   stated fields are rigid within their rounding is adopted bit for
//!   bit; any other is built by `build_axes`.
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
//! ring, which this reader adopts on no curved chart yet — `topo`
//! measures one on a cone wall and on a cylinder or torus wall bounded
//! by rims and rulings, and no other — and edges whose built description does
//! not certify. Open CASCADE's
//! seamless periodic band NORMALIZES since
//! M7-5: cylinder and torus bands take the seam re-mint
//! ([`NormalizationKind::SeamlessPeriodicBand`]); band shapes on
//! other charts keep a typed refusal naming that re-mint as the
//! recourse.

mod adopt;
mod assemble;
mod build;
mod chart;
// `chart`'s randomized sweep, in a module of its own so the per-file test
// gate can skip it without skipping `chart`'s deterministic rows.
#[cfg(test)]
mod chart_review_fuzz;
#[cfg(test)]
mod cr_r1_probes;
mod entities;
mod error;
mod geometry;
mod interpret;
mod knot_split;
mod normalize;
mod one_record;
mod parse;
mod recognize;
mod recognize_curve;
mod signed_zero;
mod units;

pub use error::{
    AdoptionAttempt, AdoptionCandidate, BuiltEntity, SingularityBand, StepImportError,
};
pub use interpret::Refusal as InterpretationRefusal;

use geom_core::Tol;

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

impl FaceCensus {
    /// The census of no region: a ledger entry that re-mints no face,
    /// edge or vertex.
    pub const EMPTY: Self = Self {
        faces: 0,
        edges: 0,
        vertices: 0,
    };
}

/// **One materialized assembly instance** — the A7 record shape
/// (`crates/editor-core/ASSEMBLY.md`), kept so a flattened import can later
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
        f.write_str(
            match self {
                Self::Plane => geom::SurfaceKind::Plane,
                Self::Cylinder => geom::SurfaceKind::Cylinder,
            }
            .name(),
        )
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
    /// **A chart singularity in a face is a vertex of it.** A sphere's
    /// pole or a cone's apex that lies in a face's closure is a vertex
    /// of the face, in the pole form (`geom::pole_form`): the end of two
    /// meridians half a turn apart, the face reaching it between them.
    /// A file that states the point some other way is re-minted into
    /// that form, by where the point lies ([`SingularityFound`]), read
    /// by the kernel's one reading of it
    /// (`geom_brep::chart_singularity::locate`, whose boundary placement,
    /// `place_on_boundary`, tier 3 shares; tier 3 reads the side through
    /// the face's point-in-face door, and the import's own side reading
    /// is filed to follow it,
    /// `work/importgate/import-reads-a-chart-singularitys-side-by-the-chart-walk-tier-3-is-to-leave.md`):
    /// the face's boundary and the region its sense bit names say which.
    /// The minted meridians are the surface's own (at its `u_ref`
    /// azimuth and the half turn, or the half-turn twin of a seam the
    /// file states), each ending at the first boundary edge it meets,
    /// which is split there for every face sharing it. What the file
    /// leaves undecidable refuses typed rather than guessing: a point
    /// within the band of the boundary but neither on nor off it, a
    /// boundary carrier the crossing reads nothing of (not a line or a
    /// circle), a winding the chart inverse cannot read, and a seam
    /// ending at the point on an orientation-inverted face.
    ChartSingularityVertex {
        /// Where the file put the point.
        found: SingularityFound,
    },
    /// A **seamless periodic band** (M7-5): a cylinder or torus
    /// lateral face stated as its two full-period rim bounds with NO
    /// seam generator between them (Open CASCADE never splits a
    /// periodic face on export). The kernel's face model has one outer
    /// loop plus rings, and a ring that winds the chart's period is a
    /// loop whose chart walk does not close on its start
    /// (`topo::loop_windings`), so the band cannot adopt as stated. Re-minted as the kernel's
    /// own shape for the same locus:
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
    /// **Two edges the file states on one carrier, joined** (Ev's PR
    /// 4251 ruling; `docs/DESIGN.md`, maximal edges): the file stops an
    /// edge at a vertex where nothing else meets it and the next edge
    /// runs on along the same curve, so the vertex is no cell of the
    /// solid, and the import ends with the join every finisher ends
    /// with. One record per join, keyed by the least `ADVANCED_FACE`
    /// the joined edge bounds; its census pair is the join's own
    /// region — two edges and the vertex between them, become one edge
    /// — so the records' deltas still sum to the body's totals.
    JoinedEdges,
    /// **An edge carrier split at its interior C⁰ knots** (Ev's PR 5913
    /// ruling; `docs/DESIGN.md` `[topology]`): a B-spline carrier with an
    /// interior knot run of multiplicity at least its degree has no
    /// derivative there by its structure, so the edge is cut into its
    /// sub-arcs, a vertex at each knot. Exact: each sub-arc is a
    /// partition of the carrier's own net. One record per split edge,
    /// keyed by the least `ADVANCED_FACE` it bounds; its census pair is
    /// the edge, become `n` edges and the `n − 1` vertices between them.
    EdgeSplitAtC0Knot,
    /// **A face chart split along its interior C⁰ knots' iso lines**
    /// (the same ruling): each sub-patch is a partition of the file's
    /// net, and the iso curve between two of them is minted as the
    /// edge they share. One record per file face; its census pair is
    /// the face's boundary as the file states it (after its edges'
    /// own splits) and the faces it became with every edge and vertex
    /// they use.
    FaceSplitAtC0Knot,
    /// **A placement whose stated fields `build_axes` moved** (`[import]`,
    /// `[tolerance.boundary]`): a placement frame whose fields as stated
    /// are no rigid frame within their own rounding is built from its
    /// ratio vectors by ISO 10303-42's `build_axes` (the axis normalized,
    /// the reference direction projected perpendicular and normalized).
    /// A STEP `direction` is a ratio vector, so normalizing one is its
    /// meaning and moves nothing: `moved_by` is measured from the frame
    /// of the two directions normalized (its columns the reference, the
    /// axis crossed with it and the axis) to the built frame, as the
    /// farthest a point of the session box moves. Keyed by the
    /// `AXIS2_PLACEMENT_3D` of a surface's or curve's placement, or by an
    /// assembly's `ITEM_DEFINED_TRANSFORMATION`, whose one entry sums its
    /// two frames' moves and, where the pair reads as the identity at the
    /// file's band, the snap to it. A file's ε_in bounds no such move
    /// (`work/onehome/a-step-files-input-tolerance-has-no-bound-the-ratified-text-states.md`).
    /// Keyed by that record's instance id, not a face; its census
    /// pair is empty, since no face, edge or vertex changes.
    PlacementReMinted {
        /// The `AXIS2_PLACEMENT_3D` or `ITEM_DEFINED_TRANSFORMATION` whose
        /// frames were moved.
        transform: u64,
        /// The move (metres): over the session box, the farthest a point
        /// moves between a frame normalized and built, summed over a
        /// pair's frames and its snap to the identity.
        moved_by: f64,
    },
    /// **Two surface records of one locus, made one** (`[import]`,
    /// `crates/topo/README.md` `[topo.one-record]`): on one solid, the
    /// faces on `joined` read as lying on `kept`'s surface at the file's
    /// band, decided at the faces, and the two face one way, so they take
    /// `kept`, the earlier record in the file. Keyed by the first face on `joined`; its
    /// census pair is empty, since no face, edge or vertex changes.
    OneRecord {
        /// The surface record kept (the earlier in the file).
        kept: u64,
        /// The surface record its faces left.
        joined: u64,
        /// The move (metres): a bound on the farthest a face on either
        /// record lies from the other's surface.
        moved_by: f64,
    },
    /// **A vertex the build put off its printed point** (`[import]`, D7
    /// step 2): where its description fixes it (a root of a surface around
    /// it along a carrier through it, a pole, an apex), or, where the
    /// description leaves it free along one built carrier, the foot of its
    /// printed point there; any move, read within the file's band. Keyed by
    /// the `VERTEX_POINT`; its census pair is empty, since no face, edge or
    /// vertex is added or lost.
    VertexBuilt {
        /// The move (metres): the built point's distance from the printed
        /// one.
        moved_by: f64,
    },
    /// **An edge whose built carrier is not the printed one** (`[import]`,
    /// D7 step 2): the carrier the kernel constructed from the edge's
    /// description differs from the printed carrier as a value (a datum's
    /// bits, a re-anchored line among them); a built carrier equal to the
    /// printed one is not reported. Keyed by the `EDGE_CURVE`; its census
    /// pair is empty.
    CarrierBuilt {
        /// The move (metres): the farthest of the printed carrier's ends,
        /// quarters and middle from the built carrier, measured as read
        /// (zero for the same locus under another datum).
        moved_by: f64,
    },
    /// **A NURBS carrier promoted to an analytic kind** (D7 stage 1,
    /// #327): the carrier's residual against the analytic curve read
    /// Zero at the file's band, so every edge on it adopts the analytic
    /// carrier ([`CurvePromotion`] is this record read off the ledger).
    /// Keyed by the curve entity, not a face; its census pair is empty,
    /// since nothing is re-tessellated.
    CurvePromoted {
        /// The kind that certified.
        kind: PromotedCurveKind,
        /// The certified residual sup (metres): the carrier's worst
        /// deviation from the promoted curve over its whole domain.
        residual: f64,
    },
}

/// The K row a placement frame's two directions are decided under where
/// `build_axes` mints it ([`NormalizationKind::PlacementReMinted`]).
pub const STEP_PLACEMENT_AXES: &str = "step_placement_axes";

/// The decision whether an assembly placement pair places anything: the
/// farthest a point of the session box moves under its motion, read at
/// the file's band (Zero takes the identity).
pub const STEP_PLACEMENT_IDENTITY: &str = "step_placement_identity";

/// The K row a line's frame is built under where its direction triple is
/// read for unit length within its rounding.
pub const STEP_LINE_DIRECTION_UNIT: &str = "step_line_direction_unit";

/// Where a file put a chart singularity that is not yet a vertex in
/// the pole form ([`NormalizationKind::ChartSingularityVertex`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SingularityFound {
    /// Inside the face, off its boundary: the point is minted as a
    /// vertex with the pole form's two meridians, and the face is cut
    /// in two along them (the rim-only sphere cap, a tilted cap, a
    /// cone cap bounded by its rim alone).
    Interior,
    /// Inside a boundary edge: the edge is split at the point, for
    /// every face sharing it (a meridian arc stated across the pole).
    InsideEdge,
    /// At a vertex with one edge, a seam the face uses both ways (Open
    /// CASCADE's full cone and sphere cap): the pole form is completed
    /// with the seam's half-turn twin, and the face is cut in two along
    /// the pair.
    ValenceOneVertex,
}

/// A **reported normalization**, one row of the move ledger
/// ([`StepImport::normalizations`]): something interpretation re-minted
/// or moved beyond rounding, said as data, never silently.
///
/// Two families share the row. A **structure normalization** (D7
/// stage-3 repair, in its letter: the face kinds) re-mints a
/// tessellation the file's locus fully explains but whose boundary
/// graph is not representable; volume and validity are exact, and what
/// changes is only how the same surface is cut into faces, edges and
/// vertices, the census pair being the mapping a reader needs to
/// reconcile the file's counts with the body's. An **interpretation
/// move** ([`NormalizationKind::OneRecord`],
/// [`NormalizationKind::PlacementReMinted`],
/// [`NormalizationKind::CurvePromoted`]) changes what the file's
/// statements are read as, by at most the distance it reports (a face
/// carried onto another record, a frame built, a carrier promoted
/// within its certified residual), so the body differs from the stated
/// geometry by that bound; its census pair is [`FaceCensus::EMPTY`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StructureNormalization {
    /// The `ADVANCED_FACE` entity instance the file states; where the
    /// kind names no face, the entity it does: an `AXIS2_PLACEMENT_3D` or
    /// `ITEM_DEFINED_TRANSFORMATION` ([`NormalizationKind::PlacementReMinted`]),
    /// a curve ([`NormalizationKind::CurvePromoted`]). The ledger's own row
    /// type is filed
    /// (`work/importgate/the-move-ledger-wants-its-own-row-type.md`).
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

/// The analytic kinds D7 CURVE recognition can promote a NURBS
/// carrier to (#327 stage 1; the line #388). Ellipse, helix, and open
/// (partial) circular arcs are named exclusions with filed follow-ups;
/// such carriers stay NURBS.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PromotedCurveKind {
    /// A full, closed circle.
    Circle,
    /// A chord segment (a certified degree-agnostic polyline leg —
    /// the carrier lies on the chord between its end control points
    /// and covers exactly that segment).
    Line,
}

impl core::fmt::Display for PromotedCurveKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(
            match self {
                Self::Circle => geom::CurveKind::Circle,
                Self::Line => geom::CurveKind::Line,
            }
            .name(),
        )
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
    /// **Ask for the chart-coherence examination** of the shipped body
    /// ([`topo::examine_chart_coherence`]), reported at
    /// [`StepImport::Solid::coherence`]. Default `false`.
    ///
    /// **Off by default, which is the examination's own posture rather
    /// than timidity about its cost.** It has no shape door in front
    /// of it: a curved face whose outer loop carries a conic or spline
    /// trim carrier states none of the three conditions, so its loops
    /// land in [`topo::CoherenceReport::unexamined`] as a LANE
    /// BOUNDARY rather than a defect. A channel that is on for every
    /// caller reports that lane boundary to callers who asked no
    /// question, and a report that fires everywhere is not a report.
    /// The registry resident on this same kernel door
    /// (`editor_core`'s `CheckId::ChartCoherence`) ships `Off` for the
    /// same reason.
    ///
    /// **It decides nothing.** A finding is a MEASUREMENT of how far
    /// two of the file's own statements about one chart coordinate
    /// disagree; no file imports or refuses differently for this flag,
    /// and the at-rest gate never reads it.
    pub examine_chart_coherence: bool,
}

/// A successful import: what the file's shape representation supports.
///
/// The variant sizes differ by design (a whole body vs a curve list);
/// boxing the body would put an indirection in every consumer of the
/// common case for no benefit at an import-sized allocation rate.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum StepImport {
    /// The file carried solid records: a first-class kernel body, one
    /// kernel solid per record — a `MANIFOLD_SOLID_BREP` adopts as a
    /// solid of its one shell, and a `BREP_WITH_VOIDS` as one solid of
    /// its outer shell and a shell per void. A solid is one piece of
    /// material, which is the grouping the writer states, so pieces
    /// that only touch (the kiss assembly) arrive as the separate solids
    /// they were and independent readers report.
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
        /// A body whose native twin refuses tier 3 does not arrive here
        /// at all; the gate hands back its verdicts as
        /// [`StepImportError::TierInvalid`]. Nothing imports into a
        /// state its native twin does not occupy — and nothing imports
        /// into a state the kernel will not certify.
        ///
        /// **It arrives finished** ([`topo::AtRestBody`], the scalar's
        /// one finished type): the certificate the aggregate gate's
        /// check 7 decided the +V invariant on is kept on it rather
        /// than dropped, and is of this body by construction. The
        /// import has no purpose to state, so it measures nothing: a
        /// reader that wants the imported body's volume measures through
        /// it ([`topo::AtRestBody::measure`]) under its own predicate,
        /// which reuses every round the gate ran and pays only the rest,
        /// instead of calling [`topo::mass_properties`] again.
        ///
        /// The certificate's band is `Band::linear` of the import's
        /// `tol`, and its lane the `f64` certified quadrature.
        body: topo::AtRestBody<f64>,
        /// The import's input tolerance ε_in (meters): the override if
        /// given, else the file's declared uncertainty.
        eps_in: f64,
        /// The structure normalizations the adoption applied, in
        /// resolution order — empty for a file whose boundary graph the
        /// kernel represents as stated (every own-corpus file).
        normalizations: Vec<StructureNormalization>,
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
        /// **The chart-coherence examination of `body`**, when
        /// [`ImportOptions::examine_chart_coherence`] asked for one:
        /// [`topo::examine_chart_coherence`]'s report, whole and
        /// unfiltered, run on the body that ships at the import's
        /// `tol`.
        ///
        /// This is the door where defective source COORDINATES
        /// arrive. Every condition it states is a gap between two of
        /// the FILE's own statements about one chart coordinate,
        /// measured in metres against the band — a meridian arc whose
        /// carrier midpoint sat a half-turn from its own endpoint was
        /// the recorded witness, before import made the pole it crossed
        /// a vertex.
        ///
        /// **`None` is CONFIGURATION and `unexamined` is DATA, and
        /// they are two different facts.** `None` says the caller did
        /// not ask, which nothing about the file changes and which
        /// setting the option reverses; `Some(report)` whose
        /// `unexamined` is non-empty says the examination RAN and the
        /// body put a loop out of its reach, which no option changes.
        /// Spelling the unasked state as an empty report would fold
        /// the two — a caller who never asked would read identically
        /// to a body with nothing to report, which is the one thing
        /// this channel may not say. The kernel door draws the same
        /// line about its own two lists at
        /// [`topo::CoherenceReport::unexamined`], and the `Option` is
        /// how it survives the crossing.
        ///
        /// **There is deliberately no `StepImport::coherence()`
        /// accessor** beside [`StepImport::normalizations`] and its
        /// neighbours. Those fold a wireframe into an empty slice
        /// honestly — a wireframe really did apply no normalizations —
        /// but this crate can be in THREE states about chart
        /// coherence: not asked, examined, and no body to examine at
        /// all. An accessor returning `Option<&CoherenceReport>` would
        /// spell the first and the third the same way. The variant is
        /// where the third one lives, so a caller matches the variant.
        ///
        /// **Nothing gates on it.** The at-rest gate above has already
        /// run and passed by the time this is measured; a body with
        /// findings ships exactly as a body without them does. A
        /// finding is a measurement, and reading it as a verdict is
        /// reading intent into a number.
        coherence: Option<topo::CoherenceReport>,
        /// **The contacts between the body's solids**: one record per
        /// touch the at-rest census found between two of `body`'s
        /// solids, each citing its row in `coincidences` and confirmed
        /// by the census like any other record ([`topo::record_at_rest`]).
        /// An imported assembly whose parts touch certifies (D7 step 4),
        /// and these are what a later gate over `body` confirms rather
        /// than finds again.
        contacts: topo::ContactRecords,
        /// **The census's decisions `contacts` cite**, in the order it
        /// made them ([`topo::DecisionSite::CensusAtRest`]), each a Zero
        /// of the file's values: no imported cell is a construction of a
        /// document's variables, so each is an `unproven-coincidence`
        /// finding where a document reads it (D7, D10).
        coincidences: Vec<topo::Coincidence>,
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
        /// `normalizations` below
        /// ([`NormalizationKind::CurvePromoted`]). Recognition is a property of the
        /// carrier, not of what a body later does with it; a wireframe
        /// whose promotions were silent would be the one place in this
        /// crate where a description changed without being reported.
        curves: Vec<geom::Curve3<f64>>,
        /// As [`StepImport::Solid::eps_in`].
        eps_in: f64,
        /// The move ledger: the curve promotions applied to `curves`
        /// (#327), by ascending curve entity id, and the placements
        /// their frames were built from where the build moved them.
        normalizations: Vec<StructureNormalization>,
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

    /// **The move ledger**: one record per file entity interpretation
    /// moved beyond rounding or re-minted, naming the entity, what moved
    /// and a bound in metres: every curve promotion first, by ascending
    /// curve entity id, then the rest in the order the import made them.
    pub fn normalizations(&self) -> &[StructureNormalization] {
        match self {
            Self::Solid { normalizations, .. } | Self::Wireframe { normalizations, .. } => {
                normalizations
            }
        }
    }

    /// The curve promotions this import applied (#327), read off the
    /// ledger ([`NormalizationKind::CurvePromoted`]). Both lanes report:
    /// recognition fires at the sole `NurbsCurve3` construction site,
    /// which the `GEOMETRIC_CURVE_SET` lane reaches as well as the solid
    /// lane.
    pub fn curve_promotions(&self) -> Vec<CurvePromotion> {
        self.normalizations()
            .iter()
            .filter_map(|n| match n.kind {
                NormalizationKind::CurvePromoted { kind, residual } => Some(CurvePromotion {
                    curve: n.face,
                    kind,
                    residual,
                }),
                _ => None,
            })
            .collect()
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
/// refuses ([`StepImportError::TierInvalid`]). A body the gate admits
/// whose volume is wide at this ε is NOT an error: it imports, and its
/// caller measures it through `StepImport::Solid`'s `certificate`. Files written by
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
    let model = entities::resolve(&file, options.eps_in, tol)?;
    let eps_in = options.eps_in.unwrap_or(model.uncertainty_m);
    match model.shape {
        entities::Shape::Solids(ref solids) => {
            // **Materialization** (M8 instancing). The model says what
            // to make: one entry per placed INSTANCE, each naming a
            // solid and the frame that copy sits in. N occurrences of
            // one component representation are N entries over the same
            // solid index, and each is a body of its OWN — fresh
            // topology, fresh arena keys, no structure shared between
            // copies. Sharing was never on the table: a `SolidSpec`'s
            // maps are keyed by the file's entity ids, so two copies
            // assembled into one arena would collide id for id.
            //
            // Each placement is the motion its placement pair minted
            // (`build_axes`, then `B ∘ A⁻¹` composed as motions), so it is
            // rigid by construction, and the per-solid finish below moves a
            // finished copy by it.
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
            // The joins each solid's build ended with, reported once per
            // solid: each is built once.
            let mut normalizations = ledger(&model);
            // Each distinct solid is built once. Where the per-solid
            // subject is owed, it is finished once, on its first placed
            // instance (so it is asked in the box where it stands), and
            // that finished copy is moved to every other instance: a
            // finished body moved by an isometry stays finished and asks
            // only the session box again.
            let mut finished: std::collections::BTreeMap<
                usize,
                (topo::AtRestBody<f64>, Option<geom_core::RigidMotion<f64>>),
            > = std::collections::BTreeMap::new();
            for (index, instance) in model.instances.iter().enumerate() {
                let spec = &solids[instance.solid];
                let file = geom_core::FileCoincidence::new(eps_in);
                let motion = match instance.placed {
                    Some(entities::Placed {
                        map: Some(motion),
                        transform,
                        ..
                    }) => Some((motion, transform)),
                    _ => None,
                };
                // The motion from the copy first finished to this one, a
                // product of minted motions: only an overflow refuses.
                let not_finite = |transform: u64| StepImportError::Structure {
                    id: transform,
                    what: "an assembly placement whose motion from the copy first finished is \
                           not a finite number — its placements stand too far from the origin \
                           to compose",
                };
                let one =
                    match finished.get(&instance.solid) {
                        Some((source, at)) => {
                            let relative = |m: &geom_core::RigidMotion<f64>| {
                                at.map_or(Ok(*m), |at| at.inverse().and_then(|i| m.compose(i)))
                            };
                            let relative = match (&motion, at) {
                                (None, None) => None,
                                (Some((m, transform)), _) => {
                                    Some(relative(m).map_err(|_| not_finite(*transform))?)
                                }
                                (None, Some(at)) => Some(at.inverse().map_err(|_| {
                                    not_finite(instance.placed.map_or(0, |p| p.transform))
                                })?),
                            };
                            match relative {
                                None => source.clone(),
                                Some(r) => topo::FinishedBody::moved(source, &r, tol).map_err(
                                    |findings| StepImportError::TierInvalid {
                                        solid: Some(spec.id),
                                        errors: findings.into_findings(),
                                    },
                                )?,
                            }
                            .into_body()
                        }
                        None => {
                            let (built, joins) = assemble::build_one_solid(spec, tol, file)?;
                            normalizations.extend(joins);
                            let one =
                                match &motion {
                                    Some((m, transform)) => topo::transform_rigid(&built, m)
                                        .map_err(|source| StepImportError::Placement {
                                            transform: *transform,
                                            source,
                                            file,
                                        })?,
                                    None => built,
                                };
                            if topo::per_part_gate_owed(model.instances.len()) {
                                let one = gate(one, Some(spec.id), tol)?;
                                finished.insert(
                                    instance.solid,
                                    (one.clone(), motion.as_ref().map(|(m, _)| *m)),
                                );
                                one.into_body()
                            } else {
                                one
                            }
                        }
                    };
                topo::graft_disjoint(&mut body, &one).map_err(|source| {
                    StepImportError::Instance {
                        solid: spec.id,
                        source: Box::new(source),
                    }
                })?;
                // The A7 record, minted where the instance is: `index`
                // is the graft order, and the graft appends one solid
                // per call, so it IS the shipped body's `solids()`
                // order (pinned by `freecad.rs`'s
                // `the_assembly_record_retains_the_occurrence_structure`
                // and `the_assembly_record_covers_a_file_that_places_nothing`).
                record.push(PlacedInstance {
                    index,
                    solid: spec.id,
                    component: instance.component,
                    occurrence: instance.placed.and_then(|p| p.occurrence),
                    relationship: instance.placed.map(|p| p.relationship),
                    transform: instance.placed.map(|p| p.transform),
                    placement: motion.map(|(m, _)| m.to_affine()),
                });
            }
            // **The shared at-rest validation gate** (M7-7, the #260
            // ruling (a) + D9 engineering convention 2). Every
            // imported solid is held to the kernel's invariants by the
            // SAME function every native door's result goes through —
            // the scalar's finish (`topo::finish`), tiers 1–3 — reached only
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
            // Asked for two subjects, and when the first is asked at
            // all is `topo::per_part_gate_owed`'s to say, with its
            // reason (one solid is one subject, not two); why it is
            // asked is `docs/DESIGN.md` import step 4. Here the per-solid
            // subject is each INSTANCE's own body, which is exactly the
            // body the materialization loop above already holds, and
            // the refusal names which `MANIFOLD_SOLID_BREP` it came
            // from; the aggregate subject is the shipped body.
            //
            // The per-solid subject is the PLACED copy (M8): the body
            // that ships is the union of exactly these. The first copy
            // of each solid is finished where it stands, and every other
            // is that finished copy moved, which carries its verdict and
            // asks the session box where the copy stands.
            //
            // **The 3′ form, recording what it decides** (D7 step 4,
            // D1 (iii)): the aggregate body is finished, then held to
            // the census against its records — the SAME gate a native
            // product's aggregate runs — through the census's recording
            // door, each solid its own group: the file's instance
            // structure is not recoverable from its solids, so a solid
            // is the unit a touch is between. A touch between two solids
            // is a row and a record citing it, confirmed on the census's
            // next pass, so an imported assembly whose parts touch
            // certifies; a touch inside one solid is its defect, and
            // two solids that overlap refuse. The per-solid gates above stay
            // tier 3: contact is an aggregate-body fact, and the
            // aggregate census sweeps every entity of every instance.
            let (body, contacts, coincidences) = gate3(body, tol)?;
            // **The chart-coherence channel** — after the gate, on the
            // body that ships, and consuming nothing. The door is a
            // pure function of (body, ε) that refuses nothing and
            // returns a measurement; running it here rather than on
            // some intermediate is what makes the report a statement
            // about the value the caller receives. It is called
            // directly, with no lane trait in front of it: every door
            // in this crate is monomorphic at `f64`, so there is no
            // scalar for which the examination could be absent.
            let coherence = options
                .examine_chart_coherence
                .then(|| topo::examine_chart_coherence(&body, tol));
            Ok(StepImport::Solid {
                body,
                eps_in,
                normalizations,
                instances: record,
                coherence,
                contacts,
                coincidences,
            })
        }
        entities::Shape::Wireframe(ref curves) => Ok(StepImport::Wireframe {
            curves: curves.clone(),
            eps_in,
            normalizations: ledger(&model),
        }),
    }
}

/// The move ledger as the model's resolution left it: each curve
/// promotion ([`NormalizationKind::CurvePromoted`], keyed by the carrier),
/// then the normalizations in resolution order.
fn ledger(model: &entities::Model) -> Vec<StructureNormalization> {
    model
        .curve_promotions
        .iter()
        .map(|p| StructureNormalization {
            face: p.curve,
            kind: NormalizationKind::CurvePromoted {
                kind: p.kind,
                residual: p.residual,
            },
            file_census: FaceCensus::EMPTY,
            kernel_census: FaceCensus::EMPTY,
        })
        .chain(model.normalizations.iter().copied())
        .collect()
}

/// The reader's per-solid contact with the kernel's at-rest gate: the
/// scalar's one finish ([`topo::finish`]), the same gate every door's
/// result goes through. Pass a body, get it back finished or the
/// finish's findings back whole as a typed refusal naming the subject
/// (`solid` = the file's `MANIFOLD_SOLID_BREP` asked about alone).
/// Nothing is filtered, nothing is reworded, no verdict class is
/// privileged — the whole point is that import has no validation logic
/// that could drift from the kernel's (D9 engineering convention 2). If
/// this function ever grows a condition, the gate has grown an opinion.
fn gate(
    body: topo::Body<f64>,
    solid: Option<u64>,
    tol: Tol,
) -> Result<topo::AtRestBody<f64>, StepImportError> {
    topo::finish(body, tol).map_err(|findings| StepImportError::TierInvalid {
        solid,
        errors: findings.into_findings(),
    })
}

/// The aggregate subject's gate: the tier-3′ form, recording the
/// contacts it finds between the body's solids
/// ([`topo::record_at_rest`], each solid its own group) — the same
/// function a caller runs over a native body's contact records, with the
/// same no-opinion contract as [`gate`] — followed by the measurement
/// the reader ships.
///
/// The body is finished first ([`gate`], the scalar's one finish), so
/// what ships is the finished body, holding the certificate its check 7
/// decided on: each solid's SIGN, certified per solid and assembled over
/// the whole face arena, one read of each face, for its reader to
/// measure through ([`topo::AtRestBody::measure`]). The census then runs
/// over the finished body alone ([`topo::AtRestPolicy::census_at_rest`]),
/// as the product's does over its aggregate. The verdicts are the
/// gate's, verbatim.
///
/// **What the import as a whole pays is more than this gate.** Where
/// [`topo::per_part_gate_owed`] asks for it, each placed solid is gated
/// on its own first ([`gate`], tier 3), so every face of an assembly is
/// read once there and once again here: a two-instance assembly of
/// certifying solids records twice one measurement's quadrature
/// verdicts through `import_step`. Where it does not, the import pays
/// this gate only, once; the census runs twice where its solids touch:
/// the second pass confirms the records the first one's touches minted.
#[allow(clippy::type_complexity)] // the gate's three products, read once at its one caller
fn gate3(
    body: topo::Body<f64>,
    tol: Tol,
) -> Result<
    (
        topo::AtRestBody<f64>,
        topo::ContactRecords,
        Vec<topo::Coincidence>,
    ),
    StepImportError,
> {
    let finished = gate(body, None, tol)?;
    let instance: std::collections::BTreeMap<topo::SolidKey, u32> =
        finished.solids().map(|(solid, _)| solid).zip(0..).collect();
    let recorded = topo::record_at_rest(
        &finished,
        &topo::ContactRecords::default(),
        |records| <f64 as topo::AtRestPolicy>::census_at_rest(&finished, records, tol),
        &|solid| instance.get(&solid).copied(),
        0,
    );
    let (records, rows) = gate3_findings(&finished, tol, &instance, recorded)?;
    Ok((finished, records, rows))
}

/// The aggregate gate's findings over `recorded`, the flush pairs between
/// instances included: its records and rows where there are none, else
/// the gate's refusal.
fn gate3_findings(
    body: &topo::Body<f64>,
    tol: Tol,
    instance: &std::collections::BTreeMap<topo::SolidKey, u32>,
    recorded: topo::RecordedAtRest,
) -> Result<(topo::ContactRecords, Vec<topo::Coincidence>), StepImportError> {
    // The flush pairs between instances are rows of their own, which no
    // record cites.
    let flush = match geom_core::Band::linear(tol) {
        Ok(band) => <f64 as topo::AtRestPolicy>::flush_at_rest(
            body,
            &|solid| instance.get(&solid).copied(),
            band,
        ),
        Err(error) => topo::FlushAtRest {
            findings: vec![topo::ValidationError::Band { error }],
            ..topo::FlushAtRest::default()
        },
    };
    let mut findings = recorded.findings;
    findings.extend(flush.findings);
    if findings.is_empty() {
        let mut rows = recorded.rows;
        rows.extend(flush.rows);
        Ok((recorded.records, rows))
    } else {
        Err(StepImportError::TierInvalid {
            solid: None,
            errors: findings,
        })
    }
}
