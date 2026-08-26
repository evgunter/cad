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
  Subagents may spawn their own subagents for large tasks.
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
  (L-numbered decisions, the shape M0's used) and generally maintain
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

**Standing operational rules** (the incidents that earned them live
in git history and the M-logs, not here):

- **Monitors are tools, not mandates (Evan, 2026-08-22)**: arm, tune,
  re-cadence, or disarm any of them at will to suit the session's
  present purposes. The default remains useful at session start —
  `cp local-scripts/monitors/*.sh ~/.local/share/cad-work/monitors/`
  from an up-to-date checkout, then arm the installed copies as
  persistent Monitors (checkouts switch refs; glob the directory, do
  not maintain a named list) — but it is a default, not an
  obligation.
- **Away-channel arming**: the script fails loud (exit 78) without its
  routing env. Arm as `CAD_CHANNEL_SELF_TAG="(<ROLE> orchestrator)"
  CAD_CHANNEL_BRANCH_PREFIXES=<prefixes> bash .../github-away-channel.sh`.
  Per-comment events are scoped to your own threads; new-issue/PR
  events stay repo-wide. If you set `CAD_SIGNOFF_WATCHLIST` at arm
  time, append sign-off entries to THAT file — not the default
  `~/.local/share/cad-work/signoff-watchlist.txt`.
- **Branch-prefix convention (Evan, #396)**: each program owns ONE
  short prefix — unit branches `foo/<unit>`, orchestrator branch
  `foo/orchestrator`, armed with `CAD_CHANNEL_BRANCH_PREFIXES=foo/`.
  Programs whose branches predate this arm with their actual prefix
  list and record it in their own handoff; fold renames in at natural
  seams. No central legacy registry — it rots.
- **Away-channel etiquette**: `@ orchestrators` summons everyone,
  `@ lib` / `@ asm` / `@ m9` summon one. (i) LEAD every comment with
  your role tag — it is both the thread subscription and the
  self-suppression key; (ii) to watch a thread your filter misses,
  post "(<ROLE> orchestrator) subscribing." on it; (iii) SIGN issue
  bodies you file, which auto-subscribes you and makes authorship
  visible on the shared account.
- **Channel to Evan**: questions go out as design-conversation PRs
  (edit the doc to state the question, update in place with the
  answer) or issues — NEVER comments on merged PRs, he doesn't scan
  them. Watch 👍 reactions only on comments you explicitly requested
  sign-off on.
- **State-sync PRs (Evan, #96)**: the orchestrator branch must not
  accumulate a large unmerged delta — open a docs-only PR to main at
  every pipeline seam.
- **Every implementer dispatch**: point the lane at
  `docs/prompts/implementer-discipline.md` by path — output discipline,
  CI-first verification (local runs are an iteration tool, not the
  record), per-lane target dirs, k-lint and comment style live there. Read it once yourself; do not paste it.
- **Reviews**: assign reviewers explicit claims to falsify, AND point
  them at the style lane by path (`docs/prompts/reviewer-style-lane.md` — read it
  once yourself, do not paste it; dispatcher notes in
  `docs/REVIEW-STYLE-DISPATCH.md`) — the claims lane is
  strong on soundness and blind to structure; promote
  reviewer suites into CI after the fix pass
  ([[review-and-dependency-policy]]). Dual-review sampling per the A/B
  amendment in `docs/MODEL-AB-LOG.md`, which owns the ordinal.
- **A finding with no durable home cannot warn anyone (2026-08-26,
  #1023; adopted on the VERBS side the same day)**: at ADJUDICATION
  time — as part of reading a report, not later — any finding that
  asserts a CLASS rather than an instance gets a durable home: a log
  line or an issue. Two instances bought this rule on one day. A
  banked finding that PREDICTED #1023's defect class lived only in an
  implementer's report transcript, so when the class fired the
  citation trail broke at first use and the warning had protected
  nothing. Separately, M9-3's dual reviews (ordinal 72) were
  delivered, adjudicated, and then LOST with the orchestrator session
  that held them — the residue issues survived, so the reviews are
  attested, but every verdict label, finding count, rubric score and
  per-phase figure is gone and the ledger row records missing data
  instead. Corollary for reviews specifically: a report that only
  ever exists in a session's context is one outage from never having
  happened.

Handing the session to a successor: [[orchestrator-switch-runbook]].
