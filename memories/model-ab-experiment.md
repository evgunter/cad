---
name: model-ab-experiment
description: The Opus-vs-Fable implementation experiment, SUSPENDED 2026-09-23 (everything on Opus, dual Opus reviews for the hard units) — docs/MODEL-AB-LOG.md is normative; this holds only what binds agents outside it
metadata:
  type: feedback
---

**SUSPENDED (Ev, 2026-09-23): everything runs on Opus — implementers,
reviewers, fix passes and the orchestrator's design, specs and
rulings. A unit that would have entered the protocol (v7's triage
test) gets TWO independent Opus reviewers on the same frozen head,
with the fix pass off the adjudicated union, and the pair is recorded
in `docs/DUAL-REVIEW-LOG.md` (normative for that stream; read it
before dispatching a dual). Every other unit keeps
its single Opus review or the orchestrator's own read. No draw, no
ordinal, no row, no blinding.** The log's suspension entry holds the
detail (in-flight units, bands, resuming). The rules below apply only
to a unit still finishing under the protocol, or once Ev resumes it.

**The protocol, rubric, running data and readout policy live in
`docs/MODEL-AB-LOG.md` — that document is the single normative source,
and every live number of the experiment (protocol version, block, slot,
ordinal band, dual tally, sample number) is claimed from it ON MAIN at
dispatch. Read it before dispatching an implementer.** No copy lives
here, in a plan, or in an M-log.

**Why:** Ev wants to know whether Opus implementation produces more
bugs / worse code than Fable at lower cost — measured, not vibed.

What binds outside the log's own text:

- **Not every unit enters the protocol.** Triage is per unit and the
  orchestrator's call at spec time; what a unit gets on either side of
  it, and how the call is recorded, are the log's v7 entry.
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
