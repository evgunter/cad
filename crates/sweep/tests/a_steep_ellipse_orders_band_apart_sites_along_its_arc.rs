//! **On a steep ellipse the join orders two band-apart sites by the arc
//! between them.** A unit cylinder whose axis leans to `cos θ = 1/k` off
//! `z` meets the plane `z = 0` in an ellipse of aspect `k` (semi-axes
//! `k` along `x`, `1` along `y`). A block `Q × [−2, 0]` has that plane
//! for its top face, and `Q` is the rectangle `[0, k + 2] × [−2, 2]`
//! with two notches cut up from its bottom edge to `y = −0.3`, leaving a
//! finger between them. Both the notch nearer the ellipse's major vertex
//! and the finger are `g` = [`GAP_BANDS`] bands wide, and they cross the
//! ellipse's lower arc where it is flattest against its radius (`x = k/√2`).
//!
//! The germ where the arc enters `Q` at `(0, 1)` faces two partners
//! `2g` apart along the arc: the wall the arc meets first, its true
//! neighbour, and the finger's far wall beyond it. Seen from the plane
//! through the ellipse's axis and either of them, the other lies only
//! `2g · sin ψ` off it, where `ψ` is the angle between the radius and
//! the tangent: `2k/(k² + 1)` there, 0.198 at `k = 10` and 0.033 at
//! `k = 60`. A travel margin that read that plane distance would leave
//! the `k = 10` pose in the escalation band and tie the `k = 60` pose
//! `Zero`, and a tie would fall back to the chord from the germ, which
//! on the lower arc shrinks toward the far wall and picks the wrong one.
//! The join's travel margin reads the arc (`boolean::join`'s
//! `turned_past`), so both poses read `2g` and pair along the walk. Each
//! then stops at a later band-scale reading that is filed
//! ([`stops_where_filed`]).
//!
//! The volume each op must have is the block's share of the cylinder,
//! `∫∫_Q max(0, h(y) − x cot θ) dx dy` with `h = √(1 − y²)/sin θ`: the
//! cylinder's span on a vertical line, clipped to `z ≤ 0` (it never
//! reaches the block's floor, and its end caps lie past `Q`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_1_SQRT_2, PI};

use crate::common::differential::{every_op_both_orders, outcome};
use crate::common::germ_pair;
use geom_core::{Tol, Vec3};
use sweep::test_support::finished;
use topo::{AtRestBody, BooleanError, CertifyError, EulerOpError, SplitJoinError};

/// The notch and finger widths, in bands of the run's ε.
const GAP_BANDS: f64 = 12.0;

/// How low the block reaches, and how high its notches are cut.
const FLOOR: f64 = -2.0;
const NOTCH_TOP: f64 = -0.3;

/// The cylinder's half-length: its caps lie past `Q` on either side.
fn half_length(k: f64) -> f64 {
    k + 10.0
}

/// `Q`'s notch walls, left to right: the wide notch `(x1, x2)`, the
/// finger `(x2, x3)`, the narrow notch `(x3, x4)`.
fn walls(k: f64, g: f64) -> [f64; 4] {
    let x3 = k * FRAC_1_SQRT_2;
    [x3 - g - 1.0, x3 - g, x3, x3 + g]
}

fn profile(k: f64, g: f64) -> Vec<(f64, f64)> {
    let [x1, x2, x3, x4] = walls(k, g);
    let x_end = k + 2.0;
    vec![
        (0.0, -2.0),
        (x1, -2.0),
        (x1, NOTCH_TOP),
        (x2, NOTCH_TOP),
        (x2, -2.0),
        (x3, -2.0),
        (x3, NOTCH_TOP),
        (x4, NOTCH_TOP),
        (x4, -2.0),
        (x_end, -2.0),
        (x_end, 2.0),
        (0.0, 2.0),
    ]
}

/// The unit cylinder about the origin, its axis leaned off `z` toward
/// `x` to `cos θ = 1/k`. Its seam lies in `y = 0`, where the ellipse
/// meets it at the major vertices, off `Q`'s notches.
fn cylinder(k: f64) -> AtRestBody<f64> {
    let c = germ_pair::cyl(1.0, half_length(k));
    let leaned = germ_pair::spin(&c, Vec3::unit_y(), (1.0 / k).acos());
    finished("the cylinder", leaned, Tol::witness())
}

fn block(k: f64, g: f64) -> AtRestBody<f64> {
    let b = topo::test_support::prism_z(&profile(k, g), FLOOR, 0.0, Tol::witness()).body;
    finished("the block", b, Tol::witness())
}

/// `∫_p^q max(0, h − κ x) dx`.
fn under(p: f64, q: f64, h: f64, kappa: f64) -> f64 {
    let q = q.min(h / kappa);
    if q <= p {
        return 0.0;
    }
    let f = |x: f64| h * x - 0.5 * kappa * x * x;
    f(q) - f(p)
}

/// The block's share of the cylinder: over `y = sin φ`, Simpson's rule
/// on each piece between the kinks (the notch tops, and where the
/// ellipse `x = k cos φ` crosses a notch wall), where the integrand is
/// smooth.
fn shared(k: f64, g: f64) -> f64 {
    let (s, c) = ((1.0 - 1.0 / (k * k)).sqrt(), 1.0 / k);
    let kappa = c / s;
    let w = walls(k, g);
    let x_end = k + 2.0;
    let row = |phi: f64| {
        let (y, h) = (phi.sin(), phi.cos() / s);
        let spans: Vec<(f64, f64)> = if y < NOTCH_TOP {
            vec![(0.0, w[0]), (w[1], w[2]), (w[3], x_end)]
        } else {
            vec![(0.0, x_end)]
        };
        phi.cos()
            * spans
                .iter()
                .map(|&(p, q)| under(p, q, h, kappa))
                .sum::<f64>()
    };
    let mut kinks = vec![-PI / 2.0, NOTCH_TOP.asin(), PI / 2.0];
    kinks.extend(w.iter().map(|x| -(x / k).acos()));
    kinks.sort_by(f64::total_cmp);
    kinks
        .windows(2)
        .map(|ab| {
            let (a, b) = (ab[0], ab[1]);
            let n = 2000;
            let h = (b - a) / f64::from(n);
            let inner: f64 = (1..n)
                .map(|i| {
                    let wgt = if i % 2 == 1 { 4.0 } else { 2.0 };
                    wgt * row(a + f64::from(i) * h)
                })
                .sum();
            h / 3.0 * (row(a) + row(b) + inner)
        })
        .sum()
}

/// Where a run that does not build may stop, by `k`: the door each pose
/// reaches after the travel order, each a filed row. With the pair order
/// or the span meter fixed the pose moves on, and must then build.
///
/// - `k = 10`: the notch's two walls are parallel and `12ε` apart, and
///   their section arcs' chords differ by `1.65ε`, so the order between
///   the two pairs ties in band (`bool_join_nearest`;
///   `work/join/a-bar-through-a-ball-refuses-at-a-door-that-moves-with-scale.md`).
/// - `k = 60`: the finger's section edge is metered at the ellipse's
///   minor semi-axis, `12ε·√2/60` (`work/issues/an-ellipse-span-is-metered-at-its-minor-axis-and-refuses-a-flank-edge-k-times-longer-than-the-band.md`).
fn stops_where_filed(k: f64, e: &BooleanError) -> bool {
    match e {
        BooleanError::Escalated { diag, .. } if k == 10.0 => {
            diag.predicate == Some("bool_join_nearest")
        }
        BooleanError::Join(SplitJoinError::Euler(EulerOpError::Certification {
            error: CertifyError::IntervalNotForward { .. },
        })) => k == 60.0,
        _ => false,
    }
}

/// **Two partners `2g` apart along a steep ellipse's flank are ordered
/// along it**: no run refuses at `bool_join_arc_travel`, in any op or
/// operand order, at `k = 10`, where the plane distance read `4.78`
/// bands, or at `k = 60`, where it read `0.8` and tied. Each run builds
/// `SOUND` at the closed form or stops where [`stops_where_filed`] says.
#[test]
fn band_apart_partners_on_a_steep_ellipse_are_ordered_along_its_arc() {
    let tol = Tol::witness();
    let g = GAP_BANDS * tol.eps();
    for k in [10.0, 60.0] {
        let (a, b) = (cylinder(k), block(k, g));
        let va = PI * 2.0 * half_length(k);
        let vb = (4.0 * (k + 2.0) - (g + 1.0) * (NOTCH_TOP - FLOOR)) * -FLOOR;
        for (op, r, want) in every_op_both_orders(&a, &b, (va, vb, shared(k, g)), tol) {
            if let Err(e) = &r {
                assert!(
                    stops_where_filed(k, e),
                    "k = {k}, {op}: refused where no filed row stops it: {e:?}"
                );
                continue;
            }
            let line = outcome(r, want, tol);
            assert!(line.starts_with("OK SOUND"), "k = {k}, {op}: {line}");
        }
    }
}
