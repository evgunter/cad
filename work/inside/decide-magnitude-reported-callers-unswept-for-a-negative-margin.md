---
id: decide-magnitude-reported-callers-unswept-for-a-negative-margin
kind: issue
title: The 12 topo callers of decide_magnitude_reported are unswept for a margin that can go negative, where it panics with unreachable!
status: open
opened: 2026-11-01
priority: P3
cost: E
---


Filed by the FLUSH orchestrator from PR 6174's check pass (2026-11-01).

`geom_core::decide_magnitude` and `decide_magnitude_reported`
(`crates/geom-core/src/k_stats.rs:954`, `:973`) take a margin that is
"nonnegative by construction". Past −escalate, both panic with
`unreachable!`.

PR 6174 found one caller that broke this premise: `RayHit::pair`, whose
margin `|Δat| − slack_a − slack_b` can go negative. It now decides with
a signed `decide`. PR 6174's lane then swept the 11 plain
`decide_magnitude` call sites in `topo`. It found one more premise
break, `curved_edges.rs:182`'s `hi − lo` at `Interval`, filed on this
slate.

The 12 `decide_magnitude_reported` sites in `topo` were not swept. The
check pass found none that takes a slack-reduced margin, but did not
show each margin nonnegative.

**What closing it takes:** for each of the 12 sites, show that the
margin is nonnegative by construction, or move it to a signed `decide`.
A panic on valid input is the wrong failure under `[failure.taxonomy]`.
