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
//! a producer of any depth, a self-referential one included, is read
//! only to its own bound and refused typed with
//! [`MetaError::ProducerTooDeep`]; and a body or a file holding one past
//! the bound, or far past it, refuses rather than exhausting the stack.

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

/// The deepest a producer `to_value` reads nests, in nested
/// `serialize` calls: the refusal names it.
const MAX_PRODUCER_NESTING: usize = 512;

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

/// A producer that reads itself as its own newtype, forever.
#[derive(Debug)]
struct Loop;

impl serde::Serialize for Loop {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_newtype_struct("Loop", self)
    }
}

impl<'de> serde::Deserialize<'de> for Loop {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        struct Again;
        impl<'de> serde::de::Visitor<'de> for Again {
            type Value = Loop;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a loop")
            }
            fn visit_newtype_struct<D: serde::Deserializer<'de>>(
                self,
                de: D,
            ) -> Result<Loop, D::Error> {
                <Loop as serde::Deserialize>::deserialize(de)
            }
        }
        de.deserialize_newtype_struct("Loop", Again)
    }
}

/// A producer that serializes as its count of options around an
/// integer, holding nothing.
struct Somes(usize);

impl serde::Serialize for Somes {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        if self.0 == 0 {
            return ser.serialize_i64(0);
        }
        ser.serialize_some(&Somes(self.0 - 1))
    }
}

/// The derived producer a linked list is.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Link(Option<Box<Link>>);

/// A chain of `links` links.
fn chain(links: usize) -> Link {
    (0..links).fold(Link(None), |next, _| Link(Some(Box::new(next))))
}

/// `link` dropped one link at a time, so its depth costs the stack
/// nothing.
fn unchain(link: Link) {
    let mut next = link.0;
    while let Some(mut held) = next {
        next = held.0.take();
    }
}

/// A newtype, read through without a level of its own.
struct Newtype<T>(T);

impl<T: serde::Serialize> serde::Serialize for Newtype<T> {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_newtype_struct("Newtype", &self.0)
    }
}

/// A producer that serializes as its count of lists around an integer,
/// each level read through an option, a newtype and an option: the most
/// a producer may wrap a level in and still build one at the bound.
struct Wrapped(usize);

impl serde::Serialize for Wrapped {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        if self.0 == 0 {
            return Some(Newtype(Some(0_i64))).serialize(ser);
        }
        Some(Newtype(Some(Deeper(self.0)))).serialize(ser)
    }
}

/// A one-item list of [`Wrapped`] one level shallower.
struct Deeper(usize);

impl serde::Serialize for Deeper {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        let mut seq = ser.serialize_seq(Some(1))?;
        seq.serialize_element(&Wrapped(self.0 - 1))?;
        seq.end()
    }
}

/// The bound a producer refusal names, or a panic naming what came
/// back.
fn refused_read<T: std::fmt::Debug>(result: Result<T, MetaError>) -> usize {
    match result {
        Err(MetaError::ProducerTooDeep { bound }) => bound,
        other => panic!("expected ProducerTooDeep, got {other:?}"),
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

/// **`to_value` and `from_value` read a producer of any depth only so
/// far**, on the smallest stack: one that never ends (a newtype that
/// holds itself), a chain of options [`FAR`] deep, and a derived linked
/// list [`FAR`] long each refuse typed at [`MAX_PRODUCER_NESTING`],
/// though none builds a value deeper than a leaf; read back from a
/// leaf, the newtype that holds itself and the linked list, which a
/// present value makes read itself again, refuse the same way. A producer that wraps every level
/// in three options and newtypes builds a value at the bound; one more
/// wrapper refuses.
#[test]
fn a_producer_of_any_depth_refuses_typed_on_the_smallest_stack() {
    let link = chain(FAR);
    let link = on_the_smallest_stack(move || {
        assert_eq!(
            refused_read(to_value(&Loop)),
            MAX_PRODUCER_NESTING,
            "a newtype that holds itself"
        );
        assert_eq!(
            refused_read(to_value(&Somes(FAR))),
            MAX_PRODUCER_NESTING,
            "{FAR} options"
        );
        assert_eq!(
            to_value(&Somes(MAX_PRODUCER_NESTING - 1)),
            Ok(MetaValue::Int(0)),
            "options to the bound read through to their integer"
        );
        assert_eq!(
            refused_read(to_value(&link)),
            MAX_PRODUCER_NESTING,
            "a linked list {FAR} long"
        );
        assert_eq!(
            refused_read(from_value::<Loop>(&MetaValue::Int(0))),
            MAX_PRODUCER_NESTING,
            "a newtype that holds itself, read back"
        );
        assert_eq!(
            refused_read(from_value::<Link>(&MetaValue::Int(0))),
            MAX_PRODUCER_NESTING,
            "a linked list read from a present value"
        );
        assert!(
            matches!(from_value::<Link>(&MetaValue::Null), Ok(Link(None))),
            "a linked list read from absence ends"
        );
        let line = MetaError::ProducerTooDeep {
            bound: MAX_PRODUCER_NESTING,
        }
        .to_string();
        let problems = test_utils::refusal::problems("ProducerTooDeep", &line, &[], false);
        assert!(problems.is_empty(), "{problems:#?}");
        assert_eq!(
            to_value(&Wrapped(BOUND - 1)).map(|v| v.nesting()),
            Ok(BOUND),
            "a producer wrapping each level three times builds at the bound"
        );
        assert_eq!(
            refused_read(to_value(&Some(Wrapped(BOUND - 1)))),
            MAX_PRODUCER_NESTING,
            "one more wrapper"
        );
        assert_eq!(
            refused_bound(to_value(&Wrapped(BOUND))),
            BOUND,
            "a level more is past the value's own bound first"
        );
        link
    });
    unchain(link);
}

/// **The deserializer reads a body of any depth only one level past the
/// bound**, on the smallest stack, with serde_json's own recursion limit
/// off as the load door runs it.
#[test]
fn the_deserializer_stops_a_body_of_any_depth_on_the_smallest_stack() {
    on_the_smallest_stack(|| {
        const DEEPER: usize = 100_000;
        for (label, open, close) in [
            ("lists", "{\"List\": [", "]}"),
            ("maps", "{\"Map\": {\"k\": ", "}}"),
        ] {
            let text = format!(
                "{}{{\"Int\": 1}}{}",
                open.repeat(DEEPER),
                close.repeat(DEEPER)
            );
            let mut de = serde_json::Deserializer::from_str(&text);
            de.disable_recursion_limit();
            let read = <MetaValue as serde::Deserialize>::deserialize(&mut de)
                .expect_err("a body {DEEPER} deep refuses");
            assert!(
                read.to_string().contains(PROBLEM),
                "{DEEPER} {label} refuse naming the bound: {read}"
            );
        }
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
            deepest = deepest.max(editor_core::test_support::bracket_depth(&text));
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
