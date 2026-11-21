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

**Agreed now:** today's rule is wrong; the seconds are in reaching refusals, not in running on after them; a refusal does not settle a face pair; stopping the ask is what removes the minutes-long asks today; the engine should make finding out that a cell cannot be read cheap. **Not agreed:** the final shape of the answer once refusals are cheap. A: one finding per ask, the stop is the final state, the list is not needed. B: one fact per face pair listed, no stop at a refusal, so that a crossing elsewhere is still reported. Tokens (cumulative, harness): A 197,768; B 318,104.

## Round 3 (2026-11-21, dispatched 21:16Z)

Each designer was shown the other's round-2 report and asked the question the orchestrator reads under the difference: what a person is owed when a body both crosses itself and has a place the engine cannot read, and whether anyone needs the list of unread pairs (as against the crossings elsewhere). A was asked whether B's part 1 (a typed reason; split only when a smaller cell can change it) changes "the ask ends at its first refusal" from a final state to one that holds only while refusals are dear. B was asked to apply the needs test to its parts 2 and 3, whether its final state survives part 1 saving less than expected, and whether anything that exists needs the per-pair answer. Both were asked for a state both would sign, or the difference in one sentence. The gate lane was asked for the measurement both named (why a touching cell pair splits, and whether a child fails as its parent did; whether a search-only continuation finds the 38).

### Round 3 results (both in by 21:19Z)

Both moved again, each toward the other's round-2 position.
- **A** withdrew "the ask ending at its first finding is the final state" (right only while refusals are dear) and took B's round-2 part 1 as the design. Final state: (1) a failed certificate carries its reason and a cell splits only when a smaller cell could change it; (2) the ask ends at its first verified crossing and at nothing else, walks past refusals, and with no crossing refuses listing every refusal it met. First step until (1) is built: stop at the first refusal. Asks Ev one thing: after a crossing, stop (its lean) or read on and list.
- **B** withdrew the per-face-pair list and the E7 change after applying the needs test (nothing that exists reads a per-pair answer; a person gets the next finding from the next finish). Recommends the ask ending at its first finding, crossing or refusal; files the ungrounded `Split` as a defect against `[one-way.readings]`, not a ruling; leaves open, for a measurement, whether a refusal still ends the ask once refusals are cheap (lean: go on, if it costs about what a holding body costs). Corrected its own round-2 "28% of crossing asks" (the denominator was the 22 tests' asks, not the suite's; suite-wide it is 38 against several thousand) and A's "every refusal has the same recourse" (a sliver, a torus that is not a ring and a wide parameter box are the model's).

They appear to have met: stop now; ground every split; go on past refusals once that is cheap and measured; stop at a crossing; no per-pair list. Tokens (cumulative): A 210,208; B 339,459.

## Round 4 (2026-11-21, dispatched 21:20Z)

Each designer was shown the other's round-3 report and `round4/joint.md`, the orchestrator's statement of what the two round-3 reports share (J1 stop now, injectivity only; J2 a split must carry its ground; J3 once that is built and measured, the ask ends only at a verified crossing and reports every refusal met; J4 stop at a crossing; J5 what both dropped; J6 the text), and asked to sign it, amend it, or state the difference in one sentence, and to write a final `For Ev` that does not misdescribe the other.

### Round 4 results (both in by 21:23Z): agreed

- **A** signs J1 to J6 as they stand. Recommends yes to (a) and ruling J3 now, conditionally; the text for J3 lands with the unit that meets the condition, and the condition stays out of `DESIGN.md`.
- **B** signs with three amendments: every refusal names the two faces of its cell pair (`Sliver` and `WitnessUnverified` carry none today, as `Budget` does not); J3's condition is the price alone (how many of the 38 are still found is reported, not a condition: stopping finds none); if Ev rules J3 now, the text is one change, the rule plus a named exception until the engine row (precedent: the at-rest row of `[one-way.places]`, Ev, PR 6371). B took back the list of refusals met under J3 (the walk has them in hand; some are the model's to move) and holds J4. Recommends yes to both.

**Reconciliation ends agreed**: stop now at the first finding (the injectivity question only); a split must carry its ground (a defect, filed and built, measured first); once a refusing ask costs about what a holding one does, the ask ends only at a verified crossing and reports every refusal it met; one crossing a run. The designers differ only in how the text carries J3 if Ev rules it now (A: the sentence changes with the unit; B: one change now with a named exception), which is put to Ev inside (b). Tokens (cumulative, harness): A 220,046; B 353,160. Four rounds.
