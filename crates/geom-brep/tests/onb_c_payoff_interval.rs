//! **What the point-zero sign rule buys at `Interval`** — the payoff
//! column, measured on M10-5's 12-gon prism.
//!
//! `Vec3::orthonormal_basis` takes `s = 1.copysign(n.z)`; at `Interval`
//! a zero-containing `n.z` sends `copysign` down its two-sided-hull arm,
//! so every vertical wall stores `u_ref.z ∈ [-|n.x|, |n.x|]` and a
//! subdivision over the chart cannot converge. Rule (c) narrows that arm
//! at a POINT enclosure of zero by transferring the zero's sign BIT,
//! leaving `f64` untouched.
//!
//! This replays the 12-gon prism at `Interval` — the same dumbbell
//! polygon `m10_5_r1_probes_interval.rs`'s sound-prism row extrudes,
//! written down here as a literal — mints each wall's plane through
//! `newell_plane`, and prints, per wall: the stored `u_ref` widths
//! today, the widths under rule (c), whether the (c) frame still
//! CONTAINS the `f64` frame, and whether `Surface::eval` over a halved
//! `(u, v)` window then refines.
//!
//! The `refines` predicate is `editor_core::clearance`'s, copied rather
//! than called (it is private there): both axes must move a bound.
//!
//! `#[ignore]`d: asserts nothing, gates nothing, prints.
//!
//! ```text
//! cargo test -p geom-brep --features interval --test all \
//!     -- --ignored --nocapture onb_c_payoff_interval
//! ```

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::newell_plane;
use geom_core::{Band, Bounds, Interval, Point3, Real, Tol, Vec3};

/// THE CORPUS, written down: M10-5's dumbbell — a 12-gon whose prism
/// walls run every wall orientation the rule can meet (`n.x = 0`,
/// `n.y = 0`, and eight axis-aligned faces between them).
const DUMBBELL: [(f64, f64); 12] = [
    (0.0, 0.0),
    (2.0, 0.0),
    (2.0, 0.8),
    (3.0, 0.8),
    (3.0, 0.0),
    (5.0, 0.0),
    (5.0, 2.0),
    (3.0, 2.0),
    (3.0, 1.2),
    (2.0, 1.2),
    (2.0, 2.0),
    (0.0, 2.0),
];

/// The extrusion height, and the parameter window the refinement test
/// halves — a metre-scale cell on a metre-scale wall.
const HEIGHT: f64 = 2.0;
const WINDOW: (f64, f64) = (0.0, 1.0);
const EPS_WINDOW: (f64, f64) = (0.0, 1e-9);

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn width(e: Interval) -> f64 {
    e.hi() - e.lo()
}

/// Wall `i`'s ring: the prism's side face, wound outward.
fn wall<T: Real>(i: usize) -> [Point3<T>; 4] {
    let (a, b) = (DUMBBELL[i], DUMBBELL[(i + 1) % DUMBBELL.len()]);
    let p = |x: f64, y: f64, z: f64| {
        Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z))
    };
    [
        p(a.0, a.1, 0.0),
        p(b.0, b.1, 0.0),
        p(b.0, b.1, HEIGHT),
        p(a.0, a.1, HEIGHT),
    ]
}

/// Rule **(c)**: the sign-definite arms as today; at a POINT enclosure
/// of zero, the zero's own sign bit — and only when both endpoints
/// carry the SAME bit, since a `[-0.0, +0.0]` enclosure names no sign.
/// Everything else keeps the two-sided hull.
fn s_under_rule_c(nz: Interval) -> Option<Interval> {
    if nz.lo() > 0.0 {
        return Some(Interval::one());
    }
    if nz.hi() < 0.0 {
        return Some(-Interval::one());
    }
    if nz.lo() == 0.0 && nz.hi() == 0.0 && nz.lo().is_sign_negative() == nz.hi().is_sign_negative()
    {
        return Some(if nz.lo().is_sign_negative() {
            -Interval::one()
        } else {
            Interval::one()
        });
    }
    None
}

/// `orthonormal_basis`'s `b1`, with `s` supplied rather than taken from
/// `copysign` — the counterfactual frame, spelled exactly as `vec.rs`
/// spells the real one.
fn b1_with(n: Vec3<Interval>, s: Interval) -> Vec3<Interval> {
    let r = Interval::one() / (Interval::one() + n.z.abs());
    Vec3::new(
        Interval::one() - n.x.powi(2) * r,
        -((n.x * n.y) * r),
        -(s * n.x),
    )
}

/// `editor_core::clearance::refines`, copied: BOTH axes must move a
/// bound of the cell's enclosure when the window is halved.
fn refines(surface: &Surface<Interval>, u: (f64, f64), v: (f64, f64)) -> bool {
    let cell = |u: (f64, f64), v: (f64, f64)| {
        surface.eval(
            Interval::from_bounds(u.0, u.1),
            Interval::from_bounds(v.0, v.1),
        )
    };
    let whole = cell(u, v);
    let mid = |(lo, hi): (f64, f64)| 0.5 * (lo + hi);
    let narrower = |b: &Point3<Interval>| {
        [(b.x, whole.x), (b.y, whole.y), (b.z, whole.z)]
            .iter()
            .any(|(h, w)| h.hi() < w.hi() || h.lo() > w.lo())
    };
    narrower(&cell((u.0, mid(u)), v)) && narrower(&cell(u, (v.0, mid(v))))
}

fn contains(e: Interval, x: f64) -> bool {
    e.lo() <= x && x <= e.hi()
}

/// The `z`-extent of the cell `Surface::eval` encloses over a window.
fn cell_z_width(surface: &Surface<Interval>, u: (f64, f64), v: (f64, f64)) -> f64 {
    let p = surface.eval(
        Interval::from_bounds(u.0, u.1),
        Interval::from_bounds(v.0, v.1),
    );
    width(p.z)
}

/// **Table 4** — the payoff, wall by wall, over one parameter window.
fn payoff_over(window: (f64, f64)) {
    println!();
    println!("Window u = v = [{:e}, {:e}]", window.0, window.1);
    println!(
        "| wall | f64 n | Interval n.z | u_ref.z today | width today | u_ref.z under (c) | width under (c) | cell z-width today | cell z-width under (c) | contains f64 u_ref | refines today | refines (c) |"
    );
    println!("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |");
    let (mut narrowed, mut refined, mut contained, mut tighter) = (0usize, 0usize, 0usize, 0usize);
    for i in 0..DUMBBELL.len() {
        let Ok(Surface::Plane {
            origin,
            normal,
            u_ref,
        }) = newell_plane::<Interval>(&wall::<Interval>(i), band())
        else {
            println!("| {i} | (newell_plane refused) | | | | | | | | | | |");
            continue;
        };
        let Ok(Surface::Plane {
            normal: nf,
            u_ref: uf,
            ..
        }) = newell_plane::<f64>(&wall::<f64>(i), band())
        else {
            println!("| {i} | (f64 newell_plane refused) | | | | | | | | | | |");
            continue;
        };
        let today = Surface::Plane {
            origin,
            normal,
            u_ref,
        };
        let z_today = cell_z_width(&today, window, window);
        let refines_today = refines(&today, window, window);
        let (u_c, w_c, z_c, holds, refines_c) = match s_under_rule_c(normal.z) {
            Some(s) => {
                let b = b1_with(normal, s);
                let surf = Surface::Plane {
                    origin,
                    normal,
                    u_ref: b,
                };
                (
                    format!("[{:?}, {:?}]", b.z.lo(), b.z.hi()),
                    width(b.z),
                    cell_z_width(&surf, window, window),
                    contains(b.x, uf.x) && contains(b.y, uf.y) && contains(b.z, uf.z),
                    refines(&surf, window, window),
                )
            }
            None => (
                "(rule (c) declines)".to_string(),
                width(u_ref.z),
                z_today,
                true,
                refines_today,
            ),
        };
        if w_c < width(u_ref.z) {
            narrowed += 1;
        }
        if z_c < z_today {
            tighter += 1;
        }
        if refines_c && !refines_today {
            refined += 1;
        }
        if holds {
            contained += 1;
        }
        println!(
            "| {i} | ({:?}, {:?}, {:?}) | [{:?}, {:?}] | [{:?}, {:?}] | {:e} | {u_c} | {w_c:e} | {z_today:e} | {z_c:e} | {holds} | {refines_today} | {refines_c} |",
            nf.x,
            nf.y,
            nf.z,
            normal.z.lo(),
            normal.z.hi(),
            u_ref.z.lo(),
            u_ref.z.hi(),
            width(u_ref.z),
        );
    }
    println!(
        "walls: {}; u_ref.z narrowed by (c): {narrowed}; cell z-enclosure tightened: {tighter}; \
         newly refining under (c): {refined}; (c) frame contains the f64 frame: {contained}",
        DUMBBELL.len()
    );
}

/// **Table 4** — at a metre-scale window and at the ε-scaled window the
/// M10-5 measurement was taken over.
#[test]
#[ignore = "the (c) payoff instrument; run explicitly"]
fn what_rule_c_buys_on_the_twelve_gon_prism() {
    payoff_over(WINDOW);
    payoff_over(EPS_WINDOW);
}
