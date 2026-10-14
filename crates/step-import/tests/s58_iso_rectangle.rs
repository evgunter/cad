//! **S58 / #649: the two proven doors to a wrongly-measured body.**
//!
//! `props`' rim-group span-SUM rule once accepted a cross-shaped iso
//! domain on a cylindrical wall, and `topo::mass_properties` then
//! returned a volume **19% low with `volume_pad = 0.0`** — a certificate
//! of exactness on a wrong number, with tier-3 `validate_geometric`
//! green. Two doors reached it, both executed in #649 and both pinned
//! here; a cylinder wall's flux is now its chart Green form, which
//! integrates the region the boundary bounds, so both doors lead to the
//! EXACT volume:
//!
//! * **STEP import** — `cross.step` is a real, manifold, closed,
//!   tier-3-clean keyed shaft (24 V / 36 E / 14 F, χ = 2) whose two
//!   cylindrical walls have a plus-shaped domain;
//! * **`Body::merge_coplanar_faces()`** — a PUBLIC composition of Euler
//!   operators that moves no geometry and conserves χ, and once turned
//!   a body measuring EXACTLY 8.96e-7 m³ into the same body measuring
//!   7.2533e-7. `xsplit.step` is that same solid authored with each
//!   wall as three rectangular sub-faces on one shared cylindrical
//!   surface; merging them is what builds the plus.
//!
//! The fixtures are #649's own, committed here with their generator
//! (`fixtures/iso-rect/gen_iso_rect.py`). `rect.step` is the CONTROL —
//! the same annular sector with no arms.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

fn fixture(name: &str) -> String {
    let path = format!(
        "{}/tests/fixtures/iso-rect/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {path}: {e}"))
}

/// The generator's parameters (mm in the file, SI here): R = 10,
/// RI = 6, H = 20, Z1 = 6, Z2 = 14, UC = 0.5 rad, UO = 1.0 rad.
const DR2: f64 = 0.010 * 0.010 - 0.006 * 0.006;
/// Volume of an annular sector of angular width `du` and height `h`.
fn ring(du: f64, h: f64) -> f64 {
    0.5 * du * DR2 * h
}

/// `rect.step`: one annular sector, `u ∈ [−0.5, 0.5]`, full height.
const RECT_VOLUME: f64 = 6.4e-7;
/// `cross.step` / merged `xsplit.step`: the three-layer stack.
fn cross_volume() -> f64 {
    ring(1.0, 0.006) + ring(2.0, 0.008) + ring(1.0, 0.006)
}
/// `tee.step`: the column below `Z1`, the arms above it.
fn tee_volume() -> f64 {
    ring(1.0, 0.006) + ring(2.0, 0.014)
}

/// `name` imports as a solid, passes tier 3 and measures `exact`, pad 0.
fn imports_and_measures(name: &str, exact: f64) {
    let Ok(StepImport::Solid { body, .. }) =
        import_step(&fixture(name), &ImportOptions::default(), Tol::witness())
    else {
        panic!("{name} must import as a solid");
    };
    topo::validate_geometric(&body, Tol::witness())
        .unwrap_or_else(|e| panic!("{name} must pass tier 3: {e:?}"));
    let mp = topo::mass_properties(&body, Tol::witness())
        .unwrap_or_else(|e| panic!("{name} must measure: {e:?}"));
    assert_eq!(
        mp.volume_pad, 0.0,
        "{name}: the closed-form lane's pad is 0"
    );
    let rel = (mp.volume - exact).abs() / exact;
    assert!(
        rel < 1e-12,
        "{name} volume {:.15e} != exact {exact:.15e} (rel {rel:.3e})",
        mp.volume
    );
}

/// **The control.** A genuine iso-rectangle domain imports and measures
/// exactly, pad 0.
#[test]
fn rect_the_control_still_measures_exactly() {
    imports_and_measures("rect.step", RECT_VOLUME);
}

/// **Door 1: STEP import.** The cross-shaped domain imports and
/// measures exactly, not 19% low.
#[test]
fn cross_the_649_fixture_imports_and_measures_exactly() {
    imports_and_measures("cross.step", cross_volume());
}

/// The one-sided variant, whose rim-group sums differ (the span-sum rule
/// caught the EASY shape, which is what made #649 look covered): it
/// imports and measures exactly too.
#[test]
fn tee_the_one_sided_variant_imports_and_measures_exactly() {
    imports_and_measures("tee.step", tee_volume());
}

/// **Door 2: `Body::merge_coplanar_faces()`.** The same solid authored
/// with rectangular sub-faces measures EXACTLY. The public merge then
/// fuses the same-surface-key wall fragments into plus-shaped faces —
/// no geometry moves, χ is conserved — and the result must measure the
/// same exact volume rather than certify 7.2533e-7 with pad 0.
#[test]
fn merge_coplanar_faces_no_longer_turns_an_exact_body_into_a_wrong_one() {
    let Ok(StepImport::Solid { mut body, .. }) = import_step(
        &fixture("xsplit.step"),
        &ImportOptions::default(),
        Tol::witness(),
    ) else {
        panic!("xsplit.step (rectangular sub-faces) must import as a solid");
    };
    let exact = cross_volume();
    let before = topo::mass_properties(&body, Tol::witness()).expect("the sub-faced body measures");
    let rel = (before.volume - exact).abs() / exact;
    assert!(
        rel < 1e-12 && before.volume_pad == 0.0,
        "BEFORE merge: {:.15e} (pad {:.3e}) != exact {exact:.15e}",
        before.volume,
        before.volume_pad
    );

    let out = body
        .merge_coplanar_faces(Tol::witness())
        .expect("the merge itself is a legal Euler-op composition and still runs");
    // Exactly the two cylindrical walls, each of whose three
    // rectangular sub-faces shares one SurfaceKey. `is_empty()` alone
    // would stay green if the merge found one wall, or found something
    // else entirely; the count is what says it did the thing this row
    // is about. (The load-bearing assertion is still the volume
    // below — this one only keeps a no-op merge from making it
    // vacuous.)
    assert_eq!(
        out.groups.len(),
        2,
        "the merge must fuse exactly the two cylindrical walls, got {:?}",
        out.groups
    );

    // The door: the merged body measures what it measured before.
    let after = topo::mass_properties(&body, Tol::witness()).expect("the merged body measures");
    let rel = (after.volume - exact).abs() / exact;
    assert!(
        rel < 1e-12 && after.volume_pad == 0.0,
        "AFTER merge: {:.15e} (pad {:.3e}) != exact {exact:.15e}",
        after.volume,
        after.volume_pad
    );
}
