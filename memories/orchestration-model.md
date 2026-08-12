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
  (M0's is `docs/archive/M0-LOG.md`, L-numbered decisions) and generally maintain
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

- **Orchestrator switch procedure (Evan, PR #285 comment
  5230318576, executed 2026-08-09)**: the outgoing orchestrator
  performs the switch itself — finalize the handoff file
  (cad-work/handoff-prompt-*.md) with the LIVE resting state incl.
  per-lane resume instructions for any in-flight agents (they die
  with the session; lanes survive pushed); push every lane's local
  commits (if a lane's own agent may be alive and holding git
  state, note it in the handoff instead of fighting the lock);
  commit crucial state; then `tmux split-window` a new pane IN THE
  SAME tmux session + same CLAUDE_CONFIG_DIR (keeps the login),
  launch `claude` (model fable), send-keys the handoff kickoff,
  verify the successor is working via capture-pane, and finally
  kill ONLY YOUR OWN PANE — never the session (the successor lives
  in it), never the other orchestrator's session.
- **Session start**: install + arm the scripted monitor suite —
  `cp local-scripts/monitors/*.sh ~/.local/share/cad-work/monitors/` from
  an up-to-date checkout, then arm each as a persistent Monitor
  from the INSTALLED copies (checkouts switch refs). The
  github-away-channel script bakes in both reaction endpoints
  (issues + pulls — inline-comment 👍s live under the pulls
  endpoint). **Comment filtering (Evan, 2026-08-11)**: the
  away-channel REQUIRES routing env at arm time (fail-loud —
  it exits 78 without it); per-comment events are scoped to your
  own threads, new-issue/PR events stay repo-wide. Arm as:
  `CAD_CHANNEL_SELF_TAG="(<ROLE> orchestrator)"
  CAD_CHANNEL_BRANCH_PREFIXES=<prefixes> bash .../github-away-channel.sh`.
  **Branch-prefix convention (an explicit rule, stated as the
  CLEAN example — Evan, #396):** each program owns ONE short
  prefix; every unit/lane branch goes under it and the
  orchestrator branch is `<prefix>orchestrator`. For a program
  with role tag `(FOO orchestrator)`: unit branches
  `foo/<unit>`, orchestrator branch `foo/orchestrator`, armed
  with `CAD_CHANNEL_BRANCH_PREFIXES=foo/` — one prefix, nothing
  to enumerate. Programs whose live branches predate this
  standardization arm with their actual prefix list (each
  session records its own in its handoff; do not maintain a
  central legacy registry here — it rots, as the seven-prefix
  kernel-program entry demonstrated) and fold renames in at
  natural seams; new programs use the clean shape from day one. **Canonical summons
  keywords (Evan)**: `@ orchestrators` reaches everyone;
  `@ lib` / `@ m8` / `@ asm` reach one (derived from the tag
  automatically — no ADDRESSES env needed normally). Two
  behavioral rules: (i) LEAD EVERY comment you post with your
  role tag — the leading tag is BOTH the thread subscription
  AND the self-suppression key (the away-channel drops comments
  that lead with your own tag: your echoes, since nobody else
  signs as you; mid-body mentions still summon); (ii) to
  watch a thread your filter doesn't match, post
  "(<ROLE> orchestrator) subscribing." on it — the tag in that
  comment subscribes you from the next poll; (iii) SIGN ISSUE
  BODIES with your tag when filing — the membership check reads
  title+body+comments, so a signed filing auto-subscribes you to
  its thread (and makes authorship visible across the shared
  account). Sign-off watchlist path:
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
- **Local clippy verification is only real from a COLD lint state
  for touched crates** (2026-08-08, #264's triple-red lesson):
  cargo replays cached diagnostics at their recording-time
  severity, so a target linted warm under an allow-warnings
  invocation passes a later `-D warnings` run un-relinted — a
  clean exit is a FALSE NEGATIVE. Brief line: verify with
  `cargo clean -p <touched-crates> && cargo clippy <CI's exact
  crate list> --all-targets -- -D warnings`.
- **Every subagent spec header**: OUTPUT DISCIPLINE (≤~150 lines
  per tool call, chunked reads, skeleton-first writes, reports
  ≤150 lines — the 64k output limit kills agents that draft whole
  files in one Write; a transcript poisoned by it must be respawned
  FRESH, not resumed) and the verbatim verification sentence: "run
  every build/battery row as a synchronous FOREGROUND Bash call,
  one at a time, reading each result before the next; NEVER arm
  waiters, monitors, or background chains for your own
  builds/tests; when the build-slot queue is busy, a BLOCKING
  foreground wait is the correct state — re-issue a timed-out
  call rather than parking" (waiter-parking is endemic without
  it; the slot-queue flavor — agents assuming the flock will
  notify them — recurred 3× on 2026-08-08 even with the shorter
  sentence).
- **Reviews**: assign reviewers explicit claims to falsify; promote
  reviewer suites into CI after the fix pass
  ([[review-and-dependency-policy]]). Dual-review sampling per the
  A/B v3 amendment: every 3rd merged BLINDED-LANE implementation
  row (both orchestrators' series combined, merge order on main)
  gets an independent R2 — same brief, own lane, no R1 access;
  fix pass consumes the adjudicated union.
- **Two standing brief lines (Evan, 2026-08-08)**: (i) k-lint
  discipline — "if the k-lint gate fires, do NOT change geometry
  to silence it; a fired lint is distribution evidence — re-derive
  the baseline per the K-REPORT runbook or escalate to the
  orchestrator" (his design, #243 comment 5224869607); (ii)
  comment style — comments state the INVARIANT, not the history:
  no retired-type archaeology, no unit tags (#245 nit → #251).
