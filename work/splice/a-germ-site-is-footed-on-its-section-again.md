---
id: a-germ-site-is-footed-on-its-section-again
kind: issue
title: The join foots each germ site on its section loop again (join.rs germ_branch, carried_seats, along_facing): whether the crossing that placed the site already holds that parameter is unchecked
status: open
opened: 2026-11-03
priority: P3
cost: M
refs: [joining-a-spline-carrier-is-unbuilt, a-meeting-on-a-spline-boundary-edge-has-no-split-parameter]
---

Filed by the spline-join lane (`splice/spline-join`, PR 6434). It is a
lead from that PR's sweep, not a defect anyone has measured.

## What

The spline join and the split-parameter row are one pattern: a door
asks a carrier for a parameter that its caller already holds. The join
now carries its far parameter instead (`edge_join::carried_far`). A
sweep for the pattern's other spelling, a foot on a carrier, found three
sites in `crates/topo/src/boolean/join.rs` that foot a germ site on its
section loop:

- `germ_branch`, through `.foot(site, band)`;
- `carried_seats`, through `section.foot(site, band)`;
- `along_facing`, through `section.foot(p, band)`.

The site was placed by a crossing reading, which may hold the site's
parameter on the section already. Where it does, the foot is a second
reading of a value in hand. It can disagree with the first in the band,
and on a loop that winds near itself it can land on another branch.

## What it needs

Trace each site back to the reading that placed it. Where that reading
holds the section parameter, carry it as the split and the join do.
Where it does not, record why the foot is the only reading. Nothing is
known to go wrong today.
