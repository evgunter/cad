//! **CERT-1 R1 import-door probes** — reviewer-authored near-polar
//! STEP bodies (variants of the committed halfcap generator with
//! adversarial parameters), against PR 1220's span-derived sphere
//! extent at frozen head bc815c2c.
//!
//! * `nearpolar_*`: base latitude 1.55 rad — the rim sits 0.0208 rad
//!   off the pole, the pole-crossing arc's interior extreme is a
//!   whisker above its endpoints; exact volume 7.344188413212748e-14
//!   m³ (π·h²·(3R−h)/6, h = R(1 − sin 1.55)).
//! * `polesplit_*`: issue 723's own body with the split vertex moved
//!   EXACTLY onto the north pole — the `props_meridian_pole`
//!   decision sits on its Zero through the STEP door; same exact
//!   volume as the issue's body.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

fn fixture(name: &str) -> String {
    let path = format!(
        "{}/tests/fixtures/cert1-r1/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {path}: {e}"))
}

fn certifies_exactly(name: &str, r: f64, b: f64) {
    let Ok(StepImport::Solid { body, .. }) =
        import_step(&fixture(name), &ImportOptions::default(), Tol::witness())
    else {
        panic!("{name} must import as a solid");
    };
    topo::validate_geometric(&body, Tol::witness())
        .unwrap_or_else(|e| panic!("{name} must pass tier 3: {e:?}"));
    let mp = topo::mass_properties(&body, Tol::witness())
        .unwrap_or_else(|e| panic!("{name} must measure: {e:?}"));
    assert_eq!(mp.volume_pad, 0.0, "{name}: closed-form pad is 0");
    let h = r * (1.0 - b.sin());
    let exact = core::f64::consts::PI * h * h * (3.0 * r - h) / 6.0;
    let rel = (mp.volume - exact).abs() / exact;
    assert!(
        rel < 1e-9,
        "{name}: volume {:.15e} != exact {exact:.15e} (rel {rel:.3e})",
        mp.volume
    );
}

/// Near-polar half-cap (rim 0.0208 rad off the pole), split and
/// no-split twins: both certify the closed form exactly.
#[test]
fn probe_near_polar_half_cap_twins_certify() {
    // ε-three-outcome honesty: at ambient 1e-6 these twins never
    // reach props — the rim/plane wedge angle's ADOPTION margin
    // (~8.6e-6 rad) is inside that band's escalation window, and the
    // coarse band honestly cannot tell this near-tangency from a
    // tangency. That cell is pinned by predicate name here and in
    // `tier_gate.rs`'s EPS_ROWS; the default and fine bands certify
    // the exact closed form.
    if (geom_core::Tol::witness().eps() - 1e-6).abs() < 1e-18 {
        for f in ["nearpolar_split.step", "nearpolar_nosplit.step"] {
            let err =
                step_import::import_step(&fixture(f), &ImportOptions::default(), Tol::witness())
                    .err()
                    .unwrap_or_else(|| {
                        panic!("{f}: expected the adoption escalation at ambient 1e-6")
                    })
                    .to_string();
            assert!(
                err.contains("predicate 'dihedral_wedge' indeterminate"),
                "{f}: the coarse-band refusal must be the wedge adoption escalation, got: {err}"
            );
        }
        return;
    }
    certifies_exactly("nearpolar_split.step", 0.010, 1.55);
    certifies_exactly("nearpolar_nosplit.step", 0.010, 1.55);
}

/// The split vertex exactly AT the pole: the pole-containment
/// decision is Zero on both sub-arcs, through the whole import/tier-3
/// pipeline, and the volume is still exact.
#[test]
fn probe_split_vertex_exactly_at_the_pole_certifies() {
    certifies_exactly("polesplit_split.step", 0.010, 0.5);
}
