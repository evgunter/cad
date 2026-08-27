# PERF-SCAN — a measured audit of performance-critical code (2026-08-14)

**Status: REPORT ONLY. Nothing here is ratified, nothing is a
commitment, and no code was changed.** This is a survey, requested by
Evan, of where this kernel's time actually goes — read against
`docs/PERF-PLAN.md`, whose ranking was written at M3-start against the
M2 codebase and is now three milestones stale in specific, checkable
ways (§6). Where this report contradicts PERF-PLAN, PERF-PLAN is the
older document, not the wrong one by fiat; §6 says which claims expired
and why.

Method: six parallel domain scans (tessellation, booleans/BVH,
evaluation/memoization, topo, sweep/geometry, CI/build), each required
to anchor every claim at `file:line`, argue cost by complexity class or
call count rather than intuition, and report negative results. Every
headline finding below was then re-verified by hand against the source.
Findings marked **[verified]** were checked a second time by the
coordinator; the rest carry the scanning agent's stated confidence.

---

## 0a. Re-verification against main (2026-08-16)

The scan ran against `870c7a9`. Main has moved 326 commits since, so
every headline claim was re-checked after merging. **Line numbers below
are as of the scan base unless a finding says otherwise; the claims, not
the line numbers, are what was re-verified.** What changed:

- **One item the scan under-read has been fixed upstream, and the
  upstream measurement is a calibration warning.** The tessellation
  scan flagged `probe_stats::armed()` being evaluated per emitted
  triangle with a non-short-circuiting `env::var_os("NURBS_PROBE")`
  lookup in the disarmed case, sized it at ~158 ns/call, and I ranked it
  too low to make §1 at all. #562 (`feb60e7`) has since removed the back
  channel, and measured it at **7.9 s → 19.8 s on the demo tour's
  release binary — same binary, same arguments, ~71M extra surface
  evaluations** — while noting it also converted `tessellate()`'s typed
  `TessellateError` contract into a **panic** selected by the ambient
  environment. It was a correctness bug as much as a performance one,
  and this scan mis-sized it on both axes. `armed()` is now
  `ARMED.with(Cell::get)`. Residual, tracked upstream as issue #558: the
  module is still `pub` and unconditionally compiled; the standing rule
  is the `discipline` job's "no ambient environment in the kernel" grep.
- **Everything else still holds**, re-checked in the merged tree:
  finding 1 (`boxes.rs` untouched by main, still no `Nurbs` arm),
  finding 2 (still no `benches/`, still no `criterion`), finding 3
  (opt-2 still on `build-interval`, `interval-only-selection.py` still
  present), finding 4 (`gate` still double-validates,
  `boolean/ops.rs:1263-1265`), finding 7 (`mint_pcurves` still clears
  the whole body, `pcurves.rs:934`), finding 13 (`join.rs:524` /
  `546` / `583` / `774` / `822`), and the parallelism claim (still
  exactly one `par_iter` in the workspace).
- **§2's verdict defect still reproduces on merged main** — the probe
  was re-run after the merge and returns the same `0` against `722`,
  despite `eval/mod.rs` gaining 205 lines. The bracket is now at
  `eval/mod.rs:1160` / `:1170`.
- **The census grew** (open PR #564): `census_and_certify` now admits
  every carrier kind and adds an O(curved faces²) conformal-patch
  sweep. §5's negative result still holds — it is still reachable only
  from `validate_pseudomanifold` (`validate.rs:2335`), which is not on
  the rebuild path — but the cost behind that door is now larger.

**Open PRs, checked for overlap.** Neither blocks this report.

- **#571 (PlacedUnion)** touches `boolean/boxes.rs`, but only to widen
  `sweep_pad` / `face_box` / `edge_box` from `pub(super)` to
  `pub(crate)` for a new `topo::separation` module. It does not fix the
  `Nurbs` gap. It does, however, **independently corroborate finding
  1**: `separation.rs`'s own `certified_face_box` deliberately routes
  `Cone`, `Torus` and `Nurbs` to the poison box, and its rationale says
  the distinction matters because `face_box` "is written for the
  boolean sweep, whose operand gate has already narrowed the surface
  kinds it can meet. This door has no such gate." That premise is
  exactly the stale claim finding 1 disproves — `gate_planar` admits
  `Surface::Nurbs`. #571's own door is safe; the belief it records
  about `face_box` is not. Expect a small textual conflict in the
  `face_box` doc comment, which this branch rewrites.
- **#564 (M9-2 census door)** touches `census.rs`, `validate.rs`,
  `props.rs`. No finding depends on the lines it moves; see the census
  note above.

## 0. Read this before the rankings: the measurement problem

**This repo cannot currently demonstrate a speedup.** That is the
single most important finding, and it gates everything else.

**There is no benchmark harness.** No `benches/` directory exists
anywhere in the workspace; `criterion` appears in no manifest.
PERF-PLAN §5 item 1 specifies a post-merge Criterion harness with five
named scenarios and says "the trend only has to exist before the first
change it would police." It is still undelivered — while PERF-PLAN §2.3
simultaneously gates every micro-optimization on it ("All premature
before the criterion harness (§5) exists to show a win"). The result is
a deadlock: the doc forbids the work until the measurement exists, and
the measurement is the one item nobody has built. **[verified]**

**The one committed runtime dataset largely disqualifies itself.**
`crates/editor-core/tests/baseline/rebuild-latency.json` is the only
per-document timing in the repo, and its own `provenance` block records:

- **Cross-refresh comparisons are meaningless.** Three refreshes
  disagree by 90–98% on every row (`die` full: 51661.1 ms → ~985.9 ms →
  1103.5 ms). Contention was *ruled out* by a verified-quiet re-run,
  leaving an untested build/environment hypothesis (RUSTFLAGS /
  `CARGO_PROFILE_*` / target dir / toolchain). Nobody has captured the
  two environments side by side.
- **The `die_composed` row is explicitly not a datum** — measured in a
  contended run, "NOT comparable with the rows around them ... they
  exist so the coverage row has its key". Its 2450.6 ms is
  uninterpretable in absolute terms.
- **Not release-representative** — dev profile, `opt-level = 0` for
  every kernel crate, `opt-level = 2` only for `spade` and `mesh`.
- **Box-relative by design** — only the *shape* (relative cost per
  document, and the full-vs-incremental ratio) is claimed to transfer.

So the only trustworthy runtime signal in the repo today is the
full/incremental **ratio** within the single verified-quiet run. Every
absolute-millisecond claim in this report is provisional and labelled.

> **Update, 2026-08-17 — the producer moved; this section's findings
> stand as written.** The single committed baseline described above no
> longer exists in that form. It was split along the rot line: the
> machine-independent half (`about` / `nodes` / `cone`) stays in
> `crates/editor-core/tests/baseline/rebuild-latency.json` as an
> asserted structural manifest, and the timings moved to
> `docs/perf-data/rebuild-latency/` — one append-only entry per merge
> to `main`, produced and committed by ci.yml's `rebuild latency
> (reporting)` job on a hosted runner, each carrying its own
> `environment` block (runner, nproc, memory, toolchain, RUSTFLAGS,
> `CARGO_PROFILE_*`, debug-assertions, ε).
>
> What this does and does not fix. It does **not** resolve
> `disputed_measurement` retroactively — those three workstation
> refreshes remain mutually incomparable, and every absolute-
> millisecond claim in this report stays provisional. It makes the
> failure unable to recur: one reproducible box class produces the
> numbers, every entry records the environment the argument went
> unresolved for want of, and the history accumulates instead of being
> overwritten, so drift is recoverable rather than laundered. The
> `die_composed` caveat is superseded by the first hosted entry —
> re-measured on the same box as its neighbours, it is a datum again.
>
> The measurements are still **REPORTING ONLY**, still dev-profile, and
> still never gated. What changed is that they are now comparable.

**What partially rescues the situation:** `docs/k-report-data/` holds
per-document *predicate decision counts* — 1 792 926 recorded decisions
across 15 corpus documents and 19 demo scenes in the 1e-9 row alone —
which are exact, deterministic, machine-independent, and completely
immune to the profile and contention problems above. Cross-referencing decision counts against wall time is
how several findings below were localized, and it is a far better
foundation than the latency JSON. Notably it shows `die_composed` at
9 977 decisions / 2450 ms (245 µs/decision) against 6–12 µs/decision for
the three genuinely boolean-heavy documents — independent confirmation
that its row is an artifact, not a hot spot.

**Perf is also unscheduled work.** `docs/M8-PLAN.md` and `M8-LOG.md`
contain no performance items. PERF-PLAN's M5-window triggers (the BVH
pilot; CDT bulk-load "if the corpus shows CDT dominance") were
conditional on a corpus signal the disputed baseline cannot supply — so
the triggers the doc set for itself are unfireable until the measurement
problem is fixed. That is the structural reason the ranked list below has
gone unactioned; it is not neglect.

**Recommendation R0 (do before any optimization PR):** land the
Criterion harness PERF-PLAN §5 already specifies, and re-refresh the
latency baseline from hosted CI so the environment is captured rather
than hypothesized. Until then, prefer the decision-count corpus as the
primary evidence base. Effort M. D9-safe (measurement only).

---

## 1. Ranked findings

Ranked by (expected payoff × confidence) ÷ effort, with correctness
issues promoted above pure performance regardless of payoff.

**Not in this table: §2, the verdict-log back channel.** It is a
structural-safety obligation rather than a performance finding, and it
carries an observed correctness defect (every `InstantiatePart` node's
verdict log is empty). Read it alongside finding 6, which is the change
most likely to break the invariant it depends on.

| # | Finding | Class | Effort | Conf. |
|---|---|---|---|---|
| 1 | BVH `face_box` has no NURBS arm — conservative-superset violation | **correctness** | S | high |
| 2 | No benchmark harness; latency baseline self-disqualifying | blocker | M | high |
| 3 | CI: `opt-level=2` on the interval build lost its justification | CI wall | S | high |
| 4 | Boolean gate runs tier-1 validation twice, in release | runtime | XS | high |
| 5 | Full-body validate on every Euler op (debug) — Θ(ops × N) | CI/dev | S | high |
| 6 | Memoization and parallel eval unreachable from the shipping API | runtime | S | high |
| 7 | `mint_pcurves` re-certifies the whole body, per operation | runtime | M | high |
| 7b | CDT: `bulk_load_cdt` is a **measured 35×** on holed planar faces | export/CI | M | high |
| 8 | `point_in_loop` re-decides loop-intrinsic facts per query | runtime | M | high |
| 9 | Kill-direction Euler ops are O(arena) via orphan scans | runtime | M | high |
| 10 | CI: shard the default archive build across runners | CI wall | M | high |
| 11 | `merge_group` rescans the edge arena after every kill | runtime | M | high |
| 12 | ~~Fillet `Plan::assemble` is ~cubic in blended edges~~ | — | — | **RETIRED** — its subject was deleted |
| 13 | Boolean `join` is O(n³) with hoistable invariants | runtime | M | high |
| 14 | `graft_solid` is O(E²) — missing inverse map | runtime | S | high |
| 15 | `StableName` nests one `Box` per boolean → O(chain²) | runtime | M/L | high |
| 16 | Tier-3 validation runs twice on the product/export path | runtime | S | med |
| 17 | `geom-core` has zero `#[inline]` attributes | CI/dev | S | med |
| 18 | Per-call constant waste (clones, allocs in scans) | runtime | S | high |

### Tier A — do first

#### 1. `face_box` has no NURBS arm: a latent conservative-superset violation **[verified]**

`crates/topo/src/boolean/boxes.rs:152-178`. `face_box` special-cases
`Cylinder` and `Sphere`, then falls through to
`Aabb::from_points(<boundary vertex points>).padded(pad)`. A NURBS
patch's interior bulges past the convex hull of its *boundary vertices*
— that is the defining property of the control net. So
`Bvh::overlapping` can prune an edge×face pair that the exact predicate
would have examined.

Three facts make this a live hole rather than a hypothetical:

- **NURBS faces are admitted.** `crates/topo/src/boolean/reduce.rs:159-164`
  (`gate_planar`) explicitly lists `Surface::Nurbs(_)` among accepted
  operands. `face_box`'s own doc-comment claims curved kinds "fall
  through to the vertex hull only if they reach here at all (the
  operand gate refuses them first)" — **that comment is stale.**
- **The sibling function gets it right.** `boxes.rs:222` —
  `Some(geom_curves::Curve3::Nurbs(_)) => return Ok(Aabb::poison())`.
  `edge_box` is defensive; `face_box` is not. The asymmetry looks like
  an oversight, not a decision.
- **The sound constructor already exists and is unused in production.**
  `crates/geom-surfaces/src/boxes.rs:26` `nurbs_surface_aabb()` returns
  the control-net hull (sound by the convex-hull property with positive
  weights). Its only references workspace-wide are in
  `crates/geom-surfaces/tests/boxes.rs`.

**Why this outranks every performance item:** the pruned pair would
have reached `curved_face_arm`'s *typed refusal* at `reduce.rs:700-706`.
Pruning therefore converts a fail-loud refusal into a silently wrong
boolean result — a D4/fail-loud violation. Performance work that
narrows a candidate set is exactly where this class of bug lives, which
is why PERF-PLAN §2.1 made the conservative-superset contract an
explicit D9 obligation.

**The differential pin does not cover it.**
`crates/topo/tests/m5_pr8_bvh_diff.rs` builds every scenario from
`brick()` (`:30`) — all axis-aligned planar boxes. The `Cylinder`,
`Sphere` and NURBS arms of `face_box` are entirely unpinned by the test
whose stated job is guaranteeing the superset contract.

**Fix:** add `Surface::Nurbs(payload) => nurbs_surface_aabb(payload).padded(pad)`
to `face_box`, and add curved scenarios to `m5_pr8_bvh_diff.rs`.
D9-safe (strictly widens the candidate set). Effort S.

**Reachability caveat, stated honestly:** no corpus document or test
currently unions a lofted (NURBS-walled) body, so this is a latent hole
rather than an observed miscomputation. `crates/sweep/src/loft.rs:480`
mints `Surface::Nurbs`, and `ops.rs:386` refuses non-planar operands
only for Subtract and Intersect — so a **Union** with a lofted operand
is the reachable path.

**Related, lower confidence:** `boxes.rs:100-102` builds a cylinder
face's axial extent from `axis.{x,y,z}.lo()` — the lower bracket of each
interval component, not an enclosure of the projection. At-rest axis
intervals are ulp-wide so this is very likely harmless, but it is not
argued anywhere and no cylinder scenario exists in the differential
suite to catch it. Worth a note in the derivation or an interval-lane
fixture.

#### 2. The measurement problem

See §0. Effort M.

### Tier B — verified, cheap, high-leverage

#### 3. CI: `opt-level = 2` on the interval build outlived its justification **[verified]**

`.github/workflows/ci.yml:652-669` (scan base `:563-574`). PR #449 put
`CARGO_PROFILE_{DEV,TEST}_OPT_LEVEL: "2"` on both archive jobs on
2026-08-12, justifying it for the interval job verbatim: *"This job is
the run's critical path (its archive feeds the longest test leg in the
run) ... the interval lane's slowest leg was 828 s of pure execution,
and opt-2 is aimed squarely at that."*

**That leg no longer exists.** On 2026-08-13 — the next day — the
interval-only-selection change (`ci.yml:787-862`,
`scripts/interval-only-selection.py`) cut the interval legs to the 214
tests the feature actually adds. Measured on run 31776906935, the four
interval legs execute **3 s, 2 s, 3 s and 2 s — 10 seconds total** —
behind a **534 s** build that exists solely to produce them.

Measured breakdown of that 534 s step: rust-cache restore 16 s, crates.io
index 1.4 s, **third-party dependency compiles 0 s** (rust-cache is
hitting perfectly — not one non-path `Compiling` line), 17 workspace libs
79 s, **test-binary codegen+link 451.5 s (84.5%)**, and the nextest
*archive* itself **0.54 s**.

| lane | archive step opt-0 → opt-2 | execution it buys back |
|---|---|---|
| default | 127 s → 455 s (+328 s) | 6 legs, 168 s — opt-2 still wins |
| **interval** | **132 s → 534 s (+402 s)** | **4 legs, 10 s — opt-2 loses by ~380 s** |

(opt-0 figures are #449's own hosted measurements, `ci.yml:370-372` —
the same lines the scan cited, renumbered by #626. Note the citation
was already misaimed: those lines are a prune-tooling step, not an
opt-level knob. The knobs this table is about are `ci.yml:517-518`
(default) and `:669-670` (interval).)

**Fix:** delete the two `CARGO_PROFILE_*_OPT_LEVEL` lines from
`build-interval` only, keep the debuginfo knobs, keep opt-2 on `build`,
and rewrite the now-false rationale comment. D9-safe by the repo's own
ratified reasoning (`ci.yml:408-411`, `ci.yml:1018-1020`: "the D9
bit-exactness pins hold at any opt level — opt never moves rounding" —
both renumbered by #626, and both already misaimed: the quoted sentence
is at `ci.yml:1145-1146`, in neither range).
Effort S.

**Estimated:** `build-interval` 569 s → ~170 s; **−6.7 min billed per
run**. Wall-clock on its own only −44 s, because the 493 s default build
then becomes the critical path — which is why this wants finding 10
alongside it.

**Caveat:** removing `CARGO_*` env rotates the rust-cache key, so the
first run after the change is cold and will look like a regression. Per
`GENERICS-BUILD-COST.md` §6, a first run after an env change is never
the verdict. Measure run 2.

#### 4. The boolean gate runs tier-1 validation twice, in release **[verified]**

`crates/topo/src/boolean/ops.rs:1263-1265` (as of the merge; `:1209-1213`
at the scan base):

```rust
pub(super) fn gate<T: Real>(body: &Body<T>) -> Result<(), BooleanError> {
    validate(body).map_err(...)?;
    validate_closed(body).map_err(...)?;
    Ok(())
}
```

`validate_closed` (`crates/topo/src/validate.rs:1390`) opens with
`let Tier1Report { .. } = tier1(body);`. Tier-1 is ~40 arena sweeps plus
~13 `SecondaryMap` allocations, and it runs **twice per boolean, in
release builds**, for no additional coverage — `validate_closed`'s error
vector already contains all tier-1 errors first, in the same documented
order.

**Fix:** drop the `validate` call. The only observable difference is
that an *invalid* result's error vector may additionally carry tier-2
findings appended after the tier-1 ones — a strictly more informative
refusal, but it does change `BooleanError::ResultInvalid`'s payload, so
pin it in the acceptance suite. D9-safe (validation is read-only).
Effort XS.

#### 5. Every Euler operator runs a full-body validate in debug builds **[verified]**

`crates/topo/src/euler.rs:1975-1992` (`assert_euler_postcondition`),
called from 15 sites across `euler.rs`, `euler_kill.rs`, `euler_ring.rs`,
`null.rs`, `split.rs`, `movefac.rs`. It pairs an O(1) arena-delta assert
with `debug_assert_eq!(crate::validate::validate(self), Ok(()))` — a
full ~40-sweep tier-1 pass over the *whole body*.

Building a body with K operator calls therefore costs **Θ(K·N) instead
of Θ(K)**. Booleans and splitting drive Euler ops in inner loops
(`boolean/zip.rs:147,171,173`, `boolean/insert.rs:431`,
`boolean/rest.rs:592,1191,…`, `chord_join.rs:1797,…`), so the boolean
inner loop is quadratic in debug.

**Why this matters for the numbers everyone reads:** the rebuild-latency
baseline is a debug build, and `topo` is `opt-level = 0` with
`debug_assertions` on in *every* CI test row (the 68/67 s band-4 rows,
the 66 s rebuild-latency row, the 55/48/41/32 s shards). This is a
leading candidate for the unexplained build/environment gap that §0's
`disputed_measurement` hypothesis names.

It also interacts multiplicatively with the quadratic validator pass 13
(`validate.rs:3051-3060`, which counts referers to null-scaffold curves
by scanning all edges): mid-boolean the body carries *m* scaffold
curves, so each per-op validate costs an extra m·E — making the
scaffolding phase cubic in debug.

**Fix:** keep the O(1) arena-delta assert unconditional; move the
full-body validate behind a cargo feature (`topo/paranoid-validate`)
enabled for one dedicated CI row and the proptest/fuzz suite. D9-safe by
construction — assertions cannot affect output bits, and release
behaviour is unchanged. Effort S. Fix pass 13's scan with a reference-count
`SecondaryMap` in the same PR (effort S).

#### 6. Memoization and parallel evaluation are unreachable from the shipping API **[verified]**

`crates/pncad-py/src/py/value.rs:669-679` is the only public evaluation
entry point, and it hard-codes both levers off:

```rust
pub(crate) fn evaluate(doc: &super::doc::Doc) -> Evaluation {
    Evaluation {
        inner: d::evaluate::<f64>(&doc.inner, None, &d::CancelToken::new(),
                                  &d::EvalOptions::default()),
        ...
```

`prior` is `None`; `EvalOptions::default()` sets `parallel: false`
(`crates/editor-core/src/eval/mod.rs:761-770`). Workspace-wide,
`parallel: true` appears **exactly once** — in a test
(`crates/editor-core/tests/m4_pr4_diff.rs:277`) — and a non-`None` prior
appears only in tests plus `demos/tour/src/heatsink.rs:180`.
`crates/editor-core/src/eval/parts.rs:295` additionally pins
`parallel: false` for every nested (cross-document) evaluation, so an
assembly can never parallelize either.

The 11× that the baseline demonstrates on `die` (1106 → 100 ms) is
therefore available to the latency test and one demo, and to **nothing a
user can call**. The machinery is written, tested, D9-clean and
delivering nothing.

**Fix:** thread an optional prior `Evaluation` and `EvalOptions` through
the Python `Doc`/`Evaluation` surface; hold the last `Evaluation` on the
`Doc` wrapper. D9-safe — reuse is certified by the content key, and
`evaluate` is documented as producing the same `Evaluation` either way.
Effort S.

**Expectation management:** turning `parallel` on will not move the slow
rows. The scheduler (`eval/mod.rs:843-862`) is level-synchronous, and
the expensive corpus documents are *chains* — `heat_sink` is a 5-long
union chain, `die` a 21-long subtract chain — which are depth-N,
width-1. Parallelism helps `die`'s independent pip subtrees and the
`nested_islands` fan-in, not the chains.

### Tier C — algorithmic, medium effort

#### 7. `mint_pcurves` re-certifies the entire body, on every operation **[verified]**

`crates/topo/src/pcurves.rs:934` (scan base `:837-875`) does
`body.pcurves.clear()` and then
re-mints and re-certifies **every face in the body**, at `CERT_SAMPLES = 9`
residual samples per boundary edge.

It is called from **9 production call sites across 3 crates** (topo,
sweep, step-import):
`boolean/ops.rs:532`, `merge_faces.rs:536`, `splitting/mod.rs:632`,
`transform.rs:424`, `sweep/fillet/surgery.rs:286`, `sweep/loft.rs:532`,
`sweep/revolve/mod.rs:695`, `sweep/revolve/tube.rs:253`, plus
`step-import/src/assemble.rs:820`. (It was ten: the fillet's whole-body
assembly door held the tenth, and both it and the door were deleted when
SMELL-SCAN S7 / D3 was executed. The fillet now re-mints once, from the
surgery, per fillet.)

**The consequence is a quadratic nobody named.** A chain of N booleans on
a growing body re-certifies the whole body N times: `die`'s 21 chained
subtracts pay O(N·F) where F itself grows with N. Worse, the merge
*inside* a boolean re-mints (`merge_faces.rs:536`) immediately before the
boolean re-mints again (`boolean/ops.rs:532`) — 2–3× per boolean op.

For the fillet surgery door specifically this is a straight
locality violation: `fillet_surgery` is the in-place composition path
whose entire selling point is that it touches a handful of faces, and it
then pays `O(all faces × boundary edges × 9)` certification over the
untouched remainder. The function's own comment justifies the full clear
on the grounds that a `SecondaryMap` row can outlive its key — which is a
correctness argument for *clearing dead rows*, not for *re-minting live
untouched ones*.

**Fix:** add a face-set variant `mint_pcurves_for(&mut body, &[FaceKey])`
and have each caller pass the faces whose loops it mutated (the fillet
surgery already tracks these as `blend_faces`, `corner_faces`,
`band_faces` plus the shrunk supports). Retain the full clear for stale
rows, or clear per-face via the existing `clear_face_caches`. Skip the
merge's re-mint when the caller re-mints anyway. Bit-identical: minting
is a pure function of the face's own surface and loop geometry, so
untouched faces re-derive the same bits. Effort M. **This one fix has
nine consumers** and is the highest-leverage runtime item in the report.

Note `transform_rigid`'s re-mint (`transform.rs:414-425`) is a
*deliberate* honesty choice whose comment argues the re-derived numbers
are identical — treat as policy, not bug.

#### 7b. CDT bulk-loading: a measured 35× on holed planar faces — with a D9 landmine in the dependency

**This is the only finding in the report backed by a benchmark.** The
scanning agent built an isolated harness against the pinned spade 2.15.1
at `opt-level = 2`, mirroring the exact insertion patterns in
`planar.rs` / `curved.rs` / `trimmed.rs`. It reproduces the repo's own
washer figure to within noise, which is the main reason to trust it.

Sequential insertion is still the only path —
`crates/mesh/src/planar.rs:231-247`, `curved.rs:115-137,150-164`,
`trimmed.rs:194-229` all build a `ConstrainedDelaunayTriangulation::new()`
and `.insert()` per point. **`bulk_load` has not landed**, and it is
available in the pinned version (`spade-2.15.1/src/cdt.rs:355`).

**The quadratic is real but it is not where the docs say.** Measured:

| face shape (per-face points) | sequential `insert()` | `bulk_load_cdt` |
|---|---|---|
| swept UV rectangle, boundary + row-major grid (66 049) | 101 ms | 76 ms |
| single circular loop (65 536) | 144 ms | — |
| **annulus: two concentric loops** (16 384) | **609 ms** | 42 ms |
| **annulus** (65 536) | **8 914 ms** | 257 ms |

Rect-grid faces (cylinder/cone/sphere/torus/NURBS) are **near-linear** —
the module doc's blanket "quadratic in per-face point count"
(`crates/mesh/src/lib.rs:142-153`) does not apply to them. A single
circular loop is also near-linear. The blow-up is specifically **two or
more nested near-cocircular loops** — a planar face with a hole: 4× the
points costs 14.6×. Insertion order does not rescue it (outer-first
8914 ms, inner-first 5644 ms, interleaved 8604 ms); `bulk_load_cdt`'s
radial spatial sort does, by **35–37× at 65k points, with the gap
growing**.

Phase split at 65 536 points: vertex `insert` 9 665 ms /
`try_add_constraint` 7.6 ms / face emission + classification 1.2 ms.
**CDT insertion is >99.9% of per-face cost**; the walk, constraints,
surface evaluation and certification are all noise.

**The washer's entire 1.2 s is two planar annulus faces.**
`crates/mesh/tests/review_m2_pr6_errors.rs:41-53` tessellates it at
δ=1e-6 (≈16k points per annulus face × 2 faces); the harness gives
2 × 609 ms = 1.22 s — an exact match. This is also the test the whole
`[profile.dev.package.{spade,mesh}]` block in `Cargo.toml:150-171`
exists for. The shape class is not exotic: washer, plate-with-hole,
counterbore, boss∪plate, die pips — every planar face with a ring.

**Scope, stated honestly:** tessellation is *not* on the rebuild path
(§5), so this buys nothing on the latency corpus. Its consumers are the
STL export acceptance path, the `watertight` CI row, and the demos —
plus every future fine-δ export. It is a large win on a narrow lane.

**D9 hazard — do not land this on stock spade 2.15.1.** Two things were
checked so the implementer does not have to:

- *Safe:* `bulk_load_distance_fn` (`bulk_load.rs:33-39`) keys on
  `(Reverse(dist²), x, y)` — a **total** order, so `sort_unstable_by_key`
  has no ties and load order is a pure function of the point set,
  independent of toolchain. Vertex order is preserved, so the
  `meta[handle.index()]` scheme survives.
- **Not safe:** `bulk_load.rs:358` and `:364` iterate a
  `std::collections::HashSet` under the default randomly-seeded
  `RandomState` — precisely the banned hash-order iteration. It fires
  whenever `single_bulk_insertion_step` returns `Some`
  (`bulk_load.rs:552-557`: "can happen if the vertices have the same
  angle"), which axis-aligned UV grids can produce. Our point sets are
  cocircular, where ties are decided by insertion order — the exact
  caveat `lib.rs:136-140` already names. When it fires, **mesh bytes can
  vary run to run.**

**Recommendation:** upstream a two-line fix (`Vec`/`BTreeSet`) and
`[patch.crates-io]` it in the meantime. Effort M plus the upstream
round-trip.

**Negative result that saves effort:** swapping the hint generator does
not help. `HierarchyHintGenerator<f64>` would be a one-line,
provably bit-identical change, but measured 39.26 s vs 39.05 s on a
131k-point annulus. The quadratic is the flip/legalization cascade
against a degenerate cocircular hull, not point location.

#### 8. `point_in_loop` re-decides loop-intrinsic facts on every query **[verified]**

`crates/topo/src/splitting/containment.rs:162-197`, called from
`boolean/contain.rs:130,141` and `chord_join.rs:1951`.

Hard call-count evidence, recomputed from
`docs/k-report-data/m7-eps-1e-9.csv.gz` (15 corpus documents,
284 178 decisions): `point_in_loop_boundary` 49 290 (17.3%),
`point_in_loop_side` 22 831 (8.0%), `point_in_loop_arm` 7 755 (2.7%),
`point_in_loop_advance` 5 632 (2.0%) — **85 508, or 30.1% of every
decision the corpus makes.** The `point_in_loop_*` family is the
largest in the corpus, ahead of `bool_point_in_solid_plane` (6.8%)
and `carrier_on_surface_1` (6.5%).

**Name split since #712.** That 49 290 was ONE name deciding two
questions. #712 split it: the degeneracy gate is now
`point_in_loop_segment` and the distance keeps
`point_in_loop_boundary`, at **24 645 each** — exactly half, because
the pre-pass asks both once per segment per query. The family total,
the margins and the decision sequence are unchanged; a reader chasing
the 49 290 figure in a fresh CSV must add the two rows.

Three query-independent wastes:

- the pre-pass's degeneracy gate — `point_in_loop_segment` since #712,
  now decided inside `ray_parity::on_boundary` —
  asks *"is this loop segment degenerate?"*. `e` depends only on the
  loop, not the query point, and it runs once per segment **per query**.
  That is ~24 600 corpus decisions — **8.7% of all decisions** —
  re-answering a fixed question about a fixed loop.
- `containment.rs:162` (`loop_points`) walks the half-edge cycle and
  allocates a fresh `Vec<Point3<T>>` per call, while
  `contain.rs:80` (`loop_cycle_points`) built the identical list for the
  identical loop microseconds earlier and dropped it.
- `containment.rs:229-231` allocates `xs`, `ys`, `sides` per *ray
  attempt*, so a grazing retry re-allocates all three.

**Fix:** a per-loop cached `LoopGeometry { points, seg_dir, seg_len, degenerate }`
built once per (loop, body-generation) and threaded through
`contfp → point_in_loop`.

**D9 caveat, important:** the values and the decision *sequence* are
identical, but the degenerate-segment `decide` would fire once per loop
instead of once per query — which **changes the recorded K sample
stream**. `docs/K-REPORT.md` counts and the verdict-diff pins would need
re-pinning deliberately. That is a real contract touchpoint, not a
free win. Effort M.

#### 9. Kill-direction Euler ops are O(arena), not O(1)

`crates/topo/src/body.rs:417-428` (`remove_curve_if_orphaned` —
`self.edges.values().any(...)`, O(E)), `body.rs:440-456`
(`remove_surface_if_orphaned` — O(F) plus an O(C) curve scan in which
`description_surfaces` at `body.rs:462` allocates a fresh `Vec` per
curve), `body.rs:485-494` (`remove_point_if_orphaned`, O(V)). Called
from `euler_kill.rs:471,473,658,660,883,891`, `euler_ring.rs:515,790`,
`split.rs:265`, `attach.rs:84,255`.

Every `kev`/`kef`/`kvfs`/`kemr`/`split_edge` pays O(E+V), and when a face
dies an extra O(F+C) with C small heap allocations. A zip killing n seam
edges costs O(n·N). **This directly falsifies PERF-PLAN §1.2's
"Euler-op sequences are O(entities built) with small constants"** — in
release, not just debug.

**Fix:** maintain reference counts in
`SecondaryMap<CurveKey, u32>` / `<SurfaceKey, u32>` / `<PointKey, u32>`,
incremented at mint and decremented at kill; the scans become O(1)
lookups. Minimum viable: make `description_surfaces` return a
`[Option<SurfaceKey>; 2]` instead of a `Vec` — the allocation-inside-a-scan
is free to remove. D9-safe: identical outcome, unchanged removal order.
Effort M (refcounts) / S (the allocation).

#### 10. CI: shard the default archive build

`ci.yml:344-481` — renumbered by #626, and already misaimed: that span
is the `discipline` job, while the `build` job this item is about is
`ci.yml:434-575`. 19 test binaries at ~20 s each of codegen+link, fully
independent, executing on a 2-vCPU runner. `ci.yml:96-106` establishes
that 8-vCPU runners are not landable (`evgunter/cad` is User-owned;
larger runners need an org), so more cores can only come from more
*runners*.

Split the archive into two matrix legs, each archiving half the closure;
each `test` leg downloads both archives and runs with the same
`--partition`. Each shard re-pays the ~75 s lib chain but halves the
379 s codegen:

```
current:  75 + 379 = 455 s step   (493 s job)
2 shards: 75 + 190 = 265 s step   (~305 s job)
3 shards: 75 + 126 = 201 s step   (~240 s job, diminishing)
```

**Combined with finding 3, the run's critical path goes 612 s → ~380 s
(−3.9 min, −38%).** D9-safe — identical binaries, identical test set.

**Cost, stated plainly:** this trades billed minutes for wall-clock. The
default lane goes 493 → ~610 billed seconds because the lib chain
compiles twice. Given `ci.yml:143-146` records an Actions-budget
exhaustion as the reason the change filter exists, **that trade is
Evan's call.** Net across findings 3+10: billed −4.7 min, wall −3.9 min.

#### 11. `merge_group` rescans the whole edge arena after every kill

`crates/topo/src/merge_faces.rs:743-772` and the identical shape at
`:798-822`: `loop { for (edge_key, edge) in self.edges() { … break } … kef }`.
Absorbing k faces re-walks all E edges k times, and `in_group`
(`:740`) is `members.contains(&f)` — linear in group size, evaluated per
edge per rescan. Cost O(k·E·|members|), for information the caller
already computed and discarded (`merge_coplanar_faces_declared:400-423`
built exactly that adjacency as `neighbors`).

Merging runs on **every** boolean (`boolean/ops.rs:508,1825,1887`,
`boolean/rest.rs:282`) and every split reassembly.

**Fix:** pass the per-group shared-edge list in as a worklist; make
`in_group` a `SecondaryMap<FaceKey, ()>`. Build the worklist in
edge-arena order so the D9-relevant "first shared edge in arena order"
tie-break is preserved. Effort M.

Related, same file: `merge_faces.rs:467,487` clone the **whole body**
per merge group (10 arenas + 11 `SecondaryMap` sidecars) and run a full
`validate_closed` per group (`:489`) — O(G·N) copying and validation for
G groups. Fix by validating only the affected shell, or by staging all
groups and validating once with per-group fallback on refusal. Effort M.

#### 12. ~~Fillet `Plan::assemble` is ~cubic in blended edges~~ — **RETIRED, subject deleted**

This row's subject no longer exists. `Plan::assemble` and its `runway`
greedy face scan were the fillet's **whole-body assembly door**, deleted
in full when SMELL-SCAN S7 / D3 was executed: the composition surgery's
front door strictly contains the whole-body door's, so the second
implementation was retired rather than optimized. There is nothing left
to re-measure and the prescribed fix has no site to apply to.

The row is kept struck through rather than removed, because what it
measured is still a real cost SHAPE and a future assembler could
reintroduce it: the finding was a greedy O(V) rescan whose inner probe
(`find_half_edge` → `loop_cycle` → `bounded_walk`) walks and
**allocates** per position, giving O(V³). The surviving surgery does not
have that shape — it splits and merges named faces in place rather than
growing a patch by scanning for the longest run — so nothing inherits
the row.

Two consequences elsewhere in this document, both applied: finding 7's
`mint_pcurves` call-site list drops from ten sites to nine, and the
fillet's contribution to it is now one re-mint per fillet rather than
two doors' worth.

#### 13. Boolean `join` is O(n³) with hoistable invariants

`crates/topo/src/boolean/join.rs:280` — `while let Some(m) = find_match(&open, …)`,
where `open` shrinks by at most one per step and `find_match`
(`:550-551`) is a full double loop over `open × open`. A second
independent O(n³) sits at `:292`, where `loose_partners` is recomputed
from scratch every join step and is itself O(L²) (`:800-804`).

Two invariants sit loop levels too deep:
- `:546-548,555,558` — `slots()` returns a `Vec<usize>` of at most 2
  elements and is called *inside* the entry loop though it depends only
  on the outer candidate. For |open| = 40 that is ~6 200 heap
  allocations per `find_match`, ~250 000 per boolean node, for a value
  that fits in `[bool; 2]`.
- `:583` — `germ_section_frame(red, &rga, band)` depends only on
  `(cand, cs)` but sits two levels deeper, doing 2 `Surface::cloned()`
  and, on curved pairs, a full section classification. Hoisting takes it
  from O(|open|²) to O(|open|) calls. The same hoist is available at
  `:822`.

**Fix:** hoist both; bucket `open` by `(a_face, b_face)` once so the scan
is per-bucket (the filter at `:561`/`:809` rejects nearly everything);
compute `loose_partners` incrementally. D9-safe — same candidate set,
same arena order, same decide sequence (the face-pair filter is exact key
equality, not a predicate). Effort M.

#### 14. `graft_solid` is O(E²) for want of an inverse map

`crates/topo/src/boolean/combine.rs:396-400` —
`src.edges.iter().find(|(_, e)| e.curve == k)` inside a loop over source
curves (`:340`). O(C·E) = O(E²), plus a full re-certification per curve.
Called once per boolean to bring operand B into the result
(`boolean/rest.rs:228`, `boolean/finish.rs:304`, `ops.rs:1676,1817`).
The `RemapKeys` path short-circuits before the scan (`combine.rs:357`),
so instancing is unaffected — this is boolean-only.

**Fix:** build `SecondaryMap<CurveKey, EdgeKey>` in one O(E) pass before
the loop. Bit-identical. Effort S.

#### 15. Boolean name paths nest one `Box` per boolean → O(chain²)

`crates/editor-core/src/names/emit_topo.rs:521-527` — `wrap` applies
`RoleSeg::FromA(Box::new(inner))` to **every** face/edge/vertex of a
boolean result, not only fragmented ones. Each boolean adds a layer to
every entity name. `die`'s 21 chained subtracts leave names **21
`FromA` levels deep**; the corpus states this explicitly at
`crates/editor-core/tests/fixture/mod.rs:296-298`.

`NameTable` is `BTreeMap<StableName, Entry>` plus a reverse map
(`names/table.rs:67-70`), and `StableName`'s derived `Ord` recurses
through `Box<StableName>`, so every tree comparison is O(depth) and
`insert` (`:95-108`) deep-clones the name per row. Over a chain of N
booleans with E entities that is **O(E·N²·log E)** in name handling
alone.

It scales with exactly the documents that are expensive: `die` (21
subtracts), `nested_islands_106_depth1` → `depth2` (147 → 310 ms, one
more nesting level), `heat_sink` (5-union chain), `crossing_slots`
(boolean-of-boolean). It also inflates the persisted recipe — plausibly
contributing to the 64/59 s persistence CI rows independently of
evaluation.

**Fix:** give `StableName` a precomputed `u64` structural digest used as
the `Ord`/`Hash` fast path, falling back to deep compare only on digest
equality (effort M, D9-safe, changes no float and no wire format). Full
interning of role paths is effort L and **touches the serialized recipe
format** (`names/role.rs:57-70`), so prefer the digest.

### Tier D — smaller, verified, cheap

#### 16. Tier-3 validation runs twice on the product/export path

`crates/editor-core/src/product.rs:280` (per source body, when a product
holds >1 solid) and `:313` (on the grafted aggregate) both call
`topo::validate_geometric`, in release. The aggregate contains every
source's entities, so the per-source pass is a strict subset. Tier 3 is
the expensive tier — per-edge dihedral sampling, full mass properties
including the quadrature lane (`validate.rs:2196`), and a pcurve
certificate replay (`:2233`). On the export path
(`crates/pncad/src/export.rs:182`) and every `InstantiatePart` node
(`eval/parts.rs:318`), so it touches the demo tour and all three montage
CI rows.

**Fix:** validate the aggregate first; re-validate per source only on
failure, to localize the refusal. Same accept/reject decision, same
error content on the failure path, one pass instead of two on success.
Effort S; check `ProductError::SolidInvalid` consumers first.

#### 17. `geom-core` has zero `#[inline]` attributes **[verified]**

`crates/geom-core/src/real.rs`, `interval.rs`, `dual.rs` and all 7
files in `linalg/` contain **no `#[inline]` attributes at all**. At
`opt-level = 0` rustc does not inline across function boundaries without
the hint, so in the measured dev build every `Vec3::dot`,
`Real::from_f64` and `Real::sqrt` is a real function call.

This targets the *actual measured bottleneck* — CI and dev-loop wall
time — rather than release latency, and it is honored even at
`opt-level = 0`. Adding `#[inline(always)]` to the trivial
`impl Real for f64` bodies (`real.rs:593-663`) changes not a single bit
of arithmetic. D9-safe, effort S.

**Measure it at the binary that instantiates the generics, not the crate
that defines them** — PERF-PLAN §2.3's hard-won lesson, and the reason
`[profile.dev.package]` overrides could not capture this win.

#### 18. Per-call constant waste

Individually small, collectively systematic, all D9-safe:

- `crates/topo/src/boolean/reduce.rs:483-492` — a full `EdgeCurve<T>`
  clone taken **unconditionally before** `conic_plane_crossing_roots`
  immediately returns `Err(())` for a `Line` carrier. Every corpus
  document is 100% line-carrier, so every candidate pair pays this for
  nothing. Also hoistable from the same loop: the `Edge` clone
  (`:450-455`), the 6-arena-lookup endpoint resolution (`:456-467`), and
  `face_plane` (`:472`, constant per face). Effort S.
- `crates/sweep/src/fillet/battery.rs:562-569` — two `Surface` clones
  passed to a function taking `&Surface`, while `body` is borrowed
  immutably throughout. Gratuitous; deep-copies a whole control net for
  NURBS-supported faces. Effort S (pure borrow fix).
- `crates/sweep/src/fillet/battery.rs:882-889` — predicate 6 resolves
  each corner ~6× redundantly (12 chains × 2 ends over 8 distinct
  vertices × 3 incident edges = 72 `resolve_link` calls where 12
  suffice). Dedup corner vertices and memoize links by `EdgeKey`.
  **Verify against the refusal-order fixtures** (`m5_pr12_refusals.rs`):
  a corner that currently refuses on its second visit would refuse on
  its first. Effort S.
- `crates/topo/src/pcurves.rs:1184` — `let surface = surface.clone();`
  used only through `&surface`, with `body` borrowed immutably
  throughout. One-line removal. Effort XS.
- `crates/geom-core/src/spline/basis.rs:134-135` — `ders_basis_funs`
  allocates `a_prev`/`a_cur` **inside** the doubly-nested loop: ~25 heap
  allocations per call, ~50 per surface jet, up to **1 600 per Newton
  projection**. The a-ladder is `f64` of length ≤ 3; replace with fixed
  arrays. Cold for the rebuild corpus (all analytic surfaces), hot for
  step-import (31 s) and the wild-corpus montage (96 s). Effort S.
- `crates/geom-brep/src/ssi/enclose.rs:595-596` — `rect_box` calls
  `deriv_box` twice, and each opens with an identical `point_box(u0,u1,v0,v1)`
  call: exactly 2× duplicated work per subdivision cell (up to 10⁵
  cells). Compute once, pass in. Bit-identical. Effort S. (Cold today —
  see §5.)
- `crates/geom-core/src/k_stats.rs:82-103` — every kernel decision pays
  two thread-local accesses and a `RefCell::borrow_mut()` even on paths
  that never install a verdict log (STL export, step-import, the demos).
  Gate the borrow behind a `Cell<bool>`. Effort S; low confidence on ms,
  over-weighted by the dev-profile data.
  **Explicitly not a feature-flag leak, and not a bug:** `VERDICTS` is
  deliberately outside the `probe` feature for the same D9 reason
  `CURRENT` is — a `cfg` there would make the funnel's code path differ
  between build configurations. Turning `probe` off does not and should
  not change it. The verdict log is a *production* feature (M4 PR 4 /
  NAMING-DESIGN N5): `editor-core/src/eval/mod.rs:1160` brackets every
  node evaluation in one and retains the result on the node, so
  production genuinely records on the one path that asks to. What was
  actually wrong is **documentation**: the module header claimed
  production "records nothing", contradicting `decide`'s own contract
  sixty lines below, which has always named the verdict-log push.
  Header corrected 2026-08-14.
- `crates/topo/src/props.rs:289` — `tag_of` does a linear
  `tags.iter().position` per half-edge, O(L²) per loop. Irrelevant for
  4-gons, quadratic on the long merged loops `merge_group` produces.
- `crates/topo/src/euler.rs:1298`, `euler_ring.rs:444` —
  `cycle[..position].to_vec()` allocates a second `Vec` where
  `Vec::truncate` would do. Effort XS.
- `crates/topo/src/transform.rs:337` — a `std::collections::HashSet` in a
  crate whose docs (`lib.rs:63`) state it contains none. Membership-only
  so D9 is not violated, but `SecondaryMap` is cheaper and consistent.

---

## 2. Open obligation: the verdict-log back channel

**Not a performance finding.** Raised by Evan on reading this scan, and
recorded here because it is a structural-safety debt that the scan
surfaced and that needs a decision, not a benchmark. **The obligation is
that this gets redone, or that a better design is thoroughly proven
impossible AND the mechanism is then made structurally safe rather than
comment-enforced. Leaving it as-is because it works today is not one of
the outcomes.**

### 2.1 What it is

`wire::run_op` returns `(payload, name_table)`. Verdicts are not in that
tuple. They arrive by side effect: `editor-core/src/eval/mod.rs:1160`
(scan base `:1113`) calls `k_stats::start_verdict_log()`, runs the op,
and `:1170` calls
`take_verdict_log()` — harvesting whatever any kernel predicate anywhere
beneath pushed into a thread-local in `geom-core`. The harvest becomes
`NodeValue::verdicts`, and `resolve::vdiff` turns it into
`NodeVerdictDelta`'s flips and divergences (`vdiff.rs:223,233,356`) —
production naming output, not telemetry.

So a production data path crosses a crate boundary invisibly. Nothing in
any signature between `k_stats::decide` and `vdiff` mentions verdicts.

### 2.2 Why "it holds today" is not good enough

The correctness argument is a comment (`eval/mod.rs:1154-1159`): *"The
bracket is per-node and thread-confined (kernel ops are single-threaded;
idiom-1 parallelism runs whole nodes on one worker each), so logs never
interleave across nodes."* That invariant is true today. Nothing
enforces it, and the failure mode is a silently wrong
`NodeVerdictDelta` rather than a compile error or a panic.

Three concrete ways it breaks, in increasing order of how close they
already are:

1. **Intra-op parallelism.** The moment any kernel op spawns internally,
   verdicts interleave across nodes. This is not remote: finding 6 is a
   recommendation to turn on parallel evaluation, and PERF-PLAN §2.2
   names per-face tessellation and certification sampling as rayon
   targets. The comment's invariant is exactly what those changes would
   invalidate, and nothing would fail to compile.
2. **Bracket scope.** The bracket is around `run_op`, not around the op.
   Anything running between `start` and `take` contributes to the log,
   whether or not it belongs to that node.
3. **Nesting is unmodelled and already reachable — see 2.3.**

There is also direct evidence the invariant has been forgotten once
already: the module header drifted to claiming production "records
nothing", which stopped being true when M4 PR 4 landed and contradicted
`decide`'s own contract sixty lines below. Two independent scanning
agents in this survey read it as telemetry on the strength of that
sentence. A comment that has already misled three readers is not load
bearing.

### 2.3 The nesting case is a live defect — observed, not predicted

**Mechanism (certain, four lines of code).**
`k_stats.rs:217-219` — `start_verdict_log()` assigns `Some(Vec::new())`
*unconditionally*, discarding any log already installed.
`k_stats.rs:224-226` — `take_verdict_log()` does `.take()`, leaving
`None` behind. So an inner bracket destroys the outer's accumulated
verdicts and then leaves no log at all.

**Reachability (traced through named call sites).** An
`InstantiatePart` node evaluates another document *inside* its own
`run_op`:

```
eval_node(InstantiatePart)          eval/mod.rs:1160  start_verdict_log()
  └ wire::run_op
     └ parts::PartCache::part       eval/parts.rs:265
        └ resolve_and_evaluate      eval/parts.rs:270
           └ evaluate_nested        eval/parts.rs:302
              └ evaluate_at_descent eval/mod.rs:869    eval_node(inner)
                                    eval/mod.rs:1160   start_verdict_log()  ← discards outer
                                    eval/mod.rs:1170   take_verdict_log()   ← leaves None
eval_node(InstantiatePart)          eval/mod.rs:1170  take_verdict_log() → None → empty
```

**Consequence:** an `InstantiatePart` node's `verdicts` is always empty.
Decisions the node made *before* the nested evaluation are discarded by
the inner `start`; decisions made *after* it — including
`topo::validate_geometric` at `parts.rs:318`, which is decision-dense —
are pushed into a `None` log and dropped on the floor. Either way
`vdiff` sees an empty verdict vector for every instantiate node and can
attribute no flips through it.

**Observed.** A temporary probe added to the existing
`asm2a_instantiate` fixtures (a one-solid part, instantiated into a
one-node assembly, evaluated through the ordinary `evaluate` entry) and
then reverted:

```
INSTANTIATE verdicts len = 0
FLAT node RecipeNodeId(0) verdicts len = 69      // the same part document,
FLAT node RecipeNodeId(1) verdicts len = 653     // evaluated directly
FLAT total = 722
```

The identical geometry produces **722 verdicts when evaluated as a
document and 0 when reached through an instantiate node**. The verdict
log is not merely lossy across the seam; it is completely empty on the
far side of it.

**Scope of the observation:** this is one fixture on the f64 lane. It
establishes that the seam drops verdicts; it does not establish how much
the diff engine's output is degraded in practice, which depends on how
much assembly work real documents do. The regression test this wants is
the probe above with `assert!(!verdicts.is_empty())` — it fails today.

This defect predates this scan and is independent of every performance
finding in it. It is listed here rather than in §1 because it is a
correctness consequence of the structural problem.

**It is not blocked on §2.4's unresolved choice.** Either branch would
subsume it — a returned value cannot be clobbered by a nested call, and
an RAII guard that refuses re-entry would have failed loudly at the
first assembly evaluation rather than silently returning nothing — but
it does not have to wait for that decision. A direct fix (save and
restore the enclosing log across a nested bracket, or refuse re-entry)
plus the regression test is landable on its own, and should be, since
the design question may sit for a while. Whoever takes §2.4 later
inherits a fixed bug and a test that pins it, which is strictly better
than inheriting both.

### 2.4 The obligation

**Status: (a) vs (b) is UNRESOLVED and was left so deliberately at
merge (Evan, 2026-08-16).** Neither branch is chosen here; the choice
is recorded as open rather than settled by whoever wrote this report.
What *is* settled is that one of them has to happen — "leave it, it
works today" is not a third branch.

Either **(a) redo it** so verdicts are a value, or **(b) prove (a)
unaffordable in writing and then make the current mechanism structurally
safe.**

Under (a), the shape is that `run_op` returns verdicts in its tuple like
`name_table` already is, with the sink threaded explicitly. The honest
objection — presumably why it went the way it did — is that this means
touching every kernel predicate signature, which is enormously invasive
and would put a sink parameter in `geom-core`'s hottest function. That
objection deserves to be written down and weighed rather than assumed;
intermediate designs exist (a sink threaded only as far as each crate's
single `sign_within` funnel, rather than to every call site).

Under (b), **"structurally safe" means at minimum all four of**:

- **The bracket cannot leak or be forgotten.** `start_verdict_log()`
  returns `()`; make it return an RAII guard whose `Drop` harvests, so
  the log's lifetime is the guard's scope and a forgotten `take` is
  impossible.
- **Re-entrancy fails loud instead of silently discarding.** Installing
  a log over an existing one is a defect (2.3). It must refuse or
  compose deliberately — never overwrite.
- **The thread-confinement assumption is enforced or removed.** Today it
  is asserted in prose. Either the type prevents an op from spanning
  threads, or the log tolerates it. "We currently don't" is what breaks
  the first time finding 6 lands.
- **The cross-crate coupling is visible at both ends.** `geom-core`
  should say that `editor-core::resolve::vdiff` is a consumer, and
  `editor-core` should not depend on a `geom-core` thread-local without
  that dependency appearing in a signature or a named contract.

### 2.5 Interactions with the rest of this report

- **Finding 6** (turn on parallel evaluation) is the change most likely
  to break the comment-enforced invariant. Resolving this obligation
  should precede it, or the two should land together.
- **Finding 18**'s `Cell<bool>` guard on the `RefCell` borrow is
  orthogonal and safe under either resolution — it changes cost, not
  structure.
- Nothing else in this report depends on the outcome.

## 3. What this changes about `die`

Two of this report's findings are independent quadratics on the same
structure, and both were missed by PERF-PLAN's ranking:

1. **Whole-body pcurve re-certification per boolean** (finding 7) — 21
   subtracts each re-certifying a growing body.
2. **`Box<StableName>` nesting per boolean** (finding 15) — names 21
   levels deep, with O(depth) comparisons in two `BTreeMap`s.

Neither is the "quadratic edge×face sweep" PERF-PLAN ranked second; that
one is retired (§6). `die` is the corpus's clearest chain document
(77 nodes, 21 chained subtracts, 90 145 predicate decisions), which makes
it the natural pin for both fixes — and its 11× memoization win
(1106 → 100 ms) is the one large, quiet-run, ratio-based result in the
baseline that survives §0's caveats.

---

## 4. Recommended sequence

**Wave 1 — correctness and measurement (nothing depends on ordering):**
finding 1 (NURBS box + curved differential scenarios), finding 2 (Criterion
harness + re-refresh the baseline from CI).

**Wave 2 — free wins, no design questions:** findings 4 (XS, delete a
line), 5 (S, feature-gate the debug validate), 3 (S, two-line CI revert),
17 (S, `#[inline]`), 14 (S, inverse map), and the XS items in 18. These
are independently landable and mostly single-file.

**Wave 3 — the leveraged runtime work:** finding 7 (`mint_pcurves`
face-set variant — nine consumers), then 6 (expose the memo through the
API), then 9, 11, 12, 13. Finding 7b (CDT bulk-load) is independent of
all of these and can run in parallel — but it is gated on the upstream
spade fix, so start that round-trip early.

**Not a wave — §2 runs on its own track.** The verdict-log obligation is
a design decision, not a scheduled optimization. Two parts of it are
time-ordered against the rest, though: the regression test for the
observed empty-log defect (§2.3) can land immediately and independently,
and the structural fix (§2.4) should precede or accompany finding 6,
since turning on parallel evaluation is what invalidates the invariant
the current mechanism rests on.

**Wave 4 — contract-touching, needs discussion first:** finding 8
(re-pins the K sample stream), finding 15 (name digest; the interning
variant touches the wire format), finding 10 (trades billed minutes for
wall-clock — Evan's call).

---

## 5. Negative results

Recorded so they are not re-investigated, and so stale PERF-PLAN claims
are not re-reported as findings.

- **The boolean edge×face sweep is no longer quadratic.**
  `crates/topo/src/boolean/reduce.rs:432` queries the BVH. To be precise
  about where the brute-force scan lives: it is **shipped production
  code, not test code** — a live runtime arm (`reduce.rs:422`) of the
  public `SweepStrategy` enum, reachable through `boolean_op_with`. No
  production *caller* selects it (`union`/`intersect`/`subtract` and
  every internal entry hard-code `SweepStrategy::Realized`); only the
  differential suite passes `Idealized`. That is PERF-PLAN §4.4's
  idealized/realized pilot working as designed — the O(n²) reference
  must stay compiled and executable for the pin to run both paths and
  compare. PERF-PLAN §1.3 rank 2 is **retired**, and `reduce.rs:1-30`
  documents the retirement.
- **The memo/content-key machinery is not a cost.** Keys never traverse
  geometry — a node's key is a Merkle chain of ~10 `write_u64` calls
  regardless of whether its operands have 12 or 12 000 faces
  (`eval/mod.rs:1150-1306`). `KeyHasher` is stack-allocated FNV-1a. The
  memo is a `BTreeMap` keyed by u64 node id; the 128-bit key is only
  compared. **Cache hits never deep-clone geometry** — `NodeValue`'s
  payload is behind `Arc`, and `make_mut`/`try_unwrap`/`unwrap_or_clone`
  have **zero hits workspace-wide**.
- **The "inverted" latency rows are not a cache failure.** In
  `crossing_slots` (123→126), `die_fillet` (25→33), `loft_prism`
  (2.5→3.0), `heat_sink` (102→101) and `plate_param` (43→41), the
  dominant node is *inside the reuse cone by construction*, so there was
  nothing to save. There is no dirty-marking at all — every node is
  keyed and reused iff the key matches — so the cone is **exactly
  minimal**, and `heat_sink`'s cone of 12/15 is the true answer for a
  5-long union chain. Compounding this, the two columns **time different
  documents**: the incremental column evaluates the *bumped* doc, and
  `loft_prism`'s bump changes `VDegree: 2→1` — a different loft. A ratio
  between two different workloads is not a cache measurement. *(This
  disproves the coordinator's initial hypothesis; recorded so it is not
  revived.)*
- **`die_composed` is not an outlier.** Its baseline row is a
  contended-run artifact by the file's own provenance. Independent
  confirmation from the decision-count corpus: 9 977 decisions / 2450 ms
  = 245 µs per decision, against 6–12 µs/decision for the genuinely
  boolean-heavy documents. Rescaling by the contention factor visible in
  the same discarded run (`die_fillet` 1001.4 ms vs 25.9 ms committed,
  ≈38.7×) puts it near ~63 ms — mid-pack. **There is no combinatorial
  blowup in the octant/corner/torus-band handling**: `corner_ball`
  (`fillet/blend.rs:266-294`) is a closed-form Cramer solve,
  `octant_chart` (`build.rs:172-205`) is an argmin over ≤3 links,
  `plane_sphere_blend` is straight-line closed form, and the rim phase
  (`surgery.rs:1043-1432`) is O(n) per phase with no nesting. The one
  loop that looks like a retry (`surgery.rs:1176-1182`) is a bounded
  5-candidate branch lift.
- **Tessellation is not on the rebuild path at all, and is not in any
  latency number.** `mesh` is a **dev-dependency** of `editor-core`
  (`crates/editor-core/Cargo.toml:75-82`); every `mesh` mention in
  `editor-core/src/` is a comment. `m4_pr8_latency.rs` times
  `editor_core::evaluate` only — there is no tessellation inside its
  timer (`:79-104`). `FacePatch` has exactly one non-test consumer in
  the tree: the STL writer (`crates/stl/src/lib.rs:141`). Tessellation
  is not exposed in `pncad-py` at all. **[verified]**
- **There is no per-face tessellation memo, and no consumer that would
  benefit yet.** The memo-key contract is *documented*
  (`crates/mesh/src/lib.rs:52-71`: per-face tessellation is a pure
  function of (surface, loops, chord points, δ)) but nothing implements
  it. PERF-PLAN's rank-1 "preview lane" concerns a GUI that does not
  exist. Build the cache when the GUI lands, not before — and rank it
  below finding 7b, which is a measured win on code that runs today.
- **Current CI tessellation is microseconds.** All CI tessellation runs
  at δ=1e-2 (`crates/stl/tests/common/mod.rs:47-66`) → ~31 chord points
  per circle. The 44 s `watertight` row and the 96–150 s montage rows
  are **compile-bound**, not mesh-bound (`ci.yml:1319` records the tour
  itself as "~3 s once built" — renumbered by #626, but the quotation
  does not appear at that line or anywhere in ci.yml, so the citation
  is unresolved rather than merely stale). Do not attribute those wall times to
  meshing.
- **`mesh/cert.rs` is not a cost center and does no sampling.** All five
  certificates (`cert.rs:107-148`, `nurbs_cert.rs:183-187`) are
  closed-form O(1) per triangle. **`CERT_SAMPLES` is not referenced
  anywhere in `crates/mesh/`** — it lives in `geom-brep/src/certify.rs:66`
  and is consumed by `topo` and `step-import`. PERF-PLAN §1.2's rank-5
  "tier-3 cert" is a different crate's concern.
- **Cylinder faces have zero interior grid points** (`curved.rs:216-219`
  sets `nv = 1`, so the `for j in 1..nv` loop at `:150` never executes).
  Grid-based curved faces generally are near-linear, not quadratic.
- **`MAX_GRID_RETRIES`** (`trimmed.rs:77,193,281-286`) looks alarming —
  up to 5 full CDT rebuilds per face — but fires only when a grid point
  lands *exactly* on a boundary constraint. Cold; leave it.
- **Surface evaluation in the mesh lane is already well-managed** —
  `trimmed.rs:318` evaluates only grid slots used by kept triangles;
  `chords.rs:203-211` evaluates each edge's chords once for both
  adjacent faces. The one redundancy found is `nurbs_face_bound` being
  computed twice per NURBS face (`chords.rs:578-585` memoizes it but
  discards `muu/muv/mvv`, so `trimmed.rs:151` recomputes the whole
  `rational_face_bound` — the most expensive non-CDT computation in the
  crate, run twice for no reason). Fix: widen the memo to hold the whole
  bound. Bit-identical, effort S.
- **SSI is completely cold. [verified]** `docs/K-REPORT.md:541` and a
  direct recount of `m7-eps-1e-9.csv.gz` both show **zero `ssi_*`
  samples** across all 1 792 926 recorded decisions — 15 corpus
  documents and 19 demo scenes. Any SSI optimization is a
  CI-lane win only. (Its two clean redundancies are still recorded in
  finding 18 for whenever it goes live.)
- **`iso::canonical_form` is not on any production path** — `pub(crate)`
  with every caller inside `#[cfg(test)]`. Its true cost is worse than
  its docstring admits (O(D²·L²) *string* allocations per shell via
  `loop_sig`'s rotation-minimal encoding), but it only ever sees ~30-dart
  fixtures. Worth a comment, not work.
- **Newton loops are convergence-based, not fixed-count**
  (`geom-curves/src/projection.rs:195-241`,
  `geom-surfaces/src/projection.rs:277-342`): exit on coincidence,
  orthogonality or stagnation, with `MAX_ITERS = 32` as a refusal budget.
  One jet evaluation per step, no restarts, no redundant Jacobians.
- **Interval replay never lands on the normal path.** `sign_within`
  returns `Err(Indeterminate)` for an in-band margin and every caller
  maps that to a typed refusal — there is **no** automatic
  widen-and-retry anywhere. PERF-PLAN §1.2's "the f64 lane with K·ε
  escalation IS the fast path" is still accurate.
- **Knot spans use binary search**, not linear scan
  (`geom-core/src/spline/knots.rs:363-387`), and control-point buffers
  are not reallocated per evaluation. **No dynamic dispatch in hot
  geometry** — two `dyn` sites total across five crates, both cold.
- **The arena design is sound** — typed slotmaps, O(1) keyed lookup,
  deterministic slot-order iteration, no hashing. Every quadratic found
  in `topo` is a *caller* scanning an arena, never the storage.
- **Provenance/lineage tracking is clean** — one `SecondaryMap` per kind,
  written at birth, removed at kill, `&'static str` primordials, no
  strings, no rehash.
- **Test-binary count is fully optimized.** 367 `tests/*.rs` files exist,
  but all 14 crates with tests carry `autotests = false` plus a single
  `[[test]]` aggregator, gated by `scripts/gates/test-aggregation.sh`.
  The 19 binaries CI builds are the floor for 10 packages. Anyone reading
  "367 test files" as a lever is reading a pre-#387 number.
- **Generic monomorphization is not a material build cost** —
  `docs/GENERICS-BUILD-COST.md` measures copy counts of 1–3 in the
  geometry crates (not the 30+ that signals bloat); `Interval` appears
  zero times in a default build; `Probe` has since been feature-gated.
  62% of workspace LLVM IR is **serde derive in `editor-core`**, which
  has nothing to do with `Real`.
- **Already tried and rejected, do not re-propose:** blanket opt-2 via CI
  env (#52, measured net-slower, reverted); mold (#174 adopted, #451
  retired once the workspace hit 14 test targets — re-adoption trigger is
  "test-target count grows back toward triple digits"); sccache (reverted
  locally for incremental-compilation conflict, and on CI it would miss
  by construction on `TIER=closure` runs while third-party deps are
  already at a 100% rust-cache hit rate); 4-way test resharding (modelled
  at −8.2 min when legs were ~600 s; re-measured here at −22 s on a 612 s
  critical path — **not worth doing**); merging the default and interval
  build jobs (a DESIGN.md question, not a CI-tuning one).
- **Non-levers, checked:** `codegen-units` is unset and inherits cargo's
  default 256, already the fastest-compile setting; `CARGO_INCREMENTAL=0`
  is correct for a cold CI target dir; `split-debuginfo` has almost
  nothing left to move given the existing `line-tables-only` + strip;
  `lto` is pure compile-time cost at these leg lengths.
- **The render montage lanes cost zero wall-clock** — 470 s billed,
  ending at e+286 against a 587 s critical path. They gate on the whole
  kernel by design and are not soundly path-filterable. Running the two
  FreeCAD lanes on `push: main` only is a *policy* trade (drift caught
  after merge instead of before), not a perf finding.
- **`profile/` canonicalization is genuinely cold — but record the
  trigger.** `Profile::validate` step 3
  (`crates/profile/src/validate.rs:1020-1033`) judges every unordered
  segment pair — a real O(S²) with a per-pair contact `Vec` — and step 4
  is O(n_loops²) ray-parity queries. It is cold *only* because every
  corpus profile is a 4-to-8-vertex polygon. At S ≈ 400 it becomes ~80 000
  `pair_contacts` calls. PERF-PLAN's "leave it alone" holds; the trigger
  for revisiting is a GUI sketch or an import path with >~50 segments per
  profile. **Note for future readers:** `nested_islands_105`/`_106` are
  *issue numbers*, not loop counts — those documents are 3–5 blocks each.

---

## 6. Corrections to PERF-PLAN

PERF-PLAN §1.3's ranking was written at M3-start against the M2
codebase. Four of its six entries have moved. This is not a criticism of
the doc — it labelled itself advisory and dated — but the ranking is
cited as if current, so the deltas should be written down.

**These corrections have been annotated into `docs/PERF-PLAN.md` itself
(2026-08-14), as dated `[STALE …]` / `[SUPERSEDED …]` / `[STATUS …]`
markers beside each expired claim.** The original prose was left intact
— that doc's value is partly as a record of what was believed at
M3-start — so the annotations are additive (161 lines added, none
removed). If you are reading PERF-PLAN, the markers point back here; if
you are reading this section, PERF-PLAN now carries the same deltas
inline where they matter.

| PERF-PLAN rank | Claim | Status now |
|---|---|---|
| 1 | CDT quadratic; "100× tighter δ ⇒ ~10⁴× CDT time" | **Right that it's quadratic, wrong about where and by how much** (below) |
| 2 | Boolean edge×face sweep is quadratic | **Retired** — BVH landed at M5 PR 8 |
| 3 | Full-DAG rebuild on any edit | **Solved in the kernel, unreachable from the API** (finding 6) |
| 5 | Validation: "not the bottleneck, do not optimize" | **Wrong in three specific ways** (below) |
| — | "Euler ops are O(entities built), small constants" | **False in release** (finding 9) |

On tessellation, PERF-PLAN §1.2 called CDT "the measured dominant cost"
and derived that 100× tighter δ costs ~10⁴× CDT time. Three corrections
(all from the benchmark harness in finding 7b):

1. **The δ-scaling claim is wrong by ~150× on its own datapoint** — the
   washer's 1e-4 → 1e-6 step measures **63×**, not ~10⁴×.
2. **The quadratic is not general.** Grid-based curved faces are
   near-linear; so is a single circular loop. The blow-up is specific to
   *nested near-cocircular boundaries* — planar faces with holes.
3. **It is not on the preview lane, because there is no preview lane.**
   Tessellation is a dev-dependency of `editor-core`, absent from
   `pncad-py`, and consumed only by STL export and the demos. PERF-PLAN
   ranked it #1 for a per-edit preview path that does not yet exist.

On validation specifically, PERF-PLAN §1.2 said tier 1 is "linear arena
passes" and told readers not to optimize. Tier 1 *is* linear — but with
~40 sweeps and 13 `SecondaryMap` allocations per call, and:

1. it is invoked Θ(ops) times in debug builds (finding 5);
2. pass 13 is quadratic in null-scaffold curves (`validate.rs:3051-3060`),
   which is worst exactly mid-boolean when finding 5 fires most;
3. **tier 3 — which the doc scoped to the per-commit lane — now runs
   unconditionally in release** on the boolean, merge, product and
   step-import paths (`boolean/ops.rs:1209`, `merge_faces.rs:371,489,515`,
   `editor-core/src/product.rs:280,313`, `step-import/src/lib.rs:745`).

Also worth recording, though this one is **not** a staleness finding:
PERF-PLAN §2.1's "one BVH crate, three consumers" is a *plan*, and it is
on schedule. `Bvh::build` appears exactly once workspace-wide
(`boolean/reduce.rs:420`) — the M5 entry §5 sequences, delivered. SSI
seeding remains intended and unwired, with
`geom-brep/src/ssi/exhaust.rs:32-38,134,262` bisecting by hand and
saying why ("Brute force, deliberately, for now ... PR 8's BVH swaps in
... when profiling asks for it") — that is the doc's trigger discipline
working, not a missed delivery. Picking is blocked on there being a GUI.
The crate is 657 lines because it delivered the one thing needed. What
*was* misleading is the crate's own lib docs
(`crates/bvh/src/lib.rs:3-5`), which described all three duties in the
present tense so a reader could conclude SSI was already pruned by it;
corrected 2026-08-14 to mark which are live and which are intended.

And §2.2's parallelism vocabulary — ratified into DESIGN.md as the D9
addendum specifically so the first rayon PR would be cheap — has **one**
consumer. `rayon` is a dependency of `editor-core` alone, and
`crates/editor-core/src/eval/mod.rs:855` is the only `par_iter` in the
workspace. Of the five targets named in value order (M10 subdivision
driver, per-face tessellation, certification sampling, mass properties,
independent DAG nodes), only the last landed — and it is switched off in
every shipping caller (finding 6). **[verified]**

Worth recording because it is the cheapest of the four unbuilt targets:
**per-face tessellation is ready to parallelize, and the sole blocker is
a running counter.** `crates/mesh/src/tessellate.rs:81-136` threads
`&mut positions` through the face loop, and each lane mints interior-grid
ids as `positions.len() as u32` (`curved.rs:158`, `trimmed.rs:317`) — a
prefix dependency on every earlier face. Everything else is already
clean: `tessellate_planar` takes `positions: &[Point3]` read-only
(`planar.rs:144`), `walk::loop_polygon` likewise (`walk.rs:428`), chords
are shared immutably, and there is no other cross-face state. The fix is
exactly the idiom-1-then-idiom-2 shape DESIGN.md sanctions and already
names as a target: `par_iter()` into a pre-sized buffer emitting *local*
grid ids, then a sequential fold in face-arena order assigning base
offsets. Output is bit-identical. Two details: pick the first error in
arena order (not first-to-fail) so refusals stay deterministic, and
update the budget meter's thread-local invariant at
`mesh/src/budget.rs` ("tessellation runs on the calling thread, so armed
evidence stays attributable under a parallel test runner"), which is
where that claim lives now that `probe_stats` is gone (#709).
Effort M; ceiling is core count against face count.

Finally, a mechanism worth recording because it strengthens finding 3:
rustc enables `share-generics` **by default only at `opt-level = 0`**.
At any optimizing level it is off unless `-Zshare-generics` is passed,
which is nightly-only and therefore unavailable under the pinned stable
toolchain (`rust-toolchain.toml:4`). In a workspace where 40% of `src`
lives in files with generic-over-`Real` items, that plausibly explains
why #449's opt-2 penalty measured 3.4–4.6× hosted rather than the ~2× a
naive model predicts: at opt-0 downstream crates import upstream
instantiations; at opt-2 all 19 test binaries re-instantiate them
locally. **No action proposed** — forcing it via `RUSTC_BOOTSTRAP` on a
pinned-compiler determinism project is against the charter.

---

## 7. Method and limits

Six domain scans ran in parallel over the tree at `870c7a9`, each given
the same brief: anchor at `file:line`, argue by complexity class or call
count, tie to measured data where it exists, flag D9 hazards explicitly,
and report negative results. Every Tier A and Tier B finding was
re-verified by hand afterwards.

**What this scan did not do:**

- **Almost no profiling.** No `perf`, no flamegraph, no instrumented
  build. With one exception — finding 7b, where an isolated CDT harness
  was built and its numbers are labelled as measured — every cost claim
  is a complexity or call-count argument, not an observed hot-spot. The
  absence of a benchmark harness (finding 2) is precisely why, and the
  fact that the one agent who built a harness produced the report's only
  hard numbers is the argument for finding 2 in miniature.
- **No release-build timings.** The only runtime dataset available is a
  dev-profile measurement whose limits §0 documents. Findings whose cost
  is dominated by unoptimized generic arithmetic (17, and the `k_stats`
  item in 18) are systematically **over**-weighted by that data;
  asymptotic findings (7, 9, 11, 12, 13, 15) are **under**-weighted.
- **No code changed**, and no estimate here has been validated by
  implementing it. The CI numbers in findings 3 and 10 are arithmetic on
  measured step timings from run 31776906935, not observed outcomes.
- **CI failure history is not a perf signal.** Of the last 30 runs on
  main: 17 success, 10 cancelled (concurrency `cancel-in-progress` on
  rapid merges), 3 failure — and all three failures are correctness
  (`review_s2::fuzz_offset_carrier_construction_tangency_and_bulge`).
  **No timeout-class or OOM-class failures.** Performance is not
  currently breaking CI; it is a latency and throughput concern only.
