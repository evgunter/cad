---
name: FreeCAD render lane
description: How demos/render.sh survives this host's FreeCAD stalls — one process per scene, a measured per-scene budget with kill-and-retry, and staged publish; plus the two FreeCAD/coreutils behaviours any change here must respect.
type: operational
---

# FreeCAD render lane

## Re-baselining (2026-08-17 — read this before reaching for a download)

CI **commits its own renders**, on all four lanes. A run whose kernel,
freecad, wild or uv lane differs from the committed cells pushes the
new cells to that branch and posts a check run with conclusion `neutral` — GitHub's "!",
not its "x" — whose text says: *if the render looks right, this job is
a success, no re-run needed*. So:

    push -> wait for CI -> git pull -> look at the images

That is the whole flow. There is nothing to download and nothing to
install, and a re-baseline is **not a failure** — do not re-run the job
to try to make it green.

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
the "warm-session deadlock" of #224, seen 3/3 there and twice more
during #266.

**Keep the per-scene isolation anyway.** It was built to survive this
bug, but it is also what bounds any future hang, and the staged publish
is what keeps a half-finished pass out of the committed tree.

So: **never render more than one scene per `freecadcmd` process.**
Since #266 `render.sh` does this in both lanes, under a per-scene
budget (`FREECAD_SCENE_TIMEOUT`, default 300 s) that kills the process
*group* and retries once; a second expiry fails the pass loudly. A pass
renders into `demos/out/stage/<lane>/` and is published only when
complete, so nothing half-finished can reach the committed tree.

Rate to expect: roughly one post-render stall per 19-scene pass. It
costs one budget and is reported, never silent.

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

3–19 s per scene on an unloaded box; **106–114 s for the same scene at
load 13–19** (61% CPU — CPU/cache contention, not I/O). A full pass is
~2.5 min idle and can exceed an hour while cargo lanes are building, so
a render pass and a build battery on the same box is a bad trade in
both directions.

## GET RENDERS BY PULLING (2026-08-17, supersedes the 2026-08-11 rule)

`ci.yml`'s `renders` job calls `render.yml` on every push that builds
anything, and a lane that no longer matches is **re-baselined for you**
— CI commits the new cells to your branch and marks the run neutral.
So the way to re-render is:

    git push        # then wait for CI
    git pull        # the frames are already committed

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
sentence). Measured
2026-08-10 on a 2-core runner (llvmpipe under Xvfb): 19 scenes,
median 3 s, max 6 s, 62 s total — faster than this host, and it
does not compete with the build lanes.

### Where a hosted run's time goes (measured 2026-08-11)

Runner: **2 cores, 7 GB**, llvmpipe under Xvfb. A full 4-lane run is
two waves — `tour` gates the three lanes that read it; `wild` and `uv`
finish inside their shadow and gate nothing — so the run's wall clock
is `tour` + `kernel montage` and nothing else.

* **The tour step is a COMPILE, not geometry**: ~94-121 s of `cargo`
  against ~8 s of actual work (the binary runs in 7.8 s locally).
  `Swatinem/rust-cache` reports `full match: true` and all seventeen
  workspace crates still rebuild — the action evicts workspace members
  from what it restores, by design. Caching what the tour PRODUCES
  (`demos/out`, keyed on an exact hash of its sources) skips both
  halves: **152 s -> 16 s**, and a 4-lane run **333 s -> 184 s**.
* **`CAD_RENDER_JOBS=2`** (render.sh's concurrency knob, default 1)
  takes the kernel loop 110 s -> 85 s and the STEP loop 45 s -> 36 s.
  Contention shows up exactly where the numbers above predict — median
  scene 3 s -> 5 s, max 9 s — which is still 33x under the budget.
  **Verified byte-identical** at K=1 vs K=2, both lanes (55 files), and
  identical to the committed cells. Two hosted runs of one commit are
  byte-identical, so that comparison is real signal.

**Committed frames are the HOSTED producer's output** (Evan's
canonical-producer ruling on #338; the wholesale re-baseline unit
executed it — PNG pixels are not byte-comparable ACROSS GL stacks,
so exactly one stack can be the producer, and it is the hosted
one). Byte-stability is defined against a repeat HOSTED render;
local renders are preview-only (the local hazards above still
apply when previewing) and their frames must never be committed.
Implementer briefs no longer include local render passes.
