//! **An assembly story: the windmill.** A user authors three small
//! parts — a slim tower, a hub, a long sail blade — into one
//! directory, then builds a windmill out of them entirely through the
//! session's typed op vocabulary: instances through the workspace door
//! (with its typed refusals), hide and the free-move probe as display
//! state, mates through the mate tool's two-pick flow (a plane rest and
//! its values; the second sail spun a quarter turn, so the blades land
//! crossed), camera framing and a real ray pick feeding the selection,
//! undo/redo across the assembly edits — the tree-shaped history
//! included — and a save/reopen that resolves the whole workspace
//! again.
//!
//! The at-rest badge tells the truth twice: the mated base CERTIFIES
//! (its one contact is a declared, nested rest), and the finished
//! windmill CERTIFIES TOO — the sails overhang the hub's walls, the
//! census finds the crossings, and the crossing rung answers them
//! from the mates the user already declared: each crossing point lies
//! in its declared pair's verified overlap region with material on
//! opposite sides of the shared carrier (the legal overhang; the
//! rung's same-side and undecided verdicts still refuse). Both
//! verdicts are asserted as the values they are.
//!
//! One test, deliberately, in the exit walk's shape: the story reads
//! top-to-bottom as a session a real user could have had, each stage a
//! numbered block whose assertions say what the stage claims.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use std::path::Path;

use common::asm;
use common::{body_volume, near};
use pncad::document::{Doc, DocumentId, Frame, RecipeNodeId};
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::select::{Resolution, RunCtx, face_frame};
use viewer::camera::{self, Camera, CameraOp};
use viewer::display::{AdmissionFault, DisplayFault};
use viewer::input::ViewportSize;
use viewer::matetool::{MateTool, MateToolState};
use viewer::session::{
    AtRestBadge, DocSession, FaceSelection, Hovered, Refusal, Selection, SessionOp,
};
use viewer::tree::RowStatus;

/// The tower: a slim square post, section × height, metres — slim so
/// the hub seated on it OVERHANGS it and the sails clear its walls.
const TOWER_SIDE: f64 = 0.02;
const TOWER_HEIGHT: f64 = 0.10;
/// The hub: a cube seated over the tower's top.
const HUB_SIDE: f64 = 0.024;
/// The sail: a long thin blade (length × width × thickness).
const SAIL_LENGTH: f64 = 0.09;
const SAIL_WIDTH: f64 = 0.02;
const SAIL_THICKNESS: f64 = 0.004;
/// Where the free-move probe parks the hub while the user lines up the
/// mate picks — clear of everything else drawn.
const HUB_PARK: [f64; 3] = [0.08, 0.05, 0.04];
/// Where the two sails are parked before their mates.
const SAIL_A_PARK: [f64; 3] = [0.12, 0.0, 0.0];
const SAIL_B_PARK: [f64; 3] = [-0.12, 0.02, 0.0];

/// **A rest of two centred boxes' faces, with its values**: the picked
/// faces' planes, the target flipped (the faces rest against each
/// other), the copy slid along the two target axes `slides` names, as
/// the snap's orientation charts them, and spun `spin` about the target
/// face's own normal. Every part here is a box centred on its own origin, so a
/// slide of zero centres the copy on its target's own axis.
fn rest_choice(
    slides: [(pncad::document::TargetAxis, f64); 2],
    spin: f64,
) -> viewer::matetool::MateChoice {
    use pncad::document::Coord;
    viewer::matetool::MateChoice {
        kind: pncad::document::VarKind::Plane,
        flip: true,
        values: vec![
            (Coord::Slide(slides[0].0), common::len(slides[0].1)),
            (Coord::Slide(slides[1].0), common::len(slides[1].1)),
            (Coord::SPIN, common::ang(spin)),
        ],
    }
}

/// **A sail's rest on one of the hub's walls**: the sail's underside
/// on the wall, its middle at the wall's middle. The snap turns the
/// parked sail's underside, its `−z`, the least way onto the mated
/// direction, the wall's normal reversed, the world's `±y` (front,
/// back), and writes that quarter turn about the world's `x` as the
/// mate's orientation: the copy's mated direction is then the mated
/// one, so the chart is the world's frame — the space the hub stands
/// in — and the wall leaves the slides along its `x` and `z`. The
/// sail's origin is its underside's middle and the wall's middle is
/// `HUB_SIDE / 2` up the hub, which sits on the tower's top, so the
/// slide along `z` is `TOWER_HEIGHT + HUB_SIDE / 2` on either wall;
/// `spin` turns the blade about the wall's own normal from the world's
/// `x`.
fn sail_choice(spin: f64) -> viewer::matetool::MateChoice {
    use pncad::document::TargetAxis;
    let up = TOWER_HEIGHT + HUB_SIDE / 2.0;
    rest_choice([(TargetAxis::X, 0.0), (TargetAxis::Z, up)], spin)
}

/// A session over a throwaway document — the story starts from
/// "whatever was open".
fn boot(tol: Tol) -> DocSession {
    DocSession::inline(Doc::empty_derived("story-windmill-boot", tol), tol)
}

/// Open `path` in a fresh session through the typed door, landed.
fn open_at(path: &Path, tol: Tol) -> DocSession {
    let mut session = boot(tol);
    let outcome = session.perform(SessionOp::Open(path.to_path_buf()));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    session
}

/// Author one box part through the creation ops — a new document, one
/// centred rectangle, one extrude, the box placed into the world by hand
/// and its copy the product — assert its volume, and save it. Answers
/// the part's document id.
fn author_box_part(
    session: &mut DocSession,
    name: &str,
    file: &Path,
    [width, height, depth]: [f64; 3],
    tol: Tol,
) -> DocumentId {
    let outcome = session.perform(SessionOp::NewDocument {
        name: name.to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let extrude = common::xy_box_in(session, [width, height, depth]);
    assert!(
        near(body_volume(session, extrude, tol), width * height * depth),
        "the {name} part's volume is its box"
    );
    common::reopened_with(
        session,
        file,
        |doc| {
            let (doc, copy) = common::world_copy(doc, extrude, tol);
            common::edited(
                &doc,
                pncad::document::DocEdit::SetProduct {
                    product: Some(pncad::document::Operand::Node(copy)),
                },
                tol,
            )
            .0
        },
        tol,
    );
    let saved = session.perform(SessionOp::Save(file.to_path_buf()));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
    session.committed_doc().id()
}

/// Componentwise closeness at the solve's tolerance.
fn close3(got: Vec3<f64>, want: Vec3<f64>, what: &str) {
    assert!(
        (got.x - want.x).abs() < 1e-9
            && (got.y - want.y).abs() < 1e-9
            && (got.z - want.z).abs() < 1e-9,
        "{what}: {got:?} vs {want:?}"
    );
}

/// **The frames the session's landings superseded** since the last
/// read, drained: a frame is asked of the document at the edit, so a
/// landing has none to add.
fn landed_superseded(session: &mut DocSession) -> Vec<viewer::display::Withdrawn> {
    session
        .take_landed_withdrawals()
        .into_iter()
        .flat_map(|report| report.superseded)
        .collect()
}

/// Drive one free-move gesture to its committed display value,
/// asserting it is display state and never a document edit.
fn park(session: &mut DocSession, instance: RecipeNodeId, at: [f64; 3]) {
    let history_len = session.history().len();
    let before = session.doc().node(instance).cloned();
    for op in [
        SessionOp::BeginFreeMove { node: instance },
        SessionOp::PreviewFreeMove {
            node: instance,
            frame: Frame::translation(at),
        },
        SessionOp::CommitFreeMove { node: instance },
    ] {
        let outcome = session.perform(op);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        assert!(outcome.committed.is_empty(), "a probe commits no edit");
    }
    assert!(crate::common::asm::frame_of(session, instance).is_some());
    assert_eq!(
        session.history().len(),
        history_len,
        "the probe leaves no history state"
    );
    assert_eq!(
        session.doc().node(instance).cloned(),
        before,
        "the probe leaves the instance's offset where it stood"
    );
}

#[test]
fn the_windmill_story() {
    let tol = Tol::witness();
    let dir = common::tempdir("story-windmill");

    // ── 1. THREE SMALL PARTS, one session, the creation doors:
    // NewDocument → one centred rectangle → one extrude → save into
    // the shared directory. Each part's evaluated volume is its box.
    let mut session = boot(tol);
    let tower_file = dir.join("story-windmill-tower.pncad");
    let hub_file = dir.join("story-windmill-hub.pncad");
    let sail_file = dir.join("story-windmill-sail.pncad");
    let tower_id = author_box_part(
        &mut session,
        "story-windmill-tower",
        &tower_file,
        [TOWER_SIDE, TOWER_SIDE, TOWER_HEIGHT],
        tol,
    );
    let hub_id = author_box_part(
        &mut session,
        "story-windmill-hub",
        &hub_file,
        [HUB_SIDE, HUB_SIDE, HUB_SIDE],
        tol,
    );
    let sail_id = author_box_part(
        &mut session,
        "story-windmill-sail",
        &sail_file,
        [SAIL_LENGTH, SAIL_WIDTH, SAIL_THICKNESS],
        tol,
    );

    // ── 2. THE ASSEMBLY DOCUMENT. Before its first save there is no
    // directory, so the workspace doors refuse typed rather than
    // guessing a store; after the save, the catalogue lists the three
    // parts beside the open document's own marked entry.
    let outcome = session.perform(SessionOp::NewDocument {
        name: "story-windmill".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(
        matches!(session.part_catalogue(), Err(Refusal::NoDocumentDirectory)),
        "an unsaved assembly has no catalogue"
    );
    let refused = session.perform(SessionOp::AddInstance { id: tower_id });
    assert!(refused.committed.is_empty());
    assert!(
        matches!(refused.refusal, Some(Refusal::NoDocumentDirectory)),
        "{:?}",
        refused.refusal
    );
    let asm_path = dir.join("story-windmill.pncad");
    let saved = session.perform(SessionOp::Save(asm_path.clone()));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
    assert_eq!(
        session.resolve_dir().expect("the save wired the resolver"),
        dir,
        "references resolve against the saved file's own directory"
    );
    let entries = session.part_catalogue().expect("the directory scans");
    for id in [tower_id, hub_id, sail_id] {
        assert!(
            entries.iter().any(|entry| entry.id == id),
            "part {id} is on offer"
        );
    }
    let own = session.committed_doc().id();
    assert_eq!(
        entries
            .iter()
            .filter(|entry| entry.open_document)
            .map(|entry| entry.id)
            .collect::<Vec<_>>(),
        vec![own],
        "the open document's own entry is marked, once"
    );

    // ── 3. THE BASE'S INSTANCES, with a slip and the tree-shaped
    // recovery. The user adds the tower, double-adds it by accident,
    // undoes — and the next add (the hub) mints a SIBLING in the
    // history: nothing destroyed, the abandoned two-tower state intact
    // on its own branch.
    let tower_i = common::instance_in(&mut session, tower_id);
    let _slip = common::instance_in(&mut session, tower_id);
    assert_eq!(session.history().len(), 3, "root plus two adds");
    let undone = session.perform(SessionOp::Undo);
    assert!(undone.refusal.is_none(), "{:?}", undone.refusal);
    assert_eq!(
        session.doc().ids().len(),
        2,
        "the slip is off the path: the tower and its own space"
    );
    let abandoned = session
        .history()
        .entry(session.history().current())
        .active_child()
        .expect("undo remembers the branch it left");
    let hub_i = common::instance_in(&mut session, hub_id);
    let history = session.history();
    assert_eq!(history.len(), 4, "the sibling is minted, nothing dropped");
    let parent = history
        .entry(history.current())
        .parent()
        .expect("the sibling has a parent");
    assert_eq!(
        history.entry(parent).children().len(),
        2,
        "the branch point holds both children"
    );
    assert!(
        history.entry(parent).children().contains(&abandoned),
        "the slip's branch is still a child"
    );
    assert_eq!(
        history.entry(abandoned).doc().len(),
        4,
        "the abandoned two-tower document is intact, each tower in its own space"
    );
    // Backwards and forwards across the add: redo follows the branch
    // the cursor is on — the hub, not the abandoned second tower.
    session.perform(SessionOp::Undo);
    assert_eq!(session.doc().ids().len(), 2);
    session.perform(SessionOp::Redo);
    assert!(
        session.doc().node(hub_i).is_some(),
        "redo returns to the hub's branch"
    );

    // A document refuses to instantiate ITSELF, typed, at the door.
    let selfie = session.perform(SessionOp::AddInstance { id: own });
    assert!(selfie.committed.is_empty(), "nothing was committed");
    match selfie.refusal {
        Some(Refusal::SelfInstance { id }) => assert_eq!(id, own),
        other => panic!("expected the self-instance refusal, got {other:?}"),
    }

    // ── 4. RESOLVE: two instance rows, both ok. The tower is then
    // placed into the world by hand — it stands where the product is
    // made, and an inserted part lands in a space of its own.
    session.pump();
    common::reopened_with(
        &mut session,
        &asm_path,
        |doc| common::into_world(doc, tower_i, tol),
        tol,
    );
    let rows = common::asm::instance_rows(&session);
    assert_eq!(rows.len(), 2);
    for row in &rows {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }

    // ── 5. HIDE the hub: the picture drops it, the document keeps
    // it — display state, never an edit, never history.
    let index = asm::index_of(&session);
    let full_triangles = index
        .scene_for(&session.display_view())
        .expect("a scene")
        .stats()
        .triangles;
    let history_len = session.history().len();
    let hidden = session.perform(SessionOp::SetBodyOn {
        node: hub_i,
        on: false,
    });
    assert!(hidden.refusal.is_none(), "{:?}", hidden.refusal);
    assert!(hidden.committed.is_empty(), "hide commits no edit");
    assert!(!session.toggles().body_on(hub_i));
    assert!(
        index
            .scene_for(&session.display_view())
            .expect("a scene")
            .stats()
            .triangles
            < full_triangles,
        "the picture drops the hidden hub"
    );
    assert_eq!(session.history().len(), history_len, "no history state");
    assert!(session.doc().node(hub_i).is_some(), "the document keeps it");
    assert!(
        session.tree_rows().iter().any(|row| row.id == hub_i),
        "so does the tree"
    );
    let shown = session.perform(SessionOp::SetBodyOn {
        node: hub_i,
        on: true,
    });
    assert!(shown.refusal.is_none(), "{:?}", shown.refusal);
    assert!(session.display().toggled().is_empty());

    // ── 6. THE CAMERA AND A REAL PICK. Frame the scene off its own
    // drawn bounds, orbit, re-frame (a fit is not a reset), then aim
    // the cursor at the tower's top-face centre — project, un-project,
    // ray-pick — and make the hit the session's selection.
    let viewport = ViewportSize {
        width_px: 1280.0,
        height_px: 720.0,
    };
    let aspect = viewport.aspect().expect("a positive aspect");
    let bounds = index
        .scene_for(&session.display_view())
        .expect("a scene")
        .bounds()
        .expect("the windmill draws");
    let framed = Camera::framing(Some(bounds), aspect).expect("the windmill frames");
    let turned = camera::apply(
        &framed,
        &CameraOp::Orbit {
            yaw: 0.4,
            pitch: 0.1,
        },
    )
    .expect("a finite orbit");
    let camera = camera::apply(
        &turned,
        &CameraOp::Frame {
            extent: Some(bounds),
            aspect,
        },
    )
    .expect("the windmill re-frames");
    assert_eq!(camera.yaw(), turned.yaw(), "a fit keeps the orientation");
    for corner in common::corners(&bounds) {
        let ndc = camera
            .project(corner, aspect)
            .expect("a finite aspect projects")
            .expect("a framed corner is in front of the eye");
        assert!(
            ndc[0].abs() <= 1.0 && ndc[1].abs() <= 1.0,
            "corner {corner:?} projects outside the viewport at {ndc:?}"
        );
    }
    let target = Point3::new(0.0, 0.0, TOWER_HEIGHT);
    let ndc = camera
        .project(target, aspect)
        .expect("projects")
        .expect("the tower top is in front of the eye");
    let cursor = viewport
        .cursor_of([ndc[0], ndc[1]])
        .expect("a positive area");
    let ray = camera
        .ray_through(cursor, viewport)
        .expect("the cursor un-projects");
    let (doc, eval) = session.landed_pair().expect("landed");
    let hit = index
        .pick_for(eval, &ray, &session.display_view())
        .expect("the pick answers")
        .expect("the cursor is aimed at the tower's top");
    assert_eq!(
        hit.node,
        common::copy_of(session.committed_doc(), tower_i),
        "the highest surface is the tower's copy"
    );
    assert!(
        (hit.point.z - TOWER_HEIGHT).abs() < 1e-9,
        "the hit is the top face, at {:?}",
        hit.point
    );
    assert!(
        matches!(
            pncad::select::resolve(
                RunCtx { doc, eval },
                doc.output(hit.node, 0).expect("the tower's copy"),
                &hit.name
            ),
            Ok(Resolution::Resolved(_))
        ),
        "a just-picked name resolves in the copy it was picked on, in the run it was picked from"
    );
    let picked = FaceSelection {
        name: hit.name.clone(),
        node: hit.node,
        body: hit.body,
    };
    session.perform(SessionOp::Select(Selection::Face(picked.clone())));
    assert_eq!(session.selection().face(), Some(&picked));
    assert!(session.standing().live(), "the selection resolves");
    session.perform(SessionOp::Hover(Some(Hovered::Face(picked.clone()))));
    assert_eq!(
        session.hover().map(Hovered::name),
        Some(&picked.name),
        "hover is its own transient value"
    );
    session.perform(SessionOp::Hover(None));
    assert!(session.hover().is_none(), "leaving clears only the hover");
    assert_eq!(session.selection().face(), Some(&picked));

    // ── 7. THE FREE-MOVE PROBE. A free-move moves a shown space other
    // than the world (DI5), so the hub is given a space of its own
    // first: at the identity there, it is drawn where it stood. A cancelled probe on the
    // hub leaves no trace at all; then the user parks the hub clear of
    // the tower (a committed display value, never a document edit).
    let hub_space = asm::unpin(&mut session, hub_i);
    for op in [
        SessionOp::BeginFreeMove { node: hub_i },
        SessionOp::PreviewFreeMove {
            node: hub_i,
            frame: Frame::translation([0.0, 0.0, 0.2]),
        },
        SessionOp::CancelFreeMove,
    ] {
        let outcome = session.perform(op);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        assert!(outcome.committed.is_empty());
    }
    assert!(
        crate::common::asm::frame_of(&session, hub_i).is_none(),
        "a cancelled probe leaves no trace"
    );
    park(&mut session, hub_i, HUB_PARK);

    // ── 8. THE SEAT MATE, through the tool's two-pick flow: the hub's
    // underside (picked where it is DRAWN — the parked spot), then the
    // tower's top, a rest with every value zero (both boxes centred).
    // One committed action — the hub re-targeted into the tower's
    // space, the world, and its bundle written: the mate and its three
    // values. The hub's own space is then empty, so the edit discards
    // the hub's probe — superseded, not zeroed.
    let hub_bottom =
        common::displayed_face_at(&session, &index, &asm::up_at(HUB_PARK[0], HUB_PARK[1]));
    assert_eq!(
        hub_bottom.node,
        common::copy_of(session.committed_doc(), hub_i),
        "the parked hub's copy is picked"
    );
    let tower_top = common::displayed_face_at(&session, &index, &asm::down_at(0.0, 0.0));
    assert_eq!(
        tower_top.node,
        common::copy_of(session.committed_doc(), tower_i)
    );
    let mut tool = MateTool::new();
    tool.pick(session.doc(), hub_bottom.clone());
    tool.pick(session.doc(), tower_top);
    assert!(matches!(tool.state(), MateToolState::Two { .. }));
    let seat_proposal = {
        let (doc, _) = session.landed_pair().expect("landed");
        tool.proposal(
            doc,
            rest_choice(
                [
                    (pncad::document::TargetAxis::X, 0.0),
                    (pncad::document::TargetAxis::Y, 0.0),
                ],
                0.0,
            ),
        )
        .expect("the seat proposes")
    };
    let outcome = session.perform(seat_proposal.op_into_target_space());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        outcome.committed.len(),
        3,
        "the re-target, the mate and its values: {:?}",
        outcome.committed
    );
    let [superseded] = &outcome.withdrawn.superseded[..] else {
        panic!(
            "exactly one frame is superseded at the edit: {:?}",
            outcome.withdrawn.superseded
        )
    };
    assert_eq!(
        superseded.held,
        viewer::display::Held::Node(hub_i),
        "the mate's edit discards the park"
    );
    session.pump();
    assert!(landed_superseded(&mut session).is_empty());
    assert!(
        matches!(
            &superseded.cause,
            AdmissionFault::NotShown { space, .. } if space.id() == hub_space
        ),
        "and the report carries WHY it went, not only which went — the \
         fault's own PAYLOAD, which is what would go red if the prune paired \
         the right fault with the wrong instance: {}",
        superseded.cause
    );
    assert!(crate::common::asm::frame_of(&session, hub_i).is_none());
    let seat_mate = *common::mates_in(session.committed_doc())
        .last()
        .expect("the mate landed");

    // The placement is SOLVED: the hub's underside centre sits on the
    // tower's top centre — a known world point, the tower being
    // identity-placed — and the faces' normals meet opposed. The base,
    // once the author says it is what the document makes, CERTIFIES at
    // rest: one declared contact, nested, answered.
    common::product_is(&mut session, &[tower_i, hub_i]);
    {
        let (_, eval) = session.landed_pair().expect("landed");
        let pose = face_frame(eval, hub_bottom.node, &hub_bottom.name)
            .expect("the seated hub's underside has a pose");
        close3(
            pose.origin - Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, TOWER_HEIGHT),
            "the hub seats on the tower's top centre",
        );
        close3(
            pose.axis,
            Vec3::new(0.0, 0.0, -1.0),
            "the hub's underside normal opposes the tower top's",
        );
    }
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    assert!(
        common::row_of(&session.tree_rows(), hub_i)
            .constraints
            .iter()
            .any(|child| child.mate == seat_mate),
        "the mate is a child of the hub's row"
    );
    assert_eq!(
        session.at_rest(),
        Some(&AtRestBadge {
            loud: 0,
            quiet: 0,
            could_not_look: 0,
        }),
        "the seated base certifies, its seat recorded at rest"
    );

    // The seated hub lives in the world's space, so it refuses the
    // probe, typed.
    let refused = session.perform(SessionOp::BeginFreeMove { node: hub_i });
    match refused.refusal {
        Some(Refusal::Display(DisplayFault::Admission(AdmissionFault::InTheWorld {
            node: instance,
        }))) => assert_eq!(instance.id(), hub_i),
        other => panic!("expected the in-the-world refusal, got {other:?}"),
    }

    // ── 9. UNDO AND REDO ACROSS THE MATE: the edit steps out and back
    // in through the same doors as any other — behind the product the
    // author set after it, which steps out first and back in last.
    session.perform(SessionOp::Undo);
    session.perform(SessionOp::Undo);
    session.pump();
    assert!(
        !common::mates_in(session.doc()).contains(&seat_mate),
        "undo removes the mate"
    );
    assert_eq!(session.tree_rows().len(), 2, "the two instances again");
    session.perform(SessionOp::Redo);
    session.perform(SessionOp::Redo);
    session.pump();
    assert!(
        common::mates_in(session.doc()).contains(&seat_mate),
        "redo restores the mate"
    );
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    assert_eq!(
        session.at_rest(),
        Some(&AtRestBadge {
            loud: 0,
            quiet: 0,
            could_not_look: 0,
        }),
        "redo re-certifies what undo took away"
    );

    // ── 10. THE SAILS: two more instances of one part, parked clear,
    // then rested on the hub's opposite walls through the tool, each
    // centred on its wall. The first lies along the hub's `x`; the
    // second is spun a quarter turn about its wall's normal, so the
    // blade lands SQUARE to the first — measured against the first
    // blade's solved direction, not assumed from the walls' charts.
    let sail_a = common::instance_in(&mut session, sail_id);
    let sail_b = common::instance_in(&mut session, sail_id);
    // Each in a space of its own, so each parks; at the identity there,
    // each is drawn where it stood.
    let sail_a_space = asm::unpin(&mut session, sail_a);
    let sail_b_space = asm::unpin(&mut session, sail_b);
    park(&mut session, sail_a, SAIL_A_PARK);
    park(&mut session, sail_b, SAIL_B_PARK);
    let index = asm::index_of(&session);
    let sail_a_bottom = common::displayed_face_at(
        &session,
        &index,
        &asm::up_at(SAIL_A_PARK[0], SAIL_A_PARK[1]),
    );
    assert_eq!(
        sail_a_bottom.node,
        common::copy_of(session.committed_doc(), sail_a)
    );
    let sail_b_bottom = common::displayed_face_at(
        &session,
        &index,
        &asm::up_at(SAIL_B_PARK[0], SAIL_B_PARK[1]),
    );
    assert_eq!(
        sail_b_bottom.node,
        common::copy_of(session.committed_doc(), sail_b)
    );
    // The hub's front and back walls, picked at the seated hub's
    // mid-height from either side.
    let wall_z = TOWER_HEIGHT + HUB_SIDE / 2.0;
    let front_wall =
        common::displayed_face_at(&session, &index, &common::along_y(1.0, 0.0, wall_z));
    assert_eq!(
        front_wall.node,
        common::copy_of(session.committed_doc(), hub_i),
        "the seated hub's front wall"
    );
    let back_wall =
        common::displayed_face_at(&session, &index, &common::along_y(-1.0, 0.0, wall_z));
    assert_eq!(
        back_wall.node,
        common::copy_of(session.committed_doc(), hub_i),
        "the seated hub's back wall"
    );
    assert_ne!(front_wall.name, back_wall.name, "two distinct walls");

    let mut tool = MateTool::new();
    tool.pick(session.doc(), sail_a_bottom.clone());
    tool.pick(session.doc(), front_wall.clone());
    let sail_a_proposal = {
        let (doc, _) = session.landed_pair().expect("landed");
        tool.proposal(doc, sail_choice(0.0))
            .expect("the first sail proposes")
    };
    let outcome = session.perform(sail_a_proposal.op_into_target_space());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        outcome.committed.len(),
        3,
        "the re-target, the mate and its values"
    );
    let [superseded] = &outcome.withdrawn.superseded[..] else {
        panic!("exactly one frame is superseded at the edit")
    };
    session.pump();
    assert_eq!(
        superseded.held,
        viewer::display::Held::Node(sail_a),
        "the first sail's park goes with its mate"
    );
    assert!(
        matches!(
            &superseded.cause,
            AdmissionFault::NotShown { space, .. } if space.id() == sail_a_space
        ),
        "and the report carries WHY it went: {}",
        superseded.cause
    );

    // The first blade's solved long axis, in world.
    let long_axis = Vec3::new(1.0, 0.0, 0.0);
    let blade_a_dir = {
        let (doc, _) = session.landed_pair().expect("landed");
        common::solve(&session, doc, tol)
            .placement(doc, sail_a)
            .expect("the instance is solved")
            .affine::<f64>()
            .transform_vec(long_axis)
    };
    close3(
        blade_a_dir,
        long_axis,
        "the first blade lies along the hub's x, unspun",
    );

    let mut tool = MateTool::new();
    tool.pick(session.doc(), sail_b_bottom.clone());
    tool.pick(session.doc(), back_wall.clone());
    let sail_b_proposal = {
        let (doc, _) = session.landed_pair().expect("landed");
        tool.proposal(doc, sail_choice(core::f64::consts::FRAC_PI_2))
            .expect("the second sail proposes")
    };
    let outcome = session.perform(sail_b_proposal.op_into_target_space());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        outcome.committed.len(),
        3,
        "the re-target, the mate and its values"
    );
    let [superseded] = &outcome.withdrawn.superseded[..] else {
        panic!("exactly one frame is superseded at the edit")
    };
    session.pump();
    assert_eq!(
        superseded.held,
        viewer::display::Held::Node(sail_b),
        "and so does the second's"
    );
    assert!(
        matches!(
            &superseded.cause,
            AdmissionFault::NotShown { space, .. } if space.id() == sail_b_space
        ),
        "and the report carries WHY it went: {}",
        superseded.cause
    );

    // ── 11. THE FINISHED WINDMILL: four rows all ok, each sail's
    // underside in its wall's plane, normals opposed, the blades
    // CROSSED.
    // And the at-rest badge CERTIFIES, honestly: the long blades
    // overhang the hub's walls, the census finds the crossings, and
    // the crossing rung backs each one from the sail's own declared
    // mate — the crossing point is inside the pair's verified overlap
    // region and the material lies on opposite sides of the shared
    // wall carrier, which is exactly what an overhanging seat is. An
    // overhang is ordinary authoring; the declaration the user
    // already made says the faces rest, once, for everything the seat
    // induces — the gate's verdict is the truth about this design,
    // shown on the draw path rather than saved for an export — once the
    // author says the four are what the document makes.
    common::product_is(&mut session, &[tower_i, hub_i, sail_a, sail_b]);
    let rows = session.tree_rows();
    assert_eq!(
        rows.len(),
        5,
        "four instances, each its own placement, the mates their children, and the product's ⊔"
    );
    for row in &rows {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    let (blade_a_dir, blade_b_dir) = {
        let (doc, _) = session.landed_pair().expect("landed");
        let poses = common::solve(&session, doc, tol);
        let placed = |node: RecipeNodeId| {
            poses
                .placement(doc, node)
                .expect("the instance is solved")
                .affine::<f64>()
        };
        let (_, eval) = session.landed_pair().expect("landed");
        for (sail_pick, wall_pick) in [(&sail_a_bottom, &front_wall), (&sail_b_bottom, &back_wall)]
        {
            let sail = face_frame(eval, sail_pick.node, &sail_pick.name)
                .expect("the sail's underside has a pose");
            let wall = face_frame(eval, wall_pick.node, &wall_pick.name)
                .expect("the hub's wall has a pose");
            assert!(
                (sail.origin - wall.origin).dot(wall.axis).abs() < 1e-9,
                "the sail's mated face lies in the hub's wall"
            );
            close3(
                sail.axis,
                wall.axis * -1.0,
                "the sail's underside faces into the wall",
            );
        }
        (
            placed(sail_a).transform_vec(long_axis),
            placed(sail_b).transform_vec(long_axis),
        )
    };
    assert!(
        blade_a_dir.dot(blade_b_dir).abs() < 1e-9,
        "the quarter-turn spin crosses the blades: {blade_a_dir:?} vs {blade_b_dir:?}"
    );
    assert_eq!(
        session.at_rest(),
        Some(&AtRestBadge {
            loud: 0,
            quiet: 0,
            could_not_look: 0,
        }),
        "the declared overhanging blades certify through the crossing rung"
    );

    // ── 12. SAVE AND REOPEN THE WORKSPACE. The document round-trips
    // with its mates; the fresh session re-resolves every instance
    // through the directory rule and reproduces the solved placement;
    // no display state survives (a fresh session's is empty by
    // construction — documentation, not a gate).
    let saved = session.perform(SessionOp::Save(asm_path.clone()));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
    let saved_doc = session.committed_doc().clone();
    let reopened = open_at(&asm_path, tol);
    assert!(
        reopened.committed_doc().bit_eq(&saved_doc),
        "save → reopen is bit-identity on the document"
    );
    let rows = reopened.tree_rows();
    assert_eq!(
        rows.len(),
        5,
        "the whole recipe came back, the product's ⊔ with it"
    );
    for row in &rows {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    assert!(reopened.display().toggled().is_empty());
    assert!(crate::common::asm::frame_of(&reopened, hub_i).is_none());
    assert_eq!(
        reopened.at_rest(),
        Some(&AtRestBadge {
            loud: 0,
            quiet: 0,
            could_not_look: 0,
        }),
        "the reopened census reads the same design: {:?}",
        reopened.at_rest()
    );
    {
        let (_, eval) = reopened.landed_pair().expect("landed");
        let world = face_frame(eval, hub_bottom.node, &hub_bottom.name)
            .expect("the hub's underside has a pose after reopen")
            .origin;
        close3(
            world - Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, TOWER_HEIGHT),
            "the solved seat survives the round trip",
        );
    }

    // ── 13. THE GALLERY DOOR (`common::story_gallery_dir` states the
    // contract): the windmill saved through the session's own save
    // door — parts beside the assembly, so the file opens standalone.
    // Skipped (not weakened) when unset.
    if let Some(gallery) = common::story_gallery_dir() {
        for (source, name) in [
            (&tower_file, "story-windmill-tower.pncad"),
            (&hub_file, "story-windmill-hub.pncad"),
            (&sail_file, "story-windmill-sail.pncad"),
            (&asm_path, "story-windmill.pncad"),
        ] {
            let mut copier = open_at(source, tol);
            let saved = copier.perform(SessionOp::Save(gallery.join(name)));
            assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
        }
        let standalone = open_at(&gallery.join("story-windmill.pncad"), tol);
        for row in standalone.tree_rows() {
            assert_eq!(row.status, RowStatus::Ok, "{row:?}");
        }
    }

    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}
