---
id: the-polish-basin-meter-levers-by-the-lines-own-span
kind: issue
title: The curve-pair polish's basin meter levers its sine by the line's own span reach, so a split line can refuse where the whole decides
status: open
opened: 2026-10-15
---

Found by the §5 sweep for PR 5154's lever fix (the FUSE orchestrator ruled option A on 2026-10-15: a line pose's tilt is levered by where the spans can meet, the overlap of their projections, not by a span's own length, because a verdict that depends on how a sweep split an edge is the defect).

`line_line`'s and `line_conic`'s tilts now lever by the overlap. One lever of the same shape is left: `line_conic` hands `Polish` the lever `reach(a).max(k.speed_hi())` (`crates/geom-brep/src/curve_pair/pose.rs`, `line_conic`), and `Polish::closest` levers the basin meter `sine − turn` by it (`row::BASIN`). A line split into pieces has a shorter reach, so the meter's margin is smaller. Whether it refuses `Unsettled::Meter` can therefore depend on the split: a refusal, never a wrong answer, but not a function of the geometry alone.

The conic × conic tilts (`conic_conic`, `transverse`) lever by the conics' semi-axes, which no split changes. They are not of this shape.

Owed: lever the basin meter by the region it measures (its `region`, already the line's band stretch about the crossing), or argue why the span's reach is the right scale, with a split-invariance row like `meet_rows::two_lines_read_whole_or_in_pieces_meet_alike`.
