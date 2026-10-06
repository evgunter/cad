---
id: replace-face-transports-a-nurbs-edge-as-a-ruling
kind: issue
title: replace_face transports a NURBS edge on a cylinder or cone rigidly, as a ruling
status: open
opened: 2026-10-03
priority: P3
cost: M
refs: [planar-crossing-lane-reads-a-curved-carrier-as-a-line]
---


Filed by REACH from the sweep of
`reach/planar-crossing-lane-reads-a-curved-carrier-as-a-line` (the
"curved carrier read as a line" class). Unmeasured: found by reading,
not by a failing build.

## Finding

`crates/topo/src/replace_face.rs` `transport_curve`: on a cylinder the
arm `Curve3::Line { .. } | Curve3::Nurbs(_)` moves the curve rigidly by
the radial displacement read at `mid`, and on a cone the same arm moves
it by the action's displacement at `mid`. That transport is exact for a
ruling or a generator, which is what a line on either surface is; a
NURBS curve on a cylinder or cone that is not a ruling (a helix, a
fitted section) comes out off the offset surface.

What catches it today, per the module docs, is later: the attach
layer's certification (`set_edge_curve`, `move_points_then_rechart`)
and `validate_closed` refuse with a residual or `ResultNotClosed`, not
a typed scope refusal naming the carrier. So the answer is not silently
wrong, but the refusal names the wrong cause.

## Fix

Split the arm: `Line` keeps the rigid transport; a NURBS carrier is
transported rigidly only when it is certified to be a ruling
(generator), and otherwise refuses typed (`CarrierLaneUnsupported`, as
an ellipse or spiric does), with a row on a helical NURBS edge.
