---
id: vertex-on-edge-is-answered-from-the-claim-not-from-coordinates
kind: issue
title: Check 3 and the join's zip re-read coordinates where the claim already says which edge a vertex is on
status: open
opened: 2026-11-18
priority: P1
refs: [7079]
---

Filed by the ONEHOME orchestrator from fork 288 (PR 7079, approved
2026-11-18), which the designers flagged as needed under every option.

`docs/DESIGN.md` `[placement.frame]` now says a point minted between two
operands "is held as the claim it is ([scalars]): the carrier it was
found on, its parameter there and the reading that found it". Two
readers still answer "is this vertex on this edge" from stored
coordinates instead:

- **Endpoint check (check 3).** `crates/geom-brep/src/certify.rs:47`
  pins `carrier(t₀)`/`carrier(t₁)` against the vertex's stored point,
  so a vertex minted on the edge's own carrier is re-proven against a
  coordinate copy of that same claim.
- **The join's zip.** `crates/topo/src/boolean/zip.rs:29` requires
  zipped vertex pairs to be bitwise coincident, which holds because
  every crossing point is written into both operands' clones. With
  parts in their own frames (stage 2) the two copies stand in
  different frames, so the test fails by construction. It has to
  become "both edges carry the same claim". `boolean/join.rs:2018`
  (`bool_join_chord`) also decides on a coordinate difference.

Done when both answer from the claim, with a row where a vertex
minted on a turned operand's carrier at `Interval` passes check 3
without re-reading its coordinates, and the zip pairs two edges in
different frames by claim.
