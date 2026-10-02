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
use editor_core::ExtrudeSide;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    DimensionError, DocEdit, EditError, EvalOptions, Expr, ExprPath, LoopProgram, MeasureExpr,
    Node, NodeResult, ParamEnv, ParseError, PersistError, ProfileDoc, ProfileProgram,
    ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId, SlotId, ValuePayload, content_pin,
    eval, eval_count, parse_expr, unparse,
};
use fixture::{Recorder, len, run, scl, xy_frame};
use geom_core::{Interval, Tol};
use serde_json::Value;
use test_utils::own_thread::on_the_smallest_stack;

/// The deepest an expression nests, in levels: the refusal names it.
const BOUND: usize = 128;

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

/// A count `levels` deep: `start + 1 + … + 1`.
fn deep_count(start: i64, levels: usize) -> Expr {
    (1..levels).fold(Expr::count(start), |e, _| {
        Expr::add(e, Expr::count(1)).unwrap()
    })
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

/// A recorded document holding an expression `levels` deep in each kind
/// of place one sits: a profile step's target (the deepest slot a save
/// writes), an extrude's distance, and a measurement.
fn deep_document(levels: usize) -> (Recorder, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    let plane = r.insert(xy_frame());
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::Chain(vec![
            ProgramStep::At([len(0.0), len(0.0)]),
            ProgramStep::LineTo(ProgramTarget::Point([len(2.0), len(0.0)])),
            ProgramStep::ArcTo(ProgramArcData::Bulge {
                target: ProgramTarget::Point([deep_length(2.0, levels), len(1.0)]),
                b: scl(0.25),
            }),
            ProgramStep::LineTo(ProgramTarget::Point([len(0.0), len(1.0)])),
            ProgramStep::LineTo(ProgramTarget::Start),
        ])],
        ids: Vec::new(),
    }));
    let extrude = r.insert(Node::Extrude {
        profile,
        distance: deep_length(0.5, levels),
        side: ExtrudeSide::Along,
    });
    let measure = r.insert(Node::Measure {
        expr: deep_measure(0.5, levels, levels.div_ceil(2)),
        refs: vec![],
    });
    (r, extrude, measure)
}

/// A document whose one expression of note is an extrude's distance,
/// `distance`, and the extrude.
fn extrude_document(distance: Expr) -> (Recorder, RecipeNodeId) {
    let mut r = Recorder::new();
    let plane = r.insert(xy_frame());
    let profile = r.insert(Node::Profile(fixture::desc(
        plane,
        vec![fixture::square(0.0, 0.0, 0.5)],
    )));
    let extrude = r.insert(Node::Extrude {
        profile,
        distance,
        side: ExtrudeSide::Along,
    });
    (r, extrude)
}

/// [`extrude_document`] over a plain length, saved from its snapshot.
fn saved_extrude() -> String {
    editor_core::persist::save(&extrude_document(len(0.5)).0.doc, &[], Tol::witness())
        .expect("the document saves")
}

/// `r` saved from its snapshot and from its edit log replayed over the
/// empty document, each with its label.
fn both_saves(r: &Recorder) -> [(&'static str, String); 2] {
    let empty = ProfileDoc::empty_derived("mod", Tol::witness());
    let save = |label: &str, snapshot: &ProfileDoc, edits| {
        editor_core::persist::save(snapshot, edits, Tol::witness())
            .unwrap_or_else(|err| panic!("the {label} saves: {err}"))
    };
    [
        ("snapshot", save("snapshot", &r.doc, &[])),
        ("edit log", save("edit log", &empty, &r.edits)),
    ]
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

/// The saved `text`'s body: everything from the brace that opens it.
fn body(text: &str) -> &str {
    &text[text.find('{').expect("a saved body is an object")..]
}

/// How deep `body` nests, in JSON brackets outside strings.
fn bracket_depth(body: &str) -> usize {
    let (mut depth, mut deepest) = (0usize, 0usize);
    let (mut in_string, mut escaped) = (false, false);
    for byte in body.bytes() {
        if in_string {
            match byte {
                _ if escaped => escaped = false,
                b'\\' => escaped = true,
                b'"' => in_string = false,
                _ => {}
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'[' | b'{' => {
                depth += 1;
                deepest = deepest.max(depth);
            }
            b']' | b'}' => depth -= 1,
            _ => {}
        }
    }
    deepest
}

/// `text` with the first extrude distance wrapped in `levels` more
/// negations, spelled as the wire spells one, and the body line the
/// distance's value opens on.
fn wrap_distance(text: &str, levels: usize) -> (String, usize) {
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
    let wrapped = format!(
        "{}{}{}{}{}",
        &text[..at],
        "{\"Neg\": ".repeat(levels),
        &text[at..end],
        "}".repeat(levels),
        &text[end..]
    );
    let body_start = text.find('{').expect("a saved body is an object");
    let line = text[body_start..at].matches('\n').count() + 1;
    (wrapped, line)
}

/// **At the bound, every door takes the expression**, on the wasm32
/// stack: the constructors mint it, both evaluators value it, the
/// derived walks and `unparse` read it, the text door reads it back,
/// and a document holding it in its deepest slots applies, evaluates,
/// pins, saves from its snapshot and from its edit log, and loads back
/// to the same document. The deepest of those saves nests exactly as
/// deep as the load door reads.
#[test]
fn every_door_takes_an_expression_at_the_bound_on_the_smallest_stack() {
    on_the_smallest_stack(|| {
        let env = ParamEnv::<f64>::default();
        for (label, e, value) in [
            ("a left-nested sum", deep_length(0.5, BOUND), 0.5),
            (
                "a sum over a negative literal",
                deep_length(-0.5, BOUND),
                -0.5,
            ),
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
            eval_count(&deep_count(1, BOUND), &env),
            Ok(i64::try_from(BOUND).unwrap()),
            "a count sum at the bound evaluates exactly"
        );

        // The text door at the bound: calls nested BOUND deep, signs
        // over a literal (the innermost its own), and a sum of BOUND
        // terms.
        let calls = format!("{}1{}", "max(1, ".repeat(BOUND - 1), ")".repeat(BOUND - 1));
        let signs = format!("{}1", "-".repeat(BOUND));
        let terms = vec!["1"; BOUND].join(" + ");
        for (label, text) in [("calls", calls), ("signs", signs), ("terms", terms)] {
            let e = parse_expr(&text, &BTreeMap::new())
                .unwrap_or_else(|err| panic!("{label} nested to the bound parse: {err}"));
            assert!(eval_count(&e, &env).is_ok(), "{label} evaluate");
        }

        // A document holding the bound in its deepest slots.
        let (r, extrude, measure) = deep_document(BOUND);
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
        let mut deepest = 0;
        for (label, text) in both_saves(&r) {
            deepest = deepest.max(bracket_depth(body(&text)));
            let loaded = editor_core::persist::load(&text, Tol::witness())
                .unwrap_or_else(|err| panic!("the {label} loads back: {err}"));
            assert_eq!(
                content_pin(&loaded.doc, Tol::witness()).expect("the loaded document pins"),
                pin,
                "the {label} loads back to the document it saved"
            );
        }
        assert_eq!(
            deepest,
            editor_core::test_support::BODY_NESTING,
            "the deepest save nests exactly as deep as the load door reads"
        );
    });
}

/// **One past the bound, every door that mints an expression refuses
/// typed**, naming the bound in a sentence that meets the refusal
/// standard: the constructors (the negation that used to be total
/// among them), the measurement constructors, the text door at the
/// sign or operator whose node would pass it, the edit door's subtree
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
                Expr::count_to_scalar(deep_count(1, BOUND)).err(),
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

        // The text door refuses at the sign or operator whose node
        // would pass the bound: the outermost of BOUND + 1 signs (the
        // innermost is the literal's own), and the last `+`.
        let signs = format!("{}1", "-".repeat(BOUND + 1));
        let terms = vec!["1"; BOUND + 1].join("+");
        let last_plus = terms.rfind('+').unwrap();
        for (label, text, pos) in [("signs", signs, 0), ("terms", terms, last_plus)] {
            match parse_expr(&text, &BTreeMap::new()) {
                Err(ParseError::Dimension { pos: at, error }) => {
                    assert_eq!(refused_bound(&error), BOUND, "{label}");
                    assert_eq!(at, pos, "{label} refuses at the node that would pass it");
                }
                other => panic!("{label} one past the bound refuses typed, got {other:?}"),
            }
        }

        // The edit door: deepening the deepest leaf by one level.
        let (r, extrude) = extrude_document(at.clone());
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
        match editor_core::persist::load(&wrap_distance(&text, 1).0, Tol::witness()) {
            Err(PersistError::Dimension { error, .. }) => assert_eq!(refused_bound(&error), BOUND),
            other => panic!("the load door refuses the deeper tree, got {other:?}"),
        }
    });
}

/// **Text or a file nested far past the bound refuses typed on the
/// smallest stack** rather than exhausting it: the text door reads
/// signs, calls and terms nested a hundred thousand deep to a refusal,
/// and the load door refuses a distance negated twice the bound deep
/// before reading into it, and one negated a hundred thousand deep
/// before reading the body at all.
#[test]
fn text_and_files_nested_far_past_the_bound_refuse_typed_on_the_smallest_stack() {
    on_the_smallest_stack(|| {
        const FAR: usize = 100_000;
        for (label, text) in [
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

        let text = saved_extrude();
        match editor_core::persist::load(&wrap_distance(&text, 2 * BOUND).0, Tol::witness()) {
            Err(PersistError::Dimension { error, .. }) => assert_eq!(refused_bound(&error), BOUND),
            other => panic!("a distance negated twice the bound deep refuses typed, got {other:?}"),
        }
        match editor_core::persist::load(&wrap_distance(&text, FAR).0, Tol::witness()) {
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

/// **Brackets nest no expression**, so the text door's one nesting
/// refusal is the tree's: a literal in a hundred thousand brackets
/// reads as the literal on the smallest stack, a sum at the bound reads
/// inside brackets nested as deep again, and one term more refuses in
/// the sentence that names the tree's nesting.
#[test]
fn brackets_nest_no_expression_so_only_the_tree_refuses() {
    on_the_smallest_stack(|| {
        const FAR: usize = 100_000;
        let lone = format!("{}1{}", "(".repeat(FAR), ")".repeat(FAR));
        let read = parse_expr(&lone, &BTreeMap::new()).expect("a bracketed literal reads");
        assert!(
            read.bit_eq(&Expr::count(1)),
            "the brackets add no node: {read:?}"
        );
        let wrapped = |terms: usize| {
            let sum = vec!["1"; terms].join("+");
            format!("{}{sum}{}", "(".repeat(BOUND), ")".repeat(BOUND))
        };
        let at = parse_expr(&wrapped(BOUND), &BTreeMap::new());
        assert!(at.is_ok(), "a sum at the bound, bracketed: {at:?}");
        match parse_expr(&wrapped(BOUND + 1), &BTreeMap::new()) {
            Err(ParseError::Dimension { error, .. }) => {
                assert_eq!(refused_bound(&error), BOUND);
            }
            other => panic!("one term more refuses the tree, got {other:?}"),
        }
    });
}

/// **A negative literal at the bottom of a tree at the bound reads back
/// from its text**, node for node: its sign is the literal's own, not a
/// negation one level deeper, so the text of a legal tree is never
/// refused by the text door (the viewer's props field shows this text
/// and reads it back unchanged).
#[test]
fn a_negative_leaf_at_the_bound_reads_back() {
    on_the_smallest_stack(|| {
        for levels in [1, 2, BOUND - 1, BOUND] {
            for (label, e) in [
                ("a length", deep_length(-0.5, levels)),
                ("a count", deep_count(-3, levels)),
                ("the least count", deep_count(i64::MIN, levels)),
            ] {
                let text = unparse(&e);
                let back = parse_expr(&text, &BTreeMap::new()).unwrap_or_else(|err| {
                    panic!("{label} {levels} deep reads back from {text:.40}…: {err}")
                });
                assert!(
                    back.bit_eq(&e),
                    "{label} {levels} deep reads back as itself"
                );
                assert_eq!(unparse(&back), text, "{label} {levels} deep");
            }
        }
    });
}

/// **A measurement shares the bound, at every split of its levels
/// between measurement nodes and the expression a value leaf holds**:
/// at the bound it saves and loads back on the smallest stack, and one
/// level more refuses at the constructor.
#[test]
fn a_measurement_shares_the_bound_at_every_split() {
    on_the_smallest_stack(|| {
        for measure_levels in [1, 2, 3, BOUND / 2, BOUND - 1, BOUND] {
            let at = deep_measure(0.5, BOUND, measure_levels);
            let (mut r, _) = extrude_document(len(0.5));
            r.insert(Node::Measure {
                expr: at.clone(),
                refs: vec![],
            });
            let pin = content_pin(&r.doc, Tol::witness()).expect("the document pins");
            for (label, text) in both_saves(&r) {
                let loaded = editor_core::persist::load(&text, Tol::witness())
                    .unwrap_or_else(|err| panic!("split {measure_levels}, {label}: {err}"));
                assert_eq!(
                    content_pin(&loaded.doc, Tol::witness()).expect("pins"),
                    pin,
                    "split {measure_levels}: the {label} loads back"
                );
            }
            let one_more = MeasureExpr::add(at, MeasureExpr::value(len(0.0)))
                .expect_err("one level more refuses");
            assert_eq!(refused_bound(&one_more), BOUND, "split {measure_levels}");
        }
    });
}

/// **The load door reads an expression's child at the bound and refuses
/// the next one before reading into it.** A saved distance negated to
/// the bound loads; one negation more refuses with the constructors'
/// own refusal at the child it declined, on the line the distance opens
/// on, before the reader reaches the literal inside, rather than after
/// reading the whole tree and rebuilding it.
#[test]
fn the_load_door_refuses_the_child_past_the_bound_before_reading_it() {
    on_the_smallest_stack(|| {
        let text = saved_extrude();
        let (at_the_bound, _) = wrap_distance(&text, BOUND - 1);
        let loaded = editor_core::persist::load(&at_the_bound, Tol::witness())
            .unwrap_or_else(|err| panic!("a distance negated to the bound loads: {err}"));
        drop(loaded);
        let (past, distance_line) = wrap_distance(&text, BOUND);
        match editor_core::persist::load(&past, Tol::witness()) {
            Err(PersistError::Dimension { line, error, .. }) => {
                assert_eq!(refused_bound(&error), BOUND);
                assert_eq!(
                    line, distance_line,
                    "refused at the child it declined, not after reading the tree"
                );
            }
            other => panic!("one negation more refuses typed, got {other:?}"),
        }
    });
}

/// A body the load door refuses in each way a refusal can leave the
/// expression reader: at the bound's own guard, far past it, on a unit
/// and a variant it does not know deep inside a tree, and on a body cut
/// off mid-tree.
fn refused_bodies() -> Vec<(&'static str, String)> {
    let text = saved_extrude();
    let wrapped = |levels| wrap_distance(&text, levels).0;
    let deep = wrapped(BOUND - 8);
    // The tree's own literal: the first unit after the distance opens.
    let distance = deep
        .find("\"distance\": ")
        .expect("the save holds a distance");
    let unit = distance
        + deep[distance..]
            .find("\"unit\": \"")
            .expect("the literal's unit");
    let (before, after) = deep.split_at(unit);
    vec![
        ("one past the bound", wrapped(BOUND)),
        ("far past the bound", wrapped(2 * BOUND)),
        (
            "an unknown unit deep in the tree",
            format!(
                "{before}{}",
                after.replacen("\"unit\": \"", "\"unit\": \"zz", 1)
            ),
        ),
        (
            "an unknown variant deep in the tree",
            deep.replacen("{\"Neg\": {\"Neg\": ", "{\"Neg\": {\"Nope\": ", 1),
        ),
        ("a body cut off mid-tree", before.to_string()),
    ]
}

/// **A refused load leaves nothing behind for the next load on its
/// thread.** The expression reader counts the levels it is inside on
/// its thread, so a count one refusal left raised would refuse the
/// next document short of the bound: after each way a load can refuse
/// from inside a tree, a document at the bound loads on the same
/// thread.
#[test]
fn a_refused_load_leaves_nothing_for_the_next_on_its_thread() {
    on_the_smallest_stack(|| {
        let (r, _, _) = deep_document(BOUND);
        let [(_, at_the_bound), _] = both_saves(&r);
        for (label, text) in refused_bodies() {
            assert!(
                editor_core::persist::load(&text, Tol::witness()).is_err(),
                "{label} refuses"
            );
            editor_core::persist::load(&at_the_bound, Tol::witness())
                .unwrap_or_else(|err| panic!("after {label}, the bound loads: {err}"));
        }
    });
}

/// **Loads on several threads keep their own count**: four threads on
/// the smallest stack interleave refusals past the bound with loads at
/// it, and every load at the bound succeeds.
#[test]
fn loads_on_several_threads_keep_their_own_count() {
    let (r, _, _) = deep_document(BOUND);
    let [(_, at_the_bound), _] = both_saves(&r);
    let at_the_bound = Arc::new(at_the_bound);
    let refused = Arc::new(refused_bodies());
    let threads: Vec<_> = (0..4)
        .map(|t| {
            let (at_the_bound, refused) = (Arc::clone(&at_the_bound), Arc::clone(&refused));
            std::thread::Builder::new()
                .stack_size(test_utils::own_thread::WASM_STACK)
                .spawn(move || {
                    for round in 0..8 {
                        let (label, text) = &refused[(round + t) % refused.len()];
                        assert!(
                            editor_core::persist::load(text, Tol::witness()).is_err(),
                            "{label} refuses"
                        );
                        editor_core::persist::load(&at_the_bound, Tol::witness())
                            .unwrap_or_else(|err| panic!("thread {t}, round {round}: {err}"));
                    }
                })
                .expect("a loader thread starts")
        })
        .collect();
    for thread in threads {
        thread.join().expect("every loader thread loads the bound");
    }
}

/// Whether `v` is an expression on the wire: a tag object whose payload
/// has the expression wire's shape all the way down.
fn is_expression(v: &Value) -> bool {
    let Some((tag, inner)) = tagged(v) else {
        return false;
    };
    match tag {
        "Literal" => has_exactly(inner, &["value", "dim", "unit"]),
        "Param" => has_exactly(inner, &["name", "dim"]),
        "Count" => inner.is_i64(),
        "Neg" | "Sin" | "Cos" | "Tan" | "CountToScalar" => is_expression(inner),
        "Add" | "Sub" | "Mul" | "Div" | "Atan2" | "Min" | "Max" => inner
            .as_array()
            .is_some_and(|operands| operands.len() == 2 && operands.iter().all(is_expression)),
        _ => false,
    }
}

/// Whether `v` is a measurement on the wire.
fn is_measurement(v: &Value) -> bool {
    let Some((tag, inner)) = tagged(v) else {
        return false;
    };
    match tag {
        "Primitive" => tagged(inner).is_some(),
        "Value" => is_expression(inner),
        "Neg" => is_measurement(inner),
        "Add" | "Sub" | "Mul" | "Div" | "Min" | "Max" => inner
            .as_array()
            .is_some_and(|operands| operands.len() == 2 && operands.iter().all(is_measurement)),
        _ => false,
    }
}

/// The tag and payload of a one-key object.
fn tagged(v: &Value) -> Option<(&str, &Value)> {
    let object = v.as_object()?;
    let mut entries = object.iter();
    let (tag, inner) = entries.next()?;
    entries.next().is_none().then_some((tag.as_str(), inner))
}

/// Whether `v` is an object with exactly these keys.
fn has_exactly(v: &Value, keys: &[&str]) -> bool {
    v.as_object()
        .is_some_and(|o| o.len() == keys.len() && keys.iter().all(|k| o.contains_key(*k)))
}

/// The most JSON brackets enclosing an expression's root in `body`,
/// counting a measurement's root as one of those enclosing the
/// expression its value leaf holds, and how many roots were found.
///
/// An expression nested `n` levels spans at most `2n` brackets from its
/// root's own (a binary operator two, a unary one one, a leaf two), and
/// a measurement `2n` below its root's, so the deepest a body holding
/// every expression at the bound nests is this plus `2 · BOUND`.
fn envelope(body: &Value) -> (usize, usize) {
    let (mut deepest, mut roots) = (0, 0);
    // Each value with the bracket depth its own bracket opens at.
    let mut work = vec![(body, 1usize)];
    while let Some((v, depth)) = work.pop() {
        if is_expression(v) {
            (deepest, roots) = (deepest.max(depth - 1), roots + 1);
            continue;
        }
        if is_measurement(v) {
            (deepest, roots) = (deepest.max(depth), roots + 1);
            continue;
        }
        let children: Vec<&Value> = match v {
            Value::Object(o) => o.values().collect(),
            Value::Array(a) => a.iter().collect(),
            _ => Vec::new(),
        };
        for child in children {
            if child.is_object() || child.is_array() {
                work.push((child, depth + 1));
            }
        }
    }
    (deepest, roots)
}

/// **The load door reads exactly as deep as the deepest body a save
/// writes.** Every expression and measurement root in every corpus
/// document (which holds every node kind and every edit kind, welded to
/// their vocabularies by `m4_pr8_corpus::vocabulary_coverage_is_total`),
/// and in this suite's deepest document, saved from its snapshot and
/// from its edit log, is found by its wire shape; the most brackets
/// enclosing one, plus the `2 · BOUND` an expression at the bound spans,
/// is the load door's limit. A slot added deeper than every one before
/// it moves the left side, and this row fails until the limit follows.
#[test]
fn the_load_doors_limit_is_the_deepest_body_a_save_writes() {
    let mut recorders: Vec<(String, Recorder)> = crate::corpus::documents()
        .into_iter()
        .map(|d| {
            let r = Recorder {
                doc: d.doc,
                edits: d.edits,
            };
            (d.name.to_string(), r)
        })
        .collect();
    recorders.push((
        "this suite's deepest document".to_string(),
        deep_document(1).0,
    ));
    let (mut deepest, mut at) = (0, String::new());
    for (name, r) in &recorders {
        for (label, text) in both_saves(r) {
            let parsed: Value = serde_json::from_str(body(&text)).expect("a saved body is JSON");
            let (enclosing, roots) = envelope(&parsed);
            assert!(roots > 0, "{name}'s {label} holds an expression");
            if enclosing > deepest {
                (deepest, at) = (enclosing, format!("{name}'s {label}"));
            }
        }
    }
    assert_eq!(
        deepest + 2 * BOUND,
        editor_core::test_support::BODY_NESTING,
        "the deepest expression position ({deepest} brackets, in {at}) and the load door's \
         limit disagree"
    );
}
