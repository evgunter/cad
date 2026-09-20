---
id: join-probe-charts-hand-roll-the-across-axis-reference
kind: issue
title: join's probe charts hand-roll an across-axis reference instead of calling Vec3::orthonormal_basis
status: open
opened: 2026-09-15
---

## Finding

**Raised by SCALAR's `S393` fix pass** (2026-09-15), re-running that
unit's class sweep on the SHAPE rather than on the `0.9` literal it
first keyed on: an if/else between two world-axis constants feeding a
`cross`, which is a hand-rolled orthonormal basis whatever the
threshold.

Two sites in `crates/topo/src/boolean/join.rs` build a chart reference
across an axis that way:

- `cylinder_at` (~`:1976`) — `if axis.x.abs() < 0.5 { x̂ } else { ŷ }`,
  then `axis.cross(seed).normalize()` for the cylinder's `u_ref`. Its
  own comment says the choice exists so the reference is not poison on
  an axis-aligned pose, which is exactly the question
  `Vec3::orthonormal_basis` answers.
- the coaxial-frame probe's radial direction (~`:2468`) — the same
  choice written as a magnitude test on the cross product itself
  (`if axis.cross(x̂).norm() > 0.5 { … } else { … }`), then
  `v = axis.cross(u)` for the second in-plane axis.

`Vec3::orthonormal_basis(axis)` returns that pair, decided by
`copysign` rather than by a threshold, with a stated `Interval`
behaviour at the equator and a bitwise-pinned `f64` spelling. Neither
site is WRONG today — both produce a valid pair — but each carries its
own degenerate-axis policy in a file about boolean joins, and neither
moves when the kernel's does.

**Where**: `crates/topo/src/boolean/join.rs`, `cylinder_at` (~1976) and
the coaxial-frame probe's radial direction (~2468).

**Confidence**: sure (both read as quoted).

The test-side members of the same class are on S-TINT's slate
(`geom-curve-test-frames-hand-roll-the-helper-axis-cone`), and the
viewer's is on VIEW's
(`datums-basis-hand-rolls-the-least-aligned-axis-basis`).

**Verdict:**

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to CURVED (its charter names S-BOOL's ceded ground and inherits at S-BOOL's exit) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.
