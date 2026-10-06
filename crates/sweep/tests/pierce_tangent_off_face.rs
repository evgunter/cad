//! **An edge tangent to a curved carrier where the face is not.** A
//! line edge of one operand touches the carrier of the other's curved
//! face at one point, and that point lies on a part of the carrier the
//! face does not hold: trimmed away by a lens's other sphere, or in the
//! mouth a 270° extrusion or revolution leaves open. The bodies are
//! disjoint, so every op builds, in both operand orders, against the
//! volumes of the operands computed here from their dimensions alone.
//! The brick is turned about the touching edge so that its two faces
//! through it leave the carrier at 45°: only the edge touches.
//!
//! The near-miss rows slide the touch across the lens's rim. Outside by
//! a few balls the touch is no event; inside it, it is a tangency of the
//! two boundaries, and within the ball of the rim it is not certified
//! clear of the face's edge: both refuse typed. The ball's radius is
//! `√(2r·(zero + escalate))` (`reduce::CarrierTouch`), computed here
//! from the run's band.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_1_SQRT_2, PI};

use geom_core::{Affine3, Band, Mat3, Point2, Tol, Vec2, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, BooleanError, BooleanOp};

const R1: f64 = 1.0;
const R2: f64 = 0.8;
const D: f64 = 1.4;

/// A ball of radius `r` centred at `(0, y, 0)`, poled on y.
fn ball(r: f64, y: f64) -> Body<f64> {
    let b = sweep::test_support::revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    moved(&b, &Affine3::translation(Vec3::new(0.0, y, 0.0)))
}

fn moved(b: &Body<f64>, m: &Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(b, m, Tol::witness()).unwrap()
}

fn boolean(
    op: BooleanOp,
    a: &Body<f64>,
    b: &Body<f64>,
) -> Result<topo::BooleanResult<f64>, BooleanError> {
    let (a, b) = (
        &sweep::test_support::finished("operand A", a.clone(), Tol::witness()),
        &sweep::test_support::finished("operand B", b.clone(), Tol::witness()),
    );
    match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    }
}

/// The boolean's body, `None` for an empty result; a refusal fails with
/// the payload.
fn built(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Option<Body<f64>> {
    boolean(op, a, b)
        .unwrap_or_else(|e| panic!("{op:?} refused: {e:?}"))
        .body()
        .map(|b| (*b.body).clone())
}

/// The lens `ball(R1, 0) ∩ ball(R2, D)`. Its unit-sphere face is the cap
/// above the radical plane `y = RIM`.
fn lens() -> Body<f64> {
    built(BooleanOp::Intersect, &ball(R1, 0.0), &ball(R2, D)).expect("the lens")
}

/// The height of the lens's rim, the radical plane of its two spheres.
const RIM: f64 = (D * D + R1 * R1 - R2 * R2) / (2.0 * D);

/// The volume of a spherical cap of height `h` on a sphere of radius `r`.
fn cap_volume(r: f64, h: f64) -> f64 {
    PI * h.powi(2) * (3.0 * r - h) / 3.0
}

fn lens_volume() -> f64 {
    cap_volume(R1, R1 - RIM) + cap_volume(R2, R2 - (D - RIM))
}

/// The brick `[−h, h] × [0, 1] × [0, 1]` carried so that its x edge at
/// the origin runs along `along` through `at`, its faces through that
/// edge leaving along `(n ± m)/√2`: at 45° to the plane of `along` and
/// `m`, on `n`'s side of it. Volume `2h`.
fn edge_brick(h: f64, at: Vec3<f64>, along: Vec3<f64>, n: Vec3<f64>, m: Vec3<f64>) -> Body<f64> {
    let (s1, s2) = ((n + m) * FRAC_1_SQRT_2, (n - m) * FRAC_1_SQRT_2);
    let (s1, s2) = if along.cross(s1).dot(s2) > 0.0 {
        (s1, s2)
    } else {
        (s2, s1)
    };
    let brick = sweep::test_support::brick((-h, h), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    moved(
        &brick,
        &Affine3::from_parts(Mat3::from_cols(along, s1, s2), at),
    )
}

/// A brick of half-length `h` whose edge touches the unit sphere at
/// polar angle `theta` from `+y` in the plane `x = 0`, along x, leaving
/// outward: it holds the closed half-space beyond that tangent plane,
/// which meets the unit ball at the touch alone.
fn brick_touching_unit_sphere(h: f64, theta: f64) -> Body<f64> {
    let n = Vec3::new(0.0, theta.cos(), theta.sin());
    let m = Vec3::new(0.0, -theta.sin(), theta.cos());
    edge_brick(h, n, Vec3::new(1.0, 0.0, 0.0), n, m)
}

/// Every tier of validation, a closed tessellation, then the volume
/// against `want` through the kernel's mass properties (exact on
/// closed-form faces), to 1e-9 of the result floored at unit scale.
fn assert_body(label: &str, body: &Body<f64>, want: f64) {
    let tol = Tol::witness();
    assert_eq!(topo::validate(body), Ok(()), "{label}: validate");
    assert_eq!(
        topo::validate_closed(body),
        Ok(()),
        "{label}: validate_closed"
    );
    assert_eq!(
        topo::validate_geometric(body, tol),
        Ok(()),
        "{label}: validate_geometric"
    );
    let m = mesh::tessellate(body, 1e-3, tol)
        .unwrap_or_else(|e| panic!("{label}: tessellates, got {e:?}"));
    assert_eq!(
        mesh::validate::check_mesh(&m),
        Ok(()),
        "{label}: a closed manifold mesh"
    );
    let p = topo::mass_properties(body, tol)
        .unwrap_or_else(|e| panic!("{label}: mass properties, got {e:?}"));
    assert_eq!(p.volume_pad, 0.0, "{label}: closed-form faces only");
    assert!(
        (p.volume - want).abs() <= 1e-9 * want.max(1.0),
        "{label}: volume {} against the closed form {want}",
        p.volume
    );
}

/// Every op in both operand orders on disjoint operands of volumes `va`,
/// `vb`: the union is both, the intersection empty, each difference its
/// minuend.
fn assert_every_op_apart(label: &str, a: &Body<f64>, b: &Body<f64>, (va, vb): (f64, f64)) {
    for (op, x, y, want) in [
        (BooleanOp::Union, a, b, va + vb),
        (BooleanOp::Union, b, a, va + vb),
        (BooleanOp::Intersect, a, b, 0.0),
        (BooleanOp::Intersect, b, a, 0.0),
        (BooleanOp::Subtract, a, b, va),
        (BooleanOp::Subtract, b, a, vb),
    ] {
        let name = format!("{label}: {op:?}");
        match built(op, x, y) {
            Some(out) => assert_body(&name, &out, want),
            None => assert!(want == 0.0, "{name}: empty against {want}"),
        }
    }
}

const OPS: [BooleanOp; 3] = [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract];

/// Every op in both operand orders refuses, and `expect` holds of each
/// refusal.
fn assert_every_op_refuses(
    label: &str,
    a: &Body<f64>,
    b: &Body<f64>,
    expect: fn(&BooleanError) -> bool,
) {
    for op in OPS {
        for (order, x, y) in [("A·B", a, b), ("B·A", b, a)] {
            match boolean(op, x, y) {
                Err(e) => assert!(expect(&e), "{label} {op:?} {order}: got {e:?}"),
                Ok(_) => panic!("{label} {op:?} {order}: built across a touch it cannot clear"),
            }
        }
    }
}

fn pierce_refusal(e: &BooleanError) -> bool {
    matches!(e, BooleanError::CurvedPierceUnsupported { .. })
}

/// The radius of the ball about a touch on a carrier of radius `r`.
fn reach(r: f64) -> f64 {
    let band = Band::linear(Tol::witness()).unwrap();
    (2.0 * r * (band.zero() + band.escalate())).sqrt()
}

/// **Sphere, the item's pose.** The brick `[−1, 1] × [−2, −1] × [0, 1]`
/// has its edge `y = −1, z = 0` tangent to the unit sphere at
/// `(0, −1, 0)`, far below the lens's unit-sphere face.
#[test]
fn a_brick_edge_tangent_to_the_lens_sphere_below_its_face_builds() {
    let brick: Body<f64> =
        sweep::test_support::brick((-1.0, 1.0), (-2.0, -1.0), (0.0, 1.0), Tol::witness());
    assert_every_op_apart("lens, brick", &lens(), &brick, (lens_volume(), 2.0));
}

/// **Sphere, only the edge touching.** The turned brick touches the unit
/// sphere at polar angle `π/2`, on the equator, which the lens's face
/// (above the rim) does not reach.
#[test]
fn a_turned_brick_touching_the_lens_sphere_by_its_edge_alone_builds() {
    let brick = brick_touching_unit_sphere(0.5, PI / 2.0);
    assert_every_op_apart("lens, turned brick", &lens(), &brick, (lens_volume(), 1.0));
}

/// The polar angle of the lens's rim on the unit sphere.
fn rim_angle() -> f64 {
    (RIM / R1).acos()
}

/// **Just outside the face, clear of its rim by three balls**: no event.
#[test]
fn a_touch_just_outside_the_lens_face_builds() {
    let brick = brick_touching_unit_sphere(0.25, rim_angle() + 3.0 * reach(R1));
    assert_every_op_apart("lens, brick outside", &lens(), &brick, (lens_volume(), 0.5));
}

/// **Just inside the face**: the brick's edge touches the lens's own
/// boundary, which the crossing layer cannot represent, and it refuses
/// at the pierce.
#[test]
fn a_touch_just_inside_the_lens_face_refuses() {
    let brick = brick_touching_unit_sphere(0.25, rim_angle() - 3.0 * reach(R1));
    assert_every_op_refuses("lens, brick inside", &lens(), &brick, pierce_refusal);
}

/// **Outside the face but inside the ball of its rim**: the touch places
/// `Out`, but the rim reaches the ball, so the face may hold part of the
/// cap the touch's crossings can land in. It refuses at the pierce.
#[test]
fn a_touch_outside_the_lens_face_within_reach_of_its_rim_refuses() {
    let brick = brick_touching_unit_sphere(0.25, rim_angle() + 0.3 * reach(R1));
    assert_every_op_refuses("lens, brick at the rim", &lens(), &brick, pierce_refusal);
}

/// A 270° sector of the unit disc, its mouth the quadrant `x > 0, y < 0`.
fn pac_man() -> Vec<(Point2<f64>, f64)> {
    let bulge = (3.0 * PI / 8.0).tan();
    vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), bulge),
        (Point2::new(0.0, -1.0), 0.0),
    ]
}

/// **Cylinder.** The unit 270° extrusion `z ∈ [0, 2]` of [`pac_man`],
/// and a turned brick whose edge touches the wall's carrier at azimuth
/// −45°, height 1: in the mouth, where the wall's box reaches.
#[test]
fn a_brick_edge_tangent_to_a_wall_in_its_mouth_builds() {
    let tol = Tol::witness();
    let profile = Profile::new(SketchPlane::xy(), vec![bulge_loop(pac_man())])
        .validate(tol)
        .unwrap();
    let sector = extrude(
        &profile,
        Extrusion::Distance {
            depth: 2.0,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    let c = FRAC_1_SQRT_2;
    let n = Vec3::new(c, -c, 0.0);
    let brick = edge_brick(
        0.3,
        n + Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(c, c, 0.0),
        n,
        Vec3::new(0.0, 0.0, 1.0),
    );
    assert_every_op_apart("sector, brick", &sector, &brick, (1.5 * PI, 0.6));
}

/// **Torus.** The 270° revolution of the donut (`R = 2`, `r = 1/2`)
/// about y, and a turned brick whose edge grazes the torus on its outer
/// equator in the mouth. The line × torus quartic answers a graze as an
/// uncertain count, which the localization reads.
#[test]
fn a_brick_edge_grazing_a_torus_in_its_mouth_builds() {
    let tol = Tol::witness();
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![bulge_loop(vec![
            (Point2::new(2.0, -0.5), 1.0),
            (Point2::new(2.0, 0.5), 1.0),
        ])],
    )
    .validate(tol)
    .unwrap();
    let donut = revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(1.5 * PI),
        tol,
    )
    .unwrap()
    .body;
    let c = FRAC_1_SQRT_2;
    let n = Vec3::new(c, 0.0, c);
    let brick = edge_brick(
        0.5,
        n * 2.5,
        Vec3::new(-c, 0.0, c),
        n,
        Vec3::new(0.0, 1.0, 0.0),
    );
    let donut_volume = 0.75 * 2.0 * PI.powi(2) * 2.0 * 0.5f64.powi(2);
    assert_every_op_apart("donut, brick", &donut, &brick, (donut_volume, 1.0));
}

/// **A circle edge.** A coin of radius 0.3 and thickness 0.5 on the x
/// axis, its rim at `x = 0` centred `(0, −1.3, 0)`: that rim circle
/// touches the unit sphere at `(0, −1, 0)`, below the lens's face, and
/// the circle × sphere door answers the tangency as uncertain.
#[test]
fn a_rim_circle_tangent_to_the_lens_sphere_below_its_face_builds() {
    let tol = Tol::witness();
    let r = 0.3;
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![
            profile::circle(Point2::new(0.0, -1.3), r, tol)
                .unwrap()
                .into(),
        ],
    )
    .validate(tol)
    .unwrap();
    let rod = extrude(
        &profile,
        Extrusion::Distance {
            depth: 0.5,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    // A quarter about y carries `(x, y, z)` to `(z, y, −x)`.
    let coin = moved(
        &rod,
        &Affine3::rotation_about_axis(
            geom_core::Point3::origin(),
            Vec3::new(0.0, 1.0, 0.0),
            PI / 2.0,
        ),
    );
    assert_every_op_apart(
        "lens, coin",
        &lens(),
        &coin,
        (lens_volume(), PI * r * r * 0.5),
    );
}
