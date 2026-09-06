//! **The at-rest gate refuses a mate read below a product root in the
//! operand's voice.**
//!
//! `P(T(top))` with the mate read AT `T`: the solve places it, the
//! product gathers, and the product's table has no row for the bare
//! `top/…` spelling — the pattern is the root and its rows wear
//! `Instance(i)`. The gate then asks the OPERAND's own table, and a
//! name spelled there at a node the product does not list refuses
//! `RefusedRef::ReadBelowARoot { at }` naming that node, rather than
//! calling the name vanished. `Vanished` is only ever a name that
//! names nothing where the mate reads it.
//!
//! Nothing is admitted here that was refused: every row that refused
//! before refuses still, and the control (the same document read AT
//! the pattern, with the instance spelling) holds as it did.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::fixture;

use std::sync::Arc;

use editor_core::{
    Alignment, AssemblyError, AxisSense, CapEnd, ContactClass, DocEdit, DocumentId, EntityKind,
    EvalOptions, Expr, MateFrame, MatePrimitive, MateRole, MateSide, Node, PartSelect, PatternKind,
    ProfileDoc, ProfileProgram, RecipeNodeId, RefusedRef, RoleSeg, SitedRef, StableName, product,
    solve_document,
};
use fixture::resolver::{PART_BODY, PartStore, in_part};
use fixture::{gate, in_copy, insert, len, on_frame, run, scl, step, xform};
use geom_core::Tol;

// ---- the scene ----

/// A `w x w x h` box as a whole part document.
fn box_part(label: &str, w: f64, h: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (w, 0.0), (w, w), (0.0, w)]],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(h),
        },
    );
    doc
}

/// The slab is wide enough that the block seated on it stands clear
/// of its edges, and the pattern's spacing puts every other copy
/// clear of the slab altogether — the rows are about the copy the
/// mate names, and a sibling resting uninvited is an undeclared
/// contact the gate is right to refuse.
const BASE_WIDTH: f64 = 3.0;
const BASE_HEIGHT: f64 = 1.0;
const TOP_HEIGHT: f64 = 3.0;
const SPACING: f64 = 5.0;

/// `base` (the slab) and `top` (the block), then `T(top)` lifted well
/// clear of the slab and `P(T(top))`, a two-copy linear pattern of it.
struct Scene {
    doc: ProfileDoc,
    opts: EvalOptions,
    base: RecipeNodeId,
    top: RecipeNodeId,
    xf: RecipeNodeId,
    pattern: RecipeNodeId,
}

fn scene(label: &str) -> Scene {
    let mut store = PartStore::new();
    let base_ref = store.insert(
        box_part(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let top_ref = store.insert(
        box_part(&format!("{label}-top"), 1.0, TOP_HEIGHT),
        Tol::witness(),
    );
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, xf) = insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: xf,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(SPACING),
            },
        },
    );
    Scene {
        doc,
        opts,
        base,
        top,
        xf,
        pattern,
    }
}

/// A `Rest` mate seating `b`'s bottom cap on `a`'s top cap by frame
/// coincidence, both frames in their member's own part coordinates
/// and both axes outward, so the block stands ON the slab.
fn seat(a: SitedRef, b: SitedRef) -> Node<ProfileProgram> {
    Node::Mate {
        a,
        b,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: MateFrame {
                origin: [1.0, 1.0, BASE_HEIGHT],
                axis: [0.0, 0.0, 1.0],
                reference: [1.0, 0.0, 0.0],
            },
            b: MateFrame {
                origin: [0.0, 0.0, 0.0],
                axis: [0.0, 0.0, -1.0],
                reference: [1.0, 0.0, 0.0],
            },
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    }
}

/// Insert `mate` and answer its id.
fn mated(doc: ProfileDoc, mate: Node<ProfileProgram>) -> (ProfileDoc, RecipeNodeId) {
    let (doc, id) = step(doc, DocEdit::InsertNode { node: mate });
    (doc, id.expect("the mate mints"))
}

/// The `Reference` refusal's three fields, or a panic naming what the
/// gate said instead.
fn reference_refusal(err: &AssemblyError) -> (RecipeNodeId, MateSide, &RefusedRef) {
    let AssemblyError::Reference {
        mate, side, why, ..
    } = err
    else {
        panic!("expected the reference refusal, got {err:?}");
    };
    (*mate, *side, why)
}

// ---- A1: the issue's document ----

/// **The issue's own document.** The mate is read at `T`, below the
/// pattern that consumes it. The solve places it (`Determining`), the
/// product gathers, and the gate refuses — in the operand's voice:
/// `ReadBelowARoot { at: T }`, whose message names `T` and says it is
/// not a root of the product.
#[test]
fn the_issues_document_refuses_read_below_a_root_naming_the_transform() {
    let s = scene("msolve5-a1");
    let a = SitedRef::at_mint(in_part(s.base, CapEnd::End));
    let b = SitedRef::new(s.xf, in_part(s.top, CapEnd::Start));
    let (doc, mate) = mated(s.doc, seat(a, b));

    let poses = solve_document(&doc, Tol::witness());
    assert!(
        poses.fault(mate).is_none(),
        "the solve places a mate read at the transform: {:?}",
        poses.fault(mate)
    );
    assert_eq!(poses.role(mate), Some(MateRole::Determining));
    let ev = run(&doc, &s.opts);
    assert!(
        product(&doc, &ev, Tol::witness()).is_ok(),
        "the product gathers"
    );

    let err = gate(&doc, &ev).expect_err("the gate refuses a mate read below a root");
    let (named, side, why) = reference_refusal(&err);
    assert_eq!((named, side), (mate, MateSide::B));
    assert_eq!(*why, RefusedRef::ReadBelowARoot { at: s.xf });
    let said = err.to_string();
    assert!(
        said.contains(&format!("read at node {}", s.xf.0)) && said.contains("not a root"),
        "the message names the operand, not a vanished name: {said}"
    );
    assert!(
        !said.contains("no entity of the product answers"),
        "the vanished sentence is gone: {said}"
    );
}

// ---- the control: read AT the pattern ----

/// The same document read AT `P` with the `Instance(0, top/…)`
/// spelling holds at the gate: copy 0 IS `T(top)`, and the pattern's
/// row is a product root's own row.
#[test]
fn read_at_the_pattern_with_the_instance_spelling_the_gate_holds() {
    let s = scene("msolve5-control");
    let a = SitedRef::at_mint(in_part(s.base, CapEnd::End));
    let b = SitedRef::new(
        s.pattern,
        in_copy(s.pattern, 0, in_part(s.top, CapEnd::Start)),
    );
    let (doc, mate) = mated(s.doc, seat(a, b));
    let poses = solve_document(&doc, Tol::witness());
    assert!(poses.fault(mate).is_none(), "{:?}", poses.fault(mate));
    let ev = run(&doc, &s.opts);
    assert!(
        gate(&doc, &ev).is_ok(),
        "read at the root, the gate holds: {:?}",
        gate(&doc, &ev)
    );
}

// ---- a pass-through root over the pattern ----

/// A mate read at a `Part { Instance(0) }` root over the pattern
/// holds: a pass-through root's rows are verbatim — the `Part`
/// projects the pattern's `Instance(0)` rows under their own heads —
/// so the product's table answers and the operand is never asked.
#[test]
fn a_mate_read_at_a_part_root_over_the_pattern_holds() {
    let s = scene("msolve5-part-root");
    let (doc, part) = insert(
        s.doc,
        Node::Part {
            of: s.pattern,
            select: PartSelect::Instance(Expr::count(0)),
        },
    );
    assert!(
        doc.roots().contains(&part) && !doc.roots().contains(&s.pattern),
        "the Part consumed the pattern's root: {:?}",
        doc.roots()
    );
    let a = SitedRef::at_mint(in_part(s.base, CapEnd::End));
    let b = SitedRef::new(part, in_copy(s.pattern, 0, in_part(s.top, CapEnd::Start)));
    let (doc, mate) = mated(doc, seat(a, b));
    let poses = solve_document(&doc, Tol::witness());
    assert!(poses.fault(mate).is_none(), "{:?}", poses.fault(mate));
    let ev = run(&doc, &s.opts);
    assert!(
        gate(&doc, &ev).is_ok(),
        "read at a pass-through root, the gate holds: {:?}",
        gate(&doc, &ev)
    );
}

// ---- A2: the order of the two questions ----

/// A mate read at `T` naming a face `top` does not have refuses
/// `Vanished`: the operand's own table is silent on the name, so the
/// first question answers and the second is never reached. This is
/// what keeps "vanished" honest — the name names nothing where the
/// mate reads it.
#[test]
fn a_name_the_operand_does_not_spell_stays_vanished() {
    let s = scene("msolve5-vanished");
    // The block's part has no node 99, so no face of `top` wears this
    // spelling — at `T`, at the pattern, or anywhere.
    let nowhere = StableName {
        kind: EntityKind::Face,
        node: s.top,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(99),
                path: vec![RoleSeg::Cap(CapEnd::Start)],
            }),
        }],
    };
    assert_ne!(nowhere.node, PART_BODY);
    let a = SitedRef::at_mint(in_part(s.base, CapEnd::End));
    let b = SitedRef::new(s.xf, nowhere);
    let (doc, mate) = mated(s.doc, seat(a, b));
    let ev = run(&doc, &s.opts);
    let err = gate(&doc, &ev).expect_err("a name nothing answers to refuses");
    let (named, side, why) = reference_refusal(&err);
    assert_eq!((named, side), (mate, MateSide::B));
    assert_eq!(*why, RefusedRef::Vanished);
}

// ---- what it is precedes where it is rooted ----

/// A mate naming a root's BODY refuses `NotAFace { kind: Body }`. The
/// product's table is silent on it — the product's own body is
/// nobody's root body, so body rows do not carry — and the operand's
/// own table answers with a body: a non-face never mints anywhere,
/// so the gate says what the name IS before asking where it is
/// rooted. (On main this row refused `Vanished`; it still refuses.)
#[test]
fn a_mate_naming_a_roots_body_refuses_not_a_face() {
    let s = scene("msolve5-body-row");
    let body = StableName {
        kind: EntityKind::Body,
        node: s.base,
        path: vec![RoleSeg::OutputBody],
    };
    let a = SitedRef::at_mint(body.clone());
    let b = SitedRef::new(
        s.pattern,
        in_copy(s.pattern, 0, in_part(s.top, CapEnd::Start)),
    );
    let (doc, mate) = mated(s.doc, seat(a, b));
    let ev = run(&doc, &s.opts);
    let Some(editor_core::NodeResult::Ok(value)) = ev.result(s.base) else {
        panic!("the base evaluates");
    };
    assert!(
        value.name_table.lookup(&body).is_some(),
        "the root's own table spells its body"
    );
    assert!(doc.roots().contains(&s.base));
    let err = gate(&doc, &ev).expect_err("a body reference never mints");
    let (named, side, why) = reference_refusal(&err);
    assert_eq!((named, side), (mate, MateSide::A));
    assert_eq!(
        *why,
        RefusedRef::NotAFace {
            kind: EntityKind::Body
        }
    );
}

/// The same body read BELOW a root — `T(top)`'s body, read at `T`
/// under the pattern — refuses `NotAFace { kind: Body }` too, not
/// `ReadBelowARoot`: the kind question is asked before the root
/// question, so a non-face is a non-face wherever it is read.
#[test]
fn a_body_read_below_a_root_refuses_not_a_face_before_the_root_question() {
    let s = scene("msolve5-body-below");
    let a = SitedRef::at_mint(in_part(s.base, CapEnd::End));
    let body = StableName {
        kind: EntityKind::Body,
        node: s.top,
        path: vec![RoleSeg::OutputBody],
    };
    let ev0 = run(&s.doc, &s.opts);
    let Some(editor_core::NodeResult::Ok(value)) = ev0.result(s.xf) else {
        panic!("the transform evaluates");
    };
    assert!(
        value.name_table.lookup(&body).is_some(),
        "the transform's own table spells the instance's body"
    );
    let b = SitedRef::new(s.xf, body);
    let (doc, mate) = mated(s.doc, seat(a, b));
    assert!(!doc.roots().contains(&s.xf));
    let ev = run(&doc, &s.opts);
    let err = gate(&doc, &ev).expect_err("a body reference never mints");
    let (named, side, why) = reference_refusal(&err);
    assert_eq!((named, side), (mate, MateSide::B));
    assert_eq!(
        *why,
        RefusedRef::NotAFace {
            kind: EntityKind::Body
        }
    );
}
