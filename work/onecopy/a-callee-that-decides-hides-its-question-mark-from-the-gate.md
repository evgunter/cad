---
id: a-callee-that-decides-hides-its-question-mark-from-the-gate
kind: issue
title: escalation-question-marks.sh cannot see a ? on a callee that decides inside it: about 265-295 band-reading ? lines in loops
status: open
opened: 2026-11-06
priority: P2
cost: H
---


Found by PR 6553's review (`onecopy/refusal-standing`).

## What

`scripts/gates/escalation-question-marks.sh` counts a `?` only on a call
whose name holds `decide`. A loop that returns an escalation from a
callee that decides inside it is invisible to the gate. This is the
blind spot its header names first, and how `CarrierLoop::place_off`
hid.

**Size:** an upper bound by a rough scan. These are lines inside a loop
body, in production sources, holding `band` and `)?` and no `decide*(`
call: **265 lines in 76 files** at this branch. The review counted about
295 with a wider shape. Most read every element needed, or a premise.

**Hand-rolled searches the review found among them**, each an escalation
returned ahead of a later reading that could answer:
- `boolean::sectors` (the `while k < sectors.len()` cut search):
  `extreme_directions(…)?` and `within(…)?`, ahead of a later direction
  or face that cuts;
- `census` (the wedge-pair search returning `Ok(Some(i))`): `read(…)?` and
  `strictly_inside(…)?`, ahead of a later probe or pair inside;
- `boolean::overlap::piece_witness`'s corner loop:
  `corner_holds(…).map_err(…)?`, ahead of a later corner that holds;
- `boolean::solid_contain::point_in_faces`: `region.contains(q, band)?`
  inside the on-face read, beside the six `decide(…)?` sites already
  registered.

**A second gap: a loop that is an expression** (`let x = loop {`, `let
found = 'outer: for …`). Its header does not start its line, so the gate's
loop detection misses its body. The header now says so.

## What closing it takes

Seeing these cannot come from the gate's line shape. It needs the
callee's error type to state its standing (`geom_brep::ladder`'s module
docs), so that a `?` on a kept reading's type is the visible shape. One
way would be a register of `?` on calls returning `Result<_, E>` where `E`
has a `refusal()`.

Until then, the work is a pass over the 265 lines that dispositions each
as needed-every-element, premise, or hand-rolled, with the hand-rolled
ones ported or listed. Widen the gate's loop detection to `= loop {` and
`= 'label: for` headers in the same pass.
