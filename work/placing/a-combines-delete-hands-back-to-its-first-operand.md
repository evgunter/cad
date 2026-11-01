---
id: a-combines-delete-hands-back-to-its-first-operand
kind: issue
title: A combine's delete hands its readers back to the boolean's first operand, which need not be the operand the combine took over
status: open
opened: 2026-11-01
---


Found by `placing/gesture-placements` (the delete hand-back).

The combine's takeover (`combine_in_world` in
`crates/viewer/src/session.rs`) re-points the readers of the FIRST
operand, in the spec's order, that anything reads along a seen path,
and deletes every later operand's copies. The delete's inverse
(`handed_back` / `taken_body`, same file) reads the document alone, so
for a boolean it hands the result's readers back to the boolean's
first operand (`Subtract`'s `from`, a `∪`'s or `∩`'s first member).
The two agree whenever the first operand is the one with readers; they
disagree on a combine `A ∘ B` where only `B` is read along a seen path
(the product reads `B`'s copy, say): the add re-points `B`'s readers to
the result, and deleting the result hands them to `A`.

Two shapes of fix, neither taken here: the combine takes over only its
first operand's readers (the takeover and its inverse then agree by
construction), or the delete stops handing a boolean back. The later
operands' copies the combine deletes are not restored by any delete;
undo is the way back for them.
