---
id: self-overlapping-spines-build-and-validate
kind: issue
title: A loft or sweep whose spine revisits itself (a planar arc past a full turn) builds a self-overlapping body and every validation tier says Ok
status: open
opened: 2026-09-16
refs: [2752, 368]
---

Found by both of BOOL-6's reviews (PR 2752) and filed by the S-BOOL
orchestrator on sweep ground (`crates/sweep/src/loft.rs`). The
per-slab stacking fold correctly admits every spine whose per-slab
turn stays under π, which includes a unit-radius planar arc curled
9.0 or 13.0 rad over 17 stations — past one and two full turns — where
stations three or more apart come within less than the section's own
width. The body builds; `validate`, `validate_closed` and
`validate_geometric` all return Ok; only `LevelIndex::contains`
notices ("2 level rings claim … the body overlaps itself there"). The
old end-to-end π wall excluded every self-overlapping circular spine
by accident; nothing downstream refuses one now, and the kernel has
never claimed a self-intersection gate. BOOL-6 pins the fact in one
named row (`the suite's self-overlap row`) and says so in its docs
rather than presenting the builds as wins. The question for the owner
is whether a loft/sweep door should refuse a self-overlapping spine
(a per-station clearance decide against the section's extent, or a
tier-3 self-intersection check) or whether self-overlap is a legal
body the certifying door must name. Measured, not acted on;
difficulty M.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to BLEND (the sweep crate and the profile fillet door are BLEND's charter; crates/sweep/src/loft.rs passes to BLEND at this exit) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.
