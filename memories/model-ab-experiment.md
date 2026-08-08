---
name: model-ab-experiment
description: Standing experiment (Evan, 2026-07-25) — blocked-randomization Opus 5 vs Fable 5 for implementation dispatches; blinded reviewers; fixed code-quality rubric; protocol + data + readouts live in docs/MODEL-AB-LOG.md
metadata:
  type: feedback
---

**The protocol, rubric, running data table, and milestone readouts
all live in `docs/MODEL-AB-LOG.md` — that document is the single
normative source. Read it before dispatching an implementer.** This
memory only records that the experiment exists and the standing
instructions that bind agents outside the log's own text.

**Why:** Evan wants to know, now that Opus 5 is out, whether Opus
implementation produces more bugs / worse code than Fable at lower
cost — measured, not vibed ("with actual random numbers").

**Standing instructions:**

- Applies to implementation tasks dispatched after 2026-07-25.
  **Current protocol is v3 (2026-08-08): blocked randomization
  over TRIPLES {opus, opus, fable}** (unbiased urandom draw for
  fable's position — reject bytes ≥252, mod 3), pre-draw
  difficulty logging, blinded reviewers, fixed-rubric CODE
  QUALITY REPORT in every review, row recorded AT MERGE.
  In-flight v2 pair blocks completed as pairs. Details in the log.
- **Recording discipline (2026-08-08, from the bayes-analysis
  readout)**: tokens AND wall-clock recorded PER PHASE (impl /
  fix / review) at merge for every row, gaps annotated; ≥1 line
  of prose per MAJOR; "silent" = silent spec deviation only;
  record who ran the fix pass. Full text in the log's protocol
  section.
- **Blinding vs merge-only collision (2026-08-02, ruled)**: the
  harness's Co-Authored-By trailer NAMES THE MODEL — in an
  implementer lane that breaks blinding. Implementer briefs say "NO
  Co-Authored-By trailer in lane commits (blinding overrides the
  harness convention); if a model mention lands in a PUSHED commit,
  STOP and report to the orchestrator — never rewrite history
  yourself." Orchestrator commits keep the trailer (the
  orchestrator is not blinded).

**Readouts (conclusions live in the log, not here):** M4-close
(n=10) and M5-close (n=40, arms 15/15) both conclude NO EVIDENCE
either arm produces more bugs or worse code — the M5 sample is
large enough that a large effect would probably have shown; a small
one would not. Post-M5 rows (M6/M7) continue under their own
section header in the log. See [[orchestration-model]].
