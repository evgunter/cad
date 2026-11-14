## For Ev

**Bottom line: I now hold the same position as C on both points. Against B one question remains, the scale (point 2), and the frustum below answers it.**

**What I reproduced.** I ran the tree's own mesh crate, unmodified, on C's whole frustum: profile `[(0,0), (1,0), (r,1), (0,1)]` revolved about y, rim radius `r = 2e-8` (20ε at the default ε), built with `finished`.
- **At δ 1e-8:**
  - Main meshes it: 81,792 triangles, `check_mesh` passes.
  - 6,780 triangles have all three corners on boundary polylines and a least height (2·area ÷ longest side) within the rounding allowance (8 ulps of the largest coordinate). That matches C's on-axis count exactly.
  - Every one of those corners is a chord point the mesher placed, not a vertex of the body.
- **At δ 1e-7:** none is unresolved. But 11,803 boundary-only triangles have least height ≤ 1e-9 m, and 37,351 do at δ 1e-8.
- **Not run:** the cut frustum, because it needs the clearance prototype I do not have. I checked C's mechanism by arithmetic instead:
  - At δ 1e-4 the cone's angular step is about √(8δ) ≈ 0.028.
  - A chord on the 2e-8 rim is then 2e-8 × 0.028 = 5.6e-10 m long.
  - A triangle's least height is at most its shortest side, so any triangle with a lattice corner standing on such a chord is at or under ε. That is C's measured least, 5.6e-10, exactly.
  - The clearance bounds a node's distance from a side, not the side's length. My round-2 claim that "a triangle with a lattice corner is at least `c`·step high" is true of the height over that side only.

**Point 2: the check reads the rounding allowance. ε is a promise the design cannot keep.**
- On this valid body, "least height exceeds ε" fails twice over:
  - for lattice-cornered triangles on the cut frustum, whatever the clearance;
  - for thousands of boundary-only triangles in the whole frustum's cap, at a δ where nothing is unresolved.
- Both failures happen with no ε read in the mesher, which your D9 ruling forbids anyway. So B's sentence ("no corner of any triangle lies within ε of its opposite side") would be false of meshes `tessellate` returns for valid bodies, and its check would fail sound, certified meshes.
- B's census (no difference between the two scales across the suite, at every ε) is right, but it is evidence about the suite, not about the promise.
- What rounding scale tells a consumer: **every triangle's winding is a fact the f64 positions determine**. Its normal is that triangle's own, never rounding noise. Shape and meaning relative to ε are not promised: a sub-ε triangle comes from a sub-ε feature or a fine δ, and the body's validation, not the mesh, owns ε.

**Point 1: refuse typed, decided where each face's triangles join the mesh, on every lane.**
- The instance overturns the ground for "kept". Its corners are chord points the mesher placed, not corners the body gave. So the noise-normal triangles are the mesher's own product, and the mesher can be held to the promise.
- Refusing a body main meshes today is acceptable: nothing is released, and what main returns carries 6,780 facets whose normals are noise.
- The refusal sits beside the crate's existing `RoundingUnresolved`. C measured that same body refusing `CertificateExceeded` only three times finer in δ.
- The verdict must sit at the join, not only in the trimmed lane's emission check: the cap is a planar face, and that check never sees it. The trimmed emission check keeps the agreed three-way reading (outward, folded, unresolved) for its own fold decision, using the same definition.
- **The root of this instance is its own row.** The chord pass chords the small rim at the cone's angular lattice step: about 20,000 points spaced 6e-12 m apart on a 2e-8 m circle. That is waste under "as coarse as possible". Curing it makes this body mesh again, under either answer to point 1. It does not change this recommendation, because the promise should not depend on no other route ever producing such a triangle.

**The final state (points 1 and 2):**
- **Promise** (`crates/mesh/README.md`, beside the δ promise, and in `FacePatch`): every triangle's winding is resolved above its own rounding. No other shape is promised.
- **Definition:** one, in `mesh::validate`. A triangle is resolved when its least height exceeds `cert::rounding_allowance` at its largest coordinate.
- **Enforcement:** `tessellate` reads that definition as each face's triangles join the mesh, on every lane, and refuses an unresolved triangle typed with a new variant naming the face. The trimmed lane's emission check reads the same definition to tell a fold from noise.
- **Check:** a public `mesh::validate` check beside `check_volume` re-derives it. It needs no ε and no boundary polylines.

**The one question for you (against B):** does the mesh promise that every triangle is not flat at ε, or only that every winding is resolved above f64 rounding?
- **ε:** the promise speaks of the surface, but the design cannot keep it. The 20ε-rim frustum breaks it at δ 1e-4 (cut, with lattice corners) and at δ 1e-7 (whole, in the cap). Holding it would need the mesher to read ε, which reverses your D9 ruling, or bodies like that frustum refusing.
- **Rounding:** the promise is kept by construction (plus the refusal above). The 20ε frustum meshes at δ 1e-7 and refuses only at δ 1e-8, until its chord row is cured.

## For the orchestrator

- **Ran:** `…/scratchpad/fork-scratch-A` (binary `needle-a`, the tree's own crates, shared `target/`), arguments `rim δ`:
  - `2e-8 1e-7` gives 25,864 triangles, 0 unresolved, 11,803 boundary-only triangles at or under 1e-9 m, least height 1.13e-14;
  - `2e-8 1e-8` gives 81,792 triangles, 6,780 unresolved, 37,351 at or under 1e-9 m, least height 1.13e-15.
- **A false start, now corrected:** my first count compared height against 8u·M·L, which is dimensionally wrong and read 0 unresolved. With the allowance as a length (8u·M) it reads 6,780, which agrees with C.
- **Not run:** the cut frustum (checked by the arithmetic above); any ε other than the default.
- **B's round 3** puts the refusal in the trimmed emission check only ("whatever its corners"). That misses the planar cap; C's join-point placement is needed.
- **To file:** the small-rim chording row (C's), with unit 1.
