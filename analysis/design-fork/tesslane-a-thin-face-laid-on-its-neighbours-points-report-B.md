## For Ev

**Recommendation (likely).** The split goes in. It needs three changes before it merges, and the third is an addition to the promise:

1. **Change what triggers a split.** A curved face splits *every* inside side that joins two points of its own boundary, at a point of its own surface. Whether another patch made the same side no longer matters. Call this the **own-point rule**: every inside side of a curved patch has at least one end that the face itself placed.
2. **Change where the point goes.** Use the point of the face's surface nearest the side's 3-D midpoint (the **foot**), not the point over the side's midpoint in the chart.
3. **Add a body-level orientation read.** `tessellate` refuses, typed, any mesh whose signed volume is not positive and resolved above its rounding. The refusal names δ, and the way out is a finer δ.

With all three, no flat or inverted mesh comes back as `Ok`. The bodies the base meshed should mesh again; that still has to be measured. The parked row closes with this unit.

Terms:
- **Inside side:** a triangle side that is not a chord of the face's boundary.
- **Boundary point:** a mesh point on one of the face's edges. All such points are shared ids.

### The premise, corrected (sure)

The retired base rule (insert a point on any inside side whose two ends one neighbouring face also holds) was treated as a bad *prediction* of where two patches make the same side: 997 points were inserted where about 502 sides were in fact made twice. It was doing a second job as well: **separation**. On a face whose patch has no points of its own, those inserted points are the only places the face's surface shows up in the mesh.

The thin cap at latitude −60 shows this:
- Both patches are triangulations of the same 8 coplanar circle points, so both lie flat in the plane.
- That is within δ for both faces, because the cap is thinner than δ.
- On the base, the sphere's inserted points held that patch off the plane, which gave 0.73 of the true volume.

The split triggered by the census only works when two patches happen to share a side. Here they lay different diagonals and share none, so nothing happens. The defect is not "a side made twice". It is **"a curved patch whose inside sides carry none of its own surface"**. A side made twice is one symptom of that; a flat thin body is another.

### Q1: what the mesher owes, and what decides it

**It owes distance and orientation, not volume (likely).**
- `[tolerance.chordal]` already promises distance.
- `[mesh.winding]` promises each triangle is outward against its own face.
- The missing half is the same promise for the whole body: the mesh encloses positive volume, resolved above its own rounding. That is the "global orientation invariant" that `validate::signed_volume`'s doc names and that nothing currently holds.

**What decides it is a fact of the mesh**: the sign of its signed volume, read where the patches join. This reads no ε and no mass properties, and it is cheap. A zero or negative result is row 1 of `[failure.taxonomy]` (reachable by input, invalid), with δ as the lever.

Volume itself is not promised. A thin body may come back at 0.015 of its true volume and still be correct under the promise. Anyone who needs volume should measure the body (`[tolerance.measure]`), not the mesh. If you want the mesh to promise more than the sign, that is a separate fork.

**The cure that makes the read rarely fire is the own-point rule.** It is a fact of the patch alone (face, δ), so the patch stays memoisable and reads nothing across its edges. Its consequences:
- **No side made twice between curved faces (sure).** If neither of two curved patches has an inside side between two boundary points, they cannot share one. The census becomes a check, and the census-triggered split is needed only for plane against plane (see the orchestrator section).
- **The thin cap separates.** The sphere's diagonals get sphere points, as on the base.
- **The bored ball still separates.** Both faces split, and the wall reopens as already measured.

**Alternative trigger (defensible).** Split only an inside side whose two ends lie on edges that one same other face also bounds. This is the base's test, applied by splitting instead of reinserting a point and rebuilding.
- For: fewer extra triangles. Its count is known (about 997 points over the `mesh` suite).
- Against: the patch then depends on which faces lie across its edges, so the memo key has to carry them again (the thing PR 7241 had to add and this unit removes).

The own-point rule costs triangles on ordinary faces, for example a cylinder band with no interior rows, whose rim-to-rim diagonals nothing else could make. **I lean to the own-point rule if that overhead is modest, and to the alternative if it is large.** This is a measurement to take, not a matter of principle.

**Rejected:**
- The head as it stands: it returns flat meshes as `Ok`, which you called annoying.
- The sign read alone: it turns the −60 cap into a refusal where the base returned 0.73 of the volume.
- Re-meshing at a finer δ inside `tessellate`: the whole body pays for one thin wall.

### Q2: where the split point goes (likely)

The chart midpoint depends on the chart. Near a sphere's pole, two points at very different azimuths have a chart midpoint far from the straight side between them. The pieces then fold, and `join::held` refuses the face. That is the cause of all 29 refusals.

That verdict depends on where the face's frame puts its pole, which the body's shape does not fix (`[intrinsic.contingent]`). The same cap, rotated, meshes.

The foot depends only on the side and the surface:
- On a sphere it is the radial projection of the midpoint, the middle of the great-circle arc.
- On cylinder, cone and torus it has a closed form.
- On a spline it is a projection seeded at the chart midpoint.
- It moves each piece off its parent's plane by no more than the sagitta, so it folds only where the parent triangle is already thin.

Unsure:
- On cones, the stage-1 file measured the chart midpoint against the development midpoint but never against the foot.
- Near a concave trim, the foot might land outside the face's region. The existing fold and certificate reads would refuse that case rather than pass it silently.

### Q3: what lands before the merge

All three changes land in the unit: the own-point rule, the foot, and the orientation read with its typed refusal. Without the read, the unit trades the base's 0.73 for an unchecked `Ok` of zero volume. Without the other two, the read turns those cells into lost bodies.

The parked row then closes into this unit. Whether `check_volume` stays as it is (its δ·area bound is honest, just weak) becomes moot once the sign is held. If you would rather keep the unit smaller, the read could follow as the parked row. Even then, the unit cannot merge while it returns flat meshes where the base returned volume.

### Q4: ratified text (both mesh clauses came through your review)

- **Add to `[mesh.winding]`:** the body-level read ("a returned mesh encloses positive volume resolved above its rounding; read where the patches join; otherwise the body refuses typed, naming δ").
- **Add a clause, `[mesh.own-points]`:** "every inside side of a curved patch has an end the face placed." It binds every future meshing path.
- **`[mesh.sizing]`:** one sentence saying that points added under the own-point rule are not margin. The sizing rule chooses the grid step and says nothing about separation.
- **`[tolerance.chordal]` is unchanged.** "Distance-only" is about the sizing criterion and does not make this hard.
- **The unit file** claims "no clause changes" and calls the base rule a wrong prediction. Both claims need correcting.

## For the orchestrator

**Measurements to request.** Trees: the head; the head with the own-point rule and the foot; the head with the alternative trigger and the foot; the base. Run each at ε 1e-9, 1e-6 and 1e-12.
1. **Overhead.** Triangle and position counts over the `mesh` suite and `sweep`'s meshing modules, for each tree. This decides between the own-point rule and the alternative.
2. **The class.** Signed volume and refusals on the 435 trimmed-cap cells and the 180 cells where nothing moved. Report each mesh's volume as a ratio to both the true volume and the base's. Separately, count any cell whose sign read would refuse.
3. **The 29 refusals.** Re-run them with the foot. Also run one cap with the sphere's frame rotated so that its pole is far from the cap. That confirms the refusal follows the frame.
4. **Cones.** Fold counts for the foot against the development midpoint, over every side a cone face could split (the same set as stage 1's 685 / 1,675 / 10 / 57 / 0 / 6).
5. **Foot outside the face.** Count splits whose foot falls outside the union of the two parent triangles in the chart.
6. **Thin bodies under the own-point rule.** The bored-ball table, the −60 cap, and the off-axis lens. The lens loses 6 to 17% at coarse δ on the head.

**What I checked in the code.**
- `join::census` skips a side with at most 2·max(chords, 1) uses, which explains why the −60 cap's different diagonals are never seen.
- The split is placed by `join::midpoint`, a chart average with a special case for cones, and checked by `join::held` (winding and certificate).
- The base point was "the developed centroid read back to the chart", reinserted into the CDT and rebuilt (base `trimmed.rs`, module doc and `faces_holding_side`). That is why the base's points never folded near the pole. The join's split does not re-triangulate.
- `Book::splits` already keeps exactly the set the own-point rule needs (every side between two shared ids that two of a patch's triangles have). The rule could therefore run per patch, inside the patch's memo entry, instead of at the join.

**Assumed, not checked.**
- The measurements in the brief. I built nothing and ran nothing.
- That a valid body cannot have two plane faces that both make one inside side. Such a side would lie on the line where the two planes meet and would be inside both faces, which means a knife edge. If one is ever reached, it should be a typed refusal. Under the own-point rule, the census-triggered split (`Splits::Midpoint`, straight midpoint) becomes either dead code or that refusal, and should be sorted out in the build.

**Not yet defined.** The read's rounding allowance needs a definition. Suggested: 6V must exceed `cert::ROUNDING_ULPS` f64 epsilons of the sum of |a·(b×c)| over all triangles, about the bounding-box centre that `signed_volume` already uses. It belongs beside `resolved_normal`, as the one definition of the read.

**Off the question.**
- The unit file says no `crates/mesh/README.md` clause changes. Under this recommendation, two do.
- The fold cure inherited one of the retired insert's three jobs; this report finds a second, separation. The record should say the retired rule did three jobs.
