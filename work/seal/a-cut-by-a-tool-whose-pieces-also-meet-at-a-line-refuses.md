---
id: a-cut-by-a-tool-whose-pieces-also-meet-at-a-line-refuses
kind: issue
title: A cut by a tool whose solids rest on each other and also meet at a point or along a line refuses ToolSeatUnread
status: open
opened: 2026-11-12
priority: P3
cost: M
refs: [a-cut-across-its-tools-seat-leaves-a-sliver-between-the-seats-two-sections]
---



Filed by SEAL's lane on `a-cut-across-its-tools-seat-leaves-a-sliver-between-the-seats-two-sections`, which built the cut that reads its tool as material (`crates/topo/README.md` `[topo.cut-material]`).

## What

Every solid of such a tool is a member of the fold that fuses it (`crates/topo/src/boolean/nary.rs`, `tool_parts`). A record within one solid rides with its member. A planar face–face record between two solids is the step's given verdict. Any other record between two solids refuses `ToolSeatUnread` naming it: a vertex–vertex, vertex–face, vertex–edge, edge–edge or edge–face record.

Such a record is a touch between two pieces the union is about to fuse. A step's own inputs can carry only each operand's records, and a touch between the accumulation and the joining member would be the union's to decide. Taking it as given needs a given-verdict form for a point or line touch, which `GivenDecision` does not have.

Not measured: whether the census records a lower-cell record between two solids beside a face–face record of the same pair. For the bolt on the plate, which records one face–face record and nothing else, it does not.

## Done when

A tool whose solids rest on each other over a planar area and also meet elsewhere along a line is cut by its material, or the refusal is pinned with a measured fixture and its reason.
