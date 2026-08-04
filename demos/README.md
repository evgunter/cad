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
```

Outputs: `demos/out/*.{stl,step}` + `demos/out/scenes.json` (untracked),
`demos/renders/*.png` (tracked — one per scene plus `montage.png`),
`demos/renders-freecad/*.png` (tracked — the montage cells plus
`montage-freecad.png`).

## The two montages (#159)

The tour ships **two montage sheets** with identical grids, captions,
scene order, and cameras (both read `scenes.json`) — cell-for-cell
comparable, differing ONLY in whose tessellation is on screen:

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
| `die` | 21 pip pockets across all six faces, 21 sequential Seamed subtracts, exact volume after every op; standalone render since the M6 curation pass |
| `table` | tabletop ∪ 4 corner-straddling legs; coplanar-touching and inset-overlap variants attempted and narrated live |
| `silhouette` | **first `intersect`**: one solid whose z-shadow is an H and x-shadow is a T (equal letter heights); the NAIVE coincident-plane variant's tier-3′ refusal is narrated (the coincidence ladder made visible); standalone render (the montage carries only the 3-way) |
| `silhouette3` | the H×T solid ∩ a blocky **C** prism along +y — intersect-of-intersect, boolean-of-boolean; all C planes axis-aligned yet sharing no carrier with any H/T plane |
| `crosslap` | cross-lap joint, assembled: two half-depth-notched beams (each a boolean result); the glued union refuses typed today and is **tripwired for M4 PR 5** (`demo_tripwires.rs`) |
| `crosslap_exploded` | the same joint exploded via `transform_rigid` (re-minted witnesses, #84) |
| `projectbox` | enclosure: cavity + 6 vent through-slots + 4 floor bosses + 4 pilot pockets — 15 sequential boolean nodes, the longest chain; square-only until M5 |
| `cutaway` | **first `topo::split`**: the project box split by a tilted plane, halves translated apart — a machinist's section pair (replaces the void box translucency hack) |
| `lily` | **the globe lily** (*Calochortus albus*, the fairy lantern) — the tour's first ORGANIC subject and a deliberate stress test: eight closed analytic solids (three torus-segment stem tubes from `revolve(Partial)` of a circle about a distant axis, two sphere-zone lanterns with conical mouths from `revolve(Full)`, three extruded two-arc crescent leaves), walked by a turtle so consecutive stem arcs are **G1 by construction**. Nothing is approximated: every wall is torus, sphere, cone or plane exactly. Nothing is JOINED either — the stop is followed by **seven live wall probes** that attempt the joins and shapes a plant actually wants (glue the stem arcs, weld flower to stem, oblique-extrude a swept leaf, stretch a bud into an ovoid, mirror a leaf, fillet the mouth rim, carve a tepal seam) and assert each typed refusal, panicking if one ever retires |
| `heatsink5/7/9` | **the M4 layer**: ONE recipe document, fin count 5 → 7 → 9 via `SetStructuralParam` on a `LinearPattern`; each re-eval recomputes exactly 1 node and reuses 4 (counted in the caption); stable names survive the edits (135/135); the montage carries only the 9-fin panel |

Three committed **shadow proofs** ride beside the montage panels
(`renders/silhouette3_shadow_{z,x,y}.png`, standalone — excluded from
the montage): the 3-way solid viewed straight down each axis renders
an **H** (z), a **T** (x), and a **C** (y) — the C near-unclipped
(only its 1/16 x-overshoot margins are trimmed by the solid's width;
the T loses two 1/16 z-slivers the same way — stated because it is
true, not visible).

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
the rings; `diepips` is the sheet's die now) and `bracket` (`rocker`
covers profile fillets far more comprehensively, six corners across
the whole line/arc taxonomy, and `diefillet` covers the rolling-ball
kind). `rocker` joined the sheet in the same pass — its `montage:
false` was a staging leftover from the demo unit, not a decision.

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
flips it) remains available for the next frontier; `skinned.rs`
still carries a narration-level pin of the loft-solid frontier.

The tour's coda feeds a self-intersecting (bowtie) profile to
`Profile::validate` and prints the typed rejection — the fail-loud
contract, demonstrated rather than claimed.

## The STEP lane (#88)

Every scene body exports an AP214 STEP file beside its STL — **all 37
of them since M5 PR 13** (26 at that PR; the M5 PR 12 die pieces, the
M6 composed die and the globe lily's eight since), where the in-house
writer's analytic subset
grew from planes/lines to the whole elementary-surface vocabulary
(`PLANE`, `CYLINDRICAL_`, `CONICAL_`, `SPHERICAL_`, `TOROIDAL_SURFACE`)
with `LINE`/`CIRCLE`/`ELLIPSE`/`B_SPLINE_CURVE_WITH_KNOTS` carriers.
Every arm is an **exact native entity**: a cylinder leaves as a
cylinder, never as a spline approximation of one.

TWENTY tour bodies now carry a curved surface (bracket, plate, vase,
sheave, chute, rocker, bossplate, the two tiltedcut halves, the three
die pieces, and all eight globe-lily bodies); every one of them carries
`same_sense = .F.` faces, the concave-wall bit S11 introduced. All
twenty import into FreeCAD 1.1.2 as valid single-solid shapes whose
volumes agree with the kernel's own tessellation to within faceting
error. The lily is the widest single-scene spread the writer has been
asked for: `TOROIDAL_` (stem tubes), `SPHERICAL_` + `CONICAL_`
(lanterns) and `CYLINDRICAL_` (leaf blades) all in one cell.

Two typed refusals remain as named frontiers, and no tour body is in
either: a NURBS **face** (which the loft-assembly unit mints) and a
multi-shell **curved** solid (whose outward/void classification has no
closed form yet). The tour still fails loud if a body it expects to
export does not.

## Renderers

`render.sh` (kernel lane) prefers **headless FreeCAD** (`freecadcmd`,
`QT_QPA_PLATFORM=offscreen`, no display/Xvfb) importing the tour's STL
meshes — the facets on screen are the kernel's own tessellation
regardless of renderer. (Before the #159 split this lane preferred
STEP imports; once M5 PR 13 made every body export STEP that would
have turned the "kernel" montage 100% OCC, so the STL source is now
unconditional and the STEP imports live in the `--freecad` lane.)
Set `FREECADCMD` to override the binary location. All scenes render
in one warm document with per-scene visibility toggling (per-scene
document cycling races the offscreen view-provider setup — observed
as blank frames/hangs). freecadcmd's Qt teardown can crash AFTER a
successful pass, so `render.sh` keys on the `renders/.freecad_ok`
sentinel, not the exit status.

`render.sh --freecad` (STEP lane) runs **one `freecadcmd` process per
scene** under `timeout` (`FREECAD_SCENE_TIMEOUT`, default 300 s) —
bulk imports have stalled before, and per-scene isolation means a
stall or import failure costs one cell: the failure reason lands in
`renders-freecad/<scene>.fail.txt` (full log under
`out/freecad-logs/`) and `compose_montage.py` draws a labeled
placeholder cell naming it — never a silent gap. This lane has no
matplotlib fallback: its whole point is the OCC reference render, so
a missing `freecadcmd` is a loud exit.

`render.py` is the zero-dependency fallback for the kernel lane
(numpy + matplotlib, pure CPU, demo-local venv): binary-STL parsing,
flat shading, exact backface culling (guaranteed by tier 3's +V
invariant) — the same kernel facets, drawn without FreeCAD (the STL
lane in CI keeps mesh coverage either way).

`compose_montage.py` builds the montage sheet from the per-scene PNGs
in `scenes.json` order with captions, for every render path;
`--montage=NAME` / `--banner=TEXT` give the STEP lane its own filename
and provenance banner on the same grid.
