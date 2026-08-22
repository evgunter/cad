//! The **tessellation-budget sweep** (issue #320): every tour scene
//! tessellated at its own δ with `mesh::budget` armed, dumped as one
//! CSV row per face.
//!
//! This is the DRIVER of the budget instrument — the same shape as the
//! K-telemetry sweep beside it (`probe`): the kernel records
//! (`mesh::budget`), `tools/tess-meter` turns the measurements into
//! rows, the tour drives every scene it has, and `tools/tess-lint`
//! reads the rows and names the findings. The tour is the right driver
//! for the same reason it is the K sweep's: it is the only place that
//! holds every scene the kernel can build, and it holds each one
//! exactly once ([`crate::walk_tour`]).
//!
//! ```sh
//! cargo run --features budget -- tess-budget /tmp/b.csv              # sizing
//! cargo run --features budget -- tess-budget /tmp/b.csv --deviation  # + resampling
//! ```
//!
//! Behind the `budget` feature because `mesh::budget` is: the meter is
//! gated at its own module boundary so a render build's kernel carries
//! no telemetry state, and `arm`/`take` therefore do not exist without
//! it. `scripts/tess_budget_sweep.sh` passes it; without it the mode
//! says so and exits, exactly as `k-probe` does.
//!
//! Without `--deviation` the sweep costs one tessellation per scene
//! and nothing else — the sizing columns are read off the sizing the
//! lane already did. With it, every emitted triangle is resampled
//! against its exact surface, which is where `worst_dev` (and so the
//! certificate-slack and total-slack factors) comes from, and it is
//! MUCH slower on exactly the faces the issue is about.
//!
//! Scenes are namespaced `<stop>/<body>`, mirroring the K sweep's
//! `demo/<scene>`: a stop can carry several bodies, and a budget row
//! that could not say which body it came from would be unusable for
//! the one question the sweep exists to answer.

use std::io::Write as _;

use pncad::mesh::budget::{self, Mode};

use crate::walk_tour;
use pncad::geom_core::Tol;

/// Runs the sweep, writing CSV to `path` (stdout when `None`).
pub fn run(path: Option<String>, deviation: bool, tol: Tol) {
    let mut out = String::from(tess_meter::CSV_HEADER);
    out.push('\n');
    let mut faces = 0usize;
    let mut triangles = 0usize;
    walk_tour(
        &mut |stop| {
            for sb in &stop.bodies {
                let scene = format!("{}/{}", stop.name, sb.name);
                budget::arm(if deviation {
                    Mode::Deviation {
                        samples_per_edge: tess_meter::DEV_SAMPLES,
                    }
                } else {
                    Mode::Sizing
                });
                // The mesh is what the rows are ABOUT: a face's chart and
                // triangle count are already in it, so the meter is not
                // asked to report them.
                let mesh = pncad::mesh::tessellate(&sb.body, stop.delta, tol).unwrap_or_else(|e| {
                    panic!("{scene}: tessellate at delta {}: {e:?}", stop.delta)
                });
                let measures = budget::take();
                let rows = tess_meter::face_rows(stop.delta, &sb.body, &mesh, &measures);
                for row in &rows {
                    triangles += row.triangles;
                    out.push_str(&row.csv_row(&scene));
                    out.push('\n');
                }
                faces += rows.len();
                println!(
                    "   [{scene}] delta = {:.0e}: {} faces, {} triangles",
                    stop.delta,
                    rows.len(),
                    rows.iter().map(|r| r.triangles).sum::<usize>()
                );
            }
        },
        tol,
    );
    println!("tess-budget: {faces} face rows, {triangles} triangles total");
    match path {
        Some(p) => {
            std::fs::write(&p, out).unwrap_or_else(|e| panic!("write {p}: {e}"));
            println!("wrote {p}");
        }
        None => {
            let mut stdout = std::io::stdout();
            stdout.write_all(out.as_bytes()).expect("write stdout");
        }
    }
}
