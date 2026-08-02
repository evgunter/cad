---
name: model-ab-experiment
description: Standing experiment (Evan, 2026-07-25) — coin-flip Opus 5 vs Fable 5 for implementation dispatches; blinded reviewers; fixed code-quality rubric; log in docs/MODEL-AB-LOG.md
metadata:
  type: feedback
---

For every implementation dispatch, flip a fair coin from
/dev/urandom (e.g. `[ $(od -An -N1 -tu1 /dev/urandom) -lt 128 ]`):
heads = Opus 5 (`model: "opus"` on the Agent call), tails = Fable 5
(session default). Design, specs, adversarial reviews, and fix-pass
RULINGS stay Fable. The fix pass inherits the implementer's arm
(same agent).

**Why:** Evan wants to know, now that Opus 5 is out, whether Opus
implementation produces more bugs / worse code than Fable at lower
cost — measured, not vibed ("with actual random numbers").

**How to apply:** protocol + rubric + running data table live in
`docs/MODEL-AB-LOG.md` (read it before dispatching an implementer).
Non-negotiables: log the difficulty guess BEFORE flipping; never
reveal the arm to a reviewer (check implementation reports for
model mentions before handing them over); every review prompt must
request the fixed-rubric CODE QUALITY REPORT; fill the data row as
results arrive. Applies to implementation tasks dispatched after
2026-07-25 (PR 5's implementer predates it: Fable, excluded).
See [[orchestration-model]].

**Protocol v2 (Evan, 2026-07-25)**: blocked randomization replaces
the fair coin — opus/fable PAIRS, order shuffled per block
(/dev/urandom), after four consecutive fable draws left the
experiment opus-less. Block 1 forced (opus, fable) as a recorded
balance correction; random order from block 2. NEXT DISPATCH =
OPUS. Blinding and pre-flip difficulty logging unchanged.

**Blinding vs merge-only collision (2026-08-02, ruled)**: the
harness instructs agents to end commits with a Co-Authored-By
trailer that NAMES THE MODEL — in an implementer lane that
breaks blinding. One agent amended + force-pushed its own
seconds-old unshared tip to remove it (right goal, ratified-rule
violation; zero exposure, self-reported). Standing resolution:
implementer briefs say "NO Co-Authored-By trailer in lane
commits (blinding overrides the harness convention); if a model
mention lands in a PUSHED commit, STOP and report to the
orchestrator — never rewrite history yourself." Orchestrator
commits keep the trailer (the orchestrator is not blinded).

**M4-close readout (2026-07-27, n=10 rows, drafted for 8c)**: full
table in docs/MODEL-AB-LOG.md. Opus rows (5, 8, 9): zero
substantive MAJORs, rubric lines 4-5/5/5 incl. the milestone's
only zero-fix-pass unit and an upheld evidence-backed dispute of a
reviewer finding. Fable rows carried the two largest builds (PR 6,
#101) with more findings at higher absolute scope. Honest
conclusion at this n: NO EVIDENCE the Opus arm produces more bugs
or worse code; suggestive of parity; difficulty mix confounds any
stronger claim. Experiment CONTINUES into M5 (blocked pairs,
blinding, rubric unchanged) — the successor orchestrator runs it.
