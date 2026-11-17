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
//! [`DimensionError::NestedTooDeep`], the load door included wherever
//! in a file the expression sits, the deepest place a save writes one
//! too; and text or a file nested far past it refuses typed rather than
//! exhausting the stack, a file nested deeper than any save writes at
//! the load door's scan, before any reader descends.
//!
//! The bound is written here as the number the refusal carries, and the
//! one-past row reads it back out of the refusal, so the two cannot
//! drift apart silently.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    DimensionError, DocEdit, EditError, EvalOptions, ExprPath, Formula, LoopProgram, Node,
    NodeResult, ParseError, PersistError, ProfileProgram, ProgramArcData, ProgramStep,
    ProgramTarget, RecipeNodeId, SlotId, ValuePayload, VarEnv, content_pin, eval, eval_count,
    parse_formula, unparse,
};
use fixture::{Recorder, len, run, scl};
use geom_core::{Interval, Tol};
use serde_json::Value;
use test_utils::own_thread::on_the_smallest_stack;

/// The deepest an expression nests, in levels: the refusal names it.
const BOUND: usize = 128;

/// `metres` as a length `levels` deep: `metres + 0 + … + 0`, nested to
/// the left as the text door nests a sum, so it evaluates to `metres`
/// exactly.
fn deep_length(metres: f64, levels: usize) -> Formula {
    (1..levels).fold(len(metres), |e, _| Formula::add(e, len(0.0)).unwrap())
}

/// `levels - 1` negations over `metres`.
fn negations(metres: f64, levels: usize) -> Formula {
    (1..levels).fold(len(metres), |e, _| Formula::neg(e).unwrap())
}

/// A count `levels` deep: `start + 1 + … + 1`.
fn deep_count(start: i64, levels: usize) -> Formula {
    (1..levels).fold(Formula::count(start), |e, _| {
        Formula::add(e, Formula::count(1)).unwrap()
    })
}

/// A sum `levels` deep: `sum_levels` nested sums, the first term
/// holding `metres` as an expression nested the rest of the way, so it
/// evaluates to `metres + 0.25 · (sum_levels - 1)`.
fn deep_sum(metres: f64, levels: usize, sum_levels: usize) -> Formula {
    let first = deep_length(metres, levels - sum_levels + 1);
    (1..sum_levels).fold(first, |m, _| Formula::add(m, len(0.25)).unwrap())
}

/// The nesting refusal a door raised, with the bound it names.
fn refused_bound(error: &DimensionError) -> usize {
    match error {
        DimensionError::NestedTooDeep { bound } => *bound,
        other => panic!("expected the nesting refusal, got {other:?}"),
    }
}

/// A recorded document holding an expression `levels` deep in each kind
/// of place one sits: a profile step's target, an extrude's distance, a
/// measurement, and an index's count (the deepest place a save writes
/// one, `persist::nesting::ENVELOPE`).
fn deep_document(levels: usize) -> (Recorder, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    let profile = r.insert(Node::Profile(ProfileProgram {
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
        on: None,
    }));
    let extrude = r.insert(Node::Extrude {
        profile: profile.into(),
        distance: deep_length(0.5, levels),
    });
    let measure = r.insert(Node::Assertion(editor_core::Claim::Value {
        value: deep_sum(0.5, levels, levels.div_ceil(2)),
        bound: len(0.0),
        relation: editor_core::AssertionRelation::AtLeast,
    }));
    // The count's deepest leaf reads a variable, which spells two
    // brackets where an integer spells one.
    r.push(DocEdit::DeclareVar {
        name: editor_core::VarName::from_static("n"),
        def: editor_core::VarDecl::Free(editor_core::FreeVar::Count { value: 1 }),
    });
    let n = r.doc.var_named("n").expect("declared");
    r.push(DocEdit::DeclareVar {
        name: editor_core::VarName::from_static("k"),
        def: editor_core::VarDecl::Index {
            count: (1..levels).fold(Formula::var(n, editor_core::Dimension::Count), |e, _| {
                Formula::add(e, Formula::count(1)).unwrap()
            }),
            within: None,
        },
    });
    (r, extrude, measure)
}

/// A document whose one expression of note is an extrude's distance,
/// `distance`, and the extrude.
fn extrude_document(distance: Formula) -> (Recorder, RecipeNodeId) {
    let mut r = Recorder::new();
    let profile = r.insert(Node::Profile(fixture::desc(vec![fixture::square(
        0.0, 0.0, 0.5,
    )])));
    let extrude = r.insert(Node::Extrude {
        profile: profile.into(),
        distance,
    });
    (r, extrude)
}

/// **A copy placed by a long pose chain**: the extrude of
/// [`extrude_document`] placed into its own space by a plane mate of its
/// start cap against its end cap's plane spun about a lateral edge by an
/// angle `levels` deep, and then slid along its normal `slides`
/// times — written inline in the mate's `to`.
fn pose_chain_document(levels: usize, slides: usize) -> Recorder {
    use editor_core::pose::{PoseDef, PoseRole, RoleRead};
    use editor_core::{Mate, Operand};
    let (mut r, extrude) = extrude_document(len(0.5));
    let cap = |end| {
        fixture::plane(Operand::select(
            extrude,
            vec![fixture::fname(extrude, editor_core::RoleSeg::Cap(end))],
        ))
    };
    let top = cap(editor_core::CapEnd::End);
    let normal = || RoleRead::through(top.clone(), PoseRole::Normal);
    let by = (1..levels).fold(fixture::ang(0.0), |e, _| {
        Formula::add(e, fixture::ang(0.0)).unwrap()
    });
    let edge = fixture::axis(Operand::select(
        extrude,
        vec![fixture::ename(
            extrude,
            editor_core::RoleSeg::LateralEdge(fixture::vpiece(&r.doc, extrude, 0, 0)),
        )],
    ));
    let mut to = Operand::Pose(Box::new(PoseDef::Spin {
        pose: RoleRead::plain(top.clone()),
        about: RoleRead::plain(edge),
        by,
    }));
    for _ in 0..slides {
        to = Operand::Pose(Box::new(PoseDef::Slide {
            pose: RoleRead::plain(to),
            along: normal(),
            by: len(0.0),
        }));
    }
    let space = r
        .doc
        .operand_space(&extrude.into())
        .expect("the extrude's space");
    let placed = editor_core::Placed::of(
        &r.doc,
        vec![Mate {
            on: cap(editor_core::CapEnd::Start),
            to: fixture::flip(to),
            orient: editor_core::Orientation::IDENTITY,
        }],
        Vec::new(),
    );
    // A recorded edit applies or the recorder panics: the chain at the
    // bound is admitted.
    r.push(DocEdit::place(
        editor_core::BodyRead::plain(Operand::from(extrude)),
        Operand::Var(space),
        placed.bundle(),
    ));
    r
}

/// How many slides [`pose_chain_document`] nests its spin under. The
/// chain nests nothing in the file: the edit door mints each pose a
/// variable of its own, and a save writes each beside the rest. What
/// bounds it here is the edit door's stack, which no door guards
/// (`work/doctail/a-pose-chain-written-deeper-than-the-load-doors-envelope-does-not-load.md`).
const POSE_SLIDES: usize = 8;

/// **A long pose chain evaluates on the smallest stack.** It does not
/// yet: a chain of [`LONG_POSE_SLIDES`] slides, authored and saved on a
/// large stack, loads on the smallest one (the save writes each pose a
/// variable of its own) and then overflows it in evaluation, a process
/// abort rather than a typed refusal. So do 128, 512 and 2048. The row
/// is ignored because the abort takes the test process with it; the fix
/// (`work/doctail/a-pose-chain-written-deeper-than-the-load-doors-envelope-does-not-load.md`)
/// un-ignores it. Run it with
/// `cargo nextest run -p editor-core --test all --run-ignored only -E
/// 'test(=expr_nesting_bound::a_long_pose_chain_evaluates_on_the_smallest_stack)'`.
#[test]
#[ignore = "aborts the process: a pose chain's depth has no bound; see work/doctail/a-pose-chain-written-deeper-than-the-load-doors-envelope-does-not-load.md"]
fn a_long_pose_chain_evaluates_on_the_smallest_stack() {
    let text = std::thread::Builder::new()
        .stack_size(1 << 30)
        .spawn(|| saved(&pose_chain_document(1, LONG_POSE_SLIDES)))
        .expect("a thread with a large stack starts")
        .join()
        .unwrap_or_else(|payload| std::panic::resume_unwind(payload));
    on_the_smallest_stack(move || {
        let doc = editor_core::persist::load(&text, Tol::witness())
            .unwrap_or_else(|err| panic!("the chain loads on the smallest stack: {err}"));
        let placed = *doc.ids().last().expect("the chain's document inserts");
        let ev = run(&doc, &EvalOptions::default());
        assert!(
            matches!(ev.result(placed), Some(NodeResult::Ok(_))),
            "the copy the chain places evaluates"
        );
    });
}

/// How many slides [`a_long_pose_chain_evaluates_on_the_smallest_stack`]
/// nests its spin under: the fewest the evaluation overflow was seen at.
const LONG_POSE_SLIDES: usize = 64;

/// [`extrude_document`] over a distance that is an expression,
/// saved: a literal saves as a free variable, so the distance is a
/// negation, which saves as the definition of the variable the slot
/// reads, [`SAVED_LEVELS`] deep.
fn saved_extrude() -> String {
    saved(&extrude_document(Formula::neg(len(-0.5)).expect("a negated length")).0)
}

/// How many levels the expression [`saved_extrude`] saves nests: a
/// negation over the variable its literal saved as.
const SAVED_LEVELS: usize = 2;

/// `r`'s document, saved.
fn saved(r: &Recorder) -> String {
    editor_core::persist::save(&r.doc, Tol::witness())
        .unwrap_or_else(|err| panic!("the document saves: {err}"))
}

/// The value an assertion compared.
fn measured(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> f64 {
    match ev.result(id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Assertion(
                editor_core::AssertionVerdict::Holds { measured, .. }
                | editor_core::AssertionVerdict::Violated { measured, .. },
            ) => *measured,
            other => panic!("node {id:?} is a {}", other.kind_name()),
        },
        _ => panic!("node {id:?} did not evaluate"),
    }
}

/// The saved `text`'s body: everything from the brace that opens it.
fn body(text: &str) -> &str {
    &text[text.find('{').expect("a saved body is an object")..]
}

/// `text` with its first variable definition wrapped in `levels` more
/// negations, spelled as the wire spells one.
fn wrap_definition(text: &str, levels: usize) -> String {
    let key = "\"Defined\": ";
    let at = text.find(key).expect("the save holds a definition") + key.len();
    // The definition is the object that opens here; its end is the
    // brace that closes it.
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

/// The body line of the tag of the negation [`saved_extrude`] saved, in
/// `text`: where the reader stands when it declines that negation's
/// operand, before reading into it. A save lays the tag out on the line
/// after the one the definition opens on, where the wrapping negations
/// are written.
fn saved_negation_line(text: &str) -> usize {
    let definition = text
        .find("\"Defined\": ")
        .expect("the save holds a definition");
    let body_start = text.find('{').expect("a saved body is an object");
    text[body_start..definition].matches('\n').count() + 2
}

/// **At the bound, every door takes the expression**, on the wasm32
/// stack: the constructors mint it, both evaluators value it, the
/// derived walks and `unparse` read it, the text door reads it back,
/// and a document holding it in its deepest slots applies, evaluates,
/// pins, saves and loads back to the same document, as does a copy
/// placed by a pose chain [`POSE_SLIDES`] deep whose spin's angle is at
/// the bound. The deeper of those saves nests exactly as deep as
/// `persist::nesting` says an expression at the bound saves.
#[test]
fn every_door_takes_an_expression_at_the_bound_on_the_smallest_stack() {
    on_the_smallest_stack(|| {
        let env = VarEnv::<f64>::default();
        for (label, e, value) in [
            ("a left-nested sum", deep_length(0.5, BOUND), 0.5),
            (
                "a sum over a negative literal",
                deep_length(-0.5, BOUND),
                -0.5,
            ),
            ("a chain of negations", negations(0.5, BOUND), -0.5),
        ] {
            assert_eq!(
                eval(&Clone::clone(&e), &env),
                Ok(value),
                "{label} evaluates at f64"
            );
            assert!(
                eval(&Clone::clone(&e), &VarEnv::<Interval>::default()).is_ok(),
                "{label} evaluates at Interval"
            );
            let copy = e.clone();
            assert!(copy.bit_eq(&e), "{label} clones bit for bit");
            assert!(format!("{e:?}").contains("Quantity"), "{label} prints");
            let text = unparse(&e, &|_| None);
            let back = parse_formula(&text, &BTreeMap::new())
                .unwrap_or_else(|err| panic!("{label} reads back through the text door: {err}"));
            assert!(back.bit_eq(&e), "{label} round-trips through its text");
        }
        assert_eq!(
            eval_count(&Clone::clone(&deep_count(1, BOUND)), &env),
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
            let e = parse_formula(&text, &BTreeMap::new())
                .unwrap_or_else(|err| panic!("{label} nested to the bound parse: {err}"));
            assert!(
                eval_count(&Clone::clone(&e), &env).is_ok(),
                "{label} evaluate"
            );
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

        // Saved, it loads back to the document it was; and so does a
        // copy placed by a long pose chain, its spin's angle at the
        // expression's bound.
        let chain = pose_chain_document(BOUND, POSE_SLIDES);
        let chain_pin = content_pin(&chain.doc, Tol::witness()).expect("the document pins");
        let mut deepest = 0;
        for (label, r, pin) in [
            ("deep document", &r, pin),
            ("pose chain", &chain, chain_pin),
        ] {
            let text = saved(r);
            deepest = deepest.max(editor_core::test_support::bracket_depth(&text));
            let loaded = editor_core::persist::load(&text, Tol::witness())
                .unwrap_or_else(|err| panic!("the {label} loads back: {err}"));
            assert_eq!(
                content_pin(&loaded, Tol::witness()).expect("the loaded document pins"),
                pin,
                "the {label} loads back to the document it saved"
            );
        }
        assert_eq!(
            deepest,
            editor_core::test_support::EXPR_BODY_NESTING,
            "the deepest save nests exactly as deep as an expression at the bound saves"
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
        let error = Formula::add(at.clone(), len(0.0)).expect_err("a sum one past the bound");
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
            ("neg", Formula::neg(at.clone()).err()),
            ("mul", Formula::mul(at.clone(), scl(1.0)).err()),
            ("min", Formula::min(len(0.0), at.clone()).err()),
            (
                "sin",
                Formula::sin((1..BOUND).fold(fixture::ang(0.5), |e, _| Formula::neg(e).unwrap()))
                    .err(),
            ),
            (
                "count_to_scalar",
                Formula::count_to_scalar(deep_count(1, BOUND)).err(),
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
            match parse_formula(&text, &BTreeMap::new()) {
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
            expr: Formula::neg(len(0.5)).unwrap(),
        };
        match editor_core::apply(&r.doc, &edit, Tol::witness()) {
            Err(EditError::Dimension(error)) => assert_eq!(refused_bound(&error), BOUND),
            other => panic!("the edit door refuses the deeper tree, got {other:?}"),
        }

        // The load door: the saved distance's definition wrapped in one
        // more negation.
        match editor_core::persist::load(&wrap_definition(&saved(&r), 1), Tol::witness()) {
            Err(PersistError::Dimension { error, .. }) => assert_eq!(refused_bound(&error), BOUND),
            other => panic!("the load door refuses the deeper tree, got {other:?}"),
        }
    });
}

/// **Text or a file nested far past the bound refuses typed on the
/// smallest stack** rather than exhausting it: the text door reads
/// signs, calls and terms nested a hundred thousand deep to a refusal,
/// and the load door refuses a definition negated nearly twice the
/// bound deep before reading into it, and one negated a hundred
/// thousand deep before reading the body at all.
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
            match parse_formula(&text, &BTreeMap::new()) {
                Err(ParseError::Dimension { error, .. }) => {
                    assert_eq!(refused_bound(&error), BOUND, "{label}");
                }
                other => panic!("{label} nested far past the bound refuses typed, got {other:?}"),
            }
        }

        let text = saved_extrude();
        // Within the scan's limit, which a chain of negations packs one
        // level into each JSON level of.
        let nearly_twice = 2 * BOUND - 8;
        match editor_core::persist::load(&wrap_definition(&text, nearly_twice), Tol::witness()) {
            Err(PersistError::Dimension { error, .. }) => assert_eq!(refused_bound(&error), BOUND),
            other => panic!(
                "a definition negated nearly twice the bound deep refuses typed, got {other:?}"
            ),
        }
        match editor_core::persist::load(&wrap_definition(&text, FAR), Tol::witness()) {
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
        let read = parse_formula(&lone, &BTreeMap::new()).expect("a bracketed literal reads");
        assert!(
            read.bit_eq(&Formula::count(1)),
            "the brackets add no node: {read:?}"
        );
        let wrapped = |terms: usize| {
            let sum = vec!["1"; terms].join("+");
            format!("{}{sum}{}", "(".repeat(BOUND), ")".repeat(BOUND))
        };
        let at = parse_formula(&wrapped(BOUND), &BTreeMap::new());
        assert!(at.is_ok(), "a sum at the bound, bracketed: {at:?}");
        match parse_formula(&wrapped(BOUND + 1), &BTreeMap::new()) {
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
                let text = unparse(&e, &|_| None);
                let back = parse_formula(&text, &BTreeMap::new()).unwrap_or_else(|err| {
                    panic!("{label} {levels} deep reads back from {text:.40}…: {err}")
                });
                assert!(
                    back.bit_eq(&e),
                    "{label} {levels} deep reads back as itself"
                );
                assert_eq!(unparse(&back, &|_| None), text, "{label} {levels} deep");
            }
        }
    });
}

/// **An assertion's value shares the bound, at every split of its
/// levels between sums and the expression a term holds**: at the bound
/// it saves and loads back on the smallest stack, and one level more
/// refuses at the constructor.
#[test]
fn an_assertion_value_shares_the_bound_at_every_split() {
    on_the_smallest_stack(|| {
        for measure_levels in [1, 2, 3, BOUND / 2, BOUND - 1, BOUND] {
            let at = deep_sum(0.5, BOUND, measure_levels);
            let (mut r, _) = extrude_document(len(0.5));
            r.insert(Node::Assertion(editor_core::Claim::Value {
                value: at.clone(),
                bound: len(0.0),
                relation: editor_core::AssertionRelation::AtLeast,
            }));
            let pin = content_pin(&r.doc, Tol::witness()).expect("the document pins");
            let loaded = editor_core::persist::load(&saved(&r), Tol::witness())
                .unwrap_or_else(|err| panic!("split {measure_levels}: {err}"));
            assert_eq!(
                content_pin(&loaded, Tol::witness()).expect("pins"),
                pin,
                "split {measure_levels}: the save loads back"
            );
            let one_more = Formula::add(at, len(0.0)).expect_err("one level more refuses");
            assert_eq!(refused_bound(&one_more), BOUND, "split {measure_levels}");
        }
    });
}

/// **The load door reads an expression's child at the bound and refuses
/// the next one before reading into it.** A saved definition negated to
/// the bound loads; one negation more refuses with the constructors'
/// own refusal at the child it declined — the saved negation's operand,
/// at that negation's tag — before the reader reads into it, rather
/// than after reading the whole tree and rebuilding it.
#[test]
fn the_load_door_refuses_the_child_past_the_bound_before_reading_it() {
    on_the_smallest_stack(|| {
        let text = saved_extrude();
        let at_the_bound = wrap_definition(&text, BOUND - SAVED_LEVELS);
        let loaded = editor_core::persist::load(&at_the_bound, Tol::witness())
            .unwrap_or_else(|err| panic!("a definition negated to the bound loads: {err}"));
        drop(loaded);
        let past = wrap_definition(&text, BOUND - SAVED_LEVELS + 1);
        match editor_core::persist::load(&past, Tol::witness()) {
            Err(PersistError::Dimension { line, error, .. }) => {
                assert_eq!(refused_bound(&error), BOUND);
                assert_eq!(
                    line,
                    saved_negation_line(&past),
                    "refused at the child it declined, not after reading the tree"
                );
            }
            other => panic!("one negation more refuses typed, got {other:?}"),
        }
    });
}

/// The object that opens after the first `key` that follows the first
/// `after` in `text`, as the byte range it spans.
fn object_after(text: &str, after: &str, key: &str) -> core::ops::Range<usize> {
    let from = text.find(after).expect("the save holds the place");
    let at = from + text[from..].find(key).expect("the place holds the key") + key.len();
    let mut depth = 0usize;
    for (i, c) in text[at..].char_indices() {
        match c {
            '{' | '[' => depth += 1,
            '}' | ']' => {
                depth -= 1;
                if depth == 0 {
                    return at..at + i + 1;
                }
            }
            _ => {}
        }
    }
    panic!("the object after {key} closes")
}

/// `text` with the index count [`deep_document`] saves summed with
/// itself `times` times over, each sum one expression level deeper and
/// spelled in two brackets, as many as a level ever costs.
fn deepen_the_index_count(text: &str, times: usize) -> String {
    let span = object_after(text, "\"Index\": ", "\"count\": ");
    let count = &text[span.clone()];
    let deeper = (0..times).fold(count.to_string(), |e, _| {
        format!("{{\"Add\": [{e}, {count}]}}")
    });
    format!("{}{deeper}{}", &text[..span.start], &text[span.end..])
}

/// **One past the bound in the deepest place a save writes an
/// expression, the load door still refuses typed**, naming the bound:
/// an index's count at the bound, summed once more, nests the file
/// exactly as deep as the load door's scan reads, so the expression's
/// own reader is reached and refuses it with
/// [`DimensionError::NestedTooDeep`], as it does at every shallower
/// place. One level more is deeper than any save writes anywhere, and
/// that is the scan's to refuse.
#[test]
fn one_past_the_bound_at_the_index_count_refuses_typed() {
    on_the_smallest_stack(|| {
        let (r, _, _) = deep_document(BOUND);
        let text = saved(&r);
        let one_past = deepen_the_index_count(&text, 1);
        assert_eq!(
            editor_core::test_support::bracket_depth(&one_past),
            editor_core::test_support::BODY_NESTING,
            "one past the bound at the index count nests as deep as the scan reads"
        );
        match editor_core::persist::load(&one_past, Tol::witness()) {
            Err(PersistError::Dimension { error, .. }) => assert_eq!(refused_bound(&error), BOUND),
            other => panic!("one past the bound at the index count refuses typed, got {other:?}"),
        }
        match editor_core::persist::load(&deepen_the_index_count(&text, 2), Tol::witness()) {
            Err(err @ PersistError::Parse { .. }) => assert!(
                err.to_string()
                    .contains("deeper than any saved document does"),
                "{err}"
            ),
            other => panic!("deeper than any save refuses at the scan, got {other:?}"),
        }
    });
}

/// A body the load door refuses in each way a refusal can leave the
/// expression reader: at the bound's own guard, far past it, on a
/// dimension and a variant it does not know deep inside a tree, and on
/// a body cut off mid-tree.
fn refused_bodies() -> Vec<(&'static str, String)> {
    let text = saved_extrude();
    let wrapped = |levels| wrap_definition(&text, levels);
    let deep = wrapped(BOUND - 8);
    // The tree's own leaf: the first dimension after the definition
    // opens.
    let definition = deep
        .find("\"Defined\": ")
        .expect("the save holds a definition");
    let dim = definition
        + deep[definition..]
            .find("\"dim\": \"")
            .expect("the leaf's dimension");
    let (before, after) = deep.split_at(dim);
    vec![
        ("one past the bound", wrapped(BOUND - SAVED_LEVELS + 1)),
        ("far past the bound", wrapped(2 * BOUND)),
        (
            "an unknown dimension deep in the tree",
            format!(
                "{before}{}",
                after.replacen("\"dim\": \"", "\"dim\": \"zz", 1)
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
        let at_the_bound = saved(&r);
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
    let at_the_bound = saved(&r);
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
    if v.as_str() == Some("Turn") {
        return true;
    }
    let Some((tag, inner)) = tagged(v) else {
        return false;
    };
    match tag {
        "Quantity" => {
            has_exactly(inner, &["value", "dim", "unit"])
                || has_exactly(inner, &["value", "dim", "unit", "distribution"])
        }
        "Ratio" => has_exactly(inner, &["num", "den"]),
        "Var" => has_exactly(inner, &["var", "dim"]),
        "Name" => has_exactly(inner, &["name", "dim"]),
        "Fresh" => has_exactly(inner, &["index", "dim"]),
        "Integer" => inner.is_i64(),
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
        // An authored value leaf is a formula; a stored one, the
        // variable its value lowered to.
        "Value" => is_expression(inner) || has_exactly(inner, &["var", "dim"]),
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

/// **The load door's envelope is the deepest body a save writes.** Every expression and measurement root in every corpus
/// document (which holds every node kind, welded to the vocabulary by
/// `m4_pr8_corpus::vocabulary_coverage_is_total`), in this suite's
/// deepest document and in its long pose chain, saved, is found by its
/// wire shape; the most brackets enclosing one, plus the `2 · BOUND` an
/// expression at the bound spans, is the load door's envelope, and its
/// limit is one expression level past that. A slot added deeper than
/// every one before it moves the left side, and this row fails until
/// the envelope follows.
#[test]
fn the_load_doors_limit_is_the_deepest_body_a_save_writes() {
    let mut documents: Vec<(String, editor_core::ProfileDoc)> = crate::corpus::documents()
        .into_iter()
        .map(|d| (d.name.to_string(), d.doc))
        .collect();
    documents.push((
        "this suite's deepest document".to_string(),
        deep_document(1).0.doc,
    ));
    documents.push((
        "this suite's long pose chain".to_string(),
        pose_chain_document(1, POSE_SLIDES).doc,
    ));
    let (mut deepest, mut at, mut found) = (0, String::new(), 0);
    for (name, doc) in &documents {
        let text = editor_core::persist::save(doc, Tol::witness())
            .unwrap_or_else(|err| panic!("{name} saves: {err}"));
        let parsed: Value = serde_json::from_str(body(&text)).expect("a saved body is JSON");
        let (enclosing, roots) = envelope(&parsed);
        found += roots;
        if enclosing > deepest {
            (deepest, at) = (enclosing, name.clone());
        }
    }
    // A document whose slots all read free variables holds no
    // expression, so the count is the corpus's, not each document's.
    assert!(found > 0, "the saves hold an expression");
    assert_eq!(
        deepest + 2 * BOUND,
        editor_core::test_support::EXPR_BODY_NESTING,
        "the deepest expression position ({deepest} brackets, in {at}) and the load door's \
         envelope disagree"
    );
    // The scan reads one expression level past it, so an expression one
    // past the bound there meets its own refusal
    // (`one_past_the_bound_at_the_index_count_refuses_typed`).
    assert_eq!(
        editor_core::test_support::BODY_NESTING,
        editor_core::test_support::EXPR_BODY_NESTING
            .max(editor_core::test_support::META_BODY_NESTING)
            + 2,
        "the load door reads one level past the deepest save"
    );
}
