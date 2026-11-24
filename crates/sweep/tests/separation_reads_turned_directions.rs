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
    placed_apart_from(proto, RigidMotion::identity(), mj)
}

/// Whether placements `mi` and `mj` of `proto` certify apart.
fn placed_apart_from(proto: &Body<f64>, mi: RigidMotion<f64>, mj: RigidMotion<f64>) -> bool {
    let sep = Separation::of(proto, tol()).expect("boxes");
    match sep.apart(&[mi, mj]).as_slice() {
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

/// The quarter turn about the world's `z` axis through the origin: the
/// base the turned-base rows place the first copy at.
fn quarter_z() -> RigidMotion<f64> {
    turned(
        axis(0.0, 0.0, 1.0),
        Point3::new(0.0, 0.0, 0.0),
        90.0,
        Vec3::new(0.0, 0.0, 0.0),
    )
}

/// **A solid in another's material is denied, though their boundaries
/// are apart.** A rod `r ≤ 0.2` over `y ∈ [0.5, 1.5]` stands in the wall
/// of a tube `r ∈ [0.4, 1.2]` over `y ∈ [0, 2]`, at `x = 0.8`, so every
/// face pair is apart in depth about one axis or the other. A ball
/// `r = 0.3` sits at the centre of the unit cube, its sphere clear of
/// every face. Both are denied at every turn: the rod's point and the
/// ball's lie in the other solid's material. Red where the door reads
/// only the boundaries.
#[test]
fn a_solid_inside_anothers_material_is_denied_at_every_turn() {
    let rod_in_wall = |deg| {
        two_solids(
            &revolved(0.4, 1.2, 2.0),
            &revolved(0.0, 0.2, 1.0),
            &moved(0.8, 0.5, 0.0),
            &turned(
                axis(0.0, 0.0, 1.0),
                Point3::new(0.0, 1.0, 0.0),
                deg,
                Vec3::new(0.0, 0.0, 0.0),
            ),
        )
    };
    let ball = sweep::test_support::ball_poled_y(0.3, Vec3::new(0.0, 0.0, 0.0), tol());
    let ball_in_cube = |deg| {
        two_solids(
            &cube(),
            &ball,
            &moved(0.5, 0.5, 0.5),
            &turned(
                axis(1.0, 1.0, 0.0),
                Point3::new(0.5, 0.5, 0.5),
                deg,
                Vec3::new(0.0, 0.0, 0.0),
            ),
        )
    };
    for deg in TURNS {
        assert!(
            !certified(&rod_in_wall(deg)),
            "the rod in the wall, turned {deg}°: denied"
        );
        assert!(
            !certified(&ball_in_cube(deg)),
            "the ball in the cube, turned {deg}°: denied"
        );
    }
}

/// **Two placements of a two-solid prototype, one copy's rod in the
/// other's tube wall, are denied.** The prototype is the tube of
/// [`a_solid_inside_anothers_material_is_denied_at_every_turn`] beside
/// that rod at `x = 5`; the second copy, moved by `(−4.2, 0.5, 0)`,
/// stands its rod at `x = 0.8` in the first copy's wall, every face pair
/// apart. Red where the placement door reads only the boundaries.
#[test]
fn a_placed_copy_in_anothers_material_is_denied() {
    let proto = two_solids(
        &revolved(0.4, 1.2, 2.0),
        &revolved(0.0, 0.2, 1.0),
        &moved(5.0, 0.0, 0.0),
        &RigidMotion::identity(),
    );
    for base in [RigidMotion::identity(), quarter_z()] {
        let mj = base.compose(moved(-4.2, 0.5, 0.0)).unwrap();
        assert!(
            !placed_apart_from(&proto, base, mj),
            "denied from base {base:?}"
        );
    }
}

// Placements read from a turned base: each pair is `[B, B ∘ M]` for a
// turn `B` about `z`, so the copies' relative
// motion is `M`, and a door that composes the two motions in the other
// order reads `B M B⁻¹` instead. Each row below goes red under that
// order.

/// **Two rods 0.1 clear from a turned base, the second spun about its own
/// axis, certify at every spin.** `M` is the spin, then `0.9` along `x`;
/// read as `B M B⁻¹` the second rod slides along the first's axis.
#[test]
fn rods_from_a_turned_base_certify_at_every_spin() {
    let (base, rod) = (quarter_z(), rod());
    for deg in TURNS {
        let m = turned(
            axis(0.0, 1.0, 0.0),
            Point3::new(0.0, 0.5, 0.0),
            deg,
            Vec3::new(0.9, 0.0, 0.0),
        );
        assert!(
            placed_apart_from(&rod, base, base.compose(m).unwrap()),
            "spun {deg}°: certified"
        );
    }
}

/// **Two balls from a turned base certify.** Balls `r = 0.45` about
/// `c = (0.4, 0, 0)`; `M` is the half turn about the `z` line through
/// `q`, `0.5` from `c` on the segment from `c` to `B⁻¹ c`, so the
/// centres lie `1.0` apart on a diagonal, `0.1` clear, their hulls
/// overlapping. Read as `B M B⁻¹` they lie `2 |B⁻¹ c − q| ≈ 0.13` apart.
#[test]
fn balls_from_a_turned_base_certify() {
    let base = quarter_z();
    let c = Point3::new(0.4, 0.0, 0.0);
    let back = Point3::new(0.0, -0.4, 0.0);
    let toward = (back - c) * (0.5 / (back - c).norm());
    let q = c + toward;
    let ball = sweep::test_support::ball_poled_y(0.45, Vec3::new(c.x, c.y, c.z), tol());
    let half = turned(axis(0.0, 0.0, 1.0), q, 180.0, Vec3::new(0.0, 0.0, 0.0));
    assert!(placed_apart_from(&ball, base, base.compose(half).unwrap()));
}

/// **Two bricks `[0, 2] × [0, 1]²` from a base an eighth turn about `z`,
/// stacked 0.1 clear along `y`, certify.** Their hulls overlap on the
/// diagonal. `M` is `1.1` along `y`; read as `B M B⁻¹` it is `1.1` along
/// the diagonal `(−1, 1)/√2`, and the bricks, two long in `x`, overlap.
#[test]
fn bricks_from_a_turned_base_certify() {
    let base = turned(
        axis(0.0, 0.0, 1.0),
        Point3::new(0.0, 0.0, 0.0),
        45.0,
        Vec3::new(0.0, 0.0, 0.0),
    );
    let brick = sweep::test_support::block(2.0, 1.0, 1.0, tol());
    assert!(placed_apart_from(
        &brick,
        base,
        base.compose(moved(0.0, 1.1, 0.0)).unwrap()
    ));
}

/// **The unit cube at the quarter turn and at `(−1, −0.5, 0)`, which
/// overlap, is denied.**
#[test]
fn overlapping_cubes_from_a_turned_base_are_denied() {
    assert!(!placed_apart_from(
        &cube(),
        quarter_z(),
        moved(-1.0, -0.5, 0.0)
    ));
}

/// **A box off a rod's side certifies at every turn of the pair, and
/// one touching the rod is denied.** The rod `r ≤ 0.5` over `y ∈ [0, 1]`
/// and the unit box over the same `y` whose edge nearest the axis runs
/// along `x = z = (0.5 + gap)/√2`, `gap` from the wall (the census's
/// `cross_solid_apart::a_box_off_a_rod_passes_until_it_touches`). No
/// world axis, face normal, anchor axis or depth parts the box from the
/// wall at `gap = 0.04`; the offset from the rod's axis to the box's
/// nearest edge does (`depth_gap::toward`). Red where the door reads no
/// direction the pair's own geometry aims.
#[test]
fn a_box_off_a_rods_side_certifies_until_it_touches() {
    let rod = rod_of(0.5);
    let mid = Point3::new(0.5, 0.5, 0.5);
    for (gap, apart) in [(0.04, true), (0.0, false)] {
        let s = (0.5 + gap) * core::f64::consts::FRAC_1_SQRT_2;
        let block =
            sweep::test_support::brick::<f64>((s, s + 1.0), (0.0, 1.0), (s, s + 1.0), tol());
        for deg in TURNS {
            let body = two_solids(
                &rod,
                &block,
                &RigidMotion::identity(),
                &turned(axis(0.0, 0.0, 1.0), mid, deg, Vec3::new(0.0, 0.0, 0.0)),
            );
            assert_eq!(certified(&body), apart, "gap {gap}, turned {deg}°");
        }
    }
}

/// The rod `r ≤ radius` over `y ∈ [0, 1]`.
fn rod_of(radius: f64) -> Body<f64> {
    revolved(0.0, radius, 1.0)
}
