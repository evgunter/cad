---
id: two-rods-a-hair-apart-crossing-at-a-shallow-angle-refuse-triangulation
kind: issue
title: Two rods of radius 1 and 0.999 crossing at 0.5 to 0.9, A minus B, refuse Triangulation on a plane face
status: open
opened: 2026-11-23
priority: P3
---


Found by PR 7526's review, not re-measured here. Two rods built as `reach_probes::crossing_rods_keep_no_side_a_neighbour_can_make` builds them (`rod(1, −2, 4)` less `rod(0.999, −2, 4)` turned `θ` about `y`, `A ∖ B`) refuse `TessellateError::Triangulation`, **on main and on PR 7526's head alike**, so the rows that unit removed are not the cause. The refusing face (`1v1`) is a **plane**: the refusal is the planar lane's (`mesh/src/planar.rs`), not the trimmed lane's. Each reviewer's cells as measured:

- One reviewer: `θ = 0.6` refuses at every δ that run read; `θ = 0.8` refuses at δ 0.3 and 5·10⁻² and meshes at δ 5·10⁻³ and 10⁻³, on both trees.
- The other reviewer: the pair refuses over `θ` 0.5 to 0.9 — at every δ read through `θ = 0.8`, at δ 0.5 and 0.1 at `θ = 0.9` — and meshes at `θ = 1.0`.

The two runs disagree at `θ = 0.8` below δ 5·10⁻²; which δ each read there is part of what is owed.

The row in `crates/mesh/tests/reach_probes.rs` reads `θ = π/2` and `1.2`, which mesh.

**Owed:** the cells as a row that pins the refusal (`Triangulation`, on the plane face, at which attempt), then the cause read off the raising site in the planar lane, and the sweep of `θ` between 0.4 and 1.2, and of δ at each, that says how wide the class is.
