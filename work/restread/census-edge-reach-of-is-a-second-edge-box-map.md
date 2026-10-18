---
id: census-edge-reach-of-is-a-second-edge-box-map
kind: issue
title: census::edge_reach_of is a second EdgeBoxRule → box map beside boxes' (its spiric arm restates spiric_arc_aabb without the outward rounding), and census::axial_carrier re-spells Unclaimable as None
status: closed
opened: 2026-10-18
priority: P1
cost: M
closed: 2026-10-18
---


Found by the verifier of PR 5261 (`restread/edge-face-lane`), after that
PR gave the `EdgeBoxRule → AxialCarrier` map one home
(`boxes::axial_carrier`).

- `census::edge_reach_of` (`crates/topo/src/census.rs`, about :4277) maps
  `EdgeBoxRule` to a box a second time, beside the box lane's own map. Its
  spiric arm says of itself that it is "`spiric_arc_aabb`'s construction
  without its outward rounding": the same construction written twice,
  one copy unrounded.
- `census::axial_carrier` (about :4156), the census's wrapper over
  `boxes::axial_carrier`, turns `AxialCarrier::Unclaimable` into
  `Option::None`. That is a second spelling of "unclaimable" beside the
  enum's own variant.

Owed: one `EdgeBoxRule → box` reading that both the box lane and the census
read, each at its own scalar (as `boxes::axial_carrier` does), with the
rounding decided once; the census reads `Unclaimable` as the enum. A seam
on BOXES/CLEAVE/HONE ground (`boolean/boxes.rs`); announce it there.

## Closed

`boolean::boxes::edge_rule_box` is the one `EdgeBoxRule → box` map. The
box lane reads it at `f64` brackets (`edge_box`) and the census reads it
at its own scalar (`edge_reach_of`). Each lane enters the rule's points,
directions and lengths itself, as `axial_carrier` does. The conic arm's
arithmetic is the lane's: the exact door for the box lane, `arc_extent`
for the census. The spiric arm is `spiric_extent`, written once in
`Span` arithmetic. The rounding is decided once, in `Span`'s docs: it is
not directed on either lane, and the reader's pad covers it.

The census reach was never rounded outward, on any arm, and every one of
its readers takes it through a margin of band or pad size. So the census
never needed a tighter box. It wrote its own copy because it cannot read
`geom`'s `Bounds`-only door. The box lane's spiric box loses
`spiric_arc_aabb`'s per-operation outward ulps (measured at 2–27 ulps of
each end) and keeps its pad, as every face arm already does. The census
reads `AxialCarrier::Unclaimable` as the enum.
`the_two_spiric_box_readings_agree_but_for_the_rounding` pins all three
relations.
