# Pre-GUI demo tour

A visual tour of what the kernel can do today, from a pure outside
consumer's seat: six bodies built through the public `profile` /
`sweep` APIs, narrated (operations used, topology census + genus,
validation tiers passed, exact-vs-meshed mass properties), exported as
binary STL, and rendered to PNG.

This directory is **deliberately outside the cargo workspace** (root
manifest `workspace.exclude`, plus the empty `[workspace]` table in
`tour/Cargo.toml`): STL viewing/rendering tooling is demo-only and must
never become a kernel dependency. The renderer is a demo-local Python
venv (numpy + matplotlib, pinned, created on first run) — pure CPU,
headless, no GPU/GL required.

## Run

```sh
cd demos/tour
cargo run --release -- ../out   # build + narrate + export STLs
cd ..
./render.sh                     # venv on first run, then PNGs
```

Outputs: `demos/out/*.stl` (untracked), `demos/renders/*.png` (tracked
— one per body plus `montage.png`, the contact sheet).

## The stops

| body | what it shows |
| --- | --- |
| `bracket` | extrude of a polyline + tangent-arc profile (`LoopBuilder`, inner fillet) |
| `plate` | extrude with two circular holes — genus 2, ring loops in both caps |
| `vase` | full revolve, axis-touching profile: sphere-zone belly + cone lip |
| `donut` | full revolve of an off-axis circle — closed all-curved torus, genus 1 |
| `pulley` | full revolve of an off-axis polyline — V-groove, center bore |
| `wedge` | partial (90°) revolve — wedge caps, arc rims |

The tour's coda feeds a self-intersecting (bowtie) profile to
`Profile::validate` and prints the typed rejection — the fail-loud
contract, demonstrated rather than claimed.

Every stop pre-flights all three validator tiers (structural,
closed-solid census, geometric incl. the +V orientation invariant),
prints exact B-rep volume/area from `topo::mass_properties`, and
cross-checks the tessellation's signed volume against the exact value.

## Renderer notes

`render.py` parses binary STL with numpy and draws flat-shaded
`Poly3DCollection`s (matplotlib Agg). Because every exported body is
closed and outward-oriented (tier 3's +V invariant), the renderer culls
back-faces exactly — which is also what makes matplotlib's
painter's-algorithm depth sort artifact-free here. Orthographic
projection, per-body camera and palette at the top of the script.

An f3d prebuilt binary was the preferred renderer but needs OpenSSL 3 /
newer glibc than this environment has; the numpy+matplotlib fallback is
fully headless and has no system requirements beyond Python.
