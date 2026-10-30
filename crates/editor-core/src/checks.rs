//! The advisory-check registry (DISCIPLINES-DESIGN DS6, grade 4) and
//! its five residents: the connectedness check (LONGTERM-IDEAS
//! I1(0b)), the chart-coherence examination
//! (`topo::examine_chart_coherence`, read per rest body and default
//! `Off`), the unused check (D10 `[operations]`: what the person made
//! and nothing uses, read off the recipe, [`Doc::unused`]), the
//! unproven-coincidence check (D10: each coincidence a
//! node decided from values that the coincidence door,
//! [`crate::coincide::prove`], does not prove structural), and the
//! at-rest check (D10 `[assertions]`: what the census at each `⊔`
//! found between its arrivals, [`mod@at_rest`]).
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
//! - **[`run_checks_on`] reports, never gates** (the
//!   [`crate::mixed_pins`] posture), and so does [`run_checks`], the
//!   wrapper that derives a [`Subject`] for it: nothing calls either
//!   from `apply`, from the load door, or from evaluation. A document
//!   with findings is valid at every one of those doors.
//! - **[`enforce_checks`] is the ONLY refusing path**: it consumes a
//!   finished report and refuses on `Error`-severity findings; the
//!   CALLER chooses where (and whether) to gate.
//! - **`Off` checks are VISIBLY skipped** ([`ChecksReport::skipped`]):
//!   "checked and fine" and "not checked" are different answers.
//! - **`skipped` is CONFIGURATION, and a check that could not look
//!   says so as a FINDING.** The two are never folded. A resident
//!   whose subject put something out of its reach — a shell whose
//!   orientation would not read, a loop the chart-coherence door could
//!   not walk — reports that with its own typed evidence, because
//!   "the caller turned me off" and "the data would not let me look"
//!   are two answers and only the first is reversible by changing the
//!   configuration.
//! - **Deterministic order** (D9): a body check's findings follow the
//!   placements' document order; the unused check's are its operations
//!   then its variables, each in id order; the unproven-coincidence check's
//!   follow evaluation order, then each node's decision order; the
//!   at-rest check's follow the gathers' order, the product first and
//!   then each other `⊔` in document order, each `⊔`'s in the census's
//!   order of first mention with its carried findings after — a
//!   report that changes only when the document or its evaluation does.
//!
//! The check set is a CLOSED enum ([`CheckId`], the D3 philosophy): a
//! new check is a new variant, and the compiler enumerates every match
//! site that must learn about it. There is no dynamic registration.

use std::collections::BTreeMap;
use std::sync::Arc;

use core::fmt;

use geom_core::{BandError, CertifiedBounds, Decide, Tol};
use topo::{AtRestPolicy, ContactRecords, ShellClassifyError, ShellRole, classify_shells};

use crate::doc::Doc;
use crate::eval::{Evaluation, NodeStanding};
use crate::node::RecipeNodeId;
use crate::product;

pub mod at_rest;

/// The closed set of checks. A new check = a new variant; every match
/// over this enum is a site the compiler then walks you to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CheckId {
    /// The connectedness check (I1(0b)): a body at rest with more (or
    /// fewer) disconnected components than expected. Components are
    /// counted as `Outer` shells ([`topo::classify_shells`]); internal
    /// voids are boundary, not components, and are never counted.
    Connectedness,
    /// The chart-coherence examination
    /// ([`topo::examine_chart_coherence`]): a curved face's CARRIERS
    /// and its VERTICES disagree, in metres, about a chart coordinate
    /// they both state.
    ///
    /// A MEASUREMENT and never a verdict — the door decides nothing,
    /// this resident refuses nothing, and a body can be tier-3 green
    /// and report, or tier-3 refused and stay quiet.
    ///
    /// **Defined at the `f64` lane only** ([`ChartCoherenceLane`]).
    /// A lane without an examination says so as a finding
    /// ([`CheckEvidence::ChartCoherenceUnavailable`]) rather than
    /// reporting a clean body it never read.
    ChartCoherence,
    /// The `unproven-coincidence` lint (D10): a coincidence an
    /// operation decided from values that the door
    /// ([`crate::coincide::prove`]) does not prove structural. It holds
    /// at the current values; whether it holds across the family the
    /// door has not shown. Read off every evaluated
    /// node's [`crate::eval::NodeValue::coincidences`]; it refuses
    /// nothing and changes no body.
    UnprovenCoincidence,
    /// The at-rest check (D10 `[assertions]`, ASSEMBLY A5): at each
    /// `⊔`, the overlaps the census found between two arrivals
    /// ([`CheckEvidence::Interference`], and a part's,
    /// [`CheckEvidence::CarriedInterference`]), and each pair it could
    /// not look at ([`CheckEvidence::AtRest`]). A contact is the
    /// `unproven-coincidence` check's, never this one's.
    AtRest,
    /// The `unused` check (D10 `[operations]` "The product"): what the
    /// person made and nothing uses, at the tip of each dead chain
    /// ([`Doc::unused`]). Read off the recipe; nothing evaluated.
    Unused,
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
            // Certified: an overlap is the census's decided verdict,
            // bounded by the kernel's own intersection, and a pair it
            // could not decide is reported as exactly that.
            Self::AtRest => CheckKind::Certified,
            // Certified: the gap, the lever arm and their product are
            // closed forms over the body's own stated coordinates, and
            // the band is an exact `<` against eps. What the finding
            // asserts is that the two statements differ by this many
            // metres, which is a theorem about the data — never that
            // the body is wrong, which it does not say.
            Self::ChartCoherence => CheckKind::Certified,
            // Certified in the direction it stays SILENT: a row it does
            // not report is one the door proved structural. What it
            // reports is the absence of that proof, which is exactly
            // what the finding says — never "this holds only at these
            // values", which a later rung may refute.
            Self::UnprovenCoincidence => CheckKind::Certified,
            // Certified: whether anything reads a variable is a fact of
            // the recipe, decided by no value.
            Self::Unused => CheckKind::Certified,
        }
    }
}

impl CheckId {
    /// Every check, in the order [`run_checks_on`] runs them (D9).
    ///
    /// A new variant that is not here fails
    /// `dsc_checks::the_registry_order_is_every_check`, whose match is
    /// the compiler's own walk of the closed set.
    /// The residents that read no subject come FIRST, so their own
    /// refusals ([`ChecksError::Placement`]) are the ones reported before
    /// [`run_checks_on`] can refuse [`ChecksError::Product`] on one that
    /// does. Order decides only which refusal is reported: a refusal
    /// discards the whole report, every resident's findings with it.
    pub const ALL: [Self; 5] = [
        Self::Connectedness,
        Self::ChartCoherence,
        Self::Unused,
        Self::UnprovenCoincidence,
        Self::AtRest,
    ];

    /// Whether this resident reads the registry's [`Subject`].
    ///
    /// The one definition of that, and the reason the registry can
    /// decide whether to gather AT ALL before it does: connectedness
    /// reads the evaluation per placement and needs no product, so a run
    /// with only connectedness enabled has nothing to gather for.
    pub fn reads_subject(self) -> bool {
        match self {
            Self::Connectedness => false,
            // Each census rides the gathered product.
            Self::AtRest => true,
            // The examination is over one body at rest, read per placement
            // output exactly as connectedness is; no product, nothing
            // to gather.
            Self::ChartCoherence => false,
            // The rows ride each node's value, and the at-rest census's
            // ride the gathered product.
            Self::UnprovenCoincidence => true,
            Self::Unused => false,
        }
    }
}

impl fmt::Display for CheckId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connectedness => f.write_str("connectedness"),
            Self::AtRest => f.write_str("at-rest"),
            // ONE token, like its two neighbours: `ChecksReport`'s
            // skipped list is space-separated, so a two-word name
            // would make the list unreadable at the boundary between
            // two entries.
            Self::ChartCoherence => f.write_str("chart-coherence"),
            Self::UnprovenCoincidence => f.write_str("unproven-coincidence"),
            Self::Unused => f.write_str("unused"),
        }
    }
}

/// The certified/heuristic split (DS6): honesty of language and a
/// default level, not a force cap. A certified finding is a theorem
/// about the evaluated geometry; a heuristic finding is a judgment
/// that can be wrong in both directions and must never be dressed as
/// the former.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

/// **Which decision lanes carry a chart-coherence examination.**
///
/// [`topo::examine_chart_coherence`] reads a body's stored POINTS and
/// carriers as lengths in metres, and a [`topo::Body`]'s geometry
/// arenas are parameterised by the lane's scalar. The door is written
/// at `f64`, so the examination exists at `f64` and nowhere else; the
/// registry is generic over the lane, so it needs this to ask.
///
/// **`None` is a fact about the LANE, not a quiet pass.** The
/// resident turns it into
/// [`CheckEvidence::ChartCoherenceUnavailable`], one finding per rest
/// body, because a lane that cannot look and reports nothing renders
/// identically to a body with nothing to report — the F6 posture the
/// rest of this module already takes.
///
/// **Why every lane but `f64` answers `None`.** The examination is
/// written at `f64` ([`topo::examine_chart_coherence`] takes a
/// `&Body<f64>`), so the absence is LANE-0's class, *the derivation is
/// not written at this scalar*, and not *this scalar may not certify*.
/// `Sym` and `Dual` have no impl at all, so the resident's pass cannot be
/// instantiated at them: a static exclusion, not a `None`.
pub trait ChartCoherenceLane: geom_core::Real {
    /// This lane's chart-coherence examination of `body` at `tol`, or
    /// `None` when the lane has none.
    fn examine_chart_coherence(body: &topo::Body<Self>, tol: Tol) -> Option<topo::CoherenceReport>;
}

impl ChartCoherenceLane for f64 {
    fn examine_chart_coherence(body: &topo::Body<Self>, tol: Tol) -> Option<topo::CoherenceReport> {
        Some(topo::examine_chart_coherence(body, tol))
    }
}

#[cfg(feature = "probe")]
impl ChartCoherenceLane for geom_core::Probe {
    fn examine_chart_coherence(
        _body: &topo::Body<Self>,
        _tol: Tol,
    ) -> Option<topo::CoherenceReport> {
        // Not written at this scalar (the examination takes a
        // `Body<f64>`): the trait's LANE-0 absence, not a refusal to certify.
        None
    }
}

impl ChartCoherenceLane for geom_core::interval::Interval {
    fn examine_chart_coherence(
        _body: &topo::Body<Self>,
        _tol: Tol,
    ) -> Option<topo::CoherenceReport> {
        // Not written at this scalar (the examination takes a
        // `Body<f64>`): the trait's LANE-0 absence, not a refusal to certify.
        None
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
    /// `(placement, output_ix)`; a missing key means 1. This is the
    /// resident's acknowledgment mechanism: a deliberately disjoint
    /// body is EXPECTED disjoint, stated as data, and stays clean.
    pub expected_components: BTreeMap<(RecipeNodeId, u32), u32>,
    /// [`CheckId::ChartCoherence`]'s severity — [`Advisory`], NOT
    /// [`Severity`], on the same DS6 `iff`: this resident ships no
    /// per-finding acknowledgment record with a staleness direction,
    /// so `Error` is unrepresentable rather than asked-not-to-be-used.
    ///
    /// **Default `Off`, and that is the one default on this type that
    /// is not `Warn`.** Two measured reasons, neither of them taste:
    ///
    /// - the examination has no shape door, so a chart-bearing face
    ///   whose outer loop carries a conic or spline trim carrier puts
    ///   every one of its loops in
    ///   [`CheckEvidence::ChartCoherenceUnexamined`] — two per body on
    ///   an ordinary trimmed cylinder
    ///   (`mesh::mesh8_corpus_coherence`). That is a correct report of
    ///   a lane boundary and it is not actionable, and a resident that
    ///   fires on ordinary bodies by default is not a report;
    /// - it reads every face of every rest body on every run, and this
    ///   registry is an advisory pass whose per-resident cost is
    ///   measured and pinned. Nothing has measured this one, so it
    ///   does not join the default pass.
    ///
    /// Turning it on is one field, and `Off` is VISIBLY off
    /// ([`ChecksReport::skipped`]).
    pub chart_coherence: Advisory,
    /// [`CheckId::UnprovenCoincidence`]'s severity — [`Advisory`], so
    /// `Error` is unrepresentable. An assertion quiets a row, an
    /// operation's or an at-rest one, under [`mod@at_rest`]'s contact rule,
    /// and a row none could quiet never refuses
    /// ([`CheckEvidence::never_refuses`]). A per-run choice and never a
    /// document's: turning it off removes reports, never changes a body
    /// (DS3).
    pub unproven_coincidence: Advisory,
    /// [`CheckId::AtRest`]'s severity — a full [`Severity`], default
    /// `Warn`. `Error` is legal because a finding refuses only where an
    /// assertion could have quieted it (DS3, DS6): at `Error`,
    /// [`enforce_checks`] refuses a loud interference finding some
    /// assertion the quieting rule accepts could quiet, and nothing else
    /// this check reports ([`CheckEvidence::never_refuses`]): not an
    /// overlap the kernel could not intersect, not one no such assertion
    /// could be written over, and not a pair the census could not look
    /// at.
    pub at_rest: Severity,
    /// [`CheckId::Unused`]'s severity — [`Advisory`], default `Warn`:
    /// nothing quiets an unused thing but using it or deleting it.
    pub unused: Advisory,
}

impl Default for ChecksConfig {
    fn default() -> Self {
        Self {
            connectedness: Severity::Warn,
            expected_components: BTreeMap::new(),
            chart_coherence: Advisory::Off,
            unproven_coincidence: Advisory::Warn,
            at_rest: Severity::Warn,
            unused: Advisory::Warn,
        }
    }
}

impl ChecksConfig {
    /// The configured severity of `check` (the one match site the
    /// closed enum walks a new check to).
    pub fn severity(&self, check: CheckId) -> Severity {
        match check {
            CheckId::Connectedness => self.connectedness,
            CheckId::AtRest => self.at_rest,
            CheckId::ChartCoherence => self.chart_coherence.severity(),
            CheckId::UnprovenCoincidence => self.unproven_coincidence.severity(),
            CheckId::Unused => self.unused.severity(),
        }
    }

    /// Whether a run under this configuration needs a [`Subject`] at
    /// all: true iff some resident that reads one is not `Off`
    /// ([`CheckId::reads_subject`], [`CheckId::ALL`]).
    ///
    /// The ONE definition of that question, and what lets
    /// [`run_checks`] decide BEFORE it gathers. A configuration whose
    /// enabled residents all read the evaluation has nothing to gather
    /// FOR — which is what the registry did when each resident owned
    /// its own gather, and is the difference between an advisory pass
    /// and the most expensive call in a landing.
    #[must_use]
    pub fn needs_a_subject(&self) -> bool {
        CheckId::ALL
            .into_iter()
            .any(|check| check.reads_subject() && self.severity(check) != Severity::Off)
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
    /// A shell's orientation could not be read (an in-band or zero
    /// signed volume, or a volume bracket straddling zero): the component
    /// count for this subject is UNKNOWABLE, which is a finding, never a
    /// silent skip (F6).
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
    /// The subject is not a finished body: tier 3 refuses it, so its
    /// shells are not classified (the shell door reads finished bodies,
    /// `&AtRestBody`) and the component count is not read. A finding,
    /// never a silent skip (F6).
    NotAtRest {
        /// The validator's findings on the subject, verbatim.
        errors: Vec<topo::ValidationError>,
    },
    /// An expectation with no subject: `expected_components` carries
    /// an entry at this `(placement, output_ix)` and no evaluated subject
    /// consumed it — the body vanished (a boolean may have consumed
    /// the whole part) or the key names no placement output. Expectations
    /// are two-directional: an entry that binds nothing is stale, not
    /// silently ignored. (The DEFAULT expectation — no entry — binds
    /// only existing subjects: a placement whose copy legitimately
    /// holds no body, with nothing stated about it, stays clean.)
    StaleExpectation {
        /// The entry's stated component count.
        expected: u32,
    },
    /// One chart-coherence measurement, carried whole
    /// ([`topo::CoherenceFinding`]: the condition, the gap in chart
    /// units, the lever arm, their product in metres, and the band it
    /// was judged at).
    ///
    /// The kernel's own value rides here rather than being unpacked
    /// into fields, on the [`CheckEvidence::Escalated`] precedent: it
    /// is `Clone + PartialEq`, so it can, and a consumer that wants
    /// the coordinate gets the kernel's own struct rather than this
    /// layer's restatement of it.
    ChartCoherence {
        /// The measurement.
        finding: topo::CoherenceFinding,
    },
    /// One loop of a chart-bearing face the examination COULD NOT
    /// READ, carried whole ([`topo::Unexamined`]).
    ///
    /// **This is data, and [`ChecksReport::skipped`] is
    /// configuration.** The two are not the same fact and are not
    /// folded: `skipped` says a caller set this check's severity to
    /// `Off` and nothing was looked at ON PURPOSE, reversible by
    /// changing the configuration; this says the check RAN and the
    /// body put a loop out of its reach, which no configuration
    /// changes. The kernel door states the same distinction about its
    /// own two lists at [`topo::CoherenceReport::unexamined`], and
    /// this arm is how it survives the crossing: an unexamined loop
    /// arrives as a FINDING (the F6 posture the `Escalated` and
    /// `Unsupported` arms already take one resident over), never as a
    /// skipped check and never as silence.
    ChartCoherenceUnexamined {
        /// The loop, the face owning it, and why.
        unexamined: topo::Unexamined,
    },
    /// This evaluation's decision lane carries no chart-coherence
    /// examination ([`ChartCoherenceLane`]), so this body's chart
    /// coordinates were never read.
    ///
    /// One finding per rest body rather than a clean report (F6): a
    /// lane that cannot look and says nothing is indistinguishable
    /// from a body with nothing to report, which is the one thing this
    /// resident may not be.
    ChartCoherenceUnavailable,
    /// A coincidence the finding's node decided from values that the
    /// door does not prove structural ([`crate::coincide::prove`]).
    UnprovenCoincidence {
        /// The row, its cells named in the node's inputs' tables.
        row: Box<crate::coincide::NamedCoincidence>,
        /// What separates the cells' constructions.
        residual: Box<crate::coincide::Residual>,
        /// What would make it structural.
        recourse: crate::coincide::Recourse,
        /// The node's unproven fold-step rows reported under this
        /// judged row (DM4), each with what separates its cells and its
        /// recourse: each names a member vertex this row names, so the
        /// two are decided at that vertex
        /// ([`crate::coincide::NamedCoincidence::at_one_vertex`]). A
        /// fold row decided at several judged rows' vertex goes under
        /// the least of them by cells, relation and site; one at no
        /// judged row's vertex is its own finding.
        restated: Vec<(
            crate::coincide::NamedCoincidence,
            crate::coincide::Residual,
            crate::coincide::Recourse,
        )>,
        /// Whether an assertion could quiet the row
        /// ([`at_rest::contact_quietable`]): it decided what a `Gap = 0`
        /// says, and a `Gap` over its two cells reads zero. A row no
        /// assertion could quiet never refuses ([`Self::never_refuses`]).
        quietable: bool,
    },
    /// An overlap of two arrivals' material at rest, quiet or loud
    /// ([`at_rest::InterferenceFinding`]).
    Interference {
        /// The finding.
        finding: Box<at_rest::InterferenceFinding>,
    },
    /// An overlap a document below this one reported between two of
    /// its own arrivals, quiet or loud as the part's assertions judged
    /// it ([`at_rest::CarriedInterference`]).
    CarriedInterference {
        /// The carried finding.
        finding: Box<at_rest::CarriedInterference>,
    },
    /// A coincidence a document below this one leaves unproven, as its
    /// own door judged it, quiet only as its own assertions quiet it
    /// ([`at_rest::CarriedCoincidence`]).
    CarriedCoincidence {
        /// The carried finding.
        finding: Box<at_rest::CarriedCoincidence>,
    },
    /// A pair the at-rest census could not look at
    /// ([`at_rest::AtRestEvidence`]): loud, unquietable, and refusing at
    /// no severity (DS6).
    AtRest {
        /// Why.
        evidence: at_rest::AtRestEvidence,
    },
    /// The subject is used by nothing: not the product, not an
    /// assertion, and nothing reads it.
    Unused {
        /// An operation's outputs the person named, which are reported
        /// with it rather than each on its own; empty for a variable.
        named: Vec<crate::VarId>,
    },
}

impl CheckEvidence {
    /// Whether this is a finding the check could not judge (DS6): loud,
    /// never quietable, and never refused by [`enforce_checks`].
    #[must_use]
    pub fn could_not_look(&self) -> bool {
        matches!(self, Self::AtRest { .. })
    }

    /// **Whether this finding is outside what the document can yet say**
    /// (CHECKS DS6): a pair the census could not look at
    /// ([`Self::could_not_look`]), an overlap no assertion could quiet
    /// ([`at_rest::InterferenceFinding::quietable`]): one the kernel
    /// could not intersect, or one whose bounding faces offer no pair an
    /// assertion the quieting rule accepts could be written over; or a
    /// coincidence no assertion could quiet
    /// ([`at_rest::contact_quietable`]). Loud, and refused by
    /// [`enforce_checks`] at no severity: a refusal there would have no
    /// recourse but changing geometry.
    #[must_use]
    pub fn never_refuses(&self) -> bool {
        match self {
            Self::UnprovenCoincidence { quietable, .. } => !quietable,
            Self::Interference { finding } => !finding.quietable,
            Self::CarriedInterference { finding } => !finding.finding.quietable,
            Self::CarriedCoincidence { finding } => finding.finding.evidence.never_refuses(),
            other => other.could_not_look(),
        }
    }
}

/// One finding of one check on one subject ([`FindingSubject`]).
#[derive(Clone, Debug, PartialEq)]
pub struct CheckFinding {
    /// The check that fired.
    pub check: CheckId,
    /// What the finding is about.
    pub subject: FindingSubject,
    /// What was found.
    pub evidence: CheckEvidence,
    /// The assertion that quiets it, or `None` when it is loud. A quiet
    /// finding is listed, naming its assertion, so "checked and
    /// intended" stays distinct from "not checked"; [`enforce_checks`]
    /// never refuses it. A part's carried finding names the part's own
    /// assertion, in the part's ids.
    pub quiet: Option<RecipeNodeId>,
}

/// **What a finding is about.** A body check's subject is a
/// body-denoting placement output; the unproven-coincidence check's is the
/// node that decided the row, placement or not, which denotes no one body;
/// the unused check's is an operation or a named variable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FindingSubject {
    /// A placement output, named as `(placement, output_ix)`, which
    /// [`subject_body`] resolves.
    Output {
        /// The placement whose value carries the subject body.
        placement: RecipeNodeId,
        /// Which output body of that placement's value (0 for a single
        /// body).
        output_ix: u32,
    },
    /// A node of the document, placement or not.
    Node(RecipeNodeId),
    /// The body a census at rest was taken over, by the read it is: a
    /// `⊔`'s output, or the body the product slot reads.
    Census {
        /// The read.
        read: crate::VarId,
    },
    /// Two copies of the product, earlier in gather order first: an
    /// at-rest contact between them that no mate speaks for.
    Copies {
        /// The earlier copy.
        a: crate::checks::at_rest::CopyRef,
        /// The other copy.
        b: crate::checks::at_rest::CopyRef,
    },
    /// A variable of the document, said by its name.
    Var(crate::VarId),
}

/// A copy as a finding says it: its placement's output, and its member
/// where the placement defines several.
fn copy_said(copy: crate::checks::at_rest::CopyRef, by: crate::spoken::Speaker<'_>) -> String {
    match copy.member {
        0 => by.read(copy.var),
        member => format!("{} member {member}", by.read(copy.var)),
    }
}

// One story, one recourse, in one place, through the document layer's
// one sink ([`crate::finding`]), each placement said by the speaker the
// finding is said by. The subject is the finding's (placement, output)
// attribution. Five arms end in a recourse the story already
// carries, so `recourse` answers "" ("already told") there:
// - Unsupported forwards its payload's `Display`, recourse included.
// - Escalated renders the refusal's data view,
//   [`ShellClassifyError::payload`] (no stage prefix and no arena key,
//   neither of which a document user can act on), then the shell-role
//   decision's ending read at rest ([`ShellClassifyError::recourse`]):
//   the checks window reads a body at rest.
// - StaleExpectation's pinned prose ends in its own ". Recourse:".
// - CarriedInterference and CarriedCoincidence end in the part's
//   recourse, naming the part, since only the part can act on it.
impl crate::finding::Finding for SaidFinding<'_> {
    fn subject(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "check {}: ", self.0.check)?;
        self.about(f)
    }

    fn story(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(finding, by) = *self;
        self.evidence_story(f)?;
        match (finding.quiet, &finding.evidence) {
            (None, _) => Ok(()),
            // A part's finding is quieted by the part's own assertion, in
            // its ids: said from the part's document where it is carried.
            (Some(_), CheckEvidence::CarriedInterference { .. }) => {
                f.write_str("; quiet: an assertion of the part's says so")
            }
            (Some(assertion), CheckEvidence::CarriedCoincidence { finding }) => write!(
                f,
                "; quiet: the part's {} says so",
                crate::spoken::Speaker::of(&**finding.doc.get()).node_as(assertion, "assertion")
            ),
            (Some(assertion), _) => {
                write!(f, "; quiet: {} says so", by.node_as(assertion, "assertion"))
            }
        }?;
        // A part's finding is acted on in the part: its recourse, there.
        match &finding.evidence {
            CheckEvidence::CarriedInterference { finding } => write!(
                f,
                " — the overlap is the part's, so act in document {}: {INTERFERENCE_RECOURSE}",
                finding.route.of
            ),
            CheckEvidence::CarriedCoincidence { finding } => write!(
                f,
                " — the row is the part's, so act in document {}: {}",
                finding.route.of,
                SaidFinding(
                    &finding.finding,
                    crate::spoken::Speaker::of(&**finding.doc.get())
                )
                .recourse_words()
            ),
            _ => Ok(()),
        }
    }

    fn recourse(&self) -> &str {
        self.recourse_words()
    }
}

/// What to do about an overlap of two arrivals, in the document whose
/// arrivals they are.
const INTERFERENCE_RECOURSE: &str = "Recourse: move the two apart, combine them with a boolean \
     if they are one body, or, for a fit meant to interfere, assert a negative Gap between the \
     two faces that bound the overlap";

impl SaidFinding<'_> {
    /// What the finding is about, by the speaker.
    fn about(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(finding, by) = *self;
        match finding.subject {
            FindingSubject::Node(node) => write!(f, "{}", by.node_as(node, "node")),
            FindingSubject::Output {
                placement,
                output_ix,
            } => {
                write!(
                    f,
                    "{} output {}",
                    by.node_as(placement, "placement"),
                    output_ix
                )
            }
            FindingSubject::Census { read } | FindingSubject::Var(read) => {
                f.write_str(&by.read(read))
            }
            FindingSubject::Copies { a, b } => {
                write!(f, "copies {} and {}", copy_said(a, by), copy_said(b, by))
            }
        }
    }

    /// What the evidence says was found.
    fn evidence_story(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(finding, by) = *self;
        match &finding.evidence {
            CheckEvidence::Connectedness { actual, expected } => write!(
                f,
                "{actual} disconnected component(s) where {expected} was expected"
            ),
            CheckEvidence::Escalated { source } => {
                write!(f, "the component count is unknowable: {}", source.payload())?;
                match source.recourse(geom_brep::recourse::Reading::AtRest) {
                    Some(recourse) => write!(f, ". {recourse}"),
                    None => Ok(()),
                }
            }
            CheckEvidence::Unsupported { source } => write!(
                f,
                "the component count cannot be computed for this body: {source}"
            ),
            CheckEvidence::NotAtRest { errors } => match errors.as_slice() {
                [] => write!(
                    f,
                    "the component count is not read: the body does not pass validation"
                ),
                [first] => write!(
                    f,
                    "the component count is not read: the body does not pass validation: {first}"
                ),
                [first, rest @ ..] => write!(
                    f,
                    "the component count is not read: the body does not pass validation: {first} \
                     (and {} more)",
                    rest.len()
                ),
            },
            CheckEvidence::StaleExpectation { expected } => write!(
                f,
                "an expectation of {expected} component(s) has no subject: the body \
                 vanished (a boolean may have consumed the part) or the key names no \
                 placement output. Recourse: remove the ChecksConfig::expected_components \
                 entry, or fix the placement"
            ),
            CheckEvidence::ChartCoherence { finding } => {
                let what = match finding.condition {
                    topo::CoherenceCondition::MeridianClosure { .. } => {
                        "a meridian edge's carrier midpoint and one of its own endpoint \
                         vertices state azimuths"
                    }
                    topo::CoherenceCondition::RimContinuation { .. } => {
                        "two edges carrying one rim row state row coordinates"
                    }
                    topo::CoherenceCondition::MeridianContinuation { .. } => {
                        "two edges carrying one meridian column state azimuths"
                    }
                };
                write!(
                    f,
                    "{what} {:e} m apart (band {:e} m)",
                    finding.metres, finding.eps
                )
            }
            CheckEvidence::ChartCoherenceUnexamined { unexamined } => {
                f.write_str("a loop of a chart-bearing face could not be examined: ")?;
                match unexamined.why {
                    // The read's name is the kernel's own word for it
                    // and is what a consumer branches on; the typed
                    // value rides in the evidence beside this sentence.
                    topo::Unexaminable::Corrupt { at } => write!(
                        f,
                        "its structure did not resolve ({at:?}), which is a tier-1 defect \
                         rather than a coherence one"
                    ),
                    topo::Unexaminable::NullScaffoldEdge { .. } => f.write_str(
                        "an edge of it carries construction scaffolding rather than a \
                         certified curve, so it states no coordinate to compare",
                    ),
                    topo::Unexaminable::NonIsoCarrier { .. } => f.write_str(
                        "the face's outer loop carries a conic or spline trim carrier, so \
                         its boundary is not a chart iso curve and none of the three \
                         conditions is about it",
                    ),
                }
            }
            CheckEvidence::ChartCoherenceUnavailable => f.write_str(
                "this evaluation's lane does not examine chart coherence, so nothing was read",
            ),
            CheckEvidence::UnprovenCoincidence {
                row,
                residual,
                restated,
                ..
            } => {
                let cell = |c: &crate::coincide::NamedCell| match c {
                    crate::coincide::NamedCell::Entity { name, .. } => by.name(name).to_string(),
                    crate::coincide::NamedCell::Tool { input, role } => match role {
                        Some(role) => format!("the {role} of {}", by.read(*input)),
                        None => format!("the plane of {}", by.read(*input)),
                    },
                    crate::coincide::NamedCell::Piece { profile, piece } => {
                        crate::names::words::piece(piece, *profile, by)
                    }
                };
                write!(
                    f,
                    "{}, decided from values ({}, margin {:e}), is not proven structural: \
                     {residual}",
                    crate::coincide::statement_words(row, cell),
                    crate::coincide::site_words(row.site),
                    row.margin,
                )?;
                if let Some(axes) = &residual.axes {
                    axes_words(f, axes, by)?;
                }
                match restated.len() {
                    0 => Ok(()),
                    1 => f.write_str("; a fold step's row at its cells is reported with it"),
                    n => write!(
                        f,
                        "; {n} fold steps' rows at its cells are reported with it"
                    ),
                }
            }
            CheckEvidence::Interference { finding } => interference_words(f, finding, by),
            CheckEvidence::CarriedInterference { finding } => {
                write!(
                    f,
                    "{} reports an overlap between its own arrivals: ",
                    crate::spoken::Said(&finding.route, by)
                )?;
                interference_words(f, &finding.finding, crate::spoken::Speaker::TAG)
            }
            CheckEvidence::CarriedCoincidence { finding } => {
                let part = SaidFinding(
                    &finding.finding,
                    crate::spoken::Speaker::of(&**finding.doc.get()),
                );
                write!(
                    f,
                    "{} leaves a coincidence of its own unproven, at ",
                    crate::spoken::Said(&finding.route, by)
                )?;
                part.about(f)?;
                f.write_str(": ")?;
                part.evidence_story(f)
            }
            CheckEvidence::AtRest { evidence } => {
                write!(
                    f,
                    "the census could not look: {}",
                    crate::spoken::Said(evidence, by)
                )
            }
            CheckEvidence::Unused { named } => {
                f.write_str("used by nothing")?;
                if !named.is_empty() {
                    let names: Vec<String> = named.iter().map(|&var| by.read(var)).collect();
                    write!(f, ", its outputs named {} with it", names.join(", "))?;
                }
                f.write_str(": not the product, not an assertion, and nothing reads it")
            }
        }
    }

    fn recourse_words(&self) -> &'static str {
        match &self.0.evidence {
            CheckEvidence::Connectedness { .. } => {
                "Recourse: a stray component usually means a boolean that missed its operand \
                 or an instance placed nowhere; if it is deliberate, state the expected count \
                 in ChecksConfig::expected_components"
            }
            CheckEvidence::Interference { .. } => INTERFERENCE_RECOURSE,
            CheckEvidence::Unused { .. } => "Recourse: use it, or delete it",
            CheckEvidence::ChartCoherence { .. } => {
                "a MEASUREMENT, not a refusal: judge the metres against the band; an \
                 imported part usually wants re-exporting at more digits, and a minted one \
                 is a kernel finding"
            }
            CheckEvidence::ChartCoherenceUnexamined { unexamined } => match unexamined.why {
                topo::Unexaminable::Corrupt { .. } => {
                    "the body's structural validation names the defect this read could not \
                     get past"
                }
                // A scaffolding edge and a trimmed face are lane
                // boundaries, not defects: the report names them so
                // the quiet is accounted for, and there is nothing to
                // act on.
                topo::Unexaminable::NullScaffoldEdge { .. }
                | topo::Unexaminable::NonIsoCarrier { .. } => "",
            },
            CheckEvidence::ChartCoherenceUnavailable => {
                "Recourse: evaluate the document at f64 to measure it, or turn this check \
                 off rather than reading its silence as a clean body"
            }
            CheckEvidence::UnprovenCoincidence { recourse, .. } => recourse.words(),
            // The story ends in the part's own recourse, in the part.
            CheckEvidence::CarriedInterference { .. }
            | CheckEvidence::CarriedCoincidence { .. } => "",
            // The first finding's own sentence carries its recourse.
            CheckEvidence::NotAtRest { .. }
            | CheckEvidence::Escalated { .. }
            | CheckEvidence::Unsupported { .. }
            | CheckEvidence::StaleExpectation { .. } => "",
            CheckEvidence::AtRest { evidence } => match evidence {
                at_rest::AtRestEvidence::NotRun => {
                    "Recourse: evaluate the document at f64, where the census runs, rather \
                     than reading this silence as a clean assembly"
                }
                // The kernel's verdict, the gather's refusal and the
                // part's row each end in their own recourse.
                _ => "",
            },
        }
    }
}

/// **An unproven row's axes, in words** ([`crate::coincide::Axes`]):
/// each cell's axis as the pose its mates reach, the component not
/// established, and the edit for that component alone. Two axes' edit
/// mates the copy no axis mate reaches onto the pose the other cell's
/// mates reach, whichever cell the row names first; a radius never
/// read is said so.
fn axes_words(
    f: &mut fmt::Formatter<'_>,
    axes: &crate::coincide::Axes,
    by: crate::spoken::Speaker<'_>,
) -> fmt::Result {
    use crate::coincide::Component;
    let one = |f: &mut fmt::Formatter<'_>| match axes.reached {
        [Some(a), Some(b)] if a == b => {
            write!(f, "; the axes are one, both reading {}", by.read(a))
        }
        _ => f.write_str("; the axes are one"),
    };
    match axes.left {
        Component::Axis => {
            let axis = |i: usize| match axes.reached[i] {
                Some(pose) => format!("reads {}", by.read(pose)),
                None => "is its own".to_owned(),
            };
            write!(
                f,
                "; the axes are two: the first cell's {}, the second's {}",
                axis(0),
                axis(1)
            )?;
            let edit = [(0, 1), (1, 0)].into_iter().find_map(|(i, j)| {
                let pose = axes.reached[i]?;
                let copy = axes.copies[j].filter(|_| axes.reached[j].is_none())?;
                Some((pose, copy, axes.copies[i]))
            });
            match edit {
                Some((pose, copy, reader)) => {
                    write!(
                        f,
                        ". Edit, for the axis: mate {}'s axis onto {}",
                        by.read(copy),
                        by.read(pose)
                    )?;
                    match reader {
                        Some(reader) => write!(f, ", which {} reads", by.read(reader)),
                        None => Ok(()),
                    }
                }
                None => f.write_str(". Edit, for the axis: read both cells' axes from one Axis"),
            }
        }
        Component::Radius => {
            one(f)?;
            f.write_str(
                "; the radii are equal only at the current values. Edit, for the radius: read \
                 one Length for both radii, at both ends of each wall",
            )
        }
        Component::RadiusUnread => {
            one(f)?;
            f.write_str("; the radii were not read")
        }
        Component::Sense => f.write_str(
            "; the axes and the radii are one; the senses are not the ones the row decided",
        ),
    }
}

/// An interference finding's story: the two arrivals, and where they
/// overlap.
fn interference_words(
    f: &mut fmt::Formatter<'_>,
    finding: &at_rest::InterferenceFinding,
    by: crate::spoken::Speaker<'_>,
) -> fmt::Result {
    write!(
        f,
        "{} and {} overlap",
        copy_said(finding.a, by),
        copy_said(finding.b, by)
    )?;
    match &finding.overlap {
        at_rest::Overlap::Bounded { faces } => {
            let names: Vec<String> = faces.iter().map(|s| by.name(&s.face).to_string()).collect();
            write!(f, ", bounded by {}", names.join(", "))
        }
        at_rest::Overlap::Unlocalized(why) => {
            f.write_str(", and the kernel could not bound the overlap: ")?;
            match why {
                at_rest::Unlocalized::Refused { refusal }
                | at_rest::Unlocalized::Containment { refusal } => f.write_str(refusal),
                at_rest::Unlocalized::Invalid { errors } => {
                    write!(
                        f,
                        "a copy is not a valid body ({} finding(s))",
                        errors.len()
                    )
                }
                at_rest::Unlocalized::Empty => {
                    f.write_str("it is thinner than the boolean's tolerance")
                }
                at_rest::Unlocalized::Unnamed => {
                    f.write_str("a face of it descends from no named face")
                }
                at_rest::Unlocalized::Lane => {
                    f.write_str("this evaluation's scalar has no lane to intersect them")
                }
            }
        }
    }
}

/// A finding and the speaker its placements are said by: the one shape the
/// sink composes.
#[derive(Clone, Copy)]
struct SaidFinding<'a>(&'a CheckFinding, crate::spoken::Speaker<'a>);

impl crate::spoken::Say for CheckFinding {
    fn say(&self, f: &mut fmt::Formatter<'_>, by: crate::spoken::Speaker<'_>) -> fmt::Result {
        crate::finding::compose(f, &SaidFinding(self, by))
    }
}

/// The sentence where no document is at hand: each placement by its tag.
impl fmt::Display for CheckFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

/// The findings, each said by `by`, one per line ([`crate::finding::render_list`]).
fn say_findings(
    f: &mut fmt::Formatter<'_>,
    findings: &[CheckFinding],
    by: crate::spoken::Speaker<'_>,
) -> fmt::Result {
    let said: Vec<SaidFinding<'_>> = findings.iter().map(|x| SaidFinding(x, by)).collect();
    crate::finding::render_list(f, &said)
}

/// The result of one [`run_checks`] run: findings in deterministic
/// order, and the checks that were configured `Off` — visibly skipped,
/// because "checked and fine" and "not checked" are different answers.
#[derive(Clone, Debug, PartialEq)]
pub struct ChecksReport {
    /// The document the checks were run over: its placements' ids are
    /// spelled in it, so it is the one document the report's human
    /// form speaks from ([`ChecksReport::spoken`]).
    pub document: crate::DocumentId,
    /// Findings in deterministic order (D9): each resident's own
    /// findings by placement-list position then output index, residents in
    /// registry order. NOT one global sort — placement 5's connectedness
    /// finding precedes placement 1's at-rest finding, because each
    /// resident appends its own pass in sequence, and the coincidence
    /// door's are settled into the place its pass holds
    /// ([`ChecksReport::settle`]).
    pub findings: Vec<CheckFinding>,
    /// Checks that did not run because their severity is `Off`.
    pub skipped: Vec<CheckId>,
    /// **Checks whose answer a job at rest still owes** ([`DoorJob`]):
    /// a state of the report, not a finding. Only
    /// [`run_checks_at_rest_on`] leaves one here, until its job's
    /// findings are settled in ([`ChecksReport::settle`]); a
    /// synchronous run's report has none.
    pub pending: Vec<CheckId>,
}

impl crate::spoken::Say for ChecksReport {
    fn say(&self, f: &mut fmt::Formatter<'_>, by: crate::spoken::Speaker<'_>) -> fmt::Result {
        if self.findings.is_empty() {
            write!(f, "checks: no findings")?;
        } else {
            write!(f, "checks: {} finding(s)", self.findings.len())?;
            say_findings(f, &self.findings, by)?;
        }
        if !self.skipped.is_empty() {
            write!(f, "\nchecks skipped (severity Off):")?;
            for check in &self.skipped {
                write!(f, " {check}")?;
            }
        }
        if !self.pending.is_empty() {
            write!(f, "\nchecks pending (running at rest):")?;
            for check in &self.pending {
                write!(f, " {check}")?;
            }
        }
        Ok(())
    }
}

/// The sentence where no document is at hand: each placement by its tag.
impl fmt::Display for ChecksReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

impl ChecksReport {
    /// **Settles a job's findings into the report it was split from**:
    /// they take the place in [`ChecksReport::findings`] the
    /// synchronous run gives them, and the check is no longer pending.
    ///
    /// # Panics
    ///
    /// When `door` is not the answer to a job this report is pending
    /// on: another document's, or a report that pends on no door.
    pub fn settle(&mut self, door: DoorFindings) {
        assert_eq!(
            door.document, self.document,
            "a door job's findings settle only into its own document's report"
        );
        let before = self.pending.len();
        self.pending.retain(|c| *c != CheckId::UnprovenCoincidence);
        assert!(
            self.pending.len() < before,
            "the report is not pending on the coincidence door"
        );
        self.findings.splice(door.at..door.at, door.findings);
    }

    /// **The speaker for this report's placements**: each as `doc` holds it
    /// now. A frame that draws the findings one at a time says each
    /// through it ([`crate::spoken::Said`]).
    ///
    /// The guard is the [`crate::DocumentId`], which survives every
    /// edit: it refuses another document, and cannot tell one version
    /// of the checked document from another. Which version — the one
    /// the checks ran over — is the caller's to hand in.
    ///
    /// # Panics
    ///
    /// When `doc` is not the document the checks were run over.
    #[must_use]
    pub fn speaker<'a, P: crate::ProfilePayload>(
        &self,
        doc: &'a Doc<P>,
    ) -> crate::spoken::Speaker<'a> {
        crate::spoken::assert_taken_of("this checks report", self.document, doc);
        crate::spoken::Speaker::of(doc)
    }

    /// **The report as the frame holding the checked document says
    /// it**: each placement as `doc` holds it now.
    ///
    /// # Panics
    ///
    /// When `doc` is not the document the checks were run over.
    #[must_use]
    pub fn spoken<P: crate::ProfilePayload>(&self, doc: &Doc<P>) -> String {
        crate::spoken::Said(self, self.speaker(doc)).to_string()
    }
}

/// Typed refusal of [`run_checks`] itself (closed enum, D4 ¶3) —
/// distinct from a finding: these mean the checks could not be RUN,
/// not that a check fired.
#[derive(Clone, Debug, PartialEq)]
pub enum ChecksError {
    /// A placement has no value in this evaluation, and its standing says
    /// why and where the repair is — the [`crate::product()`] posture:
    /// checks are defined over placements that evaluated, and a report over
    /// a partial evaluation would claim more than was checked.
    Placement(NodeStanding),
    /// The run's tolerance cannot form a band.
    Band {
        /// The band construction failure.
        error: BandError,
    },
    /// The evaluation, or the subject, is of ANOTHER document (DI3).
    ///
    /// Mirrors [`crate::ProductError::EvaluationOfAnotherDocument`],
    /// and for its reason: node ids are minted per document, so a
    /// foreign evaluation can answer every lookup a resident makes and
    /// produce a finding about the wrong geometry without one lookup
    /// missing. Raised before any resident runs.
    EvaluationOfAnotherDocument {
        /// The document the checks were asked for.
        expected: crate::ident::DocumentId,
        /// The document the handed evaluation or subject is of.
        found: crate::ident::DocumentId,
    },
    /// The document's product did not gather, so a
    /// resident that reads the subject has none. The gather's own
    /// refusal, forwarded — this layer has no second opinion about
    /// what a product is.
    ///
    /// Raised by the door, from [`Subject::Unavailable`], and only
    /// when an enabled resident actually reads the subject: residents
    /// that read the evaluation have answered before it.
    Product {
        /// The gather's refusal, whole: its class
        /// ([`product::ProductRefusal::kind`]) is what a consumer
        /// branches on, and its sentence is said by the frame holding
        /// the checked document ([`ChecksError::spoken`]). `None` when
        /// nothing refused: the caller handed a subject that was never
        /// taken ([`Subject::Unavailable`]).
        refusal: Option<product::ProductRefusal>,
    },
}

/// **The pairing predicate's finding, in this door's vocabulary.**
///
/// A2a's rule is one predicate (`ident::mispaired`) and one arm per
/// error type over it. The projection lives HERE, at the type that
/// owns the arm, so a door that runs the predicate writes `?` or
/// `m.into()` and no site re-spells which field goes where.
impl From<crate::ident::Mispaired> for ChecksError {
    fn from(m: crate::ident::Mispaired) -> Self {
        Self::EvaluationOfAnotherDocument {
            expected: m.expected,
            found: m.found,
        }
    }
}

impl crate::spoken::Say for ChecksError {
    fn say(&self, f: &mut fmt::Formatter<'_>, by: crate::spoken::Speaker<'_>) -> fmt::Result {
        match self {
            Self::Placement(standing) => {
                write!(
                    f,
                    "checks: {}",
                    crate::spoken::Said(&standing.of_placement(), by)
                )
            }
            Self::Band { error } => write!(f, "checks: {error}"),
            Self::EvaluationOfAnotherDocument { expected, found } => write!(
                f,
                "checks: the evaluation is of document {found}, not of \
                 document {expected}",
            ),
            Self::Product {
                refusal: Some(refusal),
            } => write!(f, "checks: {}", crate::spoken::Said(refusal, by)),
            Self::Product { refusal: None } => write!(
                f,
                "checks: an enabled check reads the document's product, and none was taken. {}",
                crate::sentence::Recourse("pass the gathered product as the subject")
            ),
        }
    }
}

/// The sentence where no document is at hand: each node by its tag.
impl fmt::Display for ChecksError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

impl ChecksError {
    /// **The refusal as the frame holding the checked document says
    /// it**: each node, a placement's or the gather's, as `doc` holds it now
    /// ([`crate::Doc::spoken`]).
    #[must_use]
    pub fn spoken<P: crate::ProfilePayload>(&self, doc: &crate::Doc<P>) -> String {
        crate::spoken::spoken_by(self, doc)
    }
}

impl core::error::Error for ChecksError {}

/// Refusal from [`enforce_checks`]: the report carries findings whose
/// check is configured at `Error`.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckRefusal {
    /// The document of the report refused ([`ChecksReport::document`]),
    /// the one its human form speaks from ([`CheckRefusal::spoken`]).
    pub document: crate::DocumentId,
    /// The refusing findings (every `Error`-severity finding of the
    /// report, in report order).
    pub findings: Vec<CheckFinding>,
}

impl crate::spoken::Say for CheckRefusal {
    fn say(&self, f: &mut fmt::Formatter<'_>, by: crate::spoken::Speaker<'_>) -> fmt::Result {
        write!(
            f,
            "{} check finding(s) at Error severity:",
            self.findings.len()
        )?;
        say_findings(f, &self.findings, by)
    }
}

/// The sentence where no document is at hand: each placement by its tag.
impl fmt::Display for CheckRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

impl CheckRefusal {
    /// **The refusal as the frame holding the checked document says
    /// it**: each placement as `doc` holds it now.
    ///
    /// # Panics
    ///
    /// When `doc` is not the document the refused report was run over.
    #[must_use]
    pub fn spoken<P: crate::ProfilePayload>(&self, doc: &Doc<P>) -> String {
        crate::spoken::Said(self, self.speaker(doc)).to_string()
    }

    /// **The speaker for this refusal's placements**: each as `doc` holds it
    /// now, under the guard [`ChecksReport::speaker`] states.
    ///
    /// # Panics
    ///
    /// When `doc` is not the document the refused report was run over.
    #[must_use]
    pub fn speaker<'a, P: crate::ProfilePayload>(
        &self,
        doc: &'a Doc<P>,
    ) -> crate::spoken::Speaker<'a> {
        crate::spoken::assert_taken_of("this check refusal", self.document, doc);
        crate::spoken::Speaker::of(doc)
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
/// The three arms are three different facts, and no arm ever stands in
/// for another:
///
/// - [`Subject::Product`] — the gather succeeded and this is it. It
///   must be a product OF THE PAIR the door is handed
///   ([`run_checks_on`] refuses otherwise).
/// - [`Subject::EmptyProduct`] — nothing is placed in the world, the
///   reading [`product::ProductErrorKind::means_no_body`] states and
///   this arm carries ([`Subject::refused`] routes a gather refusal
///   of that class here). A resident that needs a body has no
///   subject here and contributes no finding; a resident that does not
///   (connectedness reads the evaluation) runs exactly as it would
///   otherwise.
/// - [`Subject::Unavailable`] — there is no subject and the reason is
///   carried. Either the gather REFUSED for a cause that is not the
///   absence above, or no enabled resident reads
///   a subject and none was taken
///   ([`ChecksConfig::needs_a_subject`]). A
///   subject-reading resident that is enabled and meets this arm makes
///   the door refuse [`ChecksError::Product`] carrying the refusal;
///   residents that read no subject have already answered by then, so
///   their own refusals still come first.
#[derive(Debug)]
pub enum Subject<'a, T: Decide> {
    /// The gathered product, borrowed for the run.
    Product(&'a product::Product<T>),
    /// Nothing is placed in the world, so there is no product to be
    /// had.
    EmptyProduct,
    /// There is no subject, and this is why.
    Unavailable {
        /// The gather's refusal. `None` when nothing refused: no
        /// enabled resident asked for a subject, so none was taken.
        refusal: Option<product::ProductRefusal>,
    },
}

impl<T: Decide> Subject<'_, T> {
    /// The subject a gather REFUSAL is — and which of two arms it is
    /// is decided HERE, on [`product::ProductErrorKind::means_no_body`].
    ///
    /// **A gather refusal is two different facts, and this is where
    /// they part.** The one class that predicate reads as an ABSENCE
    /// rather than a fault becomes [`Subject::EmptyProduct`]: a merely
    /// empty document is checkable, and routing it to
    /// [`Subject::Unavailable`] would make a subject-reading resident
    /// refuse [`ChecksError::Product`] over it — the outcome that arm
    /// exists to prevent. Every other class is a refusal and becomes
    /// `Unavailable`. Because the routing lives here, a caller holding
    /// a gather error hands it over whole rather than re-deriving a
    /// classification that has one home.
    ///
    /// The `Unavailable` it builds carries the refusal whole, its ids
    /// bare, so the door's refusal is said by the frame holding the
    /// document ([`ChecksError::spoken`]).
    #[must_use]
    pub fn refused(source: product::ProductError) -> Self {
        if source.kind().means_no_body() {
            return Self::EmptyProduct;
        }
        Self::Unavailable {
            refusal: Some(source.into()),
        }
    }

    /// The arm a run that needs no subject is handed
    /// ([`ChecksConfig::needs_a_subject`]). No refusal is behind this
    /// absence, so there is no class to carry.
    fn not_needed() -> Self {
        Self::Unavailable { refusal: None }
    }
}

/// Runs every configured check over `doc`'s evaluated placements and
/// returns the report. **Reports, never gates** (the
/// [`crate::mixed_pins`] posture): nothing calls this from `apply`,
/// from the load door, or from evaluation, and running it changes
/// nothing — the evaluation is read, never rebuilt (DS3 by
/// construction).
///
/// Subjects are the body-denoting outputs of `doc.placements()`, each
/// checked per `(placement, output_ix)`. For the connectedness check, the
/// component count of a subject is its number of definitely-`Outer`
/// shells ([`topo::classify_shells`]) — an internal void is boundary,
/// not a component, at every tolerance row — compared EXACTLY (`!=`)
/// against the expectation. A subject tier 3 refuses is not classified
/// and surfaces as [`CheckEvidence::NotAtRest`]; a shell the door cannot
/// classify surfaces as its own typed finding
/// ([`CheckEvidence::Escalated`] / [`CheckEvidence::Unsupported`]). Neither
/// is a silent skip.
///
/// Expectations are TWO-DIRECTIONAL: every
/// [`ChecksConfig::expected_components`] entry must be consumed by a
/// subject, and an entry no subject consumed — a vanished body (a
/// boolean's honest ∅ result denotes zero subjects), a dead placement, a
/// mistyped key — is its own finding
/// ([`CheckEvidence::StaleExpectation`], appended after the subject
/// findings in key order). The DEFAULT expectation binds only
/// existing subjects: a legitimately empty result with nothing stated
/// about it stays clean.
///
/// # Errors
///
/// [`ChecksError`] — an evaluation of another document, a placement without
/// a value in `ev`, a band the tolerance cannot form, or a document
/// whose placements do not gather into a product for a resident that reads
/// one. These mean the checks could not run at all; a check that ran
/// and disagreed is a FINDING, not an error.
pub fn run_checks<
    P: crate::ProfilePayload,
    T: at_rest::LocalizeLane + CertifiedBounds + ChartCoherenceLane,
>(
    doc: &Doc<P>,
    ev: &Evaluation<T>,
    cfg: &ChecksConfig,
    tol: Tol,
) -> Result<ChecksReport, ChecksError> {
    // LAZY, and the laziness is the point: the gather is by far the
    // most expensive thing this door can do, and a configuration whose
    // enabled residents all read the evaluation has nothing to gather
    // FOR. Asking `needed_by` before gathering is also what keeps a
    // gather refusal from reaching a run no resident would have shown
    // it to.
    if !cfg.needs_a_subject() {
        return run_checks_on(doc, ev, Subject::not_needed(), cfg, tol);
    }
    let subject = match product::product_recorded(doc, ev, tol) {
        Ok(ref gathered) => return run_checks_on(doc, ev, Subject::Product(gathered), cfg, tol),
        Err(source) => Subject::refused(source),
    };
    run_checks_on(doc, ev, subject, cfg, tol)
}

/// The registry over a subject the CALLER derived — the door
/// [`run_checks`] wraps, and the one a caller with a product in hand
/// uses so the document is gathered once.
///
/// Everything [`run_checks`] documents about what is checked, in what
/// order, and what a finding means holds here verbatim; the only
/// difference is where the subject came from.
///
/// **The pairing is checked HERE** (DI3), not inherited from a gather
/// this door does not run: `ev` must be an evaluation of `doc`, and a
/// [`Subject::Product`] must have been gathered from that same
/// document. Both go through [`crate::ident::mispaired`], the one
/// predicate that spells the comparison, before any resident runs —
/// because a resident reading `doc.placements()` against a foreign `ev`
/// finds a value for every placement and reports about the wrong geometry.
///
/// # Errors
///
/// [`ChecksError::EvaluationOfAnotherDocument`] for a mispaired
/// argument; [`ChecksError::Placement`] and [`ChecksError::Band`], the
/// registry's own preconditions; and [`ChecksError::Product`] when an
/// enabled resident reads the subject and finds
/// [`Subject::Unavailable`] — after the residents that read no subject
/// have answered, so their refusals still come first.
pub fn run_checks_on<
    P: crate::ProfilePayload,
    T: at_rest::LocalizeLane + CertifiedBounds + ChartCoherenceLane,
>(
    doc: &Doc<P>,
    ev: &Evaluation<T>,
    subject: Subject<'_, T>,
    cfg: &ChecksConfig,
    tol: Tol,
) -> Result<ChecksReport, ChecksError> {
    let (mut report, door) = run_checks_at_rest_on(doc, ev, subject, cfg, tol)?;
    if let Some(job) = door {
        let Ok(findings) = job.run(doc, &crate::CancelToken::new()) else {
            unreachable!("a token no one else holds is never canceled")
        };
        report.settle(findings);
    }
    Ok(report)
}

/// **The registry with the coincidence door left to run at rest**:
/// what [`run_checks_on`] answers, except that the
/// `unproven-coincidence` check is [`ChecksReport::pending`] and its
/// door is handed back as a [`DoorJob`] for the caller to run off its
/// draw path. [`run_checks_on`] is this with the job run at once. A
/// frame that redraws on every edit takes this one: the door's replay
/// costs seconds on a filleted document, and the next edit cancels the
/// job ([`DoorJob::run`]). Where no row asks for the door, there is no
/// job and nothing is pending.
///
/// # Errors
///
/// As [`run_checks_on`].
pub fn run_checks_at_rest_on<
    P: crate::ProfilePayload,
    T: at_rest::LocalizeLane + CertifiedBounds + ChartCoherenceLane,
>(
    doc: &Doc<P>,
    ev: &Evaluation<T>,
    subject: Subject<'_, T>,
    cfg: &ChecksConfig,
    tol: Tol,
) -> Result<(ChecksReport, Option<DoorJob>), ChecksError> {
    let pairing = |found| crate::ident::mispaired(doc.id(), found).map(ChecksError::from);
    if let Some(refusal) = pairing(ev.document) {
        return Err(refusal);
    }
    if let Subject::Product(gathered) = &subject
        && let Some(refusal) = pairing(gathered.document)
    {
        return Err(refusal);
    }
    let mut report = ChecksReport {
        document: doc.id(),
        findings: Vec::new(),
        skipped: Vec::new(),
        pending: Vec::new(),
    };
    if cfg.severity(CheckId::Connectedness) == Severity::Off {
        report.skipped.push(CheckId::Connectedness);
    } else {
        connectedness(doc, ev, cfg, tol, &mut report)?;
    }
    if cfg.severity(CheckId::ChartCoherence) == Severity::Off {
        report.skipped.push(CheckId::ChartCoherence);
    } else {
        chart_coherence(doc, ev, tol, &mut report)?;
    }
    if cfg.severity(CheckId::Unused) == Severity::Off {
        report.skipped.push(CheckId::Unused);
    } else {
        unused(doc, &mut report);
    }
    let mut door = None;
    if cfg.severity(CheckId::UnprovenCoincidence) == Severity::Off {
        report.skipped.push(CheckId::UnprovenCoincidence);
    } else if let Subject::Unavailable { refusal } = &subject {
        // The at-rest rows ride the product: a mated pair's, and every
        // contact between two copies, so a document whose product did
        // not gather has rows nobody can read.
        return Err(ChecksError::Product {
            refusal: refusal.clone(),
        });
    } else {
        let rows = door_rows(doc, ev, &subject, tol);
        let carried = carried_coincidences(ev);
        if !rows.is_empty() || !carried.is_empty() {
            report.pending.push(CheckId::UnprovenCoincidence);
            door = Some(DoorJob {
                document: doc.id(),
                rows,
                carried,
                resolver: ev.resolver().cloned(),
                tol,
                at: report.findings.len(),
            });
        }
    }
    if cfg.severity(CheckId::AtRest) == Severity::Off {
        report.skipped.push(CheckId::AtRest);
    } else if let Subject::Unavailable { refusal } = &subject {
        // The resident is on and there is no subject: the registry
        // could not run, and the refusal carried here is the one it
        // could not run over.
        return Err(ChecksError::Product {
            refusal: refusal.clone(),
        });
    } else {
        at_rest(doc, ev, &subject, tol, &mut report);
    }
    Ok((report, door))
}

/// **The coincidence door, split off a report to run at rest**
/// ([`run_checks_at_rest_on`]): the rows it owes an answer about, named
/// in the document the report was taken of, and those its parts owe
/// their own doors, each named in its part's ids and judged over that
/// part's pinned document.
#[derive(Clone, Debug)]
pub struct DoorJob {
    document: crate::DocumentId,
    rows: Vec<DoorRows>,
    /// The rows its parts owe their own doors, each once per part row,
    /// in its part's ids.
    carried: Vec<at_rest::CarriedCoincidenceRows>,
    /// The resolver the report's evaluation crossed the seam with, so
    /// the replay builds the same parts.
    resolver: Option<std::sync::Arc<dyn crate::part::PartResolver>>,
    tol: Tol,
    /// Where its findings go among the report's.
    at: usize,
}

impl DoorJob {
    /// **Runs the door** over the job's rows in `doc`: the findings to
    /// settle into the report it was split from
    /// ([`ChecksReport::settle`]).
    ///
    /// # Errors
    ///
    /// [`crate::coincide::Canceled`] once `cancel` is canceled: the
    /// replay checks it between nodes, and the door between the margins
    /// it discharges.
    ///
    /// # Panics
    ///
    /// When `doc` is not the document the report was taken of.
    pub fn run<P: crate::ProfilePayload>(
        &self,
        doc: &Doc<P>,
        cancel: &crate::CancelToken,
    ) -> Result<DoorFindings, crate::coincide::Canceled> {
        crate::spoken::assert_taken_of("this door job", self.document, doc);
        let mut findings =
            door_findings(doc, &self.rows, self.resolver.as_ref(), self.tol, cancel)?;
        for part in &self.carried {
            let own = door_findings(
                &**part.rows.doc.get(),
                &part.rows.sets,
                self.resolver.as_ref(),
                self.tol,
                cancel,
            )?;
            findings.extend(own.into_iter().map(|finding| CheckFinding {
                check: CheckId::UnprovenCoincidence,
                subject: FindingSubject::Node(part.route.through),
                quiet: finding.quiet,
                evidence: CheckEvidence::CarriedCoincidence {
                    finding: Box::new(at_rest::CarriedCoincidence {
                        route: part.route.clone(),
                        pin: part.pin,
                        finding,
                        doc: part.rows.doc.clone(),
                    }),
                },
            }));
        }
        Ok(DoorFindings {
            document: self.document,
            at: self.at,
            findings,
        })
    }
}

/// **A [`DoorJob`]'s answer**, to settle into its report
/// ([`ChecksReport::settle`]).
#[derive(Clone, Debug, PartialEq)]
pub struct DoorFindings {
    document: crate::DocumentId,
    at: usize,
    findings: Vec<CheckFinding>,
}

/// One row set: the subject its findings are reported against, the
/// node whose decision the door proves it at, and its rows in decision
/// order.
pub(crate) type DoorRows = (FindingSubject, RecipeNodeId, Vec<DoorRow>);

/// **One row the door is asked about**, with what the contact half of
/// the quieting rule says of it in the evaluation that decided it.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DoorRow {
    /// The row.
    pub row: crate::coincide::NamedCoincidence,
    /// The assertion that quiets it ([`at_rest::quiet_contact`]), if one
    /// does.
    pub quiet: Option<RecipeNodeId>,
    /// Whether one could ([`at_rest::contact_quietable`]).
    pub quietable: bool,
}

/// **The rows the `unproven-coincidence` resident asks the door
/// about** (D10): every evaluated node's rows, in evaluation order then
/// decision order, then the at-rest census's rows on the gathered
/// product and on each other `⊔` it carries
/// ([`crate::Product::assembles`]), in their order; each
/// under the subject it is reported against — the node that decided it,
/// or the two copies the census found in contact. Every row, a node's
/// or an at-rest one, is quiet under the contact half of the quieting
/// rule ([`at_rest::quiet_contact`]), read here, where the evaluation
/// that decided it is in hand.
pub(crate) fn door_rows<P: crate::ProfilePayload, T: Decide + AtRestPolicy>(
    doc: &Doc<P>,
    ev: &Evaluation<T>,
    subject: &Subject<'_, T>,
    tol: Tol,
) -> Vec<DoorRows> {
    // The rows of the `⊔`s the at-rest check reads (`at_rest`): the
    // product's and each other `⊔`'s, or, with no world product, every
    // `⊔`'s, gathered here as the at-rest check gathers them, so the two
    // residents read one census.
    let elsewhere;
    let (world, assembles): (&[at_rest::AtRestRow], &[product::AssembleGather<T>]) = match subject {
        Subject::Product(gathered) => (&gathered.coincidences, &gathered.assembles),
        Subject::EmptyProduct => {
            elsewhere = product::assemble_gathers(doc, ev, tol, None);
            (&[], &elsewhere)
        }
        Subject::Unavailable { .. } => (&[], &[]),
    };
    let at_rest: Vec<&at_rest::AtRestRow> = world
        .iter()
        .chain(
            assembles
                .iter()
                .filter_map(|assemble| assemble.gather.as_ref().ok())
                .flat_map(|assemble| assemble.coincidences.iter()),
        )
        .collect();
    // A row of two copies is proved at the first copy's placement, which
    // is no decision the replay retakes: the door reads a census row off
    // the replay's copies.
    let at_rest_node = |a: crate::checks::at_rest::CopyRef| {
        doc.operation_of(a.var)
            .unwrap_or_else(|| unreachable!("a gathered copy is a live placement's output"))
    };
    let judged = |row: &crate::coincide::NamedCoincidence| DoorRow {
        row: row.clone(),
        quiet: at_rest::quiet_contact(doc, ev, row, tol),
        quietable: at_rest::contact_quietable(doc, ev, row, tol),
    };
    ev.order
        .iter()
        .filter_map(|&node| {
            ev.value(node).map(|value| {
                (
                    node,
                    value.coincidences.iter().map(judged).collect::<Vec<_>>(),
                )
            })
        })
        .filter(|(_, rows)| !rows.is_empty())
        .map(|(node, rows)| (FindingSubject::Node(node), node, rows))
        .chain(at_rest.into_iter().map(|r| {
            let (a, b) = r.copies;
            (
                FindingSubject::Copies { a, b },
                at_rest_node(a),
                vec![judged(&r.row)],
            )
        }))
        .collect()
}

/// **The rows `ev`'s parts owe their own doors**: every evaluated
/// value's carried rows, in evaluation order, each once per part row
/// ([`at_rest::once_per_part_row`]), so a part placed twice, or an
/// instance and its placements, ask once, by the first route.
pub(crate) fn carried_coincidences<T: Decide>(
    ev: &Evaluation<T>,
) -> Vec<at_rest::CarriedCoincidenceRows> {
    let mut rows = Vec::new();
    for value in ev.order.iter().filter_map(|&node| ev.value(node)) {
        at_rest::once_per_part_row(&mut rows, value.carry.findings.coincidences.iter().cloned());
    }
    rows
}

/// The `unproven-coincidence` resident's findings: each row the door
/// leaves unproven, reported against its row set's subject.
fn door_findings<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    rows: &[DoorRows],
    resolver: Option<&std::sync::Arc<dyn crate::part::PartResolver>>,
    tol: Tol,
    cancel: &crate::CancelToken,
) -> Result<Vec<CheckFinding>, crate::coincide::Canceled> {
    let named: Vec<Vec<crate::coincide::NamedCoincidence>> = rows
        .iter()
        .map(|(_, _, rows)| rows.iter().map(|asked| asked.row.clone()).collect())
        .collect();
    let nodes: Vec<(RecipeNodeId, &[crate::coincide::NamedCoincidence])> = rows
        .iter()
        .zip(&named)
        .map(|((_, node, _), rows)| (*node, &rows[..]))
        .collect();
    let proofs = crate::coincide::prove(doc, &nodes, resolver, tol, cancel)?;
    let mut findings: Vec<CheckFinding> = Vec::new();
    for ((subject, _, rows), proofs) in rows.iter().zip(proofs) {
        let unproven: Vec<_> = rows
            .iter()
            .zip(proofs)
            .filter_map(|(asked, proof)| match proof {
                crate::coincide::Proof::Unproven { residual, recourse } => {
                    Some((&asked.row, residual, recourse, asked))
                }
                crate::coincide::Proof::Structural(_) => None,
            })
            .collect();
        // Each fold row's parent: of the unproven judged rows decided at
        // its point, the least by statement, so the choice does not
        // follow the order the rows were decided in.
        fn statement(
            r: &crate::coincide::NamedCoincidence,
        ) -> (
            Vec<&crate::coincide::NamedCell>,
            topo::Relation,
            topo::DecisionSite,
        ) {
            let mut cells: Vec<&crate::coincide::NamedCell> = r.cells.iter().collect();
            cells.sort();
            (cells, r.relation, r.site)
        }
        let parent = |row: &crate::coincide::NamedCoincidence| {
            unproven
                .iter()
                .enumerate()
                .filter(|(_, (j, _, _, _))| {
                    matches!(j.origin, crate::coincide::RowOrigin::Judged { .. })
                        && j.at_one_vertex(row)
                })
                .min_by(|(i, (a, _, _, _)), (k, (b, _, _, _))| {
                    statement(a).cmp(&statement(b)).then(i.cmp(k))
                })
                .map(|(i, _)| i)
        };
        let parents: Vec<Option<usize>> = unproven
            .iter()
            .map(|(row, _, _, _)| match row.origin {
                crate::coincide::RowOrigin::Fold { .. } => parent(row),
                _ => None,
            })
            .collect();
        let mut at: Vec<Option<usize>> = vec![None; unproven.len()];
        for (i, (row, residual, recourse, asked)) in unproven.iter().enumerate() {
            if parents[i].is_some() {
                continue;
            }
            at[i] = Some(findings.len());
            findings.push(CheckFinding {
                check: CheckId::UnprovenCoincidence,
                subject: *subject,
                evidence: CheckEvidence::UnprovenCoincidence {
                    row: Box::new((*row).clone()),
                    residual: Box::new(residual.clone()),
                    recourse: *recourse,
                    restated: Vec::new(),
                    quietable: asked.quietable,
                },
                quiet: asked.quiet,
            });
        }
        for (i, (row, residual, recourse, _)) in unproven.iter().enumerate() {
            let Some(p) = parents[i] else { continue };
            let Some(CheckEvidence::UnprovenCoincidence { restated, .. }) = at[p]
                .and_then(|f| findings.get_mut(f))
                .map(|f| &mut f.evidence)
            else {
                unreachable!("a fold row's parent is a judged row reported as its own finding")
            };
            restated.push(((*row).clone(), residual.clone(), *recourse));
        }
    }
    Ok(findings)
}

/// The connectedness resident's own pass (I1(0b)) — [`run_checks`]'s
/// body before the registry grew a second resident, moved out
/// unchanged so each resident is independently `Off`-able.
fn connectedness<P: crate::ProfilePayload, T: Decide + CertifiedBounds + AtRestPolicy>(
    doc: &Doc<P>,
    ev: &Evaluation<T>,
    cfg: &ChecksConfig,
    tol: Tol,
    report: &mut ChecksReport,
) -> Result<(), ChecksError> {
    // Entries not yet consumed by a subject; whatever remains after
    // the walk is stale (an expectation with no subject).
    let mut unconsumed = cfg.expected_components.clone();
    for placement in doc.placements() {
        let value = ev.usable(placement).map_err(ChecksError::Placement)?;
        let Some(sources) = product::sources_of(value) else {
            continue;
        };
        for (output_ix, body, _contacts, _rows) in sources {
            unconsumed.remove(&(placement, output_ix));
            // The shell door reads a finished body. Nothing upstream keeps a
            // placement's verdict in its value yet (F2 carries it), so the placement is
            // gated here, once.
            let body = match T::gate_at_rest_kept(body.body().clone(), tol) {
                Ok(body) => body,
                Err(errors) => {
                    report.findings.push(CheckFinding {
                        check: CheckId::Connectedness,
                        subject: FindingSubject::Output {
                            placement,
                            output_ix,
                        },
                        evidence: CheckEvidence::NotAtRest { errors },
                        quiet: None,
                    });
                    continue;
                }
            };
            match classify_shells(&body, tol) {
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
                        .get(&(placement, output_ix))
                        .copied()
                        .unwrap_or(1);
                    if actual != expected {
                        report.findings.push(CheckFinding {
                            check: CheckId::Connectedness,
                            subject: FindingSubject::Output {
                                placement,
                                output_ix,
                            },
                            evidence: CheckEvidence::Connectedness { actual, expected },
                            quiet: None,
                        });
                    }
                }
                Err(ShellClassifyError::Band { error }) => {
                    return Err(ChecksError::Band { error });
                }
                Err(
                    source @ (ShellClassifyError::Escalated { .. }
                    | ShellClassifyError::ZeroVolume { .. }
                    | ShellClassifyError::Straddles { .. }
                    | ShellClassifyError::RayInBand { .. }
                    | ShellClassifyError::Corner { .. }),
                ) => {
                    report.findings.push(CheckFinding {
                        check: CheckId::Connectedness,
                        subject: FindingSubject::Output {
                            placement,
                            output_ix,
                        },
                        evidence: CheckEvidence::Escalated { source },
                        quiet: None,
                    });
                }
                Err(
                    source @ (ShellClassifyError::Props { .. }
                    | ShellClassifyError::Ray { .. }
                    | ShellClassifyError::SignUnresolved { .. }),
                ) => {
                    report.findings.push(CheckFinding {
                        check: CheckId::Connectedness,
                        subject: FindingSubject::Output {
                            placement,
                            output_ix,
                        },
                        evidence: CheckEvidence::Unsupported { source },
                        quiet: None,
                    });
                }
            }
        }
    }
    // The reverse direction: every stated expectation must have found
    // its subject. BTreeMap iteration = key order (deterministic, D9).
    for ((placement, output_ix), expected) in unconsumed {
        report.findings.push(CheckFinding {
            check: CheckId::Connectedness,
            subject: FindingSubject::Output {
                placement,
                output_ix,
            },
            evidence: CheckEvidence::StaleExpectation { expected },
            quiet: None,
        });
    }
    Ok(())
}

/// The chart-coherence resident's pass: [`topo::examine_chart_coherence`]
/// over each rest body, with BOTH of that door's lists carried across
/// — and carried across as two different facts.
///
/// # The two lists, and the one fold this door must not make
///
/// [`topo::CoherenceReport`] is `{ findings, unexamined }` and
/// [`ChecksReport`] is `{ findings, skipped }`. The shapes rhyme and
/// the meanings do not: `skipped` is CONFIGURATION (a check a caller
/// set to `Off`), `unexamined` is DATA (a loop the body put out of the
/// door's reach). Nothing in `unexamined` reaches `skipped` here. Each
/// entry becomes its own finding
/// ([`CheckEvidence::ChartCoherenceUnexamined`]), which is what the
/// connectedness resident already does with its own could-not-look
/// arms (`Escalated`, `Unsupported`) and is the F6 posture: a check
/// that could not look says so, and never by staying quiet.
///
/// # Order (D9)
///
/// Placements in document order, and within one copy the door's own
/// total order — its findings, then its unexamined loops. A pure function of the
/// evaluation and eps, as the door itself is.
fn chart_coherence<P: crate::ProfilePayload, T: Decide + ChartCoherenceLane>(
    doc: &Doc<P>,
    ev: &Evaluation<T>,
    tol: Tol,
    report: &mut ChecksReport,
) -> Result<(), ChecksError> {
    for placement in doc.placements() {
        let value = ev.usable(placement).map_err(ChecksError::Placement)?;
        let Some(sources) = product::sources_of(value) else {
            continue;
        };
        for (output_ix, body, _contacts, _rows) in sources {
            let at = |evidence| CheckFinding {
                check: CheckId::ChartCoherence,
                subject: FindingSubject::Output {
                    placement,
                    output_ix,
                },
                evidence,
                quiet: None,
            };
            let Some(examined) = T::examine_chart_coherence(body.body(), tol) else {
                report
                    .findings
                    .push(at(CheckEvidence::ChartCoherenceUnavailable));
                continue;
            };
            report.findings.extend(
                examined
                    .findings
                    .into_iter()
                    .map(|finding| at(CheckEvidence::ChartCoherence { finding })),
            );
            report.findings.extend(
                examined
                    .unexamined
                    .into_iter()
                    .map(|unexamined| at(CheckEvidence::ChartCoherenceUnexamined { unexamined })),
            );
        }
    }
    Ok(())
}

/// The `unused` check: [`Doc::unused`]'s tips, its operations then its
/// variables, each operation with the outputs the person named, which
/// its finding speaks for. A part's are the part's, and none is
/// carried here.
fn unused<P: crate::ProfilePayload>(doc: &Doc<P>, report: &mut ChecksReport) {
    let tips = doc.unused();
    let operations = tips.operations.into_iter().map(|node| {
        let named = doc
            .outputs(node)
            .into_iter()
            .filter(|var| doc.var_name(*var).is_some())
            .collect();
        (FindingSubject::Node(node), named)
    });
    let vars = (tips.vars.into_iter()).map(|var| (FindingSubject::Var(var), Vec::new()));
    report
        .findings
        .extend(operations.chain(vars).map(|(subject, named)| CheckFinding {
            check: CheckId::Unused,
            subject,
            evidence: CheckEvidence::Unused { named },
            quiet: None,
        }));
}

/// **The at-rest resident's pass** (D10 `[assertions]`): each census at
/// rest as the gather kept it, the product's first and then each `⊔` the
/// product does not read ([`crate::Product::assembles`]), in document
/// order. Each reports, under its `⊔`, its interference findings and
/// what it could not look at ([`at_rest::reading`]), then the findings
/// its parts carried up, each part row once across the whole report. A
/// `⊔` whose operands did not gather is one could-not-look finding. A
/// world with no body reports every `⊔` beside it all the same.
///
/// It runs no census and no predicate of its own: the census ran once,
/// in the gather, and this reads it.
fn at_rest<P: crate::ProfilePayload, T: at_rest::LocalizeLane>(
    doc: &Doc<P>,
    ev: &Evaluation<T>,
    subject: &Subject<'_, T>,
    tol: Tol,
    report: &mut ChecksReport,
) {
    let elsewhere;
    let (world, assembles) = match subject {
        Subject::Product(gathered) => (
            doc.product().map(|read| (read, &**gathered)),
            &gathered.assembles[..],
        ),
        Subject::EmptyProduct => {
            elsewhere = product::assemble_gathers(doc, ev, tol, None);
            (None, &elsewhere[..])
        }
        Subject::Unavailable { .. } => return,
    };
    let mut carried_interference: Vec<at_rest::CarriedInterference> = Vec::new();
    let mut carried_at_rest: Vec<at_rest::CarriedAtRestFinding> = Vec::new();
    let finding = |read, evidence, quiet| CheckFinding {
        check: CheckId::AtRest,
        subject: FindingSubject::Census { read },
        evidence,
        quiet,
    };
    let gathers = world
        .map(|(read, product)| (read, Ok(product)))
        .into_iter()
        .chain(
            assembles
                .iter()
                .map(|a| (a.var, a.gather.as_ref().map(|p| &**p))),
        );
    for (read, gather) in gathers {
        let product = match gather {
            Ok(product) => product,
            Err(refusal) if refusal.kind().means_no_body() => continue,
            Err(refusal) => {
                let evidence = at_rest::AtRestEvidence::NotGathered(refusal.clone());
                report
                    .findings
                    .push(finding(read, CheckEvidence::AtRest { evidence }, None));
                continue;
            }
        };
        let reading = at_rest::reading(product, tol);
        for interference in reading.interference {
            let quiet = interference.quiet;
            let evidence = CheckEvidence::Interference {
                finding: Box::new(interference),
            };
            report.findings.push(finding(read, evidence, quiet));
        }
        for evidence in reading.could_not_look {
            report
                .findings
                .push(finding(read, CheckEvidence::AtRest { evidence }, None));
        }
        let earlier = carried_interference.len();
        at_rest::once_per_part_row(
            &mut carried_interference,
            product.carried_interference.iter().cloned(),
        );
        for row in &carried_interference[earlier..] {
            let quiet = row.finding.quiet;
            let evidence = CheckEvidence::CarriedInterference {
                finding: Box::new(row.clone()),
            };
            report.findings.push(finding(read, evidence, quiet));
        }
        let earlier = carried_at_rest.len();
        at_rest::once_per_part_row(
            &mut carried_at_rest,
            product.carried_at_rest.iter().cloned(),
        );
        for row in &carried_at_rest[earlier..] {
            let evidence = at_rest::AtRestEvidence::Carried(Box::new(row.clone()));
            report
                .findings
                .push(finding(read, CheckEvidence::AtRest { evidence }, None));
        }
    }
}

/// The door from a finding's attribution back to its subject: the body
/// at `(placement, output_ix)` in this evaluation, AND the declared contact
/// records that body carries — the same enumeration the connectedness
/// resident walks, so a [`CheckFinding`]'s attribution always resolves
/// against the evaluation it was produced from. `None` when the placement has no
/// value, denotes no body, or has no output at that index (exactly the
/// attributions a [`CheckEvidence::StaleExpectation`] finding names).
///
/// The two halves travel TOGETHER because a body without its
/// declarations is a different subject from the one the check flagged.
/// A body's records ride its value's carry ([`crate::Carry::contacts`]):
/// a boolean's own, an instance's carried D-1 declarations, each member's
/// of a family its own; [`product::sources_of`] builds the pair, and
/// this door hands the whole pair on rather than dropping half of it.
/// Splitting them again downstream re-opens the failure
/// this signature closes: a subject that IS a declared boolean result
/// reporting its own certified seam as an UNRECORDED contact under the
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
    placement: RecipeNodeId,
    output_ix: u32,
) -> Option<(crate::eval::Framed<T>, Arc<ContactRecords>)> {
    let sources = product::sources_of(ev.value(placement)?)?;
    sources
        .into_iter()
        .find(|(ix, _, _, _)| *ix == output_ix)
        .map(|(_, body, contacts, _)| (body, contacts))
}

/// The ONE refusing path of the registry: refuses iff `report` carries
/// a loud finding whose check `cfg` sets to [`Severity::Error`]. The
/// caller chooses where to gate — this module never does. A quiet
/// finding ([`CheckFinding::quiet`]) never refuses, and neither does a
/// finding outside what the document can yet say
/// ([`CheckEvidence::never_refuses`]), at any severity (DS6).
///
/// Consumes a FINISHED report (DS3: enforcement is a read over
/// findings; it evaluates nothing and can change nothing). `Warn`
/// findings pass; a report whose findings are all `Warn`-severity is
/// `Ok(())` with the findings still in the report for the caller to
/// render.
///
/// # Errors
///
/// [`CheckRefusal`], carrying every refusing finding in report order.
pub fn enforce_checks(report: &ChecksReport, cfg: &ChecksConfig) -> Result<(), CheckRefusal> {
    let findings: Vec<CheckFinding> = report
        .findings
        .iter()
        .filter(|finding| {
            cfg.severity(finding.check) == Severity::Error
                && finding.quiet.is_none()
                && !finding.evidence.never_refuses()
        })
        .cloned()
        .collect();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(CheckRefusal {
            document: report.document,
            findings,
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{CheckEvidence, RecipeNodeId, Subject};

    /// A stand-in finding, told apart by its count.
    fn finding(check: super::CheckId, expected: u32) -> super::CheckFinding {
        super::CheckFinding {
            check,
            subject: super::FindingSubject::Node(RecipeNodeId::new(0, 7)),
            evidence: CheckEvidence::StaleExpectation { expected },
            quiet: None,
        }
    }

    /// INVARIANT: a door job's findings settle where the synchronous
    /// run puts them, after the residents before the door and before
    /// those after it, and the check stops pending. Reds if `settle`
    /// appends rather than splices.
    #[test]
    fn a_door_job_settles_its_findings_in_the_doors_place() {
        use super::CheckId::{AtRest, Connectedness, UnprovenCoincidence};
        let document = crate::DocumentId(1);
        let mut report = super::ChecksReport {
            document,
            findings: vec![finding(Connectedness, 1), finding(AtRest, 3)],
            skipped: Vec::new(),
            pending: vec![UnprovenCoincidence],
        };
        report.settle(super::DoorFindings {
            document,
            at: 1,
            findings: vec![finding(UnprovenCoincidence, 2)],
        });
        assert!(report.pending.is_empty());
        assert_eq!(
            report.findings,
            [
                finding(Connectedness, 1),
                finding(UnprovenCoincidence, 2),
                finding(AtRest, 3)
            ]
        );
    }

    /// The variant name `Debug` opens with.
    fn variant_of(debug: &str) -> String {
        debug
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect()
    }

    /// INVARIANT: the subject door's unavailable arm carries the gather
    /// refusal it was handed, whole, so the checks door can forward it
    /// for the frame holding the document to say.
    ///
    /// The refusal below is reachable: a product whose body was
    /// deleted gathers into `ProductError::StrandedProduct`.
    #[test]
    fn the_subject_door_carries_the_gather_refusal_it_saw() {
        let refusal = || crate::ProductError::StrandedProduct {
            read: crate::VarId::new(0, 7),
        };
        let subject: Subject<'_, f64> = Subject::refused(refusal());
        let Subject::Unavailable {
            refusal: Some(carried),
        } = &subject
        else {
            panic!("the arm this row is about: {subject:?}");
        };
        assert_eq!(
            format!("{:?}", carried.error()),
            format!("{:?}", refusal()),
            "the refusal crosses whole"
        );
        assert_eq!(
            format!("{:?}", carried.kind()),
            variant_of(&format!("{:?}", refusal())),
            "and its class is the arm the error is"
        );
    }

    /// A door handed a subject that was never taken, with a check on
    /// that reads one, says so and says what to pass.
    #[test]
    fn a_subject_never_taken_refuses_in_its_own_words() {
        assert_eq!(
            super::ChecksError::Product { refusal: None }.to_string(),
            "checks: an enabled check reads the document's product, and none was taken. \
             Recourse: pass the gathered product as the subject"
        );
    }

    /// INVARIANT: the subject door ROUTES the refusal it is handed.
    /// The one class [`product::ProductErrorKind::means_no_body`]
    /// reads as an ABSENCE — `ProductError::EmptyProduct`, a document
    /// whose world holds nothing — becomes [`Subject::EmptyProduct`];
    /// every other class becomes [`Subject::Unavailable`].
    ///
    /// That arm is the one refusal that must not go through the
    /// unavailable door: a merely empty document routed there makes
    /// every enabled subject-reading resident refuse
    /// [`ChecksError::Product`] over a document where nothing has gone
    /// wrong. The routing lives in the door precisely so no caller
    /// has to remember it.
    ///
    /// **Which arm is which is not written down on the left.** The arm
    /// the door picks is compared against what the CLASSIFICATION says
    /// of the same error, so a door that stopped reading it reds here
    /// while a re-classification moves both sides together — and the
    /// set of classes that read as an absence is pinned where it is
    /// stated, at `product`'s `exactly_one_arm_reads_as_no_body`.
    #[test]
    fn the_subject_door_routes_an_absence_away_from_the_unavailable_arm() {
        for refusal in [
            crate::ProductError::EmptyProduct,
            crate::ProductError::Read(crate::NodeStanding::Poisoned {
                node: RecipeNodeId::new(0, 7),
                through: RecipeNodeId::new(0, 2),
            }),
        ] {
            let absence = refusal.kind().means_no_body();
            let shown = format!("{refusal:?}");
            let subject: Subject<'_, f64> = Subject::refused(refusal);
            assert_eq!(
                matches!(subject, Subject::EmptyProduct),
                absence,
                "{shown}: the door's arm must follow the classification"
            );
            assert_eq!(
                matches!(subject, Subject::Unavailable { .. }),
                !absence,
                "{shown}: and the arm it is NOT is the refusing one"
            );
        }
    }

    /// INVARIANT: a subject that is absent because nothing ASKED for
    /// one carries no class — `None` is the honest answer where there
    /// is no refusal, never a kind minted to fill the field.
    #[test]
    fn a_subject_no_resident_asked_for_carries_no_refusal_class() {
        let subject: Subject<'_, f64> = Subject::not_needed();
        assert!(
            matches!(subject, Subject::Unavailable { refusal: None }),
            "{subject:?}"
        );
    }

    /// INVARIANT: a could-not-look finding never refuses, at any
    /// severity, and a quiet finding never refuses either; a loud one at
    /// `Error` does. Reds if `enforce_checks` reads severity alone.
    #[test]
    fn only_a_loud_judged_finding_refuses() {
        let document = crate::DocumentId(1);
        let at = |evidence, quiet| super::CheckFinding {
            check: super::CheckId::AtRest,
            subject: super::FindingSubject::Census {
                read: crate::VarId::WORLD,
            },
            evidence,
            quiet,
        };
        let not_run = || CheckEvidence::AtRest {
            evidence: super::at_rest::AtRestEvidence::NotRun,
        };
        let report = |findings| super::ChecksReport {
            document,
            findings,
            skipped: Vec::new(),
            pending: Vec::new(),
        };
        let error = super::ChecksConfig {
            at_rest: super::Severity::Error,
            ..super::ChecksConfig::default()
        };
        let could_not_look = report(vec![at(not_run(), None)]);
        assert!(super::enforce_checks(&could_not_look, &error).is_ok());
        let loud = report(vec![finding(super::CheckId::Connectedness, 2)]);
        let connectedness_error = super::ChecksConfig {
            connectedness: super::Severity::Error,
            ..super::ChecksConfig::default()
        };
        assert!(super::enforce_checks(&loud, &connectedness_error).is_err());
        let mut quiet = finding(super::CheckId::Connectedness, 2);
        quiet.quiet = Some(RecipeNodeId::new(0, 1));
        assert!(super::enforce_checks(&report(vec![quiet]), &connectedness_error).is_ok());
    }
}
