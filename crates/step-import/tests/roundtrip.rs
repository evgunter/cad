//! Acceptance rows 1–3 (M7-1 spec §2): the committed-corpus row, the
//! fixed-point row, and — by construction throughout — the comparison
//! discipline (counts, certified scalars, structural invariants only;
//! see `common`'s module docs).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{
    SOLID_FIXTURES, census, expect_sidecar, fixture, import_body, kernel_census_override,
};

/// Row 1 per fixture: census against the sidecar (kernel counts where
/// the sidecar records OCC normalisation — `kernel_census_override`),
/// certified volume against `EXPECT_VOLUME_MM3 × 1e-9` m³, and the
/// validity ladder at default ε.
fn committed_corpus_row(name: &str) {
    let (body, _eps) = import_body(name);
    let expect = expect_sidecar(name);
    let (faces, edges, vertices) =
        kernel_census_override(name).unwrap_or((expect.faces, expect.edges, expect.vertices));
    assert_eq!(
        census(&body),
        (expect.solids, expect.shells, faces, edges, vertices),
        "{name}: census (solids, shells, faces, edges, vertices)"
    );

    // Volume tolerance derivation (spec row 1): the quadrature's own
    // certified bound is `volume_pad` (zero for closed-form-only
    // bodies); the sidecar's print precision is half the last
    // significant printed decimal place of the literal
    // (`print_precision_mm3`); plus one f64 ulp of the expected value
    // PER FACE for the closed forms' own rounded accumulation — the
    // volume is a fixed-order sum of per-face contributions, each
    // rounded (exact only on dyadic inputs), so the roundoff budget
    // scales with the number of summands (die_pips' 48 faces land a
    // measured 6.00 ulps off the printed analytic value).
    let props = topo::mass_properties(&body).unwrap_or_else(|e| panic!("{name}: {e}"));
    let expected_m3 = expect.volume_mm3 * 1e-9;
    let print_precision_m3 = print_precision_mm3(&expect.volume_literal) * 1e-9;
    let ulps = (faces as f64) * (expected_m3.next_up() - expected_m3);
    let tolerance = props.volume_pad + print_precision_m3 + ulps;
    assert!(
        (props.volume - expected_m3).abs() <= tolerance,
        "{name}: volume {} m³ vs expected {} m³ (tolerance {}: pad {} + print {} + ulps {})",
        props.volume,
        expected_m3,
        tolerance,
        props.volume_pad,
        print_precision_m3,
        ulps
    );

    // The validity ladder, at default ε — the same tiers the corpus
    // bodies pass natively.
    assert_eq!(topo::validate(&body), Ok(()), "{name}: tier 1");
    assert_eq!(topo::validate_closed(&body), Ok(()), "{name}: tier 2");
    assert_eq!(topo::validate_geometric(&body), Ok(()), "{name}: tier 3");
}

/// Half the last *significant* printed decimal place of a sidecar
/// volume literal, in mm³ — the standard significant-figures reading:
/// `952914984` → 0.5; `4188790204.7863903` → 5e-8; `965231000`
/// (three non-significant trailing zeros — the filleted die's value
/// was rounded to the nearest 1000 mm³; the exact value is
/// ≈965230999.48) → 500; `3000000000.0` → 0.05; exact scientific
/// forms (`1e9`, `8.75e8`) → 0 (closed-form exact, printed exactly).
fn print_precision_mm3(literal: &str) -> f64 {
    if literal.contains(['e', 'E']) {
        return 0.0;
    }
    match literal.split_once('.') {
        None => {
            let trailing_zeros = literal.len() - literal.trim_end_matches('0').len();
            0.5 * 10f64.powi(trailing_zeros as i32)
        }
        Some((_, frac)) => 0.5 * 10f64.powi(-(frac.len() as i32)),
    }
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
        let export1 = step_export::step_string(&body1, &options)
            .unwrap_or_else(|e| panic!("{name}: re-export 1: {e}"));
        let reimport = step_import::import_step(&export1, &step_import::ImportOptions::default())
            .unwrap_or_else(|e| panic!("{name}: re-import: {e}"));
        let step_import::StepImport::Solid { body: body2, .. } = reimport else {
            panic!("{name}: re-import lost the solid");
        };
        assert_eq!(
            census(&body1),
            census(&body2),
            "{name}: census identical across the adoption pass"
        );
        let v1 = topo::mass_properties(&body1).unwrap().volume;
        let v2 = topo::mass_properties(&body2).unwrap().volume;
        assert_eq!(
            v1.to_bits(),
            v2.to_bits(),
            "{name}: certified volume bit-identical across the adoption pass"
        );
        let export2 = step_export::step_string(&body2, &options)
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
