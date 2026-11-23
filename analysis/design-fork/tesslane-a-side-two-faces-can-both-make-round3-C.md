## For Ev

**Recommendation (likely), as one recommendation: split at the side's straight 3-D midpoint, at the join, in every patch that has the side inside it except one.** I have moved back from the surface point. The reason is what the tree promises, below. One fact about that promise is yours to rule on, and it would overturn this if you rule the other way.

### What a mesh promises at such a side (read, then measured)

- **`[tolerance.chordal]`** is the governing text: "The tessellation criterion is DISTANCE-ONLY … δ bounds how far any point of the mesh lies from the true surface, and nothing else."
- **`check_mesh`** reads ids and counts only. Its doc says coincident points under distinct indices pass.
- **The STL writer** drops ids and does "no snapping, no deduplication". It promises the winding's normal and nothing about closure.
- **The one check that joins by position** is `scripts/check_admesh.sh`, run nightly on the acceptance STLs only. It accepts no repair: no disconnected or reversed facet, one part. Touching assemblies are kept out of it on purpose, and that row (`stl/tests/m3_pr6_exports.rs`) accepts "one POSITION" held by two vertices.
- **`check_volume`** allows `δ · area`. `[mesh.winding]` is per triangle against its own face. The viewer draws unindexed triangles. I found nothing else that joins vertices by position.

Measured, with admesh on STLs written from the probe (the 1.05 bored ball, seam split, lane rules off):

| mesh | admesh |
|---|---|
| edge of four triangles left uncured | clean: admesh pairs facets greedily and cannot see it |
| chord midpoint, all but one | clean |
| chord midpoint, every patch (two private vertices at one position) | clean |

- **Two private vertices at one position break no promise the tree makes.** (sure)
- **A double layer breaks none that `tessellate` makes** (δ, winding, closed by ids), and passes `check_volume`. But where the whole body is thinner than δ it is the entire mesh. Its signed volume is zero to rounding (6·10⁻¹⁶ and −2.5·10⁻¹⁶ on two cells; the body's is 0.137). On the cell where rounding lands negative, admesh reverses every facet, which the gate refuses. That is so with no cure and under both chord variants alike. It belongs to the regime, not to the split. (sure)
- **Today's mesh is worse on such a cell.** The walk's one-sided surface cut, sign −1 at δ 1, returns `Ok` with signed volume −0.0715, and admesh reverses all 64 facets. (sure)

So under the promise as written, the chord midpoint meets everything promised. What the surface point buys (the sheets parted, a body thinner than δ keeping positive volume) is something `[tolerance.chordal]` says δ does not promise.

**The ruling that would overturn this:** if a mesh should also enclose positive volume, or pass the export gate at any δ, then no chord variant does it and neither does today's code. The cure becomes a surface point in every patch on both sides, and `[tolerance.chordal]` gains a sentence. I would not offer that as an equal option until it is shown to restore the volume. Nobody has measured that.

### Who gives way: all but one (likely)

- **A chord midpoint forces it.** If both patches split at the chord midpoint, both new points are one position. A consumer with no ids then finds the two half-sides used four times each: the defect, moved. The in-tree gate would not notice, but the reason to cure the side at all is consumers stricter than that gate.
- **The keeper** is the first patch in arena order. Where a face's boundary chord carries the side, every patch with it inside splits.
- **The arena-order tie-break is acceptable under `[determinism.equivariance]` as written.** The clause asks rules to commute with rigid motions and reflections, and arena order does not change under either. Its preference for intrinsic quantities over construction order yields where equivariance "costs something real", and here the symmetric rule re-creates the defect. The residual is small and should be documented: the two meshes are the same set of points, and only which face holds the extra vertex differs.

### What moved me, and what did not

- **Moved:** "distance-only … and nothing else" is ratified, so my round-two argument from better geometry was for a promise the tree does not make. The equivariance clause does not forbid arena order as I had read it. admesh passes every variant, so my worry about position joins is not a promise of the tree.
- **Not moved:** a chord midpoint and "every patch" do not go together (A's round-two pairing). A one-sided surface cut is wrong, now measured. The chord floor and the fold's own cure are still needed.

## For the orchestrator

- **Before this goes to Ev:** nothing more for the recommendation; it is measured across the default suite. If the surface option is to be put beside it, two numbers are needed, since they are that option's whole case: the certificate and fold refusals of the halves over the suite (A was running a `JOIN_PROBE=surf` suite when I looked), and the signed volume and admesh result of the both-sided surface split on the whole-body double-layer cells (split 1.05 ball, sign −1 δ 1 and sign +1 δ 0.3).
- **Can wait for the build unit:** ε 10⁻⁶ and 10⁻¹², the slow set, other crates, what the pick memo does with a cut face, how often a child fails the winding read, and the chord-carried branch (never fired).
- **Shared build directory hazard, hit this round.** The worktrees produce the same artifact hashes, and cargo calls another worktree's newer binary "fresh". Three of my runs executed another designer's test binary; they matched 0 tests, so no wrong data came of it. I then built with `--no-run`, copied my binary to `C/scratch/all_C`, ran it from there, and removed my `mesh` artifacts and fingerprints from the shared directory so nobody runs mine as theirs. The next mesh build there recompiles, about 3 minutes. Any designer's run taken after another worktree's build should be checked for its own markers.
- **Files** (under `/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad/fork3/C/scratch/`): STLs in `stl_base`, `stl_pass`, `stl_cure`, `stl_cure_all`. `probe.diff` is the round-one probe; this round added a `pass` mode, a `cure_all` mode and an STL writer test, not saved as a diff. Worktree restored and clean.
- **Counts behind the table:** today's split ball, sign −1 δ 1: 64 triangles, volume −0.0715. Chord all-but-one: 60 triangles. Chord every patch: 86. Uncured: 34, which `check_mesh` refuses. Unsplit with the chord floor: volume +0.0151 under every variant.
- **Defect to file whatever is chosen:** a body thinner than δ everywhere returns `Ok` with a mesh of zero or negative signed volume. `split_rulings::a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge` passes such cells because `check_volume`'s bound, `δ · area`, exceeds the body's volume there. It may be accepted under distance-only, but nothing says so, and `signed_volume`'s doc calls positive volume "the global orientation invariant".
- **Wording that moves under the recommendation:** `signed_volume`'s aside "every position lies on the body's surface" and `Mesh::positions`' mint order both gain the join's split points.
- **Wrong in round two:** A pairs the chord midpoint with "every patch" and calls two ids at one position harmless; it is harmless to `check_mesh` and admesh, and it re-creates the four-use edge for any stricter consumer without ids. B's keeper rule matches mine. Both say the surface point "can fold or fail its certificate" as if shown; it is unmeasured either way.
