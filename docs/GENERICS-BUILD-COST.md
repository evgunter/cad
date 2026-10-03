# Are the core generics hurting build times? — measured findings (2026-08-12)

**Verdict: no.** The hypothesis under test was that monomorphization of
the kernel's `Real` generics — specifically the `f64` and `Interval`
instantiations — is a material build-time cost. It is not. Copy counts in
the geometry crates are **1–3**, not the 30+ that signals bloat;
`Interval` appears **zero** times in a default build; and **62% of the
workspace's LLVM IR is serde derive code in one crate that has nothing to
do with `Real`**.

The real build-time wins were elsewhere and have landed: #449, #450 took
CI's critical path from ~1065 s to ~789 s (−26%) and billed job-minutes
from ~137 to ~72.

Companion to `docs/LOCAL-BUILD-PERF.md` (the local box, 2026-08-11).
That document is about *machine conditions*; this one is about *what the
compiler is actually spending time on*. Read both before tuning build
config.

---

## 0. Method and where the numbers come from

Every number below traces to a command. Local rows ran on the box
described in `LOCAL-BUILD-PERF.md` §0 (i7-1065G7, 4 physical cores, 10 GB
to WSL2), under `local-scripts/with-build-slot.sh -x` so no other lane
was compiling concurrently. Hosted rows come from `gh api
repos/evgunter/cad/actions/runs/<id>/jobs`, medians over 11 recent
full-matrix runs unless stated.

§9's local rows are the exception and say so: they ran on a shared
4-vCPU box with another lane compiling, not on `LOCAL-BUILD-PERF.md`'s
box under the build slot, and they carry their own environment block.

**Carry ratios, not seconds, between the two.** Local is 4 cores / 8
threads; runners are 2 vCPU. §6 records a case where that mattered by 3×.

---

## 1. Baselines, and the `check`-vs-`build` verdict

Clean builds, whole workspace:

| | dev | release |
|---|---|---|
| `cargo build` (libs) | 30.1 s | 45.3 s |
| `cargo build --workspace --all-targets` | 122.9 s | 333.0 s |

Incremental, after a **real semantic edit** to the generic core
(`crates/geom-core/src/real.rs`):

| | |
|---|---|
| `cargo check --workspace --all-targets` | **6.0 s / 6.6 s** |
| `cargo build --workspace --all-targets` | **15.6 s / 15.3 s** |
| same, editing leaf crate `stl` instead | **2.7 s** |

**`check` is fast and `build` is ~2.6× it, so the delta is codegen, not
trait solving.** By the investigation brief's rubric that points at
monomorphization — but note the absolute number: **15.5 s** for a full
workspace rebuild from the root of the dependency DAG. Editing the
generic core costs about **+13 s over editing a leaf**. That is the
entire fan-out cost of `Real`, and it is not a problem.

### A measurement trap worth recording

`touch <file>` does **not** measure a rebuild. It bumps mtime, cargo
re-runs rustc, and rustc's incremental cache then finds every dep-graph
input unchanged and reuses everything. Measured: `touch real.rs && cargo
build` = **3.4 s**, versus 15.6 s for a real edit. The `touch` number
measures cargo's freshness path. Append a real item instead.

---

## 2. Workspace shape

16 members (`crates/*`); `demos/`, `tools/`, `interval-transcendentals/`
are deliberately excluded separate workspaces. 164,415 lines of `src`,
~119,000 lines of `tests`. Every crate that instantiates the core
generics is **internal** — there are no external consumers, because
`publish = false` until Q9 settles.

Source concentration (`src` lines): topo 50,965, editor-core 21,302,
geom-brep 20,688, geom-core 15,946, step-import 12,051, sweep 11,774,
profile 9,491.

**40% of `src` (66,219 lines) lives in files containing generic-over-`Real`
items**, so the generic surface is genuinely large. That makes the
findings below more surprising, not less.

---

## 3. The actual instantiation set

`Real` has exactly four impls:

| impl | where | in a default build? |
|---|---|---|
| `f64` | `real.rs:593` | yes |
| `Probe` | `k_stats.rs:325` | **yes — always compiled** |
| `Interval` | `interval.rs:234` | **yes — the impls, and every lane impl above this crate, compile in every build (§11: the `interval` feature is deleted)** |
| `Dual<T>` | `dual.rs:410` | only via tests |

`Dual<T: KinkJacobian>` has two inhabitants: `Dual<f64>` and
`Dual<Interval>`.

Counting concrete scalar type arguments in codegen'd symbol names
(`nm -S --defined-only <rlib> | rustfilt`), censused 2026-08-12, when
the `interval` module was itself gated — `geom-core`'s own row reads 44
today (§9); the consuming crates' rows still read zero without the
feature:

| unit | text symbols | `f64` | `Probe` | `Interval` | `Dual` |
|---|---|---|---|---|---|
| geom-brep | 2,249 | 839 | 534 | **0** | **0** |
| geom-core | 1,710 | 447 | 29 | **0** | **0** |
| topo | 1,581 | 231 | 164 | **0** | **0** |
| editor-core | 12,591 | 490 | 1 | **0** | **0** |

**No default build INSTANTIATES `Interval`**, which is the cost this
table is about; the impls themselves are compiled in every build. The
scalar that
actually multiplies `Real`-generic code on every build is `Probe` — the
K-experiment recording scalar, a transparent `f64` newtype, which is
feature-gated nowhere.

Static census of instantiation sites in `src`: `<f64>` 1168, `<Interval>`
37 (all inside `cfg`'d code), `<RingInterval>` 86. In `tests`: `<f64>`
2132, `<Interval>` 205, `<Probe>` 29, `<Dual<f64>>` 2, `<Dual<Interval>>` 2.

---

## 4. `cargo llvm-lines` — where the IR actually is

Release, `--lib`, default features:

| crate | LLVM IR lines | share | max copies of any one fn |
|---|---|---|---|
| **editor-core** | **626,103** | **62%** | 16 |
| geom-brep | 106,897 | 11% | 1 |
| topo | 95,033 | 9% | 1 |
| geom-core | 50,655 | 5% | — |
| sweep | 49,255 | 5% | — |
| profile | 44,487 | 4% | — |
| geom-curves | 33,413 | 3% | — |
| geom-surfaces | **192** | 0% | 2 |
| **total** | **~1,006,000** | | |

### 4a. The generics are in the "leave it alone" quadrant

The brief's diagnostic table says high-lines × **high copies (30+)** is
monomorphization bloat; high lines × **1–3 copies** is "a big function,
not a generics problem." Every `Real`-generic entry is in the second
category. The highest copy count anywhere in the geometry crates is
**3** — `nurbs_patch_face::<f64>`, `::<Interval>`, `::<Probe>`, 1,687
lines each.

The duplication that *does* exist is over **const** generics, not the
scalar: `march::<2,3,ImplicitPairR3>` vs `march::<3,4,ParametricPairR4>`,
`Svd<2,3>` vs `Svd<3,4>` — dimension, not `T`.

### 4b. The bloat is serde, in one crate

editor-core's top eleven entries are all serde/serde_json derive
machinery — `deserialize_struct`, `visit_enum`, `serialize` — at **11–16
copies each**. Only 490 of its 12,591 symbols mention `f64` at all. This
is the one place in the workspace matching the classic monomorphization
signature, and it has nothing to do with `Real`.

### 4c. Error `Display` impls cost more than `Probe`

| | IR lines | share of workspace |
|---|---|---|
| all `fmt::Display>::fmt` impls | **77,400** | **7.7%** |
| all `Probe` instantiations | 36,227 | 3.6% |

topo's five largest functions are the `Display` impls for
`ValidationError`, `BooleanError`, `EulerOpError`, `SplitJoinError`,
`SplitReduceError` — **26,077 lines, 27.4% of that crate's IR, one copy
each**, more than `f64` (16,574) and `Probe` (14,779) *combined*.

`ValidationError` has 57 variants; its `Display` impl is 310 source lines
producing 7,328 IR lines, ~130 per match arm. Each `write!(f, "…{a:?}…")`
expands to `format_args!`, which builds a `&[&str]` piece array plus a
`&[core::fmt::rt::Argument]` array where every interpolated value gets an
`Argument::new_display`/`new_debug` construction carrying a formatter
function pointer.

**This is reported, not flagged as a defect.** It is the D9 fail-loud
charter working as specified — every failure is a typed error, and these
messages cite the design clauses they enforce. Because it is one copy
each, it is code volume, not monomorphization: none of the standard
remedies (outlining, `dyn` erasure, collapsing the type product) apply.
The only lever is fewer interpolated arguments per arm, paid for in
diagnostic quality.

### 4d. The `interval` feature's marginal cost

| crate | default | `--features interval` | delta |
|---|---|---|---|
| geom-brep | 106,897 | 130,739 | **+22.3%** |
| topo | 95,033 | 111,453 | **+17.3%** |

Well short of the doubling the `f64`-vs-`Interval` framing implies,
because most of the code is not `Real`-generic in the first place.

The feature is also **well-behaved**: geom-brep's delta (23,842) is
*exactly* the `Interval` attribution row, and `Probe`'s 21,342 is
byte-identical across both builds. Enabling it adds precisely its own
instantiation set and perturbs nothing else.

Per-scalar, apples to apples in the interval build:

| scalar | geom-brep | topo |
|---|---|---|
| `f64`\* | 30,796 | 18,005 |
| `Interval` | 23,842 | 16,420 |
| `Probe` | 21,342 | 14,779 |

**`Probe` and `Interval` cost within ~11% of each other** — each is one
full monomorphization of the same bodies. `Interval` is slightly larger
because its directed-rounding arithmetic expands more per operation;
`Probe` delegates to `f64` and comes out near `f64`'s size. *The `f64`
column greps any symbol mentioning `f64`, so it also catches const-generic
dimension variants and plain non-generic helpers — treat it as an upper
bound.*

### 4e. Cross-crate codegen is real but tiny

`geom-surfaces` compiles to **192 IR lines and 2 copies** — of which 191
are a single `Display` impl. Everything else in the crate is generic and
deferred to its consumers, exactly the cross-crate-codegen story the
brief predicted. It is simply too small to matter.

---

## 5. Mechanism: what the data supports, and what it rules out

**Supported.** The build/check delta is codegen. Editing the generic core
costs +13 s over a leaf edit because instantiations are re-codegen'd
downstream.

**Ruled out — monomorphization bloat.** Copy counts are 1–3 everywhere in
the geometry crates. There is no outlining candidate (brief §4a), no
closure-per-call-site explosion (§4b), and no exploding type-parameter
product (§4c) — the scalar has one inhabitant in default builds.

**Ruled out — trait solving.** `cargo check --workspace --all-targets`
after invalidating the root of the DAG is **6.0 s**. No `-Zself-profile`
run was needed.

**Ruled out — the generics being the top cost at all.** 62% of workspace
IR is serde in editor-core; error `Display` impls are 7.7%; all `Probe`
instantiations are 3.6%.

**The actual dominant term was test execution, not compilation.** Compile
work was ~21% of CI run wall and ~18–23% of billed job time. The critical
path was `filter` → `build-interval` (218 s) → `test (interval,
eps=default, 2/2)` (**828 s**) → `cleanup`, and that 828 s job compiles
nothing.

---

## 6. What landed, with before/after

### #449 — opt-level 2 on the two archive jobs

Reverses the #52/#53 opt-0 verdict, whose two premises had expired
(#179/#387 took the workspace from 261 test binaries to 14; test
execution became ~79% of wall).

| | before | after |
|---|---|---|
| `build + archive (default)` archive step | 127 s | 432 s (3.40× worse) |
| `build + archive (interval)` archive step | 132 s | 605 s (4.58× worse) |
| `test (interval, eps=default, 2/2)` | 828 s | **117 s (7.1× better)** |
| ten test legs, summed | 5620 s | 895 s |
| **critical path** | ~1065 s | **~840 s (−21%)** |
| **billed** | ~137 min | **~72 min** |

`debug-assertions` and `overflow-checks` are unaffected by opt-level
(cargo defaults them ON for dev/test), so fail-loud keeps its teeth.
2791/2791 and 3001/3001 green at both opt levels locally.

### #450 — interval clippy + doc-tests off the critical path

`test-interval` declares `needs: build-interval`, and `needs` waits for
the whole **job**, not the artifact it consumes — so the run's longest leg
sat behind a clippy pass it has no dependency on.

| | before | after |
|---|---|---|
| `build + archive (interval)` job | 704 s | **656 s** |
| critical path | ~840 s | **~789 s** |

### Two estimation errors worth remembering

Local projections said 3.24× compile penalty and 2.29× interval execution
win, forecasting −16% wall. Hosted reality: **4.58×** and **7.1×**.

* The compile penalty is *worse* hosted — 2 vCPU parallelises opt-2
  codegen badly.
* The execution win is *much better* — the local 2.29× was taken with an
  express-lane job running on the same box, so it was pessimistic.

They cancelled favourably. **The local:hosted ratio is not 1.**

### The cache trap that nearly produced a wrong conclusion

`Swatinem/rust-cache` caches **dependencies, not workspace crates**, and
hashes `CARGO_*`/`RUST*` env into its key. So any knob change buys one
cold rebuild, and on #449's first run the wall was **18m22s — worse than
the ~17.8 min baseline**, while every check passed green. Merging on "all
checks pass" would have shipped a claim the data contradicted at that
moment. The warm rerun showed the real −21%.

**A first run after an env change is never the verdict.**

---

## 7. Ranked recommendations, and what was deliberately not done

**Landed:** #449 (opt-2), #450 (lint split), #451 (retire mold).

#451's basis: #174 adopted mold on a −24%/−20% result across **261**
static links; there are now **14**, and `LOCAL-BUILD-PERF.md` had already
measured it as noise locally (189 s vs 186 s) and said "revisit only if
the test-target count grows back toward triple digits." Measured on its
own warm CI runs: **archive step 620 s → 625 s (+0.8%, within noise)**,
`install mold` **17 s → 0 s**. So the link penalty is nil and the install
is genuinely removed — but a single sample put the job totals only 4 s
apart, so the defensible claim is "no measurable link penalty, one fewer
system dependency on the critical path", not the full 17 s.

**Deferred deliberately — resharding.** 2 → 4 shards modelled at −8.2 min
wall for +7% billed, and the current 2-way split is structurally
imbalanced (10 of the 12 slowest tests land in shard 2, identically every
run, because `--partition count:N/2` is deterministic by list position and
reads no timings). **But opt-2 changed the premise**: legs are now ~117 s
against a measured 15 s per-leg fixed cost, so more shards buy much less
and cost more. Re-measure before acting. Floor for any scheme:
`step-import::all rw2_probes::probe_round_trip_bit_identity_and_reorder`
at 296 s pre-opt-2.

**Gating `Probe` — DONE**, behind a `probe` cargo feature. Measured
before: 20.0% of geom-brep's IR, 15.6% of topo's, 3.6% of the workspace
(editor-core's serde swamps it), and **5.0% of the workspace's
test-binary symbols**, which is the share that predicts a CI saving
because the archive step compiles test binaries.

Measured after (release `--lib`, default features):

| crate | before | after | delta | with `--features probe` |
|---|---|---|---|---|
| geom-brep | 106,897 | **85,360** | **−20.1%** | 106,527 |
| topo | 95,033 | **80,176** | **−15.6%** | 94,830 |
| geom-core | 50,655 | 49,786 | −1.7% | — |

Residual `Probe` symbols with the feature off: **0**. With it on, totals
return to within 0.3% of the originals — the code is opt-in, not lost.

**The wall-clock win did NOT survive measurement, and that is the
honest result.** On the gating PR's own CI run the interval archive step
read 625 s → 444 s and the default 432 s → 519 s — *opposite directions*.
Removing ~5% of test-binary symbols should move both lanes by about the
same small amount; a −29%/+20% split is run-to-run variance. CI's
archive-step noise band at single-sample resolution is wider than the
~25 s the census predicts, and more samples cannot resolve a 25 s effect
inside a ±100 s band. Per this investigation's own rule — *a change that
reduces IR but not wall-clock has not accomplished anything* — the
build-time case for gating Probe is **unproven**.

It was landed anyway, on the justification that does not depend on
timing: `Probe` is a diagnostics scalar, and before the gate the python
wheel, both demos and every release render compiled it in. `render.yml`
builds `demos/tour` at `--release` twice per run and never invokes
`k-probe`. That is an argument about what ships, not about build speed,
and it is the one that carries the change.
The symbol-attribution method predicted 85,555 and 80,254, so it was
accurate to 0.2%; that is worth knowing, because it means the census in
§4d can be trusted to size this kind of change *before* doing it.

Shape of the change, for anyone doing something similar:

* It is a cargo feature like `interval`, **not** a `cfg(test)` — `Probe`
  is wired into production `src` through the three crates that carry a
  production `cfg(feature = "probe")` arm (`grep -rl 'cfg(feature =
  "probe")' crates/*/src` also lists `geom-brep`, whose one arm is a
  `#[cfg(test)]` wiring row): `geom-core` (the scalar impls, the
  span-locate `Sealed` marker and `bit_identity.rs`'s downcast), `topo`
  (`AtRestPolicy`) and `editor-core` (`ContentBits` and its own lane
  traits). The quadrature lane is a `topo::QuadLane` value, the
  chart-region lane a `topo::RegionLane` value, the fitted-pcurve lane
  a `geom_brep::FittedLane` value and the plane × NURBS lane a bound,
  none a trait, so none of the four adds a crate to that list.
* **`k_stats::decide` and the `CURRENT` thread-local stay ungated.** That
  funnel is the path every shipped decision takes, and it must be
  byte-identical with the feature on and off (D9). A `cfg` there would
  have made the production decision path differ between build
  configurations — the one change this gate must not make.
* `docs/DESIGN.md` Q1 ratifies the instantiation set as "`f64`,
  `Interval`, `Dual<f64>`, `Dual<Interval>`" — **`Probe` is not in it**,
  so the gate reopened no ratified decision.
* The consumer is `scripts/k_probe_sweep.sh` (feeding the live `k-lint`
  gate against the committed baselines in `docs/k-report-data/`); it opts
  in. `demos/tour` gained the feature too, gating its `k-probe` mode —
  that is where a real share of the win is, since `render.yml` builds the
  tour at `--release` twice per run and never invokes it.

**Two traps this hit, both worth repeating.** Selecting files by name or
by grep count is wrong twice over: `step-import/tests/rw2_probes.rs` says
"Probe" only in prose, and gating it would have removed the suite's
single slowest test (296 s) from the default lane; and of the 13 test
files that genuinely use the scalar, only 6 were Probe-dedicated — the
other 7 were mixed (`review_m2_pr2.rs` is 21 tests of which 2 touch
Probe). Gating a mixed file whole **silently deletes default-lane
coverage**. They were split instead, with counts conserved on both sides.
One test resisted even that: `topo`'s
`r5_crossing_vertex_on_is_declared_not_measured` has a Probe block inside
a test whose other parts are f64 claims, so the *block* carries the
`cfg` — precedent at `geom-core/tests/spline_hull.rs:440`.

**Not recommended — merging the default and `interval` lanes.** Tempting
(one feature resolution, one archive, one build job) and the *original*
reason for the gate is gone: M5 PR 1 replaced inari/gmp with the in-repo
pure-Rust backend, so the feature no longer drags a C toolchain or LGPL
code. But:

1. It is a billed win and a **wall loss** — the two build jobs run in
   parallel, so the path is `max(127, 132)`, not their sum.
2. You would not remove a configuration, you would rename it. The
   f64-only build has live consumers (`python-suite`, both demos) and
   keeping it tested costs back what you saved.
3. `docs/DESIGN.md` Q1 ratifies `Interval` as living behind the feature.
4. `ring_interval.rs`'s docs lean on the gate literally ("the module does
   not exist in a default build") to keep `Interval` (a `Real`
   instantiation) distinct from `RingInterval` (certification substrate).
5. It deletes none of the ~90 interval-gated test files — the split's cost
   is a test cost, not a build cost.

---

## 8. Corrections to existing documents

* **`local-scripts/test-fast.sh`'s header claims 15×** ("75 s → 4.9 s,
  807 tests", 2026-07-21). Measured 2026-08-12: **4.30×** default,
  **2.29×** interval locally. The suite has grown to 2791 tests and
  558.7 s at opt-0 — 3.5× the test count but 7.4× the wall, so the tests
  added since are less compute-bound. That header is stale.
* **"Interval is ~1.0× slower than f64 at runtime"** — derivable from
  opt-0 CI legs, and an artifact of the unoptimized build. At opt-0 the
  interval lane is 1.24× the default lane (691.7 s vs 558.7 s); at opt-2
  it is **2.32×** (301.6 s vs 130.0 s). opt-0's own overhead masks the
  enclosure cost regime that `geom-core/Cargo.toml` has always claimed.
  Anyone sizing interval-vs-`f64` runtime off an opt-0 leg is reading a
  flattered number.

---

## 9. Addendum — the TYPE and the INSTANTIATION priced apart (2026-09-21)

§4d above measures the feature as one thing. It is two, and they price
differently: the `geom_core::interval` MODULE (the scalar, its
arithmetic and its `Real`/`Decide`/`Bounds`/`SpanLocate` impls, over the
`interval-transcendentals` backend) and the kernel's INSTANTIATION at
that scalar (the lane impls in the crates above `geom-core` and the
interval test files).

### The 2026-09-15 battery

Method as in "Reproducing" below, with the CI build job's env block
(`CARGO_PROFILE_{DEV,TEST}_OPT_LEVEL=1`, `line-tables-only` debug,
`CARGO_PROFILE_TEST_STRIP=debuginfo`), `CARGO_INCREMENTAL=0` and a fresh
target directory per row on a 4-vCPU box. Min of two runs. Another lane
was compiling on the same box throughout, so the rows carry that noise
and the `--tests` deltas are ±50 s at best.

| # | measurement | run 1 | run 2 | **min** | target |
|---|---|---|---|---|---|
| 1 | `build --workspace`, default (baseline) | 107.3 | 112.2 | **107.3** | 410M |
| 2 | `build --workspace`, type ungated, instantiation still gated | 105.6 | 107.9 | **105.6** | 410M |
| 3 | `build --workspace --features interval` (= the feature dropped) | 155.6 | 145.9 | **145.9** | 551M |
| 4a | `build --workspace --tests`, default | 617.3 | 567.7 | **567.7** | 2.1G |
| 4b | `build --workspace --tests`, type ungated | 578.0 | — | **578.0** | 2.1G |
| 4c | `build --workspace --tests --features interval` | 631.2 | 771.5 | **631.2** | 2.4G |
| 5a | `build -p geom-core`, default | 8.0 | 9.0 | **8.0** | 34M |
| 5b | `build -p geom-core`, type ungated | 8.3 | 8.7 | **8.3** | 34M |

Against the baseline: the **type** is **−1.7 s (0%, noise)** on the lib
build, +10 s (+2%, one run, inside the noise) on `--tests`, +0.3 s
(+4%, noise) at `-p geom-core`, and +0 target either way. Dropping the
**feature** is **+38.6 s (+36%)** on the lib build, **+63.5 s (+11%)**
on `--tests`, +141 MB / +0.3 GB of target.

The dependency graph (`cargo tree --workspace -e normal --prefix none |
sort -u | wc -l`) goes **84 → 85** when the type is ungated, and the one
addition is `interval-transcendentals`: in-repo, `libm`-only, and its
own gmp-backed oracle is a dev-dependency of *that* crate, so it enters
no kernel build.

So the gate's cost is the instantiation, which is what §3 and §4d
already showed from the other side (`Interval`: 0 symbols instantiated
in a default build; +22.3%/+17.3% of LLVM IR in geom-brep/topo with the
feature on). The type was ungated on that basis.

### At the landing (2026-09-21)

**No local re-take is recorded here.** `memories/local-battery-scope.md`
puts committed measurements on hosted CI, one reproducible box class,
every sample with its own environment block. The landing box (a shared
container: 4 vCPU, 15 GB RAM, rustc 1.97.0, `CARGO_INCREMENTAL=0`, the
CI build job's opt-level-1 profile env, one-minute load 4.5–10 with two
other lanes compiling) read 153 s on the default `build --workspace`
against the battery's 107 s above: a 43 % slower baseline is a
different regime, not the same noise, so the two cannot be differenced,
and a second local box class in this doc would be that rule bent
twice.

The hosted numbers instead, from this branch's two full runs: run
**35555608396** (head `cf15f29bc5`) and run **35559102712** (head
`30ca6a8697`), whole-run wall 60 min and 28 min, with `build + archive
(default)` **418 s** and `build + archive (interval)` **506 s** on the
later one. Both are single samples over whatever rust-cache state the
runner restored, so they bound nothing finer than "no gross
regression", and neither is a measurement of the type: for that, the
2026-09-15 battery above is what this doc has.

Symbol level, reproducible at any head: `nm --defined-only
target/debug/deps/libgeom_core-*.rlib | grep ' T ' | grep 8interval |
wc -l` over a default `cargo build -p geom-core` answers **44**, where
a gated module answered 0. Forty-three of those are
`geom_core::interval`; the forty-fourth is a `PartialOrd` impl for the
backend's `Decoration`, which the module filter matches too — which is
why the filter and the number are written down together. The consuming
crates' `Interval` counts stay at zero without the feature. §3's table
carries this split in its own row rather than being reinterpreted from
here.

Two of §7's five reasons NOT to merge the lanes were about the type and
have gone with it: `docs/DESIGN.md` Q1 no longer says the scalar lives
behind the feature (it says the feature gates the lane-trait impls and
the interval test files), and `ring_interval.rs` no longer leans on the
module being absent from a default build. The other three — the
wall-vs-billed asymmetry, the f64-only build having live consumers, and
the **126** interval-gated test files being a test cost rather than a
build cost — are untouched by this, and row 3 above is what merging the lanes would cost today.

---

## 10. Addendum — the C9 ring as a newtype over the backend (2026-09-21)

`geom_core::RingInterval` is now `RingInterval(DInterval)`: the
certification substrate's surface over `interval-transcendentals`'
arithmetic, with the backend's decoration as its poison channel. §9
priced the interval TYPE and the kernel's INSTANTIATION at it; this
prices the ring's own body, which is the third thing in that family
and the smallest.

**No dependency is added, mechanically.** The ring names
`interval_transcendentals` from `src` where it previously named
nothing but `core`, and `geom-core` already depends on that crate
unconditionally (RING-1, cut (i)). `cargo tree --workspace -e normal
--prefix none | sort -u | wc -l` answers **85** on this head, which is
§9's post-ungating number unchanged — the newtype reaches an edge that
was already there.

**The build delta is noise, as §9's row 5 predicted.** One run each on
the landing box, the ring's file swapped and nothing else, a fresh
target directory per row:

| measurement | seconds | target |
|---|---|---|
| `build -p geom-core`, the retired ring | 8.86 | 49 MB |
| `build -p geom-core`, the newtype | 8.99 | 49 MB |

**+0.13 s, +1.5 %, +0 MB** — inside the ±4 % §9 measured between the
same two shapes of this row, and a single sample either way.

**Not a committed measurement of record.** The box is a shared
container (4 vCPU, 15 GB RAM, rustc 1.97.0, `CARGO_INCREMENTAL=0`,
`CARGO_PROFILE_DEV_OPT_LEVEL=1`, one-minute load 0.4 at the first row
and 0.4 at the second, no other lane compiling), and
`memories/local-battery-scope.md` puts committed timings on hosted CI
with one reproducible box class. What this row is for is the negative
claim the unit had to check — that making the ring a newtype costs no
dependency and no measurable compile time — and one local pair at an
idle box answers that without pretending to be a benchmark.

---

## 11. Addendum — the `interval` feature dropped (2026-09-24)

H5 ruling 1 cut (iii) (PR 2701) drops the `interval` feature at a measured
ceiling of ~+36 % clean workspace build and ~+11 % test compile (§9 rows 3
and 4c). RING-4 is that drop, and it measured first, against the cost gate
its spec set (+50 % hosted build+archive over the default lane, beyond which
the call returns to Ev).

**Hosted, the reading of record** — the `build + archive` jobs of the three
most recent green code-tier `pull_request` runs before the change, job wall
from the Actions jobs listing and the archive step alone from the step
listing. One sample per run, sccache off, whatever rust-cache state each
runner restored:

| run | default job | interval job | Δ job | default step | interval step | Δ step |
|---|---|---|---|---|---|---|
| 35947793407 | 5.3 min | 5.6 min | +6 % | 286 s | 311 s | +9 % |
| 35947275144 | 6.6 min | 9.1 min | +38 % | 363 s | 509 s | +40 % |
| 35946382951 | 4.2 min | 6.4 min | +52 % | 219 s | 359 s | +64 % |
| **sum of three** | **16.1** | **21.1** | **+31 %** | **868** | **1179** | **+36 %** |
| sum of ten (the seven before them too) | 57.5 | 66.4 | +15 % | 3099 | 3701 | +19 % |

The six test rows each way, same three runs (job-minutes, summed over the
six legs): default 4.7 / 10.7 / 5.1, interval 28.7 / 38.7 / 31.7. The
interval rows run the WHOLE suite at the certified scalar; the default
rows are the part a run stops paying.

The gate reads the three-run aggregate — +31 % job wall, +36 % archive
step — which is inside the ratified ~+36 % and under +50 %; one of the
three samples reads +52 % / +64 % on its own, and the ten-run window
around it reads +15 % / +19 %, which is the spread a single sample of a
single job on a shared runner fleet carries. **Go.**

**Local, one run each, not a measurement of record** (the box: 4 vCPU,
15 GB, rustc 1.97.0, `CARGO_INCREMENTAL=0`, the build job's opt-level-1 and
`line-tables-only` env, a fresh target directory per row, the machine-wide
build slot held; another lane's test battery ran throughout, one-minute
load 1.8 → 8):

| row | seconds | target | load before → after |
|---|---|---|---|
| `build --workspace --lib`, default | 111.5 | 422M | 1.8 → 5.8 |
| `build --workspace --lib --features interval` | 216.7 | 566M | 5.8 → 8.0 |
| `build --workspace --tests`, default | 813.0 | 2.2G | 8.0 → 7.8 |
| `build --workspace --tests --features interval` | 894.9 | 2.6G | 7.8 → 6.5 |

+94 % on the libs and +10 % on the tests, under a load that doubled
between the two lib rows — the direction §9 measured (+36 % / +11 %), with
the lib row's size set by the load as much as by the feature. Committed
timings come from hosted CI (`memories/local-battery-scope.md`); this table
is the spec's one local pair and nothing reads it.

**What the drop changes in CI.** One compile mode, so one archive: the
interval lane's build, lint and test rows were already the superset of the
default lane's, and their content now runs under the default rows' names
(`build + archive`, `clippy`, `test (eps = …, n/2)`), while the default
rows' separate f64-only runs are gone.

What a code-tier run costs after it, read on run 35979996296 (head
`bafb21a81f`, every job green): **136.8 job-minutes**, against 133.2 /
148.0 / 137.6 on the three runs of the table above. Its render lane's tour
step was served from the tour-output cache (the same inputs had been walked
on run 35978891191 minutes earlier, 7.6 job-minutes); uncached it is about
143.8. So the fold's saving — the default lane's build, clippy and six test
legs — is roughly spent again, not banked: the certified code now compiles
into every build, the release tour builds included. The tessellation-budget
sweep step reads 116 s on base run 35947793407 and 217 s here, of which
203 s is the release build (`pncad` and `demo-tour` the long units) and
13 s the walk; the render lane's `demo tour` step reads 116 s at the base
and 191 s on 35978891191, its build included and not split out. Neither
walk narrates the certified cells: they
run in `demo-tour certified`, which only `demos tour suite` invokes.

The critical path is `k-lint (gate, release-default)`, which needs only
the change filter: 29.5 min on base run 35947793407 and 26.7 min here
(its `demos tour suite` step 1569 s), against `build + archive` plus the
slowest test leg at 9.0 + 15.2 = 24.2 min. The fold did not move it.

---

## Reproducing

The measurement scripts are not committed — they are throwaway harnesses,
and the commands they wrap are short enough to restate:

* baselines — `cargo clean && cargo build --workspace --all-targets`,
  with a real appended item (not `touch`) for the incremental rows
* symbol census — `nm -S --defined-only target/debug/deps/lib<c>-*.rlib |
  grep ' [Tt] ' | awk '{print $NF}' | rustfilt`, then group by name with
  generic arguments erased
* IR census — `cargo llvm-lines -p <crate> --lib --release` (the rows
  above that read `--features interval` were taken while that feature
  existed; §11)
* execution — `cargo nextest run --workspace`, with
  `CARGO_PROFILE_{DEV,TEST}_OPT_LEVEL=2` for the opt-2 rows

**Use nextest, not `cargo test`.** Several suites need process-per-test
isolation (`Tolerance` is a `OnceLock`); `cargo test --workspace` fails on
`tolerance_init` for that reason. #448 fixed that specific case by
self-re-exec, but nextest remains the harness CI uses and the one these
numbers were taken under.

All heavy rows must go through `local-scripts/with-build-slot.sh -x`, and
a full battery holds the machine-wide mutex for the better part of an
hour — announce it before starting.
