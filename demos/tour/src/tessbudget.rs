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
//!
//! That namespace is a CONTRACT with `tools/tess-lint`, not a
//! convention: `(scene, face)` is the gate's per-face join key and its
//! `parse` refuses a repeated pair as harness breakage. This driver is
//! the only place that can make the key unique — the tour's scene list
//! is where the names come from — so [`run`] checks it, per body, and
//! panics naming the collision (`D402`).

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
    // The scene key's uniqueness guarantee (`D402`), owed to
    // `tools/tess-lint`: `(scene, face)` is its per-face join key, and
    // its `parse` refuses a second row for one pair outright, because
    // every index by that key would otherwise resolve the collision by
    // keeping whichever row it saw last — a mis-join dressed as a
    // reading. `face` is the mesh-patch ordinal, unique within a body
    // by construction, so uniqueness of the join key is exactly
    // uniqueness of the scene key across the whole walk. Nothing else
    // can make it: the names are the tour's, and this is the mode that
    // formats them.
    //
    // Two ways it breaks cheaply, and both fail LOUD here rather than
    // in the CSV — a demo that silently renamed or dropped a body would
    // hand the gate a sweep it could not say was wrong:
    //
    //   * two bodies of one stop (or two stops) sharing a name — the
    //     seen-set below;
    //   * a `/` in either name, which would let two DISTINCT
    //     `(stop, body)` pairs format to one key. Banned rather than
    //     left to the seen-set, so the format stays INJECTIVE and the
    //     panic names the pair that broke the namespace rather than
    //     whichever body happened to arrive second.
    //
    // A `,` or a newline in a name is the third shape, and it is
    // already loud one step downstream: it would widen or split the
    // row, and `parse` refuses on the field count before it ever reads
    // the key.
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    walk_tour(
        &mut |stop| {
            for sb in &stop.bodies {
                assert!(
                    !stop.name.contains('/') && !sb.name.contains('/'),
                    "tess-budget: a `/` in a tour name makes the `<stop>/<body>` \
                     namespace ambiguous — stop {:?}, body {:?}. Rename one of them; \
                     the separator is the namespace",
                    stop.name,
                    sb.name
                );
                let scene = format!("{}/{}", stop.name, sb.name);
                assert!(
                    seen.insert(scene.clone()),
                    "tess-budget: scene key {scene:?} is not unique — two tour bodies \
                     wear one `<stop>/<body>` name, and `tools/tess-lint`'s parse \
                     refuses the repeated `(scene, face)` join key as harness breakage. \
                     Rename the body"
                );
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
        // The sweep writes a CSV, not a scene directory, so it has no
        // outdir to hand the assembly stop its document store. A
        // scratch directory is the honest answer: the store is an
        // INPUT to the tour, and this mode only measures the meshes
        // that come out.
        &std::env::temp_dir().join("pncad-tess-budget-assembly"),
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
