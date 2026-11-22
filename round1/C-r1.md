## For Ev

**Recommendation: decide each edge's wedge once, in tier 3, and have the embedding question read that answer. Don't try to make the question's per-cell reading equal to check 4's.** The question still reads, in each cell, how that cell's part of a face leaves the other face, levered at the faces' reach as PR 7329 has it. That reading is the question's own fact about the cell. It is not a second decision about the edge. Confidence: **likely**.

Terms used below:
- **Class**: tier 3's verdict for an edge (crease, smooth, or end, with a crease's turn).
- **Carrier arm**: the separator that is one face's own surface. It is the only separator whose sine is the wedge's.
- **Departure**: the sine at which a face leaves a separator, enclosed over a cell and levered at the reach.

### The premise, corrected

1. **Check 4 is not the right answer in the table either.** Take the cylinder of radius r = 1e-3 and a plane through its ruling at sine 1e-4. The plane sits r·cos θ from the axis, so it cuts the cylinder again 2r·sin θ ≈ 2e-7 m away, and between the two cuts the faces part by at most r·θ²/2 ≈ 5e-12 m. Move the plane 5e-12 m outward and it misses the cylinder: the edge itself is not there to within ε. Check 4 still reads Corner (1e-4 × its folded arm of 1e-3 = 1e-7, above K·ε), and the reach alone reads 1e-4. By your test (can geometry ε off change the answer?) both readings over-read here. The folded arm counts a face curving back at 1/κ when the faces stop parting at about θ/κ. So the choice between the reach and the fold is not the decision to make. Arithmetic **sure**; the diagnosis of the fold **likely**. It belongs to tier 3 and should get its own row.
2. **The question's per-cell reading is more than the wedge.** `sign_on` encloses the departure over the whole cell, not only at the edge. So it also catches a face that leaves the edge cleanly and then flattens back toward the other face, which no fact about the edge contains. That is what sends such a cell on to be split until the distance test reads it. Deleting it in favour of tier 3's fact would let that cell certify. **sure** (read from `sign_on`).
3. **The verdict holds no tier 3 facts today.** `AtRestBody` keeps the body, its sign certificate and a record, and nothing per edge. "Reads tier 3's facts from its verdict" is ratified (fork row 264, PR 6893) but not built, so there is nothing for the question to read yet. **likely** (read from the struct).

### The answers, as final states

**A. Tier 3 decides the class; the question owns the cell (recommended).**
- The verdict keeps each edge's class, as row 264 already says.
- The question never decides a class:
  - The carrier arm is asked only across an edge whose class is a crease.
  - The cusp reads the class `End` instead of deciding wedge-0/2π from a cell's ball.
  - Across a smooth edge only the other separators (a plane through the edge, the edge's graph) are read.
- In each near cell, the question decides only that this cell's parts leave the separator to opposite sides, by more than the band at the faces' reach.
- Where tier 3 has no fact (two faces sharing only a vertex, a pole or apex, a face's own turning), that cell reading is the only one, and nothing new is invented in tier 3 for it.

What this makes true:
- Rows 2 and 3 of the table can no longer disagree. At sine 5e-6 tier 3 refuses the body. At 5e-7 the edge is smooth, so no crease reading runs across it.
- The arms that still differ (the reach in the cell, the fold in tier 3) can only cost a refusal on the safe side.
- Folding curvature into the engine, and the cone-apex refusal it caused, stop being questions.
- "Never re-deciding one over a cell" becomes true. "One reading for both" holds as one class per edge.

What it leaves possible:
- Tier 3's class is read at stations, so a pinch between stations is classed as a crease. The cell reading covers the whole edge and still refuses it, so this stays safe, but the fact itself is sampled. That is tier 3's own defect (the RESTFRONT row).

It depends on the reach's site only through the cell reading's arm, which is the one recipe both readers share, so ONEROUND's fix lands once. It is easy to reverse.

**B. One function and one arm, read by both.**
- Check 4 and the question call one departure function at one arm: tier 3 over the edge, the question over every cell.
- Problems:
  - Picking the arm reopens the measured problem. The fold refuses cones that own their apex; the reach alone makes check 4 read row 3 as Corner.
  - The question still decides the class in every cell, so "never re-deciding" has to go.
  - A station reading and a cell enclosure of one function can still disagree, by enclosure width and by site.
- Not recommended.

**C. PR 7329 as it is, with the text saying there are two readings.**
- This contradicts "presumably unified".
- The question certifies a crease where tier 3 classed the edge smooth (row 3). That is harmless only because tier 3 is asked first, and no type enforces that order.
- Not recommended.

### Ratified text I would change

`[topology.tiers]`, the sentence starting "That reading is the edge's wedge as tier 3 reads it, one reading for both". It is the agent's wording of your PR 7095 ruling (commit `4e9ba70f43`), and your own words were "presumably unified with the other case". Proposed replacement:

> Where faces do meet, or a face is read against itself, the question reads from the verdict the wedge tier 3 decided at the shared edge (its class, a crease's turn, an end) and decides none. In each cell it reads how the cell's part of each face leaves what they share: an angle metered at the reach of the faces it governs ([tolerance.epsilon]), never at a cell of the question's own subdivision, passing by more than the band. Where tier 3 holds no fact (a vertex the faces share alone, a pole or apex, a face's own turning), that reading is the only one.

This narrows how your ruling was written down: what gets unified is the decision, one class per edge, not two quantities read over different regions. If you meant more than that, B is the reading that does it.

"Never re-deciding one over a cell" is agent text that you approved on PR 6893 ("seems good", "looks good!") and broadened. A keeps it and makes it true.

### The other two items

- **The five cell-levered margins:** same rule. A cell's size is never the lever on an angle: a margin is either an angle at the faces' reach or a separation in metres. That is the existing P1 row, not a fork. **likely**
- **Check 4's own meter** (premise 1): a separate tier 3 question, raised to you on its own. A does not depend on its answer.

## For the orchestrator

- I read the problem, both reviews, DESIGN `[topology.tiers]`, `[one-way.readings]` and `[tolerance.epsilon]`, fork rows 264 and 285, and on `origin/embed/remeter` `sign_on`, `departure`, `arm_of`, `levered_sign` and the sliver row. On main I read `edge_wedge.rs`, and in `dihedral.rs` `classify_dihedral`, `wedge_of`, `folded_lever_arm` and `interior_stations`. I read the `AtRestBody` and `KeptSign` structs. I ran nothing.
- **Error in the brief, item 1.** It frames check 4 as the faithful side and the reach as the unsound one. On the table's own geometry both fail Ev's test (premise 1). Review 2's "which is right: check 4" holds only at sine 5e-7. It does not hold at 1e-4, which check 4 reads Corner.
- **File: check 4's fold over-reads a face that curves back** (TOPO or RESTFRONT). Cite `geom_brep::folded_lever_arm` and `wedge_of` in `crates/geom-brep/src/dihedral.rs`, with the r = 1e-3, sine 1e-4 numbers. The candidate fix is the greatest parting before the face turns, about sin θ · min(reach, θ/κ) for the relative normal curvature toward the other face. That fix is **unsure** and needs its own fork.
- **Check whether this is filed:** DESIGN says "the verdict keeps tier 3's per-entity facts beside its certificate", but `AtRestBody` (`validate.rs`) and `KeptSign` (`props.rs`) keep none. Row 264 put building it "with C". A cannot land without it.
- **The cusp** (`SELF_INTERSECTION_CUSP`, "times the ball's reach") still decides wedge-0/2π from a cell, where row 264 ratified that it reads `End`. Confirm it is on the five-margins row. If not, add it there.
- **What A changes in PR 7329's code:**
  - Gate the carrier arm on the verdict's crease class.
  - Remove `sign_on`'s claim that its sine "is tier 3's wedge check's own".
  - Correct `wedge_reach.rs`'s module text ("caps the lever … in any case") and `folded_lever_arm`'s "route new callers" note: the question's cell reading is deliberately not a reader of the fold.
- **Not checked:** whether the carrier arm's definite sign on a crease always matches tier 3's convexity, which would be a cheap consistency check under A. Sign conventions: **unsure**.
- **Assumed:** stations stay interior to the edge; both reviews confirm `1..CERT_SAMPLES-1`.
- A does not bring back the exact-sign proxy Ev rejected on PR 7095. The cell reading keeps its band at the reach.
