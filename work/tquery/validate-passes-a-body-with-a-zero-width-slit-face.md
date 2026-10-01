---
id: validate-passes-a-body-with-a-zero-width-slit-face
kind: issue
title: split runs no validation tier on its own outputs, so a spurred half left the op unchallenged
status: open
opened: 2026-09-24
priority: P3
cost: D
---


## What

Before PR 3133, `topo::split` of `brick(0..1.5, 0..1, 0..1) ∪
brick(1.2..1.3, −1..2, 0.5..3)` by the plane y + z = 2 returned two
halves carrying a zero-width slit:
- the section face ran out along y = z = 1 and back;
- the Below half held two coincident vertices at each point along the
  slit.

The slit came from the pinch lane's mirrored rerun
(`splitting/mod.rs`). PR 3133's join now refuses it as
`SplitJoinError::SectionSpur`.

**Which tiers refuse those halves** (measured by PR 3133's review):
- tier-2 `validate_closed`, `validate_geometric` and the
  pseudomanifold validation all refuse them;
- tier-1 `validate` passes them. That is by design: tier 1 checks
  combinatorial consistency, not geometric degeneracy.

## The gap

`split` runs no tier on the bodies it returns. The degenerate halves
left the op as a success and surfaced only downstream, as
`NamingError::Duplicate` in editor-core's name emission, which pointed
at the wrong module.

A tier-2 or geometric check on `split`'s outputs, or a debug-assertion
one, would have caught this at the op that made the body. Unmeasured:
its cost on the split suites.

## Found by

The sweep for EMIT's `split-section-face-keeps-a-zero-area-spur-along-a-tangent-edge`,
corrected by PR 3133's review (m4).

## Re-homed to TQUERY, 2026-09-24 (ATREST orchestrator)

Moved from `work/atrest/` by `git mv`, id and body unchanged. The at-rest
tiers answered correctly here — tier 2 and every geometric door refuse
the spurred halves, and tier 1 passing them is its charter. What the
row asks for is a validation tier run by `split` ON ITS OWN OUTPUTS,
and `crates/topo/src/split.rs` is TQUERY's ground: whether an op
validates what it returns, and at what cost to the split suites, is the
op's posture to decide, not the validator's.
