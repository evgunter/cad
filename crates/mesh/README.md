# mesh — design

Ratified design for the certified tessellator, kept beside the code it
governs. `docs/DESIGN.md` `[tolerance.chordal]` states the promise:
every emitted triangle lies within δ of the true surface (honestly
δ + ε, plus rounding), distance only, nothing about count or shape.
This page states how the mesher sizes its output to keep that promise.

## The sizing rule

`[mesh.sizing]`

**Each lane sizes its grid so that the worst triangle its own
triangulation can admit certifies at exactly δ, by a theorem stated
and pinned at that lane's step function.** There is no global safety
factor. A lane's step is the coarsest that its theorem allows: unspent
δ is a defect, not a margin.

- **The theorem is the lane's.** It names the worst admissible
  triangle and bounds it in closed form:
  - a lattice cell's halves, where every triangle is a lattice
    triangle;
  - the Delaunay sliver beside an off-lattice point where the
    triangulation admits one.

  It is pinned with equality, in the shape of
  `torus_grid_steps_meet_the_bound_with_equality`. A constant that
  follows from a lane's theorem is written as derived from it, never
  by hand: the NURBS lane's residual line `MAX_RESIDUAL_SQ` is its
  sliver theorem solved for the aspect.
- **Rim chords are sized to the face's grid.** A boundary chord point
  that the grid did not place makes the triangles beside it
  off-lattice. A lane whose theorem is a lattice theorem therefore
  chords its rims at its own grid step (`torus_boundary_step`,
  `nurbs_tighten`), so that its theorem covers every triangle it
  emits.
- **A lane without its theorem carries named headroom.** Its call site
  carries an `UNPROVED_HEADROOM` factor, and the debt is filed as a
  work item. That headroom is removed by writing the theorem, never by
  re-tuning the number.
- **ε and rounding are not what sizing buys.** They sit outside the
  certificate, additively, as the promise says. A rounding edge at a
  step that meets δ exactly belongs in the comparison, not in a
  factor.

## The mesher's other constants

Every other number in the mesher is one of four kinds, independent of
the sizing rule. Each says which kind it is at its definition:
- **structural:** `MAX_ANGULAR_STEP`, so that unwrapping picks its
  branch unambiguously; `MAX_COUNT`, an allocation cap;
- **quality:** the NURBS split's aspect cap `ASPECT_CAP` (A = 16,
  `docs/TESS-BUDGET.md`), a regularizer of the split optimizer and not
  a shape promise;
- **termination:** `trimmed::MAX_GRID_RETRIES`, which decides refuse
  or continue and never a mesh's shape;
- **certificate resolution:** the rational bound's piece count
  (`geom_brep::patch_bound::RATIONAL_CERT_SPLITS`).

The meters' sample densities (`tess-meter`'s `DEV_SAMPLES`, the test
falsifiers') measure a mesh and size nothing.
