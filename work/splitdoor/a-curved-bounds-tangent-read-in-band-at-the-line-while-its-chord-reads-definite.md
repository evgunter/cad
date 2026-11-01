---
id: a-curved-bounds-tangent-read-in-band-at-the-line-while-its-chord-reads-definite
kind: issue
title: A curved bound's tangent reads in band at the face line while its chord's far vertex reads definite: the plane-offset reading's cost is unprobed for curved bounds
status: open
opened: 2026-11-01
priority: P2
cost: S
refs: [main-red-a-prism-tilted-within-the-zero-escalates-at-the-sector-reach-since-arm-vs-reach]
---


Found by the FULL review of PR 6212 (`splitdoor/prism-tilt-reach`), claim 3.

## What

PR 6212 reads two faces' line un-normalized (`boolean/sectors.rs` `rays_within`). Against a bound `e` at its reach `L`, the reading is `L·((n_a·e) cos φ − n_b·e)`: the line's offset from the other face's plane, read along the bound's **tangent**. On faces near parallel, a bound a small angle off their line now reads in band wherever that far point lies within about ε of the other plane. Where the unit line read the line outside the sector, the pair used to be no record; it is now a refusal (`sectors::tests::a_bound_in_band_off_the_others_plane_reads_no_wrong_pair`).

For a **straight** bound the review showed that nothing which built before is newly refused. The chord's far vertex is the same point, and on both heads a planar body refuses there first, at reduce's `bool_vertex_face_side` (probes `crates/topo/tests/rv_vertex_touch_probe.rs` on `splitdoor/prism-tilt-reach-review`).

A **curved** bound breaks that coincidence. `rays_within` reads its departure tangent at its extent, while its side code (`side_code`, `Reach::Conic`) and the vertex–face pass read the edge itself. A tangent read in band at the line can stand beside a chord whose far vertex reads definitely off the other plane. Then the sector pass refuses where no later pass would, a refusal main did not make.

## What closing it takes

Build the pose: an arc bound bending off the faces' line, its tangent's far point within about ε of the other face's plane and its far vertex definitely off it. Run a full Boolean on both heads.
- If PR 6212's head refuses where main built sound, the line's reading against a curved bound has to read the edge as its code does.
- If both heads refuse, or both build, record the pose as the row's witness and close it.
