//! **What a predicate decides, in words — the one lookup — and how its
//! undecided refusal ends.**
//!
//! A predicate's static name is routing: it rides `Debug` and the K
//! stream, never a sentence. A sentence that reports a decision — an
//! escalation that could not call it, a flip between two runs — states
//! what was being decided instead, and reads the words here.
//!
//! The words live with each predicate's owner, beside the decision
//! they describe: the naming layer's (`names::decision_words`), this
//! crate's evaluation's (`eval::decision_words`), the boolean's
//! (`topo::decision_words`) and the sketch validator's
//! (`profile::decision_subject`). This function only consults them, so
//! a predicate is given words in one place, where it is decided.

use geom_brep::recourse::{LeverOnly, Reading, RefusedArm};
use geom_core::{COINCIDENCE_RECOURSE, Indeterminate};

/// The words for `predicate`, from whichever owner holds them; `None`
/// when no owner has words for it (the caller then states
/// `geom_core::UNNAMED_DECISION`).
pub(crate) fn words(predicate: &str) -> Option<&'static str> {
    crate::names::decision_words(predicate)
        .or_else(|| crate::eval::decision_words(predicate))
        .or_else(|| topo::decision_words(predicate))
        .or_else(|| profile::decision_subject(predicate))
}

/// The second column of `table`'s row for `predicate`.
pub(crate) fn lookup<V: Copy>(table: &[(&str, V)], predicate: &str) -> Option<V> {
    table
        .iter()
        .find(|(name, _)| *name == predicate)
        .map(|(_, v)| *v)
}

/// **How an undecided decision's refusal ends** (D4 ¶1 (i)), by what its
/// margin measures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Ending {
    /// A size the user may have chosen: the levers, with the tolerance
    /// arm a value gives ([`Indeterminate::ending`]).
    Sized,
    /// No size the user chose — an angle, a sine, a direction's gauge:
    /// the lever alone, noted where the margin could not be read
    /// ([`LeverOnly`]).
    Lever,
}

/// **Every decision whose escalation this crate renders whole**
/// (`NodeErrorKind::Escalated`, `NamingError::Escalated`), with how it
/// ends. The census below holds it equal to the decisions those doors
/// are fed.
pub(crate) const ENDINGS: &[(&str, Ending)] = &[
    (crate::eval::EVAL_DIRECTION_NORM, Ending::Lever),
    ("revolve_full_vs_partial", Ending::Lever),
    (crate::eval::PATTERN_SPACING, Ending::Sized),
    (crate::eval::PATTERN_STEP, Ending::Lever),
    (crate::eval::PATTERN_STEP_TURN, Ending::Lever),
    ("bool_plane_parallel", Ending::Lever),
    ("carrier_cyl_axis_parallel", Ending::Lever),
    (crate::names::ORDER_ALONG, Ending::Sized),
    (crate::names::ON_MEMBER_EDGE, Ending::Sized),
    (crate::names::ON_SEAM_LINE, Ending::Sized),
    (crate::names::CHORD_ON_RIM, Ending::Sized),
];

/// The one ending of an escalation under `predicate`, read at the build
/// its door reports: [`ENDINGS`]' kind, and the bare escalation's sized
/// ending for a name this crate does not decide.
pub(crate) fn ending(predicate: &str, source: &Indeterminate) -> String {
    match lookup(ENDINGS, predicate) {
        Some(Ending::Lever) => LeverOnly {
            lever: COINCIDENCE_RECOURSE,
        }
        .recourse(RefusedArm::Undecided(source), Reading::Build),
        Some(Ending::Sized) | None => source.ending(COINCIDENCE_RECOURSE),
    }
}

#[cfg(test)]
mod census {
    use std::collections::BTreeSet;

    use super::{ENDINGS, words};

    /// The names `eval/measure.rs` hands its `parallel` trilean, read
    /// off its source: the measure door's escalations reach
    /// `NodeErrorKind::Escalated` under them, worded by `topo`.
    fn measure_names() -> BTreeSet<&'static str> {
        let src = include_str!("eval/measure.rs");
        let mut names = BTreeSet::new();
        for (at, _) in src.match_indices("parallel(") {
            let before = src[..at].chars().next_back();
            if before.is_some_and(|c| c.is_alphanumeric() || c == '_') {
                continue;
            }
            let rest = src[at + "parallel(".len()..].trim_start();
            if let Some((name, _)) = rest.strip_prefix('"').and_then(|q| q.split_once('"')) {
                names.insert(name);
            }
        }
        names
    }

    /// **The ending table is exactly the decisions this crate's whole
    /// renders are fed**: the naming layer's and the evaluation's own
    /// worded decisions, and the measure door's parallelism names. A
    /// worded decision added without a kind, or a row left behind by a
    /// renamed one, is red here; each row has words to state.
    #[test]
    fn the_ending_table_is_the_decisions_the_doors_render() {
        let listed: BTreeSet<&str> = ENDINGS.iter().map(|(n, _)| *n).collect();
        assert_eq!(listed.len(), ENDINGS.len(), "a name is listed twice");
        let measured = measure_names();
        assert!(
            measured.len() >= 2,
            "the measure scan found {measured:?}: its pattern no longer matches"
        );
        let fed: BTreeSet<&str> = crate::names::DECISION_WORDS
            .iter()
            .chain(crate::eval::DECISION_WORDS)
            .map(|(n, _)| *n)
            .chain(measured)
            .collect();
        assert_eq!(
            listed, fed,
            "ENDINGS against the decisions the doors are fed"
        );
        for name in listed {
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
    /// the pattern spacing is a size the user chose, and offers the
    /// tolerance on a value, subdivision on a straddle and the inputs on
    /// an unreadable margin; the circular step is an angle, and ends in
    /// the lever alone, noted where the margin could not be read.
    #[test]
    fn an_evaluations_undecided_decision_ends_by_its_kind() {
        check(
            "a sized decision",
            |m| node(crate::eval::PATTERN_SPACING, m),
            [
                "whether the pattern spacing is positive is undecided: margin 5e-9 lies inside \
                 the ambiguity band (1e-9, 1e-8). Recourse: move the geometry, or, if this size \
                 is intended, tighten the tolerance below 5e-10 m",
                "whether the pattern spacing is positive is undecided: enclosure [-2e-9, 5e-9] \
                 cannot be classified against the ambiguity band (1e-9, 1e-8). Recourse: \
                 subdivide the parameter box for a tighter enclosure, or move the geometry",
                "whether the pattern spacing is positive is undecided: margin is invalid (NaN or \
                 a refused enclosure) against the ambiguity band (1e-9, 1e-8). Recourse: check \
                 the operation's inputs upstream, then move the geometry; an unreadable margin \
                 may indicate a kernel bug worth reporting",
            ],
        );
        check(
            "a lever-only decision",
            |m| node(crate::eval::PATTERN_STEP, m),
            [
                "whether the pattern step is zero is undecided: margin 5e-9 lies inside the \
                 ambiguity band (1e-9, 1e-8). Recourse: move the geometry",
                "whether the pattern step is zero is undecided: enclosure [-2e-9, 5e-9] cannot \
                 be classified against the ambiguity band (1e-9, 1e-8). Recourse: move the \
                 geometry",
                "whether the pattern step is zero is undecided: margin is invalid (NaN or a \
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
             the tolerance below 5e-10 m"
        );
    }

    /// **Naming's undecided decision says what the name rests on, and
    /// ends sized**: a point's place along an edge is a distance in the
    /// result the user's geometry put there.
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
    /// sized**: the distance from the datum against the stated value is
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
                    predicate: crate::SEL_DATUM_DISTANCE,
                    source: at(crate::SEL_DATUM_DISTANCE, m),
                }
                .to_string()
            },
            [
                "select: whether the face (the face of node 000000000000) is in or out is \
                 undecided: margin 5e-9 lies inside the ambiguity band (1e-9, 1e-8). Recourse: \
                 move the geometry, or, if this size is intended, tighten the tolerance below \
                 5e-10 m",
                "select: whether the face (the face of node 000000000000) is in or out is \
                 undecided: enclosure [-2e-9, 5e-9] cannot be classified against the ambiguity \
                 band (1e-9, 1e-8). Recourse: subdivide the parameter box for a tighter \
                 enclosure, or move the geometry",
                "select: whether the face (the face of node 000000000000) is in or out is \
                 undecided: margin is invalid (NaN or a refused enclosure) against the ambiguity \
                 band (1e-9, 1e-8). Recourse: check the operation's inputs upstream, then move \
                 the geometry; an unreadable margin may indicate a kernel bug worth reporting",
            ],
        );
    }
}
