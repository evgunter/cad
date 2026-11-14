---
id: a-cut-by-a-tool-whose-pieces-also-meet-at-a-line-refuses
kind: issue
title: A cut by a tool whose solids rest on each other refuses ToolSeatUnread where two solids its fuse joins also meet at a point or along a line
status: open
opened: 2026-11-12
priority: P2
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

## Measured (2026-11-12, PR 6860's fix pass): narrowed to records inside a fused group

The first build refused every lower-cell record between two of the tool's solids, which was a regression against main: a record between two solids the fuse never joins refused too. That case now rides, as on main. The fuse folds only the solids its verdicts join, one group at a time. A solid no verdict joins stands as it is, carved out of the tool with its keys. A record between two solids that stay apart rides into the cut as a record of the material (`nary::tool_parts`, `material::assembled`).

Pinned at rest and turned in `crates/topo/tests/a_vertex_through_a_declared_seat.rs`, `a_piece_touching_the_seats_plate_at_a_line_or_point_rides_its_record`. The tool is the bolt on the plate, plus a block beside the plate's top edge (an edge–edge record and two vertex–edge records) or on its corner (a vertex–vertex record). Main was run on `6b2ffc3f4b`:
- **Same body as main** wherever main built: the cut off the seat, the cut flush with the plate's side, the cut round the corner (the vertex record carried, as on main) and the cut under the plate.
- **Builds where main did not**: a pin through the seat and a block over half the stack build the cut by each piece in turn. Main refused `ToolSeatCrossed`, or kept the seat's crack with its record (16 faces against the fold's 15).
- **Same refusal as main**: a block over the shared edge refuses `LineWedges`, as on main and as the fold does.

**What remains is reachable.** A block resting on the side of a bolt that stands on the plate, the block's bottom edge lying along the plate's top edge. The census records the two seats and an edge–edge record between the block and the plate, so the fuse would join the three into one solid with that edge inside it. The cut refuses `ToolSeatUnread` naming the record (`a_lower_record_inside_what_the_fuse_joins_refuses`). A union that decides the edge itself builds one solid of 14 faces, and the block cut by it builds 8 faces, 0.072. So the refusal is a limit of the given-verdict fold, not of the geometry. Re-banded P3 → P2: a tool of three boxes reaches it.

The "Done when" now reads: the edge inside the fused group is taken as given, for instance a given point or line touch handed to the step that joins the block, and the bracket is cut by its material.
