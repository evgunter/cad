//! **Issue 723's import-door reproduction, committed: the sphere
//! polar extent through `import_step`.**
//!
//! The fixtures (re-derived from the issue's text by
//! `fixtures/halfcap/gen_halfcap.py`; the originals died with their
//! machine) are literally half of a spherical cap — R = 10 mm above
//! latitude 0.5 rad, cut by a plane through the axis — whose sphere
//! face's meridian side is ONE pole-crossing great-circle arc.
//!
//! * `halfcap.step` splits that arc with one ORDINARY vertex
//!   (3 V / 4 E / 3 F, χ = 2). Before the span-derived extent this
//!   imported, passed tier 3 and certified **−47.187%** of the true
//!   volume at `pad = 0.0`: the endpoint fold saw latitudes
//!   `{sin 0.5, sin 1.0}` and never the pole in the arc's interior.
//! * `halfcap_nosplit.step` is the identical solid with the arc as
//!   one edge (2 V / 3 E / 3 F). Its endpoint latitudes coincide, so
//!   the endpoint fold refused it `DegenerateFace` — the alarm shape:
//!   one vertex of pure topology flipping a refusal into a wrong
//!   certified number.
//!
//! With the extent taken from each arc's stored span, the two twins
//! are what they geometrically are — the SAME solid — and both must
//! certify the SAME exact closed-form volume, pad 0. The no-split
//! twin's old refusal was an artifact of the endpoint fold, not a
//! fact about the geometry: the face is an honest half-cap with
//! positive extent, so measuring it exactly is the honest answer and
//! the refusal legitimately retires.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

fn fixture(name: &str) -> String {
    let path = format!(
        "{}/tests/fixtures/halfcap/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {path}: {e}"))
}

/// The generator's parameters, SI: R = 10 mm, base latitude 0.5 rad.
const R: f64 = 0.010;
const B: f64 = 0.5;

/// Half of the spherical-cap volume `π·h²·(3R − h)/3`, `h = R(1 − sin B)`
/// — 3.518158565e-7 m³ at these parameters (issue 723's exact figure).
fn exact_volume() -> f64 {
    let h = R * (1.0 - B.sin());
    core::f64::consts::PI * h * h * (3.0 * R - h) / 6.0
}

fn certifies_exactly(name: &str) {
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
    let exact = exact_volume();
    let rel = (mp.volume - exact).abs() / exact;
    assert!(
        rel < 1e-12,
        "{name}: volume {:.15e} != exact {exact:.15e} (rel {rel:.3e})",
        mp.volume
    );
}

/// **The split twin** — the executed −47% door — certifies the exact
/// closed-form volume.
#[test]
fn the_split_half_cap_certifies_the_exact_volume() {
    certifies_exactly("halfcap.step");
}

/// **The no-split twin** — the old `DegenerateFace` refusal —
/// certifies the same exact volume: the twins are one solid, and the
/// answer no longer depends on a vertex that moves no geometry.
#[test]
fn the_no_split_twin_certifies_the_same_volume() {
    certifies_exactly("halfcap_nosplit.step");
}

/// **The near-pole split twins** — the split vertex 1e-6 / 1e-7 rad
/// off the pole (10 nm / 1 nm of arc), landing the pole-membership
/// margin inside or beside the default band. Reviewer-executed red:
/// before the indeterminate outcome folded, both flipped
/// certify-exactly into
/// `TierInvalid { VolumeUncomputable { Escalated { .. "props_meridian_pole" } } }`
/// — an import refusal of a solid whose exact area is not in doubt
/// (the fold choices differ by ~band²/2). Both must certify exactly,
/// like every other authoring of the same solid.
#[test]
fn a_split_vertex_a_hair_off_the_pole_certifies_through_the_door() {
    certifies_exactly("halfcap_eps6.step");
    certifies_exactly("halfcap_eps7.step");
}
