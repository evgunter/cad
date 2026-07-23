# Pre-GUI demo tour

A visual tour of what the kernel can do today, from a pure outside
consumer's seat: six bodies built through the public `profile` /
`sweep` APIs, then a boolean leg through the M3 `union` / `subtract`
ops (headlined by a full 21-pip die) — narrated (operations used,
topology census + genus, validation tiers passed, exact-vs-meshed mass
properties), exported as binary STL, and rendered to PNG.

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

### The boolean leg (M3 PRs 5 + 5.5)

All boolean operands are axis-aligned boxes from one shared builder
(`slab`) — raw `extrude` output fed directly to the ops (the chord
re-description workaround retired after PR 5.5's review proved it
unnecessary). Every scene checks the op's result against an exact
box-arithmetic volume oracle.

| stop | what it shows |
| --- | --- |
| `die` | the full die: 21 pip pockets across all six faces (opposite faces sum to 7), 21 sequential Seamed subtracts with the exact volume after every op (final V = 7.8359375); blocked pre-PR 5.5 by the R1 orientation-dependent refusals, promoted when the seam discipline closed them |
| `table` | tabletop ∪ 4 corner-straddling legs — 4 sequential Seamed union nodes; coplanar-touching and inset-overlap leg variants attempted and narrated |
| `openbox` | the pure open box: fully-interior cavity cutter, open only through the top — a single-ring pocket (refused pre-PR 5-fix-pass, rendered since) |
| `voidbox` | inner box strictly inside, subtracted — kind `Voided`, TWO shells, the first legitimate voids; V = 8 − 1 = 7 exactly; rendered translucent so the internal void shell is visible; a cutaway subtract of the two-shell body is attempted and its typed refusal narrated |

Historical narration retired as the kernel caught up: the PR 5-era
silent wrong-component defect (typed refusals since the PR 5 fix
pass), the R1 orientation matrix (closed by PR 5.5 — the die above IS
the promotion payload), and the scoop-box stand-in for the open box.
What the tour still touches of the post-5.5 refusal envelope
(boundary-on-boundary seams, e.g. flush-stacked/corner-flush unions;
reflex-corner tilted crossings) is narrated live from the actual typed
refusals where scenes attempt it. Boolean outputs carry chord
descriptions on seam edges, so tier 3 runs on an
`Intersection`-upgraded clone (the documented posture; the honest
upgrade op is a PR 6 obligation).

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
