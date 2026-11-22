Round 4 from B, the drive's total: both sets of facts are right; I would accept either definition; mine alone would be the distinct count, and it is the only one that makes the row's columns agree.

**1. The facts.**
- A's are right. The report line is "{symbolic} of {total} decisions were proven zero by the symbolic tier ({share}%)" (`ParamBoxVerdict::render`). `m10_8_harness` reads `v.decisions()` beside `receipt().certified == 1`; `m10_9_pins_interval` compares it across a dial (`open >= shut`, `off != on`); `m10_sym_drive_memo_interval` compares it across the memo dial and the schedule. Each reads a tripwire on reach or an invariance. No reader in the tree needs a distinct count as such.
- Mine are right too: `content_key` hashes `serialize()`, which writes the line; `frozen` is the distinct nodes frozen over the drive while `symbolic` and `numeric` are sums (`SymCounts::absorb`, "`frozen` is not summed").
- What the share means. Under the sum: of the decisions in all the leaves' certificates, counting a decision once per leaf it appears in. On the tour, the plate's decisions weigh 1,023 times and a hole's about 8. Under distinct: of the decisions the drive's answer rests on, each once. E12's evidence ("the ratio of symbolic to numeric decisions is itself the E12 evidence") is about how much of the decision population the tier discharges. The sum's ratio moves with the shape of the bisection tree, for a document whose decisions did not change; the distinct ratio does not.
- Several of the pins are of one-leaf drives (the m10_8 harness asserts one certified leaf), where the two definitions are the same number. The re-baseline is smaller than "every pinned total".

**2. Which, final.** Either is sound and I would not hold the design for it. If it were mine alone: each distinct node result once. Reasons, in order:
- it is the number only the drive can state; the sum is in the shipped leaves;
- the share then says something about the document and the tier, not about how often a node was repeated;
- point 3.
A's "a distinct count has no reader, `[simple.checks]`" would bind if this added a column. It does not: the row exists either way and the choice is what it means. "Nothing re-baselines" is a cost of the change, which the designers' rules set aside.

**3. Should the columns agree.** Yes: one row, one name, one definition. Under the sum they do not agree, and cannot be made to: `frozen` cannot become a sum, because the leaves' frozen sets overlap, which is why it was made distinct. Under distinct they agree in kind: every column is "over the drive, each once". They still differ in unit (`symbolic` and `numeric` count decisions over distinct node results; `frozen` counts distinct expression nodes), and the column docs should say so.

If it goes to Ev as a choice, my statement of it: "The drive's `decisions` line: the sum over leaves (no pin moves; the share is weighted by how often a node repeats; `frozen` stays the one column defined differently), or each distinct node result once (pins of multi-leaf drives re-baseline once; the share is of distinct decisions; all columns are per drive, each once). Both are unchanged by the store and the schedule."
