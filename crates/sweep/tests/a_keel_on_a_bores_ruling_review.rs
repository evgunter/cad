//! **Review probes for the keel on a bore's ruling** (PR 6061): a keel
//! with one curved wall on the bore's ruling, which the Boolean's new
//! reading and the census arm both leave to planar walls.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{SketchPlane, test_support::bulge_loop};
use sweep::test_support::{extruded, finished};
use topo::{
    AtRestBody, Body, BooleanResult, EdgeKey, FaceKey, LoopBoundary, SplitPart, split, subtract,
    validate_geometric, validate_pseudomanifold,
};

fn tol() -> Tol {
    Tol::witness()
}

fn block() -> AtRestBody<f64> {
    finished(
        "the bored block",
        extruded(
            SketchPlane::xy(),
            vec![bulge_loop(vec![
                (Point2::new(0.0, -2.0), 0.0),
                (Point2::new(4.0, -2.0), 0.0),
                (Point2::new(4.0, 2.0), 0.0),
                (Point2::new(0.0, 2.0), 0.0),
                (Point2::new(0.0, 1.0), -1.0),
                (Point2::new(0.0, -1.0), 0.0),
            ])],
            4.0,
            tol(),
        ),
        tol(),
    )
}

/// A trench with its apex on the bore's ruling through `(1, 0)`: wall a
/// a plane, the closing wall from `(1.6, 0.3)` back to the apex an arc
/// of bulge `bulge` (a cylinder wall, transverse to the bore).
fn curved_trench(bulge: f64) -> AtRestBody<f64> {
    finished(
        "the curved trench",
        extruded(
            SketchPlane::xy_at(Point3::new(0.0, 0.0, 2.0)),
            vec![bulge_loop(vec![
                (Point2::new(1.0, 0.0), 0.0),
                (Point2::new(1.6, -0.3), 0.0),
                (Point2::new(1.6, 0.3), bulge),
            ])],
            3.0,
            tol(),
        ),
        tol(),
    )
}

fn at(body: &Body<f64>, v: topo::VertexKey) -> Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

fn keel_edges(body: &Body<f64>) -> Vec<(EdgeKey, Vec<Option<geom::SurfaceKind>>)> {
    let on = |p: Point3<f64>| (p.x - 1.0).abs() < 1e-9 && p.y.abs() < 1e-9;
    body.edges()
        .map(|(e, _)| e)
        .filter(|&e| {
            let h = body.get_edge(e).unwrap().he_plus;
            [
                body.get_half_edge(h).unwrap().start,
                body.half_edge_end(h).unwrap(),
            ]
            .iter()
            .all(|&v| on(at(body, v)))
        })
        .map(|e| {
            let h = body.get_edge(e).unwrap().he_plus;
            let kinds = [h, body.mate(h).unwrap()]
                .iter()
                .map(|&h| topo::query::face_surface_kind(body, body.face_of_half_edge(h).unwrap()))
                .collect();
            (e, kinds)
        })
        .collect()
}

fn winds(ring: &[Point3<f64>], q: Point3<f64>) -> bool {
    let mut w = 0i32;
    for (a, b) in ring.iter().zip(ring.iter().cycle().skip(1)) {
        let cross = (b.x - a.x) * (q.y - a.y) - (q.x - a.x) * (b.y - a.y);
        if a.y <= q.y && b.y > q.y && cross > 0.0 {
            w += 1;
        } else if a.y > q.y && b.y <= q.y && cross < 0.0 {
            w -= 1;
        }
    }
    w != 0
}

fn rings(body: &Body<f64>, face: FaceKey) -> Vec<Vec<Point3<f64>>> {
    let f = body.get_face(face).unwrap();
    core::iter::once(f.outer)
        .chain(f.rings.iter().copied())
        .filter_map(|l| match body.get_loop(l).unwrap().boundary {
            LoopBoundary::Cycle { first } => Some(
                body.loop_cycle(first)
                    .unwrap()
                    .into_iter()
                    .map(|h| at(body, body.get_half_edge(h).unwrap().start))
                    .collect(),
            ),
            LoopBoundary::Empty { .. } => None,
        })
        .collect()
}

/// **A keel with one curved wall on the bore's ruling**: printed, then
/// pinned to the reviewer's expectation that it is not shipped silently.
#[test]
fn review_a_keel_with_a_curved_wall_on_the_bores_ruling() {
    let mut silent = Vec::new();
    for bulge in [0.2, -0.2] {
        let r = subtract(&block(), &curved_trench(bulge), tol());
        let body = match r {
            Ok(BooleanResult::Body(b)) => b.body,
            other => {
                eprintln!("bulge {bulge}: {other:?}");
                continue;
            }
        };
        let keel = keel_edges(&body);
        let t3 = validate_geometric(&body, tol());
        let t3p = validate_pseudomanifold(&body, &topo::ContactRecords::default(), tol());
        eprintln!("bulge {bulge}: keel {keel:?}\n  tier 3 {t3:?}\n  tier 3' {t3p:?}");
        let mp = topo::test_support::pinned_volume("probe", &body, tol());
        eprintln!("  volume {} ± {:e}", mp.volume, mp.volume_pad);
        let z = 3.0;
        let plane =
            topo::test_support::split_plane(Point3::new(1.0, 0.0, z), Vec3::unit_z(), tol());
        let in_trench = Point3::new(1.3, 0.0, z);
        let mut covered = false;
        match split(&body, &plane, tol()) {
            Ok(s) => {
                for (side, part) in [("above", &s.above), ("below", &s.below)] {
                    let SplitPart::Body(b) = part else { continue };
                    let covering: Vec<FaceKey> = b
                        .faces()
                        .map(|(f, _)| f)
                        .filter(|&f| {
                            let loops = rings(b, f);
                            loops.iter().flatten().all(|p| (p.z - z).abs() < 1e-12)
                                && loops.iter().filter(|ring| winds(ring, in_trench)).count() % 2
                                    == 1
                        })
                        .collect();
                    eprintln!(
                        "  split {side}: covering {covering:?}, tier 3 {:?}, tier 3' {:?}",
                        validate_geometric(b, tol()),
                        validate_pseudomanifold(b, &topo::ContactRecords::default(), tol())
                    );
                    covered |= !covering.is_empty();
                }
            }
            Err(e) => eprintln!("  split: {e:?}"),
        }
        let bore: Vec<FaceKey> = body
            .faces()
            .map(|(f, _)| f)
            .filter(|&f| {
                topo::query::face_surface_kind(&body, f) == Some(geom::SurfaceKind::Cylinder)
                    && rings(&body, f).iter().flatten().any(|p| p.x < 0.5)
            })
            .collect();
        let one_edge_through_bore = keel.len() == 1 && {
            let h = body.get_edge(keel[0].0).unwrap().he_plus;
            ![h, body.mate(h).unwrap()]
                .iter()
                .any(|&h| bore.contains(&body.face_of_half_edge(h).unwrap()))
        };
        eprintln!("  bore {bore:?}, one edge through the bore: {one_edge_through_bore}");
        if one_edge_through_bore && t3p.is_ok() {
            silent.push((bulge, covered));
        }
    }
    assert!(
        silent.is_empty(),
        "one keel edge with the bore through its wedge passes tier 3' at (bulge, split covers \
         the trench): {silent:?}"
    );
}

/// **Outside touches on a ruling**, printed on both heads: a prism in the
/// bore's void with its apex edge on the ruling (the wedge clear of the
/// bore's material), and a rod with a box whose edge sits on its ruling,
/// one wall tangent to the rod.
#[test]
fn review_outside_touches_on_a_ruling() {
    let poker = finished(
        "the poker",
        extruded(
            SketchPlane::xy_at(Point3::new(0.0, 0.0, 1.0)),
            vec![bulge_loop(vec![
                (Point2::new(1.0, 0.0), 0.0),
                (Point2::new(0.5, 0.2), 0.0),
                (Point2::new(0.5, -0.2), 0.0),
            ])],
            2.0,
            tol(),
        ),
        tol(),
    );
    let rod = finished(
        "the rod",
        extruded(
            SketchPlane::xy(),
            vec![bulge_loop(vec![
                (Point2::new(1.0, 0.0), 1.0),
                (Point2::new(-1.0, 0.0), 1.0),
            ])],
            4.0,
            tol(),
        ),
        tol(),
    );
    let boxed = finished(
        "the box",
        extruded(
            SketchPlane::xy_at(Point3::new(0.0, 0.0, 1.0)),
            vec![bulge_loop(vec![
                (Point2::new(1.0, 0.0), 0.0),
                (Point2::new(2.0, 0.0), 0.0),
                (Point2::new(2.0, 1.0), 0.0),
                (Point2::new(1.0, 1.0), 0.0),
            ])],
            2.0,
            tol(),
        ),
        tol(),
    );
    for (what, r) in [
        ("block ∪ poker", topo::union(&block(), &poker, tol())),
        ("rod ∪ box", topo::union(&rod, &boxed, tol())),
        ("rod ∖ box", subtract(&rod, &boxed, tol())),
    ] {
        match r {
            Ok(BooleanResult::Body(b)) => eprintln!(
                "{what}: built, records {}, tier 3 {:?}, tier 3' {:?}",
                b.contacts.len(),
                validate_geometric(&b.body, tol()),
                validate_pseudomanifold(&b.body, &b.contacts, tol())
            ),
            other => eprintln!("{what}: {other:?}"),
        }
    }
}
