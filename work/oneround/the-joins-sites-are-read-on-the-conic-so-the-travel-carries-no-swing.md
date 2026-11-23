---
id: the-joins-sites-are-read-on-the-conic-so-the-travel-carries-no-swing
kind: issue
title: The join's travel reads each site on the conic at the conic's own parameter, so its off-conic swing goes
status: open
opened: 2026-11-22
priority: P2
cost: H
refs: [7410, 7141]
---



Filed by `oneround/readings-not-levers` (the unit
`the-ten-readings-that-are-not-levers-are-decided-by-what-they-are`),
which left this out of its scope: `docs/DESIGN-FORK-LOG.md` row 303
names it as the join's final state, "the read on the conic its own row
under the records condition".

## What

`crates/topo/src/boolean/join.rs` `travel` decides `turned_past`, a
site's side of the plane through the conic's axis read as arc, across a
swing `off_conic_slack`, `2ε cot ψ`
(`geom_core::k_stats::decide_across`). The swing stands for two sites
each known only within the band of the conic: a site `δ` off the conic
across it moves the axis-plane reading by `δ cot ψ`. It is 9.9 bands at
the flank of an ellipse of aspect 10 and 60 at aspect 60, so partners
closer than that along the conic escalate (`bool_join_arc_clear`, or
the reading's own row where the two together stand in band) while they
are apart in space by far more than the band.

The swing belongs to the reading, not to the sites. A site read on the
conic, at the conic's own parameter (its foot on the conic), is known
along the conic to the band with no `cot ψ` factor, and two such
parameters order by their difference metered at the conic's speed, as
`bool_join_foot_travel` already orders two feet on a carried section.

## Why it is its own row

`Turn` holds a site, its tangent and the conic's axis line, and no
conic: reading a site on the conic needs the conic (or its parameter at
the site) in the record. That changes what a record holds, which is
Ev's condition on PR 7141 for the records unit, so it is decided there
and not inside a conversion of the reading's door.

## Related

`work/sect/the-travel-slack-could-be-read-away-near-the-site.md` (a
tangent projection near the site, the axis plane far off) and
`work/sect/the-travel-slack-charges-a-partner-at-the-sites-cot-psi.md`
describe the same amplification from the reading's side; a read on the
conic answers both.

## Done when

`travel` carries no swing: each site is read at its parameter on the
conic, `off_conic_slack` is gone, and `travel_rows`' two dozen bands at
aspect 60 order instead of escalating.
