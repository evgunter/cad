//! The cone arm's enclosures, swept: over a box about a point near
//! either nappe, from beside the apex out to ten metres along the
//! slant, each certified enclosure holds the value at every sampled
//! point of its box, and a box holding the apex is refused. In a file of
//! its own so the per-file gate can skip the sweep without skipping
//! `enclose`'s deterministic rows.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/ssi/enclose.rs",
    "crates/geom-brep/src/implicit.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/interval/",
];

use geom_core::{Point3, Vec3};
use test_utils::fuzz;

use super::tests::{cone, cone_box_reads};

fn unit(rng: &mut fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let n = v.norm();
        if n > 0.1 && n <= 1.0 {
            return v / n;
        }
    }
}

/// **Every certified cone enclosure holds its box's sampled values.**
/// A counterexample search: a random cone, a point beside one nappe at
/// a slant distance log-uniform from `10⁻⁴` to `10` (so boxes reach the
/// apex and the axis), a box about it of a log-uniform size relative to
/// that distance, and sixteen points of the box (its corners and eight
/// interior points), each read by [`cone_box_reads`]. How many boxes
/// certify is no part of this search: the static witness
/// `the_cone_enclosures_certify_and_hold_on_fixed_boxes` is the floor.
#[test]
fn the_cone_enclosures_hold_every_sampled_point_of_their_box() {
    let mut rng = fuzz::start("enclose::cone_arm");
    for _ in 0..fuzz::scaled(400) {
        let axis = unit(&mut rng);
        let apex = Point3::new(
            rng.range(-5.0, 5.0),
            rng.range(-5.0, 5.0),
            rng.range(-5.0, 5.0),
        );
        let alpha = rng.range(0.05, 1.5);
        let c = cone([apex.x, apex.y, apex.z], [axis.x, axis.y, axis.z], alpha);
        let sigma = if rng.below(2) == 0 { 1.0 } else { -1.0 };
        let slant = rng.log_uniform(1e-4, 10.0);
        let out = {
            let v = unit(&mut rng);
            (v - axis * v.dot(axis)).normalize()
        };
        let on = apex + axis * (sigma * slant * alpha.cos()) + out * (slant * alpha.sin());
        let r = slant * rng.log_uniform(1e-4, 2.0);
        let centre = on + unit(&mut rng) * rng.range(0.0, r);
        let interior: Vec<Vec3<f64>> = (0..8)
            .map(|_| Vec3::new(rng.range(-r, r), rng.range(-r, r), rng.range(-r, r)))
            .collect();
        let case = format!(
            "cone apex {apex:?} axis {axis:?} α {alpha}, box about {centre:?} of half-width \
             {r} (nappe {sigma}, slant {slant}) — {}",
            fuzz::replay()
        );
        cone_box_reads(&c, apex, (centre, r), &interior, &case);
    }
}
