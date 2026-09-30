//! **An expression nests to its bound and refuses one past it, at every
//! door, on the smallest stack a door runs on.**
//!
//! Every row runs on a thread whose stack is the wasm32 build's (one
//! mebibyte, the smallest any door runs on). An expression at the bound
//! passes every door that mints, walks, keeps or drops one: the smart
//! constructors, the text door, both evaluators, `Clone`, equality,
//! `Debug`, `unparse`, the edit door, a document's evaluation (its
//! content key included), the content pin, save and load. One past the
//! bound, every door that mints one refuses typed with
//! [`DimensionError::NestedTooDeep`], and text or a file nested far
//! past it refuses typed rather than exhausting the stack.
//!
//! The bound is written here as the number the refusal carries, and the
//! one-past row reads it back out of the refusal, so the two cannot
//! drift apart silently.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;

use editor_core::{
    DimensionError, DocEdit, EditError, EvalOptions, Expr, ExprPath, LoopProgram, MeasureExpr,
    Node, NodeResult, ParamEnv, ParseError, PersistError, ProfileDoc, ProfileProgram,
    ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId, SlotId, ValuePayload, content_pin,
    eval, eval_count, parse_expr, unparse,
};
use fixture::{Recorder, len, run, scl, xy_frame};
use geom_core::{Interval, Tol};

/// The deepest an expression nests, in levels: the refusal names it.
const BOUND: usize = 128;

/// The wasm32 build's default stack, the smallest a door runs on.
const WASM_STACK: usize = 1 << 20;

/// Runs `f` on a thread with the wasm32 build's stack.
fn on_the_smallest_stack<R: Send + 'static>(f: impl FnOnce() -> R + Send + 'static) -> R {
    std::thread::Builder::new()
        .stack_size(WASM_STACK)
        .spawn(f)
        .expect("the thread starts")
        .join()
        .expect("the subject returns")
}

/// `metres` as a length `levels` deep: `metres + 0 + … + 0`, nested to
/// the left as the text door nests a sum, so it evaluates to `metres`
/// exactly.
fn deep_length(metres: f64, levels: usize) -> Expr {
    (1..levels).fold(len(metres), |e, _| Expr::add(e, len(0.0)).unwrap())
}

/// `levels - 1` negations over `metres`.
fn negations(metres: f64, levels: usize) -> Expr {
    (1..levels).fold(len(metres), |e, _| Expr::neg(e).unwrap())
}

/// A count `levels` deep: `1 + 1 + … + 1`.
fn deep_count(levels: usize) -> Expr {
    (1..levels).fold(Expr::count(1), |e, _| Expr::add(e, Expr::count(1)).unwrap())
}

/// A measurement `levels` deep: `measure_levels` nested sums over value
/// leaves, the first holding `metres` as an expression nested the rest
/// of the way, so it evaluates to `metres + 0.25 · (measure_levels - 1)`.
fn deep_measure(metres: f64, levels: usize, measure_levels: usize) -> MeasureExpr {
    let first = MeasureExpr::value(deep_length(metres, levels - measure_levels + 1));
    (1..measure_levels).fold(first, |m, _| {
        MeasureExpr::add(m, MeasureExpr::value(len(0.25))).unwrap()
    })
}

/// The nesting refusal a door raised, with the bound it names.
fn refused_bound(error: &DimensionError) -> usize {
    match error {
        DimensionError::NestedTooDeep { bound } => *bound,
        other => panic!("expected the nesting refusal, got {other:?}"),
    }
}

/// A recorded document holding an expression at the bound in each kind
/// of place one sits: a profile step's target (the deepest slot a save
/// writes), an extrude's distance, and a measurement.
fn deep_document() -> (Recorder, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    let plane = r.insert(xy_frame());
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::Chain(vec![
            ProgramStep::At([len(0.0), len(0.0)]),
            ProgramStep::LineTo(ProgramTarget::Point([len(2.0), len(0.0)])),
            ProgramStep::ArcTo(ProgramArcData::Bulge {
                target: ProgramTarget::Point([deep_length(2.0, BOUND), len(1.0)]),
                b: scl(0.25),
            }),
            ProgramStep::LineTo(ProgramTarget::Point([len(0.0), len(1.0)])),
            ProgramStep::LineTo(ProgramTarget::Start),
        ])],
        ids: Vec::new(),
    }));
    let extrude = r.insert(Node::Extrude {
        profile,
        distance: deep_length(0.5, BOUND),
    });
    let measure = r.insert(Node::Measure {
        expr: deep_measure(0.5, BOUND, BOUND / 2),
        refs: vec![],
    });
    (r, extrude, measure)
}

/// The value a measurement node evaluated to.
fn measured(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> f64 {
    match ev.result(id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => *value,
            other => panic!("node {id:?} is a {}", other.kind_name()),
        },
        _ => panic!("node {id:?} did not evaluate"),
    }
}

/// **At the bound, every door takes the expression**, on the wasm32
/// stack: the constructors mint it, both evaluators value it, the
/// derived walks and `unparse` read it, the text door reads it back,
/// and a document holding it in its deepest slots applies, evaluates,
/// pins, saves from its snapshot and from its edit log, and loads back
/// to the same document.
#[test]
fn every_door_takes_an_expression_at_the_bound_on_the_smallest_stack() {
    on_the_smallest_stack(|| {
        let env = ParamEnv::<f64>::default();
        for (label, e, value) in [
            ("a left-nested sum", deep_length(0.5, BOUND), 0.5),
            ("a chain of negations", negations(0.5, BOUND), -0.5),
        ] {
            assert_eq!(eval(&e, &env), Ok(value), "{label} evaluates at f64");
            assert!(
                eval(&e, &ParamEnv::<Interval>::default()).is_ok(),
                "{label} evaluates at Interval"
            );
            let copy = e.clone();
            assert!(copy.bit_eq(&e), "{label} clones bit for bit");
            assert!(format!("{e:?}").contains("Literal"), "{label} prints");
            let text = unparse(&e);
            let back = parse_expr(&text, &BTreeMap::new())
                .unwrap_or_else(|err| panic!("{label} reads back through the text door: {err}"));
            assert!(back.bit_eq(&e), "{label} round-trips through its text");
        }
        assert_eq!(
            eval_count(&deep_count(BOUND), &env),
            Ok(i64::try_from(BOUND).unwrap()),
            "a count sum at the bound evaluates exactly"
        );

        // The text door at the bound: brackets and calls nested BOUND
        // deep, and a sum of BOUND terms.
        let brackets = format!("{}1{}", "(".repeat(BOUND), ")".repeat(BOUND));
        let calls = format!("{}1{}", "max(1, ".repeat(BOUND - 1), ")".repeat(BOUND - 1));
        let terms = vec!["1"; BOUND].join(" + ");
        for (label, text) in [("brackets", brackets), ("calls", calls), ("terms", terms)] {
            let e = parse_expr(&text, &BTreeMap::new())
                .unwrap_or_else(|err| panic!("{label} nested to the bound parse: {err}"));
            assert!(eval_count(&e, &env).is_ok(), "{label} evaluate");
        }

        // A document holding the bound in its deepest slots.
        let (r, extrude, measure) = deep_document();
        let ev = run(&r.doc, &EvalOptions::default());
        assert!(
            matches!(ev.result(extrude), Some(NodeResult::Ok(_))),
            "the extrude over the deep profile evaluates"
        );
        let expected = 0.5 + 0.25 * f64::from(u32::try_from(BOUND / 2 - 1).unwrap());
        assert_eq!(
            measured(&ev, measure),
            expected,
            "the measurement evaluates"
        );
        let pin = content_pin(&r.doc, Tol::witness()).expect("the document pins");

        // Saved as a snapshot and as an edit log replayed over the empty
        // document, it loads back to the document it was.
        let empty = ProfileDoc::empty_derived("mod", Tol::witness());
        for (label, snapshot, edits) in [
            ("snapshot", &r.doc, &[][..]),
            ("edit log", &empty, &r.edits[..]),
        ] {
            let text = editor_core::persist::save(snapshot, edits, Tol::witness())
                .unwrap_or_else(|err| panic!("the {label} saves: {err}"));
            let loaded = editor_core::persist::load(&text, Tol::witness())
                .unwrap_or_else(|err| panic!("the {label} loads back: {err}"));
            assert_eq!(
                content_pin(&loaded.doc, Tol::witness()).expect("the loaded document pins"),
                pin,
                "the {label} loads back to the document it saved"
            );
        }
    });
}

/// **One past the bound, every door that mints an expression refuses
/// typed**, naming the bound in a sentence that meets the refusal
/// standard: the constructors (the negation that used to be total
/// among them), the measurement constructors, the text door at the
/// offset it declined to read past, the edit door's subtree
/// replacement, and the load door.
#[test]
fn one_past_the_bound_refuses_typed_at_every_door_that_mints_one() {
    on_the_smallest_stack(|| {
        let at = deep_length(0.5, BOUND);
        let error = Expr::add(at.clone(), len(0.0)).expect_err("a sum one past the bound");
        assert_eq!(
            refused_bound(&error),
            BOUND,
            "the refusal names this suite's bound"
        );
        let line = error.to_string();
        assert!(
            line.contains(&format!("deeper than {BOUND} levels")),
            "{line}"
        );
        let problems = test_utils::refusal::problems("NestedTooDeep", &line, &[], false);
        assert!(problems.is_empty(), "{problems:#?}");

        for (label, refused) in [
            ("neg", Expr::neg(at.clone()).err()),
            ("mul", Expr::mul(at.clone(), scl(1.0)).err()),
            ("min", Expr::min(len(0.0), at.clone()).err()),
            (
                "sin",
                Expr::sin((1..BOUND).fold(fixture::ang(0.5), |e, _| Expr::neg(e).unwrap())).err(),
            ),
            (
                "count_to_scalar",
                Expr::count_to_scalar(deep_count(BOUND)).err(),
            ),
            (
                "measure neg",
                MeasureExpr::neg(deep_measure(0.5, BOUND, 3)).err(),
            ),
            (
                "measure add",
                MeasureExpr::add(
                    MeasureExpr::value(len(0.0)),
                    deep_measure(0.5, BOUND, BOUND),
                )
                .err(),
            ),
        ] {
            let error = refused.unwrap_or_else(|| panic!("{label} one past the bound refuses"));
            assert_eq!(refused_bound(&error), BOUND, "{label}");
        }

        // The text door refuses at the bracket, sign or operator it
        // declines to read past.
        let brackets = format!("{}1{}", "(".repeat(BOUND + 1), ")".repeat(BOUND + 1));
        let signs = format!("{}1", "-".repeat(BOUND));
        let terms = vec!["1"; BOUND + 1].join("+");
        let last_plus = terms.rfind('+').unwrap();
        for (label, text, pos) in [
            ("brackets", brackets, BOUND),
            ("signs", signs, 0),
            ("terms", terms, last_plus),
        ] {
            match parse_expr(&text, &BTreeMap::new()) {
                Err(ParseError::Dimension { pos: at, error }) => {
                    assert_eq!(refused_bound(&error), BOUND, "{label}");
                    assert_eq!(at, pos, "{label} refuses at the offset it declines");
                }
                other => panic!("{label} one past the bound refuses typed, got {other:?}"),
            }
        }

        // The edit door: deepening the deepest leaf by one level.
        let mut r = Recorder::new();
        let plane = r.insert(xy_frame());
        let profile = r.insert(Node::Profile(fixture::desc(
            plane,
            vec![fixture::square(0.0, 0.0, 0.5)],
        )));
        let extrude = r.insert(Node::Extrude {
            profile,
            distance: at.clone(),
        });
        let edit = DocEdit::SetExpression {
            path: ExprPath {
                node: extrude,
                slot: SlotId::Distance,
                path: vec![0; BOUND - 1],
            },
            expr: Expr::neg(len(0.5)).unwrap(),
        };
        match editor_core::apply(&r.doc, &edit, Tol::witness(), &editor_core::RefusingReach) {
            Err(EditError::Dimension(error)) => assert_eq!(refused_bound(&error), BOUND),
            other => panic!("the edit door refuses the deeper tree, got {other:?}"),
        }

        // The load door: the saved distance wrapped in one more negation.
        let text = editor_core::persist::save(&r.doc, &[], Tol::witness()).unwrap();
        let deeper = wrap_distance(&text, 1);
        match editor_core::persist::load(&deeper, Tol::witness()) {
            Err(PersistError::Dimension { error, .. }) => assert_eq!(refused_bound(&error), BOUND),
            other => panic!("the load door refuses the deeper tree, got {other:?}"),
        }
    });
}

/// `text` with the first extrude distance wrapped in `levels` more
/// negations, spelled as the wire spells one.
fn wrap_distance(text: &str, levels: usize) -> String {
    let key = "\"distance\": ";
    let at = text.find(key).expect("the save holds a distance") + key.len();
    // The distance's value is the object that opens here; its end is
    // the brace that closes it.
    let mut depth = 0usize;
    let mut end = at;
    for (i, c) in text[at..].char_indices() {
        match c {
            '{' | '[' => depth += 1,
            '}' | ']' => {
                depth -= 1;
                if depth == 0 {
                    end = at + i + 1;
                    break;
                }
            }
            _ => {}
        }
    }
    format!(
        "{}{}{}{}{}",
        &text[..at],
        "{\"Neg\": ".repeat(levels),
        &text[at..end],
        "}".repeat(levels),
        &text[end..]
    )
}

/// **Text or a file nested far past the bound refuses typed on the
/// smallest stack** rather than exhausting it: the text door reads
/// brackets, signs, calls and terms nested a hundred thousand deep to
/// a refusal, and the load door refuses a distance negated twice the
/// bound deep before reading into it, and one negated a hundred
/// thousand deep before reading the body at all.
#[test]
fn text_and_files_nested_far_past_the_bound_refuse_typed_on_the_smallest_stack() {
    on_the_smallest_stack(|| {
        const FAR: usize = 100_000;
        for (label, text) in [
            (
                "brackets",
                format!("{}1{}", "(".repeat(FAR), ")".repeat(FAR)),
            ),
            ("signs", format!("{}1", "-".repeat(FAR))),
            (
                "calls",
                format!("{}1{}", "max(1,".repeat(FAR), ")".repeat(FAR)),
            ),
            ("terms", vec!["1"; FAR].join("+")),
        ] {
            match parse_expr(&text, &BTreeMap::new()) {
                Err(ParseError::Dimension { error, .. }) => {
                    assert_eq!(refused_bound(&error), BOUND, "{label}");
                }
                other => panic!("{label} nested far past the bound refuses typed, got {other:?}"),
            }
        }

        let mut r = Recorder::new();
        let plane = r.insert(xy_frame());
        let profile = r.insert(Node::Profile(fixture::desc(
            plane,
            vec![fixture::square(0.0, 0.0, 0.5)],
        )));
        r.insert(Node::Extrude {
            profile,
            distance: len(0.5),
        });
        let text = editor_core::persist::save(&r.doc, &[], Tol::witness()).unwrap();
        match editor_core::persist::load(&wrap_distance(&text, 2 * BOUND), Tol::witness()) {
            Err(PersistError::Dimension { error, .. }) => assert_eq!(refused_bound(&error), BOUND),
            other => panic!("a distance negated twice the bound deep refuses typed, got {other:?}"),
        }
        match editor_core::persist::load(&wrap_distance(&text, FAR), Tol::witness()) {
            Err(err @ PersistError::Parse { .. }) => {
                assert!(
                    err.to_string().contains("the body nests deeper than"),
                    "{err}"
                );
            }
            other => panic!("a body nested far past the limit refuses unread, got {other:?}"),
        }
    });
}
