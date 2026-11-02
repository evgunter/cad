---
id: min-separation-floors-by-boxes-not-directions
kind: issue
title: min_separation prices each pair's floor by its boxes' separation, so near-parallel walls tilted against the axes floor the bracket low where a support along a direction would raise it
status: open
opened: 2026-11-01
---

Found by DIRSUPPORT's sweep (2026-11-01). DIRSUPPORT gave the bound
sweep's distance test a second compare, `clearance_support_gap`
(`crates/topo/src/clearance.rs` `apart_along`, `support_separation`):
a lower bound on two cells' separation read off their supports along a
direction. On the Klein loop's facing walls it takes the pair count
from 12575 to 1141.

`min_separation` does not read it. Each pair's contribution to the
bracket's lower end is still the box separation's `lo`:

- the root pass, `root_separations`;
- `Bracket::pair` (`self.floor = self.floor.min(sep.lo())` and the
  `above(sep.lo())` prune);
- `Bracket::overflow`, for an abandoned pair.

`max(box lo, support lo)` is a lower bound too, so the floor and the
prune could read it. The incumbent `hi` is read off stations and is
unaffected. On walls tilted against the axes the box `lo` is about
zero until the cells are about as small as the gap, so the bracket's
lower end stays low at `DEFAULT_MAX_MIN_SEP_PAIRS` (512).

Not measured. A row that measures it would bracket the Klein loop's
3v3 against 6v1 at the default budget and report `lo` with and without
the support.
