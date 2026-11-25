//! Typed import failure (closed enum, D4 ¶3). Every variant names its
//! subject as precisely as the failure has one — the offending entity
//! id or line where the failure is a FILE's, the anchor or the
//! verdicts where it is not — and the reader never panics and never
//! silently degrades a file into a different model.

use core::fmt;

use geom_brep::recourse::{LeverOnly, RefusedArm, SizedDecision};
use geom_core::{FileCoincidence, Indeterminate};

/// One certification of an edge's built description, as data: which
/// intensional interpretation was stated, and the kernel gate's typed
/// refusal.
#[derive(Debug)]
pub struct AdoptionAttempt {
    /// The interpretation attempted.
    pub candidate: AdoptionCandidate,
    /// The certification/attachment gate's refusal, rendered at the
    /// import door with the file's ε_in (`topo::EulerOpError::render`).
    pub refusal: topo::EulerOpError,
}

/// The intensional interpretations an edge's built description can state
/// (D7 stage 2's vocabulary — `geom_brep::EdgeDescription`'s variants,
/// named as data for future remedy flows).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdoptionCandidate {
    /// Transverse intersection of the two adjacent surfaces.
    Intersection,
    /// Tangential contact locus of the two adjacent surfaces.
    TangentIntersection,
    /// A wrap edge: the edge both of whose uses bound one face, across
    /// which that face's periodic chart closes (D1).
    Wrap,
    /// A NURBS wall's own `u ∈ {0, 1}` boundary iso-curve (M7-3): the
    /// loft/sweep wall–wall seam class, offered when the parsed
    /// carrier bitwise-matches an adjacent wall's boundary column.
    IsoCurve,
    /// A conventional mapped-curve self-description (the locus the
    /// surfaces under-determine).
    MappedCurve,
}

impl fmt::Display for AdoptionCandidate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Intersection => "intersection",
            Self::TangentIntersection => "tangent intersection",
            Self::Wrap => "wrap edge",
            Self::IsoCurve => "boundary iso-curve",
            Self::MappedCurve => "mapped curve",
        })
    }
}

/// What [`StepImportError::BuildEscalated`] asks.
pub(crate) const BUILD_SUBJECT: &str =
    "where an edge's surfaces meet, read while building its carrier or its ends";

/// **Where an edge's surfaces meet, read at the run's band**, the build's
/// sized decision of it: a reading of the run's ε that the band does not
/// call (two surfaces nearly tangent, an end nearly at a third surface)
/// refuses, and moving the geometry clear of it, or a smaller tolerance
/// that decides it, passes ([tolerance.recourse]).
pub(crate) const BUILD_READING: SizedDecision = SizedDecision {
    lever: "move the edge's surfaces so they clearly cross, clearly part, or touch",
    size: "distance",
    passes: geom_brep::recourse::SizedPass::AnySign,
    side: geom_brep::recourse::RefusedSide::Lever,
    no_size: None,
};

/// What [`StepImportError::SingularityUndecided`] asks.
pub(crate) const SINGULARITY_SUBJECT: &str = "whether a sphere pole or cone apex, or where a meridian minted from one meets a face's \
     boundary, lies on that boundary or at one of its vertices";

/// What [`StepImportError::SingularitySideUndecided`] asks.
pub(crate) const SINGULARITY_SIDE_SUBJECT: &str =
    "which side of a face's boundary a sphere pole or cone apex lies on";

/// **Whether a chart singularity lies on a face's boundary**, the
/// kernel's sized decision of it
/// ([`geom_brep::chart_singularity::ON_BOUNDARY`]), which tier 3 ends
/// the same refusal by.
pub(crate) const SINGULARITY_ON_BOUNDARY: SizedDecision = geom_brep::chart_singularity::ON_BOUNDARY;

/// The same decision where the FILE's band could not call it (`(ε_in,
/// K·ε_in)`, coarser than the run's): no smaller tolerance decides it,
/// so it ends in its lever and the ε_in that would decide it
/// ([`crate::interpret::Refusal::Ambiguous`]), or in its lever alone where
/// the margin is no point value.
pub(crate) const SINGULARITY_IN_FILE_BAND: LeverOnly = LeverOnly {
    lever: "move the face's boundary, or its vertex, onto that point or clear of it, or, if \
            this distance is intended, re-export the file with a smaller declared \
            uncertainty (or pass a smaller ε_in override): the distance lies in the band past \
            the file's coincidence distance, which no tolerance of this run decides",
};

/// The singularity decision's move, which its file-band ending names
/// beside the ε_in that decides it.
const SINGULARITY_MOVE: &str =
    "move the face's boundary, or its vertex, onto that point or clear of it";

/// Which band a [`StepImportError::SingularityUndecided`] could not call
/// its distance in.
///
/// **Not carried by the `pncad` façade, argued.** It picks the refusal's
/// ending, and the ending is the whole of what it changes for a caller:
/// a smaller tolerance decides a `Run` reading, a finer ε_in a `File`
/// one, and the rendered message names the lever for each. The façade
/// carries the variant (and Python its word, `singularity_undecided`),
/// which is what a caller routes on. The falsifier: a caller that must
/// act on WHICH knob decides it without reading the message (a GUI
/// offering the ε_in override in place of a tolerance) — then the carry
/// is `pncad::prelude`'s export, the façade's matchability row, and a
/// Python attribute beside `variant`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SingularityBand {
    /// The run's own band: a smaller tolerance decides it.
    Run,
    /// The file's band, `(ε_in, K·ε_in)`: a finer ε_in decides it.
    File,
}

/// **Which kind of entity a build refusal names** (`[import]`, D7 step
/// 2): an edge whose carrier, or a vertex whose point, the build does not
/// construct.
///
/// **Not carried by the `pncad` façade, argued.** It picks the word the
/// rendered message names the entity by ("edge #id", "vertex #id"), and
/// the message is the whole of what it changes for a caller: the id is
/// carried, and the recourse (the lane, or the run's tolerance) does not
/// depend on the kind. The façade carries the variants (and Python their
/// words, `not_yet_supported`, `build_escalated`), which is what a caller
/// routes on. The falsifier: a caller that must act on WHICH kind of
/// entity refused without reading the message (a GUI highlighting the
/// vertex rather than the edge) — then the carry is `pncad::prelude`'s
/// export, the façade's matchability row, and a Python attribute beside
/// `variant`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuiltEntity {
    /// An `EDGE_CURVE`.
    Edge,
    /// A `VERTEX_POINT`.
    Vertex,
}

impl core::fmt::Display for BuiltEntity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Edge => "edge",
            Self::Vertex => "vertex",
        })
    }
}

/// Typed import failure. `#id` fields are Part 21 entity instance
/// ids; `line` fields are 1-based line numbers in the input text.
#[derive(Debug)]
pub enum StepImportError {
    /// The Part 21 exchange structure is malformed: bad token, missing
    /// section, unterminated record. The line and a static description
    /// say what was expected.
    Syntax {
        /// 1-based line number of the offending text.
        line: usize,
        /// What the tokenizer/parser expected (static description).
        expected: &'static str,
    },
    /// An entity instance references an id no record defines — a
    /// dangling reference, named.
    DanglingReference {
        /// The referencing entity instance.
        from: u64,
        /// The undefined id it references.
        to: u64,
    },
    /// An entity record exists but has the wrong type for the slot
    /// that references it (e.g. an `EDGE_CURVE` where a
    /// `CARTESIAN_POINT` is required).
    WrongEntityType {
        /// The referenced entity instance.
        id: u64,
        /// The type the referencing slot requires (static name).
        expected: &'static str,
        /// The type the record actually has.
        found: String,
    },
    /// An entity record's argument list does not match its schema
    /// shape (arity, argument kind).
    MalformedRecord {
        /// The offending entity instance.
        id: u64,
        /// What was expected (static description).
        expected: &'static str,
    },
    /// An entity type outside the imported subset appears where the
    /// reader must interpret it. The subset is what the kernel's own
    /// writer emits — since M7-3 that includes both
    /// `B_SPLINE_SURFACE_WITH_KNOTS` arms (the old named M7 frontier,
    /// retired by the S9 flip exactly as this doc predicted) — plus
    /// the knots-implied `QUASI_UNIFORM_CURVE`/`_SURFACE` sub-types,
    /// whose clamped knot vectors are synthesized closed-form (D7
    /// stage 1). Other spline sub-types (`UNIFORM_CURVE`, Bézier
    /// forms) stay here, typed.
    UnsupportedEntity {
        /// The offending entity instance.
        id: u64,
        /// The unsupported type name.
        keyword: String,
    },
    /// The file's unit context is outside the subset the kernel
    /// understands. Parsed, not assumed. The subset resolves any of the
    /// sixteen ISO 10303-41 SI prefixes on `.METRE.`, unprefixed
    /// `.RADIAN.`/`.STERADIAN.`, and `CONVERSION_BASED_UNIT` chains
    /// against those bases (`crate::units` module docs); a **prefixed**
    /// angle unit, a scaled steradian, and any other unit context
    /// refuse here rather than being guessed.
    UnsupportedUnit {
        /// The offending unit entity instance.
        id: u64,
        /// The unit text the subset does not cover.
        found: String,
    },
    /// The data section declares no usable shape representation (no
    /// `MANIFOLD_SOLID_BREP` and no `GEOMETRIC_CURVE_SET`).
    NothingToImport,
    /// The data section's shape-representation structure is outside
    /// the subset: mixed solid+wireframe content, a second curve set,
    /// or a solid/curve-set no representation references (an orphan —
    /// refused rather than guessed in or silently dropped).
    Structure {
        /// The offending entity instance.
        id: u64,
        /// The structural defect (static description).
        what: &'static str,
    },
    /// The header/context carries no
    /// `UNCERTAINTY_MEASURE_WITH_UNIT`, so ε_in has no file default
    /// and no override was given.
    MissingUncertainty,
    /// An explicit [`crate::ImportOptions::eps_in`] override is not
    /// finite and strictly positive.
    InvalidEpsOverride {
        /// The rejected value.
        value: f64,
    },
    /// The import's ε_in (the file's declared uncertainty, or the
    /// override) makes no band the file can be read at: `K·ε_in` is no
    /// finite length.
    UnbandedUncertainty {
        /// ε_in, in metres.
        eps_in: f64,
        /// Whether ε_in is the caller's override
        /// ([`crate::ImportOptions::eps_in`]) rather than the file's
        /// declaration.
        overridden: bool,
    },
    /// A real token failed to parse as an f64.
    MalformedReal {
        /// The entity instance carrying the token.
        id: u64,
        /// The offending token text.
        token: String,
    },
    /// The topology records do not describe an assemblable closed
    /// 2-manifold shell (the message names the structural defect and
    /// the entity).
    Topology {
        /// The offending entity instance.
        id: u64,
        /// The structural defect (static description).
        what: &'static str,
    },
    /// An Euler-operator call refused during assembly — corrupt or
    /// non-manifold input surfaced through the kernel's own gates.
    Assembly {
        /// The entity instance being assembled when the op refused.
        id: u64,
        /// The refusing operator's error, displayed.
        source: topo::EulerOpError,
        /// The file's ε_in, which picks the words of a certification
        /// refusal's ending (D4 ¶1).
        file: FileCoincidence,
    },
    /// The edge's built description does not certify: the description
    /// stated and its typed refusal — **structured data, not prose**, so
    /// a future remedy flow (GUI-offered healing, M7-2+) can act on what
    /// failed without re-parsing messages.
    Adoption {
        /// The `EDGE_CURVE` entity instance.
        id: u64,
        /// The description stated and its refusal (one attempt: the build
        /// chooses one description per edge).
        attempts: Vec<AdoptionAttempt>,
        /// The file's ε_in, which picks the words of each refusal's
        /// ending (D4 ¶1).
        file: FileCoincidence,
    },
    /// A described NURBS wall's own boundary column would not re-wrap
    /// as a curve while an edge was being adopted against it: a weight
    /// on the extracted column is not a positive finite number, which
    /// a surface that passed `geom::NurbsSurface::new` cannot hold. Unreachable from any body
    /// this reader assembles, and surfaced rather than swallowed (D4
    /// ¶2, and [`geom_brep::boundary_iso_u`]'s own `# Errors`
    /// contract): the payload says WHICH structural invariant the wall
    /// broke, and a discarded one would leave a kernel-bug report
    /// saying only that a kernel bug happened.
    ///
    /// It is **not** a "this edge is not that shape" answer. The
    /// recognizers state their negatives some other way — a wall that
    /// is not a described NURBS, a column the parsed carrier does not
    /// match bitwise, a rim whose metered deviation exceeds the
    /// ambient tolerance — and each of those returns a candidate list
    /// or a residual, never this.
    WallColumnStructure {
        /// The `EDGE_CURVE` entity instance being adopted.
        id: u64,
        /// The extraction door's refusal, carried rather than
        /// discarded: which count or weight the wall's stored net
        /// broke. A count is the whole net's; a weight's `index`
        /// counts along the extracted COLUMN, not through the net.
        source: geom_core::spline::Built<geom_core::spline::SplineError>,
    },
    /// D7's typed ambiguity at ε_in (stage-1 surface recognition,
    /// ruling #256): a face that cannot import WITHOUT promotion (a
    /// multi-bound curved face — the trim-ring class) sits on a NURBS
    /// surface whose recognition estimator is ill-conditioned by its
    /// own margin trilean (e.g. a cylinder axis from a patch whose
    /// azimuth samples are collinear within ε_in). No answer exists at
    /// the interpretation budget, so the refusal names the condition
    /// rather than guessing a kind or falling back to the bare
    /// topology refusal. An IMPORTABLE face with the same marginal
    /// recognition stays NURBS instead — this variant fires only where
    /// promotion was the face's only door.
    RecognitionAmbiguous {
        /// The `ADVANCED_FACE` entity instance that needed promotion.
        id: u64,
        /// The surface entity instance under recognition.
        surface: u64,
        /// The kind whose estimator declined.
        kind: crate::PromotedKind,
        /// The estimator's conditioning margin (meters) that fell
        /// inside ε_in.
        margin: f64,
    },
    /// Pcurve re-minting refused on the adopted body.
    Pcurves {
        /// The minting pass's error, displayed.
        source: topo::PcurveMintError,
        /// The file's ε_in: the import door the refusal is read at.
        file: FileCoincidence,
    },
    /// The join every finisher ends with (`docs/DESIGN.md`, maximal
    /// edges; Ev's PR 4251 ruling) refused on the adopted solid: where
    /// the file states two edges of one carrier meeting at a vertex, the
    /// kernel joins them, and here it could not — most often because
    /// whether the vertex is a regular point reads in the band, a size
    /// finer than the run's tolerance.
    Join {
        /// The `MANIFOLD_SOLID_BREP` whose join refused.
        solid: u64,
        /// Why the join refused.
        refusal: topo::JoinRefusal,
        /// The file's ε_in: the import door the refusal is read at.
        file: FileCoincidence,
    },
    /// Whether a sphere pole or cone apex lies on a face's boundary (at
    /// one of its vertices, or inside one of its edges), or whether a
    /// wrap edge minted to one lands at a boundary vertex or
    /// beside it, reads in the band
    /// the import decides it at, the coarser of the file's and the run's:
    /// so whether it is a vertex of the face, lies inside an edge, or lies
    /// inside the face is undecided, and the normalization that makes it a
    /// vertex ([`crate::NormalizationKind::ChartSingularityVertex`])
    /// refuses rather than guess. The distance is a size the file may
    /// intend, so the refusal ends as the import door ends a sized
    /// decision ([`SINGULARITY_ON_BOUNDARY`]).
    SingularityUndecided {
        /// The `ADVANCED_FACE` whose boundary the point lies near.
        id: u64,
        /// The decision's escalation: the distance, and the band.
        cause: Indeterminate,
        /// Which band could not call it.
        band: SingularityBand,
        /// The file's ε_in: the import door the refusal is read at.
        file: FileCoincidence,
        /// The run's ε (metres), which floors the band the file is read at.
        run_eps: f64,
    },
    /// Which side of a face's boundary a sphere pole or cone apex lies on,
    /// the point being off that boundary, cannot be read: its reading does
    /// not clear its own error bound, or a band decision of it is in band
    /// (`geom_brep::chart_singularity::SideUnread`). Whether the point lies
    /// inside the face, and so whether the normalization that makes it a
    /// vertex ([`crate::NormalizationKind::ChartSingularityVertex`]) owes
    /// the face a cut, is undecided, and it refuses rather than guess. The
    /// side is no distance the file's band or the run's decides, so the
    /// refusal names neither: it ends as the kernel ends the side
    /// ([`geom_brep::chart_singularity::LOOP_SIDE`]).
    SingularitySideUndecided {
        /// The `ADVANCED_FACE` whose boundary the point lies beside.
        id: u64,
        /// Why the side is unread.
        side: geom_brep::chart_singularity::SideUnread,
        /// The file's ε_in: the import door the refusal is read at.
        file: FileCoincidence,
    },
    /// Where a wrap edge's meridian minted from a sphere pole or cone apex
    /// meets a face's boundary did not run to an answer: the kernel's
    /// meeting reading ([`geom_brep::curve_pair::meet`]) of the meridian
    /// and a boundary edge stopped at a limit with no margin to rank it
    /// (a root finder or a walk that did not settle, or a pose with no
    /// closed form). Where the meridian ends is unread, so the
    /// normalization that makes the point a vertex
    /// ([`crate::NormalizationKind::ChartSingularityVertex`]) refuses
    /// rather than guess; the refusal ends as the reading's own standing
    /// does ([`geom_brep::curve_pair::MeetUnread::recourse`]).
    SingularityMeetUnread {
        /// The `ADVANCED_FACE` the meridian is minted in.
        id: u64,
        /// The reading that did not run to an answer.
        why: geom_brep::curve_pair::MeetUnread,
        /// The file's ε_in: the import door the refusal is read at.
        file: FileCoincidence,
    },
    /// **An interpretation reading refused** (`[import]`, D7): what the
    /// file's statements at `id` are evidence of is no description at
    /// its band, or more than one, or a value no construction takes.
    Interpretation {
        /// The file entity the reading is of.
        id: u64,
        /// The reading, the distance and which way it refused.
        refusal: crate::interpret::Refusal,
        /// The file's ε_in: the import door the refusal is read at.
        file: FileCoincidence,
    },
    /// An assembly instance's rigid placement refused at the kernel's
    /// own [`topo::transform_rigid`] door (M7-4 Leg D): a carrier that
    /// failed to re-certify against the mapped geometry.
    Placement {
        /// The `ITEM_DEFINED_TRANSFORMATION` that states this
        /// instance's frame — with N instances in a file, which one
        /// refused is the whole question.
        transform: u64,
        /// The transform op's error, displayed.
        source: topo::TransformError,
        /// The file's ε_in: the import door the refusal is read at.
        file: FileCoincidence,
    },
    /// A placed assembly instance refused to graft into the result
    /// body at the kernel's own [`topo::graft_disjoint`] door — the
    /// transplant found the placed component ill-formed. That is a
    /// kernel-side refusal about a body this reader had already built
    /// and gated, and it is not silently absorbed.
    Instance {
        /// The `MANIFOLD_SOLID_BREP` whose instance was being placed.
        solid: u64,
        /// The graft's error, displayed.
        source: Box<topo::BooleanError>,
    },
    /// A face whose chart has an interior C⁰ knot whose iso line crosses
    /// the face, or is not certified clear of it, where the import
    /// door's split cannot place the line (`crate::knot_split`): every
    /// face chart is C¹ by its knot
    /// structure strictly inside its cell (`docs/DESIGN.md` `[topology]`),
    /// and the split needs the line to part the face's single loop at
    /// two points of its boundary, one chain on each sub-patch. A trim
    /// that crosses the line more than once, or a face with rings, needs
    /// the line's crossings with the trim, which the door cannot invert
    /// yet. The refusal names the face, the knot and what is missing.
    C0KnotUnsplit {
        /// The `ADVANCED_FACE` entity instance.
        face: u64,
        /// The crease knot's value, in the chart's own parameter.
        knot: f64,
        /// What the split lacks (static description).
        what: &'static str,
    },
    /// **An edge or vertex whose description the kernel cannot yet
    /// construct** (`[import]`, D7 step 3): the description the file's
    /// statements are evidence of has no construction in this kernel, so
    /// no cache can be built from it, and no value the file printed stands
    /// in.
    NotYetSupported {
        /// Which kind of entity `id` is.
        entity: BuiltEntity,
        /// The `EDGE_CURVE` entity instance, or the `VERTEX_POINT` the
        /// build does not construct.
        id: u64,
        /// What has no construction: the pair or kind and why.
        what: String,
        /// The lane that would construct it.
        lane: &'static str,
        /// The file's ε_in, the import door the refusal is read at.
        file: FileCoincidence,
    },
    /// A reading the build takes at the run's band escalated: the run's
    /// tolerance does not decide the construction of edge or vertex `id`.
    BuildEscalated {
        /// Which kind of entity `id` is.
        entity: BuiltEntity,
        /// The `EDGE_CURVE` or `VERTEX_POINT` entity instance.
        id: u64,
        /// The escalated reading.
        source: Indeterminate,
        /// The file's ε_in, the import door the refusal is read at.
        file: FileCoincidence,
    },
    /// The adopted body failed the kernel's shared at-rest validation
    /// gate (M7-7, the #260 ruling (a)): `topo::validate_geometric` —
    /// tiers 1–3, the SAME function on the SAME body that a native
    /// construction's caller runs at rest — refused the assembled
    /// solid.
    ///
    /// This is a **validity refusal about the file's geometry**, not
    /// the Corrupt-class kernel-bug voice: a STEP file may describe a
    /// body that violates the kernel's invariants (inside-out senses,
    /// negative volume, a carrier off its planar face), and D7's
    /// adoption contract is that such geometry is brought up to the
    /// invariants or refused loudly naming the unhealable. Every
    /// verdict is carried verbatim, each naming its failing check and
    /// the entities (and, where the check measures one, the margin)
    /// that failed it.
    ///
    /// Escalated / indeterminate verdicts refuse here too
    /// (escalate-never-guess): the gate's `Err` is the gate's answer
    /// whatever the verdict kinds, so import holds no opinion of its
    /// own about what "valid at rest" means.
    TierInvalid {
        /// The `MANIFOLD_SOLID_BREP` id of the solid asked about on its
        /// own, or `None` when the verdict is about the whole assembled
        /// body. A one-instance file is only ever asked as the whole
        /// body ([`topo::per_part_gate_owed`]), so its refusals carry
        /// `None`.
        solid: Option<u64>,
        /// The verdicts, verbatim: tiers 1–3 for a solid asked on its
        /// own, tiers 1–3′ (the contact-record census) for the
        /// assembled body.
        errors: Vec<topo::ValidationError>,
    },
}

impl StepImportError {
    /// **This refusal as the import door reports it**: each nested
    /// refusal ended at the import door, by the file's declared
    /// coincidence distance its variant carries. Its `Display` is the
    /// payload alone.
    ///
    /// It takes no reading and is not `geom_brep::recourse::Ended`: the
    /// import door is the only door it is read at, and that door's
    /// reading (`ReadAt::File` at the file's ε_in) is part of each
    /// variant's payload, so a caller has no reading to pass, and an
    /// `ended(at)` would have to ignore its argument.
    #[must_use]
    pub fn render(&self) -> String {
        fmt::from_fn(|f| self.write(f, true)).to_string()
    }

    /// The text, its nested refusals ended at the import door where
    /// `ended`, and their payloads alone otherwise (the `Display`).
    #[allow(clippy::too_many_lines)] // one sentence per arm
    fn write(&self, f: &mut fmt::Formatter<'_>, ended: bool) -> fmt::Result {
        use geom_brep::recourse::{ReadAt, Reading, read_at};
        let door = |file: &geom_core::FileCoincidence| ended.then_some(ReadAt::File(*file));
        match self {
            Self::Syntax { line, expected } => {
                write!(f, "step import: line {line}: expected {expected}")
            }
            Self::DanglingReference { from, to } => write!(
                f,
                "step import: entity #{from} references #{to}, which no record defines"
            ),
            Self::WrongEntityType {
                id,
                expected,
                found,
            } => write!(
                f,
                "step import: entity #{id} is {found}, where {expected} is required"
            ),
            Self::MalformedRecord { id, expected } => {
                write!(f, "step import: entity #{id}: expected {expected}")
            }
            Self::UnsupportedEntity { id, keyword } => write!(
                f,
                "step import: entity #{id} ({keyword}) is outside the imported subset \
                 (the subset the kernel exports — elementary analytic surfaces plus \
                 both B_SPLINE arms with stated knots — plus the knots-implied \
                 QUASI_UNIFORM sub-types; other foreign vocabulary refuses here, typed)"
            ),
            Self::UnsupportedUnit { id, found } => write!(
                f,
                "step import: entity #{id} declares unit {found} — the subset covers \
                 SI metres under any prefix, unprefixed radians/steradians, and \
                 CONVERSION_BASED_UNIT chains onto those (refused rather than guessed)"
            ),
            Self::NothingToImport => f.write_str(
                "step import: the data section carries no MANIFOLD_SOLID_BREP and no \
                 GEOMETRIC_CURVE_SET — nothing to import",
            ),
            Self::Structure { id, what } => {
                write!(f, "step import: entity #{id}: {what}")
            }
            Self::MissingUncertainty => f.write_str(
                "step import: no UNCERTAINTY_MEASURE_WITH_UNIT in the representation \
                 context and no per-call override — ε_in has no honest default",
            ),
            Self::InvalidEpsOverride { value } => write!(
                f,
                "step import: ε_in override {value} is not finite and strictly positive"
            ),
            Self::UnbandedUncertainty { eps_in, overridden } => {
                let (whose, recourse) = if *overridden {
                    ("the ε_in override", "pass a smaller ε_in override")
                } else {
                    (
                        "the file's declared uncertainty",
                        "re-export the file with a smaller declared uncertainty, or pass a \
                         smaller ε_in override",
                    )
                };
                write!(
                    f,
                    "step import: {whose} (the coincidence distance ε_in = {eps_in:e} m) makes \
                     no band the file can be read at: K·ε_in is no finite length"
                )?;
                if ended {
                    write!(f, ". Recourse: {recourse}")?;
                }
                Ok(())
            }
            Self::MalformedReal { id, token } => write!(
                f,
                "step import: entity #{id}: token '{token}' is not a Part 21 real"
            ),
            Self::Topology { id, what } => {
                write!(f, "step import: entity #{id}: {what}")
            }
            Self::Assembly { id, source, file } => write!(
                f,
                "step import: assembling entity #{id}: {}",
                read_at(source, door(file))
            ),
            Self::Adoption { id, attempts, file } => {
                write!(
                    f,
                    "step import: edge #{id}: no intensional description certifies (a lever \
                     applies to the model the file was exported from, which is then \
                     re-exported) — "
                )?;
                for (i, attempt) in attempts.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(
                        f,
                        "{}: {}",
                        attempt.candidate,
                        read_at(&attempt.refusal, door(file))
                    )?;
                }
                Ok(())
            }
            Self::WallColumnStructure { id, source } => write!(
                f,
                "step import: edge #{id}: an adjacent NURBS wall's control net, or the \
                 boundary column read from it, is not valid spline structure — {source}. \
                 No validated surface can be in \
                 that state, so the file is refused rather than adopted against a wall \
                 whose stored structure is corrupt"
            ),
            Self::RecognitionAmbiguous {
                id,
                surface,
                kind,
                margin,
            } => write!(
                f,
                "step import: face #{id}: recognition at ε_in is ambiguous — the face \
                 cannot import without promoting its NURBS surface #{surface}, and the \
                 {kind} estimator is ill-conditioned there (its own conditioning margin, \
                 {margin:e} m, falls inside ε_in): no interpretation exists at the \
                 interpretation budget, so the refusal is typed rather than guessed \
                 (D7's ambiguity contract)"
            ),
            Self::Pcurves { source, file } => write!(
                f,
                "step import: pcurve re-mint on the adopted body: {}",
                read_at(source, door(file))
            ),
            Self::C0KnotUnsplit { face, knot, what } => write!(
                f,
                "step import: face #{face}: its B-spline chart has a C⁰ knot at {knot:e} whose \
                 iso line crosses the face, or is not certified clear of it, where the import \
                 splits it so that no cell \
                 carries a crease, and the split cannot be placed: {what} \
                 (work/importgate/the-c0-knot-split-refuses-what-it-cannot-place.md)"
            ),
            Self::NotYetSupported {
                entity,
                id,
                what,
                lane,
                ..
            } => write!(
                f,
                "step import: {entity} #{id}: {what}, which the kernel does not yet construct \
                 (no value the file printed stands in for a construction); the lane that \
                 would: {lane}"
            ),
            Self::BuildEscalated {
                entity,
                id,
                source,
                file,
            } => {
                let ending =
                    door(file).map(|at| BUILD_READING.recourse(RefusedArm::Undecided(source), at));
                write!(
                    f,
                    "step import: {entity} #{id}: {}",
                    geom_brep::recourse::undecided_at(source, BUILD_SUBJECT, ending)
                )
            }
            Self::TierInvalid { solid, errors } => {
                // Both forms of each verdict: the Debug shape names
                // the failing CHECK and its entity keys structurally
                // (greppable, and what a caller matching on
                // `errors` sees); the Display prose says what the
                // check means and carries the margins the measured
                // checks report.
                // The at-rest gate's findings, ended there.
                let at_rest = ended.then_some(ReadAt::Run(Reading::AtRest));
                let verdicts: Vec<String> = errors
                    .iter()
                    .map(|e| format!("{e:?} — {}", read_at(e, at_rest)))
                    .collect();
                let subject = match solid {
                    Some(id) => format!("solid #{id}, asked about on its own,"),
                    None => "the assembled body".to_owned(),
                };
                write!(
                    f,
                    "step import: {subject} is a body the kernel's shared at-rest \
                     validation gate refuses ({} verdict{}): {} — every imported solid is \
                     held to the same tiers, by the same function, as a natively \
                     constructed one, so a body that is invalid at rest refuses at \
                     import rather than shipping the failure downstream",
                    errors.len(),
                    if errors.len() == 1 { "" } else { "s" },
                    verdicts.join("; ")
                )
            }
            Self::Join {
                solid,
                refusal,
                file,
            } => write!(
                f,
                "step import: joining the edges of the solid at #{solid}: {}",
                read_at(refusal, door(file))
            ),
            Self::SingularityUndecided {
                id,
                cause,
                band,
                file,
                run_eps,
            } => {
                use crate::interpret::Refusal;
                let arm = RefusedArm::Undecided(cause);
                // The ending that names every lever that decides it; the
                // run's own sized ending where a smaller tolerance alone
                // does (ε_in already below the distance over K).
                let ending = door(file).map(|at| {
                    match (band, Refusal::in_band(SINGULARITY_SUBJECT, cause, *run_eps)) {
                        (SingularityBand::Run, Some(Refusal::Ambiguous { gap, k, .. }))
                            if !crate::interpret::lever_decides(file.eps_in(), gap / k) =>
                        {
                            SINGULARITY_ON_BOUNDARY.recourse(arm, at)
                        }
                        (_, Some(refusal)) => refusal.ending_with(SINGULARITY_MOVE, file.eps_in()),
                        (SingularityBand::Run, None) => SINGULARITY_ON_BOUNDARY.recourse(arm, at),
                        (SingularityBand::File, None) => SINGULARITY_IN_FILE_BAND.recourse(arm, at),
                    }
                });
                write!(
                    f,
                    "step import: face #{id}: {}",
                    geom_brep::recourse::undecided_at(cause, SINGULARITY_SUBJECT, ending)
                )
            }
            Self::SingularitySideUndecided { id, side, file } => {
                use geom_brep::chart_singularity::{SideUnread, Undecided};
                let ending = door(file).map(|at| Undecided::Side(*side).recourse(at));
                match side {
                    SideUnread::Banded(cause) => write!(
                        f,
                        "step import: face #{id}: {}",
                        geom_brep::recourse::undecided_at(cause, SINGULARITY_SIDE_SUBJECT, ending)
                    ),
                    SideUnread::Straddle => write!(
                        f,
                        "step import: face #{id}: {SINGULARITY_SIDE_SUBJECT} cannot be read: \
                         the reading lies within its own error bound{}",
                        ending.map(|e| format!(". {e}")).unwrap_or_default()
                    ),
                }
            }
            Self::SingularityMeetUnread { id, why, file } => {
                write!(
                    f,
                    "step import: face #{id}: where a meridian minted from a sphere pole or \
                     cone apex meets the face's boundary is unread: {}",
                    why.sentence()
                )?;
                if door(file).is_some() {
                    write!(f, ". {}", why.recourse().render())?;
                }
                Ok(())
            }
            Self::Interpretation { id, refusal, file } => {
                write!(
                    f,
                    "step import: entity #{id}: {}",
                    refusal.payload(file.eps_in())
                )?;
                if ended {
                    write!(f, ". {}", refusal.ending(file.eps_in()))?;
                }
                Ok(())
            }
            Self::Placement {
                transform,
                source,
                file,
            } => write!(
                f,
                "step import: the assembly placement stated at #{transform} refused \
                 at the kernel’s transform door: {}",
                read_at(source, door(file))
            ),
            Self::Instance { solid, source } => write!(
                f,
                "step import: an assembly instance of the solid at #{solid} refused \
                 to graft into the result body at the kernel’s disjoint-graft door: \
                 {}",
                // The graft's own reading, at a build.
                read_at(
                    source.as_ref(),
                    ended.then_some(ReadAt::Run(Reading::Build))
                )
            ),
        }
    }
}

impl fmt::Display for StepImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(f, false)
    }
}

impl std::error::Error for StepImportError {}

#[cfg(test)]
mod join_text {
    //! **The import door's join reads the adopted body at rest**: an
    //! undecided join keeps the reading's own lever in front of the
    //! join's, and a poisoned margin takes the stored-geometry note in
    //! place of a lever about an operation's inputs.
    #![allow(clippy::unwrap_used)]

    use geom_core::{
        Band, DAMAGED_FILE_ENDING, FileCoincidence, Indeterminate, MarginDiag,
        UNREADABLE_FILE_MARGIN_NOTE,
    };

    use super::StepImportError;

    fn join(margin: MarginDiag) -> (Indeterminate, String) {
        let diag = Indeterminate {
            margin,
            band: Band::new(1e-9, 1e-8).unwrap(),
            predicate: Some("dihedral_arm"),
            terminal_sliver: false,
        };
        let error = StepImportError::Join {
            solid: 7,
            refusal: topo::JoinRefusal::Undecided(topo::JoinUndecided {
                vertex: topo::VertexKey::default(),
                reading: topo::JoinReading::Regularity(diag),
            }),
            file: FileCoincidence::new(1e-6),
        };
        (diag, error.render())
    }

    #[test]
    fn an_undecided_join_at_import_keeps_the_readings_lever() {
        let lead = "step import: joining the edges of the solid at #7: ";
        let subject = topo::JOIN_SUBJECT;
        let lever = topo::JOIN_LEVER;
        let (straddle, text) = join(MarginDiag::enclosure(-2e-9, 5e-9));
        assert_eq!(
            text,
            format!(
                "{lead}{subject} is undecided ({}). Recourse: subdivide the parameter box \
                 for a tighter enclosure, or {lever}",
                straddle.payload()
            ),
            "a straddle"
        );
        let (poisoned, text) = join(MarginDiag::INVALID);
        assert_eq!(
            text,
            format!(
                "{lead}{subject} is undecided ({}). Recourse: {lever}; \
                 {UNREADABLE_FILE_MARGIN_NOTE}",
                poisoned.payload()
            ),
            "an unreadable margin names the file at the import door"
        );
    }

    /// **A join refusal a bad file reaches names the file at the import
    /// door** (D2 (iv)): the kernel-defect arm ends in the damaged
    /// file's ending, not the at-rest kernel-or-file one. Red if the
    /// door reads the join at rest.
    #[test]
    fn a_refused_join_at_import_names_the_file() {
        let error = StepImportError::Join {
            solid: 7,
            refusal: topo::JoinRefusal::Kernel {
                kind: topo::BooleanErrorKind::JoinDesync,
            },
            file: FileCoincidence::new(1e-6),
        };
        assert_eq!(
            error.render(),
            format!(
                "step import: joining the edges of the solid at #7: the join of two edges \
                 meeting at a vertex on one curve refused (JoinDesync). {DAMAGED_FILE_ENDING}"
            )
        );
    }
}

#[cfg(test)]
mod nested_at_the_door {
    //! **A nested refusal the import door renders reads that door**: the
    //! placement's map meeting a corrupt body, and the pcurve re-mint's
    //! escalation past the file's ε_in, end in the damaged file's ending. Red if
    //! the door renders the nested refusal's `Display` (a build) or reads
    //! it at rest.
    #![allow(clippy::unwrap_used)]

    use geom_core::{Band, DAMAGED_FILE_ENDING, FileCoincidence, Indeterminate, MarginDiag};

    use super::StepImportError;

    #[test]
    fn a_placements_refused_map_names_the_file() {
        let error = StepImportError::Placement {
            transform: 9,
            source: topo::TransformError::Corrupt {
                found: "a vertex references a missing point",
            },
            file: FileCoincidence::new(1e-6),
        };
        let text = error.render();
        assert!(
            text.starts_with("step import: the assembly placement stated at #9 refused")
                && text.ends_with(DAMAGED_FILE_ENDING),
            "{text}"
        );
    }

    /// The re-mint's undecided escalation opens on the file's door and
    /// reads as rest does: undecided, so the last resort (ONEHOME fork
    /// 259), not the damaged file.
    #[test]
    fn a_pcurve_remints_escalation_names_the_file() {
        let error = StepImportError::Pcurves {
            source: topo::PcurveMintError::Certify {
                half_edge: topo::HalfEdgeKey::default(),
                error: geom_brep::PcurveCertifyError::Escalated {
                    check: geom_brep::PcurveCheck::ChartWinding,
                    sample: 0,
                    cause: Indeterminate {
                        margin: MarginDiag::value(5e-9),
                        band: Band::new(1e-9, 1e-8).unwrap(),
                        predicate: None,
                        terminal_sliver: false,
                    },
                },
            },
            file: FileCoincidence::new(1e-6),
        };
        let text = error.render();
        assert!(
            text.starts_with("step import: pcurve re-mint on the adopted body:")
                && text.ends_with(geom_core::KERNEL_LIMIT_RECOURSE),
            "{text}"
        );
    }

    /// **An adopted curve off its face names the damaged file, with no
    /// ε_in offer** (D3, D4 ¶1 (iv)): a definite incidence miss read at
    /// the import door, with no margin or with the pose height it may
    /// quote (`OffMargin::Past`, within ε_in here), is the file's, whoever
    /// declared the curve; its margin is no reading of the miss, so the
    /// door's within-ε_in sentence is not offered.
    #[test]
    fn an_adopted_curve_off_its_face_names_the_damaged_file() {
        for margin in [
            None,
            Some(geom_brep::OffMargin::Past(MarginDiag::value(5e-7))),
        ] {
            let error = StepImportError::Pcurves {
                source: topo::PcurveMintError::Certify {
                    half_edge: topo::HalfEdgeKey::default(),
                    error: geom_brep::PcurveCertifyError::Incidence(
                        geom_brep::IncidenceRefusal::off_chart(
                            geom::SurfaceKind::Cone,
                            geom::CurveKind::Circle,
                            "the circle dips below the apex",
                            margin,
                        ),
                    ),
                },
                file: FileCoincidence::new(1e-6),
            };
            let text = error.render();
            assert!(text.ends_with(DAMAGED_FILE_ENDING), "{margin:?}: {text}");
            for word in ["ε_in", "declared", "kernel defect"] {
                assert!(!text.contains(word), "{margin:?}: {word}: {text}");
            }
        }
    }
}

#[cfg(test)]
mod meet_unread_text {
    //! **A meridian's meeting that did not run to an answer ends as its
    //! own standing does**: a pose with no closed form is no way through
    //! yet, at the import door, and the bare text carries no ending.
    #![allow(clippy::unwrap_used)]

    use geom_brep::curve_pair::{MeetFault, MeetUnread};
    use geom_core::FileCoincidence;

    use super::StepImportError;

    #[test]
    fn an_unread_meeting_ends_as_its_standing() {
        let why = MeetUnread::of(MeetFault::Unsupported {
            pose: "a pose this row names",
        })
        .unwrap();
        let error = StepImportError::SingularityMeetUnread {
            id: 7,
            why,
            file: FileCoincidence::new(1e-6),
        };
        let ended = error.render();
        assert!(
            ended.contains("face #7") && ended.contains("a pose this row names"),
            "the payload names the face and the pose: {ended}"
        );
        assert!(
            ended.ends_with(&*why.recourse().render()),
            "the door's ending is the reading's own: {ended}"
        );
        let bare = error.to_string();
        assert!(
            !bare.contains(&*why.recourse().render()),
            "the bare text ends at its payload: {bare}"
        );
    }
}
