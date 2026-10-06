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

/// Every point of a lattice over `lo..hi` whose `signed_depth` (positive
/// in the body) is more than 1e-3 from zero is classified by
/// `point_in_solid` as its sign says;
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

/// A second operand: what it is, its radius, its centre, and whether it
/// lies wholly inside the cut body.
type Operand = (&'static str, f64, Vec3<f64>, bool);

/// A cut: its name, the box's `x` range, the cut body's volume, and the
/// operands it meets.
type Cut = (&'static str, (f64, f64), f64, [Operand; 3]);

/// **A ball cut by planes tilted against its chart is an operand.** The
/// `y`-poled unit ball, whose seam meridians lie in `z = 0`, less two
/// boxes, each against the ball's and the box's own point tests and then
/// against a ball nested in the cut, one disjoint from it, and one inside
/// it across a cutting plane where no face of the box is, each against its
/// closed form:
///
/// - the box `[0.5, 3] × [−2, 2] × [0, 2]` (`tilted_sphere_pair.rs`'s
///   pose), whose face `x = 0.5` crosses the chart and keeps the half of
///   the cap beyond it below `z = 0`. Two sphere faces are left;
/// - the slab `[−0.2, 0.2] × [−2, 2] × [0, 2]`, whose faces `x = ±0.2`
///   cut a slot in the `z > 0` half: ONE sphere face is left, whose
///   complement is the slot and not another face, so a region read the
///   wrong way round answers wrongly instead of being covered by its
///   neighbour.
#[test]
fn a_tilted_cut_of_a_ball_is_an_operand() {
    let o = Vec3::new(0.0, 0.0, 0.0);
    let half_slab = (ball_volume(1.0) - 2.0 * cap_volume(1.0, 0.8)) / 2.0;
    let poses: [Cut; 2] = [
        (
            "the box beyond x = 0.5",
            (0.5, 3.0),
            ball_volume(1.0) - cap_volume(1.0, 0.5) / 2.0,
            [
                ("a nested ball", 0.2, Vec3::new(-0.4, 0.0, 0.0), true),
                ("a disjoint ball", 0.3, Vec3::new(4.0, 0.0, 0.0), false),
                (
                    "a ball across x = 0.5, below the box",
                    0.2,
                    Vec3::new(0.6, 0.0, -0.4),
                    true,
                ),
            ],
        ),
        (
            "the slab |x| < 0.2",
            (-0.2, 0.2),
            ball_volume(1.0) - half_slab,
            [
                ("a nested ball", 0.2, Vec3::new(-0.6, 0.0, 0.3), true),
                ("a disjoint ball", 0.3, Vec3::new(4.0, 0.0, 0.0), false),
                (
                    "a ball across x = 0.2, below the slab",
                    0.2,
                    Vec3::new(0.1, 0.0, -0.5),
                    true,
                ),
            ],
        ),
    ];
    for (pose, (x0, x1), v_cut, balls) in poses {
        let box_ = finished(
            "the box",
            brick((x0, x1), (-2.0, 2.0), (0.0, 2.0), Tol::witness()),
            Tol::witness(),
        );
        let cut = run(BooleanOp::Subtract, &ball(1.0, o), &box_)
            .unwrap()
            .body()
            .unwrap()
            .body
            .clone();
        let sphere_faces = cut
            .faces()
            .filter(|(_, f)| {
                matches!(
                    cut.get_surface(f.surface),
                    Some(geom::Surface::Sphere { .. })
                )
            })
            .count();
        assert_eq!(
            sphere_faces,
            if x0 < 0.0 { 1 } else { 2 },
            "{pose}: sphere faces left"
        );
        assert_lattice(
            pose,
            &cut,
            Point3::new(-1.1, -1.1, -1.1),
            Point3::new(1.1, 1.1, 1.1),
            |q| {
                let in_box = (q.x - x0).min(x1 - q.x).min(q.z);
                (1.0 - dist(q, o)).min(-in_box)
            },
        );
        for (what, r, c, inside) in balls {
            let label = format!("{pose}, {what}");
            assert_every_op(
                &label,
                &cut,
                v_cut,
                &ball(r, c),
                ball_volume(r),
                if inside { ball_volume(r) } else { 0.0 },
            );
        }
    }
}

/// **A point a hair off a pole whose antipode is a vertex of the face is
/// read.** The union of the unit ball at `(2, 2, 0.5)` and a ball of r 0.5
/// one unit along `(0.3, 0.9, 0.3)`: the point 1e-9 from A's `+y` pole is
/// 0.063 deep inside B, so in the union. One of A's faces there has a
/// vertex at A's `−y` pole, the point's antipode, which every great
/// circle through the point meets at `s = π`; the arcs a ray can decide
/// on lie behind the point. Asked at the unit scale and at ×1e3.
#[test]
fn a_point_whose_antipode_is_a_face_vertex_is_read() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    for scale in [1.0, 1e3] {
        if scale > 1.0 && tol.eps() < 1e-10 {
            test_utils::vacuity::stood_down(
                "the ×1e3 antipodal pose at this ε",
                "a revolved ball of radius 1e3 is not finished at 1e-12, so this pose is not read",
            );
            continue;
        }
        let d = Vec3::new(0.3, 0.9, 0.3);
        let a = Vec3::new(2.0, 2.0, 0.5) * scale;
        let b = a + d * (scale / d.norm());
        let u = run(BooleanOp::Union, &ball(scale, a), &ball(0.5 * scale, b))
            .unwrap()
            .body()
            .unwrap()
            .body
            .clone();
        let pole = Point3::new(a.x, a.y + scale, a.z);
        let q = pole + Vec3::new(-0.916_549_5, 0.0, -0.399_921_1) * (1e-9 * scale);
        assert!(
            dist(q, b) < 0.5 * scale - 0.06 * scale,
            "the point is deep in B at ×{scale}"
        );
        let got = point_in_solid(&u, q, band, tol)
            .unwrap_or_else(|e| panic!("×{scale}: {q:?} refused: {e:?}"));
        assert_eq!(got, SolidContainment::In, "×{scale}");
    }
}

/// **A face whose only edge is a seam is the whole sphere.** Killing one
/// of a ball's two meridians (`kef`) leaves ONE sphere face whose loop
/// runs the other meridian pole to pole and back: both half-edges of
/// that edge are the face's, so it bounds nothing, and every point of the
/// sphere off it is in the face. Every great circle through such a point
/// crosses the meridian once, at a crossing the loop holds twice, so a
/// reading that crossed the seam would tie there on every ray. Asked at
/// the face door, which reads the face whatever group it sits in.
#[test]
fn a_face_whose_only_edge_is_a_seam_is_the_whole_sphere() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let mut slit = revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Full,
        tol,
    );
    let meridian = slit.edges().next().map(|(_, e)| e.he_plus).unwrap();
    slit.kef(meridian).expect("one meridian kills");
    let faces: Vec<_> = slit.faces().map(|(k, _)| k).collect();
    assert_eq!(faces.len(), 1, "one sphere face is left");
    for (x, y, z) in [
        (0.6, 0.0, 0.8),
        (-0.6, 0.0, -0.8),
        (0.0, 0.6, 0.8),
        (0.48, -0.6, -0.64),
        (0.0, 0.999, 0.0447102),
    ] {
        let w = Vec3::new(x, y, z);
        let q = Point3::origin() + w * (1.0 / w.norm());
        assert_eq!(
            topo::curved_face_containment(&slit, faces[0], q, band).unwrap(),
            Some(topo::FaceContainment::In),
            "{q:?}"
        );
    }
}

/// **The region is read at the `Interval` scalar too.** The lens union
/// built from enclosures answers the item's four queries as the `f64`
/// body does. At ε 1e-12 the union itself does not build at this scalar
/// (`tilted_sphere_pair`'s interval row pins the mint's escalation), so
/// nothing is asked there.
#[test]
fn the_lens_union_classifies_points_at_the_interval_scalar() {
    use crate::common::interval::iv;
    use geom_core::Interval;
    let tol = Tol::witness();
    if tol.eps() < 1e-10 {
        test_utils::vacuity::stood_down(
            "the interval lens union at this ε",
            "the union does not build at the Interval scalar at 1e-12, so its region is not read",
        );
        return;
    }
    let ball_iv = |c: Vec3<f64>| -> AtRestBody<Interval> {
        let b = sweep::test_support::revolved_about_y_at::<Interval>(
            vec![
                (Point2::new(iv(0.0), iv(-1.0)), iv(1.0)),
                (Point2::new(iv(0.0), iv(1.0)), iv(0.0)),
            ],
            Revolution::Full,
            tol,
        );
        let to = Vec3::new(iv(c.x), iv(c.y), iv(c.z));
        finished(
            "the ball",
            topo::transform_rigid(&b, &Affine3::translation(to), tol).unwrap(),
            tol,
        )
    };
    let u = topo::boolean::union(&ball_iv(A), &ball_iv(B), tol)
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    let band = Band::linear(tol).unwrap();
    for ((x, y, z), want) in [
        ((2.7, 2.0, 0.5), SolidContainment::In),
        ((2.7, 2.0, 1.4), SolidContainment::Out),
        ((4.3, 2.0, 0.5), SolidContainment::In),
        ((2.7, 2.6, 0.5), SolidContainment::In),
    ] {
        let q = Point3::new(iv(x), iv(y), iv(z));
        let got = point_in_solid(&u, q, band, tol)
            .unwrap_or_else(|e| panic!("({x}, {y}, {z}) refused: {e:?}"));
        assert_eq!(got, want, "({x}, {y}, {z})");
    }
}
