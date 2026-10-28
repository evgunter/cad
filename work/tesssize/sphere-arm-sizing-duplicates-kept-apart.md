---
id: sphere-arm-sizing-duplicates-kept-apart
kind: issue
title: three sizing pieces the sphere arm spells beside an existing one (widest-latitude cosine, harmonic speed bound, tightening side loop)
status: open
opened: 2026-10-28
---

Filed by the PR 5920 fix pass (S7 of its review). The sphere arm of the
trimmed lane spells three sizing pieces beside an existing one. Unifying
each moves bits the curved or NURBS lanes already ship, so they are kept
apart here and named. The fourth duplicate the review named, the routing
predicate, is unified: `tessellate::lane_of` routes a sphere face by
`trimmed::takes_sphere_arm`.

1. **Widest-latitude cosine.** `mesh::trimmed::sphere_widest_cos` reads
   each boundary circle's latitude range in closed form
   (`trimmed::harmonic_range`). `mesh::sizing::sphere_face_widest_cos`
   reads the face's vertices, which bounds the range only on an
   iso-bounded face (parallels are constant, meridians monotone). On
   such a face the two agree in exact arithmetic but not bitwise: one
   takes `cos v`, the other `√(1 − s²)`. Routing the curved lane through
   the closed form would re-size every curved sphere face by rounding.
2. **Harmonic speed bound.** `chords::sphere_uv_speeds` bounds a
   harmonic image's per-axis speed by `√(pa² + pb²) + |pl|`, the
   supremum. `chords::nurbs_tighten`'s harmonic arm uses
   `|pa| + |pb| + |pl|`, which is looser by up to √2. Moving it to the
   supremum changes NURBS-adjacent chord counts, so goldens move.
3. **Tightening side loop.** `chords::sphere_arm_tighten` repeats
   `nurbs_tighten`'s walk over an edge's two sides (`sides_of`, the face
   test, the half-edge's cache, the per-axis `ceil_count`s). One
   side-walk taking the per-face speed and step reader as a closure
   would serve both.

Each is a refactor with a measured diff: the chord counts and lattice
sizes it moves are compared on the D9 goldens and tess-meter before it
lands.
