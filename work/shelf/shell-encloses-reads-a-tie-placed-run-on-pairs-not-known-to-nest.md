---
id: shell-encloses-reads-a-tie-placed-run-on-pairs-not-known-to-nest
kind: issue
title: encloses' chart read lands a full-period run on the floor's branch at a tie, on pairs not known to nest
status: open
opened: 2026-10-13
priority: P3
cost: M
refs: [periodic-branch-floor-remains-at-the-raw-picks]
---


Found by PCERT's `periodic-branch-floor-remains-at-the-raw-picks` (PR
4941's review), by reading. Not measured.

`crates/topo/src/shell.rs` `chart_read` moves each later run by whole
periods until its mean `u` is within half a period of the first run's,
by an open-coded `τ·⌊(at − mean)/τ + ½⌋`. `encloses` then decides
`shell_rim_nesting` on the runs' mean radii about their centroid.
- Its doc says the pairs it compares are concentric by construction,
  and for those the inner's mean lies inside the outer's span, so no tie
  arises.
- Its callers also ask it of pairs NOT known to nest:
  - the rim pairing asks both orders before trying a band seam (`ring_first`);
  - `pair_rings` asks both orders before refusing "do not nest".
- There, two full-period runs can have means half a period apart. A tie
  then lands the later run on whichever branch the floor rounds to,
  moving it a whole period across the flat picture and the centroid with
  it, and the nesting decision reads that placement.

Whether such a reading can ever say "encloses" for a pair that does not
nest is not shown either way. What would close it: an argument at the
site that the callers' following steps (the band seam, the typed
refusal) make the tie's answer harmless, or a refusal on a tie with a
row that reaches it.
