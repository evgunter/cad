## For Ev

**Recommendation (likely).** The embedding question ends at its first finding, a verified crossing or a refusal, and reports that one finding. I withdraw the list of findings per face pair and the change to `ERROR-DESIGN` E7 that went with it. Both reports now recommend this.

**Beside it, a defect in the engine, already against ratified text:** a local certificate that fails answers "split, a smaller cell may hold" without any ground for it. That is where the seconds go, and no stopping rule reaches most of them.

**One thing is left open, and it does not need your ruling now:** once the engine finds out cheaply that a place cannot be read, should a refusal still end the ask, or should the ask go on so that a crossing elsewhere is what the person hears? Both reports say to measure first. It is stated at the end.

**Terms.**
- An *ask*: the question put to one solid, "is your boundary embedded".
- A *cell pair*: a patch of one face against a patch of another. The engine halves cells until each pair is decided. *Touching* cell pairs share a point of the boundary and are decided by a *local certificate* (how the two faces leave their shared edge or vertex). The rest are decided by distance.
- A *refusal*: a cell pair the engine could not decide. A *crossing*: a verified pair of points where the boundary meets itself.
- The *depth floor*: 40 halvings, where `f64` stops carrying information.

**What was measured** (372 refusing asks: the twelve dearest, a sample by refusal shape, and the asks of the 22 tests that cross).
- The twelve dearest: 13 asks, 4,434 s, 39% of all refusal seconds in the suites. Each refuses within 3 splits. Then 4 to 21 other face pairs refuse in their turn, and 3,250 s go by before each reaches its own first refusal.
- Stopping the ask leaves 4.6 s of those 4,434. Settling each face pair separately leaves 3,280 s.
- On the cheap asks the first refusal comes at the depth floor, 71 to 93% of the time precedes it, and stopping saves 6 to 37%.
- Over the whole suite stopping should remove roughly 5,000 to 5,600 of the 11,434 s. That is an estimate from the sample.
- 38 asks found a crossing after meeting a refusal, in 22 tests, out of several thousand asks that cross.

**Why I give up the list.** I asked of each thing the list held who reads it and what breaks without it.
- *The other unread pairs.* A person gets the next one from the next finish, and a finish that stops at its first finding takes under a second. An agent sizing engine work can read past the first refusal in the measuring harness, and `[simple.checks]` keeps that out of the gate. Nothing in the product reads the list.
- *Further crossings.* Same answer: the next finish reports the next one, in milliseconds.
- *A crossing behind a refusal.* This is the one real loss. See below.
- Nothing that exists needs an answer per face pair. PR 7101 carries the facts of finished bodies, and a refused body is never finished.
- "Every finding kept, none folded" (your words, PR 6326) holds: the ask found one thing and reports it.

**What stopping makes true.**
- The answer stays one of three verdicts per ask, as E7 has it.
- The engine keeps no "first refusal so far". A refusal ends the walk as a crossing already does.
- A person waits for the first finding and no longer.

**What it leaves possible.**
- **A body that crosses itself and also has a place the engine cannot read is told only about the unread place, when the walk meets it first.** The person has no lever on that finding, so the crossing stays unseen until the place is readable or remodelled. The body is refused either way and never ships.
- Which refusal is reported depends on face order and depth. Each is a true obstruction.
- A correction to both earlier reports: not every refusal has the kernel's recourse. One inside the band, or a torus that is not a ring, is the model's to move. It still needs no list: the next finish reports it.

**Worked example.**
- *A valid body the engine cannot read*: a fillet T (a torus face) meets a plate P along an edge with a corner no certificate reads. Today: 150 to 564 s, then "T and P: unread". Stopping: the same finding in about a third of a second.
- *The same body with a rib R passing through a wall W.* Today: "R and W cross here", quickly (the 38 such asks took 5 s together). Stopping: "T and P: unread" if the walk meets that corner first, and nothing about the rib.

**The engine defect (likely).**
- A failed certificate answers "split" with no reason attached (`Outcome::Split`). The engine cannot tell "this cell is too big" from "no reading exists for this shape at any size", and learns the second only by reaching the depth floor or the budget.
- The certificate knows more than it says. A part unread "for a reason no smaller cell changes" still splits unless both parts are. A departure sign decided inside the band comes back as "no sign" and splits, where a distance cell refuses at once.
- Final state: "split" carries its ground, the thing a smaller cell would reduce. A failure with no such ground refuses that cell now, typed.
- `[one-way.readings]` already says it: "an unlisted arm refuses typed, naming what it lacks; no arm answers a default value". So this is a defect to file, not a ruling to make.
- It matters under any stopping rule. After stopping, what remains is the time before the first refusal.
- Its saving is not measured.

**Ratified text.**
- `[topology.tiers]` gains one sentence: the embedding question ends at its first crossing or refusal, and that is its finding.
- E7's single trichotomy stays. I withdraw my change to it.

**What is left open.**
- *The question:* once refusals are cheap to reach, does a refusal still end the ask?
- *Staying stopped:* one rule, a certain price, the text above final. The 38 asks keep telling a person "the kernel cannot read this" where the model crosses. That count shrinks as the engine reads more.
- *Going on:* the walk continues after a refusal and a crossing answers. It is the kernel's own rule for a search (`geom_core::ladder`: an undecided reading "refuses only where no later reading answers"). It reaches every one of the 38, since it is today's walk. It costs one ordinary walk of the rest of the body per refusing ask, which is only affordable if the engine fix works.
- I lean to going on if a measurement shows it costs about what the same body costs when it holds. The other report leans to staying stopped unless a measurement shows a search is cheap and effective. If the engine fix removes less than I expect, the stop stays, and I would not reopen it.

**Confidence.**
- Stopping the ask now: sure on its numbers, likely as the rule.
- No reader for a list: likely.
- The engine defect as the cause of the 3,250 s: likely. It fits the code and the tracker's own rows, and nobody has traced it cell by cell.
- The open half: unsure, by construction.

## For the orchestrator

**1. The needs test on my parts 2 and 3.** Applied above. Only the crossing behind a refusal survives it. The least mechanism for that is no list and no per-pair type: it is not breaking the walk at a refusal, which is today's code with the engine defect fixed. So the full per-pair list was more than the need, and I drop it. I do not think a separate search mode exists that is smaller: near a shared edge the certificates are what end the subdivision, so a search without them has no natural end. The other report's probe-only continuation would need a depth dial.

**2. If part 1 removes less than expected.** The stop stays. My round-2 final state was conditional on an unmeasured saving and I presented it as the final state. The unconditional decision is the stop.

**3. Who needs the per-pair answer.** Nobody. The carry reads holding facts of finished operands. `BoundReport::across` is a per-pair count for test rows. The gate's finding is per solid, the shell's cavity gate folds to one error, and the editor's assertion has one verdict. Only the list wanted it.

**4. What we both sign.** Stop the embedding ask at its first finding; one sentence in `[topology.tiers]`; E7 unchanged; `EngineRefusal::Budget` gains the faces of the cell pair it was refused on; an engine row for ungrounded `Split`; measure before anything goes on past a refusal.

**Where we still differ** (none needs Ev unless you want it ruled).
- *The default on the open half.* Stated to Ev above with each side's cost. Both gate it on a measurement, so it is a difference of burden only.
- *The form of the engine fix.* The other report: one row per certificate, recognising each unreadable configuration up front, as the walled-spiric arm does. Mine: `Outcome::Split` cannot be built without a ground, so a configuration nobody has listed refuses by default. Rows alone leave the default at "split", and each new unread shape pays the depth floor again until someone lists it. Mine costs typing about 30 `Unpresented::Local` sites and risks over-refusing where a ground is typed wrongly.
- *The scope of the stop.* The other report: one rule for every lower bound, the editor's clearance assertion included, and `lower_refusal` goes from `Bound`. Mine: the embedding question only. Every measurement here is of the embedding ask. A metric walk has no certificate work to save, so stopping it buys nothing measured and can turn a definite `Violated` into `Refused(Sliver)` with a tolerance recourse that cannot change the answer. The uniform rule is simpler code; the scoped one changes nothing in CLEAR's territory. I expect the practical loss to be small either way, since violations come from root probes and slivers from deep cells. CLEAR should be asked.

**An error in my round-2 report.** I wrote that stopping reports "28% of crossing asks" as kernel limits. The 136 crossing asks are those of the 22 tests behind the 38, not the suite's. Suite-wide it is 38 asks against several thousand that cross (8,727 in the first census). The figure should not be quoted. The Ev section above uses the right one.

**A correction to the other report.** "Every such finding has the same recourse" is not so: `Sliver`, `TorusNotRing`, a parameter box too wide at `Interval`, and the near-coincident bodies behind some `Budget(Depth)` refusals are the model's. The conclusion stands for the reason given to Ev.

**Measurements.**
- *Grounds* (my M5, the other report's M6): on the 13 dearest asks and a sample of depth-floor asks, for every touching cell that answers `Split`, which part failed and how (`Structural`, `Local` and its site, a sign undecided with its margin and whether it lies wholly in band), and whether the child failed as its parent did. Per refusing pair: the depth at which a ground that cannot shrink first appears, and the splits after it.
- *Going on*, once a prototype of the engine fix exists: over the refusing population, the cost of not breaking at a refusal against the cost of the same walk stopped; and whether the 38 are still found, since refusing cells earlier sheds fewer distance cells beside an unread edge.
- *The whole suite* by refusal kind under the stop. The 5,000 to 5,600 s is the other report's extrapolation and matches mine.

**For the unit.** I agree with the other report's list (break at each lower-bound refusal site, the receipt counting the rest `abandoned`; `upper_refusal` stays; the three doc comments re-worded), with the scope question above decided first. Under the scoped form the break keys on `Bound::injective` and `lower_refusal` stays for the metric questions.

**The engine row, in the tree.**
- `Outcome::Split { arm, shared }` in `injective.rs`: "No arm held here; a smaller pair may."
- `departure` returns `split` when either part is not presented; `stuck` refuses only when both are. `work/embed/a-contradicted-extent-refuses-with-no-typed-cause.md` records the one-sided case running to the depth budget.
- `interval_sign_as` maps `Ok(Sign::Zero)` and `Err(Indeterminate)` both to `None`. Distance cells refuse there through `Bound::terminal`.
- `CUSP_DEPTH`'s doc says an uncertifiable edge otherwise ends "by budget exhaustion" and calls its ten "a trade and not a derivation". A grounded `Split` retires it.
- The two "reads local at every depth" rows and `curved-presentation-reads-plane-and-cylinder-images-only` are instances.
- At `Interval` and `Sym`, a margin wide because the parameter box is refuses at once and names the box. That replaces `blocking_width` on `embed/gate`, which guesses from vertex coordinates.

**Not checked.** That the pairs behind the 3,250 s split on grounds that cannot shrink: inferred from the code and the rows. The second census's count of crossing asks. Not read: `draw.md`.