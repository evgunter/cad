## For Ev

**Recommendation (likely): a conic stores the point its mint was handed, with that point's place on the conic. The one limit is that a centre may be stored only on a conic no bigger than the session box (no semi-axis over 1 km).**
- **Small conics:** a circle authored by its centre keeps that centre. A section minted at its vertex keeps that vertex.
- **Large conics:** always stored at a point of the arc they serve, with the parameter zero there.
- **Readers:** every reader states the curve from the stored point, through the conic's one evaluator.

This is size as the limit, with the mint's own datum stored inside it. Size alone is also defensible, and one measurement decides between the two (end of this section).

Terms:
- *Stored point and pair*: the one position the conic holds, and where that point sits in the conic's unit coordinates: (0, 0) for the centre, (cos θ₀, sin θ₀) for a point of the curve. One evaluator serves both: `P(θ) = point + u·a·(cos θ − k_u) + v·b·(sin θ − k_v)`.
- *Box scalars*: `Interval` and the symbolic scalar.
- *Admission floor*: the rounding a run already allows for a short sum of box-sized numbers, about 1.8e-12 m.

**Three criteria were weighed for choosing the stored point**

*1. The centre's position: store the centre wherever it lies inside the box.*
- Makes true: every stored coordinate is in the box.
- Leaves possible:
  - **A rigid move can change a conic's form.** A part moved so that a circle's centre crosses the box face must be re-made at its arc inside the move. Re-making it changes its stored data, so certificates the moved body carries no longer apply.
  - "A finished body moved by an isometry stays finished" (DESIGN, at-rest validity) then holds only with slack added to the threshold, and with slack this is a size rule.
- Conflicts with:
  - DESIGN `[intrinsic.invariance]`: "a designed rule commutes with rigid motions".
  - "Float rounding": the session box is "one predicate over the region a piece of geometry occupies, never a carrier's stored terms". A large cylinder legitimately stores an axis point outside the box, so reading `[tolerance.units]` ("no coordinate larger") as a rule about stored terms is wrong. (sure: both texts checked in the tree)
- Under a reversal or a re-seam it behaves like the others.
- Not recommended.

*2. Size alone: centre if no semi-axis exceeds the box, otherwise a point of the arc.*
- Makes true:
  - One line, decided by the shape alone.
  - Moves, reversals and re-seams leave it unchanged: a quarter turn swaps the semi-axes but not the larger one.
  - A large conic stored at its centre cannot be built.
- Leaves possible: **a small section stored at a centre its mint never computed.** The cone section's mint holds its near vertex independently. Under this rule it must store `vertex − u·a` and read the vertex back as that plus `u·a`.
  - At the box scalars that is the semi-axis's width twice, at the very incidence the section exists to satisfy ("the vertex lies on the ruling").
  - It is the same effect as the concentric hole and boss in the worked case below, applied to a vertex.
  - The lane measured cone sections narrower at `Interval` stored at their vertex than stored at their centre. That measurement is not split by size.

*3. Size as the limit, the mint's datum within it (recommended).*
- Makes true: no door converts anything. Each door stores what it holds:
  - an authored centre;
  - a section's vertex;
  - a sketch arc's start point.
- **The one check:** a door handed a centre for a conic bigger than the box refuses loudly. A far centre has already lost its digits, and summing an anchor from it fixes that loss in place: sections minted that way miss their own surfaces by 1–3.6 u·a (measured).
- **Moves and reversals:** they map the stored point and flip the pair's sine. The form never changes.
- **A re-seam:**
  - On a centre-stored conic it is an exact permutation.
  - On a small conic stored on its arc it turns the pair and keeps the point. The parameter is then no longer zero at the stored point, but on a small conic that costs at most u times the box.
  - A large conic stored on its arc is not re-seamed: it is minted at the part it serves.
- Leaves possible: two small conics with the same curve stored by different points. At `f64` both are within the floor, and the pair says which point is stored. Carriers that differ by one ulp are already distinct today, so nothing new is lost.
- Agrees with:
  - "Float rounding": the box used as a scale for rounding, not as a predicate on stored terms;
  - `[intrinsic.invariance]`;
  - your PR 6329 allowance of several stored forms behind one interface.
- It differs from the two-forms design rejected on PR 7194 on the two counts that rejection rested on:
  - a large conic stored at its centre cannot be built;
  - nothing records how a conic was made, only which point it stores.

**Worked case: the box scalars** (sure)
- A hole and a boss share one centre, and each radius is a range of ±0.1 mm.
- Stored at their centres, they read the same `c`, and "concentric?" is decided.
- Stored on their arcs, each centre reads back as `(c + u·r) − u·r`, about ±0.2 mm wide, and the question cannot be decided.
- No reader change removes this. The same reasoning is why a section's vertex should be kept when the vertex is what the mint holds.

**Agreed by everyone who weighed this**
- **One evaluator and the pair**, with no record of how a conic was made.
- **Readers start from the stored point.** Covers, extents, chart images and the inverse go through the conic. Readers that need only directions read the stored axis and seam.
- **The unsound cover on the 1e7 ellipse is main's own.** `Cover::arc` rebuilds the arc from the centre. Main's evaluator rounds the same way, so the test passed there by bit agreement, and the new evaluator exposes it.
- **"Parameter zero at the anchor" stays**, for large conics stored on their arc.

**Contracts:**
- a cover holds its arc;
- a carried certificate equals a fresh one, when both are stated from one stored point;
- an exact centre for an authored circle at the box scalars;
- the STEP fixed point, which is import's to provide.

Last bits, mesh digests and query counts are habits.

**Proposed sentence** (agent text you approved on PR 7194, before these measurements):
> "A stored form that loses digits is re-minted in a well-conditioned form: a conic stores the point its mint holds and where that point sits on it; the point may be its centre only where no semi-axis is longer than the session box's extent, and is otherwise a point of the arc it serves, its parameter zero there. Every reading of where a conic is starts from the stored point."

**What decides between criteria 2 and 3:**
- Measure cone sections of semi-axis at most 1 km at `Interval`, stored at their vertex against stored at their centre.
- If there is no difference that matters, criterion 2's one-line rule wins on simplicity.
- If there is a difference, criterion 3 wins.

**What would change either answer:**
- **The pending measurement by size** (radius 10 m to 1e4 m). If centre-stored conics of radius 1 km or less give wrong definite answers at an admitted ε, the limit must drop below the box.
- Refusals that grow smoothly with size are not such a failure. They belong to a reader-side fix.

**Confidence:**
- recommendation: likely;
- criterion 1 dominated: sure;
- criterion 3 over criterion 2: unsure, one measurement away;
- a box-sized centre form staying within the floor: likely, not yet measured.

## For the orchestrator

**What moved me this round, and to what**
- **From size alone to size as the limit, with the mint's datum inside it.** The third report's argument is the box-scalar argument for authored centres, applied to sections' vertices. My own case for storing centres was exactly that a derived value folded into the stored point widens the box scalars, so size alone was inconsistent with it.
- It also removes a conversion step at the anchored door. Under size alone, the cone-section door must convert a small conic's vertex to a centre.
- **Moves are settled against the centre-position rule.** My round-0 rule (the centre's position) is now the second report's. Both of its citations cut against it, and I checked both in the tree:
  - "Float rounding": the box is "never a carrier's stored terms";
  - DESIGN `[intrinsic.invariance]`.

  My round-0 reading of `[tolerance.units]` as excluding far stored centres was wrong for the same reason.

**My round-1 arguments**
- Answered and kept:
  - size cannot change under a move;
  - refusing a far centre at the door;
  - the parameter clause scoped to on-arc conics;
  - the cover is main's own.
- Not answered by the position rule: that a move can change a conic's form. The second report concedes it ("must re-mint, or the threshold needs slack"), and slack turns it into a size rule.

**Where I still differ, and why**
- **The second report's "an on-arc conic is never re-seamed".** That holds for large conics. Under criterion 3, small on-arc conics may re-seam harmlessly: on a small conic, the step-A phase permutation costs at most u·box.
- **The 1e-12 rows.** ε = 1e-12 is admitted only at K ≥ 2.8 (`ADMISSION_FLOOR / (K − 1)`). Rows run at K = 2 are outside the rule and are not evidence of in-box loss. This agrees with the third report.
- **Classify the pending measurement** into wrong verdicts and typed refusals, and add phase 1's reader-side chord variant at each radius. A smooth fall in near-gap counts is expected and is not a failed threshold.
- **Symbolic cause.** I accept the third report's narrowing: the failing assertion reads `(q_from − circle.frame().origin()).norm()`, so no evaluator is involved. Forms freezing remains the likely cause. It is moot under criteria 2 and 3, since an authored rim stores its centre. Still unprobed.

**Owed by the lane (unchecked by me)**
- Whether every door that mints a large conic holds a point of its arc. Unchecked: revolve rims, blend arms, the shallow cylinder cut.
- Whether the symbolic scalar folds `0·x` and `x − 0`, which the (0, 0) pair needs. If not, `center()` should return the stored point structurally when the pair is (0, 0).
- The `Interval` comparison above: cone sections of semi-axis at most 1e3, vertex-stored against centre-stored.

**Housekeeping:** I read only the two named reports this round. Nothing was edited or run.
