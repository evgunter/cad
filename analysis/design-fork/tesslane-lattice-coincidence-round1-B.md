## For Ev

**Recommendation now (likely).** I still recommend that the lattice keep clear of the boundary, but I have moved on the size of the gap.

- **The lattice.** A lattice node (a point of the trimmed lane's regular interior grid) is not laid if the boundary passes within `c` lattice steps of it on either chart axis. "The boundary" here means a side of the trim polygon, the face's shared chord points joined in the chart.
  - `c` is small and is its own named quality constant, recorded with the mesher's other constants in `crates/mesh/README.md`.
  - On the spline lane the step is each band's own.
- **What does the work.** The clearance alone prevents the defect.
  - Nothing anywhere decides whether two points are the same, so no ε is read and D9 stays as Ev ruled it.
  - Points the mesher adds later (ear and refinement centroids) need nothing: each lies strictly inside a triangle that has area.
- **At emission.** Nothing new is decided. The existing winding check stays, and under the clearance it never sees a flat triangle with a lattice corner. A flat triangle made only of boundary points belongs to the body and is kept, as the planar lane keeps its #111 needle.
- **The promise.** `crates/mesh/README.md` and `FacePatch` gain one sentence: a triangle with a corner the mesher placed has area. This is what "outward winding" already presupposes. Construction guarantees it. A test-level check in `mesh::validate` re-derives it at rounding scale, using the boundary polylines to tell the mesher's corners from the boundary's; it reads no ε.

**1. What moved me.** Report C measured that my first answer, `c` = ½ (a node's own cell), refuses `CertificateExceeded` at today's sizing on cone and cylinder faces. I checked this by running a filtered copy of the trimmed lane on `trimmed`'s cone, torus and tilted-cylinder test bodies plus the tilted sphere pair, 5 values of δ each:

| `c` | refusals out of 50 meshes | least 2·area/L² |
|---|---|---|
| main | 0 | 2.4e-3 |
| 1/16 | 0 | 2.4e-3 |
| 1/4 | 0 | 2.4e-3 |
| 1/2 | 7 | — |

The seven refusals at ½ are the narrowing and widening frusta at δ 2e-3 and 1e-3, and the tilted cylinder of radius 0.3 at δ 1e-2, 2e-3 and 1e-3. My first round said "no new refusals", but I had measured the sphere arm only, so that claim was wrong. ½ needs the unproved containment lemma (a bound on the triangles beside a boundary that crosses lattice lines) and finer sizing. That works against the ratified rule "as coarse as possible", whereas shape is not promised at all. So the final state wants a small `c`.

**2. Arguments against my recommendation, and whether they hold.**
- *(A) "The triangles beside the boundary grow, which ties the fix to the unproved lemma."* This holds for ½ (measured above). It does not hold for small `c`: no refusal at 1/16 or 1/4, and C measured the worst certificate rising from 1.06 to 1.22 cell bounds at 1/16, inside the headroom of 2.
- *(A) "It does not cover ear and refinement points."* This does not hold. Those points sit strictly inside a triangle whose height is far above rounding, so they cannot make a flat triangle against a side.
- *(A) "Every trimmed face moves."* That is true and it is a cost, not a ranking argument. With small `c`, a face moves only where a node lies within `c` steps of the boundary: C counted 292 of 486 faces at 1/16.
- *(A) "The ε sentence in `[tolerance.chordal]` has no ruling of yours behind it (it was written in the editing pass 99cc678)."* This does not hold. Commit `1240686` ("D9 keeps its promise unqualified; the pole classification is guarded (#896)") is authored by your account. Its text reads: "The ruling: this paragraph keeps its promise unqualified … this project does not read intent into numerical coincidence." Deciding flatness at ε would move emitted coordinates with ε, so A's answer reverses that ruling.
- *(C) "`c` should be 1/ASPECT_CAP (1/16)."* The value holds; the name does not. `ASPECT_CAP` is the NURBS split's heuristic, and the page says it "can be replaced by the criterion it stands in for". Tying the clearance to it would give one name two jobs and retire the clearance by accident when the cap goes. The clearance should be its own named quality constant (1/16 is fine).
- *(C) "A flat triangle made only of boundary points refuses typed."* This does not hold. The planar lane's #111 needle is a flat boundary-only triangle that keeps a face watertight across a vertex an ulp off its neighbour's edge. A was right that the promise covers only triangles with a mesher-placed corner.

**3. Where we still disagree, each as one question for you.**
- **With A:** may the run's ε decide whether a mesher-placed point survives, so that the mesh moves when ε moves (reversing the 2026-08-21 ruling)? Or does the lattice keep clear of the boundary in its own unit, so the mesh stays a function of (body, δ)? I hold the second (sure that A requires the reversal; likely that the reversal is not worth it). A's own sweep found every case is an exact coincidence split by rounding. Any rule cures it, so nothing forces ε into the mesh.
- **With C:** is the clearance its own named quality constant, or a reuse of `ASPECT_CAP`? And is a flat triangle made only of boundary points kept, or refused? I say: its own constant, and kept.

**4. Ratified text.**
- `crates/mesh/README.md` gains:
  - the area sentence beside the δ promise;
  - the clearance under "quality";
  - a note that unit 5's lemma takes `c` as its hypothesis.
- "Not a promise about triangle shape" stays.
- No change to DESIGN.md.
- `work/tesslane/plan.md`'s "a run on a chart iso line gets a lattice line" stands; that line's nodes on the run are simply not laid.

## For the orchestrator

- **What I ran.**
  - I reused my first-round copy of `crates/` (`/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad/latws-q7`). Its `trimmed.rs` takes `CLEAR=<c>`, analytic arms only, and drops a candidate whose box of half-size `c`·step meets any polygon side.
  - I added a unit test `b_clearance_sweep` inside the copy's `trimmed` tests. It covers `cone_and_torus_bodies`, `tilted_cylinder_above(0.3, 0.5, 0.8)` and `tilted_sphere_pair` at δ 2e-2, 1e-2, 5e-3, 2e-3 and 1e-3.
  - Binary: `/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad/fork-scratch-B/mesh-sweep-B`. Outputs: `out-{none,0.0625,0.25,0.5}.txt` in the same directory.
  - Total triangles at 1/16 are 133,992 against 134,344 on main.
- **Not checked.**
  - The spline lane under clearance.
  - The worst certificate against δ: I recorded refusals only and rely on C's 1.22 figure.
  - The torus instance from the brief.
- **For the implementer.**
  - Under clearance, both the duplicate drop in `insert_points` and the node-on-a-side retry in `constrain_boundary` become unreachable for lattice nodes and can become typed checks. C saw the retry never fire, on main or under the rule.
  - The winding check's raw `> 0.0` stays harmless only while the clearance holds. A rounding-resolved verdict there is cheap insurance, and it is not ε.
- **Correction to my first report.** "No new refusals" held only for the sphere arm. Cone and cylinder faces refuse at `c` = ½.
- **Provenance A misread:** `git show 1240686092b -- docs/DESIGN.md` is the ruling. Its author is `evgunter`; 99cc678 only restated it later.
