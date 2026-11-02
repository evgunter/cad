//! FIX — the split seam's crossing collector and A11's member
//! vocabulary (`split-crossings-skip-pattern-mate-ends`).
//!
//! The collector that fills an instance's [`InterfaceRecord`] asks ONE
//! predicate for "is this reference a member reference" — the same one
//! the solve's member walk asks. These rows pin what that identity
//! buys, on the two shapes a family brings (D10 `[repetition]`):
//!
//! - a constraint whose end is a family's member IS an edge: kept
//!   across a cut of the family side, it is one crossing, and a cut of
//!   its own placement alone severs its read;
//! - a constraint whose end is a family head its name UNDERQUALIFIES —
//!   a `Member` keying one index of a family over two — is outside the
//!   vocabulary, and the AQ8 (b)-SKIP ruling makes it contribute no
//!   crossing however its ends fall. This is the reachable construction
//!   that tells a gate asking the vocabulary apart from one merely
//!   matching a head's spelling.
//!
//! - a constraint whose reference is read at an operand the walk cannot
//!   reach its head from is outside the vocabulary too, and here the
//!   head IS a live instance — so this is the shape a gate matching a
//!   head's spelling gets WRONG in the other direction, minting a
//!   record for a constraint that never solved.
//!
//! That SKIP ruling is `crates/editor-core/ASSEMBLY.md`'s AQ8 clause.
//!
//! The whole cut also pins A4's recorded map over a `Member` head: the
//! node ids and the index variable remap, the index's VALUE does not.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeSet;

use editor_core::{
    CapEnd, DocRef, DocumentId, EvalOptions, Formula, MateRef, Node, ProfileDoc, RecipeNodeId,
    RoleSeg,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{in_copy, in_member, insert, len, on_frame};
use geom_core::Tol;

/// The unit cube `[0,1]³`, as a whole part document, and its body.
fn block(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    )
}

/// [`block`] as a reference, and its body, placed in the part's world
/// ([`fixture::resolver::PartStore::insert_part`], which is what lets
/// [`in_part`] spell its caps through the placement).
pub(crate) fn block_ref(label: &str) -> (DocRef, RecipeNodeId) {
    fixture::resolver::PartStore::new().insert_part(block(label), Tol::witness())
}

/// **An instance of [`block`] `label`, pinned to the world at the
/// identity** over its own geometry, its part served alone: the
/// instance and the part's body.
pub(crate) fn pinned_block(
    doc: ProfileDoc,
    label: &str,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut store = PartStore::new();
    let (part, body) = store.insert_part(block(label), Tol::witness());
    let (doc, id) = fixture::insert_pinned(
        doc,
        crate::fixture::mated_instance(part),
        &with_resolver(store),
    );
    (doc, id, body)
}

/// **The reach a document of [`block`]s evaluates through**: a store
/// holding the block of each label.
pub(crate) fn blocks_reach(labels: &[&str]) -> EvalOptions {
    let mut store = PartStore::new();
    for label in labels {
        store.insert_part(block(label), Tol::witness());
    }
    with_resolver(store)
}

/// The reach a cut of the four-legs document evaluates through: a store
/// holding both blocks.
fn legs_reach() -> EvalOptions {
    blocks_reach(&["fix-xs-leg", "fix-xs-top"])
}

/// **`mover`'s face seated on `onto`** by `placement`'s mate: a `Rest`
/// of the two planes, the mover's origin over the target's
/// ([`crate::p2_split::seat_at`]). The door is structural, so a target
/// whose face reaches no member is admitted as written.
pub(crate) fn seat(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    onto: impl crate::fixture::Sited,
    mover: impl crate::fixture::Sited,
) -> (ProfileDoc, MateRef) {
    crate::p2_split::seat_at(
        doc,
        placement,
        fixture::head(mover),
        fixture::head(onto),
        [0.0, 0.0],
    )
}

/// The member the family-headed rows mate to.
const COPY: u32 = 2;

/// Four legs, one top: `leg`, a row `pattern` of it in the world (count
/// 4, every member in the product), a `top`, and a seat constraint
/// placing the top on member [`COPY`]. Returns
/// `(doc, leg, pattern, top, mate, leg_body)`, the last the leg part's
/// body.
///
/// The part document mints its own id space, so the recorded map's
/// rewrite is observable rather than an identity.
fn four_legs(
    label: &str,
) -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    MateRef,
    RecipeNodeId,
) {
    let o = legs_reach();
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (leg_ref, leg_body) = block_ref("fix-xs-leg");
    let (doc, leg) = fixture::insert_pinned(doc, crate::fixture::mated_instance(leg_ref), &o);
    let (doc, k, pattern) = fixture::linear(doc, leg, [1.0, 0.0, 0.0], 2.0, 4);
    let (top_ref, top_body) = block_ref("fix-xs-top");
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, mate) = seat(
        doc,
        top,
        in_copy(pattern, k, COPY, in_part(leg, leg_body, CapEnd::End)),
        in_part(top, top_body, CapEnd::Start),
    );
    (doc, leg, pattern, top, mate, leg_body)
}

/// **`ids` and the world copies of what it moves**: every world
/// placement of a cut body ([`fixture::with_placements`]) — what a split
/// moves with a body, since a part delivers only its world.
fn cut(doc: &ProfileDoc, ids: impl IntoIterator<Item = RecipeNodeId>) -> BTreeSet<RecipeNodeId> {
    let out: BTreeSet<RecipeNodeId> = ids.into_iter().collect();
    fixture::with_placements(doc, &out)
}

/// [`fixture::mate_members`] for `mate` alone: every copy here also
/// stands in the world by a mate of its own, whose side lands on its
/// own instance and is not what these rows ask about.
fn seam_members(doc: &ProfileDoc, mate: MateRef) -> Vec<(MateRef, RecipeNodeId)> {
    fixture::mate_members(doc)
        .into_iter()
        .filter(|&(m, _)| m == mate)
        .collect()
}

fn crossings(
    doc: &editor_core::ProfileDoc,
    instance: RecipeNodeId,
) -> &editor_core::InterfaceRecord {
    let Some(Node::InstantiatePart { interface, .. }) = doc.node(instance) else {
        panic!("the split minted an instance");
    };
    interface
}

/// INVARIANT: a family-headed constraint end is a member end, so the
/// constraint reads the family's placement at its member and the other
/// member — which is what puts a family-headed constraint under AQ8's
/// crossing gate with plain edges rather than beside it.
#[test]
fn a_family_headed_mate_is_an_edge_and_reads_the_familys_placement() {
    let (doc, _leg, pattern, top, mate, _) = four_legs("fix-xs-edge");
    assert_eq!(
        seam_members(&doc, mate),
        vec![(mate, top), (mate, pattern)],
        "the family-headed head reads through the family's placement"
    );
}

/// INVARIANT (AQ8, for a FAMILY-HEADED edge): cutting the family side
/// keeps the top's constraint, whose target reads a cut member in the
/// world; the top is an instance, whose face is its
/// part's name, so the crossing goes unrecorded (the D′3 seam hazard).
/// Cutting the top alone takes its constraint with it, whose read of
/// the kept pattern copy the cut severs. The whole cut carries both
/// ends, so its record is empty.
///
/// *Breaks if* a crossing is minted from a part's name, or a cut
/// placement's read of a kept copy is carried in silence.
#[test]
fn a_family_headed_mate_edge_crosses_a_cut_of_its_target_and_severs_a_cut_of_its_placement() {
    let o = legs_reach();
    let (doc, leg, pattern, top, _mate, _leg_body) = four_legs("fix-xs-torn");
    let across = editor_core::split(
        &doc,
        &cut(&doc, [leg, pattern]),
        DocumentId::derive("fix-xs-torn-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("a cut of the family side splits");
    // The kept top is an instance, so its face is its part's name, which
    // a record has no spelling for: the crossing goes unrecorded
    // (`work/intent/a-part-name-in-the-host-is-spelled-in-the-parts-ids.md`).
    let record = crossings(&across.remainder, across.instance);
    assert!(
        record.crossings.is_empty(),
        "a kept instance's face crosses unrecorded: {record:?}"
    );

    // The top alone: its constraint goes with it, and reads the kept
    // member.
    let severed = editor_core::split(
        &doc,
        &cut(&doc, [top]),
        DocumentId::derive("fix-xs-torn-top-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect_err("a cut placement's read of a kept member is severed");
    let editor_core::SplitError::SeveredEdge {
        consumer,
        consumer_is_cut,
        ..
    } = severed
    else {
        panic!("cutting the top alone severs its constraint's read: {severed:?}");
    };
    assert_eq!(
        (consumer, consumer_is_cut),
        (doc.spoken(top), true),
        "the top is the cut-side consumer"
    );

    // The family WITHOUT its source is a severed recipe edge — D-2
    // closure, and the premise that keeps a family head's derivation
    // nodes on the same side as the member it resolves to.
    let severed = editor_core::split(
        &doc,
        &cut(&doc, [pattern]),
        DocumentId::derive("fix-xs-severed-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect_err("a severed recipe edge refuses");
    let editor_core::SplitError::SeveredEdge {
        consumer,
        input,
        consumer_is_cut,
        ..
    } = severed
    else {
        panic!("cutting the family alone severs its source edge: {severed:?}");
    };
    assert_eq!(
        (consumer, input, consumer_is_cut),
        (doc.spoken(pattern), doc.spoken(leg), true),
        "the family is the cut-side consumer and its instance source is the severed end"
    );

    // The whole cut: accepted, and nothing crosses.
    let out = editor_core::split(
        &doc,
        &cut(&doc, [leg, pattern, top]),
        DocumentId::derive("fix-xs-whole-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("a whole cut splits");
    assert!(
        crossings(&out.remainder, out.instance).is_empty(),
        "both ends moved with the cut, so the constraint says nothing about the seam"
    );
}

/// INVARIANT (A4's recorded map over a `Member` head): the map is a
/// correspondence between ID SPACES and nothing else. The moved
/// constraint's family end has its ids rewritten — the family head, the
/// master under the qualifier, and the index the qualifier keys — while
/// the index's VALUE crosses unchanged, because the family's own values
/// move verbatim and member `i` denotes the same member on both sides.
#[test]
fn the_recorded_map_rewrites_a_family_head_s_ids_and_never_its_member_value() {
    let (doc, leg, pattern, top, _, leg_body) = four_legs("fix-xs-remap");
    let o = legs_reach();
    let out = editor_core::split(
        &doc,
        &cut(&doc, [leg, pattern, top]),
        DocumentId::derive("fix-xs-remap-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("a whole cut splits");

    let (new_leg, new_pattern, new_top) = (
        out.node_map[&leg],
        out.node_map[&pattern],
        out.node_map[&top],
    );
    assert_ne!(new_pattern, pattern, "the part mints its own id space");

    let moved: Vec<_> = fixture::bundle(&out.part, new_top)
        .into_iter()
        .filter_map(|c| c.mate().cloned())
        .collect();
    let [moved] = moved.as_slice() else {
        panic!("the top's one mate moved into the part with it: {moved:?}");
    };
    let [a] = fixture::face_reads(&out.part, moved.to)[..] else {
        panic!("the seat's target reads a face");
    };
    let a = editor_core::head_of(&out.part, a).expect("the moved head resolves");
    let new_k = out
        .part
        .var_named("k")
        .expect("the index crosses with its family");
    assert_eq!(
        a,
        crate::fixture::head(in_copy(
            new_pattern,
            new_k,
            COPY,
            in_part(new_leg, leg_body, CapEnd::End)
        )),
        "ids remap through the recorded map; the member's value does not"
    );
    let RoleSeg::Member { ref at, .. } = a.name.path[0] else {
        panic!("the head keeps its Member qualifier");
    };
    assert_eq!(
        at.as_slice(),
        [(new_k, COPY)],
        "the index is the part's, its value the member's"
    );
}

/// A head whose `Member` qualifier keys FEWER indices than its family
/// is over is outside A11's member vocabulary, so the constraint is not
/// an edge, and its two ends reach opposite sides of an accepted cut.
///
/// A member of a family of families is keyed by both indices, and a
/// name keying both resolves. The name here keys ONE of the two — the
/// name the family's table never mints — so the walk refuses it at the
/// family, structurally, before any value is read.
///
/// INVARIANT (AQ8 option (b), SKIP — `crates/editor-core/ASSEMBLY.md`'s
/// AQ8 clause):
/// such a constraint contributes NO crossing however its names fall,
/// because it never solved and a record minted from it would be
/// trusted-at-rest state. This is the row that separates a gate asking
/// the member vocabulary from one matching a head's SPELLING: admitting
/// a family's placement by shape mints a record here, for a constraint the
/// member walk never resolved.
#[test]
fn an_underqualified_family_head_reaches_the_seam_and_contributes_no_crossing() {
    let o = blocks_reach(&["fix-xs-n-leg"]);
    let doc = ProfileDoc::empty(DocumentId::derive("fix-xs-nested"), Tol::witness());
    let (doc, leg, leg_body) = pinned_block(doc, "fix-xs-n-leg");
    let (doc, i, inner) = fixture::linear(doc, leg, [1.0, 0.0, 0.0], 2.0, 2);
    let (doc, j) = fixture::index_of(doc, "j", Formula::count(2), Some(i));
    let (doc, outer) =
        fixture::family_of(doc, inner, fixture::slides_along(j, [0.0, 1.0, 0.0], 5.0));
    let (top_ref, top_body) = block_ref("fix-xs-n-top");
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    // The constraint door is structural, so a target whose head
    // resolves to no member is admitted as written.
    let (doc, mate) = seat(
        doc,
        top,
        in_member(outer, vec![(j, 1)], in_part(leg, leg_body, CapEnd::End)),
        in_part(top, top_body, CapEnd::Start),
    );

    // No member at the nested head: only the top's side resolves.
    assert_eq!(
        seam_members(&doc, mate),
        vec![(mate, top)],
        "an underqualified head resolves to no member"
    );

    // The cut is accepted, with the constraint's ends on opposite
    // sides — the reachable seam a spelling-matched gate would mint on.
    let out = editor_core::split(
        &doc,
        &cut(&doc, [leg, inner, outer]),
        DocumentId::derive("fix-xs-nested-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("the cut is closed: the kept top reads the cut side");
    assert!(
        crossings(&out.remainder, out.instance).is_empty(),
        "a constraint that is not an edge says nothing about the seam (AQ8 SKIP)"
    );
}

/// INVARIANT: an operand that is a live `InstantiatePart` under a
/// name its copy does not carry is refused where the mate evaluates.
///
/// A copy carries its part's names as they are (D′3), so a name names
/// no instance: the member walk is structural and reads no part, and
/// it admits the side read at the stranger as the stranger's member.
/// Whether the stranger's copy carries the name is the evaluation's to
/// say, and it refuses the side (`work/intent/a-part-name-in-the-host-
/// is-spelled-in-the-parts-ids.md`).
///
/// *Breaks if* a side read at an instance whose copy carries no such
/// name evaluates.
#[test]
fn a_stranded_operand_over_an_instance_head_is_refused_where_it_evaluates() {
    let doc = ProfileDoc::empty(DocumentId::derive("fix-xs-stranded"), Tol::witness());
    // A live instance that consumes nothing: it neither places nor
    // projects `leg`, so no walk from it reaches `leg`. Its part is a
    // wider block, so its ids are not the leg part's: a copy carries
    // its part's names, and two parts of one recipe name alike.
    let mut store = PartStore::new();
    let wider = {
        let doc = ProfileDoc::empty(DocumentId::derive("fix-xs-st-other"), Tol::witness());
        let (doc, profile) = on_frame(
            doc,
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)]],
        );
        insert(
            doc,
            Node::Extrude {
                profile: profile.into(),
                distance: len(1.0),
            },
        )
    };
    let (other, _) = store.insert_part(wider, Tol::witness());
    let (doc, stranger) = fixture::insert_pinned(
        doc,
        crate::fixture::mated_instance(other),
        &with_resolver(store.clone()),
    );
    let (doc, leg, leg_body) = pinned_block(doc, "fix-xs-st-leg");
    let (top_ref, top_body) = block_ref("fix-xs-st-top");
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    // The leg's cap, read at the stranger's output.
    let stranded = fixture::head_at(stranger, in_part(leg, leg_body, CapEnd::End));
    let (doc, mate) = fixture::constrain_at(
        doc,
        top,
        fixture::rest(
            fixture::head_at(top, in_part(top, top_body, CapEnd::Start)),
            stranded,
        ),
    );
    assert_eq!(
        seam_members(&doc, mate),
        vec![(mate, top), (mate, stranger)],
        "the walk reads no part: the stranger is the target's member"
    );
    for label in ["fix-xs-st-leg", "fix-xs-st-top"] {
        store.insert_part(block(label), Tol::witness());
    }
    let o = with_resolver(store);
    assert!(
        matches!(
            crate::fixture::solve(&doc, &o, Tol::witness()).fault(mate.placement),
            Some(editor_core::MateFault::SideUnevaluated {
                side: editor_core::MateSide::To,
                ..
            })
        ),
        "the stranger's copy carries no such name, so the target refuses"
    );
}

/// **A top seated on a family's member is proven by its mate.** The
/// four-legs document seats the top on member [`COPY`], which the
/// family places in the world: the at-rest census's row of the seat
/// names the leg's cap in that member, and the mate's target reads the
/// same cap through the family, so the door's first rung reads the row
/// as the mate's statement and the `unproven-coincidence` lint reports
/// nothing against the mate.
///
/// *Breaks if* the door stops reading a mate side through the member
/// that places it, or reads it in another member.
#[test]
fn a_top_seated_on_a_family_member_is_proven_by_its_mate() {
    let (doc, _, _, _, mate, _) = four_legs("fix-xs-seat-door");
    let ev = fixture::run(&doc, &legs_reach());
    let product =
        editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("the legs gather");
    assert!(
        product
            .coincidences
            .iter()
            .any(|row| row.row.relation == topo::Relation::SameOpposite),
        "the premise: the census decided the seat: {:?}",
        product.coincidences
    );
    let report = editor_core::run_checks(
        &doc,
        &ev,
        &editor_core::ChecksConfig::default(),
        Tol::witness(),
    )
    .expect("checks run over the legs");
    assert!(
        !report
            .findings
            .iter()
            .any(|f| f.check == editor_core::CheckId::UnprovenCoincidence
                && f.subject == editor_core::FindingSubject::Node(mate.placement)),
        "the seat on the member is proven: {report}"
    );
}
