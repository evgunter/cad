## For Ev

**I have moved on both points. I now hold: point 1, refuse typed; point 2, the rounding allowance. On both I now stand with the third designer; the other designer's round-2 reply holds the opposite (ε, kept).**

**Point 2: does the census settle the scale?** Yes, combined with one fact that I checked in the repository rather than reran.
- CI runs the mesh's tests at ε = 1e-9 (the default), and also at 1e-6 and 1e-12 (`ci.yml`'s extra ε step; `nightly.yml` runs the whole suite at all three).
- Your 2026-08-21 ruling (D9) makes the mesh a function of (body, δ) alone. So every triangle in the census (40.6 million, every lane) is emitted unchanged at all three ε, whenever its body builds at that ε.
- The census found 972 triangles with least height (2·area ÷ longest side) between 1e-9 and 1e-6 m, many of them well-shaped triangles on tiny features.
- What an ε-scale check would answer on those same bytes:
  - at 1e-12: nothing is flat;
  - at 1e-9: nothing is flat (the least height is 3.1e-8);
  - at 1e-6: up to 972 triangles are "flat", wherever their bodies build at 1e-6.
- That last answer is wrong as a statement about the mesh. One identical mesh would keep its promise at one ε and break it at another, and the mesher cannot meet such a promise without reading ε, which your ruling forbids.
- My round-2 argument (ε is the floor a consumer can trust, because boundary points carry ε of slack against the surface) is about the body's tolerance. A triangle smaller than ε comes from a feature smaller than ε, and that verdict belongs to the body's validation at ε, not to the mesh.
- "Resolved above its own rounding" is arithmetic on the positions as written: the same bytes get the same answer at every ε, and the clearance meets it with about 10⁷ to spare (the census's nearest triangle is 1.7e7 times above the allowance).
- **Recommendation:** one definition, "winding resolved above `cert::rounding_allowance` at the triangle's scale". The emission check reads it, and so does a public `mesh::validate` check beside `check_volume` that the test suites call. No ε anywhere in this. Confidence: likely.

**Point 1: what body makes a triangle on three boundary points with unresolved winding?** Constructions, one by one (each line of reasoning was checked against the code where cited):
- **Three consecutive chord points of one straight edge.** `chords::tighten` does chord a straight wall edge into many points, and they are collinear only up to rounding. But a triangle on three consecutive polygon points is an *ear*.
  - On the trimmed lane's analytic arms the ear cure splits every ear.
  - Elsewhere, Delaunay rules it out: the circumcircle of three nearly collinear points is huge, and it contains the boundary's other points unless the face itself has no width there.
  - So this reduces to the next case.
- **A face that is a sliver, or whose loop has a spike of no width.** For example, a vertex within rounding of another side of its own loop: the #111 shape, four vertices on one carrier line with one of them 1 ulp off.
  - That body passes every validation tier at rest (`sweep/tests/issue93_az_intersect.rs`, cited in the #111 test's docs).
  - Today it no longer emits a needle: my run and the third designer's measure a least height of 0.06 m or more on all three A×Z bodies.
  - I could not build a current instance. The census found none in 40.6 million triangles.
- **A vertex within rounding of a neighbour's edge it is not on** (a near-T). This is the same state seen from the neighbouring face: the spike lies in this face's own polygon.

So the two outcomes can be told apart only by a body that is valid at rest and carries a feature below ε that arises from a value coincidence (the #111 class). Such bodies exist in principle, so this is a fork, but nobody has seen one today. What each answer leaves the caller holding:
- **Kept.** A watertight mesh with one triangle whose normal is rounding noise. `FacePatch::triangles` promises outward winding for every triangle, so this breaks the letter of the promise. It needs the promise scoped to triangles with a corner the mesher placed, and the STL file then carries a facet with a noise normal.
- **Refused.** A typed `TessellateError` naming the face, for a body that is valid at rest. This is true to the promise as written and to the lane's charter ("drawn, or refused typed with a reason").

Your ruling text in `1240686` already speaks to this: "Weakening a ratified promise to accommodate a state that could only arise from value coincidence would make this document quieter about exactly the case worth hearing about." Scoping the promise for this triangle is exactly such a weakening. **Recommendation:** refused, under its own refusal variant, with or without a lattice corner. The promise stays whole. Confidence: likely.

**If you want to rule rather than accept that:** *does the outward-winding promise cover every triangle, so a face whose own boundary forces an unresolved one refuses typed; or is it scoped to triangles with a mesher-placed corner, so such a triangle is emitted as the body's?*
- Whole: the promise holds for every mesh, and the rare body that hits it refuses.
- Scoped: every valid body meshes, and the mesh may then carry a facet with no meaningful normal.

## For the orchestrator

- **Ran or checked this round:**
  - Read `.github/workflows/ci.yml` (the extra ε step loops `for eps in 1e-6 1e-12`, under `CAD_TOLERANCE_EPS`) and `nightly.yml` (a matrix of `eps: [default, 1e-6, 1e-12]` over the full suite).
  - Read `chords.rs`: `tighten` chords straight edges too ("a straight wall edge is one 3-D chord but many UV steps").
  - No new build. The census numbers are the third designer's round-2 figures, which I did not rerun.
- **Not established.** Whether each body behind the 972 triangles builds at ε = 1e-6. If none does, the ε check would never fire on them in CI. The argument does not depend on this: the same bytes would still carry a promise that depends on ε.
- **Not built.** A current body that produces a flat triangle on three boundary points. The candidate shape is a planar face with a vertex 1 ulp off its own loop's side, built through a boolean as in #111.
- **For the implementer.**
  - With "refused", the validator needs no boundary polylines.
  - The emission check's verdict has three outcomes: outward; folded (handled as today); unresolved, which refuses under the new variant on every lane that runs the check.
  - The spline arm has no outward normal, so it reads only the resolved/unresolved half: whether the triangle's least height exceeds its rounding allowance.
- **Correction to my round 2:** I held "kept" and "ε" there; both are withdrawn, for the reasons above.
