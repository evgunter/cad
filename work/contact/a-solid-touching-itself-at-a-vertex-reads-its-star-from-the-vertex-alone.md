---
id: a-solid-touching-itself-at-a-vertex-reads-its-star-from-the-vertex-alone
kind: issue
title: A solid touching itself at a point reads its touch star from its vertex there alone, so a second solid resting at that point refuses MixedTouch
status: open
opened: 2026-10-07
priority: P2
cost: M
---


Filed by TANG's touch-only re-read lane.

## What

The census's touch analysis builds each side's star at a vertex-vertex
site from that vertex's own faces
(`crates/topo/src/census.rs`, `Site::stars`: `VertexVertex(a, b) =>
(vertex(a), vertex(b))`). Where the solid also touches itself at the
point, that star is not the solid's material there. Take a plate with a
pyramidal void whose apex touches the top from inside, at a point of
the top with no vertex there. The void's apex star is the void's
complement, which includes everything above the top. The plate's
material there is only the half-space below the top less the void.

So a second solid resting on the top at that point (a pyramid standing
on its apex) is read as passing into the first, and tier 3′ refuses
`CensusUndecidable`, "one passes into the other where they touch". The
first solid alone passes 3′ with its own vertex-on-face record.

## Witnesses

`crates/topo/tests/a_vertex_read_by_two_sector_passes.rs`, at every
pose of `meeting::poses`:
- "standing over the cavity": `cavity ∪ cone`, `cone ∪ cavity`,
  `cavity − cone`. `cavity` is the plate less a pyramid hanging from
  `MEET`, and `cone` is a pyramid standing on its apex there.
- "hanging below the arch": `one − hang`, where `one` is the plate
  united with a standing pyramid and `hang` hangs from `MEET`.

Each of these builds at tier 3, its volumes adding up, and its
material at 32+ points around `MEET` matches the op over the operands'
(`material_holds`). The rows accept this refusal and cite this item
(`three_prime`).

## The shape to give

A solid's star at a point has to be read from every entity of that
solid through the point together: its vertices there, and the edges and
faces passing through it. Here that is the top's half-space less the
void's cone, not either one alone. A vertex star is right only where
nothing else of the solid reaches the point.
