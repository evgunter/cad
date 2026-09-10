---
id: freecad-lane-reports-no-drift-on-a-cell-whose-geometry-changed
kind: issue
title: the freecad lane reports no drift on the teapot, whose geometry changed
status: open
opened: 2026-09-10
---



`demos/renders-freecad/teapot.png` draws a straight, round, un-bent
spout. The scene has not built that shape since #2286, where the spout
became a `Node::Loft` through seven OCTAGONAL annular sections on a
circular arc's tangent frames — 18 faces where the frustum had 4,
bending 45 degrees and tapering.

Its kernel twin, `demos/renders/teapot.png`, re-baselined on the merge
and shows the canal correctly. The two lanes now disagree about what
the code builds, and only one of them is right.

**The freecad lane did not fail, and did not decline to act. It
rendered this scene and reported that nothing had changed.** From the
post-merge run on `415d77c8` (job 102746834983):

```
STEP lane: 4 batch(s): median 4s, max 4s, total 13s, ...
  stripped out/stage/renders-freecad/teapot.png: tEXt, zTXt
strip_png_stamps: 16/16 files stripped
check_render_provenance: 31 PNG(s) carry their lane's renderer signature
...
demos/renders-freecad matches this render.
```

Every part of that is the healthy path: 16 scenes rendered, 16 stripped,
provenance satisfied, and then a clean no-drift verdict.

**The freshly written file is what makes it strange.** `strip_png_stamps`
reports the chunks it removed, and it removed `tEXt, zTXt` from the
staging copy. A committed cell has already had those stripped, so a
staging file that still carries them was written by this pass — the
teapot was really re-rendered by FreeCAD, not copied forward. And the
result was byte-identical to a file last written on 2026-09-06.

**Freshness, per scene, from `git log`:**

| scene | kernel cell | freecad cell |
|---|---|---|
| `impeller12` | 2026-09-10 02:51 | 2026-09-10 02:51 |
| `fivewall` | 2026-09-10 00:29 | 2026-09-10 00:30 |
| `twisted_tube` | 2026-09-10 02:01 | 2026-09-10 02:01 |
| `teapot` | **2026-09-10 04:39** | **2026-09-06, by an ordinary PR merge** |

So the lane is not broken in general — three other scenes re-baselined
normally, each within seconds of its kernel twin. Only this cell is
frozen, and no `render(freecad)` commit has touched it in four days.

**The input is correct**, which rules out the easy explanation. The
scene's manifest entry lists `teapotspout.step` among the teapot cell's
four bodies, and the exported file is the new shape:

```
demos/out/teapotspout.step   ADVANCED_FACE 18   B_SPLINE_SURFACE 16
```

18 faces and 16 spline walls is the octagonal canal, not the 4-face
frustum. The uv sheet re-baselined in the same run off the same tour,
so the scene-inputs job ran on the merged code.

**What is verified above, and what is not.** Everything to this point is
read off committed artefacts and the run log. The MECHANISM is not:
FreeCAD was handed a changed STEP and produced unchanged pixels, and
this issue does not claim to know why. Two hypotheses worth testing
first, neither confirmed:

1. **A per-scene failure that the lane-level verdict cannot see.**
   `demos/.gitignore` carries `renders-freecad/*.fail.txt`, so a scene
   failing to draw is an expected, uncommitted artefact. `render.yml`
   already warns about the lane-level version of this hazard — *"the
   directory a failed pass leaves behind is the COMMITTED one,
   untouched — it would read as 'no drift' and quietly pass"* — and
   fixed it by making `freecad_ok` per lane rather than per job. If a
   single scene can fail the same way underneath a healthy lane, the
   same argument wants the guard one level finer still. The freshly
   written `tEXt` chunks argue against a plain skip, but not against a
   fallback that writes something.
2. **The teapot cell's bodies are resolved somewhere stale** — the
   right STEP exists on disk and a different one, or an older cached
   copy, is what the importer opens.

**Why this matters more than one wrong cell.** The `render drift`
checks are `neutral` by design, so they report and never gate. That is
fine while a lane that renders a changed scene produces changed pixels
— drift is loud, and the merge commits it. This cell breaks that
assumption: the pipeline said *matches this render* about geometry that
had just changed completely. A silent no-drift is indistinguishable
from a correct cell, on the contact sheet a reader looks at first.

**How to see it without rendering anything:** open
`demos/renders/teapot.png` and `demos/renders-freecad/teapot.png` side
by side. The spout is curved and faceted in one and straight and round
in the other.
