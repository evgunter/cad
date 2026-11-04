---
id: props-reads-a-cone-generator-apart-from-the-mints-ruling-gate
kind: issue
title: Props reads whether a line is a cone's generator by its own two predicates, apart from the mint's ruling gate
status: open
opened: 2026-11-04
priority: P2
cost: E
---



Filed from the sweep of `pcert/one-ruling-reading`, which gave the
question "is this line a ruling of its chart" one home,
`pcurve_cache::ruling_miss`. The mint's cylinder and cone arms read it,
and so do props' cylinder parse (`props_meridian_axial`) and topo's
`solid_contain::wall_crosses_by_turn`.

## What

Props asks the same question of a cone in two other places, in its own
form:

- `props/curved.rs`, `require_cone_generator`:
  - `props_meridian_generator`: the direction's angle off the cone of
    generator directions (`implicit::cone_offset` of `‖dir × axis‖` and
    `|dir·axis|`), levered by the edge's reach from the apex;
  - `props_meridian_apex`: the apex's distance from the line
    (`‖(apex − p0) × dir‖`, which assumes `dir` is unit).
- `props/curved.rs`, `require_one_chart_branch`: `props_meridian_apex`
  again, as a classify, used as the generator filter.

Both are in metres over the edge, so this is not the units defect the
home fixed. But it is still a second spelling of one reading. The mint
asks "does the generator its anchor names miss the line by more than
the band over the span?". Props asks "is the line within the band of
the apex, and within an angle of some generator direction?". On an
in-band line the two can disagree.

## Fix

Have props read `ruling_miss` against a generator it names. The natural
choice is the unit direction from the apex to the span's farther end,
which is well conditioned wherever the edge has length. The two
predicates then fold into one. Golden props margins and k-stats counts
under these names move, and each verdict that moves needs a measure.
This was not folded in the sweep's unit because it can move props
verdicts that neither of its rows calls for.
