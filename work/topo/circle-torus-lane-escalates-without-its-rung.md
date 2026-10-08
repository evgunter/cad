---
id: circle-torus-lane-escalates-without-its-rung
kind: issue
title: topo: the circle × torus lane's escalation names its predicate only, so its plane-height rung (a length) cannot offer the tolerance it gives
status: open
opened: 2026-09-30
---


(TOPO, PR 3513's third fix pass: the review's m-4.)

## What

`boolean::circle_torus::circle_torus_roots` returns a bare
`Indeterminate` from every rung it asks: the plane-height depth
(`bool_circle_torus_plane_height`, `depth − charge`, a length that
passes positive), the contour residuals and side (`ρ − R`, a length
that passes either sign), the coaxial tilt and offset (lengths), and the
quartic's own rows (`bool_circle_torus_disc`, `_shape`, `_depth`, `_odd`,
`_split`, the pole and root-slack rows), whose margins are no length the
user chose. `reduce::wall_crossing` wraps it as
`BooleanDecision::ArcTorusRoots` (`crates/topo/src/boolean/reduce.rs`),
which ends on its lever alone (`LeverPass::ByRung`,
`refusal_routes.rs`): the escalation does not say which rung refused, so
no one pass set or unit is the decision's. The raise at the plane-height
rung (`reduce::declaration_order_rows::the_circle_torus_lane_escalates_as_its_own_decision_and_no_declaration_settles_it`,
an arc `mid` under a tube's crown) is a length the user may intend, and
D4 ¶1 (i) would offer the tolerance it gives there.

## Repair shape

Carry the rung: `circle_torus_roots` returns a typed escalation (a
closed rung type, as `solid_contain::WallRootFault` carries `WallRung`),
and `wall_crossing` routes each rung to its own ending: the length rungs
sized from their own pass sets, the quartic's rows on the lever alone
(clause-(i) debt, #214), as `TorusRoots` does.
