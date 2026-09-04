//! The advisory-check registry (DISCIPLINES-DESIGN DS6, grade 4) and
//! its two residents: the connectedness check (LONGTERM-IDEAS I1(0b))
//! and the product-separation check (the establishment of disjointness
//! `topo::graft_disjoint_all_keyed` leaves to its callers).
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

use geom_core::{BandError, CertifiedBounds, Decide, Tol};
use topo::{
    AtRestPolicy, Body, ContactRecords, PropsQuadLane, ShellClassifyError, ShellRole,
    classify_shells,
};

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
    /// The product-separation check: two solids the gather contributed
    /// from DIFFERENT root subjects that are not certifiably disjoint
    /// ([`topo::SolidSeparation`]).
    ///
    /// The gather gathers, it does not fuse — a product's solids are
    /// disjoint solids of one aggregate, and
    /// [`topo::graft_disjoint_all_keyed`] asserts nothing about that,
    /// so every caller owes an establishment of disjointness. This is
    /// the document layer's, run as a report.
    Separation,
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
            // Certified in the direction it stays SILENT: a pair this
            // check does not report is one the box rule PROVED apart,
            // and the box rule is a sound superset for every surface
            // kind. What it reports is the denial of that certificate,
            // which is a fact about the certificate and is exactly what
            // the finding says — never "these two overlap", which the
            // boxes do not decide. See `topo::SolidsMeet`.
            Self::Separation => CheckKind::Certified,
        }
    }
}

impl fmt::Display for CheckId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connectedness => f.write_str("connectedness"),
            Self::Separation => f.write_str("separation"),
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

/// A severity knob for a resident that may not refuse — [`Severity`]
/// minus `Error`.
///
/// DS6 lets a check offer `error` only if it ships a waiver
/// vocabulary. A resident without one needs a knob whose refusing
/// position does not exist, rather than a full [`Severity`] and a
/// comment asking callers not to use it: the check registry is a
/// public API, and "cannot be spelled" is the only form of that rule
/// a caller cannot get wrong.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Advisory {
    /// The check does not run; it is listed in
    /// [`ChecksReport::skipped`].
    Off,
    /// Findings are reported; nothing refuses.
    Warn,
}

impl Advisory {
    /// This knob as a [`Severity`] — the widening the registry's one
    /// severity match goes through. Total and injective, and it never
    /// produces `Error`, which is the whole point.
    pub fn severity(self) -> Severity {
        match self {
            Self::Off => Severity::Off,
            Self::Warn => Severity::Warn,
        }
    }
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
    /// [`CheckId::Separation`]'s severity — [`Advisory`], NOT
    /// [`Severity`], so `Error` is unrepresentable here.
    ///
    /// **DS6's waiver rule is an `iff`** (DISCIPLINES-DESIGN, round 3,
    /// Ev): a check may offer `error` *iff* it ships a per-finding,
    /// stable-name-keyed acknowledgment record with a staleness
    /// direction. The connectedness resident satisfies it through
    /// `expected_components` + [`CheckEvidence::StaleExpectation`].
    /// This resident ships nothing analogous, so it may not offer
    /// `error`, and the type says so rather than a comment asking the
    /// caller not to.
    ///
    /// **The declared-contact suppression is NOT that waiver** and
    /// must not be mistaken for one: it is derived from mates rather
    /// than authored about a finding, it is keyed by kernel arena
    /// entities rather than stable names, it carries no provenance,
    /// and it has no staleness direction — a declaration that stops
    /// matching any pair is never flagged. A real waiver would be
    /// keyed by the `(root, output)` PAIR a finding names, which is
    /// the shape `expected_components` has one subject down; it is
    /// what this resident owes before `Error` becomes representable,
    /// and `heatsink.pncad` is the standing demonstration that it is
    /// owed (five findings that are all correct and none of which can
    /// be acknowledged short of turning the resident off).
    pub separation: Advisory,
}

impl Default for ChecksConfig {
    fn default() -> Self {
        Self {
            connectedness: Severity::Warn,
            expected_components: BTreeMap::new(),
            separation: Advisory::Warn,
        }
    }
}

impl ChecksConfig {
    /// The configured severity of `check` (the one match site the
    /// closed enum walks a new check to).
    pub fn severity(&self, check: CheckId) -> Severity {
        match check {
            CheckId::Connectedness => self.connectedness,
            CheckId::Separation => self.separation.severity(),
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
    /// Two solids the gather contributed from different root subjects
    /// could not be certified apart. The finding's own `(root,
    /// output_ix)` is the FIRST subject in gather order; this names the
    /// second.
    ///
    /// Not a claim that the two overlap — see [`topo::SolidsMeet`].
    /// What it denies is the certificate, and a pair that merely
    /// touches lands here too.
    NotSeparated {
        /// The counterpart root.
        other_root: RecipeNodeId,
        /// The counterpart root's output-body index.
        other_output: u32,
    },
    /// The separation machinery could not be built over the product at
    /// all — the box builder's typed refusal. The check has no verdict
    /// for ANY pair, which is a finding, never a silent pass (F6).
    SeparationUnavailable {
        /// Which arm of the kernel's refusal fired — the typed half,
        /// and the one a consumer branches on.
        ///
        /// `topo::BooleanError` itself is neither `Clone` nor
        /// `PartialEq` and a report is both, so the error cannot ride
        /// here; its class projection can, and does.
        kind: topo::BooleanErrorKind,
        /// The kernel's own refusal, rendered, for a reader — it
        /// carries the arena keys and margins `kind` drops.
        ///
        /// The payload's own `Display` is the vocabulary this module
        /// forwards (the one-story rule at `crate::finding`), so what
        /// a caller READS is the kernel's own sentence; `kind` beside
        /// it is what a caller MATCHES on, so neither half is a
        /// substring hunt through the other.
        reason: String,
    },
}

impl CheckEvidence {
    /// [`CheckEvidence::SeparationUnavailable`] built from ONE refusal:
    /// `kind` is the class a consumer matches, `reason` the kernel's
    /// own sentence a reader reads. Both come off the same error, which
    /// is the invariant the door holds and a hand-built literal does
    /// not — so the door goes through here rather than writing the two
    /// fields at the raise site.
    fn separation_unavailable(source: &topo::BooleanError) -> Self {
        Self::SeparationUnavailable {
            kind: source.kind(),
            reason: source.to_string(),
        }
    }
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

// One story, one recourse, in one place, through the document
// layer's one sink ([`crate::finding`]; the eval/mod.rs one-vocabulary
// lesson: a payload with no Display forces every consumer to invent
// its own second vocabulary). The Unsupported arm FORWARDS its
// payload's Display — the payload's own recourse rides the story, so
// `recourse` answers "" there ("already told"). The Escalated arm
// deliberately does NOT forward the funnel's generic coincidence
// recourse ("declare the coincidence / move the geometry") — it is
// meaningless for a shell-volume sign, and a kernel arena key names
// nothing a document user can act on — so that arm renders the
// margin-payload view (name + numbers, no recourse tail, no key) and
// states the check's own recourse. StaleExpectation's recourse is
// pinned prose riding the story's own "; " joint, so it too answers
// "" rather than growing a second tail. The subject is the finding's
// (root, output) attribution.
impl crate::finding::Finding for CheckFinding {
    fn subject(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "check {}: root {} output {}",
            self.check, self.root.0, self.output_ix
        )
    }

    fn story(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.evidence {
            CheckEvidence::Connectedness { actual, expected } => write!(
                f,
                "{actual} disconnected component(s) where {expected} was expected"
            ),
            CheckEvidence::Escalated { source } => {
                f.write_str("the component count is unknowable at this tolerance: ")?;
                match source {
                    ShellClassifyError::Escalated { source, .. } => {
                        write!(f, "{}", source.payload())
                    }
                    ShellClassifyError::ZeroVolume { .. } => f.write_str(
                        "a shell's signed volume is definitely zero (or its certified \
                         bracket straddles zero)",
                    ),
                    // run_checks routes only the two sign-read arms
                    // here; any other source forwards its own story.
                    other => write!(f, "{other}"),
                }
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
            CheckEvidence::NotSeparated {
                other_root,
                other_output,
            } => write!(
                f,
                "not certifiably disjoint from root {} output {other_output}: the \
                 product gathers both, so any space they share is gathered twice",
                other_root.0
            ),
            CheckEvidence::SeparationUnavailable { reason, .. } => write!(
                f,
                "no pair of this product's solids could be checked for separation: \
                 {reason}"
            ),
        }
    }

    fn recourse(&self) -> &str {
        match &self.evidence {
            CheckEvidence::Connectedness { .. } => {
                "a stray component usually means a boolean that did not reach its operand \
                 or an instance placed nowhere; if the disjoint body is deliberate, state \
                 the expected count for this root output in ChecksConfig::expected_components"
            }
            CheckEvidence::Escalated { .. } => {
                "a shell's volume is too close to zero for a certified outer/void \
                 orientation read; thicken or remove the degenerate geometry, or lower \
                 the tolerance"
            }
            CheckEvidence::NotSeparated { .. } => {
                "usually a recipe that grew a second sink by accident: a feature left \
                 dangling when its consumer was rewired is still a product root, so \
                 delete it or feed it into the root downstream of it. Two roots meant \
                 to TOUCH want a mate, whose declaration the assembly door certifies; \
                 two meant to INTERPENETRATE want a boolean, not a gather"
            }
            CheckEvidence::Unsupported { .. }
            | CheckEvidence::StaleExpectation { .. }
            | CheckEvidence::SeparationUnavailable { .. } => "",
        }
    }
}

impl fmt::Display for CheckFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        crate::finding::compose(f, self)
    }
}

/// The result of one [`run_checks`] run: findings in deterministic
/// order, and the checks that were configured `Off` — visibly skipped,
/// because "checked and fine" and "not checked" are different answers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChecksReport {
    /// Findings in deterministic order (D9): each resident's own
    /// findings by root-list position then output index, residents in
    /// registry order. NOT one global sort — root 5's connectedness
    /// finding precedes root 1's separation finding, because each
    /// resident appends its own pass in sequence.
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
            crate::finding::render_list(f, &self.findings)?;
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
    /// The document's roots did not gather into a product, so the
    /// registry has no subject to run over. The gather's own refusal,
    /// forwarded — this layer has no second opinion about what a
    /// product is.
    ///
    /// The refusal of the door that GATHERS, and of no resident: a
    /// registry handed its subject never raises this.
    Product {
        /// The gather's own refusal, rendered — see
        /// [`CheckEvidence::SeparationUnavailable`] for why the message
        /// and not the value.
        reason: String,
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
            Self::Product { reason } => write!(f, "checks: {reason}"),
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
        crate::finding::render_list(f, &self.findings)
    }
}

impl core::error::Error for CheckRefusal {}

/// What the registry runs its residents over: the document's product,
/// gathered ONCE by whoever calls the registry.
///
/// A resident does not derive its own subject. Two residents deriving
/// the same subject differently would be two answers to "what is this
/// document's product", and a caller that already holds the product
/// would pay for it again.
///
/// [`Subject::NoBodyRoots`] is the document that denotes no body at
/// all — an empty document, or one holding only sketches and datums.
/// It is not a failure to run the registry: a resident that needs a
/// body has no subject there and contributes no finding, and a
/// resident that does not (connectedness reads the evaluation) runs
/// exactly as it would otherwise.
#[derive(Debug)]
pub enum Subject<'a, T: Decide> {
    /// The gathered product, borrowed for the run.
    Product(&'a product::Product<T>),
    /// No root denotes a body, so there is no product to be had.
    NoBodyRoots,
}

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
/// [`ChecksError`] — a root without a value in `ev`, a band the
/// tolerance cannot form, or a document whose roots do not gather into
/// a product. These mean the checks could not run at all; a check that
/// ran and disagreed is a FINDING, not an error.
pub fn run_checks<P, T: Decide + AtRestPolicy + CertifiedBounds>(
    doc: &Doc<P>,
    ev: &Evaluation<T>,
    cfg: &ChecksConfig,
    tol: Tol,
) -> Result<ChecksReport, ChecksError> {
    match product::product_recorded(doc, ev, tol) {
        Ok(gathered) => run_checks_on(doc, ev, Subject::Product(&gathered), cfg, tol),
        Err(product::ProductError::NoBodyRoots) => {
            run_checks_on(doc, ev, Subject::NoBodyRoots, cfg, tol)
        }
        Err(source) => {
            // PRECEDENCE: the registry's own preconditions answer
            // before the subject does. A root without a value in this
            // evaluation, or a tolerance that forms no band, is a
            // statement about whether the checks may run AT ALL, and
            // it holds whether or not the document also has a product.
            // The report that run would have produced is not this
            // call's answer and is dropped.
            run_checks_on(doc, ev, Subject::NoBodyRoots, cfg, tol)?;
            Err(ChecksError::Product {
                reason: source.to_string(),
            })
        }
    }
}

/// The registry over a subject the CALLER gathered — the door
/// [`run_checks`] wraps, and the one a caller with a product in hand
/// uses so the document is gathered once.
///
/// Everything [`run_checks`] documents about what is checked, in what
/// order, and what a finding means holds here verbatim; the only
/// difference is where the subject came from.
///
/// # Errors
///
/// [`ChecksError::Root`] and [`ChecksError::Band`] — the registry's own
/// preconditions. [`ChecksError::Product`] is [`run_checks`]'s alone:
/// this door was handed its subject and has no gather to refuse.
pub fn run_checks_on<P, T: Decide + AtRestPolicy + CertifiedBounds>(
    doc: &Doc<P>,
    ev: &Evaluation<T>,
    subject: Subject<'_, T>,
    cfg: &ChecksConfig,
    tol: Tol,
) -> Result<ChecksReport, ChecksError> {
    let mut report = ChecksReport::default();
    if cfg.severity(CheckId::Connectedness) == Severity::Off {
        report.skipped.push(CheckId::Connectedness);
    } else {
        connectedness(doc, ev, cfg, tol, &mut report)?;
    }
    if cfg.severity(CheckId::Separation) == Severity::Off {
        report.skipped.push(CheckId::Separation);
    } else {
        separation(&subject, tol, &mut report);
    }
    Ok(report)
}

/// The connectedness resident's own pass (I1(0b)) — [`run_checks`]'s
/// body before the registry grew a second resident, moved out
/// unchanged so each resident is independently `Off`-able.
fn connectedness<P, T: Decide + PropsQuadLane>(
    doc: &Doc<P>,
    ev: &Evaluation<T>,
    cfg: &ChecksConfig,
    tol: Tol,
    report: &mut ChecksReport,
) -> Result<(), ChecksError> {
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
    Ok(())
}

/// The separation resident's pass: every pair of gathered solids that
/// came from DIFFERENT root subjects, held to the kernel's box-level
/// separation certificate.
///
/// # What "different subjects" excludes, and why
///
/// Two solids of ONE root output are that node's own body, validated
/// on its own by the gather (`product`'s per-source tier-3 pass) and
/// the responsibility of whatever constructed it — a pattern's
/// instances, a multi-solid import. The gather did not put them
/// together, so their relationship is not the gather's to answer for.
/// What the gather DID do is place different roots' bodies in one
/// aggregate without establishing that they may share the space, and
/// that is exactly the pair set this walks.
///
/// # Determinism (D9)
///
/// `solid_roots` is in gather order, which is root-list order then
/// output order then the source body's own solid order. The walk is
/// `i < j` over that list, so the findings come out in a stable order
/// that does not depend on arena iteration luck.
///
/// # Cost, and the shape it would grow if it mattered
///
/// One box per face, one small tree per solid, then a hull test per
/// cross-subject pair — quadratic in the SOLID count, not in the
/// entity count, which is the whole reason this resident exists
/// instead of running the tier-3′ census over the aggregate.
///
/// **The two terms are separable and separately measured**, because
/// the registry no longer gathers its own subject: over the corpus
/// heat sink at 160 fins (161 solids / 991 faces), the gather is
/// ~250 ms and this registry over a subject already in hand is ~8 ms
/// — the gather dominates by more than an order of magnitude, and a
/// caller that already holds the product pays only the second term.
/// The tier-3′ census over the same aggregate is the ~1.1 s figure
/// this resident exists to avoid.
///
/// Those numbers are a dev-profile wall clock and are machine-
/// dependent; the figures OF RECORD are the hosted ones, re-taken on
/// every merge to `main` by the `registry split` row of
/// `crates/editor-core/tests/m4_pr8_latency.rs` and appended to
/// `docs/perf-data/rebuild-latency/`. The SIZE they are taken at is
/// exact rather than measured and gates on every PR
/// (`docm5_subject::the_registry_split_is_measured_at_a_pinned_point`).
///
/// A document with solids in the thousands would make the pair walk
/// the term that matters, and the fix is already sitting here — one
/// `Bvh` over the per-solid hulls, queried instead of the `S²` loop.
/// Not built, because nothing has measured it as the bottleneck.
///
/// # Declared contact suppresses
///
/// A pair whose contact the product DECLARES is not reported: a mate
/// that says two faces rest on each other has said the solids touch,
/// and the assembly door ([`crate::assemble`]) is where that
/// declaration is certified against the geometry. Reporting it here
/// too would make a correctly-mated assembly noisy about the thing it
/// got right. The suppression reads the declarations only — it never
/// blesses a contact from discovery (F1).
fn separation<T: Decide + CertifiedBounds>(
    subject: &Subject<'_, T>,
    tol: Tol,
    report: &mut ChecksReport,
) {
    // A document whose roots denote no body has no SUBJECT for this
    // resident, so it contributes no finding — the reading
    // [`Subject::NoBodyRoots`] states.
    let Subject::Product(gathered) = subject else {
        return;
    };
    // Fewer than two gathered solids cannot make a pair.
    if gathered.solid_roots.len() < 2 {
        return;
    }
    let boxes = match topo::SolidSeparation::of(&gathered.body, tol) {
        Ok(boxes) => boxes,
        Err(source) => {
            // No pair has a verdict. One finding against the first
            // subject says so rather than a silent clean report (F6).
            let first = gathered.solid_roots[0];
            report.findings.push(CheckFinding {
                check: CheckId::Separation,
                root: first.node,
                output_ix: first.output,
                evidence: CheckEvidence::separation_unavailable(&source),
            });
            return;
        }
    };
    let declared = declared_pairs(gathered);
    for (j, later) in gathered.solid_roots.iter().enumerate() {
        for earlier in &gathered.solid_roots[..j] {
            if (earlier.node, earlier.output) == (later.node, later.output) {
                continue;
            }
            if boxes.certify(earlier.solid, later.solid).is_ok() {
                continue;
            }
            let pair = if earlier.solid <= later.solid {
                (earlier.solid, later.solid)
            } else {
                (later.solid, earlier.solid)
            };
            if declared.contains(&pair) {
                continue;
            }
            report.findings.push(CheckFinding {
                check: CheckId::Separation,
                root: earlier.node,
                output_ix: earlier.output,
                evidence: CheckEvidence::NotSeparated {
                    other_root: later.node,
                    other_output: later.output,
                },
            });
        }
    }
}

/// Which solid pairs are DECLARED to touch, keyed as an ordered pair
/// of solid keys.
///
/// Every record kind that names two entities contributes the pair of
/// solids those entities live in. A record whose entity the aggregate
/// cannot resolve contributes nothing — the strict direction: an
/// unresolvable declaration suppresses no finding, so a broken record
/// can only make this check louder, never quieter.
fn declared_pairs<T: Decide>(
    gathered: &product::Product<T>,
) -> std::collections::BTreeSet<(topo::SolidKey, topo::SolidKey)> {
    // The gather's own record set is the whole answer: it holds the
    // records that rode UP from the source bodies AND this document's
    // mate-minted declarations, because minting is the gather's act.
    // Re-minting here would be a second opinion about what a mate
    // declares, and a duplicate record besides.
    //
    // A mate the gather could not mint declares nothing and so
    // suppresses nothing — `gathered.unminted` is not consulted, and
    // the honest reading is that a broken declaration makes this
    // resident LOUDER about the pair it named, never quieter. The
    // unresolvable-ENTITY direction is `note`'s doing below.
    let contacts = &gathered.contacts;
    let owner = topo::SolidOwners::of(&gathered.body);
    let mut out = std::collections::BTreeSet::new();
    let mut note = |a: Option<topo::SolidKey>, b: Option<topo::SolidKey>| {
        if let (Some(a), Some(b)) = (a, b)
            && a != b
        {
            out.insert(if a <= b { (a, b) } else { (b, a) });
        }
    };
    for c in &contacts.vv {
        note(owner.vertex(c.a), owner.vertex(c.b));
    }
    for c in contacts.a_on_b.iter().chain(&contacts.b_on_a) {
        note(owner.vertex(c.vertex), owner.face(c.face));
    }
    for c in &contacts.curves {
        note(owner.face(c.face_a), owner.face(c.face_b));
    }
    for c in &contacts.patches {
        note(owner.face(c.face_a), owner.face(c.face_b));
    }
    out
}

/// The door from a finding's attribution back to its subject: the body
/// at `(root, output_ix)` in this evaluation, AND the declared contact
/// records that body carries — the same enumeration [`run_checks`]
/// walks, so a [`CheckFinding`]'s attribution always resolves against
/// the evaluation it was produced from. `None` when the root has no
/// value, denotes no body, or has no output at that index (exactly the
/// attributions a [`CheckEvidence::StaleExpectation`] finding names).
///
/// The two halves travel TOGETHER because a body without its
/// declarations is a different subject from the one the check flagged.
/// A record set has two homes — [`crate::eval::NodeValue::contacts`]
/// for an instantiate's carried D-1 declarations, `BooleanValue::
/// contacts` for a boolean's own — and reconciling those two is
/// precisely what [`product::sources_of`] exists to do; it builds the
/// pair, and this door hands the whole pair on rather than dropping
/// half of it. Splitting them again downstream re-opens the failure
/// this signature closes: a subject that IS a declared boolean result
/// reporting its own certified seam as an UNDECLARED contact under the
/// tier-3′ gate, while the identical body read through its value
/// passes. (That direction fails loud, never silently — an absent
/// record set can only make the gate refuse — but a loud wrong answer
/// is still a wrong answer.)
///
/// A caller wanting only the body says so with a `.map(|(body, _)| …)`
/// at its own site, where dropping the records is a visible decision
/// rather than this door's silent narrowing.
pub fn subject_body<T: Decide>(
    ev: &Evaluation<T>,
    root: RecipeNodeId,
    output_ix: u32,
) -> Option<(Arc<Body<T>>, Arc<ContactRecords>)> {
    let sources = product::sources_of(ev.value(root)?)?;
    sources
        .into_iter()
        .find(|(ix, _, _)| *ix == output_ix)
        .map(|(_, body, contacts)| (body, contacts))
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::CheckEvidence;

    /// INVARIANT: the separation door's evidence carries the class and
    /// the prose OF ONE REFUSAL — `kind` is the arm the error actually
    /// is, not a class written down beside it.
    ///
    /// The refusing branch itself cannot be reached from `run_checks`
    /// over a well-formed document: every refusal
    /// `topo::SolidSeparation::of` can raise needs either a corrupt
    /// body or an ε within a factor K of `f64::MAX`, and `Tol` is a
    /// zero-sized witness for the run's committed tolerance, so a test
    /// cannot hand it one. This row therefore pins the door's
    /// CONSTRUCTION, which is the part a caller can get wrong, and says
    /// so rather than implying the branch was executed.
    #[test]
    fn the_separation_door_carries_the_class_of_the_error_it_saw() {
        let refusal = topo::BooleanError::ClassificationInvariant {
            what: "solid separation: the ambient tolerance band is unusable",
        };
        let CheckEvidence::SeparationUnavailable { kind, reason } =
            CheckEvidence::separation_unavailable(&refusal)
        else {
            panic!("the arm this row is about");
        };
        // The reader's half is the kernel's own sentence, whole.
        assert_eq!(reason, refusal.to_string());
        // The consumer's half is the arm the error IS — compared
        // against the variant name `Debug` prints for the error, so a
        // class hardcoded here would have to be the right one by
        // accident to pass.
        let variant: String = format!("{refusal:?}")
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        assert_eq!(format!("{kind:?}"), variant);
    }
}
