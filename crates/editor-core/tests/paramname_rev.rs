//! Review lane `paramname-rev` probes (PR #3164, frozen head 8d1fb1a2a).
//!
//! Each row falsifies one claim of the parameter-name door by execution.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use crate::wire::doctored;

use std::collections::{BTreeMap, HashMap};

use editor_core::persist::{load, save};
use editor_core::range::{RangeField, RangeRefusal, RangeSeed, derive};
use editor_core::{
    Dimension, DocEdit, DocParam, Expr, LoopProgram, Node, ParamName, ParseError, PersistError,
    ProfileDoc, ProfileProgram, RecipeNodeId, SlotId, parse_expr,
};
use proptest::prelude::*;

use fixture::{Recorder, tol, xy_frame};

/// The spec's oracle: the parser, asked with an EMPTY table, reads the
/// whole text as one unresolved reference to exactly that text; and,
/// asked with the name declared, builds `Param(name)`.
fn parser_reads_back(text: &str) -> bool {
    let empty = matches!(
        parse_expr(text, &BTreeMap::new()),
        Err(ParseError::UnknownParam { pos: 0, ref name }) if name == text
    );
    if let Ok(name) = ParamName::new(text) {
        let table = BTreeMap::from([(name.clone(), Dimension::Scalar)]);
        let full = parse_expr(text, &table) == Ok(Expr::param(name, Dimension::Scalar));
        assert_eq!(empty, full, "{text:?}: the two readings of the oracle disagree");
    }
    empty
}

fn agrees(text: &str) {
    assert_eq!(
        ParamName::new(text).is_ok(),
        parser_reads_back(text),
        "{text:?}: the constructor and the parser disagree"
    );
}

/// Claim 1, table form.
#[test]
fn rev_admissible_iff_parser_reads_back_table() {
    let long = "a".repeat(100_000);
    let long_bad = format!("{}-", "a".repeat(10_000));
    let rows: Vec<&str> = vec![
        "", " ", "\t", "\n", "   ", "\u{a0}", "\u{2003}", "\u{200b}", "\u{feff}",
        "1", "1a", "2width", "1e3", "1e", "0x", "_", "__", "_1", "a_", "_a_b_",
        "δ", "Δx", "ñ", "日本", "ß", "Ⅷ", "x²", "x٣", "٣x", "a\u{301}", "\u{301}a", "é",
        "e\u{0345}", "sin", "cos", "atan2", "scalar", "pi", "mm", "rad", "deg", "e", "E",
        "inf", "nan", "NaN", "Infinity", &long, &long_bad, "a\0", "\0", "a\0b", "width\n",
        "\nwidth", "width\r\n", " width", "width ", "a b", "a+b", "a-b", "a.b", "a,b",
        "a(", "a)", "(a)", "-a", "a#", "a:b", "query:certified-range:1:distance",
        "query_certified_range_1_distance", "a·b", "·", "sin(x)", "pi rad", "a\u{2028}",
    ];
    for text in rows {
        agrees(text);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(4096))]
    /// Claim 1, property form over a hostile alphabet.
    #[test]
    fn rev_admissible_iff_parser_reads_back_prop(
        text in "[a-zA-Z0-9_ \\t\\n+\\-*/(),.#:·δ\u{301}\u{a0}\u{0}é²٣]{0,8}"
    ) {
        agrees(&text);
    }

    #[test]
    fn rev_admissible_iff_parser_reads_back_any_unicode(text in "\\PC{0,6}") {
        agrees(&text);
    }
}

/// Claim 2: a name inside a PERSISTED EXPRESSION arrives from outside
/// and must refuse at the token, as the `params` key does.
#[test]
fn rev_an_expression_param_ref_refuses_an_inadmissible_name_at_load() {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: ParamName::literal("depth"),
        value: DocParam::continuous(Dimension::Length, 1.0),
    });
    let f = r.insert(xy_frame());
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: f,
        loops: vec![square()],
        ids: Vec::new(),
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: Expr::param(ParamName::literal("depth"), Dimension::Length),
    });
    let text = save(&r.doc, &[], tol()).expect("saves");
    load(&text, tol()).expect("loads");
    let mut hits = 0;
    let corrupt = doctored(&text, |wire| {
        fn walk(v: &mut serde_json::Value, hits: &mut usize) {
            match v {
                serde_json::Value::Object(m) => {
                    if let Some(serde_json::Value::Object(inner)) = m.get_mut("Param")
                        && inner.get("name") == Some(&serde_json::json!("depth"))
                    {
                        inner.insert("name".into(), serde_json::json!("1 2"));
                        *hits += 1;
                    }
                    for (_, c) in m.iter_mut() {
                        walk(c, hits);
                    }
                }
                serde_json::Value::Array(a) => a.iter_mut().for_each(|c| walk(c, hits)),
                _ => {}
            }
        }
        walk(&mut wire["snapshot"], &mut hits);
    });
    assert!(hits > 0, "aimed at an expression's param ref: {text}");
    let fault = ParamName::new("1 2").expect_err("refused");
    match load(&corrupt, tol()) {
        Err(PersistError::Unreadable { detail, .. }) => {
            assert!(detail.contains(&fault.to_string()), "{detail}");
        }
        other => panic!("an expression's name must refuse at the token, got {other:?}"),
    }
}

fn square() -> LoopProgram {
    LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]).expect("square")
}

fn profile_slab() -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    let f = r.insert(xy_frame());
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: f,
        loops: vec![square()],
        ids: Vec::new(),
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: Expr::literal(1.0, Dimension::Length).expect("finite"),
    });
    (r.doc, p)
}

/// Claim 3 (deviation: the synthetic range name). A profile step
/// argument is a continuous, non-structural slot a range query can be
/// aimed at; its label is `loop L step S · point x`, which carries a
/// digit after `_` (fine) and a MIDDLE DOT (outside the alphabet).
/// `synthetic_name`'s doc says no slot label is outside the alphabet.
#[test]
fn rev_a_profile_slot_range_query_is_not_refused_for_its_own_name() {
    let (doc, p) = profile_slab();
    let node = doc.node(p).expect("the profile");
    let slot = node
        .slots()
        .into_iter()
        .find(|s| matches!(s, SlotId::Profile { .. }))
        .expect("a profile step slot");
    assert!(node.expr(slot).is_some(), "the profile carries the slot: {}", slot.label());
    assert!(!slot.is_structural());
    match derive(&doc, &RangeField::Slot { node: p, slot }, RangeSeed::symmetric(0.1), tol()) {
        Ok(_) => {}
        Err(RangeRefusal::SyntheticNameUnspellable { fault }) => panic!(
            "a range over the profile slot {:?} refuses because the query cannot name its own \
             axis: {fault}",
            slot.label()
        ),
        Err(other) => panic!("refused for another reason: {other:?}"),
    }
}

/// Claim 3: a user who declares the query's synthetic spelling. The
/// guard must refuse typed (not silently widen the user's parameter).
#[test]
fn rev_a_user_declared_synthetic_name_is_refused_typed_not_shadowed() {
    let mut r = Recorder::new();
    let f = r.insert(xy_frame());
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: f,
        loops: vec![square()],
        ids: Vec::new(),
    }));
    let e = r.insert(Node::Extrude {
        profile: p,
        distance: Expr::literal(1.0, Dimension::Length).expect("finite"),
    });
    let spelled = format!("query_certified_range_{}_distance", e.0);
    let users = ParamName::new(spelled.clone()).expect("a user can type it");
    r.push(DocEdit::SetDocParam {
        name: users.clone(),
        value: DocParam::continuous(Dimension::Length, 7.0),
    });
    let got = derive(
        &r.doc,
        &RangeField::Slot {
            node: e,
            slot: SlotId::Distance,
        },
        RangeSeed::symmetric(0.1),
        tol(),
    );
    assert_eq!(got.err(), Some(RangeRefusal::SyntheticNameTaken { param: users }));
}

/// Claim 5: `Borrow<str>` agrees with `Hash`/`Eq`/`Ord`.
#[test]
fn rev_borrow_str_lookups_agree() {
    let names = ["width", "Width", "w", "δ", "_", "a1", "a_1", "z", "B", "é"];
    let bt: BTreeMap<ParamName, usize> = names
        .iter()
        .enumerate()
        .map(|(i, n)| (ParamName::new(*n).expect("admissible"), i))
        .collect();
    let hm: HashMap<ParamName, usize> = bt.iter().map(|(k, v)| (k.clone(), *v)).collect();
    for (i, n) in names.iter().enumerate() {
        assert_eq!(bt.get(*n), Some(&i), "BTreeMap by &str: {n}");
        assert_eq!(hm.get(*n), Some(&i), "HashMap by &str: {n}");
    }
    let order: Vec<&str> = bt.keys().map(ParamName::as_str).collect();
    let mut sorted: Vec<&str> = names.to_vec();
    sorted.sort_unstable();
    assert_eq!(order, sorted, "Ord agrees with str's");
    assert_eq!(bt.get("nope"), None);
}

/// Claim 4: `'static` does not keep runtime text out of `literal` — a
/// leaked runtime string type-checks and reaches the panicking door.
#[test]
#[should_panic(expected = "parameter name \"1 2\"")]
fn rev_runtime_text_reaches_literal_through_a_leak() {
    let runtime: String = ["1", " ", "2"].concat();
    let leaked: &'static str = runtime.leak();
    let _ = ParamName::literal(leaked);
}
