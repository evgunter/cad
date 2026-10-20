//! Review R1 probes of PR 5422 that read the head's probe surface
//! (`test_support::aimed_rays_all`, on the review branch only).

#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::{Band, Point3, Tol};

/// Every aimed ray of a needle `1 × w × w` along the axes, `EMR_M` ε
/// across, both ways out: each ray's crossings and reading.
#[test]
#[ignore = "probe: run by hand"]
fn dump_needle_rays() {
    let tol = Tol::witness();
    let m: f64 = std::env::var("EMR_M").ok().and_then(|x| x.parse().ok()).unwrap_or(10.5);
    let w = m * tol.eps();
    for inverted in [false, true] {
        let body = topo::test_support::brick::<f64>((0.0, 1.0), (0.0, w), (0.0, w), tol);
        let body = if inverted { body.revert().expect("reverts") } else { body };
        for r in topo::test_support::aimed_rays_all(&body, Band::linear(tol).expect("band")) {
            println!("R1DUMP m={m} inverted={inverted}: {r}");
        }
    }
}

/// The same needle (and a unit cube) built through each rotation of
/// the parameter cube that maps it onto itself: re-indexed faces, edges
/// and loop starts, one geometry. Each pose's readings, every aimed ray.
#[test]
#[ignore = "probe: run by hand"]
fn permuted_needles_read_alike() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    // Rotations (det +1) of [0,1]^3 that keep the needle (x long, y and z w).
    let maps: [fn(f64, f64, f64) -> (f64, f64, f64); 6] = [
        |a, b, c| (a, b, c),
        |a, b, c| (1.0 - a, c, b),
        |a, b, c| (a, 1.0 - c, b),
        |a, b, c| (a, 1.0 - b, 1.0 - c),
        |a, b, c| (1.0 - a, 1.0 - b, c),
        |a, b, c| (1.0 - a, b, 1.0 - c),
    ];
    for m in [10.5, 12.0, 15.0, 21.0, 30.0, 1.0 / tol.eps()] {
        let w = m * tol.eps();
        for inverted in [false, true] {
            let mut lines = Vec::new();
            for (k, f) in maps.iter().enumerate() {
                let f = *f;
                let body = topo::test_support::mapped_cube::<f64>(
                    move |a, b, c| {
                        let (x, y, z) = f(a, b, c);
                        Point3::new(x, y * w, z * w)
                    },
                    tol,
                );
                let body = if inverted { body.revert().expect("reverts") } else { body };
                let all = topo::test_support::aimed_rays_all(&body, band);
                let reads: Vec<String> = all
                    .iter()
                    .filter(|s| !s.contains(" hits "))
                    .map(|s| s.chars().take(60).collect())
                    .collect();
                let side = topo::validate_geometric_structural(&body, tol);
                lines.push(format!("map{k}: {side:?} rays {reads:?}"));
            }
            for l in lines {
                println!("R1PERM eps={:e} m={m} inv={inverted} {l}", tol.eps());
            }
        }
    }
}
