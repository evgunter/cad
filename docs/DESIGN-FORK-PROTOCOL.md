# Design-fork review — protocol

Opened by Ev, in-chat, 2026-09-24. This file holds ONLY the protocol
in force now; it is edited in place when the protocol changes and
carries no history — git does. The rows live in
`docs/DESIGN-FORK-LOG.md`.

**The question: how do the designers' recommendations compare
with what Ev decides?** Every design fork put to Ev is first weighed
by one Opus and one Fable designer, and on a randomly drawn third of
forks by a second Opus designer as well
(`memories/orchestration-model.md`, `docs/prompts/designer.md`);
this log records what each recommended and, once Ev answers, what Ev
decided. This is process data, not a design reference: nothing here
binds kernel design.

**Changing the protocol** is Ev's call, and is a commit that edits
this file. Every row names the protocol it was recorded under by
COMMIT HASH: the last commit on main that touched this file when the
row is recorded (`git log -1 --format=%h origin/main --
docs/DESIGN-FORK-PROTOCOL.md`, after a fetch).

## Rules

1. **Population.** Every design fork that goes to Ev as an `[ev]` PR
   is a row. A sequencing decision, a file move, or any other `[ev]`
   PR that asks no design question is not.
2. **The designers.** One Opus and one Fable designer; on a fork
   drawn for it (rule 3), a second Opus designer too. All are
   dispatched CONCURRENTLY with the SAME statement of the problem (no
   candidate solutions) and the same pointer to
   `docs/prompts/designer.md`. None sees another's report until all
   are delivered. The third designer is a full participant: it is
   reconciled with the others and its `For Ev` section goes in the PR
   like theirs. A recommendation is still one the designers converged
   on, or a stated split; how many designers hold a position does not
   make it the recommendation.
3. **Draws and blinding.** At dispatch, two `/dev/urandom` bytes,
   both recorded. The first decides the arm: below 86 (about a third)
   → three designers, otherwise two. The second assigns the labels.
   With two designers: even → Opus is A, odd → Fable is A. With
   three: redraw while the byte is 255, then the byte mod 3 is
   Fable's label (0 → A, 1 → B, 2 → C) and the two Opus designers
   take the other two. The bytes and the mapping are committed at
   once to a branch under `analysis/design-fork/`, never to the
   `[ev]` PR's branch, so nothing Ev reads before deciding names a
   model: the PR body, the row and every reconciliation round use
   the labels only.
4. **Record the first reports before any reconciliation.** Each
   designer's first recommendation (answer, one line of argument,
   confidence, whether it rejected the framing, whether it proposed
   changing ratified text) goes in the row as delivered. Agreement is
   judged on the recommended final state, not the wording. A
   three-designer fork is a row in the log's second table, which also
   records each designer's token count as the harness reports it, for
   the first report and in total.
5. **Reconciliation.** When the designers disagree, the orchestrator
   may hand each the others' reports, and/or dispatch further designers to
   weigh them, until there is a clear recommendation or a clearly
   stated split. A round in which designers cross (each moves to
   another's previous position) does not end reconciliation: another
   round follows, each shown the others' revised reports. The row
   records each round: who was dispatched, what they were shown, and
   what moved; a further designer's model goes
   in the analysis-branch record, not the row, until Ev has decided.
   A further designer is not rule 2's third designer: the
   orchestrator dispatches one by its own judgement, in either arm.
6. **Record in two commits on the `[ev]` PR.** The recommendation
   half of the row (labels only) is committed when the PR opens,
   before Ev answers. Once Ev answers, a second commit fills in Ev's
   decision and the label→model mapping from the analysis branch,
   before merge.
7. **Readouts are off-file**, on the standing A/B rule: a directional
   result ("Ev usually sides with X") creates expectancy effects on how
   forks are framed and reconciled. They live on a branch under
   `analysis/design-fork/`; an orchestrator with a fork in flight
   should not read them.
