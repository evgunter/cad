---
id: a-u-profile-touching-the-axis-twice-panics-the-half-revolve
kind: issue
title: a half revolve of a profile touching its axis along two separate runs panics (unreachable!: a kernel driver passed an Euler operator a bad argument) instead of building or refusing
status: closed
opened: 2026-11-22
priority: P1
cost: M
closed: 2026-11-23
pr: 7422
branch: lathe/u-profile-half-revolve
---


Found by an EXPORT probe (2026-11-22, default ε, main at 3a9a9b82e1), filed by the EXPORT orchestrator.

## Reproduction

`sweep::revolve` of a U-shaped profile on `SketchPlane::zx()`. Sketch (u, v) = (0,0), (1,0), (1,1), (2,1), (2,0), (3,0), (3,2), (0,2), a single loop that touches the axis along two separate runs. Axis origin (0,0), dir (−1,0). `Revolution::Partial(π)`.

It panics at `unreachable!` in `crates/sweep/src/revolve/mod.rs` (about line 1085): "a kernel driver passed an Euler operator a bad argument (the surfaces argument, edge EdgeKey(8v1), does not resolve in this body)". The path is `revolve/partial.rs` (about :407) → `revolve/upgrade.rs` (about :44, `dihedral_across`), on the second axis edge.

## Why P1

A public verb panics on a profile a user can draw. Fail-loud means a typed refusal or a built body, never a panic. The same profile revolved by a different angle was not tried.

## Done when

The profile builds, or refuses typed and names what it cannot do. A row pins the outcome.
