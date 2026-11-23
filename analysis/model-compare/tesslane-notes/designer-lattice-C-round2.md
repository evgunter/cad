## For Ev

**Where I stand.** I now hold one point apart from one designer: point 1, against B. On points 2 and 3 I have moved to A's position; a measurement this round settled both. On point 4 I have moved to B's. Your reading of my round-1 reply was right.

**The measurement.** I ran the mesh crate's whole integration suite on main with a census at the place where each face's triangles join the mesh: 45,370 faces on every lane (27,819 planar), 40.6 million triangles.
- No triangle has least height (2·area ÷ longest side) under 1e-9 m. The least is 3.1e-8 m.
- No triangle's cross product is within 8 ulps of its coordinates times its longest side. The nearest is 1.7e7 times above that.
- 972 triangles have least height between 1e-9 and 1e-6 m. Many are well-shaped triangles of tiny features (a planar one with `q` 0.07 and sides of 5e-7 m; a sphere one with `q` 0.30). 114 of them have three boundary corners.

**1. A flat triangle on three boundary points: refused typed.** I return to my first position, which A now holds; B holds "kept".
- B's ground is the issue-111 needle. A and I each ran those three bodies and none emits one. This round's census finds no flat triangle of any kind on any lane.
- So the two answers differ only on a state nobody has seen. Refusing un-meshes nothing today; keeping keeps nothing.
- Refusing is fail-loud. It leaves `FacePatch`'s "outward winding" true of every triangle with no carve-out, and it is what the analytic arms already do when the product is exactly zero.
- This is a real difference of final state: the promise is either whole or scoped.

**2. The outside check reads "flat" at the rounding allowance, not ε.** I withdraw ε.
- CI sweeps ε at 1e-6. At that row an ε-scale check would call 972 sound triangles flat, well-shaped ones included.
- "Resolved above its own rounding" is arithmetic on the positions given. It needs no bound on how the positions were evaluated, which was B's first-round objection to a rounding count; A's reply (it holds for a rule that drops, not for one that verifies) is right.
- It is a public function in `mesh::validate` beside `check_volume`. The suites call it; `tessellate` does not, as with `check_mesh`. "Test-level" (B) and "what a caller runs" (A) describe the same thing. Wording only.
- With point 1 decided as "refused", the check needs no boundary polylines.

**3. The emission check's bare `> 0.0` is named, at the same scale.** A's proposal; B calls it cheap insurance. No one is apart on the mechanism.
- It answers three ways: outward, folded, unresolved. Folded is handled as today.
- Unresolved refuses typed under its own name, with or without a lattice corner. Under the clearance a lattice corner cannot make one, so there is nothing to drop.
- For three boundary points this replaces today's coin flip (kept, or `FoldedTriangle`, by the sign of rounding noise) with one answer. Which answer is point 1.

**4. The clearance is its own named quality constant, value 1/16.** B's argument holds. The page says `ASPECT_CAP` "can be replaced by the criterion it stands in for", so a clearance read from it would retire by accident, and the cap is a 3-D cell aspect where the clearance is in chart steps. The value was never apart; the dependency was, so this is a small real difference in the code. I yield.

**One wording point.** A and B would write into the page that unit 5's lemma "takes `c` as its hypothesis". That came from my first report and I withdrew it in round 1. The cell bound is about a triangle's extent, and a node too near the boundary enlarges none. The lemma's bound is a function of `c` (boundary triangles reach `1 + c` steps; worst certificate 1.22 cell bounds at 1/16). The page should say that and no more.

**The final state.**

*Lattice (trimmed lane, every arm).*
- `BOUNDARY_CLEARANCE = 1/16`, kind *quality*.
- A lattice node is not laid when the closed box of that fraction of its own steps each way, per chart axis, meets a side of the trim polygon or a crease line. On the spline arm the steps are its band's.
- It is applied once, before the retry loop, to lattice nodes only. Ear and refinement points are not filtered.
- No ε is read and nothing is merged or moved.

*What retires or stays.*
- `Creases::cut_candidates` stops moving nodes onto a crease; the row points it adds on the crease stay.
- The duplicate drop in `insert_points` and the node-on-a-side retry in `constrain_boundary` stay as backstops, unreachable for lattice nodes.

*Emission (`TrimmedFace::emit`).* The three-way verdict of point 3, from `cert::rounding_allowance` at the triangle's coordinate scale times its longest side.

*Outside the mesher.* One definition of "resolved" in `mesh::validate`, read by the emission check, by the public check of point 2, and named by the STL writer's doc. `check_mesh` stays combinatorial. `reach_probes`' `MIN_SHAPE` retires into it.

*The page (`crates/mesh/README.md`, your sign-off).*
- Beside the δ promise: "every triangle's winding is resolved above its own rounding; shape beyond that is not promised."
- A clause for the relation both ways: a boundary chord spans at most one lattice step; a lattice node keeps `BOUNDARY_CLEARANCE` steps from every side and crease.
- The constant under *quality*, with its measurement: worst certificate 1.22 cell bounds at 1/16, the same as at 1/100; 2.21 with refusals at 1/2.
- `docs/DESIGN.md` is unchanged.

**The one question for you (against B).** A triangle whose three corners are boundary points and whose winding is not resolved: does its face refuse typed, or is it emitted?
- Refused: the outward-winding promise stays whole. Emitted: the promise is scoped to triangles with a lattice corner.
- No instance exists in 40.6 million triangles, so neither costs a mesh today.
- I say refused. `likely`.

## For the orchestrator

- **Run this round.** `fork-scratch-C/census/mesh` is a copy of `crates/mesh` with its tests. One function was added in `tessellate.rs`: it appends to `census/flat.log` every placed triangle with least height under 1e-6 m, with its surface kind, lane, and whether all three corners are below the shared-id prefix.
  - Built with `cargo test --release --test all --no-run` into the shared `target/` (test binary `all-42dd98cae45152f6`; the name is cargo's, so I could not suffix it).
  - Run with 4 threads: 256 passed, 1 failed, 37 s. The failure is `profile_overrides::dev_profile_still_optimizes_spade_and_mesh`, which reads the workspace's cargo profile and has none in the copy.
  - The crate's unit tests under `src/` were not run.
- **Per-lane least heights in that log.**
  - Curved cylinder: 3.1e-8 m with three boundary corners, `q` 1.6e-8, longest side 2 m.
  - Curved spindle: 3.1e-8 m, `q` 9.9e-8.
  - Planar: 3.4e-8 m, on a triangle of 5e-7 m sides.
  - Trimmed cone and curved sphere: 1.3e-7 m, both on tiny well-shaped triangles.
  - The first two are needle-class rows for `a-needle-cone-face-meshes-thin-triangles-beside-its-rulings`, on the curved lane.
- **Still unmeasured.**
  - The spline arm under the clearance (B's risk: that arm sizes its worst sliver at δ).
  - The clearance on any corpus but my round-1 one.
  - The eight coincident faces from round 1 were not rerun with the three-way verdict; nothing here prototypes it.
- **Implementer notes.**
  - The unresolved refusal needs a new `TessellateError` arm, for which I suggest no name.
  - `FoldedTriangle` keeps its meaning.
  - The d9 goldens named `tilted_above` and `tilted_below` are what `d9_mesh_goldens.rs`'s header names; A also lists `tilted_halves`, which I did not find in that header.
- **Corrections to my own earlier replies.**
  - Round 1's "ε, read by the validator only" is withdrawn, on this round's census.
  - Round 1's "kept" on point 1 is withdrawn; I held it on a claim two runs had already emptied.
  - Round 0's "read of `ASPECT_CAP`" is withdrawn.