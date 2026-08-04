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
    /// reader must interpret it. `B_SPLINE_SURFACE_WITH_KNOTS` is the
    /// named M7 frontier: NURBS *faces* arrive with the loft/sweep
    /// assembly unit, and their import waits for their export (the S9
    /// flip pattern retires this refusal when they land).
    UnsupportedEntity {
        /// The offending entity instance.
        id: u64,
        /// The unsupported type name.
        keyword: String,
    },
    /// The file's unit context is outside the subset the kernel
    /// understands (the writer emits unprefixed `SI_UNIT($, .METRE.)`
    /// / `.RADIAN.` / `.STERADIAN.`). Parsed, not assumed — a foreign
    /// unit is a typed refusal until the M7-2 unit ladder.
    UnsupportedUnit {
        /// The offending unit entity instance.
        id: u64,
        /// The unit text the subset does not cover.
        found: String,
    },
    /// The data section declares no usable shape representation (no
    /// `MANIFOLD_SOLID_BREP` and no `GEOMETRIC_CURVE_SET`).
    NothingToImport,
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
    /// Pcurve re-minting refused on the adopted body.
    Pcurves {
        /// The minting pass's error, displayed.
        source: topo::PcurveMintError,
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
                 (the analytic subset the kernel exports; NURBS surfaces arrive with \
                 the loft/sweep assembly unit and their import follows their export)"
            ),
            Self::UnsupportedUnit { id, found } => write!(
                f,
                "step import: entity #{id} declares unit {found} — the subset covers \
                 unprefixed SI metre/radian/steradian only (a foreign unit context is \
                 the M7-2 ladder, refused rather than guessed)"
            ),
            Self::NothingToImport => f.write_str(
                "step import: the data section carries no MANIFOLD_SOLID_BREP and no \
                 GEOMETRIC_CURVE_SET — nothing to import",
            ),
            Self::MissingUncertainty => f.write_str(
                "step import: no UNCERTAINTY_MEASURE_WITH_UNIT in the representation \
                 context and no per-call override — ε_in has no honest default",
            ),
            Self::InvalidEpsOverride { value } => write!(
                f,
                "step import: ε_in override {value} is not finite and strictly positive"
            ),
            Self::MalformedReal { id, token } => write!(
                f,
                "step import: entity #{id}: token '{token}' is not a Part 21 real"
            ),
            Self::Topology { id, what } => {
                write!(f, "step import: entity #{id}: {what}")
            }
            Self::Assembly { id, source } => write!(
                f,
                "step import: assembling entity #{id}: {source}"
            ),
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
            Self::Pcurves { source } => {
                write!(f, "step import: pcurve re-mint on the adopted body: {source}")
            }
        }
    }
}

impl std::error::Error for StepImportError {}
