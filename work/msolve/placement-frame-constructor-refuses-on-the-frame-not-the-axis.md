---
id: placement-frame-constructor-refuses-on-the-frame-not-the-axis
kind: issue
title: Frame::rotate_then_translate normalizes a degenerate axis to NaN and is refused downstream on the frame it built, not on the axis
status: open
opened: 2026-09-05
---


Reported by MSOLVE-1's implementer lane (PR 1929), outside its fence.

`crates/editor-core/src/placement.rs`, `Frame::rotate_then_translate`:
the axis is normalized with a bare `.normalize()`, not through the
`eval_direction_norm` door every other direction in the tree takes. Its
own doc says what happens: a zero or non-finite axis normalizes to NaN,
the frame is non-finite, and `DocEdit::SetPlacement` refuses typed on
`Frame::is_finite`. So nothing is silent — but the refusal names the
FRAME the constructor built, not the axis the caller passed, which is
`memories/refusal-text-is-not-cause.md`'s shape at the placement
registry's door: the raising site and the cause disagree. The
transform node's map (`transform_map` in `eval/wire.rs`, MSOLVE-1's
one home) decides the axis first and refuses
`DegenerateDirection`/`NonFiniteDirection` in the axis's own voice; the
constructor could take the same decided unit axis and stop being the
one place a direction is normalized undecided. Small; a candidate
rider on MSOLVE-3, whose subject is refusals naming their cause.
