//! **A carved sphere body is classified against and reused as an
//! operand.** A tilted section leaves sphere faces bounded by circles
//! that are neither rims nor meridians of the face's chart: the union of
//! two unit balls 1.4 apart along `x`, cut by their radical-plane circle,
//! and a ball cut by a plane tilted against its chart. Each such body is
//! queried point by point against the balls' own distance tests, and
//! handed to the next boolean against a ball nested in it, one disjoint
//! from it, and one at the lens centre crossing nothing of its boundary,
//! under every op in both orders, each against its closed form.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{brick, finished, revolved_about_y};
use topo::{AtRestBody, BooleanOp, SolidContainment, point_in_solid};

/// A ball of radius `r` centred at `c`, poles on world `y`.
fn ball(r: f64, c: Vec3<f64>) -> AtRestBody<f64> {
    let b = revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    let b = topo::transform_rigid(&b, &Affine3::translation(c), Tol::witness()).unwrap();
    finished("the ball", b, Tol::witness())
}

fn ball_volume(r: f64) -> f64 {
    4.0 / 3.0 * PI * r.powi(3)
}

/// The volume of a spherical cap of height `h` on a sphere of radius `r`.
fn cap_volume(r: f64, h: f64) -> f64 {
    PI * h.powi(2) * (3.0 * r - h) / 3.0
}

fn run(
    op: BooleanOp,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
) -> Result<topo::BooleanResult<f64>, topo::BooleanError> {
    match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    }
}

/// `op(a, b)` is a body of volume `expected` held to every tier, or, for
/// `expected == 0`, empty.
fn assert_op(label: &str, op: BooleanOp, a: &AtRestBody<f64>, b: &AtRestBody<f64>, expected: f64) {
    let out = run(op, a, b).unwrap_or_else(|e| panic!("{label} refused: {e:?}"));
    let Some(bb) = out.body() else {
        assert_eq!(expected, 0.0, "{label}: came back empty");
        return;
    };
    assert!(expected > 0.0, "{label}: a body where none is owed");
    let tol = Tol::witness();
    topo::validate_closed(&bb.body).unwrap_or_else(|e| panic!("{label}: tier 2: {e:?}"));
    topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol)
        .unwrap_or_else(|e| panic!("{label}: tier 3′: {e:?}"));
    topo::validate_geometric_certificate(&bb.body, tol)
        .unwrap_or_else(|e| panic!("{label}: certificate: {e:?}"));
    let v = topo::mass_properties(&bb.body, tol)
        .unwrap_or_else(|e| panic!("{label}: mass properties: {e:?}"))
        .volume;
    assert!(
        (v - expected).abs() <= 1e-9 * expected.max(1.0),
        "{label}: volume {v} against the closed form {expected}"
    );
}

/// The carved body `u` (volume `vu`) against a ball `s` (volume `vs`)
/// sharing volume `shared` with it, under ∪ and ∩ in both orders and
/// both differences.
fn assert_every_op(
    pose: &str,
    u: &AtRestBody<f64>,
    vu: f64,
    s: &AtRestBody<f64>,
    vs: f64,
    shared: f64,
) {
    for (label, op, x, y, expected) in [
        ("U ∪ S", BooleanOp::Union, u, s, vu + vs - shared),
        ("S ∪ U", BooleanOp::Union, s, u, vu + vs - shared),
        ("U ∩ S", BooleanOp::Intersect, u, s, shared),
        ("S ∩ U", BooleanOp::Intersect, s, u, shared),
        ("U ∖ S", BooleanOp::Subtract, u, s, vu - shared),
        ("S ∖ U", BooleanOp::Subtract, s, u, vs - shared),
    ] {
        assert_op(&format!("{pose}, {label}"), op, x, y, expected);
    }
}

/// Every point of a lattice over `lo..hi` that `inside` decides by more
/// than `clearance` is classified by `point_in_solid` as `inside` says;
/// the count of points read on each side guards against a lattice that
/// misses the body.
fn assert_lattice(
    label: &str,
    body: &AtRestBody<f64>,
    lo: Point3<f64>,
    hi: Point3<f64>,
    signed_depth: impl Fn(Point3<f64>) -> f64,
) {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let n = 7;
    let (mut ins, mut outs) = (0, 0);
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let at = |l: f64, h: f64, s: usize| l + (h - l) * (s as f64 + 0.5) / n as f64;
                let q = Point3::new(at(lo.x, hi.x, i), at(lo.y, hi.y, j), at(lo.z, hi.z, k));
                let depth = signed_depth(q);
                if depth.abs() < 1e-3 {
                    continue;
                }
                let want = if depth > 0.0 {
                    ins += 1;
                    SolidContainment::In
                } else {
                    outs += 1;
                    SolidContainment::Out
                };
                let got = point_in_solid(body, q, band, tol)
                    .unwrap_or_else(|e| panic!("{label}: {q:?} refused: {e:?}"));
                assert_eq!(got, want, "{label}: {q:?}");
            }
        }
    }
    assert!(ins > 20 && outs > 20, "{label}: {ins} in, {outs} out");
}

const A: Vec3<f64> = Vec3::new(2.0, 2.0, 0.5);
const B: Vec3<f64> = Vec3::new(3.4, 2.0, 0.5);

/// The union of the two unit balls, and its volume: both balls less
/// the lens, whose caps beyond the radical plane `x = 2.7` are each of
/// height `0.3`.
fn lens_union() -> (AtRestBody<f64>, f64) {
    let u = run(BooleanOp::Union, &ball(1.0, A), &ball(1.0, B))
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    (u, 2.0 * ball_volume(1.0) - 2.0 * cap_volume(1.0, 0.3))
}

fn dist(q: Point3<f64>, c: Vec3<f64>) -> f64 {
    (q - Point3::new(c.x, c.y, c.z)).norm()
}

/// **Every point query on the lens union answers**, against the two
/// balls' distance tests: the item's four queries (in the lens, above
/// it outside both balls, in B beyond A, and in both off the seam
/// plane), then a lattice over the union's box.
#[test]
fn the_lens_union_classifies_every_point() {
    let (u, _) = lens_union();
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    for (q, want) in [
        (Point3::new(2.7, 2.0, 0.5), SolidContainment::In),
        (Point3::new(2.7, 2.0, 1.4), SolidContainment::Out),
        (Point3::new(4.3, 2.0, 0.5), SolidContainment::In),
        (Point3::new(2.7, 2.6, 0.5), SolidContainment::In),
    ] {
        let got = point_in_solid(&u, q, band, tol).unwrap_or_else(|e| panic!("{q:?}: {e:?}"));
        assert_eq!(got, want, "{q:?}");
    }
    assert_lattice(
        "the lens union",
        &u,
        Point3::new(0.9, 0.9, -0.6),
        Point3::new(4.5, 3.1, 1.6),
        |q| (1.0 - dist(q, A)).max(1.0 - dist(q, B)),
    );
}

/// **The lens union is a later boolean's operand**: a ball nested in it
/// (r 0.2 in the lens), a ball disjoint from it (r 0.3 at `x = 7`), and
/// the r 0.5 ball at the lens centre, which crosses both spheres only
/// where each is inside the other ball and so meets none of the union's
/// boundary.
#[test]
fn the_lens_union_is_an_operand() {
    let (u, vu) = lens_union();
    for (pose, r, c, shared) in [
        (
            "a nested ball",
            0.2,
            Vec3::new(2.7, 2.0, 0.5),
            ball_volume(0.2),
        ),
        ("a disjoint ball", 0.3, Vec3::new(7.0, 2.0, 0.5), 0.0),
        (
            "the ball at the lens centre",
            0.5,
            Vec3::new(2.7, 2.0, 0.5),
            ball_volume(0.5),
        ),
    ] {
        assert_every_op(pose, &u, vu, &ball(r, c), ball_volume(r), shared);
    }
}

/// **A ball cut by a plane tilted against its chart is an operand.** The
/// `y`-poled unit ball less the box `[0.5, 3] × [−2, 2] × [0, 2]`, whose
/// face `x = 0.5` cuts it across its chart, keeps the half of the cap
/// beyond that plane below `z = 0`. It answers point queries against the
/// ball's and the box's own tests, and takes a ball nested in it, one
/// disjoint from it, and one inside it across the plane `x = 0.5` below
/// the box, each against its closed form.
#[test]
fn a_tilted_cut_of_a_ball_is_an_operand() {
    let box_ = finished(
        "the box",
        brick((0.5, 3.0), (-2.0, 2.0), (0.0, 2.0), Tol::witness()),
        Tol::witness(),
    );
    let o = Vec3::new(0.0, 0.0, 0.0);
    let cut = run(BooleanOp::Subtract, &ball(1.0, o), &box_)
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    let v_cut = ball_volume(1.0) - cap_volume(1.0, 0.5) / 2.0;
    assert_lattice(
        "the tilted cut",
        &cut,
        Point3::new(-1.1, -1.1, -1.1),
        Point3::new(1.1, 1.1, 1.1),
        |q| {
            let in_box = (q.x - 0.5).min(q.z);
            (1.0 - dist(q, o)).min(-in_box)
        },
    );
    for (pose, r, c, shared) in [
        (
            "a nested ball",
            0.2,
            Vec3::new(-0.4, 0.0, 0.0),
            ball_volume(0.2),
        ),
        ("a disjoint ball", 0.3, Vec3::new(4.0, 0.0, 0.0), 0.0),
        (
            "a ball across the plane, below the box",
            0.2,
            Vec3::new(0.6, 0.0, -0.4),
            ball_volume(0.2),
        ),
    ] {
        assert_every_op(pose, &cut, v_cut, &ball(r, c), ball_volume(r), shared);
    }
}
