---
id: cone-generator-at-the-apex-refuses-at-unit-scale
kind: issue
title: A cone generator ending at or crossing the apex refuses limb 2 at unit scale (KernelLimit), and a window centred on the apex refuses NoOffsetBound
status: open
opened: 2026-10-27
priority: P2
cost: M
---


Filed by PR 5728's implementer lane, from the PR's dual review
(2026-10-27); not fixed there.

**Finding.** Limb 2 against a cone (`crates/geom-brep/src/edge_nurbs.rs`
`analytic_edge_limbs`, the composite `crate::pcurve_cache::limb2_offset_sup`)
refuses two unit-scale inputs that lie outside the ruled kernel-limit
classes (kilometre extents, the thin torus):

- **A generator ending at or crossing the apex** refuses as
  `AnalyticRung3Refusal::KernelLimit`. The reviewer's numbers, a cone
  with α = 0.5 and its apex at the origin, ε = 1e-9: the generator
  ending at the apex bounds at 1.95e-9 m, the one crossing it at
  3.2e-9 m, both past the band. The cone's composite reads `√|f|` near
  the apex (C2), where `f` is the product of the distances from the
  meridian half-plane's two generator lines, and its metre conversion
  degenerates as the lever shrinks to the apex, so the bound does not
  fall under the band at unit scale.
- **A window centred on the apex** refuses
  `AnalyticRung3Refusal::NoOffsetBound`: `chart_at`
  (`crates/geom-brep/src/pcurve_cache/projected.rs:1045`) picks the
  cone's nappe from the carrier's height at its window's middle
  (`:1031`–`:1037`), and at the apex that height is zero, which
  decides `Zero` and refuses the sector on its lever before any part
  is read.

**Why it matters.** Both are well-conditioned inputs at unit scale: a
cone's own generator is on the cone. The `KernelLimit` story there
claims a limit the session box does not explain, and the window case is
a chart choice read at one point where the parts on each nappe could
each be charted.

**Direction, not decided here.** Split the carrier at the apex before
the composite (each piece on one nappe, read from its own middle), or
read the generator's identity against the cone directly as the
enclosure arm's free-`t` residual does.
