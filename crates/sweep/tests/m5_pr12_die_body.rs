//! **The die body** (M5 PR 12 §4): filleting every edge of a box at a
//! constant radius produces a tier-3-valid rounded solid.
//!
//! One acceptance row, asserted in the order a reader would check it
//! by hand: the three validator tiers, the Euler counts, the closed
//! forms for volume and area, a watertight tessellation, and — the
//! claim the whole prefer-intrinsic story rests on — that every blend
//! and corner edge carries `TangentIntersection`, not a `MappedCurve`
//! pushforward of the construction that happened to mint it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_brep::EdgeGeometry;
use geom_core::Band;
use sweep::fillet::{FilletError, Filleted, fillet_edges};
use sweep::test_support::cube;
use topo::{Body, EdgeKey, FaceKey};
use geom_core::Tol;

fn band() -> Band {
    let tol = Tol::witness().get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

fn all_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    body.edges().map(|(k, _)| k).collect()
}

fn die(l: f64, r: f64) -> Filleted<f64> {
    let body = cube(l, Tol::witness());
    let edges = all_edges(&body);
    assert_eq!(edges.len(), 12, "a box has twelve edges");
    fillet_edges(&body, &edges, r, band(), Tol::witness()).expect("the die body")
}

/// The acceptance row: the whole rounded die, top to bottom.
#[test]
fn filleting_every_edge_of_a_box_yields_a_tier3_valid_rounded_solid() {
    let (l, r) = (1.0, 0.15);
    let f = die(l, r);

    // 1. The three validator tiers, coarsest first.
    assert_eq!(topo::validate(&f.body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&f.body), Ok(()), "tier 2");
    assert_eq!(topo::validate_geometric(&f.body, Tol::witness()), Ok(()), "tier 3");

    // 2. The Euler counts: 6 shrunk planes + 12 quarter-cylinders + 8
    // sphere octants; 24 trimlines + 24 corner arcs; 3 feet per box
    // vertex. 26 − 48 + 24 = 2.
    let (faces, edges, verts) = (
        f.body.faces().count(),
        f.body.edges().count(),
        f.body.vertices().count(),
    );
    assert_eq!(
        (faces, edges, verts),
        (26, 48, 24),
        "the die's Euler counts"
    );
    assert_eq!(f.blend_faces.len(), 12, "one blend per box edge");
    assert_eq!(f.corner_faces.len(), 8, "one octant per box vertex");

    // 3. The closed forms: core + 6 slabs + 12 quarter-cylinders + 8
    // octants (which sum to one whole ball).
    let core = l - 2.0 * r;
    let volume = core.powi(3)
        + 6.0 * r * core.powi(2)
        + 12.0 * (PI * r * r / 4.0) * core
        + (4.0 / 3.0) * PI * r.powi(3);
    let area = 6.0 * core.powi(2) + 12.0 * (PI * r / 2.0) * core + 4.0 * PI * r * r;
    let props = topo::mass_properties(&f.body, Tol::witness()).unwrap();
    assert!(
        (props.volume - volume).abs() <= 1e-9 * volume,
        "volume {} vs closed form {volume}",
        props.volume
    );
    assert!(
        (props.surface_area - area).abs() <= 1e-9 * area,
        "area {} vs closed form {area}",
        props.surface_area
    );

    // 4. Watertight under tessellation.
    let mesh = mesh::tessellate(&f.body, 1e-2, Tol::witness()).expect("the die tessellates");
    mesh::validate::check_mesh(&mesh).expect("the die's mesh is watertight");

    // 5. Prefer-intrinsic, from birth: every edge of every blend and
    // corner face is a stored TANGENTIAL contact locus.
    let mut checked = 0usize;
    for &face in f.blend_faces.iter().chain(&f.corner_faces) {
        for edge in face_edges(&f.body, face) {
            let curve = f.body.get_edge(edge).unwrap().curve;
            let described = *f
                .body
                .get_curve_geom(curve)
                .unwrap()
                .certified()
                .unwrap()
                .description();
            assert!(
                matches!(described, EdgeGeometry::TangentIntersection { .. }),
                "edge {edge:?} of face {face:?} is {described:?}, not a tangential contact locus",
            );
            checked += 1;
        }
    }
    assert_eq!(checked, 12 * 4 + 8 * 3, "every blend/corner boundary edge");
}

/// The Euler-count identity holds at a second radius too — the row
/// above is not a fixture coincidence at r = 0.15.
#[test]
fn the_die_is_tier3_valid_at_a_second_radius() {
    let (l, r) = (2.0, 0.4);
    let f = die(l, r);
    assert_eq!(topo::validate_geometric(&f.body, Tol::witness()), Ok(()), "tier 3");
    let core = l - 2.0 * r;
    let volume = core.powi(3)
        + 6.0 * r * core.powi(2)
        + 12.0 * (PI * r * r / 4.0) * core
        + (4.0 / 3.0) * PI * r.powi(3);
    let props = topo::mass_properties(&f.body, Tol::witness()).unwrap();
    assert!(
        (props.volume - volume).abs() <= 1e-9 * volume,
        "volume {} vs closed form {volume}",
        props.volume
    );
}

/// A partially-requested corner (a run-out) is outside the assembly
/// front door: it refuses TYPED, naming what is not implemented,
/// rather than half-building.
#[test]
fn a_subset_of_the_edges_refuses_at_the_assembly_front_door() {
    let body = cube(1.0, Tol::witness());
    let edges = all_edges(&body);
    let err = fillet_edges(&body, &edges[..1], 0.15, band(), Tol::witness())
        .expect_err("one edge of a box leaves its corners partly requested");
    assert!(
        matches!(err, FilletError::UnsupportedRunOut { .. }),
        "expected the assembly front-door refusal, got {err}",
    );
    assert!(
        err.to_string().contains("not implemented"),
        "the refusal names the missing front door: {err}",
    );
}

fn face_edges(body: &Body<f64>, face: FaceKey) -> Vec<EdgeKey> {
    let outer = body.get_face(face).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("a blend face's boundary is not a cycle");
    };
    body.loop_cycle(first)
        .unwrap()
        .into_iter()
        .map(|he| body.get_half_edge(he).unwrap().edge)
        .collect()
}
