//! **A mate reads its face where it says.**
//!
//! A mate's target is a pose read off a selection of the body it is
//! read in, and the at-rest gate mints the mate's declaration on the
//! world copy of that body: on the placement that reads it. A target
//! whose body no placement reads — one read below a union, a
//! transform, a blend or a shell that is placed in its stead — has no
//! world copy, and the mate mints nothing.
//!
//! The rows build the documents the gather's own recourse leads to — a
//! union over two transforms of one pinned instance, the slabs seated
//! under it — and measure the gate's verdict on each, the member walk's
//! descent through the union, the member key that makes two spellings
//! of one placement one member, and what split and inline answer across
//! a mate read at a union (A3).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use editor_core::{
    CapEnd, DocumentId, EvalOptions, Formula, MateRef, Node, PartSelect, PatternKind, ProfileDoc,
    RecipeNodeId, SitedFace, Space, SplitError, StableName, member_of,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{
    gate, head, head_at, in_copy, insert, len, member_name, on_frame, run, scl, solve, xform,
};
use geom_core::Tol;

// ---- the scene ----

/// **The copy of `body` a node over an instance carries** — `node`'s
/// copies keyed as the instance keys the copy of its part's `body`
/// ([`fixture::resolver::copy_key`]): what a seat reading one body
/// reads of a transform over the instance.
fn one_copy(node: RecipeNodeId, body: RecipeNodeId) -> editor_core::BodyRead<Formula> {
    editor_core::BodyRead::keyed(
        editor_core::Operand::Node(node),
        fixture::resolver::copy_key(body),
    )
}

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
/// part bodies. The block is pinned to the world at the identity; the
/// slabs' bundles are empty, and sit where the constraints added to
/// them put them.
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
/// `base1` itself, and nothing is left loose.
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
    let (doc, base1) = insert(doc, fixture::mated_instance(base_ref));
    let (doc, base2) = if two {
        insert(doc, fixture::mated_instance(base_ref))
    } else {
        (doc, base1)
    };
    let (doc, top) = fixture::insert_pinned(doc, Node::instantiate_part(top_ref), &opts);
    // Each row seats the slabs under what it builds over `top`.
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

/// **`slab` seated under the block**: the slab's top cap rests on
/// `block`, a face of the block read wherever the row says, in the
/// slab's bundle, its corner under the block's
/// ([`crate::p2_split::seat_at`]).
fn seat(doc: ProfileDoc, slab: RecipeNodeId, cap: SitedFace, block: SitedFace) -> (ProfileDoc, MateRef) {
    crate::p2_split::seat_at(doc, slab, cap, block, [0.0, 0.0])
}

/// **One copy of `body` in the world, pinned at the identity** — the
/// copies read off the parts `o` serves.
fn place(doc: ProfileDoc, body: RecipeNodeId, o: &EvalOptions) -> (ProfileDoc, RecipeNodeId) {
    fixture::insert_pinned(
        doc,
        Node::place(vec![editor_core::BodyRead::plain(body)]),
        o,
    )
}

/// `top` lifted by `t1` (z + 10) and `t2` (z + 20), and the two
/// transforms fused by a union `U`, placed — the gather's recourse for
/// `PlacedUnderTwoRoots`, followed. `m1` seats the first slab under the
/// block through `t1`, `m2` the second through `t2`, each in its slab's
/// bundle. Each mate reads the block's face as `at_t1` says.
struct Fused {
    doc: ProfileDoc,
    t1: RecipeNodeId,
    t2: RecipeNodeId,
    union: RecipeNodeId,
    m1: MateRef,
    m2: MateRef,
}

fn fused(s: &Scene, at_t1: impl Fn(&ProfileDoc, RecipeNodeId, RecipeNodeId) -> SitedFace) -> Fused {
    let (doc, t1) = insert(
        s.doc.clone(),
        xform(s.top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0),
    );
    let (doc, t2) = insert(doc, xform(s.top, [0.0, 0.0, 20.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                one_copy(t1, s.top_body),
                one_copy(t2, s.top_body),
            ]),
        },
    );
    let (doc, _) = place(doc, union, &s.opts);
    let (at1, at2) = (at_t1(&doc, t1, union), at_t1(&doc, t2, union));
    let (doc, m1) = seat(doc, s.base1, s.base_cap(s.base1), at1);
    let (doc, m2) = seat(doc, s.base2, s.base_cap(s.base2), at2);
    Fused {
        doc,
        t1,
        t2,
        union,
        m1,
        m2,
    }
}

/// The solve places every instance, and both seats pin their copies
/// into the world.
fn assert_placed(s: &Scene, f: &Fused, what: &str) {
    let poses = solve(&f.doc, &s.opts, Tol::witness());
    for node in [s.base1, s.base2, s.top] {
        assert!(
            poses.fault(node).is_none(),
            "{what}: the solve places {node:?}: {:?}",
            poses.fault(node)
        );
    }
    assert_eq!(
        (poses.space(f.m1.placement), poses.space(f.m2.placement)),
        (Some(Space::World), Some(Space::World)),
        "{what}: both seats pin their copies"
    );
}

// ---- A1 (a): the recourse, followed ----

/// **A union placed over two transforms of a pinned instance**: each
/// slab's mate reads the block's cap at its own transform, which no
/// placement reads, so neither mints; read at the union instead, both
/// do (A1(c)). The union's copy rests on both slabs where the mates
/// seated them, and with nothing declared the at-rest census reports
/// each seat as an unattributed contact: a mate read below a placed
/// consumer states nothing about the product.
#[test]
fn a1a_mates_read_at_the_transforms_below_a_placed_union_mint_nothing() {
    let s = scene("msolve13-a1a");
    let f = fused(&s, |_, t1, _| head_at(t1, s.top_cap()));
    assert_placed(&s, &f, "A1(a)");
    let ev = run(&f.doc, &s.opts);
    crate::fixture::assert_mints_nothing(&f.doc, &ev, f.m1);
    crate::fixture::assert_mints_nothing(&f.doc, &ev, f.m2);
    assert!(
        gate(&f.doc, &ev).is_ok(),
        "A1(a): the unmated seats do not refuse"
    );
    crate::fixture::assert_copies_rows_alone(&f.doc, &ev);
}

// ---- A1 (b): a transform above the operand ----

/// **`t3 = Transform(t1)` is placed, and the mate reads at `t1`.**
/// The solve seats the slab under the block where `t1` moved it; the
/// product holds the face where `t3` moved it. No placement reads `t1`,
/// so the mate has no world copy to mint on and states nothing about
/// the product.
#[test]
fn a1b_a_transform_above_the_operand_mints_nothing() {
    let s = scene("msolve13-a1b");
    let (doc, t1) = insert(
        s.doc.clone(),
        xform(s.top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0),
    );
    let (doc, t3) = insert(doc, xform(t1, [5.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, _) = place(doc, t3, &s.opts);
    let (doc, mate) = seat(
        doc,
        s.base1,
        s.base_cap(s.base1),
        head_at(t1, s.top_cap()),
    );
    let poses = solve(&doc, &s.opts, Tol::witness());
    assert!(
        poses.fault(mate.placement).is_none(),
        "the solve seats the slab at t1: {:?}",
        poses.fault(mate.placement)
    );
    assert_eq!(poses.space(mate.placement), Some(Space::World));
    let ev = run(&doc, &s.opts);
    crate::fixture::assert_mints_nothing(&doc, &ev, mate);
}

// ---- A1 (c): a pick on the fused body ----

/// **A target read at the union, naming member `t1`'s face**, is what
/// a pick on the fused body authors. The member walk descends the union
/// at the member the name says, and the mate is valid: it solves, and
/// the gate holds.
#[test]
fn a1c_a_reference_read_at_the_union_is_a_member_and_gates() {
    let s = scene("msolve13-a1c");
    let f = fused(&s, |doc, t1, union| {
        head_at(union, member_name(doc, union, t1, s.top_cap()))
    });
    let r = head_at(f.union, member_name(&f.doc, f.union, f.t1, s.top_cap()));
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

/// **Where the first slab seated through `spelling` solves**: `doc`
/// with the slab's seat added, and the slab's solved pose.
fn seated_through(
    s: &Scene,
    doc: ProfileDoc,
    spelling: SitedFace,
) -> (ProfileDoc, editor_core::Frame) {
    let (doc, mate) = seat(doc, s.base1, s.base_cap(s.base1), spelling);
    let poses = solve(&doc, &s.opts, Tol::witness());
    assert!(
        poses.fault(mate.placement).is_none(),
        "A2: the seat solves: {:?}",
        poses.fault(mate.placement)
    );
    assert_eq!(
        poses.space(s.base1),
        Some(Space::World),
        "A2: the slab is pinned"
    );
    let at = poses.placement(&doc, s.base1).expect("the slab has a pose");
    (doc, at)
}

/// Whether two poses agree to well inside the witness band.
fn near(a: &editor_core::Frame, b: &editor_core::Frame) -> bool {
    let (a, b) = (a.affine::<f64>(), b.affine::<f64>());
    let cols = |m: &geom_core::linalg::Affine3<f64>| {
        [m.linear.c0, m.linear.c1, m.linear.c2, m.translation]
    };
    cols(&a)
        .iter()
        .zip(cols(&b))
        .all(|(p, q)| (p.x - q.x).abs() <= 1e-9 && (p.y - q.y).abs() <= 1e-9 && (p.z - q.z).abs() <= 1e-9)
}

/// **One placement spelled two ways is one member.** The block's cap
/// read at `t1`, and read at `U` naming member `t1`'s face: the member
/// walk lands both on the block, and a slab seated through either
/// spelling stands at the same pose, where the gate holds. Two members
/// would put the two seats on two copies.
#[test]
fn a2_two_spellings_through_a_union_are_one_member() {
    let s = scene_with("msolve13-a2-union", false);
    let (doc, t1) = insert(
        s.doc.clone(),
        xform(s.top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0),
    );
    let (doc, t2) = insert(doc, xform(s.top, [0.0, 0.0, 20.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                one_copy(t1, s.top_body),
                one_copy(t2, s.top_body),
            ]),
        },
    );
    let (doc, _) = place(doc, union, &s.opts);
    let at_t1 = member_of(&doc, &head_at(t1, s.top_cap()));
    let through_union = head_at(union, member_name(&doc, union, t1, s.top_cap()));
    let at_u = member_of(&doc, &through_union);
    assert!(at_t1.is_some(), "the t1 spelling is a member");
    assert_eq!(at_t1, at_u, "A2: one placement, one member");
    let (_, by_t1) = seated_through(&s, doc.clone(), head_at(t1, s.top_cap()));
    let (by_union_doc, by_union) = seated_through(&s, doc, through_union);
    assert!(
        near(&by_t1, &by_union),
        "A2: the two spellings seat the slab alike: {by_t1:?} vs {by_union:?}"
    );
    let ev = run(&by_union_doc, &s.opts);
    let gated = gate(&by_union_doc, &ev);
    assert!(gated.is_ok(), "A2: the gate holds: {gated:?}");
}

/// **`Part { P, 1 }` and `P` naming copy 1 are one placement.** The
/// block's copy-1 cap read at the `Part`, and read at the pattern
/// naming copy 1: one member, so a slab seated through either stands at
/// the same pose, and the gate holds — the pattern's row lifts through
/// the `Part` verbatim.
#[test]
fn a2_a_part_and_its_pattern_naming_one_copy_are_one_member() {
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
    let (doc, _) = place(doc, part, &s.opts);
    let copy1 = in_copy(pattern, 1, s.top_cap());
    assert_eq!(
        member_of(&doc, &head_at(part, copy1.clone())),
        member_of(&doc, &head_at(pattern, copy1.clone())),
        "A2: one placement, one member"
    );
    let (_, by_part) = seated_through(&s, doc.clone(), head_at(part, copy1.clone()));
    let (by_pattern_doc, by_pattern) = seated_through(&s, doc, head_at(pattern, copy1));
    assert!(
        near(&by_part, &by_pattern),
        "A2: the two spellings seat the slab alike: {by_part:?} vs {by_pattern:?}"
    );
    let ev = run(&by_pattern_doc, &s.opts);
    let gated = gate(&by_pattern_doc, &ev);
    assert!(
        gated.is_ok(),
        "A2: the gate holds the pattern's spelling: {gated:?}"
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

/// `top` lifted by `t1`, and the first slab seated under the block
/// where `t1` moved it.
fn seated_at_t1(s: &Scene) -> (ProfileDoc, RecipeNodeId, MateRef) {
    let (doc, t1) = insert(
        s.doc.clone(),
        xform(s.top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0),
    );
    let (doc, mate) = seat(
        doc,
        s.base1,
        s.base_cap(s.base1),
        head_at(t1, s.top_cap()),
    );
    (doc, t1, mate)
}

/// **A consumer placed over the operand reads it; the mate read at the
/// operand mints nothing**: a pair boolean that keeps the face, over
/// the one copy the transform carries. The world copy is the
/// consumer's. (A chamfer or a shell reads a selection, whose body is
/// one body, and a transform over an instance carries its copies, so
/// neither is written over the operand.)
#[test]
fn a_mate_read_below_a_placed_boolean_mints_nothing() {
    let s = scene_with("msolve13-lift-consumers", false);
    let (doc0, t1, mate) = seated_at_t1(&s);
    let (with_far, far) = local_block(doc0, [10.0, 10.0, 10.0], 1.0, 1.0);
    let (doc, fused) = insert(
        with_far,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![one_copy(t1, s.top_body), far.into()]),
        },
    );
    let doc = place(doc, fused, &s.opts).0;
    let ev = run(&doc, &s.opts);
    assert!(
        ev.result(mate.placement)
            .is_some_and(|r| r.value().is_some()),
        "the mate's placement evaluates"
    );
    crate::fixture::assert_mints_nothing(&doc, &ev, mate);
}

/// **A `Part` above a union is refused at the door.** A `Part` picks
/// one copy out of a pattern's copies and reads nothing else (DM3): the
/// union is one body, so a `Part` over it refuses `SlotVarKind` at its
/// source before a member walk or a gather could meet it.
#[test]
fn a_part_above_a_union_is_refused_at_the_door() {
    let s = scene_with("msolve13-part-over-union", false);
    let (doc, t1) = insert(
        s.doc.clone(),
        xform(s.top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0),
    );
    let (doc, far) = local_block(doc, [10.0, 10.0, 10.0], 1.0, 1.0);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![one_copy(t1, s.top_body), far.into()]),
        },
    );
    let (doc, moved) = insert(doc, xform(union, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0));
    let refusal = fixture::insert_refused(
        &doc,
        Node::Part {
            of: moved.into(),
            select: PartSelect::Instance(Formula::count(0)),
        },
    );
    assert!(
        matches!(
            &refusal,
            editor_core::EditError::SlotVarKind {
                slot: editor_core::SlotId::Operand(editor_core::OperandSlot::Of),
                found: editor_core::VarKind::Body,
                expected: editor_core::SlotKind::Is(editor_core::VarKind::Bodies),
                ..
            }
        ),
        "{refusal:?}"
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
    doc.placements()
        .into_iter()
        .find(|&p| {
            matches!(doc.node(p), Some(Node::Place { shapes, .. })
                if shapes.iter().any(|shape| doc.operation_of(shape.read) == Some(body)))
        })
        .expect("the body is placed")
}

/// **A3(a).** A cut that takes the union a mate reads, and its
/// placement, but not the transforms below it, is the union's own
/// severed read: the cut union reads a kept transform. The mate reads
/// the union, which moves whole with its placement, and says nothing.
#[test]
fn a3a_a_cut_taking_the_union_but_not_its_members_severs_the_union() {
    let s = scene("msolve13-a3a");
    let f = fused(&s, |doc, t1, union| {
        head_at(union, member_name(doc, union, t1, s.top_cap()))
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

/// **A3(b).** The reverse: a cut taking the slab whose bundle holds
/// the mate, the block and the transforms below the union, and leaving
/// the union, is the kept union's severed read.
#[test]
fn a3b_a_cut_taking_the_members_but_not_the_union_severs_the_union() {
    let s = scene_with("msolve13-a3b", false);
    let (doc, t1) = insert(
        s.doc.clone(),
        xform(s.top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0),
    );
    let (doc, t2) = insert(doc, xform(s.top, [0.0, 0.0, 20.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                one_copy(t1, s.top_body),
                one_copy(t2, s.top_body),
            ]),
        },
    );
    let (doc, _) = place(doc, union, &s.opts);
    let (doc, _) = seat(
        doc,
        s.base1,
        s.base_cap(s.base1),
        head_at(t1, s.top_cap()),
    );
    let err = split_of(&s, &doc, &[s.base1, s.top, t1, t2], "msolve13-a3b-part")
        .expect_err("the kept union reads the cut transforms");
    assert!(
        matches!(
            &err,
            SplitError::SeveredEdge { consumer, consumer_is_cut: false, .. }
                if *consumer == doc.spoken(union)
        ),
        "{err:?}"
    );
}

/// **A3(c).** A kept constraint — the second slab's seat — whose
/// target reads the cut union, naming its member's face, crosses: the
/// union's placement is cut at the identity, so its copy stands in the
/// instance where it stood, and the target is written afresh over the
/// instance's copies. The second slab stands where it stood.
///
/// *Breaks if* the kept seat refuses, keeps reading the cut union, or
/// moves the slab.
#[test]
fn a3c_a_mate_read_through_a_union_crosses_onto_the_instance() {
    let s = scene("msolve13-a3c");
    let f = fused(&s, |doc, t1, union| {
        head_at(union, member_name(doc, union, t1, s.top_cap()))
    });
    let up = placement_of(&f.doc, f.union);
    let before = solve(&f.doc, &s.opts, Tol::witness())
        .placement(&f.doc, s.base2)
        .expect("the second slab is seated");
    let out = split_of(
        &s,
        &f.doc,
        &[s.base1, s.top, f.t1, f.t2, f.union, up],
        "msolve13-a3c-part",
    )
    .unwrap_or_else(|e| panic!("the kept seat crosses: {e}"));
    let seat = fixture::constraint(&out.remainder, f.m2);
    let [select] = fixture::face_reads(&out.remainder, *seat.to.pose().expect("a pose target"))[..]
    else {
        panic!("the seat's target reads one face")
    };
    assert_eq!(
        out.remainder.selection(select).map(|s| s.body),
        out.remainder.output(out.instance, 0),
        "the kept seat's target reads the instance's copies"
    );
    let mut store = s.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let after = solve(&out.remainder, &with_resolver(store), Tol::witness())
        .placement(&out.remainder, s.base2)
        .expect("the second slab is seated after the split");
    assert!(
        near(&before, &after),
        "the second slab is where it was: {before:?} vs {after:?}"
    );
}

/// **A3(d).** A whole assembly with its mate read at a union — the
/// slab whose bundle holds the mate, the block, a transform of it and
/// their union — cuts into a part and inlines back: `inline(split(d))`
/// is `d` up to node ids, the mate still reading the union naming the
/// block's face, and the gate holds on the result. The union is read,
/// not placed: the block is its own placement, so a placement of a
/// union over its copy would be a second copy of the block where it
/// stands.
#[test]
fn a3d_an_assembly_with_a_mate_read_at_a_union_splits_and_inlines_back() {
    let s = scene_with("msolve13-a3d", false);
    let (doc, t1) = insert(
        s.doc.clone(),
        xform(s.top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0),
    );
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                fixture::copy(s.top, s.top_body),
                one_copy(t1, s.top_body),
            ]),
        },
    );
    let through_top = member_name(&doc, union, s.top, s.top_cap());
    let (doc, _) = seat(
        doc,
        s.base1,
        s.base_cap(s.base1),
        head_at(union, through_top),
    );
    let ev = run(&doc, &s.opts);
    let gated = gate(&doc, &ev);
    assert!(gated.is_ok(), "A3(d): the document gates: {gated:?}");
    let cut = fixture::with_placements(&doc, &[s.base1, s.top, t1, union].into_iter().collect());
    let cut: Vec<_> = cut.into_iter().collect();
    let out = split_of(&s, &doc, &cut, "msolve13-a3d-part")
        .unwrap_or_else(|e| panic!("A3(d): the whole assembly cuts: {e}"));
    let mut store = s.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let back = editor_core::inline(
        &out.remainder,
        out.instance,
        &(std::sync::Arc::new(store.clone()) as std::sync::Arc<dyn editor_core::PartResolver>),
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
