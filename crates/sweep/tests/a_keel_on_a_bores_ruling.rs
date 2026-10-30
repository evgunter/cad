//! **A keel resting on a bore's ruling.** The block
//! `[0, 4] × [−2, 2] × [0, 4]` with a half-bore of radius 1 about the z
//! axis in its `x = 0` face, less the V-trench over the triangle
//! `(1, 0)`, `(1.6, ±0.3)` from `z = 2` up out of the top: the trench's
//! keel lies on the bore's ruling through `(1, 0)`, its walls leaving
//! the bore outward. About the keel the material is two wedges, bore |
//! wall a and wall b | bore, so the bore's face carries the keel as a
//! slit of two coincident edges (`crates/topo/README.md`, tier 3: no face
//! passes through an edge's material wedge).
//!
//! - **The Boolean refuses typed.** `block ∖ trench`, and
//!   `block ∩ (box ∖ trench)`, read the keel as an edge lying in the
//!   bore's face whose material parts it, and refuse
//!   `EdgeInFaceGap::CurvedFace` rather than ship the one edge: a slit
//!   in a curved face is neither meshed nor measured yet
//!   (`work/tesslie/the-walk-panics-on-a-slit-in-a-curved-face.md`).
//! - **A cut through the slit caps around the trench.** The slit form,
//!   built as the union of the block's two sides of the trench, split at
//!   `z = 3`: each side holds its closed form, and no cap face on the
//!   plane covers the trench, read by winding over the cap faces' own
//!   vertices, not by the kernel's classification. So do the controls:
//!   the trench's apex off the bore, and the keel on a planar wall.
//! - **The one-edge form refuses at tier 3′.** The slit zipped shut by
//!   Euler surgery (the two top vertices fused across the bore's face,
//!   and the two-edge face left between the keel edges folded into wall
//!   b) is one keel edge between the walls with the bore's face through
//!   its wedge: tier 3 passes it, and tier 3′ refuses it with the keel's
//!   `FaceThroughEdgeWedge` alone.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{SketchPlane, test_support::bulge_loop};
use sweep::test_support::{extruded, finished};
use topo::construction::{Construct, MefSite};
use topo::euler::FaceSurface;
use topo::{
    AtRestBody, Body, BooleanError, BooleanResult, Cell, EdgeInFaceGap, EdgeKey, FaceKey,
    LoopBoundary, SplitPart, ValidationError, VertexKey, intersect, split, subtract, union,
    validate_geometric, validate_pseudomanifold,
};

fn tol() -> Tol {
    Tol::witness()
}

fn polygon(pts: &[(f64, f64)]) -> profile::ProfileLoop<f64> {
    bulge_loop(pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect())
}

/// A prism over `pts`, from `z0` up `h`.
fn prism(what: &str, pts: &[(f64, f64)], z0: f64, h: f64) -> AtRestBody<f64> {
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
    finished(what, extruded(plane, vec![polygon(pts)], h, tol()), tol())
}

/// The block with its half-bore.
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

const TRENCH: [(f64, f64); 3] = [(1.0, 0.0), (1.6, -0.3), (1.6, 0.3)];

/// The trench's triangle's area.
const TRENCH_AREA: f64 = 0.18;

fn trench() -> AtRestBody<f64> {
    prism("the trench", &TRENCH, 2.0, 3.0)
}

/// The block's cross-section less the half-bore.
fn section() -> f64 {
    16.0 - PI / 2.0
}

fn whole() -> f64 {
    4.0 * section() - 2.0 * TRENCH_AREA
}

const TIP: Point3<f64> = Point3::new(1.0, 0.0, 2.0);
const TOP: Point3<f64> = Point3::new(1.0, 0.0, 4.0);

fn at(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

fn ends(body: &Body<f64>, e: EdgeKey) -> [VertexKey; 2] {
    let h = body.get_edge(e).unwrap().he_plus;
    [
        body.get_half_edge(h).unwrap().start,
        body.half_edge_end(h).unwrap(),
    ]
}

/// The edges of `body` on the keel's line, `x = 1, y = 0`.
fn keel_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    let on = |p: Point3<f64>| (p.x - 1.0).abs() < 1e-12 && p.y.abs() < 1e-12;
    body.edges()
        .map(|(e, _)| e)
        .filter(|&e| ends(body, e).iter().all(|&v| on(at(body, v))))
        .collect()
}

fn bore_face(body: &Body<f64>) -> FaceKey {
    let bores: Vec<FaceKey> = body
        .faces()
        .map(|(f, _)| f)
        .filter(|&f| topo::query::face_surface_kind(body, f) == Some(geom::SurfaceKind::Cylinder))
        .collect();
    assert_eq!(bores.len(), 1, "one bore face: {bores:?}");
    bores[0]
}

/// The two faces of `e`.
fn faces_of(body: &Body<f64>, e: EdgeKey) -> [FaceKey; 2] {
    let h = body.get_edge(e).unwrap().he_plus;
    [h, body.mate(h).unwrap()].map(|h| body.face_of_half_edge(h).unwrap())
}

fn holds(what: &str, b: &AtRestBody<f64>, want: f64) {
    let mp = topo::test_support::pinned_volume(what, b, tol());
    let rounding = 64.0 * f64::EPSILON * want;
    assert!(
        (mp.volume - want).abs() <= mp.volume_pad + rounding,
        "{what}: holds {} ± {:e}, not {want}",
        mp.volume,
        mp.volume_pad
    );
    assert_eq!(validate_geometric(b, tol()), Ok(()), "{what}: tier 3");
}

fn body_of(r: Result<BooleanResult<f64>, BooleanError>, what: &str) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("{what}: {other:?}"),
    }
}

/// The slit form: the union of the block's two sides of the trench,
/// each the block cut by a prism, the lower one holding the V below
/// `z = 2`.
fn slit_block() -> AtRestBody<f64> {
    let upper = prism(
        "the upper tool",
        &[
            (1.0, 0.0),
            (1.6, 0.3),
            (1.6, -5.0),
            (5.0, -5.0),
            (5.0, 5.0),
            (-5.0, 5.0),
            (-5.0, 0.0),
        ],
        -1.0,
        6.0,
    );
    let lower = body_of(
        union(
            &prism(
                "the lower tool",
                &[
                    (1.0, 0.0),
                    (-5.0, 0.0),
                    (-5.0, -5.0),
                    (1.6, -5.0),
                    (1.6, -0.3),
                ],
                -1.0,
                6.0,
            ),
            &prism("the V", &TRENCH, -1.0, 3.0),
            tol(),
        ),
        "the lower tool",
    );
    body_of(
        union(
            &body_of(intersect(&block(), &upper, tol()), "the upper side"),
            &body_of(intersect(&block(), &lower, tol()), "the lower side"),
            tol(),
        ),
        "the two sides",
    )
}

/// **The Boolean refuses typed**: the keel lies in the bore's face, its
/// material parts there, and the parting is not built on a curved face.
#[test]
fn a_keel_on_the_bores_ruling_refuses_its_one_edge_form_typed() {
    let holed = body_of(
        subtract(
            &prism(
                "the box",
                &[(-1.0, -3.0), (5.0, -3.0), (5.0, 3.0), (-1.0, 3.0)],
                -1.0,
                6.0,
            ),
            &trench(),
            tol(),
        ),
        "the box less the trench",
    );
    for (door, r) in [
        ("block ∖ trench", subtract(&block(), &trench(), tol())),
        ("block ∩ (box ∖ trench)", intersect(&block(), &holed, tol())),
    ] {
        assert!(
            matches!(
                r,
                Err(BooleanError::EdgeInFace {
                    gap: EdgeInFaceGap::CurvedFace,
                    ..
                })
            ),
            "{door}: refuses the parting on the bore: {r:?}"
        );
    }
}

/// **The slit form holds**: two keel edges from the top to the tip, each
/// bounding the bore's face, the closed form, and tiers 3 and 3′.
#[test]
fn the_slit_form_holds_its_closed_form() {
    let body = slit_block();
    holds("the slit block", &body, whole());
    assert_eq!(
        validate_pseudomanifold(&body, &topo::ContactRecords::default(), tol()),
        Ok(()),
        "tier 3′"
    );
    let bore = bore_face(&body);
    let keel = keel_edges(&body);
    assert_eq!(keel.len(), 2, "two keel edges: {keel:?}");
    for e in keel {
        assert!(
            faces_of(&body, e).contains(&bore),
            "keel edge {e:?} bounds the bore's face"
        );
        assert!(
            ends(&body, e)
                .iter()
                .any(|&v| at(&body, v).distance(TIP) < 1e-12),
            "keel edge {e:?} ends at the tip"
        );
    }
}

/// Whether the polygon through `ring`'s points winds round `q`, in the
/// plane `z = q.z`.
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

/// The points of each loop of `face`, in walk order.
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

/// The split of `body` at `z = 3`: each side holds `[above, below]`,
/// and no cap face on the plane covers `in_trench`, its loops wound over
/// their own vertices.
fn caps_around_the_trench(
    what: &str,
    body: &AtRestBody<f64>,
    want: [f64; 2],
    in_trench: Point3<f64>,
) {
    let z = in_trench.z;
    let plane = topo::test_support::split_plane(in_trench, Vec3::unit_z(), tol());
    let r = split(body, &plane, tol()).unwrap_or_else(|e| panic!("{what}: the split: {e:?}"));
    for (side, part, want) in [("above", &r.above, want[0]), ("below", &r.below, want[1])] {
        let SplitPart::Body(body) = part else {
            panic!("{what}: nothing {side}");
        };
        holds(&format!("{what}, {side}"), body, want);
        let covering: Vec<FaceKey> = body
            .faces()
            .map(|(f, _)| f)
            .filter(|&f| {
                let loops = rings(body, f);
                loops.iter().flatten().all(|p| (p.z - z).abs() < 1e-12)
                    && loops.iter().filter(|ring| winds(ring, in_trench)).count() % 2 == 1
            })
            .collect();
        assert!(
            covering.is_empty(),
            "{what}, {side}: cap faces {covering:?} cover the trench at {in_trench:?}"
        );
    }
}

/// **A cut through the slit caps around the trench**, and so do the
/// controls: the trench's apex off the bore, `(1.2, 0)`, which the
/// Boolean builds as a hole of the block (a ring of the cap), and the
/// keel on a planar wall, `x = 1`, which it builds as a slit of the wall
/// (a pinched cap).
#[test]
fn a_split_through_the_keel_caps_around_the_trench() {
    let (above, below) = (section() - TRENCH_AREA, 3.0 * section() - TRENCH_AREA);
    caps_around_the_trench(
        "the slit form",
        &slit_block(),
        [above, below],
        Point3::new(1.3, 0.0, 3.0),
    );
    let off = prism(
        "the trench off the bore",
        &[(1.2, 0.0), (1.8, -0.3), (1.8, 0.3)],
        2.0,
        3.0,
    );
    caps_around_the_trench(
        "the trench off the bore",
        &body_of(subtract(&block(), &off, tol()), "the block less it"),
        [above, below],
        Point3::new(1.5, 0.0, 3.0),
    );
    let walled = prism(
        "the walled block",
        &[(1.0, -2.0), (4.0, -2.0), (4.0, 2.0), (1.0, 2.0)],
        0.0,
        4.0,
    );
    caps_around_the_trench(
        "the keel on a planar wall",
        &body_of(
            subtract(&walled, &trench(), tol()),
            "the walled block less the trench",
        ),
        [12.0 - TRENCH_AREA, 36.0 - TRENCH_AREA],
        Point3::new(1.3, 0.0, 3.0),
    );
}

/// **The one-edge form refuses at tier 3′**: the slit zipped shut is one
/// keel edge between the walls lying in the bore's face, which passes
/// through its wedge.
#[test]
fn the_keel_as_one_edge_through_the_bore_refuses_at_tier_3_prime() {
    let mut body: Body<f64> = (*slit_block()).clone();
    let bore = bore_face(&body);
    // The bore's loop passes the top twice, one vertex per wedge: fuse
    // them across the face, which cuts off the two keel edges' face.
    let outer = body.get_face(bore).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("the bore's loop is a cycle");
    };
    let tops: Vec<_> = body
        .loop_cycle(first)
        .unwrap()
        .into_iter()
        .filter(|&h| at(&body, body.get_half_edge(h).unwrap().start).distance(TOP) < 1e-12)
        .collect();
    assert_eq!(tops.len(), 2, "the slit parts the top: {tops:?}");
    let joint = body
        .mef(
            MefSite::Chords {
                he1: tops[0],
                he2: tops[1],
            },
            geom_brep::EdgeCurveSpec::self_loop_circle_at(TOP),
            FaceSurface::Inherit,
            tol(),
        )
        .unwrap();
    let keep = body.get_half_edge(tops[0]).unwrap().start;
    let dying = if body.get_half_edge(joint.he_plus).unwrap().start == keep {
        joint.he_plus
    } else {
        joint.he_minus
    };
    body.kev_describing(dying, &[], tol()).unwrap();
    // The two-edge face between the keel edges folds into wall b.
    let gusset = [joint.face, bore]
        .into_iter()
        .find(|&f| rings(&body, f)[0].len() == 2)
        .expect("a two-edge face between the keel edges");
    let gusset_outer = body.get_face(gusset).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(gusset_outer).unwrap().boundary else {
        panic!("the two-edge face's loop is a cycle");
    };
    let to_wall_b = body
        .loop_cycle(first)
        .unwrap()
        .into_iter()
        .find(|&h| {
            let wall = body.face_of_half_edge(body.mate(h).unwrap()).unwrap();
            rings(&body, wall)[0].iter().any(|p| p.y < -0.1)
        })
        .unwrap();
    body.kef_minting(to_wall_b, tol()).unwrap();
    let keel = keel_edges(&body);
    assert_eq!(keel.len(), 1, "one keel edge: {keel:?}");
    let keel = keel[0];
    let bore = bore_face(&body);
    assert!(
        !faces_of(&body, keel).contains(&bore),
        "the keel edge runs between the walls"
    );
    // Described on the two walls it now bounds.
    let (s1, s2) = topo::readback::edge_sides(&body, keel).unwrap().surfaces();
    let [p0, p1] = ends(&body, keel).map(|v| at(&body, v));
    let mut spec = geom_brep::EdgeCurveSpec::line_between(p0, p1);
    spec.description = topo::EdgeDescriptionSpec::Intersection {
        pair: topo::SurfacePair::new(s1, s2),
        selector: geom_brep::CurveSelector::Witness(p0.lerp(p1, 0.5)),
    };
    body.set_edge_curve(keel, spec, tol()).unwrap();
    topo::mint_pcurves(&mut body, tol()).unwrap();
    assert_eq!(validate_geometric(&body, tol()), Ok(()), "tier 3");
    let tip = body
        .vertices()
        .find(|&(v, _)| at(&body, v).distance(TIP) < 1e-12)
        .unwrap()
        .0;
    let records = topo::test_support::records(&body, [(Cell::Vertex(tip), Cell::Face(bore))]);
    let errors = validate_pseudomanifold(&body, &records, tol()).unwrap_err();
    assert!(
        matches!(
            errors[..],
            [ValidationError::FaceThroughEdgeWedge { edge, face, .. }] if edge == keel && face == bore
        ),
        "tier 3′ refuses the keel's wedge alone: {errors:?}"
    );
}
