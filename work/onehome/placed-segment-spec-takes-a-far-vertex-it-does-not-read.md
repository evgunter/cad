---
id: placed-segment-spec-takes-a-far-vertex-it-does-not-read
kind: issue
title: placed_segment_spec takes a far vertex q_to it no longer reads; 22 callers pass it
status: open
opened: 2026-11-14
priority: P3
cost: E
---


Filed by ONEHOME's rim-span lane (PR 6953). Since that PR a placed line's carrier comes from its sketch segment and placement alone (the span `‖b − a‖`, the direction `L·(b − a)/‖b − a‖`), and its end is registered against the placed sketch end it builds, as an arc's is. So `crates/sweep/src/swept.rs` `placed_segment_spec_about` reads its `q_to` nowhere (it is `_q_to`), and `placed_segment_spec` passes it through unread. The far vertex is still what the certifier's far-end check (check 3) reads the spec against, through the edge's own vertices, not through this argument.

About 22 call sites pass it (`grep -rn "placed_segment_spec(" crates`): extrude's sides and caps, loft and its stacking door, revolve's chain and full turn, and swept.rs's own rows.

**Owed.** Drop the argument from both, or say at the site why the spec should keep it. BOXFINISH's `boxfinish/profile-directions` rewrites this site and may take it with it.
