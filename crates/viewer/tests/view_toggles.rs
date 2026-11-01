//! **The View pane toggles spaces and bodies** (`crates/viewer/README.md`,
//! *The picture draws bodies*): the pane lists the world first, then
//! every space holding a body, each body under its space with its
//! toggle; the drawing rule decides only the default, so a body it
//! leaves out can be shown and any drawn one hidden; the feature tree's
//! and the Properties pane's toggles read the same state; and none of
//! it reaches the document (GUI-DESIGN G3).
//!
//! The index each row reads is the application's own: a [`PickCache`]
//! synced to [`DocSession::index_inputs`], which carries the author's
//! shows, so a show the cache does not rebuild for draws nothing here.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use pncad::document::{Doc, Node, ProfileProgram, RecipeNodeId, VarId};
use pncad::geom_core::Tol;
use viewer::display::{self, BodyToggle, DisplayView, SpaceToggles};
use viewer::pickcache::{CacheStep, PickCache};
use viewer::pickindex::PickIndex;
use viewer::session::{BooleanSpec, DocSession, SessionOp};

/// The box's side and depth: plate-scale, so `common::plate_delta`
/// draws it.
const SIDE: f64 = 0.02;
const DEPTH: f64 = 0.01;

/// A square extruded `DEPTH`, its profile offset by `x`, which nothing
/// places: the document and the extrude.
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

/// An inline session over `doc`, landed, and a cache indexing it.
fn landed(doc: Doc<ProfileProgram>, tol: Tol) -> (DocSession, PickCache) {
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let mut cache = PickCache::inline();
    sync(&session, &mut cache);
    (session, cache)
}

/// The cache brought to `session`'s picture, as the application's frame
/// does; what the sync did.
fn sync(session: &DocSession, cache: &mut PickCache) -> CacheStep {
    let step = cache.sync(session.index_inputs(), Some(common::plate_delta()));
    cache.pump();
    step
}

/// Perform `op`, which the session must accept.
fn perform(session: &mut DocSession, op: SessionOp) {
    let outcome = session.perform(op.clone());
    assert!(outcome.refusal.is_none(), "{op:?}: {:?}", outcome.refusal);
}

/// Turn `node`'s toggle `on` or off, as a click on its checkbox does.
fn toggle(session: &mut DocSession, node: RecipeNodeId, on: bool) {
    perform(session, SessionOp::SetBodyHidden { node, hidden: !on });
}

/// The View pane's list over `session`'s committed document.
fn toggles(session: &DocSession) -> Vec<SpaceToggles> {
    display::toggles(session.committed_doc(), &session.display_view())
}

/// `node`'s row in the View pane's list, and whether its space's toggle
/// is on.
fn row(session: &DocSession, node: RecipeNodeId) -> (BodyToggle, bool) {
    toggles(session)
        .into_iter()
        .find_map(|group| {
            group
                .bodies
                .iter()
                .find(|body| body.node == node)
                .map(|body| (*body, group.on))
        })
        .unwrap_or_else(|| panic!("{node:?} is listed"))
}

/// Whether the picture draws some body of `node` — itself, or for a
/// `⊔` a piece of it — under `view`, as the scene and the pick read it:
/// a part of the index that no hide drops and a shown drawing of `node`
/// covers.
fn visible(index: &PickIndex, view: &DisplayView, node: RecipeNodeId) -> bool {
    index.parts().iter().any(|part| {
        !view.hides(part.node(), part.body())
            && view
                .shown_drawn
                .iter()
                .any(|drawn| drawn.of_node(node) && drawn.draws(part.node(), part.body()))
    })
}

/// **Every toggle says what the picture does**: a body whose own toggle
/// and its space's are on is drawn, and no other is. What the feature
/// tree's and the Properties pane's checkboxes show is
/// [`DisplayView::body_on`], so this holds them to the picture too.
fn assert_toggles_agree(session: &DocSession, index: &PickIndex, at: &str) {
    let view = session.display_view();
    for group in toggles(session) {
        for body in group.bodies {
            assert_eq!(
                body.on,
                view.body_on(body.node),
                "{at}: the View pane and the tree read one toggle for {:?}",
                body.node
            );
            assert_eq!(
                body.on && group.on,
                visible(index, &view, body.node),
                "{at}: {:?}'s toggle ({}, its space's {}) says what is drawn",
                body.node,
                body.on,
                group.on
            );
        }
    }
}

/// The document as it would be saved.
fn saved(session: &DocSession, tol: Tol) -> String {
    pncad::document::save(session.committed_doc(), &[], tol).expect("the document saves")
}

/// Two extrudes, one placed into the world and one left in its own
/// space: the document, the world's copy and the unplaced body.
fn world_and_unplaced(tag: &str, tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let (doc, a) = extrude(&Doc::empty_derived(tag, tol), 0.0, tol);
    let (doc, b) = extrude(&doc, 2.0 * SIDE, tol);
    let (doc, copy) = common::placed(&doc, a, tol);
    (doc, copy, b)
}

/// **The world is listed first and its toggle hides it**: the copy in
/// the world leaves the picture, the unplaced body stays, and the copy's
/// own toggle stays on, so showing the world again draws it.
///
/// *Breaks if* the world is not the first group, its toggle does not
/// reach the bodies drawn in it, or a space's hide reaches a body of
/// another space.
#[test]
fn the_worlds_toggle_hides_the_world() {
    let tol = Tol::witness();
    let (doc, copy, unplaced) = world_and_unplaced("toggles-world", tol);
    let (mut session, mut cache) = landed(doc, tol);
    let groups = toggles(&session);
    assert_eq!(groups[0].space, Some(VarId::WORLD), "the world first");
    assert!(
        groups[0].bodies.iter().any(|body| body.node == copy),
        "the copy is listed under the world"
    );
    let index = cache.index().expect("indexed");
    assert_toggles_agree(&session, index, "at rest");

    perform(
        &mut session,
        SessionOp::SetSpaceHidden {
            space: VarId::WORLD,
            hidden: true,
        },
    );
    assert_eq!(sync(&session, &mut cache), CacheStep::Current);
    let index = cache.index().expect("indexed");
    let view = session.display_view();
    assert!(!visible(index, &view, copy), "the world's copy goes");
    assert!(visible(index, &view, unplaced), "the unplaced body stays");
    let (listed, world_on) = row(&session, copy);
    assert!(!world_on, "the world's toggle reads off");
    assert!(listed.on, "the copy's own toggle stays on");
    assert_toggles_agree(&session, index, "the world hidden");

    perform(
        &mut session,
        SessionOp::SetSpaceHidden {
            space: VarId::WORLD,
            hidden: false,
        },
    );
    assert!(
        visible(index, &session.display_view(), copy),
        "and it is back"
    );
}

/// **An unplaced body's space has its own toggle**, listed after the
/// world, which hides that body and not the world's.
///
/// *Breaks if* an unplaced body's space is not listed, or its toggle
/// hides nothing or the world too.
#[test]
fn a_spaces_toggle_hides_the_space() {
    let tol = Tol::witness();
    let (doc, copy, unplaced) = world_and_unplaced("toggles-space", tol);
    let own = doc.node_space(unplaced).expect("its own space");
    let (mut session, cache) = landed(doc, tol);
    let groups = toggles(&session);
    let at = groups
        .iter()
        .position(|group| group.space == Some(own))
        .expect("the unplaced body's space is listed");
    assert!(at > 0, "after the world");
    assert_eq!(
        groups[at].bodies.iter().map(|b| b.node).collect::<Vec<_>>(),
        vec![unplaced],
        "with its body under it"
    );
    perform(
        &mut session,
        SessionOp::SetSpaceHidden {
            space: own,
            hidden: true,
        },
    );
    let index = cache.index().expect("indexed");
    let view = session.display_view();
    assert!(!visible(index, &view, unplaced), "the space's body goes");
    assert!(visible(index, &view, copy), "the world's stays");
    assert_toggles_agree(&session, index, "the space hidden");
}

/// **An operand a boolean took over is listed off, and its toggle shows
/// it**: the union is drawn and its two operands are not; turning one
/// operand's toggle on draws it whole where it stands — the index is
/// rebuilt for it — and turning it off again leaves nothing held, so
/// the body follows the default.
///
/// *Breaks if* an undrawn body is unlisted or listed on, a show is not
/// part of the picture's key (the cache answers `Current` and the
/// index holds no part of the operand), or a show turned back holds a
/// show.
#[test]
fn an_operand_a_boolean_took_over_can_be_shown() {
    let tol = Tol::witness();
    let (doc, a) = extrude(&Doc::empty_derived("toggles-operand", tol), 0.0, tol);
    let (doc, b) = extrude(&doc, SIDE / 2.0, tol);
    let (mut session, mut cache) = landed(doc, tol);
    let union = common::session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Union(vec![a.into(), b.into()]),
        },
    );
    session.pump();
    sync(&session, &mut cache);
    let (operand, _) = row(&session, a);
    assert!(
        !operand.on && !operand.drawn_by_default,
        "the operand is listed, off, as one an operation reads: {operand:?}"
    );
    assert!(row(&session, union).0.drawn_by_default);
    assert_toggles_agree(&session, cache.index().expect("indexed"), "at rest");

    toggle(&mut session, a, true);
    assert!(session.display().shown_bodies().contains(&a));
    assert_eq!(
        sync(&session, &mut cache),
        CacheStep::Submitted,
        "a show is a new picture"
    );
    let index = cache.index().expect("indexed");
    assert!(
        !index.ids_of_node(a).is_empty(),
        "the index holds the operand"
    );
    assert!(visible(index, &session.display_view(), a), "and draws it");
    assert!(!visible(index, &session.display_view(), b), "not the other");
    assert!(
        visible(index, &session.display_view(), union),
        "the union stays"
    );
    assert_toggles_agree(&session, index, "the operand shown");

    toggle(&mut session, a, false);
    assert!(
        session.display().shown_bodies().is_empty() && session.display().hidden_bodies().is_empty(),
        "a toggle turned back to the default holds nothing"
    );
    sync(&session, &mut cache);
    let index = cache.index().expect("indexed");
    assert!(
        index.ids_of_node(a).is_empty(),
        "the operand leaves the index"
    );
    assert_toggles_agree(&session, index, "the operand off again");
}

/// **A drawn body's toggle hides it**, and turned back on holds nothing,
/// so a boolean that then takes the body over takes it out of the
/// picture as the default says.
///
/// *Breaks if* the hide does not drop the body, or a toggle turned back
/// on is kept as a show that draws the operand beside the boolean.
#[test]
fn a_drawn_bodys_toggle_hides_it() {
    let tol = Tol::witness();
    let (doc, a) = extrude(&Doc::empty_derived("toggles-drawn", tol), 0.0, tol);
    let (doc, b) = extrude(&doc, 2.0 * SIDE, tol);
    let (mut session, mut cache) = landed(doc, tol);
    toggle(&mut session, a, false);
    assert!(session.display().hidden_bodies().contains(&a));
    let index = cache.index().expect("indexed");
    assert!(!visible(index, &session.display_view(), a), "the body goes");
    assert!(
        visible(index, &session.display_view(), b),
        "its sibling stays"
    );
    assert!(!row(&session, a).0.on);
    assert_toggles_agree(&session, index, "the body hidden");

    toggle(&mut session, a, true);
    assert!(session.display().hidden_bodies().is_empty());
    assert!(session.display().shown_bodies().is_empty());
    let subtract = common::session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Subtract {
                from: b.into(),
                tool: a.into(),
            },
        },
    );
    session.pump();
    sync(&session, &mut cache);
    let index = cache.index().expect("indexed");
    assert!(
        !visible(index, &session.display_view(), a),
        "the boolean takes the body over"
    );
    assert!(visible(index, &session.display_view(), subtract));
    assert_toggles_agree(&session, index, "after the boolean");
}

/// **An instance a pattern reads is listed off, and the toggle the tree
/// and the Properties pane draw for it shows it**: the instance's row
/// reads off where it used to read "shown" over an undrawn body, and the
/// op the tree's checkbox pushes draws it.
///
/// *Breaks if* the tree's checkbox state is the hide set alone (it reads
/// on while nothing is drawn), or the op it pushes changes nothing.
#[test]
fn an_instance_a_pattern_reads_is_listed_off_and_shown_by_its_toggle() {
    let tol = Tol::witness();
    let (_bench, path, instance, space, family) =
        crate::picture_draws_bodies::family_in_an_instances_space("toggles", tol);
    let mut session = crate::picture_draws_bodies::opened(path, tol);
    let mut cache = PickCache::inline();
    let step = cache.sync(session.index_inputs(), Some(common::asm::delta()));
    assert_eq!(step, CacheStep::Submitted);
    cache.pump();
    let (listed, space_on) = row(&session, instance);
    assert!(space_on);
    assert!(
        !listed.on && !session.display_view().body_on(instance),
        "the instance the pattern reads is off in every pane"
    );
    assert_eq!(
        session.committed_doc().node_space(family),
        Some(space),
        "the premise: the family is in the instance's space"
    );
    toggle(&mut session, instance, true);
    assert_eq!(
        cache.sync(session.index_inputs(), Some(common::asm::delta())),
        CacheStep::Submitted
    );
    cache.pump();
    let index = cache.index().expect("indexed");
    assert!(visible(index, &session.display_view(), instance), "shown");
    assert!(visible(index, &session.display_view(), family), "beside it");
    assert!(row(&session, instance).0.on);
}

/// **None of it reaches the document**: every toggle above, turned, and
/// the document saves to the same bytes, with no history step taken.
///
/// *Breaks if* a display op writes the document or enters the history.
#[test]
fn no_toggle_reaches_the_document() {
    let tol = Tol::witness();
    let (doc, copy, unplaced) = world_and_unplaced("toggles-document", tol);
    let (doc, operand) = extrude(&doc, 4.0 * SIDE, tol);
    let (doc, _) = common::placed(&doc, operand, tol);
    let (mut session, _cache) = landed(doc, tol);
    let before = saved(&session, tol);
    let steps = session.history().len();
    let own = session
        .committed_doc()
        .node_space(unplaced)
        .expect("its own space");
    for space in [VarId::WORLD, own] {
        perform(
            &mut session,
            SessionOp::SetSpaceHidden {
                space,
                hidden: true,
            },
        );
    }
    toggle(&mut session, copy, false);
    toggle(&mut session, operand, true);
    assert!(
        !session.display().shown_bodies().is_empty()
            && !session.display().hidden_bodies().is_empty()
            && !session.display().hidden_spaces().is_empty(),
        "the premise: every kind of toggle state is held"
    );
    assert_eq!(saved(&session, tol), before, "the document is unchanged");
    assert_eq!(session.history().len(), steps, "and no step was taken");
}
