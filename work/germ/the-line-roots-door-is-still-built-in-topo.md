---
id: the-line-roots-door-is-still-built-in-topo
kind: issue
title: "[topo.homes] arm 1: route's LineRoots door (solid_contain's line × quadric roots, reduce::line_wall_roots_of) takes no Body and is still built in topo"
status: closed
opened: 2026-11-03
priority: P1
cost: M
parent: curve-surface-crossing-has-no-kind-pair-table
pr: 7179
branch: germ/line-roots-door
closed: 2026-11-20
---


Found by `crossing-doors-leave-topo-for-geom-brep`'s function-level sweep (2026-11-03).

`geom_brep::curve_face::route` marks `Door::LineRoots` built, and the door is `topo`'s: `crates/topo/src/boolean/reduce.rs` (`line_wall_roots_of`, the edge × face arm) over `crates/topo/src/solid_contain.rs`'s root finders (`line_wall_roots`, `line_cone_quadratic`, `line_sphere_roots`, `line_torus_roots`), which the ray lane shares. None takes a `Body` or an arena key. Their escalations name `BooleanDecision::{TorusRoots, SphereRoots, …}`, so they would refuse in their own type as the conic doors now do (`geom_brep::curve_face::circle_roots::DoorRefusal`, `BooleanError::of_door_refusal`), and their `bool_*` K names take the moved module's prefix. `solid_contain.rs` reads Bodies elsewhere, so this is the root finders' move, not the file's.

**Evidence (PR 6488 fix pass):** the edge × face arm's line × torus mapping, `.map_err(BooleanError::of_door_refusal(BooleanDecision::TorusRoots))` in `reduce::line_wall_roots_of` (`crates/topo/src/boolean/reduce.rs`, ~:4969), has no behavioural row; only `offer_rows::SITES` counts it. A line grazing a torus's top parallel at a height in the witness band's escalation gap comes back `Ok(Some(Ok(Err(Unsettled::Tangency))))`, not an escalation, so the fixture that pins `the_wall_root_lane_escalates_as_its_own_decision` for the cylinder has no torus twin yet. When the door moves, its in-door row should name a pose whose margin escalates, and the caller's row should pin `TorusRoots`.

**Moved to GERM (2026-11-20):** claimed as U1 of the curve × surface
crossing reading (`curve-surface-crossing-has-no-kind-pair-table`): the
door moves into `geom_brep::curve_face::line_quadric` with no behaviour
change, and the ray lane's own cone rungs and its `About::Nearest`
posing stay in `solid_contain` for U1b. The torus twin this row asks
for is `line_quadric::door_rows::a_line_through_a_thin_tube_escalates_as_the_torus_arm`
(the door's row) and
`reduce::torus_root_tests::the_torus_root_lane_escalates_as_its_own_decision`
(the caller's, pinning `TorusRoots`).

## Closed (2026-11-20, PR 7179)

The line × quadric door is `geom_brep::curve_face::line_quadric`, with no behaviour change. Its K names take the `line_quadric_` prefix, and `bool_point_in_solid_denom` is split between the wall's axis-parallel rung and the ray's plane arm. Its refusal type is narrowed per arm. The style review approved it, and a four-item fix pass followed. Left to U3a: the door's public API still takes an untyped `&geom::Surface` and answers a pose (`LineRoots::Constant`, the apex rung), and the census rows for its predicates still live in topo.
