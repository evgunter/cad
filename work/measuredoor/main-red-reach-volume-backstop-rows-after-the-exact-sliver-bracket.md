---
id: main-red-reach-volume-backstop-rows-after-the-exact-sliver-bracket
kind: issue
title: Main red (slow set): two reach_volume_backstop rows fail since PR 5734's exact cylinder-sliver bracket
status: closed
opened: 2026-10-26
priority: P1
cost: M
closed: 2026-11-11
branch: measuredoor/sliver-bracket-close
pr: PRNUM
---


## What

Two slow-set rows in `crates/sweep/tests/reach_volume_backstop.rs`
fail on `origin/main` (reproduced at `2f9e6f9e`). Slow sets do not run
in hosted CI, so the PR gate missed them. Found by SPLICE's
slow-set triage lane (branch `splice/slow-set-reds`).

**Bisect** (`git bisect run --first-parent`, both rows):
- first bad: `ee4a094e`, the merge of PR 5734
  (`flux/cylinder-sliver-exact-bracket`);
- last good: `4bc935de`, its first parent.

**Failures, verbatim:**

- `a_result_equal_in_truth_but_larger_at_the_midpoint_passes`,
  `reach_volume_backstop.rs:492` (the fixture's premise):

      the fixture's premise: the two rods' midpoints differ (2.7488935718910685, 2.7488935718910685)

  Both rods now read the same midpoint, which is `3.5·π·0.25²`
  (2.748893571891069) to the last bit. The row needs two readings
  with different midpoints for its test of the enclosure arithmetic's
  direction, and it has lost that witness.
- `planted_subtraction_results_refuse_down_to_the_stated_resolution`,
  `reach_volume_backstop.rs:392`:

      assertion `left == right` failed: a fattening by δ = ε/100 is below the stated resolution
        left: Err(ResultVolumeImplausible)
       right: Ok(Validated)

  The `1e-3`, `100·ε` and `ε` plants still refuse, and the right
  answer still passes. The `ε/100` plant is now resolved and refused
  as well, so the open range `last_round_open` states (`1.2e-13·s`)
  no longer matches what the backstop does on this pair.

## What is owed

Decide, on FLUX's ground, whether the new readings are right. If they
are, re-meter `last_round_open` and its doc and give the midpoint row
a pair whose midpoints still differ. If they are not, fix the bracket.

## Update 2026-10-27: two duplicates folded in

Two later rows filed the same failure, and both closed into this one:
`main-red-reach-volume-backstop-midpoint-premise-and-planted-subtraction`
(PCERT E1b's suite round, PR 5797; red on `be2b7a139c` and
`2f9e6f9e39`) and `main-red-5734-moves-tilted-cut-upper-and-two-reach-backstop-rows`
(a RESTREAD triage lane on `9d97abd704`, the same bisect). The latter's
third row, `tilted_cut_upper` in the thread-count digest, has since been
re-cut by QUAD (PR 5818), so the two backstop rows are all that is left red.

## 2026-10-27 — the same merge moved a step-export sidecar (nightly triage)

`step-export` `kernel_sidecars::kernel_sidecar_fields_match_live_kernel`
went red at the default ε on nightly run 40455870959 (main
`fb476030a9`). Bisected over first-parent merges from `38f1a24ed3`:
first bad `ee4a094e` (PR 5734), last good `4bc935de`. `cut_cylinder`'s
midpoint moved from `3926990816.9872403` to `3926990816.987241`
(`π·1.25e9` is `3926990816.9872413`) and its pad narrowed from
`1022.98` to `1022.37` mm³: the same tightening as the rows above, the
analytic volume inside both enclosures. The triage branch
(`triage/nightly-2026-10-27`) re-syncs the sidecar to the live kernel;
if this row's decision reverts the bracket, the sidecar follows it.

## 2026-10-28 — pinned on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`. Both rows red at the default ε and 1e-6, green at 1e-12. Each is
pinned at the default ε and 1e-6 on its message (`the fixture's premise:
the two rods' midpoints differ` and `a fattening by δ = ε/100 is below
the stated resolution`); each pin goes red the day its row passes.

Un-pinning owes a re-measure: each pin stops its row at main's first stop, so whatever runs after it in the row is unmeasured.

## The two backstop rows left with the backstop (2026-10-27)

PR 5842, under `[ev]` PR 5780 (the boolean's volume relations are a test oracle, `topo::test_support::volume_relations`), deleted the volume backstop. `reach_volume_backstop.rs` is now `curved_trims_build_at_their_volumes.rs`: the midpoint row tested the backstop's enclosure arithmetic and is deleted with it, and the planted subtractions are oracle rows (`planted_subtraction_results_break_the_volume_relations`: 1e-3 and 100ε fattenings flagged, the right answer and ε/100 held), green at 1e-9, 1e-6 and 1e-12. Neither row is red any more, and their pins go with them. What stays here is whether PR 5734's tightened readings are right, and the step-export sidecar that follows them.

## 2026-11-11 — the sidecar is red again on main

On main `5e01d02d2a6`, `step-export` `kernel_sidecars::kernel_sidecar_fields_match_live_kernel` fails at the default ε. It is in the fast set:

    cut_cylinder: want KERNEL_VOLUME_PAD_MM3=1022.370482199146, sidecar has ["1022.3704888604841"]

The pad narrowed again by about 6.7e-6 mm³, after the re-sync in `7487300d562` (2026-10-27). In the same run, `curved_trims_build_at_their_volumes` passes at the default ε, including `planted_subtraction_results_break_the_volume_relations`.

## Dispatch

2026-11-11, MEASUREDOOR orchestrator. One implementer, lane `mdoor-sidecar`, branch `measuredoor/sliver-bracket-close`. The lane will:
- bisect the commit that moved `cut_cylinder`'s pad after `7487300d562`;
- decide by meaning whether the new pad is right;
- decide this row's open question: are PR 5734's tightened readings right? QUAD's PR 5818 already checked `tilted_cut_upper` against its closed form, in `work/quad/tilted-cut-upper-digest-moved-by-5734.md`;
- then re-sync the sidecar, or fix the source.

Review tier: the orchestrator's read if the outcome is a re-sync with the analytic volume inside the new pad. If the lane changes kernel code, a single full review.

## Closed (2026-11-11)

**PR 5734's readings are right.** `quad::trig_at_limit` (`crates/geom-brep/src/props/trig.rs`) brackets `(cos a, sin a)` at `a = limit(t)`, the midpoint of the parameter bracket. That is the same limit `TrimEdgeQ::limits` integrates from (`crates/geom-brep/src/props/quad/rotational.rs`), and `topo::props::quad_lane` forms both ends' brackets with it (`crates/topo/src/props/quad_lane.rs`, the `EdgeTermQ::Harmonic` arm). The distance from that limit to the true end is paid by the endpoint-bracket pad, `|f|·(w(t₀) + w(t₁))`, and at a junction by `end_slack`'s bracket half-width times the edge's speed. Before 5734, the bracket was taken at the endpoint vertex and widened by `ε/r`, so it bracketed a different point, more loosely. `trig_at` itself is sound:
- its seed is halved exactly to `|s| ≤ 0.05`;
- the seed's alternating-series truncations hold for `s² ≤ 56`;
- the double-angle steps and the rotation are outward-rounded interval arithmetic;
- the `[−1, 1]` clamps hold the true values.

The oracles hold. `tilted_cut_upper` is within its new brackets at 1e-6, 1e-9 and 1e-12 (QUAD, PR 5818: `|v − V|` at most 9.6e-16 against `vpad` of at least 2.48e-10). `cut_cylinder`'s `π·1.25e9` mm³ (`EXPECT_VOLUME_MM3`) is inside every pinned enclosure: within 4.8e-7 mm³ of the midpoint, against a pad of 1022 mm³ at ε = 1e-9, and by overlap at 1e-6 and 1e-12. The planted subtractions, now oracle rows in `curved_trims_build_at_their_volumes.rs`, pass.

**The sidecar.** `KERNEL_VOLUME_PAD_MM3` in `crates/step-export/tests/fixtures/cut_cylinder.expect` goes from `1022.3704888604841` to `1022.370482199146`, the field only, at the default ε. `kernel_sidecar_fields_match_live_kernel` passes at 1e-9, 1e-6 and 1e-12. Two merges moved it after `7487300d562` (merged in PR 5845), each found by a first-parent bisect:
- **PR 6478** (`898df16c9c`, the rotational lane, stage 1): the pad fell to `1022.3704820511159`, and the midpoint rose 9.5e-7 mm³ to `3926990816.9872417`. Per wall face, the flux pad fell about 2.2e-14 m³. On each wall face, the bottom rim's term (`u` linear, `v` flat) was a 64-piece composite sum, 2.6e-14 wide from rounding alone. Now `harmonic_edge_integral` reads it exactly as `p.c0·u.cl·span`, 8.9e-16 wide. The same merge adds the chart-term pad, `chart_term_pad`, 2.2e-15 per face, where the old lane's `o·A⃗` was a point. No pad term went missing: the remainder (3.0671112e-6), the map-residual pad (4.57e-16 per curved edge) and the endpoint pad (0, point brackets) read the same bits before and after.
- **PR 6509** (`e2435632d8`, the rotational lane, stage 2): the pad rose to `1022.370482199146`, and the midpoint returned to `3926990816.987241`. Every junction's connector, 1.44e-16 wide, enters the region term, and the chart-term pad grows to 2.40e-15 from its corner term. It is a pad added, and it widens.

Both enclosures contain the truth, and each change has a reason that holds: a rim read exactly, and a term added.

**A premise corrected.** `kernel_sidecar_fields_match_live_kernel` is in the `ci` profile's slow set (`.config/nextest.toml`, the `step-export::all` entry, since `fd62884037`), not the fast set. Neither 6478 nor 6509 seeded step-export, so neither gate ran the test. The count is added to `work/ciw/dependents-slow-sets-skip-a-kernel-pr.md`.
