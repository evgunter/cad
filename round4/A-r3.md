## For Ev

**Recommendation (likely).** One final state, reached in two steps.

**The final state has two parts:**
1. **Finding out that a place cannot be read becomes cheap.** When a local certificate fails, the engine carries the reason it failed. It splits the cell only when a smaller cell could change that reason, and otherwise refuses that cell at once.
2. **The ask ends at its first verified crossing, and at nothing else.** It always walks past refusals. If it finds no crossing, it refuses and lists every refusal it met.

**The first step comes before part 1 is built:** the ask stops at its first refusal. It is one line, and it is the only change that removes today's minutes-long asks. It comes out when part 1 lands.

Both reports now agree on part 1, on the first step, and on "nothing stops at a refusal" in the final state. One question is left for you, stated in full at the end. Once the ask has found a crossing, does it stop there, or read on and list every face pair that is not certified apart?

**Terms**
- **Ask:** the embedding question put to one solid: is its boundary embedded?
- **Cell pair:** a patch of one face against a patch of another. The engine halves cells (a **split**) until each pair is decided.
- **Touching cell pairs** share a point of the boundary. They are decided by a **local certificate**: how the two faces leave the edge or vertex they share. The rest are decided by distance.
- **Refusal:** a cell pair the engine could not decide. It names its two faces and its place: the shared edge or vertex, and which certificate failed there.
- **Crossing:** a verified pair of points where the boundary meets itself.

**Why stop at a crossing and never at a refusal (sure).** "Embedded" means "every pair of faces is apart". Read with three values (apart, crosses, unread), that "every" is settled the moment one pair crosses, since the answer is then no whatever else is unread. An unread pair settles nothing, because another pair may still cross. So the walk may stop at a crossing and must not stop at a refusal. This is the logic of the question, not a ranking of findings. The person is owed this: a crossing is the model's fault and has a lever, and a refusal is the kernel's limit and has none (`[loud-failure]`: the recourse says whose fault it is). Stopping at a refusal tells a person whose body crosses only that the kernel cannot read it. That happened in all 38 measured asks where a crossing came after a refusal, 28% of the asks that cross.

**What the measurement showed** (the gate lane, 372 refusing asks)
- **The time goes into reaching refusals, not into running on after them.**
  - **The 13 slowest asks** (torus bodies, 150 to 564 s each, 4,434 s in all). Each refuses first within 3 splits. Then 4 to 21 other face pairs refuse in turn, 3,250 s of it spent before each one reaches its own first refusal.
  - **The cheap asks.** Their first refusal comes at the depth floor of 40 splits. 71 to 93% of their time comes before it.
- **The cause.** Today a failed certificate says only "split, a smaller cell may certify". The engine learns that no cell of any size will only by running out of depth or budget. Part 1 fixes that claim. It is `[topology.tiers]`'s "a reading inside the band refuses as ambiguous" made reachable without exhausting the walk.
- **A refusal does not settle its face pair.** In 19 of the 38 crossings, the crossing lies in the same face pair that refused first, away from the place that could not be read.

**Worked example.** A fillet T (a torus face) meets a plate P along edge e and a wall W along edge g. Each edge ends in a corner that no certificate reads. Away from e, T dips through P, and a rib R passes through W.
- **Today:** the cells around each corner are halved again and again, each failing as its parent did, for minutes. Then one finding is reported.
- **First step:** "T and P: unread at the corner of e", within a second. Neither crossing is reported.
- **Final state, this report:** the corner cells refuse a few splits in, and the walk goes on. The first crossing it reaches, say T through P, ends the ask: "T and P cross here". Without the dip and the rib, the result would be "unread at the corner of e" and "unread at the corner of g".
- **Final state, the other report:** all three findings are listed: T and P cross, T and W unread at the corner of g, R and W cross.

**The question for you.** Once the ask has found a crossing, does it stop there, or read on and list every face pair that is not certified apart?
- **Stop at the crossing** (my lean):
  - The answer stays one of three verdicts per ask: embedded, crosses, or refused. ERROR-DESIGN E7's single three-way verdict is unchanged, and the engine keeps no per-face-pair state.
  - A body with several crossings shows one per run.
  - No other unread place is reported once a crossing is known. Those have no lever anyway.
- **Read on and list** (the other report):
  - Every crossing is shown at once, and every finding is filed under its face pair.
  - It costs about three times as much on asks that cross (measured: 7.8 s becomes 15.3 s over 136 crossing asks). It needs per-face-pair state in the engine, and E7 is read per face pair for this question.
- **Reversibility:** either can become the other later.

**The price**
- **First step:** measured on the sample. It saves 99.9% on the 13 slowest asks and 6 to 37% elsewhere, roughly 5,000 to 5,600 s of the suite's 11,434 s of refusing asks (extrapolated). It misreports those 38 crossings as kernel limits.
- **Part 1:** its saving is not measured yet. It aims at the 3,250 s and the 71 to 93% that no stopping rule touches. The first step stays until the measurement shows refusals are cheap.

**Rejected**
- **Today's behaviour.** It is the right logic paid at the worst price, and it keeps one refusal out of many.
- **Settling a face pair at its first finding.** It hides 19 of the 38 crossings.
- **A smaller second budget.**

**Ratified text.** `[topology.tiers]`, after "one fact per pair of faces", add: "The question ends at its first verified crossing; without one, any refusal refuses it, and every refusal it met is listed." Keeping every refusal met is what your PR 6326 words ask: "every finding kept, none folded". The walk meets them anyway, so listing them costs nothing. The PR 7101 backstop is unchanged.

**Confidence**
- The stop rule follows from the logic: sure.
- Part 1 is where to act: likely.
- How much part 1 saves: unsure (measurement M5).
- Stopping rather than listing after a crossing: a lean. Either is defensible.

## For the orchestrator

**What moved**
- I now accept the other report's part 1 as the design rather than as one row per certificate. `Outcome::Split` making its claim without grounds is the same finding as my round-2 engine row, generalised.
- I withdraw "the ask ending at its first finding is the final state". It is right only while refusals are dear.
- The needs test, split as asked:
  - Other unread pairs: no reader needs them, but the final walk meets them for free, so they are listed under "none folded".
  - Crossings: a reader needs one, since it is the only finding with a lever. Beyond the first, no need has been shown.
  - So the least mechanism that delivers the crossing is today's verdict logic: violation, else refusal, with a verified violation ending the walk. Add part 1, keep every refusal rather than the first, and drop the stop. It needs no per-candidate state, so it is smaller than the other report's part 3.

**The difference in one sentence:** "After a verified crossing, does the embedding ask stop (one crossing reported) or read on to list every face pair not apart (×3 on crossing asks, per-face-pair state)?"

**For the unit**
- **First step:** `Break` at each lower-bound refusal site of the injectivity question only. The metric questions in `editor-core/src/clearance.rs` and `eval/wire.rs` keep today's rule. This narrows my round-2 note, which made the stop uniform: a violation hidden behind a refusal is the same harm in a clearance assertion.
- **Final:** `Bound.lower_refusal: Option<EngineRefusal>` becomes a set keyed by the refusal's place, deduplicated by payload: faces, shared cell and arm for `Uncertified`, and the cell pair's faces for `Budget`, which arrives at admission (M1, column d = 0). `EmbeddingUncertified` on `embed/gate` carries the list, and the gate makes one finding per entry.
- **Part 1**, per the other report's tree notes:
  - `Outcome::Split` gains its reason.
  - `stuck` refuses when either part is unread for a structural reason.
  - `interval_sign_as` keeps Zero apart from Indeterminate.
  - `CUSP_DEPTH` retires.
  - A margin that is wide because the parameter box is wide refuses at once at `Interval` and `Sym`, which replaces `blocking_width`.

**Measurements**
- **M5:** the other report's version (per refusing pair, the depth at which a reason that no smaller cell changes first appears, and the splits after it). Part 1's saving and the removal of the first step turn on it.
- **M3 on a part 1 prototype:** whether the 38 crossings are still found once cells beside an unread edge refuse earlier.

**Off the question, still open:** on `origin/embed/gate`, `validate.rs` puts `embedding_refusal_words` between `linear_band`'s doc comment and `linear_band`. `GATE_MAX_CELL_PAIRS`'s doc ("does not depend on who asked") is stale since PR 7101.

**Not read:** `draw.md`.
