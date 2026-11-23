---
id: the-nary-door-panics-on-an-accumulation-face-with-no-member-lineage
kind: issue
title: The n-ary boolean door panics, on an accumulation face with no member-face lineage, when a variable of the tour's composed die is edited
status: closed
opened: 2026-11-23
priority: P0
cost: M
closed: 2026-11-23
branch: heldrecords/nary-lineage
pr: 7516
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

## 2026-11-23 — measured: the lineage never read the re-chart

Re-run on `origin/main` (`f64`, `Tol::witness()`, the reviewer's edit
`value + max(|value|, 1) × 0.3`, no prior). All eight edits panic with
the quoted text, at `MemberLineage::of` reached from
`MemberLineage::step` (`crates/topo/src/boolean/nary.rs`), inside the
`Node::Union` of the 21 pip balls (`editor_core::eval::wire::fold_members`
→ `topo::nary_placed` → `run_members`). The document's other 42
variables at a nominal of 0.28 or 0.5 do not panic at the same move
(the rest were not re-run).

**Which face, which stage.** For `#129` the panic is at the step
joining member 3. The accumulation entering it has faces
`1v1 … 6v1`; the step's naming has `a_keys: Direct`, and its
`face_fragments_a` rows divide `4v3` and `3v3`, keys the accumulation
does not hold. The result face `4v3` roots to itself, `7v3` to `3v3`.
Those keys were minted by the sphere re-chart: the joining pip's ball
crosses an accumulated ball, the extent scan asks for a re-chart
(`Recuts::rechart`), `apply_recuts` turns that ball's shell about its
centre, and the pair door re-enters on the turned operands
(`boolean_op_recut`). Its naming records, in `recharted[0]`, every
turned cell with the input shell's faces it was turned over.

**Why the invariant failed.** `MemberLineage::step` rooted each result
face through `FaceDescent::result_face` alone (graft rows, fragment
rows), which stops at the turned face; it never read `recharted`.
`Holdings::of_records` reads the same naming and does read `recharted`
(its `input` closure), so the step's holdings were right and its lineage
was not. On operand B the same gap is silent: a turned face of the
joining member was entered as `(member, turned key)`, a member face that
names nothing of the member.

**Is the edited die buildable.** The variables are pip-centre
coordinates (`0.5 − 0.22 = 0.28`; `0.5`). Each move puts one pip's ball
0.08 from a neighbour's (radius 0.09 each), so two dimples run into one
— a part a user can draft by dragging one pip, and a solid: the cube
less a union of overlapping balls. The row is P0 by its own test.

**After the fix** (the lineage reads each turned face as its input
shell's faces, `FaceSource::inputs`): each of the eight edits
evaluates; the union refuses typed,
`Naming(UnsupportedTurnedCell { cell: Edge(_) })`, the turned-seam
refusal ORBIT's re-chart row owns
(`work/orbit/closed-sphere-escape-is-re-charted-by-rotation-beside-the-meridian-cut.md`),
and the cut and fillets downstream are poisoned.

## Closed

2026-11-23, by PR 7516 (`heldrecords/nary-lineage`). Banded P0: the
eight edits are a part a user drafts by dragging one pip, as measured
above.

- The lineage reads every result face through one door,
  `FaceDescent::source` (`crates/topo/src/boolean/nary.rs`). It returns a
  `FaceSource`: the operand, the root in the keys the op ran on, and
  the input faces it stands for, which for a face a re-chart turned are
  every face of the input shell. Holdings and the at-rest readers
  (`crates/editor-core/src/checks/at_rest.rs`) read the same door. The
  bare root door is gone, so no caller can take a re-cut key for an
  input key.
- Each of the eight edits now evaluates to the union's typed
  `Naming(UnsupportedTurnedCell)` refusal (ORBIT's re-chart row):
  `editor-core::all the_tours_die_with_two_pips_run_together_refuses_their_turned_seam`.
- The lineage is pinned on both operands, and each pin goes red with
  that side reading the root alone:
  `sweep::all the_nary_door_reads_a_re_charted_accumulation_through_its_shell`.

Filed from this row's sweep, both on ORBIT:
`a-re-cut-booleans-verdicts-keep-the-re-cut-keys-its-rows-are-respelled-off`
and
`a-step-given-pairs-and-re-charting-its-operand-reads-them-by-the-keys-the-re-chart-moved`.
