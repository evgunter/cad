---
id: shell-answers-for-the-complement-of-an-inside-out-operand
kind: issue
title: topo::shell consumes an inside-out operand and answers for its complement: no orientation gate on the way in
status: open
opened: 2026-10-03
priority: P1
cost: M
---


## What

`topo::shell` (`crates/topo/src/shell.rs`, `shell` / `shell_open`)
reads no orientation on its operand, so an inside-out body is hollowed
as if its complement were the material. Its closing `validate_geometric`
passes the result, because the result is a valid body: it is the answer
to a different question.

Measured on `cleave/inside-out-gate` (main 82b9ceb2 plus the Boolean's
orientation gate), `Tol::witness()`: `prism_z` over the clockwise
triangle (0,0), 190°, 80° (unit radius), z ∈ (0.5, 1), volume −0.2349
(tier 3's check 7 refuses it `NegativeVolume`), `shell(&wedge, 0.05,
tol)` returns `Ok` with a body of volume 0.1667. What the counterclockwise
wedge returns, and so which wall the inside-out one built, is
unmeasured.

Found by the sweep of
`work/cleave/an-inside-out-operand-passes-the-boolean-gates-as-its-complement.md`,
whose fix refuses an inside-out Boolean operand typed
(`BooleanError::InsideOutOperand`, from `boolean::reduce::gate_operand`).

## Owed

Refuse an inside-out operand at `shell`'s door, typed, per solid
(`validate::inside_out_solids` reads tier 3's check 7 at the scalar's
lane), before any offset reads it. The finished-body adoption
(`work/reach/boolean-door-adopts-the-finished-body-type.md`) subsumes it
once `shell` takes `AtRestBody`. The blend and offset doors' posture is
unmeasured: the clockwise wedge's fillet requests refused alike in both
orientations, so no measurement reached them.

## 2026-10-03 — the boolean's refusal with typed operands (REACH)

With the boolean's typed operands
(`boolean-door-adopts-the-finished-body-type`), an inside-out body
refuses at `AtRestBody::validate`, tier 3's `NegativeVolume` per solid,
and the door keeps `validate::inside_out_solids` for an operand with no
verdict (a dual, `reduce::gate_unverdicted_operand`). `shell` reaches
the same refusal by taking `AtRestBody` (its own adoption unit) with the
same per-solid read where no verdict rides, or reads check 7 per solid
at its door until then.

## 2026-10-06 — measured on main `575b309d` (CLEAVE, `cleave/split-operand-gate`)

The same clockwise wedge, `Tol::witness()`, thickness 0.02:

- `shell` returns `Ok`, volume 0.05961; the counterclockwise wedge's
  shell is 0.05097. A valid-looking body of the wrong volume.
- `shell_open` with the top face open refuses ("the assembled thin solid
  is not valid (1 errors) and is discarded"); counterclockwise: 0.04297.
- `replace_face_offset` of the top face by +0.1 returns `Ok` with volume
  −0.18794 (counterclockwise: +0.28191 from +0.23492): the region
  shrank where the counterclockwise wedge's grew.

On the slit dome (`crates/sweep/tests/pole_slit_window.rs`'s `slits`, a
strut tip at the pole), `shell` refuses "offsetting a face inward
refused: replace_face_offset: the re-described body is not tier-2 valid":
the operand's own strut, reported as the offset's result failure.

The blend doors' half of the posture is filed on BAND:
`work/band/blend-doors-answer-an-inside-out-operand-with-an-inside-out-body.md`.
The split's is closed by that branch (its doors take `AtRestBody`).
