(iso-run-density implementer) The four interim items are done and held locally as 638e764c03, on top of cfacd1584f, on top of the frozen a79bf304d5. Nothing is pushed.

1. **One home for "no rows".**
   - `tessellate_trimmed` no longer special-cases the cylinder. Every analytic arm now runs `nu = pole_side_columns(..)` and `rows = a.lattice_rows(..)`.
   - A cylinder has no singular point, so `pole_side_columns` returns `columns` unchanged. Behaviour is identical: mesh 566/566 at the default ε, d9 goldens and route_rows digests included.
   - Mutant, in `chart_table::AnalyticRow::lattice_rows` only (main's rows back):
     `Analytic::Cylinder { radius } => { let n = ceil_count(v1 - v0, radius * hu)?; Ok((1..n).map(|j| lerp(v0, v1, j, n)).collect()) }`
     - On the fixed tree it reds all three rowless rows. For example, "a notch across the seam, δ 1e-1: p 24" and "δ 1e-2: p 275". It also reds `chart_table::the_cone_arms_rows_stand_a_certified_chord_apart`.
     - On the frozen head's `trimmed.rs`, the same mutant reds only that `chart_table` row; the three rowless rows pass. That confirms the review's finding.
   - A caution: a mutant that just folds the cylinder into the sphere arm's `ceil_count(v1 - v0, hv)` is equivalent, not a mutant. `steps()` gives a cylinder `hv = ∞`, so it still lays no row.

2. `trimmed.rs` now cites `AnalyticRow::trimmed_metric` where it cited `AnalyticRow::metric`. No other stale `::metric` reference remains in `crates/` or `docs/`.

3. **Mesher errors in `two_holes`.** Confirmed: `rowless.rs` has no `Result`, `map_err`, `ok()` or `unwrap_or` left.
   - The probe's `mesh::tessellate` is `.unwrap()`, and both cuts go through `less`, which unwraps.
   - The row calls `counts`, which panics on a mesh error or a `check_mesh`/geometry error.
   - So every mesher refusal on any path is a failure.
   - `try_less` is gone too: `less` is the original unwrapping form again.

4. **Rods issue.**
   - The title drops "at every delta". The body states each pair by its own reviewer's run:
     - θ 0.6: refuses at every δ that reviewer read.
     - θ 0.8: refuses at δ 0.3 and 5e-2, meshes at 5e-3 and 1e-3, on both trees.
   - "Found by PR 7526's review, not re-measured here" stays. "Owed" now asks for the δ sweep at each θ.
   - The PR body's line on it (my draft's line 80) still says "at every δ". That one is yours to edit.

Checks: fmt, mesh clippy (all-targets, all-features), run-gates, work.py lint all pass; the rowless rows pass at 1e-9, 1e-6 and 1e-12.

Holding for both deltas.
