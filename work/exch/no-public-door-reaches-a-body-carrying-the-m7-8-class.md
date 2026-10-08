---
id: no-public-door-reaches-a-body-carrying-the-m7-8-class
kind: issue
title: No public door reaches a body carrying an M7-8 edge, so the fold's Python-observable content has no Python fixture
status: open
opened: 2026-09-21
priority: P3
cost: D
---

## Finding

The M7-8 class — a plane × described-NURBS `Intersection` edge
attached through the plane × NURBS lane
(`Body::set_edge_curve`, which reads it off `AtRestPolicy::nurbs_lane`;
a lane-free `EdgeCurve::recertify` answers `NurbsLaneNotSupplied` on it) — is reachable from
no public door today. The lane's one production minting caller is STEP
adoption (`crates/step-import/src/adopt.rs`, the `Intersection`
candidate on a plane/NURBS pair), and on the bodies the tree can
import that candidate never certifies where the pair is plane ×
NURBS: the exported loft's stays-NURBS-wall cap rims land on the
conventional chart rung, pinned at (4, 4, 4) by
`crates/step-import/tests/nurbs_import.rs`'s
`loft_prism_descriptions_land_in_the_native_classes` (the four
`Intersection`s are on the PROMOTED, planar walls), and the one wild
fixture with a `B_SPLINE_SURFACE`
(`tests/fixtures/wild/stepcode/dm1-id-214.stp`) refuses at the arc-rim
mint (`MapResidual`, `wild.rs`). The lane-certified body the tree
does have — `crates/topo/src/cert_m3r1_probes.rs`'s cube with a flat
described-NURBS wall — has no certified flux lane on that wall, so it
refuses check 7 (`VolumeUncomputable`) at every certified door and
would refuse at `step-import`'s gate too; and the class's other rows
(`crates/geom-brep/tests/m7_8_plane_nurbs_edge.rs`) build the edge
below the body.

Why it matters now: LANE-1 (PR 3010) folded the certified bodies into
the plain `validate_pseudomanifold` / `contact_marks` names, which
`pncad-py`'s `Body.validate_pseudomanifold` and the `pncad` prelude
call at `f64`. On a body carrying the class whose certificates have
gone stale, the verdict moved from `Err([VolumeUncomputable])` to one
`EdgeCertification` per lane edge (both LANE-1 reviewers executed it;
`cert_m3r1_probes::m3_the_plain_names_report_the_corrupt_m7_8_wall_edge_by_edge_and_nothing_else`
pins it in-crate). The fix pass owed a Python row on such a body and
could not write one: the python suite reaches bodies only through
`Doc` recipes and `import_step`, neither of which produces the class,
and the corruption itself writes `Body::surfaces` (`pub(crate)`).
So the fold's Python-observable content is latent — it surfaces the
day an importable body carries a lane-certified rim — and has no
Python fixture.

## What to do

Either land an importable M7-8 body (a STEP fixture whose plane ×
NURBS rim certifies through the lane AND whose NURBS face the
quadrature encloses, so the gate passes) and add the Python row on it
in `crates/pncad-py/tests/test_validate.py` — the in-crate probe's
door table is the verdict to pin — or record at `adopt.rs`'s
`Intersection` candidate that the plane × NURBS arm has no reachable
instance, so the class's reach is a stated fact rather than an
assumed one.

## Evidence, 2026-10-01 (CLEAVE, PR 3678)

`crates/step-import/tests/placed_m7_8_instance.rs` (from that PR's
second review lane) builds the M7-8 cube with a BOWED NURBS wall,
exports it through `step_export`, and imports it placed through
`step_import::import_step`: the imported body carries the class
(`Intersection` descriptions on `Nurbs` carriers, asserted `> 0`).
So `import_step` does reach a body carrying an M7-8 edge from a STEP
file the tree writes itself; whether that body passes the import gate's
check 7, which this row's Python fixture needs, the row does not
assert.
