//! **The plain named operands** several suites build a boolean from:
//! the axis-aligned boxes below, the three-arc cylinder and the
//! rounded plate the conic corpus cuts with, and the framed bar the
//! torus-door suites pierce the donut with. Body authoring, so it
//! routes here beside [`super::cavity`] rather than into a suite.
//!
//! Nothing here derives anything — each item is one call to the box
//! door, or one profile extruded — so sharing a fixture cannot make
//! two suites agree by construction about what they check. What it
//! buys is that suites naming the same body get the same body: a row
//! in one suite and its twin in another are then about each other,
//! and a fixture that moves reddens both at once instead of splitting
//! the corpus silently.
//!
//! The boxes are the kernel's own construction, reached through
//! [`sweep::test_support`]'s door; this module holds only their
//! placements and the names the rows read them by. A suite whose own
//! prose calls one of them something else imports it under that name
//! (`use crate::common::operands::slab as plate;`) — a name, not a
//! second body.
//!
//! **Deliberately not absorbed**, and the whole of it — the suites
//! this module drew from are its neighbours, and one box in them
//! stayed:
//!
//! - `s16_box_soundness::top_rim_x_plate`, the only box left in those
//!   suites that just one of them builds. A helper one suite uses
//!   stays in that suite ([`super`]'s routing rule), and the rule does
//!   not bend for a sibling of something that did come here;
//! - the `Point3`-cornered box in [`super::cavity`], which is that
//!   module's own corner vocabulary over the same door;
//! - `super::approx::unit_box`, which is the boolean gate's FACE rule
//!   fixture and belongs with the surgery vocabulary that reads it;
//! - [`super::sphere_recut`]'s `plate`, a box that stays with the ball
//!   it is cut by and the constant measured on the pair;
//! - [`super::shell_operands`]' vessel, tube and hollow boxes, which
//!   stay beside the role readers the shell rows run over them.

use geom_core::{Decide, Point2, Point3, Tol};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, RawLoop};
use sweep::ExtrudeSide;
use sweep::test_support::{block, brick, extruded, prism, sketch_at};
use topo::{Body, EdgeKey};

/// The 4 x 4 x 1 slab, `z in [0, 1]` — the plainest operand a boolean
/// row puts something else against.
pub fn slab<T: Decide + topo::AtRestPolicy>() -> Body<T> {
    block(4.0, 4.0, 1.0, Tol::witness())
}

/// The 6 x 4 plate, `z in [z0, z0 + 1]`.
pub fn plate6<T: Decide + topo::AtRestPolicy>(z0: f64) -> Body<T> {
    brick((0.0, 6.0), (0.0, 4.0), (z0, z0 + 1.0), Tol::witness())
}

/// The pellet: `x in [0.9, 1.1]`, `y in [1.25, 1.35]`,
/// `z in [0.3, 0.7]`, volume `0.2 * 0.1 * 0.4 = 0.008`.
///
/// It sits strictly inside the concave notch of
/// `m5_s10_face_sense::mixed_turn_arcs`, and every point of it is
/// genuinely OUTSIDE that body — the notch floor at `x = 1` is
/// `y ~ 1.0858` — so the two solids are disjoint.
pub fn pellet<T: Decide + topo::AtRestPolicy>() -> Body<T> {
    brick((0.9, 1.1), (1.25, 1.35), (0.3, 0.7), Tol::witness())
}

/// **The three-arc cylinder**: [`super::three_arc`] of `radius` about
/// `centre`, its first joint at `first` degrees, extruded `height` from
/// a sketch plane lifted to `z0` ([`sweep::test_support::sketch_at`]).
/// One curved wall cut by three seam struts; six vertices, three on
/// each rim.
///
/// **The two placements are two knobs, not one**, because suites pose
/// it both ways: `centre` slides the PROFILE in the sketch plane (the
/// conic corpus's `n3r1_prune`, and every boss on a slab), while `z0`
/// lifts the SKETCH PLANE (`s16_box_soundness`'s raised tool). The two
/// are not interchangeable bit for bit — a plane slid in `x` builds a
/// different body from a profile slid in `x` — so a caller turns the
/// knob its rows were written with. A third pose is not this door's:
/// `s16_box_soundness::cylinder_apart` moves a finished cylinder by a
/// rigid translation, which is yet another body from the same point
/// set.
pub fn three_arc_cylinder(
    centre: Point2<f64>,
    radius: f64,
    z0: f64,
    height: f64,
    first: f64,
) -> Body<f64> {
    extruded(
        sketch_at(z0),
        vec![super::three_arc(centre, radius, first)],
        height,
        Tol::witness(),
    )
}

/// **The M5 boss**: a cylinder of radius 0.35 about `centre`, its
/// circle authored as `n` equal arcs indexed in RADIANS (joint `i` at
/// `2π·i/n`), extruded `len` from a sketch plane lifted to `z0`, at any
/// scalar the extrusion takes. Not [`three_arc_cylinder`] at `n = 3`:
/// that door places its joints through `to_radians` from degrees, and
/// the two spellings are different bits.
pub fn n_arc_boss<T: Decide + topo::AtRestPolicy>(
    centre: Point2<f64>,
    n: usize,
    z0: f64,
    len: f64,
) -> Body<T> {
    let theta = 2.0 * core::f64::consts::PI / n as f64;
    let bulge = T::from_f64((theta / 4.0).tan());
    let at = |i: usize| {
        let th = theta * i as f64;
        Point2::new(
            T::from_f64(centre.x + 0.35 * th.cos()),
            T::from_f64(centre.y + 0.35 * th.sin()),
        )
    };
    extruded(
        sketch_at(T::from_f64(z0)),
        vec![bulge_loop((0..n).map(|i| (at(i), bulge)).collect())],
        T::from_f64(len),
        Tol::witness(),
    )
}

/// [`n_arc_boss`] at `(1.2, 1.7)`: the boss the M5 curved-op suites
/// (`m5_s12_curved_ops`, its interval twin, and the PR 9 boss review)
/// cut from and union onto their 3 × 3 plate.
pub fn m5_boss<T: Decide + topo::AtRestPolicy>(n: usize, z0: f64, len: f64) -> Body<T> {
    n_arc_boss(Point2::new(1.2, 1.7), n, z0, len)
}

/// A three-arc cylinder standing on [`plate6`]'s midline: radius `r`
/// about `(cx, 2)`, first joint at 0°, `z in [z0, z0 + h]` — the peg
/// and bore the M9-3 suites union into and cut from the plate.
pub fn plate6_cyl(cx: f64, z0: f64, h: f64, r: f64) -> Body<f64> {
    three_arc_cylinder(Point2::new(cx, 2.0), r, z0, h, 0.0)
}

/// A small axis-aligned box of half-width `h` centred at `(cx, 0, .)`,
/// spanning `z in [z0, z0 + 0.4]`.
///
/// This fixture and its siblings below, through [`rounded_plate`],
/// are the conic-pruning corpus's operand vocabulary. They originated
/// in `s16_box_soundness` and were adopted from there.
pub fn small_box(cx: f64, h: f64, z0: f64) -> Body<f64> {
    brick((cx - h, cx + h), (-h, h), (z0, z0 + 0.4), Tol::witness())
}

/// [`small_box`] at `z in [0.3, 0.7]`. Against the corpus's cylinder
/// (`z in [0, 1]`) that is clear of both caps, so a pair there is
/// nested or separated in `x` alone, never touching.
pub fn nested_box(cx: f64, h: f64) -> Body<f64> {
    small_box(cx, h, 0.3)
}

/// A plate straddling the cylinder's bottom rim (`z = 0`) about the
/// rim's x-extremum at 180 degrees, which lies mid-arc between the
/// vertices at 120 and 240: `x in [-0.9, x_max]`, thin in `y`,
/// `z in [-0.1, 0.1]`. The rim reaches `x = -0.5`; whether the plate
/// meets it is decided by `x_max` alone.
pub fn rim_plate(x_max: f64) -> Body<f64> {
    brick((-0.9, x_max), (-0.15, 0.15), (-0.1, 0.1), Tol::witness())
}

/// The same about the top rim's y-extremum at 90 degrees, mid-arc
/// between the vertices at 0 and 120: the rim reaches `y = 0.5`.
pub fn top_rim_plate(y_min: f64) -> Body<f64> {
    brick((-0.15, 0.15), (y_min, 0.9), (0.9, 1.1), Tol::witness())
}

/// A rounded plate — bulge arcs on two sides, so its extruded walls
/// carry rims whose `u_ref` the sweep mints rotated.
pub fn rounded_plate() -> Body<f64> {
    let pts = [
        ((-1.0, -0.4), 0.0),
        ((1.0, -0.4), 0.35),
        ((1.3, 0.0), 0.0),
        ((1.0, 0.4), 0.0),
        ((-1.0, 0.4), 0.35),
        ((-1.3, 0.0), 0.0),
    ];
    prism(
        pts.iter()
            .map(|&((x, y), b)| (Point2::new(x, y), b))
            .collect(),
        0.8,
        Tol::witness(),
    )
}

/// A `w × w` square bar along the unit direction `d`, from `o + d·t0`
/// to `o + d·t1`: the square lies in the plane normal to `d` at the
/// start, in the frame `u = normalize(d × ŷ)`, `v = d × u` (`d` must
/// not be parallel to `ŷ`). The torus-door suites pierce their donut
/// with it.
pub fn framed_bar(o: Point3<f64>, d: geom_core::Vec3<f64>, t0: f64, t1: f64, w: f64) -> Body<f64> {
    use geom_core::{Affine3, Mat3, Vec3};
    let d = d.normalize();
    let u = d.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
    let v = d.cross(u);
    let h = w / 2.0;
    let lp = ProfileLoop::polygon([
        Point2::new(-h, -h),
        Point2::new(h, -h),
        Point2::new(h, h),
        Point2::new(-h, h),
    ]);
    let start = o + d * t0;
    let plane = profile::SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(u, v, d),
        start - Point3::origin(),
    ));
    let vp = profile::Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("the framed bar's profile validates");
    sweep::extrude(
        &vp,
        sweep::Extrusion::Distance {
            depth: t1 - t0,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .expect("the framed bar extrudes")
    .body
}

/// **A prism whose right side is a half-round**, unit high, and the
/// top front edge that ends there: the one plane–plane edge the blend
/// suites reach whose end face is CURVED, a run-out both verbs refuse
/// (`blend::battery::END_FACE_CURVED`).
pub fn half_round_end() -> (Body<f64>, EdgeKey) {
    let body = prism(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.5),
            (Point2::new(2.0, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        1.0,
        Tol::witness(),
    );
    let at = |v| {
        let p = body
            .get_point(body.get_vertex(v).expect("a vertex").point)
            .expect("a point");
        (p.x, p.y, p.z)
    };
    let edge = topo::query::all_edges(&body)
        .into_iter()
        .find(|&e| {
            let he = body.get_edge(e).expect("an edge").he_plus;
            let mut ends = [
                at(body.get_half_edge(he).expect("a half").start),
                at(body.half_edge_end(he).expect("an end")),
            ];
            ends.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
            ends == [(0.0, 0.0, 1.0), (2.0, 0.0, 1.0)]
        })
        .expect("the top front edge");
    (body, edge)
}

/// **A prism whose top corner turns unsymmetrically**: extruded `1.5`
/// along `−y` over the trapezoid `(0, 0), (2, 0), (2, 1), (s, 1)` in
/// `xz`, its left wall leaning in by `s` and every other face square;
/// and the two top edges that turn at `(s, 0, 1)` — along `x` over the
/// square end face `y = 0`, and along `y` over the leaning wall. The
/// turn is isosceles only at `s = 0`, so at a definite lean both verbs
/// refuse it (`blend::battery::TURN_NOT_ISOSCELES`).
pub fn leaning_turn(s: f64) -> (Body<f64>, [EdgeKey; 2]) {
    let plane = sweep::test_support::sketch_from_axes(
        Point3::new(0.0, 0.0, 0.0),
        geom_core::Vec3::new(1.0, 0.0, 0.0),
        geom_core::Vec3::new(0.0, 0.0, 1.0),
        Tol::witness(),
    );
    let body = sweep::test_support::prism_on(
        plane,
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(2.0, 1.0), 0.0),
            (Point2::new(s, 1.0), 0.0),
        ],
        1.5,
        Tol::witness(),
    );
    let point = |v| {
        *body
            .get_point(body.get_vertex(v).expect("a vertex").point)
            .expect("a point")
    };
    let between = |a: Point3<f64>, b: Point3<f64>| {
        topo::query::all_edges(&body)
            .into_iter()
            .find(|&e| {
                let he = body.get_edge(e).expect("an edge").he_plus;
                let (p, q) = (
                    point(body.get_half_edge(he).expect("a half").start),
                    point(body.half_edge_end(he).expect("an end")),
                );
                let at = |x: Point3<f64>, y: Point3<f64>| (x - y).norm() < 1e-12;
                (at(p, a) && at(q, b)) || (at(p, b) && at(q, a))
            })
            .unwrap_or_else(|| panic!("an edge between {a:?} and {b:?}"))
    };
    let corner = Point3::new(s, 0.0, 1.0);
    let edges = [
        between(corner, Point3::new(2.0, 0.0, 1.0)),
        between(corner, Point3::new(s, -1.5, 1.0)),
    ];
    (body, edges)
}

/// **A box sheared along its diagonal**: the parallelepiped
/// `{0 ≤ z ≤ 1, s z ≤ x ≤ 2 + s z, s z ≤ y ≤ 1.5 + s z}`, two
/// parallelogram prisms intersected, its lateral edges along
/// `(s, s, 1)`. Its top corners `(s, s, 1)` and `(2 + s, 1.5 + s, 1)`
/// are isosceles turns about their lateral edge; at `(2 + s, s, 1)` and
/// `(s, 1.5 + s, 1)` the two top edges make supplementary angles with
/// it.
pub fn parallelepiped(s: f64) -> Body<f64> {
    let z = geom_core::Vec3::new(0.0, 0.0, 1.0);
    let tol = Tol::witness();
    let along_y = sweep::test_support::prism_on(
        sweep::test_support::sketch_from_axes(
            Point3::new(0.0, 3.5, 0.0),
            geom_core::Vec3::new(1.0, 0.0, 0.0),
            z,
            tol,
        ),
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(2.0 + s, 1.0), 0.0),
            (Point2::new(s, 1.0), 0.0),
        ],
        5.0,
        tol,
    );
    let (z0, z1) = (-0.5, 1.5);
    let along_x = sweep::test_support::prism_on(
        sweep::test_support::sketch_from_axes(
            Point3::new(3.5, 0.0, 0.0),
            geom_core::Vec3::new(0.0, -1.0, 0.0),
            z,
            tol,
        ),
        vec![
            (Point2::new(-1.5 - s * z0, z0), 0.0),
            (Point2::new(-s * z0, z0), 0.0),
            (Point2::new(-s * z1, z1), 0.0),
            (Point2::new(-1.5 - s * z1, z1), 0.0),
        ],
        5.0,
        tol,
    );
    sweep::test_support::realized(topo::boolean::BooleanOp::Intersect, &along_y, &along_x, tol)
}

/// The axis-aligned block `x × y × z` ([`brick`]), at rest.
pub fn bar(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> topo::AtRestBody<f64> {
    sweep::test_support::finished("the bar", brick(x, y, z, Tol::witness()), Tol::witness())
}
