---
id: main-red-restate-stored-edge-is-off-the-tier1-mutation-roster
kind: issue
title: Main red: restate_stored_edge is a public mutation path off the tier-1 roster
status: closed
closed: 2026-10-21
pr: 5533
branch: encl/main-red-restate
opened: 2026-10-20
---


**Main red.** Filed by the NURBS orchestrator from NURBS build A's report.

`topo` `review_m1_pr5_internal::every_public_mutation_path_preserves_tier1` (`crates/topo/src/review_m1_pr5_internal.rs:506`) fails on main `911e0c18`. It names `Body::restate_stored_edge` (`crates/topo/src/attach.rs:1362`): a `pub` mutation door added by ENCL's PR 5418 (`543f4866`/`302b6705`, merged 2026-10-20). The door neither declares the tier-1 postcondition nor sits on the roster's allowlist.

Build A found it on #5433, whose `topo/` is identical to main's. Every PR running topo's slow set will read this row red until it is fixed.

**Done when** `restate_stored_edge` either declares and checks the tier-1 postcondition like the other public mutation paths, or goes on the allowlist with the argument for why it can't break tier 1 (description-only, `Kernels(StoredRestatement)`). The row is green on main again.

## Closed

2026-10-21. PR 5533. `restate_stored_edge` is on the allowlist of `every_public_mutation_path_preserves_tier1`, with the same argument as `set_edge_curve` and `describe_at_rest`. The argument this row suggested was not used: at the arena level the door is not description-only. Through `restate_edge` it runs `set_edge_curve_certifying`, which swaps the curve (`replace_edge_curve`: a new curve, and the old one removed if orphaned) and applies pcurve site rows. The `Kernels(StoredRestatement)` panic belongs to the gate, not to tier 1. What preserves tier 1 is that the same certifying path ends with the debug `assert_tier1_postcondition` sweep. A second sweep in the wrapper would only run the check twice. The roster test, the pcurve-posture guard and the two-table guard are green at default eps.
