//! M5 S1 review CONTROLS, adopted (fix pass): the discrimination
//! probes that attributed MAJOR-1 to the merge's provisional ring
//! designation rather than to tessellation or the pre-existing
//! ring-face paths — kept as pins so the attribution stays checked.
//!
//! - the SAME final solid as the glued bridge, built by a plain
//!   transversal subtract (no REST lane), meshes clean — the defect
//!   was lane-door-reached, not a tessellator bug;
//! - the pre-existing interior-pillar ring path (PR 5.5 vintage,
//!   ring minted by the seam machinery, not by a hole-closing merge)
//!   meshes clean — the defect was merge-made, not ring-faces-in-
//!   general;
//! - the ring-sense census: on every ring-carrying planar face of
//!   both bodies, the outer loop winds positively and the ring
//!   negatively (what tier-3 check 6 now enforces globally).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{flush_declarations, prism_z};
use topo::{Body, BooleanResult, mass_properties, subtract, union_with};
use geom_core::Tol;

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    prism_z::<f64>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

fn checked_mesh_volume(body: &Body<f64>) -> f64 {
    let mesh = mesh::tessellate(body, 1e-2, Tol::witness()).expect("tessellate");
    mesh::validate::check_mesh(&mesh).expect("watertight, consistently oriented");
    mesh::validate::signed_volume(&mesh)
}

/// Signed area of a cycle loop projected on `n` (the reviewer's
/// census helper, assertion form).
fn loop_signed_area(body: &Body<f64>, l: topo::LoopKey, n: geom_core::Vec3<f64>) -> f64 {
    let topo::LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary else {
        return 0.0;
    };
    let cyc = body.loop_cycle(first).unwrap();
    let pts: Vec<geom_core::Point3<f64>> = cyc
        .iter()
        .map(|&he| {
            let v = body.get_half_edge(he).unwrap().start;
            *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
        })
        .collect();
    let o = pts[0];
    let mut acc = geom_core::Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    for i in 1..pts.len() - 1 {
        let a = pts[i] - o;
        let b = pts[i + 1] - o;
        acc = acc + a.cross(b);
    }
    0.5 * acc.dot(n)
}

/// Every ring-carrying planar face has a positively-wound outer loop
/// and negatively-wound rings; returns how many ring faces were seen.
fn assert_ring_senses(body: &Body<f64>, label: &str) -> usize {
    let mut seen = 0;
    for (k, f) in body.faces() {
        if f.rings.is_empty() {
            continue;
        }
        let n = match body.get_surface(f.surface) {
            Some(&topo::Surface::Plane { normal, .. }) => normal,
            _ => continue,
        };
        seen += 1;
        let outer = loop_signed_area(body, f.outer, n);
        assert!(outer > 0.0, "{label} face {k:?}: outer area {outer}");
        for &r in &f.rings {
            let ring = loop_signed_area(body, r, n);
            assert!(ring < 0.0, "{label} face {k:?}: ring area {ring}");
        }
    }
    seen
}

/// Control 1: the same final solid as the glued bridge, via a plain
/// transversal subtract — no REST lane, no hole-closing merge.
#[test]
fn control_same_shape_via_subtract() {
    let big = brick((0.0, 3.0), (0.0, 1.0), (0.0, 2.0));
    let notch = brick((1.0, 2.0), (-0.5, 1.5), (1.0, 1.5));
    let BooleanResult::Body(s) = subtract(&big, &notch, Tol::witness()).unwrap() else {
        panic!("control subtract yields a body");
    };
    assert_eq!(mass_properties(&s.body, Tol::witness()).unwrap().volume, 5.5);
    let v = checked_mesh_volume(&s.body);
    assert!(((v - 5.5) / 5.5).abs() < 1e-9, "control mesh volume {v}");
}

/// Control 2: the pre-existing interior-pillar ring path (the seam
/// machinery's ring, not a merge-closed hole) meshes clean, with the
/// ring senses correct.
#[test]
fn control_ring_face_from_interior_pillar() {
    let a = brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick((0.75, 1.25), (0.75, 1.25), (2.0, 3.0));
    let g = match union_with(&a, &b, &flush_declarations(&a, &b), Tol::witness()).unwrap() {
        BooleanResult::Body(g) => g,
        BooleanResult::Empty => panic!("pillar union cannot be empty"),
    };
    assert!(assert_ring_senses(&g.body, "pillar") >= 1);
    let exact = mass_properties(&g.body, Tol::witness()).unwrap().volume;
    assert_eq!(exact, 8.25);
    let v = checked_mesh_volume(&g.body);
    assert!(((v - exact) / exact).abs() < 1e-9, "pillar mesh volume {v}");
}

/// Control 3 (the isolating census): the glued bridge's ring faces
/// carry correct senses post-fix — pre-fix this census showed
/// outer −0.5 / ring +6.0 on both merged side faces.
#[test]
fn ring_sense_bridge_census() {
    let a = brick((0.0, 3.0), (0.0, 1.0), (0.0, 1.0));
    let blank = brick((0.0, 3.0), (0.0, 1.0), (1.0, 2.0));
    let notch = brick((1.0, 2.0), (-0.5, 1.5), (0.5, 1.5));
    let BooleanResult::Body(bb) = subtract(&blank, &notch, Tol::witness()).unwrap() else {
        panic!("bridge subtract yields a body");
    };
    let g = match union_with(&a, &bb.body, &flush_declarations(&a, &bb.body), Tol::witness()).unwrap() {
        BooleanResult::Body(g) => g,
        BooleanResult::Empty => panic!("bridge union cannot be empty"),
    };
    assert_eq!(assert_ring_senses(&g.body, "bridge"), 2);
}
