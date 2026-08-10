---
name: FreeCAD render lane
description: How demos/render.sh survives this host's FreeCAD stalls — one process per scene, a measured per-scene budget with kill-and-retry, and staged publish; plus the two FreeCAD/coreutils behaviours any change here must respect.
type: operational
---

# FreeCAD render lane

`demos/render.sh` drives two montage lanes through headless `freecadcmd`.
Both are byte-reproducible: a re-render that changes nothing leaves
`git status` clean. That is a standing rule, not a nicety.

## The hazard, and what it actually is

This host's FreeCAD **stalls after writing a frame** — a complete,
byte-correct PNG, then the process hangs (0% CPU, `futex_do_wait`). It
picks a different scene every pass, on an idle box as well as a loaded
one. In a warm multi-scene session that stall lands mid-pass and every
remaining scene is lost: that is the "warm-session deadlock" of #224,
seen 3/3 there and twice more during #266.

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

## RENDER-IN-ACTIONS IS THE NORM (Evan's ruling, 2026-08-10)

The hosted "render (demos)" workflow (#323/#324, wedge root-caused
and fixed by #331 — a FreeCAD NotificationArea SELF-DEADLOCK, not
this host's stall or budget calibration) runs all lanes: tour scene
inputs, kernel montage, freecad montage, UV sheet. Full fan-out
verified green 2026-08-10. **The norm going forward: renders happen
in GitHub Actions.** Implementer briefs no longer require local
FreeCAD passes — local renders are a preview-only iteration tool
(the local hazards above still apply when previewing). Byte-
stability and provenance-guard checks ride the hosted lane.
