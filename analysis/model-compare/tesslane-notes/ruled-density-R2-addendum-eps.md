ADDENDUM 3 (PR 7096, 6e9fa7091b). **My verdict stands: APPROVE-WITH-FIXES.** The negative volume is the regime's, not the walk's (§2). The fix it owes is a filing, not a hold on the merge.

**§2. Negative signed volume. Not the walk's: main gives the same sign on all eight cells.**
Probe: bored ball r 1.05, bore r 1, the seam split at its middle, unplaced and placed by my own oblique map. My own signed-volume sum and edge census. Release, default ε. Head 6e9fa7091b against origin/main at the last merge, 0b8b41bf4b.

| cell | head | main |
|---|---|---|
| sign −1 (sphere), δ 3 / 1 / 0.3 | −0.071501 | −0.097454 |
| sign −1, δ 0.1, unplaced | −0.038972 | −0.008671 |
| sign −1, δ 0.1, placed | +0.051755 | +0.082057 |
| sign +1 (torus), δ 3 / 1 | +0.071501 | +0.097454 |
| sign +1, δ 0.3 | +0.028201 | +0.038676 |
| sign +1, δ 0.1 | +0.107613 | +0.059071 |

- On main, the sphere cells at δ 3 to 0.1 refuse UNSPLIT (OverusedSide) and return `Ok` negative when SPLIT. Every cell is manifold by my census on both trees.
- Mechanism, the same on both: δ is larger than the shell (0.05). The sphere face's chords join the rims directly, inside radius 1, while the bore's added points sit on the cylinder at radius 1: the walk's midpoint cuts on head, main's 7198-inserted CDT points on main. So the bore sheet passes outside the sphere sheet.
- The head is less negative than main at δ 3 to 0.3, and more negative at δ 0.1 unplaced. Placement flips δ 0.1 to positive on both trees.
- I would still merge. It is owed as a main-level filing: a `check_volume` that passes an inverted body because δ·area exceeds the body's volume, and meshes of sheets thinner than δ.

**(a) No ε choice: HELD on my sweep, with one corner.**
- My probe, ~3,100 cells (cones, the bicone, the bored ball ±, a bored torus, a cone-bored ball, a bead, a ring; splits at 1, 2 and 5 points; placed): mesh bytes hashed per cell. The default ε against 1e-6 is 0 differ of 2,649 common cells. The default against 1e-12 is 0 differ of 2,949.
- The apex corner: the trigger set is the same (vertex, apex) decision main's `singular_vertices` makes. But on head it switches the LANE, walked against Structural, and with it the face's target (δ against δ/2), which re-chords the neighbours' rims too. So the same trigger has a wider consequence on head than on main. Pinned by `a_split_beside_the_apex_of_a_needle_cone_refuses_undecided`: it refuses at the default ε and meshes walked at 1e-12. NOTE.

**(b) The two-allowance test: sound, BY READING.**
- With computed distance d̃ > δ(1+A) + 2a and r ≤ a, the true distance exceeds δ(1+A) + a, past any passed triangle of a neighbour whose allowance is ≤ a.
- The residual (the neighbour's allowance read off its third corner and anchor) is stated. If it is ever hit, the edge has four uses and `overused_side` refuses it, so it fails safe.
- Reaching it needs a midpoint within a few ulps of the threshold and a neighbour triangle anchored farther out. I built no such body.

**(c) The ε row: HELD.** With the mutant "an ε read put back" (+10·ε on the threshold), `the_walks_cut_is_the_same_at_every_eps` is red at 1e-6 and passes at 1e-12, as its doc says. The bored-ball row passes at both.

**(d) No mesh moved: HELD on my probe.** a020a3edca against 6e9fa7091b at the default ε: 3,099 cells byte-identical. I did not re-run the PR's pinned rows on a020a3edca.

The tree is clean at 6e9fa7091b; nothing committed, pushed or posted.
