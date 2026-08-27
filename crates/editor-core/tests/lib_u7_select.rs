//! **LIB-U7 — structural selectors and the materializer siblings.**
//!
//! Two things are pinned here.
//!
//! 1. **The siblings.** `all_faces`/`all_vertices`/`all_bodies` join
//!    `all_edges` with the same contract (filter by kind, canonical
//!    order, empty for a valueless node), and all four are exactly the
//!    kind-only structural selector — one implementation's worth of
//!    behaviour, said two ways, so neither can drift.
//!
//! 2. **The acceptance (LIB-U7 §2.4).** `die_composed`'s fourteen
//!    fillet names used to be authored by hand — P10's topology query
//!    relocated into the document layer rather than removed. They are
//!    now MATERIALIZED from a selector over role-path shape, and the
//!    result is IDENTICAL to the authored list, which stays in
//!    `corpus/die_composed.rs` as the independent oracle.
//!
//! What is deliberately absent: any geometric predicate. The selector
//! vocabulary cannot say "carrier kind", "adjacent surface pair",
//! "convex" or "z ≈ 1" — that is the LB7 fence, and the deferral is
//! stated in `crates/editor-core/src/names/select.rs`'s module docs.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use corpus::die_composed;
use editor_core::{
    CancelToken, CapEnd, Dimension, EntityKind, EvalOptions, Expr, NamePat, Node, OpGroup,
    ProfileDoc, RecipeNodeId, RoleSeg, SegPat, SegTag, Selector, StableName, evaluate,
};
use geom_core::Tol;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("a length literal")
}

fn eval(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// A unit box as an extruded square, and its extrude node.
fn box_doc() -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = fixture::insert(
        ProfileDoc::empty_derived("lib_u7_select", Tol::witness()),
        Node::Profile(fixture::desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    )
}

/// The kind-only selector: what `all_<kind>` says, in the selector's
/// own vocabulary.
fn just(kind: EntityKind) -> Selector {
    Selector::of(NamePat::of_kind(kind))
}

// ------------------------------------------------------------------
// 1. The materializer siblings.
// ------------------------------------------------------------------

/// **A box, counted four ways.** The siblings are the same door as
/// `all_edges` pointed at the other three kinds, and an extruded
/// square's table is the arithmetic that proves each one filtered on
/// the kind it claims: 6 faces, 12 edges, 8 vertices, 1 body.
#[test]
fn the_siblings_count_an_extruded_square() {
    let (doc, cube) = box_doc();
    let ev = eval(&doc);
    assert_eq!(editor_core::all_faces(&ev, cube).len(), 6);
    assert_eq!(editor_core::all_edges(&ev, cube).len(), 12);
    assert_eq!(editor_core::all_vertices(&ev, cube).len(), 8);
    assert_eq!(editor_core::all_bodies(&ev, cube).len(), 1);
}

/// **Canonical order, like `all_edges`** — sorted and deduped, ready
/// for `Node::fillet` (the M6-5 canonicality gate refuses anything
/// else at construction).
#[test]
fn the_siblings_return_canonical_order() {
    let (doc, cube) = box_doc();
    let ev = eval(&doc);
    for names in [
        editor_core::all_faces(&ev, cube),
        editor_core::all_vertices(&ev, cube),
        editor_core::all_bodies(&ev, cube),
    ] {
        assert!(names.windows(2).all(|w| w[0] < w[1]), "canonical");
    }
}

/// **Empty, never guessed at** — the `all_edges` contract verbatim: a
/// node with no value, no table, or none of the kind yields no names,
/// and the FILLET is where an empty selection refuses.
#[test]
fn the_siblings_and_the_selector_are_empty_for_a_valueless_node() {
    let (doc, p) = fixture::insert(
        ProfileDoc::empty_derived("lib_u7_select", Tol::witness()),
        Node::Profile(fixture::desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
    );
    let ev = eval(&doc);
    let absent = RecipeNodeId(999);
    for node in [p, absent] {
        assert!(editor_core::all_faces(&ev, node).is_empty());
        assert!(editor_core::all_vertices(&ev, node).is_empty());
        assert!(editor_core::all_bodies(&ev, node).is_empty());
        assert!(editor_core::select(&ev, node, &just(EntityKind::Face)).is_empty());
    }
}

/// **The siblings ARE the kind-only selector.** Two spellings, one
/// answer, on a document whose table carries every kind — so a change
/// to either filter that the other does not follow fails here.
#[test]
fn every_sibling_agrees_with_the_kind_only_selector() {
    let (doc, cube) = box_doc();
    let ev = eval(&doc);
    for (kind, materialized) in [
        (EntityKind::Face, editor_core::all_faces(&ev, cube)),
        (EntityKind::Edge, editor_core::all_edges(&ev, cube)),
        (EntityKind::Vertex, editor_core::all_vertices(&ev, cube)),
        (EntityKind::Body, editor_core::all_bodies(&ev, cube)),
    ] {
        assert_eq!(
            editor_core::select(&ev, cube, &just(kind)),
            materialized,
            "the {kind:?} sibling and the kind-only selector must agree"
        );
    }
}

// ------------------------------------------------------------------
// 2. The structural vocabulary, on a shape whose names are known.
// ------------------------------------------------------------------

/// **Motivating example 1: "the cap rim of the top face."** One
/// segment pattern — variant `RimEdge`, side `Top`, profile segment
/// free — and the four names come back. The `Bottom` rim is a
/// different value of the same pattern, which is the point of naming
/// the side rather than enumerating.
#[test]
fn a_rim_edge_pattern_says_the_cap_rim_of_one_end() {
    let (doc, cube) = box_doc();
    let ev = eval(&doc);
    for end in [CapEnd::Top, CapEnd::Bottom] {
        let sel = Selector::of(
            NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::RimEdge).side(end)),
        );
        let names = editor_core::select(&ev, cube, &sel);
        assert_eq!(names.len(), 4, "a square has four rim edges per cap");
        for n in &names {
            assert!(
                matches!(n.path.as_slice(), [RoleSeg::RimEdge(e, _)] if *e == end),
                "every hit is a {end:?} rim edge, got {n:?}"
            );
        }
    }
    // Unconstrained side: both caps, so eight.
    let both = Selector::of(NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::RimEdge)));
    assert_eq!(editor_core::select(&ev, cube, &both).len(), 8);
}

/// **Op groups are addressable**, so a pattern can say "whatever the
/// extrude minted" without naming five variants. A box's table is the
/// extrude group plus the one shared `OutputBody`.
#[test]
fn op_groups_partition_a_boxs_table() {
    let (doc, cube) = box_doc();
    let ev = eval(&doc);
    let by_group = |g: OpGroup| {
        editor_core::select(
            &ev,
            cube,
            &Selector::of(NamePat::any().seg(SegPat::group(g))),
        )
        .len()
    };
    assert_eq!(by_group(OpGroup::Shared), 1, "the output body");
    assert_eq!(by_group(OpGroup::Extrude), 26, "6 + 12 + 8");
    assert_eq!(by_group(OpGroup::Boolean), 0, "no boolean in this recipe");
}

/// **A constrained path is EXACT in length.** A two-segment pattern
/// never matches a one-segment name — composition is visible in the
/// pattern or it is not there at all, which is what keeps
/// `FromA(RimEdge(..))` distinguishable from a bare `RimEdge(..)`.
#[test]
fn a_path_pattern_matches_length_for_length() {
    let (doc, cube) = box_doc();
    let ev = eval(&doc);
    let two = Selector::of(NamePat::any().path([SegPat::any(), SegPat::any()]));
    assert!(editor_core::select(&ev, cube, &two).is_empty());
    let one = Selector::of(NamePat::any().path([SegPat::any()]));
    assert_eq!(editor_core::select(&ev, cube, &one).len(), 27);
    // No path constraint at all matches every row, whatever its shape.
    let free = Selector::of(NamePat::any());
    assert_eq!(editor_core::select(&ev, cube, &free).len(), 27);
}

/// **An empty selector selects nothing** — and says so here rather
/// than at the fillet, which is where an empty SELECTION refuses
/// (`FilletSelectionEmpty`, one door for that refusal).
#[test]
fn an_empty_selector_matches_nothing() {
    let (doc, cube) = box_doc();
    let ev = eval(&doc);
    assert!(editor_core::select(&ev, cube, &Selector::default()).is_empty());
}

// ------------------------------------------------------------------
// 3. The acceptance: `die_composed`'s fourteen names, materialized.
// ------------------------------------------------------------------

/// The composed die's fillet node, the target it blends, and the two
/// operand nodes the authored list needs — all recovered from the
/// document and its own table, exactly as `m6_composed_node.rs` does
/// (nothing here restates an id the document already knows).
fn composed_ids(
    doc: &ProfileDoc,
    ev: &editor_core::Evaluation<f64>,
) -> (RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let (_, pipped) = doc
        .order()
        .iter()
        .find_map(|id| match doc.node(*id) {
            Some(Node::Fillet { target, .. }) => Some((*id, *target)),
            _ => None,
        })
        .expect("the composed die has a fillet node");
    let operand = |pick: fn(&RoleSeg) -> Option<&StableName>| {
        editor_core::all_edges(ev, pipped)
            .iter()
            .find_map(|n| n.path.first().and_then(pick).map(|inner| inner.node))
            .expect("the target's table carries this operand")
    };
    let cube = operand(|s| match s {
        RoleSeg::FromA(inner) => Some(&**inner),
        _ => None,
    });
    let ball = operand(|s| match s {
        RoleSeg::FromB(inner) => Some(&**inner),
        _ => None,
    });
    (cube, ball, pipped)
}

/// **THE ACCEPTANCE.** The structural selector materializes exactly
/// the fourteen names `die_composed` used to author by hand — same
/// set, same order, bit for bit. The authored list stays in the corpus
/// as an INDEPENDENT statement of the answer, so this is a real
/// equivalence and not a tautology: it is written in the boolean
/// emitter's vocabulary (twelve `prism_edges` names lifted through
/// `FromA`, two `Seam` names spelled out), while the selector is
/// written in role-path shapes and never names a profile segment, a
/// vertex index or an operand node at all.
///
/// P10's document-layer relocation is REMOVED for the structural case:
/// the document says what it wants, not which fourteen things that is.
#[test]
fn the_selector_materializes_exactly_the_authored_die_composed_selection() {
    let doc = die_composed::document();
    let ev = eval(&doc.doc);
    let (cube, ball, pipped) = composed_ids(&doc.doc, &ev);

    let materialized = editor_core::select(&ev, pipped, &die_composed::selector());
    let mut authored = die_composed::selection(cube, ball, pipped);
    authored.sort();
    authored.dedup();

    assert_eq!(authored.len(), 14, "the document's own count");
    assert_eq!(
        materialized, authored,
        "the structural selector and the authored fourteen must be the same names"
    );
}

/// **And the document actually STORES that set.** The corpus document
/// now materializes at build time; what lands in the recipe is the
/// frozen `Vec<StableName>`, canonical, indistinguishable from an
/// authored one — which is why every existing `die_composed` row
/// (evaluation at every ε, the interval lane, save/load/replay,
/// latency, the refusal pins in `m6_composed_node.rs`) is untouched.
#[test]
fn the_stored_selection_is_the_materialized_set() {
    let doc = die_composed::document();
    let ev = eval(&doc.doc);
    let (cube, ball, pipped) = composed_ids(&doc.doc, &ev);
    let stored = match doc.doc.node(doc.result.expect("a result node")) {
        Some(Node::Fillet { selection, .. }) => selection.clone(),
        other => panic!("expected a fillet, got {other:?}"),
    };
    let mut authored = die_composed::selection(cube, ball, pipped);
    authored.sort();
    assert_eq!(stored, authored);
}

/// **The exclusion is now a consequence, not an omission.** The two
/// co-surface cavity meridians are `FromB(Meridian(..))` names, and no
/// alternative of the selector admits a `FromB` segment — so the
/// refusal `die_composed` exists to document stays excluded BY
/// DESCRIPTION. Nobody has to remember to leave them out.
#[test]
fn the_selector_excludes_the_cavity_meridians_by_shape() {
    let doc = die_composed::document();
    let ev = eval(&doc.doc);
    let (_, ball, pipped) = composed_ids(&doc.doc, &ev);
    let selected = editor_core::select(&ev, pipped, &die_composed::selector());
    let all = editor_core::all_edges(&ev, pipped);
    assert_eq!(all.len(), 16, "the target's edge table");
    for meridian in die_composed::excluded_meridians(ball, pipped) {
        assert!(all.contains(&meridian), "the meridian is a live edge");
        assert!(
            !selected.contains(&meridian),
            "and the selector does not select it: {meridian:?}"
        );
    }
}

/// **Prefix argument matching**: constraining only the A side of a
/// `Seam` is a coarser, still-honest way to say the same pip rim —
/// "wherever the cube's top cap crosses something". The two arcs come
/// back without the pattern naming either band variant.
#[test]
fn a_seam_pattern_may_constrain_only_its_a_side() {
    let doc = die_composed::document();
    let ev = eval(&doc.doc);
    let (_, _, pipped) = composed_ids(&doc.doc, &ev);
    let sel = Selector::of(NamePat::of_kind(EntityKind::Edge).seg(
        SegPat::tag(SegTag::Seam).of([
            NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(CapEnd::Top)),
        ]),
    ));
    assert_eq!(editor_core::select(&ev, pipped, &sel).len(), 2);
}
