---
id: a-fillet-on-a-drawn-unions-operand-is-drawn-nowhere
kind: issue
title: A fillet on an operand of a drawn union (a tree pick) is inserted and drawn nowhere: the takeover re-points placements only
status: parked
opened: 2026-10-25
priority: P1
cost: M
blocked_on: [5636]
refs: [the-viewer-cannot-author-a-mirror]
---


Found by the mirror arrival fork's designer pair (2026-10-25; likely, by reading `add_blend`). The fillet gesture's takeover re-points only `placements_of_var(body)`. A body drawn only as an operand of a drawn union, picked in the tree, has no placements of its own, so the fillet is inserted and shows nowhere, and nothing says so. The same holds for any feature on one piece of a takeover's result, such as the mirror's `B ∪ m`.

**Shape of the fix.** The generalised takeover re-points the reader on the path the body was seen through: here the union's operand slot. After D′3 a pick's path (`VarDef::Pick`) spells that slot. Before then, the union's operand can be found by `world::seat_read`'s inverse; decide when this is dispatched.

## Moved (2026-10-26)

AUTHTAIL folded into DOORS (Ev, in chat, 2026-10-26): both were AUTHOR's
successors on the viewer authoring files. Id and body unchanged.
