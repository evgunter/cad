## For Ev

**Decision (likely): pairing (2).** When two patches both make the same side, split it in every patch that has it inside, each at a point on that face's own surface, and certify the new triangles. A face whose boundary carries the side never splits. No tie-break is needed. The chord-pass floor and the fold fix go with it. I have moved off the straight midpoint, because I measured the cells the dispute was about.

**What I measured:** the bored ball (a 1.05 ball with a 1.0 bore), at δ 3, 1 and 0.3, with and without the bore split, for both outer surfaces (torus and sphere). That is 12 cells, every one with δ larger than the 0.05 wall. For each cell I took the signed volume and ran admesh on an STL of it. The same body meshed at δ 0.05 or finer encloses 0.100 to 0.117.

| what | signed volume | admesh "Facets reversed" |
|---|---|---|
| today's code, split, sphere neighbour | **−0.0715** | 64 of 64: fails the gate |
| today's code, split, torus neighbour | +0.0715 at δ 3 and 1, +0.0282 at δ 0.3 | 0 |
| straight midpoint, all but one, split | ±10⁻¹⁶, i.e. zero | torus neighbour: 60 or 94 of all facets, **fails**; sphere neighbour: 0, by rounding |
| straight midpoint, all but one, unsplit | +0.015 at δ 3 and 1 (torus +0.010 at δ 0.3) | 0 |
| **surface point, every patch** (12 cells) | **+0.078 to +0.092** | **0 on all 12** |

The surface split on those 12 cells:
- 20 to 42 splits per cell;
- no folded half, and no half farther than 0.42 δ from its surface (true distance);
- `check_mesh` and the geometry row pass on every cell.

**What this means (sure on these cells):**
- On a body thinner than δ, any straight-midpoint split leaves two sheets on top of each other. They enclose nothing, and the orientation comes down to rounding.
- The surface split in every patch separates the sheets, each toward its own surface. The mesh then encloses most of the body, and the right way out.
- `signed_volume`'s doc calls a positive volume for an outward-wound closed mesh "the global orientation invariant". The export gate refuses reversed facets with no repair. The straight midpoint breaks both on these cells and the surface split keeps both.
- So my round-three claim that "no promise decides it" was wrong for this regime. A double layer breaks no promise of `tessellate` itself. It does break the orientation that the volume check and the export gate read.
- A second point for (2): `signed_volume`'s doc says every position the crate mints lies on the body's surface. A straight-midpoint point does not.

**The gate (sure):** it cannot see an edge of four triangles.
- Two tetrahedra sharing one edge read clean, except for "2 parts". Designer C's uncured bored ball reads clean too.
- So "no edge of four triangles, joined by position" is written into the words: the STL tests' "watertight STL" and the script's header "watertight/manifold". The check does not enforce it.
- What the gate does enforce, strictly, is "no reversed facets". That is the test the straight midpoint fails on the torus-neighbour split cells.
- The gate needs its own by-position count of edge uses. That is separate from this fork.

**What this costs:**
- Each half needs its lane's certificate, and each patch must carry where its corners sit in its chart.
- A half that fails its certificate refuses with a typed error; across the suite it has not happened (designer A's 1,416 splits, and these 12 cells).
- Two extra triangles per conflict, compared with all but one.

**If another designer still holds (1), the one sentence to rule on:** *"Must the mesh of a body thinner than δ still enclose positive volume, outward, so that it passes the export gate's orientation check?"* Yes means (2). No means (1), and `[tolerance.chordal]` should then say that a mesh of a body thinner than δ may enclose nothing.

## For the orchestrator

**How I measured:**
- I applied designer A's `fork3/A/scratch/surf_probe.diff` to my worktree.
- I extended its `zz_probe_join` row to print `signed_volume` behind a `BPROBE` marker, and to write an f32 ASCII STL for each cell (zero normals; admesh recomputes them).
- I built with `--no-run`. The first copy was stale: another worktree's binary, without my marker. I touched the files and rebuilt, then confirmed the marker with `grep -a`.
- I copied the binary to `B/scratch/all_B` and ran `zz_probe_join` three times: plain; `NO_LANE_RULE CHORD_FLOOR FOLD_INSERT JOIN_PROBE=surf`; and the same with `JOIN_PROBE=cure` (straight midpoint, all but one, first in arena order kept).
- I ran admesh on every 1.05-ball STL.
- Outputs are in `/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad/fork3/B/scratch/`: `base.txt`, `surf.txt`, `cure.txt`, and `stl/`. The two-tetrahedra STLs from round three are in `adm/`.
- My worktree is restored and clean. I deleted my test binary from the shared directory so nobody runs it as theirs; the fingerprints are left, so the next build relinks.

**Limits:**
- The split point is the foot of the 3-D midpoint (`Surface::foot`), standing in for the chart midpoint.
- The deviation is true distance on a 13×13 sample, not each lane's certificate.
- The winding check was not run on the halves.
- Default ε only, one body family.
- A's spline lens has no foot to fall back on, so it falls back to coincident straight midpoints. The chart-midpoint design avoids that, but that is unverified, and it is the one place (2) could degrade.
- All of these belong to the build unit.

**Answers to the two disputed facts:**
- The gate does not make the four-uses promise. It enforces orientation and connectivity only.
- The surface split does restore the thin-body volume and the gate's verdict on the cells C named, and on ten more.

**My last argument, point by point:**
- Equivariance allows an arena-order keeper: true, but now moot.
- "Nothing in the tree asks for separated sheets": answered by my own measurement, because the orientation invariant and the gate's reversed-facet check do, in this regime.

**Rows to file whatever is chosen:**
- The admesh gate is blind to four-use edges. Add a by-position edge census to `check_admesh.sh`, or to an STL test.
- Today's code returns `Ok` with negative volume: the split bored ball, sphere neighbour, δ 3, 1 and 0.3.
- `split_rulings::a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge` passes that negative-volume case, because `check_volume`'s bound of δ·area exceeds the body's volume (designer C's row).
