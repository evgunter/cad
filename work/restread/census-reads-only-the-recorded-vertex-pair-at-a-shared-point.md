---
id: census-reads-only-the-recorded-vertex-pair-at-a-shared-point
kind: issue
title: At a point several vertices share, the census reads only the vertex pair a record names, so a rest there can clear an unrecorded cross-solid pair on the same key that crosses
status: open
opened: 2026-10-18
priority: P1
cost: M
rides_with: a-pinch-point-the-census-reads-as-mixed-touch-where-the-body-holds-no-overlap
---


Found by the two designers weighing
`a-pinch-point-the-census-reads-as-mixed-touch-where-the-body-holds-no-overlap`
(2026-10-18); both confirmed it against the code.

`census::pair_vertex_vertex` (`crates/topo/src/census.rs`) emits a
finding only when the two vertices are *not* on one point key
(`!geo.same_point(ka, kb)`) and the pair is unrecorded. The pair-level
clear (`blocks`) reads standing findings and the pairs records name
(`recorded` → `declared.vv`, `declared.vf`). So a cross-solid vertex
pair that shares a point key and carries no record is never read by the
touch analysis.

Shape: one solid with two cones at `v` (two cubes united corner to
corner), and a third solid's corner recorded against one cone. The rest
read at the recorded pair clears it, while the unrecorded pair, which
may cross the other cone, is never read. The boolean does not build that
body, but tier 3′ reads the body alone, so an import or a surgery can
present it (likely; not built). Unchecked: whether solids meeting *only*
at an unrecorded shared-key pair leave the point unread altogether
(trace `meets_found` and the material probe).

A false clear at tier 3′, hence P1. The fix is the point-level unit the
pinch fork is deciding (each solid's whole link at a point, every cycle
read, recorded or not). Whichever answer is taken there must close this;
until then it stands on its own.
