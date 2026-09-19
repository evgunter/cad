---
name: model-ab-experiment
description: The standing Opus-vs-Fable implementation experiment — docs/MODEL-AB-LOG.md is normative; this holds only what binds agents outside it
metadata:
  type: feedback
---

**The protocol, rubric, running data and readout policy live in
`docs/MODEL-AB-LOG.md` — that document is the single normative source,
and every live number of the experiment (protocol version, block, slot,
ordinal band, dual tally, sample number) is claimed from it ON MAIN at
dispatch. Read it before dispatching an implementer.** No copy lives
here, in a plan, or in an M-log.

**Why:** Ev wants to know whether Opus implementation produces more
bugs / worse code than Fable at lower cost — measured, not vibed.

What binds outside the log's own text:

- **Not every unit enters the protocol** (v7 triage, 2026-09-19): a
  unit runs the protocol only if its logic is especially tricky or it
  settles an architectural/design question that is broad or hard to
  change later; ambiguous units enter. Every other unit runs an OPUS
  implementer and an OPUS reviewer outside it — style review by
  default, full review where correctness bugs are a real possibility —
  with no draw, no ordinal and no row, and its tier named in the
  program's log at dispatch. Program-level review postures no longer
  decide this; the unit does.
- **The orchestrator's own model is a recorded field** on every row
  from 2026-09-19 — open at dispatch (it is not an implementer arm),
  blinded at analysis like the reviewer labels.
- **Blinding vs the merge-only convention**: the harness's
  Co-Authored-By trailer NAMES THE MODEL, so implementer briefs say
  "NO Co-Authored-By trailer in lane commits (blinding overrides the
  harness convention); if one lands in a PUSHED commit, note it in the
  PR body and CARRY ON — never rewrite history, never stop the unit
  over it." Orchestrator and reviewer commits keep the trailer.
- **Any reviewer-visible surface that names, or determines, an arm is a
  leak** — a unit's log entry, a block record naming unstarted slots
  (each remaining slot follows by arithmetic), and **a warning about a
  leak posted where the blinded party reads**. Route protocol warnings
  to a surface reviewers do not read, and re-read the PR thread
  immediately before briefing reviewers. Flag contamination on the
  EXPOSURE, not on whether a reviewer discloses noticing it.
- **Readouts are deliberately not summarised anywhere agents read**
  (standing rule, Ev): a directional arm result creates expectancy
  effects on difficulty logging, adjudication and dispatch sequencing.
  They live on branch `ev/ab-bayes-analysis` under `analysis/model-ab/`
  — **an orchestrator with a dispatch in flight should not read them.**

See [[orchestration-model]].
