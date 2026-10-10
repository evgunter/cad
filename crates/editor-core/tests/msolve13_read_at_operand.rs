//! **A mate reads its face where it says.**
//!
//! A mate side is a selection of the body it is read in, and the
//! at-rest gate mints the mate's declaration on the world copy of that
//! body: on the placement that reads it. A side whose body no placement
//! reads — one read below a union, a transform, a blend or a shell that
//! is placed in its stead — has no world copy, and the mate mints
//! nothing.
//!
//! The rows build the documents the gather's own recourse leads to — a
//! union over two transforms of one mated instance — and measure the
//! gate's verdict on each, the member walk's descent through the
//! union, the member key that makes two spellings of one placement
//! one member, and what split and inline answer across a mate read at
//! a union (A3).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::AuthoredNode;
use editor_core::ExtrudeSide;

use editor_core::{
    Alignment, AssemblyError, AxisSense, CapEnd, ContactClass, DocEdit, DocumentId, EvalOptions,
    Formula, MateFrame, MatePrimitive, MateRole, Node, PartSelect, PatternKind, ProfileDoc,
    RecipeNodeId, SitedFace, SplitError, StableName, member_of,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{
    gate, head, head_at, in_copy, insert, len, member_name, on_frame, run, scl, solve, step, xform,
};
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
            profile: profile.into(),
            distance: len(h),
            side: ExtrudeSide::Along,
        },
    )
}

/// Two slab instances and one block instance, with their resolver and
/// part bodies, the slabs placed in the world. The first slab carries
/// the world offset and roots the group; the others sit where their
/// mates put them.
struct Scene {
    doc: ProfileDoc,
    store: PartStore,
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
    let opts = with_resolver(store.clone());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base1) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, base2) = if two {
        insert(doc, fixture::mated_instance(base_ref))
    } else {
        (doc, base1)
    };
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    // The slabs are in the world; each row places what it builds over
    // `top`.
    let bases: &[RecipeNodeId] = if two { &[base1, base2] } else { &[base1] };
    let doc = fixture::place_all(doc, bases);
    Scene {
        doc,
        store,
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
) -> AuthoredNode {
    Node::Mate {
        a: a.into(),
        b: b.into(),
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

fn seat(a: SitedFace, b: SitedFace) -> AuthoredNode {
    seat_with(a, b, MatePrimitive::FrameCoincidence, None)
}

/// Insert `mate` and answer its id.
fn mated(doc: ProfileDoc, mate: AuthoredNode) -> (ProfileDoc, RecipeNodeId) {
    let (doc, id) = step(
        doc,
        DocEdit::InsertNode {
            node: Box::new(mate),
            fresh: Vec::new(),
        },
    );
    (doc, id.expect("the mate inserts"))
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
            members: vec![t1.into(), t2.into()],
            declare: Vec::new(),
        },
    );
    let (doc, _) = crate::fixture::place(doc, union);
    let (doc, m1) = mated(doc, seat(s.base_cap(s.base1), at_t1(t1, union)));
    let (doc, m2) = mated(doc, seat(s.base_cap(s.base2), at_t1(t2, union)));
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

/// **A union placed over two transforms of a mated instance**: each
/// mate reads the block's cap at its own transform, which no placement
/// reads, so neither mints; read at the union instead, both do (A1(c)).
#[test]
fn a1a_mates_read_at_the_transforms_below_a_placed_union_mint_nothing() {
    let s = scene("msolve13-a1a");
    let f = fused(&s, |t, _| head_at(t, s.top_cap()));
    assert_placed(&s, &f, "A1(a)");
    let ev = run(&f.doc, &s.opts);
    crate::fixture::assert_mints_nothing(&f.doc, &ev, f.m1);
    crate::fixture::assert_mints_nothing(&f.doc, &ev, f.m2);
}

// ---- A1 (b): a transform above the operand ----

/// **`t3 = Transform(t1)` is placed, and the mate reads at `t1`.**
/// The solve seats the block at `t1`'s placement; the product holds
/// the face where `t3` moved it. No placement reads `t1`, so the mate
/// has no world copy to mint on and states nothing about the product.
#[test]
fn a1b_a_transform_above_the_operand_mints_nothing() {
    let s = scene("msolve13-a1b");
    let (doc, t1) = insert(
        s.doc.clone(),
        xform(s.top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0),
    );
    let (doc, t3) = insert(doc, xform(t1, [5.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, _) = crate::fixture::place(doc, t3);
    let (doc, mate) = mated(doc, seat(s.base_cap(s.base1), head_at(t1, s.top_cap())));
    let poses = solve(&doc, &s.opts, Tol::witness());
    assert!(
        poses.fault(mate).is_none(),
        "the solve places the mate at t1: {:?}",
        poses.fault(mate)
    );
    assert_eq!(poses.role(mate), Some(MateRole::Determining));
    let ev = run(&doc, &s.opts);
    crate::fixture::assert_mints_nothing(&doc, &ev, mate);
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
            members: vec![t1.into(), t2.into()],
            declare: Vec::new(),
        },
    );
    let (doc, _) = crate::fixture::place(doc, union);
    let (doc, m2) = mated(
        doc,
        seat(
            s.base_cap(s.base2),
            head_at(union, member_name(union, t2, s.top_cap())),
        ),
    );
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
            input: s.top.into(),
            count: Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(0.0), scl(1.0), scl(0.0)],
                spacing: len(5.0),
            },
        },
    );
    let (doc, part) = insert(
        doc,
        Node::Part {
            of: pattern.into(),
            select: PartSelect::Instance(Formula::count(1)),
        },
    );
    let (doc, _) = crate::fixture::place(doc, part);
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
            profile: profile.into(),
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

/// **A consumer placed over the operand reads it; the mate read at the
/// operand mints nothing**, whatever the consumer does with the face —
/// a pair boolean that keeps it, a chamfer that trims it, a shell that
/// keeps it as a wall or opens it. The world copy is the consumer's.
#[test]
fn a_mate_read_below_a_placed_consumer_mints_nothing() {
    let s = scene_with("msolve13-lift-consumers", false);
    let (doc0, t1, mate) = seated_at_t1(&s);
    let placed_over = |node: AuthoredNode| {
        let (doc, consumer) = insert(doc0.clone(), node);
        crate::fixture::place(doc, consumer).0
    };
    let (with_far, far) = local_block(doc0.clone(), [10.0, 10.0, 10.0], 1.0, 1.0);
    let (doc, fused) = insert(
        with_far,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: t1.into(),
            b: far.into(),
            declare: Vec::new(),
        },
    );
    let boolean = crate::fixture::place(doc, fused).0;
    let edges = every_edge(&run(&doc0, &s.opts), t1);
    let chamfer = placed_over(Node::Chamfer {
        distance: len(0.1),
        selection: editor_core::Operand::select(t1, edges),
    });
    let shell = |open: StableName| {
        placed_over(Node::Shell {
            thickness: len(0.1),
            open: editor_core::Operand::select(t1, vec![open]),
        })
    };
    for (what, doc) in [
        ("a pair boolean", boolean),
        ("a chamfer", chamfer),
        (
            "a shell keeping the face",
            shell(in_part(s.top, s.top_body, CapEnd::End)),
        ),
        ("a shell opening the face", shell(s.top_cap())),
    ] {
        let ev = run(&doc, &s.opts);
        assert!(
            ev.result(mate).is_some_and(|r| r.value().is_some()),
            "{what}: the mate itself evaluates"
        );
        crate::fixture::assert_mints_nothing(&doc, &ev, mate);
    }
}

/// Every edge of the block, as `t1` names them.
fn every_edge(ev: &editor_core::Evaluation<f64>, t1: RecipeNodeId) -> Vec<StableName> {
    fixture::table(ev, t1)
        .iter()
        .filter(|(n, _)| n.kind == editor_core::EntityKind::Edge)
        .map(|(n, _)| n.clone())
        .collect()
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
            members: vec![t1.into(), far.into()],
            declare: Vec::new(),
        },
    );
    let (doc, moved) = insert(doc, xform(union, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, part) = insert(
        doc,
        Node::Part {
            of: moved.into(),
            select: PartSelect::Instance(Formula::count(0)),
        },
    );
    let (doc, placement) = crate::fixture::place(doc, part);
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
    let err = gate(&doc, &ev).expect_err("the gather refuses the poisoned placement");
    assert!(
        matches!(
            &err,
            AssemblyError::Product(e)
                if matches!(
                    **e,
                    editor_core::ProductError::Root(editor_core::NodeStanding::Poisoned {
                        node,
                        through,
                    }) if (node, through) == (placement, part)
                )
        ),
        "the gather refuses at the Part's placement, poisoned through the Part, before any \
         reference is read: {err:?}"
    );
}

// ---- A3: split and inline across a mate read at a union ----

/// The split of `cut` out of `doc`, its refusal or outcome.
fn split_of(
    s: &Scene,
    doc: &ProfileDoc,
    cut: &[RecipeNodeId],
    label: &str,
) -> Result<editor_core::SplitOutcome, SplitError> {
    editor_core::split(
        doc,
        &cut.iter().copied().collect(),
        DocumentId::derive(label),
        Tol::witness(),
        s.opts.resolver.as_ref(),
    )
}

/// The placement of `body` in `doc`.
fn placement_of(doc: &ProfileDoc, body: RecipeNodeId) -> RecipeNodeId {
    doc.ids()
        .into_iter()
        .find(|&p| {
            matches!(doc.node(p), Some(Node::PlaceInWorld { .. })) && doc.upstream(p) == vec![body]
        })
        .expect("the body is placed")
}

/// The scene with the slabs on a gauge of their own and the block at an
/// offset, so a mate between them declares rather than welds.
fn declaring(label: &str) -> Scene {
    let s = scene(label);
    let (mut doc, g) = insert(
        s.doc.clone(),
        Node::gauge(
            None,
            editor_core::Placement::literal(&editor_core::Frame::translation([0.0, 0.0, -5.0])),
        ),
    );
    for base in [s.base1, s.base2] {
        doc = step(
            doc,
            DocEdit::SetGauge {
                node: base,
                gauge: Some(g),
            },
        )
        .0;
    }
    let (doc, _) = step(
        doc,
        DocEdit::SetOffset {
            instance: s.top,
            offset: Some(editor_core::Placement::literal(
                &editor_core::Frame::translation([0.0, 0.0, 5.0]),
            )),
            fresh: Vec::new(),
        },
    );
    Scene { doc, ..s }
}

/// **A3(a).** A cut that takes the union a mate reads, and its
/// placement, but not the transforms below it, is the union's own
/// severed read: the cut union reads a kept transform. The mate reads
/// the union, which moves whole with its placement, and says nothing.
#[test]
fn a3a_a_cut_taking_the_union_but_not_its_members_severs_the_union() {
    let s = scene("msolve13-a3a");
    let f = fused(&s, |t1, union| {
        head_at(union, member_name(union, t1, s.top_cap()))
    });
    let up = placement_of(&f.doc, f.union);
    let err = split_of(&s, &f.doc, &[f.union, up], "msolve13-a3a-part")
        .expect_err("the union reads the kept transforms");
    assert!(
        matches!(
            &err,
            SplitError::SeveredEdge { consumer, consumer_is_cut: true, .. }
                if *consumer == f.doc.spoken(f.union)
        ),
        "{err:?}"
    );
}

/// **A3(b).** The reverse: a cut taking the block and the transforms
/// below the union and leaving the union is the kept union's severed
/// read. The mates declare (the slabs and the block each stand where
/// they are put), so no group is torn before the read is reached.
#[test]
fn a3b_a_cut_taking_the_members_but_not_the_union_severs_the_union() {
    let s = declaring("msolve13-a3b");
    let f = fused(&s, |t1, union| {
        head_at(union, member_name(union, t1, s.top_cap()))
    });
    let err = split_of(&s, &f.doc, &[s.top, f.t1, f.t2], "msolve13-a3b-part")
        .expect_err("the kept union reads the cut transforms");
    assert!(
        matches!(
            &err,
            SplitError::SeveredEdge { consumer, consumer_is_cut: false, .. }
                if *consumer == f.doc.spoken(f.union)
        ),
        "{err:?}"
    );
}

/// **A3(c).** A declaring mate whose cut side reads the union through
/// a transform crosses with a frame the part cannot hold: the member's
/// chain carries the transform, so the face's frame is no frame of the
/// instance left behind, and the cut refuses `MateFrameCrosses` on
/// that side.
#[test]
fn a3c_a_declaring_mate_read_through_a_union_and_a_transform_refuses_its_frame() {
    let s = declaring("msolve13-a3c");
    let f = fused(&s, |t1, union| {
        head_at(union, member_name(union, t1, s.top_cap()))
    });
    let up = placement_of(&f.doc, f.union);
    let err = split_of(
        &s,
        &f.doc,
        &[s.top, f.t1, f.t2, f.union, up],
        "msolve13-a3c-part",
    )
    .expect_err("the mates' cut side reads through a transform");
    assert!(
        matches!(
            &err,
            SplitError::MateFrameCrosses { mate, side: editor_core::MateSide::B, promote: None }
                if *mate == f.doc.spoken(f.m1)
        ),
        "{err:?}"
    );
}

/// **A3(d).** A whole group with its mate read at a union — the slab,
/// the block, a transform of it, their union, the placements and the
/// mate — cuts into a part and inlines back: `inline(split(d))` is `d`
/// up to node ids, the mate still reading the union naming the
/// block's face, and the gate holds on the result.
#[test]
fn a3d_a_group_with_a_mate_read_at_a_union_splits_and_inlines_back() {
    let s = scene("msolve13-a3d");
    let (doc, t1) = insert(
        s.doc.clone(),
        xform(s.top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0),
    );
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: vec![s.top.into(), t1.into()],
            declare: Vec::new(),
        },
    );
    let (doc, _) = crate::fixture::place(doc, union);
    let (doc, mate) = mated(
        doc,
        seat(
            s.base_cap(s.base1),
            head_at(union, member_name(union, s.top, s.top_cap())),
        ),
    );
    let ev = run(&doc, &s.opts);
    assert!(gate(&doc, &ev).is_ok(), "A3(d): the document gates");
    let cut = fixture::with_placements(
        &doc,
        &[s.base1, s.top, t1, union, mate].into_iter().collect(),
    );
    let cut: Vec<_> = cut.into_iter().collect();
    let out = split_of(&s, &doc, &cut, "msolve13-a3d-part")
        .unwrap_or_else(|e| panic!("A3(d): the whole group cuts: {e}"));
    let mut store = s.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let back = editor_core::inline(
        &out.remainder,
        out.instance,
        &crate::p2_gauges::resolver(store.clone()),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("A3(d): the part inlines back: {e}"));
    let (map, steps) = fixture::round_trip::composed(&doc, &out, &back);
    fixture::round_trip::same_up_to_ids(&doc, &back.doc, &map, &steps)
        .unwrap_or_else(|e| panic!("A3(d): inline(split(d)) is d up to node ids:\n{e}"));
    let ev = run(&back.doc, &with_resolver(store));
    assert!(
        gate(&back.doc, &ev).is_ok(),
        "A3(d): the gate holds on the round trip"
    );
}
