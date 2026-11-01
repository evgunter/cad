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
box lane reads it at `f64` brackets (`edge_box`), and the census reads
it at its own scalar in its frame (`edge_reach_of`). The conic arm's
arithmetic is chosen by the lane: the exact door for the box lane,
`arc_extent` for the census.

The spiric arm is `geom::spiric_period_box` on both lanes. It is also
the construction `geom`'s own `spiric_arc_aabb` reads, and so
`sweep::blend::surgery::boxed_reach` reads it too. One construction
remains, and the duplicate is gone.

The census's unrounded box was never meant to be tighter: it was a copy
the census needed because it cannot read a `Bounds`-only door. The
review of PR 5308 showed that neither unrounded copy was sound near the
fold. That was true of the census arm already on main, and of the box
lane in this PR's first pass. `f_min` is the root of a cancelling
difference, so radicand rounding at the scale of `ρ²` moves it by about
`ρ²·u/(2·f_min)`, far beyond any pad.

So the rounding is decided once, inside the construction. Each radicand
is widened by `8·u·W` at its own scale (`W = (R ∓ r)² + d²`) before its
root, and each coordinate end by `32·u·S`,
and the box encloses the period at every scalar by itself, with no pad.

What moved:
- Both lanes' spiric boxes widen.
- Off the fold the widening is about `32·u·S`.
- Near the fold the root's widening adds at most
  `f_min − √(max(f_min² − 16·u·W_min, 0))`, where
  `W_min = (R − r)² + d²`. Measured at both ε over the gate's
  fixtures (scales 10 to 10⁴, r/R 0.02 to 0.9), the widening is
  4.7e-12 to 2.0e-7 m. It is 1.1e-7 m on the review's fold, which the
  unrounded box missed by 1.2e-8 m, and 2.0e-7 m on its fat torus.
- No public door reaches either lane with a spiric, so no census
  finding or pruning count moves.

`the_spiric_box_holds_its_exact_extremes_within_its_charge` holds both
lanes and `geom`'s door to an exact-reference box in both directions,
over admitted spirics at scales 10 and 100 near the fold.
`the_spiric_period_boxs_inner_root_clamps_in_its_charge_and_poisons_past_it`
pins the root's two arms. The census reads `AxialCarrier::Unclaimable`
as the enum.

Filed: `work/boxes/census-lane-span-extents-are-not-self-enclosing`.
