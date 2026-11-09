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

**The tool holds the read a pick reaches.** Each boolean seat holds a node
and a `Port` (`crates/viewer/src/seats.rs`). The seats are a union's or an
intersect's member, and a subtraction's body kept and body removed. A port
is two things: its output, and its members.
- **The output** (`Output`) is the node alone, a port, or a split's half,
  which a tree pick of the split waits for.
- **The members** (`Members`), at a seat that reads one member
  (`Seat::reads_member`, an exhaustive match), are read off a family that
  nothing reads along a seen path (`world::is_seen`):
  - a viewport pick on a drawn member reads that member by its indices
    (`world::seat_member`, over `world::members_of`, the one enumeration
    `members_read` and `body_count` now read too);
  - a tree pick reads every member, and the panel offers each member
    (`pane::create::member_rows`, one button each up to `MEMBER_BUTTONS`,
    else a member number and a read button).

A split of a family reads one half of one member, and the half and the
member can be chosen in either order. `BooleanSpec`
(`crates/viewer/src/session/author.rs`) holds `BodyRead<Formula>`
operands. A member lowers through `combine::member_read`, which the part
form shares, the spelling Python's `IndexedRead` lowers to. editor-core
already spelled it. The panel line says the read: `Pattern 1[1]`,
`…, every member`, `Split 1[1], above half`.

The session door gates an indexed operand on a family
(`NodeKindWanted::Instances`; `admits_read` now counts a variable
evaluated per member as one, as `admits` counts a node). `in_one_space`
places a member as itself. A family something copies is read whole, its
copy taken over, as before member reads.

**Filed:**
- `work/doors/a-combine-of-a-world-copied-familys-member-leaves-the-copy-drawing-it.md`
  (P1): member reads of a copied family, which wait on DOORS' takeover
  ruling.
- `work/doors/the-split-mirror-and-pattern-seats-read-a-picked-members-whole-family.md`:
  the other body seats.
- `work/refs/a-split-of-a-family-has-no-halves-a-read-reaches.md`: the
  evaluator refuses `split.above[k]`.
- `work/placing/a-pattern-of-a-copy-is-read-as-a-copy-of-its-source.md`:
  `copy_source` on a repeating placement in a copy space.

Selections are untouched.
