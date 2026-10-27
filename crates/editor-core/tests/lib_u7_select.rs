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

use crate::corpus;
use crate::fixture;

use crate::fixture::len;
use corpus::die_composed;
use editor_core::{
    CancelToken, CapEnd, EntityKind, EvalOptions, NamePat, Node, OpGroup, ProfileDoc, RecipeNodeId,
    RoleSeg, SegPat, SegTag, Selector, StableName, evaluate,
};
use geom_core::Tol;

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
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("lib_u7_select", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: p.into(),
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
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("lib_u7_select", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let ev = eval(&doc);
    let absent = RecipeNodeId::new(0, 999);
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
    for end in [CapEnd::End, CapEnd::Start] {
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
/// than at the fillet, whose insert is where an empty SELECTION
/// refuses (`SelectionFault::Count`, one door for that refusal).
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
/// document exactly as `m6_composed_node.rs` does
/// (nothing here restates an id the document already knows).
fn composed_ids(doc: &ProfileDoc) -> (RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let (_, pipped) = doc
        .ids()
        .iter()
        .find_map(|id| match doc.node(*id) {
            Some(Node::Fillet { selection, .. }) => Some((*id, doc.read_operation(*selection)?)),
            _ => None,
        })
        .expect("the composed die has a fillet node");
    let Some(Node::Subtract { from, tool, .. }) = doc.node(pipped) else {
        panic!("the fillet's target is the die's subtract");
    };
    let operand = |read| {
        doc.operation_of(read)
            .expect("an operand read has a producer")
    };
    // The tool is the pip, a placement of the ball; the ball's names
    // ride it verbatim, still carrying the revolve's id.
    let ball = match doc.node(operand(tool.read)) {
        Some(Node::Place { source, .. }) => operand(source.read),
        other => panic!("the die's tool is the placed ball, got {other:?}"),
    };
    (operand(from.read), ball, pipped)
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
    let (cube, ball, pipped) = composed_ids(&doc.doc);

    let materialized = editor_core::select(&ev, pipped, &die_composed::selector());
    let mut authored = die_composed::selection(&doc.doc, cube, ball, pipped);
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
    let (cube, ball, pipped) = composed_ids(&doc.doc);
    let stored = match doc.doc.node(doc.result.expect("a result node")) {
        Some(Node::Fillet { selection, .. }) => crate::fixture::selected(&doc.doc, *selection),
        other => panic!("expected a fillet, got {other:?}"),
    };
    let mut authored = die_composed::selection(&doc.doc, cube, ball, pipped);
    authored.sort();
    assert_eq!(stored, authored);
}

/// **The exclusion is now a consequence, not an omission.** The two
/// co-surface cavity meridians are `From(Meridian(..))` names, carried
/// from the tool's read, and no alternative of the selector admits a
/// `From` over a meridian — so the
/// refusal `die_composed` exists to document stays excluded BY
/// DESCRIPTION. Nobody has to remember to leave them out.
#[test]
fn the_selector_excludes_the_cavity_meridians_by_shape() {
    let doc = die_composed::document();
    let ev = eval(&doc.doc);
    let (_, ball, pipped) = composed_ids(&doc.doc);
    let selected = editor_core::select(&ev, pipped, &die_composed::selector());
    let all = editor_core::all_edges(&ev, pipped);
    assert_eq!(all.len(), 16, "the target's edge table");
    for meridian in die_composed::excluded_meridians(&doc.doc, &ev, ball, pipped) {
        assert!(all.contains(&meridian), "the meridian is a live edge");
        assert!(
            !selected.contains(&meridian),
            "and the selector does not select it: {meridian:?}"
        );
    }
}

/// **A seam's sides are a set, so a pattern matches them as one**:
/// each argument fits a distinct side, in either order, and a pattern
/// constraining one side fits a seam where either side does. The pip
/// rim's two arcs come back from the cap's side alone, from the band's
/// side alone, and with the band written first.
#[test]
fn a_seam_pattern_may_constrain_either_side_in_either_order() {
    let doc = die_composed::document();
    let ev = eval(&doc.doc);
    let (_, _, pipped) = composed_ids(&doc.doc);
    let carried = |seg: SegPat| {
        NamePat::of_kind(EntityKind::Face)
            .seg(SegPat::tag(SegTag::From).of([NamePat::of_kind(EntityKind::Face).seg(seg)]))
    };
    let cap = || carried(SegPat::tag(SegTag::Cap).side(CapEnd::End));
    let seam = |sides: Vec<NamePat>| {
        NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::Seam).of(sides))
    };
    let bands = |sides: &dyn Fn(NamePat) -> Vec<NamePat>| {
        Selector::any_of(
            [SegTag::Band, SegTag::BandPi].map(|t| seam(sides(carried(SegPat::tag(t))))),
        )
    };
    let rim = editor_core::select(&ev, pipped, &bands(&|band| vec![cap(), band]));
    assert_eq!(rim.len(), 2, "the pip rim is two arcs");
    for (what, sel) in [
        ("the cap's side alone", Selector::of(seam(vec![cap()]))),
        ("the band's side alone", bands(&|band| vec![band])),
        ("the band's side first", bands(&|band| vec![band, cap()])),
    ] {
        assert_eq!(
            editor_core::select(&ev, pipped, &sel),
            rim,
            "{what} says the pip rim"
        );
    }
}

/// Two boxes and their union, the low box `[0, 1]³` and the tall one
/// `[0.5, 1.5]² × [0, 2]`, the low one inserted first or second: which
/// one's read mints first, and so which side of each seam names first.
/// Each stands in the world where it was drawn. Returns the document,
/// the low box's extrude, the tall one's and the union.
fn two_boxes(low_first: bool) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let square = |doc, lo: f64, d: f64| {
        let (doc, p) = fixture::insert(
            doc,
            Node::Profile(fixture::desc(vec![vec![
                (lo, lo),
                (lo + 1.0, lo),
                (lo + 1.0, lo + 1.0),
                (lo, lo + 1.0),
            ]])),
        );
        let (doc, body) = fixture::insert(
            doc,
            Node::Extrude {
                profile: p.into(),
                distance: len(d),
            },
        );
        let (doc, placed) = fixture::in_world(doc, body, &editor_core::Frame::IDENTITY);
        (doc, body, placed)
    };
    let doc = ProfileDoc::empty_derived("lib_u7_select_two_boxes", Tol::witness());
    let (doc, (low, low_placed), (tall, tall_placed)) = if low_first {
        let (doc, low, low_placed) = square(doc, 0.0, 1.0);
        let (doc, tall, tall_placed) = square(doc, 0.5, 2.0);
        (doc, (low, low_placed), (tall, tall_placed))
    } else {
        let (doc, tall, tall_placed) = square(doc, 0.5, 2.0);
        let (doc, low, low_placed) = square(doc, 0.0, 1.0);
        (doc, (low, low_placed), (tall, tall_placed))
    };
    let (doc, union) = fixture::insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![low_placed.into(), tall_placed.into()]),
        },
    );
    (doc, low, tall, union)
}

/// **A seam pattern holds under re-authoring.** The same pattern over
/// the two boxes' union, written in either argument order, selects the
/// same seams whichever box was inserted first, though that decides
/// which side each seam names first. The union meets along four seams:
/// the low box's two walls inside the tall one and its top against the
/// tall one's two walls inside it. Each argument fits a DISTINCT side,
/// so two arguments that both ask for the low box fit none of them.
#[test]
fn a_seam_pattern_selects_alike_in_either_order_under_either_mint_order() {
    for low_first in [true, false] {
        let (doc, low, tall, union) = two_boxes(low_first);
        let ev = eval(&doc);
        let of = |node: RecipeNodeId| {
            NamePat::of_kind(EntityKind::Face)
                .seg(SegPat::tag(SegTag::From).of([NamePat::any().node(node)]))
        };
        let top = NamePat::of_kind(EntityKind::Face).seg(
            SegPat::tag(SegTag::From).of([NamePat::any()
                .node(low)
                .seg(SegPat::tag(SegTag::Cap).side(CapEnd::End))]),
        );
        let seams = |sides: Vec<NamePat>| {
            editor_core::select(
                &ev,
                union,
                &Selector::of(
                    NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::Seam).of(sides)),
                ),
            )
        };
        let all = seams(Vec::new());
        assert_eq!(
            all.len(),
            4,
            "low first {low_first}: the union's four seams"
        );
        // The premise: the mint order decides which side names first.
        for seam in &all {
            let RoleSeg::Seam { a, .. } = &seam.path[0] else {
                panic!("low first {low_first}: a seam is named by its sides: {seam:?}");
            };
            assert_eq!(
                of(low).matches(a),
                low_first,
                "low first {low_first}: the first-minted box's side names first"
            );
        }
        for (what, sides, want) in [
            ("low, tall", vec![of(low), of(tall)], 4),
            ("tall, low", vec![of(tall), of(low)], 4),
            ("low alone", vec![of(low)], 4),
            ("tall alone", vec![of(tall)], 4),
            ("the low top alone", vec![top.clone()], 2),
            ("tall, then the low top", vec![of(tall), top.clone()], 2),
            ("low twice", vec![of(low), of(low)], 0),
        ] {
            let got = seams(sides);
            assert_eq!(got.len(), want, "low first {low_first}: {what}");
            assert!(
                got.iter().all(|n| all.contains(n)),
                "low first {low_first}: {what} selects only seams"
            );
        }
    }
}

/// **A set's patterns are matched to distinct members, and a first fit
/// is not the answer.** On a `Merged` set of three faces, a wildcard
/// written before the end cap fits the end cap first; the cap pattern
/// then needs that member, and the match holds by moving the wildcard
/// to another. Three caps, or the end cap twice, fit no set with one end
/// cap: each pattern takes a member of its own.
#[test]
fn a_set_pattern_gives_each_argument_a_member_of_its_own() {
    let node = RecipeNodeId::new(0, 3);
    let face = |seg: RoleSeg| StableName {
        kind: EntityKind::Face,
        node,
        path: vec![seg],
    };
    let merged = StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Merged(vec![
            face(RoleSeg::Cap(CapEnd::End)),
            face(RoleSeg::Cap(CapEnd::Start)),
            face(RoleSeg::OutputBody),
        ])],
    };
    let cap = |end: CapEnd| NamePat::any().seg(SegPat::tag(SegTag::Cap).side(end));
    let any_cap = || NamePat::any().seg(SegPat::tag(SegTag::Cap));
    let set = |members: Vec<NamePat>| NamePat::any().seg(SegPat::tag(SegTag::Merged).of(members));
    for (what, members, want) in [
        (
            "a wildcard, then the end cap",
            vec![NamePat::any(), cap(CapEnd::End)],
            true,
        ),
        (
            "any cap, then the end cap",
            vec![any_cap(), cap(CapEnd::End)],
            true,
        ),
        (
            "every member",
            vec![NamePat::any(), NamePat::any(), NamePat::any()],
            true,
        ),
        (
            "the end cap twice",
            vec![cap(CapEnd::End), cap(CapEnd::End)],
            false,
        ),
        ("three caps", vec![any_cap(), any_cap(), any_cap()], false),
        ("four members", vec![NamePat::any(); 4], false),
    ] {
        assert_eq!(set(members).matches(&merged), want, "{what}");
    }
}

/// **A seam junction's lines are a set.** A junction vertex is named
/// by the sorted run of the seam lines meeting at it, so which line
/// comes first is only name order; a path pattern over it gives each
/// segment pattern a distinct line, in either order. A pattern twice
/// over one line fits no junction with one such line, and a path of
/// another shape still matches segment for segment.
#[test]
fn a_junction_path_pattern_matches_its_lines_in_either_order() {
    let node = RecipeNodeId::new(0, 3);
    let face = |seg: RoleSeg| StableName {
        kind: EntityKind::Face,
        node,
        path: vec![seg],
    };
    let line = |a: RoleSeg, b: RoleSeg| RoleSeg::Seam {
        a: face(a).into(),
        b: face(b).into(),
    };
    let end = || RoleSeg::Cap(CapEnd::End);
    let start = || RoleSeg::Cap(CapEnd::Start);
    let junction = StableName {
        kind: EntityKind::Vertex,
        node,
        path: vec![
            line(end(), RoleSeg::OutputBody),
            line(start(), RoleSeg::OutputBody),
        ],
    };
    let on = |cap: CapEnd| {
        SegPat::tag(SegTag::Seam).of([NamePat::any().seg(SegPat::tag(SegTag::Cap).side(cap))])
    };
    let path = |segs: Vec<SegPat>| NamePat::of_kind(EntityKind::Vertex).path(segs);
    for (what, segs, want) in [
        ("end, start", vec![on(CapEnd::End), on(CapEnd::Start)], true),
        ("start, end", vec![on(CapEnd::Start), on(CapEnd::End)], true),
        ("start, any", vec![on(CapEnd::Start), SegPat::any()], true),
        ("end twice", vec![on(CapEnd::End), on(CapEnd::End)], false),
        ("one line", vec![on(CapEnd::Start)], false),
    ] {
        assert_eq!(path(segs).matches(&junction), want, "{what}");
    }
    // A path that is not a junction is matched segment for segment.
    let mut edge = junction.clone();
    edge.kind = EntityKind::Edge;
    let path = |segs: Vec<SegPat>| NamePat::of_kind(EntityKind::Edge).path(segs);
    assert!(path(vec![on(CapEnd::End), on(CapEnd::Start)]).matches(&edge));
    assert!(!path(vec![on(CapEnd::Start), on(CapEnd::End)]).matches(&edge));
}
