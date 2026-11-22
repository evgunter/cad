(4c implementer) **48b6eed279 is pushed: the last items in one push.** Your four queued notes (19:15, 20:25, 20:42, 21:04) only reached me at 21:23, together. The 21:15 stop is done: the chained digests are killed.

1. **The apex is read once.** The route (`walkable`) now reads the vertex through `topo::readback::vertex_point`, the read behind `Body::vertex_points`; the lane's mesh position is that copy.
   - Both sites decide through `chart_table::at_a_singular_point`, over `Chart::of(surface).poles()`, at the run's one band.
   - The route goes through `at_a_pole`, the lane through `AnalyticRow::is_singular` on the same surface.
   - `tessellate` mints that band once and hands it to `Routes::of` and to `SizingTols`. So it is one function on the same arguments, and an undecided reading on the route's side means the face is not walked. This is written at the second `unreachable!`.
   - **Cells across the band, on this head:**
     - The 0.001° cone split 1e-8 from its apex: `SingularUndecided` at the default ε, meshes at 1e-12, and the profile does not validate at 1e-6.
     - A sweep of the split gap, 81 values logarithmic from 1e-12 to 1e-4, at δ 0.1 and 0.01. Default ε: only gap 1e-8 refuses (`SphereTrimShape`, 2 cells), 80 mesh, and 40 splits are refused by `split_edge`. 1e-12: 104 mesh; 38 refuse `UnresolvedWinding` (gaps 1e-11 to 6.3e-10: needle triangles f64 cannot wind, typed), and 10 splits are refused.
     - **No panic anywhere** (each cell under `catch_unwind`).

2. **`may_make`'s soundness, at the site:**
   - A passed triangle lies within δ(1+s) + a of its surface, where a is the certificate's allowance, 8 ulps of the magnitude, and the verdict passes only while a ≤ sδ.
   - "Decided greater" puts the midpoint past δ(1+s) + Kε − r, where r is the distance's own short-chain rounding. So the exclusion is sound while **Kε > a + r ≈ 6e-12 m at the session box's edge**. Every ε the rows run has Kε ≥ 1e-11.
   - **I cannot show it from the admission floor alone**: (K−1)ε ≥ 1.8e-12 admits ε_min ≈ 2e-13 (Kε ≈ 2e-12). For a body at the box's edge at ε_min, the band may decide "greater" on a side a passed triangle still holds; the census refuses it. The site says so. Your call whether that wants a row.

3. **The 1.3 bore** is in the bored-ball row: 48 cells, in the slow set (0.81 s).
   - Its unsplit body meshes at every δ, so every cut is over-cut, and its threshold falls between δ 0.25 and 0.2.
   - Pins: sign +1 δ 3: 15, 0.5: 21, 0.25: 29, 0.2: 0, 0.1: 0. Sign −1 δ 3 and 0.5: 15, 0.25: 7, 0.2: 0, 0.1: 0. The same at the three ε.

4. **Stale citations:**
   - thin-arc item: the removed `a_ruled_face_with_a_split_ruling_meshes`;
   - split-ruling-certificate item: `lane_of` → `route_of`, and the removed row.

5. **`walk_face`'s loop:** the per-triangle verdict accumulation, its worst and its refusal are now one type, `cert::FaceVerdict` (`read`, `worst`, `refusal`). The rectangle lane, the CDT emit (its `AttemptMeasure` holds one) and the walk all read it. No byte moved: route_rows' bytes rows, d9 goldens, digest rows, curved, errors, split_rulings and trimmed tests: 109/109.
   - What stays per lane: the winding and fold readings. The CDT emit drops a fold and retries, while the walk and the rectangle lane refuse on it, so they are not one loop.

6. **D-prism** (0.5 rad, δ 0.1): three cells pinned. Unsplit meshes; caps' chords alone refuse `UnresolvedWinding` (on main too); every line edge refuses the same here. The doc says it is a regression against main the unit keeps. The thin-arc item has a dated line with the 12 cells.

7. **Filed:** `work/tesslane/a-walked-sides-reach-exclusion-is-read-only-by-the-walk.md` (P3, C).
   - The exclusion is true of every arm that certifies through `cert::verdict` at the run's δ (planar exactly).
   - How often main's rule fires where it would exclude: "not counted".

Checks on 48b6eed279: gates rc=0, fmt clean, workspace clippy (all-features) clean, mesh rustdoc (private items, -D warnings) clean, work lint ok.

Next, on 48b6eed279, each sent as it finishes: `mesh` whole at three ε, mutants, binding census against a wheel, the viewer's slow rows, sweep's meshing modules, and the `mesh`-only digest against 8ad2d3fd0f.
