---
id: the-nary-door-panics-on-an-accumulation-face-with-no-member-lineage
kind: issue
title: The n-ary boolean door panics, on an accumulation face with no member-face lineage, when a variable of the tour's composed die is edited
status: open
opened: 2026-11-23
priority: P1
cost: M
---


Found by the reviewer of BOXFINISH PR 7458 (2026-11-23), by execution,
on that PR's frozen head `cc23f94cf21` (merge base `e1830a1cc`). The PR
touches nothing in `topo`; the reviewer reports it as pre-existing and
outside its fence. **Not re-run by the filer.** Filed here because the
record the panic names is this program's kind of ground; re-home it if
another owner of `crates/topo/src/boolean/nary.rs` fits better.

**The panic:** `crates/topo/src/boolean/nary.rs`, the `unreachable!`
whose text is "accumulation face FaceKey(4v3) has no member-face
lineage, though the n-ary door records one for every face of every
accumulation it builds".

**Reached by input:** the corpus document `die_composed_tour`,
evaluated at `f64` after one `SetVarValue` edit, with no prior:

- a variable whose nominal is 0.28 set to 0.58 (the reviewer's record
  names variables #129, #357, #744, #972, #1352, #1504), or
- a variable whose nominal is 0.5 set to 0.8 (#433, #509).

The reviewer's sweep moved each continuous free variable alone by
2^-20, 2% and 30% over the whole corpus and caught panics; these eight
edits, all at 30%, are the ones that panicked. Its record is
`probe/review-split-reads/head/differential.txt` on
`origin/boxfinish/review-split-reads` @ `ceba9d94e`, with the probe
that drives it in that branch's `crates/editor-core/tests/`.

**Band:** P1, by the precedent of ATREST's
`run-checks-panics-on-a-cut-of-an-instances-copy`: an edit of a
document's variable is a normal verb, and the kernel answers it with a
panic where DESIGN `[failure.taxonomy]` asks for a typed refusal or a
body. Whether the edited die is buildable at those values is not
known. If it is geometry a user would build, the row is P0.

**Closing test:** each of the eight edits returns a body or a typed
refusal, as a row; and the lineage invariant the `unreachable!` states
is either held by construction or its failure is typed.

(BOXFINISH orchestrator)
