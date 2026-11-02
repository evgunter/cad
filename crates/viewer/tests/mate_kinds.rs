//! **Every one-pick mate kind, picked through the cursor and committed.**
//!
//! The mate tool reads a plane off a face, an axis off a face's carrier
//! or an edge's line, a point off a vertex or a carrier's or a circle's
//! centre, and a direction off a face's normal or an edge's line. Each
//! row aims a cursor at drawn geometry (the cursor DERIVED from the
//! drawn polyline through the camera, never typed in), feeds the pick
//! the cursor path answers to the open tool through `Tools::feed`, and
//! commits the proposal through the session's one door; then it reads
//! the drawn picture back to see the mate hold.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::asm;
use pncad::document::{MateSide, RecipeNodeId, VarKind};
use pncad::geom_core::{Point3, Tol};
use pncad::select::EntityKind;
use viewer::camera::Camera;
use viewer::display::DisplayView;
use viewer::input::ViewportSize;
use viewer::matetool::{MateChoice, MateToolError};
use viewer::pickindex::{PickIndex, PickKinds, PickedKind};
use viewer::session::{DocSession, Hovered, ProfileShape, SessionOp};
use viewer::tools::{ToolKind, Tools};

/// The pane every row picks in.
fn pane() -> ViewportSize {
    ViewportSize {
        width_px: 1280.0,
        height_px: 720.0,
    }
}

/// An eye in front of the scene (toward `-y`), a little above it.
fn front_camera(target: Point3<f64>) -> Camera {
    Camera::new(
        target,
        0.3,
        -std::f64::consts::FRAC_PI_2 + 0.2,
        0.35,
        std::f64::consts::FRAC_PI_4,
        0.2,
    )
    .expect("a camera")
}

/// Where a world point lands in the pane, in physical pixels.
fn pixel_of(camera: &Camera, point: Point3<f64>) -> [f64; 2] {
    let aspect = pane().aspect().expect("a positive aspect");
    let ndc = camera
        .project(point, aspect)
        .expect("the projection is defined")
        .expect("a framed point is in front of the eye");
    pane().cursor_of([ndc[0], ndc[1]]).expect("a positive area")
}

/// **What the cursor at `point` picks under `kinds`**, by the viewport's
/// own door.
fn hovered(
    session: &DocSession,
    index: &PickIndex,
    camera: &Camera,
    point: Point3<f64>,
    kinds: PickKinds,
) -> Hovered {
    let (_, eval) = session.landed_pair().expect("landed");
    index
        .hovered_for(
            eval,
            camera,
            pane(),
            pixel_of(camera, point),
            &session.display_view(),
            kinds,
        )
        .expect("the pick answers")
        .expect("the cursor is over the scene")
}

/// The drawn edge of `node` every point of which `on` holds, and its
/// polyline.
fn edge_where(
    index: &PickIndex,
    node: RecipeNodeId,
    on: impl Fn(Point3<f64>) -> bool,
) -> Vec<Point3<f64>> {
    index
        .edges_in(node, 0)
        .iter()
        .map(|&id| index.edge_polyline_for(id, &DisplayView::none()))
        .find(|points| !points.is_empty() && points.iter().all(|&p| on(p)))
        .expect("the fixture draws the edge")
}

/// A polyline's middle point.
fn middle(points: &[Point3<f64>]) -> Point3<f64> {
    let (a, b) = (points[0], points[points.len() - 1]);
    if points.len() > 2 {
        return points[points.len() / 2];
    }
    Point3::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0, (a.z + b.z) / 2.0)
}

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 1.0e-6
}

/// The mate tool opened, fed `picks` by the selection ops a click on
/// each makes, as the frame feeds them.
fn mate_fed(session: &DocSession, index: &PickIndex, picks: &[Hovered]) -> Tools {
    let mut tools = Tools::new();
    tools.open(ToolKind::Mate);
    let ops: Vec<SessionOp> = picks
        .iter()
        .map(|pick| SessionOp::Select(pick.selection()))
        .collect();
    let notices = tools.feed(session.doc(), Ok(index), &ops);
    assert!(notices.is_empty(), "every pick is taken: {notices:?}");
    tools
}

fn choice(kind: VarKind, flip: bool) -> MateChoice {
    MateChoice {
        kind,
        flip,
        values: Vec::new(),
    }
}

/// **The cursor's rule admits exactly what the chosen kind reads**:
/// for every offered kind, the entities the rule may answer are the
/// entities the kind's poses read (`matetool::reads`, the kernel's
/// seats), so a plane's cursor can never land on an edge and a point's
/// can always land on a vertex; with no mate tool open the kind chosen
/// changes nothing.
#[test]
fn the_cursors_rule_admits_what_the_chosen_kind_reads() {
    let mut tools = Tools::new();
    assert_eq!(tools.pick_kinds_choosing(VarKind::Point), PickKinds::Any);
    tools.open(ToolKind::Mate);
    let entities = [EntityKind::Face, EntityKind::Edge, EntityKind::Vertex];
    for (kind, _) in viewer::matetool::MATE_KINDS {
        let rule = tools.pick_kinds_choosing(kind);
        let admitted: Vec<EntityKind> = entities
            .into_iter()
            .filter(|&entity| PickedKind::of(entity).is_some_and(|picked| rule.admits(picked)))
            .collect();
        assert_eq!(
            admitted,
            viewer::matetool::reads(kind),
            "{kind:?}: the rule {rule:?} admits what the kind reads"
        );
        assert!(
            !rule.admits(PickedKind::Profile),
            "{kind:?}: no mate reads a profile, so the rule {rule:?} offers none"
        );
    }
    assert_eq!(
        viewer::matetool::reads(VarKind::Plane),
        [EntityKind::Face],
        "a plane reads a face alone"
    );
    assert!(
        viewer::matetool::reads(VarKind::Point).contains(&EntityKind::Vertex),
        "a point reads a vertex"
    );
}

/// **Two posts' vertical edges, made collinear** (`Axis` on straight
/// edges): each post's front-right edge (inside its silhouette from the
/// eye, so a cursor on it is over the body), each
/// picked by a cursor on its middle; after the commit post_b's edge is
/// drawn on post_a's line. A plane chosen over the same picks refuses
/// them typed, naming the side and the entity.
#[test]
fn two_straight_edges_mated_as_axes_are_collinear() {
    let tol = Tol::witness();
    let bench = asm::bench("matekinds-axis", tol);
    let mut session = asm::open_bench(&bench, tol);
    let index = asm::index_of(&session);
    let x_a = asm::POST_SECTION;
    let x_b = asm::POST_B_AT[0] + asm::POST_SECTION;
    let vertical_at = |x: f64| move |p: Point3<f64>| near(p.x, x) && near(p.y, 0.0);
    let on_b = edge_where(&index, bench.post_b, vertical_at(x_b));
    let on_a = edge_where(&index, bench.post_a, vertical_at(x_a));
    let camera = front_camera(Point3::new(0.04, 0.0, 0.025));
    let picks =
        [&on_b, &on_a].map(|edge| hovered(&session, &index, &camera, middle(edge), PickKinds::Any));
    let [Hovered::Edge(b_edge), Hovered::Edge(a_edge)] = &picks else {
        panic!("a cursor on each edge's middle picks the edge: {picks:?}")
    };
    assert_eq!((b_edge.node, a_edge.node), (bench.post_b, bench.post_a));
    let tools = mate_fed(&session, &index, &picks);
    asm::unpin(&mut session, bench.post_b);
    let (doc, _) = session.landed_pair().expect("landed");
    let tool = tools.mate().expect("the mate tool is open");
    match tool.proposal(doc, choice(VarKind::Plane, true)) {
        Err(MateToolError::KindDoesNotRead { side, kind, entity }) => {
            assert_eq!(
                (side, kind, entity),
                (MateSide::On, VarKind::Plane, EntityKind::Edge)
            );
        }
        other => panic!("a plane reads no edge: {other:?}"),
    }
    let proposal = tool
        .proposal(doc, choice(VarKind::Axis, false))
        .expect("two edges mate as axes");
    assert_eq!(proposal.placement, bench.post_b, "the on pick's copy");
    common::commit_mate(&mut session, proposal.op_into_target_space());
    let index = asm::index_of(&session);
    let ids = index.edges_of_target(b_edge);
    let [id] = ids[..] else {
        panic!("post_b draws its edge once: {ids:?}")
    };
    let moved = index.edge_polyline_for(id, &DisplayView::none());
    assert!(
        !moved.is_empty()
            && moved
                .iter()
                .all(|p| near(p.x, x_a) && near(p.y, 0.0) && p.z > -1.0e-6),
        "post_b's edge lies on post_a's line: {moved:?}"
    );
    std::fs::remove_dir_all(&bench.dir).expect("removable");
}

/// **A vertex on a vertex** (`Point`): post_b's top front-right corner
/// onto post_a's. The cursor on a corner picks the
/// vertex under a point's rule and an edge under the bare rule; a flip
/// on a point refuses (`NoSense`); the commit draws post_b's corner at
/// post_a's.
#[test]
fn a_vertex_mated_on_a_vertex_coincides() {
    let tol = Tol::witness();
    let bench = asm::bench("matekinds-point", tol);
    let mut session = asm::open_bench(&bench, tol);
    let index = asm::index_of(&session);
    let top = asm::POST_HEIGHT;
    let corner_b = Point3::new(asm::POST_B_AT[0] + asm::POST_SECTION, 0.0, top);
    let corner_a = Point3::new(asm::POST_SECTION, 0.0, top);
    let camera = front_camera(Point3::new(0.04, 0.0, 0.025));
    assert!(
        matches!(
            hovered(&session, &index, &camera, corner_b, PickKinds::Any),
            Hovered::Edge(_)
        ),
        "under the bare rule a corner is an edge's"
    );
    let picks = [corner_b, corner_a]
        .map(|at| hovered(&session, &index, &camera, at, PickKinds::WithVertices));
    let [Hovered::Vertex(b_vertex), Hovered::Vertex(a_vertex)] = &picks else {
        panic!("a cursor on each corner picks the vertex: {picks:?}")
    };
    assert_eq!((b_vertex.node, a_vertex.node), (bench.post_b, bench.post_a));
    assert_eq!(b_vertex.name.kind, EntityKind::Vertex);
    // The vertex is drawn where it was aimed at: its mark crosses there.
    let crossing = |index: &PickIndex, vertex| {
        let legs = index.vertex_legs_for(vertex, &DisplayView::none());
        let [from, to] = *legs.first().expect("a drawn vertex is marked");
        Point3::new(
            (from.x + to.x) / 2.0,
            (from.y + to.y) / 2.0,
            (from.z + to.z) / 2.0,
        )
    };
    let was = crossing(&index, b_vertex);
    assert!(
        near(was.x, corner_b.x) && near(was.y, 0.0) && near(was.z, top),
        "post_b's corner is marked where it is drawn: {was:?}"
    );
    let tools = mate_fed(&session, &index, &picks);
    asm::unpin(&mut session, bench.post_b);
    let (doc, _) = session.landed_pair().expect("landed");
    let tool = tools.mate().expect("the mate tool is open");
    assert!(
        matches!(
            tool.proposal(doc, choice(VarKind::Point, true)),
            Err(MateToolError::NoSense {
                kind: VarKind::Point
            })
        ),
        "a point has no sense to turn round"
    );
    let proposal = tool
        .proposal(doc, choice(VarKind::Point, false))
        .expect("two vertices mate as points");
    common::commit_mate(&mut session, proposal.op_into_target_space());
    let index = asm::index_of(&session);
    let now = crossing(&index, b_vertex);
    assert!(
        near(now.x, corner_a.x) && near(now.y, corner_a.y) && near(now.z, corner_a.z),
        "post_b's corner is drawn on post_a's: {now:?}"
    );
    std::fs::remove_dir_all(&bench.dir).expect("removable");
}

/// **An edge kept parallel to an edge** (`Direction`): post_b's
/// vertical front-left edge made parallel to the shelf's long top front
/// edge, so post_b lies along the shelf; after the commit post_b's edge
/// runs along `x`.
#[test]
fn an_edge_mated_as_a_direction_runs_parallel() {
    let tol = Tol::witness();
    let bench = asm::bench("matekinds-direction", tol);
    let mut session = asm::open_bench(&bench, tol);
    let index = asm::index_of(&session);
    let [_, shelf_y, shelf_z] = asm::SHELF_AT;
    let on_b = edge_where(&index, bench.post_b, |p| {
        near(p.x, asm::POST_B_AT[0]) && near(p.y, 0.0)
    });
    let along = edge_where(&index, bench.shelf_i, |p| {
        near(p.y, shelf_y) && near(p.z, shelf_z + asm::SHELF_THICKNESS)
    });
    let camera = front_camera(Point3::new(0.04, 0.04, 0.025));
    let picks = [
        middle(&on_b),
        Point3::new(0.04, shelf_y, shelf_z + asm::SHELF_THICKNESS),
    ]
    .map(|at| hovered(&session, &index, &camera, at, PickKinds::Any));
    let [Hovered::Edge(b_edge), Hovered::Edge(shelf_edge)] = &picks else {
        panic!("a cursor on each edge picks it: {picks:?}")
    };
    assert_eq!(shelf_edge.node, bench.shelf_i);
    assert!(along.len() >= 2, "the shelf's edge is drawn");
    let tools = mate_fed(&session, &index, &picks);
    asm::unpin(&mut session, bench.post_b);
    let (doc, _) = session.landed_pair().expect("landed");
    let proposal = tools
        .mate()
        .expect("the mate tool is open")
        .proposal(doc, choice(VarKind::Direction, false))
        .expect("two edges mate as directions");
    common::commit_mate(&mut session, proposal.op_into_target_space());
    let index = asm::index_of(&session);
    let ids = index.edges_of_target(b_edge);
    let moved = index.edge_polyline_for(ids[0], &DisplayView::none());
    let (a, b) = (moved[0], moved[moved.len() - 1]);
    assert!(
        near((b.x - a.x).abs(), asm::POST_HEIGHT) && near(b.y, a.y) && near(b.z, a.z),
        "post_b's edge runs along x: {a:?} → {b:?}"
    );
    std::fs::remove_dir_all(&bench.dir).expect("removable");
}

/// **A circular edge made coaxial with a cylinder's face** (`Axis` on
/// a circular edge and on a face's carrier): a small peg's top rim
/// onto a large peg's side, each peg's body placed in the world; after the commit the rim is drawn about the large peg's
/// axis.
#[test]
fn a_circular_edge_is_coaxial_with_a_cylinder_face() {
    const BIG: f64 = 0.01;
    const SMALL: f64 = 0.005;
    const SMALL_AT: [f64; 2] = [0.05, 0.03];
    let tol = Tol::witness();
    let mut session = DocSession::inline(
        pncad::document::Doc::empty_derived("mate-kinds-pegs", tol),
        tol,
    );
    let peg = |session: &mut DocSession, centre, radius, height| {
        let profile = common::session_insert(
            session,
            SessionOp::AddProfile {
                loops: vec![common::shape(&ProfileShape::Circle { centre, radius })],
            },
        );
        common::session_insert(
            session,
            SessionOp::AddExtrude {
                profile,
                distance: common::len(height),
            },
        )
    };
    let big = peg(&mut session, [0.0, 0.0], BIG, 0.02);
    let small = peg(&mut session, SMALL_AT, SMALL, 0.01);
    // A new body lands unplaced: each peg is placed in the world by
    // hand, as a person would, so the mate has copies to place.
    let doc = session.committed_doc().clone();
    let (doc, big_copy) = common::world_copy(&doc, big, tol);
    let (doc, small_copy) = common::world_copy(&doc, small, tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let index = asm::index_of(&session);
    let camera = front_camera(Point3::new(0.025, 0.015, 0.01));
    let rim_front = Point3::new(SMALL_AT[0], SMALL_AT[1] - SMALL, 0.01);
    let side_front = Point3::new(0.0, -BIG, 0.01);
    let picks = [
        hovered(&session, &index, &camera, rim_front, PickKinds::Any),
        hovered(&session, &index, &camera, side_front, PickKinds::FacesOnly),
    ];
    let [Hovered::Edge(rim), Hovered::Face(side)] = &picks else {
        panic!("the rim's front is an edge and the side's a face: {picks:?}")
    };
    assert_eq!((rim.node, side.node), (small_copy, big_copy));
    let tools = mate_fed(&session, &index, &picks);
    let (doc, _) = session.landed_pair().expect("landed");
    let proposal = tools
        .mate()
        .expect("the mate tool is open")
        .proposal(doc, choice(VarKind::Axis, false))
        .expect("a rim and a cylinder mate as axes");
    assert_eq!(proposal.placement, small_copy, "the rim's copy is placed");
    common::commit_mate(&mut session, proposal.op());
    let index = asm::index_of(&session);
    let ids = index.edges_of_target(rim);
    let moved = index.edge_polyline_for(ids[0], &DisplayView::none());
    assert!(
        moved.len() >= 2
            && moved
                .iter()
                .all(|p| ((p.x * p.x + p.y * p.y).sqrt() - SMALL).abs() < 1.0e-6),
        "the rim is drawn about the big peg's axis: {moved:?}"
    );
}
