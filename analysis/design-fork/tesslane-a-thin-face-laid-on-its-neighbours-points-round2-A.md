Reconciliation round 2, designer A.

**(1) Is your reading of my position right?** Yes, with two corrections.

- My "no rule" was held at `unsure`, as you say; it has now changed (below).
- "Per shell, by a check the STL door calls" needs one detail. The `stl` writer holds no body by its own doc ("it has no access to the body at all — and it must not"), and a void's shell is rightly negative, so a per-shell read needs the body. It is therefore a check that takes the body, beside `check_volume`, called by the export path that holds both. The writer alone could only read the whole-mesh sign. This is the parked row's detail, not a disagreement.

**(2) The pair trigger: yes, it lands in the unit, on a measured condition** (`likely`; was "no", `unsure`).

Why I moved:
- All three premise checks said the census looks for a symptom (one side made twice) of the defect (two patches covering one region with only the points they share). Leaving the census as it is acts on the symptom only.
- The unit's own split creates the +60 / −60 asymmetry: it opens the wall where diagonals coincide and not where they differ. The pair trigger completes one join fact; it is not a second idea.
- My objection was "density with no clause behind it". With the sign held at export, there is something behind it: the trigger is what keeps that check from refusing caps the base wrote to a file. Removing it breaks those exports, so it earns its place.
- `[mesh.sizing]` governs the grid step. The existing split already adds points that distance does not need.

What I still hold, and it is the condition:
- **A split with no promise behind it may never cost a body in `tessellate`.** A failed piece under the existing trigger refuses for a reason (the mesh would not be manifold). A failed piece under the pair trigger would turn a correct flat `Ok` into a refusal for nothing.
- On a sphere it cannot fail: the foot theorem holds for the side split and for the centroid point alike (det[a, b, a+b+c] = det[a, b, c], and the pieces are pushed outward).
- On a cone, torus or cylinder at very coarse δ it can: the foot is far, or undefined on the axis. The viewer's probe at δ = 1e9 is exactly where two-face bodies are bare toward each other.
- Skipping a failed pair split silently is not an answer either: it is the fallback `[loud-failure]` rules out.

**(3) A rule all three could sign.**

Tree: head + foot + pair trigger + centroid point. Cells: the 435-cell class, the plain-ball cap, and the `mesh` and `sweep` suites at the three ε, each also at δ = 1e9 and down the viewer's fit ladder.

The trigger lands in the unit if all three hold:
- (a) no body is refused that head + foot alone returns `Ok`, at any δ including 1e9;
- (b) no class cell is flat where the base was positive (cells at 1e9 are exempt: flat is correct there);
- (c) the added triangles are under about 1% of the suites' total.

It stays out if:
- (a) fails: the trigger goes to the parked row, with "refuse or leave flat when a pair split fails" as its own question for Ev;
- (b) fails on more than a handful of cells: the mechanism does not do its job (the criterion another designer set for themselves).

The centroid point lands only if cells in the class need it, not for the probe alone. Either way the foot and the corrected wording land. If the trigger stays out, the flat cells are a correct `Ok`, listed in the parked row, and do not hold the merge. That last sentence is the part the others may not sign.

## For Ev

**Recommendation, four decisions.**

1. **`tessellate` owes distance and a closed manifold, not volume** (`likely`). A flat mesh of a body thinner than δ is a correct `Ok`. The sign is read where a solid is needed: a check that takes the body, per shell, which the export path calls and refuses typed on, naming δ. Your "zero volume sounds annoying" is met where it bites: no zero-volume mesh reaches a file.
2. **Split at the foot** (`sure` on sphere and cylinder, `likely` on cone and torus): the point of the face's own surface nearest the side's straight midpoint, not the point over its chart midpoint. Spline and spindle faces keep the chart midpoint.
3. **Widen what the join looks for, if the measurement allows** (`likely`). Besides a side two patches both make, the join acts where two faces' patches each hold a triangle built only from points the two faces share. Each patch splits those triangles' inside sides at its own surface.
4. **The unit merges with (2), (3) if it passes, and corrected wording.** The check in (1) and one README sentence are the parked row's.

**Terms.** A *patch* is one face's triangles. An *inside side* is a triangle side that is not a chord of the face's boundary. The *foot* of a point is the nearest point of a surface. *Flat* means signed volume at rounding.

**Premise check.**
- The join's census finds a side made twice, a combinatorial fact. The defect is geometric: two patches covering one region with the same points. The two coincide only when both lanes lay the same diagonal, so the thin cap keeps 0.59 of its volume at latitude +60 and is flat at −60.
- The base's retired rule kept such faces apart as a by-product; it was not only a wrong prediction.
- No rule inside the mesher reaches every case. Two faces closer than δ that share no point (a hollow ball) give the join nothing to split.
- The viewer meshes every body at δ = 1e9 to size it (`scene::SCALE_PROBE_DELTA`), so thin-against-δ is routine. A sign refusal inside `tessellate` would break that probe.
- "Thinner than δ" has a sharp form: δ times the body's area reaches its volume. Past that, distance promises nothing about volume from any mesher.

**Decision 2 in detail.**
- On a sphere the foot is the radial projection of the midpoint. Each piece winds as its parent and lies no farther from the sphere, so a split there cannot refuse. That recovers the bodies the unit loses near poles.
- The chart midpoint's verdict depends on where the face's frame puts its pole, which the body does not fix.
- On a cylinder the foot is today's point. On a cone and torus it is one rule in place of the cone's special cases, to be measured.

**Decision 3 in detail.**
- The patch stays a function of its face and δ. Nothing is meshed again and no lane reads its neighbours.
- It lands only if, measured, it refuses no body that the unit without it returns, at any δ including the viewer's probe. A split with no promise behind it must never cost a body.
- It must also leave no class cell flat where the base had volume, and add under about 1% of triangles.
- If it fails the first test, it goes to the parked row with one question for you: when such a split fails, refuse or leave the mesh flat.

**Rejected.**
- Every curved face splitting every inside side between two of its own boundary points: it triples plain bands, against `[mesh.sizing]`.
- Reading which faces lie across an edge: it puts neighbours back in the patch's memo key.
- A sign refusal inside `tessellate`: see the viewer's probe above.

**Ratified text, for your sign-off.**
- Add to `crates/mesh/README.md`: once δ times the body's area reaches its volume, a mesh within δ may enclose nothing; `tessellate` returns it, and a reader that needs a solid checks.
- Clarify `[mesh.winding]`: "outward" is each triangle against its own face and says nothing about the whole body.
- `[tolerance.chordal]` is unchanged.
