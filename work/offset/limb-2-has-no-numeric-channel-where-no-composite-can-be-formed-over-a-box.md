---
id: limb-2-has-no-numeric-channel-where-no-composite-can-be-formed-over-a-box
kind: issue
title: Limb 2 has no numeric channel where no composite can be formed over a box: a tilted face whose frame does not certify bisects a box its identity proves
status: open
opened: 2026-10-27
priority: P3
cost: M
parent: closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them
---



Filed by PR 5728's implementer lane (fork row 204's mechanism).

**Finding.** At `Sym<Interval>` limb 2 decides the composite's enclosure
`[floor, upper]` with the carrier's free-parameter residual as its
identity (`crates/geom-brep/src/edge_nurbs.rs`, `limb2_enclosure`,
`free_residual`). The identity is only asked once a numeric channel
exists, and the numeric channel is the composite, which reads the
carrier's net written in the operand's orthonormal frame
(`crates/geom-brep/src/pcurve_cache.rs`, `limb2_offset_sup`;
`crates/geom-brep/src/carrier_net.rs`, `framed_carrier_net`). Where that
frame does not certify over the box, the net refuses
(`AnalyticRung3Refusal::CarrierEnclosure`) before any margin is formed,
so an identity that would prove the edge is never asked.

Measured on M10-3's slab (`crates/editor-core/tests/m10_8_pins_interval.rs`,
`m10_8_the_shipped_set_is_inert_on_straight_geometry`): at the root box
the tilted face's normal, read over the box, is ±1.73 wide and its
in-plane frame axes are `Trv` with infinite ends. The leaf refuses on
`CarrierEnclosure`, `classify_replay` bisects it through read (3), each
half refuses the same way, and each quarter certifies with every limb-2
decision a theorem. The drive takes 4 leaves and 3 splits where `main`'s
sampled check took one: plain 456/282 → 2898/1668, shipped 464/274 →
2954/1612.

**Wanted.** A numeric channel for limb 2 where the composite cannot be
formed over a box, so the identity can decide it. One candidate is the
residual's own enclosure over the box and the window, valued through the
same free variable, which certifies wherever the residual's computation
is defined. That is a margin in the implicit form's metres, first order
near the surface, so whether it may stand for a distance bound there is
a design question. Today bisecting is honest, only dear.
