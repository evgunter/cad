---
id: the-line-roots-door-is-still-built-in-topo
kind: issue
title: "[topo.homes] arm 1: route's LineRoots door (solid_contain's line × quadric roots, reduce::line_wall_roots_of) takes no Body and is still built in topo"
status: open
opened: 2026-11-03
priority: P2
---


Found by `crossing-doors-leave-topo-for-geom-brep`'s function-level sweep (2026-11-03).

`geom_brep::curve_face::route` marks `Door::LineRoots` built, and the door is `topo`'s: `crates/topo/src/boolean/reduce.rs` (`line_wall_roots_of`, the edge × face arm) over `crates/topo/src/solid_contain.rs`'s root finders (`line_wall_roots`, `line_cone_quadratic`, `line_sphere_roots`, `line_torus_roots`), which the ray lane shares. None takes a `Body` or an arena key. Their escalations name `BooleanDecision::{TorusRoots, SphereRoots, …}`, so they would refuse in their own type as the conic doors now do (`geom_brep::curve_face::circle_roots::DoorRefusal`, `BooleanError::of_door_refusal`), and their `bool_*` K names take the moved module's prefix. `solid_contain.rs` reads Bodies elsewhere, so this is the root finders' move, not the file's.

**Evidence (PR 6488 fix pass):** the edge × face arm's line × torus mapping, `.map_err(BooleanError::of_door_refusal(BooleanDecision::TorusRoots))` in `reduce::line_wall_roots_of` (`crates/topo/src/boolean/reduce.rs`, ~:4969), has no behavioural row; only `offer_rows::SITES` counts it. A line grazing a torus's top parallel at a height in the witness band's escalation gap comes back `Ok(Some(Ok(Err(Unsettled::Tangency))))`, not an escalation, so the fixture that pins `the_wall_root_lane_escalates_as_its_own_decision` for the cylinder has no torus twin yet. When the door moves, its in-door row should name a pose whose margin escalates, and the caller's row should pin `TorusRoots`.
