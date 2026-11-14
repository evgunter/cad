## For Ev

**I now agree with the other two designers on both open points.** The check outside the mesher reads the rounding allowance, not ε. A triangle whose winding f64 cannot resolve makes its face refuse typed, decided where faces join the mesh on every lane. Both of C's bodies reproduce, and they settle the two points: on point 2 against my round-3 position, on point 1 for the position I took in round 3, though not for the reason I gave.

**1. Reproduced (my own probe; the tree is untouched).**

| body | δ | what I saw |
|---|---|---|
| frustum, small rim 20ε = 2e-8 m, upper half cut by the tilted plane | 1e-4 | valid at all tiers. 17,420 triangles, `check_mesh` passes. **226** triangles with a non-boundary corner have least height ≤ ε (least 5.6e-10 m). None is unresolved at the rounding allowance. |
| same, with the agreed 1/16 clearance | 1e-4 | 17,388 triangles, the same 226. The clearance does not touch them. |
| same body | 5e-3 | 0 such triangles (least 3.9e-9 m) |
| rim 200ε | 1e-4 | 0 such triangles |
| same frustum whole | 1e-8 | 81,792 triangles, `check_mesh` passes. **6,780** triangles unresolved at the rounding allowance, every one on three boundary points. |
| same frustum whole | 1e-7 | 0 unresolved |
| rim 200ε, whole | 1e-8 | 0 unresolved |

C's numbers stand: 226, and 6,780 on the axis.

**2. Point 2: "least height exceeds ε" cannot be kept. I withdraw ε.**
- The cut frustum is valid at rest. Its sub-ε triangles come from the mesher's own chord points on the small rim: the rim is chorded at the cone's angular step, so its chords are r·θ < ε long. A triangle's least height is at most its shortest side, so no clearance of the lattice reaches them.
- Your D9 ruling keeps ε out of the mesher, so nothing in the design can aim at an ε-scale promise.
- The scale also matches `[tolerance.chordal]`. The mesh promise is distance-only and has no angular criterion. "Is the winding meaningful at ε" is in effect a promise about normals, which the page says the mesh does not make.
- **The promise then tells a consumer:** every triangle's outward winding is determined by its stored positions. A normal computed from them (the STL writer's, for one) points the way the face does, never by rounding.
- It does not tell the consumer that a triangle is large beside ε. Geometry stays bounded by δ + ε, as now.

**3. Point 1: refuse stands, on every lane, and the chord point is the mesher's.**
- The 6,780 corners are chord points that the chord pass placed (about 20,000 on a circle of radius 2e-8 m), not vertices of the body. So these are the mesher's triangles, and the scope argument that made me keep such triangles in round 2 ("the lane cannot remove a corner the body gave") does not apply.
- Refusing does refuse a body valid at rest that main meshes today, at δ 1e-8 only (it meshes at 1e-7). That is a semantically correct flip, which the output-stability rule allows. It sits beside the crate's existing `RoundingUnresolved` refusal: the page's "if a triangle cannot be certified, the face refuses typed".
- It must be decided where patches join the mesh, on every lane. The cap is planar, and the trimmed lane's emission check never sees it. So the verdict lives once, at the join.
- **The root is the chord rule on a small rim**, which has its own cure: chord a rim from its own sagitta at δ, not at the face's angular step. With that cure, this body meshes at every δ and nothing refuses. The cure is its own row and does not change the recommendation; it removes the only instance anyone has built.
- What remains in principle: an unresolved triangle on three body vertices (the issue-111 class). Nobody has seen one today; it too would refuse.

**4. Recommendation (I hold nothing apart from the others).**
- **The promise**, in `FacePatch` and the page: every triangle's winding is resolved above its own rounding (`cert::rounding_allowance` at the triangle's scale), with no exceptions by corner kind. Shape is not otherwise promised.
- **In the mesher:**
  - the clearance, already agreed;
  - the trimmed lane's three-way emission check, already agreed;
  - one rounding verdict where patches join the mesh, on every lane, refusing an unresolved face typed under a new variant.
- **Checked by** a public `mesh::validate` function at the same definition, reading no ε. The suites call it in place of `MIN_SHAPE`.
- **Filed with it:** the small-rim chord row.

## For the orchestrator

- **What I ran.**
  - Probe source: `…/scratchpad/fork-scratch-B/frustum-B/`. Binary: `…/fork-scratch-B/frustum-B-bin`. Arguments: rim multiple of ε, δ, cut (0/1).
  - It is built against my copied crates `…/scratchpad/latws-q7/crates`: `CLEAR=<c>` turns on the clearance (analytic arms only); unset, it behaves as main.
  - It builds the reach probes' frustum `[(0,0), (1,0), (r,1), (0,1)]` and, when cut, splits it with the tilted plane through `(0, 0.5, 0)` of normal `(sin 0.3, cos 0.3, 0)`.
  - Its "unresolved" test is |cross| ≤ 8·u·M·L, C's reading of the allowance.
  - "Non-boundary corner" means a corner on no boundary polyline.
  - I ran default ε only. I did not rerun C's ε 1e-6 and 1e-12 rows or the shifted pose.
- **Note for whoever implements the verdict:** the allowance scales with the coordinate magnitude M. So whether a triangle is unresolved depends on where the body sits. C reports 6,610 unresolved with the frustum shifted by (1, 0, 1), against 6,780 on the axis. That is honest about f64 storage, but worth one sentence on the page.
- **A row to file:** the chord pass chords a small rim at its face's angular lattice step (`chords::tighten`). That places about 20,000 points 6e-12 m apart on a 2e-8 m rim at δ 1e-8. It belongs with unit 1. With it fixed, the only built instance of point 1 disappears.
