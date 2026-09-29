---
id: boolean-side-codes-lever-a-chord-direction-where-a-zero-is-read-as-on
kind: issue
title: The boolean and splitting lanes' side codes read a chord direction's side of a face plane times an arm, and a Zero there is read as ON: check per verdict whether the arm is the reading's own length
status: dispatched
opened: 2026-09-28
priority: P2
cost: M
parent: CONTACT-9
---


Filed from CONTACT-7's designer pair. Both designers raised this
independently, and both marked it unverified.

`boolean/sectors.rs`'s `side_code` / `enters_material(dir, normal, arm)`
and `boolean/vtxfac.rs` read a chord's side of a face plane as
`dir·n × arm`, where `arm` is the shorter chord. The Zero (Tangent) is a
verdict: it becomes On and is then reclassified. This is the lever
shape CONTACT-1..5 found unsound wherever a Zero is a verdict, and a
bisector's code has no vertex behind it. The boolean's
`oriented_plane_eq` coplanarity gate may backstop it; neither designer
checked.

The metric principle, if it applies: an edge is read at its own
endpoints (exact for a line), and a face at its own vertices. Trace one
consumer of each Zero before deciding anything; this may be a no-op.
