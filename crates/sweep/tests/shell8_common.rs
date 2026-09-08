//! Helpers shared by the SHELL-8 suites (`shell8_multi_solid`,
//! `shell8_dump`, `shell8_r1_probes`, `shell8_r2_probes`): the
//! multi-solid fixtures and the readers every row uses. Authored across
//! the unit's lane and its two review lanes, and kept in ONE place —
//! the same helper spelled per suite is the same helper drifting per
//! suite.

#![allow(dead_code, clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Band, Point3, Tol, Vec3};
use topo::{Body, EdgeKey, FaceKey, ShellKey, SolidKey, SurfaceKey, VertexKey};

pub(crate) fn tol() -> Tol {
    Tol::witness()
}

pub(crate) fn band() -> Band {
    Band::linear(tol()).expect("a band")
}

/// `body` with `other` placed `by` beside it, as a second solid of one
/// body — the public disjoint-graft door, which is what a user
/// assembling two parts reaches for. Validated, since every row that
/// uses it is a claim about a valid operand.
pub(crate) fn beside_by(body: &Body<f64>, other: &Body<f64>, by: Vec3<f64>) -> Body<f64> {
    let out = beside_raw(body, other, by).0;
    assert_eq!(
        topo::validate_geometric(&out, tol()),
        Ok(()),
        "the multi-solid operand is valid"
    );
    out
}

/// [`beside_by`] without the validation, and with the placed copy's new
/// solid key — for the rows that build a deliberately odd operand.
pub(crate) fn beside_raw(
    body: &Body<f64>,
    other: &Body<f64>,
    by: Vec3<f64>,
) -> (Body<f64>, SolidKey) {
    let mut out = body.clone();
    let placed =
        topo::transform_rigid(other, &Affine3::translation(by), tol()).expect("a rigid map");
    let key = topo::graft_disjoint(&mut out, &placed, tol()).expect("the placed copy grafts");
    (out, key)
}

/// `other` placed `dx` along `+x` beside `body`.
pub(crate) fn beside(body: &Body<f64>, other: &Body<f64>, dx: f64) -> Body<f64> {
    beside_by(body, other, Vec3::new(dx, 0.0, 0.0))
}

pub(crate) fn volume(body: &Body<f64>) -> f64 {
    topo::mass_properties(body, tol()).expect("props").volume
}

/// The solid a face belongs to.
pub(crate) fn solid_of(body: &Body<f64>, face: FaceKey) -> SolidKey {
    let shell = body.get_face(face).unwrap().shell;
    body.get_shell(shell).unwrap().solid
}

/// The solid a vertex belongs to, through its emanating half-edge.
pub(crate) fn solid_of_vertex(body: &Body<f64>, vertex: VertexKey) -> SolidKey {
    let he = body.get_vertex(vertex).unwrap().emanating.unwrap();
    solid_of(body, face_of_he(body, he))
}

/// Every face of `solid`, in arena order.
pub(crate) fn faces_of(body: &Body<f64>, solid: SolidKey) -> Vec<FaceKey> {
    body.faces()
        .filter(|(k, _)| solid_of(body, *k) == solid)
        .map(|(k, _)| k)
        .collect()
}

/// The chart groups of `solid`: faces by surface key, in arena order.
pub(crate) fn charts_of(body: &Body<f64>, solid: SolidKey) -> Vec<Vec<FaceKey>> {
    let mut out: Vec<(SurfaceKey, Vec<FaceKey>)> = Vec::new();
    for face in faces_of(body, solid) {
        let key = body.get_face(face).unwrap().surface;
        match out.iter_mut().find(|(k, _)| *k == key) {
            Some((_, v)) => v.push(face),
            None => out.push((key, vec![face])),
        }
    }
    out.into_iter().map(|(_, v)| v).collect()
}

pub(crate) fn face_of_he(body: &Body<f64>, he: topo::HalfEdgeKey) -> FaceKey {
    let lp = body.get_half_edge(he).unwrap().parent_loop;
    body.get_loop(lp).unwrap().face
}

/// Every vertex point of `body`, in arena order.
pub(crate) fn points(body: &Body<f64>) -> Vec<(VertexKey, Point3<f64>)> {
    body.vertices()
        .map(|(k, v)| (k, *body.get_point(v.point).unwrap()))
        .collect()
}

/// The bit pattern of a point — the only comparison that says
/// "untouched" rather than "close".
pub(crate) fn bits(p: &Point3<f64>) -> (u64, u64, u64) {
    (p.x.to_bits(), p.y.to_bits(), p.z.to_bits())
}

/// A DEEP per-solid dump: every face's sense, rings and surface, every
/// edge's carrier, parameters and description, every vertex's point as
/// BITS. This is what "untouched" has to mean for a solid a door was
/// not asked about — a vertex-point comparison alone would miss a
/// re-authored edge description whose geometry is unchanged.
pub(crate) fn deep_dump(body: &Body<f64>, solid: SolidKey) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mine = faces_of(body, solid);
    for &f in &mine {
        let d = body.get_face(f).unwrap();
        out.push(format!(
            "face sense={} rings={} surface={:?}",
            d.sense,
            d.rings.len(),
            body.get_surface(d.surface)
        ));
    }
    for (k, e) in body.edges() {
        if !mine.contains(&face_of_he(body, e.he_plus)) {
            continue;
        }
        let c = body
            .get_curve_geom(e.curve)
            .and_then(topo::CurveGeom::certified)
            .unwrap();
        out.push(format!(
            "edge {k:?} carrier={:?} params={:?} description={:?}",
            c.carrier(),
            c.params(),
            c.description()
        ));
    }
    for (k, vx) in body.vertices() {
        let Some(em) = body.get_vertex(k).unwrap().emanating else {
            continue;
        };
        if !mine.contains(&face_of_he(body, em)) {
            continue;
        }
        out.push(format!(
            "vertex bits={:?}",
            bits(body.get_point(vx.point).unwrap())
        ));
    }
    out.sort();
    out
}

/// Every edge of `body` keyed by its own key, with the carrier,
/// parameters and description it carries — the reading that says
/// whether an edge was RE-AUTHORED, whatever the geometry came out as.
pub(crate) fn edge_rows(body: &Body<f64>) -> Vec<(EdgeKey, String)> {
    body.edges()
        .map(|(k, e)| {
            let c = body
                .get_curve_geom(e.curve)
                .and_then(topo::CurveGeom::certified)
                .unwrap();
            (
                k,
                format!(
                    "carrier={:?} params={:?} description={:?}",
                    c.carrier(),
                    c.params(),
                    c.description()
                ),
            )
        })
        .collect()
}

/// The whole CHART of `shell` whose plane is normal to `axis` (a unit
/// world direction) and sits at `value` along it — every face wearing
/// it, since a full revolve splits a cap into two half-discs and the
/// rim surgery lifts a chart as one.
pub(crate) fn cap(body: &Body<f64>, shell: ShellKey, axis: Vec3<f64>, value: f64) -> Vec<FaceKey> {
    for &face in &body.get_shell(shell).unwrap().faces {
        let f = body.get_face(face).unwrap();
        let Some(geom::Surface::Plane { origin, normal, .. }) = body.get_surface(f.surface) else {
            continue;
        };
        if normal.cross(axis).norm() > 1e-9 {
            continue;
        }
        if (Vec3::new(origin.x, origin.y, origin.z).dot(axis) - value).abs() < 1e-9 {
            let chart = f.surface;
            return body
                .faces()
                .filter(|(_, g)| g.surface == chart)
                .map(|(k, _)| k)
                .collect();
        }
    }
    panic!("no cap of {shell:?} normal to {axis:?} at {value}")
}

/// The whole chart of `solid` whose plane is normal to `+z` and sits at
/// `z`.
pub(crate) fn top_chart(body: &Body<f64>, solid: SolidKey, z: f64) -> Vec<FaceKey> {
    for face in faces_of(body, solid) {
        let f = body.get_face(face).unwrap();
        let Some(geom::Surface::Plane { origin, normal, .. }) = body.get_surface(f.surface) else {
            continue;
        };
        if normal.x.abs() > 1e-9 || normal.y.abs() > 1e-9 || (origin.z - z).abs() > 1e-9 {
            continue;
        }
        let chart = f.surface;
        return body
            .faces()
            .filter(|(_, g)| g.surface == chart)
            .map(|(k, _)| k)
            .collect();
    }
    panic!("no z = {z} cap on {solid:?}")
}

/// The `(outer, void)` shells of a two-shell solid, decided through the
/// shell classifier restricted to that solid's own shells.
pub(crate) fn outer_and_void_of(body: &Body<f64>, solid: SolidKey) -> (ShellKey, ShellKey) {
    let shells = body.get_solid(solid).unwrap().shells.clone();
    let roles = topo::classify_shells_of(body, &shells, tol()).expect("the solid classifies");
    let pick = |r: topo::ShellRole| {
        roles
            .iter()
            .find(|c| c.role == r)
            .unwrap_or_else(|| panic!("no {r:?} shell on {solid:?}"))
            .shell
    };
    (pick(topo::ShellRole::Outer), pick(topo::ShellRole::Void))
}
