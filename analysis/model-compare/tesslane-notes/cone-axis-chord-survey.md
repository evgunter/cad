cone-axis-winding — the reviewer's second note (a chord point on a rectangle-lane ruling): NOT REACHABLE by any body the suites build. One mechanism exists in principle.

By reading the code: `compute_chords` gives a line one chord, and `tighten` lets each side's lattice raise it, skipping a rectangle-lane side. The trimmed cylinder and cone arms have `hv = ∞` (`AnalyticRow::steps`: a ruled chart needs no step along a ruling), so they never raise a ruling's count. The sphere, torus and spindle charts have no line edges. Only a spline face (NURBS or Approx) on the chart lane can raise a line edge's count.
Measured on the head by instrumenting `compute_chords` to record every line edge whose final chord count is above 1, with both sides' surface and lane:
- mesh suite (494 rows): 236 such edges, all between {Plane, Nurbs} or {Nurbs, Nurbs};
- sweep's 63 meshing modules (584 rows): 80, all between {Plane, Nurbs}, {Nurbs, Nurbs} or {Plane, Approx};
- step-import (329 rows): 3, the same kinds.
None has a rectangle-lane face on either side. The tour went unsurveyed: in debug its eps_regression rows ran past 600 s each, so I stopped it.
So the case the reviewer describes needs a straight edge shared by a rectangle-lane cylinder or cone face and a spline face on the chart lane, and no body in these suites builds one. If one ever arrives, it refuses typed (`SPLIT_SIDE_ROUTED_HERE`) where the base laid its fallback lattice. I propose saying this at the refusal site and in the PR body, plus a tracker row filed with the cure: the route also sends a ruled face whose straight edge is shared with a spline-chart face. That is a read of the neighbour's surface kind, still structural, and I would not build it without a body. Your call.

Still waiting on your answer to the SeamTwice cure question, (a) or (b). The head is unchanged at 4f1bb5c44c.
