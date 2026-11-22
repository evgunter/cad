---
id: a-pose-chain-written-deeper-than-the-load-doors-envelope-does-not-load
kind: issue
title: A pose chain written inline has no depth bound, and the edit door and evaluation overflow the wasm stack on a deep one
status: open
opened: 2026-10-23
priority: P2
cost: E
---


A pose read is written inline wherever an edit authors one — a
mate's `on` or `to`, a split's tool — as nested `PoseDef`s
(`Slide { pose, .. }`, `Spin { pose, .. }`, `Flip { pose }`), and no
door bounds how deep the chain nests. The load door's body limit is
`persist::nesting::BODY_NESTING`, `ENVELOPE + 2 · MAX_NESTING`
(`crates/editor-core/src/persist/nesting.rs:76`), where `ENVELOPE` is
the deepest position a save writes an expression at.

Until D′2 that deepest position was a mate's world read, which the
edit door bounded (`MAX_WORLD_READ_DEPTH`), and `ENVELOPE` was 35. D′2
retired world reads; the deepest position is now a pose chain in a
mate's `to`, and `ENVELOPE` is 34, the depth
`expr_nesting_bound::deepest_pose_document` writes (six slides over a
spin, flipped). A chain written one slide deeper saves, and its edit
log refuses to load ("the body nests deeper than … levels"); the
snapshot holds each pose as a variable of its own and is not affected.

Done when a pose chain's depth is bounded where it is authored (the
edit door refusing typed, as `MAX_NESTING` does for an expression),
and `ENVELOPE` is that bound's deepest position, or the load door
reads a pose chain at a depth of its own.

Moved from `work/issues/` to `work/doctail/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

## Parked (2026-11-17, DOCTAIL triage)

P3, E, parked on `the-save-format-drops-its-edit-log`. The defect is
that the saved EDIT LOG does not load, and the snapshot is unaffected.
That row removes the log from the format, which takes this defect with
it. When it lands, check whether `persist::nesting::ENVELOPE` still names a
position a save writes. Then close this row, or re-open it for the
snapshot if a pose chain reaches the snapshot's own nesting bound.

## 2026-11-17 — the file half is gone, and the defect survives at the edit door

`the-save-format-drops-its-edit-log` removed the edit log from the
format. A save now writes each pose as a variable of its own, so a chain
nests nothing in the file. `persist::nesting::ENVELOPE` is 6, which is an index
count's position, and `expr_nesting_bound` saves and loads a chain of
eight slides over a spin whose angle sits at the expression bound.

The unbounded depth survives one door earlier. On the smallest stack
(`test_utils::own_thread::on_the_smallest_stack`, 1 MiB), applying the
placement whose mate's `to` holds the chain inline overflows the stack:

- 64 slides over a flip with a literal angle overflow inside
  `apply` of the `DocEdit::place`. `Placed::of` and building the
  `Operand` both complete. 16 slides complete everything (apply, pin,
  save, load).
- With the spin's angle at the expression bound (`BOUND` = 128), 16
  slides already overflow. 8 complete everything.

Reproduce by raising `POSE_SLIDES` in
`crates/editor-core/tests/expr_nesting_bound.rs` and running
`every_door_takes_an_expression_at_the_bound_on_the_smallest_stack`.

## 2026-11-17 — evaluation overflows too, and Python reaches it (PR 7094 review)

The review of PR 7094 reproduced the edit-door overflow and found a
second one. A chain of 64 slides over a spin with a literal angle,
authored and saved on a large stack, LOADS on the 1 MiB stack (the save
writes each pose a variable of its own) and then aborts in EVALUATION.
64, 128, 512 and 2048 slides all abort. So a file a user can save, on a
desktop's stack, cannot be opened and evaluated by the viewer's
workers, and nothing refuses it typed. Python reaches the same chain
through `PoseDef.slide` (`crates/pncad-py/src/py/pose.rs:206`), which
nests without a bound.

The evaluation half is pinned by the ignored row
`expr_nesting_bound::a_long_pose_chain_evaluates_on_the_smallest_stack`
(64 slides). It aborts the test process, so it runs only when asked:

    cargo nextest run -p editor-core --test all --run-ignored only \
      -E 'test(=expr_nesting_bound::a_long_pose_chain_evaluates_on_the_smallest_stack)'

Re-banded P2: the overflow is a process abort on a file a save writes,
reachable from the Python surface, not only a load-door corner.

Done when every door a pose chain crosses either takes it on the
smallest stack or refuses it typed, naming a bound:

- the edit door, for a chain written inline in a mate's `to`, a split's
  tool or any other operand (`MAX_NESTING`'s pattern: refuse one past
  the bound, at authoring);
- evaluation, for a chain that loads (the ignored row above passes, and
  is un-ignored);
- a chain built variable by variable, each pose reading the one before
  it by id, which nobody has checked yet: the edit door admits each
  step alone, so the depth only shows at evaluation;
- `PoseDef.slide` and the other pose constructors in pncad-py, which
  meet the same bound.
