//! **R2 independent review probes (PR #264, dual-review protocol)** —
//! probes branch only, NOT part of the PR. End-to-end falsification of
//! the promotion plumbing and the byte-identity claims through the
//! public import/export API, written blind to R1's findings.

mod common;

use common::{census, fixture};
use step_import::{ImportOptions, NormalizationKind, PromotedKind, StepImport};

fn import(text: &str) -> Result<StepImport, step_import::StepImportError> {
    step_import::import_step(text, &ImportOptions::default())
}

fn promotions(import: &StepImport) -> Vec<(u64, PromotedKind, f64)> {
    let StepImport::Solid { normalizations, .. } = import else {
        panic!("expected a solid");
    };
    normalizations
        .iter()
        .filter_map(|n| match n.kind {
            NormalizationKind::SurfacePromotion { to, residual } => Some((n.face, to, residual)),
            _ => None,
        })
        .collect()
}

/// **T1 — the near-plane boundary, end-to-end.** loft_prism's wall
/// surface #87 is exactly planar (y = −1); its control #85 perturbed
/// 0.5×ε_in must still promote #104 (with the honest residual
/// recorded ≤ ε_in), and at 2×ε_in the promotion must vanish — with
/// the file either importing (stays-NURBS is the M7-3-legal state) or
/// refusing TYPED, never promoting past the budget.
#[test]
fn r2_t1_near_plane_boundary_end_to_end() {
    let orig = fixture("loft_prism", "step");
    let target = "#85 = CARTESIAN_POINT('', (1.75, -1.0, 1.0));";
    assert!(orig.contains(target), "the perturbation target exists");

    let half = orig.replace(target, "#85 = CARTESIAN_POINT('', (1.75, -1.0000000005, 1.0));");
    let imported = import(&half).expect("0.5x eps perturbation imports");
    let promos = promotions(&imported);
    let f104 = promos.iter().find(|(f, _, _)| *f == 104);
    let Some((_, kind, residual)) = f104 else {
        panic!("0.5x eps: face #104 must still promote; got {promos:?}");
    };
    assert_eq!(*kind, PromotedKind::Plane);
    assert!(
        *residual > 0.0 && *residual <= 1e-9,
        "recorded residual is honest and inside eps: {residual:e}"
    );

    let twice = orig.replace(target, "#85 = CARTESIAN_POINT('', (1.75, -1.000000002, 1.0));");
    match import(&twice) {
        Ok(ref imported) => {
            let promos = promotions(imported);
            assert!(
                !promos.iter().any(|(f, _, _)| *f == 104),
                "2x eps: face #104 must NOT promote; got {promos:?}"
            );
            println!("2x eps imported without the promotion: {promos:?}");
        }
        Err(e) => println!("2x eps refused typed (never promoted past budget): {e}"),
    }
}

/// **T2 — the promoted one-cycle byte fixed point, re-executed
/// independently for all three promoting fixtures**: import → export
/// → import → export, second export byte-identical to the first;
/// censuses and volumes bit-stable across the cycle; the FIRST import
/// carries exactly the two claimed plane promotions.
#[test]
fn r2_t2_promoted_one_cycle_byte_fixed_point() {
    for (name, faces) in [
        ("loft_prism", [104u64, 142]),
        ("nonuniform_loft", [104, 142]),
        ("swept_elbow", [165, 228]),
    ] {
        let text = fixture(name, "step");
        let first = import(&text).unwrap_or_else(|e| panic!("{name}: import: {e}"));
        let promos = promotions(&first);
        let got: Vec<u64> = promos.iter().map(|(f, _, _)| *f).collect();
        assert_eq!(got, faces, "{name}: the claimed promotions, exactly");
        for (f, kind, residual) in &promos {
            assert_eq!(*kind, PromotedKind::Plane, "{name} #{f}");
            assert!(*residual <= 1e-15, "{name} #{f}: residual {residual:e}");
        }
        let StepImport::Solid { body: b1, .. } = first else {
            panic!("{name}: solid");
        };
        let options = step_export::StepOptions {
            product_name: name.to_owned(),
            ..step_export::StepOptions::default()
        };
        let e1 = step_export::step_string(&b1, &options).unwrap();
        let second = import(&e1).unwrap_or_else(|e| panic!("{name}: re-import: {e}"));
        let StepImport::Solid { body: b2, .. } = second else {
            panic!("{name}: solid 2");
        };
        let e2 = step_export::step_string(&b2, &options).unwrap();
        assert_eq!(e1, e2, "{name}: one-cycle export is a byte fixed point");
        assert_eq!(census(&b1), census(&b2), "{name}: census across the cycle");
        let (v1, v2) = (
            topo::mass_properties(&b1).unwrap().volume,
            topo::mass_properties(&b2).unwrap().volume,
        );
        assert_eq!(v1.to_bits(), v2.to_bits(), "{name}: volume bits across the cycle");
    }
}

/// **T3 — D9 end-to-end**: the same bytes imported twice export the
/// same bytes, promotions included.
#[test]
fn r2_t3_same_input_twice_is_bit_identical() {
    let text = fixture("loft_prism", "step");
    let run = || {
        let StepImport::Solid { body, .. } = import(&text).unwrap() else {
            panic!("solid");
        };
        step_export::step_string(&body, &step_export::StepOptions::default()).unwrap()
    };
    assert_eq!(run(), run(), "two imports of the same bytes, same export bytes");
}
