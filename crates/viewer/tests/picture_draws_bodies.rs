//! **The picture draws bodies, not placements** (`crates/viewer/README.md`,
//! *The picture draws bodies*): a body is drawn unless an operation reads
//! it as a body; a read of one member takes that member alone; a `⊔` is
//! drawn as its pieces; the shown spaces are the world and every space
//! holding a drawn body; a drawn body the product slot does not reach is
//! tinted; free-move admits any shown space but the world's; hide is
//! keyed by body and by space.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::collections::BTreeSet;

use crate::common;
use common::asm;
use pncad::document::{
    Doc, DocumentId, Frame, Node, ProfileDoc, ProfileProgram, RecipeNodeId, VarId,
};
use pncad::geom_core::Tol;
use pncad::workspace::Workspace;
use viewer::display::{AdmissionFault, DisplayFault};
use viewer::pickindex::PickIndex;
use viewer::scene::SceneMesh;
use viewer::session::{DocSession, Refusal, SessionOp};
use viewer::world;

/// The box's side and depth: plate-scale, so `common::plate_index`'s δ
/// draws it.
const SIDE: f64 = 0.02;
const DEPTH: f64 = 0.01;

/// A square extruded `DEPTH`, its profile at the plane origin offset by
/// `x`: the document and the extrude, which nothing places.
fn extrude(doc: &Doc<ProfileProgram>, x: f64, tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let (doc, profile) = common::inserted(doc, common::rectangle([x, 0.0], SIDE, SIDE), tol);
    common::inserted(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            distance: common::len(DEPTH),
        },
        tol,
    )
}

/// An inline session over `doc`, landed.
fn landed(doc: Doc<ProfileProgram>, tol: Tol) -> DocSession {
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    session
}

/// Whether every corner `index` draws under `session`'s display view
/// carries the outside-the-product flag, and whether any does.
fn outside_flags(index: &PickIndex, session: &DocSession) -> (bool, bool) {
    let scene = index.scene_for(&session.display_view()).expect("a scene");
    let flagged = |word: &u32| word & SceneMesh::FLAG_OUTSIDE != 0;
    assert!(!scene.flags().is_empty(), "the premise: something is drawn");
    (
        scene.flags().iter().all(flagged),
        scene.flags().iter().any(flagged),
    )
}

/// **A construction nothing places is drawn at once, in its own space,
/// pickable as itself and tinted as outside the product; placed into the
/// world and put in the product, its own picture goes and the copy is
/// drawn untinted, while a sibling nothing reads stays drawn and
/// tinted.**
///
/// *Breaks if* the picture draws placements alone (nothing is drawn at
/// first), a pick on the construction seats anything but it, the tint
/// is not carried, or a placement's source stays drawn beside its copy.
#[test]
fn a_construction_nothing_places_is_drawn_tinted_and_goes_once_placed() {
    let tol = Tol::witness();
    let (doc, body) = extrude(&Doc::empty_derived("picture-construction", tol), 0.0, tol);
    let session = landed(doc.clone(), tol);
    let own = doc.node_space(body).expect("a construction has a space");
    assert_ne!(own, VarId::WORLD);
    assert_eq!(world::drawn_nodes(&doc), vec![body]);
    assert_eq!(
        world::shown_spaces(&doc),
        BTreeSet::from([VarId::WORLD, own]),
        "the construction's own space is shown"
    );
    let index = common::plate_index(&session);
    let hit = index
        .face_at(
            session.landed_pair().expect("landed").1,
            &common::down_at(SIDE / 2.0, SIDE / 2.0),
        )
        .expect("the pick answers")
        .expect("the ray meets the construction");
    assert_eq!(hit.node, body, "a pick on the construction is on it");
    assert_eq!(world::seat_of(&doc, hit.node), body, "and seats it");
    assert!(index.is_outside(body, 0), "the product reads nothing");
    assert_eq!(
        outside_flags(&index, &session),
        (true, true),
        "every corner is tinted"
    );

    // Placed into the world, beside a sibling nothing reads.
    let (doc, sibling) = extrude(&doc, 2.0 * SIDE, tol);
    let (doc, copy) = common::placed(&doc, body, tol);
    let session = landed(doc.clone(), tol);
    assert_eq!(
        world::drawn_nodes(&doc),
        vec![sibling, copy],
        "the copy's source leaves the picture; the sibling stays"
    );
    let sibling_space = doc.node_space(sibling).expect("its own space");
    assert_eq!(
        world::shown_spaces(&doc),
        BTreeSet::from([VarId::WORLD, sibling_space]),
        "the placed body's own space is no longer shown"
    );
    let index = common::plate_index(&session);
    assert!(!index.is_outside(copy, 0), "the product reads the copy");
    assert!(index.is_outside(sibling, 0), "and not the sibling");
    assert_eq!(outside_flags(&index, &session), (false, true));
}

/// **A fillet of a construction is drawn in its place**: the fillet
/// reads the body as its input, through the selection it blends, so the
/// body leaves the picture and the fillet is drawn where it stood.
///
/// *Breaks if* a selection's body is not read as a body (both drawn), or
/// the fillet is not drawn (nothing places it).
#[test]
fn a_fillet_of_a_construction_draws_the_fillet_alone() {
    let tol = Tol::witness();
    let (doc, body) = extrude(&Doc::empty_derived("picture-fillet", tol), 0.0, tol);
    let mut session = landed(doc, tol);
    let edges = {
        let eval = session.evaluation().expect("landed");
        pncad::select::all_edges(eval, body)
    };
    let fillet = common::session_insert(
        &mut session,
        SessionOp::AddFillet {
            target: body,
            radius: common::len(0.001),
            selection: edges,
        },
    );
    session.pump();
    let doc = session.committed_doc().clone();
    assert_eq!(world::drawn_nodes(&doc), vec![fillet]);
    let index = common::plate_index(&session);
    assert!(index.ids_of_node(body).is_empty(), "the input is not drawn");
    assert!(!index.ids_of_node(fillet).is_empty(), "the fillet is");
}

/// The bench's post, an instance moved into a space of its own, and a
/// row of three of its copies placed into that same space: the stored
/// assembly, the instance, its space and the family.
pub(crate) fn family_in_an_instances_space(
    tag: &str,
    tol: Tol,
) -> (
    asm::Bench,
    std::path::PathBuf,
    RecipeNodeId,
    VarId,
    RecipeNodeId,
) {
    let bench = asm::bench(tag, tol);
    let mut doc = ProfileDoc::empty(DocumentId::derive(&format!("picture-{tag}")), tol);
    let instance = common::insert_into(&mut doc, crate::fixture::mated_instance(bench.post), tol);
    let (doc, space) = crate::fixture::into_new_space(doc, instance);
    let (doc, _, family) = crate::fixture::linear(doc, instance, [0.05, 0.0, 0.0], 1.0, 3);
    let mut ws = Workspace::open(&bench.dir).expect("the store opens");
    let path = ws.create(&doc, tol).expect("the assembly stores");
    (bench, path, instance, space, family)
}

/// The session over the stored assembly at `path`, landed.
pub(crate) fn opened(path: std::path::PathBuf, tol: Tol) -> DocSession {
    let mut session = DocSession::inline(Doc::empty_derived("picture-boot", tol), tol);
    let outcome = session.perform(SessionOp::Open(path));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    session
}

/// **A family placed into an instance's own space shows that space,
/// draws every member and not the instance** — member 0 stands where
/// the instance stood. The family targets the space and reads the
/// instance; it is the read, not the target, that takes the instance
/// over. Then **a read of member 1 alone takes member 1 alone**.
///
/// *Breaks if* a placement into a space hides it (the space is not
/// shown), a family's source stays drawn, or a member read takes the
/// whole family or another member.
#[test]
fn a_family_in_an_instances_space_draws_its_members_and_a_member_read_takes_one() {
    let tol = Tol::witness();
    let (bench, path, instance, space, family) = family_in_an_instances_space("family", tol);
    let session = opened(path.clone(), tol);
    let doc = session.committed_doc().clone();
    assert_eq!(doc.node_space(family), Some(space), "placed into the space");
    assert!(
        world::shown_spaces(&doc).contains(&space),
        "the instance's space is shown"
    );
    assert_eq!(world::drawn_nodes(&doc), vec![family]);
    let index = asm::index_of(&session);
    assert!(
        index.ids_of_node(instance).is_empty(),
        "the instance is read"
    );
    for member in 0..3 {
        assert!(
            !index.ids_in(family, member).is_empty(),
            "member {member} is drawn"
        );
    }

    // Member 1, read alone, placed into the same space.
    let (doc, read) = common::member_read(&doc, family, 1, tol);
    let mut ws = Workspace::open(&bench.dir).expect("the store opens");
    ws.resave(&doc, tol).expect("the assembly stores");
    let session = opened(path, tol);
    let index = asm::index_of(&session);
    assert!(index.ids_in(family, 1).is_empty(), "member 1 is taken");
    for member in [0, 2] {
        assert!(
            !index.ids_in(family, member).is_empty(),
            "member {member} stays drawn"
        );
    }
    assert!(!index.ids_of_node(read).is_empty(), "the read is drawn");
}

/// **A `⊔` is drawn as its pieces**: the product's `⊔` of two world
/// copies draws each copy as itself, and a hide of the `⊔` takes both.
///
/// *Breaks if* a `⊔`'s operands are hidden with nothing drawn for them,
/// or a body's hide does not reach the pieces drawn for it.
#[test]
fn a_disjoint_union_is_drawn_as_its_pieces() {
    let tol = Tol::witness();
    let (doc, a) = extrude(&Doc::empty_derived("picture-pieces", tol), 0.0, tol);
    let (doc, b) = extrude(&doc, 2.0 * SIDE, tol);
    let (doc, a_copy) = common::placed(&doc, a, tol);
    let (doc, b_copy) = common::placed(&doc, b, tol);
    let assemble = doc
        .product()
        .and_then(|var| doc.operation_of(var))
        .expect("the product reads the world's copies");
    assert!(matches!(doc.node(assemble), Some(Node::Assemble { .. })));
    let mut session = landed(doc.clone(), tol);
    let drawn = world::drawn_bodies(&doc);
    assert_eq!(
        drawn
            .iter()
            .map(|d| (d.node, d.within.clone()))
            .collect::<Vec<_>>(),
        vec![(a_copy, vec![assemble]), (b_copy, vec![assemble])],
        "each copy, as a piece of the product's ⊔"
    );
    let index = common::plate_index(&session);
    assert!(
        index
            .scene_for(&session.display_view())
            .expect("a scene")
            .stats()
            .triangles
            > 0
    );
    let hidden = session.perform(SessionOp::SetBodyOn {
        node: assemble,
        on: false,
    });
    assert!(hidden.refusal.is_none(), "{:?}", hidden.refusal);
    assert_eq!(
        index
            .scene_for(&session.display_view())
            .expect("a scene")
            .stats()
            .triangles,
        0,
        "the ⊔'s hide takes every piece"
    );
}

/// **Free-move opens on a construction's space, moving what is drawn
/// there, and refuses the world; a space's hide takes every body drawn
/// in it.**
///
/// *Breaks if* the probe is gated on a node kind (the construction is
/// refused), the world is admitted, the frame does not reach the
/// construction, or a space's hide does not drop its bodies.
#[test]
fn free_move_opens_on_a_constructions_space_and_refuses_the_world() {
    let tol = Tol::witness();
    let (doc, body) = extrude(&Doc::empty_derived("picture-free-move", tol), 0.0, tol);
    let (doc, other) = extrude(&doc, 2.0 * SIDE, tol);
    let (doc, copy) = common::placed(&doc, other, tol);
    let mut session = landed(doc.clone(), tol);
    let own = doc.node_space(body).expect("its own space");
    let probe = Frame::translation([0.0, 0.0, 0.03]);
    for op in [
        SessionOp::BeginFreeMove { node: body },
        SessionOp::PreviewFreeMove {
            node: body,
            frame: probe,
        },
        SessionOp::CommitFreeMove { node: body },
    ] {
        let outcome = session.perform(op.clone());
        assert!(outcome.refusal.is_none(), "{op:?}: {:?}", outcome.refusal);
    }
    assert_eq!(session.display().free_move_of(own), Some(&probe));
    let view = session.display_view();
    assert_eq!(view.moved_nodes.get(&body), Some(&probe));
    assert!(!view.moved_nodes.contains_key(&copy), "the world stays");

    match session
        .perform(SessionOp::BeginFreeMove { node: copy })
        .refusal
    {
        Some(Refusal::Display(DisplayFault::Admission(AdmissionFault::InTheWorld { node }))) => {
            assert_eq!(node.id(), copy);
        }
        other => panic!("a body in the world refuses typed, got {other:?}"),
    }

    let index = common::plate_index(&session);
    let full = index
        .scene_for(&session.display_view())
        .expect("a scene")
        .stats()
        .triangles;
    let hidden = session.perform(SessionOp::SetSpaceHidden {
        space: own,
        hidden: true,
    });
    assert!(hidden.refusal.is_none(), "{:?}", hidden.refusal);
    let shown = index
        .scene_for(&session.display_view())
        .expect("a scene")
        .stats()
        .triangles;
    assert!(
        0 < shown && shown < full,
        "the space's body leaves the picture and the world's stays: {shown} of {full}"
    );
    assert!(
        matches!(
            session
                .perform(SessionOp::SetSpaceHidden {
                    space: VarId::new(0, test_utils::refusal::tagged(77)),
                    hidden: true,
                })
                .refusal,
            Some(Refusal::Display(DisplayFault::Admission(
                AdmissionFault::NotASpace { .. }
            )))
        ),
        "a variable that is no space refuses"
    );
}

/// A row of `count` copies of `body` in its own space over a fresh index
/// `name`, stepped clear along `x`: the document and the family.
fn row_of(
    doc: Doc<ProfileProgram>,
    body: RecipeNodeId,
    name: &str,
    count: i64,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let (doc, k) = crate::fixture::index(doc, name, count);
    crate::fixture::family_of(
        doc,
        body,
        crate::fixture::slides_along(k, [1.0, 0.0, 0.0], 2.0 * SIDE),
    )
}

/// A family over a fresh index `name` of `count` members, each reading
/// one member of `family` by `at(index)`, standing where it stands.
fn reads_by_index(
    doc: Doc<ProfileProgram>,
    family: RecipeNodeId,
    name: &str,
    count: i64,
    at: impl Fn(VarId) -> pncad::document::Formula,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let (doc, k) = crate::fixture::index(doc, name, count);
    let space = crate::fixture::space(&doc, family);
    crate::fixture::insert(
        doc,
        Node::place(
            pncad::document::BodyRead::indexed(family, vec![at(k)]),
            pncad::document::Operand::Var(space),
            pncad::document::stand().bundle(),
        ),
    )
}

/// The members of `node` the picture draws, as the index holds them.
fn members_drawn(index: &PickIndex, node: RecipeNodeId, count: u32) -> Vec<u32> {
    (0..count)
        .filter(|&member| !index.ids_in(node, member).is_empty())
        .collect()
}

/// **A read of members by an index takes the members its index ranges
/// over, and no other**: `ys[k] = xs[k]` over two takes members 0 and 1
/// of three, `ys[k] = xs[k + 1]` takes 1 and 2, and a family of one
/// reading `xs[j]` takes member 0; each time the members no read names
/// stay drawn.
///
/// *Breaks if* an indexed read whose member only an evaluation knows
/// takes the whole family (nothing of `xs` drawn), or the index is
/// evaluated once rather than per member of the reader.
#[test]
fn a_read_by_index_takes_the_members_its_index_names() {
    use pncad::document::{Dimension, Formula};
    let tol = Tol::witness();
    let count = |k| Formula::var(k, Dimension::Count);
    type At = Box<dyn Fn(VarId) -> Formula>;
    let cases: [(&str, i64, At, Vec<u32>); 3] = [
        ("same", 2, Box::new(count), vec![2]),
        (
            "next",
            2,
            Box::new(move |k| Formula::add(count(k), Formula::count(1)).expect("a count")),
            vec![0],
        ),
        ("one", 1, Box::new(count), vec![1, 2]),
    ];
    for (label, readers, at, left) in cases {
        let (doc, body) = extrude(
            &Doc::empty_derived(&format!("picture-index-{label}"), tol),
            0.0,
            tol,
        );
        let (doc, xs) = row_of(doc, body, "k", 3);
        let (doc, ys) = reads_by_index(doc, xs, "j", readers, at);
        let session = landed(doc, tol);
        let index = common::plate_index(&session);
        assert_eq!(
            members_drawn(&index, xs, 3),
            left,
            "{label}: xs less what ys reads"
        );
        assert_eq!(
            members_drawn(&index, ys, 3),
            (0..u32::try_from(readers).expect("a count")).collect::<Vec<_>>(),
            "{label}: every member of ys is drawn"
        );
    }
}

/// **A split read at one port takes that half alone**: a copy of the
/// lower half placed into the world leaves the upper half drawn in the
/// body's own space.
///
/// *Breaks if* a port read takes the whole split.
#[test]
fn a_split_half_read_alone_leaves_the_other_half_drawn() {
    use pncad::select::SplitHalf;
    let tol = Tol::witness();
    let (doc, body) = extrude(&Doc::empty_derived("picture-split", tol), 0.0, tol);
    let mut session = landed(doc, tol);
    let split = common::session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            origin: common::len3([0.0, 0.0, DEPTH / 2.0]),
            normal: common::scl3([0.0, 0.0, 1.0]),
        },
    );
    let (doc, _) = common::placed_read(
        session.committed_doc(),
        pncad::document::Operand::output(split, SplitHalf::Below.port()),
        tol,
    );
    let session = landed(doc, tol);
    let index = common::plate_index(&session);
    assert_eq!(
        members_drawn(&index, split, 2),
        vec![SplitHalf::Above.output_body()],
        "the half nothing reads stays drawn"
    );
}

/// **A piece two `⊔`s draw stays drawn until both are hidden**: the
/// product's `⊔` reads two copies, a second `⊔` the first copy again.
/// Hiding the product's `⊔` takes the second copy off and leaves the
/// first, which the other `⊔` still draws; hiding both takes it off.
///
/// *Breaks if* a drawing is keyed by the body alone, so one `⊔`'s hide
/// takes a piece another still draws.
#[test]
fn a_piece_two_unions_draw_stays_until_both_are_hidden() {
    use pncad::document::{Bodies, BodyRead};
    let tol = Tol::witness();
    let (doc, a) = extrude(&Doc::empty_derived("picture-shared-piece", tol), 0.0, tol);
    let (doc, b) = extrude(&doc, 2.0 * SIDE, tol);
    let (doc, a_copy) = common::placed(&doc, a, tol);
    let (doc, b_copy) = common::placed(&doc, b, tol);
    let product = doc
        .product()
        .and_then(|var| doc.operation_of(var))
        .expect("the product reads the world's copies");
    let (doc, other) = common::inserted(
        &doc,
        Node::Assemble {
            members: Bodies::Spelled(vec![BodyRead::plain(pncad::document::Operand::Node(
                a_copy,
            ))]),
        },
        tol,
    );
    let mut session = landed(doc, tol);
    let hide = |session: &mut DocSession, node| {
        let outcome = session.perform(SessionOp::SetBodyOn { node, on: false });
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    };
    hide(&mut session, product);
    let view = session.display_view();
    assert!(view.hides(b_copy, 0), "the product's own piece goes");
    assert!(
        !view.hides(a_copy, 0),
        "the other ⊔ still draws the shared piece"
    );
    hide(&mut session, other);
    assert!(
        session.display_view().hides(a_copy, 0),
        "with both hidden, the piece goes"
    );
}

/// **The product slot's body is always drawn**, whatever reads it: with
/// the product one copy that a union also reads, the copy and the union
/// are both drawn, coincident — a pick there is `Ambiguous`, which is a
/// truthful picture of two bodies in one place — and the other operand,
/// which only the union reads, is not drawn.
///
/// *Breaks if* the product's body is hidden by an operation reading it,
/// or the picture stops drawing the union beside it.
#[test]
fn the_products_body_is_drawn_whatever_reads_it() {
    use pncad::document::{Bodies, BodyRead};
    use pncad::select::HitTestError;
    let tol = Tol::witness();
    let (doc, a) = extrude(&Doc::empty_derived("picture-product-read", tol), 0.0, tol);
    let (doc, b) = extrude(&doc, SIDE / 2.0, tol);
    let (doc, a_copy) = common::placed(&doc, a, tol);
    let (doc, b_copy) = common::placed(&doc, b, tol);
    let (doc, union) = common::inserted(
        &doc,
        Node::Union {
            members: Bodies::Spelled(vec![
                BodyRead::plain(pncad::document::Operand::Node(a_copy)),
                BodyRead::plain(pncad::document::Operand::Node(b_copy)),
            ]),
        },
        tol,
    );
    let mut session = landed(doc, tol);
    let outcome = session.perform(SessionOp::SetProduct {
        bodies: vec![a_copy.into()],
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    assert_eq!(
        world::drawn_nodes(session.committed_doc())
            .into_iter()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([a_copy, union]),
        "the product's copy and the union; not the copy only the union reads"
    );
    let index = common::plate_index(&session);
    let (_, eval) = session.landed_pair().expect("landed");
    assert!(
        matches!(
            index.face_at(eval, &common::down_at(SIDE / 4.0, SIDE / 2.0)),
            Err(HitTestError::Ambiguous { .. })
        ),
        "the copy and the union coincide there"
    );
}

/// **The copy-to-body door**, as a gesture reads it:
/// - a row `P` of a world copy `W` is a copy of `W`: a pick on it seats
///   `W`, and `W`'s copies are `[P]`;
/// - a copy of member 1 of a family names member 1, and seats itself
///   (the variable it reads is the family);
/// - a copy of a body into that body's own space is no gesture's copy:
///   the body's copies are its world copy alone.
///
/// *Breaks if* families are excluded from the copies (the row seats
/// itself), the door drops a copy's member (it reads member 0), or a
/// copy into its body's own space is taken over with the world's.
#[test]
fn a_gesture_reads_a_copy_through_one_door() {
    let tol = Tol::witness();
    let (doc, body) = extrude(&Doc::empty_derived("picture-door", tol), 0.0, tol);
    let (doc, w) = common::placed(&doc, body, tol);
    let (doc, row) = row_of(doc, w, "k", 2);
    let w_out = doc.output(w, 0).expect("a copy");
    assert_eq!(world::seat_of(&doc, row), w, "a pick on the row seats W");
    assert_eq!(world::placed_var(&doc, row), Some(w_out));
    assert_eq!(world::placements_of_var(&doc, w_out), vec![row]);

    // A copy of member 1 of a family, placed into the world.
    let (doc, xs) = row_of(doc, body, "j", 3);
    let (doc, member) = common::inserted(
        &doc,
        Node::place(
            pncad::document::BodyRead::indexed(xs, vec![pncad::document::Formula::count(1)]),
            pncad::document::Operand::Var(VarId::WORLD),
            pncad::document::stand().bundle(),
        ),
        tol,
    );
    assert_eq!(world::body_of(&doc, member, 0), (xs, 1), "member 1, not 0");
    assert_eq!(world::seat_of(&doc, member), member);
    assert_eq!(world::placed_var(&doc, member), None);

    // A copy of the body into its own space.
    let own = doc.node_space(body).expect("its own space");
    let (doc, inside) = common::inserted(
        &doc,
        Node::place(
            pncad::document::BodyRead::plain(pncad::document::Operand::Node(body)),
            pncad::document::Operand::Var(own),
            pncad::document::stand().bundle(),
        ),
        tol,
    );
    let body_out = doc.output(body, 0).expect("a body");
    assert_eq!(
        world::placements_of_var(&doc, body_out),
        vec![w],
        "the copy into the body's own space is not a gesture's copy"
    );
    assert_eq!(world::seat_of(&doc, inside), inside);
}
