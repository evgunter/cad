//! R1 review probes for PR 1361 (issue 303, signed_volume recentring).
//! Authored by R1; adopted onto the branch by the fix pass as the
//! permanent record of the review's measurements. Each probe prints
//! its measured digits. Fix-pass edit (disclosed in PR 1361): the e2e
//! probe tolerates typed upstream refusals, which its offset fixtures
//! hit at the ε = 1e-12 CI band.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use mesh::validate::{check_mesh, signed_volume};
use mesh::{FacePatch, Mesh, tessellate};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::Body;

fn prism_on(plane: SketchPlane<f64>, poly: &[(f64, f64)], h: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon(poly.iter().map(|&(x, y)| Point2::new(x, y)));
    let vp = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("profile validation");
    extrude(&vp, Extrusion::Distance(h), Tol::witness())
        .expect("extrude")
        .body
}

fn plane_at(offset: f64) -> SketchPlane<f64> {
    SketchPlane::from_frame(
        Point3::new(offset, offset, offset),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    )
}

/// The OLD spelling: world-origin-anchored fold (pre-fix behavior).
fn origin_fold(m: &Mesh) -> f64 {
    let mut six_v = 0.0;
    for patch in &m.patches {
        for tri in &patch.triangles {
            let a = m.positions[tri[0] as usize];
            let b = m.positions[tri[1] as usize];
            let c = m.positions[tri[2] as usize];
            let (a, b, c) = (
                a - Point3::origin(),
                b - Point3::origin(),
                c - Point3::origin(),
            );
            six_v += a.dot(b.cross(c));
        }
    }
    six_v / 6.0
}

fn hand_mesh(positions: Vec<Point3<f64>>, triangles: Vec<[u32; 3]>) -> Mesh {
    let body = prism_on(
        SketchPlane::xy(),
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        1.0,
    );
    let fk = body.faces().next().unwrap().0;
    Mesh {
        positions,
        patches: vec![FacePatch {
            face: fk,
            triangles,
        }],
        boundaries: Vec::new(),
    }
}

/// Open tetra (one face removed) at two placements: did the recentring
/// change the open-mesh answer? (Claim 1's premise attack.)
#[test]
fn r1_open_mesh_answer_changed() {
    let open = |ox: f64| {
        hand_mesh(
            vec![
                Point3::new(ox, ox, ox),
                Point3::new(ox + 1.0, ox, ox),
                Point3::new(ox, ox + 1.0, ox),
                Point3::new(ox, ox, ox + 1.0),
            ],
            // Drop face [1,2,3]: mesh has a boundary.
            vec![[0, 2, 1], [0, 1, 3], [0, 3, 2]],
        )
    };
    for ox in [0.0, 100.0] {
        let m = open(ox);
        assert!(check_mesh(&m).is_err(), "must be open");
        println!(
            "open tetra at {ox}: old(origin-anchored) = {:.6}, new(bbox) = {:.6}",
            origin_fold(&m),
            signed_volume(&m)
        );
    }
}

/// A dyadic axis-aligned box under the OLD fold shows no drift at
/// offsets (pairwise-exact cancellation) — the PR's non-dyadic-prism
/// reasoning, measured. (Claim 2.)
#[test]
fn r1_dyadic_box_hides_defect() {
    let poly = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let mesh_at = |plane| {
        let m = tessellate(&prism_on(plane, &poly, 1.0), 1e-2, Tol::witness()).expect("tessellate");
        assert_eq!(check_mesh(&m), Ok(()));
        m
    };
    let v0 = origin_fold(&mesh_at(SketchPlane::xy()));
    for offset in [1.0e3, 1.0e6, 1.0e8] {
        let v = origin_fold(&mesh_at(plane_at(offset)));
        println!(
            "dyadic box OLD fold at {offset:e}: v = {v}, drift = {:e}",
            ((v - v0) / v0).abs()
        );
    }
}

/// bbox-centre arithmetic at extreme placements. (Claim 3.)
#[test]
fn r1_bbox_extremes() {
    // Tetra edge 1e90 at 1e100: commensurate, huge magnitudes.
    let t = |o: f64, e: f64| {
        hand_mesh(
            vec![
                Point3::new(o, o, o),
                Point3::new(o + e, o, o),
                Point3::new(o, o + e, o),
                Point3::new(o, o, o + e),
            ],
            vec![[0, 2, 1], [0, 1, 3], [0, 3, 2], [1, 2, 3]],
        )
    };
    let v = signed_volume(&t(1.0e100, 1.0e90));
    let exact = 1.0e270 / 6.0;
    println!(
        "tetra e=1e90 at 1e100: v = {v:e}, rel err = {:e}",
        ((v - exact) / exact).abs()
    );
    // Mixed-sign extents, extent sum finite: fine.
    let v2 = signed_volume(&t(-0.5e308, 1.0e308));
    println!("tetra spanning -0.5e308..0.5e308: v = {v2:e} (overflow domain)");
    // hi - lo overflows f64: what comes out?
    let m = hand_mesh(
        vec![
            Point3::new(-1.2e308, 0.0, 0.0),
            Point3::new(1.2e308, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
        ],
        vec![[0, 2, 1], [0, 1, 3], [0, 3, 2], [1, 2, 3]],
    );
    println!("bbox extent > f64::MAX: v = {:e}", signed_volume(&m));
}

/// E2E: my own body (non-convex L prism), public tessellation door,
/// volume read near and far. (The brief's required exercise.) A
/// placement whose BUILD refuses typed at the current ε band is
/// printed and skipped — at ε = 1e-12 the offset fixture refuses at
/// extrude, which is the door's loud path, not this probe's subject.
#[test]
fn r1_e2e_l_prism_near_far() {
    let poly = [
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let read = |plane, label: &str| -> Option<f64> {
        let lp = ProfileLoop::polygon(poly.iter().map(|&(x, y)| Point2::new(x, y)));
        let vp = match Profile::new(plane, vec![lp]).validate(Tol::witness()) {
            Ok(v) => v,
            Err(e) => {
                println!("L prism {label}: typed refusal at profile validation: {e:?}");
                return None;
            }
        };
        let body = match extrude(&vp, Extrusion::Distance(0.4), Tol::witness()) {
            Ok(x) => x.body,
            Err(e) => {
                println!("L prism {label}: typed refusal at extrude: {e:?}");
                return None;
            }
        };
        let m = match tessellate(&body, 1e-2, Tol::witness()) {
            Ok(m) => m,
            Err(e) => {
                println!("L prism {label}: typed refusal at tessellate: {e:?}");
                return None;
            }
        };
        assert_eq!(check_mesh(&m), Ok(()));
        Some(signed_volume(&m))
    };
    let near = read(SketchPlane::xy(), "near");
    let far = read(plane_at(1.0e5), "far(1e5)");
    if let Some(near) = near {
        assert!((near - 1.2).abs() < 1e-12, "near volume {near}");
        if let Some(far) = far {
            let rel = ((far - near) / near).abs();
            println!("L prism: near = {near}, far(1e5) = {far}, rel drift = {rel:e}");
            assert!(rel < 1e-9, "far drift {rel:e}");
        }
    }
    // At 2e7 the DOOR refuses typed (cap-plane newell residual at
    // ulp(2e7) scale) — a consumer there sees a refusal, not a number.
    let e = extrude(
        &Profile::new(
            plane_at(2.0e7),
            vec![ProfileLoop::polygon(
                poly.iter().map(|&(x, y)| Point2::new(x, y)),
            )],
        )
        .validate(Tol::witness())
        .expect("profile validation"),
        Extrusion::Distance(0.4),
        Tol::witness(),
    )
    .err();
    println!("L prism at 2e7: extrude refusal = {e:?}");
}
