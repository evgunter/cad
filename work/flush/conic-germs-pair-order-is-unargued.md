---
id: conic-germs-pair-order-is-unargued
kind: issue
title: The free pair order (first_pair) is argued for straight germs only: two conic-germ poses still escalate on bool_join_nearest between mutual pairs
status: open
opened: 2026-10-12
---


Found by FLUSH's two-pinch proof lane (PR 4769) in its fix pass.
`boolean::join::first_pair` reads the order among a round's pairs as
free only where three things hold: every germ's nearest partner is
mutual, every germ is straight, and the band has `K > 2`. Its doc
comment carries the argument. The straight premise is what makes a
germ's scan rank its candidates by one number each, the chord, with
each reading the difference of two of them. That is what lets a scan
that has lost non-winning candidates keep its winner.

Along a conic, `nearer_along` ranks differently. Two sites within
`bool_join_chord`'s zero of each other read as one site and fall to
the chord. Otherwise the sign of `turned_past` is read from the
*incumbent's* site, cleared of `off_conic_slack`. That margin
approximates arc length only near the incumbent (`travel`), so it is
not a difference of one number per candidate, and the chained-tie
bound in `first_pair` does not carry over as written. Whether it is
1-Lipschitz in the site is unargued.

Two poses with all-mutual conic germs still escalate on
`bool_join_nearest` between disjoint pairs:
- `crates/sweep/tests/band_apart_partners_on_a_steep_ellipse.rs`,
  `k = 10`, at every ε (margin −3.3 zero). With the conic arm free,
  this pose went on to `interval_span_forward`
  (`work/issues/an-ellipse-arcs-span-is-levered-by-its-global-speed-floor.md`).
- `crates/sweep/tests/germ_plane_torus_frame.rs`,
  `a_nearest_tie_in_band_on_a_bar_across_the_hole_escalates_by_name`,
  at ε 1e-6. With the conic arm free, it reached the dispatch.

## Done when

Either the conic ranking is argued to keep a scan's winner as
candidates go (one number per candidate up to a bound that `K`
absorbs), and `first_pair` admits conic germs with a conic row in
`first_pair_rows`; or a pose is found where a conic tie moves the
matching, and the escalation is pinned as honest.
