//! **A profile side subdivided by a declared straight continuation,
//! swept, then used as an operand.**
//!
//! `line(len)` off a directed point, then `continue_to`, authors two
//! segments the author has declared to lie on one carrier. What each
//! sweep verb makes of that declaration, and what the boolean, the
//! merge ladder and the fillet then do with the body:
//!
//! - **extrude, revolve**: the two walls share ONE surface key (the
//!   cosurface run structure the sweep lowering decides per join), and
//!   the body is tier-3 valid — but the verb does not merge them, so a
//!   PLANAR pair of them reaches `topo`'s maximal-faces gate unmerged
//!   and the boolean refuses `NonMaximalFaces` at their shared edge.
//!   The structural rung merges them on request
//!   (`Body::merge_coplanar_faces`), after which the boolean runs.
//! - **loft**: each segment's wall is its own NURBS surface under its
//!   own key, so no rung merges them; the boolean refuses the body's
//!   spline edges before its gate is reached.
//! - **fillet**: the subdivided rim is a two-link chain whose joint is
//!   collinear, and the blend door refuses it as unbuilt junction
//!   carry-through, merged or not.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::FRAC_PI_2;

use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{ClosedLoop, Open, Profile, ProfileLoop, RawLoop, SketchPlane, Start};
use sweep::blend::BlendError;
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, loft_body, revolve};
use topo::{Body, BooleanError, EdgeKey, FaceKey, Operand, union, validate_closed};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// `[0,2]²` whose bottom side is authored as `line(1)` and then the
/// straight continuation to `(2, 0)`: five vertices, four corners, and
/// segments 0 and 1 declared to share one carrier.
fn subdivided_square(t: Tol) -> ClosedLoop<f64> {
    Open.at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(p2(2.0, 0.0), t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .line_to(Start, t)
        .unwrap()
}

fn subdivided_prism(t: Tol) -> sweep::Extruded<f64> {
    let lp: ProfileLoop<f64> = subdivided_square(t).into();
    assert_eq!(lp.vertices().len(), 5, "the continuation minted one vertex");
    let v = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(t)
        .unwrap();
    extrude(&v, Extrusion::Distance(2.0), t).unwrap()
}

/// An axis-aligned cube of side `s` with its low corner at `(x0, y0, z0)`.
fn cube_at(x0: f64, y0: f64, z0: f64, s: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        p2(x0, y0),
        p2(x0 + s, y0),
        p2(x0 + s, y0 + s),
        p2(x0, y0 + s),
    ]);
    let plane = SketchPlane::new(Affine3::from_parts(
        Mat3::identity(),
        Point3::new(0.0, 0.0, z0) - Point3::origin(),
    ));
    let v = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&v, Extrusion::Distance(s), Tol::witness())
        .unwrap()
        .body
}

fn key_of(body: &Body<f64>, f: FaceKey) -> topo::SurfaceKey {
    body.get_face(f).unwrap().surface
}

fn is_plane(body: &Body<f64>, f: FaceKey) -> bool {
    matches!(
        body.get_surface(key_of(body, f)),
        Some(geom::Surface::Plane { .. })
    )
}

fn edges_between(body: &Body<f64>, f: FaceKey, g: FaceKey) -> Vec<EdgeKey> {
    body.edges()
        .filter(|(_, e)| {
            let a = body.face_of_half_edge(e.he_plus);
            let b = body.face_of_half_edge(e.he_minus);
            (a == Some(f) && b == Some(g)) || (a == Some(g) && b == Some(f))
        })
        .map(|(k, _)| k)
        .collect()
}

fn volume(body: &Body<f64>, t: Tol) -> f64 {
    topo::mass_properties(body, t).unwrap().volume
}

/// **Extrude: one key, not merged, refused at the gate; merged on
/// request, the union runs.** The cube `[0.5,1.5]×[−0.5,0.5]×[0.5,1.5]`
/// crosses the subdivided wall `y = 0` across the continuation's strut.
#[test]
fn extruded_continuation_walls_share_a_key_and_refuse_until_merged() {
    let t = Tol::witness();
    let ex = subdivided_prism(t);
    let (w0, w1) = (ex.side_faces[0][0], ex.side_faces[0][1]);
    assert_eq!(
        key_of(&ex.body, w0),
        key_of(&ex.body, w1),
        "the declared continuation's two walls share one plane key"
    );
    assert!(is_plane(&ex.body, w0));
    assert_ne!(key_of(&ex.body, w1), key_of(&ex.body, ex.side_faces[0][2]));
    let interior = ex.strut_edges[0][1];
    assert_eq!(edges_between(&ex.body, w0, w1), vec![interior]);
    assert_eq!(validate_closed(&ex.body), Ok(()), "tier 2");
    assert_eq!(topo::validate_geometric(&ex.body, t), Ok(()), "tier 3");

    let cube = cube_at(0.5, -0.5, 0.5, 1.0);
    match union(&ex.body, &cube, t) {
        Err(BooleanError::NonMaximalFaces { operand, edge }) => {
            assert_eq!(operand, Operand::A);
            assert_eq!(edge, interior, "refused at the continuation's strut");
        }
        other => panic!("expected NonMaximalFaces at the strut, got {other:?}"),
    }

    let mut merged = ex.body.clone();
    let out = merged.merge_coplanar_faces(t).unwrap();
    assert!(out.skipped.is_empty(), "{:?}", out.skipped);
    assert_eq!(out.groups.len(), 1, "one structural run");
    assert_eq!(out.groups[0].killed_edges, vec![interior]);
    assert_eq!(topo::validate_geometric(&merged, t), Ok(()), "tier 3");
    assert!(
        (volume(&merged, t) - 8.0).abs() < 1e-12,
        "the merge is pure structure"
    );

    let r = union(&merged, &cube, t).expect("the merged operand is maximal-faced");
    let body = &r.body().expect("non-empty").body;
    assert_eq!(validate_closed(body), Ok(()), "tier 2");
    // [0,2]²×[0,2] plus the half of the cube outside it.
    assert!((volume(body, t) - 8.5).abs() < 1e-12, "{}", volume(body, t));
}

/// **Revolve, full and partial: the same branch.** The subdivided
/// square at `x ∈ [1, 3]` revolved about the sketch's y axis: its
/// subdivided bottom side sweeps to two same-key annulus planes (the
/// gate refuses them), its subdivided outer side to two same-key
/// cylinder bands (the gate's canonical maximal form).
#[test]
fn revolved_continuation_walls_share_a_key_and_refuse_until_merged() {
    let t = Tol::witness();
    let lp: ProfileLoop<f64> = Open
        .at(p2(1.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(p2(3.0, 0.0), t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(p2(3.0, 2.0), t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .line_to(Start, t)
        .unwrap()
        .into();
    let v = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(t)
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    // A cube across the annulus plane y = 0 over the split circle at
    // radius 2, clear of the axis.
    let cube = cube_at(1.5, -0.5, -0.5, 1.0);
    for rev in [Revolution::Full, Revolution::Partial(FRAC_PI_2)] {
        let r = revolve(&v, axis, rev, t).unwrap();
        let w = |j: usize| r.walls[0][j].expect("off-axis segment has a wall");
        assert_eq!(key_of(&r.body, w(0)), key_of(&r.body, w(1)), "{rev:?}");
        assert!(is_plane(&r.body, w(0)), "{rev:?}");
        assert_eq!(key_of(&r.body, w(2)), key_of(&r.body, w(3)), "{rev:?}");
        assert!(!is_plane(&r.body, w(2)), "{rev:?}");
        assert_eq!(topo::validate_geometric(&r.body, t), Ok(()), "{rev:?}");

        let split_circle = edges_between(&r.body, w(0), w(1));
        assert_eq!(split_circle.len(), 1, "{rev:?}");
        match union(&r.body, &cube, t) {
            Err(BooleanError::NonMaximalFaces { operand, edge }) => {
                assert_eq!(operand, Operand::A, "{rev:?}");
                assert_eq!(edge, split_circle[0], "{rev:?}: the annulus pair");
            }
            other => panic!("{rev:?}: expected NonMaximalFaces, got {other:?}"),
        }
    }
}

/// **Loft: one key per segment, so nothing merges.** Two copies of the
/// subdivided square stacked 2 apart: every wall is its own NURBS
/// surface, the structural rung finds no run, and the boolean refuses
/// the body's spline edges before its maximal-faces gate is reached.
#[test]
fn lofted_continuation_walls_carry_one_key_per_segment() {
    let t = Tol::witness();
    let lp: ProfileLoop<f64> = subdivided_square(t).into();
    let places: Vec<Affine3<f64>> = [0.0, 2.0]
        .iter()
        .map(|z| Affine3::from_parts(Mat3::identity(), Vec3::new(0.0, 0.0, *z)))
        .collect();
    let l = loft_body::<f64>(&[vec![lp.clone()], vec![lp]], &places, 1, t).unwrap();
    let (w0, w1) = (l.side_faces[0][0], l.side_faces[0][1]);
    assert_ne!(key_of(&l.body, w0), key_of(&l.body, w1));
    assert!(matches!(
        l.body.get_surface(key_of(&l.body, w0)),
        Some(geom::Surface::Nurbs(_))
    ));
    assert_eq!(topo::validate_geometric(&l.body, t), Ok(()), "tier 3");
    let mut m = l.body.clone();
    assert!(m.merge_coplanar_faces(t).unwrap().groups.is_empty());
    let err = union(&l.body, &cube_at(0.5, -0.5, 0.5, 1.0), t).unwrap_err();
    assert!(
        !matches!(err, BooleanError::NonMaximalFaces { .. }),
        "refused before the gate: {err:?}"
    );
}

/// **Fillet: the subdivided rim is a two-link chain.** Every edge of
/// the prism but the continuation's own (flat) strut, requested at
/// once, refuses as unbuilt junction carry-through — merged or not,
/// because the merge keeps the continuation's rim vertices. The plain
/// cube's twelve edges, the same request without the subdivision,
/// build.
#[test]
fn subdivided_rim_fillet_refuses_as_junction_carry_through() {
    let t = Tol::witness();
    let plain = cube_at(0.0, 0.0, 0.0, 2.0);
    let all: Vec<_> = plain.edges().map(|(k, _)| k).collect();
    let f = sweep::fillet::fillet_edges(&plain, &all, 0.25, t).unwrap();
    assert_eq!(topo::validate_geometric(&f.body, t), Ok(()));

    let ex = subdivided_prism(t);
    let interior = ex.strut_edges[0][1];
    let mut merged = ex.body.clone();
    merged.merge_coplanar_faces(t).unwrap();
    for (label, body) in [("split", &ex.body), ("merged", &merged)] {
        let req: Vec<_> = body
            .edges()
            .map(|(k, _)| k)
            .filter(|k| *k != interior)
            .collect();
        assert_eq!(req.len(), 14, "{label}: 12 cube edges + the split rims");
        let err = sweep::fillet::fillet_edges(body, &req, 0.25, t).unwrap_err();
        assert!(
            matches!(err.error, BlendError::UnsupportedChain { detail, .. }
                if detail.contains("junction carry-through")),
            "{label}: {err}"
        );
    }
}
