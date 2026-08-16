# The tessellation budget — measuring over-tessellation (issue #320)

**Status: measurement complete, no fix applied.** #320 asked whether
the NURBS-wall grid sizing is "honestly tight … or systematically
over-conservative", and asked for measurement first. This is the
measurement, the instrument that produced it, and what the numbers say
a fix would have to do. The shipped certificate path is unchanged: the
work here is an instrument beside it, not a change to it.

## The instrument

Three pieces, each usable on its own:

| piece | what it is |
|---|---|
| `mesh::budget` | A per-face meter inside the kernel, behind the crate's `budget` feature. Armed, it records one row per face: chart, triangles, the grid the lane used, the whole-patch Hessian bound, and what CHEAPER grids the same certificates admit. |
| `mesh::nurbs_cert::nurbs_cell_bounds` | The certificate assembly reported PER KNOT-SPAN CELL instead of maxed over the patch. A second path, deliberately, so the shipped bound stays bit-identical. Its honesty is falsified cell by cell against densely sampled true second partials. |
| `demo-tour tess-budget` → `tools/tess-lint` | The sweep (every tour scene, one CSV) and the consumer (report + regression gate). Same two-halves shape as the K-telemetry sweep and `k-lint`. |

```sh
scripts/tess_budget_sweep.sh /tmp/b.csv        # add --sizing-only to skip resampling
cd tools/tess-lint && cargo run -- /tmp/b.csv --top 20
# gate against the committed sweep:
cargo run -- /tmp/b.csv --baseline ../../docs/tess-budget-data/tess-budget-baseline.csv
```

## Gating

The meter is **opt-in** (`mesh`'s `budget` feature, forwarded through
`pncad` to `demos/tour`), and it is gated at the MODULE boundary, not
at the call sites: with the feature off, `budget::armed()` is a
constant `false` and every recording branch in `tessellate` and
`trimmed` folds away, so a shipped kernel carries no telemetry state.
The tessellation lane itself has no `#[cfg]` in it — the call sites are
shared and only the module behind them changes, which is what keeps the
two configurations from drifting. `arm`/`take` exist ONLY in the armed
half, so a build without the feature fails to compile the sweep rather
than writing an empty CSV.

The module itself is `pub` only under the feature (`pub(crate)`
otherwise, which is all the inert half needs), and so are the row types
— `Mode`, `NurbsBudget`, `FaceBudget`, the CSV. A default build exports
no part of the meter. What stays compiled either way is exactly what
the shared call sites name: `Chart`, `Sizing`, `DEV_SAMPLES`, and the
no-op recorders.

The default `cargo test` therefore exercises the inert half; the armed
half has its own CI row (`cargo test -p mesh --features budget`),
mirrored in `local-scripts/ci-local.sh`.

The sweep resamples `|S − Π|` on every emitted triangle unless
`--sizing-only` is passed; without that pass it costs one tessellation
per scene and nothing else, because the sizing columns are read off
sizing the lane already performed. Both run in ~4 s over the whole tour
in release.

The committed baseline is `docs/tess-budget-data/tess-budget-baseline.csv`
(1025 face rows, cut at the head this document was written against,
WITH the resampling pass — its `worst_dev` column is where the
total-slack figures below come from). CI runs the sweep
`--sizing-only` and gates on REGRESSION against it: a scene's mesh
growing, a face's sizing getting wastefuller, or a scene silently
dropping out of the sweep. The gate reads triangle counts and the
sizing columns only, so the resampling is a cost the gate has no use
for; re-cutting the baseline drops the flag.

## What the four numbers mean

All four are ratios of GRID CELL COUNTS over the same trim box with the
same `ceil` discipline, so they compare directly.

The per-triangle certificate is `Q/4` with
`Q = muu·a_u² + 2·muv·a_u·a_v + mvv·a_v²`, and the lane budgets a
triangle at two grid cells per axis, so a grid `(h_u, h_v)` is legal
exactly when

```
muu·h_u² + 2·muv·h_u·h_v + mvv·h_v²  ≤  δ_s .
```

That is the interior of an ellipse in `(h_u, h_v)`. Every "cheaper"
grid counted below is a point of that same region — nothing here is
achieved by weakening a certificate.

* **split** = `uniform_cells / opt_cells`. The shipped schedule reaches
  the region through the decoupling `2·a_u·a_v ≤ a_u² + a_v²`, which
  lands on a particular point of it, not the cheapest one.
* **span** = `uniform_cells / span_cells`. The shipped grid against one
  sized per knot-span cell from that cell's own certified bound —
  #320's own hypothesis, metered.
* **both** = `uniform_cells / span_opt_cells`. Not the product.
* **total** = triangles against `triangles · worst_dev / δ`, summed
  over the faces the sweep resampled: the deviation budget that went
  unspent. The softest number of the four — `worst_dev` is a SAMPLED
  sup (so it under-reports deviation and over-reports slack) and the
  `deviation ~ h²` scaling is a first-order extrapolation.

## The finding

Over the whole tour, at each scene's own δ:

```
1025 faces, 1,149,528 triangles
  64 NURBS faces (6.2% of faces) carry 782,104 triangles (68.0% of the mesh)
  390,100 grid cells used
   95,090 at the cheapest split      (4.1x)
  154,129 sized per knot-span cell   (2.5x)
   44,457 with both                  (8.8x)
```

| scene | tris | split | span | both | total |
|---|---|---|---|---|---|
| lily/lily_leaf_a | 261,780 | 4.8x | 3.8x | **17.0x** | 26.8x |
| lily/lily_sepal_a | 101,102 | 4.2x | 3.2x | 12.0x | 30.6x |
| lily/lily_sepal_c | 100,106 | 4.1x | 3.2x | 12.0x | 25.7x |
| lily/lily_sepal_b | 99,592 | 4.1x | 3.2x | 11.8x | 27.7x |
| twisted_duct | 71,002 | 4.4x | 1.8x | 7.5x | 36.1x |
| s_duct | 43,196 | 4.6x | 1.6x | 6.1x | 42.5x |
| nonuniform_loft | 23,372 | 1.6x | 1.0x | 1.6x | 18.7x |
| loft_prism | 13,144 | 1.6x | 1.0x | 1.6x | 18.2x |
| lily/lily_leaf_b | 976 | 4.0x | 0.9x | 3.8x | 36.3x |
| lily/lily_leaf_c | 826 | 4.0x | 1.0x | 3.7x | 38.6x |

**Yes, measurably — and the dominant cause is not the one #320
guessed.**

### 1. The u/v split, not the leaf, is the biggest single factor (~4x, everywhere)

`split` is ~4x on essentially every NURBS wall in the tour, including
the swept blades `leaf_b` and `leaf_c` that #320 held up as the
well-behaved siblings. It is not a property of the lofted leaf; it is a
property of the schedule.

The leaf's walls are **degree 1 in u** — ruled — so `muu = 3.8e-162`
(rounding dust for an exact zero), with `muv ≈ 0.4 … 3.0` and
`mvv ≈ 16 … 51`. The constraint degenerates to
`2·muv·h_u·h_v + mvv·h_v² ≤ δ_s`, a hyperbola whose cheapest point puts
`h_u` at the whole extent — ONE division across the flat direction —
and spends everything on `v`. The AM-GM decoupling instead charges the
cross term `muv` to BOTH directions, and so divides a ruled surface
70–78 ways across the direction it is straight in.

### 2. The whole-patch sup is real, and secondary (~3.8x on the leaf)

`span` is 3.8x on `leaf_a` and 3.2x on the sepals, against 0.9–1.0x on
the swept blades — exactly the shape #320 predicted, on exactly the
faces it predicted it for. The leaf's 14 knot-span cells differ enough
that sizing the whole patch from its worst one costs nearly 4x. On a
uniform wall it costs nothing (and can cost a few percent, since a
per-cell grid pays a `ceil` per cell — `leaf_b`'s 0.9x is that, and it
is reported rather than clipped to 1).

### 3. Together: the leaf's 261,780 triangles project to ~15,400

`both` is 17.0x on `leaf_a` — 130,177 grid cells against 7,659, all of
them certified by the same per-triangle bound. Its swept siblings sit
at ~900 triangles, so this does not close the gap #320 opens with;
`leaf_a` is a genuinely larger and more curved surface than
`leaf_b`/`leaf_c`, and the residue after sizing is real geometry.

### 4. Beyond sizing, ~1.5x more sits in the certificate itself

Per leaf wall, the deviation side decomposes as
`δ/worst_cert · worst_cert/worst_dev`: budget slack 11.3–13.7x,
certificate slack 2.1–3.1x, total 24–36x. The `both` factor (17x)
recovers most but not all of that: the remainder is the Hessian bound
sitting above the deviation actually attained, which is what a bound
does. Chasing it means a tighter certificate, not a better schedule.

## What a fix has to deal with

Nothing here is a patch waiting to be applied. Three things a sizing
change must answer, all visible in the measurement:

1. **Anisotropy.** The cheapest split is a STRIP: `70 × 328` becomes
   `1 × 4905` on `leaf_a` face 2, a parameter aspect near 5·10³. The
   certificate does not object and nothing downstream has been asked.
   `opt_cells` is therefore an upper bound on what an aspect-respecting
   schedule would recover, and capping aspect honestly needs the
   surface's first fundamental form — parameter aspect is not 3-D
   aspect. The `span` factor has no such caveat.
2. **The chord pass shares these steps.** `nurbs_cert`'s grid steps
   also bound the boundary chord schedule of every adjacent edge (the
   adjacent-face tightening in `chords`), so a schedule change is not
   local to the grid.
3. **A per-cell schedule needs its grid lines on the cell boundaries.**
   That is what makes each triangle's certificate the certificate of
   the cell it is in. The cells are half-open — a knot is where a C¹
   surface's second derivative jumps — and the shared boundary is
   measure-zero, which the Taylor remainder already tolerates; it is
   the same fact the whole-patch assembly rests on at its own interior
   knots.

## Re-cutting the baseline

The gate compares differences, never absolute slack. The absolute
factors above are large and known; a threshold set above them would
certify nothing, and a threshold set below them could only be satisfied
by coarsening a demo's δ or simplifying its geometry — which destroys
the measurement this whole lane exists to keep.

When a growth is intended, re-cut:

```sh
scripts/tess_budget_sweep.sh docs/tess-budget-data/tess-budget-baseline.csv
```

and say WHY in the commit. A `vanished` finding is never re-baselined
without reading it first: a scene the sweep stopped covering improves
every total it used to appear in.
