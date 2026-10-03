//! **A pattern's spacing is a positive length; its step is a signed
//! angle within a turn.**
//!
//! A linear rule's direction is the one home of which way the copies
//! step, so a spacing below zero refuses and names the direction that
//! steps the other way; a zero (or tolerance-small) spacing refuses
//! because every copy would land on the master. A circular rule's
//! step keeps its right-hand sign about the datum axis, and refuses
//! at zero, at a full turn and past one. Each check runs where a step
//! reads the value, so a one-copy rule reads none.
//!
//! Every row goes through `evaluate` on an ordinary document; the
//! mate solve's road is `msolve3_placer_refused`'s, and the range
//! certificate's is `docm9_range`'s.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;
use editor_core::ExtrudeSide;

use std::collections::BTreeMap;

use editor_core::{
    Expr, Node, NodeErrorClass, NodeErrorKind, NodeResult, PatternKind, ProfileDoc, RecipeNodeId,
    ValuePayload, parse_expr,
};
use fixture::{ang, len, scl};

use corpus::{eval, failures};

/// A block off the z axis (so a turn about it moves the copies), and
/// the z-axis datum a circular rule turns it about.
fn block() -> (corpus::Recorder, RecipeNodeId, RecipeNodeId) {
    let mut r = corpus::Recorder::new();
    let axis = r.insert(Node::Datum(editor_core::Datum::Axis {
        origin: [len(0.0), len(0.0), len(0.0)],
        direction: [scl(0.0), scl(0.0), scl(1.0)],
    }));
    let p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(4.0, -0.5), (5.0, -0.5), (5.0, 0.5), (4.0, 0.5)]],
    );
    let solid = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
        side: ExtrudeSide::Along,
    });
    (r, solid, axis)
}

fn linear(direction: [f64; 3], spacing: f64) -> PatternKind {
    PatternKind::Linear {
        direction: direction.map(scl),
        spacing: len(spacing),
    }
}

/// A pattern (or, with `union`, a placed union) of the block.
fn patterned(count: i64, kind: impl FnOnce(RecipeNodeId) -> PatternKind, union: bool) -> Built {
    let (mut r, solid, axis) = block();
    let kind = kind(axis);
    let node = if union {
        Node::placed_union(solid, Expr::count(count), kind).expect("a stepped rule takes a count")
    } else {
        Node::Pattern {
            input: solid,
            count: Expr::count(count),
            kind,
        }
    };
    let node = r.insert(node);
    Built { doc: r.doc, node }
}

struct Built {
    doc: ProfileDoc,
    node: RecipeNodeId,
}

impl Built {
    /// The node's refusal: its class and its sentence.
    fn refusal(&self) -> (NodeErrorClass, String) {
        let ev = eval::<f64>(&self.doc);
        match ev.result(self.node) {
            Some(NodeResult::Failed(e)) => (e.kind.class(), e.to_string()),
            other => panic!("the pattern refuses, got {other:?}"),
        }
    }

    /// How many bodies the node builds; panics on any failure.
    fn bodies(&self) -> usize {
        let ev = eval::<f64>(&self.doc);
        assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
        match &ev.value(self.node).expect("the node builds").payload {
            ValuePayload::Instances(bodies) => bodies.len(),
            ValuePayload::Body(body) => body.shells().count(),
            other => panic!("a pattern builds bodies, got {other:?}"),
        }
    }
}

/// Below the band's zero: a value the tolerance cannot tell from zero.
fn sliver() -> f64 {
    fixture::band().zero() / 4.0
}

/// **The spacing's four regions**: zero and a tolerance-small value
/// refuse as degenerate, a negative value refuses naming the reversed
/// direction, and a positive one builds.
#[test]
fn a_linear_spacing_is_a_positive_length() {
    for (spacing, class) in [
        (0.0, NodeErrorClass::DegenerateSpacing),
        (sliver(), NodeErrorClass::DegenerateSpacing),
        (-sliver(), NodeErrorClass::DegenerateSpacing),
        (-4.0, NodeErrorClass::NegativeSpacing),
    ] {
        let (got, text) = patterned(3, |_| linear([1.0, 0.0, 0.0], spacing), false).refusal();
        assert_eq!(got, class, "spacing {spacing}: {text}");
    }
    let (_, text) = patterned(3, |_| linear([3.0, 4.0, 0.0], -4.0), false).refusal();
    assert!(
        text.contains("-4 m") && text.contains("(-3.0, -4.0, 0.0)"),
        "the sentence quotes the spacing and the direction negated: {text}"
    );
    assert_eq!(
        patterned(3, |_| linear([-3.0, -4.0, 0.0], 4.0), false).bodies(),
        3,
        "the recourse, followed, builds"
    );
}

/// **The step's regions**: zero and a sliver refuse as degenerate; a
/// full turn either way and anything past one refuse as full-range,
/// past a turn naming the step a turn nearer zero; an ordinary step of
/// either sign builds.
#[test]
fn a_circular_step_is_a_signed_angle_within_a_turn() {
    use std::f64::consts::TAU;
    let circular = |step: f64| {
        move |axis| PatternKind::Circular {
            axis,
            step: ang(step),
        }
    };
    for (step, class) in [
        (0.0, NodeErrorClass::DegenerateStep),
        (sliver(), NodeErrorClass::DegenerateStep),
        (TAU, NodeErrorClass::FullRangeStep),
        (-TAU, NodeErrorClass::FullRangeStep),
        (7.0, NodeErrorClass::FullRangeStep),
        (-7.0, NodeErrorClass::FullRangeStep),
    ] {
        let (got, text) = patterned(3, circular(step), false).refusal();
        assert_eq!(got, class, "step {step}: {text}");
    }
    // Past a turn the refusal spells the step a turn nearer zero, in
    // the grammar; written back in, it builds.
    for (written, nearer) in [
        ("400 deg", "400 deg - 360 deg"),
        ("-400 deg", "-400 deg + 360 deg"),
    ] {
        let step = parse_expr(written, &BTreeMap::new()).unwrap();
        let built = patterned(3, |axis| PatternKind::Circular { axis, step }, false);
        let (_, text) = built.refusal();
        assert!(
            text.contains(&format!("write it as {nearer},")),
            "past a turn, the step a turn nearer zero: {text}"
        );
        let step = parse_expr(nearer, &BTreeMap::new()).unwrap();
        assert_eq!(
            patterned(3, |axis| PatternKind::Circular { axis, step }, false).bodies(),
            3,
            "the recourse, followed, builds: {nearer}"
        );
    }
    for step in [0.5, -0.5, TAU / 3.0] {
        assert_eq!(
            patterned(3, circular(step), false).bodies(),
            3,
            "step {step}"
        );
    }
}

/// **A one-copy rule reads no step**: copy 0 is the master, so a
/// blade count of one under `360° / blades` builds, and so does a
/// one-copy rule whose spacing would refuse at copy 1.
#[test]
fn a_one_copy_rule_reads_no_step() {
    use std::f64::consts::TAU;
    for union in [false, true] {
        let ring = patterned(
            1,
            |axis| PatternKind::Circular {
                axis,
                step: ang(TAU),
            },
            union,
        );
        assert_eq!(
            ring.bodies(),
            1,
            "one blade at a full turn (union: {union})"
        );
        let row = patterned(1, |_| linear([1.0, 0.0, 0.0], -4.0), union);
        assert_eq!(
            row.bodies(),
            1,
            "one copy at a negative spacing (union: {union})"
        );
    }
}

/// **A placed union refuses for the right reason.** A zero spacing
/// lands every copy on the master, which the union used to find out
/// only through its disjointness certificate; the rule itself now
/// says so, before any certificate is asked.
#[test]
fn a_placed_union_refuses_the_spacing_not_the_placements() {
    for (spacing, class) in [
        (0.0, NodeErrorClass::DegenerateSpacing),
        (-4.0, NodeErrorClass::NegativeSpacing),
    ] {
        let (got, text) = patterned(3, |_| linear([1.0, 0.0, 0.0], spacing), true).refusal();
        assert_eq!(got, class, "spacing {spacing}: {text}");
    }
    let step = patterned(
        3,
        |axis| PatternKind::Circular {
            axis,
            step: ang(0.0),
        },
        true,
    );
    assert_eq!(step.refusal().0, NodeErrorClass::DegenerateStep);
    assert_eq!(
        patterned(3, |_| linear([1.0, 0.0, 0.0], 2.0), true).bodies(),
        3,
        "a positive spacing builds three shells"
    );
}

/// **The kinds carry what their sentences quote**, so a caller that
/// matches reads the same numbers a reader does.
#[test]
fn the_refusals_carry_their_values() {
    let ev = eval::<f64>(&patterned(3, |_| linear([0.0, 2.0, 0.0], -1.5), false).doc);
    let kind = ev
        .nodes
        .values()
        .find_map(|r| match r {
            NodeResult::Failed(e) => Some(&e.kind),
            _ => None,
        })
        .expect("the pattern refuses");
    let NodeErrorKind::NegativeSpacing { reversed, .. } = kind else {
        panic!("a negative spacing, got {kind:?}");
    };
    assert_eq!(
        reversed.each_ref().map(String::as_str),
        ["0.0", "-2.0", "0.0"],
        "the authored direction, negated"
    );
}
