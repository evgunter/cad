---
id: the-clearance-determinism-row-runs-under-a-thousand-cells-since-6902
kind: issue
title: The clearance determinism row runs on 992 cells since PR 6902, under the thousands it wants: pinned at 992
status: open
opened: 2026-11-14
priority: P3
cost: C
---


Found by GREEN's `green/goldens` lane, from nightly 43371638371 (main `e36675c1`), red at all three ε rows.

**What failed.** `crates/editor-core/tests/m10_5_clearance_interval.rs` `the_answer_is_deterministic_across_repeats` asserted `cells > 1_000` on the tube's walls at 0.3 against their 0.5 ("the determinism row runs on a multi-thousand-cell subdivision"). Main reads `CellReceipt { candidates: 1, discharged: 992, violated: 0, within: 0, refused: 0, splits: 991, outside: 344, .. }`, the same at 1e-6, 1e-9 and 1e-12.

**Bisected, first-parent, at default ε** over `9a256348..e36675c1`: the row passes at `aa9c1c9bb` and fails with this receipt at `904dcef4b`, the merge of PR 6902 (`clear/analytic-support`, "the support test reads an analytic cell's support, not its box"), which is the next first-parent commit. PR 6902's description does not mention the row.

**What GREEN did.** The floor is pinned as `assert_eq!(cells, 992)`, with this row named in the file, so the suite is green and the count reds on its next move. The three repeats and their bit-equality still run.

**Owed.** The row's claim is that the answer is stable on an expensive run, thousands of cell pairs. A tighter support test discharging the pair sooner is the kernel getting better, so the fixture no longer asks a hard enough question: give the row a bound that still refines to thousands of cells (the tube at a clearance nearer its 0.5, say) and restore the floor, or say why about a thousand is enough and make it one.

Reproduce: `cargo nextest run -p editor-core -E 'test(the_answer_is_deterministic_across_repeats)'` (slow set; revert the pin to see the floor).

**The pin reds on the next improvement too.** `assert_eq!(cells, 992)` goes red whenever clearance discharges this pair in a different number of cells, which a tighter support test will do again. That red is the pin, not a defect: re-take the count, or better, do what this row owes and take the count out of the determinism row (a floor bolted onto another row's claim is what `docs/prompts/implementer-discipline.md` §8 says not to do; the pin is there only so the row is not vacuous in the meantime).
