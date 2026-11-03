//! **An edge on a ruling of a cylinder face, lying in a planar face of
//! its own solid, reads its side** (`crates/topo/README.md` tier 3: no
//! face passes through an edge's material wedge). Tier 3′ reads the
//! cylinder face across the edge by its chord
//! (`topo::link::FaceSector::chord`), near the origin and a kilometre
//! off it.
//!
//! - **A ridge on its floor rests.** The C-shaped block, its slot
//!   `[−1, 3] × [1, 2.5]` across `z ∈ [0, 3]`, united with a ridge hanging
//!   from its roof whose tip rests on the floor `y = 1` along
//!   `z ∈ [1, 2]`: one solid, the tip one edge lying in the floor, its
//!   wedge outside the floor's material. One face of the tip is a
//!   cylinder whose ruling the tip is: an arc wall leaving the floor
//!   across it, bowed either way, or tangent to the floor and bending
//!   up off it, which only its bend parts. Tier 3′ passes each.
//! - **A keel through its face crosses.** The block
//!   `[1, 4] × [−2, 2] × [0, 4]` less a trench from `z = 2` up out of the
//!   top whose keel lies in the face `x = 1`, its closing wall an arc
//!   bowed either way (or a plane, the control): the Boolean's slit
//!   form zipped shut by Euler surgery is one keel edge between the walls
//!   with the face through its wedge, and tier 3′ refuses it with the
//!   keel's `FaceThroughEdgeWedge` alone. (The Boolean refuses a trench
//!   wall tangent to the face along the keel, `EdgeInFaceGap::Tangent`,
//!   so that crossing is witnessed by `topo::link`'s `chord_rows`.)
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{SketchPlane, test_support::bulge_loop};
use sweep::test_support::{extruded, finished};
use topo::construction::{Construct, MefSite};
use topo::euler::FaceSurface;
use topo::{
    AtRestBody, Body, BooleanBody, BooleanResult, Cell, EdgeKey, FaceKey, LoopBoundary,
    ValidationError, VertexKey, subtract, union, validate_geometric, validate_pseudomanifold,
};

fn tol() -> Tol {
    Tol::witness()
}

/// Near the origin, and a kilometre off it.
const OFFSETS: [Vec3<f64>; 2] = [Vec3::new(0.0, 0.0, 0.0), Vec3::new(1000.0, -1000.0, 1000.0)];

/// `tan(π/8)`: the bulge of a quarter turn.
const QUARTER: f64 = 0.414_213_562_373_095_1;

/// A prism over `pts` (each `(x, y, bulge)`), from `o + z0·ẑ` up `h`.
fn prism(what: &str, o: Vec3<f64>, pts: &[(f64, f64, f64)], z0: f64, h: f64) -> AtRestBody<f64> {
    let plane = SketchPlane::xy_at(Point3::new(o.x, o.y, o.z + z0));
    let ring = bulge_loop(
        pts.iter()
            .map(|&(x, y, b)| (Point2::new(x, y), b))
            .collect(),
    );
    finished(what, extruded(plane, vec![ring], h, tol()), tol())
}

fn built(what: &str, r: Result<BooleanResult<f64>, topo::BooleanError>) -> BooleanBody<f64> {
    match r {
        Ok(BooleanResult::Body(b)) => b,
        other => panic!("{what}: builds, got {other:?}"),
    }
}

/// The C-shaped block: `[0, 4]² × [0, 3]` less the slot
/// `[−1, 3] × [1, 2.5]`, its floor `y = 1`.
fn c_block(o: Vec3<f64>) -> BooleanBody<f64> {
    let block = prism(
        "the block",
        o,
        &[
            (0.0, 0.0, 0.0),
            (4.0, 0.0, 0.0),
            (4.0, 4.0, 0.0),
            (0.0, 4.0, 0.0),
        ],
        0.0,
        3.0,
    );
    let slot = prism(
        "the slot",
        o,
        &[
            (-1.0, 1.0, 0.0),
            (3.0, 1.0, 0.0),
            (3.0, 2.5, 0.0),
            (-1.0, 2.5, 0.0),
        ],
        -1.0,
        5.0,
    );
    built("the C-shaped block", subtract(&block, &slot, tol()))
}

/// **A ridge on its floor rests**: each ridge, its tip at `(1.5, 1)`
/// along `z ∈ [1, 2]` and one wall from it a cylinder the tip is a
/// ruling of, united with the C-shaped block into one solid whose tip
/// lies in the floor on record. The floor's material is below it and
/// the ridge's wedge above. Red if the chord is not read (the tip
/// refuses `FaceThroughEdgeWedgeUndecided`, its cylinder face no
/// sector), or reads the arc on the wrong side of the floor, or the
/// tangent wall's bend is not read.
#[test]
fn a_ridge_on_a_cylinders_ruling_resting_on_its_floor_passes_tier_3_prime() {
    let ridges: [(&str, &[(f64, f64, f64)]); 3] = [
        (
            "an arc wall bowed out",
            &[(1.5, 1.0, 0.2), (2.1, 3.0, 0.0), (0.9, 3.0, 0.0)],
        ),
        (
            "an arc wall bowed in",
            &[(1.5, 1.0, -0.2), (2.1, 3.0, 0.0), (0.9, 3.0, 0.0)],
        ),
        (
            "an arc wall tangent to the floor",
            &[
                (1.5, 1.0, QUARTER),
                (2.5, 2.0, 0.0),
                (2.5, 3.0, 0.0),
                (1.5, 3.0, 0.0),
            ],
        ),
    ];
    for o in OFFSETS {
        let c = c_block(o);
        for (what, ridge) in ridges {
            let what = format!("{what}, {o:?} off");
            let ridge = prism(&what, o, ridge, 1.0, 1.0);
            let r = built(&what, union(&c.body, &ridge, tol()));
            assert_eq!(r.body.solids().count(), 1, "{what}: one solid");
            assert_eq!(
                r.contacts.edge_face().count(),
                1,
                "{what}: the tip lies in the floor on record: {:?}",
                r.contacts
            );
            assert_eq!(
                validate_pseudomanifold(&r.body, &r.contacts, tol()),
                Ok(()),
                "{what}: the ridge's wedge lies outside the floor's material"
            );
        }
    }
}

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

/// The edges of `body` on the keel's line, `(1, 0)` off `o`.
fn keel_edges(body: &Body<f64>, o: Vec3<f64>) -> Vec<EdgeKey> {
    let on = |p: Point3<f64>| (p.x - o.x - 1.0).abs() < 1e-9 && (p.y - o.y).abs() < 1e-9;
    body.edges()
        .map(|(e, _)| e)
        .filter(|&e| ends(body, e).iter().all(|&v| on(at(body, v))))
        .collect()
}

/// The one face of `body` on `x = 1` off `o`.
fn keel_face(body: &Body<f64>, o: Vec3<f64>) -> FaceKey {
    let faces: Vec<FaceKey> = body
        .faces()
        .map(|(f, _)| f)
        .filter(|&f| {
            rings(body, f)
                .iter()
                .flatten()
                .all(|p| (p.x - o.x - 1.0).abs() < 1e-9)
        })
        .collect();
    assert_eq!(faces.len(), 1, "one face on x = 1: {faces:?}");
    faces[0]
}

/// The block `[1, 4] × [−2, 2] × [0, 4]` less the trench over `pts` from
/// `z = 2` up out of the top, its keel at `(1, 0)`, zipped shut: the
/// Boolean's slit form, whose face `x = 1` passes the top twice, one
/// vertex per wedge, fused across the face, and the two-edge face left
/// between the keel edges folded into the wall on `y < 0`. One keel
/// edge between the walls, described on them, lying in the face; and
/// the keel's tip on it.
fn zipped(o: Vec3<f64>, pts: &[(f64, f64, f64)]) -> (Body<f64>, EdgeKey, FaceKey, VertexKey) {
    let top = Point3::new(o.x + 1.0, o.y, o.z + 4.0);
    let tip = Point3::new(o.x + 1.0, o.y, o.z + 2.0);
    let block = prism(
        "the block",
        o,
        &[
            (1.0, -2.0, 0.0),
            (4.0, -2.0, 0.0),
            (4.0, 2.0, 0.0),
            (1.0, 2.0, 0.0),
        ],
        0.0,
        4.0,
    );
    let trench = prism("the trench", o, pts, 2.0, 3.0);
    let mut body: Body<f64> = (*built("the slit", subtract(&block, &trench, tol())).body).clone();
    let face = keel_face(&body, o);
    let outer = body.get_face(face).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("the face's loop is a cycle");
    };
    let tops: Vec<_> = body
        .loop_cycle(first)
        .unwrap()
        .into_iter()
        .filter(|&h| at(&body, body.get_half_edge(h).unwrap().start).distance(top) < 1e-9)
        .collect();
    assert_eq!(tops.len(), 2, "the slit parts the top: {tops:?}");
    let joint = body
        .mef(
            MefSite::Chords {
                he1: tops[0],
                he2: tops[1],
            },
            geom_brep::EdgeCurveSpec::self_loop_circle_at(top),
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
    let gusset = [joint.face, face]
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
            rings(&body, wall)[0].iter().any(|p| p.y - o.y < -0.1)
        })
        .unwrap();
    body.kef_minting(to_wall_b, tol()).unwrap();
    let keel = keel_edges(&body, o);
    assert_eq!(keel.len(), 1, "one keel edge: {keel:?}");
    let keel = keel[0];
    let (s1, s2) = topo::readback::edge_sides(&body, keel).unwrap().surfaces();
    let [p0, p1] = ends(&body, keel).map(|v| at(&body, v));
    let mut spec = geom_brep::EdgeCurveSpec::line_between(p0, p1);
    spec.description = topo::EdgeDescriptionSpec::Intersection {
        pair: topo::SurfacePair::new(s1, s2),
        selector: geom_brep::CurveSelector::Witness(p0.lerp(p1, 0.5)),
    };
    body.set_edge_curve(keel, spec, tol()).unwrap();
    topo::mint_pcurves(&mut body, tol()).unwrap();
    let tip = body
        .vertices()
        .find(|&(v, _)| at(&body, v).distance(tip) < 1e-9)
        .unwrap()
        .0;
    let face = keel_face(&body, o);
    (body, keel, face, tip)
}

/// **A keel through its face crosses**: the zipped trench's one keel
/// edge, its closing wall a cylinder the keel is a ruling of, bowed
/// either way, lies in the face `x = 1` with the face's sheet inside its
/// wedge (about 330° of material, the face's material behind it on one
/// side). Tier 3 passes it, and tier 3′ refuses it with the keel's
/// `FaceThroughEdgeWedge` alone; so does the planar closing wall, the
/// control. Red if the chord is not read (the keel refuses undecided),
/// or reads as a rest.
#[test]
fn a_keel_on_a_cylinders_ruling_through_its_face_refuses_at_tier_3_prime() {
    let walls: [(&str, f64); 3] = [
        ("a planar closing wall", 0.0),
        ("an arc closing wall bowed out", 0.2),
        ("an arc closing wall bowed in", -0.2),
    ];
    for o in OFFSETS {
        for (what, bulge) in walls {
            let what = format!("{what}, {o:?} off");
            let (body, keel, face, tip) =
                zipped(o, &[(1.0, 0.0, 0.0), (1.6, -0.3, 0.0), (1.6, 0.3, bulge)]);
            assert_eq!(validate_geometric(&body, tol()), Ok(()), "{what}: tier 3");
            let records =
                topo::test_support::records(&body, [(Cell::Vertex(tip), Cell::Face(face))]);
            let errors = validate_pseudomanifold(&body, &records, tol()).unwrap_err();
            assert!(
                matches!(
                    errors[..],
                    [ValidationError::FaceThroughEdgeWedge { edge, face: f, .. }]
                        if edge == keel && f == face
                ),
                "{what}: tier 3′ refuses the keel's wedge alone: {errors:?}"
            );
        }
    }
}
