---
id: main-red-loop-keys-guard-flags-spline-region-rows
kind: issue
title: main red: live::tests::loop_keys_is_read_only_where_a_miss_is_reported flags boolean/spline_region_rows.rs, which 925fa061ab (the seam row reads a face's loops through Face::loop_keys) added
status: closed
opened: 2026-11-03
priority: P1
cost: E
closed: 2026-11-12
---


Filed by the ONEDOOR orchestrator on 2026-11-03. A ONEDOOR lane (`onedoor/carrier-and-window`, PR 6480) saw this red after merging main and reproduced it on main's tree. It is not that branch's.

`crates/topo/src/live.rs`'s guard `loop_keys_is_read_only_where_a_miss_is_reported` lists the sites allowed to read `Face::loop_keys`. Main's `925fa061ab` ("splice: the seam row reads a face's loops through Face::loop_keys") added such a read in `crates/topo/src/boolean/spline_region_rows.rs`, and the guard reds on it.

The fix is the guard's: either the new row reads the face's loops through the door that reports a miss, or the site joins the guard's list with its reason. Which one is SPLICE's call.

## Closed (2026-11-12): overtaken on main

This row sat unmerged on ONEDOOR's orchestrator branch. By the time it landed, `crates/topo/src/boolean/spline_region_rows.rs` on main no longer read `Face::loop_keys`, and main's kernel suite was green. It lands closed, so the record of the red is kept.
