# PERF-PLAN — Performance & GPU Roadmap; the Idealized/Realized Dual-Code Question

**Status: MERGED AND ADVISORY (Q-P1 answered by Evan's sign-off, #49,
2026-07-21). The ratified pieces — §2.2's deterministic-parallelism
idioms and §3.3's GPU boundary table — are folded into DESIGN.md as
the D9 addendum (M3 exit sweep); DESIGN.md is the single contract,
this doc the advisory detail behind it. Update at the M4 8c exit
sweep (2026-07-27): the rebuild-latency lane EXISTS — M4 8a (#118)
wired per-document full-rebuild and incremental-recompute timings
into hosted CI as REPORTING rows over the Band 4 corpus (committed
baseline JSON, box-relative numbers, no threshold gate), exactly the
measured-not-gated posture F8 ratified; this doc stays advisory.**
**Update at the 2026-08-14 performance scan
(`docs/PERF-SCAN-2026-08.md`): §1.3's ranking is now three milestones
old and four of its six entries have moved. Every expired claim is
marked inline below with a dated `[STALE …]` / `[SUPERSEDED …]` note
pointing at the finding that retired it; the original prose is left
intact, because what this doc believed at M3-start is the record it
exists to keep. Nothing in §3 (GPU) or §4 (dual-code) was touched by
the scan — those sections stand as written.**
Companion to `DESIGN.md`
(never overrides D1–D9) and
`GUI-DESIGN.md`. Written at M3-start (2026-07-21) against the M2
codebase; claims cite files. Purpose: decide *now* which performance
work is architectural (cheap early, brutal to retrofit), which is
profile-gated engineering, and which is honestly premature — so Band 4
"performance at scale" never gets invented ad hoc. Method note: no
benchmark suite was built; quantitative grounding is the measured
characteristic pinned in `crates/mesh/src/lib.rs` §Performance plus
complexity reasoning against the code. GUI assumptions are labeled.

## 1. What "interactive" demands

### 1.1 Assumed workload (stated, not ratified)

The GUI is unbuilt; these are the standard interactive-CAD envelope,
assumed not measured: **~16 ms**/frame (camera, hover, selection);
**~50–150 ms** per gesture preview (drag a dimension → new solid);
**~1 s** per committed edit; background for the rest. Scale per
Band 4: hundreds of features, thousands of faces. GQ2's per-node
result DAG and G1's evaluation service are taken as given.

### 1.2 The four latency lanes, mapped to this codebase

**Per-frame (60 Hz).** Nothing in the kernel runs here — a design
conclusion to preserve, not an accident. Rendering and hover-picking
consume artifacts the kernel already produced: the mesh with
per-triangle `Face` / per-polyline `Edge` back-references (the
ratified M2 PR 6 picking contract, `crates/mesh/src/types.rs`) plus a
client-side BVH. This lane is entirely GUI-side and GPU-shaped (§3).

**Per-edit preview (the critical path).** An edit mid-recipe implies:
re-evaluate downstream features (M4 DAG; today: full rebuild via
`crates/sweep/src/extrude.rs` / `revolve` Euler-op sequences), then
booleans/splitting (M3: the edge×face sweep, `docs/archive/M3-PLAN.md` PR 4 —
**documented quadratic**, deferred deliberately: "Boolean performance
(BVH/spatial indexing for the edge×face sweep) — correctness first"),
then re-tessellate changed faces. Ranked by (frequency × cost ×
latency-sensitivity), the preview lane's costs are:

1. **Tessellation** — the measured dominant cost.
   `crates/mesh/src/lib.rs` pins it: CDT insertion is quadratic in
   per-face point count (`spade` sequential point location); washer
   body ~19 ms at δ = 1e-4, ~1.2 s at 1e-6, >11 min at 1e-9; point
   count ~1/√δ per axis, so 100× tighter δ ⇒ ~10⁴× CDT time. Even at
   preview δ, full-body re-tessellation per drag frame is the first
   casualty at Band 4 scale.

   **[STALE 2026-08-14 — wrong in three ways; PERF-SCAN finding 7b,
   the one benchmarked finding in that report.]** (i) The δ-scaling
   arithmetic is wrong by ~150× on its own datapoint: the washer's
   1e-4 → 1e-6 step measures **63×**, not ~10⁴×. (ii) The quadratic is
   not general. Grid-based curved faces (cylinder/cone/sphere/torus/
   NURBS) are **near-linear**, and so is a single circular loop; the
   blow-up is specific to **nested near-cocircular boundaries** — a
   planar face with a hole. The washer's entire 1.2 s is its two planar
   annulus faces. (iii) It is not on a preview lane, because there is
   none: `mesh` is a **dev-dependency** of `editor-core`, absent from
   `pncad-py`, and its only non-test consumer is the STL writer. So
   this belongs in the background/export lane, not rank 1 of the
   preview lane. The remedy half of this doc's advice survives and got
   stronger — `bulk_load_cdt` measures **35×** on the holed-planar case
   — but see the D9 hazard in finding 7b: spade 2.15.1's bulk loader
   iterates a `HashSet` under `RandomState`, so it must not land
   unpatched. The "hierarchy-hinted insertion" half of the remedy named
   in `mesh/lib.rs` is a **dead end** (measured 39.26 s vs 39.05 s):
   the quadratic is the legalization cascade, not point location.
2. **Booleans/splitting (M3 on)** — the quadratic edge×face sweep
   plus per-pair trilean classification; every boolean node
   downstream of the edit re-runs it.

   **[SUPERSEDED 2026-08-14 — the sweep is no longer quadratic.]**
   `topo/src/boolean/reduce.rs:432` queries the BVH. Precisely: the
   brute-force scan is **shipped production code, not test code** — a
   live runtime arm (`reduce.rs:422`) of the public `SweepStrategy`
   enum, selectable through `boolean_op_with`. What is true is that no
   production *caller* selects it: `union`/`intersect`/`subtract` and
   every internal entry hard-code `SweepStrategy::Realized`, so only
   the differential suite passes `Idealized`. That is §4.4's
   idealized/realized pilot working exactly as designed — the O(n²)
   reference stays compiled and executable so the pin can run both
   paths and compare, which is the whole point. What replaced it as
   the boolean's dominant term is **whole-body pcurve re-certification**
   — `mint_pcurves` clears and re-mints every face on every boolean, so
   a chain of N booleans on a growing body is quadratic (PERF-SCAN
   finding 7, nine call sites across five crates). Two further
   quadratics the M3 ranking did not anticipate: `merge_group` rescans
   the whole edge arena per kill (finding 11), and `join` is O(n³) with
   hoistable invariants (finding 13).
3. **Feature rebuild** — Euler-op sequences are O(entities built)
   with small constants (`crates/topo/src/euler*.rs`); cheap per
   feature, but linear in downstream-DAG size without M4 memoization.

   **[STALE 2026-08-14 — false in release; PERF-SCAN finding 9.]**
   Kill-direction ops are **O(arena), not O(1)**: `kev`/`kef`/`kvfs`/
   `kemr`/`split_edge` each pay three full-arena orphan-hygiene scans
   (`topo/src/body.rs:417-428,440-456,485-494`), one of which allocates
   a `Vec` per curve. A zip killing n seam edges costs O(n·N). The
   splice itself is O(loop length) as claimed — the tail is not.
   Memoization did land and works (11× on `die`), but is **switched
   off in every shipping caller** (finding 6).
4. **Validation + certification** — tier 1 is per-op debug-assert
   (absent in release); tier 3 samples per-edge dihedrals and 9-point
   residual schedules (`CERT_SAMPLES`, `geom-brep/src/certify.rs`);
   the 12-pass tier-1/2 pipeline (`topo/src/validate.rs`) is linear
   arena passes. Not the bottleneck; do not optimize.

   **[STALE 2026-08-14 — "do not optimize" is now wrong in three
   specific ways; PERF-SCAN findings 4, 5, 16.]** The pipeline is 13
   passes, not 12, and each is not one sweep: tier 1 is ≈40 arena
   sweeps plus ~13 `SecondaryMap` allocations per call. And: (a) the
   per-op debug-assert is a **full-body** tier-1 pass, not an O(1)
   check, making body construction Θ(ops × N) in every CI test row
   (`euler.rs:1975-1992`, 15 call sites); (b) pass 13 is **quadratic**
   in null-scaffold curves (`validate.rs:3051-3060`), worst exactly
   mid-boolean when (a) fires most; (c) **tier 3 — scoped here to the
   per-commit lane — now runs unconditionally in release** on the
   boolean, merge, product and step-import paths, and twice over the
   same entities on the product path. The boolean gate also runs tier 1
   twice per call (`boolean/ops.rs:1209-1213`). The general claim that
   validation is linear still holds; the conclusion drawn from it does
   not.

**Per-commit (~1 s budget).** Full tier-2/3 validation, certified-δ
tessellation, mass properties. Mass props are divergence-theorem
closed forms per face (`crates/geom-brep/src/props/` — `planar_face`,
`curved_face`, no quadrature): O(faces), microseconds-to-milliseconds;
background-eligible but cheap enough not to bother.

**Background.** STL/STEP export (`crates/stl/` is a trivial linear
writer; f32 narrowing documented), K-telemetry (`geom_core::k_stats`),
the M10 interval subdivision driver (Q1 calls it embarrassingly
parallel — correctly, see §2.2), fine-δ export tessellation.

**The interval lane's cost, honestly.** Interval replay costs roughly
4–8× f64 flops plus lost vectorization (measured **pre-M5**, against
the inari `DecInterval` backend; the `interval-transcendentals` backend
that replaced it in M5 PR 1 has not been re-measured on this axis),
and the subdivision driver multiplies whole-model replays by sub-box
count. But it is *never* on the preview path — it is the M10
certification lane and a CI lane — so it is a throughput problem
(parallelize, §2.2), not a latency problem. Stated plainly: **nothing
about interactive latency justifies weakening the trilean
architecture; the f64 lane with K·ε escalation IS the fast path.**

### 1.3 Ranking summary

| Rank | Cost center | Lane | Fix class |
|---|---|---|---|
| 1 | CDT tessellation (quadratic insertion; full-body re-tess) | preview | algorithmic (§2.1) |
| 2 | Boolean edge×face sweep (quadratic) | preview (M3+) | algorithmic (§2.1) |
| 3 | Full-DAG rebuild on any edit | preview | architectural (M4 memoization) |
| 4 | Interval subdivision driver | background (M10) | parallelism (§2.2) |
| 5 | Tier-3 certification/validation | commit | leave alone until profiled |
| 6 | Mass props, exports | background | leave alone |

**[2026-08-14 status of the table above.]** Four of six entries moved.
Read this column with it:

| Rank | Status as of the scan |
|---|---|
| 1 | **Re-scoped.** Real, but export/background lane — not preview, and quadratic only for nested-cocircular (holed planar) faces. |
| 2 | **Retired.** BVH landed at M5 PR 8. Replaced by per-op whole-body pcurve re-mint as the boolean's dominant term. |
| 3 | **Solved, then stranded.** Memoization works (11× on `die`); unreachable from the shipping API. |
| 4 | **Unchanged and still unbuilt** — M10 has not started. |
| 5 | **Wrong.** Tier 3 now runs in release on four paths; tier 1 runs twice per boolean and once per Euler op in debug. |
| 6 | **Unchanged.** Mass props and exports remain cheap; the scan found nothing. |

The scan's own ranking — which supersedes this table for planning
purposes — leads with a **correctness** item this doc could not have
foreseen: `face_box` has no NURBS arm, so the BVH introduced in service
of rank 2 can prune a pair the exact predicate would examine, violating
the very conservative-superset contract §2.1 makes a D9 obligation.
Fixing performance is how that class of bug gets in; see PERF-SCAN
finding 1.

## 2. CPU-first roadmap

The order is a commitment: **algorithms, then architecture, then
parallelism, then micro-optimization** — each item names its target
code and its trigger; the trigger is the license to start.

### 2.1 Algorithmic wins (they dominate; do these first)

- **CDT bulk loading** (`crates/mesh/src/tessellate.rs` insertion
  path). The module docs already name the remedy; `spade` ships
  `bulk_load`. Determinism needs care: the D9 argument in
  `mesh/lib.rs` leans on *fixed insertion order*, so a bulk path must
  be verified input-order-deterministic (and that argument re-pinned
  in the module docs) or fed through a deterministic pre-sort. Effect:
  kills the quadratic term — the 1.2 s washer at δ = 1e-6 becomes
  tens of ms. Trigger: first real fine-δ export need, or the M4
  corpus showing CDT dominance.
- **Incremental re-tessellation** — the architectural sibling. The
  tessellator is already per-face (walk → CDT → certify), and the
  ratified content-keyed cache principle makes the memo key the
  bit-content of the face's geometry: D9 turns "same bits ⇒ same mesh
  patch" into a theorem. An edit that moves one boss re-tessellates
  only changed faces. The biggest preview-lane win, and it is keyed
  work, not speculative — the cache service is already slotted for
  editor-core. Lands with M4's evaluation service.
- **One BVH/spatial-index crate, three consumers.** The quadratic
  boolean sweep (M3-PLAN PR 4), viewport ray-picking (Band 1), and M5
  SSI seeding / M10 clearance all want the same structure: a
  deterministic AABB-BVH built in arena order with fixed splits and
  total tie-breaks — no hash order, no parallel-build nondeterminism
  in v1. (The Banked cache principle already names "BVH node" as a
  content-keyed artifact — the design anticipated this crate.)
  Trigger: M5 curved booleans at the latest; earlier only if the M4
  corpus shows the planar sweep dominating. The
  **conservative-superset contract** is the D9 obligation: a BVH may
  only prune pairs the exact predicate would reject, so the result
  stays a function of exact tests only.

  **[STATUS 2026-08-14 — one consumer extant, two still correctly
  pending; the contract is currently violated.]** This bullet is a
  *plan*, and it is on schedule: §5 sequences the BVH at M5 (done —
  `Bvh::build` is live at `boolean/reduce.rs:420`, the workspace's only
  call site) and picking at the GUI milestone (not started, because
  there is no GUI). SSI seeding remains intended and unwired —
  `geom-brep/src/ssi/exhaust.rs:32-38` still bisects with a linear scan
  over tubes and says why ("Brute force, deliberately, for now ... PR
  8's BVH swaps in ... when profiling asks for it"), which is this
  doc's own trigger discipline working, not a missed delivery. What
  *was* misleading is `crates/bvh/src/lib.rs:3-5`, which described all
  three duties in the present tense; corrected 2026-08-14 to mark which
  are live and which are intended. **The superset contract is broken
  for NURBS faces**: `boolean/boxes.rs:152-178` has no `Surface::Nurbs` arm and
  falls through to a hull of the face's *boundary vertices*, which the
  patch interior bulges past — while the sibling `edge_box` correctly
  poisons its NURBS arm and `gate_planar` admits NURBS operands. The
  sound constructor (`geom-surfaces/src/boxes.rs:26`) exists with zero
  production callers, and the differential suite meant to guarantee the
  contract builds every scenario from planar bricks. PERF-SCAN
  finding 1.
- **Feature-DAG memoization (M4, ratified Band 1).** Not re-argued;
  noted because it converts rank 3 from linear-in-model to
  linear-in-edited-cone — the demo/product difference. D9 makes the
  memo keys sound.

### 2.2 Parallelism under D9 (rayon, with the determinism discipline stated)

D9 permits "parallelism only in fixed reduction shapes". Concretely,
two allowed idioms — worth ratifying as the project's parallelism
vocabulary so every future use cites them instead of re-deriving:

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

Targets, in value order: the **M10 subdivision driver** (Q1's
"embarrassingly parallel" is literally idiom 1); **per-face
tessellation** (faces independent; mesh vertex minting switches from
a running counter to per-face offset ranges via a sequential prefix
pass); **certification sampling** (per-edge, idiom 1); **mass
properties** (per-face fluxes, arena-order sum — the canonical
idiom-2 example); **independent DAG nodes** in M4's evaluation
service. Euler-op sequences stay serial — each op mutates shared
arenas, and they are cheap; rank 3 is solved by memoization, not by
parallelizing surgery.

**[STATUS 2026-08-14 — one of five targets landed, and it is off.]**
`rayon` is a dependency of `editor-core` alone, and
`editor-core/src/eval/mod.rs:855` is the **only `par_iter` in the
workspace**. Of the five targets above, only "independent DAG nodes"
was built; it is D9-clean as written (indexed map into per-node slots)
but `EvalOptions::default()` sets `parallel: false` and every shipping
caller takes the default — `parallel: true` appears once, in a test.
Tempering expectation for whoever turns it on: the scheduler is
level-synchronous and the expensive corpus documents are *chains*
(`heat_sink` is a 5-long union chain, `die` a 21-long subtract chain),
which are depth-N and width-1, so it will not move those rows.

Of the four unbuilt targets, **per-face tessellation is the cheapest**
and the blocker is small: `mesh/src/tessellate.rs:81-136` threads a
`&mut positions` running counter through the face loop, and each lane
mints grid ids as `positions.len()`. Everything else is already
read-only per face. Emitting *local* ids into a pre-sized buffer and
assigning base offsets in a sequential arena-order fold is exactly the
idiom-1-then-idiom-2 shape above, and is bit-identical. PERF-SCAN §5.

### 2.3 Micro level (profile-gated; mostly "not yet")

- Cheap now, **landed and measured** (#52/#54, 2026-07-21):
  `[profile.dev.package]` opt-level 2 for `spade` + `mesh` is in main.
  Two measured lessons narrowed the original recommendation: (i)
  blanket opt-2 via CI profile env is net-SLOWER on core-crate PRs —
  the per-push recompile of the changed crate graph plus all test
  binaries costs more than the test time saved (#52, reverted); (ii)
  per-package lib overrides capture only part of the win (91.7s →
  37.4s on the worst test, not the whole-graph 15x) because generic
  `T: Real` hot code monomorphizes into the CALLING crate's binaries,
  which lib-level overrides can't reach — so overrides stay confined
  to rarely-edited packages, and the full whole-graph speedup lives in
  `local-scripts/test-fast.sh` as an opt-in local recipe where warm caches
  absorb the build cost. The same monomorphization fact will apply to
  any future "optimize the hot dep" plan: measure at the binary that
  instantiates the generics, not the crate that defines them.
- Later, evidence-gated: SoA layouts for batch predicate/cert
  sampling; LTO/PGO on release; SIMD in BVH traversal. All premature
  before the criterion harness (§5) exists to show a win.
- Never: fast-math flags, FMA contraction, or per-platform intrinsics
  in kernel code — D9 and the Q1 no-fused-ops rule (`Real` has no
  `mul_add`) already ban them; stated here so "optimization" never
  reintroduces them by reflex.

## 3. GPU acceleration, honestly scoped

### 3.1 Where GPU genuinely pays

- **Rendering and picking — the big one, and it is GUI-side.**
  Effectively decided: GUI-DESIGN.md commits to wgpu regardless of
  framework and ratifies GPU ID-buffer picking + CPU ray-cast
  confirm. Viewport LOD, silhouettes, section views live here. The
  kernel's whole obligation is what M2 PR 6 built: meshes with stable
  back-references. No kernel changes; the GUI milestone owns it.
- **Preview-grade tessellation — plausible, display-lane only.** A
  compute-shader evaluator for analytic surface grids could produce
  *uncertified preview* meshes for drag feedback — exactly parallel
  to the ratified "preview may march uncertified" SSI stance: a
  degraded lane that never feeds the kernel. Certified-δ meshes
  (`mesh/cert.rs`, the export promise) stay CPU. Honest caveat: CDT
  *topology* (the actual bottleneck, §1.2) does not GPU-parallelize —
  GPU buys vertex evaluation/refinement of existing patch topology.
  Verdict: a GUI-milestone experiment, not a kernel commitment.
- **Batch f64 value evaluation (M10 Monte Carlo) — marginal.** Each
  sample is a full model *rebuild* (topology surgery, CPU-shaped),
  not a bare function evaluation; rayon over samples is the right
  tool. GPU would accelerate the cheap part.

### 3.2 Batch certified predicates on GPU: assessed and tabled

The tempting idea — evaluate thousands of interval predicates (M10
clearance, SSI exhaustiveness) on GPU — fails today on three
independent grounds, each disqualifying:

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

Conclusion: **certified/trilean predicates on GPU are research-grade;
tabled** (revisit post-M10 at the earliest, alongside the Tabled
in-house interval transcendentals — same "rigorous numerics we fully
control" prerequisite). Not a loss: the interval lane's workloads are
throughput-shaped and rayon-parallel (§2.2).

### 3.3 The boundary, recommended for ratification

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

## 4. The idealized/realized dual-code question (Evan's proposal)

The proposal: an **idealized** implementation that nails down end
behavior (maximally readable — the code *is* the definition) and a
**realized** implementation that runs fast, pinned to the idealized
one by tests — possibly a debug build running BOTH and asserting
identical outputs. Assessment: **adopt — selectively, per hot kernel,
with CI differential testing as the standing pin and shadow execution
as an opt-in scalpel, not a build mode.**

### 4.1 Prior art (this is a proven pattern, not an invention)

- **Crypto reference implementations**: every serious crypto library
  ships a slow, obviously-correct reference per primitive, pinned to
  the optimized (asm/SIMD/bitsliced) form by exhaustive vectors and
  differential fuzzing; fiat-crypto *generates* the realized form
  from the idealized one with proof. The structural precondition is
  ours too: pure functions with bit-exact expected outputs.
- **Differential / back-to-back testing** (McKeeman; csmith;
  DO-178-style avionics): two independently derived implementations
  disagreeing is a cheap, high-yield bug oracle. Known limit
  (Knight–Leveson): independence is partial — two versions written
  from one misreading agree on the same wrong answer. The pin catches
  *divergence*, not shared spec error; adversarial review remains the
  defense for the latter.
- **Refinement stacks** (CompCert, seL4): the same split with a
  machine-checked simulation proof instead of tests — the gold
  standard, beyond our budget; test-pinned refinement is its
  engineering-grade approximation.
- **This repo already does it**, four ways: `review_m*` suites are
  independently-derived behavioral pins (differential testing where
  the second implementation is a test suite); `num-dual` is a
  demoted *dev-only derivative oracle* for `Dual<T>`; opencascade-rs
  is banked as the M3 boolean oracle; M3-PLAN pins
  `A∖B ≡ A∩revert(B)` as an executable cross-check. The proposal
  generalizes a house pattern.

### 4.2 Why this kernel is unusually well suited

- **D9 makes "identical" a real oracle.** Ordinary numerics drowns
  ref-vs-fast comparison in tolerance fudge ("agree to 1e-12 —
  usually"). Here both versions must produce **bit-identical
  outputs** (same reduction shapes, libm, no FMA): the pin is
  `assert_eq!` on bytes — zero false-pass headroom; a divergence is a
  definite bug, never noise. The machinery is ready-made:
  `topo::iso::canonical_form`, lineage-scoped key identity for
  arena diffs, and the multi-ε CI matrix + interval lane multiplying
  every differential corpus for free.
- **Purity makes replay cheap.** Models are values; the recipe is
  data (D8). A differential corpus is a directory of recipes replayed
  through both implementations — no mocking, no setup; the Band 4
  real-model corpus (online at M4) doubles as the differential corpus
  at zero marginal authoring cost.

### 4.3 The costs, stated without discount

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
- **Correlated error** (Knight–Leveson): the pin cannot catch a
  shared misunderstanding. Existing answer stands: the idealized
  version is exactly what the adversarial `review_*` process is best
  at attacking — readable code is auditable code.

### 4.4 Where it pays, and where the single version IS the realized one

**Adopt (hot kernels whose fast form stops being self-evident):**

- **BVH build + traversal** (§2.1 — the pilot). Idealized:
  brute-force all-pairs — ten readable lines that *define* the
  candidate set. Realized: SAH build, flattened nodes, stackless
  traversal. Pin: realized candidates ⊇ idealized-exact pairs
  (the conservative-superset contract made executable) and final
  results bit-equal through either path.
- **Tessellation insertion path** when bulk-loading lands: the
  current sequential-insertion CDT (trusted, documented) becomes the
  idealized reference; `bulk_load` the realized path; pin =
  byte-identical `Mesh` (the D9 mesh contract gives this meaning).
- **Batch predicate/certification evaluation** if SoA/SIMD ever
  lands: the scalar per-edge loop stays as the definition.
- **M5 SSI marching stepper** (flagged early): exactly the "tricky
  optimized numerics" shape, and its idealized form doubles as the
  spec the exhaustiveness contract audits.

**Do not adopt (the readable version IS the realized one):** Euler
operators (O(1) surgery, never hot; the code is the definition —
DESIGN.md's own thesis), validators (they *are* the executable spec;
a dual would pin the spec to itself), mass properties (closed forms),
profile canonicalization, STL writers. Default for new code: **single
version until the optimization diff stops being reviewable**; the
dual structure is earned by a measured win, never speculative.

### 4.5 Verdict on shadow execution (run both in debug, assert equal)

Feasible — purity makes it a five-line wrapper — but **wrong as a
standing build mode**, for two reasons: (a) the asymptotic gap is the
point — on exactly the inputs where the realized BVH matters, the
idealized O(n²) shadow is unusable, so always-on shadowing forces
tiny models and samples the least interesting region; (b) the debug
lane already carries per-op tier-1 asserts; doubling it taxes every
developer run to re-check what CI checks better. Recommended shape:

- **CI differential suites** (the 95%-for-10% answer — endorsed):
  proptest-generated + pinned corpora through both versions,
  byte-equality asserts, all ε rows + interval lane. The standing pin.
- **A `shadow-exec` cargo feature** per dual module: opt-in wrapper
  asserting bit-equality, for hunting a divergence a real model
  exhibited. A scalpel, never default-on.
- **Nightly corpus differential** once Band 4's corpus exists —
  catches the distribution-shift bugs proptest's generators miss,
  without taxing interactive dev.

## 5. Sequencing

*(Historical record — written during M3. Every milestone entry below
except M10's has since been delivered as described: item 2 and item 3
are DONE, M4's window and M5's BVH/SSI entries shipped (#135 etc.).
The one still-undelivered item is the Criterion harness (item 1),
which remains deferrable by its own terms — no `benches/` exists,
and the trend only has to predate the first optimization PR that
needs it.)*

**[CORRECTION 2026-08-14.]** Two things in that parenthetical need
fixing, and the second one now blocks this whole section.

First, **M5's SSI entry did not ship as described.** The BVH landed for
the boolean sweep; SSI seeding still uses hand-rolled bisection
(`ssi/exhaust.rs:32-38`). And CDT bulk-loading — "if the corpus shows
CDT dominance (likely)" — did not land either.

Second, **"deferrable by its own terms" has stopped being true.** The
harness is not merely undelivered; its absence is now the binding
constraint on everything else here, for three compounding reasons:

- §2.3 gates every micro-optimization on it ("All premature before the
  criterion harness (§5) exists to show a win"), so the doc forbids the
  work until the measurement exists, and the measurement is the one
  item nobody built. That is a deadlock, not a deferral.
- The M5/M10 triggers this doc sets for itself ("if the corpus shows
  CDT dominance", "if certification wall-times demand it") are
  **unfireable**, because the only corpus timing artifact —
  `crates/editor-core/tests/baseline/rebuild-latency.json` as it stood
  before the 2026-08-17 split described below — largely
  disqualifies itself in its own provenance: three refreshes disagree
  by 90–98% with contention ruled out, the `die_composed` row is
  explicitly "not a datum", and the whole file is a dev-profile
  (opt-level 0) measurement. Only the full-vs-incremental *ratio*
  within one verified-quiet run is trustworthy.
- Consequently the 2026-08-14 scan could profile almost nothing: of its
  ~20 findings, exactly one carries measured numbers, and that one
  produced the report's largest result (35×) precisely because its
  author built a harness. That is this item's argument in miniature.

**So item 1 is now the first thing to do, not the last.** The second
half of that ask — capturing the environment rather than hypothesizing
it — landed 2026-08-17: hosted CI is the canonical producer of the
rebuild-latency numbers, `docs/perf-data/rebuild-latency/` accumulates
one entry per merge to `main`, and every entry carries the build
environment (runner, nproc, memory, toolchain, RUSTFLAGS,
`CARGO_PROFILE_*`, debug-assertions, ε). That does not un-disqualify
the three historical workstation refreshes, and it is not the criterion
harness item 1 asks for — it is a reporting lane, still ungated — but
the corpus timings are comparable across merges now, which is the
precondition the M5/M10 triggers above were missing. Until item 1
itself lands,
`docs/k-report-data/`'s predicate decision counts are the better
evidence base: exact, deterministic, machine-independent, and immune to
both the profile and contention problems (1 792 926 decisions in the
1e-9 row alone; it is what localized PERF-SCAN findings 8 and the SSI
cold-path result).

**Now (during M3) — three cheap things, nothing else:**

1. **Criterion benchmark harness — post-merge only, never a PR gate**
   (Evan, #49 review): runs on pushes to main (and optionally
   path-filtered to fire only when perf-relevant crates changed), so
   no PR waits on it and no shared-runner wall-clock flake can block
   a merge; regressions are read off the archived per-commit trend,
   not a threshold. Five scenarios: washer tessellation at
   δ ∈ {1e-4, 1e-6} (re-pins the module-doc numbers), tier-2+3
   validation of a revolved body, mass props, extrude build, and —
   once M3 PR 5 lands — the two-brick boolean. Adding it can wait
   until an optimization PR actually needs the baseline; the trend
   only has to exist before the first change it would police.
2. **Dev-profile opt-level for hot deps** — DONE, narrowed by
   measurement to `spade` + `mesh` plus the local `test-fast.sh`
   recipe (§2.3).
3. **Ratify §2.2's parallelism idioms into DESIGN.md (D9)** — a
   paragraph, so the first rayon PR cites vocabulary instead of
   re-litigating determinism.

**M4 (feature DAG)** — the architectural window: DAG memoization +
content-keyed caches (ratified Band 1; this doc adds only urgency);
the Band 4 corpus with rebuild-latency tracking; first rayon
deployment (independent DAG nodes + per-face tessellation, idiom 1);
incremental re-tessellation via the cache service.

**M5 (NURBS/SSI)** — the BVH crate lands (boolean sweep, SSI seeding,
picking) as the **idealized/realized pilot** with its CI differential
suite; CDT bulk-loading if the corpus shows CDT dominance (likely);
SSI marching written with its idealized stepper from day one.

**M10** — parallel subdivision driver (idiom 1 over sub-boxes);
interval-lane throughput work if certification wall-times demand it.

**GUI milestone owns** everything in §3.1: wgpu rendering, ID-buffer
picking, LOD, the preview-tessellation experiment. The kernel's
deliverables to it are already scoped (meshes with back-refs,
cancelable evaluation service, BVH).

**Premature now, named to stay dead until their triggers:** any GPU
work; SIMD/SoA/PGO/LTO; parallelizing Euler sequences; replacing the
M3 quadratic sweep before correctness ships (M3-PLAN's own call);
benchmark gates; micro-tuning validators or mass props.

## 6. Open questions for Evan

- **Q-P1** — ANSWERED (Evan's sign-off, #49, 2026-07-21; executed at
  the M3 exit sweep): the §3.3 GPU boundary table and §2.2
  parallelism idioms are ratified into DESIGN.md as the D9 addendum;
  this doc stays merged-and-advisory.
- **Q-P2** — ANSWERED (Evan, #49, 2026-07-21): as recommended —
  selective adoption, CI differential pin, `shadow-exec` opt-in
  ("exactly the kind of thing I was thinking of"); no always-on
  debug shadow.
- **Q-P3** — ANSWERED (Evan, #49, 2026-07-21): degraded previews are
  acceptable.
- **Q-P4** — ANSWERED (Evan, #49, 2026-07-21): no pre-merge gate.
  The harness runs post-merge on main (optionally path-filtered to
  perf-relevant crates), and adding it is deferred until an
  optimization PR needs the baseline — the trend must merely predate
  the first change it would police.
