---
id: kev-describing-a-null-member-leaves-its-face-missing-its-rows
kind: issue
title: kev_describing gives a listed null member its first description without the re-mint set_edge_curve now runs
status: open
opened: 2026-09-30
priority: P3
cost: E
---


Found by `mev-null-leaves-a-complete-curved-face-half-minted`
(branch `topo/null-edge-remint`) in its sweep of the doors that
describe a null edge's carrier.

That unit makes a null edge's first description re-mint a minted
face its halves are on through the site mint (`pcurves::site_rows`,
selected by `StoredRows::remints`): every loop no other null edge
holds open, and the whole face, whatever it misses, once no null edge
is left on it. `Body::set_edge_curve`
(`crates/topo/src/attach.rs`) plans it
(`null_description_rows`, through `pcurves::site_rows`) before it
mutates. `describe_at_rest` refuses a null edge
(`NullScaffoldCurve`).

`Body::kev_describing` (`crates/topo/src/euler_kill.rs`) is the one
other door that writes a certified curve onto an existing edge
(`replace_edge_curve`, bypassing `set_edge_curve`). A merged
member it lists may be a null edge — `kev_describing_gate` checks
membership, adjacency and certification, not the curve's kind — and
then the kill is that edge's first description, and the face its
halves were on keeps missing the rows the description would have
re-minted.

Not reached today: instrumented over topo's suite and sweep's `ci`
profile, no `kev_describing` call lists a null member (the blend's
two listing sites list chord members; `zip`, `rest`, the revolve and
`splitting::reassembly` list none). Closing it needs the planner
stated over the loop as the kill leaves it (the kill's own site
description), not as `set_edge_curve` finds it; or the gate refusing a
listed null member, if no caller should list one.
