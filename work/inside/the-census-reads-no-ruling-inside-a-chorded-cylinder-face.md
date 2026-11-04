---
id: the-census-reads-no-ruling-inside-a-chorded-cylinder-face
kind: issue
title: Tier 3′'s census refuses a body whose ruling line edge lies in a cylinder face its spline chords trim: the containment door does not read that trim
status: open
opened: 2026-11-03
priority: P2
cost: M
refs: [cylinder-chart-trim-reads-a-walk-it-cannot-take-as-a-corrupt-face]
---



Filed by SPIRICARM's rotational-chart lane, stage 1 (branch
`spiricarm/rotational-lane-cylinder`, 2026-11-03).

## What

Since the rotational lane measures cylinder faces trimmed by spline
chords, skew cylinder pairs build. One of them is the unit cylinder
`z ∈ [0, 2]` ∪ a rod of radius `0.3`, `z ∈ [−1.5, 1.5]`, turned `π/6`
about `x` and moved by `(0.95, 0.1, 2)`
(`crates/sweep/tests/tang_circle_cylinder.rs`
`a_tilted_rod_through_a_rim_builds_at_its_sliced_volume`). The union
builds, its volume within `1e-9` of the sliced oracle. Its
`validate_pseudomanifold` refuses:

```text
CensusUnsupported { subject: Entity(Face(FaceKey(11v1))),
  cause: ContactLane(NotCertifiable { what: "a line edge on a curved
  face's ruling whose trim the containment door does not read" }) }
```

The ∖ and ∩ of the same pair pass tier 3′.

The refusal is raised in `crates/topo/src/census.rs`, in the
edge-in-curved-face lane (the `face_holds` call with
`"pm_census_ef_curved_contain"`), where `face_holds` answers
`Some(None)`: `boolean::contain`'s `face_placement` read the face's trim
as `CurvedPlacement::Trim(None)`.

## Not checked

That face `11v1` is the chorded cylinder face, and that the trim read
fails at a projected chord edge. Both are inferred. The closed row
`cylinder-chart-trim-reads-a-walk-it-cannot-take-as-a-corrupt-face`
lists "a run edge with no closed-form image" among
`cylinder_chart_trim`'s refusals, which `face_placement` reads as
`Trim(None)`. Confirm it before building a fix.

## Why it matters

Every skew or unequal-radius cylinder union whose ruling lands inside
the partner's chorded wall fails tier 3′, and `differential::outcome`
reads it `BAD`, although the body and its volume are right.
