//! **A side code's levered Zero, read as On, at each site that reads
//! one**, and what stands behind it.
//!
//! The boolean reads a sector bound's side of a face plane as
//! `d̂·n̂ × arm` (`boolean::sectors::side_code`), with `arm` the
//! shorter chord of the sector (`vtxfac`) or of the sector pair
//! (`pair_search`). A Zero there becomes `On`, and `On` is a verdict:
//! "this chord lies in the plane". A long chord beside a short one
//! reads Zero while its far end stands hundreds of bands off.
//!
//! Each fixture poses that: a needle whose short (1 mm) edges set the
//! arm while its long (10 m) edge dips `500·ε` over its length. The
//! levered reading is `ε/20`; the far end is `500·ε` into the other
//! solid. The rows pin what backstops the wrong `On`:
//! - a single On chord drops or moves a section germ at the vertex, so
//!   the two faces flanking that chord each keep a section end with no
//!   partner, and the join refuses `UnpairedLooseEnds`;
//! - an all-On sector pair (the faces read parallel at the same arm)
//!   descends to the carrier ladder, which refuses an undeclared
//!   coincidence, and a declared one is contradicted at the door.
//!
//! Every refusal of an undeclared pose is a false one: the control
//! beside it, the same dip under 1 m edges so the reading is definite,
//! answers correctly. The splitting lane's twin reads a line edge at
//! its far vertex, in metres, and splits the short-armed needle
//! correctly.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{
    Body, BooleanError, BooleanResult, CarrierDesc, FaceKey, SolidContainment, SplitJoinError,
    SplitPlane, face_carrier, mass_properties, point_in_solid, split, subtract, subtract_with,
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

fn is_unpaired_join(r: &Result<BooleanResult<f64>, BooleanError>) -> bool {
    matches!(
        r,
        Err(BooleanError::Join(SplitJoinError::UnpairedLooseEnds { .. }))
    )
}

fn body_of(r: Result<BooleanResult<f64>, BooleanError>, what: &str) -> Body<f64> {
    let r = r.unwrap_or_else(|e| panic!("{what}: the control answers, got {e}"));
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

fn contains(body: &Body<f64>, q: Point3<f64>) -> SolidContainment {
    let tol = Tol::witness();
    point_in_solid(body, q, Band::linear(tol).unwrap(), tol).unwrap()
}

/// `vtxfac`: the needle's tip on a slab's top face. The dipping edge
/// reads On against the pierced plane at the 1 mm arm; on-edge
/// resolution hands it its neighbours' Out, so the tip mints no
/// germs, and the two long faces' sections (from the tip to the
/// crossings near the far end) are left unpaired.
#[test]
fn a_pierce_reads_a_dipping_edge_on_at_a_short_arm_and_the_join_refuses() {
    let tol = Tol::witness();
    let slab = common::brick::<f64>((-20.0, 20.0), (-20.0, 20.0), (-5.0, 0.0), tol);
    let rise = [[0.0, 1.0, 1.0], [0.0, 0.0, 1.0]];
    let short = parallelepiped(needle(10.0, 0.0, 1e-3, rise));
    for (what, r) in [
        ("slab - needle", subtract(&slab, &short, tol)),
        ("needle - slab", subtract(&short, &slab, tol)),
        ("union", union(&slab, &short, tol)),
    ] {
        assert!(
            is_unpaired_join(&r),
            "{what}: the levered On is caught at the join, got {:?}",
            r.as_ref().err()
        );
    }

    // The control: the same dip under 1 m edges reads definite.
    let e = needle(10.0, 0.0, 1.0, rise);
    let control = parallelepiped(e);
    let dip = DIP * tol.eps();
    let q = at(e, 0.99, 0.1 * dip, 0.1 * dip);
    assert_eq!(
        contains(&control, q),
        SolidContainment::In,
        "q in the needle"
    );
    assert_eq!(contains(&slab, q), SolidContainment::In, "q in the slab");
    let cut = body_of(subtract(&slab, &control, tol), "slab - needle");
    assert_eq!(
        contains(&cut, q),
        SolidContainment::Out,
        "the sliver is cut"
    );
    let joined = body_of(union(&slab, &control, tol), "union");
    assert_eq!(contains(&joined, q), SolidContainment::In, "union holds q");
}

/// `pair_search`: the needle's tip on a block's corner, the dipping
/// edge over the block's top. The pair arm is the needle's 1 mm, so
/// the edge reads On against the block's top plane, and the germ the
/// crossing needs is not minted.
#[test]
fn a_vertex_pair_reads_a_dipping_chord_on_at_a_short_arm_and_the_join_refuses() {
    let tol = Tol::witness();
    let block = common::brick::<f64>((0.0, 20.0), (0.0, 20.0), (-20.0, 0.0), tol);
    let rise = [[-1.0, 0.0, 1.0], [0.0, -1.0, 1.0]];
    let short = parallelepiped(needle(7.0, 7.0, 1e-3, rise));
    for (what, r) in [
        ("block - needle", subtract(&block, &short, tol)),
        ("union", union(&block, &short, tol)),
    ] {
        assert!(
            is_unpaired_join(&r),
            "{what}: the levered On is caught at the join, got {:?}",
            r.as_ref().err()
        );
    }

    let e = needle(7.0, 7.0, 1.0, rise);
    let control = parallelepiped(e);
    let dip = DIP * tol.eps();
    let q = at(e, 0.99, 0.1 * dip, 0.1 * dip);
    assert_eq!(
        contains(&control, q),
        SolidContainment::In,
        "q in the needle"
    );
    assert_eq!(contains(&block, q), SolidContainment::In, "q in the block");
    let cut = body_of(subtract(&block, &control, tol), "block - needle");
    assert_eq!(
        contains(&cut, q),
        SolidContainment::Out,
        "the sliver is cut"
    );
    let joined = body_of(union(&block, &control, tol), "union");
    assert_eq!(contains(&joined, q), SolidContainment::In, "union holds q");
}

/// The all-On arm: a wedge whose bottom shares the block's corner and
/// tilts `500·ε` over 10 m beside a 1.4 mm edge. The sector pair
/// reads parallel, and every bound On, at that arm, so the record
/// goes to the carrier ladder. Undeclared, the ladder refuses the
/// coincidence, though the far end stands 500 bands off; declared,
/// the door's verification contradicts it.
#[test]
fn a_sector_pair_read_coplanar_at_a_short_arm_refuses_at_the_carrier_ladder() {
    let tol = Tol::witness();
    let dip = DIP * tol.eps();
    let block = common::brick::<f64>((0.0, 20.0), (0.0, 20.0), (-20.0, 0.0), tol);
    let wedge = parallelepiped([[10.0, 1.0, -dip], [-1e-3, 1e-3, 0.0], [0.0, 0.0, 1e-3]]);
    for (what, r) in [
        ("block - wedge", subtract(&block, &wedge, tol)),
        ("union", union(&block, &wedge, tol)),
    ] {
        assert!(
            matches!(r, Err(BooleanError::UndeclaredCoincidence { .. })),
            "{what}: the all-On pair refuses as undeclared, got {:?}",
            r.as_ref().err()
        );
    }

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

/// The splitting twin: a line edge's class is its far vertex's side,
/// in metres, so the short-armed needle splits, and its below part is
/// the sliver `det·dip²/(6·b_z·c_z)` (to 1%: at ε = 1e-12 the
/// sliver is 4e-19 m³, and the volume integral cancels to 3e-4).
#[test]
fn the_splitting_twin_reads_the_dipping_edge_at_its_far_vertex() {
    let tol = Tol::witness();
    let dip = DIP * tol.eps();
    let e = needle(10.0, 0.0, 1e-3, [[0.0, 1.0, 1.0], [0.0, 0.0, 1.0]]);
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
    };
    let r = split(&parallelepiped(e), &plane, tol).unwrap();
    let below = mass_properties(r.below.body().expect("a below part"), tol)
        .unwrap()
        .volume;
    let (bz, cz) = (e[1][2], e[2][2]);
    let det = 10.0 * (e[1][1] * cz - bz * e[2][1]);
    let sliver = det * dip * dip / (6.0 * bz * cz);
    assert!(
        (below - sliver).abs() <= 1e-2 * sliver,
        "the below part is the sliver: {below} vs {sliver}"
    );
}
