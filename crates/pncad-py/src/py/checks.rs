//! **The advisory-check registry** (DISCIPLINES-DESIGN DS6).
//!
//! `run_checks` REPORTS. Nothing in this module gates: no door here is
//! called from `apply`, from `load`, or from `evaluate`, and running
//! the registry changes nothing about a document or an evaluation. A
//! finding is a VALUE the caller reads.
//!
//! `enforce_checks` is the ONE refusing path, and it refuses only on
//! what the CALLER configured at `Severity.Error`. It consumes a
//! finished report — it evaluates nothing, checks nothing further, and
//! can change nothing — so "where does this program gate" stays a
//! question about the caller's code and never about the kernel's.
//! That is the same split the Rust façade curates, kept whole across
//! the language boundary: the report door answers a value, the gate
//! door raises a typed refusal, and no door does both.
//!
//! # Not in the prelude, and the Python analogue of that
//!
//! The façade curates these names into `pncad::document` and
//! deliberately NOT into the prelude, because prelude membership there
//! is corpus-measured: a `use pncad::prelude::*` consumer does not
//! silently acquire the checks vocabulary. Python has one flat module
//! and no second, narrower door, so the argument crosses as
//! DISCOVERABILITY rather than as a second namespace — nothing in the
//! authoring path mentions the registry, `evaluate` does not run it,
//! and a caller reaches these names by asking for them. A program that
//! never says `run_checks` is a program the registry never touches.
//!
//! # The two knobs are two types, and that is DS6's waiver rule
//!
//! `connectedness` takes a `Severity` (`Off`/`Warn`/`Error`);
//! `separation` takes an `Advisory` (`Off`/`Warn`). DS6 lets a check
//! offer `error` **iff** it ships a per-finding acknowledgment record
//! with a staleness direction. Connectedness has one —
//! `expected_components`, whose unconsumed entries come back as
//! `stale_expectation` findings. The separation resident ships
//! nothing analogous, so its refusing position is UNREPRESENTABLE
//! rather than merely undocumented, in Python exactly as in Rust.

use std::collections::BTreeMap;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::ErrorClass;
use crate::py::typed_err;
use crate::tags::{check_evidence_tag, checks_error_tag};
use pncad::document as d;
use pncad::tolerance::Tol;

use super::doc::{Doc, NodeId};
use super::value::{Body, Evaluation};

/// Which check a finding came from — the registry's closed set.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::CheckId` variant of the same name"
)]
pub(crate) enum CheckId {
    Connectedness,
    Separation,
}

/// The certified/heuristic label (DS6): honesty of language and a
/// default level, never a force cap.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::CheckKind` variant of the same name"
)]
pub(crate) enum CheckKind {
    Certified,
    Heuristic,
}

/// A check's severity knob: off, warn, or refuse at the gate.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::Severity` variant of the same name"
)]
pub(crate) enum Severity {
    Off,
    Warn,
    Error,
}

/// The knob of a resident that may not refuse — `Severity` minus
/// `Error`.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::Advisory` variant of the same name"
)]
pub(crate) enum Advisory {
    Off,
    Warn,
}

/// Crossing helper: the kernel check id as the Python mirror.
/// Exhaustive over the KERNEL enum with no wildcard arm, so a resident
/// this binding does not mirror stops the build.
fn check_id(check: d::CheckId) -> CheckId {
    match check {
        d::CheckId::Connectedness => CheckId::Connectedness,
        d::CheckId::Separation => CheckId::Separation,
    }
}

/// Crossing helper: the kernel label as the Python mirror.
fn check_kind(kind: d::CheckKind) -> CheckKind {
    match kind {
        d::CheckKind::Certified => CheckKind::Certified,
        d::CheckKind::Heuristic => CheckKind::Heuristic,
    }
}

#[pymethods]
impl CheckId {
    /// This check's honesty label — whether its finding is a theorem
    /// or a judgment.
    ///
    /// Read from the kernel's own table, never restated here, so the
    /// label a Python caller sees cannot disagree with the one the
    /// check ships under.
    #[getter]
    fn kind(&self) -> CheckKind {
        check_kind(self.to_kernel().kind())
    }

    fn __str__(&self) -> String {
        self.to_kernel().to_string()
    }
}

impl CheckId {
    /// The kernel id this mirror names. Total over the MIRROR, the
    /// safe direction: every variant Python can spell has a kernel
    /// counterpart.
    fn to_kernel(self) -> d::CheckId {
        match self {
            Self::Connectedness => d::CheckId::Connectedness,
            Self::Separation => d::CheckId::Separation,
        }
    }
}

impl Severity {
    /// The kernel knob this mirror names.
    fn to_kernel(self) -> d::Severity {
        match self {
            Self::Off => d::Severity::Off,
            Self::Warn => d::Severity::Warn,
            Self::Error => d::Severity::Error,
        }
    }

    /// The Python mirror of a kernel knob.
    fn from_kernel(sev: d::Severity) -> Self {
        match sev {
            d::Severity::Off => Self::Off,
            d::Severity::Warn => Self::Warn,
            d::Severity::Error => Self::Error,
        }
    }
}

impl Advisory {
    /// The kernel knob this mirror names.
    fn to_kernel(self) -> d::Advisory {
        match self {
            Self::Off => d::Advisory::Off,
            Self::Warn => d::Advisory::Warn,
        }
    }

    /// The Python mirror of a kernel knob.
    fn from_kernel(adv: d::Advisory) -> Self {
        match adv {
            d::Advisory::Off => Self::Off,
            d::Advisory::Warn => Self::Warn,
        }
    }
}

/// Per-run check configuration: a plain argument to the door, with no
/// ambient state and nothing persisted.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct ChecksConfig(pub(crate) d::ChecksConfig);

#[pymethods]
impl ChecksConfig {
    /// The DS6 defaults are the no-argument form: both residents at
    /// `Warn`, nothing expected, nothing refused.
    ///
    /// `expected_components` is a list of `(root, output_ix,
    /// expected)` triples — the connectedness resident's
    /// acknowledgment record, keyed by the same `(root, output_ix)`
    /// pair a finding is attributed to. It is an ordered list rather
    /// than a dict because a `NodeId` pair is not a Python key: the
    /// entries are read in order and never looked up by one id (the
    /// `SplitOutcome.node_map` spelling). A key stated twice is
    /// REFUSED rather than resolved last-wins — two expectations of
    /// one subject is a mistake, and silently keeping one of them
    /// would report "checked and fine" about a number the caller did
    /// not state.
    #[new]
    #[pyo3(signature = (connectedness = Severity::Warn, expected_components = None, separation = Advisory::Warn))]
    fn new(
        connectedness: Severity,
        expected_components: Option<Vec<(NodeId, u32, u32)>>,
        separation: Advisory,
    ) -> PyResult<Self> {
        let mut expected: BTreeMap<(d::RecipeNodeId, u32), u32> = BTreeMap::new();
        for (root, output_ix, count) in expected_components.unwrap_or_default() {
            if expected.insert((root.0, output_ix), count).is_some() {
                return Err(PyValueError::new_err(format!(
                    "expected_components states subject (node {}, output {output_ix}) twice",
                    root.0.0
                )));
            }
        }
        Ok(Self(d::ChecksConfig {
            connectedness: connectedness.to_kernel(),
            expected_components: expected,
            separation: separation.to_kernel(),
        }))
    }

    /// The connectedness resident's knob.
    #[getter]
    fn connectedness(&self) -> Severity {
        Severity::from_kernel(self.0.connectedness)
    }

    /// The separation resident's knob — an `Advisory`, so `Error` is
    /// not spellable here.
    #[getter]
    fn separation(&self) -> Advisory {
        Advisory::from_kernel(self.0.separation)
    }

    /// The stated expectations, ascending by subject.
    #[getter]
    fn expected_components(&self) -> Vec<(NodeId, u32, u32)> {
        self.0
            .expected_components
            .iter()
            .map(|((root, output_ix), count)| (NodeId(*root), *output_ix, *count))
            .collect()
    }

    /// The severity this configuration gives `check` — the widening
    /// `Advisory` goes through, so a caller reads one vocabulary.
    fn severity(&self, check: CheckId) -> Severity {
        Severity::from_kernel(self.0.severity(check.to_kernel()))
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        format!(
            "ChecksConfig(connectedness={:?}, expected_components={}, separation={:?})",
            self.0.connectedness,
            self.0.expected_components.len(),
            self.0.separation
        )
    }
}

/// What one finding found, as a value with a stable `variant` tag and
/// the arm's payload as attributes.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct CheckEvidence(pub(crate) d::CheckEvidence);

#[pymethods]
impl CheckEvidence {
    /// The stable tag of the arm that fired.
    #[getter]
    fn variant(&self) -> &'static str {
        check_evidence_tag(&self.0)
    }

    /// Components actually found, on `connectedness` alone.
    #[getter]
    fn actual(&self) -> Option<u32> {
        match &self.0 {
            d::CheckEvidence::Connectedness { actual, .. } => Some(*actual),
            _ => None,
        }
    }

    /// The expectation this subject was held to — on `connectedness`,
    /// and on `stale_expectation`, where it is the entry nothing
    /// consumed.
    #[getter]
    fn expected(&self) -> Option<u32> {
        match &self.0 {
            d::CheckEvidence::Connectedness { expected, .. }
            | d::CheckEvidence::StaleExpectation { expected } => Some(*expected),
            _ => None,
        }
    }

    /// The counterpart subject's root, on `not_separated` alone. The
    /// finding's own `root`/`output_ix` is the first subject in gather
    /// order; this is the second.
    #[getter]
    fn other_root(&self) -> Option<NodeId> {
        match &self.0 {
            d::CheckEvidence::NotSeparated { other_root, .. } => Some(NodeId(*other_root)),
            _ => None,
        }
    }

    /// The counterpart subject's output index, on `not_separated`.
    #[getter]
    fn other_output(&self) -> Option<u32> {
        match &self.0 {
            d::CheckEvidence::NotSeparated { other_output, .. } => Some(*other_output),
            _ => None,
        }
    }

    /// The underlying refusal's own prose, where the arm carries one:
    /// the shell door's on `escalated` and `unsupported`, the box
    /// builder's on `separation_unavailable`. Prose because it is the
    /// kernel's own story, and `variant` is the branchable part.
    #[getter]
    fn reason(&self) -> Option<String> {
        match &self.0 {
            d::CheckEvidence::Escalated { source } | d::CheckEvidence::Unsupported { source } => {
                Some(source.to_string())
            }
            d::CheckEvidence::SeparationUnavailable { reason } => Some(reason.clone()),
            _ => None,
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        format!("CheckEvidence({:?})", check_evidence_tag(&self.0))
    }
}

/// One finding of one check on one subject — a body-denoting root
/// output, attributed as `(root, output_ix)`.
///
/// A finding is a REPORT about geometry, not a verdict on the program:
/// reaching one changes nothing, and whether it stops anything is the
/// caller's `Severity` and the caller's `enforce_checks` call.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct CheckFinding(pub(crate) d::CheckFinding);

#[pymethods]
impl CheckFinding {
    /// The check that fired.
    #[getter]
    fn check(&self) -> CheckId {
        check_id(self.0.check)
    }

    /// The root whose value carries the subject body.
    #[getter]
    fn root(&self) -> NodeId {
        NodeId(self.0.root)
    }

    /// Which output body of that root — 0 for a single-body root.
    #[getter]
    fn output_ix(&self) -> u32 {
        self.0.output_ix
    }

    /// What was found.
    #[getter]
    fn evidence(&self) -> CheckEvidence {
        CheckEvidence(self.0.evidence.clone())
    }

    /// The finding as the library renders one: its subject, then the
    /// kernel's own story. The recourse is in that story; nothing is
    /// appended here.
    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        format!(
            "CheckFinding({}, node {}, output {}, {:?})",
            self.0.check,
            self.0.root.0,
            self.0.output_ix,
            check_evidence_tag(&self.0.evidence)
        )
    }
}

/// One run's report: the findings, and the checks that did not run.
///
/// `skipped` is why this is a report and not a list: "checked and
/// fine" and "not checked" are different answers, and a caller reading
/// an empty `findings` without reading `skipped` has confused them.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct ChecksReport(pub(crate) d::ChecksReport);

#[pymethods]
impl ChecksReport {
    /// The findings, in the registry's deterministic order: each
    /// resident's own pass by root-list position then output index,
    /// residents in registry order. NOT one global sort.
    #[getter]
    fn findings(&self) -> Vec<CheckFinding> {
        self.0.findings.iter().cloned().map(CheckFinding).collect()
    }

    /// The checks that did not run because the caller set them `Off`.
    #[getter]
    fn skipped(&self) -> Vec<CheckId> {
        self.0.skipped.iter().copied().map(check_id).collect()
    }

    /// How many findings — `len(report)`.
    fn __len__(&self) -> usize {
        self.0.findings.len()
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        format!(
            "ChecksReport({} finding(s), {} skipped)",
            self.0.findings.len(),
            self.0.skipped.len()
        )
    }
}

/// Raise `ChecksError` carrying the refusal's stable tag and the arm's
/// own payload.
///
/// Every attribute is set on every arm, `None` where the arm does not
/// carry it — the `WorkspaceError` posture: handling reads `err.node`
/// without first branching on `err.variant`.
fn checks_err(py: Python<'_>, err: &d::ChecksError) -> PyErr {
    let none = || py.None();
    let node = match err {
        d::ChecksError::Root { node } => Py::new(py, NodeId(*node))
            .map(|v| v.into_any())
            .unwrap_or_else(|_| py.None()),
        _ => none(),
    };
    typed_err(
        py,
        ErrorClass::Checks,
        err.to_string(),
        &[
            (
                "variant",
                PyString::new(py, checks_error_tag(err)).unbind().into_any(),
            ),
            ("node", node),
        ],
    )
}

/// **Run every configured check over an evaluated document and report.**
///
/// REPORTS, NEVER GATES. Nothing here refuses on a finding, nothing
/// re-evaluates, and nothing about `doc` or `evaluation` changes: the
/// evaluation is read. A document with findings is a document that
/// still draws, still measures and still exports — the finding is the
/// only thing that says the almost-right picture is wrong, and taking
/// action on it is the caller's move (`enforce_checks`).
///
/// `evaluation` must be an evaluation OF `doc`, and one that ran to
/// completion: subjects are the body-denoting outputs of the
/// document's roots, so a root that failed, was poisoned, or was cut
/// short refuses `root_without_value` rather than reporting over a
/// partial evaluation.
///
/// The residents:
///
/// * `Connectedness` — a subject's component count, counted EXACTLY as
///   its number of outer shells (an internal void is boundary, not a
///   component, at every tolerance), compared with `!=` against
///   `ChecksConfig.expected_components` (default 1). A shell the
///   orientation read cannot decide surfaces as its own `escalated` /
///   `unsupported` finding, never as a silent skip.
/// * `Separation` — every pair of gathered solids the product took
///   from DIFFERENT roots, held to the kernel's box-level disjointness
///   certificate. What a finding denies is the CERTIFICATE, never
///   "these two overlap": a pair that merely touches lands here too.
///
/// Expectations are TWO-DIRECTIONAL: an `expected_components` entry no
/// subject consumed — a vanished body, a dead root, a mistyped key —
/// comes back as its own `stale_expectation` finding. A stale
/// acknowledgment must not read as "checked and fine".
///
/// Raises `ChecksError`, typed, when the checks could not RUN at all.
/// A check that ran and disagreed is a finding in the report and never
/// an exception.
#[pyfunction]
#[pyo3(signature = (doc, evaluation, config = None))]
pub(crate) fn run_checks(
    py: Python<'_>,
    doc: &Doc,
    evaluation: &Evaluation,
    config: Option<&ChecksConfig>,
) -> PyResult<ChecksReport> {
    let tol = Tol::witness();
    let default = d::ChecksConfig::default();
    let cfg = config.map_or(&default, |c| &c.0);
    d::run_checks(&doc.inner, &evaluation.inner, cfg, tol)
        .map(ChecksReport)
        .map_err(|err| checks_err(py, &err))
}

/// **The registry's one refusing path.** Refuses iff `report` carries a
/// finding whose check `config` sets to `Severity.Error`.
///
/// The CALLER chooses where to gate — this is the door that lets them,
/// and it is the only one. It consumes a FINISHED report: it evaluates
/// nothing, checks nothing further, and can change nothing. `Warn`
/// findings pass and stay in the report for the caller to render.
///
/// Under the default configuration this can never refuse: no resident
/// defaults to `Error`, and the separation resident cannot be set to
/// it at all (its knob is `Advisory`, DS6's waiver rule as a type). A
/// program that gates does so because it SAID `Severity.Error`.
///
/// Raises `CheckRefusal`, carrying `findings` — every `Error`-severity
/// finding, in report order.
#[pyfunction]
#[pyo3(signature = (report, config = None))]
pub(crate) fn enforce_checks(
    py: Python<'_>,
    report: &ChecksReport,
    config: Option<&ChecksConfig>,
) -> PyResult<()> {
    let default = d::ChecksConfig::default();
    let cfg = config.map_or(&default, |c| &c.0);
    d::enforce_checks(&report.0, cfg).map_err(|refusal| {
        let findings = refusal
            .findings
            .iter()
            .cloned()
            .map(CheckFinding)
            .collect::<Vec<_>>()
            .into_pyobject(py)
            .map(|v| v.unbind().into_any())
            .unwrap_or_else(|_| py.None());
        typed_err(
            py,
            ErrorClass::Enforce,
            refusal.to_string(),
            &[("findings", findings)],
        )
    })
}

/// The door from a finding's attribution back to its subject: the body
/// at `(root, output_ix)` in this evaluation.
///
/// A finding names a subject and this resolves it, against the same
/// enumeration `run_checks` walks — so a finding's attribution always
/// resolves in the evaluation it was produced from. `None` where the
/// root has no value, denotes no body, or has no output at that index:
/// exactly the attributions a `stale_expectation` finding names, which
/// is what makes that arm's `None` an answer rather than a failure.
#[pyfunction]
pub(crate) fn subject_body(evaluation: &Evaluation, root: &NodeId, output_ix: u32) -> Option<Body> {
    d::subject_body(&evaluation.inner, root.0, output_ix).map(|inner| Body { inner })
}

/// Register the advisory-check registry on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CheckId>()?;
    m.add_class::<CheckKind>()?;
    m.add_class::<Severity>()?;
    m.add_class::<Advisory>()?;
    m.add_class::<ChecksConfig>()?;
    m.add_class::<CheckEvidence>()?;
    m.add_class::<CheckFinding>()?;
    m.add_class::<ChecksReport>()?;
    m.add_function(wrap_pyfunction!(run_checks, m)?)?;
    m.add_function(wrap_pyfunction!(enforce_checks, m)?)?;
    m.add_function(wrap_pyfunction!(subject_body, m)?)?;
    Ok(())
}
