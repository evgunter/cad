---
name: FreeCAD render lane
description: How demos/render.sh survives FreeCAD's TWO failure modes (a notification-area deadlock, root-caused; and an intermittent SIGSEGV in document teardown that swallows the real error) — CAD_RENDER_BATCH scenes per freecadcmd process, a per-PROCESS budget with kill-and-retry, and staged publish; plus the FreeCAD/coreutils behaviours any change here must respect.
type: operational
---

# FreeCAD render lane

## Re-baselining (2026-08-17 — read this before reaching for a download)

CI **re-baselines its own renders**, on all four lanes — you never
hand-commit cells. **PRs REPORT, `main` COMMITS:**

    push -> CI posts a neutral ("!") drift check naming the cells
         -> merge -> main's run commits them -> git pull

A drift check is **not a failure** — if the render is what you intended
it is a pass; do not re-run the job to make it green.

**Why PRs do not commit** (learned the hard way on #598): a bot commit
onto a PR branch becomes the PR's head, and a GITHUB_TOKEN push
triggers no run of its own, so the PR showed ONE neutral check with
every green check stranded on the parent commit. The recursion guard and
that blank slate are the same fact — you cannot have the bot's commit
skip CI *and* carry CI's checks, not with GITHUB_TOKEN. So the commit
moved to main, matching the rebuild-latency history's rule. The cost,
accepted: a PR merges with stale cells and main heals within minutes.

To LOOK at the cells before merging, take the run's artifact with
`local-scripts/render-hosted.sh --lane <lane>`.

ci.yml's `uv sheet drift (demos)` row was **retired** when the uv lane
started re-baselining: both fired on the same condition and read the
same cached tour output, so it was a duplicate signal that would have
reported red while the re-baseline reported neutral. Its local mirror
in `ci-local.sh` stays — a developer box cannot re-baseline itself.

What still needs `local-scripts/render-hosted.sh`: a dispatch aimed at
a bare SHA (no branch to commit to), `--on-demand`, and `--verify`.

What still FAILS loudly, unchanged: a wedged pass, and the
`assert no matplotlib fallback` check. The re-baseline is only reached
when the render itself succeeded, so a wedge is still reported as a
wedge and never as drift.

Why neutral rather than red: nothing here is branch-protected and
agents self-merge, so a red X was never blocking — it was a signal that
happened to be red, and a render change is a normal event, not an
error. The cost, chosen deliberately: an agent polling "any failures?"
sails past a neutral check; one polling "is everything success?" stops
on it. Reasoning in `.github/actions/rebaseline-lane`.

`demos/render.sh` drives two montage lanes through headless `freecadcmd`.
Both are byte-reproducible: a re-render that changes nothing leaves
`git status` clean. That is a standing rule, not a nicety.

## The hang: a FreeCAD self-deadlock, and what it actually is

**ROOT CAUSE FOUND (2026-08-10). It is a FreeCAD self-deadlock, not a
host quirk, and `demos/render_freecad.py` now disables the thing that
causes it.** Caught on a hosted runner, where it fired on every attempt
instead of occasionally — main-thread backtrace, read bottom-up:

    Gui.updateGui()
      -> a queued QTimer fires
      -> Gui::NotificationArea::showInNotificationArea()   TAKES the lock
      -> NotificationBox::showText -> QWidget::raise()
      -> QPlatformWindow::raise() warns "This plugin does not
         support raise()"   <- the offscreen QPA plugin, every time
      -> FreeCAD routes every Qt message into its own Console
      -> NotificationAreaObserver::sendLog
      -> Gui::NotificationArea::pushNotification()         RETAKES it
      -> non-recursive mutex, same thread: deadlock, forever.

It needs a pending notification AND a Qt warning emitted while that
notification is on screen. `Part`'s "STEP import is deprecated" warning
supplies the first; the offscreen plugin supplies the second on every
`raise()`. That is why it was frequent headless, random-looking (it
depends on where the timer lands), and unheard-of interactively.

The fix is two parameter writes before `FreeCADGui` is imported —
`NotificationAreaEnabled` and `NonIntrusiveNotificationsEnabled` off,
under `User parameter:BaseApp/Preferences/NotificationArea`. Order
matters: the notification area is constructed with the main window.
**Side effect:** FreeCAD parameters are global to the user config, so
any machine that has run a render has the notification area off in
interactive FreeCAD too (Preferences -> General -> Notification Area to
restore). Isolating renders behind their own `--user-cfg` would fix
that and make committed pixels independent of a developer's
accumulated preferences — worth doing, not done.

The historical symptom, for recognising it if it ever returns: a stall
*after* writing a complete, byte-correct PNG (0% CPU, `futex_do_wait`),
a different scene every pass, on an idle box as well as a loaded one —
the "warm-session deadlock" of #224 and #266.

**Keep the process boundary anyway.** It was built to survive this
bug, but it is also what bounds any future hang, and the staged publish
is what keeps a half-finished pass out of the committed tree.

## Scenes per process is a DIAL, not a prohibition (2026-08-22)

Until 2026-08-22 the rule here was "never render more than one scene per
`freecadcmd` process". **Evan reopened and approved batching**, so the
rule is now a knob: **`CAD_RENDER_BATCH` (owned by `render.sh`, default
1) is how many scenes one `freecadcmd` process renders.** At the default
the behaviour is exactly what it was since #266 — one process per scene,
down to the log filenames.

Why it was safe to move, since a standing rule does not move for free.
Byte-identity was measured on a real FreeCAD 1.1.2 install, from the
same AppImage CI uses: kernel lane B=1 vs B=1 (**the control arm**)
36/36 frames identical, B=1 vs B=5 36/36, B=1 vs **B=35** — the whole
lane in ONE process — 36/36, and STEP lane B=1 vs B=5 21/21. The control
arm passing is what makes the other three mean anything: it separates
"batching changes nothing" from "this comparison cannot detect change".
A warm document leaks nothing into pixels.

Of the rule's three original justifications, exactly ONE expired. It was
built to survive the notification-area deadlock — root-caused and fixed
(#331), so that reason is gone. "It bounds any future hang" still
stands, and is now answered by the batch SIZE rather than by the
prohibition. The staged publish was always independent of both.

**The budget is per PROCESS, not per scene**: a batch of N gets one
`FREECAD_SCENE_TIMEOUT` (whose default `render.sh` owns), not N x it, so
a hang's worst case — two attempts — is INVARIANT under batch size.
`render.sh` kills the process *group* and retries once; a second expiry
fails the pass loudly. A pass renders into `demos/out/stage/<lane>/` and
is published only when complete, so nothing half-finished can reach the
committed tree.
Expect the occasional post-render stall: it costs one budget and is
reported, never silent.

What actually bounds the batch now is **contention headroom** — not
blast radius, and not hangs. Measured uncontended lane work is 81-101 s
(freecad lane) and ~105 s (kernel lane), against a 300 s budget, and the
note below records a single scene taking 106 s on a loaded host. So a
large batch on a CONTENDED box can exhaust a budget that the same scenes
would not exhaust one at a time. That headroom, not blast radius, is the
number to think about before raising B.

An in-process per-scene watchdog was **rejected on the root cause**, and
should stay rejected — it is the obvious-looking redesign and it cannot
work. The hang is a mutex re-entry on FreeCAD's own main thread, which
never gets back to a bytecode boundary and never releases the GIL, so
neither a Python signal handler nor a watchdog thread would ever run.
Only a process outside can kill it, and `timeout` is that process.

## The OTHER failure mode: a SIGSEGV in document teardown (2026-08-22)

The deadlock above is not the only way this lane fails, and the two look
NOTHING alike. A failure-history analysis found ten events of this second
class, and a reader who knows only the hang will go hunting a stall at 0%
CPU in `futex_do_wait` and find a process that has been dead for four
seconds.

The signature:

* **`freecadcmd rc=1`, dead in 3-5 s** against the 300 s budget — 10/10
  of them, 0/10 on any timeout path. These are CRASHES, not hangs.
* the log's only report of the primary error is **"Unknown exception
  while processing file"**, followed by `File format not supported: ..`
  — that second line is `freecadcmd` treating leftover argv as documents
  to open, a SYMPTOM of the swallow and not the cause.
* at teardown, `closeAllDocuments()` -> `slotDeleteDocument` ->
  `setActiveDocument` -> `runString` -> `PyException::PyException()` ->
  **SIGSEGV**: it crashes while constructing the very object that would
  have described the failure. Frames #3/#5/#6/#9/#10 are
  character-identical across all ten events.

Intermittent and **not scene-specific**: roughly **1 failure in 4,300
scene renders**. Every scene that has failed (`chute`, `crosslap`,
`crosslap_exploded`, `lily`, `silhouette3`, `tiltedcut`) also succeeded
in other runs, exactly one scene fails per affected run, and never the
same one twice running. It costs one cell and reports a failed pass —
loud, not silent, but until now undiagnosable.

**The primary error was SWALLOWED until 2026-08-22**, which is precisely
why this class still has no root cause. `render_freecad.py` now wraps its
top-level `main()` and prints `traceback.format_exc()` to stderr,
explicitly flushed, BEFORE letting the exception continue outwards — the
print has to happen there, because the SIGSEGV is at teardown and
anything deferred until then may never run at all. The next occurrence
should carry a real traceback in `demos/out/freecad-logs/<batch>.log`.
Read it before theorising.

One adjacency worth recording without overclaiming: this crash sits in
`FreeCADGui`'s document-teardown path, and #331's deadlock fix was also
`FreeCADGui`-adjacent. **That is an adjacency, not a link** — nothing
has been shown to connect them, and either way the PRIMARY failure is
upstream, in the Python script; the segfault only destroys the report of
it.

## Two behaviours any change here must respect

* **FreeCAD stamps the output PATH into every PNG** (a `tEXt` `Title`
  chunk), so a frame rendered to a different path differs in BYTES even
  when the pixels are identical. Since 2026-08-22 `strip_png_stamps.py`
  drops `Title` along with the two wall-clock chunks (Evan authorised
  it), so committed frames no longer encode where they were written.
  That **retires the old constraint** that the staging tree had to
  mirror the lane directory's *name*: the scene process still runs with
  the staging root as its cwd and the tree still mirrors the lane, but
  that is now convenience, not a correctness requirement.
* **`timeout`'s exit status is not a reliable timeout signal.** With a
  SIGTERM-ignoring child, `timeout -k` escalates to SIGKILL, which
  coreutils sends to its own process group — `timeout` dies too and
  reports 137, not 124. Judge a budget by the clock.

## Cost of a pass

A scene costs seconds on an unloaded box and **minutes under a cargo
load** — CPU/cache contention, not I/O — so a whole pass goes from
minutes to hours. A render pass and a build
battery on the same box is a bad trade in both directions.

## GET RENDERS BY PULLING (2026-08-17, supersedes the 2026-08-11 rule)

`ci.yml`'s `renders` job calls `render.yml` on every push that builds
anything, and a lane that no longer matches is **re-baselined for you**
— the PR run reports it with a neutral check, and main's run commits
the new cells. So the way to re-render is:

    git push        # CI posts a neutral drift check naming the cells
    # merge the PR  # main's run commits them
    git pull        # on main, the frames are there

Then look at the images; if they are what you meant, you are done. It
supersedes the old `local-scripts/render-hosted.sh` default, which
resolved the run and installed each lane by hand.

**Dispatch (`--on-demand`, or `render.yml` directly) only when CI has
not covered the tree**: unpushed branch, no CI run, or a deliberate
re-render at a different scene budget. Dispatching otherwise renders
the same tree twice — and that run re-baselines too, so it also ends
in a pull.

Expect the two PNG lanes to re-baseline when the runner image's mesa
bumps (roughly monthly). That is the lane working, and it now costs a
pull rather than a mechanical commit.

Historical note worth keeping: the old failing row deliberately did NOT
print a bare `gh run download`, because that command refuses to
overwrite existing files and so fails on the first cell when aimed at a
populated lane directory. `render-hosted.sh` stages into a temp dir
first — still true, and still why the bare-SHA fallback path names the
script rather than the raw command.

## RENDER-IN-ACTIONS IS THE NORM (Evan's ruling, 2026-08-10; hosted = CANONICAL PRODUCER)

The hosted "render (demos)" workflow (#323/#324, wedge root-caused
and fixed by #331 — a FreeCAD NotificationArea SELF-DEADLOCK, not
this host's stall or budget calibration) runs all lanes on every push and
on demand (`local-scripts/render-hosted.sh`, the #338 wrapper —
trigger, poll, byte-exact artifact pull-back — is now the DISPATCH
front end only; the ordinary path is push-and-pull, above. Local entry
points still refuse without the explicit CAD_RENDER_LOCAL_OVERRIDE
sentence). A hosted pass is faster than this host and it does not
compete with the build lanes.

### Where a hosted run's time goes

The runner is small (2 cores, llvmpipe under Xvfb). A full 4-lane run is
two waves — `tour` gates the three lanes that read it; `wild` and `uv`
finish inside their shadow and gate nothing — so the run's wall clock
is `tour` + `kernel montage` and nothing else.

* **The tour step is a COMPILE, not geometry**: nearly all of it is
  `cargo`, against seconds of actual work. `Swatinem/rust-cache`
  reports `full match: true` and every workspace crate still rebuilds —
  the action evicts workspace members from what it restores, by design.
  Caching what the tour PRODUCES (`demos/out`, keyed on an exact hash
  of its sources) skips both halves — and the tour is most of what a
  4-lane run costs.
* **`CAD_RENDER_JOBS`** (render.sh's concurrency knob) shortens both
  render loops. Contention then shows up exactly where the compile/
  render split predicts — a slower median scene — and still far under
  the process budget. **Verified byte-identical** across concurrency
  settings, both lanes, and identical to the committed cells. Two
  hosted runs of one commit are byte-identical, so that comparison is
  real signal.

**Committed frames are the HOSTED producer's output** (Evan's
canonical-producer ruling on #338; the wholesale re-baseline unit
executed it — PNG pixels are not byte-comparable ACROSS GL stacks,
so exactly one stack can be the producer, and it is the hosted
one). Byte-stability is defined against a repeat HOSTED render;
local renders are preview-only (the local hazards above still
apply when previewing) and their frames must never be committed.
Implementer briefs no longer include local render passes.
