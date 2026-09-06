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
    Alignment, AssemblyError, AxisSense, BooleanOp, CapEnd, ContactClass, Datum, DocEdit,
    DocumentId, EntityKind, Entry, EvalOptions, Evaluation, Expr, MateFrame, MatePrimitive,
    MateRole, MateSide, NameTable, Node, NodeResult, PartSelect, PatternKind, ProductError,
    ProfileDoc, ProfileProgram, RecipeNodeId, RefusedRef, RoleSeg, SitedRef, StableName,
    ValuePayload, product, solve_document,
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

/// The U-shaped outline, in the plane's own coordinates: a `4 x 2`
/// bar with a `3 x 1` notch cut in from its left end.
const U_OUTLINE: [(f64, f64); 8] = [
    (2.0, 1.0),
    (6.0, 1.0),
    (6.0, 3.0),
    (2.0, 3.0),
    (2.0, 2.5),
    (5.0, 2.5),
    (5.0, 1.5),
    (2.0, 1.5),
];

/// A `4 x 4 x 4` box with a U-shaped channel subtracted through its
/// middle (`z` from 1 to 3, clear of both caps): the cutter's tongue
/// leaves a side face in two fragments under ONE name, so the part's
/// product — and every table above it — holds a TIED face row.
fn slotted_part(label: &str) -> ProfileDoc {
    let doc = box_part(label, 4.0, 4.0);
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![U_OUTLINE.to_vec()],
    );
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(2.0),
        },
    );
    let (doc, _) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: PART_BODY,
            b,
            declare: None,
        },
    );
    doc
}

/// A U-shaped prism split by the plane `x = 4`, which cuts both arms
/// of the U: the split's rows tie the fragments of each cut edge
/// under one name, so the part's product holds TIED EDGE rows.
fn u_split_part(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![U_OUTLINE.to_vec()],
    );
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(2.0),
        },
    );
    assert_eq!(body, PART_BODY);
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(4.0), len(0.0), len(0.0)],
            normal: [scl(1.0), scl(0.0), scl(0.0)],
        }),
    );
    let (doc, _) = insert(
        doc,
        Node::Split {
            target: body,
            tool: plane,
        },
    );
    doc
}

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

/// The scene up to `T(top)`, over the given top part.
fn lifted(label: &str, top_part: ProfileDoc) -> Scene {
    let mut store = PartStore::new();
    let base_ref = store.insert(
        box_part(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let top_ref = store.insert(top_part, Tol::witness());
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, xf) = insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
    Scene {
        doc,
        opts,
        base,
        top,
        xf,
        pattern: xf,
    }
}

/// The two-copy linear pattern over `T(top)`.
fn patterned(s: Scene) -> Scene {
    let (doc, pattern) = insert(
        s.doc,
        Node::Pattern {
            input: s.xf,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(SPACING),
            },
        },
    );
    Scene { doc, pattern, ..s }
}

/// `P(T(top))` over the given top part.
fn scene_with(label: &str, top_part: ProfileDoc) -> Scene {
    patterned(lifted(label, top_part))
}

/// `P(T(top))` over the plain block.
fn scene(label: &str) -> Scene {
    scene_with(label, box_part(&format!("{label}-top"), 1.0, TOP_HEIGHT))
}

/// `T(top)` over the plain block, with NO pattern: `T` is a root.
fn scene_no_pattern(label: &str) -> Scene {
    lifted(label, box_part(&format!("{label}-top"), 1.0, TOP_HEIGHT))
}

/// The node's own table, as the gate reads it.
fn table_of(ev: &Evaluation<f64>, node: RecipeNodeId) -> &NameTable {
    let Some(NodeResult::Ok(v)) = ev.result(node) else {
        panic!("node {} did not evaluate: {:?}", node.0, ev.result(node));
    };
    &v.name_table
}

/// The first TIED row of `kind` in the node's table.
fn tied_row(ev: &Evaluation<f64>, node: RecipeNodeId, kind: EntityKind) -> (StableName, u32) {
    table_of(ev, node)
        .iter()
        .find_map(|(n, e)| match e {
            Entry::Tied(c) if n.kind == kind => {
                Some((n.clone(), u32::try_from(c.len()).expect("a small tie")))
            }
            Entry::Tied(_) | Entry::Unique(_) => None,
        })
        .unwrap_or_else(|| panic!("node {} holds a tied {kind:?} row", node.0))
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
    // The words are pinned once, in `display_contract`; the value is
    // what says the gate named the operand.
    assert_eq!(*why, RefusedRef::ReadBelowARoot { at: s.xf });
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
    // The block's part has no node 99 — its body is `PART_BODY` — so
    // no face of `top` wears this spelling: at `T`, at the pattern,
    // or anywhere.
    const NO_SUCH_PART_NODE: RecipeNodeId = RecipeNodeId(99);
    assert_ne!(NO_SUCH_PART_NODE, PART_BODY);
    let nowhere = StableName {
        kind: EntityKind::Face,
        node: s.top,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: NO_SUCH_PART_NODE,
                path: vec![RoleSeg::Cap(CapEnd::Start)],
            }),
        }],
    };
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

// ---- ties: unique or tied, the kind question comes first ----

/// A TIED face read at `T` below the pattern refuses `ReadBelowARoot
/// { at: T }` — a tie among faces below a root is still read below a
/// root. The same tie read AT the pattern with the instance spelling
/// is the product's own row, and the product decides its own ties:
/// `Ambiguous { width: 2 }`.
#[test]
fn a_tied_face_below_a_root_refuses_read_below_a_root_and_at_the_root_ambiguous() {
    let s = scene_with("msolve5-tied-face", slotted_part("msolve5-tied-face-top"));
    let ev0 = run(&s.doc, &s.opts);
    let (tied, width) = tied_row(&ev0, s.xf, EntityKind::Face);
    assert_eq!(tied.node, s.top, "the tie is worn by the instance");
    assert_eq!(width, 2);
    let a = SitedRef::at_mint(in_part(s.base, CapEnd::End));

    let b = SitedRef::new(s.xf, tied.clone());
    let (doc, mate) = mated(s.doc.clone(), seat(a.clone(), b));
    let ev = run(&doc, &s.opts);
    assert!(
        product(&doc, &ev, Tol::witness()).is_ok(),
        "the product gathers"
    );
    let err = gate(&doc, &ev).expect_err("read below a root");
    let (named, side, why) = reference_refusal(&err);
    assert_eq!((named, side), (mate, MateSide::B));
    assert_eq!(*why, RefusedRef::ReadBelowARoot { at: s.xf });

    let b = SitedRef::new(s.pattern, in_copy(s.pattern, 0, tied));
    let (doc, mate) = mated(s.doc, seat(a, b));
    let ev = run(&doc, &s.opts);
    let err = gate(&doc, &ev).expect_err("a tie is never broken by picking");
    let (named, side, why) = reference_refusal(&err);
    assert_eq!((named, side), (mate, MateSide::B));
    assert_eq!(*why, RefusedRef::Ambiguous { width });
}

/// A TIED EDGE read at `T` below the pattern refuses `NotAFace {
/// kind: Edge }`: the kind question is asked before the root question
/// for a tied entry exactly as for a unique one — the name's kind is
/// every candidate's kind. The same tie AT the pattern is the
/// product's own row and answers `Ambiguous`, as every tie the
/// product holds does.
#[test]
fn a_tied_edge_below_a_root_refuses_not_a_face() {
    let s = scene_with("msolve5-tied-edge", u_split_part("msolve5-tied-edge-top"));
    let ev0 = run(&s.doc, &s.opts);
    let (tied, width) = tied_row(&ev0, s.xf, EntityKind::Edge);
    let a = SitedRef::at_mint(in_part(s.base, CapEnd::End));

    let b = SitedRef::new(s.xf, tied.clone());
    let (doc, mate) = mated(s.doc.clone(), seat(a.clone(), b));
    let ev = run(&doc, &s.opts);
    let err = gate(&doc, &ev).expect_err("an edge never mints");
    let (named, side, why) = reference_refusal(&err);
    assert_eq!((named, side), (mate, MateSide::B));
    assert_eq!(
        *why,
        RefusedRef::NotAFace {
            kind: EntityKind::Edge
        }
    );

    let b = SitedRef::new(s.pattern, in_copy(s.pattern, 0, tied));
    let (doc, mate) = mated(s.doc, seat(a, b));
    let ev = run(&doc, &s.opts);
    let err = gate(&doc, &ev).expect_err("a tie is never broken by picking");
    let (named, side, why) = reference_refusal(&err);
    assert_eq!((named, side), (mate, MateSide::B));
    assert_eq!(*why, RefusedRef::Ambiguous { width });
}

// ---- the claim is about the root list, not about the product's rows ----

/// `Intersect(T(top), far)` — a boolean whose result is EMPTY — is
/// the root over `T`. The product holds no trace of `top`'s face at
/// all, and the gate still refuses `ReadBelowARoot { at: T }`: the
/// arm says the operand is not a root and that a reference resolves
/// against a root's own rows, and nothing more — it does not claim
/// the product spells the face some other way.
#[test]
fn an_operand_under_an_empty_boolean_root_still_refuses_read_below_a_root() {
    let s = scene_no_pattern("msolve5-empty-root");
    let (doc, far_profile) = on_frame(
        s.doc,
        [50.0, 50.0, 50.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, far) = insert(
        doc,
        Node::Extrude {
            profile: far_profile,
            distance: len(1.0),
        },
    );
    let (doc, empty) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Intersect,
            a: s.xf,
            b: far,
            declare: None,
        },
    );
    assert!(
        doc.roots().contains(&empty) && !doc.roots().contains(&s.xf),
        "the boolean consumed the transform's root: {:?}",
        doc.roots()
    );
    let a = SitedRef::at_mint(in_part(s.base, CapEnd::End));
    let b = SitedRef::new(s.xf, in_part(s.top, CapEnd::Start));
    let (doc, mate) = mated(doc, seat(a, b));
    let ev = run(&doc, &s.opts);
    assert!(
        product(&doc, &ev, Tol::witness()).is_ok(),
        "an empty root contributes nothing and the product gathers"
    );
    let err = gate(&doc, &ev).expect_err("read below a root");
    let (named, side, why) = reference_refusal(&err);
    assert_eq!((named, side), (mate, MateSide::B));
    assert_eq!(*why, RefusedRef::ReadBelowARoot { at: s.xf });
}

// ---- an operand that is not live never reaches the gate ----

/// The top part cannot be resolved, so `top`, `T` and `P` are failed
/// or poisoned — while the SOLVE, which evaluates nothing, places the
/// mate (`Determining`) and the mate IS a live value. What keeps the
/// gate from reading a table that does not exist is not the solve:
/// every live node sits under some root, so the gather's first pass
/// refuses the document at that root before any mate is read.
#[test]
fn a_poisoned_operand_never_reaches_the_gate() {
    let mut store = PartStore::new();
    let base_ref = store.insert(
        box_part("msolve5-poisoned-base", BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    // The top part lives in ANOTHER store: unresolvable here.
    let mut elsewhere = PartStore::new();
    let top_ref = elsewhere.insert(
        box_part("msolve5-poisoned-top", 1.0, TOP_HEIGHT),
        Tol::witness(),
    );
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let doc = ProfileDoc::empty(DocumentId::derive("msolve5-poisoned"), Tol::witness());
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
    let a = SitedRef::at_mint(in_part(base, CapEnd::End));
    let b = SitedRef::new(xf, in_part(top, CapEnd::Start));
    let (doc, mate) = mated(doc, seat(a, b));
    let poses = solve_document(&doc, Tol::witness());
    assert_eq!(poses.role(mate), Some(MateRole::Determining));
    let ev = run(&doc, &opts);
    assert!(
        matches!(ev.result(top), Some(NodeResult::Failed(_))),
        "the instance fails to resolve: {:?}",
        ev.result(top)
    );
    assert!(
        matches!(ev.result(xf), Some(NodeResult::Poisoned { .. })),
        "the operand is poisoned: {:?}",
        ev.result(xf)
    );
    assert!(
        matches!(ev.result(mate), Some(NodeResult::Ok(v)) if matches!(v.payload, ValuePayload::Mate(_))),
        "the mate itself is live: {:?}",
        ev.result(mate)
    );
    let err = gate(&doc, &ev).expect_err("the gather refuses");
    assert!(
        matches!(
            &err,
            AssemblyError::Product(e)
                if matches!(**e, ProductError::RootPoisoned { node, .. } if node == pattern)
        ),
        "the gather refuses at the poisoned root before any reference is read: {err:?}"
    );
}
