## For Ev

**The question beneath both points.** Each side has assumed that a boundary point is a corner the body gave. Only the vertices are. The chord points between them are placed by the mesher's own chord pass, at a count it takes from δ and the face's lattice step. Both points turn on that, and I built the body that shows it.

**Point 2: does my census settle the scale? No, and one sentence of mine was wrong.**
- At the default ε the census has no triangle under 1e-9 m, so it does not refute an ε check.
- My round-2 line that "at the 1e-6 row 972 sound triangles would read flat" was an inference from the default-ε run. I reran the suite at both ε rows CI sweeps. At ε 1e-6: 44,570 faces, no triangle at or under 1e-6 m. At ε 1e-12: 44,361 faces, least height 3.1e-8 m. The suite's small bodies scale with ε. That sentence is withdrawn.
- I also miscounted: 116 of the 972 have three boundary corners, not 114.

**What does settle it: a body valid at rest on which the ε check fails a sound mesh.**
- The body: the frustum `revolved[(0,0), (1,0), (r,1), (0,1)]` of the reach probes with small rim `r = 20ε`, its upper half cut by the tilted plane of `trimmed`'s tests. All three validation tiers pass. It meshes on the trimmed cone arm.
- At δ 1e-4 and the agreed clearance of 1/16, the mesh has 17,388 triangles and `check_mesh` passes. 226 triangles with a lattice corner have least height at or under ε (least 5.6e-10 m at ε 1e-9). The count is the same 226 at ε 1e-6 and at ε 1e-12, and the same without the clearance. At δ 1e-5 it is 712.
- None of them is unresolved at the rounding allowance.
- Why: a triangle's least height is at most its shortest side. The chord pass chords the small rim at the cone's angular step, so its chords are `r·θ` long, under ε while the rim itself is well over K·ε. The clearance bounds a node's distance from a side, not the side's length. "A triangle with a lattice corner is at least `c`·step high" is true of its height over the boundary side and false of its least height.
- So an ε-scale check, as A and B word the promise, answers "flat" on 226 sound, certified triangles of a valid body, at every ε. That answer is wrong: their windings are resolved, and ε of slack against the surface is a bound on the promise, not noise in the positions.

**Point 1: what makes a triangle on three boundary points with an unresolved winding.**
- *Three points of one straight edge.* Valid at rest. They are collinear in the chart of both faces. A triangulation makes no triangle on exactly collinear points; an ulp-off one is outside the face or not Delaunay. None appears in the suite's 27,819 planar faces. The second half is reasoned, not run.
- *A vertex an ulp off the line of its own two collinear edges* (issue 111). Valid at rest. Main emits no needle on its three bodies (A's run and mine).
- *A vertex within rounding of an edge it is not on; a face that is itself a sliver.* Not valid at rest inside the band. A rim of 12ε is refused at the build (`chord_side`, margin 8.5e-9 in the band 1e-9 to 1e-8). Outside the band a sliver is at least K·ε wide, resolved by about seven orders at unit scale.
- *A small circle chorded finely.* Valid at rest, and built: the same frustum, whole, rim 20ε = 2e-8 m, at δ 1e-8.
  - Main meshes it: 81,792 triangles, `check_mesh` passes, volume 1.047198 (π/3).
  - 6,610 triangles on three boundary points have a cross product within 8 ulps of their coordinates times their longest side (0.64 of it). By size they are in the small planar cap: about 20,000 chord points lie on a circle of radius 2e-8 m, and three consecutive ones bend by 1.1e-15 m.
  - In this run none is wound against its face, on the axis or moved one metre off it.
  - At the ε 1e-12 row the same body at δ 1e-5 has 219.
  - Three times finer in δ, the cone face refuses `CertificateExceeded`.
- So an instance exists on a body valid at rest, and its three corners are the mesher's chord points. It is a fork.

**What a caller holds.**
- *Kept:* the mesh main returns today. It is watertight, with 6,610 triangles whose winding sits inside the positions' own rounding. `FacePatch` then reads "outward, except triangles on three boundary points". The exemption covers every triangle of every planar face and of every ruled face on the curved lane, because neither places an interior point (the whole frustum has no triangle with a mesher-placed interior corner).
- *Refused:* `tessellate` refuses that frustum at δ 1e-8, naming the cap; from about δ 2e-8 up it meshes as today. Nothing in the suite refuses (nearest margin 1.7e7).
  - This needs the verdict where faces join the mesh, on every lane. The cap is planar, and the agreed check in `TrimmedFace::emit` never sees it. On the trimmed arm alone I found no instance (none unresolved on the cut frustum down to δ 1e-5).
- *Which is true to the page:* refused. The page says "if a triangle cannot be certified, the face refuses typed", and the crate already refuses a face whose certificate f64 cannot resolve (`RoundingUnresolved`). The instance sits within a factor of three in δ of that refusal.

**My recommendation on the two points.**
- **Point 2:** the outside check reads the rounding allowance over every triangle, with no scope and no ε. `sure` that ε is the wrong scale.
- **Point 1:** refused, by one verdict where faces join the mesh, so the promise is whole on every lane. `likely`.
- The cause of the instance is the chord rule on a small rim. Curing it there removes the instance under either answer; it is its own row.

**The one question for you.** A triangle whose winding f64 does not resolve: does `tessellate` refuse its face on every lane, or does the mesh carry it?
- *Refuse* makes true: every triangle of every returned mesh has a winding f64 stands behind. The 20ε-rim frustum refuses at δ 1e-8, where main meshes it.
- *Carry* makes true: every body that meshes today still meshes. The promise then exempts triangles on three boundary points, the outside check is scoped to lattice corners (still at rounding), and a consumer's normal there may be noise.

## For the orchestrator

- **Runs this round**, all under `fork-scratch-C/`, all built into the shared `target/release`.
  - `rimsplit/` (binary `rimsplit-probe-C`): my round-0 prototype copy of the mesh crate, so `PROBE_CLEAR=0.0625` applies the clearance. Arguments are the rim as a multiple of ε, then δ.
    - Rim 20ε at δ 5e-3 gives 0 lattice-cornered triangles at or under ε; at δ 1e-4 it gives 226; at δ 1e-5 it gives 712 (167,536 triangles).
    - Rim 200ε at δ 1e-4 gives 0.
    - Run at `CAD_TOLERANCE_EPS` unset, 1e-6 and 1e-12.
  - `frustum/` (binary `frustum-probe-C`): the tree's own mesh crate. Arguments are rim multiple, δ, and an optional shift.
    - At default ε, δ 1e-7 has nothing unresolved (ratio 6.4); δ 1e-8 has 6,780 on the axis and 6,610 shifted by (1, 0, 1); δ 3e-9 and 1e-9 refuse `CertificateExceeded`.
    - At ε 1e-12, δ 1e-5 has 214 on the axis and 219 shifted; δ 1e-6 on the axis has 1,512, and shifted it refuses `CertificateExceeded`.
    - Rim 12ε panics in the fixture on the kernel's in-band refusal.
  - `census/`: the round-2 suite binary rerun with `CAD_TOLERANCE_EPS` 1e-6 and 1e-12 (`flat_1e-6.log`, `flat_1e-12.log`). 256 passed and 1 failed at each, the same profile row as before.
- **Caveats.**
  - "Unresolved" is my reading of the agreed allowance: 8 ulps × largest coordinate × longest side. It models the positions' evaluation rounding and is conservative: the cap's ears are about 5 ulps high and all came out wound right.
  - That the 6,610 are in the cap is from their size, not from reading the patch.
  - "From about δ 2e-8 up it meshes" is interpolated between the ratios at 1e-7 and 1e-8.
  - Not run: an unresolved triangle on three boundary points on the trimmed arm (none found); the vertex-near-a-foreign-edge case against tier 3; the straight-edge case on a fixture built for it.
- **A row to file** (I may not commit): the chord pass chords a small rim at its face's angular lattice step. On a frustum with a rim of radius 2e-8 m at δ 1e-8 that is about 20,000 chord points 6e-12 m apart. It is a sizing waste under "as coarse as possible" and the only source I found of a rounding-flat triangle on a valid body. It belongs with unit 1's rows.
- **For the other two designers' text.** The promise sentence "a triangle with a corner the mesher placed is not flat: its least height exceeds ε" is false of the cut frustum above under the agreed design.
- **Corrections to my round-2 reply:** the 1e-6-row sentence and the count of 114, both stated in "For Ev". Round 2's "nothing new in the mesher for other lanes" is also withdrawn: holding point 1 as "refused" needs the verdict at the fold.