(join-split implementer) Item 2, the class, measured on both trees at three ε. RED, and wider than the snowman row.

Method:
- A local probe in sweep's `all` target, the same file on both trees and not committed.
- Each cell is meshed at δ = 10t, 3t, t, 0.3t and 0.1t, where t is the thickness.
- The truth is topo::mass_properties. For every cell I record per-patch triangles, signed volume over the truth, and check_mesh.
- "FLAT" means |volume| ≤ 1e-12 of the truth.

**Every cell reads the same at ε default, 1e-6 and 1e-12, on both trees.** Nothing below depends on ε. check_mesh is Ok on every cell that meshes. No cell is negative past rounding on either tree, and no cell is FLAT on main.

A. The family meshed through the polar and rectangle lanes, where nothing moves. 180 cells per ε, 168 identical:
- Caps of the plain ball at h 1e-3, 1e-2 and 1e-1, at lat ±90, ±60, ±30 and 0 (the boolean re-charts the cap polar).
- Axial lenses of two spheres (equal radii, and radii 1 and 2) at t 1e-3, 1e-2 and 1e-1.
- A thin cylinder segment cut along its axis, at h 1e-3, 1e-2 and 1e-1.
- A cone's tip cap at h 1e-3, 1e-2 and 1e-1.
- The torus sliver does not build on either tree: FallbackExtentUnsupported at the boolean.
- What moved, the same at every ε:
  - The OFF-AXIS lens of two unit spheres, t 1e-3/1e-2/1e-1, at the two coarsest δ: [18,18] r 0.713–0.755 → [16,16] r 0.626–0.671. Positive, losing 6–17% of main's.
  - Plain-ball cap h 0.1 lat +60: [71,47] r 0.711 → [53,49] r 0.685 at δ 1 and 0.3; [92,68] 0.817 → [80,68] 0.814 at δ 0.1.
  - **Plain-ball cap h 0.1 lat −60, δ 1 and 0.3: main meshes, [71,47] r 0.715; the head REFUSES FoldedTriangle.**

B. The snowman's own class: caps cut off a TRIMMED sphere face (the ball less the box x ≥ 0.5, as in the row):
- h 1e-3, 1e-2 and 3e-2.
- lat ±90, ±75, ±60, ±45, ±30 and 0.
- az 30, 120, 210 and 300.
- The same boolean refusals on both trees (27 bodies: FallbackExtentUnsupported or empty).

Per ε: 435 cells, 329 identical, 106 moved. Of the moved:

| h | FLAT on the head | REFUSED on the head (FoldedTriangle; main meshes) | positive, < ½ of main | moved, ≥ ½ of main |
|---|---|---|---|---|
| 1e-3 | 11 | 0 | 8 | 11 |
| 1e-2 | 11 | 2 | 9 | 4 |
| 3e-2 | 5 | 27 | 11 | 7 |

Where the moves fall:
- The FLAT and < ½ cells are mostly δ ≥ t. The head's patches there are [n−2, n−2] on the n chord points, the same shape as the reviewer's lat −60. Lowest positive ratios: 0.015, 0.057, 0.071 and 0.073, against main's 0.73–0.84.
- The refusals are mostly lat ±75 at h 3e-2: the cap reaches to within a degree of the pole, and main had [354,118] … [520,424]. Refusals there run down to δ 9e-3, which is a third of the thickness. So this is not only "thinner than δ".
- A refusal where main meshed is a lost body, which the unit's rulings said must not happen ("no body may be lost"). I have not yet traced whether those refusals come from the join's piece check or from the lane's fold cure. That is my first step in item 3.

Why the head goes flat: main's sphere arm put a sphere point on every side between two boundary points, which kept the face off its plane. With that insert retired, a sphere patch whose every point is a chord point of its one circle is a plane triangulation of the chord polygon. The join adds a point only where the two patches happen to lay the SAME diagonal.

Next: item 3. Cures (a) and (b) go behind a JS_CURE env switch on the head (local only, never pushed) and (c) is counted. All three are measured on this table, the 1.05 bored ball's table and the per-patch differential over `mesh` and `sweep`'s meshing modules.
