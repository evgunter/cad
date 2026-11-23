## For Ev

**Recommendation (likely).** The unit's census looks for the wrong thing. The real defect is two faces' patches covering **the same region with triangles built only from the points the two faces share**. Two patches making the same side is just one form of that. The fix has three parts:
1. **Widen the census.** The join should also find any two faces whose patches both contain a *bare triangle* toward each other. Both patches then split those triangles' inside sides, using the split that already exists.
2. **Move the split point.** Split at the surface point nearest the side's 3-D midpoint, not over its chart midpoint.
3. **Make the sign a promise, checked where the patches join.** Every closed shell of a returned mesh must have its signed volume pointing the same way as the body's shell, and clear of rounding. A mesh that fails this is refused with a typed error that names δ. This needs a new clause, which is yours to ratify.

Parts 1 and 2 land in the unit. Part 3 is the parked row's work, but it should be decided now.

Terms used below:
- **Patch**: the triangles one face is meshed into.
- **Shared point**: a point on a face's boundary, which the face across that edge also has.
- **Bare triangle of face F toward face G**: a triangle in F's patch whose three corners all lie on G's boundary as well as F's.
- **Inside side**: a triangle side that is not a boundary chord.

**Premise corrections**
- *The thin cap is not something the census missed; it is outside what the census looks for* (sure). At lat −60, each patch triangulates the same n circle points, but with different diagonals. So the patches cover the same disc without sharing a side. The unit's census only looks for a side used past its count, so it has nothing to act on.
- *The base rule was not wrong on this body* (likely). The base read "both ends of the side lie on one face across's boundary". That is exactly the test for "this triangle could lie on the neighbour's sheet". The unit's figure of "997 inserts against about 502 sides actually made by both" measured that rule against the narrower defect. Many of the extra inserts were guarding against overlaps that share no side.
- *Where it starts* (sure). The trimmed lane can lay no interior point at all: its docs say "A face may lay no node … its triangles then stand on its boundary points". A curved face meshed that way is a mesh of its boundary, not of its surface. It passes its certificate whenever the surface lies within δ of that boundary. Two faces bounded by the same points can then produce the same sheet, and no check sees it.
- *The pole refusals come from the chart, not from splitting* (likely). Near a pole the latitude–longitude chart puts a side's chart midpoint far from the side, so the pieces fold. The design probes that preceded the unit split at the surface point nearest the 3-D midpoint, and no piece folded over the `mesh` suite. The build switched to the chart midpoint, and the trimmed-sphere caps were never part of that suite.

**Answers, as final states**

*A. Wider census + nearest-point split + sign promise (recommended).*
- The join's census has two triggers:
  - (i) a side that two patches have inside, as now;
  - (ii) faces F and G that each have a bare triangle toward the other. Each then splits every inside side of those triangles at a point of its own surface.
- Still decided at the join. No lane reads anything about its neighbours, nothing is meshed again, and Ev's own-surface decision stands.
- **Worked example, lat −60.** The sphere patch splits its n − 3 diagonals at sphere points. The plane patch splits its own diagonals at their midpoints, which changes nothing about its shape. Every sphere point lies on the cap's convex side, so the volume comes out positive (roughly the cap's inscribed volume) instead of the plane counted twice.
- The sign check catches whatever this misses. An example: a shell that is not convex and is thinner than δ, where one sheet's chords sag through the other sheet, as the bored ball can.
- Reversible: (ii) is one extra census rule, and the sign check is one function.
- Left open: two sheets can still cross while the total volume stays positive. Only a test that no two triangles of different faces intersect would close that. I would leave it as a filed row, not a promise.

*B. The lanes decide from topology (the base rule made uniform, applied once, with no rebuild).* Each curved patch splits every inside side whose two ends lie on one other face's boundary.
- For: one pass inside the lane; covers both triggers above.
- Against: a patch would again depend on which faces lie across its edges. The memo key would have to carry that grouping, which is the thing the unit retires. It also splits where the neighbour never overlaps. I lean against it (likely).

*C. Each curved patch splits every inside side between two shared points.* This reads nothing outside the face, so the census becomes a defect check only.
- Against: every strip between two rims gets split, for example a cylinder between its two caps, roughly doubling its triangles to cure a corner case. That conflicts with `[mesh.sizing]` ("as coarse as possible without extra work"). Reject (likely).

*D. The unit as it stands + the sign check only.* The flat cells would refuse where the base meshes them at 0.73 of the true volume, which breaks the no-loss ruling. Reject (sure).

**The split point (question 2).** Use the point of the face's surface nearest the side's 3-D midpoint.
- On a sphere this is the great-circle midpoint, and on a cylinder it is the chart midpoint.
- On a cone it is close to the development's midpoint. Whether the cone keeps the development's midpoint is a measurement.
- It does not depend on the chart, so a stretched chart cannot pull it away from the side.
- It has a closed form for every analytic surface kind. Spline faces keep the chart midpoint: they have no closed-form nearest point, and the suite shows 4 splits there, all on the lens.
- This stays within your decision, which was a point on the face's own surface versus the straight midpoint (likely).

**What lands before merge (question 3).**
- The unit: A(i)+(ii) and the nearest-point split. It must lose no body the base meshes, and must return no flat mesh where the base returned positive volume.
- A positive but small volume (0.015 of the true volume) is not a defect. The distance promise allows any mesh within δ, and the base's 0.73 was an accident too. The same goes for the off-axis lens losing 6–17%.
- The sign check and its clause: the parked row, decided now so the unit can be measured against it.

**Ratified text (question 4).**
- `[tolerance.chordal]` and the opening of `crates/mesh/README.md` say δ bounds distance "and nothing else". That remains true: sign is a property of the output, not a sizing criterion. But nothing promises orientation.
- I would extend `[mesh.winding]` from one triangle to whole shells: "every shell of a returned mesh has the signed volume of the body's shell, resolved above rounding; otherwise the call refuses typed, naming δ". Your call.
- `validate::signed_volume`'s doc calls positive volume "the global orientation invariant". That comes from a code comment written by an agent in the original mesh-crate PR (2026-07-20), not from ratified text. Under A it becomes true, and it is what the check would enforce.
- `check_volume`'s argument assumes δ times curvature is far below one. That does not hold on these bodies, so its bound passes anything. It should say so, not be tightened.

**Confidence:** recommendation likely; premise corrections as marked; whether A(ii) clears every flat cell is unsure until measured (below).

## For the orchestrator

Measurements I need (the implementer's instrument, ε 1e-9 / 1e-6 / 1e-12):
1. **Nearest-point split alone vs the head.** Cells: the 435-cell trimmed-cap class, the plain-ball cap h 0.1 at lat −60 (δ 1, 0.3), the off-axis lens, the `mesh` suite and `sweep`'s meshing modules. Count:
   - refusals, split by `held` fold vs certificate;
   - flat cells, and volume as a fraction of the true volume.
   - On cones: nearest point vs development midpoint, with fold counts per lane like the unit's table (685/1,675 etc.).
2. **Nearest-point split + census trigger (ii).** Same cells. Count:
   - flat/negative cells;
   - triangles vs the base;
   - splits made by (ii) per pair kind.
   - Expect the lat −60 caps to be positive. Any flat cell left means a bare triangle with no inside side.
3. **Bare triangles with all three sides boundary chords.** For example a circle chorded at 3 points; the cap h 0.1 at δ 1 is a candidate, since its rim radius 0.44 is below δ. Here (ii) has no side to split, and the design needs a point inside the triangle (the nearest surface point to its centroid) or a typed refusal. Count them.
4. **Per-shell sign over the cells in 2.** Which cells would the proposed check refuse, and are any of them meshed with positive volume on the base?

Assumptions and claims I did not check:
- I could not run anything. Every number is the brief's.
- I assumed shells can be split out at the join (face → shell). The per-shell sign needs that. A whole-mesh sign alone would miss an inverted thin void.
- I did not trace whether the "DISTANCE-ONLY" sentence in `docs/DESIGN.md` is Ev's own wording. The first commit `-S` finds is 2026-08-03 ("PR 14: K-REPORT M5 addendum…"), and the checkout is shallow, so that may be a graft.
- The brief says the unit "cannot reach a patch that shares no side". That is accurate. I reframed it as the census looking for coincident sides rather than overlapping regions.

Defects found off the question (file under tesslane):
- `validate::check_volume`'s doc states a first-order argument that fails exactly where δ is at least the body's thickness. The two tests the parked row names pass inside-out meshes because of it.
- `trimmed.rs`'s module doc treats "a face may lay no node" as harmless. It is the source of this class and should cite the row.
- `join::split` on a `Splits::Midpoint` patch reads no certificate and no winding. Harmless for a plane, but trigger (ii) will exercise it far more often.
