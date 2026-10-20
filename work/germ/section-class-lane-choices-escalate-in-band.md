---
id: section-class-lane-choices-escalate-in-band
kind: issue
title: Four section-class readings in intersect.rs (pc_rim_alignment, pn_axis_normal, pt_axis_normal, pt_axis_in_plane) escalate in band where D3 takes the general lane
status: open
opened: 2026-10-20
---

Filed by PCERT unit E1 (`pcert/lane-choices`), from its sweep of lane
choices outside `pcurve_cache`.

D3 (`[verdicts.consumers]`) treats a decision that picks between two forms
of the SAME cells (a closed form at Zero, a general one otherwise) as a
lane choice. In band it takes the general lane, as definite does, or it
refuses lever-only "not yet" where no general lane is written. It never
escalates with a tolerance offer and never ends as a defect.

## What

Each site below escalates in band today:

- `crates/geom-brep/src/intersect.rs:926` `pc_rim_alignment`: the rim
  circle or the tilted ellipse. In band should take the ellipse; check
  that `Curve3::ellipse` holds as its tilt goes to 0. It also reaches
  `chord_join::agreed_section`.
- `crates/geom-brep/src/intersect.rs:2501` `pn_axis_normal`: plane ×
  cone, the axis-normal circle or the conic lane (`pn_conic_type`).
- `crates/geom-brep/src/intersect.rs:2924` `pt_axis_normal` and :2847
  `pt_axis_in_plane`: plane × torus. Their definite arm already routes
  to the general rung, a lever-only "not yet"; in band should end the
  same way. First settle whether a tilted torus cut is a lane choice or
  a structural act, since its loop count can change.

`route_pose` (:499) passes these escalations through unchanged.

ROOTS and TANG share `intersect.rs`; this is filed once, on GERM's
slate.

## Fix

In band takes the definite arm's lane, recording nothing. Add a row
with an in-band pose for each site, and a mutant that restores the
escalation and reds it.
