---
id: census-edge-reach-of-is-a-second-edge-box-map
kind: issue
title: census::edge_reach_of is a second EdgeBoxRule → box map beside boxes' (its spiric arm restates spiric_arc_aabb without the outward rounding), and census::axial_carrier re-spells Unclaimable as None
status: open
opened: 2026-10-18
priority: P1
cost: M
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
