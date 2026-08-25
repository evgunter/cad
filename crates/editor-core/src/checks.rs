//! The advisory-check registry (DISCIPLINES-DESIGN DS6, grade 4) and
//! its first resident, the connectedness check (LONGTERM-IDEAS I1(0b)).
//!
//! A check is a **pure analysis over a finished evaluation** producing
//! findings — no declaration vocabulary, no verify table, and by
//! construction no effect on any evaluated body: nothing here mutates
//! or rebuilds geometry, and no severity or configuration value
//! reaches evaluation (the DS3 invariant holds because there is no
//! path by which it could fail to).
//!
//! Postures, each load-bearing:
//!
//! - **[`run_checks`] reports, never gates** (the [`crate::mixed_pins`]
//!   posture): nothing calls it from `apply`, from the load door, or
//!   from evaluation. A document with findings is valid at every one
//!   of those doors.
//! - **[`enforce_checks`] is the ONLY refusing path**: it consumes a
//!   finished report and refuses on `Error`-severity findings; the
//!   CALLER chooses where (and whether) to gate.
//! - **`Off` checks are VISIBLY skipped** ([`ChecksReport::skipped`]):
//!   "checked and fine" and "not checked" are different answers.
//! - **Deterministic order** (D9): findings follow root-list order,
//!   then output-index order within a root — a report that changes
//!   only when the document or its evaluation does.
//!
//! The check set is a CLOSED enum ([`CheckId`], the D3 philosophy): a
//! new check is a new variant, and the compiler enumerates every match
//! site that must learn about it. There is no dynamic registration.

use std::collections::BTreeMap;
use std::sync::Arc;

use core::fmt;

use geom_core::{BandError, Decide, Tol};
use topo::{Body, PropsQuadLane, ShellClassifyError, ShellRole, classify_shells};

use crate::doc::Doc;
use crate::eval::Evaluation;
use crate::node::RecipeNodeId;
use crate::product;

/// The closed set of checks. A new check = a new variant; every match
/// over this enum is a site the compiler then walks you to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CheckId {
    /// The connectedness check (I1(0b)): a body at rest with more (or
    /// fewer) disconnected components than expected. Components are
    /// counted as `Outer` shells ([`topo::classify_shells`]); internal
    /// voids are boundary, not components, and are never counted.
    Connectedness,
}

impl CheckId {
    /// The honesty label (DS6): whether this check's finding is a
    /// theorem or a judgment. `Connectedness` is certified — the count
    /// is exact and combinatorial; the one decided predicate under it
    /// is the named shell-orientation read, whose in-band outcomes
    /// surface as their own typed findings rather than moving the
    /// count.
    pub fn kind(self) -> CheckKind {
        match self {
            Self::Connectedness => CheckKind::Certified,
        }
    }
}

impl fmt::Display for CheckId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connectedness => f.write_str("connectedness"),
        }
    }
}

/// The certified/heuristic split (DS6): honesty of language and a
/// default level, not a force cap. A certified finding is a theorem
/// about the evaluated geometry; a heuristic finding is a judgment
/// that can be wrong in both directions and must never be dressed as
/// the former.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckKind {
    /// The finding is a theorem (the connectedness count).
    Certified,
    /// The finding is a labeled judgment (no resident yet).
    Heuristic,
}

/// A check's severity knob (DS6's check shape: off / warn / error).
/// Legal under DS3 because no position ever changes an evaluated
/// body — positions differ only in what is *accepted*: `Off` skips
/// visibly, `Warn` and `Error` produce identical findings, and `Error`
/// additionally refuses at [`enforce_checks`] (nowhere else).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    /// The check does not run; it is listed in
    /// [`ChecksReport::skipped`].
    Off,
    /// Findings are reported; nothing refuses.
    Warn,
    /// Findings are reported AND refuse at [`enforce_checks`].
    Error,
}

/// Per-run check configuration (the [`crate::EvalOptions`] mold: a
/// plain argument to the door, no ambient state, nothing persisted).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChecksConfig {
    /// [`CheckId::Connectedness`]'s severity. Default `Warn` (the
    /// I1(0b) charter).
    pub connectedness: Severity,
    /// The expected component count per subject, keyed by
    /// `(root, output_ix)`; a missing key means 1. This is the
    /// resident's acknowledgment mechanism: a deliberately disjoint
    /// body is EXPECTED disjoint, stated as data, and stays clean.
    pub expected_components: BTreeMap<(RecipeNodeId, u32), u32>,
}

impl Default for ChecksConfig {
    fn default() -> Self {
        Self {
            connectedness: Severity::Warn,
            expected_components: BTreeMap::new(),
        }
    }
}

impl ChecksConfig {
    /// The configured severity of `check` (the one match site the
    /// closed enum walks a new check to).
    pub fn severity(&self, check: CheckId) -> Severity {
        match check {
            CheckId::Connectedness => self.connectedness,
        }
    }
}

/// The typed evidence of one finding.
#[derive(Clone, Debug, PartialEq)]
pub enum CheckEvidence {
    /// The component count disagrees with the expectation. Exact: the
    /// count is a count of definitely-`Outer` shells, compared with
    /// `!=` — no ε anywhere in the comparison.
    Connectedness {
        /// Components found (`Outer` shells; voids never counted).
        actual: u32,
        /// Components expected ([`ChecksConfig::expected_components`],
        /// default 1).
        expected: u32,
    },
    /// A shell's orientation read escalated (in-band or zero signed
    /// volume): the component count for this subject is UNKNOWABLE at
    /// this tolerance, which is a finding, never a silent skip (F6).
    Escalated {
        /// The typed refusal from the shell door.
        source: ShellClassifyError,
    },
    /// A face of this subject is outside the flux inventory (the
    /// props lane's typed refusal, inherited from tier-3 check 7's
    /// machinery): the count cannot be computed, and says so.
    Unsupported {
        /// The typed refusal from the shell door.
        source: ShellClassifyError,
    },
    /// An expectation with no subject: `expected_components` carries
    /// an entry at this `(root, output_ix)` and no evaluated subject
    /// consumed it — the body vanished (a boolean may have consumed
    /// the whole part) or the key names no root output. Expectations
    /// are two-directional: an entry that binds nothing is stale, not
    /// silently ignored. (The DEFAULT expectation — no entry — binds
    /// only existing subjects: a root that legitimately denotes zero
    /// bodies with nothing stated about it stays clean.)
    StaleExpectation {
        /// The entry's stated component count.
        expected: u32,
    },
}

/// One finding of one check on one subject — a body-denoting root
/// output, attributed as `(root, output_ix)` (the
/// [`crate::AtRestFinding`]/`Attribution` rhyme).
#[derive(Clone, Debug, PartialEq)]
pub struct CheckFinding {
    /// The check that fired.
    pub check: CheckId,
    /// The root whose value carries the subject body.
    pub root: RecipeNodeId,
    /// Which output body of that root (multi-output ops; 0 for a
    /// single-body root).
    pub output_ix: u32,
    /// What was found.
    pub evidence: CheckEvidence,
}

// One story, one recourse, in one place (the eval/mod.rs D54 lesson:
// a payload with no Display forces every consumer to invent its own
// second vocabulary). The Unsupported arm FORWARDS its payload's
// Display; the Escalated arm deliberately does NOT — the funnel's
// generic coincidence recourse ("declare the coincidence / move the
// geometry") is meaningless for a shell-volume sign, and a kernel
// arena key names nothing a document user can act on, so this arm
// renders the margin-payload view (name + numbers, no recourse tail,
// no key) and states the check's own recourse. The subject is named
// by the finding's (root, output) attribution above.
impl fmt::Display for CheckFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "check {}: root {} output {}: ",
            self.check, self.root.0, self.output_ix
        )?;
        match &self.evidence {
            CheckEvidence::Connectedness { actual, expected } => write!(
                f,
                "{actual} disconnected component(s) where {expected} was expected — a stray \
                 component usually means a boolean that did not reach its operand or an \
                 instance placed nowhere; if the disjoint body is deliberate, state the \
                 expected count for this root output in ChecksConfig::expected_components"
            ),
            CheckEvidence::Escalated { source } => {
                f.write_str("the component count is unknowable at this tolerance: ")?;
                match source {
                    ShellClassifyError::Escalated { source, .. } => {
                        write!(f, "{}", source.payload())?;
                    }
                    ShellClassifyError::ZeroVolume { .. } => f.write_str(
                        "a shell's signed volume is definitely zero (or its certified \
                         bracket straddles zero)",
                    )?,
                    // run_checks routes only the two sign-read arms
                    // here; any other source forwards its own story.
                    other => write!(f, "{other}")?,
                }
                write!(
                    f,
                    " — a shell's volume is too close to zero for a certified outer/void \
                     orientation read; thicken or remove the degenerate geometry, or lower \
                     the tolerance"
                )
            }
            CheckEvidence::Unsupported { source } => write!(
                f,
                "the component count cannot be computed for this body: {source}"
            ),
            CheckEvidence::StaleExpectation { expected } => write!(
                f,
                "an expectation of {expected} component(s) has no subject — the body \
                 vanished (a boolean may have consumed the whole part) or the key names \
                 no root output; remove the ChecksConfig::expected_components entry or \
                 fix the root"
            ),
        }
    }
}

/// The result of one [`run_checks`] run: findings in deterministic
/// order, and the checks that were configured `Off` — visibly skipped,
/// because "checked and fine" and "not checked" are different answers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChecksReport {
    /// Findings, ordered by root-list position then output index (D9).
    pub findings: Vec<CheckFinding>,
    /// Checks that did not run because their severity is `Off`.
    pub skipped: Vec<CheckId>,
}

impl fmt::Display for ChecksReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.findings.is_empty() {
            write!(f, "checks: no findings")?;
        } else {
            write!(f, "checks: {} finding(s)", self.findings.len())?;
            for finding in &self.findings {
                write!(f, "\n  {finding}")?;
            }
        }
        if !self.skipped.is_empty() {
            write!(f, "\nchecks skipped (severity Off):")?;
            for check in &self.skipped {
                write!(f, " {check}")?;
            }
        }
        Ok(())
    }
}

/// Typed refusal of [`run_checks`] itself (closed enum, D4 ¶3) —
/// distinct from a finding: these mean the checks could not be RUN,
/// not that a check fired.
#[derive(Clone, Debug, PartialEq)]
pub enum ChecksError {
    /// A root produced no value in this evaluation (failed, poisoned,
    /// or past a cancelation's prefix) — the [`crate::product()`]
    /// posture: checks are defined over roots that evaluated, and a
    /// report over a partial evaluation would claim more than was
    /// checked.
    Root {
        /// The root without a value.
        node: RecipeNodeId,
    },
    /// The run's tolerance cannot form a band.
    Band {
        /// The band construction failure.
        error: BandError,
    },
}

impl fmt::Display for ChecksError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Root { node } => write!(
                f,
                "checks: root {} produced no value in this evaluation — evaluate to \
                 completion (and fix or remove the failing root) before running checks",
                node.0
            ),
            Self::Band { error } => write!(f, "checks: {error}"),
        }
    }
}

impl core::error::Error for ChecksError {}

/// Refusal from [`enforce_checks`]: the report carries findings whose
/// check is configured at `Error`.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckRefusal {
    /// The refusing findings (every `Error`-severity finding of the
    /// report, in report order).
    pub findings: Vec<CheckFinding>,
}

impl fmt::Display for CheckRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} check finding(s) at Error severity:",
            self.findings.len()
        )?;
        for finding in &self.findings {
            write!(f, "\n  {finding}")?;
        }
        Ok(())
    }
}

impl core::error::Error for CheckRefusal {}

/// Runs every configured check over `doc`'s evaluated roots and
/// returns the report. **Reports, never gates** (the
/// [`crate::mixed_pins`] posture): nothing calls this from `apply`,
/// from the load door, or from evaluation, and running it changes
/// nothing — the evaluation is read, never rebuilt (DS3 by
/// construction).
///
/// Subjects are the body-denoting outputs of `doc.roots()`, each
/// checked per `(root, output_ix)`. For the connectedness check, the
/// component count of a subject is its number of definitely-`Outer`
/// shells ([`topo::classify_shells`]) — an internal void is boundary,
/// not a component, at every tolerance row — compared EXACTLY (`!=`)
/// against the expectation. A shell the door cannot classify surfaces
/// as its own typed finding ([`CheckEvidence::Escalated`] /
/// [`CheckEvidence::Unsupported`]), never as a silent skip.
///
/// Expectations are TWO-DIRECTIONAL: every
/// [`ChecksConfig::expected_components`] entry must be consumed by a
/// subject, and an entry no subject consumed — a vanished body (a
/// boolean's honest ∅ result denotes zero subjects), a dead root, a
/// mistyped key — is its own finding
/// ([`CheckEvidence::StaleExpectation`], appended after the subject
/// findings in key order). The DEFAULT expectation binds only
/// existing subjects: a legitimately empty result with nothing stated
/// about it stays clean.
///
/// # Errors
///
/// [`ChecksError`] — a root without a value in `ev`, or a band the
/// tolerance cannot form. These mean the checks could not run at all;
/// a check that ran and disagreed is a FINDING, not an error.
pub fn run_checks<P, T: Decide + PropsQuadLane>(
    doc: &Doc<P>,
    ev: &Evaluation<T>,
    cfg: &ChecksConfig,
    tol: Tol,
) -> Result<ChecksReport, ChecksError> {
    let mut report = ChecksReport::default();
    if cfg.severity(CheckId::Connectedness) == Severity::Off {
        report.skipped.push(CheckId::Connectedness);
        return Ok(report);
    }
    // Entries not yet consumed by a subject; whatever remains after
    // the walk is stale (an expectation with no subject).
    let mut unconsumed = cfg.expected_components.clone();
    for &root in doc.roots() {
        let Some(value) = ev.value(root) else {
            return Err(ChecksError::Root { node: root });
        };
        // Non-body roots (datums, mates, profiles, declarations)
        // denote no subject; an empty boolean denotes zero subjects.
        let Some(sources) = product::sources_of(value) else {
            continue;
        };
        for (output_ix, body, _contacts) in sources {
            unconsumed.remove(&(root, output_ix));
            match classify_shells(body.as_ref(), tol) {
                Ok(classes) => {
                    let outer = classes
                        .iter()
                        .filter(|c| c.role == ShellRole::Outer)
                        .count();
                    // A shell count past u32 is a kernel-bug state
                    // (no input reaches it), not an input state.
                    let actual = u32::try_from(outer)
                        .unwrap_or_else(|_| unreachable!("shell count {outer} exceeds u32"));
                    let expected = cfg
                        .expected_components
                        .get(&(root, output_ix))
                        .copied()
                        .unwrap_or(1);
                    if actual != expected {
                        report.findings.push(CheckFinding {
                            check: CheckId::Connectedness,
                            root,
                            output_ix,
                            evidence: CheckEvidence::Connectedness { actual, expected },
                        });
                    }
                }
                Err(ShellClassifyError::Band { error }) => {
                    return Err(ChecksError::Band { error });
                }
                Err(
                    source @ (ShellClassifyError::Escalated { .. }
                    | ShellClassifyError::ZeroVolume { .. }),
                ) => {
                    report.findings.push(CheckFinding {
                        check: CheckId::Connectedness,
                        root,
                        output_ix,
                        evidence: CheckEvidence::Escalated { source },
                    });
                }
                Err(source @ ShellClassifyError::Props { .. }) => {
                    report.findings.push(CheckFinding {
                        check: CheckId::Connectedness,
                        root,
                        output_ix,
                        evidence: CheckEvidence::Unsupported { source },
                    });
                }
            }
        }
    }
    // The reverse direction: every stated expectation must have found
    // its subject. BTreeMap iteration = key order (deterministic, D9).
    for ((root, output_ix), expected) in unconsumed {
        report.findings.push(CheckFinding {
            check: CheckId::Connectedness,
            root,
            output_ix,
            evidence: CheckEvidence::StaleExpectation { expected },
        });
    }
    Ok(report)
}

/// The door from a finding's attribution back to its subject: the body
/// at `(root, output_ix)` in this evaluation — the same enumeration
/// [`run_checks`] walks, so a [`CheckFinding`]'s attribution always
/// resolves against the evaluation it was produced from. `None` when
/// the root has no value, denotes no body, or has no output at that
/// index (exactly the attributions a [`CheckEvidence::StaleExpectation`]
/// finding names).
pub fn subject_body<T: Decide>(
    ev: &Evaluation<T>,
    root: RecipeNodeId,
    output_ix: u32,
) -> Option<Arc<Body<T>>> {
    let sources = product::sources_of(ev.value(root)?)?;
    sources
        .into_iter()
        .find(|(ix, _, _)| *ix == output_ix)
        .map(|(_, body, _)| body)
}

/// The ONE refusing path of the registry: refuses iff `report` carries
/// a finding whose check `cfg` sets to [`Severity::Error`]. The caller
/// chooses where to gate — this module never does.
///
/// Consumes a FINISHED report (DS3: enforcement is a read over
/// findings; it evaluates nothing and can change nothing). `Warn`
/// findings pass; a report whose findings are all `Warn`-severity is
/// `Ok(())` with the findings still in the report for the caller to
/// render.
///
/// # Errors
///
/// [`CheckRefusal`], carrying every `Error`-severity finding in report
/// order.
pub fn enforce_checks(report: &ChecksReport, cfg: &ChecksConfig) -> Result<(), CheckRefusal> {
    let findings: Vec<CheckFinding> = report
        .findings
        .iter()
        .filter(|finding| cfg.severity(finding.check) == Severity::Error)
        .cloned()
        .collect();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(CheckRefusal { findings })
    }
}
