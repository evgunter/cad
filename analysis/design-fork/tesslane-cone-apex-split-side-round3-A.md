## For Ev

**What moved.** On the form of the rule I now take the zipper: walk the face's two boundary rows in angle order and join them, with no Delaunay step. I drop my own form, which held an edge from each rim point to its "partner", the point at the same angle on the other rim. That form is wrong for a frustum, a cone cut off below its apex. Another designer measured that a frustum's two rims get different chord counts from δ 0.03 down (51 points against 43 at δ 0.001), so partners mostly do not exist. I take that measurement unverified, and my answer does not depend on it, because the zipper needs no partners. I also prefer the zipper to the third form on the table, "no rows plus one held edge per split side". That form leaves an unsplit face's strips to whatever Delaunay produces. It was measured to work in the chart lane, but nothing holds it. That is the kind of premise the sentence all three reports propose for `[mesh.sizing]` forbids: *a lane's triangulation is constrained to what its theorem assumes, never left to what Delaunay does.*

**What did not move.** Scope, home, interim answer and what needs your word are unchanged, and all three reports now agree on them:
- Scope: every cylinder and cone face with a split straight side.
- Home: the rule is a fact on the cylinder's and cone's chart-table row, read by the chart lane (`mesh::trimmed`, the lane the plan keeps).
- Until it lands: those faces route to the chart lane now, the rectangle lane's fallback lattice is deleted, and PR 7043's `UnresolvedWinding` stays as the backstop.

**The rule (likely).** It applies to a face on a cylinder or a cone whose boundary is two *rows* and two *rulings*.
- A *row* is a rim, or the apex counted as a row of one point.
- A *ruling* is a straight line of the surface at one angle round the axis.

The rule:
- **No interior points.**
- **Build by zipper.** Sort each row's boundary points by angle and walk both rows together. At each step, join the next point of whichever row comes earlier in angle to the current point of the other row. Each step makes one triangle, so every triangle has two neighbouring points of one row and one point of the other.
- **Split sides.** A vertex part-way along a split side lies on an end ruling. It divides the triangle on that ruling, in that triangle's own plane, which adds one triangle per vertex.
- **What this guarantees.** No triangle's corners span more than one rim chord in angle. No triangle lies along one ruling, since three corners on one ruling are never chosen.

**Why it is within δ.**
- **Sure:** when a triangle's corners lie on two rulings, those rulings meet at the apex (cone) or are parallel (cylinder), so the triangle is flat in their plane. It sits inside the fan triangle from the apex to the two rim points, and `sizing::cone_grid_step` already bounds that triangle at δ.
- **Likely, owed as a written proof before it is ratified:** a triangle whose corners fall within one rim chord's angle but on three rulings stays inside the cone's convex hull over that angle. That hull lies within the same fan-triangle distance of the surface.

**Splitting a straight edge.** With this rule, a split never refuses and never changes density beyond the added triangle.

**Faces outside the rule.** A face that is not two rows and two rulings (one with an oblique trim) keeps the chart lane's general lattice-and-Delaunay path. Its order relative to the zipper is only sequencing.

**Fallback (likely as a fallback).** If you would rather the chart lane keep one triangulator: "no rows, plus one held edge per split side" (an edge the triangulation is forced to keep), measured to work. Pair it with a test that fails if any triangle spans more than one chord in angle, which makes its premise checked rather than assumed.

## For the orchestrator

- No measurement this round; the answer does not turn on one. The tree is clean and the target directory is deleted.
- Rule form is the one open difference.
  - A (mine) is withdrawn on C's frustum rim counts, taken unverified.
  - B (the zipper) is my choice. It is the only form whose strip premise is constructed, not observed, and it subsumes the apex face as a row of one point.
  - C (held edge per split side) is the measured fallback. Its unsplit case rests on chart-lane Delaunay behaving well, which contradicts the `[mesh.sizing]` sentence C itself proposes.
- What would move me to C: evidence that a separate triangulation path inside the chart lane costs more in the code than the principle is worth. For example, a zipper-built face would also have to go through the lane's even-odd classification, ear cure or retry loop, and those do not compose with a triangulation built without the CDT.
- What would move B or C to the other: a written proof of the three-ruling case, or a falsifying pose for it.
- Owed whichever form is chosen:
  - The convexity lemma for a triangle on three rulings within one chord's angle, written beside `sizing::cone_grid_step`.
  - A cylinder analogue, which is trivial: the rectangle between rulings.
- Unchanged from earlier rounds: the rows to file (cylinder split refusal, the `TOTAL_REFUSED` comment, reopening `curved-lane-triangulates-in-raw-chart-coordinates`), and the sphere tangent-normal remark, which is still unmeasured.
