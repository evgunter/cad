//! Review probes for PR 5116 (round 1): which doors reach the two new
//! `Breach` kinds, and what each refusal reads like there.

use geom::Surface;
use geom_core::{Band, Point3, Tol, Vec3};
use sweep::blend::battery::{BlendRequest, run_battery_for};
use sweep::blend::build::fillet_edges;
use sweep::blend::{BlendError, BlendKind, Breach};
use sweep::test_support::{all_links, cube};

const L: f64 = 1.0;

fn tol() -> Tol {
    Tol::witness()
}

fn null_strut_cube() -> (topo::Body<f64>, topo::EdgeKey) {
    let mut body: topo::Body<f64> = cube(L, tol());
    let v = body.vertices().next().map(|(k, _)| k).expect("a vertex");
    let he = body.get_vertex(v).and_then(|d| d.emanating).expect("a fan");
    let null = body
        .mev_null(
            topo::MevSite::Fan { he1: he, he2: he },
            topo::NewVertexSide::Below,
        )
        .expect("a null strut off a cube corner")
        .edge;
    (body, null)
}

/// The public battery entry takes a bare `Body`, so no tier-2 gate
/// stands before it: requesting a null edge reaches `link_carrier` in
/// `resolve_link`. Pins what it now refuses (it was `ChainNotConnected`
/// on the merge base).
#[test]
fn public_battery_on_a_requested_null_edge() {
    let (body, null) = null_strut_cube();
    let band = Band::linear(tol()).expect("band");
    for kind in [BlendKind::Fillet, BlendKind::Chamfer] {
        let req = BlendRequest {
            body: &body,
            edges: vec![null],
            size: 0.1 * L,
        };
        let Err(e) = run_battery_for(&req, band, kind) else {
            panic!("{kind:?}: a null edge cannot be blended");
        };
        eprintln!("PROBE battery null edge {kind:?}: {e:?}\n  => {e}");
        assert!(
            matches!(
                e,
                BlendError::BodyNotIntact {
                    breach: Breach::Scaffolding,
                    ..
                } | BlendError::ChainNotConnected { .. }
            ),
            "{kind:?}: {e:?}"
        );
    }
}

/// The door's gate: a body carrying a null strut never becomes an
/// `AtRestBody`, so `fillet_edges` cannot reach `Scaffolding`.
#[test]
fn at_rest_gate_refuses_the_null_strut() {
    let (body, _) = null_strut_cube();
    let r = topo::AtRestBody::validate(body, tol());
    eprintln!("PROBE at-rest null strut: {:?}", r.as_ref().err());
    assert!(r.is_err(), "a null strut is scaffolding at rest");
}

fn degenerate_cylinder_cube() -> (topo::Body<f64>, topo::EdgeKey, topo::FaceKey) {
    let mut body = cube(L, tol());
    let link = all_links(&body, tol()).into_iter().next().expect("links");
    let off = body
        .faces()
        .map(|(k, _)| k)
        .find(|f| *f != link.face_a && *f != link.face_b)
        .expect("an off face");
    // Lifts RechartUnvouched: the degenerate chart is the probe's premise.
    body.set_face_surface_unvouched_for_tests(
        off,
        topo::FaceSurface::New {
            surface: Surface::Cylinder {
                origin: Point3::origin(),
                axis: Vec3::zero(),
                radius: L,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            sense: true,
        },
    )
    .expect("the face takes the degenerate chart");
    (body, link.edge, off)
}

/// Does the at-rest gate refuse a zero-axis cylinder face? If it
/// admits it, the door reaches `Degenerate` (or something earlier).
#[test]
fn door_on_a_zero_axis_cylinder_face() {
    let (body, edge, off) = degenerate_cylinder_cube();
    match topo::AtRestBody::validate(body, tol()) {
        Err(errs) => eprintln!("PROBE at-rest zero-axis cylinder refused: {errs:?}"),
        Ok(rest) => {
            let r = fillet_edges(&rest, &[edge], 0.1 * L, tol());
            let e = r
                .expect_err("a degenerate face cannot be filleted beside")
                .error;
            eprintln!("PROBE door zero-axis cylinder (off face {off:?}): {e:?}\n  => {e}");
        }
    }
}

/// The public battery on the degenerate face: the battery runs before
/// the band reach, so which refusal does a caller see?
#[test]
fn public_battery_on_a_zero_axis_cylinder_face() {
    let (body, edge, _) = degenerate_cylinder_cube();
    let band = Band::linear(tol()).expect("band");
    let req = BlendRequest {
        body: &body,
        edges: vec![edge],
        size: 0.1 * L,
    };
    let r = run_battery_for(&req, band, BlendKind::Fillet);
    eprintln!("PROBE battery zero-axis cylinder: {:?}", r.as_ref().err());
}
