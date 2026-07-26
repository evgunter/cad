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
