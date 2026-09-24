# Design-fork review — protocol

Opened by Ev, in-chat, 2026-09-24. This file holds ONLY the protocol
in force now; it is edited in place when the protocol changes and
carries no history — git does. The rows live in
`docs/DESIGN-FORK-LOG.md`.

**The question: how do the two design reviewers' recommendations
compare with what Ev decides?** Every design fork put to Ev is first
weighed by one Opus and one Fable reviewer
(`memories/orchestration-model.md`, `docs/prompts/design-reviewer.md`);
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
2. **The pair.** One Opus and one Fable reviewer, dispatched
   CONCURRENTLY with the SAME statement of the question and the same
   pointer to `docs/prompts/design-reviewer.md`. Neither sees the
   other's report until both are delivered.
3. **Record the first reports before any reconciliation.** Each
   reviewer's first recommendation (option, one line of argument,
   confidence, and whether it rejected the framing) goes in the row as
   delivered. Agreement is judged on the recommended final state, not
   the wording.
4. **Reconciliation.** When the two disagree, the orchestrator may
   hand each the other's report, and/or dispatch further reviewers to
   weigh both, until there is a clear recommendation or a clearly
   stated split. The row records each round: who was dispatched (model),
   what they were shown, and what moved.
5. **Record in two commits on the `[ev]` PR.** The recommendation
   half of the row is committed when the PR opens, before Ev answers;
   Ev's decision is filled in once Ev answers, before merge. The
   PR body presents the recommendations; a readout does not.
6. **Readouts are off-file**, on the standing A/B rule: a directional
   result ("Ev usually sides with X") creates expectancy effects on how
   forks are framed and reconciled. They live on a branch under
   `analysis/design-fork/`; an orchestrator with a fork in flight
   should not read them.
