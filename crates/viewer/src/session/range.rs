//! **How far a field can move and stay provably the same build**, as
//! the session asks it: the field a `range?` run is taken for, the
//! reading the run leaves, and that reading's words.
//!
//! The search is the kernel's (`pncad::analysis::RangeSearch`), run on
//! its own seam ([`crate::evalseam::RangeService`]); this module only
//! says what it answered. Every number is the answer's, read through
//! [`props::shown_text`] in the field's own unit — a proven end rounded
//! toward the current value and a point proven different away from it
//! ([`props::shown_text_toward`]), so no text claims more than the proof.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::analysis::{
    BudgetKind, CertifiedRange, DifferenceKind, RangeField, RangeRefusal, RangeSide, RefusalReason,
    Unproven, Unsettled, range::STALL_LEVELS,
};
use pncad::document::{
    Doc, FreeVar, ProfileProgram, RecipeNodeId, Say, Sentence, SlotId, Speaker, SpokenVar, VarId,
    spoken_by,
};
use pncad::quantity::UnitDef;

use crate::evalseam::{RangeRun, RangeState};
use crate::props::{self, Notation};

/// The field a `range?` run is taken for.
///
/// Two arms rather than one with an `Option`, for the reason
/// `BeginGesture` and `BeginVariableGesture` are two doors: a slot and a
/// document variable are addressed differently.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RangeTarget {
    /// A node's named slot.
    Slot {
        /// The node.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
    },
    /// A document variable.
    Variable {
        /// The variable.
        var: VarId,
    },
}

impl RangeTarget {
    /// The kernel's name for this field.
    pub fn field(&self) -> RangeField {
        match self {
            Self::Slot { node, slot } => RangeField::Slot {
                node: *node,
                slot: *slot,
            },
            Self::Variable { var } => RangeField::Param(*var),
        }
    }

    /// **The unit the reading is written in**: the one the field is
    /// written in, or the working `notation`'s for a value nobody wrote
    /// ([`props::rendering_unit`]).
    pub(super) fn unit(&self, doc: &Doc<ProfileProgram>, notation: Notation) -> Option<UnitDef> {
        match self {
            Self::Slot { node, slot } => props::rendering_unit(
                slot.expr_dimension(),
                props::slot_unit(doc, *node, *slot),
                notation,
            ),
            Self::Variable { var } => match doc.var(*var)?.free()? {
                FreeVar::Continuous { display_unit, .. } => Some(display_unit.def()),
                FreeVar::Count { .. } => None,
            },
        }
    }

    /// **The NAMED variable a slot target reads**, whose range the run
    /// is: widening it moves every slot that reads it (`RangeField::Slot`).
    /// `None` for a variable target, and for a slot whose variable has
    /// no name, which only that slot reads.
    fn shared(&self, doc: &Doc<ProfileProgram>) -> Option<SpokenVar> {
        let Self::Slot { node, slot } = self else {
            return None;
        };
        let var = doc.spoken_var(doc.slot(*node, *slot)?);
        var.name().is_some().then_some(var)
    }
}

/// **One `range?` run, as the session holds it**: the field, the
/// document it was taken on, and where the run stands.
///
/// Shown only for the document it was taken on: every change of the
/// shown document drops it, with its run (`DocSession::request_eval`,
/// and a document replaced whole), and another `range?` replaces it.
#[derive(Clone, Debug)]
pub struct RangeReading {
    /// The field the run was taken for.
    pub target: RangeTarget,
    /// The run.
    pub run: RangeRun,
    /// The unit its numbers are written in, read off that document.
    pub unit: Option<UnitDef>,
    /// Where the run stands, with what it has proven.
    pub state: RangeState,
}

impl RangeReading {
    /// Whether the run is still proving: its line offers the stop.
    pub fn proving(&self) -> bool {
        self.state.proving()
    }

    /// The run's answer, where it has one.
    pub fn answer(&self) -> Option<&CertifiedRange> {
        self.state.answer()
    }

    /// **The reading's lines**, each a sentence on its own, with the
    /// document's nodes and variables said as `doc` speaks them.
    pub fn lines(&self, doc: &Doc<ProfileProgram>) -> Vec<String> {
        self.said_lines(doc)
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    /// [`Self::lines`] as sentences, each node and variable a mention
    /// in `doc`: what the Properties pane draws.
    pub fn said_lines(&self, doc: &Doc<ProfileProgram>) -> Vec<Sentence> {
        self.said(doc)
            .iter()
            .map(|said| {
                spoken_by(
                    &Line {
                        unit: self.unit,
                        said,
                    },
                    doc,
                )
            })
            .collect()
    }

    /// What the lines say, before their numbers are written.
    fn said(&self, doc: &Doc<ProfileProgram>) -> Vec<Said> {
        let shared = self.target.shared(doc).map(Said::Shared);
        let range = match &self.state {
            RangeState::Starting => {
                return [Said::Proving(None)].into_iter().chain(shared).collect();
            }
            RangeState::Proving(range) => {
                return [Said::Proving(Some(Proven::of(range)))]
                    .into_iter()
                    .chain(shared)
                    .collect();
            }
            RangeState::StoppedUnstarted => return vec![Said::StoppedUnstarted],
            RangeState::Refused(refusal) => return vec![Said::Refused(refusal.clone())],
            RangeState::Stopped(range) | RangeState::Finished(range) => range,
        };
        let (lo, hi) = range.proven_interval();
        let seed = range.seed();
        let mut said = vec![Said::Proven(Proven::of(range))];
        said.extend(shared);
        said.push(Said::Checked {
            lo: range.absolute(seed.lo),
            hi: range.absolute(seed.hi),
            nominal: range.nominal(),
        });
        for (side, end, which) in [(range.lo(), lo, Side::Below), (range.hi(), hi, Side::Above)] {
            said.extend(side_lines(range, side, end, which));
        }
        if !range.pinned().is_empty() {
            said.push(Said::Holding(
                range
                    .pinned()
                    .iter()
                    .map(|var| doc.spoken_var(*var))
                    .collect(),
            ));
        }
        said
    }
}

/// One side's lines: the undecided ground first — it lies between the
/// proof and the point proven different, and is never called valid —
/// then the point.
fn side_lines(range: &CertifiedRange, side: &RangeSide, end: f64, which: Side) -> Vec<Said> {
    let mut said = Vec::new();
    if let Some(gap) = side.not_decided() {
        said.push(Said::NotDecided {
            side: which,
            past: end,
            nominal: range.nominal(),
            reason: gap.reason().clone(),
            more_time_helps: gap.more_time_helps(),
        });
    }
    if let Some(different) = side.different() {
        let at = range.absolute(different.at());
        said.push(match different.kind() {
            DifferenceKind::BuildsDifferently => Said::BuildsDifferently(which, at),
            DifferenceKind::StandingChange { node } => Said::StopsBuilding(which, node, at),
        });
    }
    said
}

/// Which side of the current value a line is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Side {
    /// Below the current value.
    Below,
    /// Above it.
    Above,
}

impl Side {
    fn word(self) -> &'static str {
        match self {
            Self::Below => "below",
            Self::Above => "above",
        }
    }

    /// Where a point on this side rounds to so its text never lands
    /// inside the proof: away from the current value.
    fn outward(self) -> f64 {
        match self {
            Self::Below => f64::NEG_INFINITY,
            Self::Above => f64::INFINITY,
        }
    }
}

/// What a run has proven around the current value.
#[derive(Clone, Copy, Debug)]
enum Proven {
    /// Nothing, around this value.
    Nothing(f64),
    /// This interval around `nominal`, its ends rounded toward it.
    Interval { lo: f64, hi: f64, nominal: f64 },
}

impl Proven {
    fn of(range: &CertifiedRange) -> Self {
        if range.nothing_proven() {
            Self::Nothing(range.nominal())
        } else {
            let (lo, hi) = range.proven_interval();
            Self::Interval {
                lo,
                hi,
                nominal: range.nominal(),
            }
        }
    }
}

/// **One line of a reading**, its numbers absolute field values.
#[derive(Clone, Debug)]
enum Said {
    /// The run is still proving; what it has proven so far, once its
    /// witness is built.
    Proving(Option<Proven>),
    /// What the finished or stopped run proved.
    Proven(Proven),
    /// The named variable whose range this is, which every slot that
    /// reads it moves with.
    Shared(SpokenVar),
    /// How far the search looked: the seed's ends, rounded toward the
    /// current value.
    Checked { lo: f64, hi: f64, nominal: f64 },
    /// Ground past a proven end the search did not decide.
    NotDecided {
        side: Side,
        past: f64,
        nominal: f64,
        reason: Unproven,
        more_time_helps: bool,
    },
    /// A point at which the document builds on another branch.
    BuildsDifferently(Side, f64),
    /// A point at which a node that builds now does not.
    StopsBuilding(Side, RecipeNodeId, f64),
    /// The parameters held at their nominals.
    Holding(Vec<SpokenVar>),
    /// Stopped while the witness was built: nothing replayed.
    StoppedUnstarted,
    /// The request was refused, in the refusal's own words.
    Refused(RangeRefusal),
}

/// A line with the unit its numbers are written in.
struct Line<'a> {
    unit: Option<UnitDef>,
    said: &'a Said,
}

impl Line<'_> {
    fn number(&self, value: f64) -> String {
        props::shown_text(self.unit, value)
    }

    /// `value`, rounded toward `toward`.
    fn toward(&self, value: f64, toward: f64) -> String {
        props::shown_text_toward(self.unit, value, toward)
    }

    fn proven(&self, out: &mut Sentence, proven: Proven) -> core::fmt::Result {
        match proven {
            Proven::Nothing(at) => write!(out, "nothing proven around {}", self.number(at)),
            Proven::Interval { lo, hi, nominal } => write!(
                out,
                "proven {}…{}",
                self.toward(lo, nominal),
                self.toward(hi, nominal)
            ),
        }
    }
}

impl Say for Line<'_> {
    fn say(&self, out: &mut Sentence, by: Speaker<'_>) -> core::fmt::Result {
        match self.said {
            Said::Proving(None) => out.write_str("proving…"),
            Said::Proving(Some(proven)) => {
                out.write_str("proving… ")?;
                self.proven(out, *proven)?;
                out.write_str(" so far")
            }
            Said::Proven(proven) => self.proven(out, *proven),
            Said::Shared(var) => {
                out.write_str("the range of ")?;
                out.var(by.var(var))?;
                out.write_str(", which moves every slot that reads it")
            }
            Said::Checked { lo, hi, nominal } => write!(
                out,
                "checked to {}…{}",
                self.toward(*lo, *nominal),
                self.toward(*hi, *nominal)
            ),
            Said::NotDecided {
                side,
                past,
                nominal,
                reason,
                more_time_helps,
            } => {
                write!(
                    out,
                    "{}: not decided past {}: ",
                    side.word(),
                    self.toward(*past, *nominal)
                )?;
                out.say(
                    &Reason {
                        unit: self.unit,
                        reason,
                    },
                    by,
                )?;
                out.write_str(if *more_time_helps {
                    "; more proving may go further"
                } else {
                    "; more time will not help"
                })
            }
            Said::BuildsDifferently(side, at) => write!(
                out,
                "{}: at {} it builds, differently",
                side.word(),
                self.toward(*at, side.outward())
            ),
            Said::StopsBuilding(side, node, at) => {
                write!(out, "{}: ", side.word())?;
                out.node(by.node(*node))?;
                write!(
                    out,
                    " stops building at {}",
                    self.toward(*at, side.outward())
                )
            }
            Said::Holding(vars) => {
                out.write_str("holding ")?;
                for (i, var) in vars.iter().enumerate() {
                    if i > 0 {
                        out.write_str(", ")?;
                    }
                    out.var(by.var(var))?;
                }
                out.write_str(" at their current values")
            }
            Said::StoppedUnstarted => out.write_str("stopped before anything was proven"),
            Said::Refused(refusal) => out.say(refusal, by),
        }
    }
}

/// **Why ground is not decided**, in plain words: one sentence per arm
/// of [`Unproven`], and per arm of the driver's own floor. None of them
/// claims the ground is valid, or that it is not.
struct Reason<'a> {
    unit: Option<UnitDef>,
    reason: &'a Unproven,
}

impl Say for Reason<'_> {
    fn say(&self, out: &mut Sentence, by: Speaker<'_>) -> core::fmt::Result {
        match self.reason {
            Unproven::Unfinished { replays } => write!(out, "stopped after {replays} replays"),
            Unproven::Stalled { width } => write!(
                out,
                "{STALL_LEVELS} halvings past a width of {} decided nothing",
                props::shown_text(self.unit, *width)
            ),
            Unproven::FlipUnfounded => out.write_str(
                "the proof refused this ground without naming anything that changes there",
            ),
            Unproven::Floor(floor) => match floor {
                RefusalReason::SliverTerminal { .. } => {
                    out.write_str("a decision here sits within tolerance of its boundary")
                }
                // The search reads a definite flip as a point proven
                // different, never as a floor, so this arm says only
                // what every floor says.
                RefusalReason::FlipCrossing { .. } => {
                    out.write_str("the proof could not decide this ground")
                }
                RefusalReason::Bifurcation(_) => {
                    out.write_str("the solver's solution may branch here")
                }
                RefusalReason::Infeasible => {
                    out.write_str("the solver may have no solution over part of this ground")
                }
                RefusalReason::Budget(BudgetKind::Depth { max_depth }) => write!(
                    out,
                    "the search halved this side as finely as it may ({max_depth} halvings)"
                ),
                RefusalReason::Budget(BudgetKind::Leaves { max_leaves }) => {
                    write!(out, "the search spent its {max_leaves} leaves")
                }
                RefusalReason::Budget(BudgetKind::Resolution) => {
                    out.write_str("the ground is as narrow as the number grid allows")
                }
                RefusalReason::ReadsNoVariable { node, unsettled } => {
                    out.node(by.node(*node))?;
                    match unsettled {
                        Unsettled::Undecided => {
                            out.write_str(" does not settle, and does not read this field")
                        }
                        Unsettled::Failed { cause, .. } => {
                            write!(out, " fails ({cause}), and does not read this field")
                        }
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use pncad::analysis::{BudgetKind, RangeRefusal, RefusalReason, Unproven, Unsettled};
    use pncad::document::{
        Doc, DocEdit, FreeVar, NodeErrorClass, ProfileProgram, RecipeNodeId, Speaker, VarDecl,
        VarId, VarName, spoken_by,
    };
    use pncad::geom_core::Tol;
    use pncad::quantity::{M, MM};

    use super::{Line, Proven, Reason, Said, Side};
    use crate::props;

    /// A document declaring the lengths `a` and `b`, so a line can name
    /// them.
    fn named() -> (Doc<ProfileProgram>, VarId, VarId) {
        let tol = Tol::witness();
        let mut doc: Doc<ProfileProgram> = Doc::empty_derived("range-words", tol);
        for n in ["a", "b"] {
            doc = crate::test_support::edited(
                &doc,
                DocEdit::DeclareVar {
                    name: VarName::from_static(n),
                    def: VarDecl::Free(FreeVar::continuous(
                        pncad::document::Dimension::Length,
                        0.001,
                    )),
                },
                tol,
            )
            .0;
        }
        let a = doc.var_named("a").expect("declared");
        let b = doc.var_named("b").expect("declared");
        (doc, a, b)
    }

    /// **Every line of a reading, in its words**, each number written in
    /// the field's unit, each side named: one row per arm, the arm no
    /// document reaches on main today (a node that stops building)
    /// included.
    #[test]
    fn every_line_of_a_reading_is_said_in_its_words() {
        let (doc, a, b) = named();
        let node = RecipeNodeId::new(7, 0);
        let said = |said: Said| {
            spoken_by(
                &Line {
                    unit: Some(MM.def()),
                    said: &said,
                },
                &doc,
            )
            .to_string()
        };
        let interval = Proven::Interval {
            lo: 0.001,
            hi: 0.003,
            nominal: 0.002,
        };
        for (line, words) in [
            (Said::Proving(None), "proving…"),
            (
                Said::Proving(Some(interval)),
                "proving… proven 1 mm…3 mm so far",
            ),
            (
                Said::Proving(Some(Proven::Nothing(0.002))),
                "proving… nothing proven around 2 mm so far",
            ),
            (Said::Proven(interval), "proven 1 mm…3 mm"),
            (
                Said::Proven(Proven::Nothing(0.002)),
                "nothing proven around 2 mm",
            ),
            (
                Said::Shared(doc.spoken_var(a)),
                "the range of a, which moves every slot that reads it",
            ),
            (
                Said::Checked {
                    lo: 0.0,
                    hi: 0.004,
                    nominal: 0.002,
                },
                "checked to 0 mm…4 mm",
            ),
            (
                Said::NotDecided {
                    side: Side::Below,
                    past: 0.001,
                    nominal: 0.002,
                    reason: Unproven::Unfinished { replays: 5 },
                    more_time_helps: true,
                },
                "below: not decided past 1 mm: stopped after 5 replays; more proving may go \
                 further",
            ),
            (
                Said::NotDecided {
                    side: Side::Above,
                    past: 0.003,
                    nominal: 0.002,
                    reason: Unproven::FlipUnfounded,
                    more_time_helps: false,
                },
                "above: not decided past 3 mm: the proof refused this ground without naming \
                 anything that changes there; more time will not help",
            ),
            (
                Said::BuildsDifferently(Side::Above, 0.0035),
                "above: at 3.5 mm it builds, differently",
            ),
            (
                Said::Holding(vec![doc.spoken_var(a), doc.spoken_var(b)]),
                "holding a, b at their current values",
            ),
            (Said::StoppedUnstarted, "stopped before anything was proven"),
        ] {
            assert_eq!(said(line), words);
        }
        assert_eq!(
            said(Said::StopsBuilding(Side::Below, node, 0.0005)),
            format!(
                "below: {} stops building at 0.5 mm",
                editor_core::sentence!(Speaker::of(&doc).node(node))
            ),
            "the node is named as the document speaks it"
        );
        let refusal = RangeRefusal::SeedNotABracket { lo: 1.0, hi: 2.0 };
        assert_eq!(
            said(Said::Refused(refusal.clone())),
            refusal.to_string(),
            "a refusal is said in its own words"
        );
    }

    /// **A proven end is never shown past what was proven, and a point
    /// proven different never inside the proof**, at the text's own
    /// grid: an end the grid would round outward is shown one grid step
    /// in, and the difference one step out.
    #[test]
    fn the_shown_numbers_never_claim_more_than_the_proof() {
        let m = Some(M.def());
        let line = |said: Said| {
            let doc: Doc<ProfileProgram> = Doc::empty_derived("rounding", Tol::witness());
            spoken_by(
                &Line {
                    unit: m,
                    said: &said,
                },
                &doc,
            )
            .to_string()
        };
        let back = |text: &str| -> f64 {
            text.trim_end_matches(" m")
                .parse()
                .unwrap_or_else(|_| panic!("a number: {text:?}"))
        };
        for (end, nominal) in [(0.375_123_456_789_012_3, 0.25), (0.5_f64.next_down(), 0.0)] {
            let shown = props::shown_text_toward(m, end, nominal);
            assert!(
                back(&shown) <= end && back(&shown) > nominal,
                "{end} toward {nominal} shows {shown}"
            );
            assert!(
                back(&props::shown_text(m, end)) > end,
                "the nearest text is outward, so the row is not vacuous: {}",
                props::shown_text(m, end)
            );
            let proven = line(Said::Proven(Proven::Interval {
                lo: nominal,
                hi: end,
                nominal,
            }));
            assert!(proven.ends_with(&format!("…{shown}")), "{proven}");
            let different = line(Said::BuildsDifferently(Side::Above, end));
            let at = different
                .trim_start_matches("above: at ")
                .trim_end_matches(" it builds, differently");
            assert!(
                back(at) >= end,
                "the difference is shown outward: {different}"
            );
        }
        let below = props::shown_text_toward(m, -0.375_123_456_789_012_3, -0.25);
        assert!(back(&below) >= -0.375_123_456_789_012_3, "{below}");
    }

    /// **Every reason ground is undecided has words of its own**, and
    /// none of them claims the ground is valid or different: one row
    /// per arm of `Unproven` and of the driver's floor.
    #[test]
    fn every_unproven_arm_is_said_in_plain_words() {
        let doc: Doc<ProfileProgram> = Doc::empty_derived("reasons", Tol::witness());
        let node = RecipeNodeId::new(7, 0);
        let said = |reason: Unproven| {
            spoken_by(
                &Reason {
                    unit: None,
                    reason: &reason,
                },
                &doc,
            )
            .to_string()
        };
        let flip = editor_core::drive::FlipEvidence {
            verdicts: editor_core::FlipSet::default(),
            structure: Vec::new(),
        };
        let fork = editor_core::WitnessBifurcation {
            kind: editor_core::BifurcationKind::FoldProximity,
            margin: editor_core::BranchMarginEvidence {
                margin: 0.0,
                band_zero: 0.0,
                band_escalate: 0.0,
            },
            implicated: Vec::new(),
            witness_age: editor_core::WitnessAge {
                solved_under: Vec::new(),
                at_solve: Vec::new(),
            },
        };
        for (reason, words) in [
            (
                Unproven::Unfinished { replays: 12 },
                "stopped after 12 replays",
            ),
            (
                Unproven::Stalled { width: 0.5 },
                "4 halvings past a width of 0.5 decided nothing",
            ),
            (
                Unproven::FlipUnfounded,
                "the proof refused this ground without naming anything that changes there",
            ),
            (
                Unproven::Floor(RefusalReason::SliverTerminal {
                    predicate: "segment_gap",
                }),
                "a decision here sits within tolerance of its boundary",
            ),
            (
                Unproven::Floor(RefusalReason::FlipCrossing {
                    flipped: Box::new(flip),
                }),
                "the proof could not decide this ground",
            ),
            (
                Unproven::Floor(RefusalReason::Bifurcation(Box::new(fork))),
                "the solver's solution may branch here",
            ),
            (
                Unproven::Floor(RefusalReason::Infeasible),
                "the solver may have no solution over part of this ground",
            ),
            (
                Unproven::Floor(RefusalReason::Budget(BudgetKind::Depth { max_depth: 32 })),
                "the search halved this side as finely as it may (32 halvings)",
            ),
            (
                Unproven::Floor(RefusalReason::Budget(BudgetKind::Leaves { max_leaves: 9 })),
                "the search spent its 9 leaves",
            ),
            (
                Unproven::Floor(RefusalReason::Budget(BudgetKind::Resolution)),
                "the ground is as narrow as the number grid allows",
            ),
        ] {
            assert_eq!(said(reason), words);
        }
        for (unsettled, tail) in [
            (
                Unsettled::Undecided,
                "does not settle, and does not read this field",
            ),
            (
                Unsettled::Failed {
                    class: NodeErrorClass::Extrude,
                    cause: "a sliver".to_owned(),
                },
                "fails (a sliver), and does not read this field",
            ),
        ] {
            assert_eq!(
                said(Unproven::Floor(RefusalReason::ReadsNoVariable {
                    node,
                    unsettled
                })),
                format!(
                    "{} {tail}",
                    editor_core::sentence!(Speaker::of(&doc).node(node))
                ),
                "the node is named as the document speaks it"
            );
        }
    }
}
