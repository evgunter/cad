---
id: nurbs-face-meeting-a-plane-in-an-interior-loop-is-unguarded-on-the-crossings-path
kind: issue
title: A NURBS face meeting a plane face in a loop interior to both, while crossings exist elsewhere, is seen by nothing on the crossings path
status: review
branch: germ/nurbs-plane-interior-loop
pr: 3406
opened: 2026-09-28
priority: P1
cost: M
refs: [torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere, nurbs-plane-section-has-no-component-arm, volume-backstop-reads-a-nurbs-operands-unimplemented-closed-form-as-a-classification-invariant]
---

## What

The section certificate (PR 3372) examines pairs in which one face is a
torus, sphere, cylinder or cone; a NURBS face paired with one of those
refuses on reach. A NURBS face paired with a PLANE is outside its scope
on the crossings path: nothing there asks whether the two meet in a loop
interior to both faces while crossings exist elsewhere, and the plane ×
NURBS arm is on the union's roster (`reduce.rs` `boolean_arm_exists`).
On the no-crossings path the extent scan's NURBS re-gate refuses any
fallback entry with a NURBS face (`ops.rs` `sphere_extent_scan`,
`NurbsExtentUnsupported`), so only the crossings path is open.

Unmeasured: build a NURBS bump a plate grazes in an oval while an edge
crosses elsewhere, and run ∪. If it answers, it is the interior-loop
class's last open member; the cheap guard is to refuse the pair on reach
at the certificate, as the spline × curved pairs already do.

## Measured (2026-09-29)

Not a live wrong answer. The fixture is a block `[−2, 2]² × [−1, 0]`
whose top carries a bicubic bump (peak `1.125`) and a clamp whose plate
underside `z = 0.8` grazes it in an oval, with a bar through the block
whose edges pierce its walls (`crates/sweep/tests/germ_interior_oval.rs`
`nurbs_bump`, `nurbs_clamp`). ∪ in both orders refuses
`Containment(KindUnsupported { kind: Nurbs })`, raised in the join's
role resolution (`join.rs` `resolve_roles_geometric`), which probes the
clamp's regions against the bump block; `point_in_solid` resolves every
face first (`solid_contain.rs` `face_geo`) and has no NURBS arm. That
blocks every crossings-path union with a NURBS operand, and the volume
backstop's closed form (no NURBS arm) stands behind it. ∩ and ∖ refuse
at the revert roster. The guard is defense in depth for the day
containment serves the kind.

## Resolution

Every pair with a face that is not a plane is in the certificate's
scope (`ops.rs` `SectionPath::scope`), so NURBS × plane and NURBS × NURBS
are examined. NURBS × plane is W0 when the control net is certified
strictly on one side of the plane (`section_cert.rs` `nurbs_plane`),
and refuses on reach otherwise. No row of the `topo` and `sweep` suites
moved. The residue is filed: `nurbs-plane-section-has-no-component-arm`.
