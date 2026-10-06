---
id: replace-face-offset-answers-for-the-complement-of-an-inside-out-body
kind: issue
title: replace_face_offset takes any &mut Body, so an inside-out body's face moves the complement's way and returns Ok
status: open
opened: 2026-10-06
priority: P2
cost: M
---


`topo::replace_face_offset` / `replace_faces_offset`
(`crates/topo/src/replace_face.rs`) take `&mut Body<T>` and read no
orientation, so on an inside-out body `d` moves the face toward the
complement's material. Measured by CLEAVE on main `575b309d`
(`work/shell/shell-answers-for-the-complement-of-an-inside-out-operand.md`,
the 2026-10-06 section): the clockwise wedge's top face offset by +0.1
returns `Ok` with volume −0.18794, where the counterclockwise wedge goes
+0.23492 → +0.28191. Its closing gate is tier 2, which an inside-out
body passes.

Left out of the `shell` door's adoption of `AtRestBody` (PR 4112)
because the door is two things at once: a public edit primitive, and
`shell`'s chart-by-chart step over a clone that is mid-construction
between charts — a body no verdict can be read off, so the shape the
Boolean and the split use (take `&AtRestBody`) does not fit it as is.
No editor or pncad-py route reaches it today (tests and examples only).

Owed: decide the public door's posture — split a public door that takes
a finished body (and returns one) from the crate-internal step `shell`
uses, or read check 7 per solid at the door — and refuse the wedge row
typed.
