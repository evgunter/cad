---
id: shell-open-refuses-a-curved-designated-face
kind: issue
title: shell_open refuses every non-plane designated face at construction, where only the props reading of a curved ringed rim is missing
status: open
opened: 2026-10-06
priority: P1
cost: H
design: true
refs: [shell-offset-three-followups]
---


`shell::check_designation` refuses any designated face whose surface
is not a plane (`ShellError::OpenFaceRingUnsupported`), so a vessel
cannot be opened through a curved face — a cylinder's side, a dome.
The refusal was drawn when the property inventory had no reading for
a curved face carrying a ring. That premise has moved: `topo::props`
now refuses such a face at the props call
(`MassPropsError::RingOnCurvedFace`) and already measures one shape of
it (a cylinder wall bounded by rims and rulings), so shell's
construction-time gate refuses bodies the rest of the kernel can
build, and a caller who never asks for volume is refused anyway.

The cost is not the gate but the rim stage behind it:
`shell::lift_to` assumes a plane (`unreachable!` on any other
surface), and the closing-mint doc in `shell.rs` notes a curved
designated chart would write rows after the door.

**The open question** (design): move the refusal to the props call,
as the boolean pierce does, and build curved-rim surgery in the lift;
or keep a construction-time refusal and say why. Weighed by the
designer pair before a spec.

Item 1 of the closed `shell-offset-three-followups` (GitHub 1058),
re-filed 2026-10-06 by SHELL's triage. The props reading of other
curved kinds is FLUX's (`flux/an-ellipse-trimmed-ring-on-a-cylinder-wall-has-no-volume-lane`).
Signed (SHELL orchestrator).
