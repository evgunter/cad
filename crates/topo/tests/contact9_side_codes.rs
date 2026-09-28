//! **A side code's `On` is a distance, at each site that reads one.**
//!
//! The boolean reads a sector bound's side of a face plane
//! (`boolean::sectors::side_code`), and a Zero there becomes `On`: a
//! verdict, "this bound lies in the plane". A line bound is read at its
//! far vertex, in metres. A direction levered at the sector's SHORTER
//! chord would read a 10 m edge beside a 1 mm one as On while its far
//! end stood hundreds of bands off, and the join would then refuse the
//! pose as a kernel bug.
//!
//! Each fixture poses that: a needle whose short (1 mm) edges set the
//! arm while its long (10 m) edge dips `500·ε` over its length. Under
//! the shorter-chord lever that dip reads `ε/20`, Zero. Read at the far
//! vertex it is definite, and every op answers against ground truth:
//! the analytic overlap volume, tier 3, and point membership in the
//! sliver. A control beside each poses the same dip under 1 m edges,
//! where both readings agree. A dip inside the band still reads On, and
//! the pose is a touch. The splitting lane's twin reads a line edge the
//! same way, through the same reader (`sector_shape::point_side`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{
    Body, BooleanError, BooleanResult, CarrierDesc, FaceKey, SolidContainment, SplitPlane,
    face_carrier, intersect, mass_properties, point_in_solid, split, subtract, subtract_with,
    union, union_with, validate_geometric,
};

/// The far end's depth, in zero thresholds: definite under a 1 m arm
/// (`DIP/10` zeros) and Zero under a 1 mm one (`DIP/10⁴`).
const DIP: f64 = 500.0;

/// `u·a + v·b + w·c`.
fn at(e: [[f64; 3]; 3], u: f64, v: f64, w: f64) -> Point3<f64> {
    let [a, b, c] = e;
    Point3::new(
        u * a[0] + v * b[0] + w * c[0],
        u * a[1] + v * b[1] + w * c[1],
        u * a[2] + v * b[2] + w * c[2],
    )
}

/// The parallelepiped `u·a + v·b + w·c` over the unit cube
/// (`det[a, b, c] > 0`).
fn parallelepiped(e: [[f64; 3]; 3]) -> Body<f64> {
    common::mapped_cube(move |u, v, w| at(e, u, v, w), Tol::witness())
}

/// A needle at the origin: a long edge `(x, y, -dip)` and two edges
/// of scale `s` rising from the tip.
fn needle(x: f64, y: f64, s: f64, rise: [[f64; 3]; 2]) -> [[f64; 3]; 3] {
    let dip = DIP * Tol::witness().eps();
    let [b, c] = rise;
    [
        [x, y, -dip],
        [s * b[0], s * b[1], s * b[2]],
        [s * c[0], s * c[1], s * c[2]],
    ]
}

/// The face of `body` whose outward normal is `n`, to within 1e-3.
fn face_with_normal(body: &Body<f64>, n: [f64; 3]) -> FaceKey {
    let n = Vec3::new(n[0], n[1], n[2]);
    let hits: Vec<FaceKey> = body
        .faces()
        .map(|(k, _)| k)
        .filter(|&k| {
            matches!(face_carrier(body, k), Some(CarrierDesc::Plane { normal, .. })
                if normal.dot(n) > 1.0 - 1e-3)
        })
        .collect();
    assert_eq!(hits.len(), 1, "one face faces {n:?}");
    hits[0]
}

fn body_of(r: Result<BooleanResult<f64>, BooleanError>, what: &str) -> Body<f64> {
    body_checked(r, what, true)
}

/// `tier3`: whether to run tier 3, whose signed-volume check shares the
/// volume door's floor ([`RESOLVED_VOLUME`]).
fn body_checked(r: Result<BooleanResult<f64>, BooleanError>, what: &str, tier3: bool) -> Body<f64> {
    let r = r.unwrap_or_else(|e| panic!("{what}: the control answers, got {e}"));
    let b = r
        .body()
        .unwrap_or_else(|| panic!("{what}: a body"))
        .body
        .clone();
    if tier3 {
        assert_eq!(
            validate_geometric(&b, Tol::witness()),
            Ok(()),
            "{what}: tier 3"
        );
    }
    b
}

fn contains(body: &Body<f64>, q: Point3<f64>) -> SolidContainment {
    let tol = Tol::witness();
    point_in_solid(body, q, Band::linear(tol).unwrap(), tol).unwrap()
}

/// The part of the needle `u·a + v·b + w·c` below `z = 0`, where `a`
/// dips `dip` and `b`, `c` rise `b_z`, `c_z`: `det·dip²/(6·b_z·c_z)`.
fn sliver(e: [[f64; 3]; 3]) -> f64 {
    let [a, b, c] = e;
    let det = a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
        + a[2] * (b[0] * c[1] - b[1] * c[0]);
    det * a[2] * a[2] / (6.0 * b[2] * c[2])
}

fn volume(body: &Body<f64>) -> f64 {
    mass_properties(body, Tol::witness()).unwrap().volume
}

/// The volume door resolves a sliver only down to about this: measured,
/// it returns half of a 4e-19 m³ tetrahedron whose vertices are exact,
/// and tier 3's signed-volume check reads the same integral negative.
/// Below it the vertex set, which fixes the sliver and so its volume,
/// is the oracle, and tier 3 runs on the other two results.
const RESOLVED_VOLUME: f64 = 1e-15;

/// Every op on `(solid, tool)` answers: `solid ∩ tool` has exactly the
/// sliver's `corners` (within the band) and, where the volume door
/// resolves it, the sliver's volume `overlap` (to 1%); `q`, inside
/// both, is in the intersection and the union and out of
/// `solid − tool`; every result passes tier 3.
fn answers(
    solid: &Body<f64>,
    tool: &Body<f64>,
    overlap: f64,
    corners: &[Point3<f64>],
    q: Point3<f64>,
    what: &str,
) {
    answers_with(solid, tool, overlap, corners, q, what, true);
}

/// [`answers`], with `tier3` false where the pose's seam is itself one
/// tier 3 reads as tangent (the reason is at the caller).
fn answers_with(
    solid: &Body<f64>,
    tool: &Body<f64>,
    overlap: f64,
    corners: &[Point3<f64>],
    q: Point3<f64>,
    what: &str,
    tier3: bool,
) {
    let tol = Tol::witness();
    assert_eq!(
        contains(tool, q),
        SolidContainment::In,
        "{what}: q in the tool"
    );
    assert_eq!(
        contains(solid, q),
        SolidContainment::In,
        "{what}: q in the solid"
    );
    let resolved = overlap >= RESOLVED_VOLUME;
    let meet = body_checked(
        intersect(solid, tool, tol),
        &format!("{what}: ∩"),
        resolved && tier3,
    );
    let points: Vec<Point3<f64>> = meet
        .vertices()
        .map(|(_, v)| *meet.get_point(v.point).unwrap())
        .collect();
    let near = |p: &Point3<f64>, q: &Point3<f64>| p.distance(*q) <= tol.eps();
    assert_eq!(
        points.len(),
        corners.len(),
        "{what}: ∩'s corners {points:?}"
    );
    for c in corners {
        assert!(
            points.iter().any(|p| near(p, c)),
            "{what}: ∩ has the corner {c:?}: {points:?}"
        );
    }
    if resolved {
        let v = volume(&meet);
        assert!(
            (v - overlap).abs() <= 1e-2 * overlap,
            "{what}: ∩ is the sliver: {v:e} vs {overlap:e}"
        );
    }
    assert_eq!(contains(&meet, q), SolidContainment::In, "{what}: q in ∩");
    let cut = body_checked(subtract(solid, tool, tol), &format!("{what}: −"), tier3);
    assert_eq!(
        contains(&cut, q),
        SolidContainment::Out,
        "{what}: the sliver is cut"
    );
    let joined = body_of(union(solid, tool, tol), &format!("{what}: ∪"));
    assert_eq!(
        contains(&joined, q),
        SolidContainment::In,
        "{what}: ∪ holds q"
    );
}

/// The needle's sliver below `z = 0`: its tip, its long edge's far end,
/// and where the two short edges from that end cross the plane.
fn needle_corners(e: [[f64; 3]; 3]) -> Vec<Point3<f64>> {
    let [a, b, c] = e;
    let dip = -a[2];
    vec![
        at(e, 0.0, 0.0, 0.0),
        at(e, 1.0, 0.0, 0.0),
        at(e, 1.0, dip / b[2], 0.0),
        at(e, 1.0, 0.0, dip / c[2]),
    ]
}

/// `vtxfac`: the needle's tip on a slab's top face. The dipping edge
/// is read at its far vertex, In, so the tip mints the germs the two
/// long faces' sections need.
#[test]
fn a_pierce_reads_a_dipping_edge_at_its_far_vertex() {
    let tol = Tol::witness();
    let dip = DIP * tol.eps();
    let slab = common::brick::<f64>((-20.0, 20.0), (-20.0, 20.0), (-5.0, 0.0), tol);
    let rise = [[0.0, 1.0, 1.0], [0.0, 0.0, 1.0]];
    for (what, s) in [("1 mm edges", 1e-3), ("control, 1 m edges", 1.0)] {
        let e = needle(10.0, 0.0, s, rise);
        let q = at(e, 0.99, 0.1 * dip / s, 0.1 * dip / s);
        answers(
            &slab,
            &parallelepiped(e),
            sliver(e),
            &needle_corners(e),
            q,
            what,
        );
    }
}

/// `pair_search`: the needle's tip on a block's corner, the dipping
/// edge over the block's top. The pair arm is the needle's 1 mm; the
/// edge is read at its far vertex, In, and the corner mints its germs.
#[test]
fn a_vertex_pair_reads_a_dipping_chord_at_its_far_vertex() {
    let tol = Tol::witness();
    let dip = DIP * tol.eps();
    let block = common::brick::<f64>((0.0, 20.0), (0.0, 20.0), (-20.0, 0.0), tol);
    let rise = [[-1.0, 0.0, 1.0], [0.0, -1.0, 1.0]];
    for (what, s) in [("1 mm edges", 1e-3), ("control, 1 m edges", 1.0)] {
        let e = needle(7.0, 7.0, s, rise);
        let q = at(e, 0.99, 0.1 * dip / s, 0.1 * dip / s);
        answers(
            &block,
            &parallelepiped(e),
            sliver(e),
            &needle_corners(e),
            q,
            what,
        );
    }
}

/// Delta 2's coplanar lump, at a pierce: a wedge whose corner sits on
/// the block's top, its bottom holding a 1 mm edge on that face and
/// tilting `500·ε` over 10 m. The normals agree at the 1 mm arm, which
/// only proposes coplanar: the long bound reads In at its far vertex,
/// so the sector is not lumped, and the op answers. (Lumped, the bottom
/// went to the carrier ladder, which found the planes definitely apart
/// and refused as a kernel invariant.) Declared `Rest`, the tilt is
/// contradicted at the door.
#[test]
fn a_sector_parallel_at_a_short_arm_is_coplanar_only_if_its_bounds_read_on() {
    let tol = Tol::witness();
    let dip = DIP * tol.eps();
    let block = common::brick::<f64>((0.0, 20.0), (0.0, 20.0), (-20.0, 0.0), tol);
    let e = [[10.0, 1.0, -dip], [0.2e-3, 1e-3, 0.0], [0.0, 0.0, 1e-3]];
    let corner = Vec3::new(5.0, 5.0, 0.0);
    let point = move |u, v, w| at(e, u, v, w) + corner;
    let wedge = common::mapped_cube(point, tol);
    // Below z = 0 where `w·c_z < u·dip`: `det·dip/(2·c_z)`.
    let det = e[0][0] * e[1][1] * e[2][2] - e[0][1] * e[1][0] * e[2][2];
    let overlap = det * dip / (2.0 * e[2][2]);
    let rise = dip / e[2][2];
    let corners = [
        point(0.0, 0.0, 0.0),
        point(0.0, 1.0, 0.0),
        point(1.0, 0.0, 0.0),
        point(1.0, 1.0, 0.0),
        point(1.0, 0.0, rise),
        point(1.0, 1.0, rise),
    ];
    let q = point(0.99, 0.5, 0.1 * rise);
    // No tier 3: every result's seam along the 1 mm edge joins faces
    // `dip/10` radians apart, which that edge's length cannot tell from
    // tangent, so tier 3 reads a scaffold edge at rest (and, on the
    // intersection, a lamina wedge). That is the seam description's
    // lever, filed as
    // `work/contact/seam-description-reads-a-dihedral-at-the-seams-own-length`;
    // the vertex set, the volume and the membership stand.
    answers_with(&block, &wedge, overlap, &corners, q, "tilted wedge", false);

    let mut decls = topo::BooleanDeclarations::none();
    decls.coincident_faces.push(topo::FacePairDeclaration::rest(
        face_with_normal(&block, [0.0, 0.0, 1.0]),
        face_with_normal(&wedge, [0.0, 0.0, -1.0]),
    ));
    for (what, r) in [
        ("block - wedge", subtract_with(&block, &wedge, &decls, tol)),
        ("union", union_with(&block, &wedge, &decls, tol)),
    ] {
        assert!(
            matches!(r, Err(BooleanError::ContactContradicted { .. })),
            "{what}: the declared tilt is contradicted, got {:?}",
            r.as_ref().err()
        );
    }
}

/// A dip inside the band is a real On: the far vertex stands within
/// the band of the face, so the edge lies on it to the tolerance, and
/// the pose is a touch. The needle and the slab share only boundary.
#[test]
fn a_dip_inside_the_band_still_reads_on() {
    let tol = Tol::witness();
    let dip = 0.5 * tol.eps();
    let slab = common::brick::<f64>((-20.0, 20.0), (-20.0, 20.0), (-5.0, 0.0), tol);
    let e = [[10.0, 0.0, -dip], [0.0, 1e-3, 1e-3], [0.0, 0.0, 1e-3]];
    let tool = parallelepiped(e);
    let joined = body_of(union(&slab, &tool, tol), "∪");
    let expect = volume(&slab) + volume(&tool);
    assert!(
        (volume(&joined) - expect).abs() <= 1e-9 * expect,
        "∪ of a touch is the sum: {} vs {expect}",
        volume(&joined)
    );
    let cut = body_of(subtract(&slab, &tool, tol), "−");
    assert!(
        (volume(&cut) - volume(&slab)).abs() <= 1e-9 * volume(&slab),
        "− of a touch leaves the slab: {}",
        volume(&cut)
    );
}

/// The splitting twin: a line edge's class is its far vertex's side,
/// in metres, so the short-armed needle splits, and its below part is
/// the sliver (to 1%, as above).
#[test]
fn the_splitting_twin_reads_the_dipping_edge_at_its_far_vertex() {
    let tol = Tol::witness();
    let e = needle(10.0, 0.0, 1e-3, [[0.0, 1.0, 1.0], [0.0, 0.0, 1.0]]);
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
    };
    let r = split(&parallelepiped(e), &plane, tol).unwrap();
    let below = volume(r.below.body().expect("a below part"));
    assert!(
        (below - sliver(e)).abs() <= 1e-2 * sliver(e),
        "the below part is the sliver: {below} vs {}",
        sliver(e)
    );
}
