## For Ev

**Recommendation (likely, now measured on the cells in dispute):** cut the shared side at a point of each face's own surface, in every patch that has the side inside it. No patch keeps the side because it comes first in storage order. A face whose own boundary carries the side never gives way. The chord-pass fix and the fold fix go with it.

This round I measured what the other two reports said nobody had: the bored-ball cells where δ is larger than the shell is thick (0.05), so the mesh can fold into a double layer.
- **Splitting at the straight midpoint with one patch keeping the side** leaves a mesh of zero volume on 4 of the 16 cells. Rounding makes 4 of those 8 cells negative, and the nightly admesh gate then reverses every facet and fails the file.
- **Splitting at a surface point in every patch** gives positive volume on all 16 cells and passes the gate on every file in the probe family (44 of 44).
- **Today's code** returns `Ok` at negative volume (−0.0715) on 4 cells, not 1, and the gate fails all 4.

### Measured: the 1.05 ball bored at radius 1, default ε, both signs, seam split and unsplit
Body volume ≈ 0.137. Columns are the signed volume of the mesh `tessellate` returns.

| cell | today | straight midpoint, one patch keeps | surface point, every patch |
|---|---|---|---|
| sign −1, split, δ 3 / 1 / 0.3 | **−0.0715**, gate fails | +6·10⁻¹⁶ | +0.091 |
| sign −1, split, δ 0.1 | **−0.039**, gate fails | +1.5·10⁻¹⁶ but gate fails (80 facets reversed) | +0.086 |
| sign +1, split, δ 3 / 1 | +0.0715 | +1.5·10⁻¹⁶ but gate fails (60 facets reversed) | +0.078 |
| sign +1, split, δ 0.3 | +0.028 | **−2.5·10⁻¹⁶**, gate fails | +0.080 |
| unsplit, δ 3 to 0.3 (both signs) | refuses `OverusedSide` | +0.010 to +0.015 | +0.083 to +0.092 |

Every cell in all three cures passes `check_mesh`. Gate failures over the probe family's files: today 4 of 37, straight midpoint 4 of 44, surface point 0 of 44.

**Why (sure).** At δ above the shell's thickness, the two faces' triangles lie on one set of chords. A straight cut leaves them there, so the volume is zero and its sign is a coin toss of rounding. A surface cut moves each face's new point onto its own sheet, so the two sheets part and enclose volume again.

### The two disputed facts
1. **The admesh gate cannot see an edge of four triangles (sure, from the other two reports' measurements; I accept them).** My round-three argument rested on the gate promising that edge away, and that argument falls.
   - The promise to a consumer without vertex ids is made only in words: the STL crate's "watertight STL" and "a leaky STL", and the gate's own header "watertight/manifold".
   - Nothing checks it, so the gate should get the check it names (a row for the orchestrator).
   - The gate does see one thing at issue here: an inverted volume, reported as reversed facets. That decides between the two pairings.
2. **The surface split does restore what the straight split cannot (sure on these 16 cells).**

### Ratified text
- **`[tolerance.chordal]`'s "DISTANCE-ONLY … and nothing else" does not cap what a mesh promises.** It decides that δ is a distance bound and not an angle bound ("There is no angular-deflection criterion"). Winding is promised beside it in `[mesh.winding]`.
- **Positive volume is already written down in the tree, though not as a ratified clause.**
  - `signed_volume`'s doc calls a positive volume for an outward-wound closed mesh "the global orientation invariant".
  - The nightly gate fails any reversed facet.
  - So a returned `Ok` with zero or negative volume breaks something the tree states, whatever cure is chosen.
- If you want this stated outright, it is one sentence beside `[mesh.winding]`: a mesh `tessellate` returns encloses positive volume.

### The one sentence to rule on (designers B and C chose the other pairing)
**Must a mesh that `tessellate` returns as `Ok` enclose positive signed volume (pass the gate's reversed-facet check) at every δ, including a δ larger than the body is thick?**
- **Yes:** the surface point in every patch. It is the only cure measured to do it.
- **No:** the straight midpoint with one patch keeping the side is acceptable. The zero-volume double layer then becomes documented behaviour, and the four gate failures above are accepted.

## For the orchestrator

- **How I measured (build-hazard safe).**
  - I applied my round-three probe (`A/scratch/surf_probe.diff`, which contains C's `probe.diff`), and extended `split_rulings::zz_probe_join` to:
    - print the signed volume behind the marker `AMARK`;
    - write an f32 ASCII STL for each cell when `A_STL_DIR` is set.
  - I built with `--no-run`, copied the binary into `A/scratch`, and ran it from there with the filter `zz_probe_join` in three modes:
    - today: no switches;
    - straight midpoint, one patch keeps the side: `NO_LANE_RULE CHORD_FLOOR FOLD_INSERT JOIN_PROBE=cure`, C's cure, where the first patch in arena order keeps it;
    - surface point: the same switches with `JOIN_PROBE=surf`, splitting every interior patch at `Surface::foot` of the midpoint.
  - The markers were present: 37, 44 and 44 `AMARK` lines.
  - I ran the system `admesh` on every STL.
  - Then I reverted the worktree (clean) and deleted my copied binary, the shared directory's `deps/all-62d10b8e21efa59f` and its `.fingerprint/mesh-62d10b8e21efa59f`, so nobody runs mine as theirs. The next mesh test build recompiles.
- **Outputs**, under `/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad/fork3/A/scratch/`:
  - `vol_base.txt`, `vol_cure.txt`, `vol_surf.txt`;
  - `stl_base/`, `stl_cure/`, `stl_surf/`;
  - the diff, `surf_probe_r4.diff`.
- **Limits.**
  - The surface point here is the foot of the straight midpoint, not the midpoint in the face's chart.
  - One body family (the 1.05 and 1.3 balls, both signs), default ε only.
  - The 1,416-split suite run (round three) remains the evidence on the other faces.
  - So the surface option's case is now two measurements, not one: the halves stay within δ and do not fold across the suite, and the volume holds on the double-layer cells.
- **Corrections to the other round-three reports.**
  - C counts one cell where today's code is negative. I measure four (sign −1, split, δ 3, 1, 0.3 and 0.1), all with every facet reversed by admesh.
  - C reads "DISTANCE-ONLY … nothing else" as ruling out a volume promise. In context it rules out an angular criterion.
  - C sets the condition "until it is shown to restore the volume". That is now shown on the cells C named.
  - B's statement that a double layer "breaks no promise": it breaks `signed_volume`'s stated invariant wherever rounding makes its volume negative, and it fails the gate there. Under B's own pairing that is 4 of 16 bored-ball cells.
- **Rows to file whatever is chosen.**
  1. The admesh gate is blind to an edge of four triangles. Its header claims "manifold". Add a by-position count of edge uses after merging vertices, or a Rust check that welds by position.
  2. Today's code returns `Ok` at negative signed volume on four bored-ball cells, and `check_volume`'s δ·area bound lets them through. P1 by the tree's own stated invariant.
- **What moved me this round.**
  - The gate's blindness, accepted. It removes my round-three reason.
  - C's measurement of the zero-volume double layer, which led me to measure the cells.
- **What did not move me.** The pairing: a surface point in every patch, with no tie-break. It now has a measured reason, the positive volume, in place of the one that fell.
- **Before this goes to Ev:** nothing more is needed to state the fork.
- **Can wait for the build unit:**
  - the chart-midpoint split point and the lanes' own certificates on the halves;
  - the NURBS lens;
  - the ε rows and the slow set;
  - what `PickMemo` does with a patch cut at the join.
