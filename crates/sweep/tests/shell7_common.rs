//! Helpers shared by the SHELL-7 suites (`shell7_seam_corner`,
//! `shell7_r2_probes`, `shell7_dump`, `shell7_r1_diff`): the fixtures
//! of the axial door's one-surface corner and the readers every row
//! uses. Authored across the unit's lane and its two review lanes.

#![allow(dead_code, clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, TubeWindow, revolve, tube_along_arc, tube_along_arc_hollow};
use topo::{Body, EdgeKey, FaceKey, ReplaceFaceError, ShellError, VertexKey};

pub fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

pub fn tol() -> Tol {
    Tol::witness()
}

/// A revolve of `lp` about the sketch `y` axis.
pub fn revolved(lp: ProfileLoop<f64>, turn: Revolution<f64>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        turn,
        tol(),
    )
    .expect("the meridian revolves")
    .body
}

/// A polyline meridian revolved.
pub fn polyline(pts: &[(f64, f64)], turn: Revolution<f64>) -> Body<f64> {
    revolved(
        ProfileLoop::new(
            pts.iter()
                .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
                .collect(),
        ),
        turn,
    )
}

/// The rectangle `[0, r] × [0, h]` revolved through `turn`.
pub fn wedge(r: f64, h: f64, turn: f64) -> Body<f64> {
    polyline(
        &[(0.0, 0.0), (r, 0.0), (r, h), (0.0, h)],
        Revolution::Partial(turn),
    )
}

/// The same rectangle, a full turn.
pub fn drum(r: f64, h: f64) -> Body<f64> {
    polyline(&[(0.0, 0.0), (r, 0.0), (r, h), (0.0, h)], Revolution::Full)
}

/// The tube door's full torus about `y`, seam at `u_ref = x`.
pub fn tube_torus(major: f64, minor: f64) -> Body<f64> {
    tube_along_arc::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        major,
        TubeWindow::Full,
        minor,
        tol(),
    )
    .expect("the solid torus builds")
    .body
}

/// Its hollow twin.
pub fn tube_torus_hollow(major: f64, minor: f64, wall: f64) -> Body<f64> {
    tube_along_arc_hollow::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        major,
        TubeWindow::Full,
        minor,
        wall,
        tol(),
    )
    .expect("the hollow torus builds")
    .body
}

/// `p` in the `(ρ, h)` half-plane of the `y` axis.
pub fn axial(p: Point3<f64>) -> (f64, f64) {
    ((p.x * p.x + p.z * p.z).sqrt(), p.y)
}

pub fn point(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    *body
        .get_point(body.get_vertex(v).expect("vertex").point)
        .expect("point")
}

pub fn carrier(body: &Body<f64>, e: EdgeKey) -> (Curve3<f64>, (f64, f64)) {
    let c = body
        .get_curve_geom(body.get_edge(e).expect("edge").curve)
        .and_then(|g| g.certified())
        .expect("a certified curve");
    (c.carrier().clone(), c.params())
}

pub fn face_of_he(body: &Body<f64>, he: topo::HalfEdgeKey) -> FaceKey {
    let lp = body.get_half_edge(he).unwrap().parent_loop;
    body.get_loop(lp).unwrap().face
}

/// A seam to the door: an edge whose two sides lie on one SURFACE,
/// whether or not they are one face.
pub fn same_surface(body: &Body<f64>, e: EdgeKey) -> bool {
    let d = body.get_edge(e).unwrap();
    let k = |he| body.get_face(face_of_he(body, he)).unwrap().surface;
    k(d.he_plus) == k(d.he_minus)
}

pub fn distinct_surfaces_at(body: &Body<f64>, v: VertexKey) -> usize {
    let em = body.get_vertex(v).unwrap().emanating.unwrap();
    let mut s: Vec<_> = body
        .vertex_orbit(em)
        .unwrap()
        .into_iter()
        .map(|he| body.get_face(face_of_he(body, he)).unwrap().surface)
        .collect();
    s.sort();
    s.dedup();
    s.len()
}

/// The one edge of `body` whose carrier is a LINE satisfying `pick`
/// (origin, direction, same-surface).
pub fn line_edge(body: &Body<f64>, pick: impl Fn(Point3<f64>, Vec3<f64>, bool) -> bool) -> EdgeKey {
    let hits: Vec<EdgeKey> = body
        .edges()
        .filter(|(e, _)| match carrier(body, *e).0 {
            Curve3::Line { origin, dir } => pick(origin, dir, same_surface(body, *e)),
            _ => false,
        })
        .map(|(e, _)| e)
        .collect();
    assert_eq!(hits.len(), 1, "exactly one such edge, got {hits:?}");
    hits[0]
}

/// Split `edge` at its parameter midpoint; the new vertex.
pub fn split_mid(body: &mut Body<f64>, edge: EdgeKey) -> VertexKey {
    let (_, (t0, t1)) = carrier(body, edge);
    body.split_edge(edge, (t0 + t1) * 0.5, tol())
        .expect("the edge splits")
        .vertex
}

pub fn corner_refusal(e: &ShellError<f64>) -> Option<(VertexKey, usize, &'static str)> {
    let ShellError::Face { error, .. } = e else {
        return None;
    };
    match **error {
        ReplaceFaceError::TogetherAxialCorner {
            vertex,
            surfaces,
            what,
        } => Some((vertex, surfaces, what)),
        _ => None,
    }
}

pub fn edge_refusal(e: &ShellError<f64>) -> Option<(EdgeKey, &'static str)> {
    let ShellError::Face { error, .. } = e else {
        return None;
    };
    match **error {
        ReplaceFaceError::TogetherAxialEdge { edge, what } => Some((edge, what)),
        _ => None,
    }
}

/// Every chart of `body` moved inward by `t` through the simultaneous
/// door — the moves `shell` builds, spelled at the door itself.
pub fn hollow_moves(body: &Body<f64>, t: f64) -> Vec<topo::ChartMove<f64>> {
    let mut charts: Vec<(topo::SurfaceKey, Vec<FaceKey>)> = Vec::new();
    for (k, f) in body.faces() {
        match charts.iter_mut().find(|(s, _)| *s == f.surface) {
            Some((_, v)) => v.push(k),
            None => charts.push((f.surface, vec![k])),
        }
    }
    charts
        .into_iter()
        .map(|(_, faces)| {
            let sense = body.get_face(faces[0]).expect("face").sense;
            topo::ChartMove {
                faces,
                distance: if sense { -t } else { t },
            }
        })
        .collect()
}
