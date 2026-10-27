---
id: main-red-5734-moves-tilted-cut-upper-and-two-reach-backstop-rows
kind: issue
title: Main red (sweep slow set): PR 5734 moves tilted_cut_upper in the thread-count digest at the default eps and 1e-12, and reds two reach_volume_backstop rows at the default eps
status: open
opened: 2026-10-27
priority: P1
cost: M
---


**Main red.** Measured by a restread triage lane on `origin/main` at
`9d97abd704` (2026-10-27), `sweep`'s `all` binary built with
`--release`, each row run alone under `--profile ci
--ignore-default-filter`. All three rows are in the **slow set**
(`.config/nextest.toml`'s `ci` default-filter leaves them out), so the
per-PR gate runs them only for a diff that seeds sweep's slow tests, and
the nightly.

| row | default ε | 1e-12 |
|---|---|---|
| `mass_props_are_thread_count_invariant::the_walk_matches_the_serial_golden_at_four_threads` | red | red |
| `reach_volume_backstop::a_result_equal_in_truth_but_larger_at_the_midpoint_passes` | red | green |
| `reach_volume_backstop::planted_subtraction_results_refuse_down_to_the_stated_resolution` | red | green |

**The digest** (`crates/sweep/tests/mass_props_are_thread_count_invariant.rs:418`).
Only the `tilted_cut_upper` line moves, and its verdict hash does not.
The pads shrink and the midpoints move a few ulps:

| ε | committed (`thread-count-digest/eps-*.txt:3`) | measured |
|---|---|---|
| 1e-9 | `v=…2d18 a=…c6c9 vpad=3e812c48b2000000 apad=3ea9c26d0ae00000` | `v=…2d1b a=…c6cb vpad=3e81270c3c2aaaac apad=3ea9ba925a200000` |
| 1e-12 | `v=…2d07 a=…c6bd vpad=3df11376aaaaaaac apad=3e199d31c0000000` | `v=…2d07 a=…c6bc vpad=3df110d915555556 apad=3e19994560000000` |

**The backstop rows.**

```
reach_volume_backstop.rs:492: the fixture's premise: the two rods' midpoints differ (2.7488935718910685, 2.7488935718910685)
reach_volume_backstop.rs:392: a fattening by δ = ε/100 is below the stated resolution
  left: Err(ResultVolumeImplausible)   right: Ok(Validated)
```

The first is a fixture premise: the three-arc rod and the circle rod now
measure to the same midpoint, so the row no longer tests what it says.
The second is a wrong refusal: a result fattened by ε/100 is refused
as implausible.

**Culprit: PR 5734** (`flux/cylinder-sliver-exact-bracket`, merge
`ee4a094e9c`), by a first-parent bisect on these rows at the default ε.
`ee4a094e9c` fails all three, and its first parent `4bc935de4f` passes all
three. Other points measured: `1ea8d732c5` green, `7eb29ffbe0`
red. The PR changes `crates/topo/src/props.rs` and
`crates/topo/src/props/quad_lane.rs`. The previous digest rows
(`work/tcost/tilted-cut-upper-pads-moved-in-the-thread-count-digest-at-1e-12.md`,
`work/tcost/the-thread-count-digests-sym-frozen-counts-moved-on-main.md`)
are closed and describe older moves, so they do not cover this one.

**Done when** all three rows are green on main. If the tighter pads are
right, re-cut the digest at all three ε and say what moved. Re-pose the
three-arc fixture so its premise holds again, and do not loosen its
assertion. Then find why the planted ε/100 fattening refuses, and fix it
at its source.
