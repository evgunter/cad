---
id: subtracting-a-glued-tip-reverts-its-edge-on-face-line-into-the-rider-form
kind: issue
title: ∖ reverts a tool's edge lying in a face (glued tip, records carried) into the rider form, and every tier passes it
status: closed
opened: 2026-10-14
priority: P2
cost: M
refs: [revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet, census-passes-a-face-inside-an-edges-material-wedge]
closed: 2026-10-26
---

Found by the sweep of
`reverting-a-slit-body-overlaps-its-coincident-edges-wedges-and-passes-every-tier`
(branch `cleave/revert-repairs-coincident-edges`), measured at rest.

## What

The tool: `union` of the slab `[0,4]×[−1,3]×[0,1]` and a triangular
prism, profile `(y,z)` = (1,1), (1.6,3), (0.4,3) along x from 1.5 to
3.5, its keel resting on the slab's top. Two solids, three records
(edge-on-face, two vertex-on-face), tier 3′ passes.

`boolean_op_with(Subtract, a, tool, records_b: tool's records carried)`
with `a` the slab `[−1,5]×[−2,4]×[0,1.5]` (volume 37.85) or the box
`[−1,5]×[−2,4]×[−1,4]` (161.6, the void door) builds one solid with 3
records, and tiers 3 and 3′ pass it. Through the `subtract` façade
(records dropped) the same results fail 3′ only as
`UnrecordedContact`.

The complement of the glued tip pairs the sheets differently: the
material either side of the keel, between the slab's top and one prism
wall, is two creased sectors (the slit form, here an island slit ring
in the pocket's floor). `Body::revert` flips sides and keeps the keel
one edge wall a | wall b lying in the floor, its wedge `2π −
2·atan2(0.6, 2)` ≈ 326.6° reaching under the floor: the boolean's
rider form (`work/cleave/a-boolean-builds-a-reflex-edge-lying-inside-a-face.md`).

`Body::revert` refuses coincident edges since that branch; it cannot
see this line, which only the tool's records name.

## Owed

Until the re-pairing
(`revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet`)
covers an edge lying in a face, the ∖ doors refuse typed where the
reverted part keeps an edge-on-face record (and a face–face line
touch). Blocked on 4795: under today's text the rider form is not yet
a defect.


**Unblocked (CLEAVE, 2026-10-19).** `[ev]` PR 4795 merged (`0a702a9bc1`). Ev ratified the D1 sentence in `crates/topo/README.md` tier 3: where more than two sheets meet on a line, the material decides the edges, and the complement re-pairs. Radial edges were declined.

## Note from RESTREAD (2026-10-20): tier 3′ reads it

Branch `restread/face-in-edge-wedge`: tier 3′ refuses the rider form
`FaceThroughEdgeWedge`. Merged with the Boolean's slit parting
(`topo::boolean::slit`), both results (the slab and the box round the
tool) build and pass tier 3′
(`crates/topo/tests/a_face_inside_an_edges_wedge_refuses_at_tier_3_prime.rs`,
`a_glued_tip_subtracted_with_its_records_passes_tier_3_prime`).

## Closed (2026-10-26, tracker sweep)

Fixed on main, by the Boolean's slit parting (`topo::boolean::slit`, `part_riders` in the output stage) together with RESTREAD's tier-3′ wedge check, as the 2026-10-20 note above records. The row's own construction is now a gating row. `crates/topo/tests/a_face_inside_an_edges_wedge_refuses_at_tier_3_prime.rs` `a_glued_tip_subtracted_with_its_records_passes_tier_3_prime` (`:211`) builds the same tool (the slab `[0,4]×[−1,3]×[0,1]` united with the tip), carries its records into `boolean_op_with(Subtract, ..)` against the same slab and box round the tool, and asserts `validate_pseudomanifold` passes on the slit-form results. The rider form the row was about is refused at tier 3′ (`FaceThroughEdgeWedge`, the same file's keel row), so "every tier passes it" no longer holds. What is still owed for coincident edges in general, the re-pairing, stays on `revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet`.
