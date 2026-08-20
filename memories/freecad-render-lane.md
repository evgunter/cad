---
name: FreeCAD render lane
description: How demos/render.sh survives this host's FreeCAD stalls — one process per scene, a measured per-scene budget with kill-and-retry, and staged publish; plus the two FreeCAD/coreutils behaviours any change here must respect.
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

## The hazard, and what it actually is

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

**Keep the per-scene isolation anyway.** It was built to survive this
bug, but it is also what bounds any future hang, and the staged publish
is what keeps a half-finished pass out of the committed tree.

So: **never render more than one scene per `freecadcmd` process.**
Since #266 `render.sh` does this in both lanes, under a per-scene
budget (`FREECAD_SCENE_TIMEOUT`, whose default `render.sh` owns) that
kills the process *group* and retries once; a second expiry fails the
pass loudly. A pass renders into `demos/out/stage/<lane>/` and is
published only when
complete, so nothing half-finished can reach the committed tree.
Expect the occasional post-render stall: it costs one budget and is
reported, never silent.

## Two behaviours any change here must respect

* **FreeCAD stamps the output PATH into every PNG** (a `tEXt` `Title`
  chunk). Render a frame to a different path and its bytes differ even
  when the pixels are identical — which is why the staging tree mirrors
  the lane directory's *name* and the scene process runs with the
  staging root as its cwd. `strip_png_stamps.py` drops the two
  wall-clock chunks but keeps `Title`.
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
  the per-scene budget. **Verified byte-identical** across concurrency
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
