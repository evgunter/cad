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
//! - **fillet, chamfer**: the subdivided rim is a two-link chain whose
//!   joint is collinear. Both links lie on the same two faces (the one
//!   wall and the cap), so the blend carves them as one band across the
//!   joint, as built.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::FRAC_PI_2;
use sweep::ExtrudeSide;

use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{ClosedLoop, Open, Profile, ProfileLoop, RawLoop, SketchPlane, Start};
use sweep::test_support::finished;
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, loft_body, revolve};
use topo::{AtRestBody, Body, BooleanError, EdgeKey, FaceKey, union, validate_closed};

use crate::common::oracles;

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
    extrude(
        &v,
        Extrusion::Distance {
            depth: 2.0,
            side: ExtrudeSide::Along,
        },
        t,
    )
    .unwrap()
}

/// An axis-aligned cube of side `s` with its low corner at `(x0, y0, z0)`.
fn cube_at(x0: f64, y0: f64, z0: f64, s: f64) -> AtRestBody<f64> {
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
    let cube = extrude(
        &v,
        Extrusion::Distance {
            depth: s,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap()
    .body;
    finished("the cube", cube, Tol::witness())
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

/// The one face of `body` all of whose vertices satisfy `on`.
fn face_on(body: &Body<f64>, on: impl Fn(Point3<f64>) -> bool) -> FaceKey {
    let hits: Vec<FaceKey> = body
        .faces()
        .map(|(f, _)| f)
        .filter(|f| {
            body.vertices().all(|(v, vd)| {
                let touches = body.faces_of_vertex(v).is_some_and(|fs| fs.contains(f));
                !touches || on(*body.get_point(vd.point).unwrap())
            })
        })
        .collect();
    assert_eq!(hits.len(), 1, "exactly one face lies there");
    hits[0]
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
    let prism = finished("the subdivided prism", ex.body.clone(), t);
    let r = union(&prism, &cube, t).expect("the extrusion is maximal-faced as built");
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
        let revolved = finished("the subdivided revolve", r.body.clone(), t);
        let u = union(&revolved, &cube, t)
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
    let loft = finished("the subdivided loft", l.body.clone(), t);
    let err = union(&loft, &cube_at(0.5, -0.5, 0.5, 1.0), t).unwrap_err();
    assert!(
        !matches!(err, BooleanError::NonMaximalFaces { .. }),
        "refused before the gate: {err:?}"
    );
}

/// The blend radius the rows below request.
const R: f64 = 0.25;

/// **Fillet and chamfer: a subdivided rim is ONE band across its
/// joint.** Every edge of the prism as built — the twelve cube edges,
/// two of them split at the continuation's rim vertices — blends as
/// the plain cube does, the two halves of each split rim carved as one
/// band face whose trimlines carry the joint's feet. Its volume is the
/// plain cube's closed form for both verbs. (The extrusion builds one
/// wall over the run, so both halves already lie on the same two faces;
/// no merge is needed first.)
#[test]
fn subdivided_rim_blends_as_one_band_as_built() {
    let t = Tol::witness();
    let ex = subdivided_prism(t);
    // One wall over the run, as built: there is no strut at the
    // continuation and nothing for the merge to do.
    let mut merged = ex.body.clone();
    assert!(merged.merge_coplanar_faces(t).unwrap().groups.is_empty());
    let req: Vec<_> = merged.edges().map(|(k, _)| k).collect();
    assert_eq!(req.len(), 14, "12 cube edges + the split rims");
    let joints: Vec<topo::VertexKey> = merged
        .vertices()
        .map(|(v, _)| v)
        .filter(|v| merged.edges_of_vertex(*v).is_some_and(|es| es.len() == 2))
        .collect();
    assert_eq!(joints.len(), 2, "the continuation's two rim vertices");

    let f = sweep::fillet::fillet_edges(&merged, &req, R, t).expect("the merged prism fillets");
    assert_eq!(validate_closed(&f.body), Ok(()), "fillet: tier 2");
    assert_eq!(
        topo::validate_geometric(&f.body, t),
        Ok(()),
        "fillet: tier 3"
    );
    // The plain cube's 26 faces, 48 edges and 24 vertices, plus each
    // joint's two feet, each splitting one trimline.
    assert_eq!(f.body.faces().count(), 26, "fillet: faces");
    assert_eq!(f.body.edges().count(), 52, "fillet: edges");
    assert_eq!(f.body.vertices().count(), 28, "fillet: vertices");
    assert_eq!(
        f.blend_faces.len(),
        12,
        "one band per chain, as on the cube"
    );
    assert_eq!(f.corner_faces.len(), 8);
    let rec = f.naming.as_ref().expect("birth records");
    assert_eq!(rec.joined_blends.len(), 2, "the two split rims");
    for (band, edges) in &rec.joined_blends {
        assert_eq!(edges.len(), 2, "one band over both halves");
        assert!(
            matches!(
                f.body.get_surface(key_of(&f.body, *band)),
                Some(geom::Surface::Cylinder { .. })
            ),
            "a straight band is one cylinder"
        );
    }
    for v in &joints {
        assert!(rec.dead.vertices.contains(v), "the joint is retired");
        assert_eq!(
            rec.feet.iter().filter(|(_, src, _)| src == v).count(),
            2,
            "a joint leaves a foot on each support"
        );
    }
    sweep::test_support::assert_naming_totality(&merged, &f, &req, "merged fillet");
    // A filleted cube of side `a` is the shrunk cube swept by the ball.
    let want = oracles::rounded_box_volume(2.0 - 2.0 * R, R);
    let got = volume(&f.body, t);
    assert!(
        (got - want).abs() <= 1e-12 * want,
        "fillet: {got} vs {want}"
    );
    // And the result is a maximal-faced operand: the boolean's gates —
    // F7's among them — pass it, and a disjoint union adds the cube.
    let filleted = finished("the filleted prism", f.body.clone(), t);
    let u = union(&filleted, &cube_at(5.0, 5.0, 5.0, 1.0), t).expect("the band is maximal");
    let got = volume(&u.body().expect("non-empty").body, t);
    assert!(
        (got - (want + 1.0)).abs() <= 1e-12 * want,
        "fillet ∪ far cube: {got} vs {}",
        want + 1.0
    );

    let c = sweep::chamfer::chamfer_edges(&merged, &req, R, t).expect("the merged prism chamfers");
    assert_eq!(validate_closed(&c.body), Ok(()), "chamfer: tier 2");
    assert_eq!(
        topo::validate_geometric(&c.body, t),
        Ok(()),
        "chamfer: tier 3"
    );
    assert_eq!(c.body.faces().count(), 26, "chamfer: faces");
    assert_eq!(c.body.edges().count(), 52, "chamfer: edges");
    assert_eq!(c.body.vertices().count(), 28, "chamfer: vertices");
    let want = oracles::chamfered_cube_volume(2.0, R);
    let got = volume(&c.body, t);
    assert!(
        (got - want).abs() <= 1e-12 * want,
        "chamfer: {got} vs {want}"
    );
}

/// **A boolean's merged faces carry the same joint.** Two flush unit
/// cubes unioned into a `2 × 1 × 1` box, their touching faces and
/// coplanar sides declared: each long side is one face, but the merge
/// keeps the vertices where the operands' rims met, so
/// each of the four long edges is two collinear links on the same two
/// faces. Every edge fillets, each long edge as one band, at the
/// rounded box's closed form.
#[test]
fn a_union_of_flush_cubes_fillets_its_split_rims_as_one_band() {
    let t = Tol::witness();
    let (ca, cb) = (cube_at(0.0, 0.0, 0.0, 1.0), cube_at(1.0, 0.0, 0.0, 1.0));
    // The touching pair (`Rest`), and the four pairs of coplanar sides
    // (continuations) the union's merge stage glues once declared.
    let x1 = |p: Point3<f64>| p.x == 1.0;
    let mut decls = topo::BooleanDeclarations::none();
    decls.coincident_faces.push(topo::FacePairDeclaration::rest(
        face_on(&ca, x1),
        face_on(&cb, x1),
    ));
    let sides: [fn(Point3<f64>) -> bool; 4] = [
        |p| p.y == 0.0,
        |p| p.y == 1.0,
        |p| p.z == 0.0,
        |p| p.z == 1.0,
    ];
    for side in sides {
        decls
            .coincident_faces
            .push(topo::FacePairDeclaration::continuation(
                face_on(&ca, side),
                face_on(&cb, side),
            ));
    }
    let r = topo::union_with(&ca, &cb, &decls, t).expect("flush cubes union");
    let body = &r.body().expect("non-empty").body;
    assert_eq!(body.faces().count(), 6, "the merged box has six faces");
    let req: Vec<_> = body.edges().map(|(k, _)| k).collect();
    assert_eq!(req.len(), 16, "12 box edges, the four long ones split");

    let f = sweep::fillet::fillet_edges(body, &req, R, t).expect("the union fillets");
    assert_eq!(topo::validate_geometric(&f.body, t), Ok(()), "tier 3");
    assert_eq!(f.blend_faces.len(), 12, "one band per box edge");
    let rec = f.naming.as_ref().expect("birth records");
    assert_eq!(rec.joined_blends.len(), 4, "the four long edges");
    sweep::test_support::assert_naming_totality(body, &f, &req, "union fillet");
    // The shrunk box `a × b × c` swept by the ball: core, six slabs,
    // twelve quarter-cylinders summing to `π r² (a + b + c)`, and one
    // ball's worth of octants.
    let (a, b, c) = (2.0 - 2.0 * R, 1.0 - 2.0 * R, 1.0 - 2.0 * R);
    let pi = core::f64::consts::PI;
    let want = a * b * c
        + 2.0 * R * (a * b + b * c + c * a)
        + pi * R * R * (a + b + c)
        + (4.0 / 3.0) * pi * R.powi(3);
    let got = volume(&f.body, t);
    assert!((got - want).abs() <= 1e-12 * want, "{got} vs {want}");
}
