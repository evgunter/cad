//! **A mate reads its face where it says.**
//!
//! The at-rest gate resolves a mate's face at the node the mate reads
//! it at, and carries it up the consumers to the product. A union
//! carries it under the member's name; a placer above the operand
//! moves it again before the product holds it, and refuses.
//!
//! The rows build the documents the gather's own recourse leads to — a
//! union over two transforms of one mated instance — and measure the
//! gate's verdict on each, the member walk's descent through the
//! union, and the member key that makes two spellings of one placement
//! one member.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use editor_core::{
    Alignment, AssemblyError, AxisSense, CapEnd, ContactClass, DocEdit, DocumentId, EvalOptions,
    Expr, MateFrame, MatePrimitive, MateRole, MateSide, MintRefusal, Node, PartSelect, PatternKind,
    ProfileDoc, ProfileProgram, RecipeNodeId, RefusedRef, RoleSeg, SitedFace, StableName,
    member_of, product,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{gate, head, head_at, in_copy, insert, len, on_frame, run, scl, solve, step, xform};
use geom_core::Tol;

// ---- the scene ----

const BASE_WIDTH: f64 = 3.0;
const BASE_HEIGHT: f64 = 1.0;
const TOP_HEIGHT: f64 = 3.0;

/// A `w x w x h` box as a whole part document, and its body.
fn box_part(label: &str, w: f64, h: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (w, 0.0), (w, w), (0.0, w)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(h),
            side: ExtrudeSide::Along,
        },
    )
}

/// Two slab instances and one block instance, with their resolver and
/// part bodies. The first slab carries the world offset and roots the
/// group; the others sit where their mates put them.
struct Scene {
    doc: ProfileDoc,
    opts: EvalOptions,
    base1: RecipeNodeId,
    base2: RecipeNodeId,
    top: RecipeNodeId,
    base_body: RecipeNodeId,
    top_body: RecipeNodeId,
}

fn scene(label: &str) -> Scene {
    scene_with(label, true)
}

/// The scene with one slab only when `two` is false: `base2` is then
/// `base1` itself, and nothing is left unplaced.
fn scene_with(label: &str, two: bool) -> Scene {
    let mut store = PartStore::new();
    let (base_ref, base_body) = store.insert_part(
        box_part(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let (top_ref, top_body) = store.insert_part(
        box_part(&format!("{label}-top"), 1.0, TOP_HEIGHT),
        Tol::witness(),
    );
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base1) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, base2) = if two {
        insert(doc, fixture::mated_instance(base_ref))
    } else {
        (doc, base1)
    };
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    Scene {
        doc,
        opts,
        base1,
        base2,
        top,
        base_body,
        top_body,
    }
}

impl Scene {
    /// The slab's top cap, read at the slab's own mint.
    fn base_cap(&self, base: RecipeNodeId) -> SitedFace {
        head(in_part(base, self.base_body, CapEnd::End))
    }

    /// The block's bottom cap, as the instance names it.
    fn top_cap(&self) -> StableName {
        in_part(self.top, self.top_body, CapEnd::Start)
    }
}

/// A mate seating `b`'s bottom cap onto `a`'s top cap, `b`'s corner at
/// `(1, 1)` on the slab: frame coincidence, or another primitive with
/// its rider.
fn seat_with(
    a: SitedFace,
    b: SitedFace,
    primitive: MatePrimitive,
    clocking: Option<f64>,
) -> Node<ProfileProgram> {
    Node::Mate {
        a,
        b,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: MateFrame::authored(
                [1.0, 1.0, BASE_HEIGHT],
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 0.0],
                geom_core::Tol::witness(),
            )
            .expect("a definite frame"),
            b: MateFrame::authored(
                [0.0, 0.0, 0.0],
                [0.0, 0.0, -1.0],
                [1.0, 0.0, 0.0],
                geom_core::Tol::witness(),
            )
            .expect("a definite frame"),
            primitive,
            sense: AxisSense::Opposed,
            clocking,
        },
    }
}

fn seat(a: SitedFace, b: SitedFace) -> Node<ProfileProgram> {
    seat_with(a, b, MatePrimitive::FrameCoincidence, None)
}

/// Insert `mate` and answer its id.
fn mated(doc: ProfileDoc, mate: Node<ProfileProgram>) -> (ProfileDoc, RecipeNodeId) {
    let (doc, id) = step(
        doc,
        DocEdit::InsertNode {
            node: Box::new(mate),
        },
    );
    (doc, id.expect("the mate inserts"))
}

/// `name` as the union `union` re-mints member `member`'s entity: the
/// one `FromMember` segment under the union's node.
fn member_name(union: RecipeNodeId, member: RecipeNodeId, name: StableName) -> StableName {
    StableName {
        kind: name.kind,
        node: union,
        path: vec![RoleSeg::FromMember {
            member,
            of: name.into(),
        }],
    }
}

/// `top` lifted by `t1` (z + 10) and `t2` (z + 20), each mated to its
/// own slab, and the two transforms fused by a union `U` — the gather's
/// recourse for `PlacedUnderTwoRoots`, followed. `t1`'s mate reads the
/// block's face as `at_t1` says.
struct Fused {
    doc: ProfileDoc,
    t1: RecipeNodeId,
    t2: RecipeNodeId,
    union: RecipeNodeId,
    m1: RecipeNodeId,
    m2: RecipeNodeId,
}

fn fused(s: &Scene, at_t1: impl Fn(RecipeNodeId, RecipeNodeId) -> SitedFace) -> Fused {
    let (doc, t1) = insert(
        s.doc.clone(),
        xform(s.top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0),
    );
    let (doc, t2) = insert(doc, xform(s.top, [0.0, 0.0, 20.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: vec![t1, t2],
            declare: Vec::new(),
        },
    );
    assert!(
        doc.roots().contains(&union) && !doc.roots().contains(&t1),
        "the union consumes both transforms: {:?}",
        doc.roots()
    );
    let (doc, m1) = mated(doc, seat(s.base_cap(s.base1), at_t1(t1, union)));
    let (doc, m2) = mated(doc, seat(s.base_cap(s.base2), head_at(t2, s.top_cap())));
    Fused {
        doc,
        t1,
        t2,
        union,
        m1,
        m2,
    }
}

/// The solve places every instance and both mates determine.
fn assert_placed(s: &Scene, f: &Fused, what: &str) {
    let poses = solve(&f.doc, &s.opts, Tol::witness());
    for node in [s.base1, s.base2, s.top, f.m1, f.m2] {
        assert!(
            poses.fault(node).is_none(),
            "{what}: the solve places {node:?}: {:?}",
            poses.fault(node)
        );
    }
    assert_eq!(
        (poses.role(f.m1), poses.role(f.m2)),
        (Some(MateRole::Determining), Some(MateRole::Determining)),
        "{what}: both seats determine"
    );
}

// ---- A1 (a): the recourse, followed ----

/// **A union over two transforms of a mated instance mints both
/// contacts.** Each mate reads the block's cap at its own transform;
/// the union carries each as its member's face, and the gate verifies
/// both seats in the product.
#[test]
fn a1a_a_union_over_two_transforms_of_a_mated_instance_mints_both_contacts() {
    let s = scene("msolve13-a1a");
    let f = fused(&s, |t1, _| head_at(t1, s.top_cap()));
    assert_placed(&s, &f, "A1(a)");
    let ev = run(&f.doc, &s.opts);
    assert!(
        product(&f.doc, &ev, Tol::witness()).is_ok(),
        "the product gathers"
    );
    let gated = gate(&f.doc, &ev);
    assert!(
        gated.is_ok(),
        "A1(a): the union carries each mate's face up to the product: {gated:?}"
    );
}

// ---- A1 (b): a transform above the operand ----

/// **`t3 = Transform(t1)` is the root, and the mate reads at `t1`.**
/// The solve seats the block at `t1`'s placement; the product holds
/// the face where `t3` moved it, under the same names. The gate must
/// not verify the contact there: `t3` places the face again before the
/// product holds it, so the reference refuses rather than being
/// refuted against geometry the mate never spoke about.
#[test]
fn a1b_a_transform_above_the_operand_refuses_rather_than_refutes() {
    let s = scene("msolve13-a1b");
    let (doc, t1) = insert(
        s.doc.clone(),
        xform(s.top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0),
    );
    let (doc, t3) = insert(doc, xform(t1, [5.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0));
    assert!(
        doc.roots().contains(&t3) && !doc.roots().contains(&t1),
        "t3 consumes t1's root: {:?}",
        doc.roots()
    );
    let (doc, mate) = mated(doc, seat(s.base_cap(s.base1), head_at(t1, s.top_cap())));
    let poses = solve(&doc, &s.opts, Tol::witness());
    assert!(
        poses.fault(mate).is_none(),
        "the solve places the mate at t1: {:?}",
        poses.fault(mate)
    );
    assert_eq!(poses.role(mate), Some(MateRole::Determining));
    let ev = run(&doc, &s.opts);
    let err = gate(&doc, &ev).expect_err("the gate does not certify a moved face");
    let AssemblyError::Mint { refusals } = &err else {
        panic!("A1(b): expected the reference to refuse, not a verdict on geometry: {err:?}");
    };
    let [
        MintRefusal::Reference {
            mate: m, side, why, ..
        },
    ] = refusals.as_slice()
    else {
        panic!("A1(b): expected one reference refusal, got {refusals:?}");
    };
    assert_eq!((*m, *side), (mate, MateSide::B));
    assert_eq!(
        *why,
        RefusedRef::MovedAbove {
            at: t1,
            by: t3,
            copies: false,
        }
    );
}

// ---- A1 (c): a pick on the fused body ----

/// **A reference read at the union, naming member `t1`'s face**, is
/// what a pick on the fused body authors. The member walk descends the
/// union at the member the name says, and the mate is valid: it solves,
/// and the gate holds.
#[test]
fn a1c_a_reference_read_at_the_union_is_a_member_and_gates() {
    let s = scene("msolve13-a1c");
    let f = fused(&s, |t1, union| {
        head_at(union, member_name(union, t1, s.top_cap()))
    });
    let r = head_at(f.union, member_name(f.union, f.t1, s.top_cap()));
    let m = member_of(&f.doc, &r).expect("A1(c): the walk descends the union");
    assert_eq!(m.instance, s.top);
    assert_placed(&s, &f, "A1(c)");
    let ev = run(&f.doc, &s.opts);
    let gated = gate(&f.doc, &ev);
    assert!(
        gated.is_ok(),
        "A1(c): a mate read at the union gates: {gated:?}"
    );
    let _ = f.t2;
}

// ---- A2: two spellings of one placement ----

/// **One placement spelled two ways is one member.** A coaxial mate
/// with its clocking rider (prismatic) read at `t1`, beside a planar
/// rest read at `U` naming member `t1`'s face, on the same pair: the
/// two fold, and the pair determines. Two members would leave the
/// first mate a lone prismatic tree edge, refused UNDER.
#[test]
fn a2_two_spellings_through_a_union_fold_into_one_pair() {
    let s = scene("msolve13-a2-union");
    let (doc, t1) = insert(
        s.doc.clone(),
        xform(s.top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0),
    );
    let (doc, t2) = insert(doc, xform(s.top, [0.0, 0.0, 20.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: vec![t1, t2],
            declare: Vec::new(),
        },
    );
    let (doc, m2) = mated(doc, seat(s.base_cap(s.base2), head_at(t2, s.top_cap())));
    let (doc, coax) = mated(
        doc,
        seat_with(
            s.base_cap(s.base1),
            head_at(t1, s.top_cap()),
            MatePrimitive::Coaxial,
            Some(0.0),
        ),
    );
    let (doc, plane) = mated(
        doc,
        seat_with(
            s.base_cap(s.base1),
            head_at(union, member_name(union, t1, s.top_cap())),
            MatePrimitive::PlanarRest { offset: 0.0 },
            None,
        ),
    );
    let at_t1 = member_of(&doc, &head_at(t1, s.top_cap()));
    let at_u = member_of(&doc, &head_at(union, member_name(union, t1, s.top_cap())));
    assert!(at_t1.is_some(), "the t1 spelling is a member");
    assert_eq!(at_t1, at_u, "A2: one placement, one member");
    let poses = solve(&doc, &s.opts, Tol::witness());
    for node in [coax, plane, m2] {
        assert!(
            poses.fault(node).is_none(),
            "A2: the pair folds and determines ({node:?}): {:?}",
            poses.fault(node)
        );
    }
    let ev = run(&doc, &s.opts);
    let gated = gate(&doc, &ev);
    assert!(gated.is_ok(), "A2: the gate holds: {gated:?}");
}

/// **`Part { P, 1 }` and `P` naming copy 1 are one placement.** The
/// coaxial-with-rider mate read at the `Part`, the planar rest read at
/// the pattern naming copy 1: one member, so the pair folds and
/// determines, and the gate holds both — the pattern's row lifts
/// through the `Part` verbatim.
#[test]
fn a2_a_part_and_its_pattern_naming_one_copy_fold_into_one_pair() {
    let s = scene_with("msolve13-a2-part", false);
    let (doc, pattern) = insert(
        s.doc.clone(),
        Node::Pattern {
            input: s.top,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(0.0), scl(1.0), scl(0.0)],
                spacing: len(5.0),
            },
        },
    );
    let (doc, part) = insert(
        doc,
        Node::Part {
            of: pattern,
            select: PartSelect::Instance(Expr::count(1)),
        },
    );
    let copy1 = in_copy(pattern, 1, s.top_cap());
    let (doc, coax) = mated(
        doc,
        seat_with(
            s.base_cap(s.base1),
            head_at(part, copy1.clone()),
            MatePrimitive::Coaxial,
            Some(0.0),
        ),
    );
    let (doc, plane) = mated(
        doc,
        seat_with(
            s.base_cap(s.base1),
            head_at(pattern, copy1.clone()),
            MatePrimitive::PlanarRest { offset: 0.0 },
            None,
        ),
    );
    let poses = solve(&doc, &s.opts, Tol::witness());
    for node in [coax, plane] {
        assert!(
            poses.fault(node).is_none(),
            "A2: the pair folds and determines ({node:?}): {:?}",
            poses.fault(node)
        );
    }
    assert_eq!(
        member_of(&doc, &head_at(part, copy1.clone())),
        member_of(&doc, &head_at(pattern, copy1)),
        "A2: one placement, one member"
    );
    let ev = run(&doc, &s.opts);
    let gated = gate(&doc, &ev);
    assert!(
        gated.is_ok(),
        "A2: the gate holds both spellings: {gated:?}"
    );
}

// ---- the lift, per consumer kind ----

/// A `w x w x h` block extruded in the document itself, its corner at
/// `at`.
fn local_block(doc: ProfileDoc, at: [f64; 3], w: f64, h: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        at,
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (w, 0.0), (w, w), (0.0, w)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(h),
            side: ExtrudeSide::Along,
        },
    )
}

/// `top` lifted by `t1` and mated to the first slab at `t1`: the
/// opposed seat turns the block a half turn about its corner, so it
/// stands at `[0, 1] x [0, 1] x [1, 4]` in the world.
fn seated_at_t1(s: &Scene) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, t1) = insert(
        s.doc.clone(),
        xform(s.top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0),
    );
    let (doc, mate) = mated(doc, seat(s.base_cap(s.base1), head_at(t1, s.top_cap())));
    (doc, t1, mate)
}

/// **A pair boolean carries an operand's intact face as `FromA`.** A
/// union boolean of the seated block with a block far from it: the
/// mate read at `t1` lifts through the boolean, and the gate holds.
#[test]
fn a_pair_boolean_above_the_operand_carries_the_face() {
    let s = scene_with("msolve13-lift-boolean", false);
    let (doc, t1, _) = seated_at_t1(&s);
    let (doc, far) = local_block(doc, [10.0, 10.0, 10.0], 1.0, 1.0);
    let (doc, fused) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: t1,
            b: far,
            declare: Vec::new(),
        },
    );
    assert!(doc.roots().contains(&fused), "{:?}", doc.roots());
    let ev = run(&doc, &s.opts);
    let gated = gate(&doc, &ev);
    assert!(gated.is_ok(), "the boolean carries the face: {gated:?}");
}

/// The one reference refusal a gate raised.
fn the_refusal(err: &AssemblyError) -> (RecipeNodeId, MateSide, RefusedRef) {
    let AssemblyError::Mint { refusals } = err else {
        panic!("expected the reference refusal, got {err:?}");
    };
    let [
        MintRefusal::Reference {
            mate, side, why, ..
        },
    ] = refusals.as_slice()
    else {
        panic!("expected one reference refusal, got {refusals:?}");
    };
    (*mate, *side, why.clone())
}

/// **`Vanished` blames the consumer that LOST the face, not a
/// bystander.** A datum reads the seated block's top cap at `t1` — a
/// seat that holds no face, inserted first — and an empty intersection
/// with a far block is the root over `t1`. The boolean's table holds
/// no `FromA` row for the mate's face, so the boolean is the node that
/// consumed it; the datum merely reads beside it.
#[test]
fn vanished_names_the_consumer_that_lost_the_face_not_a_reading_datum() {
    let s = scene_with("msolve13-lift-bystander", false);
    let (doc, t1, mate) = seated_at_t1(&s);
    let (doc, datum) = insert(
        doc,
        Node::Datum(editor_core::Datum::FaceFrame {
            at: t1,
            face: in_part(s.top, s.top_body, CapEnd::End),
            spin: fixture::ang(0.0),
        }),
    );
    let (doc, far) = local_block(doc, [10.0, 10.0, 10.0], 1.0, 1.0);
    let (doc, empty) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Intersect,
            a: t1,
            b: far,
            declare: Vec::new(),
        },
    );
    let positions = doc.positions();
    assert!(
        positions[&datum] < positions[&empty],
        "the premise: the datum is the earlier consumer"
    );
    let ev = run(&doc, &s.opts);
    let err = gate(&doc, &ev).expect_err("the boolean consumes the face");
    assert_eq!(
        the_refusal(&err),
        (mate, MateSide::B, RefusedRef::Vanished { by: Some(empty) })
    );
}

/// Every edge of the block, as `t1` names them.
fn every_edge(ev: &editor_core::Evaluation<f64>, t1: RecipeNodeId) -> Vec<StableName> {
    fixture::table(ev, t1)
        .iter()
        .filter(|(n, _)| n.kind == editor_core::EntityKind::Edge)
        .map(|(n, _)| n.clone())
        .collect()
}

/// **A chamfer carries a face it trims as `FromTarget`** (a fillet
/// is the same blend translation, `names::emit_blend`, under its own
/// node). Chamfering every edge of the seated block — the blend's
/// corner rule asks for all three edges at a corner — trims the mated
/// bottom cap but keeps it one face: the mate read at `t1` lifts
/// through the chamfer, and the gate holds. (A chamfer rather than a
/// fillet because the census cannot yet decide a curved face beside
/// another part.)
#[test]
fn a_chamfer_above_the_operand_carries_the_face_it_trims() {
    let s = scene_with("msolve13-lift-chamfer", false);
    let (doc, t1, _) = seated_at_t1(&s);
    let edges = every_edge(&run(&doc, &s.opts), t1);
    let (doc, chamfer) = insert(
        doc,
        Node::Chamfer {
            target: t1,
            distance: len(0.1),
            selection: edges,
        },
    );
    assert!(doc.roots().contains(&chamfer), "{:?}", doc.roots());
    let ev = run(&doc, &s.opts);
    let gated = gate(&doc, &ev);
    assert!(gated.is_ok(), "the chamfer carries the face: {gated:?}");
}

/// **A shell carries a surviving face as `FromTarget`, and an opened
/// one vanishes, naming the shell.** Opening the block's top cap keeps
/// the mated bottom cap as the outer wall's face, and the gate holds;
/// opening the mated bottom cap itself leaves no face under its name,
/// and the reference refuses `Vanished { by: shell }`.
#[test]
fn a_shell_above_the_operand_carries_a_survivor_and_loses_an_opened_face() {
    let s = scene_with("msolve13-lift-shell", false);
    let (doc0, t1, mate) = seated_at_t1(&s);
    let shell = |open: StableName| {
        insert(
            doc0.clone(),
            Node::Shell {
                target: t1,
                thickness: len(0.1),
                open: vec![open],
            },
        )
    };
    let (doc, _) = shell(in_part(s.top, s.top_body, CapEnd::End));
    let ev = run(&doc, &s.opts);
    let gated = gate(&doc, &ev);
    assert!(gated.is_ok(), "the shell carries the bottom cap: {gated:?}");

    let (doc, opened) = shell(s.top_cap());
    let ev = run(&doc, &s.opts);
    let err = gate(&doc, &ev).expect_err("the shell opened the mated face");
    assert_eq!(
        the_refusal(&err),
        (mate, MateSide::B, RefusedRef::Vanished { by: Some(opened) })
    );
}

/// **A `Part` above a union selects nothing the walk could check, and
/// the evaluation refuses it.** The member walk passes a `Part` and a
/// transform down to the union, where it continues at the member the
/// name says, so the `Part`'s index meets no pattern to agree with.
/// That is because there is no copy to select: the union is one body,
/// and a `Part` naming an instance of it refuses at evaluation
/// (`wrong_operand`), so the gather refuses at that root before any
/// mate's face is read.
#[test]
fn a_part_above_a_union_refuses_at_evaluation_before_the_gate_reads_it() {
    let s = scene_with("msolve13-part-over-union", false);
    let (doc, t1) = insert(
        s.doc.clone(),
        xform(s.top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0),
    );
    let (doc, far) = local_block(doc, [10.0, 10.0, 10.0], 1.0, 1.0);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: vec![t1, far],
            declare: Vec::new(),
        },
    );
    let (doc, moved) = insert(doc, xform(union, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, part) = insert(
        doc,
        Node::Part {
            of: moved,
            select: PartSelect::Instance(Expr::count(0)),
        },
    );
    let head = head_at(part, member_name(union, t1, s.top_cap()));
    let m = member_of(&doc, &head).expect("the walk descends the union below the Part");
    assert_eq!(m.instance, s.top);
    let (doc, mate) = mated(doc, seat(s.base_cap(s.base1), head));
    assert!(
        solve(&doc, &s.opts, Tol::witness()).fault(mate).is_none(),
        "the solve has no index to check"
    );
    let ev = run(&doc, &s.opts);
    assert!(
        matches!(ev.result(part), Some(editor_core::NodeResult::Failed(_))),
        "a Part over one body refuses: {:?}",
        ev.result(part)
    );
    let err = gate(&doc, &ev).expect_err("the gather refuses the failed root");
    assert!(
        matches!(
            &err,
            AssemblyError::Product(e)
                if matches!(**e, editor_core::ProductError::Root(editor_core::NodeStanding::Failed { node }) if node == part)
        ),
        "the gather refuses at the Part, before any reference is read: {err:?}"
    );
}
