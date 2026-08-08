//! R1 DELTA re-review probes (fix pass d058dbe). Uncommitted lane-only
//! rows; `R1_PROBE_DIR` points at the reviewer's generated fixtures.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::census;
use step_import::{ImportOptions, StepImport, import_step};

fn probe(name: &str) -> Option<String> {
    let dir = std::env::var("R1_PROBE_DIR").ok()?;
    Some(std::fs::read_to_string(format!("{dir}/{name}")).unwrap())
}

/// MAJ-2 re-adjudication: the misread window must be GONE, not moved.
/// Washer solids with the rim vertex at azimuths covering the whole
/// circle (dense over the old (0.4pi, 0.5pi] window) must ALL import
/// to the true closed form.
#[test]
fn r1_rr_wrap_window_is_gone_everywhere() {
    let Some(_) = std::env::var("R1_PROBE_DIR").ok() else {
        eprintln!("R1_PROBE_DIR unset: skipping");
        return;
    };
    let pi = std::f64::consts::PI;
    let v_true = (400.0 * pi + 40.0 * pi * pi + 32.0 * pi / 3.0) * 1e-9;
    // az 0.01 deg: the vertex sits 1.7e-6 m off the azimuth, INSIDE
    // eps_in (1e-5 m) — the mint adopts it as the seam vertex and the
    // Seam-only adoption then refuses typed (the m2 posture). Asserted
    // separately below; the window sweep proper starts past eps_in.
    {
        let text = probe("sweep_washer_0p01.stp").unwrap();
        let e = import_step(&text, &ImportOptions::default()).unwrap_err();
        eprintln!("az 0.01 (sub-eps_in): {e}");
        assert!(e.to_string().contains("seam"), "az 0.01: {e}");
    }
    let azs = [
        "15", "45", "60", "70", "72", "73", "75", "78", "80", "82", "85", "88",
        "89", "90", "91", "95", "108", "110", "135", "179", "180", "181", "225", "252",
        "260", "268", "270", "272", "315", "359",
    ];
    for az in azs {
        let text = probe(&format!("sweep_washer_{az}.stp")).unwrap();
        let Ok(StepImport::Solid { body, .. }) = import_step(&text, &ImportOptions::default())
        else {
            panic!(
                "az {az}: refused: {:?}",
                import_step(&text, &ImportOptions::default()).err()
            );
        };
        let v = topo::mass_properties(&body).unwrap().volume;
        let rel = ((v - v_true) / v_true).abs();
        assert!(
            rel < 1e-12,
            "az {az}: volume {v:e} vs true {v_true:e} (rel {rel:e})"
        );
        assert_eq!(
            topo::validate_geometric(&body),
            Ok(()),
            "az {az}: tier 3 at rest"
        );
    }
    eprintln!("all {} azimuths import at the true closed form", azs.len());
}

/// m2/n2 spot-check: a rim circle tilted 0.01 rad off the surface
/// axis (tilt*radius = 1e-4 m >> eps_in) takes the TYPED band
/// coaxiality refusal, not a generic downstream message.
#[test]
fn r1_rr_tilted_rim_takes_the_typed_band_refusal() {
    let Some(text) = probe("washer_tilted_rim.stp") else {
        eprintln!("R1_PROBE_DIR unset: skipping");
        return;
    };
    let e = import_step(&text, &ImportOptions::default()).unwrap_err();
    let msg = e.to_string();
    eprintln!("tilted rim: {msg}");
    assert!(
        msg.contains("circles coaxial with the surface within the file's own"),
        "expected the typed coaxiality band refusal, got: {msg}"
    );
}

/// MAJ-1 message shape: the c180 refusal is typed BandGeometryInvalid
/// naming NegativeVolume, raised from `import_step` itself.
#[test]
fn r1_rr_inside_out_refusal_names_negative_volume() {
    let text = std::fs::read_to_string(format!(
        "{}/tests/fixtures/band/band_c180.stp",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();
    let e = import_step(&text, &ImportOptions::default()).unwrap_err();
    let msg = e.to_string();
    eprintln!("c180: {msg}");
    assert!(
        msg.contains("tier-3 geometric validation") && msg.contains("NegativeVolume"),
        "unexpected refusal: {msg}"
    );
    // washer90's old silent complement now also both true-volume green
    // (checked by the committed pin) — and the doctored voff_mid class
    // still refuses at seam certification (adoption ladder, Seam-only).
    if let Some(mid) = probe("ftc11_voff_mid.stp") {
        let e = import_step(&mid, &ImportOptions::default()).unwrap_err();
        eprintln!("voff_mid: {e}");
        assert!(e.to_string().contains("seam"), "voff_mid: {e}");
    }
}
