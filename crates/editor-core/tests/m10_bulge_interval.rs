//! **What stands at a bulge that is not 1 — the nominal pins.** The
//! form-level mechanism's reach is the UNIT bulge (`sym.rs`'s "The
//! form-level algebra" section; `work/sym/rule-d-reaches-the-unit-bulge-only`):
//! every document M10-10 measured authors its arcs through
//! `LoopProgram::Circle`/`CircleSplit(2)`, kernel bulge `1`. These rows
//! hold the per-predicate split AT THE NOMINAL, under the shipped set,
//! of the three documents that measure the limit — R1's
//! circular-segment boss (a literal `bulge = 2`, `segment_boss`), R2's
//! D-tab with its bulge a literal `0.4` and the same D-tab with the
//! bulge a document parameter (`d_tab`) — so that the next unit's claim
//! "this rule takes the boss's 6 and 27" has a before-number a test
//! holds. What each number stands on is diagnosed on the item, from
//! the renders `m10_10_evidence_interval` makes of these documents
//! (`CAD_M10_10_DOC=r1_segment_boss|r2_d_tab_literal|r2_d_tab_parameter`,
//! and the dyadic controls `r2_d_tab_literal_dyadic|r2_d_tab_parameter_dyadic`);
//! `m10_bulge_renders.txt` beside this file is the trimmed record of
//! those renders.
//!
//! The split is the same at ε = 1e-6, 1e-9 and 1e-12 (the atoms a
//! residual carries do not depend on the band), so the rows assert
//! one table at every row of the matrix. Cost: three nominal replays,
//! about a second each for the boss and the literal D-tab and three
//! for the parameter D-tab in a dev build.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use geom_core::sym::report::ShapeOutcome;
use geom_core::{SymRules, Tol};

use crate::m10_8_arc_family_interval::replay;
use crate::m10_8_harness::nominal_box;

/// The per-predicate split of one nominal replay under the shipped
/// set: `predicate -> [theorem, gated, registered, numeric]`.
fn split_at_the_nominal(doc: &ProfileDoc, tol: Tol) -> BTreeMap<&'static str, [u64; 4]> {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let (shapes, _, _) = replay(doc, &nominal_box(&analyzed), SymRules::shipped(), tol);
    let mut table: BTreeMap<&'static str, [u64; 4]> = BTreeMap::new();
    for s in &shapes {
        let row = table.entry(s.predicate).or_default();
        row[match s.outcome {
            ShapeOutcome::Theorem => 0,
            ShapeOutcome::SignGated => 1,
            ShapeOutcome::Registered => 2,
            _ => 3,
        }] += 1;
    }
    table
}

/// Asserts each named predicate's split, naming the document and the
/// predicate in the failure.
fn assert_split(
    name: &str,
    table: &BTreeMap<&'static str, [u64; 4]>,
    expected: &[(&str, [u64; 4])],
) {
    for (pred, want) in expected {
        let got = table
            .get(pred)
            .copied()
            .unwrap_or_else(|| panic!("{name}: no {pred} decisions"));
        assert_eq!(
            got, *want,
            "{name}: {pred} moved at the nominal (theorem/gated/registered/numeric) — \
             the bulge family's before-number; if a rule took it, re-pin and say which"
        );
    }
}

/// **The boss at `bulge = 2`.** What stands: `carrier_matches_mapped_source`
/// 6 of 54 numeric — the samples at `q = 3/2, 5/2, 7/2` of both rims,
/// where the odd half-multiples' closed forms over the chord's 50-bit
/// mantissa freeze at the coefficient ring's width (a 512-bit ring
/// takes all six and moves the ceiling `8.26e2 → 9.36e2·ε`, onto
/// `line_span`); `carrier_on_surface_2` 27 of 90, `witness_on_surface_2`
/// 3 of 10, `carrier_on_surface_1` 9 of 90 and `witness_on_surface_1`
/// 1 of 10 — the arc carrier's radius `abs(signed_radius)` (the
/// profile's `seg.rs`), `abs((5/8)·sqrt(L²))` with `L` the chord, a
/// non-constant argument A0 does not fold, standing squared against
/// its own square spelled without the `abs`; and behind the `abs`, the
/// ring again (folding it takes 20 and uncovers 20 freezes). The chart
/// phase is the door's, as on the plate.
#[test]
fn m10_bulge_the_bosss_split_at_the_nominal() {
    let tol = Tol::witness();
    let (doc, _, _) = crate::m10_10_r1_probes_interval::segment_boss(1.0, tol);
    let table = split_at_the_nominal(&doc, tol);
    assert_split(
        "segment_boss",
        &table,
        &[
            ("carrier_matches_mapped_source", [72, 0, 48, 6]),
            ("carrier_on_surface_2", [63, 0, 0, 27]),
            ("witness_on_surface_2", [7, 0, 0, 3]),
            ("carrier_on_surface_1", [81, 0, 0, 9]),
            ("witness_on_surface_1", [9, 0, 0, 1]),
            ("pcurve_map_residual", [0, 0, 18, 0]),
        ],
    );
}

/// **The D-tab at a literal `bulge = 0.4`.** What stands is the
/// coefficient ring alone: `fl(0.4) = 3602879701896397·2^-53` puts an
/// odd 52-bit denominator under every sagitta coefficient, the radius
/// `L(1+b²)/(4b)` folds (A0) to a constant with a 156-bit numerator,
/// and its square — the on-surface residual's `R²` — is past the 256-bit
/// ring; the `carrier_matches_mapped_source` samples freeze on the
/// chord's `L⁴` (200 bits) times one more mantissa. At `bulge = 0.5`
/// (the dyadic control) `carrier_on_surface_2` is 18 numeric, not 27,
/// and the rest is unchanged; at 512 bits the literal's 27 is 18 and
/// its `carrier_on_surface_1` 0. The shipped ceiling of this document
/// is NOT the residue: 0.56 of its real study, bounded by
/// `arc_diameter_clearance` (the annulus's real-margin class).
#[test]
fn m10_bulge_the_d_tabs_literal_split_at_the_nominal() {
    let tol = Tol::witness();
    let (doc, _, _) = crate::m10_10_r2_probes_interval::d_tab(1.0, false, tol);
    let table = split_at_the_nominal(&doc, tol);
    assert_split(
        "d_tab (bulge a literal)",
        &table,
        &[
            ("carrier_matches_mapped_source", [126, 0, 36, 18]),
            ("carrier_on_surface_2", [117, 0, 0, 27]),
            ("witness_on_surface_2", [13, 0, 0, 3]),
            ("carrier_on_surface_1", [135, 0, 0, 9]),
            ("witness_on_surface_1", [15, 0, 0, 1]),
            ("pcurve_map_residual", [0, 0, 18, 0]),
        ],
    );
}

/// **The D-tab with its bulge a document parameter.** The split is
/// the literal's, number for number: at the nominal the bulge's SIGN
/// is not what blocks — the coefficient ring is, on the same nodes. The
/// sign enters where the dyadic control shows it once the ring is out
/// of the way: the carrier's span `4·atan|b|` (`sweep`'s `arc_span`)
/// mints `abs(b)` and `sqrt(1 + abs(b)²)` where the pushforward's
/// `4·atan b` mints `sqrt(1 + b²)` — two atoms for one quantity,
/// related only through the sign of `b` — and the radius
/// `abs(L(1+b²)/(4b))` carries it too. The ceiling `3.52e2·ε`
/// (`carrier_matches_mapped_source`, algebra on and off alike) is
/// bounded by a decision that carries both the two atoms and the
/// freezes.
#[test]
fn m10_bulge_the_d_tabs_parameter_split_at_the_nominal() {
    let tol = Tol::witness();
    let (doc, _, _) = crate::m10_10_r2_probes_interval::d_tab(1.0, true, tol);
    let table = split_at_the_nominal(&doc, tol);
    assert_split(
        "d_tab (bulge a parameter)",
        &table,
        &[
            ("carrier_matches_mapped_source", [126, 0, 36, 18]),
            ("carrier_on_surface_2", [117, 0, 0, 27]),
            ("witness_on_surface_2", [13, 0, 0, 3]),
            ("carrier_on_surface_1", [135, 0, 0, 9]),
            ("witness_on_surface_1", [15, 0, 0, 1]),
            ("pcurve_map_residual", [0, 0, 18, 0]),
        ],
    );
}
