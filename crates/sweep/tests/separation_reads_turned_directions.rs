//! **The separation doors read directions that turn with the pair**
//! (`topo::Separation`, `topo::SolidSeparation`): behind a box overlap,
//! each face pair is read as the census's arm 1 reads it, so two bodies
//! clear of each other are certified apart however they are turned, as
//! placements of one prototype and as two solids of one body; a rod in a
//! tube's bore, nested along every direction, is certified apart by its
//! gap in depth about the axis; and a pair that touches is denied at
//! every turn.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, RigidMotion, Tol, UnitVec3, Vec3};
use topo::{Body, Separation, SolidSeparation};

fn tol() -> Tol {
    Tol::witness()
}

/// A full revolve about `y` of the rectangle `x ∈ [inner, outer]`,
/// `y ∈ [0, h]`: a rod where `inner` is zero, a tube otherwise.
fn revolved(inner: f64, outer: f64, h: f64) -> Body<f64> {
    sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(inner, 0.0), 0.0),
            (Point2::new(outer, 0.0), 0.0),
            (Point2::new(outer, h), 0.0),
            (Point2::new(inner, h), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    )
}

/// The rod `r ≤ 0.4` over `y ∈ [0, 1]`.
fn rod() -> Body<f64> {
    revolved(0.0, 0.4, 1.0)
}

/// The unit cube `[0, 1]³`.
fn cube() -> Body<f64> {
    sweep::test_support::cube(1.0, tol())
}

fn axis(x: f64, y: f64, z: f64) -> UnitVec3<f64> {
    UnitVec3::new(
        Vec3::new(x, y, z),
        "test_axis",
        geom_core::Band::linear(tol()).unwrap(),
    )
    .unwrap()
}

/// The turn by `deg` degrees about `axis` through `about`, then the move
/// by `by`.
fn turned(axis: UnitVec3<f64>, about: Point3<f64>, deg: f64, by: Vec3<f64>) -> RigidMotion<f64> {
    RigidMotion::rotation_about_axis(about, axis, deg.to_radians())
        .unwrap()
        .translated(by)
        .unwrap()
}

fn moved(x: f64, y: f64, z: f64) -> RigidMotion<f64> {
    RigidMotion::translation(Vec3::new(x, y, z)).unwrap()
}

/// Two solids of one body: `a` where it stands and `b` moved by `mb`,
/// the whole pair then moved by `whole`.
fn two_solids(
    a: &Body<f64>,
    b: &Body<f64>,
    mb: &RigidMotion<f64>,
    whole: &RigidMotion<f64>,
) -> Body<f64> {
    let placed = topo::transform_rigid(b, mb).unwrap();
    let mut out = a.clone();
    topo::graft_disjoint(&mut out, &placed).expect("the placed copy grafts");
    topo::transform_rigid(&out, whole).unwrap()
}

/// Whether the body's two solids certify apart.
fn certified(body: &Body<f64>) -> bool {
    let sep = SolidSeparation::of(body, tol()).expect("boxes");
    let keys: Vec<_> = sep.keys().collect();
    assert_eq!(keys.len(), 2, "two solids");
    sep.certify(keys[0], keys[1]).is_ok()
}

/// Whether placements `identity` and `mj` of `proto` certify apart.
fn placed_apart(proto: &Body<f64>, mj: RigidMotion<f64>) -> bool {
    let sep = Separation::of(proto, tol()).expect("boxes");
    match sep.apart(&[RigidMotion::identity(), mj]).as_slice() {
        [] => false,
        [(0, 1)] => true,
        other => panic!("one pair, got {other:?}"),
    }
}

/// The relative turns each row is read at, in degrees.
const TURNS: [f64; 7] = [0.0, 15.0, 30.0, 45.0, 60.0, 75.0, 90.0];

/// **Two placements of a rod, 0.1 clear, the second spun about its own
/// axis, certify at every spin**: the spin leaves the second copy the
/// same set of points, but carrying its face boxes into the first's
/// frame widens them by up to `√2`. Red on main at 30°, 45° and 60°,
/// where the door read only those boxes; the line between the two
/// walls' axes parts them.
#[test]
fn two_placed_rods_certify_at_every_relative_spin() {
    let proto = rod();
    let mid = Point3::new(0.0, 0.5, 0.0);
    for deg in TURNS {
        let mj = turned(axis(0.0, 1.0, 0.0), mid, deg, Vec3::new(0.9, 0.0, 0.0));
        assert!(placed_apart(&proto, mj), "spun {deg}°: certified apart");
    }
}

/// **Two placements of a rod whose walls touch are denied at every
/// spin**: the second at `x = 0.8` meets the first along a ruling, and
/// no reading parts a pair with no gap.
#[test]
fn two_placed_rods_that_touch_are_denied_at_every_relative_spin() {
    let proto = rod();
    let mid = Point3::new(0.0, 0.5, 0.0);
    for deg in TURNS {
        let mj = turned(axis(0.0, 1.0, 0.0), mid, deg, Vec3::new(0.8, 0.0, 0.0));
        assert!(!placed_apart(&proto, mj), "spun {deg}°: denied");
    }
}

/// **Two placements of a cube, 0.1 clear, the second turned about `z`
/// about its own centre and moved clear by its half-diagonal, certify at
/// every turn.**
#[test]
fn two_placed_cubes_certify_at_every_relative_turn() {
    let proto = cube();
    let centre = Point3::new(0.5, 0.5, 0.5);
    for deg in TURNS {
        let reach = 0.5 * (deg.to_radians().cos().abs() + deg.to_radians().sin().abs());
        let mj = turned(
            axis(0.0, 0.0, 1.0),
            centre,
            deg,
            Vec3::new(0.5 + reach + 0.1, 0.0, 0.0),
        );
        assert!(placed_apart(&proto, mj), "turned {deg}°: certified apart");
    }
}

/// **Two parallel rods of one body, 0.1 clear, certify however the pair
/// is turned**, about their axes' direction and across it. Red on main
/// from 15° to 75° about `z`, and from 30° to 60° about `y`, where the
/// door read world-axis face boxes alone.
#[test]
fn two_solid_rods_certify_however_the_pair_is_turned() {
    let rod = rod();
    let mid = Point3::new(0.45, 0.5, 0.0);
    for (about, axis) in [("y", axis(0.0, 1.0, 0.0)), ("z", axis(0.0, 0.0, 1.0))] {
        for deg in TURNS {
            let body = two_solids(
                &rod,
                &rod,
                &moved(0.9, 0.0, 0.0),
                &turned(axis, mid, deg, Vec3::new(0.0, 0.0, 0.0)),
            );
            assert!(certified(&body), "turned {deg}° about {about}: certified");
        }
    }
}

/// **Two cubes of one body, 0.1 clear, certify however the pair is
/// turned about `z`.** A plane carries no axis or centre to read a depth
/// about, so between 0° and 90° only a direction that turns with the
/// pair parts them: a face's own normal, or the axis between two faces'
/// anchors. This row goes red if the door drops those directions.
#[test]
fn two_solid_cubes_certify_however_the_pair_is_turned() {
    let cube = cube();
    let mid = Point3::new(1.05, 0.5, 0.5);
    for deg in TURNS {
        let body = two_solids(
            &cube,
            &cube,
            &moved(1.1, 0.0, 0.0),
            &turned(axis(0.0, 0.0, 1.0), mid, deg, Vec3::new(0.0, 0.0, 0.0)),
        );
        assert!(certified(&body), "turned {deg}°: certified");
    }
}

/// **Two cubes of one body sharing a face are denied however the pair is
/// turned.**
#[test]
fn two_solid_cubes_sharing_a_face_are_denied_however_the_pair_is_turned() {
    let cube = cube();
    let mid = Point3::new(1.0, 0.5, 0.5);
    for deg in TURNS {
        let body = two_solids(
            &cube,
            &cube,
            &moved(1.0, 0.0, 0.0),
            &turned(axis(0.0, 0.0, 1.0), mid, deg, Vec3::new(0.0, 0.0, 0.0)),
        );
        assert!(!certified(&body), "turned {deg}°: denied");
    }
}

/// **A rod in a tube's bore certifies at every turn of the pair**: the
/// rod `r ≤ 0.4` over `y ∈ [0.5, 1.5]` in the tube `r ∈ [0.6, 1]` over
/// `y ∈ [0, 2]`, on the axis and 0.1 off it. Their boxes nest along
/// every direction, so no direction parts them; the gap in depth about
/// the axis does. Red on main at every turn. A rod wider than the bore,
/// `r ≤ 0.8`, is denied at every turn.
#[test]
fn a_rod_in_a_bore_certifies_at_every_turn() {
    let tube = revolved(0.6, 1.0, 2.0);
    let mid = Point3::new(0.0, 1.0, 0.0);
    for (what, inner, by, apart) in [
        ("on the axis", rod(), moved(0.0, 0.5, 0.0), true),
        ("off the axis", rod(), moved(0.1, 0.5, 0.0), true),
        (
            "crossing the bore",
            revolved(0.0, 0.8, 1.0),
            moved(0.0, 0.5, 0.0),
            false,
        ),
    ] {
        for deg in TURNS {
            let body = two_solids(
                &tube,
                &inner,
                &by,
                &turned(axis(0.0, 0.0, 1.0), mid, deg, Vec3::new(0.0, 0.0, 0.0)),
            );
            assert_eq!(certified(&body), apart, "the rod {what}, turned {deg}°");
        }
    }
}
