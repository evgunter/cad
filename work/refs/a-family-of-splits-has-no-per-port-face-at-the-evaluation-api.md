---
id: a-family-of-splits-has-no-per-port-face-at-the-evaluation-api
kind: issue
title: A family of splits answers no table, body or read per port at the Evaluation API: its value is Family with no outputs
status: open
opened: 2026-11-22
---


Found by the sweep of FORMWORDS' `a-split-of-a-family-has-no-halves-a-read-reaches`
(branch `formwords/split-of-family`), which made the evaluator project
a split of a family's halves per D10 `[repetition]`: a reader at the
same index takes the member's half, any other the family of halves
(`split_port`, `half_of` and `halves` in `crates/editor-core/src/eval/wire.rs`).
That fix is inside evaluation. The surfaces that read a split's halves
off the node's stored value still see `ValuePayload::Family`, which has
no outputs (`family_value` in `crates/editor-core/src/eval/members.rs`):

- `Evaluation::output_names` (so `table`, `groups`, `shared_table_at`),
  `named_outputs`, `read_body` and `body_read_of`
  (`crates/editor-core/src/eval/mod.rs`): for a half of a family of
  splits, `table(split.0)` is `None`, `named_outputs(split)` is empty,
  and `read_body(split.0, i)` answers `NoBodies { payload: "family" }`
  where it should give member `i`'s half. So a selection authored on a
  half's member (the viewer's face pick on `split.above[1]`) has no
  table to resolve against outside evaluation.
- The product gather (`gather` arm `ValuePayload::Family => None` in
  `crates/editor-core/src/product.rs`): a world copy of a split of a
  family contributes no body, though each member's halves are bodies.
- `payload_reach` (`crates/editor-core/src/eval/wire.rs`): a family of
  splits reaches `0.0`, the arm for a value that is no body.

None of these panics; each answers as though the family held no
bodies. The repair is a decision about where a family of a multi-output
operation keeps its per-port faces: on the stored value (a per-port
family of halves, built once in `family_value`), or derived on read as
the evaluator now does. REFS's slate as the program whose charter
(reads and ports) it fits; the ground is `eval/mod.rs` (BORDERS,
TALLY) and `product.rs` (BORDERS).
