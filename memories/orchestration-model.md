---
name: orchestration-model
description: How Evan wants implementation work run — top-level agent orchestrates and meta-reviews, subagents code and review; only applies to the session's top-level agent
metadata:
  type: feedback
---

**Scope: the top-level (orchestrator) agent of a session only.** If you
were spawned as a subagent (implementer, reviewer, explorer), this memory
is not about you — just do your delegated task well.

Evan's standing instructions for implementation work (given at M0 start,
2026-07-15):

- The top-level agent acts as **orchestrator and (meta-)reviewer**:
  central planning and design decisions stay with it; normal coding tasks
  are delegated to subagents (Opus for straightforward tasks, Fable for
  medium/hard ones), and reviews are delegated to subagents too.
  Subagents may spawn their own subagents for large tasks (verified —
  see [[multi-agent-capabilities]]).
- **Continue autonomously** until hitting a major branch point that needs
  Evan's input: automatically any change to a ratified DESIGN.md
  decision, plus important forks discovered during implementation.
  Design-conversation PRs (see [[git-workflow]]) are opened with full
  writeups and left for sign-off while work continues stacked on top.
- **Keep an orchestrator log of design decisions made unilaterally**
  (M0's is `docs/M0-LOG.md`, L-numbered decisions) and generally maintain
  state-of-work knowledge in version control.
- **Before stopping, write down and commit all crucial state** (log
  updates, memories, in-flight branch status) so the next session can
  resume cold.

**Why:** Evan wants throughput without losing the design-conversation
loop ([[cad-working-style]]) — delegation for speed, one accountable
place (the orchestrator + committed logs) for design coherence.

**How to apply:** at session start, read the current milestone's log for
in-flight state; delegate implementation with detailed specs citing
DESIGN.md decisions; review subagent output before merging; escalate to
Evan only at genuine design forks.

**Operational lessons (M0, 2026-07-16):**
- Background implementer agents sometimes stop while waiting on their
  *own* background tasks (long builds, clippy) — resume them with a
  SendMessage nudge telling them to check the task's status and finish
  in the FOREGROUND; instruct final verification steps to be foreground
  from the start.
- One implementer + one adversarial e2e reviewer + one fix pass per PR
  worked extremely well: reviews that *write and run real consumer
  programs* caught three correctness bugs code-reading missed
  (poison-laundering at a certification door, interval enclosure
  blowup from dependent multiplication, a false key-contract doc).
  Orchestrator writes the design doc BEFORE the implementer prompt and
  makes it binding; deviations must be reported, not improvised.
- Isolated-worktree agents: their branch stays checked out in their
  worktree after they finish — remove the worktree (or work inside it)
  before checking the branch out elsewhere; warm build caches in a
  finished agent's worktree are worth reusing for expensive deps
  (gmp: user-level ~/.cache/gmp-mpfr-sys survives across worktrees).
- A persistent gh-polling Monitor for PR comments/reviews/reactions
  makes phone-review loops with Evan fast; expect it to echo your own
  gh comments back (same account) — ignore those events.
- Design-PR conversations move fastest when replies include a firm
  recommendation, honest counterarguments, and an explicit "a 👍 here
  is enough to proceed" affordance.
