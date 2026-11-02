---
id: the-first-order-before-in-band-ordering-has-three-hand-written-homes
kind: issue
title: The 'a definite reading answers before an in-band one' ordering is hand-written three times with no shared home
status: open
opened: 2026-10-13
priority: P3
cost: M
---



## What

Three sites hand-write one ordering rule: where a necessary condition
reads in band, a definite reading of another condition that would
refute answers first, and the in-band escalation is spliced as
superseded (`geom_core::k_stats::splice_superseded`).

- `geom_brep::locus::definite_over_in_band` (the tangent locus: an
  in-band axis tilt before a definite gap);
- `geom_brep::certify::run_checks`'s tangent arm (an in-band
  second-order reading spliced superseded where the first-order one
  refuses on its own);
- `topo::boolean::contact_verify::tangent_table` (the second-order
  reading detached and overruled by a refusing first-order reading).

Each detaches, decides and splices by its own spelling. A shared home
(one helper that takes the in-band reading's run and the refuting
reading) would make the rule one place; per PRED's keep-out, landing it
owes an agreement test that the three sites agreed.

Found by the PR 4918 review (2026-10-13); certify.rs is PROPS' ground.

Moved from PREDTAIL to HONEONE on 2026-11-02, by `git mv`, when PREDTAIL merged into HONEONE: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so the two came in under one budget.
