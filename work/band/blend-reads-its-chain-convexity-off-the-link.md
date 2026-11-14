---
id: blend-reads-its-chain-convexity-off-the-link
kind: issue
title: Blend decides its chain convexity with its own triple-product predicate and lever, the same question topo's link answers
status: closed
opened: 2026-10-26
priority: P1
cost: M
closed: 2026-11-04
---


`crates/sweep/src/blend/battery.rs` `convexity_at` (`fillet3_convexity_sign`) decides an edge's convexity as the sign of `(n₊ × n₋)·τ̂`, with its own lever. That is the predicate `Gap`'s `gap_edge_convexity` decided. ATREST's `material-and-convexity-are-read-three-ways` (decided 2026-10-26 on a converged designer pair) makes `topo::link` the one reader of a cell's local material, and it retires `Gap`'s copy. Blend should ask the link: its turn at the edge is one `metric::Distance` reading of the minus sector's real point against the plus sector's carrier, and a tangent edge drops to the second-order pairing and κ_rel layer. Then blend carries no private lever.

Filed by the ATREST orchestrator. It is parked on the link build. Read `work/atrest/material-and-convexity-are-read-three-ways.md` `## Decided` first.

## Closed

Closed 2026-11-04 because its premise no longer holds; nothing was built. The lane `band/blend-convexity-off-the-link` measured before building:

- **Blend holds no private predicate.** `battery::wedge_at` calls `geom_brep::classify_edge_wedge`, the kernel's single edge-wedge door that PR 6344 added after this row was filed. Its lever is `geom_brep::folded_lever_arm` with `topo::wedge_reach`, the same lever `topo::link`'s pairing reads.
- **The link cannot answer the question for most of blend's edges.** `topo::link` holds planar faces and line edges only (its module docs; `FaceSector::of_body_face`, `census::edge_is_line`). At the midpoint of every edge of the blend fixtures (`disc_of_arcs`, the D-bar, the slot, `bored_block_of_arcs`), every edge on a curved face read `Unreadable`, and only plane–plane lines read. Ten of `ARM_ROSTER`'s eleven support pairs involve a curved face, and the smooth-edge door and the knife refusals are tested on curved edges only. The curved arm ATREST plans (`gap-reads-no-curved-cell-below-a-face`) reads points against a sphere or a cylinder carrier, not curved sectors along a curved edge.
- **What remains is ATREST's decision, already taken.** Its `## Decided` keeps `geom_brep::dihedral` as the wedge classifier, as it is. Asking the link on plane–plane edges only would give blend two readers, the reverse of this row's aim.

The probe is at `~/.local/share/cad-work/band-linkconv-scratch/band-linkconv-probe.rs` (local only).
