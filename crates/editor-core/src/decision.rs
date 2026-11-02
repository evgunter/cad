//! **What a predicate decides, in words — the one lookup — and how its
//! undecided refusal ends.**
//!
//! A predicate's static name is routing: it rides `Debug` and the K
//! stream, never a sentence. A sentence that reports a decision — an
//! escalation that could not call it, a flip between two runs — states
//! what was being decided instead, and reads the words here.
//!
//! The words live with each predicate's owner, beside the decision
//! they describe: the naming layer's (`names::DECISION_WORDS`), this
//! crate's evaluation's (`eval::DECISION_WORDS`), the mate solve's
//! (`mate::DECISION_WORDS`), the boolean's
//! (`topo::decision_words`) and the sketch validator's
//! (`profile::decision_subject`). This function only consults them, so
//! a predicate is given words in one place, where it is decided.

use geom_core::{COINCIDENCE_RECOURSE, EndingKind, Indeterminate};

/// The words for `predicate`, from whichever owner holds them; `None`
/// when no owner has words for it (the caller then states
/// `geom_core::UNNAMED_DECISION`). A decision the side reader's rays
/// took is recorded under the reader's family ([`topo::SIDE_READER_FAMILY`]);
/// where the family's name has no words of its own, it takes the words
/// of the decision under it.
pub(crate) fn words(predicate: &str) -> Option<&'static str> {
    let own = |predicate: &str| {
        lookup(crate::names::DECISION_WORDS, predicate)
            .or_else(|| lookup(crate::eval::DECISION_WORDS, predicate))
            .or_else(|| lookup(crate::mate::DECISION_WORDS, predicate))
            .or_else(|| topo::decision_words(predicate))
            .or_else(|| profile::decision_subject(predicate))
    };
    own(predicate).or_else(|| {
        predicate
            .strip_prefix(topo::SIDE_READER_FAMILY)
            .and_then(own)
    })
}

/// The second column of `table`'s row for `predicate`.
fn lookup<V: Copy>(table: &[(&str, V)], predicate: &str) -> Option<V> {
    table
        .iter()
        .find(|(name, _)| *name == predicate)
        .map(|(_, v)| *v)
}

/// A size the user may have chosen: the levers, with the tolerance arm
/// a value gives.
const SIZED: EndingKind = EndingKind::Sized;
/// No size the user chose (an angle, a direction's gauge, a curve
/// parameter): the coincidence lever alone.
const LEVER: EndingKind = EndingKind::Lever(COINCIDENCE_RECOURSE);

/// Declares [`Decision`] from one table, so each decision's name and
/// kind are written once, side by side, and [`Decision::ALL`] holds
/// every variant by construction.
macro_rules! decisions {
    ($($(#[$doc:meta])* $variant:ident => $name:expr, $ending:ident;)*) => {
        /// **Every decision whose escalation this crate renders whole**
        /// (`NodeErrorKind::Escalated`, `NamingError::Escalated`,
        /// `SelectRefusal::InBand`), with its funnel name and how its
        /// refusal ends. The evaluation's escalations are built from a
        /// `Decision`, so a decision fed to that door has a row.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub(crate) enum Decision {
            $($(#[$doc])* $variant,)*
        }

        impl Decision {
            /// Every decision, once.
            pub(crate) const ALL: &[Self] = &[$(Self::$variant),*];

            /// The funnel name the decision is made under.
            pub(crate) const fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $name,)*
                }
            }

            /// How its undecided refusal ends.
            pub(crate) const fn ending(self) -> EndingKind {
                match self {
                    $(Self::$variant => $ending,)*
                }
            }
        }
    };
}

decisions! {
    /// A direction vector's length, decided at the evaluation's door: a gauge.
    DirectionNorm => crate::eval::EVAL_DIRECTION_NORM, LEVER;
    /// Whether a revolve makes a full turn: an angle.
    RevolveFullTurn => "revolve_full_vs_partial", LEVER;
    /// A point's place along a member edge: a distance in the result.
    OnMemberEdge => crate::names::ON_MEMBER_EDGE, SIZED;
    /// Whether a chord lies on its rim: a distance in the result.
    ChordOnRim => crate::names::CHORD_ON_RIM, SIZED;
    /// A distance from a plane against a stated value: a length the user
    /// wrote.
    SelPlaneDistance => crate::pose::SEL_PLANE_DISTANCE, SIZED;
}

impl Decision {
    /// The decision made under `predicate`, if this crate renders it.
    pub(crate) fn named(predicate: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|d| d.name() == predicate)
    }
}

/// The one ending of an escalation under `predicate`, read at the build
/// its door reports: its [`Decision`]'s kind. A name with no row ends as
/// a sized one and says no recourse is recorded for it
/// ([`geom_core::MissingRecourse`]).
pub(crate) fn ending(predicate: &str, source: &Indeterminate) -> String {
    source.ending_by(
        Decision::named(predicate).map(Decision::ending),
        Some(predicate),
    )
}

#[cfg(test)]
mod census {
    use std::collections::BTreeSet;

    use super::{Decision, words};

    /// **Every decision has one row and words to state**, and every
    /// decision the naming layer and the evaluation give words to has a
    /// row: the naming door is fed by name, so a worded naming decision
    /// with no row would end in the missing-recourse note.
    #[test]
    fn every_rendered_decision_has_a_row_and_words() {
        let rows: BTreeSet<&str> = Decision::ALL.iter().map(|d| d.name()).collect();
        assert_eq!(rows.len(), Decision::ALL.len(), "a name has two rows");
        for (name, _) in crate::names::DECISION_WORDS
            .iter()
            .chain(crate::eval::DECISION_WORDS)
        {
            assert!(rows.contains(name), "`{name}` is worded and has no row");
        }
        for name in rows {
            assert!(words(name).is_some(), "`{name}` has no words to state");
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod undecided_text {
    use geom_core::{Band, Indeterminate, MarginDiag};

    use crate::{EntityKind, NamingError, NodeErrorKind, RecipeNodeId, SelectRefusal, StableName};

    /// The escalation under `predicate` at `margin`, against the band
    /// `(1e-9, 1e-8)`.
    fn at(predicate: &'static str, margin: MarginDiag) -> Indeterminate {
        Indeterminate {
            margin,
            band: Band::new(1e-9, 1e-8).expect("zero < escalate"),
            predicate: Some(predicate),
            terminal_sliver: false,
        }
    }

    /// A value, a straddle and an unreadable margin, by name.
    fn margins() -> [(&'static str, MarginDiag); 3] {
        [
            ("value", MarginDiag::value(5e-9)),
            ("straddle", MarginDiag::enclosure(-2e-9, 5e-9)),
            ("unreadable", MarginDiag::INVALID),
        ]
    }

    fn node(predicate: &'static str, margin: MarginDiag) -> String {
        NodeErrorKind::Escalated {
            predicate,
            source: at(predicate, margin),
        }
        .to_string()
    }

    /// Each door's text at the three margins against what it should
    /// read.
    fn check(door: &str, got: impl Fn(MarginDiag) -> String, want: [&str; 3]) {
        for ((what, margin), want) in margins().into_iter().zip(want) {
            assert_eq!(got(margin), want, "{door}, {what}");
        }
    }

    /// **An evaluation's undecided decision ends as its kind warrants**:
    /// a point's place along an edge is a size in the result, and
    /// offers the tolerance on a value, subdivision on a straddle and
    /// the inputs on an unreadable margin; a revolve's full turn is an
    /// angle, and ends in the lever alone, noted where the margin could
    /// not be read.
    #[test]
    fn an_evaluations_undecided_decision_ends_by_its_kind() {
        check(
            "a sized decision",
            |m| node(crate::names::ON_MEMBER_EDGE, m),
            [
                "a point's place along an edge is undecided: margin 5e-9 lies inside \
                 the ambiguity band (1e-9, 1e-8). Recourse: move the geometry, or, if this size \
                 is intended, tighten the tolerance below 5e-10 m",
                "a point's place along an edge is undecided: enclosure [-2e-9, 5e-9] \
                 cannot be classified against the ambiguity band (1e-9, 1e-8). Recourse: \
                 subdivide the parameter box for a tighter enclosure, or move the geometry",
                "a point's place along an edge is undecided: margin is invalid (NaN or \
                 a refused enclosure) against the ambiguity band (1e-9, 1e-8). Recourse: check \
                 the operation's inputs upstream, then move the geometry; an unreadable margin \
                 may indicate a kernel bug worth reporting",
            ],
        );
        check(
            "a lever-only decision",
            |m| node("revolve_full_vs_partial", m),
            [
                "whether the revolve makes a full turn is undecided: margin 5e-9 lies inside the \
                 ambiguity band (1e-9, 1e-8). Recourse: move the geometry",
                "whether the revolve makes a full turn is undecided: enclosure [-2e-9, 5e-9] \
                 cannot be classified against the ambiguity band (1e-9, 1e-8). Recourse: move \
                 the geometry",
                "whether the revolve makes a full turn is undecided: margin is invalid (NaN or a \
                 refused enclosure) against the ambiguity band (1e-9, 1e-8). Recourse: move the \
                 geometry; an unreadable margin may indicate a kernel bug worth reporting",
            ],
        );
    }

    /// **A name this crate does not decide** — an evaluation refusal
    /// built outside its doors — says it has no words for it, and ends
    /// as the bare escalation does.
    #[test]
    fn an_undecided_name_this_crate_does_not_decide_ends_sized() {
        assert_eq!(
            node("roster_unknown_probe", MarginDiag::value(5e-9)),
            "an unnamed decision is undecided: margin 5e-9 lies inside the ambiguity band \
             (1e-9, 1e-8). Recourse: move the geometry, or, if this size is intended, tighten \
             the tolerance below 5e-10 m — no recourse is recorded for this decision"
        );
    }

    /// **The evaluation's direction length is a gauge**, and ends in
    /// the lever alone. A pose's direction length, the
    /// same arithmetic under `pose_unit_norm`, keep their held sentence
    /// (`NodeErrorKind::PoseDirectionEscalated`), so the two end two ways
    /// while D10's pose ground is held.
    #[test]
    fn the_evaluations_undecided_direction_length_ends_in_the_lever() {
        assert_eq!(
            node(crate::eval::EVAL_DIRECTION_NORM, MarginDiag::value(5e-9)),
            "whether a direction has any length is undecided: margin 5e-9 lies inside the \
             ambiguity band (1e-9, 1e-8). Recourse: move the geometry"
        );
    }

    /// **A placement frame's rotation axis keeps its sentence** (D10's
    /// placement ground is held): the frame door's refusal, read whole
    /// through `EditError::PlacementAxis`, unchanged.
    #[test]
    fn a_placement_axis_escalation_keeps_its_sentence() {
        let band = Band::new(1e-9, 1e-8).expect("zero < escalate");
        let refusal = crate::Frame::rotate_then_translate([5e-9, 0.0, 0.0], 1.0, [0.0; 3], band)
            .expect_err("an axis inside the band escalates");
        assert!(
            matches!(refusal.kind(), NodeErrorKind::PlacementAxisEscalated { .. }),
            "{:?}",
            refusal.kind()
        );
        let got = crate::EditError::PlacementAxis {
            error: refusal.carried(),
        }
        .to_string();
        assert_eq!(
            got,
            "the placement frame's rotation axis is unusable: whether a direction has any length \
             is too close to call: margin 5e-9 lies inside the ambiguity band (1e-9, 1e-8) — a \
             near-coincidence; Recourse: move the geometry, or, if this size is intended, tighten \
             the tolerance below 5e-10 m. Recourse: give the rotation axis a direction of nonzero \
             length"
        );
    }

    /// **Naming's sized decisions say what the name rests on**: a
    /// point's place along an edge is a distance in the result the
    /// user's geometry put there.
    #[test]
    fn namings_undecided_decision_ends_sized() {
        check(
            "naming",
            |m| {
                NamingError::Escalated {
                    predicate: crate::names::ON_MEMBER_EDGE,
                    source: at(crate::names::ON_MEMBER_EDGE, m),
                }
                .to_string()
            },
            [
                "a point's place along an edge, which a name rests on, is undecided: margin \
                 5e-9 lies inside the ambiguity band (1e-9, 1e-8). Recourse: move the geometry, \
                 or, if this size is intended, tighten the tolerance below 5e-10 m",
                "a point's place along an edge, which a name rests on, is undecided: enclosure \
                 [-2e-9, 5e-9] cannot be classified against the ambiguity band (1e-9, 1e-8). \
                 Recourse: subdivide the parameter box for a tighter enclosure, or move the \
                 geometry",
                "a point's place along an edge, which a name rests on, is undecided: margin is \
                 invalid (NaN or a refused enclosure) against the ambiguity band (1e-9, 1e-8). \
                 Recourse: check the operation's inputs upstream, then move the geometry; an \
                 unreadable margin may indicate a kernel bug worth reporting",
            ],
        );
    }

    /// **The selection's undecided distance asks one question and ends
    /// sized**: the distance from the plane against the stated value is
    /// a length the user wrote.
    #[test]
    fn a_selections_undecided_distance_ends_sized() {
        check(
            "select",
            |m| {
                SelectRefusal::InBand {
                    name: Box::new(StableName {
                        kind: EntityKind::Face,
                        node: RecipeNodeId::new(0, 7),
                        path: vec![],
                    }),
                    predicate: crate::SEL_PLANE_DISTANCE,
                    source: at(crate::SEL_PLANE_DISTANCE, m),
                }
                .to_string()
            },
            [
                "whether the face (the face of node 000000000000) is in or out of the selection \
                 is undecided: margin 5e-9 lies inside the ambiguity band (1e-9, 1e-8). Recourse: \
                 move the geometry, or, if this size is intended, tighten the tolerance below \
                 5e-10 m",
                "whether the face (the face of node 000000000000) is in or out of the selection \
                 is undecided: enclosure [-2e-9, 5e-9] cannot be classified against the ambiguity \
                 band (1e-9, 1e-8). Recourse: subdivide the parameter box for a tighter \
                 enclosure, or move the geometry",
                "whether the face (the face of node 000000000000) is in or out of the selection \
                 is undecided: margin is invalid (NaN or a refused enclosure) against the ambiguity \
                 band (1e-9, 1e-8). Recourse: check the operation's inputs upstream, then move \
                 the geometry; an unreadable margin may indicate a kernel bug worth reporting",
            ],
        );
    }
}
