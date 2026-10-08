---
id: replace-face-offset-answers-for-the-complement-of-an-inside-out-body
kind: issue
title: replace_face_offset takes any &mut Body, so an inside-out body's face moves the complement's way and returns Ok
status: dispatched
opened: 2026-10-06
priority: P2
cost: M
branch: shell/face-door-at-rest
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

The two simultaneous offset doors share the shape and the reason:
`offset_planes_together` (`crates/topo/src/offset_together.rs`) and
`offset_charts_together` (`crates/topo/src/offset_axial.rs`) take
`&mut Body` and are the sealed arm's per-solid steps over the same
clone. Their inside-out posture is unmeasured; one decision should
cover all three doors.

## Decided

2026-10-08, by the orchestrator on a designer pair's agreeing reports
(byte 188 on `analysis/design-fork/shell-face-door-at-rest`; no
ratified text moves, so no `[ev]` PR and no fork-log row).

**The premise is wrong.** `d` is measured along the chart normal, and
no door reads the solid's sense, so an inside-out body's face moves the
same way in space as the right-way-round body's. The signed volume
change is `+A·d` in both windings (the wedge: +0.047 each way); the
"−0.18794" row is a body that was already negative and stayed so, not
a move toward the complement. The issue's wrong answer does not exist.

**The doors stay construction steps.** `replace_face_offset`,
`replace_faces_offset`, `offset_planes_together` and
`offset_charts_together` take `&mut Body` and are tier 2 in, tier 2
out, as D1's "a finished body is a type" already allows. The rule,
stated once in `crates/topo/README.md` (a "Shell and offset surgery"
row) and in each door's module doc:

> A door whose argument means something about material (inside,
> outside, thickness into the solid) takes a finished body
> (`AtRestBody`). A door whose argument is stated against charts alone
> takes construction state (`Body`), and its result becomes finished
> only through `AtRestBody::validate`, where an inside-out result
> refuses `NegativeVolume`.

A public verb over `&AtRestBody` (Q8's offset-faces) is later feature
work, not this fix. Reading check 7 per door is rejected.

**Owed by the implementer:**

1. The doc paragraphs: `replace_face.rs`, `offset_together.rs`,
   `offset_axial.rs`, `shell.rs`; reword `replace_faces_offset`'s
   "outward" comment to say chart-normal.
2. The README row above.
3. A pin test file under `crates/topo/tests/`, one row per door: the
   wedge in both windings through `replace_face_offset`; the wedge
   through `offset_planes_together`; a clockwise revolve through
   `offset_charts_together`. Each pins signed `ΔV = +A·d` (or the
   door's analogue) equal across windings, and that the inside-out
   operand and result both refuse `NegativeVolume` at
   `AtRestBody::validate`.
4. Optional: a typed tier-2 refusal at a door's entry, only if the
   lane measures a row where an untyped failure surfaces today.

Close as a premise correction once that lands.
