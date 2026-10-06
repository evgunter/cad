---
id: pinch-notch307-row-escalates-bool-join-nearest-at-eps-1e-6
kind: issue
title: pinch_faces_tessellate's notch307 row refuses at eps 1e-6: its subtract escalates bool_join_nearest, so every PR seeding sweep is red on the eps row
status: open
opened: 2026-10-06
priority: P1
cost: E
---


Found by the `shell/planar-gate-misses` lane (PR 4115), whose CI eps row
(`test (eps 1e-6, 1e-12 …)`, run 37425269825) went red on it. **Measured**:
the row fails identically with `origin/main`'s `crates/topo/src/shell.rs`
swapped in, so it is not that PR's, and it is deterministic.

`CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p sweep --test all a_face_through_two_vertices_on_one_point_tessellates`
(at `origin/main` `cadf2ed1`'s tree plus PR 4115):

    notch307 fib117 edge psi=1.9 cp S: the op builds no body:
    Err(Escalated { decision: Coincidence(Join, Moot), diag: Indeterminate {
      margin: MarginDiag(Value(-5.196042020649827e-6, None)),
      band: Band { zero: 1e-6, escalate: 9.999999999999999e-6 },
      predicate: Some("bool_join_nearest") } })

`crates/sweep/tests/pinch_faces_tessellate.rs` (tcost/tint ground) landed
on 2026-10-06 in the `mesh: pinch …` commits; the row's subtract has a
`bool_join_nearest` margin of about 5.2e-6, which sits in the 1e-6 band's
ambiguity window. Either the row is an eps-specific fixture that wants a
`loud_skip_marker!` at coarse eps (if the near-coincidence is the row's
design), or the join escalation is the defect. Every PR whose diff seeds
`sweep` runs this eps row in its gate, so until this is settled such PRs
are red on `gate ok`.
