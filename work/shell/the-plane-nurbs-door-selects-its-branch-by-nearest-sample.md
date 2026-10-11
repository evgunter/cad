---
id: the-plane-nurbs-door-selects-its-branch-by-nearest-sample
kind: issue
title: offset_derive::plane_wall_section selects its branch by nearest sample, not by the certified tube holding a seed (branch_at)
status: open
opened: 2026-10-10
priority: P2
cost: M
refs: [a-wall-seam-between-two-fits-has-no-section]
---

Ev's ruling on `a-wall-seam-between-two-fits-has-no-section` (PR 4515,
its `## Decided`): "`offset_derive::plane_wall_section` moves from
nearest-sample selection to `branch_at` on the plane × NURBS door. That
door gets the same handle, in the arm's unit or the next." The arm's
unit built the handle for NURBS × NURBS (`geom_brep::ssi::Section`) and
left the plane × NURBS door as it was; this is the next unit.

Today `crates/topo/src/offset_derive.rs`, `plane_wall_section`, runs the
whole `plane_nurbs_ssi` (boundary pass, every branch, the accounting)
and keeps the branch whose 64-sample schedule passes nearest the old
edge's middle (`nearest_sample`): the selection is sampled, not proved.

Owed: a plane × NURBS `Section`-shaped handle (`of`, `branch_at`,
`all`) over the existing lane (`ssi::boundary`, `ends::Ends`, the chart
tube), `plane_nurbs_ssi` becoming `all()`, and `plane_wall_section`
calling `branch_at(mid)`, the answer the branch whose certified chart
tube holds the settled seed. Re-baseline the rows that read the plane
door's refusals (`encl_curved_loft_shell`, `offd_r1_probes`).
