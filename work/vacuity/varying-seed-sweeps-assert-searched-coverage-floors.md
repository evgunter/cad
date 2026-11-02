---
id: varying-seed-sweeps-assert-searched-coverage-floors
kind: issue
title: Twelve varying-seed sweeps assert a coverage floor over their own draw, the shape-1/shape-3 mix that made review_gui1_r1 red one seed in 1300
status: open
opened: 2026-10-28
priority: P3
cost: M
---


Filed by GREEN's `green/gate-trust` lane (2026-10-28) out of the sweep
behind `pick-oracle-sweep-anti-vacuity-fails-one-seed-in-1300`. That row's
test drew a varying seed (`fuzz::start`) and then asserted
`hits_seen > 0` over the draw. About one seed in 1300 missed, and the test
went red on at least five unrelated PRs. Discipline §8 names the
shape: an anti-vacuity floor bolted onto a property sweep makes one count
serve two obligations. The fix is to make the floor's witness static
(plant it), pin a separate coverage run (`fuzz::pinned`), or split the
test.

## Hits

Instrument: a script that splits each file containing `fuzz::start(` at
`#[test]` and, inside each chunk that opens a varying stream, lists the
`assert!(X > …)`, `assert!(X >= …)`, `assert!(X)` and `assert_ne!(X, 0)`
asserts on a variable declared `let mut X = 0 | 0.0 | false | NEG_INFINITY`.
Not yet dispositioned unless marked. Some may already be safe because the
floor is met by construction, or is scaled to sit well below the draw's
mean. Each one needs its miss probability at `CAD_FUZZ_EFFORT=1` measured
(or argued) before it is called safe.

- `crates/editor-core/tests/review_gui1_r1.rs`, `random_integer_rays_match_the_exact_oracle`: `hits_seen > 0`. **Fixed** (planted case 0), in the PR that filed this row.
- `crates/mesh/src/nurbs_cert_fuzz.rs`, `cert10_the_fold_never_exceeds_the_whole_net_hull`: `strict > trials`, `saw_high_mult`. **Fixed** (split onto a pinned draw) under `cert10-strict-gap-floor-gates-on-a-varying-seed`, PR 5944.
- `crates/mesh/src/nurbs_cert_fuzz.rs`, `r1_random_rational_soundness_sweep`: `worst > 0.5`. **Left, with a reason** (PR 5944): each trial meets the floor on its own (60 of 60 above 0.5 at seed 1001, minimum 0.88), so a red needs every trial to miss at once.
- `crates/geom/tests/curves/review_m5_pr3_attack.rs`, `f4_removal_bound_contains_realized_error_lossy_and_adversarial_weights`: `checked >= WANTED && multi >= 1`.
- `crates/geom/tests/surfaces/review_m5_pr3_attack.rs`, `f4_surface_removal_bound_honest_u_and_v`: `successes >= QUOTA`.
- `crates/geom-core/src/spline/blossom.rs`, `the_thin_step_reads_the_general_products_to_the_bit`: `above > 0`.
- `crates/sweep/tests/review_d8_consumer_differential.rs`, `the_ring_bezier_decomposition_still_breaks_where_the_structure_says`: `checked >= 20`.
- `crates/sweep/tests/review_d8_consumer_differential.rs`, `make_compatible_still_lands_every_section_on_the_union_vector`: `groups >= 5`.
- `crates/sweep/tests/review_verbs_rim_lever_probes.rs`, `lever_is_bounded_by_arc_length_and_never_collapses_on_a_closed_rim`: `checked > 0`.
- `crates/topo/src/boolean/solid_contain.rs`, `a_wall_and_its_clockwise_twin_read_complementary_sides`: `read > 0`.
- `crates/topo/src/validate/gap_rows.rs`, `a_gap_read_apart_is_a_real_gap_on_points_far_from_the_origin`: `fired > 100`.
- `crates/viewer/tests/review_gui0_r1.rs`, `the_camera_contract_survives_random_operation_walks`: `refusals > walks / 4`.
- `crates/viewer/tests/review_gui2_r2.rs`, `cursors_across_the_pane_never_disagree_with_the_id_map`: `hits >= witness.len()`.
- `interval-transcendentals/tests/certify.rs`, `certify_powi`: `off_domain_cases > 0`.
- `interval-transcendentals/tests/review_fuzz_exact.rs`, `fuzz_div_witness_soundness_and_containment`: `degenerate > fuzz::scaled(DEGENERATE_FLOOR)`, `n > fuzz::scaled(CASE_FLOOR)`.

## Blind spots

1. **Floors spelled through `test_utils::vacuity::Exposure`**
   (`require`, `require_each`, `require_nonzero_among`) are invisible to
   the pattern above. These files open a varying stream and call one of
   them, but whether the floor is over that stream is unchecked:
   `crates/topo/src/review_d18.rs`, `crates/sweep/tests/review_d2_adv_probes.rs`,
   `crates/bvh/tests/ray.rs`, `crates/bvh/tests/ray_r2.rs`,
   `crates/editor-core/tests/gui1_pick_r2.rs`,
   `tools/tess-meter/tests/derivations.rs`.
2. **A sweep whose `fuzz::start` lives in a helper fn** that the test
   calls, with the floor in the test, is split across chunks the script
   does not join.
3. **Floors over a non-zero-initialised accumulator**, or spelled as
   `assert_eq!`, `!v.is_empty()` or a `match` arm, are not matched.
   (A first pass that split at `fn` rather than `#[test]` missed both
   `nurbs_cert_fuzz` rows, because a nested `fn mk` cut the body. The
   `#[test]` split found them.)
