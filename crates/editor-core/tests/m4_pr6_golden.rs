//! M4 PR 6 review MINOR-3 — the committed GOLDEN v1 fixture.
//!
//! D6.1's round-trip row proves save∘load is a fixpoint, but a
//! fixpoint is BLIND to format drift: rename a field and save/load
//! stay self-consistent while every existing v1 file breaks. This row
//! pins the frozen wire shape to CHECKED-IN BYTES
//! (`tests/golden/v1_golden.cad`): the fixture document must save to
//! exactly those bytes, and the bytes must load. Any change to either
//! is a format change and demands a ratified schema bump + migration
//! step — re-bless ONLY then (run with `M4_PR6_BLESS_GOLDEN=1` to
//! regenerate, and say so loudly in the PR).
//!
//! ε note: the golden snapshot PINS ε = 1e-9 via `SetTolerance` (a
//! committed byte stream cannot record the ambient ε — it varies by
//! CI row). Full `load` therefore ε-reconciles: under an ambient of
//! 1e-9 it succeeds; under any other row it refuses
//! `ToleranceConflict` — which still proves the bytes parsed,
//! validated, and replayed, because that door is the LAST in the load
//! sequence. Both outcomes are asserted exactly.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    Attr, Dimension, DocEdit, DocParam, EntityKind, Expr, MetaValue, Node, ParamName, PersistError,
    ProfileDesc, ProfileDoc, Rgba8, RoleSeg, StableName, WitnessDatum, apply, load, save,
};
use fixture::{desc, len};

const GOLDEN: &str = include_str!("golden/v1_golden.cad");
const GOLDEN_PATH: &str = "tests/golden/v1_golden.cad";

/// The golden document: deterministic (no ambient reads — ε pinned by
/// the SetTolerance edit) and shape-covering: params, an arc-bearing
/// profile, a param-expression slot, witness bytes, appearance attrs
/// + D7 metadata (floats, -0.0, bytes, list, nesting).
fn golden() -> (ProfileDoc, Vec<DocEdit<ProfileDesc>>) {
    let mut doc = ProfileDoc::empty();
    let push = |d: &ProfileDoc, e: &DocEdit<ProfileDesc>| apply(d, e).expect("golden edit").doc;
    doc = push(&doc, &DocEdit::SetTolerance { eps: 1e-9 });
    doc = push(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("depth"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 0.75,
            },
        },
    );
    let mut d = desc(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)]],
    );
    d.0.loops[0].vertices[1].bulge = 0.25; // an arc segment
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Profile(d),
        },
    );
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Extrude {
                profile: editor_core::RecipeNodeId(0),
                distance: Expr::param(ParamName::new("depth"), Dimension::Length),
            },
        },
    );
    doc = push(
        &doc,
        &DocEdit::ReWitness {
            node: editor_core::RecipeNodeId(0),
            witness: WitnessDatum {
                schema: 1,
                bytes: vec![0x00, 0x7f, 0x80, 0xff],
            },
        },
    );
    let body = StableName {
        kind: EntityKind::Body,
        node: editor_core::RecipeNodeId(1),
        path: vec![RoleSeg::OutputBody],
    };
    doc = push(
        &doc,
        &DocEdit::SetAppearance {
            name: body.clone(),
            attr: Attr::Color(Rgba8::opaque(10, 20, 30)),
        },
    );
    let mut m = std::collections::BTreeMap::new();
    m.insert("v".into(), MetaValue::Int(1));
    m.insert("neg_zero".into(), MetaValue::Float(-0.0));
    m.insert("blob".into(), MetaValue::Bytes(vec![0xde, 0xad]));
    m.insert(
        "list".into(),
        MetaValue::List(vec![MetaValue::Null, MetaValue::Bool(true)]),
    );
    doc = push(
        &doc,
        &DocEdit::SetAppearanceMeta {
            name: body,
            key: "tool.example/pin".into(),
            value: MetaValue::Map(m),
        },
    );
    // The committed EDIT LOG half: one trailing continuous edit.
    let edits = vec![DocEdit::SetParam {
        node: editor_core::RecipeNodeId(1),
        slot: editor_core::SlotId::Distance,
        expr: len(0.5),
    }];
    (doc, edits)
}

#[test]
fn golden_v1_bytes_are_frozen() {
    let (doc, edits) = golden();
    let text = save(&doc, &edits).expect("golden saves");
    if std::env::var("M4_PR6_BLESS_GOLDEN").is_ok() {
        std::fs::write(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_PATH),
            &text,
        )
        .expect("bless writes");
        panic!(
            "golden re-blessed — commit the file WITH its ratified schema change, then rerun without the env var"
        );
    }
    assert_eq!(
        text, GOLDEN,
        "schema-v1 wire bytes drifted from the committed golden — this is a FORMAT \
         CHANGE: it needs a ratified schema bump + migration step, never a re-bless in passing"
    );
}

#[test]
fn golden_v1_bytes_load() {
    let ambient = geom_core::Tolerance::get().eps;
    match load(GOLDEN) {
        Ok(loaded) => {
            // Only reachable when the process ε IS the golden's 1e-9.
            assert_eq!(ambient.to_bits(), 1e-9f64.to_bits());
            let (doc, edits) = golden();
            assert!(loaded.snapshot.bit_eq(&doc), "golden snapshot drifted");
            assert_eq!(loaded.edits, edits, "golden edit log drifted");
        }
        Err(PersistError::ToleranceConflict { process, document }) => {
            // The ε door is the LAST load door, so this outcome still
            // proves the golden bytes parse, validate, and replay.
            assert_eq!(document.to_bits(), 1e-9f64.to_bits());
            assert_eq!(process.to_bits(), ambient.to_bits());
            assert_ne!(ambient.to_bits(), 1e-9f64.to_bits());
        }
        Err(other) => panic!("golden v1 file failed to load: {other:?}"),
    }
}
