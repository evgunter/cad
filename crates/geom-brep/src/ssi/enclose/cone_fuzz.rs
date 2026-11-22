//! The cone arm's enclosures, swept: over a box about a point near
//! either nappe, from beside the apex out to ten metres along the
//! slant, each certified enclosure holds the value at every sampled
//! point of its box, and a box holding the apex is refused. In a file of
//! its own so the per-file gate can skip the sweep without skipping
//! `enclose`'s deterministic rows.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/implicit.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/interval/",
];

use geom_core::{Bounds, Point3, Vec3};
use test_utils::fuzz;

use super::tests::cone;
use super::{Box3, implicit_enclosure, implicit_gradient_enclosure, implicit_sign_enclosure};
use crate::implicit::{implicit_gradient, implicit_residual};

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
/// interior points). Where `g`'s enclosure is certified it holds the
/// `f64` residual at each, within that residual's own rounding; so does
/// each component of `∇g`'s; where the sign enclosure excludes zero
/// each point lies on its side of the cone; and a box holding the apex
/// encloses neither `g` nor `∇g`.
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
        let b = Box3::around(centre, r);
        let (g, grad, sign) = (
            implicit_enclosure(&c, b),
            implicit_gradient_enclosure(&c, b),
            implicit_sign_enclosure(&c, b),
        );
        let case = || {
            format!(
                "cone apex {apex:?} axis {axis:?} α {alpha}, box about {centre:?} of half-width \
                 {r} (nappe {sigma}, slant {slant}) — {}",
                fuzz::replay()
            )
        };
        let reach = (centre - apex).norm() + r + 1.0;
        let tol = 64.0 * f64::EPSILON * reach;
        let corner = |k: usize, s: usize| if k >> s & 1 == 0 { -r } else { r };
        let points: Vec<Point3<f64>> =
            (0..8)
                .map(|k| centre + Vec3::new(corner(k, 0), corner(k, 1), corner(k, 2)))
                .chain((0..8).map(|_| {
                    centre + Vec3::new(rng.range(-r, r), rng.range(-r, r), rng.range(-r, r))
                }))
                .collect();
        for p in points {
            let at = implicit_residual(&c, p);
            if g.is_certified() {
                assert!(
                    g.lo() - tol <= at && at <= g.hi() + tol,
                    "g {g:?} misses {at} at {p:?}: {}",
                    case()
                );
            }
            if grad.iter().all(|e| e.is_certified()) {
                let n = implicit_gradient(&c, p);
                for (e, v) in grad.iter().zip([n.x, n.y, n.z]) {
                    assert!(
                        e.lo() - tol <= v && v <= e.hi() + tol,
                        "∇g {grad:?} misses {n:?} at {p:?}: {}",
                        case()
                    );
                }
            }
            if sign.is_certified() && sign.lo() > 0.0 {
                assert!(at > -tol, "sign {sign:?} but g = {at} at {p:?}: {}", case());
            }
            if sign.is_certified() && sign.hi() < 0.0 {
                assert!(at < tol, "sign {sign:?} but g = {at} at {p:?}: {}", case());
            }
        }
        if Box3::between(apex, apex).contained_in(b) {
            assert!(
                !g.is_certified() && grad.iter().all(|e| !e.is_certified()),
                "a box holding the apex is enclosed: {}",
                case()
            );
        }
    }
}
