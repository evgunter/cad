---
id: ef-bound-backed-migrates-to-region-confinement
kind: issue
title: ef_bound_backed's declared face-pair arms are grandfathered (region-unconfined); their migration — CONTACT-12's dropped step 2 — waits on INTENT stage 4, which retires declared pairs
status: parked
opened: 2026-10-08
priority: P3
cost: M
blocked_on: [declared-pairs-retire]
---


Split off `overlap-lane-boundary-crossing-cuts` when CONTACT-12 was re-scoped under the D10 hold (2026-10-08). CONTACT-12 lands the boundary-crossing cuts this migration was blocked on.

The migration itself is still to do: re-attempt MATE-9's region-confined `ef_bound_backed` under the measured-migration protocol (`crates/topo/README.md`, Grandfathered rungs). The anomaly pin `review_mate4a_r2_probes::r2_an_unrelated_declared_pair_backs_the_ef_bound` is its guard.

It waits on `declared-pairs-retire` and is likely moot there. When that row lands, re-read this one against the built code and close it if the rung is gone. CONTACT-12's dual review (R1, MINOR-5) found that the parking existed only in prose, with no row of its own.
