//! **Split and inline at a gauge** (ASSEMBLY.md A4, A11 (2)–(5); the
//! spec's `## P2-split` rows S2–S6, I1–I5 and R1).
//!
//! A split votes its anchor with the cut's gauges in it: a cut gauge
//! votes its parent, a reference that stays inside the cut casts no
//! vote, and a group unplaced for lack of an offset votes its gauge. A
//! kept node hanging from a cut gauge refuses. Every cut moves
//! verbatim: each cut gauge is carried, one whose parent leaves the
//! cut onto the part's world, and each root keeps its offset. Inline
//! at a non-empty offset mints a gauge under the instance's gauge,
//! holding that offset, and moves the members the instance placed onto
//! it; a mate-placed instance inlines over a part that is one group at
//! the empty chain on its world, whose root takes its place. Every cut
//! split admits, inlined back at the empty offset, returns the document
//! it was given up to node ids (R1). `Promote` and `Fold` carry a
//! group's frame onto a gauge and back, so a part at a frame of its own
//! is a promote and a cut leaving the gauge behind.
//!
//! The scenes are `p2_gauges`'s two literal blocks; every placement is
//! a translation by dyadic lengths, so the evaluations compared are
//! exact.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::fixture;
use crate::p2_gauges::{
    Parts, TOP_HEIGHT, body_of, cut, literal, min_corner, parts, placed_pair, points, seat,
    seat_on, set_gauge, set_offset,
};

use editor_core::{
    CapEnd, DocEdit, DocumentId, EvalOptions, InlineError, InlineOutcome, Label, Node,
    PartResolver, Placement, ProfileDoc, RecipeNodeId, SplitError, SplitOutcome, StableName,
    product,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::round_trip::{composed, same_up_to_ids};
use fixture::{head, insert, offset_of, run, solve, step};
use geom_core::Tol;

// ---- substrate ----

pub(crate) fn split(
    doc: &ProfileDoc,
    ids: &[RecipeNodeId],
    label: &str,
    o: &EvalOptions,
) -> Result<SplitOutcome, SplitError> {
    editor_core::split(
        doc,
        &cut(ids),
        DocumentId::derive(&format!("{label}-part")),
        Tol::witness(),
        o.resolver.as_ref(),
    )
}

pub(crate) fn inline(doc: &ProfileDoc, instance: RecipeNodeId, store: &PartStore) -> InlineOutcome {
    let r: Arc<dyn PartResolver> = Arc::new(store.clone());
    editor_core::inline(doc, instance, &r, Tol::witness())
        .unwrap_or_else(|e| panic!("inline refused: {e}"))
}

fn inline_err(doc: &ProfileDoc, instance: RecipeNodeId, store: &PartStore) -> InlineError {
    let r: Arc<dyn PartResolver> = Arc::new(store.clone());
    match editor_core::inline(doc, instance, &r, Tol::witness()) {
        Ok(_) => panic!("inline admitted what it should refuse"),
        Err(e) => e,
    }
}

/// The whole document's product: its corners and volume.
pub(crate) fn extent(doc: &ProfileDoc, o: &EvalOptions) -> ([f64; 3], [f64; 3], f64) {
    let body = product(doc, &run(doc, o), Tol::witness()).expect("gathers");
    let pts = points(&body);
    let lo = pts.iter().fold([f64::INFINITY; 3], |m, p| {
        [m[0].min(p[0]), m[1].min(p[1]), m[2].min(p[2])]
    });
    let hi = pts.iter().fold([f64::NEG_INFINITY; 3], |m, p| {
        [m[0].max(p[0]), m[1].max(p[1]), m[2].max(p[2])]
    });
    let volume = topo::mass_properties(&body, Tol::witness())
        .expect("mass properties")
        .volume;
    (lo, hi, volume)
}

pub(crate) fn same_extent(a: ([f64; 3], [f64; 3], f64), b: ([f64; 3], [f64; 3], f64), what: &str) {
    let near = |x: [f64; 3], y: [f64; 3]| x.iter().zip(y).all(|(p, q)| (p - q).abs() <= 1e-9);
    assert!(
        near(a.0, b.0) && near(a.1, b.1) && (a.2 - b.2).abs() <= 1e-9,
        "{what}: {a:?} vs {b:?}"
    );
}

/// `name` wrapped at `instance`, as a host spells a face of its part.
fn wrap(instance: RecipeNodeId, name: StableName) -> StableName {
    editor_core::FaceName::new(name)
        .expect("a face")
        .in_part(instance)
        .into_name()
}

/// The base block's bottom cap at `base`.
fn base_bottom(p: &Parts, base: RecipeNodeId) -> StableName {
    in_part(base, p.base_body, CapEnd::Start)
}

/// **R1**: `doc` split at `ids` and the instance inlined back at the
/// empty offset it sits at is `doc` up to node ids.
pub(crate) fn round_trip(
    doc: &ProfileDoc,
    ids: &[RecipeNodeId],
    p: &Parts,
    label: &str,
) -> SplitOutcome {
    let out = split(doc, ids, label, &p.opts()).unwrap_or_else(|e| panic!("{label}: {e}"));
    assert_eq!(
        offset_of(&out.remainder, out.instance),
        Some(Placement::IDENTITY),
        "{label}: a cut holding a gauge moves verbatim at the empty offset"
    );
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let back = inline(&out.remainder, out.instance, &store);
    let (map, steps) = composed(doc, &out, &back);
    same_up_to_ids(doc, &back.doc, &map, &steps)
        .unwrap_or_else(|e| panic!("{label}: inline(split(d)) is d up to node ids:\n{e}"));
    out
}

fn gauge_parent(doc: &ProfileDoc, id: RecipeNodeId) -> Option<RecipeNodeId> {
    match doc.node(id) {
        Some(Node::Gauge { parent, .. }) => *parent,
        other => panic!("{id:?} is no gauge: {other:?}"),
    }
}

// ---- S2: a gauge moved verbatim ----

/// **A cut holding a gauge moves verbatim** (ruling 4): K on the kept
/// anchor g, an instance on K and one on g. K votes g, the instance on
/// K casts no vote, so the anchor is g; the instance left behind sits
/// on g at the empty offset, K is carried with its parent the part's
/// world, its instance on K's image, and the instance on g on the
/// part's world. The evaluation is unchanged, and the round trip is
/// the document up to node ids.
#[test]
fn s2_a_cut_holding_a_gauge_moves_verbatim_and_round_trips() {
    let p = parts("s2-verbatim");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("s2-verbatim"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 8.0, 0.0])));
    let (doc, k) = insert(doc, Node::gauge(Some(g), literal([2.0, 0.0, 0.5])));
    let (doc, on_k) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, on_k, Some(k));
    let doc = set_offset(doc, on_k, Some(literal([0.0, 0.0, 4.0])));
    let (doc, on_g) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, on_g, Some(g));
    let doc = set_offset(doc, on_g, Some(literal([16.0, 0.0, 0.0])));
    let (doc, _) = step(
        doc,
        DocEdit::SetLabel {
            node: k,
            label: Some(Label::new("bench").expect("a label")),
        },
    );
    let out = round_trip(&doc, &[k, on_k, on_g], &p, "s2-verbatim");
    assert_eq!(
        out.remainder.node(out.instance).and_then(Node::gauge_ref),
        Some(g),
        "the instance names the anchor"
    );
    let k_part = out.node_map[&k];
    assert_eq!(
        gauge_parent(&out.part, k_part),
        None,
        "K's parent leaves the cut"
    );
    assert_eq!(
        out.part.node(out.node_map[&on_k]).and_then(Node::gauge_ref),
        Some(k_part),
        "an instance on K sits on K's image"
    );
    assert_eq!(
        out.part.node(out.node_map[&on_g]).and_then(Node::gauge_ref),
        None,
        "an instance on the anchor sits on the part's world"
    );
    assert_eq!(
        out.part.label(k_part).map(ToString::to_string),
        Some("bench".to_owned()),
        "K's label follows it"
    );
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let split_o = with_resolver(store);
    let (before, after) = (extent(&doc, &o), extent(&out.remainder, &split_o));
    assert_eq!(
        before.2.to_bits(),
        after.2.to_bits(),
        "split-then-evaluate keeps the volume"
    );
    same_extent(before, after, "split-then-evaluate keeps the material");
    // Name resolution: a cut face resolves before, and through the
    // instance qualifier after.
    let resolves = |d: &ProfileDoc, o: &EvalOptions, name: &StableName| {
        let eval = run(d, o);
        matches!(
            editor_core::resolve(
                editor_core::RunCtx {
                    doc: d,
                    eval: &eval
                },
                name
            ),
            editor_core::Resolution::Resolved(_)
        )
    };
    for (face, in_part) in [
        (p.base_cap(on_k), p.base_cap(out.node_map[&on_k])),
        (p.top_cap(on_g), p.top_cap(out.node_map[&on_g])),
    ] {
        assert!(
            resolves(&doc, &o, &face),
            "{face:?} resolves before the split"
        );
        let through = wrap(out.instance, in_part);
        assert!(
            resolves(&out.remainder, &split_o, &through),
            "{through:?} resolves after the split"
        );
    }
}

/// **The carry puts a gauge ahead of what sits on it** (ruling 4): an
/// instance inserted before the gauge it was then moved onto is carried
/// after it, at a split and at an inline alike.
#[test]
fn s2_a_gauge_inserted_after_its_instance_is_carried_first() {
    let p = parts("s2-order");
    let build = |label: &str| {
        let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
        let (doc, early) = insert(doc, Node::instantiate_part(p.base));
        let (doc, k) = insert(doc, Node::gauge(None, literal([0.0, 0.0, 2.0])));
        (set_gauge(doc, early, Some(k)), early, k)
    };
    let (doc, early, k) = build("s2-order");
    round_trip(&doc, &[early, k], &p, "s2-order");

    // A part document holding the same order, inlined at the empty
    // offset.
    let mut store = p.store.clone();
    let part_ref = store.insert(build("s2-order-sub").0, Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("s2-order-host"), Tol::witness());
    let (host, h) = insert(host, Node::instantiate_part(part_ref));
    let back = inline(&host, h, &store);
    let carried = back.node_map[&early];
    assert_eq!(
        back.doc.node(carried).and_then(Node::gauge_ref),
        Some(back.node_map[&k])
    );
}

// ---- S3: the severed gauge ----

/// **A kept node hanging from a cut gauge refuses** (ruling 2): a kept
/// instance on K, and a kept gauge whose parent is K, each refuse
/// `SeveredGauge` naming the gauge and the kept node, with the recourse.
#[test]
fn s3_a_kept_instance_or_gauge_on_a_cut_gauge_refuses_severed_gauge() {
    let p = parts("s3-severed");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("s3-severed"), Tol::witness());
    let (doc, k) = insert(doc, Node::gauge(None, literal([0.0, 0.0, 2.0])));
    let (doc, cut_on_k) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, cut_on_k, Some(k));
    let (with_instance, kept) = insert(doc.clone(), Node::instantiate_part(p.base));
    let with_instance = set_gauge(with_instance, kept, Some(k));
    let (with_gauge, k2) = insert(doc, Node::gauge(Some(k), literal([1.0, 0.0, 0.0])));
    for (doc, kept, what) in [
        (&with_instance, kept, "a kept instance"),
        (&with_gauge, k2, "a kept gauge"),
    ] {
        let err =
            split(doc, &[k, cut_on_k], "s3-severed", &o).expect_err("a kept node on a cut gauge");
        assert!(
            matches!(&err, SplitError::SeveredGauge { gauge, kept: n }
                if gauge.id() == k && n.id() == kept),
            "{what}: {err:?}"
        );
        let said = err.to_string();
        assert!(
            said.contains(&format!(
                "Recourse: add {} to the cut, or set its gauge outside the cut (SetGauge)",
                doc.spoken(kept)
            )),
            "{what}: {said}"
        );
    }
}

// ---- S4: the vote ----

/// **The vote with gauges in the cut** (ruling 1, D2): K on g votes g,
/// so an instance on the world beside it refuses `TwoAnchors`; a bare
/// cut gauge whose parent was deleted refuses `DeadGaugeReference`
/// naming it; and a group unplaced for lack of an offset votes its
/// gauge, so beside a placed instance on the world it refuses
/// `TwoAnchors` rather than lose its gauge.
#[test]
fn s4_cut_gauges_vote_their_parents_and_an_unplaced_group_votes_its_gauge() {
    let p = parts("s4-vote");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("s4-vote"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 8.0, 0.0])));
    let (doc, k) = insert(doc, Node::gauge(Some(g), literal([2.0, 0.0, 0.0])));
    let (doc, on_world) = insert(doc, Node::instantiate_part(p.top));
    let err = split(&doc, &[k, on_world], "s4-vote", &o).expect_err("two anchors");
    assert!(
        matches!(&err, SplitError::TwoAnchors { node, first, second }
            if node.id() == on_world
                && first.as_ref().map(|f| f.id()) == Some(g)
                && second.is_none()),
        "{err:?}"
    );
    assert!(err.to_string().contains("Recourse:"), "{err}");

    let (dead, _) = step(doc.clone(), DocEdit::DeleteNode { id: g });
    let err = split(&dead, &[k], "s4-vote", &o).expect_err("a dead parent");
    assert!(
        matches!(&err, SplitError::DeadGaugeReference { node, gauge }
            if node.id() == k && gauge.id() == g),
        "{err:?}"
    );
    assert!(err.to_string().contains("Recourse:"), "{err}");

    // D2: the unplaced group's gauge is not lost.
    let (doc, unplaced) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, unplaced, Some(g));
    let doc = set_offset(doc, unplaced, None);
    let err = split(&doc, &[on_world, unplaced], "s4-vote", &o)
        .expect_err("the unplaced group votes its gauge");
    assert!(
        matches!(&err, SplitError::TwoAnchors { node, first, second }
            if node.id() == unplaced
                && first.is_none()
                && second.as_ref().map(|s| s.id()) == Some(g)),
        "{err:?}"
    );
    // Beside a placed instance on its own gauge, it anchors there.
    let (doc, placed) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, placed, Some(g));
    let out = split(&doc, &[unplaced, placed], "s4-vote", &o).expect("one anchor");
    assert_eq!(
        out.remainder.node(out.instance).and_then(Node::gauge_ref),
        Some(g)
    );
}

// ---- S5: the frame rule under the verbatim move ----

/// **A kept mate reading a root on a cut gauge refuses** (ruling 4's
/// `root_lands_empty`): the cut holds K and a root at the empty chain
/// on it, and a root at the empty chain on the world. A kept block on a
/// gauge of its own declares against each: the one reading the world
/// root crosses, since that root lands at the empty chain on the
/// part's world; the one reading the root on K refuses
/// `MateFrameCrosses`, since that root lands on K's image.
#[test]
fn s5_a_kept_mate_reading_a_root_on_a_cut_gauge_refuses_the_frame_rule() {
    let p = parts("s5-frame");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("s5-frame"), Tol::witness());
    let (doc, k) = insert(doc, Node::gauge(None, literal([8.0, 0.0, 0.0])));
    let (doc, on_k) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, on_k, Some(k));
    let (doc, on_world) = insert(doc, Node::instantiate_part(p.base));
    let (doc, h) = insert(doc, Node::gauge(None, literal([0.0, 0.0, 0.0])));
    let (doc, kept) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, kept, Some(h));
    let declaring = |doc: &ProfileDoc, base: RecipeNodeId| {
        insert(
            doc.clone(),
            seat(head(p.top_cap(kept)), head(p.base_cap(base))),
        )
    };
    let (crosses, _) = declaring(&doc, on_world);
    split(&crosses, &[k, on_k, on_world], "s5-frame", &o)
        .expect("a root at the empty chain on the part's world crosses");
    let (refuses, mate) = declaring(&doc, on_k);
    let err =
        split(&refuses, &[k, on_k, on_world], "s5-frame", &o).expect_err("a root on a cut gauge");
    assert!(
        matches!(&err, SplitError::MateFrameCrosses { mate: m, side }
            if m.id() == mate && *side == editor_core::MateSide::B),
        "{err:?}"
    );
    assert!(err.to_string().contains("Recourse:"), "{err}");
}

// ---- S6: would start placing ----

/// **A kept instance on the anchor declaring against a cut instance on
/// K refuses** (A4): the instance left behind sits on g beside it, so
/// the mate would start placing.
#[test]
fn s6_a_declaring_mate_from_the_anchor_into_a_cut_gauge_would_start_placing() {
    let p = parts("s6-placing");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("s6-placing"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 8.0, 0.0])));
    let (doc, k) = insert(doc, Node::gauge(Some(g), literal([4.0, 0.0, 0.0])));
    let (doc, on_k) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, on_k, Some(k));
    let (doc, kept) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, kept, Some(g));
    let (doc, mate) = insert(doc, seat(head(p.top_cap(kept)), head(p.base_cap(on_k))));
    let err = split(&doc, &[k, on_k], "s6-placing", &o).expect_err("would start placing");
    assert!(
        matches!(&err, SplitError::WouldStartPlacing { mate: m } if m.id() == mate),
        "{err:?}"
    );
    assert!(err.to_string().contains("Recourse:"), "{err}");
}

// ---- I1–I3: inline's gauge ----

/// A part of two lone bases, the second at `[16, 0, 0]`, its first base
/// the root of its group at the empty chain.
fn two_groups(p: &Parts, label: &str) -> (ProfileDoc, RecipeNodeId) {
    let sub = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (sub, first) = insert(sub, Node::instantiate_part(p.base));
    let (sub, second) = insert(sub, Node::instantiate_part(p.base));
    (
        set_offset(sub, second, Some(literal([16.0, 0.0, 0.0]))),
        first,
    )
}

/// A host instance of `sub` on a gauge g at `[0, 8, 0]`, at offset
/// `[2, 0, 4]` and labelled, and the store holding `sub`.
fn host_of(p: &Parts, sub: ProfileDoc, label: &str) -> (ProfileDoc, PartStore, [RecipeNodeId; 2]) {
    let mut store = p.store.clone();
    let sub_ref = store.insert(sub, Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 8.0, 0.0])));
    let (doc, h) = insert(doc, Node::instantiate_part(sub_ref));
    let doc = set_gauge(doc, h, Some(g));
    let doc = set_offset(doc, h, Some(literal([2.0, 0.0, 4.0])));
    let (doc, _) = step(
        doc,
        DocEdit::SetLabel {
            node: h,
            label: Some(Label::new("sub").expect("a label")),
        },
    );
    (doc, store, [g, h])
}

/// The one gauge `out` minted: a gauge of `out.doc` that neither `doc`
/// nor the part held.
fn minted_gauge(doc: &ProfileDoc, out: &InlineOutcome) -> RecipeNodeId {
    let fresh: Vec<RecipeNodeId> = out
        .doc
        .order()
        .iter()
        .copied()
        .filter(|&id| matches!(out.doc.node(id), Some(Node::Gauge { .. })))
        .filter(|id| doc.node(*id).is_none() && !out.node_map.values().any(|v| v == id))
        .collect();
    match fresh.as_slice() {
        [g] => *g,
        other => panic!("one minted gauge, not {other:?}"),
    }
}

/// **Inline mints a gauge** (ruling 5, D3): a root instance at offset o
/// on gauge g, over a part of two groups, over a part whose root sits
/// off the empty chain, and over a part holding a gauge, inlines onto a
/// gauge minted under g holding o, labelled as the instance was, from
/// which the part's gauges and instances hang; the material is where it
/// was.
#[test]
fn i1_inline_at_an_offset_over_any_other_part_mints_a_gauge() {
    let p = parts("i1-minted");
    // Each part, with where each of its instances' lowest corner lands
    // in the world: g at [0, 8, 0], o at [2, 0, 4], then the part's own.
    let two = {
        let sub = ProfileDoc::empty(DocumentId::derive("i1-two"), Tol::witness());
        let (sub, first) = insert(sub, Node::instantiate_part(p.base));
        let (sub, second) = insert(sub, Node::instantiate_part(p.base));
        let sub = set_offset(sub, second, Some(literal([16.0, 0.0, 0.0])));
        (
            sub,
            vec![(first, [2.0, 8.0, 4.0]), (second, [18.0, 8.0, 4.0])],
        )
    };
    let shifted = {
        let sub = ProfileDoc::empty(DocumentId::derive("i1-shifted"), Tol::witness());
        let (sub, root) = insert(sub, Node::instantiate_part(p.base));
        let sub = set_offset(sub, root, Some(literal([1.0, 0.0, 0.0])));
        (sub, vec![(root, [3.0, 8.0, 4.0])])
    };
    let gauged = {
        let sub = ProfileDoc::empty(DocumentId::derive("i1-gauged"), Tol::witness());
        let (sub, k) = insert(sub, Node::gauge(None, literal([0.0, 0.0, 2.0])));
        let (sub, on_k) = insert(sub, Node::instantiate_part(p.base));
        (set_gauge(sub, on_k, Some(k)), vec![(on_k, [2.0, 8.0, 6.0])])
    };
    for ((sub, corners), what) in [
        (two, "two groups"),
        (shifted, "a root off the empty chain"),
        (gauged, "a part holding a gauge"),
    ] {
        let (doc, store, [g, h]) = host_of(&p, sub.clone(), &format!("i1-host-{what}"));
        let out = inline(&doc, h, &store);
        let minted = minted_gauge(&doc, &out);
        assert_eq!(
            out.doc.node(minted),
            Some(&Node::gauge(Some(g), literal([2.0, 0.0, 4.0]))),
            "{what}: the minted gauge sits on g holding the offset"
        );
        assert_eq!(
            out.doc.label(minted).map(ToString::to_string),
            Some("sub".to_owned()),
            "{what}: the minted gauge takes the instance's label"
        );
        for (&old, &new) in &out.node_map {
            if sub.node(old).and_then(Node::gauge_ref).is_none()
                && matches!(
                    sub.node(old),
                    Some(Node::InstantiatePart { .. } | Node::Gauge { .. })
                )
            {
                assert_eq!(
                    out.doc.node(new).and_then(Node::gauge_ref),
                    Some(minted),
                    "{what}: {old:?} on the part's world hangs from the minted gauge"
                );
            }
        }
        // The roots: the minted gauge at the instance's position, then
        // the part's roots in its own order.
        let at = doc
            .roots()
            .iter()
            .position(|&r| r == h)
            .expect("h is a root");
        let spliced: Vec<RecipeNodeId> = sub.roots().iter().map(|r| out.node_map[r]).collect();
        assert_eq!(
            out.doc.roots()[at..at + 1 + spliced.len()],
            [vec![minted], spliced].concat(),
            "{what}: the minted gauge takes the instance's place in the root list"
        );
        let o = with_resolver(store);
        let ev = run(&out.doc, &o);
        for (old, corner) in corners {
            crate::p2_gauges::close(
                min_corner(&body_of(&ev, out.node_map[&old])),
                corner,
                &format!("{what}: {old:?}'s world pose"),
            );
        }
        same_extent(
            extent(&doc, &o),
            extent(&out.doc, &o),
            &format!("{what}: inline moves no material"),
        );
    }
}

/// A host whose instance of a two-group part places a top block on the
/// part's first base through a mate, and the mate.
fn member_through_a_mate(p: &Parts, label: &str) -> (ProfileDoc, PartStore, [RecipeNodeId; 4]) {
    let (sub, first) = two_groups(p, &format!("{label}-sub"));
    let (doc, store, [g, h]) = host_of(p, sub, label);
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, top, Some(g));
    let (doc, mate) = insert(
        doc,
        seat(head(p.top_cap(top)), head(wrap(h, p.base_cap(first)))),
    );
    (doc, store, [h, top, mate, first])
}

/// **The members move** (ruling 5): a top the instance places through a
/// mate moves onto the minted gauge by a recorded `SetGauge`, its mate
/// reads the inner root, and it is where it was.
#[test]
fn i2_a_member_the_instance_placed_moves_onto_the_minted_gauge() {
    let p = parts("i2-moved");
    let (doc, store, [h, top, mate, first]) = member_through_a_mate(&p, "i2-moved");
    let o = with_resolver(store.clone());
    let before = min_corner(&body_of(&run(&doc, &o), top));
    let out = inline(&doc, h, &store);
    let minted = minted_gauge(&doc, &out);
    assert_eq!(
        out.doc.node(top).and_then(Node::gauge_ref),
        Some(minted),
        "the top sits on the minted gauge"
    );
    assert!(
        out.edits.contains(&DocEdit::SetGauge {
            node: top,
            gauge: Some(minted),
        }),
        "the move is a recorded edit: {:?}",
        out.edits
    );
    match out.doc.node(mate) {
        Some(Node::Mate { b, .. }) => assert_eq!(
            &*b.name,
            &p.base_cap(out.node_map[&first]),
            "the mate reads the inner root"
        ),
        other => panic!("the mate survives: {other:?}"),
    }
    let after = min_corner(&body_of(&run(&out.doc, &o), top));
    crate::p2_gauges::close(before, after, "the top is where it was");
    same_extent(extent(&doc, &o), extent(&out.doc, &o), "no material moves");
    // The edit list is the record: it replays with no reach and no
    // solve to the same document.
    let replayed = out.edits.iter().fold(doc, |d, e| {
        editor_core::apply_replayed(&d, e, Tol::witness())
            .unwrap_or_else(|err| panic!("{e:?} replays: {err}"))
            .doc
    });
    assert!(replayed.bit_eq(&out.doc), "the list replays to the result");
}

/// **A moved member with a further offset refuses** (ruling 5): the top
/// carries a checked offset, stated in g, which the minted gauge's
/// frame would not keep.
#[test]
fn i3_a_moved_member_with_a_further_offset_refuses() {
    let p = parts("i3-offset");
    let (doc, store, [h, top, _, _]) = member_through_a_mate(&p, "i3-offset");
    let o = with_resolver(store.clone());
    let solved = solve(&doc, &o, Tol::witness())
        .placement(&doc, top)
        .expect("placed");
    // A true checked offset, and the empty chain stated in g, which is
    // false in the minted gauge's frame: both refuse.
    for offset in [Placement::literal(&solved), Placement::IDENTITY] {
        let doc = set_offset(doc.clone(), top, Some(offset.clone()));
        let err = inline_err(&doc, h, &store);
        assert!(
            matches!(&err, InlineError::MovedMemberOffset { member } if member.id() == top),
            "{offset:?}: {err:?}"
        );
        let said = err.to_string();
        assert!(
            said.contains(&format!(
                "Recourse: clear {}'s offset (SetOffset), then inline",
                doc.spoken(top)
            )),
            "{said}"
        );
    }
}

// ---- I4–I5: the mate-placed inline ----

/// A part at a frame of its own — the placed pair's base promoted, and
/// the group cut leaving the promoted gauge: a base at the empty chain,
/// a top mated onto it — in a store, its base and top, and a host in
/// which a tall block
/// `ht`, placed first, places an instance of that part through a mate
/// reading the part's base: the instance is not its group's root.
fn mate_placed(label: &str) -> (Parts, PartStore, ProfileDoc, [RecipeNodeId; 5]) {
    let (p, doc, [base, top, mate]) = placed_pair(label);
    let (doc, _) = step(doc, DocEdit::Promote { instance: base });
    let out = split(&doc, &[base, top, mate], label, &p.opts()).expect("the promoted group");
    let (part_base, part_top) = (out.node_map[&base], out.node_map[&top]);
    let mut store = p.store.clone();
    let part_ref = store.insert(out.part, Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive(&format!("{label}-host")), Tol::witness());
    let (host, ht) = insert(host, Node::instantiate_part(p.top));
    let (host, i) = insert(host, Node::instantiate_part(part_ref));
    let (host, m) = insert(
        host,
        seat_on(
            head(wrap(i, base_bottom(&p, part_base))),
            head(p.top_upper_cap(ht)),
            [0.0, 0.0, TOP_HEIGHT],
        ),
    );
    (p, store, host, [ht, i, m, part_base, part_top])
}

/// **The mate-placed inline** (ruling 6): an instance its mate places
/// on a host block, over a part that is one group at the empty chain,
/// inlines. The part's root takes its place — no offset, on its gauge —
/// the host mate reads that root, and the material is where it was.
/// With a checked offset on the instance, the root carries it. A host
/// mate reading the part through a member that is not its root still
/// refuses the frame rule.
#[test]
fn i4_a_mate_placed_instance_over_one_such_group_inlines() {
    let (p, store, host, [ht, i, m, part_base, part_top]) = mate_placed("i4-mate-placed");
    let o = with_resolver(store.clone());
    let out = inline(&host, i, &store);
    let root = out.node_map[&part_base];
    assert_eq!(offset_of(&out.doc, root), None, "the root takes no offset");
    assert_eq!(out.doc.node(root).and_then(Node::gauge_ref), None);
    match out.doc.node(m) {
        Some(Node::Mate { a, .. }) => {
            assert_eq!(
                &*a.name,
                &base_bottom(&p, root),
                "the host mate reads the root"
            );
        }
        other => panic!("the mate survives: {other:?}"),
    }
    same_extent(extent(&host, &o), extent(&out.doc, &o), "no material moves");

    let solved = solve(&host, &o, Tol::witness())
        .placement(&host, i)
        .expect("placed");
    let checked = Placement::literal(&solved);
    let stated = set_offset(host.clone(), i, Some(checked.clone()));
    assert_eq!(
        solve(&stated, &o, Tol::witness()).fault(i),
        None,
        "the checked offset is true"
    );
    let out = inline(&stated, i, &store);
    assert_eq!(
        offset_of(&out.doc, out.node_map[&part_base]),
        Some(checked),
        "the root carries the checked offset"
    );

    // Through the top, which is not the part's root.
    let (through_top, by_top) = insert(
        step(host, DocEdit::DeleteNode { id: m }).0,
        seat_on(
            head(wrap(i, p.top_cap(part_top))),
            head(p.top_upper_cap(ht)),
            [0.0, 0.0, TOP_HEIGHT],
        ),
    );
    let err = inline_err(&through_top, i, &store);
    assert!(
        matches!(&err, InlineError::MateFrameCrosses { mate, .. } if mate.id() == by_top),
        "{err:?}"
    );
}

/// **The mate-placed refusal stays narrow** (ruling 6): over a part
/// whose root sits off the empty chain the instance refuses
/// `MatePlaced`, naming the part's root and the remedy of setting its
/// offset; over a part of two groups it refuses naming the mates.
#[test]
fn i5_a_mate_placed_instance_over_any_other_part_refuses() {
    let (p, store, host, [ht, i, m, part_base, _]) = mate_placed("i5-mate-placed");
    let part_id = match host.node(i) {
        Some(Node::InstantiatePart { doc_ref, .. }) => doc_ref.id,
        other => panic!("an instance: {other:?}"),
    };
    let (shifted, _) = step(
        store.doc(part_id),
        DocEdit::SetOffset {
            instance: part_base,
            offset: Some(literal([1.0, 0.0, 0.0])),
        },
    );
    let mut shifted_store = store.clone();
    let shifted_ref = shifted_store.insert(shifted.clone(), Tol::witness());
    let (repinned, _) = step(
        host.clone(),
        DocEdit::UpdateReference {
            node: i,
            new_pin: shifted_ref.pin,
        },
    );
    let err = inline_err(&repinned, i, &shifted_store);
    assert!(
        matches!(&err, InlineError::MatePlaced { instance, part_root: Some(r), .. }
            if instance.id() == i && r.id() == part_base),
        "{err:?}"
    );
    let said = err.to_string();
    assert!(
        said.contains(&format!(
            "Recourse: in the referenced document, set {}'s offset to the empty chain \
             (SetOffset), point {} at that version (UpdateReference), then inline",
            shifted.spoken(part_base),
            host.spoken(i)
        )),
        "{said}"
    );

    let (two, first) = two_groups(&p, "i5-mate-placed-part");
    let mut two_store = p.store.clone();
    let two_ref = two_store.insert(two, Tol::witness());
    let (other, _) = step(
        step(host.clone(), DocEdit::DeleteNode { id: m }).0,
        DocEdit::UpdateReference {
            node: i,
            new_pin: two_ref.pin,
        },
    );
    let (other, mate) = insert(
        other,
        seat_on(
            head(wrap(i, base_bottom(&p, first))),
            head(p.top_upper_cap(ht)),
            [0.0, 0.0, TOP_HEIGHT],
        ),
    );
    let err = inline_err(&other, i, &two_store);
    assert!(
        matches!(&err, InlineError::MatePlaced { instance, mates, part_root: None, .. }
            if instance.id() == i && mates.iter().map(|m| m.id()).collect::<Vec<_>>() == [mate]),
        "{err:?}"
    );
    assert!(err.to_string().contains("Recourse: delete"), "{err}");

    // A part that is one group whose root sits at the empty chain on a
    // gauge K: only the gauge keeps it from being one such group, and
    // the refusal names K in the remedy.
    let gauged = {
        let sub = ProfileDoc::empty(DocumentId::derive("i5-mate-placed-part"), Tol::witness());
        let (sub, k) = insert(sub, Node::gauge(None, literal([0.0, 0.0, 2.0])));
        let (sub, root) = insert(sub, Node::instantiate_part(p.base));
        (set_gauge(sub, root, Some(k)), k, root)
    };
    let (gauged, k, gauged_root) = gauged;
    let mut gauged_store = p.store.clone();
    let gauged_ref = gauged_store.insert(gauged.clone(), Tol::witness());
    let (on_gauge, _) = step(
        step(host.clone(), DocEdit::DeleteNode { id: m }).0,
        DocEdit::UpdateReference {
            node: i,
            new_pin: gauged_ref.pin,
        },
    );
    let (on_gauge, _) = insert(
        on_gauge,
        seat_on(
            head(wrap(i, base_bottom(&p, gauged_root))),
            head(p.top_upper_cap(ht)),
            [0.0, 0.0, TOP_HEIGHT],
        ),
    );
    let err = inline_err(&on_gauge, i, &gauged_store);
    assert!(
        matches!(&err, InlineError::MatePlaced { part_root: Some(r), part_gauges, .. }
            if r.id() == gauged_root
                && part_gauges.iter().map(|g| g.id()).collect::<Vec<_>>() == [k]),
        "{err:?}"
    );
    let said = err.to_string();
    assert!(
        said.contains(&format!(
            "Recourse: in the referenced document, put {r}'s group on the world at the empty \
             chain (SetGauge, SetOffset), delete {k} (DeleteNode)",
            r = gauged.spoken(gauged_root),
            k = gauged.spoken(k)
        )),
        "{said}"
    );
}

// ---- R1: the round trip ----

/// **A cut holding a gauge, moved verbatim and inlined at the empty
/// offset, is the document it was given up to node ids** (R1): nested
/// gauges, instances on each, a mate placing a top on one of them, and
/// a parametric gauge whose parameter moves with it. The comparator is
/// not vacuous: one offset changed fails it.
#[test]
fn r1_a_cut_holding_nested_gauges_round_trips_exactly() {
    let p = parts("r1-nested");
    let doc = ProfileDoc::empty(DocumentId::derive("r1-nested"), Tol::witness());
    let doc = crate::p2_gauges::declare_lift(doc, 0.5);
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 8.0, 0.0])));
    let (doc, k) = insert(doc, crate::p2_gauges::lifting_gauge(Some(g), 0.0));
    let (doc, k2) = insert(doc, Node::gauge(Some(k), literal([4.0, 0.0, 0.0])));
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, base, Some(k));
    let doc = set_offset(doc, base, Some(literal([0.0, 2.0, 0.0])));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, top, Some(k));
    let (doc, mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let (doc, deep) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, deep, Some(k2));
    let (doc, kept) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, kept, Some(g));
    let out = round_trip(&doc, &[k, k2, base, top, mate, deep], &p, "r1-nested");
    assert_eq!(
        out.remainder.node(out.instance).and_then(Node::gauge_ref),
        Some(g)
    );
    assert!(
        out.part.params().contains_key(&crate::p2_gauges::lift()),
        "K's parameter moves with it"
    );

    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let back = inline(&out.remainder, out.instance, &store);
    let (map, steps) = composed(&doc, &out, &back);
    let moved = set_offset(back.doc, map[&base], Some(literal([0.0, 2.5, 0.0])));
    assert!(
        same_up_to_ids(&doc, &moved, &map, &steps).is_err(),
        "the comparator reads offsets"
    );
}

/// **R1 with a checked offset under a carried placing mate**: a cut
/// holding a gauge K and a placed pair on the world, the top mated
/// onto the base and carrying its solved pose as a checked offset. The
/// carried mate's insert clears the top's offset and the carry
/// re-states it, at the split and at the inline, so the round trip is
/// the document up to node ids and no `OffsetCleared` survives.
#[test]
fn r1_a_checked_offset_under_a_carried_placing_mate_round_trips_exactly() {
    let (p, doc, [base, top, mate]) = placed_pair("r1-checked");
    let o = p.opts();
    let solved = solve(&doc, &o, Tol::witness())
        .placement(&doc, top)
        .expect("placed");
    let doc = set_offset(doc, top, Some(Placement::literal(&solved)));
    assert_eq!(
        solve(&doc, &o, Tol::witness()).fault(top),
        None,
        "the checked offset is true"
    );
    let (doc, k) = insert(doc, Node::gauge(None, literal([0.0, 8.0, 0.0])));
    let (doc, on_k) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, on_k, Some(k));
    let out = round_trip(&doc, &[base, top, mate, k, on_k], &p, "r1-checked");
    assert_eq!(
        offset_of(&out.part, out.node_map[&top]),
        Some(Placement::literal(&solved)),
        "the part keeps the checked offset"
    );
    assert!(
        !out.part_maintenance
            .iter()
            .any(|m| matches!(m, editor_core::Maintenance::OffsetCleared { .. })),
        "no cleared offset survives: {:?}",
        out.part_maintenance
    );
}

// ---- No material ----

/// **A cut that holds no material refuses** (A4: split-then-evaluate
/// equals the unsplit evaluation): a bare gauge beside kept material, a
/// gauge chain, and a spare frame datum each refuse `NoMaterial` naming
/// the cut's first node, with a recourse, rather than leave an instance
/// of a part with no body.
#[test]
fn a_cut_of_gauges_or_a_datum_alone_refuses_no_material() {
    let p = parts("no-material");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("no-material"), Tol::witness());
    let (doc, k) = insert(doc, Node::gauge(None, literal([1.0, 0.0, 0.0])));
    let (doc, _kept) = insert(doc, Node::instantiate_part(p.base));
    let (chain, k2) = insert(doc.clone(), Node::gauge(Some(k), literal([0.0, 1.0, 0.0])));
    let (block, _) = crate::p2_gauges::block("no-material-datum", 2.0, 1.0);
    let frame = *block
        .order()
        .iter()
        .find(|&&id| matches!(block.node(id), Some(Node::Datum(_))))
        .expect("the block's frame");
    let (with_datum, spare) = insert(block.clone(), block.node(frame).cloned().expect("live"));
    for (doc, ids, first, what) in [
        (&doc, vec![k], k, "a bare gauge"),
        (&chain, vec![k, k2], k, "a gauge chain"),
        (&with_datum, vec![spare], spare, "a spare frame datum"),
    ] {
        let err = split(doc, &ids, "no-material", &o).expect_err(what);
        assert!(
            matches!(&err, SplitError::NoMaterial { node } if node.id() == first),
            "{what}: {err:?}"
        );
        assert!(err.to_string().contains("Recourse:"), "{what}: {err}");
    }
}

/// **`gauges_first` puts a gauge's own parent ahead of it**: an
/// instance, then K2, then K are inserted, the instance put on K2 and
/// K2 on K. Carried in document order the instance would precede both
/// gauges and K2 its parent; the round trip is the document.
#[test]
fn s2_a_gauge_chain_inserted_backwards_is_carried_parent_first() {
    let p = parts("s2-chain-order");
    let doc = ProfileDoc::empty(DocumentId::derive("s2-chain-order"), Tol::witness());
    let (doc, instance) = insert(doc, Node::instantiate_part(p.base));
    let (doc, k2) = insert(doc, Node::gauge(None, literal([1.0, 0.0, 0.0])));
    let (doc, k) = insert(doc, Node::gauge(None, literal([0.0, 0.0, 2.0])));
    let doc = set_gauge(doc, instance, Some(k2));
    let doc = set_gauge(doc, k2, Some(k));
    round_trip(&doc, &[instance, k2, k], &p, "s2-chain-order");
}

// ---- The comparator's own rows ----

/// The comparator's scene: a block's frame, profile and extrude, a
/// parametric gauge K with a label, a gauge K2, a placed base on K, a
/// top mated onto it, and a lone instance on K2. `tweak` changes one
/// thing; every variant mints the same ids in the same order, except
/// `Order`, which inserts the top before the base.
#[derive(Clone, Copy, PartialEq)]
enum Tweak {
    None,
    GaugeRef,
    Offset,
    Placement,
    Alignment,
    Head,
    Param,
    Label,
    Order,
}

fn comparator_scene(p: &Parts, tweak: Tweak) -> (ProfileDoc, [RecipeNodeId; 5]) {
    let (doc, _) = crate::p2_gauges::block("comparator-scene", 2.0, 1.0);
    let doc = crate::p2_gauges::declare_lift(doc, if tweak == Tweak::Param { 0.75 } else { 0.5 });
    let angle = if tweak == Tweak::Placement { 0.25 } else { 0.0 };
    let (doc, k) = insert(doc, crate::p2_gauges::lifting_gauge(None, angle));
    let label = if tweak == Tweak::Label {
        "other"
    } else {
        "bench"
    };
    let (doc, _) = step(
        doc,
        DocEdit::SetLabel {
            node: k,
            label: Some(Label::new(label).expect("a label")),
        },
    );
    let (doc, k2) = insert(doc, Node::gauge(None, literal([0.0, 8.0, 0.0])));
    let (doc, base, top) = if tweak == Tweak::Order {
        let (doc, top) = insert(doc, Node::instantiate_part(p.top));
        let (doc, base) = insert(doc, Node::instantiate_part(p.base));
        (doc, base, top)
    } else {
        let (doc, base) = insert(doc, Node::instantiate_part(p.base));
        let (doc, top) = insert(doc, Node::instantiate_part(p.top));
        (doc, base, top)
    };
    let doc = set_gauge(doc, base, Some(k));
    let doc = set_gauge(doc, top, Some(k));
    let x = if tweak == Tweak::Offset { 3.0 } else { 2.0 };
    let doc = set_offset(doc, base, Some(literal([x, 0.0, 0.0])));
    let onto = if tweak == Tweak::Head {
        base_bottom(p, base)
    } else {
        p.base_cap(base)
    };
    let at = if tweak == Tweak::Alignment {
        [1.0, 1.5, crate::p2_gauges::BASE_HEIGHT]
    } else {
        [1.0, 1.0, crate::p2_gauges::BASE_HEIGHT]
    };
    let (doc, mate) = insert(doc, seat_on(head(p.top_cap(top)), head(onto), at));
    let (doc, lone) = insert(doc, Node::instantiate_part(p.base));
    let gauge = if tweak == Tweak::GaugeRef { k } else { k2 };
    let doc = set_gauge(doc, lone, Some(gauge));
    (doc, [k, base, top, mate, lone])
}

/// **The comparator reads every field it claims** — and it reads them
/// itself, not through the remapping under test. A scene holding a
/// profile is itself under the identity maps; each one-thing change —
/// a gauge reference, an offset, a gauge placement, a mate alignment, a
/// head, a parameter, a label, a group's order — fails the check named,
/// as do a wrong profile step, a collapsed map and an extra root.
#[test]
fn r1_the_comparator_reads_every_field() {
    let p = parts("comparator");
    let (a, roles) = comparator_scene(&p, Tweak::None);
    let (nodes, steps) = fixture::round_trip::identity(&a);
    same_up_to_ids(&a, &a, &nodes, &steps).expect("a document is itself");
    // A variant's ids, read by position: the scenes insert alike.
    let by_position = |b: &ProfileDoc| {
        let live = |d: &ProfileDoc| -> Vec<RecipeNodeId> {
            d.order()
                .iter()
                .copied()
                .filter(|&id| d.node(id).is_some())
                .collect()
        };
        let mut map = editor_core::NodeMap::new();
        let mut step_map = editor_core::StepMap::new();
        for (x, y) in live(&a).into_iter().zip(live(b)) {
            map.insert(x, y);
            if let (Some(Node::Profile(px)), Some(Node::Profile(py))) = (a.node(x), b.node(y)) {
                step_map.extend(
                    px.ids
                        .iter()
                        .flatten()
                        .copied()
                        .zip(py.ids.iter().flatten().copied()),
                );
            }
        }
        (map, step_map)
    };
    let fails = |b: &ProfileDoc, nodes: &editor_core::NodeMap, steps: &editor_core::StepMap| {
        same_up_to_ids(&a, b, nodes, steps).expect_err("the comparator reads the change")
    };
    for (tweak, check, role) in [
        (Tweak::GaugeRef, "payload", 4),
        (Tweak::Offset, "payload", 1),
        (Tweak::Placement, "payload", 0),
        (Tweak::Alignment, "payload", 3),
        (Tweak::Head, "payload", 3),
        (Tweak::Param, "parameters", 0),
        (Tweak::Label, "label", 0),
    ] {
        let (b, _) = comparator_scene(&p, tweak);
        let (map, step_map) = by_position(&b);
        let node = roles[role];
        let said = fails(&b, &map, &step_map);
        assert!(
            said.lines().any(|l| l.starts_with(check)
                && (check == "parameters" || l.contains(&format!("{node:?}")))),
            "{check}: {said}"
        );
    }
    // A group's order: the top inserted before the base, each mapped
    // to its own role.
    let (b, b_roles) = comparator_scene(&p, Tweak::Order);
    let (mut by_role, step_map) = by_position(&b);
    for (x, y) in roles.iter().zip(b_roles) {
        by_role.insert(*x, y);
    }
    let said = fails(&b, &by_role, &step_map);
    assert!(
        said.lines().any(|l| l.starts_with("order")),
        "order: {said}"
    );
    // A profile's step read as another step.
    let mut wrong = steps.clone();
    let (&s, _) = wrong.iter().next().expect("the scene draws a profile");
    let other = *wrong.keys().nth(1).expect("two steps");
    wrong.insert(s, other);
    let said = fails(&a, &nodes, &wrong);
    assert!(
        said.lines().any(|l| l.starts_with("payload")),
        "profile step: {said}"
    );
    // Two nodes collapsed onto one.
    let mut collapsed = nodes.clone();
    collapsed.insert(roles[4], roles[1]);
    let said = fails(&a, &collapsed, &steps);
    assert!(
        said.lines().any(|l| l.starts_with("injective")),
        "collapse: {said}"
    );
    // An extra root: a gauge only the second holds.
    let (extra, _) = insert(a.clone(), Node::gauge(None, literal([5.0, 0.0, 0.0])));
    let said = fails(&extra, &nodes, &steps);
    assert!(
        said.lines().any(|l| l.starts_with("roots")),
        "roots: {said}"
    );
}

/// **The comparator over a round trip that keeps a profile** (a block
/// beside a cut gauge and its instance): equal, profile and all.
#[test]
fn r1_a_round_trip_keeping_a_profile_compares_equal() {
    let p = parts("r1-profile");
    let (doc, _) = crate::p2_gauges::block("r1-profile", 2.0, 1.0);
    let (doc, k) = insert(doc, Node::gauge(None, literal([0.0, 8.0, 0.0])));
    let (doc, on_k) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, on_k, Some(k));
    round_trip(&doc, &[k, on_k], &p, "r1-profile");
}

/// **R1 over every shape split admits**: a cut moves as selected, so
/// inlining it back at the empty offset returns the document up to node
/// ids whatever the cut holds — one placed group, one with a checked
/// member, a lone instance, a group rooted at a parametric offset, a
/// gauge at the empty chain holding a group, a gauge holding one group
/// at the empty chain, a gauge on a kept gauge holding two groups and a
/// gauge, a group on a kept gauge, and plain geometry on the world.
#[test]
fn r1_every_shape_split_admits_round_trips_exactly() {
    type Scene = (ProfileDoc, Vec<RecipeNodeId>);
    let p = parts("r1-every");
    let o = p.opts();
    let empty = |label: &str| ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let pair_on = |doc: ProfileDoc, gauge: Option<RecipeNodeId>, at: Placement| {
        let (doc, base) = insert(doc, Node::instantiate_part(p.base));
        let doc = set_gauge(doc, base, gauge);
        let doc = set_offset(doc, base, Some(at));
        let (doc, top) = insert(doc, Node::instantiate_part(p.top));
        let doc = set_gauge(doc, top, gauge);
        let (doc, mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
        (doc, [base, top, mate])
    };
    let one_group = || -> Scene {
        let (doc, ids) = pair_on(empty("r1-one-group"), None, literal([4.0, 0.0, 0.0]));
        (doc, ids.to_vec())
    };
    let checked_member = || -> Scene {
        let (doc, [base, top, mate]) =
            pair_on(empty("r1-checked-member"), None, literal([4.0, 0.0, 0.0]));
        let solved = solve(&doc, &o, Tol::witness())
            .placement(&doc, top)
            .expect("placed");
        let doc = set_offset(doc, top, Some(Placement::literal(&solved)));
        (doc, vec![base, top, mate])
    };
    let lone = || -> Scene {
        let (doc, kept) = insert(empty("r1-lone"), Node::instantiate_part(p.top));
        let doc = set_offset(doc, kept, Some(literal([0.0, 9.0, 0.0])));
        let (doc, lone) = insert(doc, Node::instantiate_part(p.base));
        let doc = set_offset(doc, lone, Some(literal([4.0, 0.0, 0.0])));
        (doc, vec![lone])
    };
    let parametric_root = || -> Scene {
        let doc = crate::p2_gauges::declare_lift(empty("r1-parametric-root"), 0.5);
        let at = Placement::from(editor_core::Step::Rigid {
            translation: [
                editor_core::Expr::param(crate::p2_gauges::lift(), editor_core::Dimension::Length),
                fixture::len(0.0),
                fixture::len(0.0),
            ],
            axis: [0.0, 0.0, 1.0].map(fixture::scl),
            angle: fixture::ang(0.0),
        });
        let (doc, ids) = pair_on(doc, None, at);
        (doc, ids.to_vec())
    };
    let empty_gauge = || -> Scene {
        let (doc, k) = insert(
            empty("r1-empty-gauge"),
            Node::gauge(None, Placement::IDENTITY),
        );
        let (doc, ids) = pair_on(doc, Some(k), literal([4.0, 0.0, 0.0]));
        (doc, [&[k][..], &ids].concat())
    };
    let group_at_empty = || -> Scene {
        let (doc, k) = insert(
            empty("r1-group-at-empty"),
            Node::gauge(None, literal([2.0, 0.0, 0.5])),
        );
        let (doc, ids) = pair_on(doc, Some(k), Placement::IDENTITY);
        (doc, [&[k][..], &ids].concat())
    };
    let gauge_on_kept = || -> Scene {
        let (doc, g) = insert(
            empty("r1-gauge-on-kept"),
            Node::gauge(None, literal([0.0, 8.0, 0.0])),
        );
        let (doc, k) = insert(doc, Node::gauge(Some(g), literal([2.0, 0.0, 0.5])));
        let (doc, pair) = pair_on(doc, Some(k), literal([0.0, 2.0, 0.0]));
        let (doc, lone) = insert(doc, Node::instantiate_part(p.base));
        let doc = set_gauge(doc, lone, Some(k));
        let doc = set_offset(doc, lone, Some(literal([16.0, 0.0, 0.0])));
        let (doc, k2) = insert(doc, Node::gauge(Some(k), literal([4.0, 0.0, 0.0])));
        let (doc, deep) = insert(doc, Node::instantiate_part(p.top));
        let doc = set_gauge(doc, deep, Some(k2));
        (doc, [&[k, lone, k2, deep][..], &pair].concat())
    };
    let group_on_kept = || -> Scene {
        let (doc, g) = insert(
            empty("r1-group-on-kept"),
            Node::gauge(None, literal([0.0, 8.0, 0.0])),
        );
        let (doc, ids) = pair_on(doc, Some(g), literal([4.0, 0.0, 0.0]));
        (doc, ids.to_vec())
    };
    let plain = || -> Scene {
        let (doc, _) = crate::p2_gauges::block("r1-plain", 2.0, 1.0);
        let all = doc.order().to_vec();
        (doc, all)
    };
    let scenes: [(&str, &dyn Fn() -> Scene); 9] = [
        ("one placed group", &one_group),
        ("one placed group with a checked member", &checked_member),
        ("a lone instance", &lone),
        ("a group at a parametric offset", &parametric_root),
        ("a gauge at the empty chain holding a group", &empty_gauge),
        (
            "a gauge holding one group at the empty chain",
            &group_at_empty,
        ),
        (
            "a gauge on a kept gauge with two groups and a gauge",
            &gauge_on_kept,
        ),
        ("a group on a kept gauge", &group_on_kept),
        ("plain geometry on the world", &plain),
    ];
    for (shape, scene) in scenes {
        let (doc, ids) = scene();
        round_trip(&doc, &ids, &p, shape);
    }
}
