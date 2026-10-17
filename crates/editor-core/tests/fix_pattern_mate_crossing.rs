//! FIX — the split seam's crossing collector and A11's member
//! vocabulary (`split-crossings-skip-pattern-mate-ends`).
//!
//! The collector that fills an instance's [`InterfaceRecord`] asks ONE
//! predicate for "is this reference a member reference" — the same one
//! the solve's member walk asks. These rows pin what that identity
//! buys, on the two shapes a pattern brings:
//!
//! - a constraint whose end is a pattern-placed instance IS an edge:
//!   kept across a cut of the pattern side, it is one crossing, and a
//!   cut of its own placement alone severs its read;
//! - a constraint whose end is a pattern head its name UNDERQUALIFIES —
//!   one `Instance(i)` over a two-level nest — is outside the
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
//! The whole cut also pins A4's recorded map over an `Instance(i)`
//! head: the node ids remap, the STRUCTURAL INDEX does not.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use std::collections::BTreeSet;

use editor_core::{
    CapEnd, ContactClass, DocRef, DocumentId, EvalOptions, Formula, MateRef, Node, PatternKind,
    ProfileDoc, RecipeNodeId, RoleSeg, StableName,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{in_copy, insert, len, on_frame, scl};
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
            side: ExtrudeSide::Along,
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
    let (doc, id) =
        fixture::insert_pinned(doc, Node::instantiate_part(part), &with_resolver(store));
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
    onto: StableName,
    mover: StableName,
) -> (ProfileDoc, MateRef) {
    crate::p2_split::seat_at(
        doc,
        placement,
        fixture::head(mover),
        fixture::head(onto),
        [0.0, 0.0],
    )
}

fn linear(spacing: f64) -> PatternKind<Formula> {
    PatternKind::Linear {
        direction: [scl(1.0), scl(0.0), scl(0.0)],
        spacing: len(spacing),
    }
}

/// The copy index the pattern-headed rows mate to.
const COPY: u32 = 2;

/// Four legs, one top: `leg`, a linear `pattern` of it (count 4), a
/// `top`, and a seat constraint placing the top on pattern copy
/// [`COPY`]. Returns `(doc, leg, pattern, top, mate, leg_body)`, the
/// last the leg part's body.
///
/// A bare datum sits AHEAD of the instances and is never cut, so the part
/// document's id space is offset from the host's — which is what makes
/// the recorded map's rewrite observable rather than an identity.
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
    let (doc, _kept) = insert(doc, fixture::xy_frame());
    let (leg_ref, leg_body) = block_ref("fix-xs-leg");
    let (doc, leg) = fixture::insert_pinned(doc, Node::instantiate_part(leg_ref), &o);
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: Formula::count(4),
            kind: linear(2.0),
        },
    );
    let (top_ref, top_body) = block_ref("fix-xs-top");
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let doc = place_copies(doc, pattern, 4, &o);
    let (doc, mate) = seat(
        doc,
        top,
        in_copy(pattern, COPY, in_part(leg, leg_body, CapEnd::End)),
        in_part(top, top_body, CapEnd::Start),
    );
    (doc, leg, pattern, top, mate, leg_body)
}

/// **`ids` and the world copies of what it moves**: each `Part` that
/// selects a copy of a cut pattern ([`place_copies`]), then every
/// world placement of a cut body ([`fixture::with_placements`]) — what
/// a split moves with a body, since a part delivers only its world.
fn cut(doc: &ProfileDoc, ids: impl IntoIterator<Item = RecipeNodeId>) -> BTreeSet<RecipeNodeId> {
    let mut out: BTreeSet<RecipeNodeId> = ids.into_iter().collect();
    for id in doc.ids() {
        if let Some(Node::Part { of, .. }) = doc.node(id)
            && doc.operation_of(*of).is_some_and(|at| out.contains(&at))
        {
            out.insert(id);
        }
    }
    fixture::with_placements(doc, &out)
}

/// **Each of `pattern`'s `count` copies placed in the world**, in
/// instance order: a `Part` per copy, each placed at the identity —
/// the world a pattern's copies are in the product by. `o` evaluates
/// the copies each pin is read off.
pub(crate) fn place_copies(
    doc: ProfileDoc,
    pattern: RecipeNodeId,
    count: i64,
    o: &EvalOptions,
) -> ProfileDoc {
    (0..count).fold(doc, |doc, i| {
        let (doc, copy) = insert(
            doc,
            Node::Part {
                of: pattern.into(),
                select: editor_core::PartSelect::Instance(Formula::count(i)),
            },
        );
        fixture::insert_pinned(doc, Node::place(vec![editor_core::BodyRead::plain(copy)]), o)
            .0
    })
}

/// The world placement of `pattern`'s copy `i` ([`place_copies`]).
fn copy_placement(doc: &ProfileDoc, pattern: RecipeNodeId, i: u32) -> RecipeNodeId {
    let part = doc
        .ids()
        .into_iter()
        .find(|&id| {
            matches!(doc.node(id), Some(Node::Part { of, select: editor_core::PartSelect::Instance(n) })
                if doc.operation_of(*of) == Some(pattern)
                    && doc.written(&editor_core::Expr::var(*n, editor_core::Dimension::Count))
                        == Formula::count(i64::from(i)))
        })
        .expect("the copy is selected");
    doc.placements()
        .into_iter()
        .find(|&p| {
            matches!(doc.node(p), Some(Node::Place { shapes, .. })
                if shapes.iter().any(|s| doc.operation_of(s.read) == Some(part)))
        })
        .expect("the copy is placed")
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

/// INVARIANT: a pattern-placed constraint end is a member end, so the
/// constraint reads the pattern's INPUT instance and the other member —
/// which is what puts a pattern-headed constraint under AQ8's crossing
/// gate with plain edges rather than beside it.
#[test]
fn a_pattern_headed_mate_is_an_edge_and_reads_the_pattern_input_instance() {
    let (doc, leg, _pattern, top, mate, _) = four_legs("fix-xs-edge");
    assert_eq!(
        crate::fixture::mate_members(&doc),
        vec![(mate, top), (mate, leg)],
        "the pattern-placed head reads through the pattern's input instance"
    );
}

/// INVARIANT (AQ8, for a PATTERN-HEADED edge): cutting the pattern
/// side keeps the top's constraint, whose target reads a cut copy
/// placed at the identity — one crossing, the top's face outside, the
/// pattern copy's inside, in the part's names. Cutting the top alone
/// takes its constraint with it, whose read of the kept pattern copy
/// the cut severs. The whole cut carries both ends, so its record is
/// empty.
///
/// *Breaks if* the crossing is dropped or minted from the wrong side,
/// or a cut placement's read of a kept copy is carried in silence.
#[test]
fn a_pattern_headed_mate_edge_crosses_a_cut_of_its_target_and_severs_a_cut_of_its_placement() {
    let o = legs_reach();
    let (doc, leg, pattern, top, mate, leg_body) = four_legs("fix-xs-torn");
    let across = editor_core::split(
        &doc,
        &cut(&doc, [leg, pattern]),
        DocumentId::derive("fix-xs-torn-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("a cut of the pattern side splits");
    let record = crossings(&across.remainder, across.instance);
    let [
        editor_core::InterfaceCrossing::Mate {
            class,
            outer,
            inner,
        },
    ] = record.crossings.as_slice()
    else {
        panic!("the kept constraint is one crossing: {record:?}");
    };
    let mate = fixture::constraint(&doc, mate);
    let [top_side] = fixture::face_reads(&doc, mate.on)[..] else {
        panic!("the seat reads the top's face");
    };
    assert_eq!(*class, ContactClass::Rest, "the crossing keeps its class");
    assert_eq!(
        *outer,
        editor_core::head_of(&doc, top_side)
            .expect("the top's side resolves")
            .name,
        "the outside reference is the kept top's face"
    );
    // The part's product spells the copy's face under the cut
    // placement of that copy (`Placed`), in the part's ids.
    let held = StableName {
        kind: editor_core::EntityKind::Face,
        node: across.node_map[&copy_placement(&doc, pattern, COPY)],
        path: vec![RoleSeg::Placed {
            of: editor_core::NameRef::new(in_copy(
                across.node_map[&pattern],
                COPY,
                in_part(across.node_map[&leg], leg_body, CapEnd::End),
            )),
        }],
    };
    assert_eq!(
        inner.clone().into_name(),
        held,
        "the inside reference is the pattern copy's face, in the part's names"
    );

    // The top alone: its constraint goes with it, and reads the kept
    // pattern copy.
    let severed = editor_core::split(
        &doc,
        &cut(&doc, [top]),
        DocumentId::derive("fix-xs-torn-top-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect_err("a cut placement's read of a kept copy is severed");
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

    // The pattern WITHOUT its input is a severed recipe edge, refused
    // before any placement rule is reached — D-2 closure, and the
    // premise that keeps a pattern head's derivation nodes on the same
    // side as the member it resolves to.
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
        panic!("cutting the pattern alone severs its input edge: {severed:?}");
    };
    assert_eq!(
        (consumer, input, consumer_is_cut),
        (doc.spoken(pattern), doc.spoken(leg), true),
        "the pattern is the cut-side consumer and its instance input is the severed end"
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

/// INVARIANT (A4's recorded map over an `Instance(i)` head): the map is
/// a correspondence between NODE ID SPACES and nothing else. The moved
/// constraint's pattern-placed end has both its node ids rewritten — the
/// pattern head and the master under the qualifier — while the
/// STRUCTURAL INDEX crosses unchanged, because the pattern's own rule
/// moves verbatim and copy `i` denotes the same copy on both sides.
#[test]
fn the_recorded_map_rewrites_a_pattern_head_s_ids_and_never_its_copy_index() {
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
    let [a] = fixture::face_reads(&out.part, *moved.to.pose().expect("a pose target"))[..] else {
        panic!("the seat's target reads a face");
    };
    let a = editor_core::head_of(&out.part, a).expect("the moved head resolves");
    assert_eq!(
        a,
        crate::fixture::head(in_copy(
            new_pattern,
            COPY,
            in_part(new_leg, leg_body, CapEnd::End)
        )),
        "ids remap through the recorded map; the copy index does not"
    );
    let RoleSeg::Instance { i, .. } = a.name.path[0] else {
        panic!("the head keeps its Instance(i) qualifier");
    };
    assert_eq!(i, COPY, "the structural index is not in the map's domain");
}

/// A head whose name qualifies FEWER copy levels than the walk meets
/// is outside A11's member vocabulary, so the constraint is not an
/// edge, and its two ends reach opposite sides of an accepted cut.
///
/// A nested COPY is a member: the walk consumes one `Instance(i)`
/// qualifier per pattern level, and a name that wears one at each
/// level resolves through the nest. The name here wears ONE over a
/// two-level nest — the name a nested pattern's table never mints —
/// so the walk consumes the outer level and then meets the inner
/// pattern where the name has already said its head is the instance.
///
/// INVARIANT (AQ8 option (b), SKIP — `crates/editor-core/ASSEMBLY.md`'s
/// AQ8 clause):
/// such a constraint contributes NO crossing however its names fall,
/// because it never solved and a record minted from it would be
/// trusted-at-rest state. This is the row that separates a gate asking
/// the member vocabulary from one matching a head's SPELLING: admitting
/// `Node::Pattern` by shape mints a record here, for a constraint the
/// member walk never resolved.
#[test]
fn an_underqualified_pattern_head_reaches_the_seam_and_contributes_no_crossing() {
    let o = blocks_reach(&["fix-xs-n-leg"]);
    let doc = ProfileDoc::empty(DocumentId::derive("fix-xs-nested"), Tol::witness());
    let (doc, leg, leg_body) = pinned_block(doc, "fix-xs-n-leg");
    let (doc, inner) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: Formula::count(2),
            kind: linear(2.0),
        },
    );
    let (doc, outer) = insert(
        doc,
        Node::Pattern {
            input: inner.into(),
            count: Formula::count(2),
            kind: linear(5.0),
        },
    );
    let (top_ref, top_body) = block_ref("fix-xs-n-top");
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let doc = place_copies(doc, outer, 2, &o);
    // The constraint door is structural, so a target whose head
    // resolves to no member is admitted as written.
    let (doc, mate) = seat(
        doc,
        top,
        in_copy(outer, 1, in_part(leg, leg_body, CapEnd::End)),
        in_part(top, top_body, CapEnd::Start),
    );

    // No member at the nested head: only the top's side resolves.
    assert_eq!(
        crate::fixture::mate_members(&doc),
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
/// name derived from ANOTHER instance resolves to no member. The
/// constraint door is structural and admits the mate as written; the
/// member walk is what knows the reference resolves to nothing, so
/// the target contributes no member.
///
/// This is the shape that separates the vocabulary from a head's
/// SPELLING in the direction the nested-pattern row cannot reach:
/// there the head is a `Pattern`, so a gate matching
/// `Node::InstantiatePart` skips it for the right answer by accident;
/// here the head IS an `InstantiatePart` and only the walk knows the
/// reference resolves to nothing.
///
/// *Breaks if* the walk admits a name another instance minted because
/// the node it is read at is an instance.
#[test]
fn a_stranded_operand_over_an_instance_head_resolves_to_no_member() {
    let doc = ProfileDoc::empty(DocumentId::derive("fix-xs-stranded"), Tol::witness());
    let (doc, _datum) = insert(doc, fixture::xy_frame());
    // A live instance that consumes nothing: it neither places nor
    // projects `leg`, so no walk from it reaches `leg`.
    let (doc, stranger, _) = pinned_block(doc, "fix-xs-st-other");
    let (doc, leg, leg_body) = pinned_block(doc, "fix-xs-st-leg");
    let (top_ref, top_body) = block_ref("fix-xs-st-top");
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    // The leg's cap, read at the stranger's output.
    let stranded = fixture::head_at(stranger, in_part(leg, leg_body, CapEnd::End));
    let (doc, mate) = fixture::constrain_at(
        doc,
        top,
        fixture::rest(
            fixture::head(in_part(top, top_body, CapEnd::Start)),
            stranded,
            ContactClass::Rest,
        ),
    );
    assert_eq!(
        crate::fixture::mate_members(&doc),
        vec![(mate, top)],
        "the stranded target resolves to no member; only the top's side does"
    );
}
