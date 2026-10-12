//! A ball of radius 10 mm with poles on ±z, the cap `x > 8` cut away and
//! its sphere split at the equator into two faces on ONE spherical
//! surface. Each face holds a chart pole off its loop, and the region
//! the two form across the equator has the cut circle alone as its
//! boundary, with neither pole on an interior edge. Validity does not
//! yet refuse a chart singularity inside a face
//! (`work/restfront/validity-refuses-an-interior-chart-singularity.md`),
//! so the region's boundary encodes no side tier 3 may read, and the
//! body imports, passes tier 3 and measures exactly.
#![allow(clippy::unwrap_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

#[test]
fn a_seam_region_holding_its_poles_inside_its_faces_imports_and_measures() {
    let tol = Tol::witness();
    let path = format!(
        "{}/tests/fixtures/seam-region/side_cut_equator_split.step",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(&path).unwrap();
    let (r, h) = (0.010_f64, 0.002_f64);
    let exact = core::f64::consts::PI * (4.0 * r * r * r - h * h * (3.0 * r - h)) / 3.0;
    let body = match import_step(&text, &ImportOptions::default(), tol) {
        Ok(StepImport::Solid { body, .. }) => body,
        other => panic!("the side-cut ball does not import as a solid: {other:?}"),
    };
    topo::validate_geometric(&body, tol).unwrap_or_else(|e| panic!("tier 3 refuses it: {e:?}"));
    let volume = topo::mass_properties(&body, tol).unwrap().volume;
    assert!(
        (volume - exact).abs() <= 1e-9 * exact,
        "the side-cut ball measures {volume}, the closed form is {exact}"
    );
}
