---
id: a-step-ellipse-stored-minor-first-reaches-readers-that-assume-the-order
kind: issue
title: A STEP ellipse stored minor first: no end-to-end row reaches the order-free bounds, and three readers still refuse on it
status: open
opened: 2026-10-06
priority: P3
cost: M
refs: [4128]
---

Found by the last fix pass of PR 4128. `geom::Curve3::Ellipse` certifies
its semi-axes positive and not ordered (`crates/geom/src/curves.rs:145-170`).
Every kernel producer mints through `Curve3::ellipse`, which refuses
`major ≤ minor` (`crates/geom-brep/src/intersect.rs:866`, `:1479`,
`:1961`), and the copies (`crates/geom/src/scalar_lift.rs:167`,
`crates/topo/src/transform.rs:562`, `crates/topo/src/replace_face.rs:1002`)
keep the order they are given. The one source of a minor-first ellipse
at rest is STEP import, which stores `semi_axis_1`/`semi_axis_2` as
written (`crates/step-import/src/entities.rs:939`), with nothing in
`adopt.rs` or `normalize.rs` reordering them.

## The reachability

PR 4128 made four bounds read the semi-axes in either order:
`crates/topo/src/boolean/carrier_touch.rs` (`speed_bound`, and
`edge_clear_of_ball`'s annulus), `crates/topo/src/splitting/containment.rs`
(`ConicArc::of`'s speed lever and `hit`'s lower bound), and
`crates/topo/src/splitting/join.rs` (`certify_section_area`'s perimeter).
Their rows are function-level (`carrier_touch_rows.rs`,
`containment.rs`'s `*_stored_minor_first_*`). No end-to-end row reaches
them. That needs a STEP solid with an elliptic edge stored minor first,
posed so that a door answers `Uncertain` (for `carrier_touch`, an edge
tangent to a sphere, cylinder or torus off its face). No STEP fixture
in the tree carries such an ellipse, and hand-writing the solid was
out of scope for that pass.

## Readers that still assume the order, on the refusal side

None of these clears or accepts anything wrongly; each refuses where a
minor-first ellipse would be answered once normalized.

- `crates/topo/src/chord_join.rs:1095`: `Margin::levered(along, conic.sa)`,
  `sa` the stored `major`. A smaller lever shrinks the margin toward the
  band, so it refuses more.
- `crates/topo/src/boolean/solid_contain.rs:1313-1345` (the wall-section
  seat): it checks `minor == radius` and `major·|cos| == radius`, so a
  tilted section stored minor first falls to `unsupported()`.
- `crates/geom-brep/src/pcurve_cache.rs:7200-7275` (the cone-section
  image): it needs `u_ref` along the major axis for `ecc` and `β`, so a
  minor-first section images wrongly and `run_cone_section_checks`
  refuses it on residual.

Normalizing at import (swap the semi-axes, turn `u_ref` a quarter, and
shift the edge's parameters by `π/2`) would close all three and the
reachability question at once.
