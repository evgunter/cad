//! **Two partners a dozen bands apart on a steep ellipse's flank, through
//! the public booleans.** A unit cylinder whose axis leans to `cos θ = 1/k` off
//! `z` meets the plane `z = 0` in an ellipse of aspect `k` (semi-axes
//! `k` along `x`, `1` along `y`). A block `Q × [−2, 0]` has that plane
//! for its top face, and `Q` is the rectangle `[0, k + 2] × [−2, 2]`
//! with two notches cut up from its bottom edge to `y = −0.3`, leaving a
//! finger between them. Both the notch nearer the ellipse's major vertex
//! and the finger are `g` = [`gap_bands`] bands wide, and they cross the
//! ellipse's lower arc where it is flattest against its radius (`x = k/√2`).
//!
//! The germ where the arc enters `Q` at `(0, 1)` faces two partners
//! `2g` apart along the arc: the wall the arc meets first, its true
//! neighbour, and the finger's far wall beyond it. Seen from the plane
//! through the ellipse's axis and either of them, the other lies only
//! `2g · sin ψ` off it, where `ψ` is the angle between the radius and
//! the tangent: `2k/(k² + 1)` there, 0.198 at `k = 10` and 0.033 at
//! `k = 60`. A travel margin that read that plane distance would leave
//! the `k = 10` pose (`g` 24 bands) in the escalation band, at 9.5, and
//! tie the `k = 60` pose (`g` 12 bands) `Zero`, at 0.8, and a tie would
//! fall back to the chord from the germ, which on the lower arc shrinks
//! toward the far wall and picks the wrong one.
//! The section table carries the plane × cylinder ellipse, so the join
//! orders the partners by their feet's travel along it
//! (`bool_join_foot_travel`), metered at the carrier's speed floor over
//! the window between the two feet: `2g` of arc, 48 bands at `k = 10`
//! and 24 at `k = 60`, clear of the band at both. A foot is orthogonal,
//! so a site moved across the conic keeps its foot and the order reads
//! no along-section slack. Both poses take the true neighbour and run on
//! to the finish, where the merge's pcurve span check meters the
//! finger's arc at the ellipse's global speed floor, its minor
//! semi-axis ([`stops_where_expected`]).
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
use topo::{AtRestBody, BooleanError};

/// The notch and finger widths, in bands of the run's ε: at `k = 10`
/// wide enough that the finger's two sites clear the slack, at `k = 60`
/// narrow enough that the plane distance would tie.
fn gap_bands(k: f64) -> f64 {
    if k == 10.0 { 24.0 } else { 12.0 }
}

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
    let b = topo::test_support::prism_z::<f64>(&profile(k, g), FLOOR, 0.0, Tol::witness()).body;
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

/// **Where a run that does not build may stop, by `k`: the coplanar
/// merge's pcurve span check** (`pcurve_interval_forward`), which meters
/// the finger's arc span at the ellipse's global speed floor, its minor
/// semi-axis 1, where the arc's own speed there is about `k/√2`
/// (`work/hone/an-ellipse-arcs-span-is-levered-by-its-global-speed-floor.md`).
/// At `k = 10` the `24ε` arc spans `3.4ε` of parameter and reads in band,
/// an escalation; at `k = 60` the `12ε` arc spans `0.28ε` and reads Zero,
/// the span not forward. A probe run on PR 7354 measured both at
/// `k = 60`: the foot order took the true neighbour on `24ε` of arc, and
/// check 2 read a span of `2.83e-10` at a rate of `1`. Either is the
/// check's own reading, at a door
/// that moves with the speed floor, not a guess
/// (`work/onecopy/a-steep-ellipses-band-apart-pose-stops-at-the-merges-pcurve-span.md`).
fn stops_where_expected(k: f64, e: &BooleanError) -> bool {
    match e {
        BooleanError::Merge(merge) if k == 10.0 => matches!(
            merge.error(),
            topo::MergeCoplanarError::Pcurve {
                source: topo::PcurveMintError::Certify {
                    error: geom_brep::PcurveCertifyError::Escalated { cause, .. },
                    ..
                },
            } if cause.predicate == Some("pcurve_interval_forward")
        ),
        BooleanError::Merge(merge) if k == 60.0 => matches!(
            merge.error(),
            topo::MergeCoplanarError::Pcurve {
                source: topo::PcurveMintError::Certify {
                    error: geom_brep::PcurveCertifyError::IntervalNotForward,
                    ..
                },
            }
        ),
        _ => false,
    }
}

/// **Every op on two partners `2g` apart along a steep ellipse's flank
/// builds `SOUND` at the closed form or refuses where
/// [`stops_where_expected`] says**, in both operand orders: past the foot
/// order at both `k`, at the merge's pcurve span check.
#[test]
fn band_apart_partners_on_a_steep_ellipse_build_or_stop_where_expected() {
    let tol = Tol::witness();
    for k in [10.0, 60.0] {
        let g = gap_bands(k) * tol.eps();
        let (a, b) = (cylinder(k), block(k, g));
        let va = PI * 2.0 * half_length(k);
        let vb = (4.0 * (k + 2.0) - (g + 1.0) * (NOTCH_TOP - FLOOR)) * -FLOOR;
        for (op, r, want) in every_op_both_orders(&a, &b, (va, vb, shared(k, g)), tol) {
            if let Err(e) = &r {
                assert!(
                    stops_where_expected(k, e),
                    "k = {k}, {op}: refused where no row expects it: {e:?}"
                );
                continue;
            }
            let line = outcome(r, want, tol);
            assert!(line.starts_with("OK SOUND"), "k = {k}, {op}: {line}");
        }
    }
}
