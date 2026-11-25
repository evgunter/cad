---
id: beside-curve-types-a-cell-by-its-first-unread-edge
kind: issue
title: beside_curve types a cell by the first unread curved edge in its local boundary, so a shared unread edge listed after an unshared one reads Local, not Structural
status: open
opened: 2026-11-18
priority: P3
cost: C
---

## Finding

From EMBED's lane `embed/lune` (2026-11-18), building
`injective::posed_rows::a_pose_is_not_tried_where_a_shared_edge_is_no_conic`.

`beside_curve` (`crates/topo/src/injective.rs`) ends by typing the cell
from `curved.iter().find(..)`: the first curved wall, in the order of
the cell's local boundary, that no presentation reads, returned as
`unread(i, near_edges)`. A cell whose local boundary lists an unshared
unread curve before a shared one therefore reads `Local`, though the
shared one is unread at every depth and the cell is `Structural` by the
function's own doc ("a curved edge no presentation reads, when shared,
is not either"). Swapping the two in `Local::edges` flips the answer.

**Cost, not soundness:** `Local` only splits on, so a pair `stuck` would
refuse at once runs on to the depth floor instead. Measured on no body;
the unit row orders its local boundary shared-first to reach the
`Structural` it pins.

**The fix** is to prefer a shared unread edge: `Structural` where any
curved wall that no presentation reads is on `near_edges`.
