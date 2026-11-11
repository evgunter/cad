# Fork `clear-enclosure` — designer B, round 3

Shown: designer A's round-2 report. Asked: PR 2701's reach, whether the door can be bypassed, the type's cost, and a final recommendation.

## For Ev

**Recommendation (likely on the substance, unsure between the two mechanisms).** Everything the two reports agree on stands. For the one open point, I would put the engine's door to you as the recommendation and the `CellBound` type as the alternative. Both of us can live with either.

**1. PR 2701: a delegating `Real` with no `Decide` is outside what you retired.** I overstated this in round 2.
- What you retired was a second arithmetic. Your words on the finding were "RingInterval probably isn't earning its keep. Probably it should be removed and we should only use the full `Interval`", and on the PR "i lean towards a more aggressive unification".
- The finding you accepted was titled "Two interval arithmetics". `RingInterval` re-implemented outward rounding, had no decoration channel, and poisoned where `Interval` clamps.
- A `CellBound` that wraps `Interval` and delegates has one arithmetic and no semantic difference, so the letter of the ruling does not reach it.
- "We should only use the full `Interval`" does lean against any second interval-shaped type. Your own words on PR 6308 ("isn't it semantically a different type") reopen that for this role.

**2. The door cannot be bypassed for the fault it is for.**
- Change 4 puts the unlogged scope inside the engine's sweep. An engine site that calls the lane's `decide` directly inside the sweep is still inside the scope, so its rows are discarded all the same.
- The shell's miss would have been caught. The shell calls `lane.sweep`, which is the sweep.
- The engine's decisions outside a sweep are facts about the body, decided at the body's scalar (for example whether a torus is a ring, in the window door). Those belong in the log.
- The finding's point reading has to be logged, so it is made after the sweep returns. `wire_clearance` already does this for its one exception.
- What the door does not do: nothing stops a cell reading from accepting a Zero. The rule "a cell bound never establishes an equality" is then held by the spec and review, not the compiler.

**3. What the type costs beyond compile time.**
- **A fifth `Real` kept in step by hand.** The kernel's vectors need `Real` for sums, dot and norm, so `CellBound` must implement all of `Real` by delegation, and follow every later change to that trait.
- **`[scalars]` changes.** Its ratified list of `Real` instantiations gains a member that is not a lane scalar. `Body<CellBound>` would typecheck, since a body needs only `Real`. It is only the certifying doors that would refuse it.
- **Wrapping at every seam.** The evaluators over a cell are written for `Surface<Interval>` (the cell enclosure, the spline cell-net memo, curve and pcurve evaluation, `geom-brep`'s helpers). Either each result is wrapped on return, or the carrier is held as `Surface<CellBound>` and those evaluators are instantiated a second time.
- **Unwrapping is the escape hatch.** Receipts and those helpers need the endpoints back. So "fails to compile" is as strong as the discipline on that one accessor.

**4. The two final states.**
- **Recommended: the door.**
  - What it makes true: the sweep and the minimum-separation walk own their unlogged scope, so no caller can forget it, and F29's double read and the callers' F30 scope go.
  - What it leaves: inside the engine a cell bound and a family value are both `Interval`, and the no-equality rule is held by the spec.
  - Lasting cost: none.
- **Alternative: `CellBound`.**
  - What it makes true: signatures say which values are cell bounds, the cell reading can have no "accepted Zero" arm, and the lane's `decide` cannot be called on a cell bound.
  - What it leaves: the unwrap accessor.
  - Lasting cost: the items in section 3.
- **Why I lean to the door.** It closes the measured fault completely, with nothing kept in step by hand. The type is the better pick if you weigh "the type should say what the value is" above that, which was your instinct on PR 6308.

## For the orchestrator

- **Converged.** Designer B agrees with A on everything except which mechanism is the recommendation and which the alternative, and can live with A's.
- **Read this round.** H5's S1 finding and its RATIFIED section at `e8d0883eda`, and PR 2701's comments (in round 1).
- **Not checked.** A's statement that `eval_node` opens a verdict bracket per node. Whether `min_separation` has any caller left (F28 says no evaluation asks it now).
- **Spec note for whichever is chosen.** The verifier closure runs inside the sweep. The logged point reading must therefore be made by the caller after the sweep returns, from the finding it carries.
- **Nothing was built.** No target directory exists.
