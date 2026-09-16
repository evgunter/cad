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
  default PLANES draw the identical picture, and an AXIS datum along
  ±y or ±z has its end ticks turned 90° about itself (±x is unchanged);
- every other normal's grid turns by up to 45°, visibly.

**Seams, both ways.** The rule this replaces is not continuous either
— its seed changes at each of the three magnitude ties, jumping the
in-plane pair by about 101–104° there (grid 11–14°, tick 76–79°). The
kernel's door has ONE seam, the equator `n.z == 0`, jumping 16–164°
(grid 16°). Three discontinuity surfaces are traded for one, and the
one is stated on the door rather than derived here.

Pinned by four rows in `crates/viewer/tests/datum_draw.rs` over a
thirteen-normal spread; two of them red on the merge base. Sibling
rows elsewhere (S-TINT's, BOOL's, FIX's) are reported, not touched.
