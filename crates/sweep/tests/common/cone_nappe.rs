//! The cone-nappe fixtures and the face reader the SHELL-6 suites
//! share: two frustums that are each other's mirror about their apex,
//! the coned tube the per-chart door's rows use, and the corner walk
//! that reads a face's own stations.
//!
//! Body authoring plus one READER, which is why it is its own module
//! rather than a growth of [`super`]'s section vocabulary: the walk is
//! what three suites check a cone face WITH, and it was copied into
//! each of them before it came here.
//!
//! **Deliberately NOT absorbed** (the neighbour rule):
//! [`super::oracles`] keeps the frustum's closed-form volume — a truth
//! derived without the kernel routes there, and the wall's closed form
//! in `sf2b_r2_probes` is a second derivation that must not come here;
//! [`super::orient`]'s facing probe is a check of a body's
//! orientation, not of a chart's nappe, and neither reads the other;
//! `revolve_common` keeps `p2` and the revolve vocabulary at large,
//! and this module builds on it rather than restating it.

use geom::Surface;
use geom_core::{Band, Point2, Point3, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, FaceKey};

/// The wall thickness the SHELL-6 rows offset by.
pub const T: f64 = 1.0 / 128.0;
/// The frustums' height.
pub const H: f64 = 8.0 / 64.0;
/// Their wide radius.
pub const R_WIDE: f64 = 4.0 / 64.0;
/// Their narrow radius.
pub const R_NARROW: f64 = 2.0 / 64.0;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The band every row here decides against.
pub fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A full revolve of the meridian through `pts` about `+y`.
pub fn revolved(pts: &[(f64, f64)]) -> Body<f64> {
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::new(
            pts.iter()
                .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
                .collect(),
        )],
    )
    .validate(Tol::witness())
    .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the meridian revolves")
    .body
}

/// A frustum of height [`H`], `r0` at the base and `r1` at the top.
pub fn frustum(r0: f64, r1: f64) -> Body<f64> {
    revolved(&[(0.0, 0.0), (r0, 0.0), (r1, H), (0.0, H)])
}

/// Narrowing upward: the wall stands BELOW its apex, on the mirror
/// nappe.
pub fn mirror_frustum() -> Body<f64> {
    frustum(R_WIDE, R_NARROW)
}

/// Widening upward: the wall stands ABOVE its apex, on the opening
/// nappe.
pub fn opening_frustum() -> Body<f64> {
    frustum(R_NARROW, R_WIDE)
}

/// `verbs_offd`'s tube whose outer wall is a cone below its apex
/// (apex at `y = 0.9`, `tan α = 4/3`).
pub fn coned_tube() -> Body<f64> {
    revolved(&[(0.4, 0.0), (0.8, 0.0), (0.8, 0.3), (0.4, 0.6)])
}

/// The faces wearing a cone. A full revolve splits a wall into two
/// bands over ONE surface key, so a cone chart is a group.
pub fn cone_faces(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Cone { .. })))
        .map(|(k, _)| k)
        .collect()
}

/// `face`'s surface, cloned.
pub fn surface_of(body: &Body<f64>, face: FaceKey) -> Surface<f64> {
    body.get_surface(body.get_face(face).unwrap().surface)
        .unwrap()
        .clone()
}

/// Every corner of `face`, in `next` order over every loop — the very
/// points `topo::face_nappe` meters.
pub fn corners(body: &Body<f64>, face: FaceKey) -> Vec<Point3<f64>> {
    let data = body.get_face(face).unwrap();
    let mut out = Vec::new();
    for lk in core::iter::once(data.outer).chain(data.rings.iter().copied()) {
        let topo::LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
            continue;
        };
        for he in body.loop_cycle(first).unwrap() {
            let v = body.get_half_edge(he).unwrap().start;
            out.push(*body.get_point(body.get_vertex(v).unwrap().point).unwrap());
        }
    }
    out
}

/// `face`'s corner stations `(p − apex)·axis` on its own cone.
pub fn stations(body: &Body<f64>, face: FaceKey) -> Vec<f64> {
    let Surface::Cone { apex, axis, .. } = surface_of(body, face) else {
        panic!("stations: {face:?} does not wear a cone");
    };
    corners(body, face)
        .iter()
        .map(|p| (*p - apex).dot(axis))
        .collect()
}

/// One `ChartMove` per surface key: the axial door names every face of
/// the body, and a chart is moved once however many bands wear it.
pub fn chart_moves(body: &Body<f64>, d: f64) -> Vec<topo::ChartMove<f64>> {
    let mut moves: Vec<topo::ChartMove<f64>> = Vec::new();
    for (k, f) in body.faces() {
        match moves
            .iter_mut()
            .find(|m| body.get_face(m.faces[0]).unwrap().surface == f.surface)
        {
            Some(m) => m.faces.push(k),
            None => moves.push(topo::ChartMove {
                faces: vec![k],
                distance: d,
            }),
        }
    }
    moves
}

/// Re-attach every face of `group` to one cone whose apex sits at
/// `apex_y` on the axis, keeping the axis, half-angle and `u_ref` the
/// group already wears. The bodies this makes are geometric nonsense —
/// which is the point: they are operands whose nappe question has an
/// answer no revolve can build.
pub fn reanchor_cone(body: &mut Body<f64>, group: &[FaceKey], apex_y: f64) -> Surface<f64> {
    let Surface::Cone {
        axis,
        half_angle,
        u_ref,
        ..
    } = surface_of(body, group[0])
    else {
        panic!("reanchor_cone: the group does not wear a cone");
    };
    let moved = Surface::Cone {
        apex: Point3::new(0.0, apex_y, 0.0),
        axis,
        half_angle,
        u_ref,
    };
    let key = body
        .set_face_surface(group[0], topo::FaceSurface::New(moved.clone()))
        .expect("the face takes a re-anchored cone");
    for &other in &group[1..] {
        body.set_face_surface(other, topo::FaceSurface::Shared(key))
            .expect("its neighbours share it");
    }
    moved
}
