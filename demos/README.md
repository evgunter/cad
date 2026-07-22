# Pre-GUI demo tour

A visual tour of what the kernel can do today, from a pure outside
consumer's seat: six bodies built through the public `profile` /
`sweep` APIs, then a boolean leg through the M3 PR 5 `union` /
`subtract` ops — narrated (operations used, topology census + genus,
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

### The boolean leg (M3 PR 5)

All boolean operands are axis-aligned boxes from one shared builder
(`slab`), so intended coincidences arise from bit-identical values.
Every scene checks the op's result against an exact box-arithmetic
volume oracle and narrates each attempted variant honestly.

| stop | what it shows |
| --- | --- |
| die (live matrix) | the R1 orientation matrix run live: pip pockets succeed exact on {+z, −x, −y} and refuse typed `SeamOrientation` on {−z, +x, +y}; a real 3-pocket cube composes on the working faces; a −z pip guard fails loudly the day PR 5.5 lands the refusing half (full die then) — no STL |
| `table` | tabletop ∪ 4 corner-straddling legs — 4 sequential Seamed union nodes; coplanar-touching and inset-overlap leg variants attempted and their typed refusals narrated |
| `openbox` | the pure open box: fully-interior cavity cutter, open only through the top — a single-ring pocket on a working R1 orientation (refused pre-fix-pass, rendered since) |
| `scoopbox` | subtract through-cut: cavity cutter overhanging top + one wall — a multi-face seam, the variant that shipped while the pure box was refused |
| `voidbox` | inner box strictly inside, subtracted — kind `Voided`, TWO shells, the first legitimate voids; V = 8 − 1 = 7 exactly; a cutaway subtract of the two-shell body is attempted and its typed refusal narrated |

Known-limitation narration baked into the tour (PR 5 review's ratified
envelope, demonstrated with exact numbers rather than claimed):

- **Orientation-dependent single-ring seams (R1)**: pockets/bosses whose
  seam ring closes within one face succeed exactly on {+z, −x, −y}
  faces and refuse typed `SeamOrientation` on {−z, +x, +y} — the
  handedness-correlated half. Double-ring configurations
  (through-pillar) refuse too. PR 5.5's charter; the die's −z guard and
  the demo's pure variants promote when it lands. (The pre-fix-pass
  SILENT wrong-component defect this narration replaced is resolved —
  every failure is now a typed refusal.)
- **Extrude operands**: `extrude` describes edges as `Intersection`
  of adjacent surfaces; the ops' carve/merge stages leave those
  references dangling (`Merge(InputNotClosed)` refusal), so the tour
  re-describes operand edges as chord lines first. (Whether PR 5's
  extrude-operand remap made this workaround unnecessary is untested —
  queued for the PR 5.5 demo unblock.)
- **Touching/coplanar-overlap unions** (flush-stacked, corner-flush,
  inset-overlap) refuse with typed Join-family errors
  (`SeamOrientation` / `UnpairedLooseEnds` / `JoinDesync` by
  configuration) — PR 5.5 territory, narrated live from the actual
  results.
- Boolean outputs carry chord descriptions on seam edges, so tier 3
  runs on an `Intersection`-upgraded clone (the test suite's
  documented posture; the honest upgrade op is a PR 6 obligation).

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
