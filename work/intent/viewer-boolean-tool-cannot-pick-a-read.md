---
id: viewer-boolean-tool-cannot-pick-a-read
kind: issue
title: The viewer's boolean tool picks nodes, so it cannot author a union of a split's two halves or an indexed read
status: open
opened: 2026-10-10
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
