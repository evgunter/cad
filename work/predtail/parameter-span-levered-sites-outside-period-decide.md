---
id: parameter-span-levered-sites-outside-period-decide
kind: issue
title: Four parameter gaps cross to metres through Margin::levered outside Period::decide
status: open
opened: 2026-10-26
priority: P3
cost: E
---


Split from PR 5724's review (`pred/turn-rule-follow-ups`). That branch
moved every `Period::decide` caller's parameter gap onto
`Margin::metered`. A parameter span or gap crosses to metres through the
rate pair (`Margin::metered`/`metered_sup`; `Margin::levered`'s own doc,
`crates/geom-core/src/predicate.rs:692`), but the `.decide(` sweep could
not see sites that call `decide` on a `Margin::levered` of a parameter
difference directly. The review named five; read one by one:

- `crates/topo/src/offset_axial.rs:2706` `offset_axial_edge_window`:
  `Margin::levered(span, floor)`, `span = t1 − t0` and `floor` the
  carrier's `param_speeds` floor (`:2686`). A forward-span claim, so
  `Margin::metered(span, InfSpeed::new(floor))`; bit-identical.
- `crates/geom-brep/src/props/curved.rs:4551` `props_torus_pieces_meet`
  and `:5261` `props_meridian_pieces_meet`: a gap between two pieces'
  parameters levered by the torus's minor radius, decided `Zero` (a
  within claim, so the sup door). On a meridian piece the tube angle's
  rate is the minor radius exactly. At `:4551` the pair may be two `Rim`
  pieces, whose parameter is the azimuth, moving at `ρ = R ± r`, not
  `r`. That looks like a wrong lever as well as a wrong door; it is read
  at `exact_band()`, so no verdict should move, but the lever should be
  checked.
- `crates/topo/src/boolean/ops.rs:7009`–`:7015` `bool_sphere_cut_span`:
  `t1 − t0` and `theta − end`, a circle arc's parameter, levered by its
  radius `cr`. That is the exact rate, so `Margin::metered` (forward and
  interior claims); bit-identical.
- `crates/topo/src/boolean/sectors.rs:2526` `bool_cone_arc_span`: **not
  a parameter**. `span = |p̂ × b̂|` is a sine of unit vectors, a
  dimensionless `x` that `Margin::levered` exists for. Legitimate; no
  change.
