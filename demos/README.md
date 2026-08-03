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
| `sheave` | rope-groove sheave — full revolve of a polyline+arc profile: hub, web, **tapered (cone) rim shoulders**, semicircular groove whose OFF-axis arc sweeps a **ring-torus zone**; all four analytic wall kinds (plane/cylinder/cone/torus) on one part; genus 1; volume checked against the closed-form Pappus value |
| `chute` | quarter-turn chute — a C-channel profile swept through a **270° partial revolve**; wedge caps showing the profile, curved trough; Pappus-exact volume |
| `rocker` | **the M5 fillet sugar**: a rocker plate whose SIX corners are all authored by `LoopBuilder::fillet_corner` — arc×line, line×line, line×arc, arc×line, line×arc around the outline, and **arc×arc** at the eye slot's rounded tip, where two tangent circles of the authored radius fit and the S8 rule **picks the one nearest the authored corner** (asserted, and narrated with both centres); genus 1; standalone render |
| `tiltedcut` | **RENDERING (M5 PR 11, the milestone's demo moment)**: a cylinder cut by a tilted plane — the section edges carry an **exact `Curve3::Ellipse`** (a = r/cos φ, b = r, residual ~1e-16, PR 5 shape (i)); the cut walls tessellate **watertight** through the pcurve-driven trimmed lane, and the volume is a **certified quadrature enclosure** (± ~1e-6 m³) asserted to bracket πr²H/2 per half; montage panel |
| `bossplate` | **the first curved boolean, visible (M5 PR 11)**: a three-arc cylindrical boss unioned into a plate (PR 9 shape (ii)) — the seam is three exact `Circle` arcs, V = 16 + π·0.25·0.6 on the nose, and the shared-chord assertion pins that the curved wall and the ringed top face consume ONE chord set per seam edge; montage panel
| `die` | 21 pip pockets across all six faces, 21 sequential Seamed subtracts, exact volume after every op |
| `table` | tabletop ∪ 4 corner-straddling legs; coplanar-touching and inset-overlap variants attempted and narrated live |
| `silhouette` | **first `intersect`**: one solid whose z-shadow is an H and x-shadow is a T (equal letter heights); the NAIVE coincident-plane variant's tier-3′ refusal is narrated (the coincidence ladder made visible); standalone render (the montage carries only the 3-way) |
| `silhouette3` | the H×T solid ∩ a blocky **C** prism along +y — intersect-of-intersect, boolean-of-boolean; all C planes axis-aligned yet sharing no carrier with any H/T plane |
| `crosslap` | cross-lap joint, assembled: two half-depth-notched beams (each a boolean result); the glued union refuses typed today and is **tripwired for M4 PR 5** (`demo_tripwires.rs`) |
| `crosslap_exploded` | the same joint exploded via `transform_rigid` (re-minted witnesses, #84) |
| `projectbox` | enclosure: cavity + 6 vent through-slots + 4 floor bosses + 4 pilot pockets — 15 sequential boolean nodes, the longest chain; square-only until M5 |
| `cutaway` | **first `topo::split`**: the project box split by a tilted plane, halves translated apart — a machinist's section pair (replaces the void box translucency hack) |
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

Every scene body exports an AP214 STEP file beside its STL — **all 26
of them since M5 PR 13**, where the in-house writer's analytic subset
grew from planes/lines to the whole elementary-surface vocabulary
(`PLANE`, `CYLINDRICAL_`, `CONICAL_`, `SPHERICAL_`, `TOROIDAL_SURFACE`)
with `LINE`/`CIRCLE`/`ELLIPSE`/`B_SPLINE_CURVE_WITH_KNOTS` carriers.
Every arm is an **exact native entity**: a cylinder leaves as a
cylinder, never as a spline approximation of one.

Nine tour bodies are curved (bracket, plate, vase, sheave, chute,
rocker, bossplate, and the two tiltedcut halves); six of them carry
`same_sense = .F.` faces, the concave-wall bit S11 introduced. All nine
import into FreeCAD 1.1.2 as valid single-solid shapes whose volumes
agree with the kernel's own tessellation to within faceting error.

Two typed refusals remain as named frontiers, and no tour body is in
either: a NURBS **face** (which the loft-assembly unit mints) and a
multi-shell **curved** solid (whose outward/void classification has no
closed form yet). The tour still fails loud if a body it expects to
export does not.

## Renderers

`render.sh` prefers **headless FreeCAD** (`freecadcmd`,
`QT_QPA_PLATFORM=offscreen`, no display/Xvfb): one session imports the
tour's OWN STEP exports — every montage panel now dogfoods the F6 lane
end-to-end (export → OCC import → render), curved bodies included since
M5 PR 13; the STL mesh-import fallback stays for anything that ever
fails to export. Set `FREECADCMD`
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
