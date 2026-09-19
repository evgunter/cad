//! **The member walk's residue**: one nominal environment per solve,
//! and the mate wire's one field-bearing type that dropped a stray
//! key.
//!
//! The solve reads every number it needs — a pattern's count, a
//! `Part`'s index, the slots a derived offset composes — at the
//! document's own parameter bindings. That environment is built ONCE
//! per `solve_document` and passed down as a parameter, which is
//! pinned here by the source rather than by a probe: no counter sees a
//! `param_env` build, so the row reads the two files of the solve and
//! asserts that the one build stands in `solve_document`'s body and
//! nowhere else. The signatures carry the rest (`check_reference` and
//! `derived_offset` take `&ParamEnv<f64>`, so a caller cannot reach
//! them without one).
//!
//! `MatePrimitive` refuses a field this build lacks through the load
//! door, in the typed arm the format uses for every stray field
//! (`PersistError::Unreadable`, naming the field) — the persist module
//! docs' rule, proved for this type. The same alignment with the key
//! removed loads.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, ContentPin, DocEdit, DocRef, DocumentId,
    EntityKind, MateFrame, MatePrimitive, Node, PersistError, ProfileDoc, RecipeNodeId, RoleSeg,
    StableName, apply, load, save,
};
use geom_core::Tol;
use test_utils::source;

// ---- A1: one environment per solve ----

const MEMBER: &str = include_str!("../src/mate/member.rs");
const SOLVE: &str = include_str!("../src/mate/solve.rs");

/// **The nominal environment is built at one site of the solve**:
/// `solve_document`'s body holds the one `param_env` build under
/// `mate/`, and `member.rs` — every reader of that environment —
/// holds none. A reader that rebuilt its own would put a second
/// build in one of these two files, which is what this row counts.
///
/// Each file is read up to its `#[cfg(test)]` module, if it has one:
/// a unit row there hands the reader an environment the way the
/// solve does, and building it is the row's business, not the solve's.
#[test]
fn a1_the_solve_builds_its_nominal_environment_exactly_once() {
    const NEEDLE: &str = "param_env";
    let shipped = |text: &str| {
        let code = source::code_only(text);
        let end = code.find("#[cfg(test)]").unwrap_or(code.len());
        code[..end].to_owned()
    };
    let member = shipped(MEMBER);
    assert_eq!(
        member.matches(NEEDLE).count(),
        0,
        "member.rs takes the environment as a parameter and builds none"
    );
    let solve = shipped(SOLVE);
    let builds: Vec<usize> = solve.match_indices(NEEDLE).map(|(at, _)| at).collect();
    assert_eq!(
        builds.len(),
        1,
        "solve.rs builds the environment once, at lines {:?}",
        builds
            .iter()
            .map(|&at| source::line(&solve, at))
            .collect::<Vec<_>>()
    );
    let head = solve
        .find("pub fn solve_document")
        .expect("`solve_document` is declared");
    let source::ItemBody::Body(body) = source::item_body(&solve, head) else {
        panic!("`solve_document` has a body");
    };
    assert!(
        body.contains(&builds[0]),
        "the one build stands in `solve_document`'s body, not in a helper it calls"
    );
}

// ---- A4: the mate wire's attribute ----

/// A document with two instances and one planar-rest mate between
/// them, saved: the text a stray key is injected into.
fn saved_with_a_planar_rest(label: &str) -> (ProfileDoc, String) {
    let doc_ref = DocRef {
        id: DocumentId::derive("msolve7-part"),
        pin: ContentPin([7u8; 32]),
    };
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..2 {
        let applied = apply(
            &doc,
            &DocEdit::InsertNode {
                node: Node::instantiate_part(doc_ref),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("an instance inserts");
        ids.push(applied.record.minted.expect("a minted id"));
        doc = applied.doc;
    }
    let name = |node| StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::InPart {
            of: StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(1),
                path: vec![RoleSeg::Cap(CapEnd::Start)],
            }
            .into(),
        }],
    };
    let f = MateFrame {
        origin: [0.0, 0.0, 0.0],
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    };
    let doc = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Mate {
                a: crate::fixture::head(name(ids[0])),
                b: crate::fixture::head(name(ids[1])),
                class: ContactClass::Rest,
                alignment: Alignment {
                    a: f,
                    b: f,
                    primitive: MatePrimitive::PlanarRest { offset: 0.5 },
                    sense: AxisSense::Opposed,
                    clocking: None,
                },
            },
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("a mate inserts")
    .doc;
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    (doc, text)
}

/// `text` with `injected` placed just inside the planar rest's own
/// object — the one place the wire has a named field to deny.
fn with_a_key_on_the_planar_rest(text: &str, injected: &str) -> String {
    const ANCHOR: &str = "\"planar_rest\": {";
    assert_eq!(
        text.matches(ANCHOR).count(),
        1,
        "the saved document spells one planar rest: {text}"
    );
    let open = text.find(ANCHOR).unwrap() + ANCHOR.len();
    format!("{}{injected}{}", &text[..open], &text[open..])
}

/// **A stray key on `planar_rest` refuses at the load door**, in the
/// format's own arm for a field this build lacks, naming the field.
#[test]
fn a4_a_stray_key_on_a_planar_rest_refuses_at_the_load_door() {
    let (_, text) = saved_with_a_planar_rest("msolve7-a4-stray");
    let doctored = with_a_key_on_the_planar_rest(&text, "\"stray\": 2.0,");
    match load(&doctored, Tol::witness()) {
        Err(PersistError::Unreadable { detail, .. }) => {
            assert!(
                detail.contains("unknown field") && detail.contains("stray"),
                "the arm names the field it could not place: {detail}"
            );
        }
        other => panic!("a stray key on the mate primitive did not refuse typed: {other:?}"),
    }
}

/// **The same alignment without the key loads**, bit for bit: the
/// attribute denies what is not there and nothing that is.
#[test]
fn a4_the_same_alignment_without_the_key_loads() {
    let (doc, text) = saved_with_a_planar_rest("msolve7-a4-clean");
    let back = load(&text, Tol::witness()).expect("loads").doc;
    assert!(back.bit_eq(&doc), "the planar rest round-trips bit for bit");
    let primitives: Vec<MatePrimitive> = back
        .order()
        .iter()
        .filter_map(|&id| match back.node(id) {
            Some(Node::Mate { alignment, .. }) => Some(alignment.primitive),
            _ => None,
        })
        .collect();
    assert_eq!(primitives, [MatePrimitive::PlanarRest { offset: 0.5 }]);
}
