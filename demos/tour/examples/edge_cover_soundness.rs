//! **Every stored edge of every tour body lies inside its cover's box**:
//! the guard of the one reading of an edge's region
//! (`docs/DESIGN.md` `[one-way.places]`). For each edge with a stored
//! curve, the carrier lifted to [`Interval`] is covered over its span
//! ([`pncad::geom::Curve3::cover_over`]) and the cover's world box is
//! read at its brackets' ends ([`Cover::box_in`]), the box
//! `topo::clearance::boundary_of` reads a face's window off. The edge's
//! carrier is then sampled at 513 parameters across the span at `f64`,
//! and every sample must lie inside that box.
//!
//! Prints one `COVER <stop>/<body> | edges <n> | worst escape <e>` line
//! per body and a total, and panics if any sample escapes its box: a
//! cover that is not an enclosure.
//!
//! ```sh
//! cd demos/tour
//! CARGO_TARGET_DIR=/home/user/<lane>-target \
//!   cargo run --release --example edge_cover_soundness | grep '^COVER'
//! ```
//!
//! [`Cover::box_in`]: pncad::geom::cover::Cover::box_in
use pncad::geom::cover::{BoxAxes, ParamWindow};
use pncad::geom_core::{Bounds, Interval, Real, Tol};

const SAMPLES: u32 = 512;

fn main() {
    let tol = Tol::witness();
    // The assembly stop's document store.
    let work = std::env::temp_dir().join("edge-cover-soundness-store");
    std::fs::create_dir_all(&work).expect("create the assembly stop's store");
    let (mut edges, mut worst_all) = (0usize, 0.0f64);
    demo_tour::walk_tour(
        &mut |stop| {
            for sb in &stop.bodies {
                let body = &sb.body;
                let (mut covered, mut worst) = (0usize, 0.0f64);
                for (ek, _) in body.edges() {
                    let Ok(curve) = body.stored_curve(ek) else {
                        continue;
                    };
                    let (t0, t1) = curve.params();
                    let lifted = curve.carrier().map_scalar(Interval::from_f64);
                    let Ok(cover) = lifted.cover_over(ParamWindow::new(
                        Interval::from_f64(t0),
                        Interval::from_f64(t1),
                    )) else {
                        continue;
                    };
                    let (lo, hi) = cover.box_in(&BoxAxes::World);
                    covered += 1;
                    for k in 0..=SAMPLES {
                        let t = t0 + (t1 - t0) * f64::from(k) / f64::from(SAMPLES);
                        let p = curve.carrier().eval(t);
                        for (x, l, h) in [
                            (p.x, lo.x.lo(), hi.x.hi()),
                            (p.y, lo.y.lo(), hi.y.hi()),
                            (p.z, lo.z.lo(), hi.z.hi()),
                        ] {
                            worst = worst.max(l - x).max(x - h);
                        }
                    }
                }
                edges += covered;
                worst_all = worst_all.max(worst);
                println!(
                    "COVER {}/{} | edges {covered} | worst escape {worst:e}",
                    stop.name, sb.name
                );
            }
        },
        &work,
        tol,
    );
    println!("COVER TOTAL | edges {edges} | worst escape {worst_all:e}");
    assert!(
        worst_all <= 0.0,
        "an edge's sample left its cover's box by {worst_all:e}"
    );
}
