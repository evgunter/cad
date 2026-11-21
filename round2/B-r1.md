## For Ev

**Recommendation (likely).** Make the embedding question answer what `[topology.tiers]` already says the certificate is: one fact per pair of faces. Each face pair the gate asks about ends as one of three facts: *apart* (certified), *crosses* (with its witness), or *unread* (with the engine's refusal). The engine stops reading a pair at the first reading that settles its fact, a verified crossing or a refusal, and goes on with every other pair. The gate's refusal lists each pair that is not apart, one finding each.

**The choice that is yours.** Whether the answer is per face pair (A, above), or one finding per solid: the ask ends at its first crossing or refusal and reports that (B). B is simpler and cheapest. I lean A. Both are laid out below. Today's behaviour is neither, and I would not keep it.

**Terms.**
- An *ask*: the question put to one solid, "is your boundary embedded".
- The engine covers each pair of faces with *cell pairs* (a patch of one face against a patch of the other) and splits them until each is decided. Cell pairs that share a point of the boundary are *touching*: they are decided by a local certificate (how the two faces leave their shared edge or vertex). The rest are decided by distance.
- A *refusal* is a cell pair the engine cannot decide: no certificate reads it, or the budget ran out.

**Premise check.**
1. The seconds are not spent looking for a crossing. What runs after a refusal is the whole proof, local certificates on touching cell pairs included, and a touching pair is never offered to the verifier, so that work cannot produce a crossing. Crossings come from a probe at the top of each subtree and are cheap: 7 ms an ask on average, and the 38 asks that found one after a refusal took 5 s together. (sure, from `Sweep::run` and `Bound::pair`)
2. "It goes on so that a crossing elsewhere is found" is not a rule the code keeps. A face whose surface the engine does not read (`Unsupported`) refuses the whole ask before any pair is read (`Side::windows`). And a crossing, once found, drops every refusal kept before it. (sure)
3. "Every finding kept, none folded into another" is your sentence from PR 6326 ("can we just return a list of failures so we don't have to agglomerate them?"). It is about the refusal's shape, not about how far the gate must look: the gate already stops between stages (embedding is asked only once tier 3 passes). So it does not oblige the engine to run on. But "keep the first refusal, count the rest", which the engine does, is the agglomeration you turned down there, one layer further in. (sure)
4. About 5,100 s of the first census's 12,173 s of refusals are asks whose only refusal is the budget itself. No stopping rule touches those. They are what the backstop costs when it fires, about 6 s an ask at CI's build, and only the engine rows or the constant move them. (sure that they are untouched; the by-kind split exists for the first census only)

**What the issue is evidence of.** The engine has no notion of a face pair's fact. It folds all pairs into one verdict with a ranking (a crossing over any refusal, the first refusal over later ones) that nobody argued for the refusals; ONECOPY's sweep of this pattern says so in those words. PR 7101 then made the face pair the unit of the certificate, and the gate lane found that on a refusing ask a pair's share "is not defined". Seen this way the question is which unit a refusal settles. Today it settles one cell pair, so everything else is still read, and one refusal is reported.

**A. One fact per face pair (recommended).**
- Makes true: the answer has the certificate's shape. A pair's fact is the same asked alone or among the others, up to the budget; PR 7101 pins that before any fact is carried, and today it can only be checked on asks that hold. Every finding names its two faces (a budget or sliver refusal names none today). No ranking between a crossing and a refusal is left, because both are listed.
- It matches the gate's other stages: tier 3 reports every bad edge and face of a stage, and the embedding family already pairs a definite finding with an undecided one per decision.
- Cost: pairs that hold are read as on a body that holds (65 ms an ask on average). An unread pair costs what it takes to reach its first refusal. A pair that neither certifies nor refuses still runs to the budget, and the finding then names the pairs still open.
- A crossing no longer ends the ask: every crossing pair is listed. A crossing body then costs about what it would if it held, against 7 ms today.
- Leaves possible: two faces unread at their shared edge that also cross elsewhere report only "unread". The finding still points at the right two faces.
- The clearance assertion keeps its single verdict. There the selection is the unit, one face pair can be the whole question, and a definite violation must still outrank an in-band refusal.
- Reversible: folding the list back to its first entry gives B.

**B. The ask ends at its first crossing or refusal.**
- Makes true: no work after the kind of verdict is known; the engine keeps no refusal at all; the lowest price.
- Leaves possible: a body that crosses itself is reported as "the kernel cannot read faces a and b" whenever an unread pair comes first in the walk, and the person has no lever on that finding. Of several unread pairs one is reported, chosen by face order and depth. In the suites this is at most the 38 asks, of 8,727 that cross.
- The gate's list for this family has one entry per solid by construction, and an agent sizing the engine work sees one obstruction per body per run.
- Reversible: A can be built on it later.

**Worked example.** A bracket. A fillet (torus face T) meets the plate (face P) along an edge the engine cannot yet read. Elsewhere a rib R passes through a wall W.
- Today: a cell pair of T against P refuses. The engine goes on splitting, up to 65,536 cell pairs (the 126 to 498 s asks). It finds R through W and reports only that. Without the rib it reports T/P, after the whole walk.
- A: T/P closes at its refusal. The other pairs are read as usual. The refusal lists "R and W cross at this point" and "T and P, at their shared edge: unread".
- B: reports whichever of the two the walk meets first, and stops.

**Rejected.**
- Keeping today's rule and letting the engine rows shrink the refusals: it leaves the engine working on a proof that cannot succeed, and reporting one refusal picked by frontier order.
- A second, smaller budget for the search after a refusal: a second dial, for a search the engine does not run.

**Ratified text I would change.**
- `[topology.tiers]`, after "one fact per pair of faces": what the three facts are, that the question stops reading a pair at the first reading that settles it, and that the gate's findings are the pairs not apart. Under B instead: the ask ends at its first crossing or refusal.
- `ERROR-DESIGN` E7, "the answer is a trichotomy": drafted by an agent in July and ratified with the whole document (#110), before the gate existed. For the embedding question it becomes a trichotomy per face pair; the clearance assertion's stays. PRs 6301, 7068 and 7101 are the later design, so I would change E7 rather than read around it.

**Confidence.**
- A as the better final state than B: likely.
- Today's run-on being wrong under either: sure.
- How many of the 11,434 s each saves: unsure, not measured. Both save what is spent after the first refusal in the pair that refused. A does not save what is spent in other pairs, and saves nothing on an ask where a second pair runs to the budget. The number that decides it: per refusing ask, the cell pairs spent before the first refusal, after it in the refused face pair, and after it in other pairs. If the last dominates, B is the answer to the price and A is a later step.

## For the orchestrator

**Checked against the tree.**
- `Bound.lower_refusal` has eight `get_or_insert` sites and `upper_refusal` six; `Sweep::run`'s final match is as the brief says.
- `embedded()` returns `Refused` with an empty receipt when `Side::windows` fails, so premise 2 holds on main and on `origin/embed/gate`.
- Provenance read on GitHub: PR 6326's comments (the list of findings is Ev's own wording) and PR 7101's (Ev chose the per-ask budget over a per-face-pair one, and "not stored"). A keeps that ruling: one constant per ask, nothing stored.
- Tier 3 is staged with `?` between stages and collects every finding inside a stage (`structural_via`, `tier3_local_checks_marked`).

**Errors or gaps in the brief.**
- "Looking for a verified crossing" describes the intent, not the work: see premise 1.
- The by-kind split of refusal seconds exists only for the first census (whole-body ask, `362cd72fd2`). The second gives the total alone. I used the first as indicative.
- "What a person or an agent does with a refusal" is unknown. I assumed: a person looks at the named faces and changes the model there; an agent on an engine row wants every obstruction of a body, not the first in frontier order.

**Measurements the recommendation turns on** (the `embed/gate-measure` harness):
1. Per refusing ask: splits and seconds (a) before the first refusal, (b) after it in the face pair that refused, (c) after it in other pairs, with (c) split by how those pairs end (certified, refused, still open at the budget). B's price is (a). A's is (a) plus the certified part of (c) plus each other pair's spend to its own first refusal.
2. Of the 38 asks that crossed after a refusal: is the crossing in the refused face pair or another? A keeps only the latter.
3. Distinct unread face pairs per refusing ask, and asks holding both a crossing and an unread pair: how long A's lists are.
4. What reading on after a crossing costs over the 8,727 crossing asks. I expect about a holding ask each, roughly 500 s over the suites, less after PR 7068 removed the between-solid crossings.

**Claims I could not check.**
- That the shared memos (`RegionMemo`, `LocalMemo`) leave a pair's readings independent on refusing asks. It is measured on holding asks only; A's "same fact alone or together" rests on it.
- Where the post-refusal splits fall. My reading of `split_by`: along a shared edge where no cell certifies, touching pairs double about every two levels, so the depth floor (40) is out of reach inside 65,536 pairs. Such an edge ends as an early `Outcome::Refuse` followed by the run-on, or as `Budget(Pairs)`. Vertex-shared pairs stay narrow and do reach depth 40.

**What A means in the code** (for the unit, not for Ev).
- `Bound.lower_refusal: Option<EngineRefusal>` becomes a per-candidate state keyed by `Task.k`: open, crossed, or unread. The `get_or_insert` sites become "close pair k". A closed pair's queued tasks are counted `abandoned` (the receipt's existing meaning) and must leave `Walk::split`'s `committed` sum.
- `overflow` closes every pair still on the frontier with `Budget(Pairs)`.
- `Side::windows` answers per face, so an unsupported face closes only the pairs that hold it.
- `EmbeddingReport` carries the facts. The gate maps each to a finding with its two faces; `EmbeddingUncertified` on `embed/gate` gains `faces`. `topo::shell`'s cavity gate folds the list to its own single error.
- With PR 7101's carry the ask's input becomes a set of face pairs, so pairs in and facts out is one signature.

**Seams.**
- `Sweep::run` and `Bound` are shared with CLEAR's metric questions. Pair closing must be the injectivity question's alone: for a metric lower bound a single face pair can be the whole assertion, and closing it at a sliver would hide a definite violation in the same pair. Announce it; the upper bound's state is untouched.
- `validate.rs` is TOPO's. The finding change is on `embed/gate`, which is unmerged, so it lands with lane C.
- The module doc of `clearance.rs`, `BoundVerdict`'s doc and `Sweep::run`'s doc each state the stop rule and would be re-worded with the change.

**Defects off the question** (none filed: this lane is read-only).
- On `origin/embed/gate`, `embedding_refusal_words` names no face ("naming no arena key"), so `EmbeddingUncertified`'s sentence does not say where the body is unread. `EngineRefusal::Sliver` and `Budget` carry no face at all. Belongs on this row if A is taken, otherwise beside `a-contradicted-extent-refuses-with-no-typed-cause`.
- `GATE_MAX_CELL_PAIRS`'s doc on `embed/gate` says a verdict "does not depend on who asked". After PR 7101 a budget refusal follows the route. Lane C's to re-word.
- An edge along which the departure arm only ever answers `Split` is reported as `Budget(Pairs)` after 65,536 cell pairs, never as `Uncertified` with its edge and arm, because breadth-first cannot reach the depth floor there. `CUSP_DEPTH` is the one existing early floor. This is the 5,100 s no stopping rule touches; it is an engine row of its own, near `nested-tori-a-thin-gap-apart-spend-the-pair-budget-on-box-distance`.

**Not read.** `draw.md`, as instructed. PR 7068's and PR 6301's comments; I used the gate row's account of them.