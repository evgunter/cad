---
id: emit-blend-restates-the-kernels-own-arguments
kind: issue
title: editor-core's emit_blend re-derives two arguments that now have their home in sweep's kernel types, and one is already narrower
status: open
opened: 2026-09-04
track: V
---


## Finding

`D324` moved two arguments to the kernel types they are about. The consumer
still carries its own derivation of both, and neither cites the type that now
owns it:

- `crates/editor-core/src/names/emit_blend.rs:263-267` now says *"`Retired`
  carries no face channel (the surgery's one `kef` door refuses a source face,
  and states why), so a face key here is a real survivor"* — the restatement
  this row was filed against (*"Faces are never retired — a support shrinks,
  it does not die"*) was replaced by that pointer in PR 1943, which also made
  the claim an enforced one at `sweep::blend::surgery`'s door rather than an
  argument at the type. The **coverage half of the finding stands**: the
  comment still names `sweep/tests/m6_5_fillet_naming.rs` as the check where
  the kernel side names two fixtures, so the two homes still disagree about
  their own coverage.
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

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/code-quality/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), and given
`track: V` — the body names its own fence ("`crates/editor-core/` is
**Track V's** fence … whoever staffs V rows it"), and
`crates/editor-core/src/names/emit_blend.rs` is not in DOCM's `paths`
(DOCM takes `names/role.rs` only), so the row belongs to the letter
rather than to a program. Id, body and header are otherwise
unchanged; any `## Home` section above naming `work/issues/` is
superseded by this line and is kept as the record of why the file was
parked there.
