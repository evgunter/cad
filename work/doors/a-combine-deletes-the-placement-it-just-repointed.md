---
id: a-combine-deletes-the-placement-it-just-repointed
kind: issue
title: combine_in_world deletes a two-shape placement after re-pointing it, when the placement copies two of the combine's operands
status: parked
opened: 2026-10-25
priority: P1
cost: E
blocked_on: [5636]
---

Found while reading `combine_in_world` (`crates/viewer/src/session.rs`) for PR 5693. Not probed. It reads off the code; it is not measured.

`combine_in_world` re-points the first placed operand's placements to the result. It then deletes every placement of every later operand. Take one `Place` whose shapes copy two of the combine's operands, `[A, B]`, and union A and B. That placement is in A's list and in B's. It is re-pointed (shape 0 now reads the union), then deleted as one of B's placements, so the union is drawn nowhere.

It shows only on a multi-shape `Place`, which D′3 retires (one source per placement). If D′3 lands first, close this as absorbed. Otherwise the repair is to skip deleting a placement that the re-point just took over, plus a row on the two-shape `Place`.

Banded P1 E and parked on PR 5636 at PR 5693's merge (orchestrator). It shows only on a multi-shape `Place`, which D′3 retires, so once D′3 lands, close it as absorbed unless it reproduces on a one-source placement.

## Moved (2026-10-26)

AUTHTAIL folded into DOORS (Ev, in chat, 2026-10-26): both were AUTHOR's
successors on the viewer authoring files. Id and body unchanged.
