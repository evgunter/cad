---
id: tipped-rod-join-escalates-at-1e-12
kind: issue
title: a_tipped_rod_whose_origin_is_stored_far_joins_along_its_rulings is red at CAD_TOLERANCE_EPS=1e-12 on main (carrier_matches_mapped_source escalates)
status: open
opened: 2026-10-07
priority: P2
cost: E
---


Seen by the BAND lane of `band/reach-meters-an-end-face-away-from-its-vertex`
running `CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep`; it fails
identically with `origin/main`'s sources (the suite calls no blend).
The row was added by the TANG fix pass (PR 4231,
`crates/sweep/tests/parallel_cylinder_join.rs`,
`a_tipped_rod_whose_origin_is_stored_far_joins_along_its_rulings`, the
`the cut rod` expect):

    CrossingInsertion { operand: A, edge: EdgeKey(3v1), source: Certification {
      error: Escalated { check: MappedSource, sample: 3, cause: Indeterminate {
        margin: 1.2533307724993392e-12, band: { zero: 1e-12, escalate: 1e-11 },
        predicate: Some("carrier_matches_mapped_source") } } } }

The margin sits ~1.25e-12, inside the 1e-12 row's escalate band: the
far-stored origin's carrier check reads at f64 resolution there. Either
the check's lever must absorb the stored origin's distance, or the row is
eps-sensitive and wants a loud skip at that row — decide which.
