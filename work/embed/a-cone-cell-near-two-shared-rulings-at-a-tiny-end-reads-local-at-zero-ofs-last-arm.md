---
id: a-cone-cell-near-two-shared-rulings-at-a-tiny-end-reads-local-at-zero-ofs-last-arm
kind: issue
title: A cone cell that reaches a frustum's near-zero end holds both of the rulings the two halves share, so zero_of's last arm reads it Local at every depth: four census tests refuse Departure with no curved exit
status: open
opened: 2026-11-23
priority: P0
cost: M
---


## Finding

From `embed/cone-image` (2026-11-23), tracing the 24 cone/cone tests of
the per-solid census (`measure/gate/census-c2a0c41f40.json`) on the
landing branch `3e433e678` with the gate asking
(`embed/cone-image-measure`, `measure/cone-image/before-detail.txt`).

Four of the 24 hold no `FocalSection` row and take no curved exit, so
they are not the section-ellipse cause the other 20 had
(`curved-presentation-reads-plane-and-cylinder-images-only`). Each
refuses `Uncertified { Departure }` between the two halves of one cone,
across a ruling they share, and both faces read `Local` at `zero_of`'s
arms (`crates/topo/src/injective.rs`), mostly its last one (`_ =>
Local`), at every depth:

| test | payload | `CVL zero_of` last arm / corners arm |
|---|---|---|
| `mesh::all winding_rows::the_cut_frustum_meshes_with_thin_triangles_resolved` | `3v1`/`5v1` across `2v1` | 29,870 / 33 |
| `mesh::all winding_rows::the_whole_frustum_refuses_in_its_window_and_meshes_outside_it` | `3v1`/`5v1` across `2v1` | 29,870 / 33 |
| `mesh::all split_rulings::a_ruled_face_with_a_split_ruling_meshes` | `1v1`/`2v1` across `1v1` | 14,543 / 4,427 |
| `sweep::all a_cone_apex_on_a_plane_through_it::a_cut_along_two_rulings_through_a_wrap_tipped_apex_refuses_the_pinch_by_name` | `2v1`/`4v5` across `1v1`, `3v3`/`4v1` across `15v3` | 1,192 / 583 (and 1,930 at `present_chart`'s `sides_are_the_faces`) |

The traced cells of the first (`CVT`): `3v1` at `u ∈ (6.277, 6.283)`,
`v ∈ (−2.76e-3, −2.83e-8)` against `5v1` at `u ∈ (0.515, 0.522)`,
`v ∈ (−1.38e-3, −2.83e-8)`, near vertices `2v1`, `5v1` and near BOTH
shared rulings (`ex = [2v1, 7v1]`), `Err(Local) | Err(Local)`. The
frustum's small end is `2.83e-8` from its apex in `v`, so a cell that
reaches it is within reach of both rulings the halves share, whatever
its `u`.

**Unchecked:** which combination of zero sides and loose vertices the
last arm met (two zero sides on one axis, or two loose vertices); the
print is on the arm, not on its operands. Whether
`the-apex-census-rows-that-still-refuse-read-cells-short-of-or-off-the-apex`
("a two-ruling corner with no plane") names the same cell.

These four rows are unchanged by `embed/cone-image`
(`measure/cone-image/after-detail.txt`): all four refuse the same pairs.

## Count

4 tests of the census, valid bodies. Cheapest:
`mesh::all winding_rows::the_cut_frustum_meshes_with_thin_triangles_resolved`
(1.1 s).

## 2026-11-23 — a fifth sighting once the section corner is read (`embed/cone-image`)

`mesh::all reach_probes::a_needle_funnel_cut_at_its_apex_refuses_typed`
(2.1 s) left the section-corner exit with `embed/cone-image` and now
ends here: `Uncertified { a: 4v1, b: 6v1, shared: Edge(2v1), arm: Departure }`,
1,739 `CVL zero_of` at the last arm and no curved exit left
(`measure/cone-image/after-summary.txt` on `embed/cone-image-measure`).
