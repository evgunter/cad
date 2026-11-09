---
id: viewer-boolean-tool-cannot-pick-a-read
kind: issue
title: The viewer's boolean tool picks nodes, so it cannot author a union of a split's two halves or an indexed read
status: closed
opened: 2026-10-10
priority: P1
cost: M
closed: 2026-11-09
---


DM4 makes a union's or intersect's members READS, and a read can be one port
of a node (a split's `above`) or one member of a family (`xs[i]`). The viewer's
boolean tool (`crates/viewer/src/combine.rs`, `BooleanTool::pick`) holds picked
NODES and `BooleanSpec::Union(Vec<RecipeNodeId>)`, so it cannot author the
unit's headline case, a union of a split's two halves (two reads of one node),
nor a member read by index. DM4's "a union or intersect seat of N body picks"
is met in count only. The fix is the tool holding the read a pick reaches (the
drawn body's variable, as `world::seat_of` already resolves for a viewport
pick) rather than its node.

Raised by review r2 of PR 4527 (style S18).

## Closed

**The tool holds the read a pick reaches.** Each boolean seat — a
union's or an intersect's member, a subtraction's body kept and body
removed — holds a node and its `Port` (`crates/viewer/src/seats.rs`):
a split's half by its port, as before, and now a family's member by its
index values (`Port::Member`). A viewport pick on a drawn member of a
pattern, or on it through a copy of the family, reads that member
(`world::seat_member`, over `world::members_of`, the evaluator's member
enumeration); a tree pick names the node, so it reads the family whole
(`Port::Family`, D10 `[repetition]`), and the panel offers each member
by its indices to read that one instead (`BooleanTool::choose_member`,
`pane::create::member_rows`). `BooleanSpec`
(`crates/viewer/src/session/author.rs`) holds `BodyRead<Formula>`
operands, the shape the union, intersect and subtract nodes take, so a
member lowers as `BodyRead::indexed(pattern, [count(i)])` — the
spelling Python's `IndexedRead` lowers to; editor-core's authored
`BodyRead` already spelled it, so no door changed. The held-picks line
says each read: `Pattern 1[1]`, `…, every member`, `…, above half`.

The session door gates an indexed operand on its read being a family
(`NodeKindWanted::Instances`, the part door's gate), places a member in
another space as the member rather than re-targeting its family
(`in_one_space`), and takes nothing over for it (`combine_in_world`).
What that leaves undrawn-correctly for a family copied into the world
is a takeover ruling, filed as
`work/doors/a-combine-of-a-world-copied-familys-member-leaves-the-copy-drawing-it.md`.

Seats of the other tools (split, mirror, pattern, duplicate, clearance)
read a family whole, as before (`Seat::reads_member`). Selections are
untouched.
