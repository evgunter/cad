# Pre-GUI demo tour

A visual tour of what the kernel can do today, from a pure outside
consumer's seat: sweep bodies through the public `profile` / `sweep`
APIs, booleans through `union` / `subtract` / `intersect`
(boolean-of-boolean chains included), a `topo::split` cutaway, the
recipe layer (editor-core document, structural edit, downstream-only
recompute, stable names), and the assembly layer (a workspace of pinned
part documents, instances, patterns, mates solved constructively,
split/inline, the pin-update door) — narrated (operations used, topology
census + genus, validation tiers passed, exact-vs-meshed mass
properties), exported as binary STL + AP214 STEP, and rendered to PNG.

This directory is **deliberately outside the cargo workspace** (root
manifest `workspace.exclude`, plus the empty `[workspace]` table in
`tour/Cargo.toml`): viewer/render tooling is demo-only and must never
become a kernel dependency.

## Run

**Renders are hosted, and the hosted lane is the canonical producer.**
Every committed frame in `renders/`, `renders-freecad/` and
`renders-wild/` is the hosted workflow's output — llvmpipe under Xvfb,
FreeCAD 1.1.2 AppImage — and byte-stability ("a clean re-render leaves
`git status` clean") is defined against that producer. A locally-drawn
frame carries this box's GL stack, **will** differ byte-wise, and must
never be committed; the guard below and `check_render_provenance.py`
enforce the commit side.

**You do not need to render at all — CI does it and commits the result.**
Every CI run on a pushed branch renders all four lanes (ci.yml's
`renders` job calls `render.yml`), and a lane that no longer matches what
the code renders is **re-baselined for you**:

```sh
git push        # CI renders; a lane that differs posts a neutral ("!")
                #   drift check naming the cells
# merge the PR  # main's own run commits the new cells
git pull        # on main, the frames are there
```

**If the render is what you intended, the drift check is a pass.** It
needs no re-run and no second commit. Re-run only if something *else* in
the run failed. To see the cells before merging, take the run's artifact
with `local-scripts/render-hosted.sh`.

**PRs report; `main` commits.** A bot commit onto a PR branch becomes the
PR's head, and a `GITHUB_TOKEN` push triggers no run of its own — so the
PR would show that one check and nothing else, with every green check
stranded on the parent commit. The recursion guard and that blank slate
are the same fact, so the commit happens on `main` instead. Same rule the
rebuild-latency history follows.

A re-baseline has two causes and they want different reactions — the
geometry changed (these cells are the new truth; check they look like
what you meant), or the runner image's mesa bumped and re-rasterised them
(roughly monthly; the pixels moved and the geometry did not).

What still **fails** loudly: a wedged pass, and the matplotlib-fallback
assertion. The re-baseline is only reached when the render itself
succeeded, so a wedge is reported as a wedge and never as drift.

### Off-box: the hosted lanes

`.github/workflows/render.yml` runs the render lanes on GitHub runners
and hands each one back as a run artifact. It has **two entry points over
one pipeline**: `workflow_call`, which is where your frames come from,
and `workflow_dispatch`, for a tree CI has not seen or a re-render at a
different scene budget.

```sh
local-scripts/render-hosted.sh --on-demand            # a tree CI has not rendered
local-scripts/render-hosted.sh --lane wild --verify   # prove the artifact path is byte-exact
local-scripts/render-hosted.sh --run <id>             # take a specific run, no re-render
local-scripts/render-hosted.sh --lane uv --no-install # leave the artifact in a temp dir
```

**Render on demand only when CI has not covered it** — an unpushed
branch, no CI run yet, or a deliberate re-render at a different scene
budget. Dispatching when CI has already rendered the same tree renders it
twice, which is why it is a flag rather than the default. Those runs
re-baseline too, so they also end in a `git pull`; the exception is a
dispatch aimed at a bare SHA, which has no branch to commit to and
reports the drift with the install command instead.

`render-hosted.sh` **refuses** if your local HEAD is not what
`origin/<branch>` points at — the runner checks out the pushed tree and
cannot see local commits, so rendering an unpushed branch would draw
scenes you are not looking at and look entirely plausible. A dirty
working tree is a warning by the same logic one step down. While the run
is going it prints per-job status on change plus a heartbeat every five
minutes (a FreeCAD leg can legitimately be silent for twenty), and on a
non-success conclusion it names the failing jobs and dumps the failing
steps' log tail. On success it downloads each requested lane's artifact,
installs it at the lane's committed path, **reports rather than deletes**
any committed file the artifact does not contain, runs
`check_render_provenance.py` over the result, and prints what moved.

`--verify` is the round-trip proof that the artifact path is *lossless*:
that a byte-reproducible lane which went out through `upload-artifact`'s
zip and came back through `gh run download` is byte-identical to what is
committed, **provenance `tEXt` chunks and all**. It is not a claim that
hosted pixels match local ones. If it ever stops holding, the provenance
guard is being handed laundered files and every lane's pull is suspect.

The raw commands, if you want them:

```sh
gh workflow run render.yml -f ref=my-branch -f lanes=all
gh run download <run-id> -n renders-kernel -D /tmp/cells   # then copy over
```

Note the `-D`: `gh run download` **refuses to overwrite existing files**,
so pointing it straight at `demos/renders/` fails on the first cell —
which is why the script (and the gate's failure message) stage into a
temp directory and install from there.

`lanes` selects `all` (default) or one of `kernel` / `freecad` / `uv` /
`wild`. The tour is built **once** and handed to the lanes that read it
as an artifact; the two PNG lanes then run as parallel matrix legs
(`fail-fast: false` — one lane wedging must not cancel the other's
evidence), while the UV sheet (no renderer) and the wild-corpus montage
(matplotlib, its own generator, no tour) land without waiting on either.

The PNG lanes provision the **same** version-pinned, checksum-verified
FreeCAD AppImage as `ci.yml`'s `step-import` job — same cache key, so a
hosted render normally downloads nothing — and add what *drawing* needs
on top of what importing needs: software GL (llvmpipe) and Xvfb, because
Coin's offscreen renderer wants a GL context and a display even though Qt
itself stays `offscreen`.

The workflow adds one check of its own, for the kernel lane: **every leg
asserts `demos/renders-preview/` does not exist.** That lane's matplotlib
fallback exits 0, so without the assertion a hosted pass could be green
having drawn nothing with FreeCAD — the frames sitting in the gitignored
preview tree while the artifact holds the committed cells unchanged.

No job commits or pushes on a PR; each lane's diff against the committed
tree is a real finding, and ci.yml's `renders` job fails on it. The one
caveat is the runner image: its mesa bumps roughly monthly and
re-rasterises the two FreeCAD PNG lanes. That is the gate working — the
committed cells are *meant* to track the canonical producer — and it
costs one mechanical commit, which the failing row spells out. Pinning
the GL stack in a container image would remove even that; it remains a
design call, not a config tweak. The UV lane carries no such caveat (it
is renderer-free, and stays gated by `k-lint`'s own row — one gate per
obligation), and neither does the wild lane, which is FreeCAD-free by
scope: its cells come from matplotlib's Agg rasterizer over pinned
`numpy`/`matplotlib` with no GL anywhere in the path, so byte-identity IS
expected there. That is what makes wild the lane `--verify` round-trips:
it exercises the whole stamp-bearing path, not just a text file.

### Preview mode: the local override

The local entry points are what the hosted lanes invoke, and what you
reach for when you are still shaping a scene and do not intend to commit
the frames:

```sh
cd demos/tour
cargo run --release -- ../out   # build + narrate + export STL/STEP + scenes.json
cargo run --release -- gallery ../gallery-out  # the document gallery
cd ..
./render.sh                     # kernel-tessellation montage (renders/montage.png)
./render.sh --freecad           # FreeCAD/OCC STEP-lane montage (renders-freecad/montage-freecad.png)
./render-uv.sh                  # UV trim-loop sheet (renders-uv/montage-uv.svg)
```

`render.sh`, `render-wild.sh` and `render-uv.sh` each source
`hosted-render-guard.sh` as their first act. Without

```sh
CAD_RENDER_LOCAL_OVERRIDE=i-accept-local-render-drift
```

in the environment they print a pointer at the push-and-pull flow above
and **exit nonzero**.

The value is a sentence on purpose. `1` / `yes` / `true` are what anybody
— human or agent — types reflexively when a script complains about an
unset variable; a sentence naming what you are accepting is one nobody
reaches by accident, and it reads as an admission in the shell history
that produced the frames. A pass run this way is **preview only**: its
frames carry *this* box's renderer and GL stack, which is the drift the
sentence names.

The rule is structural, not sniffed: there is no `GITHUB_ACTIONS` check
in the guard. The sanctioned automated callers — `render.yml`'s render
steps, `ci.yml`'s `uv sheet drift (demos)` row, and `ci-local.sh`'s
`uv_sheet_drift` — each set the sentence **in the file, at the step that
renders**, where a reviewer sees it. A sniffed exemption would be
invisible at the call site and would grow silently with every new runner
and local CI emulator.

### Outputs

`demos/out/*.{stl,step}` + `demos/out/scenes.json` + `demos/out/uv/*.svg`
+ `demos/out/uv.json` (untracked); `demos/renders/*.png` (tracked — one
per scene plus `montage.png`); `demos/renders-freecad/*.png` (tracked —
the montage cells plus `montage-freecad.png`);
`demos/renders-wild/*.png` (tracked); `demos/renders-uv/montage-uv.svg`
(tracked); and — only when the kernel lane falls back to matplotlib —
`demos/renders-preview/renders/*.png` (gitignored).

A pass in flight lives in `demos/out/stage/<lane>/` (untracked) and is
published to the lane directory only once it is complete. The staging
tree mirrors the lane directory's *name* and each scene process runs with
the staging root as its working directory, so a staged frame's path reads
the same as its published one.

Both `render.sh` lanes run `strip_png_stamps.py` over the per-scene PNGs
before composing the montage. FreeCAD's `saveImage` stamps the wall clock
into every file it writes (a `tEXt` "Creation Time" chunk and a `zTXt`
"Description" chunk carrying its MIBA XML) and the output path (a `tEXt`
"Title" chunk); the first two would make an unchanged re-render show up
dirty in `git status`, and the third made the same pixels written to two
paths two different files. All three are ancillary chunks, so dropping
them is lossless: a dirty `git status` after a re-render then means the
*pixels* changed, and two frames of the same pixels are comparable
however they were routed.

## The document gallery

`demo-tour gallery [dir]` (default `gallery/`) writes each
**document-authored** scene as a `.pncad` file the GUI can open:
`checks`, `ring`, `diefillet`, `heatsink` as single documents, plus the
assembly scene's workspace under `assembly/`. It authors them through the
same functions the tour renders — the gallery is the scenes, saved, not a
second spelling of them.

The rest of the tour drives the kernel API directly and has no document
to save; those scenes join the gallery as they are re-authored as
documents, which is per-scene library work.

Open one with `cargo run -p viewer --features app` and the toolbar's
`Open…`.

## Provenance: no fallback frame can be committed

The kernel lane falls back to the numpy+matplotlib STL renderer
(`render.py`) when FreeCAD is missing or its session does not complete.
Three layers keep such a frame out of a committed path:

* **Routing.** The fallback renders into `demos/renders-preview/renders/`
  — the preview tree mirrors the lane structure and is **gitignored**, so
  a fallback frame cannot be committed even by `git add -A`. It composes
  its own sheet there under a `PREVIEW ONLY` banner, and `render.sh`
  prints a loud stderr block naming the reason and the destination.
  Nothing under `renders/` is written on that path — not even
  `montage.png`, because recomposing the committed sheet from a stale or
  partial cell set is exactly the silent corruption this guards.
  (`renders-preview/renders-freecad/` never appears: the `--freecad` lane
  has no fallback by design — its whole point is the OCC reference
  render — and exits 1 instead.)
* **Staging.** A pass renders into `demos/out/stage/<lane>/` and is moved
  into the lane directory only once every scene is in hand, so no
  incomplete pass ever reaches `renders/` — not a crashed one, not a
  wedged one, not one killed at the terminal.
* **Guard.** `check_render_provenance.py` asserts that every committed
  per-scene PNG under `renders/` and `renders-freecad/` carries FreeCAD's
  signature `tEXt` chunks (`Author: FreeCAD (…)`, `Software: FreeCAD` —
  deterministic and provenance, so `strip_png_stamps.py` keeps them,
  unlike the wall-clock and output-path chunks it drops). A
  matplotlib-authored frame (`Software: Matplotlib …`) in a committed
  path fails loud, naming the file. Both `render.sh` lanes run it after
  the stamp strip and **before** composing the montage, so a sheet is
  never composed from an uncertified cell set; it is also an always-run
  row in `local-scripts/ci-local.sh` and a step in ci.yml's `discipline`
  job (stdlib only — no venv, no FreeCAD). The wild lane runs under the
  same guard with INVERTED per-lane rules: there matplotlib is the
  primary renderer, and cells must carry the wild lane's own `Author`
  stamp.

**The montage sheets are exempt, and here is why that is safe.** Both
FreeCAD-lane sheets are *composed* by matplotlib on purpose —
`compose_montage.py` lays the rendered cells out on a grid with captions
and a banner — so their `Software: Matplotlib` is correct, not a
fallback. The exemption is a positive assertion rather than a hole:
exactly the known sheet names are exempt, and each must actually carry
the matplotlib signature. The sheet's pixels are covered *indirectly*,
which is the honest statement of the guard's reach — a sheet is only ever
composed from the cells sitting beside it, and the guard certifies those
cells in the same pass, immediately before the compose step. What it
cannot see is a **stale** sheet (cells fixed afterwards without
recomposing); that is a `git status` / re-render question, and every lane
is byte-reproducible after the stamp strip.

`check_render_provenance.py --selftest` is the guard's own test:
synthetic PNGs (real chunk framing and CRCs, stdlib only) for the good
cell, the fallback cell, an unstamped cell, a sheet that is not a
matplotlib composition, a missing lane directory, and the wild lane's
inverted rules.

## The montage sheets

### The comparable pair: kernel vs FreeCAD/OCC

Two sheets with identical grids, captions, scene order, and cameras (both
read `scenes.json`) — cell-for-cell comparable, differing ONLY in whose
tessellation is on screen:

- `renders/montage.png` — **the kernel's own facets**. Every cell renders
  the tour's exported STL mesh, i.e. the trimmed/pcurve tessellation lane
  exactly as the kernel emitted it (flat-shaded chords on curved walls
  and all).
- `renders-freecad/montage-freecad.png` — **the FreeCAD/OCC reference**.
  Every cell is FreeCAD importing the body's OWN AP214 STEP export and
  letting OCC re-tessellate the B-rep — export → OCC import → render,
  dogfooded end-to-end.

**Both** sheets carry a provenance banner under the title naming whose
tessellation is on screen and pointing at the other sheet, so the two
superimpose exactly — cell for cell *and* banner for banner.

Reading a disagreement: the STEP lane is the reference rendering of the
*analytic surfaces the kernel claims to have exported*, and the kernel
lane is what our own tessellator makes of the same bodies — so a
cell-level mismatch is a visual differential with exactly two suspect
pools. Coarse-but-faithful shape (visible facets, correct silhouette) is
the expected gap: chordal, inscribed tessellation vs OCC's finer default
deflection. Wrong GEOMETRY in a cell pair — missing walls, displaced
features, a silhouette that differs beyond faceting — means either our
tessellator or our STEP writer is lying about the same body, and which
cell is wrong tells you which. A STEP-lane cell can also be a labeled
placeholder naming an import/render failure (per-scene `freecadcmd` with
a timeout; one bad scene costs one cell, never the sheet).

**The cells are every scene the tour marks `montage: true`.** The tour
prints the counts on its last line and `compose_montage.py` derives the
grid from the manifest — nothing is hardcoded, this file included.

**The rule for staying off the sheet**: a scene is `montage: false` when
it is a *proof* rather than a part — a shadow render (a silhouette read
against its own projection needs its own frame), a parameter variant
whose point is the comparison and not the shape, or a scene whose
interest is HOW it is built rather than how it looks. The authority is
the `montage` field at each `Stop`, and each `false` says why beside it.
Coming off the sheet costs a scene nothing else: it keeps its standalone
render, its narration, its assertions, and its corpus/latency/STEP roles.

Shadow proofs ride beside the montage panels as standalone renders. The
three-way silhouette solid viewed straight down each axis renders an **H**
(z), a **T** (x), and a **C** (y); the twisted duct gives a parabola down
z and a cubic S down y, which is the planarity refutation (affine
projections of a planar spine cannot differ in inflection count).

### The wild-corpus montage (`renders-wild/`)

A third sheet, deliberately unlike the pair above: **STEP files nobody on
this project authored** (the wild corpus,
`crates/step-import/tests/fixtures/wild/`), imported by `step-import` and
tessellated by the kernel's own tessellator — **KERNEL-TESSELLATION LANE
ONLY**, by Evan-approved scope. There is no FreeCAD import and no OCC
comparison lane for these files, so the sheet does not join the
superimposition contract; it keeps the same shape (grid, captions,
provenance banner via `compose_montage.py`) under its own title and
banner.

```sh
local-scripts/render-hosted.sh --lane wild   # the default path

# preview only — see "Preview mode: the local override"
cd demos/wild
cargo run --release -- out    # import + tessellate + STL + scenes.json
cd ..
CAD_RENDER_LOCAL_OVERRIDE=i-accept-local-render-drift ./render-wild.sh
```

**The cell set is license law plus pinned capability, not discovery.**
`docs/WILD-CORPUS-LICENSES.md` governs eligibility — only files the audit
marks render-OK may appear — and a file the importer refuses typed is
pinned as a refusal rather than dropped. `demos/wild/src/main.rs` pins
both and fails loudly on drift in any direction, so the sheet can never
detach silently from the attribution block below.

**Renderer + provenance.** Cells are drawn by `render.py` — the
numpy+matplotlib STL renderer — as the lane's PRIMARY renderer, not a
fallback: the facets on screen are exactly what `pncad::mesh::tessellate`
emitted for the imported body. Every cell carries the lane's own `Author`
stamp (`render.py --author=…`), and `check_render_provenance.py` runs
wild-lane rules over `renders-wild/`: a committed wild cell must be
matplotlib-drawn AND wild-stamped (a tour fallback frame or a FreeCAD
frame is refused), and `montage-wild.png` joins the positively-asserted
sheet exemption. matplotlib stamps no wall clock, so an unchanged
re-render is byte-identical.

#### Third-party source geometry — attribution

> **Third-party source geometry.** The bodies in this montage were imported from
> STEP files authored by others and tessellated by this project's own kernel; the
> rendered images are our derived work, the underlying models are not.
>
> **Adafruit parts** (`1982 MPR121`, `328 2500mAh battery`, `64 Halfsize
> Breadboard`, `805 slide switch`, `931 OLED 128x32 I2C`) from
> <https://github.com/adafruit/Adafruit_CAD_Parts>, used under the MIT License:
> *Copyright (c) 2016 Adafruit Industries. Permission is hereby granted, free of
> charge, to any person obtaining a copy of this software and associated
> documentation files (the "Software"), to deal in the Software without
> restriction… THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND.*
> Full text: `crates/step-import/tests/fixtures/wild/adafruit/LICENSE-adafruit.txt`.
>
> **NIST model** (`nist_ftc_09_asme1_rd`) from the National Institute of
> Standards and Technology's MBE PMI Validation and Conformance Testing project.
> Produced by an agency of the U.S. Government and not subject to copyright in
> the United States. Acknowledgement is given at NIST's request. *Neither NIST
> nor the U.S. Government endorses, recommends, or has any connection with this
> software; no NIST name or logo is used to imply endorsement.*
>
> **NIST model** (`nist_ftc_11_asme1_rb`) from the same NIST MBE PMI Validation
> and Conformance Testing project, on the same terms as above: a U.S. Government
> work not subject to copyright in the United States; acknowledgement given at
> NIST's request, and the same no-endorsement statement applies.
>
> **CadQuery test model** (`cq_red_cube_blue_cylinder`) from
> <https://github.com/CadQuery/cadquery> (`tests/testdata/red_cube_blue_cylinder.step`),
> used under the Apache License, Version 2.0 — full text committed at
> `crates/step-import/tests/fixtures/wild/occ-oss/LICENSE-cadquery.txt`. The
> geometry was modified only by our own tessellation (the rendered facets are
> this kernel's chordal approximation of the model's exact surfaces); CadQuery
> ships no NOTICE file (verified in the license audit).

`crates/step-import/NOTICE` carries the NOTICE text the audit requires
separately. A file that gains a cell must gain its attribution paragraph
in the same change — `demos/wild/src/main.rs` says so at the pin.

## The UV trim-loop lane (`render-uv.sh`)

The odd lane out: it draws no 3-D at all.

A `Surface` in this kernel is unbounded — "the infinite plane", "the
infinite cylinder". A `Face` is the patch of one that its boundary
**loops** cut out, and those loops live in the surface's own `(u, v)`
chart, stored as `geom_brep::Pcurve`s. That chart is *already* a 2-D
drawing, so rendering it needs no camera, no projection and no silhouette
machinery — which is why this is the one lane with **no external
dependency whatsoever**: the tour writes the per-face SVGs
(`demos/tour/src/uvdump.rs`, through `pncad::` like every other line of
the tour), and `compose_uv_montage.py` tiles them using Python's standard
library. No venv, no numpy/matplotlib, no `freecadcmd`.

Consequences worth stating:

* **The sheet is SVG, not PNG.** It is text, so an unchanged re-run
  produces a byte-identical file and `git status` stays clean with none
  of the wall-clock-stamp surgery the PNG lanes need. There is no
  provenance guard here because there is no second renderer to confuse it
  with — the kernel is the only thing that could have drawn it.
* **It is a diagnostic, not a depiction.** Per face the cell measures and
  prints: loop and half-edge counts, how many half-edges read a **stored**
  pcurve cache vs. were derived on demand (derived ones draw dashed —
  `mesh::trimmed` refuses those), the outer loop's signed chart area and
  its winding, and the worst **closure gap** between consecutive
  traversals. Winding is a *check*, not a readout: it is compared against
  the face's own `Face::sense` bit, since a bore or a concave groove
  carries `sense = false` and its outer loop is legitimately CW. Not every
  face is checkable — a chart carrying a branch jump has no meaningful
  shoelace — so the tour prints the split (checkable / branch-jumped /
  disagreeing) on every run and **fails outright on a disagreement**,
  which is why the alarm colour is reserved for a real contradiction
  rather than spent on every hole. Periodic charts get their seams
  (`u = k·2π`) drawn as dashed magenta lines, so a seam-crossing loop is
  visible rather than inferred. Strokes are colored by pcurve form —
  `Harmonic` blue, `IsoLine` green, `Fitted` orange.
* **Closure is measured in 3-D, not in the chart**, and that distinction
  is load-bearing. A chart-space closure metric false-alarms on every face
  touching a chart singularity or a seam, because at a sphere's pole an
  entire `u`-line is one 3-D point. Measured off the carriers instead, the
  closure gap stays at round-off. The tour prints how many charts carry a
  jump and the worst value of **both** measures per run. The chart jump is
  still drawn — greyed, and named as seam/pole structure — so it informs
  instead of alarming.
* **The interior fill is drawn only when it means something.** A ring that
  contains a branch jump — a loop crossing the seam or running through a
  pole — closes in the chart through a straight segment that is not
  boundary, so even-odd would shade a region that is not the face. Those
  cells show the strokes alone and say why; the signed area and winding
  are likewise not claimed there.
* **There is a CI drift gate**, and this is still the only lane that can
  have one. The obstacle is not FreeCAD availability — CI can run it —
  but that the runner image's mesa drifts month to month, so a firing PNG
  diff could be an image update rather than a geometry change; a standing
  CI gate for the PNG lanes needs the pinned-container work described in
  render.yml. This lane draws no 3-D, so its sheet is byte-reproducible
  anywhere: `uv sheet drift (demos)` regenerates it and diffs it, and a
  failure is either an uncommitted regeneration or a D9 determinism
  finding.
* **Nothing is refused.** Unlike the tessellator's trim walk, this one
  accepts every pcurve form and falls back to `topo::pcurve_of`'s
  derive-on-demand, because a face the tessellator refuses is exactly the
  face worth looking at. A face whose loops cannot be walked at all gets a
  cell naming the reason, first on the sheet — never a gap.
* **Selection is stated, never silent.** `out/uv.json` carries *every*
  face of every tour body. The sheet takes one representative per (body,
  chart kind) among the curved charts — the richest, by distinct pcurve
  forms then loop count then face ordinal — plus every failed walk
  unconditionally. Planar charts are dropped as a class: a plane chart's
  picture is the face's own outline, which the two 3-D lanes already show.
  The composer prints every count it dropped, and every SVG stays in
  `out/uv/` whether or not the sheet took it.

Read it in a browser; nested-SVG-shy rasterizers are why the cells are
placed with `transform="translate(…)"` rather than nested `<svg x= y=>`.

### What the sheet says about the corpus

Most cells are rectangles, and that is a fact about the corpus rather
than a limitation of the drawing: **a face whose surface came out of a
sweep has a boundary built entirely from iso-curves of its own chart, and
only a face cut obliquely to its chart carries a real trim.**

Every curved face here is *sweep-native*. Extrude, revolve, loft and
sweep choose the surface's chart so that one direction IS the sweep
parameter and the other IS the profile parameter — so a face's boundary
is the profile at the start (`v = const`), the profile at the end, and
the seams (`u = const`). Nothing is left to trim. This is what
`mesh::trimmed` means by "the definitional payoff — no fit anywhere", and
why `Pcurve::IsoLine` earns a variant of its own.

The tilted cut is different because its boundary did not come from the
sweep that made the cylinder: a plane cuts that cylinder **obliquely**, so
the section is an ellipse in 3-D and, on the cylinder chart (`u` =
azimuth, `v` = height), the sinusoid graph `v = a + b·cos(u − φ)` —
exactly the image `Pcurve::Harmonic`'s docs name.

No count of oblique faces can be read off what the lane writes: an
iso-curve of a swept chart is stored as a `Pcurve::Harmonic` whose
`cos`/`sin` coefficients are zero — the same variant carries every line
and conic of an affine plane chart — so neither `uv.json`'s form list nor
any printed line separates it from a genuine trim. The sheet is where you
see which faces those are.

Note what does *not* break the rectangle: `bossplate` is a genuine curved
boolean, and its cylinder walls are still iso-rectangles, because the
boss axis is perpendicular to the plate and the intersection circle
therefore sits at constant height. **Obliquity to the chart is what
produces a real trim, not the operation that made the edge.** The
consequence for imported geometry: the trimmed-face machinery
(`mesh::trimmed`'s CDT over an arbitrary trim polygon plus the even-odd
interior pick) is exercised by one geometric family in this corpus, and
foreign geometry will not be so courteous.

What it is NOT: a replacement for `render.sh`. The eyeball gate needs
shaded 3-D, and a chart domain is not a picture of the part. The parked
SVG lanes that *would* draw the part — a projected-edge wireframe, and
drawing-grade hidden-line removal — are filed as LONGTERM-IDEAS I4(a) and
I4(b).

## The stops

| scene | what it shows |
| --- | --- |
| `bracket` | extrude of a polyline + tangent-arc profile (the PATHS lattice, inner fillet) |
| `plate` | extrude with two circular holes — genus 2, ring loops in both caps |
| `vase` | full revolve, axis-touching profile: sphere-zone belly + cone lip |
| `sheave` | rope-groove sheave — full revolve of a polyline+arc profile: hub, web, **tapered (cone) rim shoulders**, semicircular groove whose OFF-axis arc sweeps a **ring-torus zone**; all four analytic wall kinds (plane/cylinder/cone/torus) on one part; genus 1; volume checked against the closed-form Pappus value |
| `chute` | quarter-turn chute — a C-channel profile swept through a **270° partial revolve**; wedge caps showing the profile, curved trough; Pappus-exact volume |
| `rocker` | **the fillet construction**: a rocker plate whose six corners are all authored through the PATHS fillet doors, covering arc×line, line×line, line×arc and — at the eye slot's rounded tip — **arc×arc**, where two tangent circles of the authored radius fit and the S8 rule **picks the one nearest the authored corner** (asserted, and narrated with both centres); genus 1 |
| `tiltedcut` | a cylinder cut by a tilted plane — the section edges carry an **exact `Curve3::Ellipse`** (a = r/cos φ, b = r); the cut walls tessellate **watertight** through the pcurve-driven trimmed lane, and the volume is a **certified quadrature enclosure** asserted to bracket πr²H/2 per half |
| `bossplate` | a three-arc cylindrical boss unioned into a plate — the seam is three exact `Circle` arcs, V = 16 + π·0.25·0.6 on the nose, and the shared-chord assertion pins that the curved wall and the ringed top face consume ONE chord set per seam edge, the claim no other scene makes |
| `tube_along_arc` | **the tube door, with its intent parameters STORED rather than reconstructed**: a ring-torus tube built from spine centre / axis / reference direction / major radius / angular window / minor radius. A `revolve` reaches the same walls but RECONSTRUCTS the tube radius from the profile's bulge arcs; this door keeps what it was given, and the scene asserts `minor_radius.to_bits()` against the authored value on **both** half-tube walls. Deliberately a WINDOWED tube, not the full donut, so all three parameters are visible — the ring's radius, the pipe's radius, and the window as the gap its two planar wedge caps close. No semantic fork: census, sense derivation, the `R > r > 0` convention and the pcurve mint are the revolve's own code; volume by Pappus π·r²·R·(t₁ − t₀) |
| `loft_prism` | **the first NURBS-walled render**: squares at z = 0/2, a NON-AFFINE trapezoid at z = 1, skinned at v-degree 2, so the four walls are genuinely curved degree-1×2 NURBS patches. The corpus fixture VERBATIM (`step-export/tests/common/mod.rs::loft_prism`, `editor-core/tests/corpus/loft_prism.rs`, `sweep/tests/m6_loft_body.rs`); volume DERIVED exactly: V = 8 + 8d/3 = 9 m³ (d = 0.375) |
| `nonuniform_loft` | `loft_prism`'s minimal pair: the SAME sections, the SAME height, ONLY the middle placement moved (z = 0, 0.15, 2 — the corpus fixture keeps its own spacing). The chord-length parameterization (t = 3√29/(3√29 + √5701) ≈ 0.1763) makes the degree-2 skin OVERSHOOT: a bulge wider than any authored section, at ~33% of height; derived V = 8 + 0.25/(t(1−t)) = 9.7219 m³ exactly (quadrature agrees to a small pad). Shares `loft_prism`'s camera so the pair reads as a pair |
| `s_duct` | the first CURVED-path sweep body: a 0.5 m square swept through an S — two OPPOSED quarter arcs of radius 2 (degree-3 interpolant through exact points), v-degree 3, path-following frame (planar path ⇒ no roll). Not the not-a-revolve claim: TWO GLUED partial revolves reach this shape, since each planar arc sweep is a partial revolve's orbit. Volume expectation A·L (curvature moment cancels) |
| `twisted_duct` | **nowhere-zero TORSION — the class NO assembly of revolves reaches**: a 0.5 m square swept along the twisted cubic (At, Bt², Ct³), degree-3 interpolant, v-degree 3. τ = 12ABC/\|r′×r″\|² has a constant numerator, so the spine is planar in NO plane and its curvature varies continuously too (no arc anywhere); a revolve's spine is a planar circular arc, and gluing revolves only concatenates planar arcs. The square visibly rolls as the bend plane turns. Two shadow proofs ride standalone (`twisted_duct_shadow_{z,y}`): a parabola down z, a one-inflection cubic S down y — parallel projections of a planar curve are affine images of each other and cannot differ in inflection count. Volume expectation A·L (centered symmetric section: curvature moment cancels, roll drops out) |
| `die` | 21 pip pockets across all six faces, 21 sequential Seamed subtracts, exact volume after every op |
| `table` | tabletop ∪ 4 corner-straddling legs; coplanar-touching and inset-overlap variants attempted and narrated live |
| `silhouette` | **first `intersect`**: one solid whose z-shadow is an H and x-shadow is a T (equal letter heights); the NAIVE coincident-plane variant's tier-3′ refusal is narrated — the coincidence ladder made visible |
| `silhouette3` | the H×T solid ∩ a blocky **C** prism along +y — intersect-of-intersect, boolean-of-boolean; all C planes axis-aligned yet sharing no carrier with any H/T plane |
| `crosslap` | cross-lap joint, assembled: two half-depth-notched beams (each a boolean result), UNIONED through the declared planar REST zip; the undeclared mate's typed refusal stays narrated |
| `crosslap_exploded` | the same joint exploded via `transform_rigid`, with re-minted witnesses |
| `twopeg` | **the declared CYLINDRICAL contact**: two 6×4×1 plates located on each other by a mating plane and two peg-in-hole fits — plate P is the plate ∪ two three-arc pegs, plate Q is the plate ∖ two through-bores, so both parts are boolean results and the mate is a boolean of booleans. Three declared `Rest` contacts (one planar, two cylindrical) unlock the zip; UNDECLARED the mate still refuses at the coincidence door, and that contrast is narrated live. Volume is EXACTLY additive against a closed form — vol(P) + vol(Q) = (24 + π/2) + (24 − π/2) = 48, bitwise — and full engagement removes every cylindrical patch, so the finished body carries no cylinder face at all: each peg survives as a rim circle, an inner ring on the plate's top |
| `twopeg_apart` | the same two parts apart, Q lifted by a rigid transform, so the three contacts are visible before the union makes them interior |
| `projectbox` | enclosure: cavity + 6 vent through-slots + 4 floor bosses + 4 pilot pockets — the longest sequential boolean chain |
| `cutaway` | **first `topo::split`**: the project box split by a tilted plane, halves translated apart — a machinist's section pair |
| `lily` | **the fairy lantern** (*Calochortus pulchellus*, the Mount Diablo globe lily) — the tour's organic subject and a deliberate stress test. The **ROOTSTOCK** is the plant's one JOIN: a corm revolved with a coaxial cylindrical socket authored into its meridian, and the stem's foot standing in it, glued on two declared `Rest` contacts of which one is CYLINDRICAL. The **stem** is torus-segment tubes from the tube door, walked by a turtle so consecutive arcs are **G1 by construction**, carrying the AUTHORED `minor_radius` rather than a bulge-arc reconstruction of it. The **lantern** is a sphere zone from `revolve(Full)`, with a conical mouth below and a neck cone above cut at the arch tube's own radius — its rim IS that tube's terminal meridian circle, so flower and stem meet on one shared circle rather than crossing. The **bud** is that same meridian said three times PARTIALLY: pre-tepals on three axes forming a narrow tripod about the bud's own, sharing the attachment so the tilt splays their tips, and rolled a quarter turn off their own radius so they nest chirally. The **blades** are the fitted pieces, a B-spline wall through exact spine points — the SWEPT ones hold one width base to tip and never roll, because `sweep_body` takes one profile and derives its own frame; the LOFTED ones do both, because `loft_body` takes sections and placements as separate lists, so the long basal leaf runs rectangle to wide diamond to small diamond while turning about its own spine, and the sepals stand TANGENT to the globe with the stand-off set to the section's own keel. Blade sections are straight lines today; restoring the lanceolate arcs is outstanding work on this stop and no longer gated on the kernel, since the span meter's rational arm landed (`sweep`'s `cert5_offgrid_knot_rational::the_lily_crescent_blade_certifies` is the standing row). Everything ELSE is set beside its neighbour rather than welded, and the stop is followed by **live wall probes** that attempt the joins and shapes a plant actually wants and assert each typed refusal, panicking if one ever retires |
| `klein` | **the Klein bottle** — the tour's non-orientable stop, and its densest wall list. A 2-manifold is not a body this kernel holds (D1 is manifold-and-solid-first), so the model is the honest 3-D stand-in: a THIN 3-manifold, wall 0.05 m, whose midsurface is the classic immersed Klein bottle. The **bulb** — neck, flaring body wall, the wide bottom rim the surface turns back on, and the straight tube coming back UP through that rim's hole — is ONE `revolve(Full)` of ONE meridian band, so cylinder/torus/cone/torus/cylinder plus two annular caps are all exact and every blend is an ARC IN THE MERIDIAN rather than a rolling ball afterwards, which is the better construction for coaxial supports and the one `fillet_edges` cannot make. The **top loop** is two thin elbows, `revolve(Partial)` of the annular section, 270° over the top and 90° turning back onto the axis — two arcs because ONE circle cannot be tangent to the bottle's axis at two different heights, which is geometry and not a kernel limit. The three bodies MEET on coincident annular faces and NONE can be joined: the boolean operand gate is per-face-kind and rejects any body carrying a cone or a torus, so the self-intersection an immersed Klein bottle must have is left un-trimmed too. Rendered SEE-THROUGH (the manifest's per-body `transparency`) from a camera deliberately out of the model's symmetry plane: the subject is what happens inside the bulb. Followed by **live wall probes**, one of which pins a DEFECT rather than an absence — `mesh::planar`'s banked sub-floor chart residue, which this bulb's annular cap is what hit |
| `heatsink5/7/9` | **the recipe layer**: ONE document, fin count 5 → 7 → 9 via `SetStructuralParam` on a `LinearPattern`; each re-eval recomputes exactly 1 node and reuses 4 (counted in the caption); stable names survive the edits |

## Validation posture (tier 3′)

Boolean stops validate the ACTUAL result body via
`validate_pseudomanifold` with the op's own declared `contacts`.
Non-boolean bodies run the plain tier-3 geometric gate; on contact-free
bodies the two gates agree.

Every scene body pre-flights tiers 1–2, prints exact B-rep volume/area
from `topo::mass_properties`, and cross-checks the tessellation's signed
volume. Boolean scenes assert exact (dyadic / closed-form) volume oracles
after EVERY op.

Every stop runs that standard ladder; there are no staged bodies. The
pattern for the next frontier is available where one is needed: pin the
honest refusal, and make the pin panic when the door opens.
`skinned.rs`'s narration stays as the geometry layer (control nets,
weights, the measured interpolation claim), which no render can show.

The tour's coda feeds a self-intersecting (bowtie) profile to
`Profile::validate` and prints the typed rejection — the fail-loud
contract, demonstrated rather than claimed.

## The STEP lane

Every scene body exports an AP214 STEP file beside its STL, through the
in-house writer's analytic subset: the whole elementary-surface
vocabulary (`PLANE`, `CYLINDRICAL_`, `CONICAL_`, `SPHERICAL_`,
`TOROIDAL_SURFACE`) plus `B_SPLINE_SURFACE_WITH_KNOTS`, with
`LINE`/`CIRCLE`/`ELLIPSE`/`B_SPLINE_CURVE_WITH_KNOTS` carriers. Every arm
is an **exact native entity**: a cylinder leaves as a cylinder, never as
a spline approximation of one. The tour fails loud if a body it expects
to export does not, and the STEP-lane montage draws every exported body
from its own AP214 file.

`same_sense = .F.` marks a concave ANALYTIC wall, whose chart has a
canonical normal the wall may oppose. A NURBS wall's description is
authored by the loft/sweep assembly itself, outward by construction, so a
skinned body carries none regardless of concavity — the s_duct's and
twisted duct's inner walls are concave and still `.T.`. A revolve mints
both cap planes on the profile plane's own +y normal, so exactly one cap
opposes the solid's outward normal: a `.F.` that is not on a curved wall
at all.

The lily is the widest single-scene spread the writer has been asked for
— `TOROIDAL_` (stem tubes), `SPHERICAL_` + `CONICAL_` (lanterns) and
`B_SPLINE_SURFACE_WITH_KNOTS` (leaf blades) in one cell. Its analytic
bodies are checked against independent closed forms (Pappus for the torus
segments, a zone-plus-frustum integral for the lanterns); its swept
blades have no analytic wall to check that way, so they are pinned
against Pappus on the MESH instead — kite area times the centroid's arc
length, against a **two-sided** band, since exact agreement would mean no
real mesh was measured.

One typed refusal remains as a named frontier, and no tour body is in it:
a multi-shell **curved** solid, whose outward/void classification has no
closed form yet (`CurvedShellClassification`).

## Renderers

`render.sh` (kernel lane) prefers **headless FreeCAD** (`freecadcmd`,
`QT_QPA_PLATFORM=offscreen`, no display/Xvfb) importing the tour's STL
meshes — the facets on screen are the kernel's own tessellation
regardless of renderer, which is why the STL source is unconditional here
and the STEP imports live in the `--freecad` lane. Set `FREECADCMD` to
override the binary location. Within one scene the bodies share one warm
document with visibility toggling (per-scene document cycling races the
offscreen view-provider setup — observed as blank frames and hangs).
freecadcmd's Qt teardown can crash AFTER a successful render, so a scene
counts as rendered when its PNG exists, never by exit status.

`render.sh --freecad` (STEP lane) has no matplotlib fallback: its whole
point is the OCC reference render, so a missing `freecadcmd` is a loud
exit. A scene it genuinely cannot import or render costs one cell — the
reason lands in `renders-freecad/<scene>.fail.txt` (full log under
`out/freecad-logs/`) and `compose_montage.py` draws a labeled placeholder
naming it, never a silent gap.

`render.py` is the zero-dependency renderer (numpy + matplotlib, pure
CPU, demo-local venv): binary-STL parsing, flat shading, exact backface
culling (guaranteed by tier 3's +V invariant). It is the kernel lane's
fallback and the wild lane's primary.

`compose_montage.py` builds a montage sheet from the per-scene PNGs in
`scenes.json` order with captions; `--montage=NAME` / `--banner=TEXT`
give a lane its own filename and provenance banner on the same grid.

`manifest.py` is the one reader of `scenes.json` — imported by both
renderers and the composer, and run as `manifest.py --scene-names` by
`render.sh`'s scene loop. It holds the field names, the walk, and the
`view.up` convention; the last of these in the world → display direction
only, with the display → world direction a camera needs *derived* from
it, so the two cannot drift apart. Every field it names is read, never
defaulted: both producers write all of them for every entry, so a missing
one refuses (naming the scene, the body and the key) instead of rendering
something plausible. `python3 manifest.py --selftest` pins all of that.
The UV lane's `uv.json` has one reader (`compose_uv_montage.py`) and is
walked there.

### Transparency is a scene property, not a renderer setting

`scenes.json` carries a per-body `transparency` (0–100, 0 = opaque), and
both PNG lanes honour it: FreeCAD sets `ViewObject.Transparency`, the
matplotlib renderer keeps backfaces (culling them is what a see-through
body is *for*) and drops the edge strokes, which would otherwise
double-darken every triangle boundary and read as a wire mesh. It lives
in the manifest rather than in a renderer because it is a claim about the
SHAPE: the `klein` bottle's subject is the neck crossing the body wall
and running down inside it, which no opaque render shows at any camera.
Every other scene emits `0` and takes the byte-identical path.

### One process per scene, on a budget

**Both** FreeCAD lanes run one `freecadcmd` process per scene by default,
each under a per-scene wall-clock budget (`FREECAD_SCENE_TIMEOUT`,
default 300 s; `render.sh` documents how that number was measured). The
process boundary is what BOUNDS a hang.

Each attempt runs in its own session, so the budget covers the process
*tree*: when it expires the whole group is killed and the scene is
retried **once**, in a fresh process. A second expiry is a loud, named
failure that ends the pass — never a silent skip, never a degraded cell.
Two signals come out with it: how long the process had been silent (a
slow scene keeps writing to its log; a wedged one goes quiet), and, when
the frame was written but the process still had to be killed, a note
saying so — a post-render stall costs one budget and is never mistaken
for a good pass. Because frames are staged and published only on a
complete pass, a wedge leaves the committed lane directory exactly as it
was.

`CAD_RENDER_BATCH=B` renders B consecutive scenes (scenes.json order) in
ONE process, which pays FreeCAD's startup once per B scenes instead of
once per scene — and startup is most of a typical scene, which is why
scenes of wildly different geometric complexity all cost about the same.
**Default 1: exactly the behaviour above, down to the log filenames.**
The trade it makes is blast radius, and it is linear: a wedge costs its
whole batch, and the pass takes up to `2 x B x FREECAD_SCENE_TIMEOUT` to
give up on it rather than `2 x FREECAD_SCENE_TIMEOUT`. A failure that is
NOT a wedge is split — each frameless scene of the batch is re-run alone,
one process each — so one scene FreeCAD cannot draw still costs one cell
and not its whole batch, exactly as at B=1.

`CAD_RENDER_JOBS=K` renders K scenes concurrently — still one fresh
session per scene — and prints the pass's wall clock next to the summed
per-scene times, which stop being the same number above K=1. What
concurrency trades against is the budget: K scenes on a K-core box push
every scene toward the contended figure. Keep K at or under the core
count, and treat sequential as the reference — it is what the committed
cells were rendered under.

#### Batching is byte-checked

`CAD_RENDER_BATCH` is admissible only because a batched frame is
BYTE-IDENTICAL to an unbatched one — the same bar `CAD_RENDER_JOBS` had
to clear, verified across every committed cell of both lanes plus both
sheets at several batch sizes. PNG bytes are not comparable ACROSS GL
stacks, so that is a statement about a repeat render on one box; the
canonical hosted producer has to make it again for itself before the
default moves off 1.

What makes it safe is that `render_freecad.py` keeps ONE warm document
and toggles per-scene visibility rather than cycling documents — the
document accumulates the batch's bodies, but a hidden body contributes
nothing to a render or to `fitAll`, and the camera is set outright from
the scene's own spec before every frame. (Per-scene
`newDocument`/`closeDocument` is worse: it races the event-loop-deferred
view-provider setup offscreen, which shows up as blank frames and hangs.)
