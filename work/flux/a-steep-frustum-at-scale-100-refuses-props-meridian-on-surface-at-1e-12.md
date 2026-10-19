---
id: a-steep-frustum-at-scale-100-refuses-props-meridian-on-surface-at-1e-12
kind: issue
title: the (α 1.3, S 100) steep frustum no longer revolves at ε = 1e-12: props_meridian_on_surface escalates on its cone wall, so reach_cone_split's steep-frusta row panics before its split
status: open
opened: 2026-10-19
priority: P2
cost: M
refs: [props-cone-ellipse-elevation-over-reads-a-section-ellipse-by-twenty-fold]
---

Found red on main at ε = 1e-12 by the PCERT orchestrator on
`214ff4f1c7`, and reproduced on `pcert/windings-structure` (main plus
B2b). Filed by PCERT B2b's fix pass (F8); it is FLUX's ground.

## What fails

`crates/sweep/tests/reach_cone_split.rs`,
`steep_frusta_in_a_generic_pose_split_at_a_shallow_tilt`, at
`CAD_TOLERANCE_EPS=1e-12`. Its `(α 1.3, S 100, f 0.5)` row states
`builds_from: 0.0`, so it revolves the frustum at every ε, and the
build now refuses before any split:

```
the frustum is not a finished body: [VolumeUncomputable { solid: SolidKey(1v1),
  source: Face { face: FaceKey(3v1), source: Escalated { cause: Indeterminate {
  margin: Enclosure { lo: -2.256e-13, hi: 1.167e-12 }, band: { zero: 1e-12,
  escalate: 1e-11 }, predicate: Some("props_meridian_on_surface") },
  check: Exact } } }]
```

(`crates/topo/src/lib.rs:391`). The enclosure straddles zero and
reaches past the band's zero, so the cone wall's line arm escalates.

## Likely cause

PR 5038 (`flux/line-iso-incidence`, merged as `8ef77cd2f5`, an ancestor
of `214ff4f1c7`) made the cylinder and cone line arms decide
`props_meridian_on_surface` at an upper bound on the segment's distance
from the surface: the ends' residuals plus `radial_sag`. At scale 100
that bound reaches about `1.2·ε` at ε = 1e-12. Not bisected.

## What to decide

Either the bound over-reads this generator (measure `radial_sag` and the
end residuals against the true distance at this pose and scale, as
`props-cone-ellipse-elevation-over-reads-a-section-ellipse-by-twenty-fold`
does for the section ellipse), or the row's `builds_from` for this pose
moves to `1e-9` with the cause named at it.
