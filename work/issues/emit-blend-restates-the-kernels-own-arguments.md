---
id: emit-blend-restates-the-kernels-own-arguments
kind: issue
title: editor-core's emit_blend re-derives two arguments that now have their home in sweep's kernel types, and one is already narrower
status: open
opened: 2026-09-04
---


## Finding

`D324` moved two arguments to the kernel types they are about. The consumer
still carries its own derivation of both, and neither cites the type that now
owns it:

- `crates/editor-core/src/names/emit_blend.rs:263-267` restates *"Faces are
  never retired — a support shrinks, it does not die"*. That claim now lives
  at `sweep::blend::naming::Retired`, argued from the surgery's operator set
  rather than from the consumer's expectation. The restatement is **already
  narrower than its source**: it names `sweep/tests/m6_5_fillet_naming.rs` as
  the check where the kernel names two fixtures, so the two homes disagreed
  about their own coverage on the day the kernel side landed.
- `crates/editor-core/src/names/emit_blend.rs:253-258` re-derives the slotmap
  versioned-key argument while citing *"(module docs)"* for it — the same
  citation that, before `D324`, pointed at a paragraph denying what the code
  did. The contradiction is gone; the second home is not.

Both arms should cite the kernel type instead of re-deriving it. One rule with
two homes and nothing enforcing the agreement is what `D324` was: the
difference is that the old shape was a contradiction, which is loud, and this
one is agreement that can drift, which is silent — and one half has drifted
already.

`crates/editor-core/` is **Track V's** fence, and the T-2 lane could not edit
it from `crates/sweep/`. Filed here rather than on Track V's slate because the
finding is a handoff, not a row anyone has scoped; whoever staffs V rows it.
Citations are accurate as of 2026-09-04.

## Was

`unrowed` — raised by lane T-2 and sharpened by the T-2 style review
(2026-09-04), which found the scope drift.
