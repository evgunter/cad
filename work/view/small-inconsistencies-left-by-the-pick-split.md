---
id: small-inconsistencies-left-by-the-pick-split
kind: issue
title: six small inconsistencies the pick.rs split left in the two new modules and its own prose
status: closed
opened: 2026-09-06
refs: [2079]
closed: 2026-09-06
pr: 2079
---



Found by the style review of #2079. Each is individually trivial; they
are one file because they share a cause — a 2,792-line file cut in two
without a read of either half's small details.

1. **`pick.rs` spells `fmt` two ways, 250 lines apart in a 451-line
   file.** `pick.rs:146-147` is `impl std::fmt::Debug`;
   `pick.rs:397-398` is `impl core::fmt::Display`. `pickindex.rs` is
   `core::fmt` at all five of its sites. Confidence `sure`.

2. **`Generation`'s own doc still speaks as if it lived in a seam.**
   `generation.rs:16` — *"the **seam's own** monotone counter"* —
   under a module header that opens *"A request's identity across the
   **two** seams"* (`:1`). The type doc was true in `evalseam`;
   it is the sentence the move should have re-pointed. Confidence
   `sure` on the mismatch, `unsure` whether it is worth a word.

3. **`pickindex` links `Generation` through the façade, not its
   module.** `pickindex.rs:42` — ``The key is [`crate::Generation`]``
   — while `evalseam.rs`, `pick.rs` and `app.rs` all name
   `crate::generation::Generation`. A doc link through `lib.rs`'s
   re-export points a reader at the façade rather than at the leaf the
   PR just argued for. Confidence `sure`.

4. **`BTreeSet` is written fully qualified nine times** in
   `pickindex.rs` (`:868, :880, :2080, :2104, :2114, :2128, :2129,
   :2143, :2146`) while `BTreeMap` is imported at `:53`. Pre-existing;
   conspicuous now that the file is a fresh one. Confidence `sure`.

5. **"Six readers", seven named.** `crates/viewer/README.md:638` says
   *"six modules compare one"*, which is right — `pick`, `pickindex`,
   `app`, `frame`, `evalseam`, `session` are the six that import
   `Generation`. The PR body and
   `index-seam-vocabulary-sits-in-the-wrong-module.md:225-226` both
   expand it as *"(`pick`, `pickindex`, `app`, `lib`, `frame`,
   `evalseam`, `session`)"* — seven, with `lib` in it, which only
   re-exports. Confidence `sure`.

6. **Line counts off by one in both directions.** The PR body says
   `pickindex.rs` is 2,382 lines and `pick.rs` 452; `wc -l` gives
   2,383 and 451. Confidence `sure`; consequence, none — noted because
   a count that is wrong in both directions is usually two different
   measurements and worth knowing about before one is cited.

Also worth recording without being a defect: `pick.rs` carries no
`mod tests` at all. The four unit tests in the old file were
`PartWindows`/`IdMap`'s and travelled to `pickindex.rs:2211-2383`
unedited, as #2079 says; `PickCache`'s coverage is entirely in
`crates/viewer/tests/frame_policy.rs` and `tests/eval_seam.rs`. That
was true before the split too.

## Closed

All six.

1. `pick.rs` is `core::fmt` at both sites (`:162-163` was `std::fmt`).
2. `Generation`'s type doc no longer speaks from inside a seam — *"a
   monotone counter minted by the session on every submit, and compared
   by both seams"*.
3. `pickindex`'s header links `crate::generation::Generation`, not the
   façade.
4. `BTreeSet` is imported beside `BTreeMap` and used bare at all nine
   sites.
5. *"Six readers"* is expanded as six in both places — `lib` dropped,
   since it only re-exports.
6. The counts are re-measured rather than repaired: after the two
   header rewrites in this fix pass they are `pickindex.rs` 2,414 and
   `pick.rs` 467 (`wc -l`), and the PR body carries those.

The recorded non-defect — `pick.rs` carries no `mod tests`, because the
four unit rows were `PartWindows`/`IdMap`'s and `PickCache`'s coverage
is in `tests/frame_policy.rs` and `tests/eval_seam.rs` — is left as
written; it was true before the split and is not this unit's to change.
