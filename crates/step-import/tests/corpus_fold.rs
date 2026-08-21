//! **The #210 corpus fold's acceptance row, RE-STATED at M7-6 per the
//! #256 ruling.**
//!
//! The original row asserted first-re-export byte alignment against
//! the committed bytes for `nonuniform_loft` and `swept_elbow`, with
//! two named token-divergence classes (`-0.0` hygiene, the
//! uncertainty record off-ε). Since D7 stage-1 always-promote, each
//! fixture's two exactly-planar spline walls (plane residuals ~1e-16,
//! far inside ε_in = 1e-9) import as PLANES, and the first re-export
//! states them analytically — a STRUCTURAL divergence the ruling
//! ACCEPTS and re-pins:
//!
//! * **enumerated promotions**: exactly which faces promoted, to
//!   which kind, with their certified residuals — asserted here, so
//!   the class cannot quietly grow or drift;
//! * **the promoted one-cycle fixed point**: the first re-export
//!   re-imports and re-exports byte-identically; and
//! * **censuses and certified volumes equal across the promotion** —
//!   `roundtrip.rs` executes both halves corpus-wide
//!   (`committed_corpus`: census + volume against the NATIVE
//!   `KERNEL_*` sidecar; `fixed_point`: census equality and
//!   bit-identical volume across the re-export cycle), so this suite
//!   pins the divergence SHAPE: the surface-record census moves by
//!   exactly the promotion (2 planes → 4, 4 spline walls → 2) and
//!   nothing else in the vocabulary.
//!
//! Byte-identity stays the pin for every fixture promotion does not
//! touch (`roundtrip.rs` reports it; the untouched corpus still
//! round-trips byte-golden — see `fixed_point`'s measured rows).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{fixture, import_body};
use step_import::{NormalizationKind, PromotedKind};
use geom_core::Tol;

/// One fold fixture's promotion pin: the fixture name, its two
/// promoted faces with their kinds, and the residual ceiling both
/// certify under.
type FixturePromotionRow = (&'static str, [(u64, PromotedKind); 2], f64);

/// The fold fixtures and their measured promotions: (face, kind) with
/// the residual ceiling. The residuals are the walls' true distance
/// from their planes as the loft builder left them (~1e-16 m — exact
/// up to f64 rounding of the section arithmetic), pinned by ceiling
/// rather than by bits so an unrelated last-bit wiggle in the builder
/// does not break the corpus row.
const FOLD_PROMOTIONS: [FixturePromotionRow; 2] = [
    (
        "nonuniform_loft",
        [(104, PromotedKind::Plane), (142, PromotedKind::Plane)],
        1e-15,
    ),
    (
        "swept_elbow",
        [(165, PromotedKind::Plane), (228, PromotedKind::Plane)],
        1e-15,
    ),
];

#[test]
fn the_folds_divergence_is_exactly_the_reported_promotion() {
    for (name, want, residual_ceiling) in FOLD_PROMOTIONS {
        let committed = fixture(name, "step");
        let (body, _eps) = import_body(name);
        let normalizations = {
            // Re-import through the public door to read the report.
            let step_import::StepImport::Solid { normalizations, .. } =
                step_import::import_step(&committed, &step_import::ImportOptions::default())
                    .expect("the fold fixture imports")
            else {
                panic!("{name}: must import as a solid");
            };
            normalizations
        };
        let promos: Vec<(u64, PromotedKind)> = normalizations
            .iter()
            .filter_map(|n| match n.kind {
                NormalizationKind::SurfacePromotion { to, residual } => {
                    assert_eq!(
                        n.file_census, n.kernel_census,
                        "{name}: promotion never re-tessellates"
                    );
                    assert!(
                        residual.is_finite() && residual <= residual_ceiling,
                        "{name} face #{}: promotion residual {residual:e} above the \
                         measured ceiling {residual_ceiling:e}",
                        n.face
                    );
                    Some((n.face, to))
                }
                _ => None,
            })
            .collect();
        assert_eq!(promos, want, "{name}: the enumerated promotions");

        let options = step_export::StepOptions {
            product_name: name.to_owned(),
            ..step_export::StepOptions::default()
        };
        let out = step_export::step_string(&body, &options, Tol::witness()).expect("re-export");
        let count = |s: &str, pat: &str| s.matches(pat).count();
        assert_eq!(
            (
                count(&committed, "= PLANE("),
                count(&committed, "B_SPLINE_SURFACE_WITH_KNOTS(")
            ),
            (2, 4),
            "{name}: committed vocabulary — 2 cap planes + 4 spline walls"
        );
        assert_eq!(
            (
                count(&out, "= PLANE("),
                count(&out, "B_SPLINE_SURFACE_WITH_KNOTS(")
            ),
            (4, 2),
            "{name}: the re-export's divergence is the two promoted walls"
        );

        // The promoted one-cycle fixed point (the ruling's re-stated
        // byte pin): from the first re-export on, byte-identical.
        let step_import::StepImport::Solid { body: body2, .. } =
            step_import::import_step(&out, &step_import::ImportOptions::default())
                .expect("the promoted re-export re-imports")
        else {
            panic!("{name}: the re-import must be a solid");
        };
        let out2 = step_export::step_string(&body2, &options, Tol::witness()).expect("second re-export");
        assert_eq!(out, out2, "{name}: fixed point from the first re-export on");
    }
}
