---
id: a-pair-past-the-band-beside-a-shallow-wedge-refuses-on-its-budget
kind: issue
title: a pair that parts past the band beside a shallow wedge refuses on its budget
status: open
opened: 2026-11-24
priority: P2
cost: M
---

Two valid shapes whose faces part by more than the band at their far
side are refused by the embedding question with
`Refused(Budget { Depth 40 })`, at PR 7666's head (`273f765de5`,
ε = 1e-9, K = 10). Tier 3 and the census pass both.

- **Two faces sharing only a vertex.** A probe body (branch
  `embed/run-probe`, `crates/topo/src/run_probe_embed.rs`, the taper
  body at a = 1e-3, commit `76c60cd198`): a face T whose edge leaves the
  plate P at their one shared vertex at about 5e-8 rad, so the two stay
  within ε of each other for about 20 mm and stand five bands apart at
  the far end. `embedded` on {T, P}: `Refused(Budget{Depth 40, a: T,
  b: P})`; the whole body refuses the same way. It is the vertex-shared
  analogue of the nine-contact body's sector sliver, which shares an
  EDGE and is certified.
- **Two faces sharing an edge.** A reviewer's neighbour of the sector
  sliver (PR 7666's first review, probe `review_probe_sliver_neighbours`):
  10 m long with 20ε between the faces at the far end, where the
  reading is about two bands: `Refused(Budget Depth 40)` at faces 8v7
  and 9v3. The same sliver at 500ε is certified.

In both the reading that would certify is past the band, and the walk
runs out of depth before it is taken: a cell clear of the shared cell
is read by distance, and beside a shallow wedge those cells stay inside
the band for a long way. Not measured: which cells the walk spends its
depth on in each, and whether one cause covers both (a vertex shared
alone keeps its own run; the edge case may be the walk's order alone).

P2: no kernel verb builds either shape today (both were assembled by
hand), and the verdict is a refusal, not a wrong `Embedded`; it matters
as soon as the lever beside a shared cell changes (`[ev]` PR 7673),
since both designers there expect the 20ε sliver to certify.
