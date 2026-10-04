---
id: a-tube-rim-vertex-touching-a-face-along-its-tangent-unions-past-tier-3-prime
kind: issue
title: A tube's rim vertex touching a face whose plane holds the rim tangent unions with the exact volume but fails tier 3′
status: open
opened: 2026-10-04
priority: P2
cost: M
---


## What

Found in the review of PR 4004 (`join/fan-end-one-spelling`, NOTE 4).
Identical on main and on the PR's head, so not that PR's.

**The pose.** `crates/sweep/tests/fan_end_review_probes.rs`, on branch
`join/fan-end-one-spelling-review`, `fan_end_review_rim_battery`:

- the tube: a full revolve about `y` of the annulus `0.5 ≤ ρ ≤ 1`,
  `y ∈ [0, 1]`;
- its outer rim vertex `v` (the lone vertex of a closed circle edge),
  `(1, 1, 0)` on the top rim or `(1, 0, 0)` on the bottom;
- a cube of side 10 standing on the plane through `v` with unit
  normal `m = (cos θ, sin θ, 0)`, `θ = (k + 0.37)·15°`, on `m`'s side
  (`cube_beyond`). The plane holds the rim tangent `±z`, and the cube
  touches the tube at `v` only.

**What happens.** ∪ builds, in both operand orders, with the exact
volume (`1002.356194490` = cube + `π(1 − 0.25)`), and passes tier 2,
the geometric certificate and the legal-operand check, but
`validate_pseudomanifold(&body, &contacts, tol)` fails (`t3p=false`).
The 24 lines (`FANREV tube <rim> m<i> <order> U`):

- top rim, `k = 0..5` (`θ` 5.6° to 80.6°, `m96`..`m101` at the
  battery's default `FANREV_N=96`), orders `xy` and `yx`;
- bottom rim, `k = 18..23` (`θ` 275.6° to 350.6°, `m114`..`m119`),
  orders `xy` and `yx`.

These are the poses where the plane leans over the rim, so the tube
lies wholly on the plane's far side and the touch is one point. On
the same poses ∩ is `EMPTY ok` and ∖ builds `SOUND`, in both orders. The other
rim-tangent poses refuse typed (`SingleSiteSectionLoop`,
`SectionArcWindow`, `Escalated`) or build `SOUND`.

**Likely seat.** The union ships a point touch between a closed curved
edge's lone vertex and a plane face's interior with no contact record
tier 3′ accepts: either the record is missing or `validate_pseudomanifold`
does not admit a vertex-in-face touch whose vertex lies on a closed
edge.

## How to reach it

Check out `join/fan-end-one-spelling-review`, then
`cargo test --release -p sweep --test all -- --ignored --exact
--nocapture fan_end_review_probes::fan_end_review_rim_battery | grep
t3p=false`.

## The shape to give

Decide which is right: the union carries a vertex-in-face contact
record tier 3′ accepts, or the union refuses typed. Pin one pose of
each rim with an exact-volume row that asserts tier 3′.
