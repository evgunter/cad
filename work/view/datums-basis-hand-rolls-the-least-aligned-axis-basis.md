---
id: datums-basis-hand-rolls-the-least-aligned-axis-basis
kind: issue
title: datums::basis hand-rolls a least-aligned-axis orthonormal basis, with its own cross and normalize
status: closed
opened: 2026-09-15
closed: 2026-09-16
branch: view/datums-basis
---

## Finding

**Raised by SCALAR's `S393` fix pass** (2026-09-15), from that unit's
class sweep re-run on the SHAPE — an if/else between world-axis
constants feeding a `cross` — rather than on the `0.9` literal it first
keyed on.

`crates/viewer/src/datums.rs`'s `basis` (~`:714`) picks the world axis
`n` is least aligned with, by a three-arm comparison of `|n.x|`,
`|n.y|` and `|n.z|`, and returns `(unit(n × seed), unit(n × u))`. It
uses the module's own `cross` and `unit` helpers, each with its own
doc justifying why display scaffolding may spell them locally; `unit`'s
doc then argues that its fallback arm is unreachable FROM `basis`
because a unit normal crossed with its least-aligned world axis has a
length floor. That is a hand-derived conditioning argument about a
construction the kernel already makes: `Vec3::orthonormal_basis`
returns the same kind of pair with the branch replaced by a `copysign`,
no comparison threshold, and its own stated behaviour at the equator.

This is production `src/`, not a test helper, which is why it is filed
apart from the test-side row. The comment restating the recipe's
rationale is part of the cost: the argument has to be maintained here
even though nothing in the viewer decided the policy.

**Where**: `crates/viewer/src/datums.rs`, `basis` (~714), with its
local `cross` (~728) and `unit` (~736).

**Confidence**: sure (the helper reads as quoted).

Siblings: S-TINT's `geom-curve-test-frames-hand-roll-the-helper-axis-cone`
(the test-side members) and BOOL's
`join-probe-charts-hand-roll-the-across-axis-reference`.

**Verdict:**
 adopt the kernel's door.

## Closed

`basis` is `UnitVec3::orthonormal_basis` and nothing else. The
three-arm seed comparison, the local `cross`, the local `unit` and the
local `dot` are all deleted; `frame_segments` and `grid` take `Vec3`'s
own `cross` and `dot`. The conditioning argument the module maintained
about a construction it did not decide the policy for is gone with the
construction.

**The drawn picture moves, and here is where.** Measured over the
world axes, the equator from both sides of the signed zero, the ties
the local `<=` chain breaks, and 20,000 random unit normals: the pair
turns about the normal for essentially every normal (0 of 20,000 agree
to within 1e-9°). What a reader sees is smaller than that, because a
square grid is symmetric under a quarter turn and a tick under a half:

- the **six world axes** move by an exact multiple of 90°, so the three
  default PLANES draw the same picture **while the `MAX_GRID_LINES`
  cap does not bind** (see the correction below), and an AXIS datum
  along ±y or ±z has its end ticks turned 90° about itself (±x is
  unchanged);
- every other normal's grid turns by up to 45°, visibly.

**Seams, both ways.** The rule this replaces is not continuous either:
its seed changes at each of **three tie conditions**, and the kernel's
door jumps only where `copysign` does, at the equator `n.z == 0`. The
trade is three conditions for one circle, and the one is stated on the
door rather than derived here. **The per-crossing costs are a range,
not a number, and the range runs the other way from what this row
first said** — see the correction below.

Pinned by four rows in `crates/viewer/tests/datum_draw.rs` over a
thirteen-normal spread; two of them red on the merge base. Sibling
rows elsewhere (S-TINT's, BOOL's, FIX's) are reported, not touched.

## Corrected by the VIEW review of #2783 (orchestrator, 2026-09-17)

Two measurements in the section above were wrong as written. The code
change is unaffected; what was wrong is the account of what was
measured, and this row is the durable record of that account.

**1. "The three default PLANES draw the identical picture" is true
only while `MAX_GRID_LINES` does not bind.** `rule_patch` caps `count`
at 96 and fills the patch **from `first` upward**, so a quarter turn —
which negates one family's index — changes WHICH 96 lines survive when
the cap binds. Measured by digesting the drawn segment set for the
`+z` default plane on both trees: identical at every zoom sampled for
a 1280 px and a 4000 px viewport, **different at 5 of 24 sampled zooms
at 2560 px** (an ordinary monitor, eye distance ≈ 0.30–0.35 m, both
trees drawing 193 segments = 96 + 96 + tick), and different at 8000 px
with the last line at `x = 0.2` against `x = 1.7`. The visible
difference is which end of the patch is clipped, not a turned grid.

This also falsifies a sentence on another program's slate, reported
rather than edited: `work/chrome/max-grid-lines-truncates-a-ruling-and-
calls-it-one.md` says *"the cap has never been observed firing."* It
has now — at 2560 px, at ordinary zoom, on the default `+z` plane.

**2. The discontinuity figures were seam SAMPLES presented as seam
values, and they understated the door's worst case.** Computed over
each seam rather than sampled: all three local ties have the **same**
profile — in-plane jump 90°→120°, grid 0°→30° — so the 101.1° / 103.6°
split this row asserted is sampling noise describing a structural
difference that does not exist. The kernel's equator seam is
`180° − 2t`, i.e. 0°→180°, with grid 0°→**45°** and tick 0°→90°. So
the honest statement is **three seams costing up to 30° of grid,
traded for one costing up to 45°** — still a good trade on count, and
not the trade the original table argued.
