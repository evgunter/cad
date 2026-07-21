# PERF-PLAN — Performance & GPU Roadmap; the Idealized/Realized Dual-Code Question

**Status: PROPOSED (design conversation — awaits Evan's sign-off; not
ratified).** Companion to `DESIGN.md` (never overrides D1–D9) and
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
booleans/splitting (M3: the edge×face sweep, `docs/M3-PLAN.md` PR 4 —
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
2. **Booleans/splitting (M3 on)** — the quadratic edge×face sweep
   plus per-pair trilean classification; every boolean node
   downstream of the edit re-runs it.
3. **Feature rebuild** — Euler-op sequences are O(entities built)
   with small constants (`crates/topo/src/euler*.rs`); cheap per
   feature, but linear in downstream-DAG size without M4 memoization.
4. **Validation + certification** — tier 1 is per-op debug-assert
   (absent in release); tier 3 samples per-edge dihedrals and 9-point
   residual schedules (`CERT_SAMPLES`, `geom-brep/src/certify.rs`);
   the 12-pass tier-1/2 pipeline (`topo/src/validate.rs`) is linear
   arena passes. Not the bottleneck; do not optimize.

**Per-commit (~1 s budget).** Full tier-2/3 validation, certified-δ
tessellation, mass properties. Mass props are divergence-theorem
closed forms per face (`crates/geom-brep/src/props/` — `planar_face`,
`curved_face`, no quadrature): O(faces), microseconds-to-milliseconds;
background-eligible but cheap enough not to bother.

**Background.** STL/STEP export (`crates/stl/` is a trivial linear
writer; f32 narrowing documented), K-telemetry (`geom_core::k_stats`),
the M6 interval subdivision driver (Q1 calls it embarrassingly
parallel — correctly, see §2.2), fine-δ export tessellation.

**The interval lane's cost, honestly.** Interval replay (inari
`DecInterval`) costs roughly 4–8× f64 flops plus lost vectorization,
and the subdivision driver multiplies whole-model replays by sub-box
count. But it is *never* on the preview path — it is the M6
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
| 4 | Interval subdivision driver | background (M6) | parallelism (§2.2) |
| 5 | Tier-3 certification/validation | commit | leave alone until profiled |
| 6 | Mass props, exports | background | leave alone |

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
  SSI seeding / M6 clearance all want the same structure: a
  deterministic AABB-BVH built in arena order with fixed splits and
  total tie-breaks — no hash order, no parallel-build nondeterminism
  in v1. (The Banked cache principle already names "BVH node" as a
  content-keyed artifact — the design anticipated this crate.)
  Trigger: M5 curved booleans at the latest; earlier only if the M4
  corpus shows the planar sweep dominating. The
  **conservative-superset contract** is the D9 obligation: a BVH may
  only prune pairs the exact predicate would reject, so the result
  stays a function of exact tests only.
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

Targets, in value order: the **M6 subdivision driver** (Q1's
"embarrassingly parallel" is literally idiom 1); **per-face
tessellation** (faces independent; mesh vertex minting switches from
a running counter to per-face offset ranges via a sequential prefix
pass); **certification sampling** (per-edge, idiom 1); **mass
properties** (per-face fluxes, arena-order sum — the canonical
idiom-2 example); **independent DAG nodes** in M4's evaluation
service. Euler-op sequences stay serial — each op mutates shared
arenas, and they are cheap; rank 3 is solved by memoization, not by
parallelizing surgery.

### 2.3 Micro level (profile-gated; mostly "not yet")

- Cheap now: `[profile.dev.package.spade] opt-level = 2` (and libm,
  inari) — dev-lane tessellation and interval tests get most of
  release speed while keeping our own crates debuggable. Near-zero
  cost, real dev-loop win.
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
- **Batch f64 value evaluation (M6 Monte Carlo) — marginal.** Each
  sample is a full model *rebuild* (topology surgery, CPU-shaped),
  not a bare function evaluation; rayon over samples is the right
  tool. GPU would accelerate the cheap part.

### 3.2 Batch certified predicates on GPU: assessed and tabled

The tempting idea — evaluate thousands of interval predicates (M6
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
tabled** (revisit post-M7 at the earliest, alongside the Tabled
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

**Now (during M3) — three cheap things, nothing else:**

1. **Criterion benchmark harness in CI as a trend line, not a gate**
   (wall-clock gates on shared runners manufacture flaky tests).
   Five scenarios: washer tessellation at δ ∈ {1e-4, 1e-6} (re-pins
   the module-doc numbers), tier-2+3 validation of a revolved body,
   mass props, extrude build, and — once M3 PR 5 lands — the
   two-brick boolean. Archived per commit; regressions read off the
   trend, not a threshold.
2. **Dev-profile opt-level for hot deps** (`spade`, inari's backends,
   `libm`): one Cargo.toml stanza, immediate dev-loop relief on
   tessellation-heavy tests.
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

**M6** — parallel subdivision driver (idiom 1 over sub-boxes);
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

- **Q-P1**: Ratify the §3.3 GPU boundary table and §2.2 parallelism
  idioms into DESIGN.md (D9 addendum), or keep this doc advisory?
- **Q-P2**: Idealized/realized as recommended (§4: selective, CI
  differential pin, `shadow-exec` opt-in) — or do you want the
  stronger form (shadow always-on in debug) despite §4.5's argument?
- **Q-P3**: Preview lane semantics: is an *uncertified* degraded
  preview (GPU or coarse-δ CPU) acceptable product behavior, given
  fail-loud? (Precedent says yes — the ratified SSI preview stance —
  but rendering a not-yet-validated body during a drag deserves an
  explicit yes.)
- **Q-P4**: Benchmark trend line in CI now (during M3) vs. deferring
  the harness to M4's corpus work. Recommendation: now — it is an
  afternoon, and M3's booleans are the first workload worth watching
  from birth.
