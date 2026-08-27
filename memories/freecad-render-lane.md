---
name: FreeCAD render lane
description: Operating demos/render.sh and the hosted render workflow — how frames reach the tree, the two ways FreeCAD fails, and the knobs and traps around the per-process budget.
type: operational
---

# FreeCAD render lane

`demos/render.sh` drives the montage lanes through headless `freecadcmd`.
Frames are **byte-reproducible**: a re-render that changes nothing leaves
`git status` clean. Standing rule, not a nicety.

## Getting renders: push, merge, pull

CI re-baselines its own renders, on every lane. **Never hand-commit
cells.** PRs REPORT, `main` COMMITS:

    git push        # the PR run posts a NEUTRAL drift check naming the cells
    # merge         # main's run commits them
    git pull

A drift check is **not a failure** — if the render is what you intended
it is a pass; do not re-run the job to make it green. It is neutral, not
red, so beware of how you poll: "any failures?" sails straight past it,
"is everything success?" stops on it.

To LOOK at cells before merging, pull the run's artifact with
`local-scripts/render-hosted.sh --lane <lane>`. Dispatch (`--on-demand`,
or `render.yml` directly) **only when CI has not covered the tree**: an
unpushed branch, no CI run, or a deliberate re-render at another budget.

Expect the PNG lanes to re-baseline when the runner image's mesa bumps.
That is the lane working.

## Hosted is the canonical producer

PNG pixels are not byte-comparable across GL stacks, so exactly one
stack produces committed frames and it is the hosted one (Evan's
ruling). Byte-stability is defined against a repeat HOSTED render.
**Local renders are preview-only and must never be committed**; local
entry points refuse without the explicit `CAD_RENDER_LOCAL_OVERRIDE`
sentence.

## The two ways this lane fails

They look nothing alike; tell them apart before theorising.

* **A hang.** `freecadcmd` at 0% CPU in `futex_do_wait`, a different
  scene each time, often *after* writing a complete, byte-correct PNG.
  It is FreeCAD's notification area re-entering its own non-recursive
  mutex under the offscreen QPA plugin; `render_freecad.py` disables the
  notification area before `FreeCADGui` loads, which is why the order of
  the first thing that file does is load-bearing. **Side effect:**
  FreeCAD parameters are global to the user config, so any machine that
  has run a render has the notification area off in interactive FreeCAD
  too (Preferences -> General -> Notification Area to restore).
* **A crash.** `freecadcmd` rc=1, dead in seconds, nothing in the log
  but "Unknown exception while processing file" — the
  `File format not supported: ..` line after it is a symptom, not the
  cause — and a SIGSEGV in FreeCAD's document teardown. Intermittent and
  not scene-specific; costs one cell and fails the pass. The primary
  failure is a Python exception, and `render_freecad.py` prints its
  traceback to stderr before FreeCAD can swallow it, so **read the log**
  (`demos/out/freecad-logs/`). No root cause yet.

## Knobs, budget, publish

* `CAD_RENDER_BATCH` (`render.sh`, default 1) sets scenes per
  `freecadcmd` process; `CAD_RENDER_JOBS` sets how many render at once.
  Neither may change a byte of any frame — that is their acceptance
  test, asked of a repeat render on ONE box.
* `FREECAD_SCENE_TIMEOUT` is the budget, and it is **per PROCESS, not
  per scene**: a batch of N gets one of these, not N x it, so a hang's
  worst case is invariant under batch size. The process *group* is
  killed and retried once; a second expiry fails the pass loudly,
  naming the scene. An in-process watchdog is not an option — the hang
  never releases the GIL, so only an outside process can kill it.
* What bounds the batch is **contention headroom**, not blast radius: a
  scene takes minutes on a loaded box, so a large batch there can
  exhaust a budget the same scenes would not exhaust one at a time. A
  render pass and a build battery on one box is a bad trade both ways.
* A pass renders into `demos/out/stage/<lane>/` and publishes only when
  complete, so nothing half-finished reaches the committed tree.

## The trap when changing render.sh

**`timeout`'s exit status is not a reliable timeout signal.** With a
SIGTERM-ignoring child, `timeout -k` escalates to SIGKILL, which
coreutils sends to its own process group — `timeout` dies too and
reports 137, not 124. Judge a budget by the clock.
