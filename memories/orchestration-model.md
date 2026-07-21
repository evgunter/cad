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
- **Self-merge escalation (Evan, 2026-07-16, PR #20)**: after M1 PRs 1–3
  earned trust, high-confidence design PRs (dominant-argument
  conventions, faithful elaborations of an already-ratified plan)
  **self-merge with their full writeups**; Evan reviews the backlog
  retroactively — "even when I have had a comment it's usually been
  something that is patchable, not fatal." **Fundamental design forks
  still wait for sign-off**: changes to ratified DESIGN.md decisions,
  and genuinely open questions with multiple viable answers where
  Evan's preference matters. When unsure which kind a decision is,
  treat it as a fork.
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

**Standing session-start checklist (made durable 2026-07-20; the
M2-LOG snapshots assume it):**
- Stale-monitor check, corrected (Evan, 2026-07-21): monitor
  processes do NOT outlast their session. The one observed case of
  "orphaned pollers" was a same-session continuation — Evan had run
  the `/clear` slash command, so the "new" orchestrator was the old
  session with its monitors still legitimately running. A successor
  created via tmux (fresh session) will never inherit pollers. So:
  if you may be a post-/clear continuation, check `ps aux | grep -E
  'gh api|events.jsonl'` and kill/reuse what you find; a fresh tmux
  orchestrator can skip the hunt. Then arm TWO persistent Monitors:
  1. **GitHub away-channel (refined by Evan, 2026-07-21)**: poll
     `gh api` for new issues + all issue/PR comments on the repo
     (~60s interval) — Evan may ask questions through comments when
     not in-session; expect the monitor to echo your own comments
     back (same account), ignore those. Outbound direction: status
     updates aren't wrong but Evan will likely MISS them — he only
     reviews comments he explicitly asked for, or on a thread he
     just used to ask a question (earlier sessions treated merged
     PR #41 as a standing status thread; those posts went unread).
     **Questions for Evan SHOULD go out via GitHub**: preferred
     form is a PR editing the relevant design doc to state the
     question, updated in place with the answer once resolved (the
     design-conversation-PR pattern); a GitHub issue also works.
  2. **Usage-limit watch**: tail the newest line of
     `~/.mngr/agents/<agent-id>/events/claude/usage/events.jsonl`
     (each line has `rate_limits.five_hour.used_percentage` and
     `.seven_day...`); alert at 90%/97%. The `mngr` CLI itself has
     been broken (azure plugin ImportError) — read the file
     directly. On a usage warning: flush state, write the handoff,
     notify Evan.
- On any warning-driven or planned handoff: commit log + memories +
  in-flight branch status per the stopping rule above.

**Operational lessons (M2, 2026-07-19/20):**
- **The 64k output-token-per-response limit kills agents that draft
  whole files in one Write** (or produce runaway derivations
  in-context). One implementer died 3× before the fix. Bake OUTPUT
  DISCIPLINE into every spec's header: ≤~150 lines per tool call;
  skeleton first, one function/test per Edit; split code across
  several source files; read big files chunked (offset/limit, ≤10-line
  distillation per chunk); break a growing response with a small tool
  call; derivations in scratchpad files; reports ≤150 dense lines.
  If an agent dies to this repeatedly, RESUME may replay the same
  giant response — kill it and respawn FRESH with the discipline in
  the spec (the poisoned transcript is the problem).
- Agents stopping "waiting on gate results" from a background chain
  may re-stop after a generic nudge — tell them explicitly: kill the
  chain, run each gate row as a separate SYNCHRONOUS foreground Bash
  call, read each result before the next.
- Finished agents can keep firing stale-waiter notifications; TaskStop
  them once their report is delivered.
- Convergent independent diagnosis (PR 4's implementer and PR 3's
  reviewer both finding the interval norm poison) is strong evidence;
  when two agents propose the same fix, have the second adopt the
  first's exact patch text to make the stack merge trivial.
- Mid-flight branch moves: when a reviewed branch gains commits,
  message the reviewer with precisely what changed and what to
  re-check; reviewers' executed-witness findings beat implementers'
  derivations (the powi(2) subnormal case) — record the resolution,
  scope the doc claim.

**Operational lessons (M1, 2026-07-16):**
- **Assign reviewers explicit claims to falsify** (not just "review
  this"): the falsification assignments caught a real doc
  self-contradiction (PR 4's re-make taxonomy — the "impossible"
  re-make existed), derived the corrected per-shell E–P invariant that
  became PR 5's spec, and exhaustively attacked the component pass.
  Essential for self-merging PRs where the review is the last gate.
- Reviewer suites are promoted into CI after each fix pass
  (independent derivations = regression value); details in
  [[review-and-dependency-policy]].
- Reference PDFs/notes go in the MAIN checkout's `references/` —
  git-ignored directories don't propagate across worktrees.
- Transient API-overload (529) kills background agents mid-task;
  resume via SendMessage (transcript + worktree survive), with
  exponential backoff between retries when the overload persists.
