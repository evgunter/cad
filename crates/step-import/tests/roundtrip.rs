//! Acceptance rows 1–3 (M7-1 spec §2): the committed-corpus row, the
//! fixed-point row, and — by construction throughout — the comparison
//! discipline (counts, certified scalars, structural invariants only;
//! see `common`'s module docs).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{SOLID_FIXTURES, census, expect_sidecar, fixture, import_body};
use geom_core::Tol;

/// Row 1 per fixture: census against the sidecar's `KERNEL_*` fields
/// (the NATIVE census — the `EXPECT_*` counts record OCC's import-side
/// normalisations, which D7 forbids the kernel to mirror), certified
/// volume against `KERNEL_VOLUME_MM3 × 1e-9` m³, and the validity
/// ladder at default ε.
///
/// Solids and shells alone come from `EXPECT_*`: STEP cannot state
/// solid grouping (a `MANIFOLD_SOLID_BREP` is exactly one closed shell
/// as one solid), so the FILE's solid structure is what a faithful
/// import reproduces — `kiss_assembly` is natively 1 solid / 2 shells
/// and imports as the sidecar's 2 / 2, the divergence its comment
/// documents; `KERNEL_SOLIDS` pins the native side, which no import of
/// the exported file can reach.
fn committed_corpus_row(name: &str) {
    let (body, _eps) = import_body(name);
    let expect = expect_sidecar(name);
    assert_eq!(
        census(&body),
        (
            expect.solids,
            expect.shells,
            expect.kernel_faces,
            expect.kernel_edges,
            expect.kernel_vertices
        ),
        "{name}: census (solids, shells, faces, edges, vertices)"
    );

    // Volume tolerance derivation (spec row 1): `KERNEL_VOLUME_MM3` is
    // FULL precision — the committed literal is the export writer's
    // round-tripping printer's output for (native certified volume ×
    // 1e9), asserted against the live kernel by step-export's
    // staleness row — so there is NO print-precision slop term. The
    // native volume is a certified enclosure `value ± pad` (pad zero
    // for closed-form-only bodies, and a function of ambient ε for
    // quadrature bodies — the sidecar records BOTH halves at the
    // corpus's declared ε = 1e-9), and the imported volume is another
    // certified enclosure of the SAME true volume at the ambient ε of
    // this run. Both contain the true value, so the budget is the sum
    // of the two half-widths — sound at EVERY ε row — plus ulp
    // accounting:
    //   * `kernel_faces` ulps of the expected value, twice: each side's
    //     volume is a fixed-order sum of per-face contributions, each
    //     rounded (exact only on dyadic inputs), and the two sums may
    //     accumulate in DIFFERENT arena orders (import traversal need
    //     not match construction order) — so each side sits up to
    //     `faces` ulps from the exact value.
    //   * 2 ulps for the mm³ round-trip: × 1e9 at print time and
    //     × 1e-9 here each round once (neither scale is a power of
    //     two).
    //
    // Net effect vs the old (analytic EXPECT + print-precision slop)
    // budget, MEASURED at default ε (the #199 review): NOT uniformly
    // tighter. The print-slop term is gone — the structural win — so
    // the three fixtures it dominated tightened 8e3×–8.3e7×
    // (filleted_die, composed_die, notched); the other eleven LOOSENED
    // ~2–3× at the 1e-6 mm³ scale because the native pad now counts
    // honestly (cut_cylinder's budget 1186 → 2373 mm³). The trade has
    // teeth both ways: the review's scale-corruption probe
    // (`review_k3_probe`) shows the new budget catching corruptions
    // the old slop accepted, at 1600×/8000× margins.
    let props =
        topo::mass_properties(&body, Tol::witness()).unwrap_or_else(|e| panic!("{name}: {e}"));
    let expected_m3 = expect.kernel_volume_mm3 * 1e-9;
    let native_pad_m3 = expect.kernel_volume_pad_mm3 * 1e-9;
    let ulp = expected_m3.next_up() - expected_m3;
    let ulps = (2.0 * expect.kernel_faces as f64 + 2.0) * ulp;
    let tolerance = props.volume_pad + native_pad_m3 + ulps;
    assert!(
        (props.volume - expected_m3).abs() <= tolerance,
        "{name}: volume {} m³ vs native {} m³ (tolerance {}: pad {} + native pad {} + ulps {})",
        props.volume,
        expected_m3,
        tolerance,
        props.volume_pad,
        native_pad_m3,
        ulps
    );

    // The validity ladder, at default ε — the same tiers the corpus
    // bodies pass natively.
    assert_eq!(topo::validate(&body), Ok(()), "{name}: tier 1");
    assert_eq!(topo::validate_closed(&body), Ok(()), "{name}: tier 2");
    assert_eq!(
        topo::validate_geometric(&body, Tol::witness()),
        Ok(()),
        "{name}: tier 3"
    );
}

#[test]
fn committed_corpus() {
    for name in SOLID_FIXTURES {
        committed_corpus_row(name);
    }
}

/// Row 2: import → export → import: censuses and certified volumes
/// identical run to run, and the SECOND export byte-identical to the
/// FIRST (one adoption pass is a fixed point of the deterministic
/// writer). Byte-identity of the first re-export against the
/// COMMITTED fixture is measured and reported (printed per fixture),
/// not required — entity numbering/traversal may differ.
#[test]
fn fixed_point() {
    for name in SOLID_FIXTURES {
        // The committed fixtures were written with the fixture's name
        // as product name (writer defaults otherwise), so the
        // measured comparison below is whole-file fair.
        let options = step_export::StepOptions {
            product_name: name.to_owned(),
            ..step_export::StepOptions::default()
        };
        let (body1, _) = import_body(name);
        let export1 = step_export::step_string(&body1, &options, Tol::witness())
            .unwrap_or_else(|e| panic!("{name}: re-export 1: {e}"));
        // STEP has no native contact concept, so export DROPS the
        // kiss declaration (the honest C6 posture) and the re-import
        // must re-attach it — the recipe is the save format; the STEP
        // file never was.
        let reimport_options = if name.contains("kiss_assembly") {
            step_import::ImportOptions {
                declared_contacts: vec![step_import::ImportContact::VertexRest {
                    at: [1.0, 1.0, 1.0],
                }],
                ..step_import::ImportOptions::default()
            }
        } else {
            step_import::ImportOptions::default()
        };
        let reimport = step_import::import_step(&export1, &reimport_options)
            .unwrap_or_else(|e| panic!("{name}: re-import: {e}"));
        let step_import::StepImport::Solid { body: body2, .. } = reimport else {
            panic!("{name}: re-import lost the solid");
        };
        assert_eq!(
            census(&body1),
            census(&body2),
            "{name}: census identical across the adoption pass"
        );
        let v1 = topo::mass_properties(&body1, Tol::witness())
            .unwrap()
            .volume;
        let v2 = topo::mass_properties(&body2, Tol::witness())
            .unwrap()
            .volume;
        assert_eq!(
            v1.to_bits(),
            v2.to_bits(),
            "{name}: certified volume bit-identical across the adoption pass"
        );
        let export2 = step_export::step_string(&body2, &options, Tol::witness())
            .unwrap_or_else(|e| panic!("{name}: re-export 2: {e}"));
        assert_eq!(
            export1, export2,
            "{name}: the second export must be byte-identical to the first"
        );

        // The measured (not required) comparison against the
        // committed fixture; report with `--nocapture`. The uncertainty
        // record may legitimately differ only if the ambient ε does —
        // both are 1e-9 here, so a diff is purely entity
        // numbering/traversal.
        let committed = fixture(name, "step");
        println!(
            "fixed-point measurement: {name}: first re-export {} the committed fixture",
            if export1 == committed {
                "byte-identical to"
            } else {
                "diverges from"
            }
        );
    }
}
