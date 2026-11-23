---
id: loops-reach-reads-an-arcs-start-radial-and-no-row-pins-which-end
kind: issue
title: loops_reach_from is the one reader of a validated arc's recorded radial, and no row pins which end it reads
status: open
opened: 2026-11-23
priority: P2
cost: E
---


Found by PR 7384's second review (BOXFINISH), run by the doors-read
lane on 2026-11-23.

`crates/editor-core/src/mate/reach.rs:205` `loops_reach_from` builds an
arc's cover on a frame whose `x` axis is the arc's recorded START radial
(`radials: [start, _]`, line 221), with the window `[0, |sweep|]` from
there. It is the one reader of a validated arc's recorded radial in the
workspace (the sweep doors spell theirs from the points).

**The mutant**: `radials: [_, start]` (the cover's frame on the END
radial, so the window starts at the arc's far end and runs on past it).
Under it, every `reach|mate|anchor` row passes: `cargo nextest run -p
editor-core -E 'test(/reach|mate|anchor/)'`, 280 / 280. No row reads the
reach of an arc whose far point from the origin lies in one window and
not the other, so the two read alike on every row.

**Owed:** a row pinning the reach of an arc whose farthest point from
the origin lies inside its own window and outside the window the end
radial opens (an arc under a half turn, its far point near its start),
so a reach read off the wrong end moves and the row reds.
