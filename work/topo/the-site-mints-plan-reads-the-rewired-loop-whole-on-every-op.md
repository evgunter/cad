---
id: the-site-mints-plan-reads-the-rewired-loop-whole-on-every-op
kind: issue
title: The site mint's plan still reads the rewired loop whole on every op (presence, chart box, element), so N ops on one face cost O(N²) reads
status: dispatched
opened: 2026-10-04
priority: P3
cost: M
refs: [euler-site-mint-re-walks-the-rewired-loop-on-every-op]
---


Found closing `euler-site-mint-re-walks-the-rewired-loop-on-every-op`.
Since the elements change (R, PR #4037), `site_rows` in
`crates/topo/src/pcurves.rs` derives and certifies only what a door
creates. The plan still reads every half-edge of each rewired loop on
every operator:
- the loop walk `Body::plan_site_rows` builds in `crates/topo/src/euler.rs`;
- `stored_rows`' presence reads and chart boxes, through `site_rows_from`;
- the chart box of every image, for the certification window;
- every element, for the winding.

So N operators on one minted face whose loop grows with each still cost
O(N²) reads. Measured on `cyl_wall_sheet` with N struts after N rim
splits (debug build, shared box): 0.008 / 0.025 / 0.086 / 0.351 s at
N = 25 / 50 / 100 / 200. That is roughly ×4 per doubling, at a third of
the cost before the change.

It keeps the P3 of the row it replaces: the reads are cheaper, but the
asymptotics that row's title named are unchanged.

A fix would keep a face's window and a loop's winding as data that an
operator updates by its delta rather than re-reads. The window is the
harder of the two, because it enters certification verdicts.
