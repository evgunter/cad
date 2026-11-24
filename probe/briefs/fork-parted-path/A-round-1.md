DESIGNER REPORT, round 1: a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing

## For Ev

**Recommendation (likely): gate a leaf on the build, not on the decision log. This changes my first answer.**
- **Certified:** every predicate is definite over the leaf, and every node has the witness's standing and the witness's names.
- **`FlipCrossing`:** every predicate is definite, and some node builds otherwise. This always names the first such node, together with the cause the log shows there: a turned decision, a parting, or a structure flip.
- The decision log stops being the gate and becomes the explanation. The kernel disagreeing with itself is caught by a test instrument, not by the drive: replay at a zero-width box on the witness point, compare the whole log, checks included.
- This holds only if the names say everything "the same build" has to mean; that is the first thing to measure. If they don't, the gate compares choices and standings, and a turned lane choice still refuses wrongly (see below).

**Terms.**
- *Witness*: the nominal run at f64, at one point.
- *Leaf*: a sub-box of the parameter box, replayed over interval arithmetic.
- *Verdict*: one logged decision: predicate, definite sign, and role. A *choice* is one where another sign can change what is built. A *check* is one where another sign can only make the node refuse.
- *Turn*: the same predicate, reached along identical choices, decided with the other sign.
- *Parting*: any other first difference: another predicate, a log ending early, or a pruning query (a box-overlap test) handing on other candidates.
- *Name table*: the names a node's outputs carry, minted from lineage and structure.
- *Lane choice* (DESIGN `[verdicts.consumers]`): a reading that picks the closed form or the general lane for a result the body has either way.

**What moved me: three facts checked in the tree.**
1. **A check can pass with either sign, from geometry alone (sure).** `geom_brep::edge_wedge_over_as` records each corner's `dihedral_sign` in a role its caller passes. The constructors pass `Role::Check`, because the stored result is "the same for a crease of either turn". So a crease that is convex at the nominal and concave over a leaf is a check of the other sign on the same build. My first report found no such site; this is it. The row C cites in `props_verdict_shapes.rs` (row 3) is the same shape, but with hand-written logs.
2. **Extra candidates from a box query do log decisions (sure).** In `topo`'s `boolean::ops::walk_pairs`, a pair that passes the marked box test goes on to `separating::apart` and `pair_verdict`, and both of those decide. An interval leaf's wider boxes therefore hand on pairs the witness pruned, and its log grows choices about them. `VerdictVector`'s doc ("then decide nothing about them") is false; the doc needs correcting.
3. **A lane choice is logged as a choice (likely).** `geom_brep::intersect::closed_form_shown` (for example `pn_axis_normal`, the circle-versus-ellipse frame) reads through `decide`. That logs a verdict in the caller's role, which defaults to `Choice`. A tilt that is Zero at the nominal and definitely nonzero over the leaf takes the general lane. That is a turn, and every log-based gate (mine, C's) refuses the leaf as "proven different". Yet by `[verdicts.consumers]` the leaf states the same result its cells have either way.

Fact 3 is what moves me from my first answer (gate on the choices) to gating on the build. A gate on the log, at any strictness, gets this case wrong. No fix to roles repairs it: the role has two values, and a lane choice is neither, since it changes the form built but not the cells. Asking about the build directly is the general form.

**The others' arguments.**
- *C: "a choices-only gate rests on every role stamp being right, and the side reader's rays are checks whose combined answer steers the build."* The second half does not hold. `topo::side_reader::side_at_infinity` casts its rays in a check scope, then records the combined side once as a choice (`answered(SIDE, …)`, the `side_read` verdict). Its doc argues exactly that split. The first half is right in principle, and it is a further reason to prefer the build gate, which rests on no stamp at all.
- *A and B: "comparing checks tests the kernel's two lanes against each other, not the document."* I keep this. The whole-record gate in C buys one thing the instrument does not: protection against a lane's parity bug on documents outside the corpus. It pays by refusing geometric check turns (fact 1) and lane-choice turns (fact 3) under a false name, and by bisecting every fat-box parting (fact 2) down to the floor.
- *B's open question, whether names are complete.* If standing plus names determine the cells, how they meet and what was glued, then B's is the right final state. E6 already leans on "lineage-scoped key identity" for that claim. If they don't, the fallback is my first answer: choices plus standings.
- *A parting away from a pruning query.* I called this a kernel defect and recommended `unreachable!`. I withdraw that. Under the build gate it needs no class of its own: either the build is the same, so certify, or it differs, so `FlipCrossing` with the parting named as the cause. The consumer is told only "proven the witness's build" or "proven another". The floor costs nothing extra: no bisect is spent on a parting.
- What could have made that `unreachable!` reachable, unmarked steering and lane-specific logging, belongs to the parity instrument and to the doctail marking rows. A leaf that holds the witness point and builds otherwise is a kernel bug: `[failure.taxonomy]` row 5, a `debug_assert` in the classifier. The range search's own copy then goes.

**Final state.**
- *Certify*: definite, and per node the standing and names equal the witness's, assertions excluded.
- *`FlipCrossing`*: definite, and the build differs. The evidence requires a first node: `{ node, difference, cause: Turn | Parting | Structure }`.
- Deleted: `FlipEvidence::is_empty`, the " unnamed" in `render_reason`, `range.rs`'s `flip()` filter, `Ground::Unfounded` and `Unproven::FlipUnfounded`, the viewer's sentence and its `Floor(FlipCrossing)` arm, and the Python tag `flip_unfounded`. The tour does not change.
- *Masses*: certified = proven the witness's build. `flip_crossing` = proven another build, which is the finding E6 means. Every other class = unsettled. E2 containment (`drive::contained`) becomes true as stated.
- *Reversible*: the gate is one comparison.

**Ratified text.** E6's no-flips policy is Ev's own proposal: refuse the far side, enumerate no branches. It stays. Only its condition changes, from "definite on a different verdict vector" to "definite and building otherwise". B's proposed wording is right and short. The plate's premise ruling (F48) stays, as parity hygiene. It no longer carries any weight for certification.

**Confidence.** Recommendation: likely. Facts 1 and 2: sure. Fact 3: likely, since I did not trace every caller's scope. Whether names are complete: unsure, and that is what decides between B and the fallback.

## For the orchestrator

- **What to measure, in this order:**
  1. **Do names determine the build?** Over the corpus, look for two runs, witness against leaves either side of known flips, whose standing and name tables are equal but whose cells or adjacency differ. Check too that the names come out identical at f64 and at `Sym<Interval>` on a zero-width leaf. This decides between the build gate and the fallback.
  2. **The zero-width parity instrument on the corpus,** built and green before the gate changes, because the drive will stop seeing parity defects.
  3. **On the tour and the corpus:** how many of today's refusals (empty ones, and named turns of a check or a lane choice) the build gate would certify, and how many it would refuse with each kind of cause.
  4. The cost of comparing name tables on each leaf, which is probably small.
- **Corrections to my first report:**
  - I said I found no check that passes with either sign. There is one (fact 1).
  - I relied on the `VerdictVector` doc's "decide nothing", which is false (fact 2).
  - I withdraw the `unreachable!` arm.
- **Off the question, by reading, unsure, worth checking:** `closed_form_shown` swallows an in-band `Err` from `decide`, but `decide` has already put the escalation on the node's log. `classify_replay`'s log read may then bisect, or refuse as a sliver, a leaf whose lane choice the general lane serves. That would be a lane choice ending as a refusal, which `[verdicts.consumers]` rules out. Measure on a tilt in band before filing.
- B's off-question point about `drive::contained` stands. File it with this unit or note it on the row.
