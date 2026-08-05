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
  are delegated to subagents, and reviews are delegated to subagents
  too. (Model choice for implementation dispatches is currently
  governed by the coin-flip A/B protocol — see [[model-ab-experiment]];
  design/specs/reviews/rulings stay Fable.)
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

**Standing operational rules (compressed 2026-08-05 by the docs-rot
unit; the incident narratives that earned them live in this file's
git history and the M-logs):**

- **Session start**: install + arm the scripted monitor suite —
  `cp scripts/monitors/*.sh ~/.local/share/cad-work/monitors/` from
  an up-to-date checkout, then arm each as a persistent Monitor
  from the INSTALLED copies (checkouts switch refs). The
  github-away-channel script bakes in both reaction endpoints
  (issues + pulls — inline-comment 👍s live under the pulls
  endpoint). Sign-off watchlist path:
  `~/.local/share/cad-work/signoff-watchlist-m7.txt` (per the
  sole-orchestrator wind-down). No usage-limit monitor (dropped,
  Evan 2026-07-23) — the stopping rule covers it.
- **Channel to Evan**: questions go out via GitHub as
  design-conversation PRs (edit the doc to state the question,
  update in place with the answer) or issues — NEVER comments on
  merged PRs (he doesn't scan them). Watch 👍 reactions only on
  comments you explicitly requested sign-off on (watchlist file).
- **State-sync PRs (Evan, #96)**: the orchestrator branch must not
  accumulate a large unmerged delta — open a docs-only PR to main
  at every pipeline seam.
- **Every subagent spec header**: OUTPUT DISCIPLINE (≤~150 lines
  per tool call, chunked reads, skeleton-first writes, reports
  ≤150 lines — the 64k output limit kills agents that draft whole
  files in one Write; a transcript poisoned by it must be respawned
  FRESH, not resumed) and the verbatim verification sentence: "run
  every build/battery row as a synchronous FOREGROUND Bash call,
  one at a time, reading each result before the next; NEVER arm
  waiters, monitors, or background chains for your own
  builds/tests" (waiter-parking is endemic without it).
- **Reviews**: assign reviewers explicit claims to falsify; promote
  reviewer suites into CI after the fix pass
  ([[review-and-dependency-policy]]).
