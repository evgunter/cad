//! Acceptance rows 1–3 (M7-1 spec §2): the committed-corpus row, the
//! fixed-point row, and — by construction throughout — the comparison
//! discipline (counts, certified scalars, structural invariants only;
//! see `common`'s module docs).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{SOLID_FIXTURES, arena_census, at_rest, expect_sidecar, fixture, import_body};
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
        arena_census(&body),
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
    //   * the rounding of each side's sum: each side's volume is a
    //     fixed-order sum of per-face terms, each face read about a
    //     vertex of its shell (`topo::props`' `shell_reference`, the
    //     start of the shell's first quadrature face). Import traversal need not
    //     match construction order, so the two sides may sum in
    //     different orders and read a face about different vertices:
    //     their terms are not the same values, only the same sum in the
    //     reals. Either side's `Σ|tᵢ|` is at most the body's surface
    //     area times its diameter (a face's flux about a point of its
    //     shell is at most its area times its lever), so each side's
    //     fold is bounded by `topo::test_support::fold_order_bound` at
    //     that `Σ|tᵢ|`, and the budget counts it once per side. A stated
    //     model, like the bound itself: it scales with `Σ|tᵢ|`, not
    //     `|V|`, since the terms are signed.
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
    let (props, terms) = topo::test_support::mass_properties_with_face_terms(
        &body,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap_or_else(|e| panic!("{name}: {e}"));
    let expected_m3 = expect.kernel_volume_mm3 * 1e-9;
    let native_pad_m3 = expect.kernel_volume_pad_mm3 * 1e-9;
    let ulp = expected_m3.next_up() - expected_m3;
    let (lo, hi) = body.vertex_points().fold(
        ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]),
        |(lo, hi), (_, p)| {
            let c = [p.x, p.y, p.z];
            (
                [0, 1, 2].map(|i| lo[i].min(c[i])),
                [0, 1, 2].map(|i| hi[i].max(c[i])),
            )
        },
    );
    let diameter = (0..3).map(|i| (hi[i] - lo[i]).powi(2)).sum::<f64>().sqrt();
    let reach = props.enclosure.surface_area * diameter;
    let own: f64 = terms.iter().map(|&(_, flux, _)| flux.abs()).sum();
    let fold = topo::test_support::fold_order_bound(&terms).0 * (reach / own).max(1.0);
    let ulps = 2.0 * fold + 2.0 * ulp;
    let tolerance = props.enclosure.volume_pad + native_pad_m3 + ulps;
    assert!(
        (props.enclosure.volume - expected_m3).abs() <= tolerance,
        "{name}: volume {} m³ vs native {} m³ (tolerance {}: pad {} + native pad {} + ulps {})",
        props.enclosure.volume,
        expected_m3,
        tolerance,
        props.enclosure.volume_pad,
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
        let export1 = step_export::step_string(&at_rest(&body1), &options, Tol::witness())
            .unwrap_or_else(|e| panic!("{name}: re-export 1: {e}"));
        // STEP has no native contact concept, so export drops the kiss
        // record and the re-import's gate records the touch again.
        let reimport = step_import::import_step(
            &export1,
            &step_import::ImportOptions::default(),
            Tol::witness(),
        )
        .unwrap_or_else(|e| panic!("{name}: re-import: {e}"));
        let line_promotions = reimport
            .curve_promotions()
            .iter()
            .filter(|p| p.kind == step_import::PromotedCurveKind::Line)
            .count();
        // ANCHORED, so the branch below cannot float: the re-import's
        // straight-seam promotion is a deterministic function of the
        // fixture and the ambient band (our export header states the
        // ambient ε as ε_in; loft_prism's two straight seams carry
        // the measured map fold 1.2644054553268222e-7, every other
        // fixture none). A wrong promotion count must red HERE, not
        // pick the branch that matches it.
        let expected_line_promotions =
            if name == "loft_prism" && Tol::witness().get().eps >= 1.264_405_455_326_822_2e-7 {
                2
            } else {
                0
            };
        assert_eq!(
            line_promotions, expected_line_promotions,
            "{name}: the re-import's Line promotions are the fixture's own straight seams"
        );
        let step_import::StepImport::Solid { body: body2, .. } = reimport else {
            panic!("{name}: re-import lost the solid");
        };
        assert_eq!(
            arena_census(&body1),
            arena_census(body2.body()),
            "{name}: census identical across the adoption pass"
        );
        let v1 = topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished("the measured body", body1.clone(), Tol::witness()),
            Tol::witness(),
        )
        .volume;
        let v2 = topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished(
                "the measured body",
                body2.body().clone(),
                Tol::witness(),
            ),
            Tol::witness(),
        )
        .volume;
        assert_eq!(
            v1.to_bits(),
            v2.to_bits(),
            "{name}: certified volume bit-identical across the adoption pass"
        );
        let export2 = step_export::step_string(&at_rest(body2.body()), &options, Tol::witness())
            .unwrap_or_else(|e| panic!("{name}: re-export 2: {e}"));
        if line_promotions == 0 {
            assert_eq!(
                export1, export2,
                "{name}: the second export must be byte-identical to the first"
            );
        } else {
            // The one-cycle byte pin moves EXACTLY one cycle out when
            // the ambient band is coarser than the committed fixture's
            // declared 1e-9: our re-export header states the ambient ε,
            // so the re-import reads a coarser ε_in than the first and
            // D7 promotes straight seam carriers the first left NURBS
            // (each divergent record a reported promotion, re-stated as
            // LINE). The SECOND cycle is the fixed point.
            let count = |s: &str, pat: &str| s.matches(pat).count();
            assert_eq!(
                count(&export2, "= LINE("),
                count(&export1, "= LINE(") + line_promotions,
                "{name}: the divergence is exactly the promoted carriers, as LINE"
            );
            assert_eq!(
                count(&export1, "B_SPLINE_CURVE_WITH_KNOTS("),
                count(&export2, "B_SPLINE_CURVE_WITH_KNOTS(") + line_promotions,
                "{name}: each promoted carrier retires one spline record"
            );
            let step_import::StepImport::Solid { body: body3, .. } = step_import::import_step(
                &export2,
                &step_import::ImportOptions::default(),
                Tol::witness(),
            )
            .unwrap_or_else(|e| panic!("{name}: re-import 2: {e}")) else {
                panic!("{name}: re-import 2 lost the solid");
            };
            let export3 =
                step_export::step_string(&at_rest(body3.body()), &options, Tol::witness())
                    .unwrap_or_else(|e| panic!("{name}: re-export 3: {e}"));
            assert_eq!(
                export2, export3,
                "{name}: fixed point from the second cycle on"
            );
        }

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
