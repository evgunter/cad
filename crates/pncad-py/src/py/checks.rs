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
//! # The knobs are two types, and that is DS6's waiver rule
//!
//! `connectedness` and `at_rest` take a `Severity`
//! (`Off`/`Warn`/`Error`); `chart_coherence` and
//! `unproven_coincidence` take an `Advisory` (`Off`/`Warn`). DS6 lets
//! a check offer `error` **iff** it ships a per-finding acknowledgment
//! record with a staleness direction. Connectedness has one —
//! `expected_components`, whose unconsumed entries come back as
//! `stale_expectation` findings — and the at-rest check has the
//! assertion that quiets an interference. The advisory residents ship
//! nothing analogous, so their refusing position is UNREPRESENTABLE
//! rather than merely undocumented, in Python exactly as in Rust.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::Arc;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::ErrorClass;
use crate::py::typed_err;
use crate::tags::{check_evidence_tag, checks_error_tag};
use pncad::document as d;
use pncad::tolerance::Tol;

use super::assembly::{AtRestEvidence, CarriedInterference, InterferenceFinding};
use super::doc::{Doc, NodeId, Var};
use super::value::{Body, Evaluation};

/// Which check a finding came from — the registry's closed set.
// EVERY fieldless mirror in this crate carries these four options,
// and this is the one place the reason is written down. A mirror is a
// TAG: `eq`/`eq_int` make it comparable, and a comparable value that
// does not hash is unusable as a set element or a dict key, because
// Python leaves `__hash__` unset on any type that defines `__eq__`.
// PyO3 spells `hash` as requiring `frozen` alongside `eq`, which a
// fieldless mirror satisfies for nothing: it has no fields to mutate.
// The hash is over the variant, so it agrees with the comparison by
// construction — both read the same discriminant.
#[pyclass(eq, eq_int, frozen, hash, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::CheckId` variant of the same name"
)]
pub(crate) enum CheckId {
    Connectedness,
    ChartCoherence,
    UnprovenCoincidence,
    AtRest,
    Unused,
}

/// The certified/heuristic label (DS6): honesty of language and a
/// default level, never a force cap.
#[pyclass(eq, eq_int, frozen, hash, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::CheckKind` variant of the same name"
)]
pub(crate) enum CheckKind {
    Certified,
    Heuristic,
}

/// A check's severity knob: off, warn, or refuse at the gate.
#[pyclass(eq, eq_int, frozen, hash, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
#[pyclass(eq, eq_int, frozen, hash, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
        d::CheckId::ChartCoherence => CheckId::ChartCoherence,
        d::CheckId::UnprovenCoincidence => CheckId::UnprovenCoincidence,
        d::CheckId::AtRest => CheckId::AtRest,
        d::CheckId::Unused => CheckId::Unused,
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
            Self::ChartCoherence => d::CheckId::ChartCoherence,
            Self::UnprovenCoincidence => d::CheckId::UnprovenCoincidence,
            Self::AtRest => d::CheckId::AtRest,
            Self::Unused => d::CheckId::Unused,
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
    /// The DS6 defaults are the no-argument form: the three residents
    /// of the default pass at `Warn`, the chart-coherence examination
    /// `Off`, nothing expected, nothing refused.
    ///
    /// `expected_components` is a list of `(placement, output_ix,
    /// expected)` triples — the connectedness resident's
    /// acknowledgment record, keyed by the same `(placement, output_ix)`
    /// pair a finding is attributed to. It is an ordered list rather
    /// than a dict because a `NodeId` pair is not a Python key: the
    /// entries are read in order and never looked up by one id (the
    /// `SplitOutcome.node_map` spelling). A key stated twice is
    /// REFUSED rather than resolved last-wins — two expectations of
    /// one subject is a mistake, and silently keeping one of them
    /// would report "checked and fine" about a number the caller did
    /// not state.
    #[new]
    #[pyo3(signature = (connectedness = Severity::Warn, expected_components = None, chart_coherence = Advisory::Off, unproven_coincidence = Advisory::Warn, at_rest = Severity::Warn, unused = Advisory::Warn))]
    fn new(
        connectedness: Severity,
        expected_components: Option<Vec<(NodeId, u32, u32)>>,
        chart_coherence: Advisory,
        unproven_coincidence: Advisory,
        at_rest: Severity,
        unused: Advisory,
    ) -> PyResult<Self> {
        let mut expected: BTreeMap<(d::RecipeNodeId, u32), u32> = BTreeMap::new();
        for (NodeId(placement), output_ix, count) in expected_components.unwrap_or_default() {
            if expected.insert((placement, output_ix), count).is_some() {
                return Err(PyValueError::new_err(format!(
                    "expected_components states subject (node {placement}, output {output_ix}) twice"
                )));
            }
        }
        Ok(Self(d::ChecksConfig {
            connectedness: connectedness.to_kernel(),
            expected_components: expected,
            chart_coherence: chart_coherence.to_kernel(),
            unproven_coincidence: unproven_coincidence.to_kernel(),
            at_rest: at_rest.to_kernel(),
            unused: unused.to_kernel(),
        }))
    }

    /// The connectedness resident's knob.
    #[getter]
    fn connectedness(&self) -> Severity {
        Severity::from_kernel(self.0.connectedness)
    }

    /// The chart-coherence resident's knob — an `Advisory`, so
    /// `Error` is not spellable here either, and `Off` by default:
    /// the resident reads every face of every rest body and reports a
    /// trimmed face's loops as out of its reach, so it is asked for
    /// rather than paid for on every run.
    #[getter]
    fn chart_coherence(&self) -> Advisory {
        Advisory::from_kernel(self.0.chart_coherence)
    }

    /// The `unproven-coincidence` lint's knob — an `Advisory`: an
    /// unproven coincidence is quieted by an assertion at its site,
    /// not by this report, so `Error` is not spellable here.
    #[getter]
    fn unproven_coincidence(&self) -> Advisory {
        Advisory::from_kernel(self.0.unproven_coincidence)
    }

    /// The at-rest check's knob — a full `Severity`: an assertion
    /// quiets every interference it can bound. At `Error`,
    /// `enforce_checks` refuses a loud interference and nothing else
    /// the check reports.
    #[getter]
    fn at_rest(&self) -> Severity {
        Severity::from_kernel(self.0.at_rest)
    }

    /// The `unused` check's knob — an `Advisory`: an unused thing is
    /// answered by using it or deleting it, so `Error` is not spellable.
    #[getter]
    fn unused(&self) -> Advisory {
        Advisory::from_kernel(self.0.unused)
    }

    /// The stated expectations, ascending by subject.
    #[getter]
    fn expected_components(&self) -> Vec<(NodeId, u32, u32)> {
        self.0
            .expected_components
            .iter()
            .map(|((placement, output_ix), count)| (NodeId(*placement), *output_ix, *count))
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
            "ChecksConfig(connectedness={:?}, expected_components={}, \
             chart_coherence={:?}, unproven_coincidence={:?}, at_rest={:?}, unused={:?})",
            self.0.connectedness,
            self.0.expected_components.len(),
            self.0.chart_coherence,
            self.0.unproven_coincidence,
            self.0.at_rest,
            self.0.unused
        )
    }
}

/// What one finding found, as a value with a stable `variant` tag and
/// the arm's payload as attributes.
///
/// Every payload attribute is present on every arm, `None` where the
/// arm does not carry it: `actual`, `expected`, `reason`,
/// `inner_variant`, `at_rest_variant`, `chart_variant`, `metres`,
/// `gap`, `lever`, `eps`, `relation`, `site`; and the values an arm
/// carries whole, `coincidence`, `restated`, `interference`,
/// `carried_interference` and `at_rest`.
///
/// They read off ONE record, [`crate::check_payload`], whose
/// match over the kernel enum is exhaustive with no wildcard: an
/// evidence arm added there is a compile error rather than a finding
/// every accessor here silently answers `None` about.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct CheckEvidence(
    pub(crate) d::CheckEvidence,
    /// The document the checks ran over, which names a coincidence's
    /// cells.
    Arc<d::ProfileDoc>,
);

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
        self.payload().actual
    }

    /// The expectation this subject was held to — on `connectedness`,
    /// and on `stale_expectation`, where it is the entry nothing
    /// consumed.
    #[getter]
    fn expected(&self) -> Option<u32> {
        self.payload().expected
    }

    /// The underlying refusal's own prose, where the arm carries one:
    /// the shell door's on `escalated` and `unsupported`, the
    /// validator's on `not_at_rest`, the residual's on
    /// `unproven_coincidence`, and on `at_rest` why the check could not
    /// look. Prose because it is the kernel's own story, and `variant`
    /// is the branchable part.
    #[getter]
    fn reason(&self) -> Option<String> {
        self.payload().reason.map(Cow::into_owned)
    }

    /// The inner refusal's own word, where the arm carries one: the
    /// shell door's on `escalated` and `unsupported` (`band`, `props`,
    /// `escalated`, `zero_volume`, `straddles`, `ray_in_band`,
    /// `corner`, `ray` or `sign_unresolved`), the validator's first
    /// finding's on `not_at_rest`, and on `at_rest` the validator's or
    /// the gather's (`AtRestEvidence.inner_variant`). `reason` is the
    /// same refusal's sentence; this is the part a caller matches on.
    #[getter]
    fn inner_variant(&self) -> Option<&'static str> {
        self.payload().inner_variant
    }

    /// Why the at-rest check could not look, on `at_rest` alone:
    /// `AtRestEvidence.variant`.
    #[getter]
    fn at_rest_variant(&self) -> Option<&'static str> {
        self.payload().at_rest_variant
    }

    /// The overlap, on `interference` alone: two arrivals of one `⊔`
    /// whose material overlaps at rest, quiet or loud.
    #[getter]
    fn interference(&self, py: Python<'_>) -> PyResult<Option<InterferenceFinding>> {
        match &self.0 {
            d::CheckEvidence::Interference { finding } => {
                InterferenceFinding::of(py, &self.1, finding).map(Some)
            }
            _ => Ok(None),
        }
    }

    /// A part's overlap carried up, on `carried_interference` alone:
    /// the part's own finding, quiet or loud as the part judged it.
    #[getter]
    fn carried_interference(&self, py: Python<'_>) -> PyResult<Option<CarriedInterference>> {
        match &self.0 {
            d::CheckEvidence::CarriedInterference { finding } => {
                CarriedInterference::read(py, finding).map(Some)
            }
            _ => Ok(None),
        }
    }

    /// A part's unproven coincidence carried up, on
    /// `carried_coincidence` alone: the part's own finding, as its own
    /// door judged it.
    #[getter]
    fn carried_coincidence(&self) -> Option<CarriedCoincidence> {
        match &self.0 {
            d::CheckEvidence::CarriedCoincidence { finding } => {
                Some(CarriedCoincidence((**finding).clone()))
            }
            _ => None,
        }
    }

    /// Why the at-rest check could not look, whole, on `at_rest` alone.
    #[getter]
    fn at_rest(&self) -> Option<AtRestEvidence> {
        match &self.0 {
            d::CheckEvidence::AtRest { evidence } => {
                Some(AtRestEvidence::of(evidence.clone(), &self.1))
            }
            _ => None,
        }
    }

    /// The chart-coherence arm's own inner word: which condition was
    /// measured on `chart_coherence` (`meridian_closure`,
    /// `rim_continuation`, `meridian_continuation`), and why a loop
    /// was out of reach on `chart_coherence_unexamined` (`corrupt`,
    /// `null_scaffold_edge`, `non_iso_carrier`).
    ///
    /// One attribute for two alphabets, which `boolean_variant`
    /// refuses — and for that field's own stated reason, which does
    /// not reach here: these two alphabets share no spelling, so the
    /// word answers a caller that has not branched on `variant`, and
    /// answers it correctly.
    ///
    /// **An unexamined loop is not a skipped check.**
    /// `ChecksReport.skipped` lists checks a CONFIGURATION turned off;
    /// these words name loops the DATA put out of reach, on a check
    /// that ran.
    #[getter]
    fn chart_variant(&self) -> Option<&'static str> {
        self.payload().chart_variant
    }

    /// The chart-coherence measurement as a length in metres, on
    /// `chart_coherence` alone — `gap * lever`, the only unit the band
    /// is in and the one the finding is judged in.
    #[getter]
    fn metres(&self) -> Option<f64> {
        self.payload().metres
    }

    /// That measurement's disagreement in the chart's own units, on
    /// `chart_coherence`: radians of u, or v's units by surface kind.
    #[getter]
    fn gap(&self) -> Option<f64> {
        self.payload().gap
    }

    /// That measurement's lever arm in metres per chart unit, on
    /// `chart_coherence`. Zero on a chart axis, where an azimuth
    /// carries no length.
    #[getter]
    fn lever(&self) -> Option<f64> {
        self.payload().lever
    }

    /// The band this measurement was judged against, on
    /// `chart_coherence`. It rides the finding rather than the report:
    /// `metres` read without it is a number without a claim.
    #[getter]
    fn eps(&self) -> Option<f64> {
        self.payload().eps
    }

    /// What an unproven coincidence decided between its cells, on
    /// `unproven_coincidence` alone: `same_oriented`, `same_opposite`,
    /// `on_carrier`, `common_foot`, `tangent`, `cusp`,
    /// `surface_tangent`, `coaxial`, `same_radius`, `axes_meet`,
    /// `co_ruled`, or `meet`, two cells or more sharing one point. A
    /// `profile_junction` row is `tangent` or `cusp` between two carriers
    /// and `same_oriented` where its pieces continue one; a
    /// `tangent_witness` row is `surface_tangent`, claiming nothing of
    /// the materials.
    #[getter]
    fn relation(&self) -> Option<&'static str> {
        self.payload().relation
    }

    /// Where that coincidence was decided, on `unproven_coincidence`
    /// alone: `plane_ladder`, `carrier_ladder`, `tangent_witness`,
    /// `coaxial_sphere`, `sphere_on_torus_axis`, `coaxial_cylinder_torus`,
    /// `coaxial_tori`, `crossing_cylinders`, `split_on`, `battery_turn`,
    /// `battery_joint`, `battery_support_axis`, `profile_junction`,
    /// `vertex_fusion` or `census_at_rest`.
    #[getter]
    fn site(&self) -> Option<&'static str> {
        self.payload().site
    }

    /// The row itself, its cells named, on `unproven_coincidence`
    /// alone.
    #[getter]
    fn coincidence(&self, py: Python<'_>) -> PyResult<Option<super::value::Coincidence>> {
        match &self.0 {
            d::CheckEvidence::UnprovenCoincidence {
                row,
                residual,
                recourse,
                ..
            } => super::value::Coincidence::new(
                py,
                row,
                &d::Proof::Unproven {
                    residual: (**residual).clone(),
                    recourse: *recourse,
                },
                &self.1,
            )
            .map(Some),
            _ => Ok(None),
        }
    }

    /// The fold steps' rows reported with that coincidence, on
    /// `unproven_coincidence` alone: each a union's fold-step row
    /// decided at a member vertex the coincidence names, its cells named
    /// and its residual said; empty where it reports none.
    #[getter]
    fn restated(&self, py: Python<'_>) -> PyResult<Option<Vec<super::value::Coincidence>>> {
        match &self.0 {
            d::CheckEvidence::UnprovenCoincidence { restated, .. } => restated
                .iter()
                .map(|(row, residual, recourse)| {
                    super::value::Coincidence::new(
                        py,
                        row,
                        &d::Proof::Unproven {
                            residual: residual.clone(),
                            recourse: *recourse,
                        },
                        &self.1,
                    )
                })
                .collect::<PyResult<Vec<_>>>()
                .map(Some),
            _ => Ok(None),
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        format!("CheckEvidence({:?})", check_evidence_tag(&self.0))
    }
}

impl CheckEvidence {
    /// This evidence's payload, read once per attribute.
    ///
    /// Every accessor above reads a field off THIS record rather than
    /// matching the enum itself, so the arm table is written once —
    /// exhaustively, with no wildcard, in `crate::check_payload` — and
    /// an arm added kernel-side is a compile error there instead of
    /// every attribute silently answering `None`.
    fn payload(&self) -> crate::check_payload::CheckEvidencePayload<'_> {
        crate::check_payload::check_payload(&self.0)
    }
}

/// One finding of one check on one subject: a copy a placement defines,
/// attributed as `(placement, output_ix)`; a node
/// of the document (`node`) — the node that decided an unproven
/// coincidence, or an operation nothing uses; two copies (`copies`); the
/// body a census at rest was taken over (`census`); or a named variable
/// nothing uses (`var`). Whichever it is not answers `None`.
///
/// A finding is a REPORT about geometry, not a verdict on the program:
/// reaching one changes nothing, and whether it stops anything is the
/// caller's `Severity` and the caller's `enforce_checks` call.
///
/// `str()` renders it the way the library renders a finding, recourse
/// included, each placement spoken (kind, label and tag) from the document
/// the checks ran over, the one the evaluation is of; `repr()` keeps
/// the full id. Two findings are equal when they are the same finding
/// over the same document; labels are not compared.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct CheckFinding {
    finding: d::CheckFinding,
    /// The document the checks ran over, the one its placements are
    /// spoken from.
    doc: Arc<d::ProfileDoc>,
}

#[pymethods]
impl CheckFinding {
    /// The check that fired.
    #[getter]
    fn check(&self) -> CheckId {
        check_id(self.finding.check)
    }

    /// The placement whose value carries the subject body; `None` for any
    /// other subject.
    #[getter]
    fn placement(&self) -> Option<NodeId> {
        match self.finding.subject {
            d::FindingSubject::Output { placement, .. } => Some(NodeId(placement)),
            d::FindingSubject::Node(_)
            | d::FindingSubject::Copies { .. }
            | d::FindingSubject::Census { .. }
            | d::FindingSubject::Var(_) => None,
        }
    }

    /// Which output body of that placement's value — 0 for a single body;
    /// `None` for any other subject.
    #[getter]
    fn output_ix(&self) -> Option<u32> {
        match self.finding.subject {
            d::FindingSubject::Output { output_ix, .. } => Some(output_ix),
            d::FindingSubject::Node(_)
            | d::FindingSubject::Copies { .. }
            | d::FindingSubject::Census { .. }
            | d::FindingSubject::Var(_) => None,
        }
    }

    /// The node the finding is about, placement or not; `None` for any
    /// other subject.
    #[getter]
    fn node(&self) -> Option<NodeId> {
        match self.finding.subject {
            d::FindingSubject::Node(node) => Some(NodeId(node)),
            d::FindingSubject::Output { .. }
            | d::FindingSubject::Copies { .. }
            | d::FindingSubject::Census { .. }
            | d::FindingSubject::Var(_) => None,
        }
    }

    /// The body a census at rest was taken over, by the read it is: a
    /// `⊔`'s output, or the body the product slot reads. An at-rest
    /// finding belongs to the census that decided it. `None` for any
    /// other subject.
    #[getter]
    fn census(&self) -> Option<Var> {
        match self.finding.subject {
            d::FindingSubject::Census { read } => Some(Var::of(&self.doc, read)),
            d::FindingSubject::Output { .. }
            | d::FindingSubject::Node(_)
            | d::FindingSubject::Copies { .. }
            | d::FindingSubject::Var(_) => None,
        }
    }

    /// The two copies the finding is about, earlier in gather order
    /// first, each as its placement's output and its member there, as
    /// `InterferenceFinding.a` spells one; `None` for any other subject.
    #[getter]
    fn copies(&self) -> Option<((Var, u32), (Var, u32))> {
        match self.finding.subject {
            d::FindingSubject::Copies { a, b } => {
                let copy = |c: d::CopyRef| (Var::of(&self.doc, c.var), c.member);
                Some((copy(a), copy(b)))
            }
            d::FindingSubject::Output { .. }
            | d::FindingSubject::Node(_)
            | d::FindingSubject::Census { .. }
            | d::FindingSubject::Var(_) => None,
        }
    }

    /// The variable an `unused` finding is about, spoken by its name;
    /// `None` for any other subject.
    #[getter]
    fn var(&self) -> Option<Var> {
        match self.finding.subject {
            d::FindingSubject::Var(var) => Some(Var::of(&self.doc, var)),
            d::FindingSubject::Output { .. }
            | d::FindingSubject::Node(_)
            | d::FindingSubject::Census { .. }
            | d::FindingSubject::Copies { .. } => None,
        }
    }

    /// What was found.
    #[getter]
    fn evidence(&self) -> CheckEvidence {
        CheckEvidence(self.finding.evidence.clone(), Arc::clone(&self.doc))
    }

    /// The assertion that quiets it, or `None` when it is loud. A quiet
    /// finding is listed, naming its assertion, so "checked and
    /// intended" stays distinct from "not checked"; `enforce_checks`
    /// never refuses it. A part's carried finding names the part's own
    /// assertion, an id in the part's document
    /// (`CarriedInterference.of`, `CarriedCoincidence.of`).
    #[getter]
    fn quiet(&self) -> Option<NodeId> {
        self.finding.quiet.map(NodeId)
    }

    /// The finding as the library renders one: its subject, then the
    /// kernel's own story, each placement spoken from the document the
    /// checks ran over. The recourse is in that story; nothing is
    /// appended here.
    fn __str__(&self) -> String {
        d::Said(&self.finding, d::Speaker::of(&*self.doc)).to_string()
    }

    /// The same finding over the same document; labels are not
    /// compared.
    fn __eq__(&self, other: &Self) -> bool {
        self.finding == other.finding && self.doc.id() == other.doc.id()
    }

    fn __repr__(&self) -> String {
        let subject = match self.finding.subject {
            d::FindingSubject::Output {
                placement,
                output_ix,
            } => {
                format!("placement {}, output {output_ix}", placement.full())
            }
            d::FindingSubject::Node(node) => format!("node {}", node.full()),
            d::FindingSubject::Census { read } => format!("census {read}"),
            d::FindingSubject::Var(var) => format!("var {var}"),
            d::FindingSubject::Copies { a, b } => {
                let copy = |c: d::CopyRef| match c.member {
                    0 => format!("{}", c.var),
                    member => format!("{} member {member}", c.var),
                };
                format!("copies {} and {}", copy(a), copy(b))
            }
        };
        format!(
            "CheckFinding({}, {subject}, {:?})",
            self.finding.check,
            check_evidence_tag(&self.finding.evidence)
        )
    }
}

/// A coincidence a document BELOW this one leaves unproven, with the
/// route this document reached it by: an `unproven_coincidence`
/// finding's `CheckEvidence.carried_coincidence`, as the part's own door
/// judged it and quiet only by the part's own assertions.
///
/// The finding is the part's: its ids are `of`'s, and it is spoken from
/// the version of `of` that `pin` fixes.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct CarriedCoincidence(d::CarriedCoincidence);

#[pymethods]
impl CarriedCoincidence {
    /// The part's own finding, over the part's document.
    #[getter]
    fn finding(&self) -> CheckFinding {
        CheckFinding {
            finding: self.0.finding.clone(),
            doc: Arc::clone(self.0.doc.get()),
        }
    }

    /// The instantiating node OF THIS DOCUMENT the finding came through.
    #[getter]
    fn through(&self) -> NodeId {
        NodeId(self.0.route.through)
    }

    /// Which version of `of` found it.
    #[getter]
    fn pin(&self) -> super::store::ContentPin {
        super::store::ContentPin(self.0.pin)
    }

    /// The document whose coincidence it is, as opaque id text.
    #[getter]
    fn of(&self, py: Python<'_>) -> Py<PyAny> {
        super::assembly::route_fields(py, &self.0.route).0
    }

    /// The instances this document reached it through, nearest first.
    #[getter]
    fn via(&self, py: Python<'_>) -> Py<PyAny> {
        super::assembly::route_fields(py, &self.0.route).1
    }

    fn __repr__(&self) -> String {
        format!(
            "CarriedCoincidence(of={}, loud={})",
            self.0.route.of,
            self.0.finding.quiet.is_none()
        )
    }
}

/// One run's report: the findings, and the checks that did not run.
///
/// `skipped` is why this is a report and not a list: "checked and
/// fine" and "not checked" are different answers, and a caller reading
/// an empty `findings` without reading `skipped` has confused them.
///
/// `str()` speaks each placement from the document the checks ran over, the
/// one the evaluation is of, so a label set after `evaluate` shows on
/// the next evaluation's report. Two reports are equal when they hold
/// the same findings and skips over the same document; labels are not
/// compared.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct ChecksReport {
    report: d::ChecksReport,
    /// The document the checks ran over, the evaluation's own: the
    /// report's placement ids are spelled in it, so its human form speaks
    /// from it and from no other.
    doc: Arc<d::ProfileDoc>,
}

impl ChecksReport {
    /// `findings`, each carrying the document they are spoken from.
    fn wrap(&self, findings: &[d::CheckFinding]) -> Vec<CheckFinding> {
        findings
            .iter()
            .map(|finding| CheckFinding {
                finding: finding.clone(),
                doc: Arc::clone(&self.doc),
            })
            .collect()
    }
}

#[pymethods]
impl ChecksReport {
    /// The findings, in the registry's deterministic order: each
    /// resident's own pass by placement order then output index,
    /// residents in registry order. NOT one global sort.
    #[getter]
    fn findings(&self) -> Vec<CheckFinding> {
        self.wrap(&self.report.findings)
    }

    /// The checks that did not run because the caller set them `Off`.
    #[getter]
    fn skipped(&self) -> Vec<CheckId> {
        self.report.skipped.iter().copied().map(check_id).collect()
    }

    /// How many findings — `len(report)`.
    fn __len__(&self) -> usize {
        self.report.findings.len()
    }

    /// The report, each placement spoken from the document the checks ran
    /// over.
    fn __str__(&self) -> String {
        self.report.spoken(&*self.doc)
    }

    /// The same findings and skips over the same document (the
    /// report's own `document`); labels are not compared.
    fn __eq__(&self, other: &Self) -> bool {
        self.report == other.report
    }

    fn __repr__(&self) -> String {
        format!(
            "ChecksReport({} finding(s), {} skipped)",
            self.report.findings.len(),
            self.report.skipped.len()
        )
    }
}

/// Raise `ChecksError` carrying the refusal's stable tag and the arm's
/// own payload.
///
/// Every attribute is set on every arm, `None` where the arm does not
/// carry it — the `WorkspaceError` posture: handling reads `err.node`
/// without first branching on `err.variant`.
pub(crate) fn checks_err(py: Python<'_>, err: &d::ChecksError) -> PyErr {
    checks_err_saying(py, err, err.to_string())
}

/// [`checks_err`] under `message`: the refusal as a frame holding the
/// checked document speaks it.
fn checks_err_saying(py: Python<'_>, err: &d::ChecksError, message: String) -> PyErr {
    let none = || py.None();
    // Exhaustive on purpose, no wildcard: an arm added kernel-side
    // arrives here as a compile error rather than as a silently
    // unprojected payload.
    let node = match err {
        d::ChecksError::Placement(standing) => Py::new(py, NodeId(standing.node()))
            .map(|v| v.into_any())
            .unwrap_or_else(|_| py.None()),
        // A tolerance that forms no band, a gather that yields no
        // product and a pair that is not a pair are all refusals about
        // the WHOLE document; none has a node to name.
        d::ChecksError::Band { .. }
        | d::ChecksError::EvaluationOfAnotherDocument { .. }
        | d::ChecksError::Product { .. } => none(),
    };
    typed_err(
        py,
        ErrorClass::Checks,
        message,
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
/// completion: subjects are the copies the document's placements
/// define, so a placement that failed, was poisoned, or was cut short
/// refuses `placement_without_value` rather than reporting over a partial
/// evaluation.
///
/// The residents:
///
/// * `Connectedness` — a subject's component count, counted EXACTLY as
///   its number of outer shells (an internal void is boundary, not a
///   component, at every tolerance), compared with `!=` against
///   `ChecksConfig.expected_components` (default 1). A shell the
///   orientation read cannot decide surfaces as its own `escalated` /
///   `unsupported` finding, never as a silent skip.
/// * `UnprovenCoincidence` — each coincidence a node decided from
///   values that the coincidence door does not prove structural, an
///   at-rest contact between two operands of a `⊔` included; quiet
///   under a holding `gap = 0` between its two cells.
/// * `AtRest` — at each `⊔`, the product's first and then every other
///   in document order, each overlap of two arrivals' material
///   (`interference`, quiet under a holding one-sided `gap`
///   assertion), each overlap a part carried up
///   (`carried_interference`), and each pair the census could not
///   look at (`at_rest`, loud and refusing at no severity).
///
/// Expectations are TWO-DIRECTIONAL: an `expected_components` entry no
/// subject consumed — a vanished body, a dead placement, a mistyped key —
/// comes back as its own `stale_expectation` finding. A stale
/// acknowledgment must not read as "checked and fine".
///
/// **One gather per evaluation.** The document's product is a pure
/// function of the (document, evaluation) pair `evaluation` captured
/// at `evaluate` and the run's tolerance, so it is gathered on the
/// first ask and shared with every other door that wants one
/// ([`crate::product_memo`]). Reusing an `Evaluation` is therefore
/// how a caller asks several questions for the price of one gather.
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
    // The pairing FIRST, because the answer may come from the memo and
    // a memo reaches no gather to be refused by. Same refusal, same
    // tag, one line earlier.
    evaluation
        .paired_with(doc)
        .map_err(|m| mispaired_checks(py, m))?;
    evaluation
        .gathered(|memo, doc, ev| crate::product_memo::checks_report(memo, doc, ev, cfg, tol))
        .map(|report| ChecksReport {
            report,
            doc: evaluation.doc_shared(),
        })
        .map_err(|err| checks_err_saying(py, &err, err.spoken(evaluation.doc())))
}

/// A mispaired `(doc, evaluation)` as this door's own refusal.
fn mispaired_checks(py: Python<'_>, m: d::Mispaired) -> PyErr {
    checks_err(py, &m.into())
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
/// defaults to `Error`, and the advisory residents cannot be set to it
/// at all (their knob is `Advisory`, DS6's waiver rule as a type). A
/// finding an assertion quiets never refuses, nor does one the
/// at-rest check could not look at. A program that gates does so
/// because it SAID `Severity.Error`.
///
/// Raises `CheckRefusal`, carrying `findings` — every refusing
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
    d::enforce_checks(&report.report, cfg).map_err(|refusal| {
        let findings = report
            .wrap(&refusal.findings)
            .into_pyobject(py)
            .map(|v| v.unbind().into_any())
            .unwrap_or_else(|_| py.None());
        typed_err(
            py,
            ErrorClass::Enforce,
            refusal.spoken(&*report.doc),
            &[("findings", findings)],
        )
    })
}

/// The door from a finding's attribution back to its subject: the body
/// at `(placement, output_ix)` in this evaluation.
///
/// A finding names a subject and this resolves it, against the same
/// enumeration `run_checks` walks — so a finding's attribution always
/// resolves in the evaluation it was produced from. `None` where the
/// placement has no value, denotes no body, or has no output at that index:
/// exactly the attributions a `stale_expectation` finding names, which
/// is what makes that arm's `None` an answer rather than a failure.
///
/// The body arrives WITH the declarations its producer minted for it —
/// `editor_core::checks::subject_body` hands back the pair
/// `product::sources_of` builds, and this door captures the records
/// onto the `Body` exactly as `Value.body` does. So one subject has
/// one tier-3′ verdict however it is reached: a subject that IS a
/// declared boolean result, or an instance carrying a referenced
/// document's D-1 declarations, certifies its own seam under
/// `Body.validate_pseudomanifold` here and through `Value.body`
/// alike, and neither door can bless a coincidence no record backs.
///
/// The Python signature is unchanged by that — still `Optional[Body]`,
/// because the records are CAPTURED on the body rather than returned
/// beside it (`Body`'s own docs give the reason: `ContactRecords` has
/// no Python spelling, and a door that took one could hand a body
/// another body's declarations).
#[pyfunction]
pub(crate) fn subject_body(
    py: Python<'_>,
    evaluation: &Evaluation,
    placement: &NodeId,
    output_ix: u32,
) -> PyResult<Option<Body>> {
    d::subject_body(&evaluation.inner, placement.0, output_ix)
        .map(|(body, contacts)| {
            pncad::export::body_in_space(&body, Tol::witness())
                .map(|body| Body::recorded(body, contacts))
                .map_err(|err| crate::py::value::compose_err(py, *placement, &err))
        })
        .transpose()
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
    m.add_class::<CarriedCoincidence>()?;
    m.add_class::<ChecksReport>()?;
    m.add_function(wrap_pyfunction!(run_checks, m)?)?;
    m.add_function(wrap_pyfunction!(enforce_checks, m)?)?;
    m.add_function(wrap_pyfunction!(subject_body, m)?)?;
    Ok(())
}
