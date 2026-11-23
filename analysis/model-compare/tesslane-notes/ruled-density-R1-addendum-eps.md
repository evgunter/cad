(Reviewer, PR 7096) ADDENDUM on 6e9fa7091b and the negative volume. Head 6e9fa7091b; main = the merge's other parent 0b8b41bf4b. Both built here in release.

**Verdict stands: APPROVE-WITH-FIXES. The negative volume is the regime's, not the walk's: main returns the same sign on the same cells. I would not hold the merge on it; it needs filing against main.**

**2. Negative volume (RAN, both trees).** Ball 1.05, bore 1, seam split at 0.5, placed obliquely, default ε. `signed_volume`, with the bore patch's triangle count:

| δ | sphere neighbour (sign −1): main | head | torus neighbour (sign +1): main | head |
|---|---|---|---|---|
| 3 | −0.09745 (46) | −0.07150 (48) | +0.09745 (46) | +0.07150 (48) |
| 1 | −0.09745 | −0.07150 | +0.09745 | +0.07150 |
| 0.3 | −0.09745 | −0.07150 | +0.03868 | +0.02820 |
| 0.1 | −0.00867 (42) | −0.03897 (66) | +0.05907 | +0.10761 |
| 0.05 | +0.09929 | +0.09929 | +0.08191 | +0.10332 |

- Every cell is manifold on both trees. At δ ≤ 0.05 both are positive and converge on 0.137.
- Main is negative on all four sphere-neighbour cells where the head is. Main is the larger at δ 3, 1, 0.3; the head is the larger at δ 0.1.
- Mechanism, from the per-patch volumes: the neighbour is one chord strip between the rims (Sphere: 16 triangles, +1.2074 on both trees), and the bore patch carries points ON the cylinder between the rims, so the bore sheet stands outside it (main −1.3048, head −1.2789). On main those points are the chart lane's own. On the head they are the walk's cut midpoints, so your suspect step is right for the head, but it is not what makes the sign: main inverts without it.
- The unsplit body refuses `OverusedSide` on those cells on both trees.
- This is a body thinner than δ whose two sheets are each within δ. The PR's bore row passes it because `check_volume`'s bound exceeds the body. That is a hole in the row's oracle, worth a line in the description and a filed row.

**1. The ε-free cut.**
- **(a) Held, by reading.** I grepped every line the unit adds to `crates/mesh/src` against 0b8b41bf4b for `eps`, `band`, `decide`. Three kinds of read remain:
  - the certificate verdict (`FaceVerdict::read`, `tol.eps`), which accepts or refuses and moves no coordinate, as on every lane of main;
  - the route's `walkable` → `at_a_pole` at the band. Undecided gives `Trim::Structural` and a typed refusal; decided on or off the apex gives the case the implementer names;
  - `read_domain`'s band, which is main's.
  `may_make` takes no ε or band now, and `ruled_walk.rs` reads none. On whether the named case is wider than on main: in kind, no (one classification, read once). In effect it now selects between two walked row layouts where main selects between two CDT polygons. I did not build such a body.
- **(b) Held, by reading.** The threshold is δ(1 + ALLOWANCE_SHARE) + 2·allowance at the scale of [a, b, foot]. The residual is real: the neighbour's scale can be larger. It is harmless, because `tessellate::joined`'s census runs on every mesh, so a side wrongly kept uncut refuses `OverusedSide` and does not return. Reaching it needs δ tuned to within a few ulps of the threshold AND the neighbour actually making that side; none of my bodies does.
- **(c) Held, RAN.** `split_rulings` and `route_rows` are 31/31 at ε 1e-9, 1e-6 and 1e-12. With an ε read put back (δ padded by 4ε at the call to `may_make`), `the_walks_cut_is_the_same_at_every_eps` is the one red row at ε 1e-6, and nothing is red at the default.
- **(d) Held as far as run:** the same 31 rows at three ε, which carry the per-cell cut counts and the K row. I did not re-take digests on this commit.

Nothing committed or posted; the tree is back at 6e9fa7091b and the worktree is removed.
