---
id: cover-kind-match-gate
kind: unit
title: A tree gate: no per-kind Surface or Curve3 match that builds a box, support, reach or ball outside the cover modules
status: parked
opened: 2026-11-02
priority: P1
cost: M
blocked_on: [cover-levers, cover-app-readers]
---


Last unit of the cover build (`cover-geom-readings` has the order and the ratified text).

A `scripts/gates/` check that fails when a file outside the cover modules matches on `Surface::` or `Curve3::` kinds to build a box, support, reach or ball, or writes a literal torus or sphere ball. Per Ev's ask on this design: make the code lead every future instance to the one place. The survey found that name greps miss inline matches, so the gate should key on the shape, not on function names. Its allowlist only shrinks. The door census in `boxes.rs` (`every_door_that_reads_a_box_is_inventoried`) is retargeted to cover readers or retired into the gate (`box-door-census-misses-a-point-free-read`).
