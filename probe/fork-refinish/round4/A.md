ROUND 4, A. I move to B, and I do not expect to move again: my round-3 reasons do not survive the check.

1. **B's facts are right** (checked in `drive.rs` and `sym.rs`).
   - `serialize()` writes `decisions symbolic= numeric= frozen=` (plus the refusal and retry columns when they are non-zero), and `content_key` hashes `serialize()`. So the row is part of the verdict's identity.
   - The leaves' own counts are not in `serialize()`. The sum can be derived in process from `certified()` and `refused()`. A count of distinct results cannot be derived, because the keys are not shipped.
   - `SymCounts::absorb` sums `symbolic` and `numeric`. `frozen` is "not summed: it is a set … the driver writes the drive's own". So the row mixes two meanings today.
   - **The report's share** ("N of T decisions were proven zero by the symbolic tier"), under the sum, weights each decision by how many leaves happened to replay it. That mixes the document with the bisection's depth: the tour's plate counts 1,023 times. Under the distinct count it is the share of the drive's own decisions that the tier proved. E12's evidence ("the ratio of symbolic to numeric decisions is itself the E12 evidence") is about the tier on the document, so it is the distinct count.
   - **The pins:**
     - `m10_8_harness`: one leaf per drive, so both definitions give the same number and the pin does not move.
     - `m10_9_pins_interval`'s `open >= shut` holds under either definition (likely).
     - `m10_sym_drive_memo_interval`'s on = off holds under either, since both are defined by the tree.
     - Multi-leaf pins re-baseline once.
   - **My round-3 reasons fail.** "Nothing re-baselines" is a cost of the change, which `designer.md` §4 says to disregard. "The distinct count has no reader" was wrong: the readers of the drive's total would be reading it.
2. **Final: the distinct count.** The drive's `decisions()` counts each distinct node result once, defined by the tree, so it is the same with the store off (the driver deduplicates by key) and under any schedule. Leaf receipts stay as agreed. There is no separate sum field: anyone can rebuild the sum from the leaves. I would pick this if the choice were mine alone.
3. **Yes, they should agree, and this makes them agree.** Every column of the drive's row then means "distinct over the drive": `frozen` counts distinct expression nodes, and the other columns count distinct node results. Each is a set taken once, and the comment on `absorb` stops being an exception.

For Ev: the "Receipts" paragraph is B's round-2 version except for its second sentence, which becomes: "The drive's total counts each distinct node result once, as its frozen column already counts each frozen expression once; both are unchanged by the memo." (sure)
