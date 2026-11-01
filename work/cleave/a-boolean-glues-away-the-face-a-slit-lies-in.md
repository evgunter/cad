---
id: a-boolean-glues-away-the-face-a-slit-lies-in
kind: issue
title: A Boolean that glues away the face a slit or ring lies in refuses SlitFaceGlued or LineWedges: the join does not merge the slit's two edges into one wall | wall edge
status: closed
opened: 2026-10-20
priority: P1
cost: M
branch: cleave/boolean-merges-a-glued-slit
closed: 2026-10-20
---


Found by the review of PR 5380 (`cleave/boolean-builds-the-slit-form`).
It supersedes the zero-area-face finding from that review: one row covers
both.

## What

The Boolean builds D1's slit form, so an operand may carry a slit or
ring in a face F: two coincident edges, F | wall a and F | wall b. When
the other solid glues F away (a coplanar face, outward normals opposed),
the slit's two edges should become one edge, wall a | wall b. The join
does not build that.

The probe is the slab `[0, 4]² × [0, 1]` less the trench of
`a_keel_on_the_floor_subtracts_as_a_slit.rs`, then ∪ the slab
`[0, 4]² × [−1, 0]` under its floor, at rest:

| extent | main (rider operand) | PR 5380 |
|---|---|---|
| closed | v14 e21 f10 | `SlitFaceGlued` (the join's ring run along the slit encloses no area) |
| one end | v14 e21 f9 | `LineWedges` Overlap (a two-edge zero-area face on the keel) |
| wall to wall | v14 e21 f9 | `LineWedges` Overlap |

Wall to wall, the join still builds the two-edge face of no area on the
keel, and only PR 5357's result gate catches it (`LineWedges` overlap,
not a slit gap). Main built that probe v14 e21 f9 at volume 30.8, the
slab under (16) and the slab less the wall-to-wall trench (14.8); closed,
it built v14 e21 f10 at 31.7. This
predates PR 5380 for slit operands. The split's side holding the
walls (`split` of the box less the trench by `z = 0`, which builds the
slit form on main) ∪ the same slab refuses on main `Escalated
Corner(Straight)` closed and one end (the tip PR 5380 now reads), and
`LineWedges` Overlap wall to wall. With PR 5380 it gives
`SlitFaceGlued` and Overlap, as the table does.

`SlitFaceGlued` is typed in `crates/topo/src/boolean/join.rs`
(`pinch_ray_refusal`): a degenerate ring run passing a slit's tip, of
either operand (the other's through a vertex pair or the pierces a ring
vertex serves). That is a construction gap with no lever, where the run
used to read as the kernel defect `JoinDesync`.

## Fix

When the face carrying a slit is glued away, pair the slit's two edges'
wall halves into one edge, wall a | wall b. This is the join's re-pairing
of sheets on a line
(`work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`
for the general case). Or kill the zero-area two-edge face the join
leaves, by `kef` on one of its edges. Then pin the probe building as main
does.

## Built (branch `cleave/boolean-merges-a-glued-slit`)

The join cuts the face glued to the slit's floor along the slit's two
edges and keeps the sliver between them, a face of no area: its ring
lane orders a run of no area through a glued slit's tip as the outer
lane does (`join.rs` `glued_slit_roles`), and a section loop whose
regions are only such slivers reads Out, the side of the slit's opening
(`resolve_roles_geometric`, `sliver_along_a_slit`). The output stage
then merges the slit's two edges into one, wall a | wall b, by `kef` of
the sliver (`slit.rs` `merge_glued_slits`), with lineage rows for the
killed edge and the sliver. The zero-area face never reaches the gate.

Measured on main (`763ca322a0`) at ε = default, 1e-6 and 1e-12, five
poses, both operand origins (the Boolean's slit and the split's): every
∪ with the slab under refuses, closed `SlitFaceGlued`, open and wall to
wall `LineWedges`, in both orders; so do `slit ∪ [0,4]²×[0,0.5]` closed
(`SlitFaceGlued`) and `[0,4]²×[0,0.5] ∖ slit` at every extent. Head
builds all 300 of those cells (100 per ε) at their closed-form volume
with tiers 1–3 and 3′, and the slab-under cells at main's rider-era
counts (v14 e21 f10, v14 e21 f9, v14 e21 f9), the keel one edge between
the walls. Pinned by
`a_keel_on_the_floor_subtracts_as_a_slit::a_face_glued_to_the_slit_floor_merges_its_two_edges`.

The merge acts only where the sliver's two edges are the only edges on
their line, so the pairing is forced. That also reaches 34 calls per ε
of `union_flush_onto_edge_contact`, which main refused `LineWedges`
and which now build at that test's own volume, class probes and 3′
(re-pinned; evidence on
`work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`).
The mirror case, where the slit's face stays and its walls go
(`work/cleave/a-boolean-closes-a-slits-face-across-its-line.md`), is a
different construction: it is refused in `recl` before the join, and
the face must close across the line. It is left on that row.
