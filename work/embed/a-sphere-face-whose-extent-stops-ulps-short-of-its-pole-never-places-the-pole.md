---
id: a-sphere-face-whose-extent-stops-ulps-short-of-its-pole-never-places-the-pole
kind: issue
title: A sphere face whose stored extent stops ulps short of its pole never places the pole, so a cell at the pole reads Local at every depth and six valid f64 cavities run out of pairs
status: open
opened: 2026-11-18
priority: P0
cost: M
---

Traced by lane `embed/cavity-trace` (2026-11-18) for
`the-f64-cavity-gate-refuses-thirty-two-valid-shells`. Main `da49c63f`,
f64, the cavity asked through a local, uncommitted `CavityGate` switch,
with `topo::injective` and `topo::clearance` instrumented (uncommitted).

## Evidence

Six valid f64 cavity rows refuse `Budget(Pairs { max_cell_pairs: 65536 })`,
in two cavities.

**The two-arc sphere** (four sphere faces, radius 0.95, axis `y`):
- `shell7_seam_corner::a_two_arc_sphere_shells_to_its_closed_form` (the
  cheapest, about 20 s);
- `shell9_probe::sphere_grafted_body_is_tier_3_valid_before_and_after_the_closing_mint_which_shell_runs`;
- `shell9_r2_probes::r2_e2e_two_arc_sphere_hollows_and_tessellates`.

All three read the same receipt: 45,928 discharged, 19,608 refused
(the frontier left at overflow), 65,526 splits.

**The dome** (two sphere faces, radius 0.45, beside two cylinders and a
plane), in three `shell_curved_mouth` rows:
- `a_tangent_dome_shells_at_its_tangent_circle`;
- `opening_a_tangent_dome_refuses_two_equidistant_roots`;
- `a_dome_short_of_tangent_shells_at_its_upper_root`.

Each reads about 45,850 discharged and 19,700 refused.

## Trace

**Which pair spends the budget.** In both cavities nearly every split
is one touching pair: the two sphere faces `1v1 × 2v1`, which share both
meridian halves of a great circle and its pole vertex `1v1`. That pair
splits 84,624 times on the two-arc sphere and 85,074 on the dome
(`Split { Departure, Edge(1v1) }`), down to depth 34–35. Every other
pair certifies within a few dozen cells. The pair never certifies,
which is a different defect from one that certifies slowly.

**What each face presents there.** In 76,382 of the pair's cells, both
faces read `Err(Local)`. The cells sit at the pole: `v` up to
1.5707963267948963, each face's cell in the middle of its lune in `u`.
- **The `Local` is `zero_of`'s.** The shared vertex `1v1` is loose: no
  zero side holds it, and it is no single corner of two clips.
- **The pole is never placed.** `present_chart`'s sphere-pole reading
  finds `1v1` at the north pole (`vertex_at_singular_point` answers
  `Ok(true)`). But the rectangle has already been clipped to the face's
  sheet, whose `v` top is the stored extent's, 1.5707963267948963. That
  is 1 ulp below `π/2`'s enclosure, whose low end is
  1.5707963267948966, the `f64` nearest `π/2`.
- **So the clip is skipped.** `north && y.1 < line.lo()` takes the
  `continue`: the cell is read as not reaching the pole, though the pole
  vertex is in its reach, near the shared cell.
- The dome is the same: 161,301 `zero_of` `Local`s (the dome's run
  counts both faces' calls; the two-arc sphere's figure counts cells
  where both faces read `Local`). The same `v` top, `1v1` at the pole.

**The missing reading.** A face whose extent stops within the rounding
of its chart's singular latitude still has the pole in its closure. Its
pole vertex is placed by `vertex_at_singular_point`, and its meridians
end there, so the collapsed side `v = π/2` is the face's. Where the
extent's bound and the pole's latitude are within a few ulps, read them
as one, as the sheet's other few-ulps concessions do (`sheets`, "the
touch"), or place the pole off the vertex rather than off the clipped
rectangle. Either way the collapsed side becomes a zero side at the
shared vertex, and the pair has `planes_through`'s great-circle plane.

**Not traced by that lane:** where the stored lift's `v` top comes
from, and whether PR 7086's `face_extent` reads it; both are traced in
the section below. This is the same mechanism in both cavities, and
pre-dates PR 7086: these six rows were out of pairs on 2026-11-13 too.

## Traced and fixed on `embed/pole` (EMBED, lane pole, 2026-11-18)

Main `ff2319654`, f64, the cavity asked through the trace lane's
switch (`origin/embed/cavity-trace-measure`).

**Where the short top comes from.** Nothing rounds it down on the way
to the certificate. It is the meridian's stored pcurve, read exactly:
- the two-arc sphere's cavity stores each northern meridian as
  `Harmonic { u: Angle { phase: 2π or π or 0, winding: 0 }, v: Angle {
  phase: 1.5707963267948963, winding: -1 } }` over
  `t ∈ [±2.9e-18, 0.7853981633974484]`;
- the phase is the polar angle of the edge circle's own frame vector,
  an `atan2` (`geom_brep::pcurve_cache`, the `SphereMeridian` arm). The
  offset's circle starts `2.9e-18` of parameter off the pole, and the
  angle rounds to the `f64` under `π/2`;
- `boundary_of` (`topo::clearance`) lifts each row's two ends as point
  intervals and `extent_of` takes their hull, so the extent's `v` top
  is that phase. `face_extent` returned it unchanged, and
  `present_chart` clipped the cell to it before the pole was read.
- **The south pole was placed all along**, by the same rounding going
  the other way: the row's end there is `phase − π =
  -1.5707963267948968`, the low end of `-π/2`'s enclosure.

So no source has the pole's latitude to the ulp: a meridian's end at a
pole is a rounded value on either side of it. The fact that the face
reaches the pole is the body's, the vertex decided at the pole.

**What landed.** `face_extent`, the one reader of a face's extent,
returns a sphere face's extent reaching the latitude of each pole the
face holds a vertex at (`reaching_its_poles`, by
`vertex_at_singular_point`, the reading the pole's collapsed side
already used). That is the face's fact, read in every cell from the
face's own vertices, and the extent it returns is a superset of the
stored one. No ε concession is added: nothing is dropped, and no bound
is read as the pole for being near it.

**Measured, before and after** (222 tests of the cavity rows' modules,
302 cavities asked before, the same switch):
- **The six rows shell.** The two-arc sphere's cavity reads `Embedded`
  at 526 discharged, 0 refused, 516 splits (it was 45,928 / 19,608 /
  65,526). The domes read 182 / 0 / 169 (tangent, and opening it) and
  228 / 0 / 215 (short of tangent).
- **Nothing else moves:** every other cavity's verdict and receipt is
  identical, the other six valid rows refuse as they did and the two
  crossing pins fail as they did.
