## For Ev

**Point 2, the check outside the mesher: C's census does not settle the scale; at the ε rows CI sweeps an ε-scale check flags nothing.** C argued that at ε = 1e-6 an ε check would call 972 sound triangles flat. I measured that claim and it does not hold.
- What I ran: a copy of the mesh crate whose `tessellate` logs every emitted triangle with least height (2·area ÷ longest side) under 1e-6 m. I ran the whole integration suite (`tests/all.rs`) at each of the three ε values CI sweeps: the default 1e-9, then 1e-6 and 1e-12.

| ε | triangles | least height < 1e-6 m | least height ≤ ε |
|---|---|---|---|
| 1e-9 (default) | 40,540,684 | 972 | 0 |
| 1e-6 | 40,460,472 | **0** | 0 |
| 1e-12 | 39,484,022 | 972 | 0 |

- The 972 small triangles exist only in the runs where ε is far below their size, so the bodies holding them build at 1e-9 and 1e-12 but not at 1e-6 (I did not trace why). The least height in the default run is 3.1e-8 m, against ε = 1e-9.
- The ε check and C's rounding check therefore both answer "no flat triangle" on every mesh the suite makes, at every ε. The census shows the two scales agree on what the suite makes. It does not choose between them.
- The difference is only in what the promise says. "Correct to within ε" is the kernel's meaning of a point ([intrinsic.epsilon]). A corner within ε of its opposite side is not on a side the kernel can name. So the promise reads ε, and only the validator reads it, which keeps D9 intact.
- I hold ε (likely). The emission check stays at the rounding allowance: that is the mesher's own guard, and it reads no ε.

**Point 1, a triangle on three boundary points whose winding is unresolved: I now say its face refuses typed.** I tried each construction the coordinator listed (none built, so each "refuses" below is reasoned from the rule, not run):
- **A vertex within rounding of an edge it is not on.** The body's census of cell distances ([verdicts], the census clause) reads the pair inside the band: the body refuses at rest, or the pair is recorded as a coincidence and glued, in which case the vertex is on the edge. Not admitted as a separate state.
- **A face that is itself a sliver** (two of its edges within ε over their extent). The same census refuses it. Not admitted.
- **Three points of one straight edge chorded into several.** The points are collinear in the chart and form one constraint chain, so the triangulation makes no triangle on them. If rounding bends the chain, any triangle it makes is an ear. The ear cure then inserts a centroid, giving flat triangles with a mesher-placed corner, which the agreed emission check already refuses. So no flat triangle on three boundary points survives this case.
- **Two edges tangent at a shared vertex (a cusp).** Admitted at rest, but chords at a δ-sized step leave a gap of order δ, not rounding. Thin, not flat.
- I can name no state that a body valid at rest admits and that produces such a triangle. On every mesh C and I sampled, and on 40 million triangles at three ε here, there is none.

So the two final states differ only on a body its own validation refuses, or on a body nobody has built. `tessellate` accepts an unvalidated `Body`, though, so a caller can reach the state:
- **"Kept":** the caller holds a mesh that is watertight (the triangle shares its edges as usual) but carries a facet with no winding, against `FacePatch`'s "outward".
- **"Refused":** the caller holds a typed refusal naming the face of a body that would fail its own validation.

Refused is true to both the page and `FacePatch` with no carve-out. It costs nothing on a valid body. It also lets the promise and the validator drop the boundary-polyline scoping. My earlier ground for "kept" (the lane cannot remove a corner the body gave) only mattered if a valid body gave one. **Moved to refused (likely).**

**Recommendation on points 1 and 2:**
- **The promise**, in the page and in `FacePatch`: no corner of any triangle lies within ε of its opposite side.
- **What guarantees it:** the clearance for lattice corners, and the body's validity for boundary corners.
- **In the mesher:** the agreed three-way winding check at the rounding allowance refuses an unresolved triangle typed, whatever its corners.
- **Where it is checked:** the public `mesh::validate` function, which reads ε and needs no boundary polylines.

**Still apart, with C, only on point 2.** The question for Ev: does the mesh's "not flat" promise mean height above ε (the kernel's point tolerance, read only by the validator), or above f64 rounding?
- **ε:** the promise says the winding means something about the surface. A 1e-12 m triangle on a part toleranced at 1e-9 fails it.
- **Rounding:** the promise says only that the winding can be computed.
- On every mesh the suite makes, the two answers are the same.

## For the orchestrator

- **What I ran.**
  - The census is a hook in `tessellate()` (the public entry only, not the memo entry) in my copy at `…/scratchpad/latws-q7/crates/mesh/src/tessellate.rs`, writing to the file named by `CENSUS_B`.
  - Binary: `…/scratchpad/fork-scratch-B/all-census-B`.
  - Logs: `…/fork-scratch-B/census-{default,1e-6,1e-12}.log` and `run-*.txt`.
  - Each run: 257 tests passed. The one failure is `the_eps_inventory_is_pinned`, caused by my hook's own ε read.
  - The number of `tessellate` calls differs between runs (10,887 at default, 10,686 at 1e-6).
- **Not done.**
  - I did not trace why the 972 small triangles are absent at 1e-6 (fixtures scaled to ε, or bodies that refuse at 1e-6).
  - I did not build a body with a straight edge chorded into several points on a curved face, so the ear argument in point 1 is reasoned, not run.
- **C's claim** that the 1e-6 row flags 972 is refuted by the 1e-6 run above: zero triangles under 1e-6 m.
