//! M4 PR 6 review MINOR-3 — the committed GOLDEN v1 fixture.
//!
//! D6.1's round-trip row proves save∘load is a fixpoint, but a
//! fixpoint is BLIND to format drift: rename a field and save/load
//! stay self-consistent while every existing v1 file breaks. This row
//! pins the frozen wire shape to CHECKED-IN BYTES
//! (`tests/golden/v2_golden.cad`): the fixture document must save to
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
    Attr, CancelToken, Dimension, DocEdit, DocParam, EntityKind, EvalOptions, Expr, MetaValue,
    Node, NodeResult, ParamName, PersistError, ProfileDesc, ProfileDoc, Rgba8, RoleSeg, StableName,
    WitnessDatum, apply, evaluate, load, save,
};
use fixture::{desc, len};

const GOLDEN: &str = include_str!("golden/v2_golden.cad");
const GOLDEN_PATH: &str = "tests/golden/v2_golden.cad";

/// The golden document: deterministic (no ambient reads — ε pinned by
/// the SetTolerance edit) and shape-covering: params, an arc-bearing
/// profile with a hand-DECLARED line/arc tangency (#101), a
/// fillet-CONSTRUCTED profile (tangent joints by construction), a
/// param-expression slot, witness bytes, appearance attrs + D7
/// metadata (floats, -0.0, bytes, list, nesting).
///
/// #120 history: the original golden hand-declared a COLLINEAR
/// tangency — exactly what #101's same-carrier-is-identity rule
/// refuses — so the frozen exemplar evaluated sick (node 2 Failed,
/// invisible to the byte rows). Regenerated 8b-fix-pass from this
/// HEALTHY document (the corpus's legal line/arc bracket pattern,
/// same `tangent_joints` wire coverage); content change only, format
/// unchanged — both byte generations parse under the same schema-1
/// loader.
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
    // #101 tangency coverage in the FROZEN bytes: a hand-DECLARED
    // line/arc tangency (node 2 — the #100 bracket: the quarter arc
    // leaving (1.5,1), bulge −(√2−1), is exactly tangent to both
    // neighboring lines; joints 3 and 4 declared BY HAND) and a
    // fillet-CONSTRUCTED loop (node 3, joints declared by
    // construction) — the wire's tangent_joints field is pinned by
    // the golden from day one. (#120: this replaced the original
    // COLLINEAR declaration, which the #101 same-carrier rule
    // refuses — the old exemplar was sick.)
    let mut bracket = profile::ProfileLoop::new(
        [
            (0.0, 0.0, 0.0),
            (3.0, 0.0, 0.0),
            (3.0, 1.0, 0.0),
            (1.5, 1.0, -(std::f64::consts::SQRT_2 - 1.0)),
            (1.0, 1.5, 0.0),
            (1.0, 3.0, 0.0),
            (0.0, 3.0, 0.0),
        ]
        .into_iter()
        .map(|(x, y, bulge)| profile::ProfileVertex {
            pos: geom_core::Point2::new(x, y),
            bulge,
        })
        .collect(),
    );
    bracket.tangent_joints = vec![3, 4];
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Profile(ProfileDesc(profile::Profile::new(
                profile::SketchPlane::xy(),
                vec![bracket],
            ))),
        },
    );
    let fillet_loop = profile::ProfileLoop::builder(geom_core::Point2::new(0.0, 0.0))
        .line_to(geom_core::Point2::new(3.0, 0.0))
        .line_to(geom_core::Point2::new(3.0, 1.0))
        .fillet(
            geom_core::Point2::new(1.0, 1.0),
            geom_core::Point2::new(1.0, 3.0),
            0.5,
        )
        .expect("golden fillet fits")
        .line_to(geom_core::Point2::new(1.0, 3.0))
        .line_to(geom_core::Point2::new(0.0, 3.0))
        .close();
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Profile(ProfileDesc(profile::Profile::new(
                profile::SketchPlane::xy(),
                vec![fillet_loop],
            ))),
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
fn golden_bytes_are_frozen() {
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
        "schema-v2 wire bytes drifted from the committed golden — this is a FORMAT \
         CHANGE: it needs a ratified schema bump + migration step, never a re-bless in passing"
    );
}

#[test]
fn golden_bytes_load() {
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
        Err(other) => panic!("golden v2 file failed to load: {other:?}"),
    }
}

/// #117 follow-through (the green gate; #120): byte identity is as
/// blind to evaluation health as fingerprint identity — the ORIGINAL
/// golden froze a document whose declared collinear tangency #101's
/// same-carrier rule refuses (node 2 Failed) while both byte rows
/// stayed green. The exemplar is now healthy, and this gate keeps the
/// class structurally dead: corpus docs assert green in their rows,
/// the persistence fingerprints assert green (#117), and the golden
/// asserts green HERE. Only meaningful at the golden's own pinned ε
/// (every other matrix row refuses `ToleranceConflict` at the load
/// door, asserted above), so other rows skip.
#[test]
fn golden_document_evaluates_green_at_its_pinned_eps() {
    if geom_core::Tolerance::get().eps.to_bits() != 1e-9f64.to_bits() {
        return;
    }
    let (mut doc, edits) = golden();
    for e in &edits {
        doc = apply(&doc, e).expect("golden edit").doc;
    }
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default());
    let bad: Vec<String> = ev
        .nodes
        .iter()
        .filter_map(|(id, r)| match r {
            NodeResult::Ok(_) => None,
            NodeResult::Failed(e) => Some(format!("{id:?} FAILED: {e:?}")),
            NodeResult::Poisoned { through } => {
                Some(format!("{id:?} poisoned through {through:?}"))
            }
        })
        .collect();
    assert!(
        bad.is_empty(),
        "the golden document must evaluate green (#117/#120 — a sick \
         golden freezes sick bytes):\n{}",
        bad.join("\n")
    );
}
