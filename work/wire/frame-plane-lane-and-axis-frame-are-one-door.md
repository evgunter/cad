---
id: frame-plane-lane-and-axis-frame-are-one-door
kind: issue
title: frame_plane_lane and axis_frame destructure the same DatumValue::Frame and raise the same refusal, and axis_frame's own doc says so
status: dispatched
opened: 2026-09-11
refs: [2376]
branch: wire/operand-door
---


## Finding

Found by the style review of PR 2376, by reading rather than grepping —
which is why neither this program's sweeps nor PR 2376's found it.
Confidence `likely`, the reviewer's. Accurate at `af8bbca`.

`crates/editor-core/src/eval/wire.rs:1011-1032` (`frame_plane_lane`) and
`:1072-1094` (`axis_frame`) perform the **identical** `value_of` plus
`let ValuePayload::Datum(DatumValue::Frame { origin, u, v: y })`
destructure, and raise the **identical** `WrongOperand { expected:
"datum frame", found: kind_name() }`. They differ only in the shape they
return.

`axis_frame`'s own doc says *"Same door as `frame_plane_lane`'s and same
refusal."* That is Q2's sharpest shape from
`docs/prompts/reviewer-style-lane.md`: a comment that exists **to
reconcile two spellings of one rule** — *"that comment is evidence the
rule needs one home, and it is usually the ONLY evidence, because the
code compiles either way."*

## Rides with the composed-phrase row

`"datum frame"` is one of the eight composed sites PR 2376 licensed to
stay prose, and two of its three copies are exactly these two functions.
So this row and
`composed-expected-phrases-are-hand-copied-across-sites` are the same
two lines seen from two directions: give the door one home and two of
the three copies go with it. Whoever takes either should read both.
