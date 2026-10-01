---
name: experiments
description: The process experiments Ev runs on how work is done — which are live, where each one's normative log is, and the rules any experiment binds agents to
metadata:
  type: feedback
---

**An experiment is a standing process measurement Ev asked for** —
measured, not vibed. Each has two files in `docs/`, and no copy of
either lives here, in a plan, or in a program log:

- **`docs/<NAME>-PROTOCOL.md` — the protocol in force NOW**, edited in
  place with no history of its own (git keeps it). This is the one to
  read before dispatching.
- **`docs/<NAME>-LOG.md` — the rows**, each naming the protocol it was
  recorded under by COMMIT HASH — the last commit on main that touched
  the protocol file (Ev, 2026-09-23). The log is a named exemption in
  `scripts/work.py`'s `LOG_EXEMPT` and `work/README.md`.

Both leave `docs/` when the experiment concludes.

**Live:**

- **Dual Opus review concordance** — `docs/DUAL-REVIEW-PROTOCOL.md`
  (**read it before dispatching a dual**) and `docs/DUAL-REVIEW-LOG.md`.
  What a second independent review buys; every unit the review tiers
  ([[orchestration-model]]) send to a dual is a row.
- **Design-fork review** — `docs/DESIGN-FORK-PROTOCOL.md` (**read it
  before dispatching the pair**) and `docs/DESIGN-FORK-LOG.md`. What
  the Opus and Fable designers recommended, and what Ev then
  decided; every design fork put to Ev is a row.

**Suspended** (no need to read unless Ev reinstates it):

- **Model A/B** (Opus vs Fable implementation), suspended 2026-09-23 —
  `docs/MODEL-AB-LOG.md`, whose opening section holds the rules that
  bound agents outside it. It predates the protocol/log split: its
  protocol is the log's amendment history, so reinstating it starts by
  writing `docs/MODEL-AB-PROTOCOL.md` from the entries in force.

**What binds under any experiment:**

- **Which unit gets which treatment is not the experiment's to
  decide.** The review tiers in [[orchestration-model]] choose it; the
  experiment records what happened.
- **Readouts are not summarised anywhere agents read** (standing rule,
  Ev): a directional result creates expectancy effects on triage,
  adjudication and dispatch. They live on a branch under `analysis/`,
  and an orchestrator with a dispatch in flight should not read them.
- **A row rides the unit's own PR as its LAST commit**, after every
  review is delivered, and a missing field blocks the row. (A
  design-fork row rides its `[ev]` PR instead, in the two commits its
  protocol names.)
- **A change to one side's method is recorded, naming the side**, and
  a comparison whose sides ran under different methods is recorded
  but not counted.
- **Starting, changing or ending an experiment is Ev's call**; the log
  records it as a dated entry, and this memory's lists change with it.
