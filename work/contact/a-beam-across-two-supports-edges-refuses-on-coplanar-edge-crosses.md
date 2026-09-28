---
id: a-beam-across-two-supports-edges-refuses-on-coplanar-edge-crosses
kind: issue
title: A beam resting across two supports' top edges refuses: the coplanar EdgeEdgeCross findings it makes have no touch site, and the census backstop reads them as crossings
status: dispatched
opened: 2026-09-26
priority: P0
cost: D
parent: CONTACT-5
---


Filed by CONTACT-1's last review pass; pre-existing (the base behaves
the same). A beam resting across two supports — two blocks
`[0,0.1]×[0,0.5]×[0,1]` and `[3,3.1]×[0,0.5]×[0,1]`, a beam
`[−0.2,3.3]×[0.1,0.15]×[1,1.05]` on their tops (the review's `q3` pose,
`long beam across two supports`) — makes coplanar `EdgeEdgeCross`
findings where the beam's bottom edges cross the supports' top edges in
the plane `z = 1`. `TouchSite::of` (`crates/topo/src/census.rs`)
returns `None` for an `EdgeEdgeCross`, so the backstop's `blocks`
reads it as `Undecided::Crossing` and the pair is refused — an
ordinary resting assembly the census cannot clear. A coplanar cross of
two boundary edges is a touch (the two faces' planes coincide there),
and wants the same cone analysis at the crossing point: two wedges.
Cost D.
