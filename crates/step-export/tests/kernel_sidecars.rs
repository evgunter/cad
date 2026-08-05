//! The KERNEL_* sidecar staleness row: every committed `.expect`
//! sidecar's `KERNEL_SOLIDS` / `KERNEL_SHELLS` / `KERNEL_FACES` /
//! `KERNEL_EDGES` / `KERNEL_VERTICES` / `KERNEL_VOLUME_MM3` fields are
//! asserted against the LIVE kernel census and certified volume of the
//! source body (`common::fixture_corpus()` rebuilds them), so the
//! fields can never rot.
//!
//! Semantics: `KERNEL_*` is the NATIVE body's census — what the kernel
//! actually has — as opposed to the `EXPECT_*` fields, which record
//! what FreeCAD/OCC *reports after importing* the exported file (OCC
//! adds degenerate pole edges and splits periodic carriers at seams —
//! normalisations D7 forbids the kernel to mirror, so the two censuses
//! legitimately differ on several fixtures). `scripts/check_step.sh`
//! consumes `EXPECT_*` only; `crates/step-import`'s committed-corpus
//! row consumes `KERNEL_*`. The one structural divergence beyond edge
//! normalisation is `kiss_assembly`: natively ONE solid with TWO
//! shells, but STEP has no way to group two closed shells into one
//! solid (`MANIFOLD_SOLID_BREP` carries exactly one), so both the OCC
//! and the kernel-import readings are 2 solids / 2 shells — the
//! documented divergence IS the semantics, see the sidecar's comment.
//!
//! `KERNEL_VOLUME_MM3` is FULL precision: the committed literal must be
//! byte-identical to `step_export::fmt_real`'s output for the live
//! certified volume (× 1e9 mm³/m³), and that printer's pinned property
//! is that its output parses back to the identical f64 bits — so the
//! byte equality asserted here IS the bit-exact round-trip.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

/// The staleness row (15/15): rebuilds every corpus body and checks
/// each committed sidecar's KERNEL_* lines against the live values. On
/// failure the message prints the full expected block per fixture —
/// paste-ready for regeneration after a deliberate builder change.
#[test]
fn kernel_sidecar_fields_match_live_kernel() {
    let mut failures: Vec<String> = Vec::new();
    for (name, body) in common::fixture_corpus() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(format!("{name}.expect"));
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
        let props =
            topo::mass_properties(&body).unwrap_or_else(|e| panic!("{name}: mass properties: {e}"));
        let volume_literal = step_export::fmt_real(props.volume * 1e9, "KERNEL_VOLUME_MM3")
            .unwrap_or_else(|e| panic!("{name}: printing volume: {e}"));
        let want = [
            ("KERNEL_SOLIDS", body.solids().count().to_string()),
            ("KERNEL_SHELLS", body.shells().count().to_string()),
            ("KERNEL_FACES", body.faces().count().to_string()),
            ("KERNEL_EDGES", body.edges().count().to_string()),
            ("KERNEL_VERTICES", body.vertices().count().to_string()),
            ("KERNEL_VOLUME_MM3", volume_literal),
        ];
        for (key, value) in want {
            // Exactly one line per key, byte-equal to the live value
            // (for the volume, byte equality against the printer's
            // output is the bit-exact round-trip — module docs).
            let lines: Vec<&str> = text
                .lines()
                .filter_map(|l| l.strip_prefix(&format!("{key}=")))
                .collect();
            if lines != [value.as_str()] {
                failures.push(format!("{name}: want {key}={value}, sidecar has {lines:?}"));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "stale or missing KERNEL_* sidecar fields (live kernel census/volume vs \
         committed .expect):\n{}",
        failures.join("\n")
    );
}
