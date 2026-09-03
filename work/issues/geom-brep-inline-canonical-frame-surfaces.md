---
id: geom-brep-inline-canonical-frame-surfaces
kind: issue
title: geom-brep tests spell the canonical-frame elementary surfaces inline at 34 use sites
status: open
opened: 2026-09-03
---


TCOST-8 gave `crates/geom-brep/tests/shared/surf.rs` one home for the
elementary surfaces built in the canonical frame — centred at the
origin, about `+z`, `u_ref = +x` — and routed the fifteen NAMED
helpers that built them there (`sphere`/`cylinder`/`cyl`/`zcyl`/
`unit_cylinder`/`torus`).

**Its census pattern could only see a named `fn`.** Sweeping instead
for the SHAPE — `Surface::{Sphere,Cylinder,Torus} {` with an origin
centre, a `+z` axis and a `+x` `u_ref`, in any of the spellings the
tree uses (`p3(0,0,0)`, `Point3::new(0,0,0)`, `Point3::origin()`;
`v3(0,0,1)`, `Vec3::new(0,0,1)`, `Vec3::unit_z()`; and the same for
`u_ref`) — finds **34 more, spelled inline at their use sites** on
`8433129ac`, after TCOST-8's own diff:

- `Sphere` x 11: `intersect_table.rs:302`, `m5_pr7_ssi.rs:2073`,
  `m5_pr9_tangent.rs:357`, `mesh11r2_base_probes.rs:30`,
  `offa_r1_probes.rs:118`, `offa_r1_probes.rs:245`,
  `r2_probes.rs:132`, `review_m5_pr7_enclosure.rs:105`,
  `s58_iso_rectangle.rs:236`, `s58_iso_rectangle.rs:596`,
  `s58_iso_rectangle.rs:635`
- `Cylinder` x 14: `intersect_table.rs:814`, `m5_pr7_ssi.rs:851`,
  `m5_pr7_ssi.rs:882`, `mesh11_arc_branch.rs:336`,
  `offa_r1_probes.rs:48`, `pcurve_p1a_meter.rs:203`,
  `pcurve_parameter_finding.rs:47`, `r2_mesh7_door_probes.rs:126`,
  `r2_probes.rs:215`, `review_m2_pr3_certify.rs:225`,
  `review_m2_pr3_certify.rs:275`, `review_m2_pr3_certify.rs:356`,
  `review_m5_pr7_adversarial.rs:47`, `s58_iso_rectangle.rs:51`
- `Torus` x 9: `m5_pr9_tangent.rs:308`, `mesh11_arc_branch.rs:386`,
  `offa_r1_probes.rs:128`, `offa_r1_probes.rs:156`,
  `r2_mesh7_door_probes.rs:50`, `review_pr12_meridian_probe.rs:22`,
  `s58_iso_rectangle.rs:283`, `s58_iso_rectangle.rs:692`,
  `s81_one_rim_level_rule.rs:50`

(A few of those line numbers are inside the helpers TCOST-8 rewrote —
`offa_r1_probes.rs:48` and `pcurve_parameter_finding.rs:47` were the
`zcyl`/`cylinder` bodies it took — so the residue is nearer thirty.)

**Why TCOST-8 did not take them, and what a taker owes.** These are
not helpers; they are fixtures written at the row, and the frame is
sometimes the row's subject rather than incidental to it (a sphere at
the origin about `+z` is what makes a polar-degeneracy row a polar
row). Routing them through `shared::surf` is therefore a judgement per
site, not a mechanical rewrite, and each one that stays owes a reason
AT the site under `shared/mod.rs`'s second rule. That is a unit's worth
of reading; it is not a defect in the tree today, only an unpaid
consolidation.

No behaviour is at stake: every site listed builds a value bit-identical
to `shared::surf::{sphere,cylinder,torus}` at its radii.
