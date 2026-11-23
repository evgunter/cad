//! **How far a field can move and stay provably the same build**, as
//! the session asks it: the field a `range?` run is taken for, the
//! reading the run leaves, and that reading's words.
//!
//! The search is the kernel's (`pncad::analysis::RangeSearch`), run on
//! its own seam ([`crate::evalseam::RangeService`]); this module only
//! says what it answered. Every number is the answer's, read through
//! [`props::shown_text`] in the field's own unit.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::analysis::{
    BudgetKind, CertifiedRange, DifferenceKind, RangeField, RangeRefusal, RefusalReason, Unproven,
    Unsettled, range::STALL_LEVELS,
};
use pncad::document::{
    Doc, FreeVar, ProfileProgram, RecipeNodeId, Say, SlotId, Speaker, SpokenVar, VarId, spoken_by,
};
use pncad::quantity::UnitDef;

use crate::evalseam::{RangeProgress, RangeRun};
use crate::generation::Generation;
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
}

/// **One `range?` run, as the session holds it**: the field, the
/// document it was taken on, and its answer so far.
///
/// Shown only for the document it was taken on: an edit, a value
/// gesture or another `range?` replaces the session's document or
/// run, and the reading goes with it.
#[derive(Clone, Debug)]
pub struct RangeReading {
    /// The field the run was taken for.
    pub target: RangeTarget,
    /// The run.
    pub run: RangeRun,
    /// The generation of the document it was taken on.
    pub generation: Generation,
    /// The unit its numbers are written in, read off that document.
    pub unit: Option<UnitDef>,
    /// The newest answer, `None` until the witness is built; or why the
    /// request was refused.
    pub answer: Option<Result<CertifiedRange, RangeRefusal>>,
    /// Whether more answers follow.
    pub progress: RangeProgress,
}

impl RangeReading {
    /// Whether the run is still proving: its line offers the stop.
    pub fn proving(&self) -> bool {
        self.progress == RangeProgress::Proving
    }

    /// **The reading's lines**, each a sentence on its own, with the
    /// document's nodes and variables said as `doc` speaks them.
    pub fn lines(&self, doc: &Doc<ProfileProgram>) -> Vec<String> {
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
        let range = match &self.answer {
            None => return vec![Said::Proving(None)],
            Some(Err(refusal)) => return vec![Said::Refused(refusal.clone())],
            Some(Ok(range)) => range,
        };
        let (lo, hi) = range.proven_interval();
        let proven = if range.nothing_proven() {
            Proven::Nothing(range.nominal())
        } else {
            Proven::Interval(lo, hi)
        };
        if self.proving() {
            return vec![Said::Proving(Some(proven))];
        }
        let seed = range.seed();
        let mut said = vec![
            Said::Proven(proven),
            Said::Checked(range.absolute(seed.lo), range.absolute(seed.hi)),
        ];
        for (side, end) in [(range.lo(), lo), (range.hi(), hi)] {
            // Undecided ground first: it lies between the proof and the
            // point proven different, and is never called valid.
            if let Some(gap) = side.not_decided() {
                said.push(Said::NotDecided {
                    past: end,
                    reason: gap.reason().clone(),
                    more_time_helps: gap.more_time_helps(),
                });
            }
            if let Some(different) = side.different() {
                let at = range.absolute(different.at());
                said.push(match different.kind() {
                    DifferenceKind::BuildsDifferently => Said::BuildsDifferently(at),
                    DifferenceKind::StandingChange { node } => Said::StopsBuilding(node, at),
                });
            }
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

/// What a run has proven around the current value.
#[derive(Clone, Copy, Debug)]
enum Proven {
    /// Nothing, around this value.
    Nothing(f64),
    /// This interval, its ends rounded toward the current value.
    Interval(f64, f64),
}

/// **One line of a reading**, its numbers absolute field values.
#[derive(Clone, Debug)]
enum Said {
    /// The run is still proving; what it has proven so far, once its
    /// witness is built.
    Proving(Option<Proven>),
    /// What the finished or stopped run proved.
    Proven(Proven),
    /// How far the search looked: the seed's ends.
    Checked(f64, f64),
    /// Ground past a proven end the search did not decide.
    NotDecided {
        past: f64,
        reason: Unproven,
        more_time_helps: bool,
    },
    /// A point at which the document builds on another branch.
    BuildsDifferently(f64),
    /// A point at which a node that builds now does not.
    StopsBuilding(RecipeNodeId, f64),
    /// The parameters held at their nominals.
    Holding(Vec<SpokenVar>),
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

    fn proven(&self, proven: Proven) -> String {
        match proven {
            Proven::Nothing(at) => format!("nothing proven around {}", self.number(at)),
            Proven::Interval(lo, hi) => {
                format!("proven {}…{}", self.number(lo), self.number(hi))
            }
        }
    }
}

impl Say for Line<'_> {
    fn say(&self, f: &mut core::fmt::Formatter<'_>, by: Speaker<'_>) -> core::fmt::Result {
        match self.said {
            Said::Proving(None) => f.write_str("proving…"),
            Said::Proving(Some(proven)) => write!(f, "proving… {} so far", self.proven(*proven)),
            Said::Proven(proven) => f.write_str(&self.proven(*proven)),
            Said::Checked(lo, hi) => {
                write!(f, "checked to {}…{}", self.number(*lo), self.number(*hi))
            }
            Said::NotDecided {
                past,
                reason,
                more_time_helps,
            } => write!(
                f,
                "not decided past {}: {}; {}",
                self.number(*past),
                Reason {
                    unit: self.unit,
                    reason,
                    by,
                },
                if *more_time_helps {
                    "more proving may go further"
                } else {
                    "more time will not help"
                }
            ),
            Said::BuildsDifferently(at) => {
                write!(f, "at {} it builds, differently", self.number(*at))
            }
            Said::StopsBuilding(node, at) => {
                write!(
                    f,
                    "{} stops building at {}",
                    by.node(*node),
                    self.number(*at)
                )
            }
            Said::Holding(vars) => {
                let names: Vec<String> = vars.iter().map(ToString::to_string).collect();
                write!(f, "holding {} at their current values", names.join(", "))
            }
            Said::Refused(refusal) => write!(f, "{refusal}"),
        }
    }
}

/// **Why ground is not decided**, in plain words: one sentence per arm
/// of [`Unproven`], and per arm of the driver's own floor.
struct Reason<'a> {
    unit: Option<UnitDef>,
    reason: &'a Unproven,
    by: Speaker<'a>,
}

impl core::fmt::Display for Reason<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.reason {
            Unproven::Unfinished { replays } => write!(f, "stopped after {replays} replays"),
            Unproven::Stalled { width } => write!(
                f,
                "{STALL_LEVELS} halvings past a width of {} decided nothing",
                props::shown_text(self.unit, *width)
            ),
            Unproven::FlipUnfounded => f.write_str(
                "the proof refused this ground without naming anything that changes there",
            ),
            Unproven::Floor(floor) => match floor {
                RefusalReason::SliverTerminal { predicate } => write!(
                    f,
                    "a decision ({predicate}) sits within tolerance of its boundary"
                ),
                RefusalReason::FlipCrossing { .. } => f.write_str("a decision changes here"),
                RefusalReason::Bifurcation(_) => f.write_str("the solver's solution branches here"),
                RefusalReason::Infeasible => {
                    f.write_str("the solver has no solution over part of this ground")
                }
                RefusalReason::Budget(BudgetKind::Depth { max_depth }) => {
                    write!(f, "the search halved this side its {max_depth} times")
                }
                RefusalReason::Budget(BudgetKind::Leaves { max_leaves }) => {
                    write!(f, "the search spent its {max_leaves} leaves")
                }
                RefusalReason::Budget(BudgetKind::Resolution) => {
                    f.write_str("the ground is as narrow as the number grid allows")
                }
                RefusalReason::ReadsNoVariable { node, unsettled } => match unsettled {
                    Unsettled::Undecided => write!(
                        f,
                        "{} does not settle, and does not read this field",
                        self.by.node(*node)
                    ),
                    Unsettled::Failed { cause, .. } => write!(
                        f,
                        "{} fails ({cause}), and does not read this field",
                        self.by.node(*node)
                    ),
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use pncad::analysis::{BudgetKind, RangeRefusal, RefusalReason, Unproven, Unsettled};
    use pncad::document::{
        Doc, NodeErrorClass, ProfileProgram, RecipeNodeId, Speaker, VarId, spoken_by,
    };
    use pncad::geom_core::Tol;
    use pncad::quantity::MM;

    use super::{Line, Proven, Reason, Said};

    /// **Every line of a reading, in its words**, each number written in
    /// the field's unit: one row per arm, the arm no document reaches on
    /// main today (a node that stops building) included.
    #[test]
    fn every_line_of_a_reading_is_said_in_its_words() {
        let doc: Doc<ProfileProgram> = Doc::empty_derived("lines", Tol::witness());
        let node = RecipeNodeId::new(7, 0);
        let said = |said: Said| {
            spoken_by(
                &Line {
                    unit: Some(MM.def()),
                    said: &said,
                },
                &doc,
            )
        };
        for (line, words) in [
            (Said::Proving(None), "proving…"),
            (
                Said::Proving(Some(Proven::Interval(0.001, 0.003))),
                "proving… proven 1 mm…3 mm so far",
            ),
            (
                Said::Proving(Some(Proven::Nothing(0.002))),
                "proving… nothing proven around 2 mm so far",
            ),
            (
                Said::Proven(Proven::Interval(0.001, 0.003)),
                "proven 1 mm…3 mm",
            ),
            (
                Said::Proven(Proven::Nothing(0.002)),
                "nothing proven around 2 mm",
            ),
            (Said::Checked(0.0, 0.004), "checked to 0 mm…4 mm"),
            (
                Said::NotDecided {
                    past: 0.003,
                    reason: Unproven::Unfinished { replays: 5 },
                    more_time_helps: true,
                },
                "not decided past 3 mm: stopped after 5 replays; more proving may go further",
            ),
            (
                Said::NotDecided {
                    past: 0.003,
                    reason: Unproven::FlipUnfounded,
                    more_time_helps: false,
                },
                "not decided past 3 mm: the proof refused this ground without naming anything \
                 that changes there; more time will not help",
            ),
            (
                Said::BuildsDifferently(0.0035),
                "at 3.5 mm it builds, differently",
            ),
            (
                Said::Holding(vec![
                    doc.spoken_var(VarId::new(1, 0)),
                    doc.spoken_var(VarId::new(2, 0)),
                ]),
                "",
            ),
        ] {
            let got = said(line);
            if words.is_empty() {
                assert!(
                    got.starts_with("holding ") && got.ends_with(" at their current values"),
                    "{got}"
                );
                assert_eq!(got.matches(", ").count(), 1, "two names: {got}");
            } else {
                assert_eq!(got, words);
            }
        }
        let stops = said(Said::StopsBuilding(node, 0.0035));
        assert_eq!(
            stops,
            format!("{} stops building at 3.5 mm", Speaker::of(&doc).node(node)),
            "the node is named as the document speaks it"
        );
        let refusal = RangeRefusal::SeedNotABracket { lo: 1.0, hi: 2.0 };
        assert_eq!(
            said(Said::Refused(refusal.clone())),
            refusal.to_string(),
            "a refusal is said in its own words"
        );
    }

    /// **Every reason ground is undecided has words of its own**: one
    /// row per arm, so a new arm or a reworded one is read here.
    #[test]
    fn every_unproven_arm_is_said_in_plain_words() {
        let doc: Doc<ProfileProgram> = Doc::empty_derived("reasons", Tol::witness());
        let node = RecipeNodeId::new(7, 0);
        let said = |reason: Unproven| {
            Reason {
                unit: None,
                reason: &reason,
                by: Speaker::of(&doc),
            }
            .to_string()
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
                "a decision (segment_gap) sits within tolerance of its boundary",
            ),
            (
                Unproven::Floor(RefusalReason::Budget(BudgetKind::Depth { max_depth: 32 })),
                "the search halved this side its 32 times",
            ),
            (
                Unproven::Floor(RefusalReason::Budget(BudgetKind::Leaves { max_leaves: 9 })),
                "the search spent its 9 leaves",
            ),
            (
                Unproven::Floor(RefusalReason::Budget(BudgetKind::Resolution)),
                "the ground is as narrow as the number grid allows",
            ),
            (
                Unproven::Floor(RefusalReason::Infeasible),
                "the solver has no solution over part of this ground",
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
            let words = said(Unproven::Floor(RefusalReason::ReadsNoVariable {
                node,
                unsettled,
            }));
            assert_eq!(
                words,
                format!("{} {tail}", Speaker::of(&doc).node(node)),
                "the node is named as the document speaks it"
            );
        }
    }
}
