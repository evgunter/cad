# The tessellation budget — measuring over-tessellation (issue #320)

**Status: measurement complete; the SPAN half is FIXED (TESS-SPAN,
merged as #594; its binding spec was deleted with the other
closed-unit artifacts and is recoverable through
`docs/DOC-LEDGER.md`), and the SPLIT half is FIXED (TESS-SPLIT):
the shipped point selection is now the cell minimizer on the same
certified ellipse under the ratified A = 16 first-fundamental-form
aspect cap (`mesh::nurbs_cert::ASPECT_CAP` and
`NurbsFaceBound::split_steps` are the statement of record), with
the realized-lattice sliver machinery in force over it — the
post-fix split ratio reads ~1.0 at constraint-inactive cells and
the CSV's `cap_bands`/`snap_bands` columns say which constraint
bound the rest.** #320 asked whether the NURBS-wall grid
sizing is "honestly tight … or systematically over-conservative", and
asked for measurement first. This document is that measurement, the
instrument that produced it, and what the numbers said a fix would
have to do — kept as the pre-fix record. Since TESS-SPAN the shipped
schedule sizes each knot-span cell from its own certified bound, so
the `span` factor below is REALIZED and the meter's columns were
re-derived ("The columns after TESS-SPAN", below); since TESS-SPLIT
the split factor is realized too, under the ratified aspect policy
below.

## The instrument

Four pieces, each usable on its own:

| piece | what it is |
|---|---|
| `mesh::budget` | The per-face MEASUREMENTS, taken inside the kernel behind the crate's `budget` feature — and only what nothing downstream can recover: the trim box, the cells the schedule built, the certified bounds the sizing read, the worst per-triangle certificate, and (armed for it) the sampled deviation. One hand-off per NURBS face. No schema, no arithmetic, no assertion. |
| `tools/tess-meter` | Everything downstream of the measurements: the counterfactual schedules, the split optimizer, the row for every face (a planar cap's chart and triangle count are in the body and the mesh, so the meter is not asked to report them), and the CSV. Outside the kernel workspace, because a change to how the numbers are READ must not reach the lane that produces them. |
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

The MEASUREMENT TYPES (`FaceMeasure`, `CellMeasure`) are public either
way: they are the contract with `tools/tess-meter`, which reads them
without needing the instrument compiled in, and they are data. What the
feature gates is the instrument — `Mode`, `arm`, `take`, the
thread-local, and the recording the lane does between them.

The default `cargo test` therefore exercises the inert half; the armed
half has its own CI row (`cargo test -p mesh --features budget`),
mirrored in `local-scripts/ci-local.sh`. That row also carries the
per-triangle certificate falsifier
(`probe_review::z1_per_triangle_certificate_falsification`), which
drives the deviation pass at 12 samples per edge and asserts on
`worst_ratio`. **The assertion is in the suite, not in the lane**: the
kernel reduces the samples to the largest `|S − Π| / (cert + ε)` any
sample on any triangle of the face reached, which is the per-triangle
claim as one number, so no build of the kernel can turn
`tessellate`'s typed-error contract into a panic.

The sweep resamples `|S − Π|` on every emitted triangle unless
`--sizing-only` is passed; without that pass it costs one tessellation
per scene and nothing else, because the sizing columns are read off
sizing the lane already performed. Both run in ~4 s over the whole tour
in release.

The committed baseline is `docs/tess-budget-data/tess-budget-baseline.csv`
(WITH the resampling pass). Its row count is the file's own and grows
with the tour — 1,075 at the TESS-SPLIT re-cut, and every re-cut since
is an ordinary commit under "Re-cutting the baseline" below — so read
the file, not this sentence, for the current count.
It is NOT the cut this document's measurement was taken from and its
numbers are not the ones quoted below: "The finding" reports 1,025
faces and 390,100 grid cells against the shipped whole-patch schedule
of the time, where the committed file's own `grid_cells` sum was
46,102 at the TESS-SPLIT re-cut (the TESS-SPAN re-cut's was 163,182)
and grows with the tour like the row count, and the total-slack
figures come from that same pre-TESS-SPAN cut. Read the committed file as the gate's reference point and the
figures below as the pre-fix record they are labelled as. CI runs the
sweep `--sizing-only` and gates on REGRESSION against it: a scene's
mesh growing, a face's sizing getting wastefuller, or a scene silently
dropping out of the sweep. The gate reads triangle counts and the
sizing columns only, so the resampling is a cost the gate has no use
for; re-cutting the baseline drops the flag.

## The columns after TESS-SPAN

TESS-SPAN moved the shipped schedule onto the per-cell sizing this
document measured, which would have blinded a meter whose "shipped
grid" column was defined as the whole-patch product. The CSV was
re-derived (spec D-4) so BOTH regression kinds stay visible:

* `grid_cells` — the grid the lane ACTUALLY built (per-cell), read off
  the sizing hand-off. The gate's numerator.
* `patch_cells` (with `nu`, `nv`, `muu`, `muv`, `mvv`) — the retired
  whole-patch-sup schedule, recomputed by the meter as a
  COUNTERFACTUAL. `patch_cells / grid_cells` is the **held** span
  gain; a silent revert to whole-patch sizing multiplies `grid_cells`
  by it, which fires the triangle gate and the slack gate both.
* `span_cells` — **removed.** It was identically `grid_cells` (the
  same `band_schedule` sum, `Σ nuc·nvc`), so the **agreement** ratio
  built on it was 1.00 by arithmetic rather than by check, and neither
  of its two numbers counted a realised candidate. It is gone rather
  than re-derived — see "Why there is no realisation column", below.
* `opt_cells`, `span_opt_cells` — as before (cheapest split under the
  whole-patch bound / per cell). `grid_cells / span_opt_cells` is the
  gate's per-face recoverable-slack ratio, now carrying the split
  factor PLUS the banding and malign-snap forfeits, summed.

What guards `band_schedule` itself: (i) the per-triangle certificate —
`NurbsCellGrid::cert` reads the raw per-cell bounds independent of the
schedule, so an undersizing bug ends in refinement then a typed
refusal, and is falsified per-triangle under the `budget` feature's
deviation mode; (ii) this gate's growth rules against the committed
baseline; (iii) the committed render cells. **The stated blind spot: a
schedule bug that makes the grid COARSER while still certifying is
invisible to a growth-only gate** — accepted because the certificate
is the guarantee, and recorded here so the gate is not read as more
than it is.

## Why there is no realisation column

The removed column was described as verifying the lane's REALISATION
of the schedule — candidate generation, dedup, counting. It never did.
The question the removal answers is not "how do we make that number
real" but "is a realisation check worth having", and the answer is no,
for four reasons that are about the check rather than about its cost:

1. **A realisation ratio cannot see the blind spot named above.** That
   blind spot is a SCHEDULE bug. A realisation ratio divides what the
   lane built by what the schedule asked for, so a wrong schedule
   moves both sides together and the ratio stays where it was. The
   paragraph above is not the specification for such a check; it is
   the reason the check is not the guard, and it already names the
   three things that are.

2. **Both directions of a genuine realisation failure are watched by
   instruments that read the mesh rather than a predicted count — one
   of them exactly, the other approximately.** Realise the grid
   COARSER than the schedule asked and the triangles are larger than
   the cell bound admits, so the per-triangle certificate — computed
   from the realised triangle's own uv extents, per triangle, with no
   tolerance — refuses. That direction is caught exactly. Realise it
   DENSER (a dedup that stops deduping, a band emitted twice) and the
   triangle count grows, which is the gate's first rule — but that
   rule is a SCENE TOTAL at `GROWTH_TOLERANCE`, so a densification
   worth less than 5% of a whole scene's triangles passes it, and the
   slack rule cannot help because its numerator `grid_cells` is the
   schedule's own sum and never sees realisation. The dense direction
   is therefore bounded rather than caught. A realisation ratio would
   not close that gap either — see 3, which is why its tolerance
   could not be set any tighter than this one.

3. **The ratio would have no principled target, so its tolerance
   could only be read off the baseline.** `per_cell_candidates` states
   the mismatch itself: a shared cut line carries the union of BOTH
   adjacent bands' column points, candidates outside the trim box are
   dropped, and the end columns of each band are excluded. A realised
   point count is therefore a function of band structure and trim box
   that equals neither `Σ nuc·nvc` nor any other stated value. Any
   tolerance on it would have to be widened until today's sweep went
   green — which is the objection this document already makes to
   absolute thresholds, one level down.

4. **Nothing consumed it.** The gate reads triangle counts and
   `grid_cells / span_opt_cells`; the agreement ratio reached one
   printed figure and one report column and decided nothing.

**The third option, named so the rejection is on the record: keep a
realisation column with an honest name and NO assertion, as a reported
number.** Since the old column was a literal duplicate of
`grid_cells`, that option is not "keep it" but "build a new one" — and
4 says nothing wanted the number while 2 says the realised total is
already reported, as `triangles`, from the mesh rather than from a
count of candidates. A second reported number that nothing reads and
that duplicates an existing one in a different unit is what was just
removed.

`grid_cells` remains and is still the schedule's own sum; what it is
is stated where it is declared. The report prints `held / split /
total`; `tess-lint`'s gate rules are unchanged in shape (triangle
growth, per-face recoverable slack growth, vanished scenes).

## What the four numbers meant (pre-fix record)

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

**A re-cut that FOLDS IN uncovered scenes restores coverage, it does
not verify it.** This is the sentence to read before treating a fold as
good news, whoever is doing it. Folding an uncovered scene into the
baseline buys comparison FROM NOW ON; it cannot recover the window the
scene spent outside the gate. Whatever happened to its sizing in that
interval is unaudited and is not recoverable from the sweep data, so the
values a fold blesses are **current-state, not verified-optimal** — if
the scene regressed in the window, the fold enshrines the regression as
the new reference. *Coverage restored* is not *coverage verified*, and
only an audit closes the gap.

Measured instance (M9-5, PR #1037): the baseline cut at 31f052d2
predated five scenes already on the tour — `diechamfer` 68,
`benchlayout` 30, `diechamferblank` 26, `bench` 18, `hollowring` 4 =
**146 face rows** — swept, measured, printed and compared against
NOTHING on every run, while the gate reported clean. M9-5's own
baseline change was its 47 new rows only; the five scenes' fold was
executed by VERBS-TESSFOLD WITH the audit this section demands: each
scene's values verified against an expectation the fold does not
itself define (the chamfer scenes row-for-row against their
filleted/pipped twins, `hollowring` exactly against the torus grid
step (`mesh::sizing::torus_grid_step`), the bench scenes against
their introducing PR's claim and the box-face arithmetic) before
landing as reference. The class —
a comparison gate whose coverage decays silently as the corpus
outgrows its reference, while its verdict stays green by not looking —
is **#1038**, sibling to #1023, and stays open past the fold.

## The split schedule's aspect policy (RATIFIED 2026-08-16, PR #568)

The #547 measurement located the dominant sizing slack (~4.1x,
every NURBS wall) in `grid_steps`' AM-GM u/v decoupling. Fixing
it means choosing a different point on the same certified ellipse
— and the unconstrained optimum is a degenerate STRIP (leaf_a f2:
70×328 under the AM-GM schedule, 1×4905 at the optimum; parameter
aspect ~5·10³).
Nothing downstream (render normals, sliver-sensitive consumers)
has been polled on strips, and an honest aspect cap cannot use
parameter aspect (parameter ≠ 3-D shape) — it needs the first
fundamental form. The options:

- (i) **Unconstrained optimum** — max cell recovery, degenerate
  strips. REJECTED in this proposal: mesh quality is a consumer
  contract we have not renegotiated.
- (ii) **RECOMMENDED: FFF-aspect cap at a named constant** —
  choose the ellipse point minimizing cells subject to the 3-D
  aspect (from the first fundamental form at the cell's Hessian
  sample points) not exceeding **A = 16**: one octave beyond the
  4–8 range typical quality bounds tolerate, capturing most of
  the measured 4.1x on ruled walls (where the honest optimum is
  mildly anisotropic, not a strip). A is a spec-time constant
  with its reasoning at the definition site; re-tunable by
  ordinary measurement + re-cut.
- (iii) **Status quo AM-GM** — forgoes the dominant factor.

**RATIFIED: option (ii) with A = 16** (Evan's approval on the
#568 thread, 2026-08-16, noting correctly that (ii) strictly
generalizes both extremes — A is the dial). Executed by TESS-SPLIT
(#951) over TESS-SPAN's sizing functions; `NurbsFaceBound::split_steps`
and `mesh::nurbs_cert::ASPECT_CAP` are the shipped statement of record.
