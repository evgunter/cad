//! The δ sweep behind the tessellation rows: triangles and milliseconds
//! against chordal tolerance, on one body.
//!
//! The benchmark file measures two points on this curve because that is
//! what PERF-PLAN §5 asks for. This walks the whole curve, and it exists
//! because two points cannot tell a steep constant from a bad exponent —
//! which is the entire question finding 7b poses.
//!
//! What it printed on 2026-08-27 (4-core box, release, washer, 4 faces),
//! two runs, because one run cannot separate a trend from its noise:
//!
//! | δ | triangles | wall (run 1 / run 2) | exponent in n |
//! |---|---|---|---|
//! | 1e-2 | 308 | 0.75 / 0.98 ms | |
//! | 1e-3 | 964 | 1.96 / 2.27 ms | 0.87 / 0.73 |
//! | 1e-4 | 3040 | 9.44 / 10.7 ms | 1.36 / 1.35 |
//! | 1e-5 | 9596 | 70.8 / 87.8 ms | 1.75 / 1.83 |
//! | 1e-6 | 30340 | 642 / 733 ms | 1.91 / 1.84 |
//!
//! Triangle count scales as √10 per decade of δ, as a chordal criterion
//! should, and is identical run to run — the tessellation is
//! deterministic (D9), so only the milliseconds move.
//!
//! **Time does not scale with the triangle count.** The exponent climbs
//! from ~0.8 to **~1.85** and is flat between the last two rows in one
//! run and still rising in the other, which is as much as two runs can
//! say: the asymptote is somewhere near n² and is not yet reached at
//! 30k triangles. That is the CDT quadratic, and it is reached here
//! WITHOUT a hole — the washer's annulus caps put their vertices on two
//! concentric circles, so the input is near-cocircular by construction:
//! the degeneracy finding 7b names, arrived at by a slit rather than by
//! nesting. Whether a real document's faces reach it is a separate
//! question this example does not answer.
//!
//! Re-run it after any change to the insertion path (`spade` bulk
//! loading is the one PERF-PLAN §2.1 has queued): the number to move is
//! the trailing exponent, not the 642 ms.
//!
//!     cd benches && cargo run --release --example counts

use pncad::prelude::*;
use pncad::tolerance::Tol;

fn rect(x: (f64, f64), y: (f64, f64)) -> ClosedLoop<f64> {
    Open.at(p2(x.0, y.0))
        .line_to(p2(x.1, y.0), Tol::witness())
        .and_then(|t| t.line_to(p2(x.1, y.1), Tol::witness()))
        .and_then(|t| t.line_to(p2(x.0, y.1), Tol::witness()))
        .and_then(|t| t.line_to(Start, Tol::witness()))
        .expect("the rectangle authors")
}

fn main() {
    let profile = validated(
        SketchPlane::<f64>::xy(),
        vec![rect((1.0, 2.0), (0.0, 1.0)).into()],
        Tol::witness(),
    )
    .expect("the washer profile validates");
    let body = revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: v2(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the washer revolves")
    .body;

    println!("washer: {} faces", body.faces().count());
    let mut previous: Option<(f64, f64)> = None;
    for delta in [1e-2_f64, 1e-3, 1e-4, 1e-5, 1e-6] {
        let started = std::time::Instant::now();
        let mesh = tessellate(&body, delta, Tol::witness()).expect("the washer tessellates");
        let ms = started.elapsed().as_secs_f64() * 1e3;
        let tris: usize = mesh.patches.iter().map(|p| p.triangles.len()).sum();
        let growth = match previous {
            // The exponent is the point: time ~ tris^k, k = log(Δt)/log(Δn).
            Some((prev_tris, prev_ms)) => {
                let n = tris as f64 / prev_tris;
                let t = ms / prev_ms;
                format!(
                    "  {n:.2}x tris, {t:.2}x time, exponent {:.2}",
                    t.ln() / n.ln()
                )
            }
            None => String::new(),
        };
        println!(
            "delta {delta:e}: {tris:8} tris, {:8} verts, {ms:9.3} ms{growth}",
            mesh.positions.len()
        );
        previous = Some((tris as f64, ms));
    }
}
