---
id: subtract-of-a-hollow-operand-files-the-island-under-one-solid
kind: issue
title: subtract(A, hollow B strictly inside A) files B's cavity as a second Outer shell of A's solid instead of a solid of its own
status: open
opened: 2026-09-08
priority: P0
cost: H
---


Measured by the SHELL-5 lane (PR #2159, 2026-09-08) and placed here by
the SHELL orchestrator: `topo::subtract(box 6³, shell(box 2³ at
(2,2,2), 0.25))` — B a hollow body strictly inside A — returns `Ok`
with ONE solid and THREE shells: A's outer (`Outer`, volume 216), B's
outer (`Void`, −8) and B's cavity (`Outer`, +3.375), all under one
solid; tier 3 green; total volume 211.375, which is the correct
number. The material inside B's cavity is a connected component of the
result's material disconnected from A's wall, so it is a solid of its
own, not a shell of A's solid. The containment fallback hands
`insert_void` the whole of B — the door's docs say "positively
oriented single-solid closed body" and its graft attaches every shell
under the destination solid — so after the door's reversal B's cavity
lands as an OUTWARD-facing shell of A's solid. `shell` on a hollow
operand meets the same one-solid state transiently and re-homes each
void twin with `Body::move_shells_to_new_solid` (SHELL-5); the boolean
fallback could re-home the same way, paired off B's own shell roles.
Tier 3 does not catch the shape (see TOPO's
`tier-3-does-not-check-shell-roles-per-solid`), which is why it
validates. Signed (SHELL orchestrator).

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to CURVED (its charter names S-BOOL's ceded ground and inherits at S-BOOL's exit) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.

## 2026-09-24 — the shape is valid at rest (ATREST-7)

The sentence above, "Tier 3 does not catch the shape (see TOPO's
`tier-3-does-not-check-shell-roles-per-solid`), which is why it
validates", no longer gives the reason. Tier 3 now reads the nesting
(check 10, `ValidationError::ShellWinding`) and admits this body on the
merits: inside the island the shells wind `+1 − 1 + 1 = 1`, so the
island is material and the solid's shells bound winding 0 or 1
everywhere (`crates/topo/tests/shell_winding.rs`,
`an_island_inside_a_void_of_its_own_solid_certifies`). What this row
asks for is the GROUPING — the island filed as a solid of its own —
which is the boolean's output convention to pursue, not an at-rest
invalidity.
