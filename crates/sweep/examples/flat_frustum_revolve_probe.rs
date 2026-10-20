//! **Where a flat frustum stops revolving**: a measurement probe for
//! `a-flat-frustums-revolve-refuses-its-rim-image-at-1e-12`.
//!
//! The frustum `(0, 0), (r0, 0), (top·S, S), (0, S)` with `r0 = top·S +
//! S·tan α`, revolved a full turn about `y` at the run's ε. Each pose
//! prints whether it builds, or its refusal.
//!
//! - `ALPHAS=… SCALES=…`: the grid at `top = ½`.
//! - `HUNT=n [SEED=s]`: `n` fresh poses, α ∈ [0.9, 1.565), S ∈ [0.1,
//!   100), top ∈ [0.05, 0.95), printing each refusal and the count.
//!
//! `CAD_TOLERANCE_EPS=1e-12 HUNT=300 cargo run -p sweep --release --example flat_frustum_revolve_probe`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, SketchPlane};

fn build(alpha: f64, scale: f64, top: f64) -> Result<(), String> {
    let tol = Tol::witness();
    let r1 = top * scale;
    let r0 = r1 + scale * alpha.tan();
    let verts = vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(r0, 0.0), 0.0),
        (Point2::new(r1, scale), 0.0),
        (Point2::new(0.0, scale), 0.0),
    ];
    let profile = Profile::new(
        SketchPlane::<f64>::xy(),
        vec![profile::test_support::bulge_loop(verts)],
    )
    .validate(tol)
    .map_err(|e| format!("profile {e:?}"))?;
    let axis = sweep::RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    sweep::revolve(&profile, axis, sweep::Revolution::Full, tol)
        .map(|_| ())
        .map_err(|e| format!("{e:?}"))
}

fn list(var: &str) -> Vec<f64> {
    std::env::var(var)
        .unwrap_or_else(|_| panic!("set {var}"))
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    if let Ok(n) = std::env::var("HUNT") {
        let n: usize = n.parse().unwrap();
        let mut seed: u64 =
            std::env::var("SEED").map_or(0x9E37_79B9_7F4A_7C15, |s| s.parse().unwrap());
        println!("SEED {seed}");
        let mut rnd = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed >> 11) as f64 / (1u64 << 53) as f64
        };
        let (mut built, mut refused) = (0, 0);
        for _ in 0..n {
            let alpha = 0.9 + 0.665 * rnd();
            let scale = 10f64.powf(-1.0 + 3.0 * rnd());
            let top = 0.05 + 0.9 * rnd();
            match build(alpha, scale, top) {
                Ok(()) => built += 1,
                Err(e) => {
                    refused += 1;
                    let r0 = top * scale + scale * alpha.tan();
                    println!("REFUSE alpha {alpha} S {scale} top {top} r0 {r0:.1}: {e}");
                }
            }
        }
        println!("HUNT built {built} refused {refused}");
        return;
    }
    for alpha in list("ALPHAS") {
        for scale in list("SCALES") {
            match build(alpha, scale, 0.5) {
                Ok(()) => println!("alpha {alpha} S {scale}: builds"),
                Err(e) => println!("alpha {alpha} S {scale}: {e}"),
            }
        }
    }
}
