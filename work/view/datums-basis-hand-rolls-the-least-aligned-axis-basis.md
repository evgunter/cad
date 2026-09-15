---
id: datums-basis-hand-rolls-the-least-aligned-axis-basis
kind: issue
title: datums::basis hand-rolls a least-aligned-axis orthonormal basis, with its own cross and normalize
status: open
opened: 2026-09-15
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
