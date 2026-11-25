---
id: a-cone-wall-a-band-off-the-apex-refuses-on-the-sweep
kind: issue
title: A cone against a wall standing one or two bands off its apex refuses on the value's sweep, where the retired axis-pose arm served it
status: open
opened: 2026-11-24
priority: P2
cost: M
---

A narrowing disclosed by U2 (`the-section-certificate-reads-the-cone-pairs-on-the-value`, PR 7637), pinned by `topo` `section_cert_axis_pose_rows::a_wall_a_band_off_the_apex_refuses_on_the_sweep`.

**The pose.** A wall of radius `0.2`, its axis tilted by the parallel rows' swing, stands `K + 0.35` or `K + 0.9` bands off the unit cone's apex. It is read over a unit region about its foot.
- The retired cone arm read the axis pose and served it at `K + 0.9` (one loop per nappe, essential on the wall).
- The section value reads no axis pose. Its ruling sweep meets the discriminant in the band and refuses `ruling_discriminant` at both distances.

**Reach.** A pose this close comes only from drift. An apex placed on a wall by construction decides `Zero`, a node or a touch. U2's review found no demo or slow-set model that reaches it. Refusing is fail-safe, but a near-apex pose the band separates should answer.

**What would close it:** the sweep decides a pose whose apex stands a band or so off the wall by reading the apex's neighbourhood as the divided sweep does: the near-apex sheet's passes are read as a branch point is. The row then flips to answered.
