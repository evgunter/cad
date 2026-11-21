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
