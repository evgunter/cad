//! **The reporting layer** (ERROR-DESIGN E10's derived half, E11.6):
//! the two doors every derived report carries, the honesty type the
//! accounting was missing, the leaf-mass histogram, and the one cache.
//!
//! # Two doors, and they are not each other's substitute
//!
//! Every derived report in this lane carries a `serialize()` and a
//! `render()`, and they answer different questions.
//!
//! - **`serialize()` is the GOLDENING form**: deterministic text with
//!   every float written as its exact bits. It is what a CI row
//!   compares, so it must not round, must not reorder, and must move
//!   whenever the report moves. It is unreadable on purpose — a form
//!   that is pleasant to read is a form that has dropped something.
//! - **`render()` is the HUMAN form**: masses as percentages, values as
//!   numbers, the tail on every line, every unavailability spelled out.
//!   It is what a consumer reads, and it is never what a golden pins:
//!   a rendering that gates would have to freeze its own wording.
//!
//! The pair is the E5 rule applied to text — "labeling and ordering,
//! not omission". The reviewers' consumer friction that M10-6 answers
//! was exactly this: `Stackup` had neither door, so its only rendering
//! was `Debug` and its masses read as hex bits.
//!
//! # What a content key is FOR here (D9)
//!
//! A derived report is a pure function of (the recipe slice, the box,
//! ε, K, the config). Its content key is the identity of that tuple's
//! IMAGE — taken over the goldening form, so the key moves exactly when
//! the report's own bits move and never merely because a run happened
//! twice. That is what makes the key the proof (D9) and what lets
//! [`ReportCache`] serve a report without re-deriving it.

use std::collections::BTreeMap;

use crate::analysis::{AnalyzedBox, MeasureUnavailable};
use crate::distribution::Distribution;
use crate::doc::ParamName;
use crate::eval::{ContentKey, KeyHasher, key_of};

/// **Priced or forced** — the honesty type the unresolved-mass budget
/// was missing (the M10-1 adjudication's R2 MINOR-1, this unit's named
/// obligation).
///
/// `box_mass(Band, covering) = 1` is CORRECT and it is measure-free:
/// every probability measure consistent with a band puts all of its
/// mass inside the band's own limits, so a box covering the support
/// carries mass 1 without anyone having stated a shape. The number is
/// right; the SENTENCE a report writes around it was wrong. "99.6%
/// certified, 0.4% refused" over a Band-only document reads as a
/// probability statement, and there is no probability in it — nothing
/// was priced, because nothing was ever priced-able.
///
/// So the basis rides the budget as a type. A consumer cannot read the
/// percentage without meeting it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MassBasis {
    /// Every varying parameter carries a stated distribution, so every
    /// mass in the budget is an integral of a stated measure.
    Priced,
    /// At least one varying parameter carries a
    /// [`Band`](Distribution::Band): limits without a shape. The masses
    /// are still exact — they are what set theory forces on any measure
    /// consistent with the band — but they are not probabilities, and a
    /// report that called them "priced" would be claiming a shape
    /// nobody stated.
    Forced {
        /// Every band-carrying parameter, in name order.
        by: Vec<ParamName>,
    },
}

impl MassBasis {
    /// The basis of an analyzed box: forced when any VARYING axis
    /// carries a band, priced otherwise.
    ///
    /// Fixed axes do not count and the reason is E2's own: a parameter
    /// with no distribution is the point mass at its nominal, a real
    /// measure with σ = 0, and a document of nothing but fixed
    /// parameters is priced (trivially, and truthfully).
    pub fn of(analyzed: &AnalyzedBox) -> Self {
        let by: Vec<ParamName> = analyzed
            .varying()
            .filter(|(_, p)| matches!(p.distribution, Some(Distribution::Band { .. })))
            .map(|(name, _)| name.clone())
            .collect();
        if by.is_empty() {
            Self::Priced
        } else {
            Self::Forced { by }
        }
    }

    /// The one word a rendering uses for this basis.
    pub fn word(&self) -> &'static str {
        match self {
            Self::Priced => "priced",
            Self::Forced { .. } => "forced",
        }
    }
}

impl core::fmt::Display for MassBasis {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Priced => f.write_str(
                "priced: every varying parameter carries a stated distribution, so these \
                 masses are integrals of a stated measure",
            ),
            Self::Forced { by } => write!(
                f,
                "FORCED, not priced: {} carr{} a band — limits with no shape — so these \
                 masses are what set theory forces on any measure consistent with those \
                 limits, and none of them is a probability",
                by.iter()
                    .map(|p| p.0.clone())
                    .collect::<Vec<_>>()
                    .join(", "),
                if by.len() == 1 { "ies" } else { "y" }
            ),
        }
    }
}

/// **The unresolved-mass budget, with its basis** (E2/E10's one honesty
/// gate, made type-honest).
///
/// The numbers are the drive's own, verbatim — this recomputes
/// nothing. What it adds is the [`MassBasis`] beside them and the two
/// doors every report in this lane carries.
#[derive(Debug, Clone, PartialEq)]
pub struct MassBudget {
    /// Mass on certified leaves.
    pub certified: Result<f64, MeasureUnavailable>,
    /// Refused mass plus tail — the number a CI row bounds.
    pub unresolved: Result<f64, MeasureUnavailable>,
    /// Refused mass per reason class, in class order.
    pub refused: BTreeMap<&'static str, Result<f64, MeasureUnavailable>>,
    /// The tail outside the analyzed box (E2).
    pub tail: Result<f64, MeasureUnavailable>,
    /// Whether the analyzed box's own leaves were `FlipCrossing`-
    /// refused at the boundary, which makes the budget exact rather
    /// than conservative (E2's amendment).
    pub containment: bool,
    /// Priced or forced ([`MassBasis`]).
    pub basis: MassBasis,
}

impl MassBudget {
    /// The budget of a drive over an analyzed box.
    pub fn of(accounting: &crate::drive::MeasureAccounting, analyzed: &AnalyzedBox) -> Self {
        Self {
            certified: accounting.certified.clone(),
            unresolved: accounting.unresolved(),
            refused: accounting
                .refused
                .iter()
                .map(|(class, m)| (class.name(), m.clone()))
                .collect(),
            tail: accounting.unanalyzed.clone(),
            containment: accounting.containment,
            basis: MassBasis::of(analyzed),
        }
    }

    /// The goldening form: exact bits, the driver's idiom.
    pub fn serialize(&self) -> String {
        use core::fmt::Write as _;
        let mut s = String::new();
        let _ = writeln!(s, "basis {}", self.basis.word());
        if let MassBasis::Forced { by } = &self.basis {
            for p in by {
                let _ = writeln!(s, "band {}", p.0);
            }
        }
        let _ = writeln!(s, "certified {}", mass_bits(&self.certified));
        for (class, m) in &self.refused {
            let _ = writeln!(s, "refused {class} {}", mass_bits(m));
        }
        let _ = writeln!(s, "tail {}", mass_bits(&self.tail));
        let _ = writeln!(s, "unresolved {}", mass_bits(&self.unresolved));
        let _ = writeln!(s, "containment {}", self.containment);
        s
    }

    /// The content key of everything [`Self::serialize`] renders.
    pub fn content_key(&self) -> ContentKey {
        key_of(0xEA, &self.serialize())
    }

    /// The human form: percentages, the basis spelled out, the tail on
    /// its own line, and every unavailability named.
    pub fn render(&self) -> String {
        use core::fmt::Write as _;
        let mut s = String::new();
        let _ = writeln!(s, "{}", self.basis);
        let _ = writeln!(s, "  certified   {}", percent(&self.certified));
        for (class, m) in &self.refused {
            let _ = writeln!(s, "  refused ({class}) {}", percent(m));
        }
        let _ = writeln!(s, "  tail        {}", percent(&self.tail));
        let _ = writeln!(
            s,
            "  UNRESOLVED  {} ({})",
            percent(&self.unresolved),
            if self.containment {
                "exact: the analyzed box contains the witness chamber"
            } else {
                "conservative: the witness chamber may reach past the analyzed box"
            }
        );
        s
    }
}

/// **The E11.6 histogram datum**: per certified leaf, its mass and the
/// measure's enclosure over it.
///
/// ADVISORY, and it claims nothing new. Every row is a pair of numbers
/// the drive and the evaluation already produced — a leaf's mass under
/// the product measure (E6) and the measure's interval enclosure over
/// that leaf (the same enclosure `Stackup::worst_case` hulls). What the
/// table adds is the JOIN, which is what a histogram of an output
/// quantity is made of: each leaf spreads its mass over its own output
/// interval.
///
/// It is not a density. E11.6 is explicit that true output densities
/// are v2; this is the cheap note beside that exclusion, and the
/// rendering says so on its own first line rather than leaving a reader
/// to infer it from a doc comment they are not reading.
#[derive(Debug, Clone, PartialEq)]
pub struct LeafHistogram {
    /// The measure the rows are about.
    pub measurement: crate::node::RecipeNodeId,
    /// One row per certified leaf, in the drive's own leaf order.
    pub rows: Vec<HistogramRow>,
    /// The mass this table does NOT cover: the drive's unresolved
    /// budget, carried so no row of it can be read as a whole.
    pub uncovered: Result<f64, MeasureUnavailable>,
    /// Priced or forced, as everywhere else in this module.
    pub basis: MassBasis,
}

/// One certified leaf's row: what it weighs and where it lands.
#[derive(Debug, Clone, PartialEq)]
pub struct HistogramRow {
    /// The leaf's box, rendered in the driver's own form.
    pub leaf: String,
    /// Its mass under the product measure.
    pub mass: Result<f64, MeasureUnavailable>,
    /// The measure's enclosure over it, `[lo, hi]`.
    pub enclosure: (f64, f64),
}

impl LeafHistogram {
    /// The goldening form: exact bits, one line per leaf.
    pub fn serialize(&self) -> String {
        use core::fmt::Write as _;
        let mut s = String::new();
        let _ = writeln!(
            s,
            "histogram measure={} rows={}",
            self.measurement.0,
            self.rows.len()
        );
        let _ = writeln!(s, "basis {}", self.basis.word());
        for row in &self.rows {
            let _ = writeln!(
                s,
                "leaf {} mass={} enclosure={:016x},{:016x}",
                row.leaf,
                mass_bits(&row.mass),
                row.enclosure.0.to_bits(),
                row.enclosure.1.to_bits()
            );
        }
        let _ = writeln!(s, "uncovered {}", mass_bits(&self.uncovered));
        s
    }

    /// The content key of everything [`Self::serialize`] renders.
    pub fn content_key(&self) -> ContentKey {
        key_of(0xEB, &self.serialize())
    }

    /// The human form. The advisory label is the first line, and the
    /// uncovered mass is the last: a reader meets both without looking
    /// for them.
    pub fn render(&self) -> String {
        use core::fmt::Write as _;
        let mut s = String::new();
        let _ = writeln!(
            s,
            "ADVISORY leaf-mass histogram of node {} — leaf mass against the measure's \
             certified enclosure over that leaf. Not a density: a true output density is v2 \
             (E11.6), and nothing here claims one.",
            self.measurement.0
        );
        let _ = writeln!(s, "{}", self.basis);
        for row in &self.rows {
            let _ = writeln!(
                s,
                "  {:>8} of mass in [{}, {}]   {}",
                percent(&row.mass),
                row.enclosure.0,
                row.enclosure.1,
                row.leaf
            );
        }
        let _ = writeln!(
            s,
            "  {:>8} NOT covered by any row (the drive's unresolved budget)",
            percent(&self.uncovered)
        );
        s
    }
}

/// **Builds the E11.6 histogram** over a drive's certified leaves.
///
/// One interval replay per certified leaf, exactly as
/// `stackup::worst_case` runs — and the enclosure a row carries is the
/// same enclosure that hull is taken over, read through the same
/// payload. Nothing here re-derives geometry a second way.
///
/// A leaf whose replay does not produce a measured value contributes NO
/// row and its mass falls into `uncovered`: a histogram row is a mass
/// AT a value, and a leaf with no value has no place to put its mass.
/// That is the honest direction — the uncovered column grows rather
/// than a row appearing at a made-up interval.
///
/// # Errors
///
/// Nothing: a leaf that cannot be read is accounted, not refused. The
/// mass columns carry [`MeasureUnavailable`] where a band blocks
/// pricing, exactly as the drive's own accounting does.
/// # What it costs (M10-6's review, MINOR-13)
///
/// E11.6 calls this datum "near-free" because its two columns are
/// numbers the drive and the evaluation already produced. That is true
/// of the NUMBERS and not of this function: it re-evaluates the
/// document over every certified leaf to read the measure's enclosure,
/// which duplicates exactly the replays `Stackup::worst_case` already
/// did. A consumer that wants both pays for the leaves twice.
///
/// Not shared here, and the reason is a signature: `worst_case` hulls
/// the enclosures and discards them, so sharing means threading a
/// per-leaf vector out of it and into this — a change to two public
/// shapes for a report that is advisory. Disclosed instead, so nobody
/// reads "near-free" as "free".
pub fn leaf_histogram(
    doc: &crate::doc::Doc<crate::program::ProfileProgram>,
    analyzed: &AnalyzedBox,
    verdict: &crate::drive::ParamBoxVerdict,
    measurement: crate::node::RecipeNodeId,
    tol: geom_core::Tol,
) -> LeafHistogram {
    use geom_core::Bounds as _;

    let mut rows = Vec::new();
    let mut unplaced: Result<f64, MeasureUnavailable> = Ok(0.0);
    for leaf in verdict.certified() {
        let opts = crate::eval::EvalOptions {
            param_box: Some(std::sync::Arc::new(leaf.box_.clone())),
            profile_lift: crate::eval::ProfileLift::Guided,
            ..crate::eval::EvalOptions::default()
        };
        let ev: crate::eval::Evaluation<geom_core::Interval> =
            crate::eval::evaluate(doc, None, &crate::eval::CancelToken::new(), &opts, tol);
        let mass = leaf.box_.mass(analyzed);
        let enclosure = match ev.result(measurement) {
            Some(crate::eval::NodeResult::Ok(v)) => match &v.payload {
                crate::eval::ValuePayload::Measure { value, .. } => Some((value.lo(), value.hi())),
                _ => None,
            },
            _ => None,
        };
        match enclosure {
            Some(enclosure) => rows.push(HistogramRow {
                leaf: crate::drive::render_box(&leaf.box_),
                mass,
                enclosure,
            }),
            None => {
                if let (Ok(acc), Ok(m)) = (&mut unplaced, &mass) {
                    *acc += m;
                } else if let Err(e) = mass {
                    unplaced = Err(e);
                }
            }
        }
    }
    // The uncovered column is the drive's own unresolved budget PLUS
    // the certified mass no row could place. Two different reasons for
    // "this table does not speak for it", summed because a reader's
    // question is one: how much of the measure is not in the rows.
    let uncovered = match (verdict.accounting().unresolved(), unplaced) {
        (Ok(a), Ok(b)) => Ok(a + b),
        (Err(e), _) | (_, Err(e)) => Err(e),
    };
    LeafHistogram {
        measurement,
        rows,
        uncovered,
        basis: MassBasis::of(analyzed),
    }
}

/// **The cache seam** (E10: "content-key cached on the bit-content of
/// (recipe slice, box, ε, K)").
///
/// In-process, never persisted, and deliberately small: a map from a
/// key a caller computed to the report it stands for. It is a DOOR, not
/// a subsystem — no eviction policy, no statistics, no background
/// anything — and the key function is [`report_key`], which a consumer
/// can call without ever touching the cache.
///
/// **What it can and cannot serve.** It serves a report to a key that
/// is EQUAL, and nothing else: equal keys mean equal (recipe slice,
/// box, ε, K, kind) bits, which by D9 means the derivation is
/// bit-identical, which is what makes serving it sound. It has no
/// notion of a key being close, newer, or good enough.
#[derive(Debug, Clone, Default)]
pub struct ReportCache {
    entries: BTreeMap<(u128, &'static str), String>,
}

impl ReportCache {
    /// An empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// The goldening form stored under `key` for `kind`, if any.
    pub fn get(&self, key: ContentKey, kind: &'static str) -> Option<&str> {
        self.entries.get(&(key.0, kind)).map(String::as_str)
    }

    /// Stores a report's goldening form under its key.
    ///
    /// Returns the PRIOR entry when one was there — which is how a
    /// caller learns that two different reports collided on one key, a
    /// thing that cannot happen while the key is taken over the
    /// serialized form and is worth being able to observe anyway.
    pub fn put(&mut self, key: ContentKey, kind: &'static str, report: String) -> Option<String> {
        self.entries.insert((key.0, kind), report)
    }

    /// How many reports are held.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the cache holds nothing.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// **The pure key function a consumer can call without the cache**: the
/// identity of the inputs a derived report is a function of.
///
/// `slice` is the recipe slice's own identity (the driver's verdict key
/// or a node content key — whatever the caller's report is derived
/// from), `box_` the analyzed box, `eps` the run's tolerance, `k` its
/// band constant, and `dials` the RUN DIALS the report was produced
/// under ([`Dials`]).
///
/// # Why the dials are in it (M10-6's review; deviation D11)
///
/// They were not, and the omission made the key wrong for its one
/// stated purpose. This module says a derived report is a function of
/// its inputs and that this key is their identity — but a drive at
/// `max_leaves = 4` and one at `4096` produce DIFFERENT reports over
/// the same `(slice, box, ε, K)` tuple, and an MC report's numbers move
/// with its seed and its sample count. Both reviews built the
/// collision. A cache keyed on the old tuple would have served one
/// budget's answer for another's, which is the exact failure a content
/// key exists to make impossible.
///
/// D9 is not strained by the addition and was not satisfied without
/// it: the recorded run dials are part of what a replay has to fix, and
/// a derived report is a replay's output.
pub fn report_key(
    kind: &'static str,
    slice: u128,
    box_: &crate::analysis::ParamBox,
    eps: f64,
    k: f64,
    dials: &Dials<'_>,
) -> ContentKey {
    let mut h = KeyHasher::new();
    h.write_tag(0xEC);
    h.write_str(kind);
    h.write_u64((slice >> 64) as u64);
    h.write_u64(slice as u64);
    for (name, axis) in box_.axes() {
        h.write_str(&name.0);
        let (lo, hi) = axis.span();
        h.write_f64_bits(lo);
        h.write_f64_bits(hi);
    }
    h.write_f64_bits(eps);
    h.write_f64_bits(k);
    h.write_str(&dials.serialize());
    h.finish()
}

/// **The run dials a derived report is a function of**, in the one
/// spelling [`report_key`] hashes.
///
/// A struct rather than loose arguments, so a dial ADDED to
/// [`crate::drive::DriveConfig`] or [`crate::mc::McConfig`] has one
/// place to be threaded through, and so a caller cannot quietly pass
/// defaults for a report it did not produce with them.
///
/// `mc` is `None` for a report the advisory lane had no part in — a
/// stackup, a fold, a histogram — and the absence is itself hashed, so
/// "no MC" and "MC at the default seed" are different keys rather than
/// the same one.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dials<'a> {
    /// The drive that produced the leaves under the report.
    pub drive: &'a crate::drive::DriveConfig,
    /// The advisory lane's dials, when the report has an advisory half.
    pub mc: Option<&'a crate::mc::McConfig>,
}

impl Dials<'_> {
    /// The dials' goldening form: exact, ordered, and total over every
    /// field that can move a report's numbers.
    pub fn serialize(&self) -> String {
        use core::fmt::Write as _;
        let d = self.drive;
        let mut s = format!(
            "drive max_depth={} max_leaves={} parallel={}",
            d.max_depth, d.max_leaves, d.parallel
        );
        // The probe replay changes what the k_stats funnel SEES, never
        // what the verdict is, so it is hashed only in the build where
        // it exists rather than making two builds disagree about the
        // key of a report they agree on.
        #[cfg(feature = "probe")]
        {
            let _ = write!(s, " k_probe={:?}", d.k_probe);
        }
        match self.mc {
            Some(m) => {
                let _ = write!(s, "; mc samples={} seed={:#018x}", m.samples, m.seed);
            }
            None => s.push_str("; mc none"),
        }
        s
    }
}

/// A mass as exact bits, or the unavailability's parameter — the
/// goldening form's one spelling for a column that may refuse.
pub(crate) fn mass_bits(m: &Result<f64, MeasureUnavailable>) -> String {
    match m {
        Ok(v) => format!("{:016x}", v.to_bits()),
        Err(MeasureUnavailable::BandHasNoMeasure { param }) => {
            format!("band:{}", param.0)
        }
    }
}

/// A mass as a percentage, or the reason there is none — the human
/// form's one spelling.
///
/// Four decimal places: the tail of a ±3σ box is 0.27%, and a report
/// that rounded it to 0.3% would be rounding away the number E2 exists
/// to keep.
pub(crate) fn percent(m: &Result<f64, MeasureUnavailable>) -> String {
    match m {
        Ok(v) => format!("{:.4}%", v * 100.0),
        Err(e) => format!("[no measure: {e}]"),
    }
}
