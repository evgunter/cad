# Pre-GUI demo tour

A visual tour of what the kernel can do today, from a pure outside
consumer's seat: sweep bodies through the public `profile` / `sweep`
APIs, a boolean leg through the M3 `union` / `subtract` / `intersect`
ops (boolean-of-boolean chains included), the first `topo::split`
cutaway, and the M4 recipe layer (editor-core document, structural
edit, downstream-only recompute, stable names) — narrated (operations
used, topology census + genus, validation tiers passed, exact-vs-meshed
mass properties), exported as binary STL + AP214 STEP, and rendered to
PNG.

This directory is **deliberately outside the cargo workspace** (root
manifest `workspace.exclude`, plus the empty `[workspace]` table in
`tour/Cargo.toml`): viewer/render tooling is demo-only and must never
become a kernel dependency.

## Run

```sh
cd demos/tour
cargo run --release -- ../out   # build + narrate + export STL/STEP + scenes.json
cd ..
./render.sh                     # kernel-tessellation montage (renders/montage.png)
./render.sh --freecad           # FreeCAD/OCC STEP-lane montage (renders-freecad/montage-freecad.png)
./render-uv.sh                  # UV trim-loop sheet (renders-uv/montage-uv.svg)
```

Outputs: `demos/out/*.{stl,step}` + `demos/out/scenes.json` +
`demos/out/uv/*.svg` + `demos/out/uv.json` (untracked),
`demos/renders/*.png` (tracked — one per scene plus `montage.png`),
`demos/renders-freecad/*.png` (tracked — the montage cells plus
`montage-freecad.png`), `demos/renders-uv/montage-uv.svg` (tracked),
and — only when the kernel lane falls back to matplotlib —
`demos/renders-preview/renders/*.png` (gitignored; see below).

A pass in flight lives in `demos/out/stage/<lane>/` (untracked) and is
published to the lane directory only once it is complete. FreeCAD
stamps each PNG with the path it was written to, so the staging tree
mirrors the lane directory's *name* and each scene process runs with
the staging root as its working directory — a staged frame is
byte-identical to the published one.

Both `render.sh` lanes run `strip_png_stamps.py` over the per-scene
PNGs before composing the montage: FreeCAD's `saveImage` stamps the
wall clock into every file it writes (a `tEXt` "Creation Time" chunk
and a `zTXt` "Description" chunk carrying its MIBA XML), which would
make an unchanged re-render show up dirty in `git status`. Both are
ancillary chunks — dropping them is lossless, and it makes a dirty
`git status` after a re-render mean the *pixels* changed.

## The UV trim-loop lane (`render-uv.sh`)

The third montage lane, and the odd one out: it draws no 3-D at all.

A `Surface` in this kernel is unbounded — "the infinite plane", "the
infinite cylinder". A `Face` is the patch of one that its boundary
**loops** cut out, and those loops live in the surface's own `(u, v)`
chart, stored as `geom_brep::Pcurve`s. That chart is *already* a 2-D
drawing, so rendering it needs no camera, no projection and no
silhouette machinery — which is why this is the one lane with **no
external dependency whatsoever**: the tour writes the per-face SVGs
(`demos/tour/src/uvdump.rs`, through `pncad::` like every other line
of the tour), and `compose_uv_montage.py` tiles them using Python's
standard library. No venv, no numpy/matplotlib, no `freecadcmd`.

Consequences worth stating:

* **The sheet is SVG, not PNG.** It is text, so an unchanged re-run
  produces a byte-identical file and `git status` stays clean with none
  of the wall-clock-stamp surgery the PNG lanes need. There is no
  provenance guard here because there is no second renderer to confuse
  it with — the kernel is the only thing that could have drawn it.
* **It is a diagnostic, not a depiction.** Per face the cell measures
  and prints: loop and half-edge counts, how many half-edges read a
  **stored** pcurve cache vs. were derived on demand (derived ones draw
  dashed — `mesh::trimmed` refuses those), the outer loop's signed
  chart area and its winding, and the worst **closure gap** between
  consecutive traversals. Winding is a *check*, not a readout: it is
  compared against the face's own `Face::sense` bit, since a bore or a
  concave groove carries `sense = false` and its outer loop is
  legitimately CW. 879 of the 982 M7 faces are checkable (the rest
  carry a branch jump) and all 879 agree, so the alarm colour is
  reserved for a real contradiction rather than spent on every hole.
  Periodic charts get their seams (`u = k·2π`)
  drawn as dashed magenta lines, so a seam-crossing loop is visible
  rather than inferred. Strokes are colored by pcurve form —
  `Harmonic` blue, `IsoLine` green, `Fitted` orange.
* **Closure is measured in 3-D, not in the chart**, and that
  distinction is load-bearing. A chart-space closure metric
  false-alarms on every face touching a chart singularity or a seam,
  because at a sphere's pole an entire `u`-line is one 3-D point: 103
  of the 982 M7 faces show such a jump, every one of them exactly π/2,
  π or 2π. Measured off the carriers instead, the true closure gap
  never exceeds 9e-16 m anywhere in the corpus. The chart jump is
  still printed — greyed, and named as seam/pole structure — so it
  informs instead of alarming.
* **The interior fill is drawn only when it means something.** A ring
  that contains a branch jump — a loop crossing the seam or running
  through a pole — closes in the chart through a straight segment that
  is not boundary, so even-odd would shade a region that is not the
  face. Those cells (3 of 36 on the sheet) show the strokes alone and
  say why; the signed area and winding are likewise not claimed there.
* **There is a CI drift gate**, and this is still the only lane that
  can have one — but the reason has moved. CI *can* run FreeCAD (both
  `step-import` and the hosted render lanes provision the same pinned
  AppImage), so the obstacle is no longer availability: it is that the
  PNG lanes' pixels come out of a GL stack, and a runner's llvmpipe is
  not this host's, so "unchanged" is not a property a hosted PNG pass
  can assert. This lane draws no 3-D, so its sheet is byte-reproducible
  anywhere. `uv sheet drift (demos)` regenerates it and diffs it (the
  tour is ~3s once built, and the sheet is text, so a firing diff is
  readable). A failure is either an uncommitted regeneration or a D9
  determinism finding.
* **Nothing is refused.** Unlike the tessellator's trim walk, this one
  accepts every pcurve form and falls back to `topo::pcurve_of`'s
  derive-on-demand, because a face the tessellator refuses is exactly
  the face worth looking at. A face whose loops cannot be walked at all
  gets a cell naming the reason, first on the sheet — never a gap.
* **Selection is stated, never silent.** `out/uv.json` carries *every*
  face of every tour body (982 at M7). The sheet takes one
  representative per (body, chart kind) among the curved charts — the
  richest, by distinct pcurve forms then loop count then face ordinal —
  plus every failed walk unconditionally. Planar charts are dropped as
  a class: a plane chart's picture is the face's own outline, which the
  two 3-D lanes already show. The composer prints every count it
  dropped, and all 982 SVGs stay in `out/uv/`.

Read it in a browser; nested-SVG-shy rasterizers are why the cells are
placed with `transform="translate(…)"` rather than nested `<svg x= y=>`.

### What the sheet says about the corpus today (M7)

Most cells are rectangles, and that is a fact about the corpus rather
than a limitation of the drawing. Of the 238 curved faces, **234 have
boundaries built entirely from iso-curves of their own chart; only 4
do not, and all 4 are the tilted cut.**

The reason is that every curved face here is *sweep-native*. Extrude,
revolve, loft and sweep choose the surface's chart so that one
direction IS the sweep parameter and the other IS the profile
parameter — so a face's boundary is the profile at the start
(`v = const`), the profile at the end (`v = const`), and the seams
(`u = const`). Nothing is left to trim. This is what `mesh::trimmed`
means by "the definitional payoff — no fit anywhere", and why
`Pcurve::IsoLine` earns a variant of its own.

The tilted cut is different because its boundary did not come from the
sweep that made the cylinder: a plane cuts that cylinder **obliquely**,
so the section is an ellipse in 3-D and, on the cylinder chart
(`u` = azimuth, `v` = height), the sinusoid graph
`v = a + b·cos(u − φ)` — exactly the image `Pcurve::Harmonic`'s docs
name.

Note what does *not* break the rectangle: `bossplate` is a genuine
curved boolean, and its cylinder walls are still iso-rectangles,
because the boss axis is perpendicular to the plate and the
intersection circle therefore sits at constant height. **Obliquity to
the chart is what produces a real trim, not the operation that made
the edge.**

Consequence worth carrying into M7: the trimmed-face machinery
(`mesh::trimmed`'s CDT over an arbitrary trim polygon plus the
even-odd interior pick) is exercised by exactly one geometric family
in this corpus. Everything else takes the swept-rectangle walk or
trims along iso-lines. Imported foreign geometry will not be so
courteous.

What it is NOT: a replacement for `render.sh`. The eyeball gate needs
shaded 3-D, and a chart domain is not a picture of the part. The
parked SVG lanes that *would* draw the part — a projected-edge
wireframe, and drawing-grade hidden-line removal — are filed as
LONGTERM-IDEAS I4(a) and I4(b).

## The matplotlib fallback is uncommittable (#221)

The kernel lane falls back to the numpy+matplotlib STL renderer
(`render.py`) when FreeCAD is missing or its session does not complete.
That fallback used to write `renders/` — the same directory FreeCAD
writes — so a fallback frame was indistinguishable from a real one at
the filesystem level, and one silently reached a committed montage cell
(repaired in #221). Two layers now make that structurally impossible:

* **Routing.** The fallback renders into `demos/renders-preview/renders/`
  — the preview tree mirrors the lane structure and is **gitignored**,
  so a fallback frame cannot be committed even by `git add -A`. It
  composes its own sheet there, under a `PREVIEW ONLY` banner, and
  `render.sh` prints a loud stderr block naming the reason and the
  destination. Nothing under `renders/` is written on that path — not
  even `montage.png`, because recomposing the committed sheet from a
  stale or partial cell set is exactly the silent corruption #221 hit.
  (`renders-preview/renders-freecad/` never appears: the `--freecad`
  lane has no fallback by design — its whole point is the OCC reference
  render — it exits 1 instead.)
* **Staging.** A pass renders into `demos/out/stage/<lane>/` (untracked)
  and is moved into the lane directory only once every scene is in
  hand, so *no* incomplete pass ever reaches `renders/` — not a
  crashed one, not a wedged one, not one killed at the terminal.
  Before this, a FreeCAD session that died mid-pass left a partial
  FreeCAD-authored set behind, visible in `git status` but still
  sitting in the committed path.
* **Guard.** `check_render_provenance.py` asserts that every committed
  per-scene PNG under `renders/` and `renders-freecad/` carries
  FreeCAD's signature `tEXt` chunks (`Author: FreeCAD (…)`, `Software:
  FreeCAD` — deterministic, so `strip_png_stamps.py` keeps them, unlike
  the two wall-clock chunks it drops). A matplotlib-authored frame
  (`Software: Matplotlib …`) in a committed path fails loud, naming the
  file. Both `render.sh` lanes run it after the stamp strip and
  **before** composing the montage, so a sheet is never composed from an
  uncertified cell set; it is also an always-run row in
  `scripts/ci-local.sh` and a step in ci.yml's `discipline` job (stdlib
  only — no venv, no FreeCAD). The wild-corpus lane (`renders-wild/`)
  runs under the same guard with INVERTED per-lane rules — there
  matplotlib is the primary renderer, and cells must carry the wild
  lane's own `Author` stamp (see the wild-corpus montage section).

**The montage sheets are exempt, and here is why that is safe.** Both
sheets (`renders/montage.png`, `renders-freecad/montage-freecad.png`)
are *composed* by matplotlib on purpose — `compose_montage.py` lays the
FreeCAD-rendered cells out on a grid with captions and a banner — so
their `Software: Matplotlib` is correct, not a fallback, and the guard
cannot demand a FreeCAD signature of them. The exemption is a positive
assertion rather than a hole: exactly the two known sheet names are
exempt, and each of them must actually carry the matplotlib signature.
The sheet's pixels are covered *indirectly*, which is the honest
statement of the guard's reach — a sheet is only ever composed from the
cells sitting beside it, and the guard certifies those cells in the same
pass, immediately before the compose step. What it cannot see is a
**stale** sheet (cells fixed afterwards without recomposing); that is a
`git status` / re-render question, and both lanes are byte-reproducible
after the stamp strip.

`check_render_provenance.py --selftest` is the guard's own test:
synthetic PNGs (real chunk framing and CRCs, stdlib only) for the good
cell, the fallback cell, an unstamped cell, a sheet that is not a
matplotlib composition, a missing lane directory, and the wild lane's
inverted rules (a stamped wild cell + sheet pass; an unstamped
matplotlib frame, a FreeCAD frame, and a wild frame outside its lane
are each refused — see the wild-corpus montage section).

## The two montages (#159)

The tour ships **two montage sheets** with identical grids, captions,
scene order, and cameras (both read `scenes.json`) — cell-for-cell
comparable, differing ONLY in whose tessellation is on screen:

**Cell count: 19 on each sheet** (4 columns × 5 rows, last row short
by one — `compose_montage.py` derives the row count, nothing is
hardcoded). The derivation: the tour emits 34 scenes, of which 15 are
`montage: false` — the four standalone renders kept out of the sheet
by the #91 revision notes and the M6 curation pass (`bracket`, `die`,
`silhouette`, `az`), the two under-filled heat-sink variants
(`heatsink5`, `heatsink7` — the sheet carries only the 9-fin panel),
the five shadow proofs (`silhouette3_shadow_{z,x,y}`,
`twisted_duct_shadow_{z,y}`), and the four scenes the **montage-v2
curation** (Evan's #218 follow-up) moved to standalone:
`tube_along_arc`, `diefillet`, `diepips` (interesting for how they
work — stored intent parameters, the fillet battery, the closed-group
cut — but not visually without that context), and `s_duct` (its S
solid is two glued partial revolves, shape for shape, so the honest
not-a-revolve sweep cell is now `twisted_duct`). 34 − 15 = 19. The
count was 18 at the globe lily; the **montage refresh** added
`tube_along_arc` (19), the trimmed-NURBS tessellation lane landed the
refresh's three blocked NURBS-walled scenes (22), the #218 review
re-posed the sweep cell as `s_duct`, and **montage-v2** cut four cells
and added `twisted_duct` (19).

- `renders/montage.png` — **the kernel's own facets**. Every cell
  renders the tour's exported STL mesh, i.e. the M5 trimmed/pcurve
  tessellation lane exactly as the kernel emitted it (flat-shaded
  chords on curved walls and all).
- `renders-freecad/montage-freecad.png` — **the FreeCAD/OCC reference**.
  Every cell is FreeCAD importing the
  body's OWN AP214 STEP export and letting OCC re-tessellate the
  B-rep — export → OCC import → render, the F6 lane dogfooded
  end-to-end.

**Both** sheets carry a provenance banner under the title naming whose
tessellation is on screen and pointing at the other sheet, so the two
superimpose exactly — cell for cell *and* banner for banner. (The
kernel sheet's banner is the M6 curation pass; before it, only the
STEP lane was labelled and the sheets were one text line out of
register.)

Reading a disagreement: the STEP lane is the reference rendering of
the *analytic surfaces the kernel claims to have exported*, and the
kernel lane is what our own tessellator makes of the same bodies — so
a cell-level mismatch is a visual differential with exactly two
suspect pools. Coarse-but-faithful shape (visible facets, correct
silhouette) is the expected gap: chordal, inscribed tessellation vs
OCC's finer default deflection. Wrong GEOMETRY in a cell pair —
missing walls, displaced features, a silhouette that differs beyond
faceting — means either our tessellator or our STEP writer is lying
about the same body, and which cell is wrong tells you which. A
STEP-lane cell can also be a labeled placeholder naming an import/
render failure (per-scene `freecadcmd` with a timeout; one bad scene
costs one cell, never the sheet).

## The wild-corpus montage (`renders-wild/`)

A third sheet, deliberately unlike the two above: **STEP files nobody
on this project authored** (the M7-4 wild corpus,
`crates/step-import/tests/fixtures/wild/`), imported by
`step-import` and tessellated by the kernel's own tessellator —
**KERNEL-TESSELLATION LANE ONLY**, by Evan-approved scope (2026-08-09).
There is no FreeCAD import and no OCC comparison lane for these files,
so the sheet does not join the two-sheet superimposition contract; it
keeps the same shape (grid, captions, provenance banner via
`compose_montage.py`) under its own title and banner.

```sh
cd demos/wild
cargo run --release -- out    # import + tessellate + STL + scenes.json
cd ..
./render-wild.sh              # cells + sheet -> renders-wild/
```

**Cell count: 8, and the cell set is license law plus pinned
capability, not discovery.** `docs/WILD-CORPUS-LICENSES.md` (the
license audit) governs eligibility — only files the audit marks
render-OK may appear. The derivation: 13 wild fixtures − 4 `stepcode/`
files **license-EXCLUDED** by the audit's D2 (unclear upstream rights
for redistributed CAx-IF models; the generator does not read them at
all — `sg1-c5-214.stp` imports fine and is excluded by license, not
capability) = 9 render-OK, − 1 typed import refusal
(`b123d_nema17_bracket.step`, `SURFACE_CURVE` edge geometry — pinned
in the generator, matching `wild.rs`) = **8**. Two notes on how that
differs from the audit's own snapshot:

* the audit's import-status line ("only 6 import today") predates the
  M7-5 band-seam re-mint (#252), which flipped `nist_ftc_11_asme1_rb`
  and `cq_red_cube_blue_cylinder` to imports-class — both are
  render-OK rows in the audit's own table, so both are cells;
* **the mesh-lane finding this unit surfaced, since resolved**:
  `1982_MPR121` and `328_2500mAh_battery` imported first-class
  (census exact, volumes measurable) but refused
  `pncad::mesh::tessellate` typed (`Triangulation`), on plain
  rectangular planar faces — the files' plane axes carry translator
  noise (~1e-33 components), the planar chart projection of a
  should-be-zero coordinate landed at ~1e-67, below spade's
  coordinate domain (`MIN_ALLOWED_VALUE` = 2⁻¹⁴² ≈ 1.79e-43), and
  the CDT refused the vertex. Fixed in the mesh lane (#284,
  `mesh::planar`'s module docs): the planar chart frame is re-derived
  per-face from the boundary itself (Newell normal + extent-aligned
  axes) instead of trusting stored axes, so both files are ordinary
  cells now.

`demos/wild/src/main.rs` pins the cell set AND the import refusal,
and fails loudly on drift in any direction, so the sheet can never
detach silently from the attribution block below.

**Renderer + provenance.** Cells are drawn by `render.py` — the
numpy+matplotlib STL renderer — as the lane's PRIMARY renderer, not a
fallback: the facets on screen are exactly what
`pncad::mesh::tessellate` emitted for the imported body. Every cell
carries the lane's own `Author` stamp (`render.py --author=…`), and
`check_render_provenance.py` runs wild-lane rules over
`renders-wild/`: a committed wild cell must be matplotlib-drawn AND
wild-stamped (a tour fallback frame or a FreeCAD frame is refused),
and `montage-wild.png` joins the positively-asserted sheet exemption.
matplotlib stamps no wall clock, so an unchanged re-render is
byte-identical.

### Third-party source geometry — attribution

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

The first three paragraphs are the audit's paste-ready block,
verbatim. The last two are the extension the audit itself prescribes
for the two post-audit arrivals (its "if a future montage adds the
Apache-2.0 files once the periodic-band gap closes" instruction — the
M7-5 seam re-mint closed that gap): the second NIST file rides the
NIST terms, and the CadQuery entry carries the source URL, the
Apache-2.0 grant linking the committed license text, and the
modified-only-by-tessellation statement. `b123d_nema17_bracket.step`
still refuses import (no cell), so its NOTICE-carrying entry is not
yet needed here; the NOTICE text already rides
`crates/step-import/NOTICE` (the audit's D1 action, done). All five
named Adafruit parts now appear on the sheet — `1982 MPR121` and
`328 2500mAh battery`, once pinned tessellation refusals, joined
when the #284 mesh fix landed — and the Adafruit entry already named
them all, so the block needed no change: it is the audit's text
verbatim.

## The stops

| scene | what it shows |
| --- | --- |
| `bracket` | extrude of a polyline + tangent-arc profile (`LoopBuilder`, inner fillet); standalone render since the M6 curation pass |
| `plate` | extrude with two circular holes — genus 2, ring loops in both caps |
| `vase` | full revolve, axis-touching profile: sphere-zone belly + cone lip |
| `sheave` | rope-groove sheave — full revolve of a polyline+arc profile: hub, web, **tapered (cone) rim shoulders**, semicircular groove whose OFF-axis arc sweeps a **ring-torus zone**; all four analytic wall kinds (plane/cylinder/cone/torus) on one part; genus 1; volume checked against the closed-form Pappus value |
| `chute` | quarter-turn chute — a C-channel profile swept through a **270° partial revolve**; wedge caps showing the profile, curved trough; Pappus-exact volume |
| `rocker` | **the M5 fillet sugar**: a rocker plate whose SIX corners are all authored by `LoopBuilder::fillet_corner` — arc×line, line×line, line×arc, arc×line, line×arc around the outline, and **arc×arc** at the eye slot's rounded tip, where two tangent circles of the authored radius fit and the S8 rule **picks the one nearest the authored corner** (asserted, and narrated with both centres); genus 1; montage panel (the sheet's profile-fillet cell since the M6 curation pass) |
| `tiltedcut` | **RENDERING (M5 PR 11, the milestone's demo moment)**: a cylinder cut by a tilted plane — the section edges carry an **exact `Curve3::Ellipse`** (a = r/cos φ, b = r, residual ~1e-16, PR 5 shape (i)); the cut walls tessellate **watertight** through the pcurve-driven trimmed lane, and the volume is a **certified quadrature enclosure** (± ~1e-6 m³) asserted to bracket πr²H/2 per half; montage panel |
| `bossplate` | **the first curved boolean, visible (M5 PR 11)**: a three-arc cylindrical boss unioned into a plate (PR 9 shape (ii)) — the seam is three exact `Circle` arcs, V = 16 + π·0.25·0.6 on the nose, and the shared-chord assertion pins that the curved wall and the ringed top face consume ONE chord set per seam edge; montage panel
| `tube_along_arc` | **the tube door, with its intent parameters on screen** (M6-3 Leg F, the Evan-ratified rider on the #175 thread): a ring-torus tube built from spine centre / axis / reference direction / major radius 2 / window `[0.25, 1.75]` rad / minor radius 0.5 — `sweep/tests/m6_tube.rs`'s wedge, constant for constant. The sheave's groove and the lily's stem tubes already carry torus walls, but both arrive by `revolve`, which RECONSTRUCTS the tube radius from the profile's bulge arcs (the lily drifts 3.9e-16; the review donut drifted 56 ulps). This door stores what it was given: the scene asserts `minor_radius.to_bits() == 0.5f64.to_bits()` on **both** half-tube walls, on the scene body itself. Deliberately a WINDOWED tube, not the full donut, so all three parameters are visible — the ring's radius, the pipe's radius, and the window as the gap its two planar wedge caps close. No semantic fork: census (2 walls + 2 caps), sense derivation, the `R > r > 0` convention and the pcurve mint are the revolve's own code; volume by Pappus π·r²·R·(t₁ − t₀). **Standalone since the montage-v2 curation** (Evan, #218 follow-up): the cell's content — bit-exact stored intent parameters — is interesting for how it works, not visually; without that context it reads as one more partial revolve |
| `loft_prism` | **the first NURBS-walled render** (the trimmed-NURBS tessellation lane, M7): R5 shape (iii) — squares at z = 0/2, a NON-AFFINE trapezoid at z = 1, skinned at v-degree 2, so the four walls are genuinely curved degree-1×2 NURBS patches. The corpus fixture VERBATIM (`step-export/tests/common/mod.rs::loft_prism`, `editor-core/tests/corpus/loft_prism.rs`, `sweep/tests/m6_loft_body.rs`); volume DERIVED exactly: V = 8 + 16d/3 = 9 m³ (d = 0.375); montage panel |
| `nonuniform_loft` | `loft_prism`'s TRUE minimal pair since montage-v2: the SAME sections, the SAME 2 m height, ONLY the middle placement moved — z = 0, 0.15, 2 (the corpus fixture keeps z = 0/1/3, #210/#207 — measured on the #218 sheet, that spacing's bulge peaks at 48.8% of height with half-width 1.415 vs the prism's 50%/1.375, visually the same silhouette rescaled; the scene now LEADS the corpus, the s_duct/lily precedent). The chord-length parameterization (t = 3√29/(3√29 + √5701) ≈ 0.1763) makes the degree-2 skin OVERSHOOT: bulge half-width 1.646 — wider than any authored section — at 32.6% of height; derived V = 8 + 0.25/(t(1−t)) = 9.7219 m³ exactly (quadrature agrees at ~1e-13 pad). Shares `loft_prism`'s camera so the pair reads as a pair; montage panel |
| `s_duct` | the first CURVED-path sweep body (#210/#207; #218 review): a 0.5 m square swept through an S — two OPPOSED quarter arcs of radius 2 (degree-3 interpolant through 17 exact points), 13 stations, v-degree 3, path-following frame (planar path ⇒ no roll). **Standalone since montage-v2** (Evan's follow-up was right): a single-axis revolve cannot make it, but TWO GLUED partial revolves can, shape for shape — each planar arc sweep is a partial revolve's orbit — so the honest not-a-revolve cell is `twisted_duct`. Still the one-op S construction and a fixture CANDIDATE for the next corpus fold (the corpus's sweep constant remains the quarter-arc `swept_elbow`). Volume expectation A·L = (2h)²·2R·π/2 (curvature moment cancels) |
| `twisted_duct` | **the sweep cell since montage-v2: nowhere-zero TORSION — the class NO assembly of revolves reaches**: a 0.5 m square swept along the twisted cubic (At, Bt², Ct³), A/B/C = 2.2/1.3/1.5, degree-3 interpolant through 33 exact points, 17 stations, v-degree 3. τ = 12ABC/\|r′×r″\|² has a constant numerator, so the spine is planar in NO plane and its curvature varies continuously too (no arc anywhere); a revolve's spine is a planar circular arc, and gluing revolves only concatenates planar arcs. The square visibly rolls as the bend plane turns (the path-following frame carrying the torsion). Two shadow proofs ride standalone (`twisted_duct_shadow_{z,y}`): a parabola down z, a one-inflection cubic S down y — parallel projections of a planar curve are affine images of each other and cannot differ in inflection count. Volume expectation A·L (centered symmetric section: curvature moment cancels, roll drops out); fixture CANDIDATE beside the S; montage panel |
| `die` | 21 pip pockets across all six faces, 21 sequential Seamed subtracts, exact volume after every op; standalone render since the M6 curation pass |
| `table` | tabletop ∪ 4 corner-straddling legs; coplanar-touching and inset-overlap variants attempted and narrated live |
| `silhouette` | **first `intersect`**: one solid whose z-shadow is an H and x-shadow is a T (equal letter heights); the NAIVE coincident-plane variant's tier-3′ refusal is narrated (the coincidence ladder made visible); standalone render (the montage carries only the 3-way) |
| `silhouette3` | the H×T solid ∩ a blocky **C** prism along +y — intersect-of-intersect, boolean-of-boolean; all C planes axis-aligned yet sharing no carrier with any H/T plane |
| `crosslap` | cross-lap joint, assembled: two half-depth-notched beams (each a boolean result); the glued union refuses typed today and is **tripwired for M4 PR 5** (`demo_tripwires.rs`) |
| `crosslap_exploded` | the same joint exploded via `transform_rigid` (re-minted witnesses, #84) |
| `projectbox` | enclosure: cavity + 6 vent through-slots + 4 floor bosses + 4 pilot pockets — 15 sequential boolean nodes, the longest chain; square-only until M5 |
| `cutaway` | **first `topo::split`**: the project box split by a tilted plane, halves translated apart — a machinist's section pair (replaces the void box translucency hack) |
| `lily` | **the globe lily** (*Calochortus albus*, the fairy lantern) — the tour's first ORGANIC subject and a deliberate stress test: eight closed solids (three torus-segment stem tubes from `tube_along_arc`, two sphere-zone lanterns with conical mouths from `revolve(Full)`, three keeled leaf blades from `sweep_body` — a four-line kite section carried along an arching NURBS spine), walked by a turtle so consecutive stem arcs are **G1 by construction**. The five analytic bodies approximate nothing, and since the tube door that is a claim about STORED PARAMETERS as well as surface kind: the ring centre, spine axis, `u_ref` and both radii are the world-coordinate numbers the demo passed in, so the stem's `minor_radius` IS the authored 0.060 rather than the bulge-arc reconstruction 3.9e-16 below it. The three blades are the one fitted piece — a swept skin is a B-spline wall through nine exact spine points — and they hold ONE width base to tip, because there is no tapering sweep; their section is four straight lines and not the old crescent's arcs, because the skin lane refuses a rational wall (`nurbs_span_meter` has no speed bound on a rational carrier). Nothing is JOINED either — the stop is followed by **seven live wall probes** that attempt the joins and shapes a plant actually wants (glue the stem arcs, weld flower to stem, oblique-extrude a leaf out of its plane, stretch a bud into an ovoid, mirror a leaf, fillet the mouth rim, carve a tepal seam) and assert each typed refusal, panicking if one ever retires |
| `heatsink5/7/9` | **the M4 layer**: ONE recipe document, fin count 5 → 7 → 9 via `SetStructuralParam` on a `LinearPattern`; each re-eval recomputes exactly 1 node and reuses 4 (counted in the caption); stable names survive the edits (135/135); the montage carries only the 9-fin panel |

Five committed **shadow proofs** ride beside the montage panels
(standalone — excluded from the montage). Three for the silhouette
solid (`renders/silhouette3_shadow_{z,x,y}.png`): the 3-way solid
viewed straight down each axis renders an **H** (z), a **T** (x), and
a **C** (y) — the C near-unclipped (only its 1/16 x-overshoot margins
are trimmed by the solid's width; the T loses two 1/16 z-slivers the
same way — stated because it is true, not visible). Two for the
twisted duct (`renders/twisted_duct_shadow_{z,y}.png`): a parabola
down z and a cubic S down y — the planarity refutation (affine
projections of a planar spine cannot differ in inflection count).

Retired at the #91 refresh: `donut` → sheave (the torus surface kind
now rides in a real part), `openbox` → project box, `voidbox` panel →
cutaway (the two-shell `Voided` story stays as live narration in the
tour output, including STEP's typed void-shell refusal). At the
revision pass: `pulley` → sheave (its plane/cylinder/cone kinds are a
strict subset of the sheave's four) and `wedge` → chute. A×Z
letterforms were probed and refuse typed today — banked as the
acceptance fixture for the cookie-cutter role resolver's
vertex-only-probing gap (#91 comments).

Retired at the **M6 curation pass** — from the SHEET only; every one
of these keeps its standalone render, its narration, and its
corpus/latency/STEP roles: the old `die` panel (its unique content is
21 *sequential* planar subtracts with seamed single-ring pockets, and
chaining DEPTH is not a visual property — `plate`'s holes already show
the rings; `diepips` was the sheet's die then) and `bracket` (`rocker`
covers profile fillets far more comprehensively, six corners across
the whole line/arc taxonomy, and `diefillet` covers the rolling-ball
kind). `rocker` joined the sheet in the same pass — its `montage:
false` was a staging leftover from the demo unit, not a decision.

Curated at **montage-v2** (Evan's #218 follow-up) — again from the
SHEET only; every scene keeps its standalone render, narration,
assertions and exports: `tube_along_arc` and the two partial dice
(`diefillet`, `diepips`) are interesting for how they work — stored
intent parameters, the pre-surface validity battery, the closed-group
21-ball cut — but not visually without that context (the composed die
is the sheet's die; it subsumes both halves visually). `s_duct` came
off in the same pass for an honesty reason: its S solid is exactly two
glued partial revolves, so as "the shape a revolve cannot make" it
overclaimed — `twisted_duct` (nowhere-zero torsion) is the class no
assembly of revolves reaches, and took the sweep-cell seat with its
two shadow proofs standalone. `nonuniform_loft` was re-spaced in the
same pass (z = 0/0.15/2, scene-only; the corpus fixture keeps 0/1/3)
after the measured 0/1/3 pair proved visually indistinguishable.

### Considered and NOT built: a two-peg plate

The obvious next consolidation is to fold `crosslap`/`crosslap_exploded`
and `plate`/`bossplate` into one two-peg plate shown assembled and
apart — one cell pair instead of two, more part-like than either.
It is deliberately **not** built, and the reason is a kernel fact
rather than a taste call: `crosslap`'s value on the sheet is the
**S1 planar REST zip** — a glued union across coincident PLANAR
contact — and a glued peg-in-hole is a *cylindrical* declared
contact, which the kernel does not have. A two-peg plate built today
would demonstrate transverse union (`bossplate`'s point already) plus
free-placement display, and would silently drop the zip the cell
exists to show. Cylindrical declared contact is the curved-census /
declared-contact design doc's territory (M6); revisit this
consolidation when that lands, at which point the merged cell shows
strictly more than the two it replaces.

## Validation posture (tier 3′)

Boolean stops validate the ACTUAL result body via
`validate_pseudomanifold` with the op's own declared `contacts` (M3
PR 6a). The historical `upgrade_edges_to_intersections` clone hack is
deleted. Non-boolean bodies run the plain tier-3 geometric gate; on
contact-free bodies the two gates agree.

Every scene body pre-flights tiers 1–2, prints exact B-rep volume/area
from `topo::mass_properties`, and cross-checks the tessellation's
signed volume. Boolean scenes assert exact (dyadic / closed-form)
volume oracles after EVERY op.

**Staged bodies are RETIRED** (M5 PR 11): `tiltedcut` was the only
one, gated behind `curvedcut::pin_frontier`'s three
retire-on-closure panics — all three lanes landed (tier 3's volume
row, certified mass properties, trimmed tessellation), the pins
fired, and the stop joined the standard ladder per their own
instructions. The pattern (pin the honest refusal, name the PR that
flips it) remains available for the next frontier. `skinned.rs`'s
narration-level pin of the loft-solid frontier retired the same way:
the trimmed-NURBS tessellation lane landed and the module's three
scenes (`loft_prism`, `nonuniform_loft`, `s_duct`) joined the
standard ladder — the narration stays as the geometry layer (control
nets, weights, the measured interpolation claim), which no render can
show.

The tour's coda feeds a self-intersecting (bowtie) profile to
`Profile::validate` and prints the typed rejection — the fail-loud
contract, demonstrated rather than claimed.

## The STEP lane (#88)

Every scene body exports an AP214 STEP file beside its STL — **all 44
of them since M5 PR 13** (26 at that PR; the M5 PR 12 die pieces, the
M6 composed die, the globe lily's eight, the montage refresh's
tube-door wedge, the three NURBS-walled skin scenes, and montage-v2's
twisted duct with its two shadow twins since), where
the in-house
writer's analytic subset
grew from planes/lines to the whole elementary-surface vocabulary
(`PLANE`, `CYLINDRICAL_`, `CONICAL_`, `SPHERICAL_`, `TOROIDAL_SURFACE`)
with `LINE`/`CIRCLE`/`ELLIPSE`/`B_SPLINE_CURVE_WITH_KNOTS` carriers.
Every arm is an **exact native entity**: a cylinder leaves as a
cylinder, never as a spline approximation of one.

TWENTY-SEVEN tour bodies now carry a curved surface (bracket, plate,
vase, sheave, chute, rocker, bossplate, the two tiltedcut halves, the
three die pieces, all eight globe-lily bodies, the tube-door wedge,
and the six NURBS-walled skin bodies — the loft pair, the S duct, and
the twisted duct with its two shadow twins). NINE of them carry a NURBS
wall since the lily's three leaf blades became swept skins.
**Ten of the twenty-seven**
carry `same_sense = .F.` faces, the concave-wall bit S11 introduced —
the original six (bracket 1, plate 4, vase 2, sheave 7, chute 3,
rocker 7) plus die_pips 42, the composed die 42 and each lantern 2.
Seventeen carry none, in two groups. Eight have no CONCAVE curved
wall to reverse — bossplate's boss bulges outward, diefillet's blends
are all convex, the two tiltedcut halves are a plain cylinder cut, the
lily's three stem tubes are convex tori all the way round, and the
tube-door wedge is one more of those with two plain wedge caps
(checked: 4 `.T.`, 0 `.F.`). The NINE skin bodies carry none for a
different reason: an ANALYTIC chart has a canonical normal the wall may
oppose, but a NURBS wall's description is authored by the loft/sweep
assembly itself, outward by construction — there is never anything to
reverse regardless of concavity (the s_duct's and twisted duct's inner
walls are concave and still `.T.`). The lily's leaf blades joined that
group when they stopped being extruded cylinders and became swept
skins: 6 `.T.`, 0 `.F.` on each blade, four B-spline walls and two
planar end caps, where the extruded crescent carried one `.F.` on its
concave cylindrical wall.
(The lily's lanterns reverse on
their MOUTH disc, not on a curved wall: a revolve mints both cap planes
on the profile plane's own +y normal, so exactly one cap opposes the
solid's outward normal — see `lily_lantern.expect`.)

All twenty-four import into FreeCAD 1.1.2 as valid single-solid shapes (the
STEP-lane montage draws every one of them from its own AP214 export,
with no placeholder cells); the lily's five ANALYTIC bodies are checked
against independent closed forms — Pappus for the torus segments, a
zone-plus-frustum integral for the lanterns — agreeing to ≤1.4e-14
relative. Its three swept blades have no analytic wall to check that
way, so they are pinned against Pappus on the MESH instead (kite area
times the centroid's arc length, agreeing to a few 1e-5 at δ = 2e-3 —
the tessellation's own chord error, and a two-sided band, since exact
agreement would mean no real mesh was measured). The lily is still the
widest single-scene spread the writer has been asked for: `TOROIDAL_`
(stem tubes), `SPHERICAL_` + `CONICAL_` (lanterns) and
`B_SPLINE_SURFACE_WITH_KNOTS` (leaf blades) all in one cell.

One typed refusal remains as a named frontier, and no tour body is in
it: a multi-shell **curved** solid (whose outward/void classification
has no closed form yet). NURBS faces export natively since M6-3 (the
loft stop's walls ride `B_SPLINE_SURFACE_WITH_KNOTS`). The tour still
fails loud if a body it expects to export does not.

## Renderers

`render.sh` (kernel lane) prefers **headless FreeCAD** (`freecadcmd`,
`QT_QPA_PLATFORM=offscreen`, no display/Xvfb) importing the tour's STL
meshes — the facets on screen are the kernel's own tessellation
regardless of renderer. (Before the #159 split this lane preferred
STEP imports; once M5 PR 13 made every body export STEP that would
have turned the "kernel" montage 100% OCC, so the STL source is now
unconditional and the STEP imports live in the `--freecad` lane.)
Set `FREECADCMD` to override the binary location. Within one scene the
bodies share one warm document with visibility toggling (per-scene
document cycling races the offscreen view-provider setup — observed
as blank frames/hangs). freecadcmd's Qt teardown can crash AFTER a
successful render, so a scene counts as rendered when its PNG exists,
never by exit status.

`render.sh --freecad` (STEP lane) has no matplotlib fallback: its whole
point is the OCC reference render, so a missing `freecadcmd` is a loud
exit. A scene it genuinely cannot import or render costs one cell — the
reason lands in `renders-freecad/<scene>.fail.txt` (full log under
`out/freecad-logs/`) and `compose_montage.py` draws a labeled
placeholder naming it, never a silent gap.

### One process per scene, on a budget

**Both** lanes run one `freecadcmd` process per scene, each under a
per-scene wall-clock budget (`FREECAD_SCENE_TIMEOUT`, default 300 s;
`render.sh` documents how that number was measured). A warm session
that renders many scenes deadlocks partway through on some hosts — at a
different scene each time, on an idle box as well as a loaded one — so
it is the session that wedges, not any one scene, and no session is
reused across scenes.

Each attempt runs in its own session, so the budget covers the process
*tree*: when it expires the whole group is killed and the scene is
retried **once**, in a fresh process. A second expiry is a loud,
named failure that ends the pass — never a silent skip, never a
degraded cell. Two signals come out with it: how long the process had
been silent (a slow scene keeps writing to its log; a wedged one goes
quiet), and, when the frame was written but the process still had to be
killed, a note saying so — a post-render stall costs one budget and is
never mistaken for a good pass.

Because frames are staged and published only on a complete pass (see
above), a wedge leaves the committed lane directory exactly as it was.

### Off-box: the hosted lanes

`.github/workflows/render.yml` runs the render lanes on GitHub runners,
on demand (`workflow_dispatch` — no PR trigger, no schedule), and hands
each one back as a run artifact:

```sh
gh workflow run render.yml -f ref=my-branch -f lanes=all
gh run watch                        # then, when it lands:
gh run download <run-id> -n renders-kernel    # or renders-freecad / renders-uv
```

`lanes` selects `all` (default) or one of `kernel` / `freecad` / `uv`.
The tour is built **once** and handed to the lanes as an artifact; the
two PNG lanes then run as parallel matrix legs (`fail-fast: false` — one
lane wedging must not cancel the other's evidence), and the UV sheet,
which needs no renderer, lands without waiting on either.

The PNG lanes provision the **same** version-pinned, checksum-verified
FreeCAD 1.1.2 AppImage as `ci.yml`'s `step-import` job — same cache key,
so those rows and these share one entry and a hosted render normally
downloads nothing — and add what *drawing* needs on top of what
importing needs: software GL (llvmpipe) and Xvfb, because Coin's
offscreen renderer wants a GL context and a display even though Qt
itself stays `offscreen`.

The workflow adds exactly one check of its own, and it exists for the
kernel lane: **every leg asserts `demos/renders-preview/` does not
exist.** That lane's matplotlib fallback exits 0, so without the
assertion a hosted pass could be green having drawn nothing with
FreeCAD — the frames sitting in the gitignored preview tree while the
artifact holds the committed cells unchanged. It is a structural check
on the #221 routing invariant above, not a new rule.

**Artifact-only, by construction.** No job commits or pushes, and the
PNG lanes' pixels are *not* expected to match the committed cells
byte-for-byte: those were drawn against a developer host's GL stack,
these by llvmpipe on a runner image that drifts. Each lane reports its
diff against the committed tree in the run summary as a measurement,
never a gate. Making a hosted PNG lane the canonical producer would mean
pinning the whole GL stack in a container image and re-baselining the
committed cells in one commit — a design call, not a config tweak. (The
UV lane carries no such caveat: it is renderer-free and reproducible
off-box, which is why it is the one lane CI actually *gates*.)

`render.py` is the zero-dependency fallback for the kernel lane
(numpy + matplotlib, pure CPU, demo-local venv): binary-STL parsing,
flat shading, exact backface culling (guaranteed by tier 3's +V
invariant) — the same kernel facets, drawn without FreeCAD (the STL
lane in CI keeps mesh coverage either way).

`compose_montage.py` builds the montage sheet from the per-scene PNGs
in `scenes.json` order with captions, for every render path;
`--montage=NAME` / `--banner=TEXT` give the STEP lane its own filename
and provenance banner on the same grid.
