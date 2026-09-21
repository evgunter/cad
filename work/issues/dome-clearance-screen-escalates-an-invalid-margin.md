---
id: dome-clearance-screen-escalates-an-invalid-margin
kind: issue
title: sweep: the dome's equator at r >= 0.6 escalates fillet3_face_clearance with MarginDiag::Invalid
status: open
opened: 2026-09-13
priority: P0
cost: H
---



## Finding

Filleting the dome's equator rim (`test_support::dome(1.0)`, the
plane–sphere rim of radius 1, one closed edge) across a ladder of blend
radii reads, at the default band:

| r | outcome |
|---|---|
| 0.1, 0.25 | builds, both contact edges jet-determinate at the closed form |
| 0.4, 0.5 | `FaceClearanceUncertified` (definite) |
| 0.6, 0.75, 0.9, 0.95, 0.99 | `Escalated { predicate: Some("fillet3_face_clearance"), margin: MarginDiag::Invalid }` |

A NON-VALUE margin reaches a user-facing refusal: the clearance screen
(`crates/sweep/src/blend/battery.rs`, `fillet3_face_clearance`, the
`gap − setback_here − setback_there` margin) is handed a poisoned
reading past `r = 0.6` and escalates it as if it were in band. The
rendered sentence then quotes a margin that is not a number, and the
recourse ("reduce the blend size, or enlarge the support face") is the
in-band sentence for a reading that was never classified. Measured by
BLEND-14's second reviewer and pinned as a table (not an assertion) in
`review_contact_edge_must_carry_r2_probes::r2_a_sphere_support_toward_osculation_never_reaches_the_rule`;
the likely cause is a setback the arm cannot derive once the ball's
centre passes the sphere's centre (`r > R/2`, the spine collapsing),
which the arm should refuse typed before the screen reads it.

## Disposition

Outside BLEND-14's fence (the battery's screen and the arms' setbacks);
filed for whoever owns the clearance screen. Not acted on.
