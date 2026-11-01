# mesh — design

Ratified design for the certified tessellator, kept beside the code it
governs.

The promise is `docs/DESIGN.md` `[tolerance.chordal]`. Every point of
an emitted mesh lies within δ of the true surface (honestly δ + ε,
plus rounding), and δ bounds that distance and nothing else. If a
triangle cannot be certified, the face refuses typed.

## The sizing rule

`[mesh.sizing]`

**Make the mesh as coarse as possible without extra work.** Each lane
(curved, trimmed, NURBS) chooses its grid step from a bound on its own
worst triangle, and takes the coarsest step that bound allows.

- **Each lane has a theorem.** It names the worst triangle the lane's
  triangulation can produce and bounds that triangle's deviation in
  closed form. Depending on the lane, the worst triangle is half a
  grid cell, or a Delaunay sliver beside a point the grid did not place
  (a trim or chord point). The theorem is pinned by a test next to the
  lane's step function. The shape to follow is
  `torus_grid_steps_meet_the_bound_with_equality`.
- **No safety factor.** A step is never shrunk "for margin". Where the
  step can be solved for exactly, its worst triangle lands at δ. Where
  the schedule is discrete (a repeated halving, say), it lands at that
  schedule's coarsest step. ε and rounding sit outside this choice,
  exactly as the promise says ("honestly δ + ε").
- **Derived numbers are written as derived.** A constant that follows
  from a lane's theorem is computed from it, never typed in by hand.
  For example, the NURBS lane's `MAX_RESIDUAL_SQ` is its sliver theorem
  solved for the aspect.
- **Rim chords use the face's grid step.** A rim chord point that the
  grid did not place creates triangles the lattice theorem does not
  cover. So a lane whose theorem is about grid cells chords its rims
  at its own grid step. The torus, sphere and cone do this through
  `grid_boundary_step`, NURBS through `nurbs_tighten`, and the trimmed
  sphere, whose tilted circles cross its lattice, through
  `sphere_arm_tighten`.
- **A lane without its theorem yet** keeps an explicitly named
  `UNPROVED_HEADROOM` factor at its call site, filed as a debt. The fix
  is to write the theorem, not to tune the factor.

## The mesher's other constants

These are not sizing, and each says which kind it is where it is
defined:
- **structural:** `MAX_ANGULAR_STEP` keeps angle unwrapping unambiguous;
  `MAX_COUNT` caps allocation; `trimmed::MAX_POLE_JUNCTIONS` bounds
  the search for where a trimmed sphere loop's runs between poles lie.
- **quality:** `ASPECT_CAP` (A = 16, `docs/TESS-BUDGET.md`) is a
  heuristic that keeps the NURBS split from choosing a long thin strip.
  It is not a promise about triangle shape, and it can be replaced by
  the criterion it stands in for.
- **termination:** `trimmed::MAX_GRID_RETRIES` decides when to stop and
  refuse, and never changes a mesh's shape.
- **work:** `trimmed::MAX_SPHERE_EDGE_CHORDS` (2¹⁶) is the chords the
  trimmed sphere's tightening may ask of one edge before it refuses
  typed. It keeps one face's CDT at about a second in release, and its
  measurement is stated where it is defined.
- **certificate resolution:** `geom_brep::patch_bound::RATIONAL_CERT_SPLITS`
  sets how tight the rational bound is.

The meters' sample densities (`tess-meter`'s `DEV_SAMPLES` and the test
falsifiers') measure a mesh and size nothing.
