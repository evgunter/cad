//! Delta reviewer probe (reach-delta-3977): check 10 on a valid
//! two-shell solid whose void is a thin disc (one cylindrical wall)
//! placed far from the world origin: a box with the disc subtracted
//! from its inside. Run on head and main.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Point2, Tol};
use sweep::test_support::{brick, cylinder_of_arcs_at};
use topo::{subtract, validate_geometric};

#[test]
fn dprobe_far_thin_disc_void() {
    let tol = Tol::witness();
    println!("DPROBE4|eps={:e}", tol.eps());
    for &(r, h) in &[(1e-3, 1e-6), (1e-3, 1e-5), (1e-2, 1e-6)] {
        if h < 15.0 * tol.eps() { continue; }
        for &d in &[0.0, 1e3, 5e3] {
            let (cx, cy, z0) = (d * 0.6, d * 0.48, d * 0.64);
            let a = 4.0 * r;
            let Ok(boxy) = std::panic::catch_unwind(|| brick::<f64>((cx - a, cx + a), (cy - a, cy + a), (z0 - a, z0 + a), tol)) else { continue };
            let Ok(disc) = std::panic::catch_unwind(|| cylinder_of_arcs_at(4, r, Point2::new(cx, cy), z0, h, tol)) else { continue };
            let res = subtract(&boxy, &disc, tol);
            let line = match res {
                Err(e) => format!("subtract refused: {}", e.to_string().chars().take(120).collect::<String>()),
                Ok(r) => match r.body() {
                    None => "empty".into(),
                    Some(b) => {
                        let b = &b.body;
                        let shells = b.solids().map(|(s, _)| b.shells_of_solid(s).map_or(0, |v| v.len())).sum::<usize>();
                        let g: String = format!("{:?}", validate_geometric(b, tol)).chars().take(160).collect();
                        format!("shells={shells}|tier3={g}")
                    }
                },
            };
            println!("DPROBE4|r={r:e} h={h:e} d={d:e}|{line}");
        }
    }
}
