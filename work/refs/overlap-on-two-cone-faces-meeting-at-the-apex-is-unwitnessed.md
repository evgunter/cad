---
id: overlap-on-two-cone-faces-meeting-at-the-apex-is-unwitnessed
kind: issue
title: topo::overlap on two cone faces meeting at the apex refused Unread in every pose; the witness rework no longer places the apex, but no row reads a cone
status: open
opened: 2026-11-02
priority: P3
cost: E
---



## Finding

The dual review of PR 6387 measured `topo::overlap`
(`crates/topo/src/boolean/overlap.rs`) at its frozen head
`e1ee89dea`. Two cone faces meeting at the apex refused
`OverlapRefusal::Unread` in every pose, not only in band. The cause is
that the door placed each vertex of each face in the other, and read
the outward normal there, and at the apex the cone has no tangent plane
(`face_normal::NormalAtError::AtConeApex`).

The fix pass (same PR) changed this. The door now places only the
middles of edge pieces, and reads the senses at a middle of the first
edge. It no longer reads the apex, except where an edge piece's middle
falls on it, and a piece ending at the apex has its middle clear of
it. So the refusal likely no longer fires. But no row builds a cone
pair (topo's and sweep's test support build no cone body by name).

What is open: a row that unions or reads two cone faces sharing an
apex, one sense and opposite.
