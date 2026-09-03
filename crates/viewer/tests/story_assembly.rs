//! **An assembly story: the windmill.** A user authors three small
//! parts — a slim tower, a hub, a long sail blade — into one
//! directory, then builds a windmill out of them entirely through the
//! session's typed op vocabulary: instances through the workspace door
//! (with its typed refusals), hide and the free-move probe as display
//! state, mates through the mate tool's two-pick flow (the second sail
//! with its roll reference turned a quarter turn, so the blades land
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
use common::{body_volume, insert, len, near, shape};
use pncad::document::{Doc, DocumentId, Frame, RecipeNodeId, solve_document};
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::select::{Ray, Resolution, RunCtx, resolve};
use viewer::camera::{self, Camera, CameraOp};
use viewer::display::DisplayFault;
use viewer::input::ViewportSize;
use viewer::matetool::{MateTool, MateToolState};
use viewer::pick::PickIndex;
use viewer::session::{
    AtRestBadge, DocSession, FaceSelection, Hovered, ProfileShape, Refusal, Selection, SessionOp,
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
/// centred rectangle, one extrude — assert its volume, and save it.
/// Answers the part's document id.
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
    let plane = common::xy_frame_in(session);
    let profile = insert(
        session,
        SessionOp::AddProfile {
            plane,
            loops: vec![shape(&ProfileShape::Rectangle { width, height })],
        },
    );
    let extrude = insert(
        session,
        SessionOp::AddExtrude {
            profile,
            distance: len(depth),
        },
    );
    assert!(
        near(body_volume(session, extrude, tol), width * height * depth),
        "the {name} part's volume is its box"
    );
    let saved = session.perform(SessionOp::Save(file.to_path_buf()));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
    session.committed_doc().id()
}

/// Perform one `AddInstance`, answering the minted node.
fn add_instance(session: &mut DocSession, id: DocumentId) -> RecipeNodeId {
    let node = insert(session, SessionOp::AddInstance { id });
    session.pump();
    node
}

/// Pick a face through the session's real ray path, under the current
/// display view (parked parts are picked where they are drawn).
fn pick_at(session: &DocSession, index: &PickIndex, ray: &Ray) -> FaceSelection {
    let (_, eval) = session.landed_pair().expect("landed");
    index
        .face_at_for(eval, ray, &session.display_view())
        .expect("the pick answers")
        .expect("the ray hits")
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

fn pt(m: [f64; 3]) -> Point3<f64> {
    Point3::new(m[0], m[1], m[2])
}

fn vc(m: [f64; 3]) -> Vec3<f64> {
    Vec3::new(m[0], m[1], m[2])
}

/// Drive one free-move gesture to its committed display value,
/// asserting it is display state and never a document edit.
fn park(session: &mut DocSession, instance: RecipeNodeId, at: [f64; 3]) {
    let history_len = session.history().len();
    for op in [
        SessionOp::BeginFreeMove { instance },
        SessionOp::PreviewFreeMove {
            frame: Frame::translation(at),
        },
        SessionOp::CommitFreeMove,
    ] {
        let outcome = session.perform(op);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        assert!(outcome.committed.is_empty(), "a probe commits no edit");
    }
    assert!(session.display().free_move_of(instance).is_some());
    assert_eq!(
        session.history().len(),
        history_len,
        "the probe leaves no history state"
    );
    assert!(
        session.doc().placements().get(&instance).is_none(),
        "the probe authors no placement"
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
    let tower_i = add_instance(&mut session, tower_id);
    let _slip = add_instance(&mut session, tower_id);
    assert_eq!(session.history().len(), 3, "root plus two adds");
    let undone = session.perform(SessionOp::Undo);
    assert!(undone.refusal.is_none(), "{:?}", undone.refusal);
    assert_eq!(session.doc().order().len(), 1, "the slip is off the path");
    let abandoned = session
        .history()
        .entry(session.history().current())
        .active_child()
        .expect("undo remembers the branch it left");
    let hub_i = add_instance(&mut session, hub_id);
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
        history.entry(abandoned).doc().order().len(),
        2,
        "the abandoned two-tower document is intact"
    );
    // Backwards and forwards across the add: redo follows the branch
    // the cursor is on — the hub, not the abandoned second tower.
    session.perform(SessionOp::Undo);
    assert_eq!(session.doc().order().len(), 1);
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

    // ── 4. RESOLVE: two instance rows, both ok.
    session.pump();
    let rows = session.tree_rows();
    assert_eq!(rows.len(), 2);
    for row in &rows {
        assert_eq!(row.kind, "InstantiatePart");
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
    let hidden = session.perform(SessionOp::SetInstanceHidden {
        instance: hub_i,
        hidden: true,
    });
    assert!(hidden.refusal.is_none(), "{:?}", hidden.refusal);
    assert!(hidden.committed.is_empty(), "hide commits no edit");
    assert!(session.display().hidden().contains(&hub_i));
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
    let shown = session.perform(SessionOp::SetInstanceHidden {
        instance: hub_i,
        hidden: false,
    });
    assert!(shown.refusal.is_none(), "{:?}", shown.refusal);
    assert!(session.display().hidden().is_empty());

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
        .bounds();
    let framed = Camera::framing(&bounds, aspect).expect("the windmill frames");
    let turned = camera::apply(
        &framed,
        &CameraOp::Orbit {
            yaw: 0.4,
            pitch: 0.1,
        },
    )
    .expect("a finite orbit");
    let camera = camera::apply(&turned, &CameraOp::Frame { bounds, aspect })
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
    assert_eq!(hit.node, tower_i, "the highest surface is the tower's");
    assert!(
        (hit.point.z - TOWER_HEIGHT).abs() < 1e-9,
        "the hit is the top face, at {:?}",
        hit.point
    );
    assert!(
        matches!(
            resolve(RunCtx { doc, eval }, &hit.name),
            Resolution::Resolved(_)
        ),
        "a just-picked name resolves in the run it was picked from"
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

    // ── 7. THE FREE-MOVE PROBE. A cancelled probe on the hub leaves
    // no trace at all; then the user parks the hub clear of the tower
    // (a committed display value, never a document edit).
    for op in [
        SessionOp::BeginFreeMove { instance: hub_i },
        SessionOp::PreviewFreeMove {
            frame: Frame::translation([0.0, 0.0, 0.2]),
        },
        SessionOp::CancelFreeMove,
    ] {
        let outcome = session.perform(op);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        assert!(outcome.committed.is_empty());
    }
    assert!(
        session.display().free_move_of(hub_i).is_none(),
        "a cancelled probe leaves no trace"
    );
    park(&mut session, hub_i, HUB_PARK);

    // ── 8. THE SEAT MATE, through the tool's two-pick flow: the hub's
    // underside (picked where it is DRAWN — the parked spot), then the
    // tower's top. One committed edit; the same outcome reports the
    // hub's probe superseded — discarded, not zeroed.
    let hub_bottom = pick_at(&session, &index, &asm::up_at(HUB_PARK[0], HUB_PARK[1]));
    assert_eq!(hub_bottom.node, hub_i, "the parked hub is picked");
    let tower_top = pick_at(&session, &index, &asm::down_at(0.0, 0.0));
    assert_eq!(tower_top.node, tower_i);
    let mut tool = MateTool::new();
    tool.pick(hub_bottom);
    tool.pick(tower_top);
    assert!(matches!(tool.state(), MateToolState::Two { .. }));
    let seat_proposal = {
        let (doc, eval) = session.landed_pair().expect("landed");
        tool.proposal(doc, eval, tol, asm::seat())
            .expect("the seat proposes")
    };
    let outcome = session.perform(seat_proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1, "exactly one committed edit");
    assert_eq!(
        outcome.superseded,
        vec![hub_i],
        "the mate's landing discards the park, in the same outcome"
    );
    assert!(session.display().free_move_of(hub_i).is_none());
    session.pump();
    let seat_mate = *session
        .committed_doc()
        .order()
        .last()
        .expect("the mate landed");

    // The placement is SOLVED: the hub's underside centre sits on the
    // tower's top centre — a known world point, the tower being
    // identity-placed — and the mate axes meet opposed. The base
    // CERTIFIES at rest: one declared contact, nested, answered.
    {
        let (doc, _) = session.landed_pair().expect("landed");
        let poses = solve_document(doc, tol);
        let placed_hub = poses
            .placement(doc, hub_i)
            .expect("the hub is solved")
            .affine::<f64>();
        let world = placed_hub.transform_point(pt(seat_proposal.alignment.a.origin));
        close3(
            world - Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, TOWER_HEIGHT),
            "the hub seats on the tower's top centre",
        );
        let axis = placed_hub.transform_vec(vc(seat_proposal.alignment.a.axis));
        close3(
            axis,
            Vec3::new(0.0, 0.0, -1.0),
            "the hub's mate axis opposes the tower top's normal",
        );
    }
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    assert!(
        session.tree_rows().iter().any(|row| row.kind == "Mate"),
        "the mate has a row"
    );
    assert_eq!(
        session.at_rest(),
        Some(&AtRestBadge::Certified { minted: 1 }),
        "the seated base certifies with its one declaration"
    );

    // A mate-constrained instance refuses the probe, typed, naming
    // the mate that binds it.
    let refused = session.perform(SessionOp::BeginFreeMove { instance: hub_i });
    match refused.refusal {
        Some(Refusal::Display(DisplayFault::MateConstrained { instance, mates })) => {
            assert_eq!(instance, hub_i);
            assert!(mates.contains(&seat_mate), "the refusal names the mate");
        }
        other => panic!("expected the mate-constrained refusal, got {other:?}"),
    }

    // ── 9. UNDO AND REDO ACROSS THE MATE: the edit steps out and back
    // in through the same doors as any other.
    session.perform(SessionOp::Undo);
    session.pump();
    assert!(
        session.doc().node(seat_mate).is_none(),
        "undo removes the mate"
    );
    assert_eq!(session.tree_rows().len(), 2, "two instances again");
    session.perform(SessionOp::Redo);
    session.pump();
    assert!(
        session.doc().node(seat_mate).is_some(),
        "redo restores the mate"
    );
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    assert_eq!(
        session.at_rest(),
        Some(&AtRestBadge::Certified { minted: 1 }),
        "redo re-certifies what undo took away"
    );

    // ── 10. THE SAILS: two more instances of one part, parked clear,
    // then mated onto the hub's opposite walls. The first sail goes
    // through the tool verbatim; the second goes through the AddMate
    // door with the tool's derived alignment, its roll reference
    // turned so the blade lands SQUARE to the first — measured against
    // the first blade's solved direction, not assumed from the walls'
    // parameterizations.
    let sail_a = add_instance(&mut session, sail_id);
    let sail_b = add_instance(&mut session, sail_id);
    park(&mut session, sail_a, SAIL_A_PARK);
    park(&mut session, sail_b, SAIL_B_PARK);
    let index = asm::index_of(&session);
    let sail_a_bottom = pick_at(
        &session,
        &index,
        &asm::up_at(SAIL_A_PARK[0], SAIL_A_PARK[1]),
    );
    assert_eq!(sail_a_bottom.node, sail_a);
    let sail_b_bottom = pick_at(
        &session,
        &index,
        &asm::up_at(SAIL_B_PARK[0], SAIL_B_PARK[1]),
    );
    assert_eq!(sail_b_bottom.node, sail_b);
    // The hub's front and back walls, picked at the seated hub's
    // mid-height from either side.
    let wall_z = TOWER_HEIGHT + HUB_SIDE / 2.0;
    let front_wall = pick_at(
        &session,
        &index,
        &Ray {
            origin: Point3::new(0.0, -1.0, wall_z),
            dir: Vec3::new(0.0, 1.0, 0.0),
        },
    );
    assert_eq!(front_wall.node, hub_i, "the seated hub's front wall");
    let back_wall = pick_at(
        &session,
        &index,
        &Ray {
            origin: Point3::new(0.0, 1.0, wall_z),
            dir: Vec3::new(0.0, -1.0, 0.0),
        },
    );
    assert_eq!(back_wall.node, hub_i, "the seated hub's back wall");
    assert_ne!(front_wall.name, back_wall.name, "two distinct walls");

    let mut tool = MateTool::new();
    tool.pick(sail_a_bottom);
    tool.pick(front_wall);
    let sail_a_proposal = {
        let (doc, eval) = session.landed_pair().expect("landed");
        tool.proposal(doc, eval, tol, asm::seat())
            .expect("the first sail proposes")
    };
    let outcome = session.perform(sail_a_proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
    assert_eq!(outcome.superseded, vec![sail_a]);
    session.pump();

    // The first blade's solved long axis, in world.
    let long_axis = Vec3::new(1.0, 0.0, 0.0);
    let (blade_a_dir, hub_placed) = {
        let (doc, _) = session.landed_pair().expect("landed");
        let poses = solve_document(doc, tol);
        let placed = |node: RecipeNodeId| {
            poses
                .placement(doc, node)
                .expect("the instance is solved")
                .affine::<f64>()
        };
        (placed(sail_a).transform_vec(long_axis), placed(hub_i))
    };

    let mut tool = MateTool::new();
    tool.pick(sail_b_bottom);
    tool.pick(back_wall);
    let sail_b_proposal = {
        let (doc, eval) = session.landed_pair().expect("landed");
        let base = tool
            .proposal(doc, eval, tol, asm::seat())
            .expect("the second sail proposes");
        // Turn the roll: of the derived reference and its in-plane
        // quarter turn (for unit vectors, r turned 90° about n is
        // n × r), keep whichever lands the blade square to the first —
        // the two walls are parallel planes, so exactly one does.
        // Hand-derived because no affordance clocks a mate: the coset
        // table statically refuses a clocking rider on a frame
        // coincidence, and `face_frame` roll references carry no
        // documented relation across opposite walls — issue 1461.
        let mut chosen = base;
        let derived = vc(chosen.alignment.b.reference);
        let quarter = vc(chosen.alignment.b.axis).cross(derived);
        let world_of = |candidate: Vec3<f64>| hub_placed.transform_vec(candidate);
        let candidate = if world_of(derived).dot(blade_a_dir).abs()
            < world_of(quarter).dot(blade_a_dir).abs()
        {
            derived
        } else {
            quarter
        };
        assert!(
            world_of(candidate).dot(blade_a_dir).abs() < 1e-9,
            "one of the two quarter turns is square to the first blade"
        );
        chosen.alignment.b.reference = [candidate.x, candidate.y, candidate.z];
        chosen
    };
    let outcome = session.perform(SessionOp::AddMate {
        a: sail_b_proposal.a.clone(),
        b: sail_b_proposal.b.clone(),
        class: sail_b_proposal.class,
        alignment: sail_b_proposal.alignment,
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
    assert_eq!(outcome.superseded, vec![sail_b]);
    session.pump();

    // ── 11. THE FINISHED WINDMILL: seven rows all ok, both sail
    // frames landed where their mates declare, the blades CROSSED.
    // And the at-rest badge CERTIFIES, honestly: the long blades
    // overhang the hub's walls, the census finds the crossings, and
    // the crossing rung backs each one from the sail's own declared
    // mate — the crossing point is inside the pair's verified overlap
    // region and the material lies on opposite sides of the shared
    // wall carrier, which is exactly what an overhanging seat is. An
    // overhang is ordinary authoring; the declaration the user
    // already made says the faces rest, once, for everything the seat
    // induces — the gate's verdict is the truth about this design,
    // shown on the draw path rather than saved for an export.
    let rows = session.tree_rows();
    assert_eq!(rows.len(), 7, "four instances and three mates");
    for row in &rows {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    let (blade_a_dir, blade_b_dir) = {
        let (doc, _) = session.landed_pair().expect("landed");
        let poses = solve_document(doc, tol);
        let placed = |node: RecipeNodeId| {
            poses
                .placement(doc, node)
                .expect("the instance is solved")
                .affine::<f64>()
        };
        let hub_placed = placed(hub_i);
        for (proposal, sail) in [(&sail_a_proposal, sail_a), (&sail_b_proposal, sail_b)] {
            let world_sail = placed(sail).transform_point(pt(proposal.alignment.a.origin));
            let world_hub = hub_placed.transform_point(pt(proposal.alignment.b.origin));
            close3(
                world_sail - Point3::new(0.0, 0.0, 0.0),
                world_hub - Point3::new(0.0, 0.0, 0.0),
                "the sail's mated frame lands on the hub's wall frame",
            );
        }
        (
            placed(sail_a).transform_vec(long_axis),
            placed(sail_b).transform_vec(long_axis),
        )
    };
    assert!(
        blade_a_dir.dot(blade_b_dir).abs() < 1e-9,
        "the turned roll reference crosses the blades: {blade_a_dir:?} vs {blade_b_dir:?}"
    );
    assert_eq!(
        session.at_rest(),
        Some(&AtRestBadge::Certified { minted: 3 }),
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
    assert_eq!(rows.len(), 7, "the whole recipe came back");
    for row in &rows {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    assert!(reopened.display().hidden().is_empty());
    assert!(reopened.display().free_move_of(hub_i).is_none());
    assert_eq!(
        reopened.at_rest(),
        Some(&AtRestBadge::Certified { minted: 3 }),
        "the reopened census reads the same design: {:?}",
        reopened.at_rest()
    );
    {
        let (doc, _) = reopened.landed_pair().expect("landed");
        let poses = solve_document(doc, tol);
        let placed_hub = poses
            .placement(doc, hub_i)
            .expect("the hub is solved after reopen")
            .affine::<f64>();
        let world = placed_hub.transform_point(pt(seat_proposal.alignment.a.origin));
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
