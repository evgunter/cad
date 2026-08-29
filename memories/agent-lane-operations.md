---
name: agent-lane-operations
description: Lane rules — creation, machine-wide build slots, disk, liveness, death recovery, and the ways CI silently does not run
metadata:
  type: project
---

The committed scripts enforce most of this; what follows is what a
script cannot say.

**Tooling split.** `scripts/` = what HOSTED CI runs; `local-scripts/` =
everything local. Every workflow job does `rm -rf local-scripts` after
checkout, so CI cannot depend on them — which is what lets ci-filter.py
treat `local-scripts/` changes as non-triggering.

**Lane creation.** Working clones and worktrees NEVER go in the /tmp
session scratchpad (session-derived, silently reaped) — use
`~/.local/share/cad-work/<purpose>/` or the main checkout's
`.claude/worktrees/`. Create lanes with `local-scripts/new-lane.sh
<lane> [branch]`: it sets `core.hooksPath` for the committed pre-push
fmt hook, which a hand-rolled `git clone` silently lacks. Substrate
outputs must not share a parent with the clone that cleanup is pointed
at: outputs at `cad-work/<name>-substrate/`, the clone INSIDE it,
remove only the clone at the seam.

**Disk.** Each lane grows a multi-GB `target/`; never share
`CARGO_TARGET_DIR` across parallel builds (cargo's lock serializes
them); `~/.cache/gmp-mpfr-sys` IS shared safely. `disk-watchdog.sh`
carries its own thresholds — read them there. Under pressure,
`local-scripts/clean-lanes.sh [--dry-run]` re-checks pushed/clean/no-
stash before each rm and refuses loudly, so it is safe in bulk — but it
needs **absolute paths** (a bare lane name is refused with a message
that reads like a missing directory). Never touch a running gate's
target, and confirm the OWNING agent has terminated first. After a
disk-full crash, purge torn binaries (ELF-magic scan) and treat
pressure-window test results as suspect. Sweep merged worktrees at
every pipeline seam, one `git worktree remove` per Bash call — the
permission classifier blocks batch loops.

**Reclaiming a finished lane is the ORCHESTRATOR's job**, not a lane's:
only the orchestrator knows which agents have reported, and a lane
cannot judge whether a sibling directory is live. Do it **when a review
returns**, not when a lane runs out of disk — a review lane's `target/`
is pure waste the moment its report is in hand, and review lanes are
the biggest consumers.

**Build concurrency.** Heavy cargo operations are bounded machine-wide
by `local-scripts/with-build-slot.sh` — flock slot files under
`~/.local/share/cad-work/locks/`, released on process death including
SIGKILL/OOM. **Width is 1 (a mutex), measured not assumed**: concurrent
warm workspace rebuilds are slower than sequential ones, and `-j` caps
make it worse, so there is no jobs cap either. `CAD_SLOT_WIDTH=2`
re-widens if the hardware changes; batteries then take ALL slots
(`-x`), and two concurrent batteries are the documented OOM shape.
`ci-local.sh` (hence `gate.sh`) and `test-fast.sh` self-acquire; wrap
raw `cargo` invocations yourself.

- **Express lane** (`--express [SECS]`): jobs under a ~10 min declared
  budget get their own slot with a self-enforcing timeout so they never
  starve behind a battery. Its cost model is unverified — express/main
  overlap on a memory-tight box is the leading suspect for pathological
  build waits, ahead of any compiler flag.
- Prefer `-n` (grab-or-exit-75, then retry) over the default blocking
  wait for long queues; a blocking wait can eat a Bash call's cap.
- **Kill a detached job the moment its evidence is SUPERSEDED**, not at
  its natural end — a still-running job holds the mutex against every
  queued lane. RECORD THE PID AT LAUNCH so stopping it is a one-liner;
  kill children first so none inherits the lock fd, then VERIFY the
  release (`fuser -v` on the lock): "parent dead, lock still held"
  looks identical to success from outside.
- **Kill by YOUR OWN recorded PIDs, never by pgrep pattern-matching a
  lane name** — it both over- and under-kills and trips the harness
  security policy.
- **A lock can outlive its holder if a CHILD inherited the fd.** Any
  long-lived daemon spawned under a slot keeps the flock held.
  Slot-wrapped commands must not spawn daemons; pre-start any
  cache/watcher daemon before `with-build-slot.sh` opens its fds.
- **An `-x` waiter is STARVED by single-slot arrivals**: exclusive mode
  needs ALL slots, flock has no queue or priority, and grabbers
  arriving after the waiter is armed still win. A courtesy window for
  an `-x` job means a MACHINE-WIDE quiet period, not pausing one lane.
  Waiting harder never wins; meanwhile the blocked lane can still push
  and open its PR, since hosted CI needs no local slot.
- **Re-issuing a timed-out call means killing your own previous waiter
  first** — a harness-timed-out Bash call does not kill its flock
  waiter, and the orphan burns a slot turn when the mutex frees.
- **Never pipe a slot-wrapped command through `| tail`/`| head`** — the
  pipe buffers the wrapper's progress lines away, so a live wait is
  indistinguishable from a hang and you kill a healthy waiter.
- **The ORCHESTRATOR's own worktree grows a stale `target/` too**, and a
  lane sweep cannot see it because it is not a lane. Mine held 3.0G
  untouched for two weeks (the orchestrator builds in lanes, never at
  home) — more than every idle lane combined. Check
  `~/.mngr/worktrees/<yours>/target` BEFORE sweeping another program's
  live lane, which costs them a rebuild mid-unit.
- **A red CI run's failure COUNT is not the failure surface** (#1128).
  Hosted CI passes neither fail-fast flag to `cargo nextest run` and
  nextest stops at the first failure, so a run reports ~1 failure per
  shard however many exist — measured, hosted 1-2 against local
  `--no-fail-fast` 22, nineteen of them one family. The workflow's
  `fail-fast: false` is the MATRIX setting (one shard not cancelling
  the other), which makes this read as handled. A systematic breakage
  and a lone stale assertion look identical; before concluding a red is
  small, run it locally with `--no-fail-fast`.
- An OOM-killed test shows as a bare "Terminated" single-row FAIL —
  check what else was running and rerun quiet before diagnosing a bug.

**The ways CI silently does not run.** A PR that is CONFLICTING against
main gets NO check runs at all — pushes during that window produce
nothing and merging main afterwards fires nothing retroactively. A run
can also queue with ZERO jobs behind a superseded run, `mergeable:
CLEAN`, and never start. And a green job NAME can sit over a SKIPPED
step (k-lint's demos rows are their own sampled axis). So: merge
origin/main immediately before opening a PR and whenever main moves;
after any push, confirm jobs are actually RUNNING by reading the
workflow **runs** list, not the PR's checks list; re-roll with a real
code commit (an empty commit classifies docs-only); and verify coverage
at the STEP level (`gh api .../jobs`, step conclusions). A missing row
can be ASKED FOR rather than re-rolled for: a `CI-Config:
klint=dev-probe` trailer on the head commit, or ci.yml's
`workflow_dispatch` inputs, pin lane/eps/klint for one run
(`docs/CI-MINUTES-2026-08.md`). **The run record is the instrument; the
workflow source is not.**

**Merging is destructive to checks — four rules, each guarding a silent
or permanent failure rather than a red build.** Before merging, filter
the check runs (`gh api .../check-runs`): reject any `conclusion` that
is not `success`, **and separately confirm none is still in flight** —
a check still running when you merge dies at checkout and can never be
re-run, so its failure reads as a defect forever. Confirm a *skip* is
habitual by checking earlier green runs of the same branch. Resolving a
conflict where **both sides deleted** something: take the union of the
deletions, **derived from `main`** — "keep both sides" resurrects what
another lane struck, and it looks clean. After any resolution, grep the
**whole tree** for conflict markers (`git add -A && git commit
--no-edit` stages a conflicted file verbatim and prompts for nothing),
then check the post-condition **against the merged tree**, not against
your diff — a row you never touched cannot appear in your diff.

**Liveness.** Check every running lane at least hourly (arm
`hourly-checkin.sh`) — lost wake-on-completion events are endemic.
**Transcript-mtime is NOT a liveness signal**: an agent inside a
blocking slot wait writes nothing for an hour while progressing
normally, and a queued nudge only drains at its next tool round.
Escalation order: nudge, then check the LANE for real progress (process
table, lock holders via `fuser`, `target/` mtimes), and only then
TaskStop after a full battery-length window — a wrong TaskStop costs an
interrupted battery whose rows must be re-run untrusted. Once an
agent's report is final, TaskStop it: orphaned detached timers re-wake
a finished agent forever, and a lane about to finish should cancel its
own detached waits first.

**Waiter self-test.** Run a background waiter's detection expression
ONCE in the foreground before arming it — a catch-all retry arm
(`|| echo retry`) converts a permanent error into silent eternal
waiting.

**Lane-takeover courtesy.** When the orchestrator operates in a
possibly-alive agent's lane, MESSAGE the incumbent first or
simultaneously — an unannounced takeover reads as a rogue actor from
inside the lane. The number of ALIVE agents is not capped; only
concurrent heavy cargo is.

**A restart loses the INBOX and keeps the WORKTREE**, and they then
disagree: a queued message is dropped, while the uncommitted work done
for it survives on disk, so a lane can wake holding a change with no
traceable authority. **Revert first, ask second, do not conclude** —
*"I cannot find the authority"* is evidence about the records, and
after a restart the records are the unreliable half.

**Death recovery.** A dead subagent's transcript AND its isolation
worktree survive — `git worktree list` from the main checkout, then
SendMessage resumes it. Choose **fresh over resume** when the agent has
been stalled over an hour AND the remaining work is small and fully
specifiable from pushed commits; resume only when the accumulated
context is genuinely useful (mid-design state, unreported findings).
Prevention: implementers commit AND push after every coherent unit. **A
resume resets cwd to the orchestrator worktree** — every resumed
command must carry `cd <clone> && ...` in the same Bash call, and
post-resume battery claims are trusted only after verifying the
transcript rows carried the cd.

**The session scratchpad is SHARED between concurrently running agents
of one session.** PR/issue bodies, logs and run artifacts go to
LANE-PRIVATE paths (`~/.local/share/cad-work/<lane>-*.md`,
`cad-work/<lane>/`), never the scratchpad — filenames alone leak, which
makes it a blinding channel as well as a confusion one. Orchestrator
briefs state this.
