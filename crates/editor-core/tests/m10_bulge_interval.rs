//! **What stands at a bulge that is not 1 — the nominal pins.** The
//! form-level mechanism's reach is the UNIT bulge (`sym.rs`'s "The
//! form-level algebra" section; `work/decide/rule-d-reaches-the-unit-bulge-only`):
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
//! `m10_bulge_renders.txt` beside this file is the record of those
//! renders at the tree that shipped rule G. It is stale: it has not been
//! re-rendered since PCERT B1 renamed the frame row to
//! `pcurve_chart_cut_side`. Per document the whole split, then every
//! decision the tier was asked and left numeric on an arc-family
//! predicate with its freeze causes and its attributed cause — every
//! form attributed to the sign of `b` or of the apothem UNCUT, so the
//! attribution is re-checked by evaluating it, the frozen ones cut —
//! and the boss's former value-free residual, rendered at the base and
//! gone here.
//!
//! The split is the same at ε = 1e-6, 1e-9 and 1e-12 (the atoms a
//! residual carries do not depend on the band), so the rows assert
//! one table at every row of the matrix. Cost: three nominal replays,
//! about a second each for the boss and the literal D-tab and three
//! for the parameter D-tab in a dev build.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{SymRules, Tol};

use crate::m10_8_harness::{assert_split, split_at_the_nominal};

/// **The D-tab's whole table at the nominal**, the literal's and the
/// parameter's alike on every row but one — asserted on both, so the
/// equality is a pinned fact and not a copy, and the one row that
/// PARTS is supplied per document by [`d_tab_table`]. What stands:
/// `carrier_matches_mapped_source` 10 of 180 on the literal and 14 on
/// the parameter (it was 18 on both until SYM-5's rule E; two more
/// reach the door since the certification schedule assigns its last
/// sample `t₁` itself, `geom_brep::schedule_param`).
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
/// residuals — on the parameter control `carrier_on_surface_2` 27 → 4
/// under the drive's ladder (all four frozen on terms and degree; 6
/// with one attempt per rung), `carrier_on_surface_1` 9 → 0,
/// `witness_on_surface_2` 3 → 0, `witness_on_surface_1` 1 → 0,
/// `carrier_endpoint_start` 4 → 0 — and `carrier_matches_mapped_source`
/// is 6 there under the ladder (8 with one attempt per rung), every one
/// frozen. The carrier's span is spelled from the decided turn
/// (`sweep`'s `arc_span`), so the decisions that stood on the sign
/// of `b` in the open, and those whose freeze stood on the bulk of the
/// sign's second atom, are gone (`m10_bulge_renders.txt` attributes
/// what is left). A 512-bit ring
/// (SYM-3's measurement, before rule E) took the literal's
/// `carrier_on_surface_2` to 18 and its `carrier_on_surface_1` to 0.
/// The literal's shipped CEILING is not the residue: 0.56 of
/// its real study, bounded by `arc_diameter_clearance` (the annulus's
/// real-margin class); the parameter's is `3.52e2·ε`, on and off
/// alike. Both are unmeasured with the extrude's closing pcurve mint in
/// the build, whose three `pcurve_*` rows below are its wall rows'
/// certificate; the re-measure is
/// `work/sym/pcurve-certificate-checks-widen-past-the-band-over-a-parameter-box.md`'s.
///
/// **PCERT's incidence-and-fidelity unit (PR 3812) moved seven of the
/// `pcurve_*` rows**, on both spellings. Against the pins it replaced:
///
/// - `pcurve_chart_azimuth_frame` 6/0/0/14 -> 12/0/0/26,
///   `pcurve_chart_orientation` 0/0/0/6 -> 0/0/0/12 and
///   `pcurve_chart_radial_moving` 8/0/0/6 -> 14/0/0/12: check 4
///   re-derives each row's image (on the chart's orthonormal twin,
///   which on these literal frames is the frame itself), so the
///   derivation's decisions are taken twice.
/// - `pcurve_map_residual` 0/0/28/98 -> 0/0/18/0: on a harmonic row the
///   schedule is check 4's cross-check, run on the witness lane and not
///   at this scalar; what is left is the rows of the lanes that keep
///   their schedule.
/// - `pcurve_envelope` 0/0/2/10 -> 8/0/0/4: incidence plus fidelity in
///   place of the image pushed through the chart, and, with the loop
///   walk's branch a literal `k` (`geom_brep::Period::decide`), the
///   stored azimuth is `α + k·τ`; the envelope is a theorem on eight
///   of its twelve rows.
///
/// **PCERT's placement unit (C4: a row is its image placed over the
/// edge's span in the chart's principal window) moved two rows**, on
/// every spelling:
///
/// - `pcurve_chart_azimuth_frame` 12/0/0/26 -> 0/0/0/26: a cylinder's
///   meridian azimuth is read only on a meridian, so a rim circle
///   centred on the axis no longer reads the frame of its zero radial
///   offset (the 12 the tier proved zero).
/// - `pcurve_chart_placement` is new, 0/0/0/52: each of the 26 placed
///   azimuths reads its two half-period marks about the window's
///   centre ([`geom_brep::Period::decide`]), each decided numerically.
///
/// **PCERT unit B2a (a harmonic row's windings are structure, `[ev]`
/// PR 5113's first decision) retired three rows** on every spelling:
/// `pcurve_chart_azimuth_affine` 24/0/0/0, `pcurve_chart_winding`
/// 12/0/0/18 and `pcurve_azimuth_period` 0/0/0/12 decide nothing now.
/// An angle channel stores its phase and its windings, an integer
/// certification compares exactly, so no decision reads a winding off
/// a slope, and the harmonic lane's period gate is gone with the slope
/// it read.
/// - `pcurve_fidelity_branch` is new, 0/0/0/24: fidelity's branch, two
///   sign decisions per row at the half-period marks, definite, read
///   off the value.
/// - `pcurve_loop_branch` is new, 0/0/0/18: the walk's own branch
///   decisions, definite.
/// - `pcurve_loop_continuity` 9/0/0/9 -> 9/0/4/5: with no `floor`
///   node between them, four joints' two ends go through the door.
///
/// **The frame decided across the window's cut** renames one row and
/// moves no count: `pcurve_chart_azimuth_frame` 0/0/0/26 ->
/// `pcurve_chart_cut_side` 0/0/0/26, the side of the principal window's
/// cut line each of the 26 azimuths lies on.
///
/// **Check 4 decides no whole period** (PCERT B2b): a row is its
/// placed image, so fidelity reads its offsets raw and
/// `pcurve_fidelity_branch` 0/0/0/24 decides nothing now, on every
/// spelling; no other row moves.
///
/// **The circles store their authored carrier.** Each hole's rim is the
/// authored centre and `|r|`, so the rim at a vertex folds onto the
/// radius as the tier's own algebra: 8 of `carrier_endpoint_end`'s and
/// `carrier_endpoint_start`'s door decisions, all 18 of
/// `pcurve_map_residual`'s and 36 of `carrier_matches_mapped_source`'s
/// are THEOREMS now, on this table and on the boss's alike. No numeric
/// decision moved. On the merged tree (PCERT's restated certificate) the
/// same fold takes four of `pcurve_loop_continuity`'s door decisions to
/// theorems as well, on both tables.
///
/// **The `*_on_surface_1`/`_2` slots are the pair's key order**
/// (`geom_brep::SurfacePair`), not the order a builder named the
/// surfaces in, so their decisions split evenly: `carrier_on_surface_*`
/// read `[126, 0, 0, 18]` each (they were `[135, 0, 0, 9]` and
/// `[117, 0, 0, 27]`, the same sum) and `witness_on_surface_*`
/// `[14, 0, 0, 2]` each (they were `[15, 0, 0, 1]` and `[13, 0, 0, 3]`).
/// No other row moved with it; the table below holds both changes.
///
/// **PCERT's chart-angle unit** retires the loop's chart-space angle
/// comparisons and check 5: `pcurve_loop_continuity` (16/0/0/8; the
/// boss's 16/0/2/6), `pcurve_trim_containment` (24/0/0/24; 0/24/0/24)
/// and `pcurve_loop_pole_joint` (0/0/0/12 on both) leave both tables. A
/// joint's element is decided as integers with its 3-D coincidence
/// following from the rows' envelopes, the certificate tests no row
/// against a window, and a cylinder has no singular set, so no joint of
/// it decides one.
///
/// **`cap_plane_orientation` is new, 0/0/0/4**: each extrude's two caps
/// decide once whether Newell's normal agrees with the profile's
/// winding, definite and numeric, on this table and on the boss's.
///
/// **Validation reads a missed contact candidate's segment ends.** Where
/// a span definitely misses a tangency or crossing candidate, each
/// segment end is asked whether it touches the other segment, and its
/// span only where it is not definitely off the carrier: `circle_side`
/// is new, 0/0/2/2, its registered rows the ends the arc shares with
/// its neighbours, whose rim (`Arc2::rim`) the arc registers; `chord_side`
/// 4/0/0/10 -> 6/0/0/12, `line_span` 4/0/0/4 -> 6/0/0/4, `arc_span`
/// 4/0/0/4 -> 4/0/2/4 and `contact_at_shared_vertex` 8/0/0/4 ->
/// 12/0/0/4.
///
/// **The final non-finite check decides nothing**
/// (`expr-non-finite-asks-value-times-zero`): it asks boundedness, not
/// `value · 0` against a band, so `expr_non_finite` (37/0/0/0) leaves
/// the table. Nothing else moves.
///
/// **The joint pass reads its turn** (#5354's ruling): validation
/// decides each joint by `path_junction_turn` after carrier identity,
/// so the two line/arc joints' `carrier_line_circle` reads move to it
/// and the two line/line joints add one each: `carrier_line_circle`
/// 0/5 -> 0/3, `path_junction_turn` 0/4 -> 0/8. Its fix pass reads
/// each junction a departing leg follows again at the shorter leg's
/// arm once that leg is stored, three more: 0/8 -> 0/11.
///
/// **INTENT stage 3 D** (a construction stands in the world by a mate
/// and the values it leaves): the copy's plane mate reads its chart,
/// which decides its three directions (`pose_unit_norm` 0/2 -> 0/5),
/// the two swings onto the mated direction
/// (`mate_chart_reference_antipode`, new at 2/0) and the one at-rest
/// verdict of the solved map (`solve_copy_at_identity`, new at 1/0).
/// Nothing else moves.
///
/// **A placement whose bundle reads a declared or toleranced variable
/// runs its map at the nominal** (C-5 narrowed, the orchestrator's
/// ruling). The parameter D-tab's copy reads the bulge parameter, so it
/// is no longer snapped at rest (`solve_copy_at_identity` leaves its
/// table) but run, and its transform's decisions count here as at every
/// box, then held as an overlay on this table. The literal's reads no declared or
/// toleranced variable and still snaps, so the two
/// spellings part on every row the transform decides as well as on
/// `carrier_matches_mapped_source`.
///
/// **FORK-REST** (only the identity bundle as written places by no
/// map): the literal D-tab's copy runs its map at the nominal too, so
/// both spellings decide the transform's rigidity (`transform_rigid_*`,
/// 1/0 each) and re-certify the copied body (`carrier_endpoint_*` 36 ->
/// 42, `carrier_on_surface_*` 126 -> 162, `witness_*` 4 more each,
/// `dihedral_*` 0/128 -> 0/156, `interval_span_*` and the `pcurve_*`
/// rows of the moved faces' charts), and `solve_copy_at_identity` is
/// gone with the snap. The two spellings now part on
/// `carrier_matches_mapped_source` alone. The parameter D-tab's
/// map-running rows are main's since its merge: `pcurve_chart_cut_side`
/// 0/26 -> 0/42, `pcurve_chart_placement` 0/52 -> 0/84, and the four
/// `pcurve_*azimuth*`/`pcurve_chart_winding` rows no longer decide.
///
/// Check 4 decides no whole period (PCERT B2b: a row is its placed
/// image), so `pcurve_fidelity_branch` 0/40 decides nothing (measured on
/// INTENT stage 3 D after merging main at `557e581a64`).
///
/// **D′2** (a value charts on the target space): the fold no longer
/// takes the mated direction back through the target body's motion, so
/// the chart decides one direction fewer (`pose_unit_norm` 0/5 -> 0/4).
/// Nothing else moved. `segment_gap` is the tab profile's own
/// validation, over its line pairs that share no vertex (a pair that
/// shares one is not read; `profile::seg`'s `line_line`).
const D_TAB_AT_THE_NOMINAL: &[(&str, [u64; 2])] = &[
    ("arc_apex_identity", [0, 1]),
    ("arc_diameter_clearance", [0, 6]),
    ("arc_span", [6, 4]),
    ("assert_bound", [0, 1]),
    ("cap_plane_orientation", [0, 4]),
    ("carrier_circles_identity", [3, 0]),
    ("carrier_cyl_axis_parallel", [1, 0]),
    ("carrier_endpoint_end", [42, 0]),
    ("carrier_endpoint_start", [42, 0]),
    ("carrier_line_circle", [0, 3]),
    ("carrier_on_surface_1", [162, 18]),
    ("carrier_on_surface_2", [162, 18]),
    ("chord_side", [6, 12]),
    ("circle_side", [2, 2]),
    ("contact_at_shared_vertex", [12, 4]),
    ("pose_unit_norm", [0, 2]),
    ("dihedral_arm", [0, 156]),
    ("dihedral_wedge", [0, 156]),
    ("extrusion_normal_component", [0, 2]),
    ("interval_span_forward", [0, 42]),
    ("interval_span_winding", [0, 16]),
    ("line_span", [6, 4]),
    ("mate_chart_pole", [0, 1]),
    ("newell_plane_residual", [30, 0]),
    ("path_circle_radius", [0, 1]),
    ("path_junction_turn", [0, 11]),
    ("pcurve_chart_cut_side", [0, 42]),
    ("pcurve_chart_orientation", [0, 20]),
    ("pcurve_chart_placement", [0, 84]),
    ("pcurve_chart_radial_moving", [22, 20]),
    ("pcurve_envelope", [16, 4]),
    ("pcurve_interval_forward", [0, 20]),
    ("pcurve_loop_branch", [0, 42]),
    ("pcurve_map_residual", [36, 0]),
    ("segment_gap", [0, 2]),
    ("segment_straightness", [6, 6]),
    ("side_cylinders_cosurface", [2, 0]),
    ("side_planes_cosurface", [0, 2]),
    // Limb 2's distance bound (C2), owed by every Intersection carrier:
    // 40 = the 20 Intersection edges (180 = 20 × 9 samples on
    // `carrier_on_surface_1`) × 2 operands. 36 are theorems: the margin
    // carries the carrier's free-parameter residual against the operand
    // as its identity (fork row 204), whose form is zero; the other 4
    // are the enclosure's own Zero, their identity not reaching the zero
    // form at the shipped rules.
    ("ssi_hull_sup", [36, 4]),
    ("transform_rigid_col01_orth", [1, 0]),
    ("transform_rigid_col02_orth", [1, 0]),
    ("transform_rigid_col0_unit", [1, 0]),
    ("transform_rigid_col12_orth", [1, 0]),
    ("transform_rigid_col1_unit", [1, 0]),
    ("transform_rigid_col2_unit", [1, 0]),
    ("transform_rigid_det_plus_one", [1, 0]),
    ("vertex_separation", [0, 12]),
    ("witness_at_mid_parameter", [20, 0]),
    ("witness_on_surface_1", [18, 2]),
    ("witness_on_surface_2", [18, 2]),
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
/// **This history is a record**, each paragraph in the receipt's
/// columns of its day (theorem / gated / registered / numeric, or
/// `sign_gated` by name); DECIDE-10 made the first three one column,
/// `symbolic`. A move between them recorded below is which walk
/// answered then, and is no move in today's receipt.
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
///
/// **Rule G's exact quotient takes the last of `arc_span`**, 5/0/0/1
/// -> 6/0/0/0, a THEOREM. It was the one value-free residual on the
/// boss and the decision that bounds its ceiling:
/// `2^-59·sqrt(5)·|c + 2^59·h| − sqrt(P/Q)` with `Q = (1 + h/a)⁴`
/// dividing `P = 5a²(1 + h/a)⁶` exactly (`a = c·2^-59` the chord half,
/// `h` its deviation) — a polynomial factor both halves share, which
/// rule E's monomial cancellation does not see. Minted over the
/// quotient `5(a + h)²`, the root is the first term's atom
/// (`m10_bulge_renders.txt` carries the form uncut). Nothing else on
/// the boss, either D-tab or either control moves.
///
/// **PCERT's incidence-and-fidelity unit (PR 3812)** moves the same
/// seven `pcurve_*` rows as on the D-tab, for the same reasons; the
/// boss's own numbers: `pcurve_map_residual` 0/0/37/89 -> 0/0/18/0,
/// `pcurve_envelope` 0/0/3/9 -> 6/6/0/0 (every one decided by the
/// form, six theorems and six gated on a sign), the three derivation
/// rows doubled, `pcurve_fidelity_branch` new at 0/0/0/24,
/// `pcurve_loop_branch` new at 0/0/0/18, and `pcurve_loop_continuity`
/// 9/0/0/9 -> 9/0/6/3.
///
/// **DECIDE-9** (a product with an ungated zero factor rests on that
/// factor alone), merged in after PR 3812, makes the six gated
/// `pcurve_envelope` decisions theorems: 6/6/0/0 -> 12/0/0/0. They are
/// the six `sym_9` counts on the boss (`sign_gated` 26 -> 20).
///
/// **INTENT-LITERALS PR C** (a slot holds a variable): a formula
/// written at a slot — the boss's `−chord_half` — lowers to an
/// anonymous defined variable, which the environment binds through the
/// non-finite door like every definition, so `expr_non_finite` gains
/// those bindings' theorems: 29/0/0/0 -> 33/0/0/0. Nothing else moves.
///
/// **The final non-finite check decides nothing**
/// (`expr-non-finite-asks-value-times-zero`): it asks boundedness, so
/// `expr_non_finite` (33/0/0/0) leaves the table. Nothing else moves.
///
/// **The joint pass reads its turn** (#5354's ruling): the two
/// line/arc joints' `carrier_line_circle` reads move to
/// `path_junction_turn`, 0/3 -> 0/1 and 0/2 -> 0/4. Its fix pass reads
/// each door junction again at the shorter leg's arm once the leaving
/// leg is stored, one more for the boss's arc junction: 0/4 -> 0/5.
///
/// **INTENT stage 3 D** (a construction stands in the world by a mate
/// and the values it leaves): the copy's plane mate reads its chart,
/// which decides its three directions (`pose_unit_norm` 0/2 -> 0/5),
/// the two swings onto the mated direction
/// (`mate_chart_reference_antipode`, new at 2/0) and the one at-rest
/// verdict of the solved map (`solve_copy_at_identity`, new at 1/0).
/// Nothing else moves.
///
/// **A placement whose bundle reads a declared or toleranced variable
/// runs its map at the nominal** (C-5 narrowed, the orchestrator's
/// ruling), as on the parameter D-tab: `solve_copy_at_identity` leaves, `transform_rigid_*` are new
/// at 1/0 each, and the copied body's re-certification raises
/// `carrier_endpoint_*` 24 -> 30, `carrier_matches_mapped_source` 126
/// -> 144, `carrier_on_surface_*` 90 -> 126, `witness_*` 10 -> 14,
/// `dihedral_*` 0/80 -> 0/108, `interval_span_*` and the eleven
/// `pcurve_*` rows. Every row rose; no symbolic decision went numeric.
///
/// **Main's charts of a moved face** (merged into D): a copy whose
/// placement runs its map derives its moved faces' charts again on
/// main's pcurve rows, `pcurve_chart_cut_side` 0/26 -> 0/42,
/// `pcurve_chart_orientation` 0/12 -> 0/20, `pcurve_chart_placement`
/// 0/52 -> 0/84, `pcurve_chart_radial_moving` 14/12 -> 22/20,
/// `pcurve_envelope` 12/0 -> 20/0, `pcurve_fidelity_branch` 0/24 ->
/// 0/40, `pcurve_interval_forward` 0/12 -> 0/20, `pcurve_loop_branch`
/// 0/25 -> 0/42 and `pcurve_map_residual` 18/0 -> 36/0, as on the
/// D-tab. Under FORK-REST the boss's copy, mated and not the identity
/// bundle, runs its map, as it did under the narrowing.
///
/// Check 4 decides no whole period (PCERT B2b: a row is its placed
/// image), so `pcurve_fidelity_branch` 0/40 decides nothing (measured on
/// INTENT stage 3 D after merging main at `557e581a64`).
///
/// **D′2** (a value charts on the target space): the fold no longer
/// takes the mated direction back through the target body's motion, so
/// the chart decides one direction fewer (`pose_unit_norm` 0/5 -> 0/4).
/// Nothing else moved.
#[test]
fn m10_bulge_the_bosss_split_at_the_nominal() {
    let tol = Tol::witness();
    let (doc, _, _) = crate::m10_10_r1_probes_interval::segment_boss(1.0, tol);
    assert_split(
        "segment_boss",
        &split_at_the_nominal(&doc, SymRules::shipped(), tol),
        &[
            ("arc_apex_identity", [0, 1]),
            ("arc_diameter_clearance", [0, 6]),
            ("arc_span", [6, 0]),
            ("assert_bound", [0, 1]),
            ("cap_plane_orientation", [0, 4]),
            ("carrier_circles_identity", [3, 0]),
            ("carrier_cyl_axis_parallel", [1, 0]),
            ("carrier_endpoint_end", [30, 0]),
            ("carrier_endpoint_start", [30, 0]),
            ("carrier_line_circle", [0, 1]),
            ("carrier_matches_mapped_source", [144, 0]),
            ("carrier_on_surface_1", [126, 0]),
            ("carrier_on_surface_2", [126, 0]),
            ("contact_at_shared_vertex", [6, 3]),
            ("pose_unit_norm", [0, 2]),
            ("dihedral_arm", [0, 108]),
            ("dihedral_wedge", [0, 108]),
            ("extrusion_normal_component", [0, 2]),
            ("interval_span_forward", [0, 30]),
            ("interval_span_winding", [0, 16]),
            ("line_span", [2, 0]),
            ("mate_chart_pole", [0, 1]),
            ("newell_plane_residual", [18, 0]),
            ("path_circle_radius", [0, 1]),
            ("path_junction_turn", [0, 5]),
            ("pcurve_chart_cut_side", [0, 42]),
            ("pcurve_chart_orientation", [0, 20]),
            ("pcurve_chart_placement", [0, 84]),
            ("pcurve_chart_radial_moving", [22, 20]),
            ("pcurve_envelope", [20, 0]),
            ("pcurve_interval_forward", [0, 20]),
            ("pcurve_loop_branch", [0, 42]),
            ("pcurve_map_residual", [36, 0]),
            ("segment_straightness", [2, 6]),
            ("side_cylinders_cosurface", [2, 0]),
            // Limb 2's distance bound (C2), owed by every Intersection
            // carrier: 28 = the 14 Intersection edges (one witness each,
            // below) × 2 operands. Each a theorem on the carrier's
            // free-parameter residual (fork row 204).
            ("ssi_hull_sup", [28, 0]),
            ("transform_rigid_col01_orth", [1, 0]),
            ("transform_rigid_col02_orth", [1, 0]),
            ("transform_rigid_col0_unit", [1, 0]),
            ("transform_rigid_col12_orth", [1, 0]),
            ("transform_rigid_col1_unit", [1, 0]),
            ("transform_rigid_col2_unit", [1, 0]),
            ("transform_rigid_det_plus_one", [1, 0]),
            ("vertex_separation", [0, 8]),
            ("witness_at_mid_parameter", [14, 0]),
            ("witness_on_surface_1", [14, 0]),
            ("witness_on_surface_2", [14, 0]),
        ],
    );
}

/// The whole D-tab table for one of the two spellings:
/// [`D_TAB_AT_THE_NOMINAL`] plus `carrier_matches_mapped_source`.
fn d_tab_table(mapped_source: [u64; 2]) -> Vec<(&'static str, [u64; 2])> {
    let mut out = D_TAB_AT_THE_NOMINAL.to_vec();
    out.push(("carrier_matches_mapped_source", mapped_source));
    out
}

/// The D-tab with its bulge a literal `0.4`: [`D_TAB_AT_THE_NOMINAL`],
/// with `carrier_matches_mapped_source` 126/0/44/10 (188/10 since its
/// map runs, 192/6 with rule H's angle sums) — six of the
/// eighteen that used to stand went to the DOOR with SYM-5's rule E,
/// and two more with the schedule's assigned last sample.
#[test]
fn m10_bulge_the_d_tabs_literal_split_at_the_nominal() {
    let tol = Tol::witness();
    let (doc, _, _) = crate::m10_10_r2_probes_interval::d_tab(1.0, false, tol);
    assert_split(
        "d_tab (bulge a literal)",
        &split_at_the_nominal(&doc, SymRules::shipped(), tol),
        &d_tab_table([192, 6]),
    );
}

/// The D-tab with its bulge a document parameter: the same table as
/// the literal's ([`D_TAB_AT_THE_NOMINAL`]) except on ONE row. At the
/// nominal the bulge's sign is not what blocks on any of the shared
/// rows, the ring is, on the same nodes — but with SYM-5's rule E the
/// two part on `carrier_matches_mapped_source`: 126/0/44/10 on the
/// literal against 126/0/40/14 here, so four of the six the rule takes
/// on the literal it does not take when the bulge is a parameter. The
/// sign
/// entered, where the dyadic control showed it, through the carrier's
/// span while that was spelled `4·atan|b|`: `abs(b)` and
/// `sqrt(1 + abs(b)²)` against the pushforward's `sqrt(1 + b²)`. The
/// span is now the stored sweep signed by the decided turn
/// (`sweep`'s `arc_span`), one `atan b` atom on both sides, and this
/// row's split did not move with it: the ring blocks first. The sign
/// still rides in the radius `abs(L(1+b²)/(4b))`.
#[test]
fn m10_bulge_the_d_tabs_parameter_split_at_the_nominal() {
    let tol = Tol::witness();
    let (doc, _, _) = crate::m10_10_r2_probes_interval::d_tab(1.0, true, tol);
    assert_split(
        "d_tab (bulge a parameter)",
        &split_at_the_nominal(&doc, SymRules::shipped(), tol),
        &d_tab_table([184, 14]),
    );
}
