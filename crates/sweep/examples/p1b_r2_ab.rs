//! **PCURVE P-1b, reviewer R2 — the `#1116` A/B, as an EXAMPLE binary.**
//!
//! Why an example and not a test row: `sweep`'s and `geom-brep`'s test
//! targets are single aggregated binaries (`tests/all.rs`, ~120 and ~40
//! suites), whose codegen does not fit in one 600 s express window on a
//! contended box — and a killed `rustc` leaves no artifact, so every window
//! restarts the same compile. An example links the crate and one file,
//! which does fit.
//!
//! It prints; it asserts nothing. This is EVIDENCE for a review, not a
//! gate (`memories/review-and-dependency-policy`'s promotion rule).
//!
//! ARM A is this head as it stands: `fillet`'s support strut reaches
//! rest through the scaffolding door.
//! ARM B is the same head with the conversion the spec ordered
//! reinstated at `fillet/surgery.rs`'s `"strut mev"` site.
//!
//! The question: on `die_fillet`'s own shape — a unit cube, all twelve
//! edges, so every support is a PLANE — does the conversion certify?

use geom::Surface;
use geom_core::{Band, Point2, Tol};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, SketchPlane};
use sweep::fillet::fillet_edges;
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A unit cube through the real profile → extrude path (the die blank's
/// shape; `test_support::cube` is feature-gated and not nameable here).
fn cube(l: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(l, 0.0),
        Point2::new(l, l),
        Point2::new(0.0, l),
    ]);
    let validated = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the square is a valid profile");
    extrude(&validated, Extrusion::Distance(l), Tol::witness())
        .expect("the square extrudes")
        .body
}

fn scaffold_descriptions(body: &Body<f64>) -> usize {
    body.edges()
        .filter(|(_, e)| {
            body.get_curve_geom(e.curve)
                .and_then(topo::CurveGeom::certified)
                .is_some_and(|c| matches!(c.description(), geom_brep::EdgeDescription::Scaffold(_)))
        })
        .count()
}

fn main() {
    let (l, r) = (1.0_f64, 0.15_f64);
    let blank = cube(l);

    // MAJOR-1's premise, measured on the blank the die is filleted from.
    let faces: Vec<bool> = blank
        .faces()
        .map(|(_, f)| matches!(blank.get_surface(f.surface), Some(Surface::Plane { .. })))
        .collect();
    println!(
        "[AB] die blank: {} faces, {} planes, {} edges",
        faces.len(),
        faces.iter().filter(|p| **p).count(),
        blank.edges().count()
    );

    let edges: Vec<EdgeKey> = blank.edges().map(|(k, _)| k).collect();
    match fillet_edges(&blank, &edges, r, band(), Tol::witness()) {
        Err(e) => {
            // ARM B's interesting outcome: if reinstating the conversion
            // makes the fillet refuse, the refusal (and its site) is the
            // measurement the unit's #1116 note claims.
            println!("[AB] fillet_edges REFUSED: {e:?}");
        }
        Ok(f) => {
            let scaffolds = scaffold_descriptions(&f.body);
            let tier3 = topo::validate_geometric(&f.body, Tol::witness());
            let (n_err, n_fence) = match &tier3 {
                Ok(()) => (0usize, 0usize),
                Err(v) => (
                    v.len(),
                    v.iter()
                        .filter(|e| matches!(e, topo::ValidationError::ScaffoldAtRest { .. }))
                        .count(),
                ),
            };
            println!(
                "[AB] fillet_edges OK: {} faces, {} edges | stored scaffolds = {scaffolds} | \
                 tier3 errors = {n_err} (ScaffoldAtRest = {n_fence})",
                f.body.faces().count(),
                f.body.edges().count()
            );
            if let Err(v) = &tier3 {
                for e in v.iter().take(6) {
                    println!("[AB]   tier3: {e:?}");
                }
            }
        }
    }
}
