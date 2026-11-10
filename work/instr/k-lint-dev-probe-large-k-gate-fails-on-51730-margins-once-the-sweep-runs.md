---
id: k-lint-dev-probe-large-k-gate-fails-on-51730-margins-once-the-sweep-runs
kind: issue
title: k-lint (dev-probe): the large-K lint fails on 51,730 crowding margins (24,045 in band) once the probe sweep runs to the end
status: open
opened: 2026-10-29
priority: P2
cost: M
---


Found by GREEN's nightly-76 lane (`work/green/nightly-76-reds-greened.md`).

## What

Nightly 76's `k-lint (dev-probe)` stopped at the K-telemetry sweep's
first ε row, on the probe/f64 key-split pin, as earlier nights had
stopped there or sooner. With that pin and the sweep's two later stops
greened (the lily's 1e-12 stem tube, pinned on
`work/shell/main-red-five-slow-rows-refuse-at-limb-2s-kernel-limit-since-5728.md`,
and the `intent_s3_c_mates` census floor), `scripts/k_probe_sweep.sh`
runs to the end on the GREEN branch (base main `c173123e`). The job's
next gate, `large-K lint`, then fails:

```
k-lint: 1e-6:  3924731 samples, 13028 flagged — rule 1: 9225,  rule 2: 3803,  rule 3: 3803
k-lint: 1e-9:  3957235 samples, 38349 flagged — rule 1: 14820, rule 2: 13398, rule 3: 23529
k-lint: 1e-12: 3561406 samples,   353 flagged — rule 1: 0,     rule 2: 352,   rule 3: 353
k-lint: GATE FAILED — the margin distribution changed: 51730 margin(s) crowd a decision
```

The driver lint (`--gate-rule-1-only`) passes: rule 1 is 0, with 342
advisory flags.

**Rule 1, every in-band sample**, read from the CSVs' `outcome` column:

| ε | shape:predicate | count |
|---|---|---|
| 1e-6 | `demo/lily_walls:curve_pair_walk_foot_clear` | 5532 |
| 1e-6 | `demo/lily_walls:curve_pair_walk_foot_piece` | 3312 |
| 1e-6 | `demo/lily_walls:curve_pair_walk_clear` | 347 |
| 1e-6 | `demo/lily_walls:curve_pair_walk_piece` | 15 |
| 1e-6 | `demo/lily_walls:bool_ray_torus_disc` / `demo/lily:bool_ray_torus_disc` | 14 / 4 |
| 1e-6 | `demo/lily_walls:nurbs_pair_transversality` | 1 |
| 1e-9 | `demo/lily_walls:curve_pair_walk_foot_clear` | 9349 |
| 1e-9 | `demo/lily_walls:curve_pair_walk_foot_piece` | 5181 |
| 1e-9 | `demo/lily_walls:curve_pair_walk_clear` | 290 |

Every in-band sample is the lily's curve-pair walks. They are filed on
their owner as
`work/cpair/lily-walls-curve-walks-decide-24045-margins-in-band-under-k-lint.md`;
the closed CLEAVE row on `lily_walls` was a different predicate.

**The 1e-12 file under-reports.** Its "rule 1: 0", and every other
1e-12 count above, exclude the lily: GREEN's pin stops the `lily` and
`lily_walls` scenes at 1e-12 on the stem's kernel limit
(`work/shell/main-red-five-slow-rows-refuse-at-limb-2s-kernel-limit-since-5728.md`),
so their samples are never recorded there.

**Rules 2 and 3.** The CLI caps its print, so the per-family split
below covers only the printed flags. Each family is rostered against
the row that already carries it:

| shape:predicate (printed, every ε) | row |
|---|---|
| `demo/projectbox_cutaway:bool_pair_gap` (660), `demo/tiltedcut:bool_pair_gap` (168); margins exactly 24·ε | `work/instr/bool-pair-gap-is-eps-coupled-and-flags-74-rows-per-eps-under-k-lint.md` |
| `demo/projectbox_cutaway:curve_pair_pose_{clear,parallel,travel}` (330), `demo/letterforms:curve_pair_pose_tilt` (36) | none found: this row |
| `demo/table:volume_backstop` (4) | `work/tally/table-union-backstop-margin-sits-under-the-k-lint-floor.md` |

## Owed

The lint's own recourse (`docs/K-REPORT.md`): re-derive the baseline
floor and thresholds against a fresh sweep, or demote the row to
advisory with a recorded justification. Nobody has read the margins
yet, so the population may also carry kernel defects; the CPAIR row is
where the largest share gets read.
GREEN cannot pin a distribution, so the job stays red on this step
until INSTR rules. A full count of rules 2 and 3 by family needs the
CLI's uncapped output.

Reproduce (after the GREEN pins land): `scripts/k_probe_sweep.sh
/tmp/k && (cd tools/k-lint && cargo run -- /tmp/k/k-eps-1e-6.csv
/tmp/k/k-eps-1e-9.csv /tmp/k/k-eps-1e-12.csv)`.

## Pinned red (2026-10-29, GREEN)

The nightly's `large-K lint` step pins the gate's verdict, not its
counts:
- k-lint must exit 2, so the gate still fails;
- the per-ε counts are printed in a `::warning`, so drift shows;
- an exit of 0 is red, because it means this row's fix landed and the
  gate should come back;
- any other exit is red, because the tool broke.

**Un-pinning owes restoring the gate:** closing this row puts the step
back as a plain `cargo run` whose non-zero exit fails the job.

**The reading on 2026-10-29** (main `a454950e3` plus GREEN's probe
pins; a reading, not a pin):

```
k-eps-1e-12.csv: 721 flagged — rule 1 (undecided/invalid): 0, rule 2 (near a threshold): 720, rule 3 (below a floor): 721
k-eps-1e-6.csv: 13331 flagged — rule 1 (undecided/invalid): 9225, rule 2 (near a threshold): 4106, rule 3 (below a floor): 4106
k-eps-1e-9.csv: 38717 flagged — rule 1 (undecided/invalid): 14820, rule 2 (near a threshold): 13766, rule 3 (below a floor): 23897
```

These total 52,769, more than the 51,730 in the title, which was
measured before #6023 added four corpus documents. Rule 1 is unchanged
at 24,045.

**The reading on 2026-10-30** (nightly
[41008276467](https://github.com/evgunter/cad/actions/runs/41008276467),
main `39bbdfc777`): exit 2, as pinned, but the step went red anyway —
the runner's `bash -e` exited on the pipeline before the `case` read
the status. The nightly triage of 2026-10-30 added `set +e` to the step.

```
k-eps-1e-12.csv: 721 flagged — rule 1 (undecided/invalid): 0, rule 2 (near a threshold): 720, rule 3 (below a floor): 721
k-eps-1e-6.csv: 3780 flagged — rule 1 (undecided/invalid): 1884, rule 2 (near a threshold): 1896, rule 3 (below a floor): 1896
k-eps-1e-9.csv: 5177 flagged — rule 1 (undecided/invalid): 1512, rule 2 (near a threshold): 2948, rule 3 (below a floor): 3665
```

9,678 in all, down from 52,769; rule 1 is 3,396, down from 24,045.

## The volume backstop left the operation path (2026-10-31)

PR 5842, under `[ev]` PR 5780, deleted the volume backstop: `demo/table:volume_backstop`'s four flags cannot recur, and their row (`tally/table-union-backstop-margin-sits-under-the-k-lint-floor`) is closed. The floor those margins sat under is `k-lints-definite-floor-is-read-off-a-retired-predicate`.

## Revalued (2026-11-09, `onecopy/main-reds-k-and-m10`)

The sweep's dump half, run to its end on main (merged 2026-11-09) plus that branch (corpus, M2 corpus, E6 driver and demo scenes at all three ε; the demo tour's `probe` build fixed on the way), linted as the nightly lints it:

```
k-lint: 1e-6:  4162991 samples, 4159 flagged — rule 1: 2167, rule 2: 1992, rule 3: 1992
k-lint: 1e-9:  4149904 samples, 5171 flagged — rule 1: 1830, rule 2: 2565, rule 3: 3341
k-lint: 1e-12: 3693422 samples,  703 flagged — rule 1: 0,    rule 2: 703,  rule 3: 703
k-lint: GATE FAILED — the margin distribution changed: 10033 margin(s) crowd a decision
```

51730 crowding margins on 2026-10-29 are 10033 now; rule 1 is 3997 (24045). The large-K gate still fails, as its nightly step pins (exit 2). Rule 1 by shape and predicate at 1e-9: `demo/lily_walls:bool_spline_region_carrier` 720, `demo/lily_walls:curve_pair_walk_foot_clear` 522, `demo/lily_walls:curve_pair_walk_foot_piece` 270, then `curve_pair_pose_parallel` on `lily` (72), `lily_walls` (62), `projectbox_cutaway` (48), `axis_projection` (40); at 1e-6 `bool_spline_region_carrier` is 1106. The lily's curve-pair walks are still the bulk of it, now beside the spline region's carrier read. The 1e-12 file still excludes the lily past its pinned stem.

The driver lint (`--gate-rule-1-only`) passes: rule 1 is 0, with 342 advisory flags (108 / 117 / 117), the same 342 as before.

## Read (2026-11-10, `instr/klint-large-k`)

**What was measured.** Main `9fe94da35`, running
`scripts/k_probe_sweep.sh`'s dump half.

- The plain half stops at `assemble_one_local_battery`, which
  `work/onehome/main-red-the-gather-records-one-sample-tier-3-prime-does-not.md`
  tracks.
- The large-K lint's flags match the 2026-11-09 reading exactly: 4159
  / 5171 / 703, with rule 1 at 2167 / 1830 / 0.
- The sample counts have grown, to 4 271 646 / 4 255 332 / 3 767 989.
- The driver lint still reports 342 advisory flags.

**Every flag, uncapped.** `k_lint::lint_csv`'s own flags, so no rule
was re-implemented. They fall into three populations:

| population | names | 1e-6 r1 / r2·3 | 1e-9 r1 / r2·3 | 1e-12 r2·3 |
|---|---|--:|--:|--:|
| A: ε-coupled pads, \|m\| = Kε, 2Kε or 24·ε | `bool_pair_gap`, `curve_pair_pose_{clear,parallel,tilt}` | 279 / 1164 | 318 / 1125 | 703 |
| B: refinement walks, \|m\| from ε to 3.9e-5 | `curve_pair_walk_foot_{clear,piece}`, `bool_spline_region_carrier` | 1862 / 754 | 1512 / 2118 | — |
| C: lily fine features, 3 µm to 38 µm, ε-independent except `bool_ray_torus_disc` | `plane_crossing_belly_graze`, `link_wedge`, `tangent_sector_order2`, `bool_conic_quadric_sub_clear`, `bool_ray_torus_disc` | 26 / 74 | 0 / 98 | — |

The r2·3 columns count flags. At 1e-9 rule 2 is 2565 and rule 3 is
3341; elsewhere the two are equal. The lily is not recorded at 1e-12.

**Where each population goes.**

- **Rule 1, by family:**
  - the walks' feet: `work/cpair/lily-walls-curve-walks-decide-24045-margins-in-band-under-k-lint.md`;
  - the carrier walk: `work/splice/the-spline-region-carrier-walk-records-in-band-margins-by-construction-under-k-lint.md`;
  - the pose screens: `work/cpair/the-pose-screens-clear-and-parallel-record-minus-k-eps-and-land-in-band-under-k-lint.md`;
  - the lily's micron features at 1e-6: `work/inside/a-lily-wall-ray-reads-the-torus-quartic-disc-in-band-at-1e-6.md`,
    `work/germ/a-lily-wall-conic-quadric-sub-clearance-of-3-microns-is-in-band-at-1e-6.md`
    and `work/splice/a-lily-wall-tangent-sector-order2-margin-of-8-microns-escalates-at-1e-6.md`.
- **Rules 2 and 3:**
  - A goes to `bool-pair-gap-is-eps-coupled-and-flags-74-rows-per-eps-under-k-lint`'s rule (4) fork;
  - B goes to `refinement-walks-flag-rules-2-and-3-by-construction-in-the-gated-sweep`;
  - C is the stale floor of `k-lints-definite-floor-is-read-off-a-retired-predicate`.
    The lily, added after M7, has features at 29 µm and 38 µm, under the
    4e-5 floor.

**What re-deriving the thresholds can reach.** Rule 1 reads only the
`outcome` column, so no k-lint threshold reaches it. No metre floor
reaches A, and B fills every decade from `Kε` upward. A floor re-cut
now would clear C's 74 definite rows at 1e-6 and at 1e-9; that is 148
of the 6036 flags under rules 2 and 3, and the step would still exit 2.

## Routed (2026-11-10)

The ruling is the split. Rule 1 stays with the owners of the deciding
sites. Rules 2 and 3 go to the rulings named below. No threshold is
re-cut at this reading: `docs/K-REPORT.md`'s INSTR addendum
(2026-11-10) gives the four reasons. The nightly's `large-K lint` step
stays pinned red at exit 2, unchanged.

**Demoted P1 → P2.** The step is pinned, not red. Closing this row waits
on the rows below, so INSTR has no P1 work left on it.

The row closes when all of these hold:

1. **Rule 1 is 0 at all three ε rows.** Its owners' rows:
   - `cpair/lily-walls-curve-walks-decide-24045-margins-in-band-under-k-lint`;
   - `splice/the-spline-region-carrier-walk-records-in-band-margins-by-construction-under-k-lint`;
   - `cpair/the-pose-screens-clear-and-parallel-record-minus-k-eps-and-land-in-band-under-k-lint`;
   - `inside/a-lily-wall-ray-reads-the-torus-quartic-disc-in-band-at-1e-6`;
   - `germ/a-lily-wall-conic-quadric-sub-clearance-of-3-microns-is-in-band-at-1e-6`;
   - `splice/a-lily-wall-tangent-sector-order2-margin-of-8-microns-escalates-at-1e-6`.
2. **The lily is recorded at 1e-12.** Until then that row is silent on
   the lily, so "0 at all three rows" is not yet true of it:
   `shell/main-red-five-slow-rows-refuse-at-limb-2s-kernel-limit-since-5728`.
3. **A, the ε-coupled pads, is ruled.** This is the rule (4) fork on
   `instr/bool-pair-gap-is-eps-coupled-and-flags-74-rows-per-eps-under-k-lint`,
   and the pose names join it.
4. **B, the refinement walks, is ruled:**
   `instr/refinement-walks-flag-rules-2-and-3-by-construction-in-the-gated-sweep`.
5. **C's floor is re-cut.** `instr/k-lints-definite-floor-is-read-off-a-retired-predicate`
   re-derives it at a new committed era that a fresh sweep at a stated
   head byte-reproduces, after 1 and 2.
6. **The step is restored.** It goes back to a plain `cargo run` whose
   non-zero exit fails the job, as "Un-pinning owes" above says.

Until then, a new rule-1 family landing inside the pinned count is
invisible:
`instr/the-large-k-pin-cannot-see-a-new-rule-1-family-inside-its-count`.
