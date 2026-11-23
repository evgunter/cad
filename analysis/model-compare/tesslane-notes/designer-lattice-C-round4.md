## For Ev

**Where I stand.** Both built bodies are mine, so I reran them and closed the caveats I had left open. On point 1 all three of us say "refuse"; I differ only in where it is decided. On point 2 I stand with A (rounding) against B (ε).

**1. The two bodies, rerun.**
- **The cut frustum** (small rim 20ε, upper half cut by the tilted plane, δ 1e-4, default ε). Same result as before, with the clearance at 1/16 and without it: 226 triangles with a lattice corner at least height ≤ ε (least 5.6e-10 m), 211 more on three boundary points, none unresolved at the rounding allowance. All three validation tiers pass and `check_mesh` passes.
- **The whole frustum at δ 1e-8.** 81,792 triangles; 6,610 have a cross product within 8 ulps × largest coordinate × longest side.
  - Last round I placed them in the small cap by their size. Now read by patch: all 6,610 are in the one patch whose extent is 4e-8 m (the cap, 18,680 triangles). The other three patches have none.
  - None is wound against its patch. Volume is π/3.
  - The window: 3,747 unresolved at δ 1.5e-8, none at 2e-8 or 3e-8. At 3e-9 the cone face already refuses `CertificateExceeded`. So the unresolved range on this body is about δ 3e-9 to 2e-8.
- **What I checked in the other replies.** B's suite table matches my own runs at all three ε (972 under 1e-6 m at the default and at 1e-12, none at 1e-6, least 3.1e-8 m). A's "up to 972 at the 1e-6 row" does not happen, as B says and as I withdrew last round. A's deeper point survives it: the same bytes would keep an ε promise at one ε and break it at another.

**2. Point 2: "least height exceeds ε" is not a promise the agreed design can keep.**
- On the cut frustum it fails for 226 lattice-cornered triangles of a certified mesh, at each of ε 1e-9, 1e-6 and 1e-12 (rim held at 20ε).
- B's unscoped wording ("no corner of any triangle lies within ε of its opposite side") also fails the whole frustum at an ordinary δ: at δ 5e-3, 14 of the cap's 26 triangles are under ε (least 5.0e-10 m).
- The cause is not a coincidence. A triangle's least height is at most its shortest side, and the rim's chords are `r·θ` long. To keep the promise, the chord pass would have to read ε, which your 2026-08-21 ruling forbids.
- **What the rounding promise tells a consumer:** every triangle's normal, computed from the positions as written, points outward and would not flip if each coordinate moved by its own evaluation rounding. It does not say the triangle is large against the model's tolerance. Whether a feature is above ε is a question about the body, and its validation answers it.

**3. Point 1: "refuse" stands, and it has to be decided where faces join the mesh.**
- *A chord point is the mesher's.* Only vertices are the body's. The 20,000 points on the cap's rim were placed by the chord pass. So no scope by "corners the body gave" is coherent: the promise covers every triangle.
- *It must be on every lane.* The instance is in a planar patch, which the trimmed lane's emission check never reads. A promise about every triangle, enforced in one lane, leaves `tessellate` returning `Ok` with 6,610 triangles the promise does not cover.
- *What it costs.* The whole frustum refuses for δ between about 3e-9 and 2e-8, where main returns a mesh that happens to be wound right. This sits beside the crate's existing rule: a face whose certificate f64 cannot resolve refuses typed (`RoundingUnresolved`), and this body refuses `CertificateExceeded` just below that range. Nothing in the suite's 40.6 million triangles refuses (nearest margin 1.7e7). The cut frustum does not refuse.
- *The root.* On this instance the root is the lattice's small end: a tensor grid gives the 2e-8 m rim the same column count as the 1 m rim. That is the subject of `a-needle-cone-face-meshes-thin-triangles-beside-its-rulings` and is not a small cure. It removes this instance when it lands. It does not change what is recommended here, except to confirm that the refusal cannot live in the trimmed lane alone.

**4. Recommendation on points 1 and 2, as final state.**
- **One definition** in `mesh::validate`: a triangle's winding is resolved when its cross product exceeds `cert::rounding_allowance` at its coordinate scale times its longest side. No ε.
- **In the mesher:** `tessellate` reads it once per triangle where each face's patch joins the mesh, on every lane and for a memo hit too. An unresolved triangle refuses its face typed, under its own variant. The trimmed lane's emission check uses the same definition to keep from reading a fold out of noise.
- **Outside the mesher:** a public check beside `check_volume` re-derives it over every triangle, with no polylines and no scope. The suites call it in place of `MIN_SHAPE`.
- **The promise** (the page and `FacePatch`): every triangle's winding is outward and resolved above its positions' rounding. Shape is not promised.
- Confidence: rounding over ε `sure`; refuse at the fold `likely`.

**Questions for you.**
- **With B, the scale.** Does "not flat" mean above ε or above rounding?
  - ε: the promise would speak about the surface. The cut frustum fails it at δ 1e-4 (226 triangles) and the whole frustum fails it at δ 5e-3 (14), both valid bodies. Nothing in the agreed design can prevent that.
  - Rounding: the promise speaks only about the mesh as written. Both bodies pass at those δ.
- **With A and B, the place, if they keep it in the trimmed lane.** Is the refusal decided for every lane where faces join the mesh, or only in the trimmed lane's emission check?
  - Every lane: the promise is true of every mesh `tessellate` returns. The whole frustum refuses between δ 3e-9 and 2e-8.
  - Trimmed lane only: that frustum still meshes, and the mesh carries 6,610 triangles the promise's sentence does not hold for, unless a caller runs the outside check.

## For the orchestrator

- **Runs this round** (binaries from round 3 in the shared `target/release`, sources under `fork-scratch-C/`):
  - `frustum-probe-C 20 <δ> 1` (rim multiple, δ, shift), now printing per patch. Run at δ 1e-8, 1.5e-8, 2e-8 and 3e-8 shifted, and at δ 5e-3 unshifted. The cross-product-to-allowance ratio is 0.64, 0.96, 1.27 and 1.91 across the four.
  - `rimsplit-probe-C 20 1e-4`, with and without `PROBE_CLEAR=0.0625`. Output is identical to round 3.
- **Still caveats.**
  - "Unresolved" is my spelling of the agreed allowance: 8 ulps × the largest coordinate among the corners × the longest side. If the implementer reads `cert::position_scale` differently, the window moves by a small factor.
  - The cap's ears are about 5 ulps high and all came out wound right, so on this body the refusal is conservative.
  - "Nothing in the suite refuses" is from the round-2 census of the integration suite. The crate's unit tests under `src/` were not run through it.
  - Not run: the fold verdict itself. Nothing here prototypes it.
- **The lower end of the window is not general.** It is where this body's cone face refuses `CertificateExceeded`; another body's window may reach lower.
- **For the implementer.**
  - The fold is the `Ok(placement)` arm of `tessellate`'s placement loop (`placement.place(&mut positions)`). A memo hit passes through it too.
  - Cost is one cross product per triangle.
  - The trimmed emission check needs no refusal of its own for an unresolved triangle if the fold has it. It only has to skip the sign read.
  - The existing `FoldedTriangle` refusal on an exactly zero product then moves to the new variant.
- **Row to file or annotate** (I may not commit): on `a-needle-cone-face-meshes-thin-triangles-beside-its-rulings`, record the frustum with a 20ε rim at δ 1e-8: 20,449 columns on a rim of radius 2e-8 m, and the neighbouring planar cap inherits them. I called this "the chord rule on a small rim" last round. It is the tensor lattice's column count at the small end, which the chord pass only follows.
- **Correction to my round 3:** "it is its own row" understated the cure. It is the existing needle-cone row's design question, not a chord-pass fix.