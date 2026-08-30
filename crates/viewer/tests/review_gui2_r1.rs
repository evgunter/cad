//! GUI-2 review R1 — an independent consumer's derivation of the
//! viewport-selection claims (PR #1106).
//!
//! Own fixtures throughout (two disjoint extrudes, a pattern with its
//! own dimensions, the committed gallery ring), own ray and cursor
//! constructions, own tolerance derivations — deliberately NOT the
//! unit's plate helpers, because a promoted review suite's value is
//! that it derives the claims independently
//! (`memories/review-and-dependency-policy.md`).
//!
//! Conventions per `memories/test-suite-cost.md`: the randomized rows
//! draw a fresh seed per run through `test_utils::fuzz` (logged
//! unconditionally, replayable via `CAD_FUZZ_SEED`), and their counts
//! ride the shared effort dial. The one `#[ignore]`d row is an
//! EVIDENCE PROBE for the review record, not a gate — its own docs say
//! why it must never gate.
//!
//! What no row here executes: the GPU id pass (`viewer::gpu` is
//! `app`-gated and has never run anywhere), and the egui widget→event
//! mapping in `app.rs`. Issue #1097 owns those on hardware.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use pncad::document::{Doc, LoopProgram, Node, PatternKind, ProfileProgram, RecipeNodeId, SlotId};
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::profile::SketchPlane;
use pncad::select::{Ray, Resolution, RunCtx, resolve};
use test_utils::fuzz;
use viewer::camera::Camera;
use viewer::input::{InputMap, PointerButton, ViewportEvent, ViewportSize};
use viewer::pick::{IdMap, PickIndex};
use viewer::props::SlotValue;
use viewer::scene::DisplayTolerance;
use viewer::session::{DocSession, Selection, SessionOp};
use viewer::{cursor_projection, input};

/// This suite's own display tolerance — chosen independently of the
/// unit's rows.
fn delta() -> DisplayTolerance {
    DisplayTolerance::new(1.5e-4).expect("a positive delta")
}

/// A square profile at an offset — this suite's own authoring helper,
/// so the fixtures do not share the unit's.
fn offset_square(x0: f64, y0: f64, side: f64) -> Node<ProfileProgram> {
    Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![
            LoopProgram::polygon([
                (x0, y0),
                (x0 + side, y0),
                (x0 + side, y0 + side),
                (x0, y0 + side),
            ])
            .expect("finite corners"),
        ],
    })
}

/// Insert one node, keeping the id.
fn inserted(
    doc: &Doc<ProfileProgram>,
    node: Node<ProfileProgram>,
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let applied = pncad::document::apply(doc, &pncad::document::DocEdit::InsertNode { node }, tol)
        .expect("the fixture edit applies");
    let id = *applied
        .doc
        .order()
        .last()
        .expect("the inserted node is last");
    (applied.doc, id)
}

/// Two DISJOINT extruded blocks under two separate roots: block A is
/// `[0,0.02]² × 0.01`, block B is `[0.1,0.14]×[0,0.04] × 0.02`.
fn two_blocks(tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("gui2-r1-two-blocks", tol);
    let (doc, pa) = inserted(&doc, offset_square(0.0, 0.0, 0.02), tol);
    let (doc, a) = inserted(
        &doc,
        Node::Extrude {
            profile: pa,
            distance: len(0.01),
        },
        tol,
    );
    let (doc, pb) = inserted(&doc, offset_square(0.1, 0.0, 0.04), tol);
    let (doc, b) = inserted(
        &doc,
        Node::Extrude {
            profile: pb,
            distance: len(0.02),
        },
        tol,
    );
    (doc, a, b)
}

fn len(metres: f64) -> pncad::document::Expr {
    pncad::document::Expr::literal(metres, pncad::document::Dimension::Length)
        .expect("a finite length")
}

fn scl(value: f64) -> pncad::document::Expr {
    pncad::document::Expr::literal(value, pncad::document::Dimension::Scalar)
        .expect("a finite scalar")
}

/// A landed session plus its pick index.
fn landed(doc: Doc<ProfileProgram>, tol: Tol) -> (DocSession, PickIndex) {
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let index = index_of(&session);
    (session, index)
}

fn index_of(session: &DocSession) -> PickIndex {
    index_at(session, delta())
}

fn index_at(session: &DocSession, delta: DisplayTolerance) -> PickIndex {
    let (doc, eval) = session.landed_pair().expect("the inline seam lands");
    PickIndex::build(
        doc,
        eval,
        session.landed_generation().expect("a landed generation"),
        delta,
        session.tol(),
    )
    .expect("the fixture indexes")
}

/// A ray straight down at `(x, y)` from above everything here.
fn down(x: f64, y: f64) -> Ray {
    Ray {
        origin: Point3::new(x, y, 0.5),
        dir: Vec3::new(0.0, 0.0, -1.0),
    }
}

// --- un-projection, this suite's own construction -------------------

/// The un-projection inverts the projection for RANDOM cameras and
/// RANDOM pixels, not just the shipped fixture's corners: any point
/// re-projected from anywhere along the un-projected ray lands back on
/// the cursor's own pixel.
///
/// Shape: counterexample search (fresh seed, logged; count on the
/// effort dial).
#[test]
fn unprojection_inverts_projection_for_random_cameras_and_pixels() {
    let mut rng = fuzz::start("gui2-r1 unprojection");
    for case in 0..fuzz::scaled(48) {
        let camera = Camera::new(
            Point3::new(
                rng.range(-2.0, 2.0),
                rng.range(-2.0, 2.0),
                rng.range(-2.0, 2.0),
            ),
            rng.range(0.5, 8.0),
            rng.range(-3.0, 3.0),
            rng.range(-1.2, 1.2),
            rng.range(0.3, 2.4),
            1.0,
        )
        .expect("a finite camera");
        let viewport = ViewportSize {
            width_px: rng.range(64.0, 3000.0),
            height_px: rng.range(64.0, 3000.0),
        };
        let aspect = viewport.aspect().expect("a positive aspect");
        let cursor = [
            rng.range(0.0, viewport.width_px),
            rng.range(0.0, viewport.height_px),
        ];
        let ray = camera
            .ray_through(cursor, viewport)
            .expect("a finite cursor un-projects");
        // The contract's own two clauses first: unit direction, eye
        // origin.
        let n = ray.dir.dot(ray.dir).sqrt();
        assert!((n - 1.0).abs() < 1.0e-12, "case {case}: |dir| = {n}");
        let eye = camera.eye();
        assert_eq!(
            (ray.origin.x, ray.origin.y, ray.origin.z),
            (eye.x, eye.y, eye.z)
        );
        // Then the inversion: three points along the ray re-project to
        // the cursor's pixel.
        for t in [0.25, 1.0, 6.0] {
            let p = ray.origin + ray.dir * t;
            let ndc = camera
                .project(p, aspect)
                .expect("the projection is defined")
                .expect("a point down the ray is in front of the eye");
            let px = [
                (ndc[0] + 1.0) * 0.5 * viewport.width_px,
                (1.0 - ndc[1]) * 0.5 * viewport.height_px,
            ];
            let err = ((px[0] - cursor[0]).powi(2) + (px[1] - cursor[1]).powi(2)).sqrt();
            assert!(
                err < 1.0e-6,
                "case {case} t={t}: re-projected pixel off by {err}px \
                 (cursor {cursor:?}, got {px:?}) — replay per the [fuzz] line above"
            );
        }
    }
}

/// `cursor_projection`'s algebra, checked against an independent
/// statement of it: for any clip-space point with `w > 0`, the
/// transformed NDC is `(ndc − cursor) · viewport` in both axes, and
/// depth/w are untouched.
#[test]
fn cursor_projection_is_exactly_a_shift_and_scale_in_ndc() {
    let mut rng = fuzz::start("gui2-r1 cursor-projection");
    for case in 0..fuzz::scaled(64) {
        let camera = Camera::new(
            Point3::new(0.0, 0.0, 0.0),
            rng.range(1.0, 5.0),
            rng.range(-3.0, 3.0),
            rng.range(-1.2, 1.2),
            rng.range(0.4, 2.0),
            1.0,
        )
        .expect("a finite camera");
        let aspect = rng.range(0.4, 3.0);
        let matrix = camera.view_projection(aspect).expect("defined");
        let vp32 = matrix.map(|c| c.map(|v| v as f32));
        let point = Point3::new(
            rng.range(-0.8, 0.8),
            rng.range(-0.8, 0.8),
            rng.range(-0.8, 0.8),
        );
        if camera.project(point, aspect).expect("defined").is_none() {
            continue; // behind the eye: not this row's subject
        }
        let cursor = [rng.range(-1.0, 1.0) as f32, rng.range(-1.0, 1.0) as f32];
        let size = [
            rng.range(64.0, 4000.0) as f32,
            rng.range(64.0, 4000.0) as f32,
        ];
        let shifted = cursor_projection(&vp32, cursor, size);
        let v = [point.x as f32, point.y as f32, point.z as f32, 1.0f32];
        let apply = |m: &[[f32; 4]; 4]| {
            let mut out = [0.0f32; 4];
            for (row, slot) in out.iter_mut().enumerate() {
                *slot = m[0][row] * v[0] + m[1][row] * v[1] + m[2][row] * v[2] + m[3][row] * v[3];
            }
            out
        };
        let base = apply(&vp32);
        let out = apply(&shifted);
        assert!(base[3] > 0.0);
        let want_x = (base[0] / base[3] - cursor[0]) * size[0];
        let want_y = (base[1] / base[3] - cursor[1]) * size[1];
        let got_x = out[0] / out[3];
        let got_y = out[1] / out[3];
        let tol_of = |want: f32| 1.0e-3f32.max(want.abs() * 1.0e-4);
        assert!(
            (got_x - want_x).abs() < tol_of(want_x) && (got_y - want_y).abs() < tol_of(want_y),
            "case {case}: shifted NDC ({got_x}, {got_y}) vs independent ({want_x}, {want_y})"
        );
        // Depth and w are pass-through: rows 2 and 3 of the matrix are
        // untouched by construction, so the clip z and w agree exactly.
        assert_eq!(out[2], base[2], "case {case}: depth is untouched");
        assert_eq!(out[3], base[3], "case {case}: w is untouched");
    }
}

// --- the id alphabet over this suite's own two-root fixture ---------

/// Two separate roots' patches share no id, every drawn patch's name
/// is `Ok`, and the forward/backward maps agree — the bijection over a
/// fixture whose bodies come from DIFFERENT nodes, where the unit's
/// three-instance row had one node with three bodies.
#[test]
fn two_roots_draw_under_disjoint_ids_and_every_patch_is_named() {
    let tol = Tol::witness();
    let (doc, a, b) = two_blocks(tol);
    let (session, index) = landed(doc, tol);
    let _ = &session;
    let ids = index.ids();
    assert!(!ids.is_empty(), "two blocks draw");
    let mut nodes = std::collections::BTreeSet::new();
    for id in ids.ids() {
        let key = ids.key_of(id).expect("assigned ids invert");
        assert_eq!(ids.id_of(key), Some(id), "the two directions agree");
        nodes.insert(key.node);
        let name = index
            .name_of(id)
            .expect("a drawn id has a name slot")
            .as_ref()
            .expect("every drawn patch is named");
        assert!(
            index.ids_of(name).contains(&id),
            "the name's id list carries the id that named it"
        );
    }
    assert_eq!(
        nodes.into_iter().collect::<Vec<_>>(),
        vec![a, b],
        "both roots are drawn and no third node appears"
    );
    assert_eq!(ids.key_of(IdMap::NOTHING), None, "0 stays reserved");
}

// --- the cursor path as a consumer drives it ------------------------

/// The whole event→op→selection path on this suite's own fixture: a
/// primary click on block A selects a face of A; a second on block B
/// REPLACES it; a click on empty space clears; hover never touches the
/// selection.
#[test]
fn clicks_select_replace_and_clear_and_hover_is_transient() {
    let tol = Tol::witness();
    let (doc, a, b) = two_blocks(tol);
    let (mut session, index) = landed(doc, tol);
    let viewport = ViewportSize {
        width_px: 1600.0,
        height_px: 900.0,
    };
    let aspect = viewport.aspect().expect("a positive aspect");
    // This suite's own framing: the index's scene bounds, not the
    // unit's plate helper.
    let scene = index.scene().expect("the fixture draws");
    let camera = Camera::framing(&scene.bounds(), aspect).expect("the fixture frames");
    let cursor_at = |p: Point3<f64>| {
        let ndc = camera
            .project(p, aspect)
            .expect("defined")
            .expect("in front of the eye");
        [
            (ndc[0] + 1.0) * 0.5 * viewport.width_px,
            (1.0 - ndc[1]) * 0.5 * viewport.height_px,
        ]
    };
    // Top-face centres of the two blocks.
    let on_a = cursor_at(Point3::new(0.01, 0.01, 0.01));
    let on_b = cursor_at(Point3::new(0.12, 0.02, 0.02));
    let map = InputMap::default();
    let run = |session: &mut DocSession, events: &[ViewportEvent]| {
        for action in input::pick_stream(&map, events) {
            let op = index
                .op_for(
                    session.evaluation().expect("landed"),
                    &camera,
                    viewport,
                    action,
                )
                .expect("the cursor path answers");
            session.perform(op);
        }
    };
    run(
        &mut session,
        &[ViewportEvent::Click {
            button: PointerButton::Primary,
            pos_px: on_a,
        }],
    );
    let first = session.selection().face().expect("A is selected").clone();
    assert_eq!(first.node, a);
    run(
        &mut session,
        &[
            ViewportEvent::Hover { pos_px: on_a },
            ViewportEvent::Click {
                button: PointerButton::Primary,
                pos_px: on_b,
            },
        ],
    );
    let second = session.selection().face().expect("B replaced A").clone();
    assert_eq!(second.node, b, "single-select: the second pick replaces");
    assert_ne!(first.name, second.name);
    assert_eq!(
        session.hover().map(|h| h.node),
        Some(a),
        "the hover still names A; it is not the selection"
    );
    run(&mut session, &[ViewportEvent::Leave]);
    assert_eq!(session.hover(), None, "leaving clears only the hover");
    assert_eq!(session.selection().face().map(|f| f.node), Some(b));
    // A primary click that meets nothing: empty space far off both.
    run(
        &mut session,
        &[ViewportEvent::Click {
            button: PointerButton::Primary,
            pos_px: [2.0, 2.0],
        }],
    );
    assert_eq!(*session.selection(), Selection::None, "a miss clears");
}

// --- survival, this suite's own walk --------------------------------

/// Delete the picked face's OWNING node through the op vocabulary:
/// the selection value survives, its standing goes dead and typed,
/// the panel rows vanish, and undo brings all of it back.
#[test]
fn deleting_the_owner_kills_the_standing_and_undo_revives_it() {
    let tol = Tol::witness();
    let (doc, a, b) = two_blocks(tol);
    let (mut session, index) = landed(doc, tol);
    let face = index
        .face_at(session.evaluation().expect("landed"), &down(0.01, 0.01))
        .expect("no refusal")
        .expect("block A is under this ray");
    assert_eq!(face.node, a);
    session.perform(SessionOp::Select(Selection::Face(face.clone())));
    assert!(session.standing().live());
    assert!(!session.slot_rows().is_empty(), "a live face offers rows");

    let out = session.perform(SessionOp::DeleteNode { node: a });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    assert_eq!(
        session.selection().face(),
        Some(&face),
        "no silent clear: the value is still the picked name"
    );
    let standing = session.standing();
    assert!(!standing.live());
    assert!(
        standing.unresolved().is_some(),
        "typed verdict, not an absence: {standing:?}"
    );
    assert!(session.slot_rows().is_empty(), "affordances are off");
    // The OTHER root is untouched and still selectable — the dead
    // selection poisoned nothing beside itself.
    session.perform(SessionOp::Select(Selection::Node(b)));
    assert!(session.standing().live());

    session.perform(SessionOp::Select(Selection::Face(face.clone())));
    session.perform(SessionOp::Undo);
    session.pump();
    assert!(
        session.standing().live(),
        "undoing the delete revives the same un-re-picked name"
    );
}

/// Undo across the selection's BIRTH on this suite's own pattern
/// (spacing and count differ from the unit's), picked on the youngest
/// instance's WALL by a horizontal ray rather than the unit's vertical
/// one.
#[test]
fn undo_across_the_birth_of_a_wall_pick_unresolves_and_redo_revives() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("gui2-r1-pattern", tol);
    let (doc, profile) = inserted(&doc, offset_square(0.0, 0.0, 0.03), tol);
    let (doc, extrude) = inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: len(0.015),
        },
        tol,
    );
    let (doc, pattern) = inserted(
        &doc,
        Node::Pattern {
            input: extrude,
            count: pncad::document::Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(0.0), scl(1.0), scl(0.0)],
                spacing: len(0.08),
            },
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    session.perform(SessionOp::SetSlot {
        node: pattern,
        slot: SlotId::Count,
        value: SlotValue::Count(3),
    });
    session.pump();
    let index = index_of(&session);
    // The third instance spans y ∈ [0.16, 0.19]; a horizontal ray
    // along +x at its mid-height meets its x=0 wall first.
    let wall = Ray {
        origin: Point3::new(-1.0, 0.175, 0.0075),
        dir: Vec3::new(1.0, 0.0, 0.0),
    };
    let face = index
        .face_at(session.evaluation().expect("landed"), &wall)
        .expect("no refusal")
        .expect("the third instance's wall is under this ray");
    assert_eq!(face.body, 2, "the youngest instance is output body 2");
    session.perform(SessionOp::Select(Selection::Face(face.clone())));
    assert!(session.standing().live());

    session.perform(SessionOp::Undo);
    session.pump();
    assert_eq!(session.selection().face(), Some(&face));
    assert!(!session.standing().live(), "its birth was undone");
    assert!(session.standing().unresolved().is_some());
    assert!(
        !index.current_for(session.landed_generation(), delta()),
        "and the index that answered the pick is stale, to be discarded"
    );

    session.perform(SessionOp::Redo);
    session.pump();
    assert!(session.standing().live(), "redo revives without re-picking");
}

// --- the end-to-end consumer walk on a real gallery document --------

/// The committed gallery ring, re-stamped to this run's ε the same way
/// the doc-io suite's rows are (ε is the file's only ε-dependent byte;
/// that claim has its own gate there).
const GALLERY_RING: &str = include_str!("gallery_ring.v17.pncad");

fn ring_at(tol: Tol) -> String {
    let probe: Doc<ProfileProgram> = Doc::empty_derived("gui2-r1-eps-probe", tol);
    let text = pncad::document::save(&probe, &[], tol).expect("an empty document saves");
    let is_eps = |line: &str| line.trim_start().starts_with("\"epsilon\":");
    let wanted = text.lines().find(|l| is_eps(l)).expect("ε is recorded");
    let mut out: String = GALLERY_RING
        .lines()
        .map(|l| if is_eps(l) { wanted } else { l })
        .collect::<Vec<&str>>()
        .join("\n");
    out.push('\n');
    out
}

/// **The acceptance walk, headless**: open a real gallery document,
/// aim the cursor by projecting a visible point of the drawn mesh,
/// select through the op vocabulary, watch tree/panel unity through
/// the ONE selection value, delete the owner, watch the typed
/// unresolved state, and undo it all back.
///
/// What this deliberately does NOT touch: the GPU pass, the window,
/// the dialog, and `app.rs`'s widget mapping — the excused surfaces.
#[test]
fn e2e_a_gallery_ring_is_picked_edited_killed_and_revived() {
    let tol = Tol::witness();
    // Coarser than the block fixtures' δ: the ring is a revolve and
    // this row tessellates it twice; picking semantics do not depend
    // on the facet count (`memories/test-suite-cost.md` — keep the
    // per-run cost where the claim needs it).
    let ring_delta = DisplayTolerance::new(2.0e-3).expect("a positive delta");
    let loaded = pncad::document::load(&ring_at(tol), tol).expect("the gallery ring loads");
    let mut session = DocSession::inline(loaded.snapshot, tol);
    session.pump();
    let index = index_at(&session, ring_delta);
    assert!(!index.ids().is_empty(), "the ring draws pickable patches");

    let viewport = ViewportSize {
        width_px: 1440.0,
        height_px: 810.0,
    };
    let aspect = viewport.aspect().expect("a positive aspect");
    let scene = index.scene().expect("the ring draws");
    let camera = Camera::framing(&scene.bounds(), aspect).expect("the ring frames");

    // Aim at the centroid of some triangle of the drawn tessellation
    // whose surface faces the camera — derived from the mesh itself,
    // not from knowledge of the ring's dimensions.
    let part = index.parts().first().expect("one part at least");
    let mesh = part.mesh();
    let eye = camera.eye();
    let mut cursor = None;
    'outer: for patch in &mesh.patches {
        for tri in &patch.triangles {
            let [i, j, k] = tri.map(|i| mesh.positions[i as usize]);
            let centroid = Point3::new(
                (i.x + j.x + k.x) / 3.0,
                (i.y + j.y + k.y) / 3.0,
                (i.z + j.z + k.z) / 3.0,
            );
            let n = (j - i).cross(k - i);
            let to_eye = eye - centroid;
            // Comfortably front-facing and comfortably in frame.
            if n.dot(to_eye) > 1.0e-9
                && let Ok(Some(ndc)) = camera.project(centroid, aspect)
                && ndc[0].abs() < 0.9
                && ndc[1].abs() < 0.9
            {
                cursor = Some([
                    (ndc[0] + 1.0) * 0.5 * viewport.width_px,
                    (1.0 - ndc[1]) * 0.5 * viewport.height_px,
                ]);
                break 'outer;
            }
        }
    }
    let cursor = cursor.expect("a framed body shows the camera some triangle");

    // The whole cursor path, exactly as the app runs it.
    let map = InputMap::default();
    let events = [
        ViewportEvent::Hover { pos_px: cursor },
        ViewportEvent::Click {
            button: PointerButton::Primary,
            pos_px: cursor,
        },
    ];
    for action in input::pick_stream(&map, &events) {
        let op = index
            .op_for(
                session.evaluation().expect("landed"),
                &camera,
                viewport,
                action,
            )
            .expect("the cursor path answers");
        session.perform(op);
    }
    let face = session
        .selection()
        .face()
        .expect("the click selected a ring face")
        .clone();
    assert_eq!(
        session.hover().map(|h| h.name.clone()),
        Some(face.name.clone()),
        "hover and click answered the same cursor the same way"
    );

    // Unity: the tree highlights the owner, the panel answers for it,
    // and both read the ONE value.
    let owner = session.selection().node().expect("a face has an owner");
    assert!(
        session.tree_rows().iter().any(|row| row.id == owner),
        "the owning feature is a tree row"
    );
    assert!(session.standing().live());
    // The name resolves through the shipped door against the landed
    // pair — the same verdict `standing` reports.
    let (doc, eval) = session.landed_pair().expect("a landed pair");
    assert!(matches!(
        resolve(RunCtx { doc, eval }, &face.name),
        Resolution::Resolved(_)
    ));

    // Kill the owner through the panel's own op; watch the typed
    // unresolved state; nothing crashes, nothing silently clears.
    let out = session.perform(SessionOp::DeleteNode { node: owner });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    assert_eq!(session.selection().face(), Some(&face));
    assert!(!session.standing().live());
    assert!(session.standing().unresolved().is_some());
    assert!(session.slot_rows().is_empty());
    assert!(
        !index.current_for(session.landed_generation(), ring_delta),
        "the pick index is stale after the re-evaluation and is discarded whole"
    );

    // Undo across the whole episode: the ring is back, the SAME
    // selection value resolves again, and a fresh index picks the
    // same name at the same cursor.
    session.perform(SessionOp::Undo);
    session.pump();
    assert!(session.standing().live(), "the un-deleted owner resolves");
    let fresh = index_at(&session, ring_delta);
    let re_hit = fresh
        .op_for(
            session.evaluation().expect("landed"),
            &camera,
            viewport,
            input::PickAction::Select(cursor),
        )
        .expect("the cursor path still answers");
    match re_hit {
        SessionOp::Select(Selection::Face(f)) => {
            assert_eq!(f.name, face.name, "the same cursor picks the same name");
        }
        other => panic!("expected a face selection, got {other:?}"),
    }
}

// --- evidence probe (review record; not a gate) ---------------------

/// EVIDENCE PROBE for the review record — `#[ignore]`d because it must
/// never gate: it DEMONSTRATES that the façade's "unnameable type"
/// seal on `Resolved::entity` does not prevent a `pncad`-only consumer
/// from extracting, storing and comparing an arena-keyed value through
/// type inference — the very acts the PR body says the seal prevents.
/// If the seal is later hardened (field made private, or a wrapper
/// without `PartialEq`/`Copy`), this probe stops compiling; that would
/// be the fix landing, so a red here must never block anything.
#[test]
#[ignore = "GUI-2 R1 evidence probe: documents the payload-field leak through the pncad seal; never a gate"]
fn evidence_probe_an_arena_key_is_storable_and_comparable_via_inference() {
    struct Stash<T>(T);
    let tol = Tol::witness();
    let (doc, a, _) = two_blocks(tol);
    let (session, index) = landed(doc, tol);
    let face = index
        .face_at(session.evaluation().expect("landed"), &down(0.01, 0.01))
        .expect("no refusal")
        .expect("a hit");
    assert_eq!(face.node, a);
    let (d, e) = session.landed_pair().expect("a landed pair");
    let Resolution::Resolved(r) = resolve(RunCtx { doc: d, eval: e }, &face.name) else {
        panic!("a just-picked name resolves");
    };
    // The type of `r.entity` is never spelled anywhere in this file,
    // yet it is copied out, stored in this consumer's own state, and
    // compared with a later run's key.
    let stored = Stash(r.entity);
    let Resolution::Resolved(again) = resolve(RunCtx { doc: d, eval: e }, &face.name) else {
        panic!("still resolves");
    };
    assert!(
        stored.0 == again.entity,
        "a layer-3 consumer just compared arena keys across two resolve calls"
    );
    println!(
        "[evidence] pncad-only code stored and compared an arena-keyed value: {:?}",
        stored.0
    );
}
