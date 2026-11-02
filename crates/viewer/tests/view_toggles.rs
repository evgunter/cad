//! **The View pane toggles spaces and bodies** (`crates/viewer/README.md`,
//! *The picture draws bodies*): the pane lists the world first, then
//! every space holding a body, each body under its space with its
//! toggle; the drawing rule decides only the default, so a body it
//! leaves out can be shown and any drawn one hidden, each click decided
//! against what the toggle reads now; and none of it reaches the
//! document (GUI-DESIGN G3). That the feature tree's and the Properties
//! pane's checkboxes read the same toggle is driven in
//! `app::properties_pane_tests`.
//!
//! The index each row reads is the application's own: a [`PickCache`]
//! synced to [`DocSession::index_inputs`], which carries the author's
//! shows, so a show the cache does not rebuild for draws nothing here.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use pncad::document::{Bodies, BodyRead, Doc, Node, Operand, ProfileProgram, RecipeNodeId, VarId};
use pncad::geom_core::Tol;
use viewer::display::{BodyToggle, DisplayView, Held, SpaceToggles};
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
    perform(session, SessionOp::SetBodyOn { node, on });
}

/// The View pane's list over `session`'s committed document.
fn toggles(session: &DocSession) -> Vec<SpaceToggles> {
    session.toggles().groups
}

/// Whether `node`'s toggle reads on.
fn on(session: &DocSession, node: RecipeNodeId) -> bool {
    row(session, node).0.on
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
/// and its space's are on is drawn, and no other is — read off the
/// toggles on one side and the index and the scene's view on the
/// other. A body whose value the index cannot draw (a refusal, on its
/// own badge) is not a toggle's to answer for.
fn assert_toggles_agree(session: &DocSession, index: &PickIndex, at: &str) {
    let view = session.display_view();
    let undrawn: Vec<RecipeNodeId> = index.undrawn().iter().map(|u| u.node()).collect();
    for group in toggles(session) {
        for body in group.bodies {
            if undrawn.contains(&body.node) {
                continue;
            }
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

    // On and off again before the cache looks: the picture is the
    // same one, so nothing is rebuilt.
    toggle(&mut session, a, true);
    toggle(&mut session, a, false);
    assert_eq!(
        sync(&session, &mut cache),
        CacheStep::Current,
        "the picture is keyed by what is shown"
    );

    toggle(&mut session, a, true);
    assert_eq!(
        session.display().toggled().get(&a),
        Some(&true),
        "a show is held"
    );
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
        session.display().toggled().is_empty(),
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
    let index = cache.index().expect("indexed");
    assert!(!visible(index, &session.display_view(), a), "the body goes");
    assert!(
        visible(index, &session.display_view(), b),
        "its sibling stays"
    );
    assert!(!on(&session, a));
    assert_toggles_agree(&session, index, "the body hidden");

    toggle(&mut session, a, true);
    assert!(
        session.display().toggled().is_empty(),
        "a toggle turned back on holds nothing"
    );
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

/// **An instance a pattern reads is listed off, and its toggle shows
/// it** beside the family.
///
/// *Breaks if* the instance reads on while nothing draws it, or the op
/// its checkbox pushes changes nothing.
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
    assert!(!listed.on, "the instance the pattern reads is listed off");
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
    assert!(on(&session, instance));
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
    let held = session.display().toggled();
    assert!(
        held.values().any(|&on| on)
            && held.values().any(|&on| !on)
            && !session.display().hidden_spaces().is_empty(),
        "the premise: a show, a hide and a space's hide are held: {held:?}"
    );
    assert_eq!(saved(&session, tol), before, "the document is unchanged");
    assert_eq!(session.history().len(), steps, "and no step was taken");
}

/// **A piece of a shown `⊔` reads on, and its click hides it**: a `⊔` a
/// placement reads is left out, and so are its pieces; showing the `⊔`
/// draws both pieces, each reading on, and a click on one takes that one
/// off and leaves the other and the `⊔` on.
///
/// *Breaks if* the click is decided against the default picture, which
/// leaves the piece out either way, so it holds nothing and the piece
/// stays drawn.
#[test]
fn a_piece_of_a_shown_union_is_hidden_by_its_toggle() {
    let tol = Tol::witness();
    let (doc, a) = extrude(&Doc::empty_derived("toggles-shown-union", tol), 0.0, tol);
    let (doc, b) = extrude(&doc, 2.0 * SIDE, tol);
    // The `⊔` reads its pieces in one space: `b`, placed into `a`'s.
    let space = doc.node_space(a).expect("its own space");
    let (doc, b_beside) = common::inserted(
        &doc,
        Node::place(
            BodyRead::plain(Operand::from(b)),
            Operand::Var(space),
            pncad::document::stand().bundle(),
        ),
        tol,
    );
    let (doc, union) = common::inserted(
        &doc,
        Node::Assemble {
            members: Bodies::Spelled(vec![
                BodyRead::plain(Operand::Node(a)),
                BodyRead::plain(Operand::Node(b_beside)),
            ]),
        },
        tol,
    );
    let (doc, _) = common::placed(&doc, union, tol);
    let (mut session, mut cache) = landed(doc, tol);
    assert!(
        !on(&session, union) && !on(&session, a) && !on(&session, b_beside),
        "the premise: the placement reads the ⊔, so it and its pieces are off"
    );
    toggle(&mut session, union, true);
    assert!(
        on(&session, a) && on(&session, b_beside),
        "showing the ⊔ draws its pieces"
    );
    sync(&session, &mut cache);
    assert_toggles_agree(&session, cache.index().expect("indexed"), "the ⊔ shown");

    toggle(&mut session, a, false);
    sync(&session, &mut cache);
    let index = cache.index().expect("indexed");
    let view = session.display_view();
    assert!(!on(&session, a), "the piece reads off");
    assert!(!visible(index, &view, a), "and is not drawn");
    assert!(visible(index, &view, b_beside), "the other piece stays");
    assert!(on(&session, union), "and so does the ⊔");
    assert_toggles_agree(&session, index, "a piece of the shown ⊔ hidden");
}

/// **A piece of a hidden `⊔` reads off, and its click shows it** on its
/// own, drawn once though two drawings now cover it.
///
/// *Breaks if* the piece reads on under its hidden `⊔`, its click holds
/// nothing, it is built twice, or showing it brings the other piece or
/// the `⊔` back.
#[test]
fn a_piece_of_a_hidden_union_is_shown_by_its_toggle() {
    let tol = Tol::witness();
    let (doc, a) = extrude(&Doc::empty_derived("toggles-hidden-union", tol), 0.0, tol);
    let (doc, b) = extrude(&doc, 2.0 * SIDE, tol);
    let (doc, a_copy) = common::placed(&doc, a, tol);
    let (doc, b_copy) = common::placed(&doc, b, tol);
    let product = doc
        .product()
        .and_then(|var| doc.operation_of(var))
        .expect("the product reads the world's copies");
    let (mut session, mut cache) = landed(doc, tol);
    toggle(&mut session, product, false);
    assert!(
        !on(&session, a_copy) && !on(&session, b_copy),
        "the ⊔'s pieces read off"
    );
    toggle(&mut session, a_copy, true);
    sync(&session, &mut cache);
    let index = cache.index().expect("indexed");
    let view = session.display_view();
    assert!(
        on(&session, a_copy) && visible(index, &view, a_copy),
        "the piece is drawn"
    );
    assert!(!visible(index, &view, b_copy), "the other is not");
    assert!(!on(&session, product), "the ⊔ stays off");
    assert_eq!(
        index
            .parts()
            .iter()
            .filter(|part| part.node() == a_copy)
            .count(),
        1,
        "the piece is built once"
    );
    assert_toggles_agree(&session, index, "a piece of the hidden ⊔ shown");
}

/// **A body hidden earlier that an operation then takes over reads off,
/// and its toggle shows it**: the hide is held over a body the default
/// no longer draws, and a click on decides against that.
///
/// *Breaks if* the toggle reads on (the hide is ignored as moot) or the
/// click holds nothing and the body stays out of the picture.
#[test]
fn a_body_hidden_then_taken_over_is_shown_by_its_toggle() {
    let tol = Tol::witness();
    let (doc, a) = extrude(&Doc::empty_derived("toggles-hidden-taken", tol), 0.0, tol);
    let (doc, b) = extrude(&doc, 2.0 * SIDE, tol);
    let (mut session, mut cache) = landed(doc, tol);
    toggle(&mut session, a, false);
    common::session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Subtract {
                from: b.into(),
                tool: a.into(),
            },
        },
    );
    session.pump();
    assert!(!on(&session, a), "the operand reads off");
    toggle(&mut session, a, true);
    sync(&session, &mut cache);
    let index = cache.index().expect("indexed");
    assert!(
        visible(index, &session.display_view(), a),
        "the click draws it"
    );
    assert_toggles_agree(&session, index, "the hidden operand shown");
}

/// **A show the document can no longer hold is dropped, and said**: a
/// show of the placement a boolean wrote goes with the undo of the
/// boolean, on the undo's own report as a dropped show.
///
/// *Breaks if* the show goes in silence, is reported as a hide, or
/// outlives its node.
#[test]
fn a_dropped_show_is_reported() {
    let tol = Tol::witness();
    let (doc, a) = extrude(&Doc::empty_derived("toggles-dropped-show", tol), 0.0, tol);
    let (doc, b) = extrude(&doc, SIDE / 2.0, tol);
    let (mut session, _cache) = landed(doc, tol);
    let outcome = session.perform(SessionOp::AddBoolean {
        spec: BooleanSpec::Union(vec![a.into(), b.into()]),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let made = outcome.made.expect("the union");
    let placement = *outcome
        .minted
        .iter()
        .find(|&&node| node != made && session.committed_doc().placements().contains(&node))
        .expect("the boolean placed its second operand into the first's space");
    toggle(&mut session, placement, true);
    let undone = session.perform(SessionOp::Undo);
    assert!(undone.refusal.is_none(), "{:?}", undone.refusal);
    let dropped: Vec<Held> = undone
        .withdrawn
        .dropped_shows
        .iter()
        .map(|withdrawn| withdrawn.held)
        .collect();
    assert_eq!(dropped, vec![Held::Node(placement)], "the show is reported");
    assert!(
        undone.withdrawn.dropped_hides.is_empty(),
        "and not as a hide"
    );
    assert!(session.display().toggled().is_empty(), "and is gone");
}
