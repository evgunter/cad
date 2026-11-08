---
id: a-ball-and-cylinder-boolean-costs-seconds-per-op-in-an-optimized-build
kind: issue
title: A ball's boolean with a cylinder spends 3–9 s per operation at the default ε and 19–69 s at 1e-12 in an optimized build with debug assertions off, before it refuses at the volume gate
status: open
opened: 2026-11-02
priority: P1
cost: H
---


Filed by GREEN's slow-test census (2026-11-02), answering Ev's question
in chat whether the nightly's slow rows are the kernel being slow or a
pile of `debug_assert`s.

## Measured

It is the kernel. A unit ball (`sweep::test_support::ball_poled(1.0,
origin, +z)`) against an upright cylinder, each operation timed alone
by a throwaway example (not committed) on a 4-core dev box with nothing
else running, built at the nightly's opt-level 2 with **debug
assertions and overflow checks off**:

| operation | default ε | ε 1e-12 | stops at |
|---|---|---|---|
| ball ∪ tall bar (a brick, control) | 0.03 s | 0.02 s | a body |
| ball ∖ tall bar (control) | 0.02 s | 0.01 s | a body |
| ball ∪ rod (`cyl(0.1, 0, r 0.3, z −2..2)`) | 9.29 s | 69.06 s | refusal |
| ball ∖ rod | 5.08 s | 31.98 s | refusal |
| ball ∪ stub (`cyl(0.5, 0.2, r 0.4, z 0.3..2)`) | 5.99 s | 38.11 s | refusal |
| ball ∖ stub | 2.78 s | 18.56 s | refusal |

The shapes are `the_door_reads_seam_regions_as_tier_3_does.rs`'s `unit`,
`rod` and `stub`. Every curved operation refuses at the end, at the
result's volume (`VolumeUncomputable: QuadratureUnsupported`, a
projected image on an analytic chart, which that file pins and
`work/spiricarm/a-projected-boundary-on-an-analytic-chart-has-no-volume-lane.md`
owns). So today a user pays 3–9 s per click at the default ε for a
refusal, and once that volume lane lands the same seconds buy a body.
Against a brick the same ball costs 20–30 ms, so this is the curved
pair's pipeline, 100–300× the planar one, and it grows about 7× from the
default ε to 1e-12.

**Debug assertions are not the cost.** The same tests at the same
opt level with assertions on and off (`CARGO_PROFILE_{DEV,TEST}_DEBUG_ASSERTIONS`
and `_OVERFLOW_CHECKS` = false, the binary checked to lack a
`debug_assert!` message and the overflow panic string), each pair run
side by side on the idle box:

| test | ε | asserts on | asserts off |
|---|---|---|---|
| `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_in_two_loops` | 1e-12 | 291 s | 296 s |
| `the_door_reads_seam_regions_as_tier_3_does::…_against_the_stub` | 1e-12 | 202 s | 206 s |
| `germ_sphere_torus_placements::a_ball_against_a_donut_carries_onto_the_torus_and_stops_at_the_volume` | 1e-12 | 285 s | 285 s |
| `a_ringed_wall_divided_by_a_carried_chord::a_wall_holding_one_balls_ring_is_divided_by_the_others_chord` | 1e-12 | 198 s | 198 s |
| `germ_torus_components::two_donuts_in_general_pose_pass_the_germ_mint` | default | 121 s | 118 s |
| `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs` | default | 217 s | 211 s |
| `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs` | 1e-12 | 1 386 s | 1 388 s |

## Where the time goes (from SECT's profiles, not re-taken here)

SECT sampled the seam-region ∪ and the torus rows' ops with gdb
(`work/secttorus/torus-germ-rows-march-for-an-hour-at-a-fine-band.md`,
"Where the time goes"; `work/sect/an-operation-retakes-each-carried-chords-rung-3-certificate-five-times.md`):
the cost is not the march (3–16 %) but the certificates of the fitted
chord edges, read at the join's edge door (`euler::certify_edge_spec` →
`geom_brep::analytic_rung3`, whose limb 2 composes the chord's whole
net in `pcurve_cache::limb2_offset_sup`), at the pcurve mint
(`pcurves::mint_pcurves` → `pcurve_cache::certify_projected`), and at
the result gate (`validate::validate_geometric_certificate`);
inclusive, `pcurve_cache::twin_span_hulls` and
`ssi::certify::certify_branch` (limb 3's tube). Each arc is certified
five times per operation on bit-identical input; the repeats are about
a fifth of the operation (that row's own fix). The first certificate of
an arc is 0.37 s at the default ε and 1.9 s at 1e-12, and a ball ×
cylinder union cuts about eight arcs.

## Why P1

A ball against a cylinder is everyday geometry, and a multi-second
boolean on it is a verb a user cannot work with interactively — Ev's
word for it was "unusable". The existing rows on this cost are framed
as the nightly's budget (`torus-germ-rows-march-for-an-hour-at-a-fine-band`,
P2, whose fix allows "rows sized for the nightly's budget") or as one
fifth of it (`an-operation-retakes-…-five-times`, P3); neither is the
per-operation latency, and shrinking the test rows does not fix it.
Not P0: the answers are right, and the refusal at the end is another
row's.

## What closing it takes

A per-operation budget the curved pair's pipeline meets, stated and
measured: e.g. a ball × cylinder boolean under ~0.5 s at the default ε
in an optimized build. The likely levers, largest first by SECT's
samples: certify each chord once per operation and carry the verdict
(the five-times row); make a chord's first rung-3 certificate cost
less than composing its whole net (limb 2's offset sup over a 265-point
fitted arc); and stop re-certifying at the result gate what the edge
door already certified. Measure first: one gdb- or perf-sampled ∪ at
the default ε in this build, split by those three stages.

## Reproduce

Build any sweep test or example at
`CARGO_PROFILE_DEV_OPT_LEVEL=2 CARGO_PROFILE_DEV_DEBUG_ASSERTIONS=false`
and time `topo::boolean::union(&unit, &rod, Tol::witness())` with the
shapes above; or run
`cargo nextest run -p sweep --test all -E 'test(=the_door_reads_seam_regions_as_tier_3_does::the_door_names_the_flipped_face_tier_3_names_against_the_rod)'`
(8 booleans; 157 s at 1e-12 on the nightly's runner on 2026-11-01).

Moved from SECTRUNG to SECT on 2026-11-02, by `git mv`, when SECTRUNG merged back into SECT: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so SECT and the three tracks cut from it (SECTTAIL, SECTRUNG, SECTSITE) came in under one budget.
