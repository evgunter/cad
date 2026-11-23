---
id: ray-crossings-read-the-selections-lever-as-the-rays-window
kind: issue
title: Ray crossings read the selection's reach lever as the ray's window and start
status: open
opened: 2026-11-23
---


`topo::solid_contain::selection_reach` folds the selection's faces' far distances from `q` into one `Lever` (`face_reach_from` per face, `Lever::max`). `ray_crossings` takes that lever as `arm` and binds `let reach = arm.length()`. It reads `arm` correctly as a lever (`Margin::levered(denom, arm)`), but it also hands the bare `reach` on as geometry: to `carrier_roots` and `line_wall_roots` as the run of the ray's line to search, and to `edge_meets`, `crosses_by_turn` and `wall_crosses_by_turn`. `topo::side_reader` places the aimed ray's start at `reach.length() * -2` (near line 750). A lever read back as a length is a margin's business (`geom_core::Lever`'s doc, "the exit, not an entrance").

**Wanted:** the selection is held as its region, a ball about `q` (`crate::cover::face_ball_about` per face, folded), and the ray's window and start are read from that ball (`geom_brep::ExtentBall::run_on`), with the lever kept for the margins. `FaceGeo::PartialCone` and `FaceGeo::PartialWall` already hold their face's ball this way (the `sizes-are-read-back-out-of-extent-levers` unit).
