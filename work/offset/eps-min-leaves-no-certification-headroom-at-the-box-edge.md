---
id: eps-min-leaves-no-certification-headroom-at-the-box-edge
kind: issue
title: ε_min = 1 km·2⁻⁵² leaves no headroom for a certificate at the box's edge: a certified bound there is a few times ε_min
status: open
opened: 2026-10-26
priority: P2
cost: E
design: true
---



Filed by PR 5728's implementer lane, from fork row 203's fix (the
designers' follow-up).

**Finding.** `crates/geom-core/src/tolerance.rs:102`
`EPS_MIN = SESSION_BOX_EXTENT · 2⁻⁵²` ≈ 2.2e-13 m is the length one `f64`
coordinate resolves at the box's edge. A certificate's bound there is not
one rounding but several: a carrier's net written in an operand's frame
rounds at `|p|·u` per control (`crates/geom-brep/src/carrier_net.rs`
`Frame`), and the composite adds a few roundings on top. Measured at
ε = 1e-12 on parts near 1 km (`boolean::boxes::tests::the_conic_edge_box_*`,
707 m: bound 1.1e-12, width 1.6e-12;
`boolean::boxes::sphere_rect_rows::a_zone_axis_near_a_box_row_keeps_its_radial_share`,
793 m: 3.8e-12 and 4.4e-12; `near_circular_sections`, 1118 m: 1.8e-12
and 2.2e-12), the bound is 5 to 20 times `EPS_MIN`. So a run at an ε
just above `EPS_MIN` cannot certify an edge near the box's edge,
whatever its shape: limb 2 refuses it as the kernel's approximation
limit.

**The question (Ev's number).** What certification headroom should
`EPS_MIN` carry over a coordinate's resolution: a constant factor (16?
64?) times `SESSION_BOX_EXTENT · 2⁻⁵²`, or a smaller box at fine ε?
