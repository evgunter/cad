## For Ev

**Where I moved, and where I did not.** I hold the scope (every cylinder and cone wall, not cone apexes), the home (the chart lane) and the interim answer (send split-side faces there now). I moved on the rule's form: I had it as "no triangle crosses a column"; new measurements show the unsplit face needs no rule at all in the chart lane, and each split side needs exactly one held edge. I took report A's geometric fact as the theorem, because it is simpler than mine. I do not take report A's "join every rim point to its partner" (a frustum's rims have no partners, measured), nor report B's two narrower choices: cones only, and refusing until the final state lands.

**Recommendation.**
- **Final state** (likely): a ruled face with a vertex on a straight side meshes as the same face without it, plus one triangle per vertex.
- **Until then** (sure that they mesh, likely on the route): send every ruled face with a split side to the chart lane and delete the rectangle lane's fallback lattice.
- **PR 7043's refusal stays** as the backstop on every lane (sure).

**Terms.**
- **Ruled face**: a face on a cylinder or cone; its lines `u = const` are straight **rulings**.
- **Split side**: a straight side of the face that is two or more edges.
- **Rectangle lane / chart lane**: the two meshers of curved faces (`mesh::curved`, `mesh::trimmed`).
- **Development**: the cone unrolled flat about its apex, where the chart lane triangulates cones.
- **Rows**: interior lattice points between the rims.
- **Headroom**: the chart lane sizes at half of δ where it has no theorem.
- **Held edge**: an edge the triangulation is forced to keep (a constraint).

### What the three reports agree on

- The fault is the triangulation, not the lattice or the boundary: the rectangle lane triangulates the raw chart, where the apex is a row and nothing keeps a triangle beside one ruling.
- The chart lane's header already describes this triangle and cures it with the development.
- The final home is the chart lane, with the rectangle lane's "no rows on a ruled face" rule moved into it.
- Moving a ruled face there today costs far more than the plan's "about twice": 25 to 86 times on a plain cone.

### New measurements (PR 7043's head, debug build, default ε)

1. **A frustum's two rims are chorded at different counts.** Half-turn frustum, radii 2 and 1:

   | δ | points on the large rim | points on the small rim |
   |---|---|---|
   | 0.03 | 11 | 9 |
   | 0.01 | 17 | 15 |
   | 0.001 | 51 | 43 |

   The large rim keeps its own finer count. So "the other rim's point at the same angle" does not exist from δ 0.03 down.

2. **The chart lane with rows switched off on cylinders and cones** (a temporary edit, no held edge):

   | face | answer |
   |---|---|
   | unsplit cone, frustum, cylinder | meshes at every δ, at 1.25 to 1.45 times the rectangle lane's count |
   | cylinder with a split side | meshes |
   | cone or frustum with a split side | refuses `CertificateExceeded` from δ 0.03 down, often at 0.1 too |

   On the unit 45° cone at δ 0.001 that is 100 triangles, against 72 in the rectangle lane and 6168 with rows. The 1.25 to 1.45 is the headroom's finer rim chords and nothing else.

3. **Slender cones are worse than the statement says.** A half-turn cone of half-angle 5° or 1° with a split side:

   | | δ 0.1 to 0.001 |
   |---|---|
   | today | refuses `UnresolvedWinding` at every δ |
   | sent to the chart lane | meshes at every δ, 149 to 2872 cone triangles |
   | with the held edge (prototype) | meshes at every δ, 5 to 13 cone triangles |

### The final state

**Scope: every ruled face.** A cylinder wall with a split side refuses `CertificateExceeded` from δ 0.01 down. Report A measured the same independently. A frustum meshes only on a lattice no theorem sizes.

**Where: the chart lane**, as two facts on the cylinder's and cone's chart-table rows.

1. **No rows** on a face bounded by rims, rulings and the apex. Measurement 2 shows this is enough for the unsplit face: the chart lane's triangulation makes the strip unaided. So the doubt in report B, whether the strip argument survives in the development, is settled for unsplit faces.
2. **One held edge per split side**: from an end of that side to the nearest point of the far rim, never the apex. Measurement 2 shows it is needed: without it the split cone refuses. The side's vertices then divide that one triangle.

**Why it is within δ** (report A's fact; I checked the geometry). Two rulings of a cone meet at the apex and two of a cylinder are parallel, so they lie in one plane. The side triangle is flat, and vertices on its edge cut it in its own plane. The mesh surface is the unsplit face's, at the unsplit face's bound.

**What it makes true.**
- Splitting a straight edge can never make a face refuse or change its density.
- No triangle lies along one ruling: in the development three points of a ruling are exactly collinear.
- The row `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem` closes on the theorem that already exists.
- `bare_sides`, the fallback lattice and the rectangle lane's apex case retire.

**Measured.** The prototype sits in the rectangle lane (about 40 lines). Every split cone, frustum, cylinder and slender-cone cell meshes at every δ, at the unsplit count plus one per vertex. All 18 cells of the T1 pose pass the outward and true-distance checks.

**Not measured.** The held edge inside the chart lane itself. Its inside/outside count reads constraint crossings, and its ear cure may add a point to the side triangle. That is why the confidence is likely.

**What it leaves.** Two shapes refuse by name: a one-face full-turn wall with a split seam (`SeamTwice`) and a face on both nappes (`BothNappes`). I found no door that builds either. Very thin triangles on slender cones remain, which `[mesh.winding]` allows.

**Reversible**: yes; it is a triangulation choice under an unchanged certificate.

### Where I disagree

- **Report A, the rule's form.** A holds an edge from every rim point to its partner, and flags a frustum with unequal rims as open. Measurement 1 shows that is every frustum from δ 0.03 down. One held edge per split side needs no partner and meshes those cells. On a face through its apex the two rules give the same surface.
- **Report B, scope.** B's final state covers cones only, and keeps rows wherever a side is split. That leaves cylinder walls refusing and split faces at 20 to 500 times the triangles they need (2528 against 6 on the 1° cone at δ 0.001). A split-side face where a held edge cannot be stated, or fails its certificate, would move me to B. I found none in about 300 cells.
- **Report B, a second defect.** B says the chord pass fails to cut a ruling at the row step in the rectangle lane only. The step along a ruling is infinite on the chart row both lanes read, and the chart lane's split cone gains two triangles, not a row of points. So neither lane cuts it. With no rows there is nothing to cut at.

### What the kernel answers until then

**Send every ruled face with a split side to the chart lane now.** The test is the rectangle lane's own admission test, read before the route: each straight side is one edge. The lane computes it today after its walk and answers with the unproved lattice.

- **Measured.** Every cell above meshes there, including cylinder walls and slender cones, and the 18 T1 cells pass the outward and true-distance checks.
- **Its price.** These faces carry 20 to 500 times the triangles they need until the held edge lands.
- **Against report A's narrower route** (only cones through an apex with a split side): it leaves a plain cylinder wall refusing and the frustum on a lattice with neither theorem nor headroom.
- **Against report B's refusal.** B objects that this routes by a property of the boundary, which the plan's final state rejects. That is fair, and the test dies with the rectangle lane. Meanwhile a plain cylinder wall refuses at δ 0.01 and a slender cone at every δ, while a certified mesh is one route away.
- **What would move me to B.** The held edge landing with PR 7043 or right behind it. Then the bridge buys nothing.

If only one thing can land first, PR 7043's typed refusal is better than main's noise normals (sure).

### What needs your word

1. **The bridge, as I read it, does not.** It changes no ratified sentence. The fallback arm it deletes came in an agent's fix pass on PR 5717 and is out of line with `[mesh.sizing]`'s "a lane without its theorem keeps a named headroom". Say so if you count it as part of unit 6.
2. **`[mesh.sizing]`**, which you reviewed in the S29 PR, gains the ruled-face theorem and its consequence: a vertex on a straight edge adds one triangle to each face beside it. I would add the principle too: a lane's triangulation is constrained to what its theorem assumes.
3. **Unit 6 stays yours, asked after the final state, per chart.** The plan sentence that reserves it was written by an agent (commit `f31f71b51f5`), not by you. On today's numbers its own rule would answer "stop at unit 4" for ruled faces, for a reason the two facts above remove.

## For the orchestrator

**Where to look**
- The container restarted mid-round; I rebuilt and re-ran. The build directory is deleted again.
- Probe diff: `/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad/fork2/C/probe.diff`
- Round-two output: `/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad/fork2/C/run2.out`

**What I re-ran or added this round**
- **Frustum rim counts**: printed from the rectangle lane's walk polygon.
- **Chart lane with no rows**: an env switch empties `lattice_rows` for cylinder and cone rows; every cone, or every cone and cylinder, was routed there.
- **Slender cones** (5° and 1° half-angle, half turn): today, routed, and with the strip prototype.
- **Outward check**: `reach_probes::got` answers `Meshed` only after `geometry` passes (winding resolved, outward, sampled distance within δ). So round one's 18 T1 cells, under both the route and the prototype, passed all three.
- **Cylinder with a split side**: report A's finding matches my round-one measurement.

**Taken from the other reports, unverified**
- **Report B's T1 triangle corners.** They are consistent with my dump on the quarter cone; I did not log T1 itself.
- **Report B's density table.** Its full-cone chart-lane counts are about twice mine. Report A's 2130 against 82 at δ 0.003 matches mine exactly. I did not find the reason (a different body, or a different count). Nothing rests on it.
- **Report A's cylinder bounds** (0.20, 0.14, 0.13).
- **Report A's sphere remark**: a pole copy plus two nodes of one meridian would be a resolved triangle with a tangent normal, and the join reads resolution only. My sphere cells mesh and pass the resolution check; I did not run an outward check on them. It is worth a row of its own.
- **Report B's worry that slender cones might refuse in the chart lane**: not on the two I ran.

**Corrections to my first report**
- Its phrase "constrained so no triangle crosses a column" overstated the rule. The prototype holds one edge per split side, and the unsplit face needs none in the chart lane.
- Its "20 to 80 times" for the bridge's density understates slender cones (150 to 500 times).

**Still not checked**
- The held edge inside `trimmed.rs`.
- The raw-chart prototype on a one-column apex face with both sides split: one pose passes and I have no argument that it must.
- Main itself, the release profile, other ε.
- A true both-nappes face.
- Whether any door builds a one-face full-turn ruled wall.

**The bridge's predicate.** My probe used "more than two line-carrier traversals in the outer loop", which also catches keyways, notches and slits. The real one is "an iso-rectangle whose straight side is more than one edge", composed with unit 4's domain reading. `chords` and `face_target` call `lane_of`, so it must be readable before the chord pass; the probe did it that way and it works.

**Rows to file (unchanged from round one)**
- A cylinder wall with a split side refuses `CertificateExceeded`.
- The comment on `curved::tests::TOTAL_REFUSED` blames the mirror nappe's cones; the four refusals are that fixture's cylinder wall.
- The closed row `curved-lane-triangulates-in-raw-chart-coordinates` rests on a premise a split vertex falsifies.

I read no PR thread and no `analysis/` branch.
