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
./render.sh                     # FreeCAD headless (or matplotlib fallback), then montage
```

Outputs: `demos/out/*.{stl,step}` + `demos/out/scenes.json` (untracked),
`demos/renders/*.png` (tracked — one per scene plus `montage.png`).

## The stops

| scene | what it shows |
| --- | --- |
| `bracket` | extrude of a polyline + tangent-arc profile (`LoopBuilder`, inner fillet) |
| `plate` | extrude with two circular holes — genus 2, ring loops in both caps |
| `vase` | full revolve, axis-touching profile: sphere-zone belly + cone lip |
| `sheave` | rope-groove sheave — full revolve of a polyline+arc profile; the groove arc is OFF-axis, so its wall is a **ring-torus zone**; genus 1; volume checked against the closed-form Pappus value |
| `pulley` | full revolve of an off-axis polyline — V-groove, center bore; the cone showcase |
| `wedge` | partial (90°) revolve — wedge caps, arc rims |
| `die` | 21 pip pockets across all six faces, 21 sequential Seamed subtracts, exact volume after every op |
| `table` | tabletop ∪ 4 corner-straddling legs; coplanar-touching and inset-overlap variants attempted and narrated live |
| `silhouette` | **first `intersect`**: one solid whose z-shadow is an H and x-shadow is a T; the NAIVE coincident-plane variant's tier-3′ refusal is narrated (the coincidence ladder made visible) |
| `silhouette3` | the H×T solid ∩ a 45° diamond prism — intersect-of-intersect, boolean-of-boolean |
| `crosslap` | cross-lap joint, assembled: two half-depth-notched beams (each a boolean result); the glued union refuses typed today and is **tripwired for M4 PR 5** (`demo_tripwires.rs`) |
| `crosslap_exploded` | the same joint exploded via `transform_rigid` (re-minted witnesses, #84) |
| `projectbox` | enclosure: cavity + 6 vent through-slots + 4 floor bosses + 4 pilot pockets — 15 sequential boolean nodes, the longest chain; square-only until M5 |
| `cutaway` | **first `topo::split`**: the project box split by a tilted plane, halves translated apart — a machinist's section pair (replaces the void box translucency hack) |
| `heatsink5/7/9` | **the M4 layer**: ONE recipe document, fin count 5 → 7 → 9 via `SetStructuralParam` on a `LinearPattern`; each re-eval recomputes exactly 1 node and reuses 4 (counted in the caption); stable names survive the edits (135/135) |

Three committed **shadow proofs** ride beside the montage panels
(`renders/silhouette3_shadow_{z,x,y}.png`, standalone — excluded from
the montage): the 3-way solid viewed straight down each axis renders
an H (z), a T (x), and the 45° chamfer diamond (y) — the y-view
**clipped by the solid's extents**, which is the honest form of the
third-shadow claim.

Retired at the #91 refresh: `donut` → sheave (the torus surface kind
now rides in a real part), `openbox` → project box, `voidbox` panel →
cutaway (the two-shell `Voided` story stays as live narration in the
tour output, including STEP's typed void-shell refusal). A×Z
letterforms were probed and refuse typed today — banked as the
acceptance fixture for the cookie-cutter role resolver's
vertex-only-probing gap (#91 comments).

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

The tour's coda feeds a self-intersecting (bowtie) profile to
`Profile::validate` and prints the typed rejection — the fail-loud
contract, demonstrated rather than claimed.

## The STEP lane (#88)

Every scene body attempts an AP214 STEP export beside its STL. The
in-house writer's analytic subset is planes/lines today, so curved
bodies (bracket, plate, vase, sheave, pulley, wedge) refuse **typed**
(`UnsupportedSurface`/`UnsupportedCurve` — the M5 arms), and the tour
narrates the refusal. All-planar bodies (die, table, silhouettes,
cross-lap, project box, cutaway halves, heat sinks) export STEP.

## Renderers

`render.sh` prefers **headless FreeCAD** (`freecadcmd`,
`QT_QPA_PLATFORM=offscreen`, no display/Xvfb): one session imports the
tour's OWN STEP exports — every montage panel of a planar body
dogfoods the F6 lane end-to-end (export → OCC import → render) — and
falls back to STL mesh import for the curved bodies. Set `FREECADCMD`
to override the binary location. All scenes render in one warm
document with per-scene visibility toggling (per-scene document
cycling races the offscreen view-provider setup — observed as blank
frames/hangs). freecadcmd's Qt teardown can crash AFTER a successful
pass, so `render.sh` keys on the `renders/.freecad_ok` sentinel, not
the exit status.

`render.py` is the zero-dependency fallback (numpy + matplotlib, pure
CPU, demo-local venv): binary-STL parsing, flat shading, exact
backface culling (guaranteed by tier 3's +V invariant) — and the lane
that draws OUR tessellation (FreeCAD re-tessellates from the B-rep;
the STL lane in CI keeps mesh coverage either way).

`compose_montage.py` builds `montage.png` from the per-scene PNGs in
`scenes.json` order with captions, for both render paths.
