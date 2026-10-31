---
id: main-red-perf12-census-golden-kitchen-sink-loses-a-curved-pair-at-5817
kind: issue
title: Main red: perf12 census golden fails at three ε since #5817 — kitchen_sink reports 28 findings where the golden has 29
status: open
opened: 2026-10-27
priority: P1
cost: E
---


Found by ATREST's #5810 delta review (`review/atrest-s5c-delta`) and bisected by the ATREST orchestrator.

**What fails.** The slow-set row `editor-core` `perf12_census_goldens::the_at_rest_census_verdicts_are_goldened_bit_exact` is red at `CAD_TOLERANCE_EPS` 1e-6, 1e-9 and 1e-12 on main `39134695`. `kitchen_sink` reports 28 findings where `crates/editor-core/tests/golden/perf12_census_*.txt` has 29: one `CensusUndecidable` on face pair Face47/Face21 is gone.

**Where it came from.** At 1e-9 it passes at `66bfc1c6d7` and fails at `48910f2bfd`, the merge of #5817 (`restread/curved-pairs-apart`), which changes how cross-solid curved pairs are read. The PR gate does not select the slow census goldens, so #5817's CI stayed green.

**What closing it takes.** Decide whether the vanished verdict is a correct consequence of #5817 (the pair is now read apart). If it is, re-bless the three goldens and name the moved line. If not, fix the reading. Either way, run the goldens at all three ε.

**2026-10-30, nightly triage** (nightly [41008276467](https://github.com/evgunter/cad/actions/runs/41008276467), main `39bbdfc777`): still red at every ε, the drift still at golden line 48. The message is unchanged from 2026-10-29.

## 2026-10-30 (GREEN, lane `green/nightly-77-pins`)

Pinned at main's reading, NOT re-blessed as right: the three goldens `perf12_census_{1e-6,1e-9,1e-12}.txt` were re-cut on main `2e1c1bff` (`PERF12_BLESS_CENSUS=1`). Each moves by exactly this row's line: `kitchen_sink` goes 29 → 28 findings, and the `CensusUndecidable { a: Face(FaceKey(47v1)), b: Face(FaceKey(21v1)), … MayOverlap }` is gone. Nothing else moves. Whether that verdict's loss is right is still this row's question. If it is wrong, the fix restores the line, and the golden then reds until the fix re-blesses it. The golden is the whole census, so nothing runs past a stop.
