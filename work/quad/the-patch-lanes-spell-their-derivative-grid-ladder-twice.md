---
id: the-patch-lanes-spell-their-derivative-grid-ladder-twice
kind: issue
title: patch.rs and trimmed.rs each spell the chart's derivative-grid ladder by hand (PatchGrid::deriv_u/deriv_v chains)
status: open
opened: 2026-10-27
priority: P4
---


Found by C3's split (PR 5815), which left it alone because it is not
convergence or stop logic. Both polynomial-chart lanes build the
chart's derivative grids by hand, as a chain of
`as_ref().and_then(PatchGrid::deriv_u | deriv_v)`:

- `crates/geom-brep/src/props/quad/patch.rs`, `nurbs_patch_face_rounds`:
  `s`, `su`, `sv`, then `suu`, `suv`, `svv`, `suuu`, `suuv`, `suvv`,
  `svvv`.
- `crates/geom-brep/src/props/quad/trimmed.rs`,
  `trimmed_patch_face_rounds`: `s`, `su`, `sv`, `suu`, `suv`, `svv`;
  and `Curvature::of` adds `suuu`, `suuv` on top.

The rational lane's `Ladder` (`rational.rs`) is the same idea for the
homogeneous nets, built once in one type. A `PatchGrid` ladder type
(to a given order) would give the two polynomial lanes one spelling.
A move, so no bits change.
