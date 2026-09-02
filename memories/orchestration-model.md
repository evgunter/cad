---
name: orchestration-model
description: How Evan wants implementation work run — top-level agent orchestrates and meta-reviews, subagents code and review; plus the standing operational rules for running lanes and channels
metadata:
  type: feedback
---

**Scope: the top-level (orchestrator) agent of a session only.** If you
were spawned as a subagent (implementer, reviewer, explorer), this
memory is not about you — just do your delegated task well.

Evan's standing instructions for implementation work:

- The top-level agent is **orchestrator and (meta-)reviewer**: central
  planning and design decisions stay with it; coding and reviews are
  delegated to subagents, which may spawn their own. Implementation
  model choice follows [[model-ab-experiment]]; design, specs, reviews
  and rulings stay Fable.
- **Continue autonomously** to the next genuine branch point. High-
  confidence design PRs (dominant-argument conventions, faithful
  elaborations of a ratified plan) self-merge with their full writeups;
  Evan reviews the backlog retroactively. **Fundamental design forks
  wait for sign-off**: changes to ratified DESIGN.md decisions, and open
  questions with several viable answers where Evan's preference matters.
  When unsure which kind a decision is, treat it as a fork.
- **Keep an orchestrator log of decisions made unilaterally** and keep
  state-of-work knowledge in version control.
- **Before stopping, commit all crucial state** (log updates, memories,
  in-flight branch status) so the next session can resume cold.

**Standing operational rules:**

- **Monitors are tools, not mandates (Evan)**: arm, tune, re-cadence or
  disarm any of them at will. The default at session start is
  `cp local-scripts/monitors/*.sh ~/.local/share/cad-work/monitors/`
  from an up-to-date checkout, then arm every script in the installed
  directory as a persistent Monitor — glob the directory, never
  maintain a named list. They stay separate scripts on purpose
  (independent failure domains, per-stream disarm).
- **Usage alerts: act only on ones naming YOUR OWN account.** Resolve it
  at session start from your own agent dir
  (`agent-<id>/plugin/claude/anthropic/.claude.json` →
  `oauthAccount.emailAddress`; the id is in your memory-directory path).
  `usage-watch.sh`'s events carry their own actions. Other accounts'
  alerts are informational — never act on them.
  **But your own account can cross a threshold while every alert names
  someone else** — read the truth rather than infer it from an event:
  `tail -1 <agent-dir>/events/claude/usage/events.jsonl` carries
  `rate_limits.five_hour` and `.seven_day` percentages together. A RESET
  event may name only the 7d window while the 5h one is still full
  (resuming on one and being STOP-NOW'd seconds later is how this was
  learned). One read settles both; do it before any expensive dispatch.
- **Away-channel arming**: the script fails loud (exit 78) without its
  routing env — `CAD_CHANNEL_SELF_TAG="(<ROLE> orchestrator)"
  CAD_CHANNEL_BRANCH_PREFIXES=<prefixes> bash .../github-away-channel.sh`.
  Per-comment events are scoped to your own threads; new-issue/PR events
  stay repo-wide. If you set `CAD_SIGNOFF_WATCHLIST` at arm time, append
  sign-off entries to THAT file.
- **Branch-prefix convention (Evan, #396)**: each program owns ONE short
  prefix — unit branches `foo/<unit>`, orchestrator branch
  `foo/orchestrator`, armed with `CAD_CHANNEL_BRANCH_PREFIXES=foo/`.
  Fold renames in at natural seams; no central legacy registry.
- **Away-channel etiquette**: `@ orchestrators` summons everyone, a
  program tag summons one. LEAD every comment with your role tag (it is
  both the thread subscription and the self-suppression key); to watch a
  thread your filter misses, post "(<ROLE> orchestrator) subscribing.";
  SIGN issue bodies you file.
- **Channel to Evan**: questions go out as design-conversation PRs (edit
  the doc to state the question, update in place with the answer) or
  issues — NEVER comments on merged PRs, he doesn't scan them. Watch 👍
  reactions only on comments you explicitly requested sign-off on.
- **State-sync records RIDE THE UNIT'S OWN PR (Evan, 2026-08-27)** — a
  unit's ledger row and log entries go on as one more commit to that
  unit's branch. Two conditions: **LAST, after both reviews are
  delivered** (the A/B row names the implementer's arm, and reviewers
  read `git log`), and **merge immediately without a fresh CI run** when
  the commit touches only docs/comments on an already-green head. This
  is for STATE-SYNC only: design conversations, protocol and memory
  amendments, spec ratifications and anything asking Evan a question get
  their OWN PR — burying those in a unit's merge hides exactly what
  other orchestrators should see. Keep PUSHING branches continuously;
  only the PR is batched.
  **PROPOSED REVISION — awaiting Evan (S-MESH/S-BOOL orchestrator,
  2026-09-02):** with several programs merging into main hourly, the
  row appended on the unit branch conflicted on the ledger tail at
  EVERY unit merge (MESH-3, BOOL-3, MESH-4 twice, BOOL-11 three times),
  and each re-merge of main onto a code-bearing branch cost a fresh CI
  cycle plus a sample-number correction. Revised shape, first used for
  MESH-6 (#1545 → row PR #1554, sample #101): the unit PR merges on its
  green head WITHOUT the row; the row and log entries land in a
  docs-only PR opened immediately after the merge, with the sample
  number known and no code head to re-gate. Both original conditions
  carry over (after both reviews are delivered; docs-only merges
  without a fresh CI run). The unit PR body stays the logical record;
  the row PR cites the unit PR. Until Evan rules, the S-MESH/S-BOOL
  units use the revised shape and say so in their rows.
- **Every implementer dispatch** points the lane at
  `docs/prompts/implementer-discipline.md` BY PATH (read it once
  yourself; do not paste it). Briefs carry BOTH halves of the
  foreground rule: never arm waiters or background chains for your own
  builds/tests, AND launch any job that outlives a 600 s foreground call
  `setsid`-detached, then poll it in the foreground — a harness-reaped
  background job is indistinguishable from a completed one
  ([[agent-lane-operations]]).
- **Reviews**: assign reviewers explicit claims to falsify AND point
  them at `docs/prompts/reviewer-style-lane.md` by path (dispatcher
  notes: `docs/REVIEW-STYLE-DISPATCH.md`) — the claims lane is strong on
  soundness and blind to structure. Promote reviewer suites into CI
  after the fix pass ([[review-and-dependency-policy]]).
- **A finding with no durable home cannot warn anyone.** At
  ADJUDICATION time, as part of reading a report, any finding asserting
  a CLASS rather than an instance gets a log line or an issue. A report
  that exists only in a session's context is one outage from never
  having happened.

Handing the session to a successor: [[orchestrator-switch-runbook]].
