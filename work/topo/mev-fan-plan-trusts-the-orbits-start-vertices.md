---
id: mev-fan-plan-trusts-the-orbits-start-vertices
kind: issue
title: mev_fan_plan trusts the vertex orbit's start vertices: a torn orbit is carried through a fan mev instead of refused typed, the gap kev_plan closed in PR 3161
status: open
opened: 2026-09-29
refs: [kevs-fan-merge-needs-a-re-describing-kill-door, 3161]
priority: P2
cost: E
---

## What

Found by PR 3161's fix pass while closing that PR's MAJOR, and
disclosed in its body. It is filed here because a PR-body disclosure
is not a slate row.

`Body::kev_plan` (`crates/topo/src/euler_kill.rs`) now proves that
every half of the merged fan starts at the dying vertex, and refuses
`OrbitBroken` otherwise. Without that check, two `next` tears put the
killed half into the dying vertex's orbit, and `kev_describing`'s write
reached an `unreachable!`. `mev_fan_plan` (`crates/topo/src/euler.rs`)
walks the same `vertex_orbit` (`next(mate(x))`) and has the same gap:
it trusts the orbit's start vertices. It cannot panic, because no
mutation-phase `unreachable!` depends on it. But on a torn body it
carries the corruption through a fan `mev` instead of refusing it
typed, which D1's plan-phase contract requires.

## The shape to give

The same proof in `mev_fan_plan`: every half of the moved run starts
at the vertex being split. Refuse `OrbitBroken` otherwise. Pin it with
a torn-orbit row on the `review_d18` fixture set, as PR 3161 did for
`kev`.
