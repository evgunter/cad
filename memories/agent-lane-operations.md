---
name: agent-lane-operations
description: Lane rules — creation via local-scripts/new-lane.sh, machine-wide build concurrency via with-build-slot.sh flock slots, disk budgets and cleanup, liveness, death recovery, resume-vs-fresh policy
metadata:
  type: project
---

The committed scripts enforce most of this; what follows is what a
script cannot say. Incidents that earned each rule are in git history
and the M-logs.

**Tooling split.** `scripts/` = what HOSTED CI runs; `local-scripts/` =
everything local (with-build-slot, ci-local, gate, test-fast, new-lane,
clean-lanes, fmt-all, setup-build-env, hooks/, monitors/). Every
workflow job does `rm -rf local-scripts` after checkout, so CI cannot
depend on them — which is what lets ci-filter.py treat local-scripts/
changes as non-triggering.

**Lane creation.** Working clones/worktrees NEVER go in the /tmp session
scratchpad (session-derived, silently reaped) — use
`~/.local/share/cad-work/<purpose>/` or the main checkout's
`.claude/worktrees/` (isolation subagents). Create lanes with
`local-scripts/new-lane.sh <lane> [branch]`: it sets `core.hooksPath` so
the committed pre-push fmt hook is active, and a hand-rolled `git clone`
silently lacks it. A dangling `core.hooksPath` warns loudly with the
one-liner fix. General lesson worth keeping: a repo-relative path cached
in per-clone git config is invisible to a repo-side rename — grep for
`git config` when moving directories.

**Disk.** Each lane grows a multi-GB `target/` (`disk-watchdog.sh`'s
header carries the current size, and the script its own WARN/CRITICAL
thresholds — read them there); never share `CARGO_TARGET_DIR` across
parallel builds (cargo's lock serializes them); `~/.cache/gmp-mpfr-sys`
IS shared safely. Session start: arm `disk-watchdog.sh` from the
installed monitor copies. Under pressure:
`local-scripts/clean-lanes.sh [--dry-run]`
(re-checks pushed/clean/no-stash before each rm and refuses loudly);
NEVER touch a running gate's target; confirm the OWNING agent has
terminated before cleaning its lane. After a disk-full crash, purge torn
binaries (ELF-magic scan) and treat pressure-window test results as
suspect. Sweep merged worktrees at every pipeline seam
(`git merge-base --is-ancestor` + clean status → remove), one
`git worktree remove` per Bash call — the permission classifier blocks
batch loops.

**Substrate outputs** must not share a parent with the lane clone that
cleanup is pointed at: put outputs at `cad-work/<name>-substrate/`, the
clone INSIDE it, and remove only the clone subdir at the seam.

**Build concurrency.** Bounded by the box's RAM ceiling, not by
taste. Heavy cargo operations are bounded machine-wide by
`local-scripts/with-build-slot.sh` — flock slot files in
`~/.local/share/cad-work/locks/`; flock releases on process death,
including SIGKILL/OOM. **Width is 1 (a mutex), measured not assumed**:
concurrent warm workspace rebuilds were measured slower than sequential
ones, and `-j` caps make it worse, so there is no jobs cap either
(PR #230). `CAD_SLOT_WIDTH=2` re-widens if hardware changes; batteries
then take ALL slots (`-x`), and two concurrent batteries are the
documented OOM shape. `ci-local.sh` (hence `gate.sh`) and `test-fast.sh`
self-acquire; wrap raw `cargo` invocations yourself.

- **Express lane** (`--express [SECS]`): short jobs (≤10 min declared
  budget) get their own slot with a self-enforcing timeout so they never
  starve behind a battery. Batteries and default jobs keep the main
  mutex. Its cost model is unverified — the leading suspect for
  pathological build waits is express-lane overlap with a main-slot
  build on a memory-tight box, ahead of any compiler flag.
- Choose `-n` (grab-or-exit-75, then retry) over the default blocking
  wait for long queues — a blocking wait can eat a Bash call's 10-min
  cap. Long rows that must survive the harness 590s timeout: launch
  under `setsid`, then poll the output file in the foreground.
- **Re-issuing a timed-out call means killing your own previous waiter
  first.** A harness-timed-out Bash call does NOT kill its flock waiter;
  the orphan stays queued and burns a slot turn when the mutex frees.
  Orchestrator sweeps cull same-command duplicate waiters per lane.
- **Kill by YOUR OWN recorded PIDs, never by pgrep pattern-matching a
  lane name** — pattern-matching both over- and under-kills, and trips
  the harness security policy. Record the PID when you launch it.
- **A lock can outlive its holder if a CHILD inherited the fd.** Any
  long-lived daemon spawned under a slot keeps the flock held, leaving a
  misleading dead-holder file. Diagnose with `fuser -v locks/<slot>.lock`;
  fix by killing the inheriting process. Slot-wrapped commands should
  not spawn daemons; if a cache/watcher daemon ever enters the build
  path, pre-start it before with-build-slot.sh opens its fds.
- An OOM-killed test shows as a bare "Terminated" single-row FAIL —
  check what else was running and rerun quiet before diagnosing a code
  bug.

**Lane-takeover courtesy.** When the orchestrator operates in a
possibly-alive agent's lane (pushing parked commits, merging its PR,
handing the lane to a successor), MESSAGE the incumbent first or
simultaneously — an unannounced takeover reads as a rogue actor from
inside the lane and costs the agent a diagnostic detour. The number of
ALIVE agents is not capped; only concurrent heavy cargo is.

**Liveness.** Check every running lane at least hourly (arm
`hourly-checkin.sh`) — lost wake-on-completion events are endemic, so
nudge any lane idle without a final report. **Transcript-mtime is NOT a
liveness signal**: an agent inside a blocking slot wait or a long
battery writes nothing for an hour while progressing normally, and a
queued nudge only drains at its next tool round, so "nudge queued, not
delivered" does not prove a wedge. Escalation order: nudge, then check
the LANE for real progress (process table, lock holders via `fuser`,
`target/` mtimes), and only then TaskStop after a full battery-length
window. A wrong TaskStop costs an interrupted battery whose rows must be
re-run untrusted.

**Waiter self-test.** Run a background waiter's detection expression
ONCE in the foreground before arming it. A catch-all retry arm
(`|| echo retry`) converts a permanent error into silent eternal
waiting.

**A restart loses the INBOX and keeps the WORKTREE**, and they then
disagree. A queued orchestrator message is delivered at the lane's next
tool round; a container restart before that round drops it, while the
uncommitted work done for it survives on disk. A lane wakes holding a
change with no traceable authority — and a diff caught MID-TRANSITION
between two lost instructions reads exactly like work nobody asked for,
where a coherent one would have read as somebody's finished intention.
One lane reported itself for fabricating a ruling from Evan on that
evidence; the rulings were real and both had arrived. **Revert first,
ask second, do not conclude**: *"I cannot find the authority"* is
evidence about the records, and after a restart the records are the
unreliable half.

**Death recovery.** A dead subagent's transcript AND its isolation
worktree (with uncommitted work) survive — `git worktree list` from the
main checkout, then SendMessage resumes it. Choose **fresh over resume**
when the agent has been stalled over an hour AND the remaining work is
small and fully specifiable from pushed commits (a resume replays
300–400k tokens to do a 1k-token job); resume only when the accumulated
context is genuinely useful — mid-design state, unreported findings,
unwritten judgment calls. Prevention: implementers commit AND push after
every coherent unit. **A resume resets cwd to the orchestrator
worktree**: every resumed command must carry `cd <clone> && ...` in the
same Bash call, and post-resume battery claims are trusted only after
verifying the transcript rows carried the cd.

**Merging is destructive to checks — four rules, each guarding a silent or
permanent failure rather than a red build.** Before merging, filter the check
runs (`gh api .../check-runs`): reject any `conclusion` that is not `success`,
**and separately confirm none is still in flight** — a check still running when
you merge dies at checkout, can never be re-run (a `pull_request` run cannot
re-checkout a merged ref), and its retry reproduces the failure, so it reads as
a defect forever. Confirm a *skip* is habitual by checking earlier green runs of
the same branch. Resolving a conflict where **both sides deleted** something:
take the union of the deletions, **derived from `main`**, never "keep both
sides" — keeping yours resurrects what another lane struck, and it looks clean.
After any resolution, grep the **whole tree** for conflict markers, not the file
you resolved: `git add -A && git commit --no-edit` on a merge stages a
conflicted file verbatim and needs no message, so nothing prompts. Then check
the post-condition **against the merged tree**, not against your diff — a row
you never touched cannot appear in your diff, which is why the diff cannot tell
you whether it should have been.

**CONFLICTING = silent CI outage.** A PR that goes CONFLICTING against
main runs NO check runs at all — it looks like CI is absent, not
failing. Every implementer brief and PR checklist carries "merge
origin/main immediately before opening the PR, and re-merge whenever
main moves while it is open"; after any push, confirm checks actually
STARTED by reading the workflow **runs** list, not the PR's checks list,
which cannot distinguish no run from a queued one. PR watchers treat
CONFLICTING as a loud failure.

**The session scratchpad is SHARED between concurrently running agents
of one session.** PR/issue bodies and anything else to-be-published go
to LANE-PRIVATE paths (`~/.local/share/cad-work/<lane>-*.md`), never the
scratchpad; orchestrator briefs state this.
