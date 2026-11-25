---
id: issue93-az-rows-run-past-the-fast-set-bar-on-the-gate
kind: issue
title: issue93_az_intersect's az_stencil_intersects_exact and az_plain_interval_refuses_or_encloses read past the fast set's 1 s bar on the gate
status: open
opened: 2026-11-24
priority: P2
cost: C
---


Found by ORBIT's `orbit/tilted-torus-rim` (PR 7617), whose diff touches
neither row's file (`crates/sweep/tests/issue93_az_intersect.rs`, planar).

The kernel job's fast-set time check (step 11) read, on consecutive
heads of that PR:

- run 45279932512 (head `b93ec1bc8`):
  `issue93_az_intersect::az_stencil_intersects_exact` 1.009 s;
  `join2_r2_probes::a_like_far_ends_tie_is_decided_by_the_partner_faces_trim` 1.070 s;
- run 45288991940 (head `d8370849d`, main merged at `a78ae369a`):
  `issue93_az_intersect::az_stencil_intersects_exact` 1.301 s;
  `issue93_az_intersect::az_plain_interval_refuses_or_encloses` 1.100 s.

`az_stencil_intersects_exact` recurs past the bar; the join2 row did not
recur. Settles when the rows run under 1 s again on the gate or move to
the slow set (`.config/nextest.toml`).
