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
fn family_in_an_instances_space(
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
fn opened(path: std::path::PathBuf, tol: Tol) -> DocSession {
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
    let hidden = session.perform(SessionOp::SetBodyHidden {
        node: assemble,
        hidden: true,
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
