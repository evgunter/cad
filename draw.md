# Design fork `embed-refusal` — draw

Dispatched 2026-11-18 by the EMBED orchestrator, on `work/embed/a-refusing-ask-runs-on-to-its-whole-budget-looking-for-a-verified-crossing.md` (P1, M). Raised by the gate lane's census: 45% of the gate's measured price was refusing asks; the second take answered the row's owed evidence (38 asks end in a verified crossing after keeping a refusal, 5 s in all, against 11,434 s of refusing asks). Protocol `d327ffb520`.

- Arm byte **247** (≥ 86): two designers.
- Label byte **90** (even): **Opus is A, Fable is B**.

The problem statement every designer was handed is `problem.md`.

## First reports (2026-11-18)

Each designer saw only the problem statement (`problem.md`). Both reports were delivered on 2026-11-18 (A about 12:32Z, B about 12:39Z); the orchestrator was stopped by the account's Fable limit from 12:37Z that day until 2026-11-21, so they were recorded here on 2026-11-21, before any reconciliation and with nothing shown to either designer in between.

- **A:** make the face pair the unit an ask answers. A pair is settled by its first finding, a verified crossing or a refusal, and the walk drops that pair's remaining cells and goes on over the pairs still open; the gate lists one finding per face pair that does not hold; a budget run-out is one finding naming the pairs left open; the constant stays the one per-ask backstop of PR 7101. Defensible alternative: stop the whole ask at its first finding (cheapest, but a body that crosses can be reported as a kernel limit, and which finding is reported depends on face order). Today's behaviour (walk on, keep one refusal) is rejected: it pays for every finding and reports one, and treats crossings and refusals differently. Confidence: leaving today's state, sure; the pair over the stop, likely; the seconds each saves, unsure. Rejected the framing in part ("what an ask owes after its first refusal" is half the problem: the walk goes on and then folds). Proposes changing ratified text: one clarifying sentence in `[topology.tiers]`.
- **B:** the same final state: each face pair ends as apart, crosses or unread; the engine stops reading a pair at the first reading that settles it and goes on with every other pair; the gate lists each pair not apart. The same defensible alternative (the ask ends at its first crossing or refusal), the same rejection of today's rule, and a second rejection (a smaller budget for the search after a refusal: a dial for a search the engine does not run). Confidence: the pair over the stop, likely; today's run-on wrong under either, sure; the seconds, unsure. Rejected the framing: the seconds are not spent looking for a crossing (what runs after a refusal is the whole proof; crossings come from a cheap probe), and about 5,100 s of the first census's refusal seconds are asks whose only refusal is the budget, which no stopping rule touches. Proposes changing ratified text: `[topology.tiers]` (what the three facts are) and `ERROR-DESIGN` E7 (the trichotomy becomes per face pair for the embedding question).

**They agree on the recommended final state** (the face pair as the unit; one finding per pair; crossings and refusals alike settle a pair and neither ends the ask; the per-ask constant unchanged; the clearance assertion's single verdict kept), on the alternative and on what is rejected. They differ only in which ratified text they would touch (B adds E7) and in what they checked. Both name the same missing measurement: per refusing ask, the spend before the first refusal, after it in the refused pair, and after it in other pairs. **No reconciliation round is owed.** The measurement is with the gate lane.

Harness token counts (first report):
- A (Opus): 176,815
- B (Fable): 265,287

## Round 2 (2026-11-21, dispatched 21:08Z)

Not a reconciliation of a disagreement: the first reports agree. Both named one measurement as what would decide the seconds, and it came back cutting against the recommended option on the dearest asks, so each designer was shown the measurement (`round2/measurement.md`) and the other's first report, and asked what it moves, whether its own argument is answered, what the dearest asks' deep-refusing pairs are evidence of, and what M3 says about settling a pair by its first finding. Both were also pointed at `docs/prompts/designer.md`'s two paragraphs added on 2026-11-20. Same agents, resumed.

### Round 2 results (both in by 21:15Z)

Both moved, and both withdrew "a refusal settles its face pair".
- **A** now recommends ending the ask at its first finding, a verified crossing or a refusal, as the FINAL state, reporting that one finding: it saves 99.9% on the dearest asks where the pair saves 25.9%; no reader needs the list of unread pairs (each has the same recourse); and M3 shows the face pair is the wrong unit for a refusal (19 of 38 crossings lie in the pair that refused first). Beside it, as engine work and not design text: learning that a pair is unreadable should be cheap (the departure certificate answers "split, a smaller pair may certify" without grounds). Considered and not recommended yet: after a refusal, keep looking only for a crossing. Text: one sentence in `[topology.tiers]`; E7 unchanged. likely.
- **B** now recommends one final state in three parts: (1) the engine says why a local certificate failed and halves a cell only when a smaller cell can change that reason, otherwise refusing that cell at once, typed; (2) the answer is one fact per face pair, the gate listing every pair not apart; (3) a verified crossing settles its face pair, a refusal settles only the cell pair it was made on, and nothing stops an ask or a pair at a refusal. As a FIRST STEP until part 1 is built: stop the ask at its first refusal (one line, measured, comes out again). Names stopping-as-final as the alternative Ev may prefer, with what it leaves: a self-crossing body reported as a kernel limit in 28% of crossing asks. Text: `[topology.tiers]` and E7. likely.

**Agreed now:** today's rule is wrong; the seconds are in reaching refusals, not in running on after them; a refusal does not settle a face pair; stopping the ask is what removes the minutes-long asks today; the engine should make finding out that a cell cannot be read cheap. **Not agreed:** the final shape of the answer once refusals are cheap. A: one finding per ask, the stop is the final state, the list is not needed. B: one fact per face pair listed, no stop at a refusal, so that a crossing elsewhere is still reported. Tokens (cumulative, harness): A 197,768; B to be read off its notification.
