# PERF-PLAN — the performance work still owed

**Status: advisory companion to `DESIGN.md`, never overriding D1–D9.**
Two of its sections are **ratified** and live in DESIGN.md as the D9
addendum — §2.2's parallelism idioms and §3.3's GPU boundary table.
DESIGN.md is the contract; this doc is the detail behind it and the
standing register of *unbuilt* performance work.

**This is a plan, not a record.** Delivered items are deleted rather
than annotated, and expired claims are corrected in place rather than
struck through — git and the PR descriptions are the history. Last
resurveyed against `main` on **2026-08-26**; every claim below was
re-checked at its cited `file:line` on that date.

**Reading rule.** Claims here expire. The citations are deliberately
precise so that re-checking one before acting on it costs nothing —
do that, especially before quoting a cost as current.

## 1. What "interactive" demands

### 1.1 Assumed workload (stated, not ratified)

Nothing here has been measured against the shipped v1 GUI
(`crates/viewer`); these are the standard interactive-CAD envelope,
assumed not measured: **~16 ms**/frame (camera, hover, selection);
**~50–150 ms** per gesture preview (drag a dimension → new solid);
**~1 s** per committed edit; background for the rest. Scale per
Band 4: hundreds of features, thousands of faces.

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
the first of those cheap **exists and works** —
`editor_core::evaluate`'s `prior` argument (`eval/mod.rs:1004`) reuses
every node whose content key matches. It is reachable from Rust and
**not** from Python: `pncad-py`'s binding hard-codes
`prior = None` and `EvalOptions::default()`
(`crates/pncad-py/src/py/value.rs:751-760`), so the whole
memo/parallel apparatus is invisible to the binding that most needs
it. That gap is §2.3's cheapest open item.

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

Ranked by (payoff × confidence) ÷ effort. All re-verified 2026-08-26;
none has been fixed. Sources are `docs/PERF-SCAN-2026-08.md`'s
findings, whose numbering is kept so the two docs cross-reference.

| Cost center | Where | Shape | Lane | Ref |
|---|---|---|---|---|
| Whole-body pcurve re-mint per operation | `topo/src/pcurves.rs:995` — `body.pcurves.clear()` then every face re-walked | chain of N booleans on a growing body ⇒ quadratic | commit | 7 |
| CDT insertion on nested near-cocircular loops | `mesh` — a planar face with a hole | quadratic; near-linear otherwise | export | 7b |
| Boolean gate validates tier 1 twice | `topo/src/boolean/ops.rs:1395` — `validate` then `validate_closed`, each running `tier1` | 2× a 13-pass arena sweep, in release | commit | 4 |
| Kill-direction Euler ops are O(arena) | `topo/src/body.rs:417,440,485` — three full-arena `.values().any()` orphan scans per kill; `description_surfaces` allocates a `Vec` per curve | zip killing n seams ⇒ O(n·N) | build | 9 |
| `merge_group` rescans the edge arena per kill | `topo/src/merge_faces.rs:755` — `loop { for edge in self.edges() … break }` | O(kills × E) | commit | 11 |
| Boolean `join` is O(n³) | `boolean/join.rs:282` loops `find_match`, itself O(open²) over slot pairs (`:520`) | plus a `Vec` alloc per slot scan | commit | 13 |
| `graft_solid` is O(E²) | `boolean/combine.rs:411` — `.find(\|(_, e)\| e.curve == k)` inside the per-curve loop | missing inverse map | commit | 14 |
| `StableName` nests one `Box` per boolean | `editor-core/src/names/role.rs:290-382` | O(chain²) on a long boolean chain | commit | 15 |
| Tier-3 runs twice on the product path | `editor-core/src/product.rs:410` and `:445` | duplicated over the same entities | commit | 16 |
| Tier-1 pass 13 is quadratic in null scaffolds | `topo/src/validate.rs:3876-3890` — per null-scaffold curve, a full edge-arena `filter().count()` | worst exactly mid-boolean | commit | 5 |
| Per-op debug full-body tier-1 | `topo/src/euler.rs:60-72` — D1's **ratified** postcondition clause | body construction Θ(ops × N) in every debug/CI row; **measured 2026-08-27 at 6.5× on an extrude build and 5.2× on the two-brick boolean**, and free on validation, mass props and tessellation | CI/dev | 5 |
| `point_in_loop` re-decides loop-intrinsic facts per query | `topo/src/splitting/containment.rs:238` | per-query work that is per-loop | commit | 8 |
| `geom-core` has 2 `#[inline]` attributes total | `crates/geom-core/src/` | cross-crate call overhead on the hottest scalars | CI/dev | 17 |

Two entries need their constraint stated rather than a fix assumed:

- **The per-op debug validate is D1, not an oversight.** `euler.rs`'s
  module docs call it "D1's ratified clause". Making it cheaper is a
  design change (a declared-delta check without the full tier-1
  sweep, say), so it goes through DESIGN.md, not through a
  performance PR. It now has a price: turning debug assertions on
  costs **6.5×** on `kernel/build/extrude` and **5.2×** on
  `kernel/boolean/two_bricks`, and nothing measurable on the other four
  rows — the first measurement of this clause, taken by building
  `benches/` both ways (`benches/Cargo.toml` records it). Two
  consequences. It lands squarely on the topology-surgery rows, which
  is why the benchmark profile turns it OFF: a 5× constant on exactly
  the scenarios the arena-scan family below is meant to move would
  round a real algorithmic win away. And it means every debug and CI
  row in this repository pays it — which is a fact about the cost of
  the gate, not an argument about D1.
- **The pcurve re-mint is under active surgery.** `PCURVE-PLAN.md`'s
  P-1 rewrites the edge-description vocabulary that `mint_pcurves`
  serves. Narrowing the re-mint to the faces a boolean actually
  touched is not on P-1's slate, and doing it *before* P-1 lands
  means doing it twice — but it should be **on the PCURVE slate**,
  not floating here, because that program owns the code.

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

**State: one target built, and it is switched off.** `rayon` is a
dependency of `editor-core` alone and `eval/mod.rs:1083` is the only
`par_iter` in the workspace. It is D9-clean as written (indexed map
into per-node slots), but `EvalOptions::default()` sets
`parallel: false` (`:983`) and every shipping caller takes the
default; `parallel: true` appears once, in a test.

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

- **Reach memoization and parallelism from the Python binding.**
  `pncad-py`'s `evaluate` hard-codes `prior = None` and
  `EvalOptions::default()` (`py/value.rs:751-760`). Threading a prior
  `Evaluation` and an options object through is small, and it is the
  difference between the binding rebuilding the whole document on
  every edit and rebuilding the edited cone. Cheapest item in this
  document.
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
  GUI-DESIGN.md commits to wgpu regardless of framework and ratifies
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

**The benchmark harness was this document's blocking item from the
start, and it landed on 2026-08-27.** What follows is ordered against
what it now says.

1. **Read the criterion trend before optimizing anything.** The harness
   landed 2026-08-27 (`benches/`, the nightly's `criterion benchmarks
   (reporting)` job, `docs/perf-data/criterion/`), which is what opened
   §2.3's gate. The first readings, on a 4-core box, are the numbers
   this document had lacked until then:

   | row | median |
   |---|---|
   | `tessellate/washer/1e-4` | 10.5 ms |
   | `tessellate/washer/1e-6` | 690 ms |
   | `kernel/validate/tier23_washer` | 24 µs |
   | `kernel/mass_props/washer` | 1.7 µs |
   | `kernel/build/extrude` | 24 µs |
   | `kernel/boolean/two_bricks` | 130 µs |

   **What that table settles, and what it does not.** Tessellation is
   two to four orders of magnitude above every other row, and §2.1's δ
   sweep shows its cost growing as ~n^1.85 in triangle count on a
   near-cocircular body — so the CDT quadratic is measured, and the
   `spade` `HashSet` fix that must precede bulk loading is the next
   unit of work there. It does **not** discharge that item's written
   trigger, which asks for the *corpus* showing dominance rather than
   one synthetic body at an export tolerance; §2.1 says what would.
   The other four rows are microseconds, which prices the arena-scan
   family honestly: they are complexity fixes for bodies far larger
   than these, and item 2 below does not wait on the harness precisely
   because these scenarios are too small to show them.

2. **The arena-scan family** (§2.1) — findings 9, 11, 13, 14, and
   pass 13. Local, D9-neutral, justified by complexity argument
   rather than by wall-clock, so they do not wait on item 1. Land
   them one per PR with the complexity claim in the PR description.

3. **Reach the memo from Python** (§2.3) — smallest diff in this
   document, largest change in what the binding's users experience.

4. **Boolean gate double tier-1 and product-path double tier-3**
   (findings 4, 16) — XS/S, in release, on the commit lane.

**On the trigger list:** CDT bulk loading — its quadratic is measured
now (§2.1's sweep), its trigger is not yet met, and the corpus reading
that would meet it is the cheap next step; the `spade` `HashSet` fix
precedes adoption either way. SSI
seeding on the BVH: its own module says "when profiling asks for it",
and the harness does not benchmark SSI, so that trigger needs a row
before it can fire. Per-face tessellation parallelism (§2.2) — the
tessellation rows above are the case for it.

**The GUI side owns** everything in §3.1. v1 shipped wgpu rendering
and ID-buffer picking on the kernel's deliverables (meshes with
back-refs, cancelable evaluation service, BVH); LOD and the
preview-tessellation experiment are still unbuilt there.

**M10 owns** the parallel subdivision driver (idiom 1 over sub-boxes)
and interval-lane throughput work if certification wall-times demand
it.

**Premature, named to stay dead until their triggers:** any GPU work;
SIMD/SoA/PGO/LTO; parallelizing Euler sequences; benchmark *gates*
(as opposed to the trend — the lane is reporting-only and Q-P4 is why);
micro-tuning validators or mass props beyond the specific findings
above. Mass props at 1.7 µs and validation at 24 µs are now measured,
which makes "premature" a fact about those two rather than a posture.

## 6. Settled, not re-litigated

Answered by Evan at #49 (2026-07-21) and executed:

- **Q-P1** — §3.3's GPU boundary table and §2.2's parallelism idioms
  are ratified into DESIGN.md as the D9 addendum; this doc stays
  advisory detail behind it.
- **Q-P2** — idealized/realized: selective adoption, CI differential
  pin, `shadow-exec` opt-in; no always-on debug shadow.
- **Q-P3** — degraded (uncertified) previews are acceptable.
- **Q-P4** — no pre-merge performance gate. The harness runs
  post-merge on `main`; the trend must merely predate the first
  change it would police.
