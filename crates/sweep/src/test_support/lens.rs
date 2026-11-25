//! **The volume a ball shares with a solid of revolution**, as a 1-D
//! integral of closed forms: sliced across the solid's axis, the ball's
//! slice is a disc and the solid's an annulus (a disc where its inner
//! radius is zero), and their overlap is the overlap of the disc with
//! the outer circle less its overlap with the inner one, each a
//! two-disc lens in closed form ([`disc_overlap`](crate::test_support::lens::disc_overlap)).
//!
//! The integral is the midpoint rule over the ball's heights, read as
//! `z = h + r·sin θ` so the ball's own square-root ends are smooth in
//! `θ`. The solid's slice has square-root ends of its own (a torus's
//! annulus closes at the tube's top and bottom heights), where the
//! integrand's slope is unbounded, and a panel straddling one makes the
//! rule converge irregularly: there two panel counts can agree better than
//! either agrees with the integral. So the rule takes panel edges at the
//! solid's slice ends (`ends`), and within each stretch the error falls
//! regularly as the panels halve. The rule is read at `n` and at `2n`
//! panels, and their difference stands for the error of the finer one
//! ([`ball_in_revolved`](crate::test_support::lens::ball_in_revolved)). It is an estimate, not a bound: on the four
//! poses the rows read, the finer rule's error against one 128 times
//! finer measured at most 0.34 of the difference at `n = 2000`, so a
//! reader's tenfold margin holds it with room.

use core::f64::consts::{FRAC_PI_2, PI};

/// The area of the intersection of two discs of radii `a` and `b` whose
/// centres lie `d` apart.
#[must_use]
pub fn disc_overlap(a: f64, b: f64, d: f64) -> f64 {
    if a <= 0.0 || b <= 0.0 || d >= a + b {
        return 0.0;
    }
    if d <= (a - b).abs() {
        let m = a.min(b);
        return PI * m.powi(2);
    }
    let ca = ((d.powi(2) + a.powi(2) - b.powi(2)) / (2.0 * d * a)).clamp(-1.0, 1.0);
    let cb = ((d.powi(2) + b.powi(2) - a.powi(2)) / (2.0 * d * b)).clamp(-1.0, 1.0);
    let k = (-d + a + b) * (d + a - b) * (d - a + b) * (d + a + b);
    a.powi(2) * ca.acos() + b.powi(2) * cb.acos() - 0.5 * k.max(0.0).sqrt()
}

/// **The volume of the ball of radius `r` inside a solid of revolution**:
/// the ball's centre at distance `e` from the solid's axis and height `h`
/// along it, the solid's slice at height `z` the annulus `annulus(z)`
/// (inner and outer radii; `None` past the solid), whose square-root ends
/// lie at the heights `ends`. The midpoint rule at `2n` panels, with panel
/// edges at those of `ends` inside the ball, and the difference from the
/// rule at `n`, which estimates its error (module docs).
#[must_use]
pub fn ball_in_revolved(
    r: f64,
    (e, h): (f64, f64),
    (annulus, ends): (impl Fn(f64) -> Option<(f64, f64)>, &[f64]),
    n: u32,
) -> (f64, f64) {
    let mut edges = vec![-FRAC_PI_2, FRAC_PI_2];
    edges.extend(
        ends.iter()
            .filter(|&&z| (z - h).abs() < r)
            .map(|&z| ((z - h) / r).asin()),
    );
    edges.sort_by(f64::total_cmp);
    let slice = |theta: f64| {
        let (s, c) = theta.sin_cos();
        let rho = r * c;
        annulus(h + r * s).map_or(0.0, |(inner, outer)| {
            disc_overlap(rho, outer, e) - disc_overlap(rho, inner, e)
        }) * r
            * c
    };
    let rule = |n: u32| {
        edges
            .windows(2)
            .map(|w| {
                let (a, b) = (w[0], w[1]);
                // The stretch's share of the panels, at least one.
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let m = ((f64::from(n) * (b - a) / PI).round() as u32).max(1);
                let step = (b - a) / f64::from(m);
                (0..m)
                    .map(|i| slice(a + (f64::from(i) + 0.5) * step))
                    .sum::<f64>()
                    * step
            })
            .sum::<f64>()
    };
    let (coarse, fine) = (rule(n), rule(2 * n));
    (fine, (fine - coarse).abs())
}

/// The slices of the torus of major radius `big` and tube radius `small`
/// about its axis, centred at height zero: the annulus at height `z`, and
/// the heights it closes at, the tube's top and bottom.
pub fn torus_slice(big: f64, small: f64) -> (impl Fn(f64) -> Option<(f64, f64)>, [f64; 2]) {
    let slice = move |z: f64| {
        (z.abs() < small).then(|| {
            let s = (small.powi(2) - z.powi(2)).sqrt();
            (big - s, big + s)
        })
    };
    (slice, [-small, small])
}
