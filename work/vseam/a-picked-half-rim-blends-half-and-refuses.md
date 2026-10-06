---
id: a-picked-half-rim-blends-half-and-refuses
kind: issue
title: a click on a full-revolve rim picks one half-arc, so the blend tool's fillet refuses SeamVertex and its recourse is a kernel-only query
status: open
opened: 2026-10-02
priority: P2
cost: M
---


## Finding

On a full revolve of a profile that touches the axis, every latitude
rim is TWO edges, `BandRim` (`[0, π)`) and `BandRimPi` (`[π, 2π)`),
meeting at two seam vertices. The viewer draws one circle. One pick is
one `StableName` (`crates/viewer/src/session/select.rs`,
`EdgeSelection::name`), so clicking that circle selects half of it,
and the blend tool (`crates/viewer/src/blend.rs`) commits a
`Node::Fillet` over that half. The fillet refuses
`UnsupportedCorner { corner: SeamVertex }`, whose recourse
(`sweep::blend::FILLET3_SEAM_VERTEX_RECOURSE`) says to request every
arc, as listed by `rim_of`. `rim_of` is `topo::query::rim_of`, a
kernel query over arena keys that the viewer offers no action for.
The user can recover by shift-picking the other half if they guess
that the circle is two edges. Nothing on screen says it is.

Measured on the tour's teapot lid (`demos/tour/src/teapot.rs`,
`rim_arcs`, show `teapot-lid-unbored`): each of its three rims refuses
at radii 2/256, 1/256 and 1/512 when asked for by the `BandRim` name
alone, and builds when asked for by both names.

## Relation to neighbours

- `work/offer/a-refusal-offers-no-action-in-the-viewer.md` is the
  general row (a refusal's recourse should be a control where the
  viewer can act). This is a specific instance whose control is
  obvious: "add the rest of the rim to the selection". But the better
  fix is upstream of the refusal, at the pick, so it is filed here, on
  the program that owns the pick and the blend tool.
- `work/emit/band-rim-pi-has-no-minting-builder.md` is the authoring
  side of the same gap (no builder mints the `BandRimPi` name).

## Fix shape

When the blend tool takes an edge pick whose edge is one arc of a
seam-split rim, extend the held set to the whole rim (`rim_of` on the
evaluated body, mapped back to names). Or offer that as a one-click
action on the refusal. Either way the panel should show a rim as one
pick.
