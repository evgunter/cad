# SENSEFOLD: injective.rs reads a face's sense through the sense door

One P1/E row. Program: CLEAR. Branch: `clear/sensefold`. Review: the
orchestrator's read. It is a mechanical fix pinned by an existing row.
- `work/clear/main-red-injective-rs-folds-the-face-sense-by-hand.md`.

## What is true when this lands

1. `crates/topo/src/injective.rs` negates no value by hand under a
   face's `sense` bit. Every hit that `face_normal::tests::no_source_file_folds_the_bit_by_hand`
   reports in this file is either:
   - read through the sense door that `face_normal.rs` names
     (`geom_brep::OutwardNormal::from_chart`, or a `face_normal` door);
     or
   - a genuinely signed scalar at its point of use. Such a value goes
     in the row's `sanctioned` list, with the reason at the entry.

   Prefer the door. Sanction a value only where no door says what the
   value means. Re-run the row first: CURVEDEP (#5991) may have moved
   the line numbers, so take the hits from its output, not from the
   issue file.
2. **No verdict moves.** Every `self_intersection_proves_injectivity.rs`
   receipt and every injective unit row is bit-identical. A moved
   receipt is a finding: stop and report it.
3. The row's remaining hits are only `crates/mesh/src/trimmed.rs`'s,
   which TESS owns (`tess/main-red-trimmed-rs-folds-the-face-sense-by-hand`).
   Report whether that sibling has closed. If it has, the row goes
   green: say so.

## Ground

- `crates/topo/src/injective.rs` only, plus `face_normal.rs`'s
  `sanctioned` list if item 1 needs it.
- Run the row:
  `cargo nextest run -p topo --lib -E 'test(=face_normal::tests::no_source_file_folds_the_bit_by_hand)' --run-ignored all`.
- Read `docs/prompts/implementer-discipline.md` in full.
- Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground.

## Report

≤40 lines covering:
- each hit and what it became;
- the receipts diff;
- the sibling's state;
- the PR number and its CI result.
