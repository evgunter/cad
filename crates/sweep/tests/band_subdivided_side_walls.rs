//! **A profile side subdivided by a declared straight continuation,
//! swept, then used as an operand.**
//!
//! `line(len)` off a directed point, then `continue_to`, authors two
//! segments the author has declared to lie on one carrier. What each
//! sweep verb makes of that declaration, and what the boolean, the
//! merge ladder and the fillet then do with the body:
//!
//! - **extrude, revolve**: the two segments are one run on one carrier
//!   (the cosurface verdict the sweep lowering decides per join), so
//!   the verb builds ONE wall over both (`crates/sweep/README.md`,
//!   "Walls: one per run"). Where a cap carries the profile the
//!   continuation's vertex stays, splitting the cap rim into collinear
//!   edges; nothing is left for the structural rung to merge, and the
//!   body is a boolean operand as built.
//! - **loft**: each segment's wall is its own NURBS surface under its
//!   own key, so no rung merges them; the boolean refuses the body's
//!   spline edges before its gate is reached.
//! - **fillet**: the subdivided rim is a two-link chain whose joint is
//!   collinear, and the blend door refuses it as unbuilt junction
//!   carry-through.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::FRAC_PI_2;

use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{ClosedLoop, Open, Profile, ProfileLoop, RawLoop, SketchPlane, Start};
use sweep::blend::BlendError;
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, loft_body, revolve};
use topo::{Body, BooleanError, EdgeKey, FaceKey, union, validate_closed};

/// `[0,2]²` whose bottom side is authored as `line(1)` and then the
/// straight continuation to `(2, 0)`: five vertices, four corners, and
/// segments 0 and 1 declared to share one carrier.
fn subdivided_square(t: Tol) -> ClosedLoop<f64> {
    Open.at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(Point2::new(2.0, 0.0), t)
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
        Point2::new(x0, y0),
        Point2::new(x0 + s, y0),
        Point2::new(x0 + s, y0 + s),
        Point2::new(x0, y0 + s),
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

fn volume(body: &Body<f64>, t: Tol) -> f64 {
    topo::mass_properties(body, t).unwrap().volume
}

/// **Extrude: one wall over the run, and the union runs as built.**
/// The cube `[0.5,1.5]×[−0.5,0.5]×[0.5,1.5]` crosses the subdivided wall
/// `y = 0` across the continuation's vertex.
#[test]
fn extruded_continuation_builds_one_wall_and_unions_as_built() {
    let t = Tol::witness();
    let ex = subdivided_prism(t);
    let sides = ex.side_faces();
    let (w0, w1) = (sides[0][0], sides[0][1]);
    assert_eq!(w0, w1, "the declared continuation sweeps one wall");
    assert!(is_plane(&ex.body, w0));
    assert_ne!(w0, sides[0][2]);
    assert_eq!(ex.walls[0].len(), 4, "four walls for five segments");
    let wall = &ex.walls[0][0];
    assert_eq!(wall.segments, vec![0, 1]);
    assert_eq!(
        ex.strut_edges()[0][1],
        None,
        "the continuation's vertex is a station: no strut"
    );
    // The station stays on both caps: each rim of the wall is two
    // collinear edges meeting at the continuation's vertex.
    for rims in [&wall.bottom_rims, &wall.top_rims] {
        assert_eq!(rims.len(), 2);
        let ends = |e: EdgeKey| {
            let edge = ex.body.get_edge(e).unwrap();
            let a = ex.body.get_half_edge(edge.he_plus).unwrap().start;
            let b = ex.body.get_half_edge(edge.he_minus).unwrap().start;
            [a, b]
        };
        let shared: Vec<_> = ends(rims[0])
            .into_iter()
            .filter(|v| ends(rims[1]).contains(v))
            .collect();
        assert_eq!(shared.len(), 1, "the station vertex joins the two rims");
    }
    assert_eq!(ex.body.vertices().count(), 10, "five vertices on each cap");
    assert_eq!(ex.body.faces().count(), 6);
    assert_eq!(validate_closed(&ex.body), Ok(()), "tier 2");
    assert_eq!(topo::validate_geometric(&ex.body, t), Ok(()), "tier 3");
    let mut merged = ex.body.clone();
    assert!(
        merged.merge_coplanar_faces(t).unwrap().groups.is_empty(),
        "nothing left for the structural rung to merge"
    );

    let cube = cube_at(0.5, -0.5, 0.5, 1.0);
    let r = union(&ex.body, &cube, t).expect("the extrusion is maximal-faced as built");
    let body = &r.body().expect("non-empty").body;
    assert_eq!(validate_closed(body), Ok(()), "tier 2");
    // [0,2]²×[0,2] plus the half of the cube outside it.
    assert!((volume(body, t) - 8.5).abs() < 1e-12, "{}", volume(body, t));
}

/// **Revolve, full and partial: the same branch.** The subdivided
/// square at `x ∈ [1, 3]` revolved about the sketch's y axis: its
/// subdivided bottom side sweeps to ONE annulus wall, its subdivided
/// outer side to ONE cylinder wall. The partial revolve's wedge caps
/// keep each continuation's vertex (a meridian vertex splitting the
/// cap's chain); the full revolve keeps no entity for it. The union
/// with a cube across the annulus runs as built.
#[test]
fn revolved_continuation_builds_one_wall_per_run_and_unions_as_built() {
    let t = Tol::witness();
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(1.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(Point2::new(3.0, 0.0), t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(Point2::new(3.0, 2.0), t)
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
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    // A cube across the annulus plane y = 0 over the continuation's
    // circle at radius 2, clear of the axis.
    let cube = cube_at(1.5, -0.5, -0.5, 1.0);
    for (rev, vertices) in [(Revolution::Full, 4), (Revolution::Partial(FRAC_PI_2), 12)] {
        let r = revolve(&v, axis, rev, t).unwrap();
        let walls = r.walls();
        let w = |j: usize| walls[0][j].expect("off-axis segment has a wall");
        assert_eq!(w(0), w(1), "{rev:?}: one annulus wall");
        assert!(is_plane(&r.body, w(0)), "{rev:?}");
        assert_eq!(w(2), w(3), "{rev:?}: one cylinder wall");
        assert!(!is_plane(&r.body, w(2)), "{rev:?}");
        assert_eq!(r.bands[0].len(), 4, "{rev:?}: four walls for six segments");
        assert_eq!(r.rims[0][1], None, "{rev:?}: no rim at a station");
        assert_eq!(r.rims[0][3], None, "{rev:?}: no rim at a station");
        assert_eq!(r.body.vertices().count(), vertices, "{rev:?}");
        assert_eq!(topo::validate_geometric(&r.body, t), Ok(()), "{rev:?}");
        let mut merged = r.body.clone();
        assert!(
            merged.merge_coplanar_faces(t).unwrap().groups.is_empty(),
            "{rev:?}: nothing left for the structural rung to merge"
        );
        let u = union(&r.body, &cube, t)
            .unwrap_or_else(|e| panic!("{rev:?}: the revolve is maximal-faced as built: {e:?}"));
        let body = &u.body().expect("non-empty").body;
        assert_eq!(validate_closed(body), Ok(()), "{rev:?}: tier 2");
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
/// the prism requested at once refuses as unbuilt junction
/// carry-through: the one wall keeps the continuation's rim vertices.
/// The plain cube's twelve edges, the same request without the
/// subdivision, build.
#[test]
fn subdivided_rim_fillet_refuses_as_junction_carry_through() {
    let t = Tol::witness();
    let plain = cube_at(0.0, 0.0, 0.0, 2.0);
    let all: Vec<_> = plain.edges().map(|(k, _)| k).collect();
    let f = sweep::fillet::fillet_edges(&plain, &all, 0.25, t).unwrap();
    assert_eq!(topo::validate_geometric(&f.body, t), Ok(()));

    let ex = subdivided_prism(t);
    let req: Vec<_> = ex.body.edges().map(|(k, _)| k).collect();
    assert_eq!(req.len(), 14, "12 cube edges + the split rims");
    let err = sweep::fillet::fillet_edges(&ex.body, &req, 0.25, t).unwrap_err();
    assert!(
        matches!(err.error, BlendError::UnsupportedChain { detail, .. }
            if detail.contains("junction carry-through")),
        "{err}"
    );
}
