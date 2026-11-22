---
id: carrier-frames-the-turned-disc-reading
kind: unit
title: Carrier frames (iv): read check 1 on the turned disc (30° ± 1°) before and after, and pin it
status: closed
opened: 2026-11-17
priority: P1
cost: M
parent: carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes
blocked_on: [carrier-frames-check-1s-frame-half-goes]
closed: 2026-11-22
pr: 7323
branch: onehome/check1-frame-half
---


Closes `check-1-reads-a-turning-frames-length-off-its-bracket` with
the parent. It reads check 1 on that row's turned-disc document
(radius 5e-3, one copy spun about world y by 30° ± 1°, united with a
clear copy) before and after
`carrier-frames-check-1s-frame-half-goes`. It pins that no
`SurfaceDatumUndecided` or `CurveDatumUndecided` finding with
`measure: Length` or `Tilt` remains. It records what then refuses
instead, for the parent row's "the turning-frame row's case builds".

## Closed (2026-11-22, PR 7323)

The reading is pinned as
`crates/sweep/tests/check_1_reads_no_frame_on_a_turned_disc.rs`. On the
moved disc, check 1 carries no datum finding at any `δ` from `1°` to
`10⁻¹⁰°`. Before this PR, the axis length was read off its bracket and
check 1 decided only below about `10⁻⁵°` (probe branch
`onehome/check1-turned-probe`). The row is red where the frame margins
are restored and read off the entries.

What then refuses instead is the edges' and pcurves' readings at first
order in the box. That is
`a-body-moved-by-a-boxed-turn-refuses-its-edges-and-pcurves-at-first-order-in-the-box`.
