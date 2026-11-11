# Fork `clear-enclosure` — designer A, round 1

Harness token count (first report): 213,136.

## For Ev

**Recommendation (likely).** Putting an f64 into the clearance engine as a one-point interval `[x, x]` is correct, and the engine should keep `Interval` as its arithmetic. Your unease does point at real faults, but they are elsewhere: the engine re-decides facts the body has already certified, and its refusals don't say which part of a too-wide bound stopped them. Three changes:

1. **One crossing, owned by the lane.** Every stored value enters through `Interval::from_certified`, which is documented as "the set of reals this value stands for". The one function that crosses lives on `ClearanceLane`, the value that holds certification rights, and no caller spells it again. This deletes the shell's `lift` parameter.
2. **No second reading.** At each place where the engine decides that something is exactly Zero, it takes tier 3's verdict instead of re-measuring that fact on every cell. This is also how `Sym` reaches the engine.
3. **Each refusal names which width blocked it:**
   - the family's width: only the drive can fix that, by narrowing the parameter box;
   - the cell's width: refinement or budget.

   Today only the upper-bound question says this (`BoxDependent`). The other questions should too.

**Premise check.**
- **"Interval is the scalar and also the range bound" is not the fault (likely).** Both are the same kind of object: a set of reals that contains the values of a function over a box.
  - The scalar's box is the document-parameter box.
  - The engine's box is that box times a cell of the face's parameter domain.
  - The certified-bracket contract on `CertifiedEnclosure` (the trait that marks a scalar allowed to certify) says the bracket holds "every real the value stands for". So the crossing already produces the set each value stands for, at every scalar.
- **What an f64 value stands for (sure).** It stands for itself.
  - `[simple.conditioned]` (yours, PR 6329): at f64, "values are treated as real numbers; nothing charges, pads or estimates float rounding".
  - Your words on PR 6308: the f64 certificate is that the stored body is "internally consistent".
  - So `[x, x]` is exact in meaning, not only as a conversion. Widening each f64 by ε on the way in would charge rounding, and it would turn the f64 question into a question about a family.
- **The K·ε band does not come in with the values (sure).** It is the decision band of the one decision funnel, the same at every scalar. `Interval`'s decision rule reads Positive only when the lower end is at least K·ε, and Zero only inside ±ε. So "pieces more than K·ε apart" at f64 is what the engine already decides at Interval. An f64 build changes nothing about how values enter.
- **Why not a new type for the bound (likely).**
  - The engine evaluates carriers through `geom`'s evaluators, which are generic over `Real`. Any range type of its own would have to be a full `Real`: a second interval scalar with identical arithmetic, under a second name for the same mathematical object.
  - What differs between the two roles is the box, which belongs to the computation, not to the value.

**The faults, stated exactly.**
- **(a) Second readings (likely).** Four places in the engine decide Zero on facts that the body's own readings have already decided:
  - "a shared edge lies on the other face's carrier" (`self_intersection_on_shared`). Tier 3 certifies this through its residual check.
  - "this vertex is the cone's apex" (`self_intersection_apex`). Tier 3 already makes chart singularities into vertices.
  - "the wedge at this edge is 0 or 2π" (`self_intersection_cusp`). This is tier 3's material-wedge reading.
  - a meeting's separation, which `Enclosure::separation` already reads off topology.

  `[one-way.readings]` (yours, 2026-10-14) forbids this: "two readings of one fact are two predicates that can decide it differently inside the band". It can bite at any scalar. It bites hardest at `Sym`: the engine drops the symbolic part of each value on entry, so it re-asks numerically an identity that tier 3 proved symbolically.
- **(b) One width, two levers (likely).** A cell bound's width has two sources, and they are fixed in different ways:
  - the family's spread, which only narrowing the parameter box removes;
  - the cell's extent, which subdividing removes.

  A single `Interval` cannot tell them apart. The code keeps them apart by hand in three places:
  - the `BoxDependent` refusal, which exists for upper bounds only;
  - the decisions the drive is not shown (the `unlogged` scope around the sweep), with one hand-made exception that does show it;
  - the note in `DischargeWidths` about bounds widened by the parameter box (issue 1191).

  On a wide leaf, a lower-bound or strict pair blocked by the family's spread splits cells until it hits the depth floor or the budget, then refuses as if it were a budget problem.

**Is `Sym` incompatible? No: it fits, but running the engine in it is impractical (likely).**
- It fits because `Sym<Interval>` is itself a `Real` that the funnel can decide on, so the engine's arithmetic could run at it.
- It is impractical because:
  - Sym's only extra power is proving that an expression is identically zero, and nearly every engine decision is a strict sign it cannot change (distance, departure, monotonicity). The earlier fork (row 202) found the same: a sweep run in Sym "pays `discharge_retried` on every indeterminate margin for no decision it can change".
  - Each cell's (u, v) would have to be fresh symbols, so the symbolic expression graph would grow with the subdivision, on every interval operation.
- The few places where it would decide something are fault (a): facts about an edge or a vertex, not about a cell, which tier 3 already proves for every member of the family at `Sym`. Change 2 gets the whole benefit for none of that cost.
- Worked example (likely, read from code, not run): a cylinder whose height h ∈ [9.99, 10.01] mm, capped by a plane, at `Sym<Interval>`.
  - Tier 3 proves symbolically that the rim lies on the cap plane.
  - The engine's "edge on carrier" check evaluates the plane over the cylinder's rim with the two heights treated as independent brackets. It reads about ±0.02 mm, not inside ±ε, so it refuses `Uncertified` on a body that tier 3 accepted.
  - With change 2, only the departure sign is read. That sign is macroscopic, so it decides over the whole box.

**The options as final states.**
- **X, recommended: changes 1–3.**
  - What becomes true: one crossing, held by the lane; no re-decided identities; refusals carry their lever. f64 and Probe are unchanged.
  - The cost: the engine needs tier 3's per-edge verdicts at the gate. The finished-body gate runs tier 3 just before it, so they are in hand.
  - Reversible.
- **Y: a separate range type**, a newtype `Real` over `Interval`.
  - For: the type says what the value is, and "split the cell" could be built in rather than kept by an unlogged scope.
  - Against: it duplicates the interval scalar, and it fixes neither (a) nor (b).
  - It could be added on top of X later.
- **Z: run the engine in `Sym`.** Rejected as above.

**What X changes.**
- **The at-rest gate unit's spec:**
  - no f64-specific entry and no extra margin;
  - the clearance-lane doc's claim that at f64 "a search finds a near pair, not a verdict" is false (subdividing cells gives a verdict at a point build too), and it goes with the lane split;
  - the spec lists the engine's Zero places and the tier-3 verdict each one reads;
  - a gate refusal caused by the family's width tells the drive to bisect;
  - add to "measure first": a leaf at `Sym<Interval>` on a box wide enough that tier 3 needs the symbolic proof. The prediction is that today's engine refuses there.
- **The shell's cavity check:** its `lift` parameter goes, and its verifier reads `Candidate::enclosure()`. Whether the check retires later is unaffected.
- **Ratified text:** no decision changes. One clarification is optional:
  - The clause: `[topology.tiers]`'s "At a point scalar… the band… At `Interval` and `Sym` the stored brackets carry the family instead". It was written in PR 6308 after you asked for the K·ε band, and approved with "mostly looks good".
  - The problem: it reads as if Interval does not decide with the band.
  - Proposed wording: "the question decides with the band at every scalar; what the stored values stand for is the stored body itself at a point scalar, and every member of the family at `Interval` and `Sym`".

**Confidence:** recommendation likely; f64's meaning and the band sure; the four Zero places likely; the Sym example and budget burn likely (read from code, not run).

## For the orchestrator

- **Nothing was built or run.** Every claim comes from reading the code at `b125b84b307` (main as fetched). The target directory was never created, so there is nothing to delete.
- **Unchecked: whether each Zero place matches a tier-3 verdict exactly.** This is the load-bearing part of change 2, and it needs a spec pass:
  - `self_intersection_on_shared` with an engine-built plane through straight shared edges;
  - the cusp check against tier 3's wedge reading;
  - the apex check against the chart-singularity vertices.

  Check also whether the gate can pass tier 3's verdicts to the engine (`boundary_of` builds `Boundary` with no tier-3 input today).
- **Errors in the brief or the row:**
  - The row's bullet "f64 and Probe as points, with the K·ε band" describes today's code; it is not an option.
  - The row says f64 "first enters the engine" in the at-rest gate unit. That is true of the lane: `clearance_lane()` is `None` at f64 today, while `Interval` and `Sym<Interval>` answer.
  - `Sym<f64>` also answers `None`. `[topology.tiers]` names "Sym" among the scalars allowed to certify, so the gate unit's lane split should say whether `Sym<f64>` gets the yes/no verdict door.
- **Provenance checked:**
  - The `[topology.tiers]` band clause: commit `2832ce48f66`, PR 6308, written after Ev asked for the band.
  - "Stored brackets carry the family instead": commit `53c9a148d31`, agent-written, approved in passing.
  - `[one-way.readings]`' "two readings of one fact": commit `ef53c5fee97`, an `[ev]` PR.
  - `[simple.conditioned]`: commit `31576a6c6ee`, PR 6329.
  - Row 202 was decided by an orchestrator without Ev, so its "engine stays `Interval`" choice is not ratified. This report keeps it, for a new reason.
- **Possible issue to file under `onehome`:** the same crossing is spelled in `props/quad_lane.rs` (34 sites) and `cell_region.rs`. Change 1's "the lane owns the crossing" applies to the quadrature lane too.
- **Off-question smell, not filed:** `props.rs`'s `clearance_lane` doc ("`None` at `f64`… is a shape: a point build… finds a near pair, not a verdict") still carries #1737's premise, which fork row 237 found false. It goes with the gate unit's lane split.
