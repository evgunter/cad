## For Ev

**Recommendation now (likely): split at a point of each face's own surface, in every patch that has the side inside it, each half certified.** I have moved from the straight midpoint. Last round I said I would not offer the surface point as an equal option "until it is shown to restore the volume" of a body thinner than δ. I took that measurement this round, and it does.

### The measurement (default ε; the bored balls of `split_rulings`, 28 cells)

The body is a ball of radius 1.05 bored at radius 1, a shell 0.05 thick, split and unsplit, both signs, δ from 3 to 0.1. "Fails the gate" means `scripts/check_admesh.sh`'s counters on the written STL: admesh reversed every facet, because the mesh's volume is not positive.

| variant | signed volume where the two faces collide (body: 0.132 or 0.137) | fails the gate |
|---|---|---|
| today's code | −0.0715, −0.0390, +0.0282, +0.0715; the unsplit body refuses on 7 cells | 4 of the 21 meshes it returns |
| straight midpoint, all but one patch | 0.000000 split; 0.010 to 0.015 unsplit | 4 of 28 |
| surface point, every patch | 0.078 to 0.092 (58 to 67 % of the body's) | 0 of 28 |

- The two cells asked for, the split 1.05 ball: sign −1 at δ 1 reads −0.0715 today, 0.000000 with the straight midpoint, **+0.0911** with the surface point. Sign +1 at δ 0.3 reads +0.0282, −0.000000 and **+0.0798**.
- Every surface-split mesh passes `check_mesh` and `check_winding`. No half folded and none lay past δ; the worst half equals its parent, 0.905 δ.
- Where nothing collides (the 1.3 ball, finer δ) the two splits return the same mesh.
- Cost on such a cell: 86 triangles against 60.

Why it works: on a body thinner than δ both faces lay the same triangles, so the mesh is one sheet counted twice. The straight midpoint keeps that sheet exactly. A point of each face's own surface moves each sheet towards its own face, so the wall reopens with the right sign. Today's walk cuts one face only, which is why its volume flips sign with which face that is.

### The two disputed facts

1. **The admesh gate cannot see an edge of four triangles (sure).** B showed it on two tetrahedra; I showed it on the uncured bored ball, which reads clean with one part. The promise of a closed, manifold export is made by words: "watertight" in DESIGN's crate table, the script's header "watertight/manifold", the nightly step's name. Among the things this fork touches, the instrument checks gaps and orientation only.
   - So A's claim that the gate forbids two split points at one position is true of the words and false of the instrument. The straight midpoint in every patch is still out, on the words.
   - The gate does see what the straight midpoint leaves on a thin body: a mesh of zero volume that it flips, the 4 of 28 above.
   - The gate should say what it checks, or gain a count of edge uses by position. That is a tracker row, not a ruling.
2. **The surface split in every patch does restore what the straight midpoint cannot (sure, on these 28 cells).** It is not a guarantee. Nothing proves positive volume for every thin body, and I would not write it down as a promise.

### What moved me, and what did not

- **Moved.** I held that `[tolerance.chordal]` promises distance only, so both splits keep every promise and the simpler wins. That still describes the promise. But the two are no longer equal meshes that differ only in machinery: on a thin body one returns zero volume, which the tree's own export check flips, and the other returns most of the body. A measured the cost I feared across the whole suite and found none (1,416 splits, no fold, no half past δ).
- **Not moved.**
  - A straight midpoint in every patch is out, and so is a surface point in one patch only.
  - If the straight midpoint is chosen, the keeper by arena order is acceptable under `[determinism.equivariance]` as written.
  - The chord floor and the fold's own cure are needed either way.
  - No lane reads its neighbours.

### Is my last argument answered?

- "Distance only, so the straight midpoint meets everything promised": B agrees and it stands as a statement of the promise. What answers it is the table: meeting the promise is not the same as returning a usable mesh.
- "The surface option is unmeasured where it matters": answered, by A's suite count and by the table.
- "Its certificate can refuse": partly answered. A measured true distance, not each lane's own certificate, and a cone or torus certificate is more conservative. That is the one open risk.

### If we still differ

B holds the straight midpoint. The one sentence for you to rule on:

**Where a body is thinner than δ, should its mesh keep the body's sign and most of its volume, at the price of a certificate on each split and a split in every patch, or is a mesh of zero volume acceptable there because δ promises distance and nothing else?**

If the second, the straight midpoint in all but one patch is the simplest thing that keeps every promise, and I would build that.

## For the orchestrator

- **How measured.** A's `surf_probe.diff` applied to my worktree, plus my own row `zzc_probe_stl` (marker `CSTL`), which writes binary STLs and prints signed volume, `check_mesh`, `check_winding` and the body's pinned volume. Built with `--no-run`, copied to `C/scratch/all_C4`, confirmed with `--list` that it holds `zzc_probe_stl`, and run from the copy. Three runs: no env (today); `JOIN_PROBE=cure NO_LANE_RULE=1 CHORD_FLOOR=1`; `JOIN_PROBE=surf NO_LANE_RULE=1 CHORD_FLOOR=1`. Then system `admesh` on every STL, read with the gate's own counters.
- **Files**, under `/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad/fork3/C/scratch/`: `probe_round4.diff`, `r4_today.txt`, `r4_chord.txt`, `r4_surf.txt`, and the STLs in `r4_today/`, `r4_chord/`, `r4_surf/`. Worktree restored and clean.
- **Limits.**
  - A's split point is the surface foot of the 3-D midpoint, a stand-in for the midpoint in the face's chart. On cylinder, sphere and torus the two are close; on a cone's development they differ.
  - The halves were not given the lanes' own certificates.
  - Only these bodies, default ε. The collisions in my table are cylinder against sphere and cylinder against torus; plane against a curved sliver is in A's suite count, not here.
  - The unsplit rows need the chord floor, which was on.
- **A point for the build unit.** A foot on the face's own surface needs no chart coordinates at the join for analytic faces. The certificate does on torus and spline. A spline face has no closed-form foot, and A's lens row shows two spline faces do collide, so the spline split must take the chart midpoint from the lane's polygon.
- **Build directory.** My first build this round failed writing a fingerprint file: another chain was live and the directory was removed under it. The second build was clean. Afterwards, with no build running, I removed my `mesh` artifacts and fingerprints from the shared directory, so the next mesh build there recompiles. Note that `pgrep -f` in a wait loop matches the loop's own command line; mine never saw the other chain end.
- **Rows worth filing whatever is chosen.**
  - Today's code returns `Ok` with negative signed volume on four split bored-ball cells (sign −1 at δ 3, 1, 0.3, 0.1). They pass `geometry_enclosing` because `check_volume`'s bound, `δ · area`, exceeds the body's volume.
  - The admesh gate's header claims manifold, and it cannot see an edge of four triangles.
- **Where the others' round-three reports are wrong.**
  - A: "the tree does promise that an exported body, joined by position, … has no edge of four triangles", resting on the gate. The gate cannot see one; the promise is in the words only.
  - B: "(2) is measured only on ruled faces" is superseded by A's suite run. B's "a double layer breaks no promise" is right, and it leaves out that a whole-body double layer fails the one position-joined check the tree has, on 4 of 28 cells.
- **Format for Ev.** A recommendation with one named alternative and the single question above. A and I now hold the surface point; B holds the straight midpoint.
