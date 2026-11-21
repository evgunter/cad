---
id: a-ball-and-cylinder-boolean-costs-seconds-per-op-in-an-optimized-build
kind: issue
title: A ball's boolean with a cylinder spends 3–9 s per operation at the default ε and 19–69 s at 1e-12 in an optimized build with debug assertions off, before it refuses at the volume gate
status: parked
opened: 2026-11-02
priority: P1
cost: H
blocked_on: [a-chords-locus-certificate-is-not-its-section-components-cut-to-its-span, a-ball-and-cylinder-results-volume-is-a-fifth-of-the-boolean, a-fitted-rows-first-projected-certificate-costs-a-quarter-of-a-ball-cylinder-boolean, check-twos-turn-reads-first-speed-meters-at-a-tenth-of-a-ball-cylinder-boolean]
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

## 2026-11-04: the march's 3-D sweeps retiring does not move it

The SECTARMS cylinder × sphere lane (branch
`sectarms/cylinder-sphere-ruling-sweep`) retired this pair's
`seed_r3`/`account_r3` from the section value's march. The pair now
marches from the ruling sweep's witnesses. The table's four operations
were timed on the branch and on its merge base (`aef1e58929`) one after
the other on the same 4-core box. Each was built at opt-level 2 with
debug assertions and overflow checks off and timed alone by a
throwaway example, not committed:

| operation | default ε, base → branch | ε 1e-12, base → branch |
|---|---|---|
| ball ∪ rod | 8.97 → 8.41 s | 60.23 → 59.52 s |
| ball ∖ rod | 3.54 → 3.72 s | 29.66 → 30.11 s |
| ball ∪ stub | 4.84 → 5.13 s | 35.28 → 33.66 s |
| ball ∖ stub | 2.10 → 2.09 s | 16.17 → 16.86 s |

Every one still stops at the same `VolumeUncomputable` on both. The
differences are within the box's noise. That fits the profile above:
the march is a small share of the time, and the certificates are the
cost.

## 2026-11-20: re-measured, and the tube lever (`sect/ball-cylinder-cost`)

Every operation now ends in a body (the volume lane has landed). On main
`d44f46d3`, 4 idle cores, opt-level 2, debug assertions and overflow
checks off, each op timed alone by a throwaway example:

| operation | default ε, main → branch | ε 1e-12, main → branch |
|---|---|---|
| ball ∪ rod | 4.34 → 2.43 s | 25.49 → 15.40 s |
| ball ∖ rod | 3.62 → 1.80 s | 21.00 → 11.12 s |
| ball ∪ stub | 2.47 → 1.70 s | 14.81 → 8.79 s |
| ball ∖ stub | 2.12 → 1.22 s | 12.98 → 6.64 s |

**Where the time went on main** (ball ∪ rod, 150 gdb samples of the
main thread): the chords' rung-3 certificate 53 %, and in it limb 2
(`limb2_offset_sup`) under 1 % (its memo serves it) while limb 3 was
all of it: `intersect::tube_bounds` 28 %, which composed the carrier's
control hulls and then refused the pair (`Unstated::Pair`: sphere ×
cylinder had no closed form), and the tube it left owed,
`ssi::certify::certify_branch`, 27 %. Counted: 40 rung-3 calls on 16
distinct inputs (8 arcs × 4 repeats + 8 once); the repeats were half of
the rung-3 time, 1.39 of 2.81 s. The rest: the result gate's volume
14 %, the pcurve mint 7 %, the site mint's pcurve rows 7 %.

**The lever** (`geom_brep::intersect::section_bounds`): sphere ×
cylinder states its crossing-angle floor in closed form, so the tube is
retired on every one of those calls; an unclassified pair is refused
before its hulls are composed; and each span is read whole first and
cut into its `PIECES` pieces (by de Casteljau, from one Bernstein row)
only where its whole box could lower the floors, so the floors are those
every span cut gives. `tube_bounds` in ball ∪ rod: 1.27 s of refusals →
0.15 s of certificates, and `certify_branch` no longer runs.

**Where the time goes now** (ball ∪ rod, 2.43 s, 150 samples): the
volume 22 % (`work/quad/a-ball-and-cylinder-results-volume-is-a-fifth-of-the-boolean.md`),
the projected pcurve certificates at three doors about 40 %
(`a-fitted-chords-projected-pcurve-certificate-is-taken-at-three-doors-per-operation.md`),
the rung-3 certificate 15 % (still 40 calls on 16 inputs: the
five-times row), the edge spec's speed meters 13 % (partly
`work/nurbs/a-curve-cloned-before-its-speed-meters-first-read-computes-them-again.md`),
the march 8 %. The target (under about 0.5 s) is not reached; the
largest remaining levers are certificates taken more than once, whose
sharing is the design question the five-times row names.

## 2026-11-21: re-measured after PRs 7173 and 7193

On main `afa8184d`, 4 cores with nothing else running (`top` 90 % idle
between rounds; the load average of 1.6–3.3 was these runs' own volume
workers and builds that had finished before timing began). Each op was
timed alone by a throwaway timer (not committed), each run in a fresh
process, because a second op in one process reads the first one's
session memos and comes in 20–30 % faster. Every op ends in a body.

**Build the timer outside the dev-dependencies.** An example under
`crates/sweep/examples/` links sweep's dev-dependency
`geom-brep/read-pins-testing`, whose pin derives every certificate read
by its support again to compare it (`locus_certificate::read_or_derive`).
That took 14 % of ball ∪ rod's samples (21 of 150) and 0.2–0.5 s per op. The
tables below come from a throwaway `src/bin/` target built with
`--features test-support` only, checked to lack the pin's panic string.
The viewer's own build doesn't link the pin either: viewer names it under
`[dev-dependencies]` only.

Median of three runs each, in seconds:

| operation | `--release` (opt 3, debug assertions on, as the viewer builds), default ε | same, ε 1e-12 | opt 2, assertions and overflow checks off, default ε | same, ε 1e-12 | 11-20 branch, opt 2, default → 1e-12 |
|---|---|---|---|---|---|
| ball ∪ rod | 2.05 (1.91–2.07) | 10.72 (10.68–11.03) | 2.35 (2.23–2.45) | 13.09 (12.68–13.81) | 2.43 → 15.40 |
| ball ∖ rod | 1.79 (1.67–1.99) | 10.22 (9.92–10.56) | 2.19 (2.16–2.58) | 12.41 (12.14–12.79) | 1.80 → 11.12 |
| ball ∪ stub | 1.12 (1.06–1.13) | 6.27 (6.11–6.31) | 1.49 (1.32–1.53) | 7.68 (7.39–7.82) | 1.70 → 8.79 |
| ball ∖ stub | 1.04 (1.03–1.11) | 6.34 (6.31–6.50) | 1.56 (1.53–1.69) | 7.79 (7.49–8.34) | 1.22 → 6.64 |

The unions moved a little (3–15 %). The two differences came back 20–30 %
slower than the 11-20 branch, which was not traced. The viewer's
build is about 0.85× the opt-2 one.

**Where the time goes** (ball ∪ rod, default ε, the opt-2 build with
line tables). There are 150 gdb samples, one per fresh process, each at a uniform
offset into the op. Each sample covers the main thread and the four
rayon workers. The workers run only in the volume, so the volume's main-thread
share below is its wall time. "Inclusive" is the main thread's share; seconds
are that share of 2.35 s. Timers and counts come from throwaway
`eprintln!` counters on the same build, two runs, since reverted.

| stage (main thread, exclusive) | inclusive | ≈ s | what is under it |
|---|---|---|---|
| the join's edge door (`chord_join::join` → `euler::mef` → `certify_edge_spec` → `run_checks`) | 45 % | 1.07 | below |
| · rung 3 (`edge_nurbs::analytic_rung3`) | 29 % | 0.58–0.68 timed | **12 calls**. Limb 2 (`limb2_offset_sup` → `twin_span_hulls` → `spans_of` → `piece_controls` → blossom `restrict`) is 26 %; the tube (`tube_bounds`, 12 calls, 0.06 s) is 3 % |
| · check 2's turn (`decide_turn` → the speed meters) | 13 % | 0.25 timed | 142 calls; `speed_pieces` is 11 % of it |
| · the rest of the door | 3 % | | |
| the pcurve mint (`mint_pcurves` → `certify_stated_image` → `certify_projected` → `run_projected_checks`) | 23 % | 0.48–0.53 timed | **8 calls**, one per kept row; check 4's terms 15 %, `projected_hull` 11 %, `frame_box_on` 9 % |
| the result gate's volume (`plus_v_by_sign` → `at_round_zero` → `decide_faces`, main waiting on rayon) | 22 % | 0.56 timed | about 3 of 4 workers busy. Of the workers' samples, `rotational_cut_face_rounds` is 83 %, `length_bound` 13 %, `projected_edge::of_image` 4 % |
| the section table's march (`germ_branch` → `SectionTable::carried` → `march_over_the_ball`) | 9 % | 0.22 timed | **1 march** |
| the rest of the result gate | < 1 % | | |

Cross-cutting, inclusive: `twin_span_hulls` 36 % (56 calls; under limb 2
and under the mint's check 4), the speed meters' `speed_pieces` 15 % (28
computes, 0.46 s timed, in the turn and the mint), `session_memo` 17 %.
Self (leaf) time is spread out. Main-thread futex wait is 22 %, the volume's
wait. The interval rounding kernels (`two_sum_err`, `upper_of`, `make`,
`from_bounds`, `add_lo`, `two_prod_witness`, `signed_corners`) are about
25 %, malloc/free about 11 %, the knot locate in `spline::locate` 5 %, and no
other leaf is over 3 %.

Counted: limb 2 is 40 calls, with 24 memo reads and 16 misses. Locus
certificates are 20 derived and 56 read by support. Row certificates are 56
derived and 8 read. Rung 3 is down from 40 calls on 16 inputs (11-20) to 12
(PR 7193), so what remains of it is first derivations, and limb 2's rounds are
now most of that cost. Its memo no longer serves repeats because none are left.

**Lever → row, and the saving if it landed** (ball ∪ rod, opt 2, 2.35 s):

| stage | share | owning row | estimated saving |
|---|---|---|---|
| rung 3's 12 derivations | 29 % | `work/sect/a-chords-locus-certificate-is-not-its-section-components-cut-to-its-span.md` (P2, design): read the component's certificate instead of deriving again | ≈ 0.55–0.6 s, less option 1's cut charge (small). It removes the 12 rung-3 calls; it does not remove the 16 limb-2 calls outside rung 3 |
| the volume | 22 % | `work/quad/a-ball-and-cylinder-results-volume-is-a-fifth-of-the-boolean.md` (P2) | ≈ 0.25 s for half the rounds; at most ≈ 0.5 s |
| the speed meters | 11–15 % | `work/nurbs/a-curve-cloned-before-its-speed-meters-first-read-computes-them-again.md` (P3): the clones' recomputes only | ≈ 0.13 s (that row's 8 of 18 recomputes); the first computes stay |
| the march | 9 % | `work/sect/a-surface-pairs-march-is-repeated-per-operation-and-per-germ.md` (P3) | 0 within one op (already 1 march); ≈ 0.22 s on a later op on the same pair, only with a cross-operation cache |
| (no stage) | — | `work/sect/a-same-kind-pairs-canonical-order-reads-arena-keys.md` (P3) | ≈ 0: ball ∪ rod has no same-kind pair (sphere × cylinder, plane × cylinder) |
| the pcurve mint's 8 projected certificates | 23 % | **unowned**: three doors became one in `a-fitted-chords-projected-pcurve-certificate-is-taken-at-three-doors-per-operation` (closed), but no row owns the first certificate's cost | — |
| check 2's turn, apart from the clone recomputes | ≈ 8–10 % | **unowned** | — |
| the march itself, within one op | 9 % | **unowned** (its row shares marches; it does not make one cheaper) | — |

**Is the ~0.5 s target reachable through these rows? No.** Suppose all five
land at their best: −0.6 (locus), −0.5 (volume), −0.13 (meters),
0 (march), 0 (order). That leaves ball ∪ rod at about 1.1 s at opt 2, or about
0.95 s in the viewer's build. What would remain is mostly unowned: the mint's
projected certificates (0.5 s), the turn's first speed-meter computes
(≈ 0.2 s) and the march (0.22 s). Getting under 0.5 s also needs one
of those three made cheaper, and the largest is check 4 on a fitted
carrier's row. At 1e-12 the ops are 4.5–5.5× the default ε and the
target is out of sight on any of these levers.
