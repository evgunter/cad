---
id: a-slab-on-a-bores-rulings-from-inside-refuses-at-the-join-in-every-op
kind: issue
title: A slab whose edges lie on a bore's rulings from inside the hole refuses at the join in every op (UnpairedLooseEnds, Euler NotSameFace): untyped refusals past a decided edge-sector crossing
status: open
opened: 2026-11-02
priority: P2
cost: M
---


Found by ONEREAD's `oneread/edge-sector-line-cone` (PR 6363's fix pass) while building an edge-sector row for a glued tangency's In side. `sweep::pi_seam_and_kiss_through_the_boolean::the_stadium_answers_every_op` holds only the Out side: there every glued `tangent_lump` at `recl::resolve_edge_sector` reads Out.

## What

`crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs` `a_slab_on_a_bores_rulings_from_inside_answers_every_op` (disabled red, citing this row). The fixture:
- a block `[−3, 3]² × [0.1, 0.9]` with a radius-½ bore along `z`: `rod_z(0.5, 0, 1)` subtracted, the rod turned 45° so the bore's seam rulings leave the slab's;
- a slab `x ∈ [−½, 0]`, `y ∈ [0, ½]`, `z ∈ [0.1, 0.9]` whose top face `y = ½` and side face `x = −½` are tangent to the bore along the rulings its edges `(0, ½)` and `(−½, 0)` lie on.

The edge-sector reading is decided and correct:
- At each such vertex pair `resolve_edge_sector` reads the glued tangent flanker `On`, lumping In (the bore curves away into the hole, so the face lies in the block's material), and the other flanker `Out`: a crossing in every op and both orders.
- Expected volumes are the quarter-disc arithmetic's: `v_block = 28.8 − 0.2π`, `v_slab = 0.2`, `v_both = (0.25 − π/16)·0.8`.

Every op then refuses in the join:

| op | refusal |
|---|---|
| block ∪ slab | `Join(BooleanJoin(UnpairedLooseEnds { count: 4 }))` |
| slab ∖ block | `Join(BooleanJoin(UnpairedLooseEnds { count: 4 }))` |
| slab ∪ block | `Join(BooleanJoin(Euler(NotSameFace { target, ring })))` |
| block ∖ slab | `Join(BooleanJoin(Euler(NotSameFace { target, ring })))` |
| block ∩ slab | `Join(BooleanJoin(Euler(NotSameFace { target, ring })))` |

These are untyped refusals with no recourse (D4), and the Euler one reads as a kernel invariant. The closed sibling `work/tang/two-wedges-sharing-a-tangent-plane-face-on-a-ruling-refuse-unpaired-loose-ends.md` (closed by PR 6253) had the same join symptom on a tangent-plane face's ruling. The slab's faces `z = 0.1, 0.9` are also coplanar with the block's, inside the bore's rims, which may be the trigger. That is not isolated.

## Owed

Find where the join loses the tangent rulings' loose ends here; build every op, or refuse typed with recourse. Then lift the disabled red. It is also the glued-tangency In-side row PR 6363's review asked for (an inverted `tangent_lump` at `resolve_edge_sector` must turn it red).
