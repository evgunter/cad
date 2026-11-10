# Implementer model comparison — protocol

Opened by Ev, in-chat, 2026-11-10, the day after the Fable 5.2
release. This file holds ONLY the protocol in force now; it is edited
in place when the protocol changes and carries no history — git does.
The dated entries live in `docs/IMPL-MODEL-LOG.md`, the rows one file
each in `docs/impl-model-log/rows/`.

**The question: how do Opus and Fable implementers compare on
QUALITY, TIME and TOKEN COST?** It succeeds the concluded model A/B
(`docs/doc-ledger/model-ab-experiment-concludes.md`), whose instrument it reuses where noted, on
the current models (Opus 5.5, Fable 5.2) and under the protocol/log
split. Its rows are not pooled with that log's: different models, a
different population, a different instrument. This is process data,
not a design reference: nothing here binds kernel design.

**Changing the protocol** is Ev's call, and is a commit that edits
this file. Every row names the protocol it was recorded under by
COMMIT HASH: the last commit on main that touched this file when the
row is recorded (`git log -1 --format=%h origin/main --
docs/IMPL-MODEL-PROTOCOL.md`, after a fetch).

## Rules

1. **Population.** Every implementation unit the review tiers
   (`memories/orchestration-model.md`) send to a SINGLE or DUAL
   review enters. Units on the orchestrator's read do not, nor does
   anything that is not an implementer lane (designers, specs,
   rulings, reviews, the orchestrator's own work), which stays as
   `memories/orchestration-model.md` assigns it. Which tier a unit
   gets is decided there, before the draw, and never changed because
   of the arm.
2. **Spec first, then the draw.** The difficulty letter (H / M / L /
   S, as the dual-review protocol logs it) and the review tier are
   logged where the unit lives BEFORE the draw. At dispatch, one
   `/dev/urandom` byte: even → **Opus**, odd → **Fable**. The byte,
   the arm and the date are committed at once to a branch under
   `analysis/impl-model/`, never to the unit's branch. The ratio is
   1:1; changing it (the old log moved to 1:2 when the Fable usage
   limit bound first) is a protocol change.
3. **The arm is the implementer AND its fix pass.** The fix pass runs
   as the implementer's own agent, continued, so it inherits the arm.
   If the arm's agent is lost, the re-dispatch is on the SAME arm and
   the row records the loss; a re-dispatch on the other model, or a
   harness fallback to another model, makes the row unfair (rule 7).
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
5. **Blinding.** The reviewers are the instrument and are the same for
   both arms: Opus, under the brief and tier the unit would get
   anyway. Nothing a reviewer reads names or determines the arm:
   implementer briefs say "NO Co-Authored-By trailer in lane commits
   (blinding overrides the harness convention); if one lands in a
   PUSHED commit, note it in the PR body and CARRY ON — never rewrite
   history"; the implementation report is checked for self-naming
   before it reaches a reviewer; the PR body, the unit's log entry and
   any protocol warning stay arm-free until the row lands. Re-read the
   PR thread immediately before briefing a reviewer. A leak is
   flagged on the EXPOSURE, whether or not a reviewer says it noticed.
6. **What a row records.** One file,
   `docs/impl-model-log/rows/PR-<n>.md`, named by the unit's PR, with:
   - **protocol** hash; **unit** (program, item, PR); **letter**,
     **tier** and its one-line reason; **byte** and **arm**; every
     agent's model and version (rule 4).
   - **Quality.** Per review: verdict on the dual-review ladder
     (APPROVE · APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT);
     MAJOR / MINOR / NOTE counts; each MAJOR in one line, marked
     DEMONSTRATED (a red probe, a mutant gone red, a compile-fail pin,
     a red CI row, a measured wrong value) or BY-INSPECTION; spec
     deviations, reported vs SILENT (silent ones weighted worst); and
     the rubric — idiom/structure, test quality (do the tests pin the
     real contract?), doc/comment honesty — each rated 1–5 with one
     line of evidence. The reviewer brief asks for the rubric in the
     same words for both arms. Then: whether hosted CI was green on
     the implementer's first pushed head, and the fix pass's size
     (findings taken, lines changed).
   - **Time.** Wall-clock as the harness reports it, per phase:
     implementation (dispatch to report), fix pass, each review; and
     dispatch to merge. Waits the agent did not cause (a build-slot
     lock, a CI queue) are noted where known, not subtracted.
   - **Tokens.** As the harness reports them, PER PHASE —
     implementation, fix pass, each review — three or more figures,
     never one lump sum. A missing figure is written missing, never
     zero or estimated.
   - **Relaxations**, naming the arm (rule 7), and anything else
     unusual about the unit.
7. **Fair rows.** A change to one side's method — a narrowed scope, a
   lifted battery, a permitted substitution, a re-dispatch on the
   other model, an interrupted or truncated phase — is written into
   the row naming the side. A row whose implementer ran under a method
   the other arm would not have had is recorded in full and excluded
   from every count; the test is "would the other arm have been
   permitted the same". Unrecoverable figures are missing data, never
   zeros.
8. **Record at merge.** The row rides the unit's own PR as its LAST
   commit, after every review is delivered, since it names the arm and
   reviewers read `git log`; a missing field blocks the row. A
   dual-tier unit is a row here AND in `docs/DUAL-REVIEW-LOG.md`; the
   two are recorded independently.
9. **Later escapes.** When a defect is later traced to a unit in this
   log (an issue, a red row, a fix PR), append a line to its row
   naming it, the arm, and whether any review raised it.
10. **Readouts are off-file**, on the standing A/B rule: a directional
    result creates expectancy effects on triage, adjudication and
    dispatch. They live on a branch under `analysis/impl-model/`; an
    orchestrator with a dispatch in flight should not read them. A
    full readout runs when Ev asks for one; when ten fair rows per arm
    have landed since the last readout, the orchestrator recording the
    tenth tells Ev so, with no result.
