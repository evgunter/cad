//! **The volume a ball shares with a solid of revolution**, as a 1-D
//! integral of closed forms: sliced across the solid's axis, the ball's
//! slice is a disc and the solid's an annulus (a disc where its inner
//! radius is zero), and their overlap is the overlap of the disc with
//! the outer circle less its overlap with the inner one, each a
//! two-disc lens in closed form ([`disc_overlap`]).
//!
//! The integral is the midpoint rule over the ball's heights, read as
//! `z = h + r·sin θ` so the slice's square-root ends are smooth in `θ`.
//! The overlap is only continuous where a slice meets a circle
//! tangentially, so the rule is read at `n` and at `2n` panels and their
//! difference stands for its error ([`ball_in_revolved`]); a reader
//! bounds the volume it checks by a stated multiple of that.

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
        return PI * m * m;
    }
    let ca = ((d * d + a * a - b * b) / (2.0 * d * a)).clamp(-1.0, 1.0);
    let cb = ((d * d + b * b - a * a) / (2.0 * d * b)).clamp(-1.0, 1.0);
    let k = (-d + a + b) * (d + a - b) * (d - a + b) * (d + a + b);
    a * a * ca.acos() + b * b * cb.acos() - 0.5 * k.max(0.0).sqrt()
}

/// **The volume of the ball of radius `r` inside a solid of revolution**:
/// the ball's centre at distance `e` from the solid's axis and height `h`
/// along it, the solid's slice at height `z` the annulus `annulus(z)`
/// (inner and outer radii; `None` past the solid). The midpoint rule at
/// `2n` panels, and the difference from the rule at `n`, which stands
/// for its error (module docs).
#[must_use]
pub fn ball_in_revolved(
    r: f64,
    (e, h): (f64, f64),
    annulus: impl Fn(f64) -> Option<(f64, f64)>,
    n: u32,
) -> (f64, f64) {
    let rule = |n: u32| {
        let step = PI / f64::from(n);
        (0..n)
            .map(|i| {
                let theta = -FRAC_PI_2 + (f64::from(i) + 0.5) * step;
                let (s, c) = theta.sin_cos();
                let rho = r * c;
                let slice = annulus(h + r * s).map_or(0.0, |(inner, outer)| {
                    disc_overlap(rho, outer, e) - disc_overlap(rho, inner, e)
                });
                slice * r * c * step
            })
            .sum::<f64>()
    };
    let (coarse, fine) = (rule(n), rule(2 * n));
    (fine, (fine - coarse).abs())
}

/// The slices of the torus of major radius `big` and tube radius `small`
/// about its axis, centred at height zero: the annulus at height `z`.
pub fn torus_slice(big: f64, small: f64) -> impl Fn(f64) -> Option<(f64, f64)> {
    move |z: f64| {
        (z.abs() < small).then(|| {
            let s = (small * small - z * z).sqrt();
            (big - s, big + s)
        })
    }
}
