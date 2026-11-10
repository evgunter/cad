# Model comparison — protocol

Opened by Ev, in-chat, 2026-11-10, the day after the Fable 5.2
release. This file holds ONLY the protocol in force now; it is edited
in place when the protocol changes and carries no history — git does.
The dated entries live in `docs/MODEL-COMPARE-LOG.md`, the rows one
file each in `docs/model-compare-log/rows/`.

**The questions: how do Opus and Fable compare, on QUALITY, TIME and
TOKEN COST, (1) as IMPLEMENTERS and (2) as REVIEWERS?** The two are
measured in one population with two independent draws per unit, so
each factor is balanced across the other: a unit's review findings
depend on both who wrote the code and who read it, and only an
independent draw lets a readout separate the two. It succeeds the
concluded model A/B (`docs/doc-ledger/model-ab-experiment-concludes.md`),
whose instrument it reuses where noted, on the current models (Opus
5.5, Fable 5.2) and under the protocol/log split. Its rows are not
pooled with that log's: different models, a different population, a
different instrument. This is process data, not a design reference:
nothing here binds kernel design.

**Changing the protocol** is Ev's call, and is a commit that edits
this file. Every row names the protocol it was recorded under by
COMMIT HASH: the last commit on main that touched this file when the
row is recorded (`git log -1 --format=%h origin/main --
docs/MODEL-COMPARE-PROTOCOL.md`, after a fetch).

## Rules

1. **Population.** Every implementation unit the review tiers
   (`memories/orchestration-model.md`) send to a SINGLE or DUAL
   review enters. Units on the orchestrator's read do not, nor do
   designers, specs, rulings or the orchestrator's own work, which
   stay as `memories/orchestration-model.md` assigns them. Which tier
   a unit gets is decided there, before any draw, and never changed
   because of an arm.
2. **Spec first, then the draws.** The difficulty letter (H / M / L /
   S, as the dual-review protocol logs it) and the review tier are
   logged where the unit lives BEFORE either draw. Each draw is one
   `/dev/urandom` byte, even → **Opus**, odd → **Fable**, 1:1:
   - the **implementer arm**, at the implementer's dispatch;
   - the **reviewer arm**, at the review's dispatch, for a
     SINGLE-review unit only. A dual-tier unit's two reviewers stay
     Opus under `docs/DUAL-REVIEW-PROTOCOL.md`, whose concordance
     stream is Opus/Opus; such a unit is a row for the implementer
     question and enters no reviewer count.

   Each byte, its arm and the date are committed at once to a branch
   under `analysis/model-compare/`, never to the unit's branch.
   Changing a ratio (the old log moved implementers to 1:2 when the
   Fable usage limit bound first) is a protocol change.
3. **What an arm covers.** The implementer arm is the implementer AND
   its fix pass: the fix pass runs as the implementer's own agent,
   continued. The reviewer arm is the one review; a re-review after
   the fix pass, where the tier calls for one, is the same arm. If an
   arm's agent is lost, the re-dispatch is on the SAME arm and the row
   records the loss; a re-dispatch on the other model, or a harness
   fallback to another model, makes that arm's half of the row unfair
   (rule 8).
4. **Model version, always.** Every agent the row names — the
   implementer, the fix pass, each reviewer, the orchestrator —
   carries its model's exact version ("Opus 5.5", "Fable 5.2"), never
   the family name alone, as the harness reports it for that agent
   (a session's `get_session` gives
   `external_metadata.last_served_model`; a subagent's comes from its
   result or its definition). Where it cannot be known, write
   "unknown" with the dispatch date; never infer it. A release is an
   era boundary: a readout reports each model version separately, and
   a row whose agent ran across a release SPANS it.
5. **Blinding.**
   - **The reviewer never learns the implementer's model.**
     Implementer briefs say "NO Co-Authored-By trailer in lane commits
     (blinding overrides the harness convention); if one lands in a
     PUSHED commit, note it in the PR body and CARRY ON — never
     rewrite history"; the implementation report is checked for
     self-naming before it reaches a reviewer; the PR body, the unit's
     log entry and any protocol warning stay arm-free until the row
     lands. Re-read the PR thread immediately before briefing a
     reviewer. A leak is flagged on the EXPOSURE, whether or not a
     reviewer says it noticed.
   - **The review report does not name its model**, and reviewer lane
     commits carry no Co-Authored-By trailer either, so the fix pass
     reads findings, not a model. The orchestrator dispatches the
     review and so knows its arm: its adjudication is UNBLINDED, and
     the row says so. The reviewer question therefore leans on what
     does not pass through that judgement (rule 7).
   - **Every brief is identical across arms**: the implementer brief
     for both implementer arms, the reviewer brief — the same claims
     to falsify, the same pointer to
     `docs/prompts/reviewer-style-lane.md`, the same rubric — for both
     reviewer arms.
6. **What a row records.** One file,
   `docs/model-compare-log/rows/PR-<n>.md`, named by the unit's PR,
   with:
   - **protocol** hash; **unit** (program, item, PR); **letter**,
     **tier** and its one-line reason; each draw's **byte** and
     **arm**; every agent's model and version (rule 4).
   - **Quality of the implementation.** Per review: verdict on the
     dual-review ladder (APPROVE · APPROVE-WITH-FIXES ·
     NOT-MERGEABLE-AS-IS · REJECT); MAJOR / MINOR / NOTE counts; each
     MAJOR in one line, marked DEMONSTRATED (a red probe, a mutant gone
     red, a compile-fail pin, a red CI row, a measured wrong value) or
     BY-INSPECTION; spec deviations, reported vs SILENT (silent ones
     weighted worst); and the rubric — idiom/structure, test quality
     (do the tests pin the real contract?), doc/comment honesty — each
     rated 1–5 with one line of evidence. Then: whether hosted CI was
     green on the implementer's first pushed head, and the fix pass's
     size (findings taken, lines changed).
   - **Quality of the review** (rule 7): each finding's adjudication —
     TAKEN, ARGUED DOWN (with the reason) or DUPLICATE — and whether
     the review executed anything (probes, mutants, the battery) or
     read only.
   - **Time.** Wall-clock as the harness reports it, per phase:
     implementation (dispatch to report), fix pass, each review; and
     dispatch to merge. Waits the agent did not cause (a build-slot
     lock, a CI queue) are noted where known, not subtracted.
   - **Tokens.** As the harness reports them, PER PHASE —
     implementation, fix pass, each review — never one lump sum. A
     missing figure is written missing, never zero or estimated.
   - **Relaxations**, naming the agent and its arm (rule 8), and
     anything else unusual about the unit.
7. **Measuring a reviewer.** A review's quality is what it caught and
   what it cost. What it caught is read from the findings that are
   DEMONSTRATED, and from later escapes (rule 10): a defect traced to
   the unit that its review did not raise is that review's miss. The
   orchestrator's TAKEN / ARGUED DOWN calls are recorded but, being
   unblinded, are a secondary measure, and a readout says which it
   leans on.
8. **Fair rows.** A change to one side's method — a narrowed scope, a
   lifted battery, a permitted substitution, a re-dispatch on the
   other model, an interrupted or truncated phase — is written into
   the row naming the agent and its arm. A half of a row (implementer
   or reviewer) whose agent ran under a method the other arm would not
   have had is recorded in full and excluded from that question's
   counts; the test is "would the other arm have been permitted the
   same". Unrecoverable figures are missing data, never zeros.
9. **Record at merge.** The row rides the unit's own PR as its LAST
   commit, after every review is delivered, since it names the arms
   and reviewers read `git log`; a missing field blocks the row. A
   dual-tier unit is a row here AND in `docs/DUAL-REVIEW-LOG.md`; the
   two are recorded independently.
10. **Later escapes.** When a defect is later traced to a unit in this
    log (an issue, a red row, a fix PR), append a line to its row
    naming it, both arms, and whether any review raised it.
11. **Readouts are off-file**, on the standing A/B rule: a directional
    result creates expectancy effects on triage, adjudication and
    dispatch. They live on a branch under `analysis/model-compare/`;
    an orchestrator with a dispatch in flight should not read them. A
    full readout runs when Ev asks for one; when ten fair rows per arm
    of either question have landed since the last readout, the
    orchestrator recording the tenth tells Ev so, with no result.
