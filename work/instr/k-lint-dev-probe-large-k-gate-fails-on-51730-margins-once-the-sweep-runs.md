---
id: k-lint-dev-probe-large-k-gate-fails-on-51730-margins-once-the-sweep-runs
kind: issue
title: k-lint (dev-probe): the large-K lint fails on 51,730 crowding margins (24,045 in band) once the probe sweep runs to the end
status: open
opened: 2026-10-29
priority: P1
cost: M
---


Found by GREEN's nightly-76 lane (`work/green/nightly-76-reds-greened.md`).

## What

Nightly 76's `k-lint (dev-probe)` stopped at the K-telemetry sweep's
first ε row, on the probe/f64 key-split pin, as earlier nights had
stopped there or sooner. With that pin and the sweep's two later stops
greened (the lily's 1e-12 stem tube, pinned on
`work/offset/main-red-five-slow-rows-refuse-at-limb-2s-kernel-limit-since-5728.md`,
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
(`work/offset/main-red-five-slow-rows-refuse-at-limb-2s-kernel-limit-since-5728.md`),
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
