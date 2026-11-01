//! The counterexample search for [`super::spiric_length_bound`]: tori,
//! cuts and arcs nobody chose — near-pinch cuts, cuts through the
//! tube's hole, arcs through the pinch and across whole periods — each
//! bound held over a dense polygon of the arc, which runs under the
//! arc's own length. In a file of its own so the per-file gate can skip
//! the sweep without skipping `spiric`'s deterministic rows.

#![allow(clippy::unwrap_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/props/spiric.rs",
    "crates/geom-brep/src/props/quad/",
    "crates/geom/src/curves.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/interval/",
];

use super::spiric_length_bound;
use super::tests::{framed, walked};
use core::f64::consts::PI;

/// **Every bound holds its arc from above**, at 1, 16 and 256 pieces.
/// Four kinds of cut, each with spans its bound can read: a generic cut
/// over any span; a cut short of the inner equator over a span through
/// the pinch at `π`, across `0` too when wide; a cut through the tube's
/// hole over a span inside its own arc, `|t| < acos((|d| − R)/r)`, its
/// ends as near the arc's ends as `10⁻⁹` of it; and a near-tangent cut
/// over any span. A bound that refuses (`None`: a piece whose `f` is not
/// certified positive) asserts nothing and is counted.
#[test]
fn the_length_bound_holds_every_arc_from_above() {
    let mut rng = test_utils::fuzz::start("spiric_length_bound");
    let (mut held, mut refused) = (0_usize, 0_usize);
    for _ in 0..test_utils::fuzz::scaled(60) {
        let big = rng.range(0.3, 3.3);
        let r = big * rng.range(0.02, 0.99);
        let side = if rng.unit() < 0.5 { 1.0 } else { -1.0 };
        let turn = 2.0 * PI * (rng.below(3) as f64 - 1.0);
        let anywhere = |rng: &mut test_utils::fuzz::Rng| {
            let t0 = rng.range(-2.0 * PI, 2.0 * PI);
            (t0, t0 + rng.range(1e-3, 2.0 * PI))
        };
        let (d, span) = match rng.below(4) {
            0 => ((big - r) * rng.unit(), anywhere(&mut rng)),
            1 => {
                let reach = if rng.unit() < 0.5 { 1e-3 } else { 4.0 };
                (
                    big - r - rng.log_uniform(1e-12, 0.1),
                    (
                        turn + PI - rng.log_uniform(1e-7, reach),
                        turn + PI + rng.log_uniform(1e-7, reach),
                    ),
                )
            }
            2 => {
                let d = big - r + 2.0 * r * rng.range(0.01, 0.99);
                let arc = ((d - big) / r).acos();
                let end = |rng: &mut test_utils::fuzz::Rng| {
                    let at = arc * (1.0 - rng.log_uniform(1e-9, 1.0));
                    if rng.unit() < 0.5 { at } else { -at }
                };
                let (u, v) = (end(&mut rng), end(&mut rng));
                (d, (turn + u.min(v), turn + u.max(v)))
            }
            _ => (
                (big - r) * (1.0 - rng.log_uniform(1e-3, 1.0)),
                anywhere(&mut rng),
            ),
        };
        let d = side * d;
        let (e, _) = framed(
            (big, r, d),
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0; 3],
            [0.0; 3],
            span,
            true,
        );
        for pieces in [1, 16, 256] {
            let Some(bound) = spiric_length_bound(&e, pieces) else {
                refused += 1;
                continue;
            };
            let arc = walked((big, r, d), span, 4000);
            assert!(
                arc <= bound,
                "({big}, {r}) cut at {d}, {span:?}, {pieces} pieces: bound {bound} under \
                 the arc's polygon {arc}; {}",
                test_utils::fuzz::replay()
            );
            held += 1;
        }
    }
    println!("{held} bounds held, {refused} refused");
}
