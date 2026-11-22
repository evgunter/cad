## For Ev

**Recommendation.** Tier 3 decides each edge's wedge once, over the whole edge, and the verdict keeps it. The embedding question never classes an edge and never meters an angle of its own. Near a shared edge or vertex, everything the question decides in a cell is one length: how far apart the cell's two face-parts are guaranteed to be, measured off a separating surface at the lever that entity's fact was decided at, compared with K·ε. Confidence: **likely**.

What that means in practice:
- **Edges.** The verdict keeps each edge's class (crease of a turn, smooth seam, end) and the lever it was decided at. The question reads both.
- **A pole or apex.** The question reads the face's singular vertex from the verdict, where tier 3's chart-singularity check places it. That fact was approved on PR 6893 and is not built yet.
- **Where tier 3 has no fact** (a vertex the faces share alone, a face's own turning), the question keeps a reading of its own. It is levered at the faces' reach from that entity and computed once per entity, never per cell. I would add no new tier 3 check for these until a second reader needs one.
- **A cusp** reads tier 3's `End` instead of deciding opposed normals from a cell.

Terms:
- **Separator:** a surface through the shared edge, which the question shows the two faces lie on opposite sides of.
- **Lever:** the length an angle is multiplied by to become a distance.
- **Reach:** how far the faces run from the edge.
- **Fold:** check 4's lever, the least of the reach and the faces' curvature radii.

### Why the cell's number is a length, and why it is banded

The question proves, in each cell, that the separator takes one strict sign on one face's part and the other sign on the other's. It needs a band only because the stored geometry may be ε off the real geometry. If the separator's gradient is normalised, two points on opposite sides of it are at least the sum of their distances from it apart. So "this cell's least rate times the lever, summed over the two faces" is a lower bound on how far apart the faces have got. That is a length, a maximum-deviation quantity of exactly the kind you asked about on PR 7095. Summing the two faces makes the number the same whichever separator the question chose. Today the plane that bisects a crease reads each face at sin(θ/2), half of what the carrier arm reads (the carrier arm uses one face's own surface as the separator). **likely**

A sign alone is not enough. Take a face that leaves the other at a clean crease, bends, and then runs alongside it about ε away, still drifting off at 1e-12. Everywhere it is strictly on its own side, so a bare sign certifies the large first cell that holds both faces. But geometry ε off could cross there. A banded reading of the cell's least rate refuses that cell, so it is split until the distance test reads the run. No fact about the edge holds this, because the edge itself is a fine crease. **likely**; argued by hand, not built as a body.

The lever comes from the verdict, not the reach. With the lever tier 3 decided the edge at, the reviewers' table cannot split:
- At sine 5e-7, 5e-7 × 1e-3 is below K·ε, so the question cannot read a crease where tier 3 read smooth.
- At 5e-6, tier 3 refuses the body before the question is asked.

This needs no gate per separator. A smooth seam can still be proved through a separator that the faces really do leave squarely. On a capsule, the cylinder and the dome leave their circle's plane at a right angle: rate 1, times the cylinder's radius, passes. The apex stops being the engine's problem: the lever is the entity's, and the last stretch before an apex is governed by the apex vertex. How tier 3's whole-edge reading treats that stretch is **unsure**.

### Check 4's fold does not pass your test on a face that curves back

Take the reviewers' cylinder, radius r = 1e-3, and a plane through one of its rulings at sine 1e-4.
- The plane sits r·cos θ from the axis, so it cuts the cylinder again 2r·sin θ ≈ 2e-7 m away.
- Between the two cuts the faces part by at most r·θ²/2 ≈ 5e-12 m.
- Move the plane 5e-12 m outward and it misses the cylinder: to within ε there is no edge.
- Check 4 still reads Corner (1e-4 × 1e-3 = 1e-7, above K·ε).

The first-order angle governs the gap only out to about θ/κ (here 1e-7), not out to the radius 1/κ. So the fold over-reads by about 2/θ. At sine 5e-7, as the third column of the table has it, check 4 happens to be right; at 1e-4 it is not. Arithmetic **sure**; the remedy (a lever of about min(reach, sin θ/κ), with κ the curvature toward the other face) **unsure**.

This is tier 3's own question and should be its own fork. Under this recommendation the question inherits whatever tier 3 settles there, so it is fixed in one place. The reach's site (ONEROUND) lands the same way.

### Final states compared

- **Recommended: per-entity lever from the verdict, a length per cell.**
  - One class and one lever per edge.
  - The question cannot accept where tier 3 refused, and can refuse only on the safe side (a cell's least rate is at most the edge's).
  - The stall in the bend example above is caught.
  - "Never re-deciding one over a cell" and "one reading for both" are both true.
  - Easy to reverse: the verdict's fields are additive.
- **A sign only at creases, the band once in tier 3.**
  - Simpler, and literally one band.
  - It certifies the bend example's stall.
  - Its by-hand argument leans on the rate staying near the wedge's past the edge, which a bending face breaks.
- **One function at two sites** (PR 7329 with the fold in the engine).
  - Re-decides the class in every cell.
  - Needs an ε concession at every apex.
- **PR 7329 as built** (reach alone in cells).
  - Two levers for one angle; it is safe today only because tier 3 runs first.
  - A fair step toward the recommended state.

### Ratified text to change

`[topology.tiers]`, the sentence "That reading is the edge's wedge as tier 3 reads it, one reading for both: an angle metered at the reach of the faces it governs…". It is agent wording of your "presumably unified with the other case". Proposed:

> Where faces do meet, or a face is read against itself, the question reads from the verdict what tier 3 decided at the shared edge or singular vertex (its class and the lever it was decided at) and decides neither. Over each cell it reads one length: the separation the two faces are guaranteed at that lever, which passes by more than the band. Where tier 3 holds no fact (a vertex shared alone, a face's own turning), the lever is the faces' reach from that entity. It is never a cell of the question's own subdivision.

Tier 3's page: the wedge is read over the whole edge, not at stations.

"Never re-deciding one over a cell" is agent text you approved on PR 6893. It stays, and becomes true.

## For the orchestrator

**(a) What moved me.**
- From A: the cell's number is a length at the entity's lever read from the verdict, not the reach. That makes my round-1 class gate on the carrier arm unnecessary; the verdict lever does its job.
- From A: the pole/apex fact already exists (check 6). Adopted.
- From B: the separator-dependence (the bisector reads sin(θ/2)). That led to deciding the pair's summed separation.
- From B: the capsule. A seam can be proved by a non-carrier separator, so A's "a seam goes to the second-order arm" is too strong.
- From both: check 4 over the whole edge. Adopted.

**(b) Point by point.**
1. *Cell reading.* B's sign-only is not answered for the stall case, and B's own "weak point" admits the decaying rate. A and I agree on a band, and I take A's lever.
2. *New tier 3 facts.* A adds a vertex fact and a face-turning fact. Neither has a second reader that I know of, and the question's own reading levered at an extent is not a second reading. I hold. The cheap check is whether `link.rs`'s vertex links would read a vertex fact.
3. *The fold.* A's 5e-7 analysis is right at that row. A's own premise ("the scale over which the first-order angle governs the gap") gives θ/κ, not 1/κ, so at 1e-4 it supports my point. Neither report addresses the 1e-4 case.

**(d) Deciding cases.**
- Difference 1: a unit probe through `sign_on` on the stall configuration, i.e. a crease, a bend of radius ρ, then a run at about ε drifting off at 1e-12. Does a sign alone certify the first near cell, and does the banded length refuse it? If the presentation splits it anyway, B's answer costs nothing.
- Difference 3: no measurement needed; the arithmetic decides. What is needed is Ev's ruling on whether an edge that ε can remove fails tier 3's wedge promise. Put it to Ev as a separate tier 3 fork, then file it (TOPO or RESTFRONT) citing `folded_lever_arm` and `wedge_of`.
- Difference 2: a reader search, not a measurement.

**Caveats and claims to check.**
- The summed-separation bound assumes ψ has a unit gradient, which holds for a plane and is approximately true for a carrier near the edge. The graph separator needs checking.
- Not checked: whether whole-edge check 4 can keep a single lever per edge when the curvature varies along it. It might need a lever per stretch, which the verdict would then keep per stretch.
- I ran nothing. Both other reports' tree claims agree with what I read in round 1: `AtRestBody` is `{ body, sign, held }`, and `embedded` takes a `Side`.
