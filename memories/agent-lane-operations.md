---
name: agent-lane-operations
description: Consolidated agent-lane rules (2026-08-05, replacing six overlapping memories; build-slot locks added 2026-08-06) — lane creation via local-scripts/new-lane.sh, machine-wide build concurrency via local-scripts/with-build-slot.sh flock slots, disk budgets and cleanup via local-scripts/clean-lanes.sh + monitors, death recovery and resume-vs-fresh policy
metadata:
  type: project
---

Consolidates the former disk-watchdog, worktree-disk-hygiene,
clone-placement, subagent-death-recovery, hourly-agent-checkins,
and resume-vs-fresh-subagent memories (full incident narratives:
git history of those files, removed 2026-08-05). The rules, which
the committed scripts now largely enforce:

**Tooling split (2026-08-11).** `scripts/` = the six things HOSTED CI
runs; `local-scripts/` = everything local (with-build-slot, ci-local,
gate, test-fast, new-lane, clean-lanes, fmt-all, setup-build-env,
hooks/, monitors/). Every workflow job does `rm -rf local-scripts`
after checkout, so CI CANNOT depend on them — which is what lets
ci-filter.py treat local-scripts/ changes as non-triggering (they used
to force the full matrix). Existing lanes had the OLD hooks path
cached in .git/config, so the rename silently disabled their pre-push
fmt hook (git says NOTHING when core.hooksPath is missing) — it hit the
build-perf lane itself. with-build-slot.sh now REPAIRS a dangling
core.hooksPath on the next build and says so, so no manual step is
needed — a MIGRATION SHIM, **RETIRE 2026-08-13** (grep
`RETIRE 2026-08-13`; it nags on every acquisition past that date). General lesson: a repo-relative path cached in per-clone git
config is invisible to a repo-side rename — grep for `git config` when
moving directories. See docs/LOCAL-BUILD-PERF.md §6.

**Lane creation.** Working clones/worktrees NEVER go in the /tmp
session scratchpad (session-derived, silently reaped) — use
`~/.local/share/cad-work/<purpose>/` or the main checkout's
`.claude/worktrees/` (isolation subagents). Create lanes with
`local-scripts/new-lane.sh <lane> [branch]` — it sets `core.hooksPath`
so the committed pre-push hook (fmt-all --check) is active; a
hand-rolled `git clone` silently lacks it.

**Disk.** Each lane grows a 4–8 GB `target/`; never share
`CARGO_TARGET_DIR` across parallel builds (cargo's lock serializes
them); `~/.cache/gmp-mpfr-sys` IS shared safely. Session start:
arm `disk-watchdog.sh` (WARN <15G, CRITICAL <8G; install from repo
`local-scripts/monitors/` to `~/.local/share/cad-work/monitors/`, run
the installed copies). Under pressure: remove finished lanes with
`local-scripts/clean-lanes.sh [--dry-run]` (re-checks pushed/clean/
no-stash before each rm and refuses loudly); NEVER touch a running
gate's target; after a disk-full crash, purge torn binaries
(ELF-magic scan) and treat pressure-window test results as
suspect. Merged-branch worktrees are swept at every pipeline seam
(`git merge-base --is-ancestor` + clean status → remove); one
`git worktree remove` per Bash call (the permission classifier
blocks batch loops). Confirm the OWNING agent has terminated
before cleaning its lane.

**RAM / build concurrency (locks since 2026-08-06, replacing the
soft two-lane convention and the cad-work/cargo-slots.txt
registry).** 10 GB WSL2 ceiling (`.wslconfig`, confirmed
2026-07-25). Heavy cargo operations are bounded machine-wide by
`local-scripts/with-build-slot.sh` — flock slot files in
`~/.local/share/cad-work/locks/`. flock releases on process death
(even SIGKILL/OOM), so dead agents cannot leave stale locks.
**Width is 1 (a mutex), measured not assumed** — the 2026-08-06
experiment (PR #230; cad-work/slot-exp-results.md): concurrent
warm workspace rebuilds 98s pair-wall (-j8) / 111s (-j4) vs 69s
sequential — concurrency loses ~40% to cache/membw contention,
and -j caps make it worse (solo -j4 52s vs -j8 33s), so no jobs
cap either; RAM was never tight (min 5.5 GB avail). Numbers are
post-laptop-settings-fix (Evan, 2026-08-06 — pre-fix timing
folklore is stale). CAD_SLOT_WIDTH=2 re-widens if hardware
changes; batteries then take ALL slots (`-x`) — two concurrent
batteries are the documented OOM shape. `ci-local.sh` (hence gate.sh) and
`test-fast.sh` self-acquire, so the standard entry points queue
automatically; wrap raw `cargo` invocations yourself. **Express lane (#269,
2026-08-09)**: short jobs (≤10 min declared budget) use
`with-build-slot.sh --express [SECS]` — own slot, self-enforcing
timeout, never starves behind a battery; batteries and default
jobs keep the main mutex. Holder prints now verify PID liveness
and show hold duration (#235 fixed). Long rows that must survive
the harness 590s timeout: launch under setsid, then poll the
output file foreground. Agents
choose `-n` (grab-or-exit-75, then retry/fall back) vs default
blocking wait (`-w SECS` caps it) — a blocking wait can eat a Bash
call's 10-min cap, so briefs should prefer `-n` + retry for long
queues. **Orphan-waiter stacking (2026-08-09, observed live)**: a
harness-timed-out Bash call does NOT kill its with-build-slot
flock waiter — the orphan stays queued, and "re-issue the
timed-out call" then STACKS duplicate waiters that each burn a
slot turn when the mutex frees (5 deep observed). Re-issue means:
kill your own previous waiter first (or use `-n`/`--express`);
orchestrator sweeps should scan for same-command duplicate
waiters per lane and cull all but the newest. **Kill targets are
identified by YOUR OWN recorded PIDs/job ids — NEVER by pgrep
pattern-matching a lane name** (2026-08-11: an agent
pattern-killed two shell PIDs, tripped the harness security
policy, and still MISSED its actual zombie holder, which the
orchestrator had to reap; pattern-matching both over- and
under-kills). Record the PID when you launch; kill that. **fd-inheritance lock leak
(2026-08-11, observed live)**: flock-releases-on-death is only true
if no CHILD inherited the lock fd — a daemon spawned under a slot
(sccache observed; any long-lived child qualifies) keeps the flock
held after the recorded holder dies, wedging the lane with a
misleading dead-holder file. Diagnose with
`fuser -v locks/<slot>.lock` (shows the true fd holders); the fix
is killing the inheriting process, and slot-wrapped commands
should avoid spawning daemons (sccache/watchers) or close the fd
(`flock -o` where supported). sccache was briefly the machine
rustc-wrapper on 2026-08-11 and needed exactly that guard; it was
reverted the same day (docs/LOCAL-BUILD-PERF.md), so the guard went with
it — if a cache/watcher daemon is ever added to the build path, pre-start
it before with-build-slot.sh opens its fds. **Express-lane cost model
is UNVERIFIED and suspect (2026-08-11)**: the same cold workspace build
measured **69m23s** in one window and **3m08s** in another — same
config, same tree, 182-197 crates both times, 22x apart. The slow
window had express-lane jobs (clippy, `cargo test`, the python suite, a
`pncad-py` build) running ALONGSIDE the main-slot build; the fast one
did not. #269 sized the express lane off #230's "concurrency costs
~40%", but #230 measured two BUILDS on a box that was never
memory-tight, whereas 10 GB with full-DWARF link jobs can cross into
swap, where the penalty is nonlinear. If build waits feel pathological,
suspect express-lane overlap BEFORE compiler flags — config knobs moved
single-digit percents here against a 22x environmental term. Needs a
#230-style measurement (express job concurrent with a battery, memory
sampled) before the lane is resized or kept. **Lane-takeover
courtesy (2026-08-10)**: when the orchestrator operates in a
possibly-alive agent's lane (pushing its parked commits, merging
its PR, or handing the lane to a successor), MESSAGE the incumbent
first (or simultaneously) — an unannounced takeover reads as a
rogue actor from inside the lane and costs the agent a diagnostic
detour (observed: the M8-3 PR-1 finisher escalated a
"lane-ownership violation" that was three legitimate orchestrator
actions plus its own successor). The number of ALIVE agents is no longer capped at two —
only concurrent heavy cargo is; more than two lanes may exist if
disk allows. An OOM-killed test still shows as a bare "Terminated"
single-row FAIL — check what else was running and rerun quiet
before diagnosing a code bug.

**Liveness.** Standing (Evan, 2026-07-24): check every running
lane at least hourly — arm `hourly-checkin.sh`; lost
wake-on-completion events are endemic, so nudge any lane idle
without a final report. **Transcript-mtime is NOT a liveness
signal for long batteries (2026-08-11, observed live)**: an
agent inside a blocking slot wait / long battery writes nothing
for an hour while progressing normally, and a queued nudge only
drains at its next tool round — "nudge queued, not delivered"
therefore does NOT prove a wedge. Escalation order: nudge, then
WAIT for at least one full battery-length window (60+ min) after
the nudge before TaskStop; check the LANE (process table, lock
holders via fuser, target/ mtimes) for signs of real progress
first. A wrong TaskStop costs an interrupted battery whose rows
must be re-run untrusted (one R2 review stopped mid-battery this
way — the reply "mid-battery, progressing normally" surfaced
with the kill).

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
**Substrate-output placement (lesson 2026-08-06):** clean-lanes
on a substrate dir's PARENT deletes the inventory beside the
lane (m6-5's inventory was lost this way; the implementer
re-derived). Substrate outputs and the lane clone must not share
a parent that gets passed to cleanup — put outputs at
cad-work/<name>-substrate/ and the clone INSIDE it, then remove
only the clone subdir at the seam.
