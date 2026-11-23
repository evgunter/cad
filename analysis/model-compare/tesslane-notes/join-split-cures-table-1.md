(join-split implementer) Item 3, first half: the cures on the class table. The bored ball and the full per-patch differentials are running now.

**Where the head's 29 new refusals come from.** I traced them with a local switch. Every one is the join's own piece check (`join::held`, a split piece wound against the sphere). None comes from the lane's fold cure. They sit where the cap reaches toward the pole (lat ±75 at h 0.03 reaches 89°), where the sphere's chart midpoint lies far from the side's geodesic midpoint. Plain-ball cap h 0.1 lat −60 is the same.

**Switches.** All are local env switches on the head, never pushed:
- JS_CURE=a: your (a). A curved patch with no interior point, whose boundary chords are held with exactly one other face, splits every side two of its triangles have (bar a chord it holds), through the same join::split.
- JS_CURE=b: your (b). The same patches, plus a point at the chart centre of every noted triangle (made three). The census sides are still split first, because centring alone leaves a shared side shared.
- JS_MID=geo: a cure the measurement suggested. On a sphere, the split point is the geodesic midpoint: the chord's midpoint pushed out radially, read back to the chart on the u branch nearest the chart midpoint. Elsewhere the point is as now.

Class table: the same 625 cells as item 2, at the default ε (item 2 was identical at all three ε). Polar/rect family 180, snowman 10, trimmed caps 435.

| config | FLAT | refused (main meshes) | < ½ main | moved cells' ratio to main, median | worst |
|---|---|---|---|---|---|
| head | 32 | 31 | 29 | 0.30 | 0 (flat) |
| a | 0 | 15 | 2 | 0.90 | 0.363 |
| b | 0 | 45 | 2 | – | 0.363 |
| geo alone | 32 | **0** | 47 | 0.26 | 0 (flat) |
| **a + geo** | **0** | **0** | **2** | **0.91** | 0.363 |
| b + geo | 0 | 31 | 2 | 0.82 | 0.363 |

- The two cells below ½ under every cure are the caps at lat ±45, az 300, h 0.03, δ 0.3: [12,2,6] r 0.656 → [6,2,6] r 0.238. That cap also meets the box's face, so it has three faces and is outside the one-neighbour class (a) reads.
- Nothing is negative or FLAT under a, a+geo or b+geo. Every cell that meshes passes check_mesh.
- (b) is dominated: it refuses more than the head does.

Is any of them a predictor or an ε reader?
- None reads an ε.
- (a) reads its own patch (no interior point), shared ids, and WHICH faces hold its boundary chords. It reads nothing of the neighbour's triangulation or geometry, and it splits whatever the neighbour lays, so it predicts nothing. The "one other face" test is topology read off the chord holders. If you want to read it strictly, it is the only thing taken from across the edge.
- geo reads only the face's own surface and the side's two ends.

Item 4, whether positive volume can be argued:
- **Sphere cap against one plane, under a+geo: half an argument.** The plane patch is flat, at height 0. Every split point is strictly on the material side: for a chord midpoint p on the plane n·x = s, |p| < 1, the point p/|p| has n·(p/|p|) = s/|p| > s. So every corner of the sphere sheet has height ≥ 0, and some have height > 0. The volume is Σ over the sphere's triangles of (projected signed area) × (mean corner height), so it is positive IF every piece projects onto the plane wound positively.
- That last condition is NOT what `held` reads. `held` reads the 3-D normal against the sphere's outward normal at the centroid, which is within the cap's angular radius of the plane's normal. A sliver nearly perpendicular to both can pass `held` and project negative. So this is a sufficient condition I can state, not a proof that the class is positive.
- **(a) with the chart midpoint:** no argument. The chart midpoint of two boundary points can fall outside the cap (below the plane).
- **(b):** the chart centroid lies inside the chord polygon in the chart. That puts it inside the face only where the trim is convex in the chart. Measurement.
- **Two curved faces (the lens):** there is no flat reference, so it is measurement under every cure.
- **(c), nothing built:** a sign read at rounding would refuse the 32 FLAT cells and none of the 29 thin positive ones (lowest 0.015 of the truth). Over the suites it is counted from the head's differential run, which now logs each call's signed volume.

Running in this order, at the default ε (`sweep` takes about 37 minutes a run on 4 cores, so three configs, not five):
- `mesh`: head, then a+geo, a, b+geo, with per-call signed volume
- `sweep`'s meshing modules: head, then a+geo, a

The bored ball's pins (split_rulings) come out of the `mesh` runs. Pinned rows will red under a cure by design; I will report which and why. I will send each part as it lands.
