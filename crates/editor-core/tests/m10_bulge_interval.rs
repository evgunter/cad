//! **What stands at a bulge that is not 1 — the nominal pins.** The
//! form-level mechanism's reach is the UNIT bulge (`sym.rs`'s "The
//! form-level algebra" section; `work/sym/rule-d-reaches-the-unit-bulge-only`):
//! every document M10-10 measured authors its arcs through
//! `LoopProgram::Circle`/`CircleSplit(2)`, kernel bulge `1`. These rows
//! hold the per-predicate split AT THE NOMINAL, under the shipped set
//! and WHOLE (`m10_8_harness::assert_split`: every predicate that
//! decided, at its counts, and a predicate outside the table reds
//! naming it), of the three documents that measure the limit — R1's
//! circular-segment boss (a literal `bulge = 2`, `segment_boss`), R2's
//! D-tab with its bulge a literal `0.4` and the same D-tab with the
//! bulge a document parameter (`d_tab`) — so that the next unit's claim
//! "this rule takes the boss's 6 and 27" has a before-number a test
//! holds. What each number stands on is diagnosed on the item, from
//! the renders `m10_10_evidence_interval` makes of these documents
//! (`CAD_M10_10_DOC=r1_segment_boss|r2_d_tab_literal|r2_d_tab_parameter`,
//! and the dyadic controls `r2_d_tab_literal_dyadic|r2_d_tab_parameter_dyadic`);
//! `m10_bulge_renders.txt` beside this file is the trimmed record of
//! those renders, with the boss's freeze cause uncut.
//!
//! The split is the same at ε = 1e-6, 1e-9 and 1e-12 (the atoms a
//! residual carries do not depend on the band), so the rows assert
//! one table at every row of the matrix. Cost: three nominal replays,
//! about a second each for the boss and the literal D-tab and three
//! for the parameter D-tab in a dev build.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{SymRules, Tol};

use crate::m10_8_harness::{assert_split, split_at_the_nominal};

/// **The D-tab's whole table at the nominal**, the literal's and the
/// parameter's alike on every row but one — asserted on both, so the
/// equality is a pinned fact and not a copy, and the one row that
/// PARTS is supplied per document by [`d_tab_table`]. What stands:
/// `carrier_matches_mapped_source` 12 of 180 on the literal and 16 on
/// the parameter (it was 18 on both until SYM-5's rule E).
/// `carrier_endpoint_start`'s last 4 of 36 went with SYM-5's
/// rule E (the quotient's common factor): they are 24/0/12/0 now, the
/// rim identity `‖q − c‖ = r` through the DOOR at every sample — the
/// rule cancels the shared factor in the rim's re-normalised quotient,
/// so the residual the registrant states about reaches the form
/// instead of freezing. No theorem moved; four NUMERIC decisions
/// became registered ones. On the
/// coefficient ring — `fl(0.4) = 3602879701896397·2^-53` puts an odd
/// 52-bit denominator under every sagitta coefficient, the radius
/// `L(1+b²)/(4b)` folds (A0) to a constant with a 156-bit numerator on
/// the literal, and its square — the on-surface residual's `R²` — is
/// past the 256-bit ring; `carrier_on_surface_2` 27 of 144,
/// `carrier_on_surface_1` 9, `witness_on_surface_2` 3 and
/// `witness_on_surface_1` 1 on the same ring and, on the parameter,
/// the radius's `abs(R(b))`. At the dyadic control `bulge = 0.5`
/// (evidence only) the ring is out of the way on the on-surface
/// residuals — `carrier_on_surface_2` 27 → 18, `carrier_on_surface_1`
/// 9 → 0, `witness_on_surface_2` 3 → 2, `witness_on_surface_1` 1 → 0,
/// `carrier_endpoint_start` 4 → 0 — and `carrier_matches_mapped_source`
/// is 16 on the parameter control, where the sign of `b` stands in
/// the open. At 512 bits the literal's `carrier_on_surface_2` is 18
/// and its `carrier_on_surface_1` 0; `carrier_endpoint_start` stays
/// 24/0/8/4. The literal's shipped CEILING is not the residue: 0.56 of
/// its real study, bounded by `arc_diameter_clearance` (the annulus's
/// real-margin class); the parameter's is `3.52e2·ε`, on and off
/// alike.
const D_TAB_AT_THE_NOMINAL: &[(&str, [u64; 4])] = &[
    ("arc_apex_identity", [0, 0, 0, 1]),
    ("arc_diameter_clearance", [0, 0, 0, 6]),
    ("arc_span", [4, 0, 0, 4]),
    ("assert_bound", [0, 0, 0, 1]),
    ("carrier_circles_identity", [3, 0, 0, 0]),
    ("carrier_cyl_axis_parallel", [1, 0, 0, 0]),
    ("carrier_endpoint_end", [24, 0, 12, 0]),
    ("carrier_endpoint_start", [24, 0, 12, 0]),
    ("carrier_line_circle", [0, 0, 0, 5]),
    ("carrier_on_surface_1", [135, 0, 0, 9]),
    ("carrier_on_surface_2", [117, 0, 0, 27]),
    ("chord_side", [4, 0, 0, 10]),
    ("contact_at_shared_vertex", [8, 0, 0, 4]),
    ("datum_unit_norm", [0, 0, 0, 2]),
    ("dihedral_arm", [0, 0, 0, 128]),
    ("dihedral_wedge", [0, 0, 0, 128]),
    ("expr_non_finite", [37, 0, 0, 0]),
    ("extrusion_normal_component", [0, 0, 0, 2]),
    ("interval_span_forward", [0, 0, 0, 36]),
    ("interval_span_winding", [0, 0, 0, 12]),
    ("line_span", [4, 0, 0, 4]),
    ("newell_plane_residual", [30, 0, 0, 0]),
    ("path_circle_radius", [0, 0, 0, 1]),
    ("path_junction_turn", [0, 0, 0, 4]),
    ("pcurve_chart_azimuth_frame", [0, 0, 0, 2]),
    ("pcurve_chart_radial_moving", [2, 0, 0, 0]),
    ("pcurve_map_residual", [0, 0, 18, 0]),
    ("segment_straightness", [6, 0, 0, 6]),
    ("side_cylinders_cosurface", [2, 0, 0, 0]),
    ("side_planes_cosurface", [0, 0, 0, 2]),
    ("vertex_separation", [0, 0, 0, 12]),
    ("witness_at_mid_parameter", [16, 0, 0, 0]),
    ("witness_on_surface_1", [15, 0, 0, 1]),
    ("witness_on_surface_2", [13, 0, 0, 3]),
];

/// **The boss at `bulge = 2`, whole.** `carrier_matches_mapped_source`
/// is 72/0/54/0 since SYM-5's rule E: the last 6 of 54 numeric — the
/// samples at `q = 3/2, 5/2, 7/2` of both rims, whose odd
/// half-multiples' closed forms over the chord's 50-bit mantissa froze
/// at the coefficient ring's width — are through the DOOR once the
/// shared factor in the quotient is cancelled and the coefficients
/// stay inside the ring (a 512-bit ring used to be what took them, and
/// moved the ceiling `8.26e2 → 9.36e2·ε` onto `line_span`). No theorem
/// moved; eighteen `carrier_on_surface_2` decisions and two
/// `witness_on_surface_2` became THEOREMS with it. What stands:
/// `carrier_on_surface_2` 9 of 90 (it was 27), `witness_on_surface_2`
/// 1 of 10 (it was 3), `carrier_on_surface_1` 9 of 90 and
/// `witness_on_surface_1` 1 of 10 — the arc carrier's radius `abs(signed_radius)` (the
/// profile's `seg.rs`), `abs((5/8)·sqrt(L²))` with `L` the chord, a
/// non-constant argument A0 does not fold, standing squared against
/// its own square spelled without the `abs`; and behind the `abs`, the
/// ring again (folding it takes 20 and uncovers 20 freezes). The rim
/// identity is the door's here (`carrier_endpoint_start` 12/0/12/0),
/// as is the chart phase, as on the plate.
///
/// **DECIDE-3 takes one more of `arc_span`**, 4/0/0/2 -> 5/0/0/1, and
/// **DECIDE-3 takes the `abs` wall the paragraph above describes.**
/// The canonical root spells `sqrt(L²)` as `|L|`, which is the atom
/// the carrier's own `abs` mints, so the radius `abs(signed_radius)`
/// and the square it used to stand against meet on ONE indeterminate
/// instead of two: `carrier_on_surface_1` and `carrier_on_surface_2`
/// go 81/0/0/9 -> 90/0/0/0 each, `witness_on_surface_1` and
/// `witness_on_surface_2` 9/0/0/1 -> 10/0/0/0, `arc_span` 4/0/0/2 ->
/// 5/0/0/1 and `contact_at_shared_vertex` 4/0/0/5 -> 6/0/0/3 — every
/// one of them a THEOREM, none of them a read. `line_span` gains four
/// THEOREMS, `[0, 0, 0, 8] -> [4, 0, 0, 4]`: four of its eight
/// comparisons are between two rational CONSTANTS, and A0 now decides
/// those exactly — the fix
/// `work/decide/a0-leaves-max-and-min-of-constants-opaque` asked for.
/// The other four carry a parameter and no form settles them.
#[test]
fn m10_bulge_the_bosss_split_at_the_nominal() {
    let tol = Tol::witness();
    let (doc, _, _) = crate::m10_10_r1_probes_interval::segment_boss(1.0, tol);
    assert_split(
        "segment_boss",
        &split_at_the_nominal(&doc, SymRules::shipped(), tol),
        &[
            ("arc_apex_identity", [0, 0, 0, 1]),
            ("arc_diameter_clearance", [0, 0, 0, 6]),
            ("arc_span", [5, 0, 0, 1]),
            ("assert_bound", [0, 0, 0, 1]),
            ("carrier_circles_identity", [3, 0, 0, 0]),
            ("carrier_cyl_axis_parallel", [1, 0, 0, 0]),
            ("carrier_endpoint_end", [12, 0, 12, 0]),
            ("carrier_endpoint_start", [12, 0, 12, 0]),
            ("carrier_line_circle", [0, 0, 0, 3]),
            ("carrier_matches_mapped_source", [72, 0, 54, 0]),
            ("carrier_on_surface_1", [90, 0, 0, 0]),
            ("carrier_on_surface_2", [90, 0, 0, 0]),
            ("contact_at_shared_vertex", [6, 0, 0, 3]),
            ("datum_unit_norm", [0, 0, 0, 2]),
            ("dihedral_arm", [0, 0, 0, 80]),
            ("dihedral_wedge", [0, 0, 0, 80]),
            ("expr_non_finite", [29, 0, 0, 0]),
            ("extrusion_normal_component", [0, 0, 0, 2]),
            ("interval_span_forward", [0, 0, 0, 24]),
            ("interval_span_winding", [0, 0, 0, 12]),
            ("line_span", [0, 2, 0, 0]),
            ("newell_plane_residual", [18, 0, 0, 0]),
            ("path_circle_radius", [0, 0, 0, 1]),
            ("path_junction_turn", [0, 0, 0, 2]),
            ("pcurve_chart_azimuth_frame", [0, 0, 0, 2]),
            ("pcurve_chart_radial_moving", [2, 0, 0, 0]),
            ("pcurve_map_residual", [0, 0, 18, 0]),
            ("segment_straightness", [2, 0, 0, 6]),
            ("side_cylinders_cosurface", [2, 0, 0, 0]),
            ("vertex_separation", [0, 0, 0, 8]),
            ("witness_at_mid_parameter", [10, 0, 0, 0]),
            ("witness_on_surface_1", [10, 0, 0, 0]),
            ("witness_on_surface_2", [10, 0, 0, 0]),
        ],
    );
}

/// The whole D-tab table for one of the two spellings:
/// [`D_TAB_AT_THE_NOMINAL`] plus the one row where the literal and the
/// parameter PART.
fn d_tab_table(mapped_source: [u64; 4]) -> Vec<(&'static str, [u64; 4])> {
    let mut out = D_TAB_AT_THE_NOMINAL.to_vec();
    out.push(("carrier_matches_mapped_source", mapped_source));
    out
}

/// The D-tab with its bulge a literal `0.4`: [`D_TAB_AT_THE_NOMINAL`],
/// with `carrier_matches_mapped_source` 126/0/42/12 — six of the
/// eighteen that used to stand went to the DOOR with SYM-5's rule E.
#[test]
fn m10_bulge_the_d_tabs_literal_split_at_the_nominal() {
    let tol = Tol::witness();
    let (doc, _, _) = crate::m10_10_r2_probes_interval::d_tab(1.0, false, tol);
    assert_split(
        "d_tab (bulge a literal)",
        &split_at_the_nominal(&doc, SymRules::shipped(), tol),
        &d_tab_table([126, 0, 42, 12]),
    );
}

/// The D-tab with its bulge a document parameter: the same table as
/// the literal's ([`D_TAB_AT_THE_NOMINAL`]) except on ONE row. At the
/// nominal the bulge's sign is not what blocks on any of the shared
/// rows, the ring is, on the same nodes — but with SYM-5's rule E the
/// two part on `carrier_matches_mapped_source`: 126/0/42/12 on the
/// literal against 126/0/38/16 here, so four of the six the rule takes
/// on the literal it does not take when the bulge is a parameter. The
/// sign
/// enters where the dyadic control shows it once the ring is out of
/// the way: the carrier's span `4·atan|b|` (`sweep`'s `arc_span`)
/// mints `abs(b)` and `sqrt(1 + abs(b)²)` where the pushforward's
/// `4·atan b` mints `sqrt(1 + b²)` — two atoms for one quantity,
/// related only through the sign of `b` — and the radius
/// `abs(L(1+b²)/(4b))` carries it too.
#[test]
fn m10_bulge_the_d_tabs_parameter_split_at_the_nominal() {
    let tol = Tol::witness();
    let (doc, _, _) = crate::m10_10_r2_probes_interval::d_tab(1.0, true, tol);
    assert_split(
        "d_tab (bulge a parameter)",
        &split_at_the_nominal(&doc, SymRules::shipped(), tol),
        &d_tab_table([126, 0, 38, 16]),
    );
}
