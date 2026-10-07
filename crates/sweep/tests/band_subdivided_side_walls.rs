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
//!   "Walls: one per run"), and each cap carries the run as one rim
//!   edge: the continuation's vertex has no entity (maximal edges).
//!   Nothing is left for the structural rung to merge, and the body is a
//!   boolean operand as built.
//! - **loft**: each segment's wall is its own NURBS surface under its
//!   own key, so no rung merges them; the boolean refuses the body's
//!   spline edges before its gate is reached.
//! - **a rim split by hand** (`split_edge` at the station, the shape a
//!   boolean's cut leaves): the boolean's output stage joins it back,
//!   and the fillet and the chamfer carve its two collinear links as
//!   one band across the joint, since both lie on the same two faces.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::FRAC_PI_2;
use sweep::ExtrudeSide;

use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{ClosedLoop, Open, Profile, ProfileLoop, RawLoop, SketchPlane, Start};
use sweep::test_support::finished;
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, loft_body, revolve};
use topo::{AtRestBody, Body, BooleanError, EdgeKey, FaceKey, union, validate_closed};

use crate::common::oracles;
use crate::common::stations::{cut_stations, station_vertices};

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

/// `body` with both rims of `wall`, a wall over the bottom side
/// `y = 0` of a prism of height 2, cut at each of `xs` (`common::stations`).
fn cut_wall(body: Body<f64>, wall: &sweep::SideWall, xs: &[f64], t: Tol) -> Body<f64> {
    let at = |z: f64| {
        xs.iter()
            .map(|&x| Point3::new(x, 0.0, z))
            .collect::<Vec<_>>()
    };
    let body = cut_stations(body, wall.bottom_rim, &at(0.0), t);
    cut_stations(body, wall.top_rim, &at(2.0), t)
}

/// [`subdivided_prism`] with its station cut back into both rims of
/// the run's wall.
fn stationed_prism(t: Tol) -> Body<f64> {
    let ex = subdivided_prism(t);
    let wall = &ex.walls[0][0];
    assert_eq!(wall.segments, vec![0, 1]);
    cut_wall(ex.body, wall, &[1.0], t)
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
            body.vertex_points().all(|(v, p)| {
                let touches = body.faces_of_vertex(v).is_some_and(|fs| fs.contains(f));
                !touches || on(p)
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
    // The station has no entity: each rim of the wall is one edge
    // over the run, ending at the wall's two struts.
    let strut_ends = |e: EdgeKey| {
        let edge = ex.body.get_edge(e).unwrap();
        [edge.he_plus, edge.he_minus].map(|h| ex.body.get_half_edge(h).unwrap().start)
    };
    let next = &ex.walls[0][1];
    for rim in [wall.bottom_rim, wall.top_rim] {
        let ends = strut_ends(rim);
        let on = |strut: EdgeKey| {
            ends.iter()
                .filter(|v| strut_ends(strut).contains(v))
                .count()
        };
        assert_eq!(
            (on(wall.strut), on(next.strut)),
            (1, 1),
            "the rim runs from the wall's strut to the next wall's"
        );
    }
    assert_eq!(
        topo::joinable_vertices(
            &ex.body,
            geom_core::Band::linear(geom_core::Tol::witness()).unwrap()
        )
        .unwrap(),
        vec![],
        "no station vertex on either cap"
    );
    assert_eq!(station_vertices(&ex.body), vec![]);
    assert_eq!(ex.body.vertices().count(), 8, "four vertices on each cap");
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

/// **A single-operand result has maximal edges.** The stationed prism
/// unioned with a cube strictly inside it: no boundary crosses, so the
/// union is the prism's own material, answered by the single-operand
/// fallback. The prism carries a station vertex on both cap rims, and
/// the fallback's output stage joins both: the result is a 2 × 2 × 2
/// box with 8 vertices and 12 edges. Red when the fallback skips the
/// join.
#[test]
fn a_union_answered_by_one_operand_joins_its_station_vertices() {
    let t = Tol::witness();
    let prism = finished("the stationed prism", stationed_prism(t), t);
    assert_eq!(
        topo::joinable_vertices(
            &prism,
            geom_core::Band::linear(geom_core::Tol::witness()).unwrap()
        )
        .unwrap()
        .len(),
        2,
        "the station on each cap rim"
    );
    assert_eq!(station_vertices(&prism).len(), 2, "the reader finds both");
    let cube = cube_at(0.5, 0.5, 0.5, 1.0);
    let r = union(&prism, &cube, t).expect("the cube lies inside the prism");
    let out = r.body().expect("non-empty");
    assert_eq!(out.kind, topo::BooleanResultKind::OperandA);
    assert_eq!(out.naming.edge_joins.len(), 2, "both stations joined");
    assert_eq!(
        topo::joinable_vertices(
            &out.body,
            geom_core::Band::linear(geom_core::Tol::witness()).unwrap()
        )
        .unwrap(),
        vec![]
    );
    assert_eq!(out.body.vertices().count(), 8);
    assert_eq!(out.body.edges().count(), 12);
    assert_eq!(validate_closed(&out.body), Ok(()), "tier 2");
    assert!((volume(&out.body, t) - 8.0).abs() < 1e-12);
}

/// `[0,3] × [0,2]` whose bottom side is authored as `line(1)` and two
/// straight continuations, to `(2, 0)` and `(3, 0)`, extruded by 2,
/// with both stations cut back into both rims of the run's wall: each
/// cap rim along the bottom side is three collinear edges.
fn twice_stationed_prism(t: Tol) -> Body<f64> {
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(Point2::new(2.0, 0.0), t)
        .unwrap()
        .continue_to(Point2::new(3.0, 0.0), t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(3.0, t)
        .unwrap()
        .line_to(Start, t)
        .unwrap()
        .into();
    assert_eq!(lp.vertices().len(), 6, "two continuations, two stations");
    let v = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(t)
        .unwrap();
    let ex = extrude(
        &v,
        Extrusion::Distance {
            depth: 2.0,
            side: ExtrudeSide::Along,
        },
        t,
    )
    .unwrap();
    let wall = &ex.walls[0][0];
    assert_eq!(wall.segments, vec![0, 1, 2]);
    cut_wall(ex.body, wall, &[1.0, 2.0], t)
}

/// **A seam joined twice reads through both joins.** The twice
/// stationed prism unioned with a cube strictly inside it: the
/// fallback's output stage joins each cap rim's three collinear edges
/// into one, two joins per rim, the second's `gone` the first's `kept`.
/// `BooleanNaming::joined_edge` takes every edge any join killed to the
/// live edge holding it, through both joins. Red when `joined_edge`
/// stops after one hop.
#[test]
fn a_rim_joined_twice_reads_through_both_joins() {
    let t = Tol::witness();
    let prism = finished("the twice stationed prism", twice_stationed_prism(t), t);
    assert_eq!(
        topo::joinable_vertices(
            &prism,
            geom_core::Band::linear(geom_core::Tol::witness()).unwrap()
        )
        .unwrap()
        .len(),
        4,
        "two stations per cap rim"
    );
    let cube = cube_at(0.5, 0.5, 0.5, 1.0);
    let r = union(&prism, &cube, t).expect("the cube lies inside the prism");
    let out = r.body().expect("non-empty");
    let joins = &out.naming.edge_joins;
    assert_eq!(joins.len(), 4, "every station joined");
    assert_eq!(
        topo::joinable_vertices(
            &out.body,
            geom_core::Band::linear(geom_core::Tol::witness()).unwrap()
        )
        .unwrap(),
        vec![]
    );
    assert_eq!(out.body.edges().count(), 12, "a box");
    assert!(
        joins
            .iter()
            .enumerate()
            .any(|(i, j)| joins[..i].iter().any(|earlier| earlier.kept == j.gone)),
        "some rim is joined twice through one edge: {joins:?}"
    );
    for j in joins {
        for e in [j.gone, j.kept] {
            let live = out.naming.joined_edge(e);
            assert!(
                out.body.get_edge(live).is_some(),
                "{e:?} reads to a dead edge {live:?} through {joins:?}"
            );
        }
    }
}

/// **Revolve, full and partial: the same branch.** The subdivided
/// square at `x ∈ [1, 3]` revolved about the sketch's y axis: its
/// subdivided bottom side sweeps to ONE annulus wall, its subdivided
/// outer side to ONE cylinder wall. Neither revolve keeps an entity for
/// a continuation's vertex: the partial revolve's wedge caps carry each
/// run as one meridian edge. The union with a cube across the annulus
/// runs as built.
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
    for (rev, vertices) in [(Revolution::Full, 4), (Revolution::Partial(FRAC_PI_2), 8)] {
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
        assert_eq!(
            topo::joinable_vertices(
                &r.body,
                geom_core::Band::linear(geom_core::Tol::witness()).unwrap()
            )
            .unwrap(),
            vec![],
            "{rev:?}"
        );
        assert_eq!(station_vertices(&r.body), vec![], "{rev:?}");
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

/// **Fillet and chamfer: a split rim is ONE band across its joint.**
/// Every edge of the stationed prism — the twelve cube edges, two of
/// them split at the station's rim vertices — blends as the plain cube
/// does, the two halves of each split rim carved as one band face whose
/// trimlines carry the joint's feet. Its volume is the plain cube's
/// closed form for both verbs. (The extrusion builds one wall over the
/// run, so both halves already lie on the same two faces; no merge is
/// needed first.)
#[test]
fn split_rim_blends_as_one_band() {
    let t = Tol::witness();
    // One wall over the run, as built: there is no strut at the
    // continuation and nothing for the merge to do.
    let mut merged = stationed_prism(t);
    assert!(merged.merge_coplanar_faces(t).unwrap().groups.is_empty());
    let req: Vec<_> = merged.edges().map(|(k, _)| k).collect();
    assert_eq!(req.len(), 14, "12 cube edges + the split rims");
    let joints: Vec<topo::VertexKey> = merged
        .vertices()
        .map(|(v, _)| v)
        .filter(|v| merged.edges_of_vertex(*v).is_some_and(|es| es.len() == 2))
        .collect();
    assert_eq!(joints.len(), 2, "the station's two rim vertices");

    let f = sweep::fillet::fillet_edges(&sweep::test_support::at_rest(&merged, t), &req, R, t)
        .expect("the merged prism fillets");
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

    let c = sweep::chamfer::chamfer_edges(&sweep::test_support::at_rest(&merged, t), &req, R, t)
        .expect("the merged prism chamfers");
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

/// **A joint and two cut-offs on one band**: only the split rim's two
/// halves on the top cap. They join through the station vertex as
/// one band, and each end, where the rim meets an unrequested side
/// edge, is cut off at the side wall, at the prism closed form over the
/// rim's whole length `2`.
#[test]
fn a_joined_band_is_cut_off_at_both_ends() {
    let t = Tol::witness();
    let body = sweep::test_support::finished("body", stationed_prism(t), t);
    let half = |a: f64, b: f64| {
        body.edges()
            .map(|(k, _)| k)
            .find(|&e| {
                let he = body.get_edge(e).unwrap().he_plus;
                let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                let (s, f) = (
                    p(body.get_half_edge(he).unwrap().start),
                    p(body.half_edge_end(he).unwrap()),
                );
                let on = |q: Point3<f64>, x: f64| (q - Point3::new(x, 0.0, 2.0)).norm() < 1e-12;
                (on(s, a) && on(f, b)) || (on(s, b) && on(f, a))
            })
            .expect("a half of the split top rim")
    };
    let req = [half(0.0, 1.0), half(1.0, 2.0)];
    for (verb, section) in [
        ("fillet", (1.0 - core::f64::consts::FRAC_PI_4) * R * R),
        ("chamfer", R * R / 2.0),
    ] {
        let out = match verb {
            "fillet" => sweep::fillet::fillet_edges(&body, &req, R, t),
            _ => sweep::chamfer::chamfer_edges(&body, &req, R, t),
        }
        .unwrap_or_else(|e| panic!("{verb}: the joined band builds, got {}", e.error));
        assert_eq!(
            topo::validate_geometric(&out.body, t),
            Ok(()),
            "{verb}: tier 3"
        );
        let rec = out.naming.as_ref().expect("birth records");
        assert_eq!(
            rec.joined_blends.len(),
            1,
            "{verb}: one band over both halves"
        );
        sweep::test_support::assert_naming_totality(&body, &out, &req, verb);
        let removed = volume(&body, t) - volume(&out.body, t);
        assert!(
            (removed - 2.0 * section).abs() < 1e-12,
            "{verb}: ΔV {removed} vs {}",
            2.0 * section
        );
    }
}

/// **A boolean's merged faces leave no joint.** Two flush unit cubes
/// unioned into a `2 × 1 × 1` box, their touching faces and coplanar
/// sides declared: each long side is one face, and the output stage
/// joins the vertices where the operands' rims met (maximal edges), so
/// the box has its 12 edges. Every edge fillets as its own band, at the
/// rounded box's closed form.
#[test]
fn a_union_of_flush_cubes_is_a_box_that_fillets_edge_by_edge() {
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
    assert_eq!(req.len(), 12, "the box's 12 edges, the long ones whole");

    let f = sweep::fillet::fillet_edges(&sweep::test_support::at_rest(body, t), &req, R, t)
        .expect("the union fillets");
    assert_eq!(topo::validate_geometric(&f.body, t), Ok(()), "tier 3");
    assert_eq!(f.blend_faces.len(), 12, "one band per box edge");
    let rec = f.naming.as_ref().expect("birth records");
    assert_eq!(rec.joined_blends.len(), 0, "no edge is split");
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
