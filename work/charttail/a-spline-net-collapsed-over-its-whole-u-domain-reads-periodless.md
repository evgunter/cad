---
id: a-spline-net-collapsed-over-its-whole-u-domain-reads-periodless
kind: issue
title: a spline net whose whole u domain moves a point under the band reads every joint periodless, where it may be a sliver surface that should refuse
status: open
opened: 2026-11-22
priority: P3
cost: M
refs: [a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint]
---


A lead raised by the collapsed-column design fork (design-fork row 296),
filed by the `charttail/collapsed-column` lane and not measured.

`crates/topo/src/pcurves/decision.rs`'s `singular_at` keeps a net-level
reading after the collapsed sides: a spline chart whose whole `u` knot
domain moves a point under the band (`net_reach`) reads
`Singular::Collapsed`. Every joint on it is then read periodless, and
its chart gap decides. Such a net is a surface within the band of a
curve, a sliver. DESIGN `[extensibility]` (ratified on PR 7287) says no
band decides a collapse, and that a side that is tiny but not one
point is a regular chart with a short edge. A sliver surface may need
to refuse rather than be read through.

## Fix

Find a door that builds such a net, if one does. Decide whether
`Singular::Collapsed` should refuse the face typed, as a sliver. If no
door builds one, delete the arm.
