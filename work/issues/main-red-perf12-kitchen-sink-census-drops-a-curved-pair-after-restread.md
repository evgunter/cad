---
id: main-red-perf12-kitchen-sink-census-drops-a-curved-pair-after-restread
kind: issue
title: Main red: perf12's kitchen_sink census reads 28 findings, the curved pair (face 47, face 21) cleared since #5817, golden unblessed
status: closed
opened: 2026-10-27
closed: 2026-10-28
---


Found by INTENT stage 3 E (`intent/e-operation-frame-of-reads`) over its
merge of main `39134695`, and measured on a clean `39134695` worktree
with every source rebuilt: `perf12_census_goldens::the_at_rest_census_verdicts_are_goldened_bit_exact`
(`crates/editor-core/tests/perf12_census_goldens.rs:225`) reds at line 49
of `crates/editor-core/tests/golden/perf12_census_1e-9.txt`. Blessing
shows the whole drift: `kitchen_sink`'s census reads 28 findings where
the golden holds 29, the one gone being

    CensusUndecidable { a: Face(FaceKey(47v1)), b: Face(FaceKey(21v1)), … reason: MayOverlap }

(the curved-within-reach arm, `crates/topo/src/census.rs` near `:8688`).
Main at `843fc228`, one merge earlier, passes; `39134695` merges PR 5817
(`restread/curved-pairs-apart`), whose arm 1 now reads a curved face's
offset and held reach (`crates/topo/src/census/depth_gap.rs`), so the
pair is cleared. It is not E's: E's dump of the product's points and
surfaces is main's (`843fc228`) but for one plane normal's sign (`-0.0`), and E reads
the same 28 findings.

Done when the cleared pair is judged right and the golden re-blessed at
every ε row it moves: 1e-12 reds the same way on a clean `39134695` (at line 49, the census count; on E the same pair is the one gone); 1e-6 was not measured here.

## 2026-10-27 — still red at `fd74ba95`

Main's at-rest-check merge re-blessed the census goldens (an `at rest:`
outcome line above the census) but not this pair: on a clean
`fd74ba95` the row reds at line 48 (eps=1e-9), the outcome line's count
(29 → 28 on E, which also drops the census count and the same pair).

## Closed (INTENT, 2026-10-28)

Duplicate of `work/restread/main-red-perf12-census-golden-kitchen-sink-loses-a-curved-pair-at-5817.md` (P1, RESTREAD, which owns the cause, PR 5817), as found by the 10-28 nightly triage (PR 5917). Read that row.
