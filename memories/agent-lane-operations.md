---
name: agent-lane-operations
description: Consolidated agent-lane rules (2026-08-05, replacing six overlapping memories) — lane creation via scripts/new-lane.sh, disk/RAM budgets and cleanup via scripts/clean-lanes.sh + monitors, death recovery and resume-vs-fresh policy
metadata:
  type: project
---

Consolidates the former disk-watchdog, worktree-disk-hygiene,
clone-placement, subagent-death-recovery, hourly-agent-checkins,
and resume-vs-fresh-subagent memories (full incident narratives:
git history of those files, removed 2026-08-05). The rules, which
the committed scripts now largely enforce:

**Lane creation.** Working clones/worktrees NEVER go in the /tmp
session scratchpad (session-derived, silently reaped) — use
`~/.local/share/cad-work/<purpose>/` or the main checkout's
`.claude/worktrees/` (isolation subagents). Create lanes with
`scripts/new-lane.sh <lane> [branch]` — it sets `core.hooksPath`
so the committed pre-push hook (fmt-all --check) is active; a
hand-rolled `git clone` silently lacks it.

**Disk.** Each lane grows a 4–8 GB `target/`; never share
`CARGO_TARGET_DIR` across parallel builds (cargo's lock serializes
them); `~/.cache/gmp-mpfr-sys` IS shared safely. Session start:
arm `disk-watchdog.sh` (WARN <15G, CRITICAL <8G; install from repo
`scripts/monitors/` to `~/.local/share/cad-work/monitors/`, run
the installed copies). Under pressure: remove finished lanes with
`scripts/clean-lanes.sh [--dry-run]` (re-checks pushed/clean/
no-stash before each rm and refuses loudly); NEVER touch a running
gate's target; after a disk-full crash, purge torn binaries
(ELF-magic scan) and treat pressure-window test results as
suspect. Merged-branch worktrees are swept at every pipeline seam
(`git merge-base --is-ancestor` + clean status → remove); one
`git worktree remove` per Bash call (the permission classifier
blocks batch loops). Confirm the OWNING agent has terminated
before cleaning its lane.

**RAM.** 10 GB WSL2 ceiling (`.wslconfig`, confirmed 2026-07-25):
at most TWO parallel cargo lanes machine-wide. An OOM-killed test
shows as a bare "Terminated" single-row FAIL — check what else was
running and rerun quiet before diagnosing a code bug.

**Liveness.** Standing (Evan, 2026-07-24): check every running
lane at least hourly — arm `hourly-checkin.sh`; lost
wake-on-completion events are endemic, so nudge any lane idle
without a final report.

**Death recovery.** A dead subagent's transcript AND its isolation
worktree (with uncommitted work) survive — `git worktree list`
from the main checkout, then SendMessage resumes it. Choose
**fresh over resume** when the agent has been stalled for over an
hour AND the remaining work is small and fully specifiable from
pushed commits (a resume replays 300–400k tokens to do a 1k-token
job — Evan, 2026-07-29); resume only when the accumulated context
is genuinely useful (mid-design state, unreported findings,
unwritten judgment calls). Prevention: implementers
commit AND push after every coherent unit. **Resume resets cwd to
the orchestrator worktree**: every resumed command must carry
`cd <clone> && ...` in the same Bash call, and post-resume battery
claims are trusted only after verifying the transcript rows
carried the cd (green-but-invalid numbers otherwise).

**CONFLICTING = silent CI outage (norm ratified with Evan,
2026-08-06, from PR #218):** a PR that goes CONFLICTING against
main runs NO check runs at all — it looks like CI is absent, not
failing. Standing norm: every implementer brief and PR checklist
carries "merge origin/main immediately before opening the PR,
and re-merge whenever main moves while it is open"; after any
push, confirm checks actually STARTED (`gh pr checks` shows
rows). Orchestrator side: PR watchers treat CONFLICTING as a
loud failure (never wait on a conflicted PR), and the hourly
sweep checks open PRs' mergeable state, not just lane activity.
Binary/render conflicts are never hand-picked — take a side,
regenerate through the pipeline, re-verify the reproducibility
contract.
