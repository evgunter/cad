---
id: the-conic-quadric-door-is-still-built-in-topo
kind: issue
title: "[topo.homes] arm 1: route's ConicQuadric door (boolean/conic_quadric) takes no Body and is still built in topo"
status: open
opened: 2026-11-03
---


Found by `crossing-doors-leave-topo-for-geom-brep`'s sweep (2026-11-03).

`geom_brep::curve_face::route` marks `Door::ConicQuadric` built, but the door is `topo::boolean::conic_quadric::conic_quadric_roots` (`crates/topo/src/boolean/conic_quadric/mod.rs`), whose signature `(carrier, t0, t1, surface, band)` takes no `Body` and no arena key. The circle root cores it reads now live in `geom_brep::curve_face::circle_roots`, beside the torus doors.

What the move needs that the torus doors did not: the door picks its escalation's decision by surface kind (`quadric_ladder` returns `BooleanDecision::ArcSphereRoots` or `ArcCylinderRoots`; `cone_roots` names `ArcConeRoots`). In `geom-brep` it would refuse in `circle_roots::DoorRefusal`, and the boolean's two callers (`reduce::wall_crossing`, `trace::circle_crossings`) would name the decision from the surface's kind, as they now do for `ArcTorusRoots` (`BooleanError::of_conic_door`). Its rows (`conic_quadric/{circle_sphere_rows, circle_wall_rows, cone_rows, ellipse_rows, in_band_sweep, shallow_sweep}.rs`) move with it, its `bool_conic_quadric_*` and `bool_conic_cone_*` K names take the module's prefix (a census note, as 2026-11-03's), and `offer_rows::SITES` loses its `conic_quadric/mod.rs` lines.

`crates/topo/src/boolean/trace.rs` (`ray_crossings`, `circle_crossings`, `carrier_extent`, `plane_misses`) takes no `Body` either and reads this door and the torus door; it follows the door, its refusal in its own type rather than `BooleanError`.
