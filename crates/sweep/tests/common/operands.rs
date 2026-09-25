//! **The plain named operands** several suites build a boolean from:
//! the axis-aligned boxes below and the one rounded plate the conic
//! corpus cuts against. Body authoring, so it routes here beside
//! [`super::cavity`] rather than into a suite.
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
//! this module drew from are its neighbours, and one box and one
//! cylinder family in them stayed:
//!
//! - `s16_box_soundness::top_rim_x_plate`, the only box left in those
//!   suites that just one of them builds. A helper one suite uses
//!   stays in that suite ([`super`]'s routing rule), and the rule does
//!   not bend for a sibling of something that did come here;
//! - `n3r1_prune::cylinder_at` and `s16_box_soundness::cylinder`.
//!   They are NOT one fixture: the first translates its profile in
//!   `x` on the `xy` plane, the second centres the profile and lifts
//!   the sketch plane to `z0`, and the two take different parameters.
//!   Reconciling them decides whether a rim's sketch pose is part of
//!   what those rows check, which is a verdict question and not this
//!   module's;
//! - the `Point3`-cornered box in [`super::cavity`], which is that
//!   module's own corner vocabulary over the same door;
//! - `super::approx::unit_box`, which is the boolean gate's FACE rule
//!   fixture and belongs with the surgery vocabulary that reads it.

use geom_core::{Decide, Point2, Tol};
use sweep::test_support::{block, brick, prism};
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
