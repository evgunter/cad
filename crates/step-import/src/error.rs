//! Typed import failure (closed enum, D4 ¶3). Every variant names the
//! offending entity id or line — the reader never panics and never
//! silently degrades a file into a different model.

use core::fmt;

/// One rung of the D7 edge-adoption ladder, as data: which intensional
/// interpretation was attempted, and the kernel gate's typed refusal.
#[derive(Debug)]
pub struct AdoptionAttempt {
    /// The interpretation attempted.
    pub candidate: AdoptionCandidate,
    /// The certification/attachment gate's refusal, verbatim.
    pub refusal: topo::EulerOpError,
}

/// The intensional interpretations the edge-adoption ladder can try
/// (D7 stage 2's vocabulary — `geom_brep::EdgeGeometry`'s variants,
/// named as data for future remedy flows).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdoptionCandidate {
    /// Transverse intersection of the two adjacent surfaces.
    Intersection,
    /// Tangential contact locus of the two adjacent surfaces.
    TangentIntersection,
    /// The parameterization seam of one closed surface.
    Seam,
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
            Self::Seam => "seam",
            Self::IsoCurve => "boundary iso-curve",
            Self::MappedCurve => "mapped curve",
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
    /// A position-anchored import declaration
    /// ([`crate::ImportContact`]) did not resolve to exactly the
    /// entities its kind names on the assembled body (M9-2, D7 step
    /// 4): a declaration that cannot be pinned to real entities is a
    /// typed refusal, never a silent drop — the same posture as the
    /// boolean door's dangling-declaration refusal.
    DeclarationUnresolved {
        /// The anchor position that failed to resolve.
        at: [f64; 3],
        /// How many candidate vertices were found within the gate
        /// band (a vertex-rest anchor needs exactly two).
        found: usize,
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
    },
    /// The D7 adoption ladder could not certify any intensional
    /// description for an edge: every candidate tried and its typed
    /// refusal, in ladder order — **structured data, not prose**, so a
    /// future remedy flow (GUI-offered healing, M7-2+) can act on what
    /// failed without re-parsing messages.
    Adoption {
        /// The `EDGE_CURVE` entity instance.
        id: u64,
        /// The candidates tried and their refusals, in ladder order.
        attempts: Vec<AdoptionAttempt>,
    },
    /// An ARC cap rim adjacent to a NURBS wall failed the import-side
    /// residual gate (M7-3 fix pass, review F1): sampled against the
    /// wall's own boundary column, the rim's circle does not carry it
    /// at the ambient tolerance. On a RATIONAL wall this gate IS the
    /// certification -- nothing else checks the rim there (no pcurve
    /// mint, dihedral kind-exempt, the conventional description
    /// certifies only against itself) -- so a rim that fails it is
    /// refused rather than laundered into a t1/t2-valid body
    /// indistinguishable from a correct one.
    RimOffWallBoundary {
        /// The `EDGE_CURVE` entity instance.
        id: u64,
        /// The worst sampled deviation (meters): boundary-sample
        /// distance to the rim's circle locus, or arc-length excess
        /// outside the rim's parameter range, whichever is larger.
        residual: f64,
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
    },
    /// An assembly instance's rigid placement refused at the kernel's
    /// own [`topo::transform_rigid`] door (M7-4 Leg D): a map this
    /// reader's ε_in classification let through that the kernel's
    /// decided predicates would not, or a carrier that failed to
    /// re-certify against the mapped geometry.
    Placement {
        /// The `ITEM_DEFINED_TRANSFORMATION` that states this
        /// instance's frame — with N instances in a file, which one
        /// refused is the whole question.
        transform: u64,
        /// The transform op's error, displayed.
        source: topo::TransformError,
    },
    /// A placed assembly instance refused to graft into the result
    /// body at the kernel's own [`topo::graft_disjoint`] door — the
    /// transplant found the placed component ill-formed, or a
    /// transplanted edge description did not re-certify against the
    /// destination's surfaces. Both are kernel-side refusals about a
    /// body this reader had already built and gated; neither is
    /// silently absorbed.
    Instance {
        /// The `MANIFOLD_SOLID_BREP` whose instance was being placed.
        solid: u64,
        /// The graft's error, displayed.
        source: Box<topo::BooleanError>,
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
        /// body (which for a one-solid file is the same body — see
        /// [`crate::import_step`]).
        solid: Option<u64>,
        /// The tier-1/2/3 verdicts, verbatim.
        errors: Vec<topo::ValidationError>,
    },
}

impl fmt::Display for StepImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
            Self::DeclarationUnresolved { at, found } => write!(
                f,
                "step import: the declared contact anchored at {at:?} resolved to {found} \
                 coincident vertices on the assembled body (a vertex-rest anchor needs \
                 exactly two) — fix the anchor or remove the declaration; unresolved \
                 intent never silently drops"
            ),
            Self::MalformedReal { id, token } => write!(
                f,
                "step import: entity #{id}: token '{token}' is not a Part 21 real"
            ),
            Self::Topology { id, what } => {
                write!(f, "step import: entity #{id}: {what}")
            }
            Self::Assembly { id, source } => {
                write!(f, "step import: assembling entity #{id}: {source}")
            }
            Self::Adoption { id, attempts } => {
                write!(
                    f,
                    "step import: edge #{id}: no intensional description certifies — "
                )?;
                for (i, attempt) in attempts.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "{}: {}", attempt.candidate, attempt.refusal)?;
                }
                Ok(())
            }
            Self::RimOffWallBoundary { id, residual } => write!(
                f,
                "step import: edge #{id}: an ARC cap rim does not lie on its adjacent \
                 NURBS wall's boundary — the wall's own boundary column, sampled at the \
                 certification schedule, deviates from the rim's circle by up to \
                 {residual:e} m (ambient tolerance exceeded). On a rational wall this \
                 residual gate is the rim's only certification, so the file is refused \
                 rather than adopted wrong"
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
            Self::Pcurves { source } => {
                write!(
                    f,
                    "step import: pcurve re-mint on the adopted body: {source}"
                )
            }
            Self::TierInvalid { solid, errors } => {
                // Both forms of each verdict: the Debug shape names
                // the failing CHECK and its entity keys structurally
                // (greppable, and what a caller matching on
                // `errors` sees); the Display prose says what the
                // check means and carries the margins the measured
                // checks report.
                let verdicts: Vec<String> = errors.iter().map(|e| format!("{e:?} — {e}")).collect();
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
            Self::Placement { transform, source } => write!(
                f,
                "step import: the assembly placement stated at #{transform} refused \
                 at the kernel’s transform door: {source:?}"
            ),
            Self::Instance { solid, source } => write!(
                f,
                "step import: an assembly instance of the solid at #{solid} refused \
                 to graft into the result body at the kernel’s disjoint-graft door: \
                 {source:?}"
            ),
        }
    }
}

impl std::error::Error for StepImportError {}
