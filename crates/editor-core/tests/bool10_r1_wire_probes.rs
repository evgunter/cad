//! R1 review probes for BOOL-10 (PR #2135, frozen head 3f8163dd8) —
//! the wire half: what `Step::ArcTo { spec, splits }` did to the
//! persisted shape, measured on the committed corpus document.
//!
//! Review artefact, not a unit deliverable: every row records what the
//! frozen head DOES, so a row going red on a later head is
//! information, not necessarily a defect.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::Path;

use editor_core::{PersistError, load};
use geom_core::Tol;

/// The committed die-tool document, whose one arc leg carries the new
/// shape.
fn die_tool() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/corpus/die_tool.pncad");
    std::fs::read_to_string(&path).expect("the committed die-tool document reads")
}

fn split_header(text: &str) -> (String, serde_json::Value) {
    let (id_line, body) = text.split_once('\n').unwrap();
    (
        format!("{id_line}\n"),
        serde_json::from_str(body).expect("a save's body is JSON"),
    )
}

fn join(header: &str, v: &serde_json::Value) -> String {
    format!("{header}{}\n", serde_json::to_string_pretty(v).unwrap())
}

/// Rewrites every `"ArcTo": {"spec": S, "splits": N}` back to the
/// pre-BOOL-10 newtype shape `"ArcTo": S`, in place.
fn to_pre_bool10_shape(v: &mut serde_json::Value, hits: &mut usize) {
    match v {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::Object(inner)) = map.get("ArcTo")
                && inner.contains_key("spec")
                && inner.contains_key("splits")
            {
                let spec = inner.get("spec").unwrap().clone();
                map.insert("ArcTo".to_owned(), spec);
                *hits += 1;
                return;
            }
            for (_, child) in map.iter_mut() {
                to_pre_bool10_shape(child, hits);
            }
        }
        serde_json::Value::Array(items) => {
            for child in items {
                to_pre_bool10_shape(child, hits);
            }
        }
        _ => {}
    }
}

/// Drops the `"splits"` field, keeping the struct shape — the question
/// "is the new field OPTIONAL on read".
fn drop_splits(v: &mut serde_json::Value, hits: &mut usize) {
    match v {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::Object(inner)) = map.get_mut("ArcTo")
                && inner.remove("splits").is_some()
            {
                *hits += 1;
                return;
            }
            for (_, child) in map.iter_mut() {
                drop_splits(child, hits);
            }
        }
        serde_json::Value::Array(items) => {
            for child in items {
                drop_splits(child, hits);
            }
        }
        _ => {}
    }
}

/// **The committed document loads, and its arc leg carries `splits`.**
/// The baseline the two mutations below are measured against.
#[test]
fn the_committed_document_carries_the_split_count_and_loads() {
    let text = die_tool();
    assert!(
        text.contains("\"splits\": 1"),
        "the committed die-tool document should carry the new field"
    );
    load(&text, Tol::witness()).expect("the committed document loads at this head");
}

/// **`splits` is REQUIRED on read, not optional.** A document whose
/// `ArcTo` keeps the struct shape but omits the field does not load.
/// This is the answer to "is the field append-only-optional": it is
/// not — the load refuses typed rather than defaulting to the plain
/// leg.
#[test]
fn a_document_without_the_splits_field_does_not_load() {
    let (header, mut body) = split_header(&die_tool());
    let mut hits = 0;
    drop_splits(&mut body, &mut hits);
    assert_eq!(hits, 1, "the fixture has exactly one arc leg");
    let text = join(&header, &body);
    match load(&text, Tol::witness()) {
        Err(err @ PersistError::Unreadable { .. }) => {
            eprintln!("[no splits] {err}");
        }
        other => panic!("a document missing `splits` must refuse Unreadable, got {other:?}"),
    }
}

/// **A pre-BOOL-10 document does not load either.** The variant went
/// from a newtype to a struct, so every document an earlier build
/// wrote with an arc leg is unreadable at this head — the change is a
/// FORMAT BREAK, not an additive field. Nothing in tree still carries
/// the old shape (the corpus was re-authored), so this row is the
/// record of what the break costs, on bytes derived from a real
/// committed document.
#[test]
fn a_pre_bool10_arc_to_shape_does_not_load() {
    let (header, mut body) = split_header(&die_tool());
    let mut hits = 0;
    to_pre_bool10_shape(&mut body, &mut hits);
    assert_eq!(hits, 1, "the fixture has exactly one arc leg");
    let text = join(&header, &body);
    match load(&text, Tol::witness()) {
        Err(err @ PersistError::Unreadable { .. }) => {
            eprintln!("[pre-BOOL-10 shape] {err}");
        }
        other => panic!("the pre-BOOL-10 arc shape must refuse Unreadable, got {other:?}"),
    }
}

// ------------------------------------------------------------------
// The document layer at a NON-IDENTITY split count.
// ------------------------------------------------------------------
//
// The tree's own rows only ever build `ProgramStep::ArcTo` through the
// `arc_to(spec)` constructor, i.e. `splits: 1`. Measured by mutation
// at the frozen head: discarding the count in `res_step`
// (`splits: 1`) and dropping the `xn == yn` clause from
// `step_bit_eq` leaves the whole 1142-row editor-core suite GREEN.
// The rows below are the coverage that mutation walked through.

use crate::fixture;

use editor_core::{
    LoopProgram, Node, ParamEnv, ProfileDoc, ProfileProgram, ProgramArcData, ProgramStep,
    ProgramTarget, RecipeNodeId, save,
};
use fixture::{frame, insert, len, scl};

/// A one-leg chain: the half-disc's semicircle, declared split `n`.
fn split_program(plane: RecipeNodeId, n: u32) -> ProfileProgram {
    ProfileProgram {
        plane,
        loops: vec![LoopProgram::Chain(vec![
            ProgramStep::At([len(0.0), len(-0.5)]),
            ProgramStep::ArcTo {
                spec: ProgramArcData::Bulge {
                    target: ProgramTarget::Point([len(0.0), len(0.5)]),
                    b: scl(1.0),
                },
                splits: n,
            },
            ProgramStep::LineTo(ProgramTarget::Start),
        ])],
    }
}

/// **The declared count survives resolution.** `res_step` must carry
/// the document's `splits` into `profile::Step::ArcTo`, and the
/// resolved program must replay into `n` pieces with `n - 1` declared
/// joints. Discarding the count at resolution reds this row.
#[test]
fn the_document_split_count_survives_resolution_and_replay() {
    for n in [2u32, 3, 5] {
        let resolved = split_program(RecipeNodeId(0), n)
            .resolve(&ParamEnv::<f64>::default())
            .expect("the split program resolves at f64");
        let profile::Step::ArcTo { splits, .. } = &resolved[0][1] else {
            panic!("step 1 is the arc leg");
        };
        assert_eq!(*splits, n as usize, "the declared count reached the kernel");
        let loop_ = profile::replay(&resolved[0], Tol::witness()).expect("the split replays");
        assert_eq!(loop_.vertices().len(), n as usize + 1);
        let joints: Vec<usize> = (1..n as usize).collect();
        assert_eq!(loop_.tangent_joints(), joints.as_slice());
    }
}

/// **Two programs differing ONLY in their split count are not
/// bit-equal.** `ProfileProgram`'s bit equality must compare the
/// counts; without the clause a document edit that changes only the
/// split reads as no change at all.
#[test]
fn programs_differing_only_in_the_split_count_are_not_bit_equal() {
    let plane = RecipeNodeId(0);
    assert_ne!(
        split_program(plane, 2),
        split_program(plane, 3),
        "a 2-way and a 3-way split must not compare bit-equal"
    );
    assert_eq!(
        split_program(plane, 2),
        split_program(plane, 2),
        "bit equality is reflexive here"
    );
}

/// **A split arc leg round-trips through the wire.** Save/load must
/// carry `splits: n` unchanged; the tree's own corpus only ever
/// persists `splits: 1`.
#[test]
fn a_split_arc_leg_round_trips_through_the_wire() {
    let doc = ProfileDoc::empty_derived("bool10-r1-wire", Tol::witness());
    let (doc, plane) = insert(doc, frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let program = split_program(plane, 4);
    let (doc, node) = insert(doc, Node::Profile(program.clone()));
    let text = save(&doc, &[], Tol::witness()).expect("the split document saves");
    assert!(text.contains("\"splits\": 4"), "the count is persisted");
    let back = load(&text, Tol::witness()).expect("the split document loads");
    let Some(Node::Profile(read)) = back.doc.node(node).cloned() else {
        panic!("the profile node reads back");
    };
    assert_eq!(read, program, "the round trip preserves the declared split");
}
