//! **The import door's route to the meridian-free face.** A STEP solid
//! stating a sphere cap as one rim circle and its base disc — the pole
//! in the sphere face's interior, no meridian edge, no pole vertex —
//! imports as a `Solid`, passes every tier and measures its closed-form
//! volume. `mesh::tessellate` then refuses the sphere face typed
//! (`MeridianFreeCurvedFace`, whose doc says why).
//!
//! Two statements of the rim, both adopted as they are written — two
//! half arcs (`rimonly2.step`) and one closed circle edge
//! (`rimonly1.step`). Fixtures:
//! `fixtures/rim-only-cap/gen_rim_only_cap.py`.
//!
//! **This row is expected to change, by design.** A pole inside a face
//! is a vertex of it, so import owes a normalization that re-mints this
//! face in the seamed form:
//! `work/exch/import-normalizes-the-rim-only-cap.md`. The day it lands
//! the "adopted as stated" assertions below go red and the refusal
//! stops being reachable from these files — that is the normalization
//! working. The row then becomes that unit's: the body is normalized,
//! reported, and meshes.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::SurfaceKind;
use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

fn fixture(name: &str) -> String {
    let p = format!(
        "{}/tests/fixtures/rim-only-cap/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("reading {p}: {e}"))
}

#[test]
fn an_imported_rim_only_sphere_cap_passes_every_tier_and_the_mesh_lane_refuses_it_typed() {
    let tol = Tol::witness();
    // R = 10 mm, the rim at latitude 0.5 rad: the cap's height and the
    // closed form π h² (3R − h) / 3.
    let r = 0.010_f64;
    let h = r * (1.0 - 0.5_f64.sin());
    let exact = core::f64::consts::PI * h * h * (3.0 * r - h) / 3.0;
    for (name, rim_edges) in [("rimonly2.step", 2), ("rimonly1.step", 1)] {
        let imported = import_step(&fixture(name), &ImportOptions::default(), tol)
            .unwrap_or_else(|e| panic!("{name} imports: {e:?}"));
        let StepImport::Solid { body, .. } = imported else {
            panic!("{name} imports as a solid");
        };
        assert_eq!(
            body.edges().count(),
            rim_edges,
            "{name}: adopted as stated. If import now normalizes the rim-only cap \
             (work/exch/import-normalizes-the-rim-only-cap.md), this red is that unit \
             working: turn this row into the normalization's own"
        );
        assert_eq!(topo::validate_geometric(&body, tol), Ok(()), "{name}");
        let volume = topo::mass_properties(&body, tol).unwrap().volume;
        assert!(
            (volume - exact).abs() <= 1e-9 * exact,
            "{name}: volume {volume} vs the closed form {exact}"
        );
        let (cap, _) = body
            .faces()
            .find(|(_, f)| {
                SurfaceKind::of(body.get_surface(f.surface).unwrap()) == SurfaceKind::Sphere
            })
            .expect("the cap's sphere face");
        assert_eq!(
            mesh::tessellate(&body, 1e-4, tol).map(|_| ()),
            Err(mesh::TessellateError::MeridianFreeCurvedFace {
                face: cap,
                surface: SurfaceKind::Sphere,
            }),
            "{name}"
        );
    }
}
