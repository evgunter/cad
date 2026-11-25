---
id: tier-3-prime-passes-an-edge-lying-along-a-curved-face-of-its-own-solid
kind: issue
title: Tier 3′ reads an edge lying in a face of its own solid only on a plane face, so the rider form along a cylinder's ruling passes
status: closed
opened: 2026-10-26
priority: P3
cost: M
closed: 2026-11-25
---



Found by the CLEAVE lane on `cleave/riders-read-every-edge-in-a-face`
(row `work/cleave/a-boolean-edge-lying-in-a-curved-face-is-not-read.md`).
P3: tooling that prevents a silent bug — the Boolean now parts the
case below, but the gate is what catches the next producer that does
not.

## What

The block `[−2, 2]² × [0, 1.5]` less the tunnel over the `(y, z)`
triangle `(0, 1), (∓.2, .6)` along `x`, less the rod of radius .3 about
`(y, z) = (0, 1.3)` along `x`: the tunnel's apex lies along the rod
wall's bottom ruling, and both its faces leave the line into the
wall's material. On main (`a009c0b046`) `A ∖ B` ships one edge over the
ruling, the rider form D1 refuses, and `validate_geometric` and
`validate_pseudomanifold` both pass it with no records, at every ε
(`sweep/tests/edges_lying_in_a_face_part_wherever_they_lie.rs`,
`a_keel_along_a_cylinder_floors_ruling_parts_in_every_op`, run against
main's slit reading).

The census reads an edge lying in a face of its own solid for sides
only in pass 4's in-plane overlap lane (`census.rs` `ef_overlap_lane`
→ `edge_in_own_face`, `census.rs:3714`, `census.rs:7279`), which
cuts the edge's span in a face's PLANE. A face that is no plane is
never asked, so a face through the edge's material wedge passes there.

## Fix

Ask an edge lying along a ruling of a cylinder or cone face (and a
conic lying on a quadric face) of its own solid for sides, as the
plane lane does: the overlap cells by the face's boundary crossings,
and `edge_in_own_face` at each cell's midpoint, whose links need a
curved sector (`tier-3-prime-reads-no-side-for-an-edge-with-a-curved-face-lying-in-a-face`).

Moved from RESTREAD to CONTACTHOLD on 2026-11-20, by `git mv`, when RESTREAD merged into CONTACTHOLD: RESTREAD held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CONTACTHOLD carries.

## Closed

Closed 2026-11-25 as **fixed**, in the CONTACTHOLD cleanup (`docs/doc-ledger/contacthold-leaves-the-tracker.md`), against main at `6003877ad`. `crates/topo/src/census.rs:4642-4691` `sweep_edge_in_curved_face`, called at :1069. It reads a line edge on a cylinder or cone ruling of its own solid, using `curved_overlap_cells` (:700) and `edge_in_curved_face` (:4767, `FaceThroughEdgeWedge`). The cited fixture now refuses `A ∖ B` `EdgeInFace{CurvedFace}` (`sweep/tests/edges_lying_in_a_face_part_wherever_they_lie.rs:532`). The row's extra "a conic on a quadric face" is carried by contacthold `the-census-reads-no-conic-edge-lying-in-a-face`, which cites `sweep_edge_in_curved_face` as line-only. The fix predates the graft.
