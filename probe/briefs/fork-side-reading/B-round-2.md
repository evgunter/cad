## For Ev

**Recommendation (likely).** Give the scalar one more projection, into an opaque **place**, beside the one into told text. `place()` on a deciding scalar returns a `Place`: a number drawn from the value's bracket that nothing can compare, add, print as a number or turn back into a `T`. Its only consumers are evaluators that take a place (here: a fixed line's readings against one stored plane, edge or vertex), and their results come back on the value channel and are decided through named predicates as today. The side reader forms its aim and direction at `T` as it does now, places them, and reads every member against that one line. No `Bounds` bound is added anywhere, and no signature in the three chains changes.

This does change ratified text; I was wrong in round 0 to say none would. The premise I shared does not hold: DL5.1 and DL5.2 say what may be done with an `f64` once read, and the `[scalars]` choice clause says a ray may be a choice, but none of them grants the read. The read is `Bounds`, and the reader's chains do not hold it.

Terms. *Bracket*: the two ends enclosing every real a scalar stands for. *`Bounds`*: the trait that reads them (`lo`/`hi`); by its scope rule it never rides with a deciding bound except in a file with a ratified ledger entry. *#990*: the ruling that no number leaves a `T: Decide`; a derived quantity leaves only as told text. *Place*: a number the code chose to evaluate at; any number would give a true answer, a good one gives a useful answer.

**Premise check (facts 1–5 against the tree).** 2, 3, 4 and 5 are as reported. On 1, one correction: the finish's own door (`validate_geometric`, `validate_geometric_certificate`) is bounded `CertifiedBounds`, so the gate that asks check 7 already holds a bracket right; it is the functions beneath it, shared with the dual's twins, and the other two chains that are `Decide`-only.

**What the written rules say.**
- *The `Bounds` scope rule permits a bracket read here, by a ledger entry.* "A bracket may prune a candidate set, drive a subdivision, or be reported"; choosing which ray to cast decides nothing (every ray from outside reads the same side; another ray changes whether the side is read, never which). What it forbids is how the read would have to arrive: a `Decide + Bounds` bound on the reader climbs all three chains to the public doors, which is `Bounds` riding with `Decide` across the kernel, the thing the rule says it deliberately does not do.
- *#990 forbids a plain number, and by its reason does not forbid a place.* Its sentence "No projection to a number exists, by this ruling" is absolute as written. Its reason is that a number is "a branchable channel, the same unmetered decision surface at one remove". A type that offers no value a comparison could read is the shape the ruling itself accepts for told text. So the letter stands in the way and the principle does not.
- *The #701 warning applies unless a gate reads the new name.* "A bracket reader under a name the gate does not read" is exactly what `place()` would be if it were ungated. It needs its own allowlist of call sites, each stating why the thing placed is a choice (as `register_equal`'s sites state their theorem).

**The final state.**
- *What the reading is.* One number from the bracket: the value at a point scalar, the midpoint of an enclosure. None where the bracket is unbounded or poison; the face then offers no aim.
- *What the signature claims at each scalar.* `f64`: the value, so the `f64` reader is bit for bit today's. `Interval`: one number of the enclosure, the same for the whole family. `Sym<T>`: `T`'s place; the expression is dropped. `Dual<T>`: `T`'s place; a place has no tangent (DL5.2 already says so). `Probe`: its value's.
- *Where it lives.* `geom-core`, beside `told_reading`: the scalar's two projections, each into a type nothing can branch on.
- *What a reader can do with it.* Hand it to an evaluator that takes a place; have it told as text (the aim's witness is already text). Nothing else. There is no door from a place back to a `T`, which keeps DL5.2's "a place is never a `T`" and the 2026-11-21 fix pass titled "no public place door".
- *What it must not be used for.* Standing in for a value that is a claim. A residual evaluated at a place certifies one point, not the box. The type cannot prevent that; the call-site allowlist and its stated reason are what a reviewer and a gate see.
- *Text to change, three sentences.* (a) `real.rs`, #990: "the one projection is `Real::told_reading`" becomes two projections, the second into a place, with the same reason (it "offers nothing a comparison could read"). (b) The same passage's #701 sentence: add that the place's call sites are a gated allowlist. (c) `geom-core` README DL5.2, the place row: a place may also be drawn from a value's bracket where what is placed is a choice under `[scalars]`.

**Alternatives, and why each loses.**
- *`Decide + Bounds` on the reader, with a ledger entry.* Needs no amendment. Loses: the compound bound reaches every generic door in `topo`, `sweep`, `verbs` and `editor-core`, and hands each of them `lo`/`hi`, where the need is one unbranchable number in one file.
- *The right passed as a value* (the precedent the scope rule records for the shell door): a token minted where `Bounds` is held, threaded down to the reader. Needs only a ledger entry. Loses: three chains gain a parameter every caller must supply; callers bounded without `Bounds` cannot mint it, so the riding ray survives beside the fixed one; and the inner code then holds a plain `f64` line unless it is wrapped opaque anyway, which is the recommendation with more plumbing.
- *A `T → T` "held still" operation.* No number appears, so #990 is untouched. Loses: the result is indistinguishable from a value, can be stored in a body or certified on, and silently narrows an enclosure. That is the worse hazard, and it is a place door by another name.
- *The driver supplies the ray from its `f64` witness build.* No projection at all. Loses: the finish then cannot be asked without a driver, and a body finished directly at a box scalar (every row file here) has no witness.
- *Read no fixed ray; read the side at one member.* Re-valuing the body at a point is a larger projection of the same kind, and the new measurement shows the embedding question it rests on is undecided on a tilted moving cap from the narrowest box.

**Provenance.** Not resolvable in this checkout: `git log -S'No projection to a number exists'` lands on graft commits of 2026-11-20/21 (`a9ccd8206`, `e35beb7a6`). The design-fork log has rows 291 and 292 (both 2026-11-19, `[ev]` PRs): 291 records a proposal that "`Real` gains one projection into it, so #990's 'no general projection helper' is amended to its principle" for told text, and the tree now holds that projection. If that is what Ev ratified, the recommendation asks for the same amendment a second time, on the same principle. I could not read Ev's own words on either.

**Confidence.** The premise failed as reported: sure. The scope rule permits the read by entry and forbids only its arrival by a climbing bound: sure. #990's letter forbids and its stated reason does not: likely (it is a reading of intent, Ev's to confirm). An opaque place is the right carrier: likely. That nothing else in the kernel needs a place today: unsure; I know of no second user, and one user is a thin reason for a new projection, which is the strongest argument for the token alternative.

## For the orchestrator

**Does M14 move my first-round answer?** In three respects.
- *Soundness held* (no wrong side on 100 members, in any variant): the fixed ray stands.
- *Reach is smaller than I said.* Walls read to 0.1, not my 0.19, stopping on the winding. The turn read along the ray (B2, close to my item 3) helps the tilted top (0.1 → 0.25 on the gentler tilt) and costs `lined_brick`'s bottom cap (0.45 → 0.25). I rated item 3 "likely"; it is now unsure, and should not ship as a uniform rule without finding why the bottom cap loses (true motion there is about 0.08 against a clearance of 0.19, so the loss is arithmetic, not geometry; I suspect each vertex entering two `atan2` terms).
- *The member layer is not to be built.* By the criterion I gave in round 1: the embedding census is undecided on the tilted cap from 0.05, so it offers no width the fixed ray lacks, and on that body the census row, not check 7, blocks the finish. On the axis-aligned bodies the reader already reads at all four widths.

**Other things M14 shows.**
- With a point ray, an advance enclosure of `[−106, 63.8]` on a tilted brick's wall can only come from the stored plane's own enclosure or a denominator near zero. That is the stored description (the fixture's or the door's mint), not the reader. Worth one traced representative before the unit is scoped.
- `brick_at` (edges with no stated lines) reads no further with the fixed ray. The fixed ray fixes the reader's half only.
- Plain `Interval` builds none of the bodies, so my round-1 argument that the fixed ray serves plain `Interval` is true and currently moot.

**Could not check.**
- Fact 1's full extent: I confirmed the reader's ten `T: Decide` sites and the finish door's `CertifiedBounds`, not every caller in `sweep`, `verbs` and `editor-core`.
- Whether Booleans' inner functions could mint a token from the `Bounds` their door holds (the BVH allowance); that decides how bad the token alternative's "two readers" is.
- What the 2026-11-21 "no public place door" fix pass ruled and who asked for it; I have its subject line only. If it rejected a scalar-level place projection as such, the recommendation contradicts it and the token alternative becomes first.
- How M14's switch read the midpoints (presumably `Bounds` in test code); I did not open the probe branch's diff.
- Whether a midpoint is always a usable place at `Sym<Interval>` for a direction (the midpoints of a unit vector's entries are not unit; the evaluator must renormalize at `f64`).
