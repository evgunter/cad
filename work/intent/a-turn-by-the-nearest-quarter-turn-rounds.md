---
id: a-turn-by-the-nearest-quarter-turn-rounds
kind: issue
title: A spin by the f64 nearest a quarter turn rounds, so a frame written by turns is off its axes by an ulp
status: closed
opened: 2026-10-19
closed: 2026-10-20
priority: P2
cost: M
parent: a-placement-pins-by-written-values-without-a-mate
---

A spin's rotation is built by Rodrigues from `angle.sin_cos()` (`crates/geom-core/src/linalg/mat.rs:203-212`). The f64 nearest π/2 gives `cos ≈ 6.1e-17`, not 0. So a frame, plane or placement spelled by turns of ±π/2 or π leaves the world's axes by about an ulp. That covers the `frame_at`/`moved_pose` frames in `crates/editor-core/src/mate/pin.rs` and the pin helpers' `moved_by`, and so `test_support::plane_on`, `axis_on` and every placement `place_at` writes. A `Datum` used to give those axes exactly.

Found under INTENT stage 3 D:
- `split_pinch_keeps` reads its tool `y = 1` off the body, and the notch tip at `y = 1` then splits by rounding (a naming refusal, or tables that differ under a flip). Its rows now build the plane from a face's plane and a slide, with no turn.
- `intent_s3_a_poses::a_meets_representative_moves_no_readers_outcome` allows rounding in its untilted case for the same reason.
- `p6_` in the D diagnosis lane found the same spins in `frame_at`.

D′ makes this worse: an unmated placement's rotation is a written spin chain, so a part stood upright is a quarter turn written as a number.

**Done when** a turn by a multiple of a quarter turn is exact wherever it is written. Either the rotation's sine and cosine are taken exactly at the f64 nearest a multiple of π/2, in every lane, a ruled change of geom-core's rotation, or the helpers spell axis-aligned frames by exact roles instead of turns. Add a row: a block placed upright by a written π/2 has its faces on the world's planes bit for bit.

**Evidence, the kitchen sink's census (2026-10-20, merging main into D).** The sink's revolve is placed in the world at `(-2, 0, 0)` with its own `y` along the world's `z` (`corpus/sink.rs`, `in_world` over `frame_at`), and `place_at`'s axis path writes that as spins taken by `atan2`: `-π/2` about `x`, then `-6.1e-17` about `z` where the turn is exactly zero. The composed map carries `cos(π/2)·sin(6.1e-17) ≈ 3.7e-33` where the datum's axes were exact, so the revolve's corner at `(0, 0, 1)` lands at `y = -3.7e-33`. In `perf12_census_goldens` (every ε row) four of the sink's contacts with the union's edge along the `z` axis read margins of `2.3e-33`, `3.7e-33` and `2.3e-49` where they read `0.0`, and one curved-face pair (the revolve's face against a block face) drops out of reach. The gallery ring's placement shows the same `-6.1e-17` spin (`crates/viewer/tests/creation_ops.rs`).

**Evidence, the tour die at `Interval` (2026-10-20).** `m10_di_dual_corpus::assemble_census_verdicts_match_f64_at_interval` is red on `die_composed_tour` alone: `assemble` passes at `f64` and refuses at `Interval` with 16 `CensusEscalated` findings, each `curve_pair_pose_root_gap` Indeterminate over an enclosure `[0, 6.3e-7 … 1.5e-6]` against the band `[1e-9, 1e-8]`. The pips are placed against the die by quarter-turn frames, and their solved maps read at `f64` carry `±6.1e-17` where an axis is exact, `2.3e-49`/`1.4e-65` cross terms, and slides an ulp off (`0.4999999999999999`, `0.28000000000000014`); at `Interval` each map's entries are enclosures up to `1.5e-14` wide. A touch the `f64` census reads at zero is, at `Interval`, the root of a squared gap whose enclosure is that wide, `[0, ~1e-6]`, which straddles the band. Before D the pips stood on exact datum frames and the gap enclosed `[0, 0]`. Not the at-rest snap: none of these placements is at rest. (Repro: evaluate `die_composed_tour` at `Interval` and `assemble`; print each pip placement's `NodeValue::motion`.)

**Evidence, D under FORK-REST (2026-10-20).** Both findings above still occur on D's head after FORK-REST and the identity bundle, unchanged: `m10_di_dual_corpus::assemble_census_verdicts_match_f64_at_interval` reads the same 16 `curve_pair_pose_root_gap` escalations on `die_composed_tour` there as on `434139ef1c`. A further symptom, measured on both heads and not traced to its map: at `CAD_TOLERANCE_EPS=1e-12` the `Interval` build of `die_composed` refuses its fillet (node 129, `pcurve_map_residual` at sample 5 over `[0, 1.0009918679106779e-12]` against the band `(1e-12, 1e-11)`) and poisons the copy placed after it, and the tour die's fillet (node 1961) refuses the same way (`carrier_endpoint_end` over `[0, 1.1476773500649958e-12]`). At 1e-9 and 1e-6 both build. So `m10_p_fence::the_corpus_evaluation_is_bit_identical_at_interval`, which asserts one number at every ε, reads `(0x5cd6_4b4f_4a5c_4cb0, …)` at 1e-6 and 1e-9 and `(0xb886_6296_d62c_84d2, …)` at 1e-12, and the nightly's 1e-12 row of it is red. The fillets read the pips' unions, whose pips are placed by quarter-turn frames; that this is those maps' enclosure width reaching the fillet's certification is the likely reading, not a measured one. Repro: evaluate `die_composed` at `Interval` with `CAD_TOLERANCE_EPS=1e-12` and print `ev.nodes`' failures.

## Closed

Closed by INTENT stage 3 D's exact-turn lane (2026-10-20), by the first reading of the done-when, keyed on the recipe's form rather than on a value.

- **The rotation.** `geom_core::Angle` and `Turns` (`crates/geom-core/src/linalg/angle.rs`): an angle written as a rational fraction of a turn takes its sine and cosine from the fraction, exact where the value is rational (a quarter turn's `0` and `±1`, a twelfth's `1/2`), the octant's otherwise; `I − R` reads `(sin, 1 − cos)` exactly for a whole number of quarter turns. The rotation constructors take it (`Mat3::rotation_about_unit_by`, `Affine3::rotation_about_unit_axis_by`, `RigidMotion::rotation_by`, `rotation_about_axis_by`), the radian forms delegating to the same body.
- **Where the form crosses.** `Expr::constant_turns` reads a definition's form; `Doc::bind_definition`, the door every environment ends at, records each angle variable whose definition is a constant fraction of a turn in `VarEnv::turns`; `expr::eval_angle` and `eval::slots::angle` hand it on. Every rotation fed by an angle reads it: pose and world-read spins, loft station spins, a revolve's heading, a circular pattern's steps, and a placement's mated (`mate::value::settle`'s `turn_to`, `Chart::turned`, `Chart::head_at`) and unmated (`unmated_motion`) spins. The symbolic tier folds sin, cos and tan of a rational multiple of π (`sin-and-cos-of-rational-turns-fold-exactly`, closed beside this).
- **Writers.** The corpus pips and the tour die stand on lines stated from the cube's corner (`corner_frame`, `moved_frame`); the kitchen sink's revolve and the vessel stand upright by a bundle with no mate spun `turn/4`; `test_support::plane_on` reads a face with the stated normal and `axis_on` the meet of two.
- **The row:** `intent_s3_d_placements::a_block_placed_upright_by_a_written_quarter_turn_stands_on_the_worlds_planes`, bit for bit at `f64`, `Interval`, `Dual64` and `Sym`, for a bundle with no mate, a mated spin, a world read spun by `turn/4` and an axis mate off the origin.
- **The evidence above:** `m10_di_dual_corpus` is 8/8; every corpus document builds at `Interval` at ε = 1e-12; the kitchen sink's census contacts read `0.0` and its curved pair is in reach again (`perf12_census_goldens`).

- **An axis mate off the origin at `Interval`.** Undoing the copy's measured spin about the mate's line read the backend `atan2`'s dust for an exact zero (`[-2e-323, 2e-323]`, its cosine an ulp wide), which widened the copy's translation by the line's lever (a pip at `0.5 ± 2.2e-16`); at ε = 1e-12 that left `die_composed`'s pip rims two curved-edge meetings the census could not count at `Interval`. The box lanes' `solve_atan2` encloses an exact `(0, x > 0)` as the exact zero, and `turn_to` undoes nothing for an exactly zero measured spin; the row's fourth case is that axis mate.

Residue, each in its own file: the sweep's revolve angle and tube windows still cross as radians (`work/carve/a-partial-revolve-by-a-fraction-of-a-turn-ends-off-its-axes.md`); the helpers that read a turn off geometry by `atan2` (`place_at`, `place_against`, `frame_at`, `pin_at`) are D′'s to re-spell (`a-placement-pins-by-written-values-without-a-mate`, its exact-turn section).

