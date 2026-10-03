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
    Dimension, DocEdit, DocParam, Expr, Node, NodeErrorClass, NodeErrorKind, NodeResult, ParamName,
    PatternKind, ProfileDoc, RecipeNodeId, StepTurns, ValuePayload, parse_expr,
};
use fixture::{ang, len, scl};

use corpus::{eval, failures};

/// A block off the z axis (so a turn about it moves the copies), and
/// the z-axis datum a circular rule turns it about; `th`, when given,
/// is declared an angle parameter first.
fn block(th: Option<(&ParamName, f64)>) -> (corpus::Recorder, RecipeNodeId, RecipeNodeId) {
    let mut r = corpus::Recorder::new();
    if let Some((name, radians)) = th {
        r.push(DocEdit::SetDocParam {
            name: name.clone(),
            value: DocParam::continuous(Dimension::Angle, radians),
        });
    }
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
    built(count, kind, union, None)
}

/// A pattern of the block whose rule reads the angle parameter `th`.
fn driven(
    count: i64,
    kind: impl FnOnce(RecipeNodeId) -> PatternKind,
    th: (&ParamName, f64),
) -> Built {
    built(count, kind, false, Some(th))
}

fn built(
    count: i64,
    kind: impl FnOnce(RecipeNodeId) -> PatternKind,
    union: bool,
    th: Option<(&ParamName, f64)>,
) -> Built {
    let (mut r, solid, axis) = block(th);
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

    /// A full-range step's landing, and whether it carries a reading.
    fn full_range_step(&self) -> (StepTurns, bool) {
        let ev = eval::<f64>(&self.doc);
        match ev.result(self.node) {
            Some(NodeResult::Failed(e)) => match &e.kind {
                NodeErrorKind::FullRangeStep {
                    turns, evaluated, ..
                } => (turns.clone(), evaluated.is_some()),
                other => panic!("a full-range step, got {other:?}"),
            },
            other => panic!("the pattern refuses, got {other:?}"),
        }
    }

    /// Each copy's centroid, in copy order; panics on any failure.
    fn centroids(&self) -> Vec<[f64; 3]> {
        let ev = eval::<f64>(&self.doc);
        assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
        let ValuePayload::Instances(bodies) = &ev.value(self.node).expect("builds").payload else {
            panic!("a pattern builds instances");
        };
        bodies
            .iter()
            .map(|body| {
                let (n, sum) = body.points().fold((0.0, [0.0; 3]), |(n, c), (_, p)| {
                    (n + 1.0, [c[0] + p.x, c[1] + p.y, c[2] + p.z])
                });
                sum.map(|c| c / n)
            })
            .collect()
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
/// whole number of turns either way and anything past one turn refuse
/// as full-range; an ordinary step of either sign builds.
#[test]
fn a_circular_step_is_a_signed_angle_within_a_turn() {
    use std::f64::consts::TAU;
    for (step, class) in [
        (0.0, NodeErrorClass::DegenerateStep),
        (sliver(), NodeErrorClass::DegenerateStep),
        (TAU, NodeErrorClass::FullRangeStep),
        (-TAU, NodeErrorClass::FullRangeStep),
        (2.0 * TAU, NodeErrorClass::FullRangeStep),
        (7.0, NodeErrorClass::FullRangeStep),
        (-7.0, NodeErrorClass::FullRangeStep),
        (13.0, NodeErrorClass::FullRangeStep),
    ] {
        let (got, text) = patterned(3, circular(ang(step)), false).refusal();
        assert_eq!(got, class, "step {step}: {text}");
    }
    for step in [0.5, -0.5, TAU / 3.0] {
        assert_eq!(
            patterned(3, circular(ang(step)), false).bodies(),
            3,
            "step {step}"
        );
    }
}

/// **The full-turn band**: a step a band-sized hair off a whole turn
/// is too close to call, on either side of it and of either sign.
#[test]
fn a_step_a_hair_off_a_turn_escalates() {
    use std::f64::consts::TAU;
    let hair = 2.0 * fixture::band().zero();
    for step in [TAU + hair, TAU - hair, -TAU - hair, 2.0 * TAU + hair] {
        let (got, text) = patterned(3, circular(ang(step)), false).refusal();
        assert_eq!(got, NodeErrorClass::Escalated, "step {step}: {text}");
    }
}

/// **A whole number of turns lands every copy on the master**, so the
/// refusal says so, not "past a full turn": following a "past" recourse
/// from 720° would only refuse again.
#[test]
fn a_whole_number_of_turns_refuses_as_coinciding_copies() {
    use std::f64::consts::TAU;
    for step in [
        written("360 deg"),
        written("720 deg"),
        written("-720 deg"),
        written("1080 deg"),
        ang(2.0 * TAU),
        ang(-3.0 * TAU),
    ] {
        let shown = editor_core::unparse(&step);
        let built = patterned(3, circular(step), false);
        let (_, text) = built.refusal();
        let (turns, evaluated) = built.full_range_step();
        assert_eq!(turns, StepTurns::Whole, "{shown}: {text}");
        assert!(!evaluated, "{shown}: a literal is its own reading");
        assert!(
            text.contains("is a whole number of turns, so every copy would land on the master"),
            "{shown}: {text}"
        );
    }
}

/// **Past a turn, the recourse is one angle within it, and following
/// it lands the copies where the refused step would have.** A literal
/// is rewritten as one literal in its own unit; every reduction is
/// taken at once (760° is 40°, not "760 deg - 360 deg - 360 deg"), and
/// the angle it names builds on the first try. The landing is
/// geometric, not bitwise: each copy's centroid against the master's
/// turned by the refused step, to rounding.
#[test]
fn past_a_turn_the_recourse_is_one_angle_that_lands_every_copy() {
    for (step, radians, within) in [
        ("760 deg", 760f64.to_radians(), Some("40 deg")),
        ("-760 deg", (-760f64).to_radians(), Some("-40 deg")),
        ("400 deg", 400f64.to_radians(), Some("40 deg")),
        ("700 deg", 700f64.to_radians(), Some("340 deg")),
        ("13 rad", 13.0, None),
        ("-13 rad", -13.0, None),
        ("1000 rad", 1000.0, None),
    ] {
        let built = patterned(5, circular(written(step)), false);
        let (_, text) = built.refusal();
        let (turns, evaluated) = built.full_range_step();
        let StepTurns::Within(named) = turns else {
            panic!("{step}: past a turn names an angle within one, got {turns:?}: {text}");
        };
        assert!(!evaluated, "{step}: a literal is its own reading");
        if let Some(within) = within {
            assert_eq!(named, within, "{step}: {text}");
        }
        assert!(
            text.contains(&format!("write it as {named}, which places every copy")),
            "{step}: {text}"
        );
        assert!(text.contains("up to rounding"), "{step}: {text}");
        lands_where(radians, &named, &BTreeMap::new(), None);
    }
}

/// **A driven step says what it evaluated to**, and that it must
/// evaluate within one turn; the angle it names is the expression less
/// the turns it holds, which builds and lands where the step would.
#[test]
fn a_driven_step_past_a_turn_says_what_it_evaluated_to() {
    let th = ParamName::from_static("th");
    let params = BTreeMap::from([(th.clone(), Dimension::Angle)]);
    for (step, value, times, within) in [
        ("th", 760.0, 1.0, "th - 720 deg"),
        ("-th", 760.0, -1.0, "-th + 720 deg"),
        ("th * 2.0", 200.0, 2.0, "th * 2.0 - 360 deg"),
    ] {
        let value = f64::to_radians(value);
        let radians = times * value;
        let expr = parse_expr(step, &params).unwrap();
        let built = driven(5, circular(expr), (&th, value));
        let (class, text) = built.refusal();
        assert_eq!(class, NodeErrorClass::FullRangeStep, "{step}: {text}");
        let (turns, evaluated) = built.full_range_step();
        assert_eq!(
            turns,
            StepTurns::Within(within.to_owned()),
            "{step}: {text}"
        );
        assert!(evaluated, "{step}: a driven step carries its reading");
        assert!(
            text.contains(&format!("which evaluated to {radians} rad")),
            "{step}: the reading, in radians: {text}"
        );
        assert!(
            text.contains(&format!("make it evaluate within one turn; {within} does")),
            "{step}: {text}"
        );
        lands_where(radians, within, &params, Some((&th, value)));
    }
    let expr = parse_expr("th * 2.0", &params).unwrap();
    let whole = driven(3, circular(expr), (&th, f64::to_radians(360.0)));
    let (_, text) = whole.refusal();
    assert_eq!(whole.full_range_step(), (StepTurns::Whole, true), "{text}");
    assert!(
        text.contains("make it evaluate to a nonzero angle within one turn"),
        "{text}"
    );
}

/// A circular rule about the block's axis at `step`.
fn circular(step: Expr) -> impl FnOnce(RecipeNodeId) -> PatternKind {
    move |axis| PatternKind::Circular { axis, step }
}

/// `text` parsed as a parameter-free expression.
fn written(text: &str) -> Expr {
    parse_expr(text, &BTreeMap::new()).unwrap()
}

/// **Follows a recourse**: builds five copies at `within` (with `th`
/// declared when it is driven) and checks each copy's centroid sits
/// where turning the master's by `i · radians` about the z axis puts
/// it.
fn lands_where(
    radians: f64,
    within: &str,
    params: &BTreeMap<ParamName, Dimension>,
    th: Option<(&ParamName, f64)>,
) {
    let step = parse_expr(within, params).unwrap_or_else(|e| panic!("{within:?} parses: {e:?}"));
    let built = match th {
        Some(th) => driven(5, circular(step), th),
        None => patterned(5, circular(step), false),
    };
    let centroids = built.centroids();
    assert_eq!(
        centroids.len(),
        5,
        "{within}: the recourse builds every copy"
    );
    let [x, y, z] = centroids[0];
    for (i, got) in centroids.iter().enumerate() {
        let (sin, cos) = (i as f64 * radians).sin_cos();
        let want = [x * cos - y * sin, x * sin + y * cos, z];
        let off = (0..3).map(|k| (got[k] - want[k]).abs()).fold(0.0, f64::max);
        assert!(
            off < 1e-9,
            "{within}: copy {i} lands {off} m from where the refused step put it ({got:?} vs \
             {want:?})"
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
