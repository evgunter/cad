## For Ev

**Where the three of us agree.**
- The lattice gives way to the boundary: a lattice node (an interior grid point the mesher places) is not laid when the box `c` lattice steps each way around it meets the trim polygon or a crease line. The trim polygon is the face's shared boundary points joined in the chart.
- The mesher reads no ε, so the mesh stays a function of (body, δ), as you ruled on 2026-08-21.
- A small `c`. A half cell refuses on cone and cylinder faces at today's sizing; B and C both measured this.

I hold nothing apart from the others on the design itself. Where I still differ is on two scales, below.

**1. A flat triangle on three boundary points: kept.** I have moved back to keeping it.
- The mesher cannot remove a corner the body gave it.
- A boundary-only flat triangle means the body has a vertex within rounding of another edge without lying on it. That is a state of the body, and the body's validation is the place that owes a verdict on it, not the mesher.
- C's and my runs agree that no live body produces one: the three A×Z bodies of issue 111 have a least triangle height of 0.06 m or more.
- So this is a real difference of final state, but it is never reached today. All three of us now say kept.

**2. The check outside the mesher reads "flat" at ε. I agree with C, not B.**
- ε is the floor a consumer can trust. A triangle 1e-12 m high has an exactly computed normal that means nothing about the surface, because its corners carry ε of slack.
- Reading ε only in a validator moves no output, so D9 is untouched.
- Construction meets this promise with room to spare. A triangle with a lattice corner is at least `c`·step high, and for `c`·step to fall under ε = 1e-9 the δ would have to be far below what the certificate's own rounding verdict already refuses. If that belief is ever wrong, the validator says so loudly.
- B's rounding-scale check would accept the 1e-15 to 1e-9 range, which nothing in the design needs to accept.
- The check is a public `mesh::validate` function beside `check_volume`, which already takes geometry the caller supplies. Tests run it, and so can any caller; `check_mesh` stays index-and-count only.
- This difference is real but reaches no measured mesh.

**3. The emission winding check's bare `> 0.0`: named at the rounding allowance.**
- It is the only check that now meets a flat triangle on three boundary points, and today it settles one by the sign of rounding noise: kept, or refused `FoldedTriangle`.
- Final state, at `cert::rounding_allowance` (the promise's existing "+ rounding" term, so no new constant):
  - if the triangle's signed height is resolved above the allowance, its sign decides fold or not, as today;
  - if not, the triangle is flat: kept when all three corners are boundary points, and a typed bug-refusal when a corner is a lattice node, since the clearance makes that impossible.
- This decides whether a sign means anything; it does not decide which point stays, so B's and C's objections to rounding scale (aimed at dropping points) do not apply.
- C left this as an open row with no ε-free answer; B called it "cheap insurance". This is the answer, and it is a real but small difference of final state.

**4. The clearance constant: its own named quality constant.** I agree with B.
- `ASPECT_CAP` is the spline split's heuristic, and the design page says it can be replaced by what it stands for.
- Reading it for the clearance would give one name two jobs and retire the clearance by accident when the cap goes.
- The value 1/16 is fine: C measured the worst certificate at 1/16 equal to its value at 1/100.
- This is a difference of coupling, not of the mesh emitted.

**The final state, for an implementer and the design page**
- **The lattice (does the work).** A node is laid only if the box `CLEARANCE`·step each way around it, per chart axis, is clear of every polygon side and point and every crease line. On the spline lane the step is each band's own. The page lists `CLEARANCE` = 1/16 as a *quality* constant.
- **The two exact drops become checks.** The duplicate drop in `insert_points` and the node-on-a-side retry in `constrain_boundary` become typed bug-refusals for lattice nodes. Ear and refinement points are centroids strictly inside a triangle with area, so neither drop applies to them.
- **Emission.** The winding check as in point 3.
- **The promise, one sentence in the README and in `FacePatch`.** A triangle with a corner the mesher placed is not flat: its least height exceeds the kernel's ε. A triangle on three boundary points is the body's and is carried as given. No promise is made about shape.
- **Where it is checked.** The `mesh::validate` function from point 2. The suites use it in place of `MIN_SHAPE`. The STL writer's "non-degeneracy contract" sentence points at this promise, and its stale cone-apex example goes.
- **Plan wording.** On "a run on a chart iso line gets a lattice line": that line's nodes on the run are not laid.

**Questions for you, if you want them ruled rather than settled here**
1. Does the validator's "flat" read ε (C and me) or the rounding allowance (B)? I lean ε.
2. Is the clearance its own constant (B and me) or a reading of `ASPECT_CAP` (C)? I lean its own constant.
Point 3 is not a question unless you object; nobody proposed a competing scale for it.

**Ratified text.** `crates/mesh/README.md` gains the promise sentence and the clearance clause with its constant; "not a promise about triangle shape" stays. `docs/DESIGN.md` is unchanged. I withdraw my round-0 proposal to change `[tolerance.chordal]`.

## For the orchestrator

- **Nothing new was run this round.** Every point is settled by the round-1 measurements: C's table of `c` against refusals, B's 50-mesh sweep, and the A×Z runs by me and C, which agree.
- **Not run, but runnable before unit 5 dispatches:**
  - Construct a body with a boundary-only flat triangle on an analytic arm, to exercise point 3's kept branch. It needs a face whose loop has three boundary points collinear within rounding. None is known; B suggested trying a vertex an ulp off a neighbour's straight edge on a cylinder ruling.
  - The spline lane under clearance (all three flag this).
- **Correction to how the coordinator framed my position.** In round 1 I said "refused typed" for boundary-only flat triangles; I now say kept. And "rounding allowance" was my scale for the emission check only, never for the validator: the validator reads ε.
- **For the implementer.** A triangle's least height is 2·area ÷ longest side, computed from its 3-D positions. The emission check computes its rounding allowance at the triangle's coordinate magnitude, `cert::position_scale`.
- **Defects off the question:** as in earlier rounds, plus B's row on the `d9_mesh_goldens.rs` header, which says "(body, δ, ε)" where DESIGN says (body, δ).
