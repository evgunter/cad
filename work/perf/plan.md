# PERF-PLAN — the performance work still owed

**Status: advisory companion to `DESIGN.md`, never overriding D1–D9.**
Two of its sections are **ratified** and live in DESIGN.md as the D9
addendum — §2.2's parallelism idioms and §3.3's GPU boundary table.
DESIGN.md is the contract; this doc is the detail behind it and the
standing register of *unbuilt* performance work.

**This is a plan, not a record.** Delivered items are deleted rather
than annotated, and expired claims are corrected in place rather than
struck through — git and the PR descriptions are the history. Last
resurveyed against `main` on **2026-09-10**: every `file:line` below
was re-checked by a static lane on that date, and the three seats in
§1.1 were measured by three lanes whose harnesses live on
`perf/explore-gui`, `perf/explore-kernel` and `perf/explore-dev`
(`work/perf/log.md` carries each lane's entry).

**Reading rule.** Claims here expire. The citations are deliberately
precise so that re-checking one before acting on it costs nothing —
do that, especially before quoting a cost as current.

## 1. What "interactive" demands

### 1.1 Measured workload (2026-09-10)

Three seats, measured on a 4-vCPU box in the profile each seat runs.
Every figure is a median of ≥3; cross-run spread is ±5–8 %.

**The GUI seat** (`crates/viewer`, release as shipped — the workspace's
`[profile.release]` keeps `debug-assertions = true`). On every document
whose committed edit lags, evaluation is noise and **the pick index is
83–97 % of the edit→picture wait**, rebuilt from scratch on every edit
whether one face moved or all of them:

| document | triangles | evaluate (memo) | index build | edit→picture |
|---|---:|---:|---:|---:|
| `die` | 348 | 7.9 ms | 1.7 ms | 14 ms |
| `die_composed_tour` | 974 526 | 88 ms | 1231 ms | 1375 ms |
| `gallery_ring` | 995 348 | 0.3 ms | 1517 ms | 1566 ms |
| `tube_ring` | 1 002 528 | 0.1 ms | 2130 ms | 2192 ms |

The index build splits ≈55/45 between `mesh::tessellate` and the
triangle BVH plus id map. On arrival (not per edit) the display
budget's probe tessellation adds 0.2–4.1 s of frozen window. The real
app under Xvfb lands a typed edit on `gallery_ring` in 3.5 s median,
the remainder being software rasterisation. Nothing is input-gated;
the edit door itself is 0.01–0.47 ms. 24 of the 29 corpus documents
draw under 5 000 triangles and finish an edit in under 15 ms.

**The kernel-API seat** (Rust through `demos/`, Python through
`pncad-py`; release both ways). With the shipped profile's debug
assertions, D1's per-operator tier-1 sweep is 45–72 % of a rebuild
(`die` 80 vs 36 ms; `demos/wild` 226 vs 54 ms; `demos/tour` 9.5 vs
7.0 s). With them off, `die`'s 36 ms is 55–60 % naming emission on the
boolean chain, quadratic in chain depth (`StableName` as a boxed
`BTreeMap` key). `assemble`'s tier-3′ census over the aggregate is
quadratic in solids (1.3 s at 161). Tessellation is 71 % of the tour
and 100 % of the ring rows; a NURBS-walled body's mass properties and
tier 3 are each a full certified quadrature (~160 ms). Python pays
nothing extra to evaluate (`prior=` is wired: 89 → 7.2 ms on a tail
edit; a memo-hit run is 0.29 ms) and ~22 % of a tessellation again to
read the mesh back as `Length` objects.

**The developer seat** (CI's dev/test profile). D1's per-op sweep is
5.7 % of test execution wall across the six kernel binaries and 31 %
of editor-core's. The largest single row is one `#[test]` in
`geom-brep` running a 70-cell sweep serially (46 s, the binary's whole
wall). The interval lane is 2.47× wall, 96 % of the delta in two M10
driver rows. CI wall is compile, not runtime.

Scale per Band 4 — hundreds of features, thousands of faces — is
still ahead of the corpus, which is a vocabulary corpus; the measured
documents above are the largest it has.

### 1.2 The four latency lanes

**Per-frame (60 Hz).** Nothing in the kernel runs here — a design
conclusion to preserve, not an accident. Rendering and hover-picking
consume artifacts the kernel already produced: the mesh with
per-triangle `Face` / per-polyline `Edge` back-references
(`crates/mesh/src/types.rs`) plus a client-side BVH. GUI-side and
GPU-shaped (§3).

**Per-edit preview (the critical path).** An edit mid-recipe means:
re-evaluate the changed node's downstream cone, re-run the booleans
under it, re-tessellate the faces that moved. The DAG memo that makes
the first of those cheap exists and works — `editor_core::evaluate`'s
`prior` argument (`eval/mod.rs:2249`) reuses every node whose content
key matches, from Rust and from Python (`pncad-py/src/py/value.rs:2098`).
It is worthless where the recomputed cone is the document (one wide
union, `die_composed_tour`, `heat_sink`) — a fact about those
documents, not a defect. **The third step does not exist**: the
viewer's pick index re-tessellates every root on every edit (§1.1),
and that is the preview lane's whole cost on a large document.

**Per-commit (~1 s budget).** Tier-1/2/3 validation, certified-δ
tessellation, mass properties. This lane carries most of the open
cost centers in §1.3, because the boolean and its gates all land
here.

**Background.** STL/STEP export, K-telemetry (`geom_core::k_stats`),
the M10 interval subdivision driver (embarrassingly parallel — see
§2.2), fine-δ export tessellation.

**The interval lane, honestly.** Interval replay costs several times
f64 flops plus lost vectorization, and the M10 subdivision driver
multiplies whole-model replays by sub-box count. But it is **never**
on the preview path — it is the certification lane and a CI lane — so
it is a throughput problem (parallelize, §2.2), not a latency one.
Stated plainly: **nothing about interactive latency justifies
weakening the trilean architecture; the f64 lane with K·ε escalation
IS the fast path.**

One calibration to carry: `ci.yml`'s OPT LEVEL note measured the
interval lane at 1.24× the default lane at opt-0 but **2.32×** at
opt-2 — opt-0's own overhead masks the enclosure cost. Any
interval-vs-f64 runtime read off a low-opt CI leg is flattered.

### 1.3 Open cost centers

Ranked by measured user-visible wait ÷ effort, with clarity as a hard
filter (a fix that makes the code harder to read or refactor is not a
fix here). All re-verified 2026-09-10; measured shares are from §1.1's
lanes. "Stop" means work that need not happen at all; "faster" means
the same work with a better algorithm. Each open row has an item file
in `work/perf/` or the program named.

| Cost center | Where | Measured | Kind | Ref |
|---|---|---|---|---|
| Display budget's probe can exceed the picture it sizes | `viewer/src/scene.rs:952` | 0.2–4.1 s frozen on open | stop | `fit-delta-probe-can-exceed-the-picture-it-sizes` |
| `assemble`'s aggregate tier-3′ census | `editor-core/src/product.rs:650` | n^1.96 in solids; 1.3 s at 161 | faster | `assemble-aggregate-census-is-quadratic-in-solids` |
| Gate then measure pays two certified quadratures | `demos/tour/src/main.rs:402,423`; the Python pair | ~half of the tour's 2.2–2.7 s of mass props | stop | `gate-then-measure-pays-two-quadratures` |
| Tier 3's +V check refines to the reporting target | `topo/src/validate.rs` (`validate_geometric_certified`), `geom-brep/src/props/quad.rs:113` | a false refusal at ε = 1e-12, not CPU | stop | `tier3-plus-v-needs-a-sign-and-pays-for-a-precision` |
| Tessellation is serial per face; mass props serial per face | `mesh/src/tessellate.rs:106`, props | up to ~4× on a 4-core box, on top of the rows above | faster (D9 idioms 1 and 2) | §2.2 |
| Whole-body pcurve re-mint per operation | `topo/src/pcurves.rs:1340` via `splitting/mod.rs:650`, `shell.rs:1704`, `transform.rs:650`; the narrowed door `mint_pcurves_of` exists at `:1392` | 0.1 % of `die`, 12 % of `die_composed_tour` | stop | `topo/producer-closing-mint-is-a-convention-with-thirteen-copies` |
| Boolean gate validates tier 1 twice | `topo/src/boolean/ops.rs:1423-1426` | ~2.5 % of `die` | stop | plan finding 4 |
| The arena-scan family | `body.rs:427,450,495`; `merge_faces.rs:1634`; `join.rs:282,520,539`; `combine.rs:415-419`; `validate.rs:5385-5390`; `containment.rs:238,246` | jointly ≤ 17 % of `die`, none separable at corpus size | faster | plan findings 5, 8, 9, 11, 13, 14 |
| Tier 3's per-face `Approx` grid | `PropsQuadLane::recertify_approx` | fixture blocked (no shell body carries an `Approx` face yet) | — | `tier3-approx-regrid-per-face-cost` |
| Python mesh egress as `Length` objects | `pncad-py/src/py/mesh.rs:325,339` | ~22 % of a tessellation again | faster | `python-mesh-egress-builds-length-objects` |
| CDT on nested near-cocircular loops | `mesh` | not reached by any corpus document | faster | §2.1 |
| `geom-core` has 2 `#[inline]` attributes | `geom-core/src/ring_interval.rs:87,93` | unmeasured; gated on moving a criterion row past its noise floor | faster | §2.3 |

Two constraints stated rather than a fix assumed:


## 2. CPU-first roadmap

The order is a commitment: **measure, then algorithms, then
architecture, then parallelism, then micro-optimization** — each item
names its target code and its trigger; the trigger is the license to
start.

**Measurement comes first, and it exists now.** `benches/` is a
criterion harness over the six rows §5 specifies, and
`docs/perf-data/criterion/` accumulates one entry per night that `main`
moves. Until 2026-08-27 there was no `benches/` and no `criterion`
dependency anywhere in the tree, so §2.3 gated every micro-optimization
on a harness nobody had built — a deadlock rather than a deferral, and
the reason the 2026-08-14 scan could put measured numbers on exactly one
of its ~20 findings (the one whose author built a harness first, which
then produced the report's largest result, 35×). That gate is open.

What *does* exist, and what each is good for:

| Instrument | What it measures | Trustworthy for |
|---|---|---|
| `docs/perf-data/criterion/` | per-kernel wall time for §5's six rows — tessellation at two δ, tier-2+3 validation, mass props, an extrude build, the two-brick boolean — release profile, debug assertions OFF | the **only** instrument that answers a per-kernel cost question; trend across nights, and a within-lane ratio (1e-4 vs 1e-6) |
| `docs/perf-data/rebuild-latency/` | per-document full-rebuild and incremental-recompute wall time, one entry per merge to `main`, each carrying its build environment | trend across merges; **not** absolute cost |
| `docs/perf-data/opt-level/` | nightly opt-level calibration — which `CARGO_PROFILE_*_OPT_LEVEL` the gate should run | the CI knob only |
| `docs/k-report-data/` | predicate decision counts | exact, deterministic, machine-independent — the **best** evidence base until the harness exists |
| `docs/TESS-BUDGET.md` + `tools/tess-meter` | over-tessellation ratios per knot-span cell | tessellation grid sizing |

`docs/k-report-data/` is still the one to reach for when a counter can
answer the question. It is immune to both the profile problem and the
contention problem that disqualify wall-clock numbers taken on a
developer box, and it is what localized the scan's SSI and predicate
findings. Reach for the criterion lane when the question is genuinely
about time — and read its README's noise floor first: cross-run spread
there is ~3–9% against within-run intervals of ±2–3%, so a single
entry's confidence interval is not the resolution of a comparison.

### 2.1 Algorithmic wins (they dominate; do these first)

- **CDT bulk loading.** `spade` ships `bulk_load_cdt` and it measures
  **35×** on the holed-planar case; the gap grows with point count.
  **It must not be adopted against stock `spade` 2.15.1**: that
  version's bulk loader iterates a `std::collections::HashSet` under
  the default randomly-seeded `RandomState` on its skipped-vertex and
  skipped-edge paths, which fire on cocircular input — exactly ours —
  and would let mesh bytes vary run to run, violating D9. Upstream a
  `Vec`/`BTreeSet` fix and `[patch.crates-io]` it first. The
  alternative remedy sometimes proposed — hierarchy-hinted insertion
  — is a **dead end**, measured at 39.26 s vs 39.05 s: the quadratic
  is the legalization cascade against a degenerate cocircular hull,
  not point location. `crates/mesh/src/lib.rs`'s §Performance section
  is the statement of record and is current.

  **The quadratic is now measured rather than argued, and on a body
  with no hole in it.** `benches/examples/counts.rs` sweeps δ over four
  decades on the washer and reports the exponent directly (2026-08-27,
  two runs, 4-core box):

  | δ | triangles | wall | exponent in n |
  |---|---|---|---|
  | 1e-2 | 308 | 0.75 / 0.98 ms | |
  | 1e-3 | 964 | 1.96 / 2.27 ms | 0.87 / 0.73 |
  | 1e-4 | 3040 | 9.44 / 10.7 ms | 1.36 / 1.35 |
  | 1e-5 | 9596 | 70.8 / 87.8 ms | 1.75 / 1.83 |
  | 1e-6 | 30340 | 642 / 733 ms | 1.91 / 1.84 |

  Triangle count grows as √10 per decade, as a chordal criterion should,
  and is bit-identical run to run (D9). The wall clock does not follow
  it: the exponent in triangle count climbs to **~1.85** and has not
  finished climbing at 30k triangles. The washer's annulus caps put
  their vertices on two concentric circles — near-cocircular by
  construction, which is finding 7b's degeneracy reached through a slit
  rather than through nesting. So the cost this item is about is real,
  is asymptotically n², and does not need a hole to appear.

  **The written trigger is still not met, and the distinction matters.**
  It says "a real fine-δ export need, or the corpus showing CDT
  dominance" — a claim about documents people actually build. One
  synthetic body swept by an example is not the corpus, and δ = 1e-6 is
  an export tolerance, which §1.2 puts in the *background* lane where
  642 ms is not a latency failure. What the sweep licenses is the
  measurement work: point `tools/tess-meter` or the harness at the
  corpus documents and see whether their faces reach this regime. If
  they do, the trigger is met on evidence; if they do not, this item
  stays queued and that is worth knowing too. The `spade` `HashSet` fix
  comes first either way.
- **Narrow the pcurve re-mint to the touched faces** (§1.3's top
  entry). Sequenced behind PCURVE P-1, and belongs on that program's
  slate.
- **The arena-scan family** — findings 9, 11, 13, 14, 5's pass 13.
  These are all the same bug wearing different hats: a linear arena
  scan inside a loop that already knows the key it wants. Each fix is
  an index or an inverse map, each is local, and each is
  D9-neutral (an inverse map changes no order). They are the highest
  ratio of payoff to risk in this document, and none needs the
  harness to justify — the complexity argument is the justification;
  the harness only sizes the win.
- **Incremental re-tessellation** — the architectural sibling of the
  DAG memo. The tessellator is already per-face (walk → CDT →
  certify), and the ratified content-keyed cache principle makes the
  memo key the bit-content of the face's geometry: D9 turns "same
  bits ⇒ same mesh patch" into a theorem. An edit that moves one boss
  re-tessellates only changed faces. The biggest preview-lane win
  available, and it is keyed work, not speculative.
- **One BVH crate, four duties — three wired.** `crates/bvh` is a
  deterministic AABB-BVH built in arena order with fixed splits and
  total tie-breaks (no hash order, no parallel-build nondeterminism).
  Live consumers: the boolean edge×face sweep
  (`topo/src/boolean/reduce.rs:626`), the placement-separation
  certificate (`topo/src/separation.rs:164`), and viewport picking —
  `Bvh::ray` under editor-core's hit-test service, live since GUI-1
  (`editor-core/src/resolve/pick.rs:162`). Still pending: **SSI
  seeding / C3 exhaustiveness** — `geom-brep/src/ssi/exhaust.rs` still
  enumerates cells by recursive bisection with a linear scan over
  tubes and says so ("Brute force, deliberately, for now"), which is
  this doc's trigger discipline working, not a missed delivery.
  `crates/bvh/src/lib.rs`'s header still says "two of them wired so
  far" over its four duties; it has not caught up with the picking
  consumer.

  The **conservative-superset contract** is the D9 obligation: a BVH
  may only prune pairs the exact predicate would reject, so the result
  stays a function of exact tests only. `face_box` is sound for every
  surface kind as of `2a24aa69` (`boolean/boxes.rs:723-788` — NURBS
  takes the control-net hull, which the convex-hull property makes a
  superset), and `edge_box` poisons its null-carrier arm
  (`:1153-1193`). **Any new pruning path owes the same proof**, and
  owes a differential scenario that is not built from axis-aligned
  planar bricks — the gap that let the NURBS hole live for three
  milestones was a suite whose every scenario was a `brick()`.

### 2.2 Parallelism under D9 (ratified — DESIGN.md's D9 addendum)

D9 permits "parallelism only in fixed reduction shapes". Two allowed
idioms, and the project's parallelism vocabulary — every use cites
them instead of re-deriving determinism:

1. **Indexed parallel map**: results written to slot *i* of a
   pre-sized buffer (indexed `par_iter().map().collect()`).
   Schedule-invariant by construction — combination is positional,
   not arithmetic. Bit-deterministic at any thread count.
2. **Fixed-shape reduction**: FP sums/mins are **never**
   `par_iter().reduce()` (rayon's reduction tree is
   schedule-dependent; FP non-associativity leaks the schedule into
   bits). Instead: idiom 1, then a *sequential* fold in arena order —
   or, if that fold profiles hot, a fixed-arity block tree (chunk
   size a named constant, combine order documented). Same bits every
   run, any thread count.

Euler-op sequences stay serial — each op mutates shared arenas, and
they are cheap; full-DAG rebuild is solved by memoization, not by
parallelizing surgery.

**State.** `rayon` is a dependency of `editor-core` alone; `par_iter`
lives at `eval/mod.rs:2380` (behind `EvalOptions::parallel`, default
`false` at `:2080`), `drive.rs:1184` (behind `DriveConfig::parallel`,
default `false` at `:361`), `stackup.rs:513,1831` and `mc.rs:473`
(default `true` at `mc.rs:94`). Neither `mesh` nor `bvh` names rayon.
The M10 driver's map gives 3.66× on one row solo and regresses a
saturated test binary by 6 % — a binary's floor is its longest serial
row, so turning it on is a per-row decision, not a global one.

Tempering expectation for whoever turns it on: the scheduler is
level-synchronous and the expensive corpus documents are *chains*
(`heat_sink` is a 5-long union chain, `die` a 21-long subtract chain),
which are depth-N and width-1. It will not move those rows. Turning
it on is worth doing for the wide documents and for keeping the lane
exercised — not as a fix for the corpus timings.

**The four unbuilt targets**, in value order:

- **Per-face tessellation** — the cheapest, and the blocker is small.
  `mesh/src/tessellate.rs` threads a `&mut positions` running counter
  through the face loop and each lane mints grid ids as
  `positions.len()`. Everything else is already read-only per face.
  Emitting *local* ids into a pre-sized buffer and assigning base
  offsets in a sequential arena-order fold is exactly the
  idiom-1-then-idiom-2 shape, and is bit-identical.
- **The M10 subdivision driver** — "embarrassingly parallel" is
  literally idiom 1 over sub-boxes.
- **Certification sampling** — per-edge, idiom 1.
- **Mass properties** — per-face fluxes, arena-order sum; the
  canonical idiom-2 example.

### 2.3 Micro level (profile-gated; mostly "not yet")

- **Opt-level is now a measured, moving setting — do not hard-code a
  belief about it.** The `[profile.dev.package]` opt-2 overrides for
  `spade` and `mesh` are in `Cargo.toml:246-249` and stand. What has
  moved is the workspace-wide level: the old "blanket opt-2 in CI is
  net-slower" verdict (#52/#53) was **reversed** — `ci.yml:1141`
  records why, both premises having expired (261 test binaries became
  one per crate; execution became ~79% of run wall) — and then the
  tree moved to **opt-level 1 on 2026-08-25**. `docs/perf-data/opt-level/`
  is a nightly calibration lane that re-decides it, and its README
  states the thesis directly: *a verdict expires and you can only
  tell by reading what it used to be.* Read the lane, not this
  paragraph, for today's level.
  - The durable lesson underneath, which has not expired: generic
    `T: Real` hot code monomorphizes into the **calling** crate's
    binaries, so per-package lib overrides can't reach it. Measure at
    the binary that instantiates the generics, not the crate that
    defines them.
- **`#[inline]` in `geom-core`** — two attributes in the whole crate,
  and it defines the scalars every hot loop calls through. Cheap to
  try, and the harness that confirms it now exists: the four
  microsecond rows are where a cross-crate call cost would show, and a
  candidate has to move one of them by more than the lane's ~10% noise
  floor to count.
- Later, evidence-gated: SoA layouts for batch predicate/cert
  sampling; LTO/PGO on release; SIMD in BVH traversal. The harness can
  now show a win, so these are gated on evidence rather than on the
  instrument — and on the instrument saying more than its noise floor.
- **Never**: fast-math flags, FMA contraction, or per-platform
  intrinsics in kernel code — D9 and the Q1 no-fused-ops rule
  (`Real` has no `mul_add`) already ban them; stated here so
  "optimization" never reintroduces them by reflex.

## 3. GPU acceleration, honestly scoped

### 3.1 Where GPU genuinely pays

- **Rendering and picking — the big one, and it is GUI-side.**
  `crates/viewer/README.md` commits to wgpu regardless of framework and ratifies
  GPU ID-buffer picking + CPU ray-cast confirm. Viewport LOD,
  silhouettes, section views live here. The kernel's whole obligation
  is meshes with stable back-references, which it already produces.
  No kernel changes: v1 shipped the wgpu viewport and the ID-buffer
  pass (`crates/viewer`), and LOD, silhouettes and section views stay
  on that side of the line.
- **Preview-grade tessellation — plausible, display-lane only.** A
  compute-shader evaluator for analytic surface grids could produce
  *uncertified preview* meshes for drag feedback — exactly parallel
  to the ratified "preview may march uncertified" SSI stance: a
  degraded lane that never feeds the kernel. Certified-δ meshes
  (`mesh/cert.rs`, the export promise) stay CPU. Honest caveat: CDT
  *topology* does not GPU-parallelize — GPU buys vertex
  evaluation/refinement of existing patch topology. A GUI-milestone
  experiment, not a kernel commitment.
- **Batch f64 value evaluation (M10 Monte Carlo) — marginal.** Each
  sample is a full model *rebuild* (topology surgery, CPU-shaped),
  not a bare function evaluation; rayon over samples is the right
  tool. GPU would accelerate the cheap part.

### 3.2 Batch certified predicates on GPU: assessed and tabled

The tempting idea — evaluate thousands of interval predicates (M10
clearance, SSI exhaustiveness) on GPU — fails on three independent
grounds, each disqualifying:

1. **Directed rounding.** CUDA exposes per-op directed-rounding
   intrinsics (`__dadd_rd` …) — GPU interval arithmetic is
   established research there — but that is vendor lock plus an
   immature Rust toolchain. Vulkan/SPIR-V rounding decorations cover
   conversions only; WGSL exposes no rounding control. The portable
   fallback (round-to-nearest + outward ulp widening) is sound only
   under strict-IEEE per-op guarantees wgpu-class APIs do not give
   (denormal flush and FMA contraction are permitted).
2. **f64.** wgpu's `SHADER_F64` is native-only and spotty; WGSL
   baseline is f32. An f32 interval lane widens every enclosure and
   floods the K·ε escalation band — certified answers get *rarer*.
3. **D9.** Bit-identical outputs across backends is unobtainable on
   GPU (vendor/driver reduction shapes, contraction). So GPU output
   can never decide kernel topology; at most it pre-filters — and a
   filter must satisfy §2.1's conservative-superset contract *and* be
   auditable, at which point the CPU BVH already does the job.

**Tabled** (revisit post-M10 at the earliest, alongside the tabled
in-house interval transcendentals — same "rigorous numerics we fully
control" prerequisite). Not a loss: the interval lane's workloads are
throughput-shaped and rayon-parallel (§2.2).

### 3.3 The boundary (ratified — DESIGN.md's D9 addendum)

| Work | Home | Why |
|---|---|---|
| Rendering, LOD, ID-buffer picking | GPU, GUI milestone | ratified direction (GUI-DESIGN); no kernel coupling |
| Preview (uncertified) surface evaluation | GPU-eligible, GUI milestone experiment | display lane; never re-enters kernel |
| Certified tessellation, export meshes | CPU forever* | export promise needs certified bounds |
| Booleans, splitting, SSI, predicates | CPU forever* | D9 + certification; GPU pre-filter not worth the audit |
| Euler ops, validators, arena surgery | CPU forever | pointer-chasing, serial by nature, already cheap |
| Interval lane / subdivision driver | CPU (rayon) | §3.2; embarrassingly parallel on CPU already |

\* "forever" = for this project's plannable horizon; §3.2's grounds
are re-checkable facts (rounding control, f64, portability), and the
table should be revisited only if they change materially.

## 4. The idealized/realized dual-code pattern

**Ratified shape (Q-P2):** selective adoption per hot kernel, with a
CI differential suite as the standing pin and an opt-in `shadow-exec`
feature as a scalpel. No always-on debug shadow.

### 4.1 Why it works here

- **D9 makes "identical" a real oracle.** Ordinary numerics drowns
  ref-vs-fast comparison in tolerance fudge ("agree to 1e-12 —
  usually"). Here both versions must produce **bit-identical**
  outputs (same reduction shapes, libm, no FMA): the pin is
  `assert_eq!` on bytes — zero false-pass headroom; a divergence is a
  definite bug, never noise. Machinery ready-made:
  `topo::iso::canonical_form`, lineage-scoped key identity for arena
  diffs, and the multi-ε CI matrix + interval lane multiplying every
  differential corpus for free.
- **Purity makes replay cheap.** Models are values; the recipe is
  data (D8). A differential corpus is a directory of recipes replayed
  through both implementations — no mocking, no setup.
- **Prior art, not invention.** Crypto reference implementations
  (fiat-crypto *generates* the realized form from the idealized one
  with proof); differential/back-to-back testing (McKeeman, csmith,
  DO-178 avionics); refinement stacks (CompCert, seL4) with a
  machine-checked simulation proof instead of tests. Known limit
  (Knight–Leveson): independence is partial — two versions written
  from one misreading agree on the same wrong answer. The pin catches
  *divergence*, not shared spec error; adversarial review remains the
  defense for the latter.

### 4.2 The costs, stated without discount

- **Double maintenance** on every behavioral change — acceptable only
  where the realized form is genuinely hard to read, i.e. where the
  idealized copy pays rent as the definition.
- **Divergence rot**: an idealized version not executed in CI decays
  into wishful documentation — worse than a comment because it looks
  load-bearing. Mitigation is structural: the pattern is permitted
  only **with** its CI differential suite; an unpinned idealized copy
  is deleted, not kept.
- **Dead-code inversion**: features landing realized-first "for
  speed" silently make the fast code the definition again. Rule:
  behavioral changes land idealized-first (it is the spec); the
  realized diff follows in the same PR.

### 4.3 Where it is live

The boolean sweep is the pilot and it works as designed.
`SweepStrategy` (`topo/src/boolean/reduce.rs:69`) keeps the O(n²)
brute-force scan compiled and executable as `Idealized` beside the
BVH-backed `Realized`, selectable through `boolean_op_with` and
`EvalOptions::boolean_sweep`. Production entries hard-code `Realized`;
the differential suite runs both and compares. Deliberately **not**
part of any content key — results are bit-identical either way.

### 4.4 Where it pays next, and where the single version IS the realized one

**Adopt (hot kernels whose fast form stops being self-evident):**

- **Tessellation insertion path** when bulk-loading lands (§2.1): the
  current sequential-insertion CDT (trusted, documented) becomes the
  idealized reference; `bulk_load_cdt` the realized path; pin =
  byte-identical `Mesh` (the D9 mesh contract gives this meaning).
  This is the next scheduled use.
- **SSI marching stepper** — exactly the "tricky optimized numerics"
  shape, and its idealized form doubles as the spec the exhaustiveness
  contract audits.
- **Batch predicate/certification evaluation** if SoA/SIMD ever
  lands: the scalar per-edge loop stays as the definition.

**Do not adopt (the readable version IS the realized one):** Euler
operators (small surgery, never hot; the code is the definition —
DESIGN.md's own thesis), validators (they *are* the executable spec;
a dual would pin the spec to itself), mass properties (closed forms),
profile canonicalization, STL writers. Default for new code: **single
version until the optimization diff stops being reviewable**; the
dual structure is earned by a measured win, never speculative.

### 4.5 Why shadow execution is not a build mode

Feasible — purity makes it a five-line wrapper — but wrong as a
standing mode, for two reasons: (a) the asymptotic gap is the point —
on exactly the inputs where the realized BVH matters, the idealized
O(n²) shadow is unusable, so always-on shadowing forces tiny models
and samples the least interesting region; (b) the debug lane already
carries per-op tier-1 asserts (§1.3), and doubling it taxes every
developer run to re-check what CI checks better. The shape instead:

- **CI differential suites** — proptest-generated + pinned corpora
  through both versions, byte-equality asserts, all ε rows + interval
  lane. The standing pin.
- **A `shadow-exec` cargo feature** per dual module — opt-in wrapper
  asserting bit-equality, for hunting a divergence a real model
  exhibited. A scalpel, never default-on.
- **Nightly corpus differential** — catches distribution-shift bugs
  proptest's generators miss, without taxing interactive dev.

## 5. What to do next

The dispatch order, from §1.3. Units are cut one at a time from the
top; every kernel unit runs the full v6 dual; measurement-only,
demo-only and test-only units record no A/B row.

**Block PERF-B1** (three kernel units, drawn as one protocol block):

1. **Torus chart sizing** — landed (PR 2307): `torus_grid_steps`
   with the proved `(AΔu² + 2BΔuΔv + CΔv²)/8` bound and the closed-form
   split; `hollowring` 3.98 M → 165 k triangles at 0.1 mm, the torus
   bench rows −95 %, the tour ~10 → ~7.3 s. Residue on the slate:
   `torus-sizing-reads-no-phi-window`; out-of-fence findings filed as
   `nurbs-cert-spends-the-bound-twice` and
   `sphere-sizing-margin-is-the-coupling-factor`.
2. **`StableName` keying** — landed (PR 2311): `NameRef`, a shared
   handle with a per-table order stamp sealed at first operand use,
   every name and byte unchanged; `die` 33.7 → 18.8 ms, the emitter
   60 % → 27 % of boolean time with the per-name cost flat in chain
   depth. Residual on the slate:
   `naming-a-boolean-chain-is-theta-names-per-step`.
3. **Per-face patches in local ids** — landed (PR 2308): lanes return
   `Patch { interior, triangles: [PatchVertex; 3] }` with
   `PatchVertex::{Shared, Local}`, bases assigned by an arena-order
   fold, 40 committed mesh digests unchanged across the refactor. The
   remaining `&mut FaceBounds` borrow in the trimmed lane is the one
   obstacle left to a `par_iter` over faces; the three lanes' argument
   lists are not yet one shape (the memo unit's spec takes that).

**Beside the block, low-risk (single review, no row):**

- The display budget's probe never larger than the picture (viewer).
- Gate-then-measure takes the certificate it already computed (tour,
  Python surface).
- `budget_faces.rs` split into rows nextest can spread — landed (PR 2321).
- `gathers_on_this_thread` compiles without debug assertions — landed
  (PR 2328).

**Block PERF-B2**, after B1 and Ev's D1 ruling:

- The per-face patch memo across index builds — landed (PR 2315):
  node-level `PickMemo` keyed on the eval memo's own reuse condition
  (content + naming key) and face-level `PatchMemo` behind
  `mesh::tessellate_with`, generational eviction, bit-identical to a
  fresh tessellation across edits. `die_composed_tour` index 456 → 174 ms
  (tessellate 293 → 25); `kitchen_sink` 3.2 → 0.6 ms; the ring
  documents miss everything on a bump (+2 %). The BVH is now ~85 % of a
  memo'd index (`bvh-is-the-index-after-the-memo`).
- Parallel per-face tessellation (idiom 1) and per-face mass-property
  fluxes (idiom 2).
- Tier 3's +V check with a sign-sufficient door.
- `assemble`'s aggregate census behind the BVH pre-filter.
- D1 once-per-door — landed (PR 2313, Ev's ruling on PR 2305): the
  tier-1 sweep runs once per public door (a `Surgery` guard on `Body`,
  the `per-op-postcondition` feature as the scalpel, run per PR).
  `die` 86 → 44 ms in the shipped profile against a 39 ms no-tier-1
  floor; `demos/wild` 261 → 46 ms; editor-core's test wall 56 → 20 s.
  Residue on the slate: `door-scopes-outside-topo-are-unguarded`.

**On the trigger list, unchanged:** CDT bulk loading (the corpus does
not reach the quadratic; the `spade` `HashSet` fix precedes adoption
either way); SSI seeding on the BVH; the arena-scan family (complexity
argument stands, corpus does not size it — land them when a body large
enough to show them exists, one per PR with the complexity claim).

**Owed to the measurement record:** a curved-body row in `benches/`
(criterion's six rows are planar microseconds where curved bodies are
100+ ms, so they do not predict the kernel-API seat).

**Premature, named to stay dead until their triggers:** any GPU work;
SIMD/SoA/PGO/LTO; parallelizing Euler sequences; benchmark *gates*
(the lane is reporting-only and Q-P4 is why); micro-tuning validators
beyond the rows above.

## 6. Settled, not re-litigated

Answered by Ev at #49 (2026-07-21) and executed:

- **Q-P1** — §3.3's GPU boundary table and §2.2's parallelism idioms
  are ratified into DESIGN.md as the D9 addendum; this doc stays
  advisory detail behind it.
- **Q-P2** — idealized/realized: selective adoption, CI differential
  pin, `shadow-exec` opt-in; no always-on debug shadow.
- **Q-P3** — degraded (uncertified) previews are acceptable.
- **Q-P4** — no pre-merge performance gate. The harness runs
  post-merge on `main`; the trend must merely predate the first
  change it would police.
