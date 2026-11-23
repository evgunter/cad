---
id: two-rods-a-hair-apart-crossing-at-a-shallow-angle-refuse-triangulation
kind: issue
title: Two rods of radius 1 and 0.999 crossing at 0.6 or 0.8, A minus B, refuse Triangulation at every delta
status: open
opened: 2026-11-23
priority: P3
---


Found by PR 7526's review (not re-measured by the unit's implementer). Two rods built as `reach_probes::crossing_rods_keep_no_side_a_neighbour_can_make` builds them (`rod(1, −2, 4)` less `rod(0.999, −2, 4)` turned `θ` about `y`, `A ∖ B`) refuse `TessellateError::Triangulation` at every δ the review read, at `θ = 0.6` and `θ = 0.8`, **on main and on PR 7526's head alike**, so the rows that unit removed are not the cause. The row in `crates/mesh/tests/reach_probes.rs` reads `θ = π/2` and `1.2`, which mesh.

**Owed:** the cells as a row that pins the refusal (`Triangulation`, at which face and which attempt), then the cause read off the raising site in `mesh/src/trimmed.rs` (`TrimmedFace::insert_points` or `constrain_boundary`: a point entering at an existing vertex, or a side realised through one), and the sweep of `θ` between 0.5 and 1.2 that says how wide the class is.
