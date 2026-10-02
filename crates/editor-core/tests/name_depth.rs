//! **A stable name nested past every stack, through every door, on the
//! smallest stack a door runs on.**
//!
//! A name nests one whole name per derivation level: a chain of K
//! patterns, each over the one before, names its faces K levels deep.
//! Every door that holds such a name — evaluation and the drop of its
//! result, a name's `Debug`, the edit door, the split's re-map, save,
//! load and the content pin — runs here on a thread whose stack is the
//! wasm32 build's (one mebibyte), at a depth a walk that recursed once
//! per level could not reach on it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use editor_core::{
    CapEnd, DocEdit, EntityKind, EvalOptions, MetaValue, NameRef, Node, PatternKind, PersistError,
    ProfileDoc, RecipeNodeId, RoleSeg, StableName, all_faces, canonical_bytes, content_pin, load,
    remap_name, save,
};
use fixture::{in_copy, insert, len, on_frame, run, square};
use geom_core::Tol;
use test_utils::own_thread::on_the_smallest_stack;

/// A square extrude, then `k` patterns each over the one before, one
/// copy each: the top pattern names its faces `k` levels deep. Returns
/// the document, the extrude and the patterns in order.
fn chain(label: &str, k: usize) -> (ProfileDoc, RecipeNodeId, Vec<RecipeNodeId>) {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 1.0)],
    );
    let (mut doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let mut patterns = Vec::with_capacity(k);
    let mut input = extrude;
    for _ in 0..k {
        let (next, pattern) = insert(
            doc,
            Node::Pattern {
                input,
                count: editor_core::Expr::count(1),
                kind: PatternKind::Linear {
                    direction: [fixture::scl(1.0), fixture::scl(0.0), fixture::scl(0.0)],
                    spacing: len(2.0),
                },
            },
        );
        doc = next;
        patterns.push(pattern);
        input = pattern;
    }
    (doc, extrude, patterns)
}

/// Past the depth a recursive `Debug` of one name reached on this
/// stack in a dev build (about 700 levels), and far past the reader's
/// recursion limit a saved name had to fit (41 levels for a bare name).
const PATTERNS: usize = 1_000;

/// One chain, through every door that holds its names: evaluated, a
/// name printed, the evaluation dropped; then a fillet authoring the
/// deepest edge name through the edit door, and the document saved,
/// loaded, re-saved and pinned.
#[test]
fn a_chain_of_patterns_names_past_every_stack_through_every_door_on_the_smallest_stack() {
    on_the_smallest_stack(|| {
        let tol = Tol::witness();
        let (doc, extrude, patterns) = chain("name-depth", PATTERNS);
        let top = *patterns.last().expect("a pattern");
        let ev = run(&doc, &EvalOptions::default());
        let faces = all_faces(&ev, top);
        assert_eq!(
            faces.len(),
            6,
            "the top pattern's one copy has a block's faces"
        );
        let shown = format!("{:?}", faces[0]);
        assert_eq!(
            shown.matches("Instance").count(),
            PATTERNS,
            "the name nests one copy per pattern, and Debug renders each"
        );
        drop(faces);
        drop(ev);
        // An edge of the top copy, named through every pattern: the
        // name a pick on it would author.
        let rim = fixture::rim_edge(
            extrude,
            editor_core::CapEnd::End,
            fixture::piece(&doc, extrude, 0, 0),
        );
        let deep = patterns.iter().fold(rim, |n, &p| in_copy(p, 0, n));
        // Through the edit door.
        let (doc, _) = insert(doc, Node::fillet(top, len(0.1), vec![deep.clone()]));
        let text = save(&doc, &[], tol).expect("the document saves");
        let loaded = load(&text, tol).expect("and loads back");
        assert_eq!(loaded.doc, doc, "the loaded document is the saved one");
        assert_eq!(
            canonical_bytes(&loaded.doc, tol).unwrap(),
            canonical_bytes(&doc, tol).unwrap(),
            "and pins the same bytes"
        );
        assert_eq!(
            content_pin(&loaded.doc, tol).unwrap(),
            content_pin(&doc, tol).unwrap()
        );
        let resaved = save(&loaded.doc, &[], tol).expect("the loaded document saves");
        assert_eq!(resaved, text, "and saves the same text");
        drop((doc, loaded, deep));
    });
}

#[test]
fn a_split_remaps_a_name_past_every_stack_on_the_smallest_stack() {
    on_the_smallest_stack(|| {
        let depth = 20_000;
        let leaf = StableName {
            kind: editor_core::EntityKind::Face,
            node: RecipeNodeId(1),
            path: vec![editor_core::RoleSeg::Cap(editor_core::CapEnd::End)],
        };
        let name = (0..depth).fold(leaf, |n, level| in_copy(RecipeNodeId(2 + level % 3), 0, n));
        let map = (1..=4)
            .map(|n| (RecipeNodeId(n), RecipeNodeId(n + 100)))
            .collect();
        let moved = remap_name(&name, &map, &Default::default()).expect("every id is mapped");
        let expect = (0..depth).fold(
            StableName {
                kind: editor_core::EntityKind::Face,
                node: RecipeNodeId(101),
                path: vec![editor_core::RoleSeg::Cap(editor_core::CapEnd::End)],
            },
            |n, level| in_copy(RecipeNodeId(102 + level % 3), 0, n),
        );
        assert!(
            moved == expect,
            "every level moves to the other document's ids"
        );
    });
}

/// A square extrude, and the name of one of its rim edges and of its
/// end cap.
fn block(label: &str) -> (ProfileDoc, RecipeNodeId, StableName, StableName) {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 1.0)],
    );
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let rim = fixture::rim_edge(extrude, CapEnd::End, fixture::piece(&doc, extrude, 0, 0));
    let cap = StableName {
        kind: EntityKind::Face,
        node: extrude,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    (doc, extrude, rim, cap)
}

/// `name` under `levels` part instances, as the top of a part chain
/// that long names what its bottom part holds.
fn in_parts(name: StableName, levels: usize) -> StableName {
    (0..levels).fold(name, |of, _| StableName {
        kind: of.kind,
        node: of.node,
        path: vec![RoleSeg::InPart {
            of: NameRef::new(of),
        }],
    })
}

/// The block with a fillet over its rim edge and a metadata record on
/// its cap, each name `levels` part instances deep, through the edit
/// door; saved, and the saved text loaded back.
fn names_in_parts(levels: usize) -> (ProfileDoc, String) {
    let tol = Tol::witness();
    let (doc, extrude, rim, cap) = block("names-in-parts");
    let (doc, _) = insert(
        doc,
        Node::fillet(extrude, len(0.1), vec![in_parts(rim, levels)]),
    );
    let (doc, _) = fixture::step(
        doc,
        DocEdit::SetAppearanceMeta {
            name: in_parts(cap, levels),
            key: "probe".to_owned(),
            value: MetaValue::Map([("v".to_owned(), MetaValue::Int(1))].into()),
        },
    );
    let text = save(&doc, &[], tol).expect("the document saves");
    (doc, text)
}

/// **Names nested past the load door's limit save and load, on the
/// smallest stack, and the file stays linear in their depth**: a
/// fillet's edge and a metadata record's face each named 1 024 part
/// instances deep, as a part chain at `MAX_DEPTH` names what its bottom
/// part holds, round-trip through save and load, byte for byte and pin
/// for pin. Twice as deep writes a file under twice as long: the pretty
/// layout stops indenting where only a name nests.
#[test]
fn names_a_thousand_part_instances_deep_save_and_load_on_the_smallest_stack() {
    on_the_smallest_stack(|| {
        let tol = Tol::witness();
        let (doc, text) = names_in_parts(1_024);
        let loaded = load(&text, tol).expect("the document loads back");
        assert!(loaded.doc == doc, "the loaded document is the saved one");
        assert_eq!(
            save(&loaded.doc, &[], tol).expect("the loaded document saves"),
            text,
            "and saves the same text"
        );
        assert_eq!(
            content_pin(&loaded.doc, tol).unwrap(),
            content_pin(&doc, tol).unwrap(),
            "and pins the same"
        );
        let (_, twice) = names_in_parts(2_048);
        assert!(
            twice.len() * 10 < text.len() * 21,
            "twice as deep, {} bytes against {}: the file grows linearly",
            twice.len(),
            text.len()
        );
        drop((doc, loaded));
    });
}

/// `value` wrapped `levels` times in `wrap`, a JSON object or array
/// around its one member.
fn nested(
    value: serde_json::Value,
    levels: usize,
    wrap: fn(serde_json::Value) -> serde_json::Value,
) -> serde_json::Value {
    (0..levels).fold(value, |v, _| wrap(v))
}

/// The first object under `at` holding every key of `keys`, mutably.
fn holding<'v>(
    at: &'v mut serde_json::Value,
    keys: &[&str],
) -> Option<&'v mut serde_json::Map<String, serde_json::Value>> {
    let here =
        matches!(&*at, serde_json::Value::Object(o) if keys.iter().all(|k| o.contains_key(*k)));
    match at {
        serde_json::Value::Object(o) => {
            if here {
                Some(o)
            } else {
                o.values_mut().find_map(|v| holding(v, keys))
            }
        }
        serde_json::Value::Array(a) => a.iter_mut().find_map(|v| holding(v, keys)),
        _ => None,
    }
}

/// **What nests outside a name is bounded as before, whatever keys sit
/// beside it**: a pattern's count, which sits beside the pattern's
/// `kind`; a metadata value under a user's `"kind"` key; and a metadata
/// object spelled like a name, its `"path"` an array. Each nested past
/// the load door's limit is refused by the scan, on the smallest stack,
/// before a reader descends into it.
#[test]
fn what_nests_outside_a_name_is_refused_past_the_limit_whatever_keys_sit_beside_it() {
    let tol = Tol::witness();
    let (doc, extrude, _, cap) = block("outside-names");
    let (doc, _) = insert(
        doc,
        Node::Pattern {
            input: extrude,
            count: editor_core::Expr::count(2),
            kind: PatternKind::Linear {
                direction: [fixture::scl(1.0), fixture::scl(0.0), fixture::scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, _) = fixture::step(
        doc,
        DocEdit::SetAppearanceMeta {
            name: cap,
            key: "probe".to_owned(),
            value: MetaValue::Map(
                [
                    ("v".to_owned(), MetaValue::Int(1)),
                    ("kind".to_owned(), MetaValue::Str("mine".to_owned())),
                ]
                .into(),
            ),
        },
    );
    let text = save(&doc, &[], tol).expect("the document saves");
    let (header, body) = text.split_once('\n').expect("a header line");
    let past = editor_core::test_support::BODY_NESTING + 10;
    let edited = |edit: &dyn Fn(&mut serde_json::Value)| {
        let mut value: serde_json::Value = serde_json::from_str(body).expect("a body is JSON");
        edit(&mut value);
        format!(
            "{header}\n{}\n",
            serde_json::to_string(&value).expect("it writes")
        )
    };
    let neg = |v| serde_json::json!({ "Neg": v });
    let list = |v| serde_json::json!({ "List": [v] });
    let array = |v| serde_json::json!([v]);
    let cases = [
        (
            "a pattern's count beside its kind",
            edited(&|v| {
                let pattern = holding(v, &["count"]).expect("the pattern's fields");
                let count = pattern.remove("count").expect("a count");
                pattern.insert("count".to_owned(), nested(count, past, neg));
            }),
        ),
        (
            "a metadata value under a user's kind",
            edited(&|v| {
                let meta = holding(v, &["v", "kind"]).expect("the metadata map");
                meta.insert(
                    "kind".to_owned(),
                    nested(serde_json::json!({ "Int": 1 }), past, list),
                );
            }),
        ),
        (
            "a metadata object spelled like a name",
            edited(&|v| {
                let meta = holding(v, &["v", "kind"]).expect("the metadata map");
                meta.insert("node".to_owned(), serde_json::json!(1));
                meta.insert(
                    "path".to_owned(),
                    nested(serde_json::json!([]), past, array),
                );
            }),
        ),
    ];
    for (what, text) in cases {
        let refused = on_the_smallest_stack(move || load(&text, Tol::witness()).err());
        assert!(
            matches!(
                &refused,
                Some(PersistError::Parse { message, .. }) if message.starts_with("the body nests deeper than")
            ),
            "{what}: refused by the scan, got {refused:?}"
        );
    }
}

/// **A malformed name nested deep in a document refuses where it is
/// written**, in the words the derived form gives there: a misspelled
/// variant at the bottom of a name 1 024 part instances deep, and an
/// unknown field in a name nested a few levels down, each at its own
/// line and column of the body.
#[test]
fn a_malformed_name_nested_deep_in_a_document_refuses_where_it_is_written() {
    let place = |text: &str, token: &str| {
        let body = text.split_once('\n').expect("a header line").1;
        let at = body.find(token).expect("the token") + token.len();
        let line = 1 + body[..at].matches('\n').count();
        let column = at - body[..at].rfind('\n').map_or(0, |nl| nl + 1);
        (line, column)
    };
    for levels in [3, 1_024] {
        let (_, text) = names_in_parts(levels);
        let bad = text.replacen("\"RimEdge\"", "\"RimEdgf\"", 1);
        let (line, column) = place(&bad, "\"RimEdgf\"");
        let refused = on_the_smallest_stack(move || load(&bad, Tol::witness()).err());
        match refused {
            Some(PersistError::Unreadable {
                line: l,
                column: c,
                detail,
            }) => {
                assert_eq!(
                    (l, c),
                    (line, column),
                    "{levels} deep: where the variant is written"
                );
                assert!(
                    detail.starts_with("unknown variant `RimEdgf`")
                        && detail.ends_with(&format!(" at line {line} column {column}")),
                    "{levels} deep: the derived form's words, placed: {detail}"
                );
            }
            other => panic!("{levels} deep: unreadable, got {other:?}"),
        }
        let bad = text.replacen("\"InPart\": {", "\"InPart\": {\n\"x\": 0,", 2);
        let refused = load(&bad, Tol::witness()).err();
        assert!(
            matches!(&refused, Some(PersistError::Unreadable { detail, .. }) if detail.starts_with("unknown field `x`, expected `of`")),
            "{levels} deep: an unknown field in a nested segment, got {refused:?}"
        );
    }
}
