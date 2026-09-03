---
id: retire-render-automatic-matplotlib-fallback
kind: issue
title: render.sh — retire the automatic matplotlib fallback; a crashed scene must fail loudly, not become a green preview
status: open
opened: 2026-08-19
github: 629
refs: [221, 224, 331, 626]
---

## From GitHub issue 629

Opened 2026-08-19; 0 comments.

**Ruling (Ev, 2026-08-19):** *"automatic matplotlib fallback should be removed (matplotlib can remain an explicit alternative)."*

## What happened

`render lanes / kernel montage` failed on PR #626 (run `32204571643`). The diff touched only `ci.yml`, `ci-local.sh`, `scripts/gates/*.sh` and two markdown files — nothing reachable from the renderer — and the same job was green on `main` at `e85cf9d6` and on the PR's own prior head `33004d3a`, off identical scene inputs restored from cache.

One scene, `chute`, crashed:

```
 MATPLOTLIB FALLBACK — this is NOT the committed render
   reason:  scene 'chute': freecadcmd rc=1: #13  /lib/x86_64-linux-gnu/libc.so.6(__libc_start_main+0x8b)
            [0x7f402e42a28b] #14  /home/runner/freecad-appimage/squashfs-root/usr/bin/freecadcmd(+0x78c9)
```

The tail is frames #13/#14 of FreeCAD's own crash-handler backtrace — `freecadcmd` **crashed**; it did not stall. The other 35 scenes rendered on attempt 1 at 4–10 s each against a 300 s budget.

## The shape worth fixing

`render.sh`'s `fallback()` **ends the pass with exit 0** by design, routing everything to `demos/renders-preview/` (#221's uncommittable tree). Because a failed pass therefore looks successful, `render.yml` carries a *second, separate* step — `assert no matplotlib fallback` — whose whole job is to detect the success-that-wasn't. That is the defect: a script that exits 0 on failure, plus a downstream gate that exists only to undo that.

Retiring the automatic fallback collapses both. The render fails where it fails, the workflow needs no structural check that the fallback tree is absent, and `renders-preview/` routing stays exactly as-is for explicit matplotlib runs.

The repo already contains the target shape: `render.sh:367` — the `--freecad` lane *"has no fallback (its whole point is the OCC reference render)"*.

### Proposed

1. `render.sh:452` (`freecadcmd not found`) and `:468` (`*) fallback "scene '$name': $ST_REASON"`) stop invoking `fallback()` automatically and fail nonzero with the same message text.
2. Matplotlib stays reachable behind an explicit flag (`--matplotlib`), keeping the `renders-preview/` routing and the PREVIEW-ONLY banner at `:448`.
3. `render.yml`'s `assert no matplotlib fallback` step is retired in the same change — with the fallback gone it asserts a condition that can no longer arise. **Do not retire it before step 1 lands**; it is the only thing standing between a fallback pass and a green lane today.

## Second, related: the retry is asymmetric in the wrong direction

`render.sh:200-232`. The `for attempt in 1 2` loop retries **only** on `rc=124` (per-scene budget exhausted). Every other non-zero rc falls straight through to `SCENE_REASON="freecadcmd rc=$rc: …"` and `return 1` with no second attempt.

So a **stall gets a retry and a crash gets none** — backwards, on this evidence. The repo's recorded FreeCAD hazard is the NotificationArea self-deadlock (#224/#331), a stall, and the retry was presumably written for it. But `chute` had rendered clean on the two immediately preceding runs off the same inputs, so this crash was transient, and it took down a lane that had already drawn all 36 scenes.

Worth deciding alongside step 1, since removing the fallback changes what a non-retried crash costs: it becomes a red lane rather than a silent preview. No prior record of a `freecadcmd rc=1` crash in this lane appears anywhere in `docs/` or `memories/` — new symptom, single occurrence.

## Evidence retained

`freecad-logs-kernel` artifact, ID `9348750507`, run `32204571643` — holds the full `chute.log` backtrace (14-day retention). That would separate an OCC segfault from an OOM kill or an llvmpipe fault, if anyone wants the root cause rather than the routing fix.

---

Filed from the SMELL-SCAN wave-1b lane; found while diagnosing a red on #626, not by the scan.

## Home

`work/issues/`: `demos/render.sh` and `.github/workflows/render.yml` are S-QA's gate ground and S-QA is closed; no open program's `paths` reaches the render lanes.
