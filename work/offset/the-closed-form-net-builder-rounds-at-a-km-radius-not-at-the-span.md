---
id: the-closed-form-net-builder-rounds-at-a-km-radius-not-at-the-span
kind: issue
title: A closed-form carrier's enclosing net rounds at its radius, not at its span: a km-radius circle's tiny arc carries about 3.5e-12 of rounding
status: open
opened: 2026-10-26
priority: P2
cost: M
---



Filed by PR 5728's implementer lane, from fork row 203's fix.

**Finding.** `crates/geom-brep/src/carrier_net.rs:266` `span_bernstein`
expands a circle, ellipse or spiric about the span's node `m` as
`P₀ + U·α(t) + V·β(t)`, with `P₀` the centre and `|U|`, `|V|` the radius.
Each Taylor coefficient therefore carries rounding at the radius's
scale, whatever the span's own length. Framing at the operand
(`framed_carrier_net`) removes the world origin's scale, but not the
radius's. A 1 km-radius circle's 1e-7 m arc
(`chord_join::tests::a_rim_patchs_turn_is_levered_at_its_arcs_not_their_whole_turn`
and `boolean::join::frame_dispatch_tests::a_rim_patchs_turn_is_levered_at_its_arcs_in_the_germ_frame`)
reads a net width of 3.5e-12 m and a limb-2 bound of 2.6e-12 m, so it
refuses as `AnalyticRung3Refusal::KernelLimit` at ε = 1e-12. Its extent is
1e-7 m, so the rounding is the radius's, and the story names the kernel,
not shorter edges.

**Wanted.** Anchor each span at its own point: expand
`C(t) − C(m)` (`U·(α(t) − α(m)) + V·(β(t) − β(m))`) and add `C(m)`, an
interval point, once per control. The coefficients past the constant
then round at the span's chord, not at the radius. `C(m)`'s own
enclosure stays at the radius's scale, but it enters once rather than
through every Bernstein sum.
