//! **A link's reach skips the faces at its own two vertices, not at its
//! chain's.** A face at a link's own end is a plane end face lying on the
//! cap that ends the window, so it never meets the reach's inside; a
//! face at another link's end of the same chain lies on no bound of this
//! reach and is metered like any other.
//!
//! The witness is a prism on a thin trapezoid, its three top edges one
//! chain (`L0`, `L1`, `L2`): the back face is the cut-off end face of
//! `L0` and of `L2` and cuts through `L1`'s reach. Read at the meter,
//! since the door's support screen refuses the thin top first; the same
//! body wider is the control. The L-shaped end plate pins the item's own
//! candidate: its overhanging arm's faces are at no vertex of the chain,
//! so the reach meters them, and the end face's ring meter refuses the
//! body before the reach is read.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Band, Point2, Point3, Tol};
use sweep::blend::{BlendError, BlendRequest, fillet_edges};
use sweep::test_support::band_reach;
use topo::{Body, EntityId, FaceKey, validate_geometric};

use crate::common::cavity::{brick, cut, edges_with_corners, prism};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).expect("the witness band")
}

/// The plane of `face` as its unit normal and its offset along it.
fn plane_of(body: &Body<f64>, face: FaceKey) -> ([f64; 3], f64) {
    let s = body
        .get_face(face)
        .and_then(|f| body.get_surface(f.surface))
        .expect("the face's surface");
    let Surface::Plane { origin, normal, .. } = s else {
        panic!("a plane face, got {s:?}");
    };
    let n = normal.normalize();
    ([n.x, n.y, n.z], (*origin - Point3::origin()).dot(n))
}

/// Whether `face` lies on the plane `y = y0`.
fn on_y(body: &Body<f64>, face: FaceKey, y0: f64) -> bool {
    let ([nx, ny, nz], d) = plane_of(body, face);
    nx.abs() < 1e-12 && nz.abs() < 1e-12 && (d * ny - y0).abs() < 1e-12
}

/// The prism on the trapezoid `(0,0) (4,0) (4,w) (−1,w)` from `z = 0`
/// to `2`, and its three top edges but the back one: `L0` over the
/// oblique left face, `L1` over the front, `L2` over the right, turning
/// into each other at the front corners. Every vertex of the top face is
/// a vertex of the chain, so the strip screen inside the meter passes it
/// at any width.
fn trapezoid(w: f64) -> (Body<f64>, Vec<topo::EdgeKey>) {
    let body = prism(
        &[
            Point2::new(0.0, 0.0),
            Point2::new(4.0, 0.0),
            Point2::new(4.0, w),
            Point2::new(-1.0, w),
        ],
        0.0,
        2.0,
    );
    let top = |p: Point3<f64>| (p.z - 2.0).abs() < 1e-12;
    let back = edges_with_corners(&body, |p| top(p) && (p.y - w).abs() < 1e-12);
    let edges: Vec<_> = edges_with_corners(&body, top)
        .into_iter()
        .filter(|e| !back.contains(e))
        .collect();
    assert_eq!(edges.len(), 3, "w = {w}: the chain's three top edges");
    (body, edges)
}

/// At `r = 0.5` `L1`'s band removes material up to `y = r` under the top
/// face, so a back face at `y = 0.2` or `0.45` lies in it, at `0.6`
/// clear of it. The meter names the back face — the end face of `L0`
/// and `L2`, at no vertex of `L1`; the door refuses the thin top at its
/// support screen first; the wide body builds.
#[test]
fn a_cut_off_face_at_another_links_end_is_metered_against_the_link() {
    let r = 0.5;
    for w in [0.2, 0.45] {
        let (body, edges) = trapezoid(w);
        let req = BlendRequest {
            body: &body,
            edges: edges.clone(),
            size: r,
        };
        let e = band_reach(&req, band()).expect_err("the back face cuts L1's band");
        let BlendError::FaceClearance {
            at: EntityId::Face(f),
            ..
        } = e
        else {
            panic!("w = {w}: the meter refuses a face: {e:?}");
        };
        assert!(
            on_y(&body, f, w),
            "w = {w}: the face named is the back face"
        );
        let door = fillet_edges(&body, &edges, r, tol())
            .map(|_| ())
            .map_err(|e| e.error);
        assert!(
            matches!(door, Err(BlendError::FaceClearanceUncertified { .. })),
            "w = {w}: the door refuses the thin top at its support screen: {door:?}"
        );
    }
    let (body, edges) = trapezoid(0.6);
    let req = BlendRequest {
        body: &body,
        edges: edges.clone(),
        size: r,
    };
    if let Err(e) = band_reach(&req, band()) {
        panic!("w = 0.6: clear at the meter: {e:?}");
    }
    let f = fillet_edges(&body, &edges, r, tol())
        .unwrap_or_else(|e| panic!("w = 0.6: the wide body builds: {:?}", e.error));
    assert_eq!(
        validate_geometric(&f.body, tol()),
        Ok(()),
        "w = 0.6: tier 3"
    );
}

/// A pocket's floor–wall edge `x ∈ [0, 4]` between two end plates, and
/// an arm `x ∈ [2.5, 4.5]`, `y ∈ [y0, 1.5]`, `z ∈ [h, 1.5]` standing out
/// of the far plate over the band. At `r = 1` the band adds material up
/// to `y ≈ 0.286` at `z = 0.3`: the arm at `(0.1, 0.3)` lies in it, at
/// `(0.5, 0.6)` clear. The arm's faces are at no vertex of the chain, so
/// the meter refuses one of them as a measurement; the door refuses the
/// arm's ring on the end plate first, at the end face's own meter. Clear
/// of the band, and with no arm, the body builds.
#[test]
fn an_l_shaped_end_plates_arm_over_the_band_is_metered_and_refused() {
    let p = Point3::new;
    let r = 1.0;
    let pocket = cut(
        "pocket",
        &brick(p(-1.0, -1.0, -1.0), p(5.0, 3.0, 3.0)),
        &brick(p(0.0, 0.0, 0.0), p(4.0, 3.5, 3.5)),
    );
    let with_arm = |y0: f64, h: f64| {
        let t = tol();
        let a = sweep::test_support::finished("pocket", pocket.clone(), t);
        let b = sweep::test_support::finished("arm", brick(p(2.5, y0, h), p(4.5, 1.5, 1.5)), t);
        topo::union(&a, &b, t)
            .expect("the union succeeds")
            .body()
            .expect("the union leaves material")
            .body
            .clone()
            .into_body()
    };
    let edge = |body: &Body<f64>| {
        let e = edges_with_corners(body, |q| {
            q.y.abs() < 1e-12 && q.z.abs() < 1e-12 && (-0.5..4.5).contains(&q.x)
        });
        assert_eq!(e.len(), 1, "the floor–wall edge");
        e
    };

    let body = with_arm(0.1, 0.3);
    let edges = edge(&body);
    let req = BlendRequest {
        body: &body,
        edges: edges.clone(),
        size: r,
    };
    let e = band_reach(&req, band()).expect_err("the arm stands in the band");
    let BlendError::FaceClearance {
        at: EntityId::Face(f),
        bounded: false,
        ..
    } = e
    else {
        panic!("in: the meter measures an arm face in the band: {e:?}");
    };
    let ([nx, _, _], d) = plane_of(&body, f);
    assert!(
        !(nx.abs() > 1.0 - 1e-12 && (d * nx - 4.0).abs() < 1e-12),
        "in: the face named is the arm's, not the end plate's"
    );
    let door = fillet_edges(&body, &edges, r, tol())
        .map(|_| ())
        .map_err(|e| e.error);
    assert!(
        matches!(door, Err(BlendError::RingClearance { bounded: false, .. })),
        "in: the end face's ring meter refuses the arm's ring first: {door:?}"
    );

    for (what, body) in [("clear", with_arm(0.5, 0.6)), ("no arm", pocket.clone())] {
        let edges = edge(&body);
        let f = fillet_edges(&body, &edges, r, tol())
            .unwrap_or_else(|e| panic!("{what}: builds: {:?}", e.error));
        assert_eq!(validate_geometric(&f.body, tol()), Ok(()), "{what}: tier 3");
    }
}
