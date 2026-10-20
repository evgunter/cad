---
id: main-red-restate-stored-edge-is-off-the-tier1-mutation-roster
kind: issue
title: Main red: restate_stored_edge is a public mutation path off the tier-1 roster
status: open
opened: 2026-10-20
---


**Main red.** Filed by the NURBS orchestrator from NURBS build A's report.

`topo` `review_m1_pr5_internal::every_public_mutation_path_preserves_tier1` (`crates/topo/src/review_m1_pr5_internal.rs:506`) fails on main `911e0c18`. It names `Body::restate_stored_edge` (`crates/topo/src/attach.rs:1362`): a `pub` mutation door added by ENCL's PR 5418 (`543f4866`/`302b6705`, merged 2026-10-20). The door neither declares the tier-1 postcondition nor sits on the roster's allowlist.

Build A found it on #5433, whose `topo/` is identical to main's. Every PR running topo's slow set will read this row red until it is fixed.

**Done when** `restate_stored_edge` either declares and checks the tier-1 postcondition like the other public mutation paths, or goes on the allowlist with the argument for why it can't break tier 1 (description-only, `Kernels(StoredRestatement)`). The row is green on main again.
