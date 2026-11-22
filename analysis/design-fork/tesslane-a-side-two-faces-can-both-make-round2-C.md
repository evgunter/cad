## For Ev

**Where I stand now.** All three agree that no lane reads its neighbours and the fact is acted on at the join. On the cure I have moved to A's: **each face that has the side inside it splits that side once, at a point of its own surface, certified as any triangle of that face is.** I withdraw my chord-midpoint cure as the recommendation. Two things I measured still have to sit beside any of the three answers.

### What moved me

- **My objection to a surface point was wrong.** I said a surface point is no better geometrically and can turn the sliver inside out. That holds only when one face alone is split. When every face holding the side splits it, each sheet moves towards its own surface, so the collapsed side opens with the right sign.
- **Worked case, the commonest one in the suite** (a plane cap against a thin cone sliver). The cone's triangles on three points of the shared ellipse chain lie in the cap's plane, a double layer over the cap. Splitting the cone's side at a point of the cone turns them into a proper fan of inscribed facets under the cap. My chord midpoint leaves the double layer and only renames a vertex, which reads as making `check_mesh` pass, not as curing the pinch.
- **Chord midpoint forces an arbitrary choice.** If two faces both split at the chord's midpoint, the two new points are one position, and any consumer that joins vertices by position (STL has no ids) gets the edge of four triangles back. So my cure had to leave one face uncut, picked by arena order, and `[determinism.equivariance]` asks for no enumeration-order tie-breaks where a symmetric rule exists.
- **A answers my cost argument.** I said a lane-side cure makes a patch a function of its neighbours' patches. A's split set (the face's own sides that another patch also made, as ids) is a declared input the memo key folds, the triangulation is not redone differently, and one pass ends it because every new side has a private end. That is cleaner than my "a cut face reports no stored-patch id".

### What did not move me

- **A local split, not a re-mesh in rounds (sure).** B's cure inserts a point and rebuilds. A point inside a triangle does not remove the side unless the rebuild happens to flip it. That is why today's rule has a retry budget and `UnclearedSharedSide`, both of which B keeps. A rebuild can also make new shared sides, hence B's rounds.
- **The seven bored-ball cells do not mesh under any of the three cures alone (sure, measured).** A and B both say they do. In all seven, the outer face's arc seam and the bore's line seam join the same two vertices with one chord each, so the segment (0, 1) is used four times. The census allows that by design (two uses per chord) and `check_mesh` refuses it. They mesh once the chord pass gives the curved edge a second chord: I measured all seven passing `check_mesh`, winding and volume with that floor.
- **A boundary fold needs its own cure (sure, on its cells).** A says a split triangle that folds is "not known to be reachable". Separately from the split: two needle-cone cells mesh today only because the present insert-and-rebuild lands in a folded triangle. With the lane rules off they refuse `FoldedTriangle`; with the fold given its own insert they mesh. A local split does not replace that.
- **The counts of what is actually needed** stand whichever point is used: about 908 sides in about 72 meshes of the default suite, against 997 inserted points in 103 rebuilds plus 1,211 walk cuts today.

### Recommendation on the three differences

1. **What the cure is: a local split at a point of the face's own surface, with the face's certificate (likely).** It is measured only on ruled faces, where today's walk does exactly this. On the triangulating arms, sphere, torus and spline it is argued (each half's chart footprint lies inside its parent's), not run. My chord-midpoint probe ran clean across the suite; it shows the detection is exact and one pass suffices, not that the surface-point certificates pass.
2. **Which patches give way: every patch that has the side inside it (likely).** Never a face whose boundary carries the side. It costs two triangles per extra face and needs no tie rule. This is tied to answer 1: with surface points, "all but one" would let an arbitrary choice decide the geometry.
3. **What it needs beside it: the chord floor and the fold's own insert (sure that both are needed).** Without the first, the answer does not close the cells that raised the question.

### Where the other reports are wrong

- **A**: "two coincident edges are already excluded because the count is per chord", so an overused side after the split is a kernel bug. The per-chord count is what hides the real case above: a line and an arc, on a body that is not pinched, chord to one segment. The census cannot be the last word on a closed mesh until the chord floor exists.
- **A**: lists "two coplanar plane faces" as covered. Two faces with disjoint interiors in one plane cannot both hold a side inside; harmless. The case to watch is its cousin: two faces both flat along the side would put their two points at one position. I can find no body that reaches it.
- **B**: "the overused side is the bore's own seam, which the bore must keep". Measured, the seam is not counted overused (two chords carry it). The overused sides are 10 to 23 rim-to-rim sides inside both faces. B's rule (a boundary user never gives way) is right; this reason for it is not.
- **B**: the rectangle and planar lanes "insert a point and rebuild". On the plane this often leaves the side in place, as above.

## For the orchestrator

- **Point by point, is my first report answered?**
  - No certificate needed by containment: true, but outweighed; A's certificate cost is small and unmeasured.
  - Lane re-entry and the memo: answered by A's split set in the key.
  - "A surface point is no better": withdrawn.
  - "All but one is minimal": withdrawn for surface points; it remains forced for chord midpoints, which is a reason against them.
  - Exactness, purity, ε leaving the mesh, the walked key missing the neighbour's surface: all three agree.
- **Unmeasured, and the one measurement I would ask for before building**: A's split on the triangulating arms and on sphere and torus rectangle faces, over the same suite, counting certificate and fold refusals of the halves. My probe diff (`/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad/fork3/C/scratch/probe.diff`) is the harness: the detection, the lane-rule switch, the chord floor and the fold insert are in it. What it lacks is the chart coordinates of a patch's corners at the join, which A's pass 2 needs from the lane.
- **If the surface-point split refuses where the chord midpoint meshes**, that is a finding to weigh, not a reason to fall back per cell: `[loud-failure]` forbids a fallback that answers where a method declined.
- **A's shape detail I agree with**: run the census on unplaced patches, mint split points as the face's own local points at its arena turn. The present rebuild must not be reused for the split.
- **Tension to name for Ev if asked**: "every face splits" adds two triangles per conflict over "all but one", against `[mesh.sizing]`'s "as coarse as possible". I rank the symmetric rule above two triangles; changing it later is local.
- **Both other reports ran nothing.** Their cell claims (the 7, the 14, the 52) are reasoned. Mine are from the default profile and default ε only, `--test all`; lib unit tests, the slow set and other crates were not run.
- **Agreed by all three, worth filing regardless**: the stale comment above `joined(...)` ("No lane-produced body is known to reach the census"), and the walked lane's key missing the neighbour's surface if the walk's reading survives.
- **New this round**: no further measurement; my worktree is clean and unchanged since the first report.
