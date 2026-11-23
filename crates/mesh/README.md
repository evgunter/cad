# mesh — design

Ratified design for the certified tessellator, kept beside the code it
governs.

The promise is `docs/DESIGN.md` `[tolerance.chordal]`. Every point of
an emitted mesh lies within δ of the true surface (honestly δ + ε,
plus rounding), and δ bounds that distance and nothing else. If a
triangle cannot be certified, the face refuses typed.

## Every triangle has a winding

`[mesh.winding]`

**Every triangle's winding is outward and resolved above the rounding
of its own positions.** A normal computed from the stored corners
points the way its face does, and would not flip if each coordinate
moved by its evaluation rounding. Shape is not otherwise promised: a
triangle may be thin, and it may be small beside ε.

- **One definition.** A triangle is resolved when its cross product
  exceeds `cert::rounding_allowance` at its coordinate scale times its
  longest side. It lives in `mesh::validate`.
- **Read where faces join the mesh.** `tessellate` reads it for every
  triangle of every face, on every lane and for a memo hit too. A face
  with an unresolved triangle refuses typed, under its own variant.
  This is the same rule as the certificate's: what f64 cannot stand
  behind is refused, not returned.
- **No ε is read.** The mesh stays a function of (body, δ), as
  `[tolerance.chordal]` rules. Whether a feature is large beside ε is
  a question about the body, and its validation answers it. The
  allowance scales with the coordinates, so the verdict on a triangle
  at the edge of resolution depends on where its body sits.
- **Checked from outside** by a public function beside `check_volume`
  that re-derives it over every triangle. `check_mesh` stays
  combinatorial.

## The sizing rule

`[mesh.sizing]`

**Make the mesh as coarse as possible without extra work.** Each lane
(curved, trimmed, NURBS) chooses its grid step from the bound on a
triangle inside one cell of its grid, and takes the coarsest step that
bound allows at δ. The step chooses how dense the mesh is. What holds
a triangle within δ is its certificate.

"Without extra work" means: where refining to exactly δ would be more
work than refining slightly past it, the lane refines slightly past
it. It never does something extra to get a less precise mesh on
purpose. The work meant is centrally the work of writing the code,
more than the work of running it.

- **The step is a first guess, and the certificate decides.** Each
  chart has a cell bound: a closed-form bound on the deviation of a
  triangle inside one cell of the lane's grid, pinned by a test next
  to the lane's step function. A triangle is not always inside one
  cell: a point the grid did not place (a trim or chord point) can
  join nodes further off. So every triangle is held to its
  certificate, and where one reads over δ the lane adds a point there
  and triangulates the face again. It refuses typed when its round
  budget ends.
- **A lane that can prove its first guess is never corrected says
  so**, in closed form, pinned beside its step function. The shape to
  follow is `torus_grid_steps_meet_the_bound_with_equality`. Such a
  proof is worth writing where it is cheap. Where there is none,
  nothing is owed and nothing is shrunk in its place.
- **No safety factor.** A step is never shrunk "for margin". Where the
  step can be solved for exactly, a cell's worst triangle lands at δ. Where
  the schedule is discrete (a repeated halving, say), it lands at that
  schedule's coarsest step. ε and rounding sit outside this choice,
  exactly as the promise says ("honestly δ + ε").
- **Derived numbers are written as derived.** A constant that follows
  from a lane's theorem is computed from it, never typed in by hand.
  For example, the NURBS lane's `MAX_RESIDUAL_SQ` is its sliver theorem
  solved for the aspect.
- **Rim chords use the face's grid step.** A rim chord point that the
  grid did not place creates triangles the cell bound does not
  cover. So a lane chords its rims at its own grid step, and its
  first guess is then rarely corrected there. The chord pass states this once
  (`chords::tighten`): on each chart axis, a boundary's chord count is
  at least its image's speed along that axis times the parameter span,
  over the face's lattice step on that axis. The image is the one the
  face's lane reads: on the curved lane an iso circle, which moves at
  unit speed along its own axis; on the trimmed lane's cone, sphere,
  torus and spindle arms and the NURBS lane the stored row, whose speed
  is `geom_brep::Pcurve::speed_bound`.
- **The lattice keeps clear of the boundary.** The relation between a
  face's boundary and its lattice runs both ways. A boundary chord
  spans at most one lattice step (the rim-chord rule above). A lattice
  node is laid only where the box `BOUNDARY_CLEARANCE` of its own step
  each way, per chart axis, meets no side or point of the trim polygon
  and no crease line but one it is laid on as that crease's own point.
  Nothing is merged or moved, and nothing asks whether two points are
  the same: a node near the boundary is not laid, and the boundary
  point already there serves. A triangle that joins the boundary to a
  node then reaches `1 + BOUNDARY_CLEARANCE` steps, and the bound for a
  boundary run is a function of that reach.

## The mesher's other constants

These are not sizing, and each says which kind it is where it is
defined:
- **structural:** `MAX_ANGULAR_STEP` keeps angle unwrapping unambiguous;
  `MAX_COUNT` caps allocation.
- **quality:** `ASPECT_CAP` (A = 16, `docs/TESS-BUDGET.md`) is a
  heuristic that keeps the NURBS split from choosing a long thin strip.
  It is not a promise about triangle shape, and it can be replaced by
  the criterion it stands in for.
- **quality, and a debt:** `trimmed::BOUNDARY_CLEARANCE` (1/16) is how
  far a lattice node keeps from the boundary, in its own steps. It is
  its own constant: it does not read `ASPECT_CAP`. The value is
  temporary. The worst triangle beside the boundary grows with the
  clearance, and the value is small enough that a face which meshes
  with no clearance still does. The value nobody has to choose is 1/2,
  a node not laid where the boundary crosses its own cell, and that is
  where this constant goes. At 1/2 the worst triangle beside the
  boundary is larger and more certificates add a point; what that
  costs is not measured, and the constant moves when it is.
- **termination:** `trimmed::MAX_GRID_RETRIES` decides when to stop and
  refuse, and never changes a mesh's shape.
- **termination:** `trimmed::MAX_CORRECTION_ROUNDS` (6) is the rounds a
  face's triangles read over δ may add points before it refuses typed,
  on the trimmed and the curved lane. It is a budget, not a proof:
  nothing bounds the rounds a face needs, and the most measured is 3,
  stated where it is defined.
- **work:** `trimmed::MAX_SPHERE_EDGE_CHORDS` (2¹⁶) is the chords the
  trimmed sphere's tightening may ask of one edge before it refuses
  typed. It keeps one face's CDT at about a second in release, and its
  measurement is stated where it is defined.
- **certificate resolution:** `geom_brep::patch_bound::RATIONAL_CERT_SPLITS`
  sets how tight the rational bound is.

The meters' sample densities (`tess-meter`'s `DEV_SAMPLES` and the test
falsifiers') measure a mesh and size nothing.
