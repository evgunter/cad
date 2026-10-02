---
id: ring-clearance-probe-counts-two-decisions-on-a-ringless-host
kind: issue
title: the ring-clearance probe counts two fillet3_ring_clearance decisions on the base rim's ring-free host, red on main
status: open
opened: 2026-10-02
---

Found by PCERT's `pcert/at-rest-rows-mandatory` (PR 3759) while running
sweep's suite; red on origin/main at `9b2d0528e` as well, so it is
main's and not that branch's.

`sweep`'s `review_ring_clearance_r1_probes::recorded::r1_ring_clearance_decisions_per_carve`
(the `probe` feature) pins `fillet3_ring_clearance` decisions per carve
on a revolved boss's three rims at `[0, 1, 2]`. The base rim (a
hostless, ring-free host) now records **2**:

    assertion `left == right` failed: rim (1.0, 0.0)
      left: 2
     right: 0

The last change at the site on main is the merge of
`band/annulus-host-outer-metered` (`56f4c0025`), which metres the
annulus host's outer cycle; whether that is the row's cause, and
whether the new count is right (re-pin) or a decision asked of a host
with no ring (fix), is this program's call.
