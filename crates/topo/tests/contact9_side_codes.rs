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
//! same way, through the same reader (`sector_shape::plane_offset`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{
    Body, BooleanError, BooleanResult, CarrierDesc, FaceKey, SolidContainment, ValidationError,
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
    let n = Vec3::from_array(n);
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

/// `r`'s body, which passes tier 3.
fn body_of(r: Result<BooleanResult<f64>, BooleanError>, what: &str) -> Body<f64> {
    let r = r.unwrap_or_else(|e| panic!("{what}: an answer, got {e}"));
    let b = r
        .body()
        .unwrap_or_else(|| panic!("{what}: a body"))
        .body
        .clone();
    assert_eq!(
        validate_geometric(&b, Tol::witness()),
        Ok(()),
        "{what}: tier 3"
    );
    b
}

/// `r` is the result gate's refusal of exactly one scaffold at rest:
/// the seam along the 1 mm edge, the one edge of these results whose
/// faces that length cannot tell from tangent. The refusal withholds the
/// body, so the edge is pinned by the count; a second scaffold, or a
/// different finding, turns the row red.
fn refuses_the_seam_alone(r: &Result<BooleanResult<f64>, BooleanError>, what: &str) {
    assert!(
        matches!(r, Err(BooleanError::ResultInvalid { errors })
            if matches!(errors.as_slice(), [ValidationError::ScaffoldAtRest { .. }])),
        "{what}: the 1 mm seam's scaffold, alone, is refused at rest, got {:?}",
        r.as_ref().err()
    );
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

/// The smallest intersection whose volume these rows trust, in m³. The
/// volume door's absolute error on a result spanning these fixtures'
/// ~10 m is up to ~1e-15 m³, measured on exact slivers at ε from 1e-6
/// to 1e-12 (it returns half of a 4e-19 m³ tetrahedron with exact
/// corners, and −3e-16 for a 6e-19 one). A thousand times that keeps
/// the 1% check honest. Below it the corner set, which fixes the sliver
/// and so its volume, is the oracle. The door's error is filed:
/// `work/flux/a-planar-face-sums-its-area-about-a-far-carrier-origin`.
const RESOLVED_VOLUME: f64 = 1e-12;

/// Every op on `(solid, tool)` answers: `solid ∩ tool` has exactly the
/// sliver's `corners` (within the band) and, where the volume door
/// resolves it, the sliver's volume `overlap` (to 1%); every result
/// passes tier 3; and each result holds exactly the points it should of
/// four: `q` inside both, `solid_only` inside the solid alone,
/// `tool_only` inside the tool alone, and a point 60 m away.
fn answers(
    solid: &Body<f64>,
    tool: &Body<f64>,
    overlap: f64,
    corners: &[Point3<f64>],
    [q, solid_only, tool_only]: [Point3<f64>; 3],
    what: &str,
) {
    use SolidContainment::{In, Out};
    let tol = Tol::witness();
    let far = Point3::new(60.0, 1.0, 2.0);
    for (body, name, want) in [
        (solid, "the solid", [In, In, Out, Out]),
        (tool, "the tool", [In, Out, In, Out]),
    ] {
        let got = [q, solid_only, tool_only, far].map(|p| contains(body, p));
        assert_eq!(got, want, "{what}: the points in {name}");
    }
    let meet = body_of(intersect(solid, tool, tol), &format!("{what}: ∩"));
    has_corners(&meet, corners, &format!("{what}: ∩"));
    if overlap >= RESOLVED_VOLUME {
        let v = volume(&meet);
        assert!(
            (v - overlap).abs() <= 1e-2 * overlap,
            "{what}: ∩ is the sliver: {v:e} vs {overlap:e}"
        );
    }
    let cut = body_of(subtract(solid, tool, tol), &format!("{what}: −"));
    let joined = body_of(union(solid, tool, tol), &format!("{what}: ∪"));
    for (body, op, want) in [
        (&meet, "∩", [In, Out, Out, Out]),
        (&cut, "−", [Out, In, Out, Out]),
        (&joined, "∪", [In, In, In, Out]),
    ] {
        let got = [q, solid_only, tool_only, far].map(|p| contains(body, p));
        assert_eq!(
            got, want,
            "{what}: {op} holds q, the solid's point, the tool's point and the far point as it should"
        );
    }
}

/// `body`'s vertices are exactly `corners`, each within the band.
fn has_corners(body: &Body<f64>, corners: &[Point3<f64>], what: &str) {
    let eps = Tol::witness().eps();
    let points: Vec<Point3<f64>> = body
        .vertices()
        .map(|(_, v)| *body.get_point(v.point).unwrap())
        .collect();
    assert_eq!(points.len(), corners.len(), "{what}: corners {points:?}");
    for c in corners {
        assert!(
            points.iter().any(|p| p.distance(*c) <= eps),
            "{what}: has the corner {c:?}: {points:?}"
        );
    }
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
            [q, Point3::new(-15.0, -15.0, -2.0), at(e, 0.5, 0.5, 0.5)],
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
            [q, Point3::new(15.0, 15.0, -10.0), at(e, 0.5, 0.5, 0.5)],
            what,
        );
    }
}

/// Delta 2's coplanar lump, at a pierce: a wedge whose corner sits on
/// the block's top, its bottom holding a 1 mm edge on that face and
/// tilting `500·ε` over 10 m. The normals agree at the 1 mm arm, which
/// only proposes coplanar: the long bound reads In at its far vertex,
/// so the sector is not lumped, and the ∪ answers. (Lumped, the bottom
/// went to the carrier ladder, which found the planes definitely apart
/// and refused as a kernel invariant.) The ∩ and the − refuse at the
/// result gate (the reason is in the body). Declared `Rest`, the tilt is
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
    let q = point(0.99, 0.5, 0.1 * dip / e[2][2]);
    // ∩ and − keep a seam along the 1 mm edge between faces `dip/10`
    // radians apart, which that edge's length cannot tell from tangent:
    // the seam is left a scaffold, and the result gate refuses it at
    // rest. That is the seam description's lever, filed as
    // `work/contact/seam-description-reads-a-dihedral-at-the-seams-own-length`.
    for (what, r) in [
        ("∩", intersect(&block, &wedge, tol)),
        ("−", subtract(&block, &wedge, tol)),
    ] {
        refuses_the_seam_alone(&r, &format!("tilted wedge {what}"));
    }
    let joined = body_of(union(&block, &wedge, tol), "tilted wedge: ∪");
    assert_eq!(
        contains(&joined, q),
        SolidContainment::In,
        "tilted wedge: ∪ holds q"
    );

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

/// The plane arm's lever, on a 10 m wedge resting at the origin on a
/// block's top (20 m square), its bottom tilted so the far end stands
/// `5·K·ε` below: `0.5·Kε` at a one-metre arm, which the ladder there
/// bridges, and `5·Kε` at the wedge's far end. The declared `Rest`
/// door levers the tilt at the pair's consumed extent (at least the
/// wedge's 10 m) and contradicts it, and so does every op that uses
/// the declaration.
#[test]
fn a_declared_plane_tilt_is_read_across_the_faces() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let dip = 5.0 * band.escalate();
    let block = common::brick::<f64>((-10.0, 10.0), (-10.0, 10.0), (-20.0, 0.0), tol);
    let wedge = parallelepiped([[10.0, 0.0, -dip], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    let (top, bottom) = (
        face_with_normal(&block, [0.0, 0.0, 1.0]),
        face_with_normal(&wedge, [0.0, 0.0, -1.0]),
    );
    let (ca, cb) = (
        face_carrier(&block, top).unwrap(),
        face_carrier(&wedge, bottom).unwrap(),
    );
    let declared = topo::PlaneIdentity {
        s1: None,
        s2: None,
        declared: true,
    };
    let metre =
        topo::boolean::carrier_eq::carrier_eq_verdict(&ca, &cb, declared, &metre_ball(1.0), band);
    assert!(
        matches!(
            metre,
            Ok((
                topo::PlaneRelation::SameOpposite,
                topo::ContactVerdict::Bridged
            ))
        ),
        "at a 1 m arm the tilt reads in band and the declaration bridges it: {metre:?}"
    );
    let door = topo::boolean::contact_pair_verdict(
        &block,
        top,
        &wedge,
        bottom,
        topo::ContactClass::Rest,
        None,
        band,
    );
    assert!(
        matches!(door, Err(topo::ContactRefusal::Contradicted { .. })),
        "the door reads the tilt across the faces and contradicts it: {door:?}"
    );
    let mut decls = topo::BooleanDeclarations::none();
    decls
        .coincident_faces
        .push(topo::FacePairDeclaration::rest(top, bottom));
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

/// A 10 m × 1 m wedge whose bottom (`u·(10, 0, rise) + v·(0, 1, 0)`,
/// lifted `lift`) rests declared on the 20 m block top `z = 0`, with
/// the door's verdict on the pair. The pair's consumed extent is the
/// ball about the mean of the two faces' box centres, `(2.5, 0.25, 0)`,
/// out to the top's far corner: radius `√200 + |(2.5, 0.25)| ≈
/// 16.655 m`.
fn wedge_on_the_top(
    rise: f64,
    lift: f64,
) -> (
    Body<f64>,
    FaceKey,
    Body<f64>,
    FaceKey,
    Result<topo::ContactVerdict, topo::ContactRefusal>,
) {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let block = common::brick::<f64>((-10.0, 10.0), (-10.0, 10.0), (-20.0, 0.0), tol);
    let e = [[10.0, 0.0, rise], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let wedge = common::mapped_cube(
        move |u, v, w| at(e, u, v, w) + Vec3::new(0.0, 0.0, lift),
        tol,
    );
    let (top, bottom) = (
        face_with_normal(&block, [0.0, 0.0, 1.0]),
        face_with_normal(&wedge, [0.0, 0.0, -1.0]),
    );
    let door = topo::boolean::contact_pair_verdict(
        &block,
        top,
        &wedge,
        bottom,
        topo::ContactClass::Rest,
        None,
        band,
    );
    (block, top, wedge, bottom, door)
}

/// The pair's consumed-extent radius ([`wedge_on_the_top`]).
const REACH: f64 = 16.655;

/// **The offset and the tilt add.** The wedge's bottom stands `0.9·Kε`
/// off the top at the extent's centre (`x = 2.5`), and tilts so that
/// the tilt levered at the extent's radius reads another `0.9·Kε`: each
/// in band on its own, which a datum-by-datum reading bridged, while the
/// far corner stands `0.9·Kε + 7.5 m·tilt ≈ 1.3·Kε` off. Read as one
/// displacement, the far corner contradicts the declaration.
#[test]
fn a_declared_offset_and_tilt_each_in_band_do_not_bridge_their_sum() {
    let k = Band::linear(Tol::witness()).unwrap().escalate();
    let slope = 0.9 * k / REACH;
    let lift = 0.9 * k - 2.5 * slope;
    let far = lift + 10.0 * slope;
    assert!(far > 1.25 * k, "the far corner stands past the band: {far}");
    let (block, top, wedge, bottom, door) = wedge_on_the_top(10.0 * slope, lift);
    assert!(
        matches!(door, Err(topo::ContactRefusal::Contradicted { .. })),
        "the far corner contradicts the declaration: {door:?}"
    );
    let mut decls = topo::BooleanDeclarations::none();
    decls
        .coincident_faces
        .push(topo::FacePairDeclaration::rest(top, bottom));
    let r = union_with(&block, &wedge, &decls, Tol::witness());
    assert!(
        matches!(r, Err(BooleanError::ContactContradicted { .. })),
        "the op refuses it at the door: {:?}",
        r.as_ref().err()
    );
}

/// **An upper bound past the band is not a contradiction.** The wedge
/// pivots on its near edge so its far corner stands `d` off the top,
/// for `d` across `0.3…1.5·Kε`. The door's upper bound is the offset at
/// the extent's centre plus the tilt levered at its radius,
/// `d·(2.5 + 16.655)/10 ≈ 1.92·d`; its lower bound is the far corner's
/// own `d`. So it bridges below `d ≈ 0.52·Kε` (every point in band),
/// contradicts from `d = Kε` (a corner past the band), and between them
/// — every point in band, but the ball over-states the faces — refuses
/// as unsettled rather than contradicting.
#[test]
fn a_wedge_sweep_bridges_then_escalates_then_contradicts() {
    let k = Band::linear(Tol::witness()).unwrap().escalate();
    for (d, expect) in [
        (0.3, "bridged"),
        (0.4, "bridged"),
        (0.45, "bridged"),
        (0.6, "unsettled"),
        (0.75, "unsettled"),
        (0.9, "unsettled"),
        (1.1, "contradicted"),
        (1.3, "contradicted"),
        (1.5, "contradicted"),
    ] {
        let (.., door) = wedge_on_the_top(d * k, 0.0);
        let read = match door {
            Ok(topo::ContactVerdict::Bridged) => "bridged",
            Err(topo::ContactRefusal::Escalated { .. }) => "unsettled",
            Err(topo::ContactRefusal::Contradicted { .. }) => "contradicted",
            ref other => panic!("d = {d}·Kε: unexpected {other:?}"),
        };
        assert_eq!(read, expect, "d = {d}·Kε");
    }
}

/// **The Boolean names the unsettled reach as its own question.** At
/// `d = 0.75·Kε` the sweep above reads unsettled at the door: the
/// upper bound stands past the band, and no corner is shown off. The
/// union refuses on that question, stating the bound as past the band,
/// and does not speak of a contact's witness, which no step of it read.
#[test]
fn an_unsettled_declared_pair_refuses_as_its_reach() {
    let k = Band::linear(Tol::witness()).unwrap().escalate();
    let (block, top, wedge, bottom, door) = wedge_on_the_top(0.75 * k, 0.0);
    assert!(
        matches!(door, Err(topo::ContactRefusal::Escalated { .. })),
        "the door reads it unsettled: {door:?}"
    );
    let mut decls = topo::BooleanDeclarations::none();
    decls
        .coincident_faces
        .push(topo::FacePairDeclaration::rest(top, bottom));
    let err = union_with(&block, &wedge, &decls, Tol::witness()).expect_err("it refuses");
    assert!(
        matches!(
            err,
            BooleanError::Escalated {
                decision: topo::BooleanDecision::Coincidence(topo::Coincide::DeclaredReach, _),
                ..
            }
        ),
        "{err:?}"
    );
    let text = err.to_string();
    assert!(
        text.contains("lies past the ambiguity band")
            && !text.contains("inside the ambiguity band")
            && !text.contains("witness")
            && text.ends_with(
                "Recourse: move the parts so the declared faces clearly coincide, or clearly do not"
            ),
        "{text}"
    );
}

/// The pierce germ line, read at the sector's reach: a wedge on the
/// block's top whose 1 mm edge lies on that face while its 10 m edge
/// RISES `500·ε` and its third edge descends into the block. The
/// bottom's transition sector meets the top at `dip/10` radians, which
/// the 1 mm arm reads as coplanar; the reach (10 m) reads the germ line.
/// `tool − block`, the sliver above the top, refuses at the result gate
/// for the seam reason at the wedge row above; `tool ∩ block` answers.
#[test]
fn a_pierce_germ_line_is_read_at_the_sectors_reach() {
    let tol = Tol::witness();
    let dip = DIP * tol.eps();
    let block = common::brick::<f64>((0.0, 20.0), (0.0, 20.0), (-20.0, 0.0), tol);
    let e = [[0.2e-3, 1e-3, 0.0], [10.0, 1.0, dip], [1e-4, 1e-4, -1e-3]];
    let corner = Vec3::new(5.0, 5.0, 0.0);
    let point = move |u, v, w| at(e, u, v, w) + corner;
    let tool = common::mapped_cube(point, tol);
    let q = point(0.5, 0.99, 0.1 * dip / -e[2][2]);
    // The sliver reaches the result gate, so the classification read
    // the germ line; its seam along the 1 mm edge is the wedge row's
    // scaffold, which the gate refuses at rest.
    let cut = subtract(&tool, &block, tol);
    refuses_the_seam_alone(&cut, "tool − block");
    let meet = body_of(intersect(&tool, &block, tol), "tool ∩ block");
    assert_eq!(contains(&meet, q), SolidContainment::Out, "q is not in ∩");
}

/// A near-coincidence at a vertex pair, which the classification does
/// not settle: a wedge on the block's CORNER whose 1 mm edge lies in the
/// top's plane (outside the face) and whose 10 m edge rises `500·ε`
/// over it. The pair of bottoms agrees at the 1 mm arm, so it goes to
/// the carrier ladder as a coincidence, and undeclared it refuses,
/// typed. Read by its bounds instead, the pair is half a crossing, and
/// the vertex's germs came out odd (a kernel invariant). The same pose
/// under 1 m edges answers. Making the short pose answer is filed:
/// `work/contact/a-vertex-pair-near-coincidence-refuses-where-its-long-edges-decide`.
#[test]
fn a_near_coincident_pair_at_a_corner_refuses_typed() {
    let tol = Tol::witness();
    let dip = DIP * tol.eps();
    let block = common::brick::<f64>((0.0, 20.0), (0.0, 20.0), (-20.0, 0.0), tol);
    let edges = |s: f64| {
        [
            [10.0, 1.0, dip],
            [-1e-3 * s, -0.2e-3 * s, 0.0],
            [1e-4 * s, 1e-4 * s, -1e-3 * s],
        ]
    };
    let short = parallelepiped(edges(1.0));
    for (what, r) in [
        ("∩", intersect(&short, &block, tol)),
        ("−", subtract(&short, &block, tol)),
        ("∪", union(&short, &block, tol)),
    ] {
        // UndeclaredCoincidence; at ε = 1e-6, where the 1 mm edges are
        // a thousand bands, an edge contact in band escalates first.
        assert!(
            matches!(
                r,
                Err(BooleanError::UndeclaredCoincidence { .. } | BooleanError::Escalated { .. })
            ),
            "{what}: the near-coincidence refuses typed, got {:?}",
            r.as_ref().err()
        );
    }

    let e = edges(1e3);
    let control = parallelepiped(e);
    let meet = body_of(intersect(&control, &block, tol), "control ∩");
    let cut = body_of(subtract(&control, &block, tol), "control −");
    let joined = body_of(union(&control, &block, tol), "control ∪");
    let (vt, vb, vm) = (volume(&control), volume(&block), volume(&meet));
    assert!(
        (vm + volume(&cut) - vt).abs() <= 1e-9 * vt,
        "control: ∩ and − partition the tool"
    );
    assert!(
        (volume(&joined) - (vb + vt - vm)).abs() <= 1e-9 * vb,
        "control: ∪ is block + tool − ∩"
    );
    let inside = at(e, 0.5, 0.1, 0.9);
    assert_eq!(
        contains(&meet, inside),
        SolidContainment::In,
        "control: ∩ holds a point in both"
    );
    assert_eq!(
        contains(&cut, inside),
        SolidContainment::Out,
        "control: − cuts it"
    );
    let outside = at(e, 0.01, 0.9, 0.1);
    assert_eq!(
        contains(&block, outside),
        SolidContainment::Out,
        "control: a point of the tool off the block"
    );
    assert_eq!(
        contains(&cut, outside),
        SolidContainment::In,
        "control: − keeps it"
    );
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
/// the sliver: its corners, and its volume where the door resolves it.
#[test]
fn the_splitting_twin_reads_the_dipping_edge_at_its_far_vertex() {
    let tol = Tol::witness();
    let e = needle(10.0, 0.0, 1e-3, [[0.0, 1.0, 1.0], [0.0, 0.0, 1.0]]);
    let plane = topo::test_support::split_plane(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        geom_core::Tol::witness(),
    );
    let r = split(&parallelepiped(e), &plane, tol).unwrap();
    let below = r.below.body().expect("a below part");
    has_corners(below, &needle_corners(e), "the below part");
    if sliver(e) >= RESOLVED_VOLUME {
        let v = volume(below);
        assert!(
            (v - sliver(e)).abs() <= 1e-2 * sliver(e),
            "the below part is the sliver: {v} vs {}",
            sliver(e)
        );
    }
}

/// A ball of radius `arm` about the origin, no point of either face
/// known: the extent a bare arm names.
fn metre_ball(arm: f64) -> topo::ConsumedExtent<'static, f64> {
    topo::ConsumedExtent::unwitnessed(geom_brep::ExtentBall::new(Point3::origin(), arm))
}
