//! **A metadata value nests to its bound and refuses one past it, at
//! every door that builds one, on the smallest stack a door runs on.**
//!
//! Every row runs on a thread whose stack is the wasm32 build's (one
//! mebibyte). A value at the bound passes every door that builds,
//! walks, keeps or drops one: the constructors, `to_value` and
//! `from_value`, `Clone`, equality, `Debug`, the D2 walk, the edit door,
//! the content pin, save from a snapshot and from an edit log, and load.
//! One past it, every door that builds one refuses typed with
//! [`MetaError::NestedTooDeep`]; a value ten thousand levels deep, the
//! depth that once overflowed the edit door, cannot be built at all;
//! and a file holding one past the bound, or far past it, refuses
//! rather than exhausting the stack.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;

use editor_core::{
    DocEdit, EntityKind, MetaError, MetaValue, Node, PersistError, ProfileDoc, RoleSeg, StableName,
    content_pin, from_value, to_value,
};
use fixture::{Recorder, len, xy_frame};
use geom_core::Tol;
use serde::ser::{SerializeSeq as _, Serializer};
use test_utils::own_thread::on_the_smallest_stack;

/// The deepest a value nests, in levels: the refusal names it.
const BOUND: usize = 128;

/// The problem a value past the bound is refused with, recourse aside.
const PROBLEM: &str = "the metadata value nests deeper than 128 levels of lists and maps";

/// The key the value is set under.
const KEY: &str = "tool.example/deep";

/// The depth the row's evidence overflowed the edit door at.
const FAR: usize = 10_000;

/// The bound a refusal names, or a panic naming what came back.
fn refused_bound<T: std::fmt::Debug>(result: Result<T, MetaError>) -> usize {
    match result {
        Err(MetaError::NestedTooDeep { bound }) => bound,
        other => panic!("expected NestedTooDeep, got {other:?}"),
    }
}

/// `leaf` under `levels` lists, or the refusal of the list that would
/// pass the bound; the refusal stops the fold, so a far depth is never
/// built.
fn under_lists(leaf: MetaValue, levels: usize) -> Result<MetaValue, MetaError> {
    (0..levels).try_fold(leaf, |v, _| MetaValue::list(vec![v]))
}

/// The D7 convention's map, `{"v": 1, "deep": deep}`.
fn versioned(deep: MetaValue) -> Result<MetaValue, MetaError> {
    MetaValue::map(BTreeMap::from([
        ("v".to_owned(), MetaValue::Int(1)),
        ("deep".to_owned(), deep),
    ]))
}

/// A versioned value nested exactly to the bound, an empty list at its
/// bottom (the most brackets a value at the bound saves as).
fn at_the_bound() -> MetaValue {
    let empty = MetaValue::list(Vec::new()).unwrap();
    versioned(under_lists(empty, BOUND - 2).unwrap()).unwrap()
}

/// A producer value that serializes as its count of lists around an
/// integer, holding nothing: its depth costs the producer no memory, so
/// only the door stops it.
struct Deep(usize);

impl serde::Serialize for Deep {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        if self.0 == 0 {
            return ser.serialize_i64(0);
        }
        let mut seq = ser.serialize_seq(Some(1))?;
        seq.serialize_element(&Deep(self.0 - 1))?;
        seq.end()
    }
}

/// A document of one extrude, the body name of its output, and the
/// recorder holding it.
fn extrude_body() -> (Recorder, StableName) {
    let mut r = Recorder::new();
    let plane = r.insert(xy_frame());
    let profile = r.insert(Node::Profile(fixture::desc(
        plane,
        vec![fixture::square(0.0, 0.0, 0.5)],
    )));
    let extrude = r.insert(Node::Extrude {
        profile,
        distance: len(0.5),
    });
    let body = StableName {
        kind: EntityKind::Body,
        node: extrude,
        path: vec![RoleSeg::OutputBody],
    };
    (r, body)
}

/// `value` set on the extrude's body.
fn set_meta(body: &StableName, value: MetaValue) -> DocEdit<editor_core::ProfileProgram> {
    DocEdit::SetAppearanceMeta {
        name: body.clone(),
        key: KEY.to_owned(),
        value,
    }
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

/// How deep `text` nests, in JSON brackets outside strings.
fn bracket_depth(text: &str) -> usize {
    let (mut depth, mut deepest) = (0usize, 0usize);
    let (mut in_string, mut escaped) = (false, false);
    for byte in text.bytes() {
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

/// **One past the bound, every door that builds a value refuses typed**,
/// in a sentence that meets the refusal standard: both constructors,
/// `to_value`, and the deserializer. A value [`FAR`] levels deep, built
/// the way the row's evidence built one, stops at the bound, and
/// `to_value` stops a producer [`FAR`] levels deep before reading it.
#[test]
fn one_past_the_bound_refuses_typed_at_every_door_that_builds_one() {
    on_the_smallest_stack(|| {
        let below = under_lists(MetaValue::Int(1), BOUND - 1).unwrap();
        assert_eq!(below.nesting(), BOUND, "the fixture sits at the bound");
        let error = MetaValue::list(vec![below.clone()]).expect_err("a list one past");
        let line = error.to_string();
        assert!(line.starts_with(PROBLEM), "{line}");
        let problems = test_utils::refusal::problems("NestedTooDeep", &line, &[], false);
        assert!(problems.is_empty(), "{problems:#?}");
        assert_eq!(
            refused_bound(MetaValue::map(BTreeMap::from([("k".to_owned(), below)]))),
            BOUND,
            "a map one past refuses"
        );
        assert_eq!(
            refused_bound(under_lists(MetaValue::Int(1), FAR)),
            BOUND,
            "a value {FAR} deep stops at the bound"
        );
        assert_eq!(
            to_value(&Deep(BOUND - 1)).map(|v| v.nesting()),
            Ok(BOUND),
            "to_value builds a producer at the bound"
        );
        assert_eq!(
            refused_bound(to_value(&Deep(BOUND))),
            BOUND,
            "to_value, one past"
        );
        assert_eq!(
            refused_bound(to_value(&Deep(FAR))),
            BOUND,
            "to_value, {FAR} deep"
        );
        let text = format!(
            "{}{{\"Int\": 1}}{}",
            "{\"List\": [".repeat(BOUND),
            "]}".repeat(BOUND)
        );
        let mut de = serde_json::Deserializer::from_str(&text);
        de.disable_recursion_limit();
        let read = <MetaValue as serde::Deserialize>::deserialize(&mut de)
            .expect_err("the deserializer refuses one past");
        assert!(
            read.to_string().contains(PROBLEM) && !read.to_string().contains("Recourse:"),
            "the deserializer states the problem and leaves the recourse to its door: {read}"
        );
    });
}

/// **At the bound, every door takes the value**: it walks, clones,
/// compares, prints and reads back as a producer type; the edit door
/// applies it; the document pins, saves both ways and loads back to the
/// document it was. The deepest save nests exactly as deep as
/// `persist::nesting` says a value at the bound saves, which is within
/// the load door's limit.
#[test]
fn every_door_takes_a_value_at_the_bound_on_the_smallest_stack() {
    on_the_smallest_stack(|| {
        let value = at_the_bound();
        assert_eq!(value.nesting(), BOUND);
        assert_eq!(value.first_non_finite(), None, "the D2 walk reads it");
        let copy = value.clone();
        assert_eq!(copy, value, "it clones and compares");
        assert!(format!("{value:?}").contains("List"), "it prints");
        let back: serde_json::Value = from_value(&value).expect("from_value reads it");
        assert_eq!(
            to_value(&back),
            Ok(value.clone()),
            "it round-trips as a producer"
        );

        let (mut r, body) = extrude_body();
        r.push(set_meta(&body, value.clone()));
        let pin = content_pin(&r.doc, Tol::witness()).expect("the document pins");
        let mut deepest = 0;
        for (label, text) in both_saves(&r) {
            deepest = deepest.max(bracket_depth(&text));
            let loaded = editor_core::persist::load(&text, Tol::witness())
                .unwrap_or_else(|err| panic!("the {label} loads back: {err}"));
            assert_eq!(
                content_pin(&loaded.doc, Tol::witness()).expect("the loaded document pins"),
                pin,
                "the {label} loads back to the document it saved"
            );
            assert_eq!(
                loaded
                    .doc
                    .appearance_of(&body)
                    .and_then(|record| record.metadata.get(KEY)),
                Some(&value),
                "the {label} loads the value back"
            );
        }
        assert_eq!(
            deepest,
            editor_core::test_support::META_BODY_NESTING,
            "the deepest save of a value at the bound nests as deep as the load door is told"
        );
    });
}

/// **A file holding a value one past the bound, or far past it, refuses
/// on the smallest stack**: one past, the value's own refusal; far past,
/// the load door's scan, before any reader descends.
#[test]
fn a_file_nested_past_the_bound_refuses_on_the_smallest_stack() {
    on_the_smallest_stack(|| {
        let (mut r, body) = extrude_body();
        r.push(set_meta(&body, at_the_bound()));
        // The empty list at the value's bottom as a save spells it, and
        // the same list holding `levels` more.
        let empty = "\"List\": []";
        let nest = |levels: usize| {
            format!(
                "\"List\": [{}{{{empty}}}{}]",
                "{\"List\": [".repeat(levels - 1),
                "]}".repeat(levels - 1)
            )
        };
        for (label, text) in both_saves(&r) {
            assert_eq!(
                text.matches(empty).count(),
                1,
                "the {label} holds one empty list"
            );
            let one_past = text.replace(empty, &nest(1));
            match editor_core::persist::load(&one_past, Tol::witness()) {
                Err(error @ PersistError::Unreadable { .. }) => {
                    let line = error.to_string();
                    assert!(
                        line.contains(PROBLEM),
                        "the {label} one past names the bound: {line}"
                    );
                    assert!(
                        !line.contains("Recourse:"),
                        "the {label} one past leaves the recourse to the load door: {line}"
                    );
                }
                other => panic!("the {label} one past refuses as unreadable: {other:?}"),
            }
            let far = text.replace(empty, &nest(FAR));
            match editor_core::persist::load(&far, Tol::witness()) {
                Err(PersistError::Parse { message, .. }) => assert!(
                    message.contains("deeper than any saved document does"),
                    "the {label} {FAR} deep refuses at the scan: {message}"
                ),
                other => panic!("the {label} {FAR} deep refuses at the scan: {other:?}"),
            }
        }
    });
}
