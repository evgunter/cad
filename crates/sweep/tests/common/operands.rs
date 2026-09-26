//! **The plain named operands** several suites build a boolean from:
//! the axis-aligned boxes below, and the three-arc cylinder and the
//! rounded plate the conic corpus cuts with. Body authoring, so it
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
//!   fixture and belongs with the surgery vocabulary that reads it.

use geom_core::{Decide, Point2, Tol};
use profile::test_support::bulge_loop;
use sweep::test_support::{block, brick, extruded, prism, sketch_at};
use topo::Body;

/// The 4 x 4 x 1 slab, `z in [0, 1]` — the plainest operand a boolean
/// row puts something else against.
pub fn slab<T: Decide>() -> Body<T> {
    block(4.0, 4.0, 1.0, Tol::witness())
}

/// The 6 x 4 plate, `z in [z0, z0 + 1]`.
pub fn plate6<T: Decide>(z0: f64) -> Body<T> {
    brick((0.0, 6.0), (0.0, 4.0), (z0, z0 + 1.0), Tol::witness())
}

/// The pellet: `x in [0.9, 1.1]`, `y in [1.25, 1.35]`,
/// `z in [0.3, 0.7]`, volume `0.2 * 0.1 * 0.4 = 0.008`.
///
/// It sits strictly inside the concave notch of
/// `m5_s10_face_sense::mixed_turn_arcs`, and every point of it is
/// genuinely OUTSIDE that body — the notch floor at `x = 1` is
/// `y ~ 1.0858` — so the two solids are disjoint.
pub fn pellet<T: Decide>() -> Body<T> {
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
pub fn n_arc_boss<T: Decide>(centre: Point2<f64>, n: usize, z0: f64, len: f64) -> Body<T> {
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
pub fn m5_boss<T: Decide>(n: usize, z0: f64, len: f64) -> Body<T> {
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
