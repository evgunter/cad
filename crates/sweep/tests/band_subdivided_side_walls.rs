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
//! - **fillet, chamfer**: the subdivided rim is a two-link chain whose
//!   joint is collinear. Merged, both links lie on the same two faces
//!   and the blend carves them as one band across the joint; unmerged,
//!   the two halves lie on two wall faces and the door refuses the
//!   junction as links on different support faces.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::FRAC_PI_2;

use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{ClosedLoop, Open, Profile, ProfileLoop, RawLoop, SketchPlane, Start};
use sweep::blend::BlendError;
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, loft_body, revolve};
use topo::{Body, BooleanError, EdgeKey, FaceKey, Operand, union, validate_closed};

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

/// The blend radius the rows below request.
const R: f64 = 0.25;

/// **Fillet and chamfer: once merged, a subdivided rim is ONE band
/// across its joint.** Every edge of the merged prism — the twelve
/// cube edges, two of them split at the continuation's rim vertices —
/// blends as the plain cube does, the two halves of each split rim
/// carved as one band face whose trimlines carry the joint's feet. Its
/// volume is the plain cube's closed form for both verbs.
///
/// Unmerged, the same request refuses: the two halves of each rim lie
/// on two wall FACES the unrequested flat strut separates, and a band
/// through such a junction is not built.
#[test]
fn subdivided_rim_blends_as_one_band_once_merged() {
    let t = Tol::witness();
    let ex = subdivided_prism(t);
    let interior = ex.strut_edges[0][1];
    let split_req: Vec<_> = ex
        .body
        .edges()
        .map(|(k, _)| k)
        .filter(|k| *k != interior)
        .collect();
    assert_eq!(split_req.len(), 14, "12 cube edges + the split rims");
    let err = sweep::fillet::fillet_edges(&ex.body, &split_req, R, t).unwrap_err();
    assert!(
        matches!(err.error, BlendError::UnsupportedChain { detail, .. }
            if detail.contains("different support faces")),
        "split: {err}"
    );

    let mut merged = ex.body.clone();
    merged.merge_coplanar_faces(t).unwrap();
    let req: Vec<_> = merged.edges().map(|(k, _)| k).collect();
    assert_eq!(req.len(), 14, "the merge kills only the strut");
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
    let u = union(&f.body, &cube_at(5.0, 5.0, 5.0, 1.0), t).expect("the band is maximal");
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
