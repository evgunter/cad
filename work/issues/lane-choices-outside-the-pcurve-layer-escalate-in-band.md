---
id: lane-choices-outside-the-pcurve-layer-escalate-in-band
kind: issue
title: Seven lane choices outside the pcurve layer (four section classes, the revolve wall kind, the spline boundary row, STEP surface promotion) refuse in band where D3 takes the general lane
status: open
opened: 2026-10-20
---


Filed by PCERT unit E1 (`pcert/lane-choices`), from its sweep of lane
choices outside `pcurve_cache`. These sites are other programs'
territory. They are filed once, here, with each owner noted.

## What

Each site below picks between two forms of the SAME cells: a closed
form at Zero, and a general one otherwise. Under D3
(`[verdicts.consumers]`) that is a lane choice. In band takes the
general lane as definite does, or refuses lever-only "not yet" where no
general lane is written. It never escalates with a tolerance offer.
Each site escalates in band today.

**Section classes** (`crates/geom-brep/src/intersect.rs`, GERM's, with
ROOTS and TANG):
- :926 `pc_rim_alignment`: rim circle or tilted ellipse. In band
  should take the ellipse (check `Curve3::ellipse` holds as its tilt
  goes to 0). It also reaches `chord_join::agreed_section`.
- :2501 `pn_axis_normal`: plane × cone, axis-normal circle or the
  conic lane (`pn_conic_type`).
- :2924 `pt_axis_normal` and :2847 `pt_axis_in_plane`: plane × torus.
  The definite arm already routes to the general rung, which is
  lever-only "not yet"; in band should end the same way. Whether a
  tilted torus cut is a lane choice or a structural act (its loop count
  can change) must be settled first.

`route_pose` (:499) passes these escalations through unchanged.

**The revolve wall kind** (`crates/sweep/src/revolve/axis.rs:422`
`classify_segment`, Line arm, SWEEP's): Zero takes a cylinder or a
plane annulus; both definite takes a cone. In band refuses
`SliverAxisClearance`. It should take the cone lane, or refuse
lever-only if a near-degenerate cone is the frontier.

**The spline boundary row** (`crates/geom-brep/src/nurbs_iso.rs:357`
`iso_boundary_row`, used by `crates/topo/src/replace_face.rs:2503`;
OFFSET's): Zero extracts the exact row. Definite refuses `Interior`, and
in band refuses `Escalated`. The derived section exists, but it is
taken only when the hold fails.

**STEP surface promotion** (`crates/step-import/src/recognize.rs:146`,
consumed at `entities.rs:1343`; STEP's): the tests are raw `<= eps_in`
f64, not `decide`. At the multi-bound curved-face gate, the
ill-conditioned arm refuses `RecognitionAmbiguous` with no lever, while
the definite arm refuses `Topology`. Curve recognition is already a
lane choice.

## Fix

Per site: in band takes the definite arm's lane, recording nothing. Add
a row with an in-band pose and a mutant restoring the escalation.
Where the lane does not exist, the refusal is lever-only "not yet" on
both arms.
