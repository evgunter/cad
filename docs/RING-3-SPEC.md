# RING-3 — `RingInterval` dissolves into `Interval`: the ring's refusals become named `Interval` doors, `Enclosure` and `crossing_bracket` go, C9 and DL4 say what is true

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-24).** Binds
the implementer of unit RING-3; deleted at merge with a note under
`docs/doc-ledger/`. Read `docs/prompts/implementer-discipline.md` in full
first. The item is `work/scalar/ring-3-ring-dissolves-into-interval.md`;
the ruling is `work/scalar/H5.md` §RATIFIED ruling 1 cut (iii) (PR 2701)
and ruling 2 (a tighter certified bound re-baselines). RING-0, RING-1 and
RING-2 are the precedents (their items' §Closed). The survey with every
citation is `/home/user/scalar-briefs/survey-ring3.md` (taken at
`a7c5f611cb`); re-derive every count at your merge base.

**The split (orchestrator, 2026-09-24).** Ruling 1 (iii) is one cut on
paper: the newtype dissolves AND the `interval` feature drops, with C9,
Q1 and DL4 re-worded in the PR that reaches them. Measured (survey), it is
829 `src` references in 25 files plus the feature's 55 `src` cfg sites,
249 gated test files, 16 manifests and a CI lane collapse. It is cut in
two units: **this one (the kernel dissolution; C9 and DL4)** and **RING-4
(the feature drop; Q1 and `DESIGN.md:266`; CIW's lane axis)**. RING-4
depends on this one and not the reverse: after RING-3 the kernel's
certification code runs on `Interval`, which already compiles in every
build (RING-1), so the feature keeps gating only what it gates today.

## 0. The finding, and what the tree says

Both types wrap `DInterval`; `+ − × ÷ neg`, `sqr`/`powi`, `point`/`zero`/
`one`/`from_bounds` make the same backend calls (survey §1), so the
arithmetic's bits cannot move. What the ring has and `Interval` lacks is
its REFUSAL surface: `is_poison` (`dec < Def`) where `Real::is_poison` at
`Interval` means NaI‖empty; `hull` refusing a poisoned operand (NaI) where
the backend's hull treats empty as identity; `clamped_to` over the
endpoints where the backend's `intersection` caps at `Trv`; `width`
padded up one step and NaN on refusal; `mag` NaN on refusal; `contains`
false on refusal; `poison()`; and `from_certified`'s `Def`/`Trv` cap
through the `crossing_bracket` member RING-2 added. A blind swap converts
refusals into passes on exactly the cases the ring was specified to
refuse (H5 S1's steelman) — **the unit keeps every one of them, by
construction**.

## 1. What this unit delivers

**The ring's refusal surface lands on `Interval`, as inherent methods,
with the ring's bodies.** In `crates/geom-core/src/interval.rs` (or a
`certify` submodule of it — say which): `hull`, `clamped_to`, `width`,
`mag`, `contains` and a NaI constructor, each with the ring's exact body
(read the decoration; never route `clamped_to` through `intersection`,
never drop `width`'s `up1` pad or its NaN). **The refusal predicate gets
a name that is not `is_poison`**: an inherent `Interval::is_poison` would
shadow `Real::is_poison` on concrete receivers only and give one name two
meanings. Use the existing `Interval::is_certified` (`interval.rs:229`)
and rewrite every ring `.is_poison()` read as `!x.is_certified()` (or a
second inherent name, if the negation reads badly at 104 sites — say
which and why). **Three consumer files import `Real`** (`topo/src/props.rs`,
`geom/src/curves/nurbs.rs`, `geom-brep/src/ssi/certify.rs`): there a
missed rename would compile and resolve silently to `Real::is_poison`;
audit them line by line and pin each (§3).

**`RingInterval` is deleted.** Every `src` and test site names `Interval`
(survey §5: 829 `src` references in 25 files, 250 in 41 test files —
re-derive), `crates/geom-core/src/ring_interval.rs` goes (its unit tests
re-home to `interval.rs`'s test module or a test file, keeping every
assertion), the `lib.rs` re-export goes. `from_certified<T>` becomes an
`Interval` associated function with the ring's `Def`/`Trv` cap and its
bound moves from `CertifiedEnclosure` to `CertifiedBounds` reading
`Bounds::lo/hi` — the member-free route the survey found (§2): a sole
`CertifiedBounds` bound does not fire `bounds-allowlist.sh`
(`real.rs:1350-1358`). **Compile it before you trust it**: the survey
inferred ~47 header edits (`hull.rs` 24, `ssi/certify.rs` 9,
`topo/src/props.rs` 10, `ssi/enclose.rs` 4) and did not build them. If
the gate or the compiler refuses, stop at the count and say what refused.
**`CertifiedEnclosure::crossing_bracket` goes** with its five bodies
(`real.rs`, `interval.rs`, `ring_interval.rs`, `sym.rs`, `k_stats.rs`)
once `from_certified` no longer reads it. **`Enclosure` goes**: the
trait, `impl<T: Bounds> Enclosure for T`, the ring's impl, the re-export;
its two test sites (`spline_hull.rs:40,480-508`, `bounds_census.rs:173-176,
:408`) move to `Bounds` or are deleted with the reason. No `src` bound
names it (survey §2).

**The crossing survives** for `T ∈ {f64, Probe, Sym<_>}`: 78 generic
`from_certified` calls remain. `ring_interval.rs:171-174`'s "no crossing
left to make" was false; the doc that replaces it says what the crossing
is for.

**What must not change:** every endpoint bit on every corpus
(`coeffs_bit_identity{,_ext,_interval}`, `span_bit_identity{,_ext}`, the
three sweep digests, `step-export/tests/kernel_sidecars.rs`,
`docs/tess-budget-data/`); every verdict and refusal (the ring's refusal
rows in `certified_door.rs`, `ring2_r2_probes.rs`, the endpoint census, the
ring's own unit tests, the poison rows); the K roster (no `decide(`
moves); `Real`'s semantics at `Interval` (the evaluation scalar is
untouched — `Real::is_poison` stays NaI‖empty). If ANY bit moves, it is a
finding: stop, characterise, file — this unit is bit-preserving by
construction and a move means a body was not carried faithfully.

**The census re-keys.** `ring_endpoint_census.rs` walks files whose text
names `RingInterval` (`:249`) with a `total > 60` floor; once the name is
gone its population is empty and it reds. Re-key it to the certification
reads (the files that call `from_certified` or the refusal doors, or a
fixed roster — say which and why), keep the per-file read/asking counts
and dispositions, and keep it red-able (plant an unguarded read; it
reds).

**The suites re-home.** `ring_interval_differential.rs`'s ring lane
becomes tautological: its `Interval`-vs-`DInterval` lane and the
subnormal/overflow pins (`:713-753`) stay, re-keyed; `ring_interval_fuzz.rs`
(soundness against exact arithmetic) re-keys to `Interval`. Rename the
files if their names lie; roster any renamed file in the ledgers the gates
read.

## 2. Docs — C9 and DL4 are Ev's text, and this PR waits for Ev

- **C9** (`crates/geom-brep/README.md:252-266`, companion row
  `DESIGN.md:22`, Ratified #85). RING-3 **retires decisions** there: "not
  a `Real`", the two-roles split, `is_poison()` as the refusal's name,
  `Enclosure`. Re-write the clause to what is true after the unit (the
  certification substrate IS the evaluation scalar `Interval`, its refusal
  is the decoration read as `!is_certified()` and the named refusal doors,
  the backend and no-copyleft sentences surviving). Cite each sentence's
  `git log -S` writer on the full history and say where it bottoms at a
  graft (`.git/shallow`).
- **DL4** (`docs/DUAL-DESIGN.md:117-128`, Ratified PR #1146). **It is not
  one gate line** (survey §4): `bounds-allowlist.sh` matches any
  identifier ENDING in `Enclosure`, and that suffix is what catches the
  `CertifiedEnclosure` compounds — dropping `Enclosure` from the matchers
  loses 24 pinned occurrences across 6 ratified entries. Delete only
  what is `Enclosure`'s own: the three plant rows (`:1254-1272`), the
  header paragraph (`:44-56`), the selftest prose (`:1716`), and keep the
  suffix match (or replace it with an explicit `CertifiedEnclosure`
  matcher — same counts, prove it by running the gate before and after).
  Every ratified per-file count that moves (the `CertifiedEnclosure →
  CertifiedBounds` headers change what the gate counts) is re-derived and
  stated with its entry's reason. `real.rs`'s `bounds_allowlist` ledger
  naming `RingInterval::from_certified` and "the C9 ring" (`:1123,1136,
  1139`) is re-worded naming-only.
- The PR is titled `[ev] RING-3: …`, the item carries `needs_ev: true`,
  and the PR body opens with a decision section for Ev: the C9 and DL4
  sentences before and after, one line each on what decision retires.
  Per CLAUDE.md "Asking Ev": no status scaffolding in the diff.
- Also: `ring_interval.rs`'s module doc's content that describes the
  design moves to C9 or `interval.rs`'s doc (present tense);
  `ssi/certify.rs:1011` ("`RingInterval` has no decoration channel"),
  `interval-transcendentals/src/ops.rs:12`,
  `interval-transcendentals/README.md:11` re-worded; `docs/GENERICS-BUILD-COST.md`
  dated addendum if the build moves measurably (one run, stated as such).
- **Not this unit**: Q1, `DESIGN.md:266`, the `interval` feature, CI —
  RING-4's.

## 3. The pins

- **Each refusal door, red-first.** For each of `hull`, `clamped_to`,
  `width`, `mag`, `contains` and the refusal predicate: replace its body
  with the backend's semantics (`hull` → the backend hull; `clamped_to` →
  `intersection`; `width` without the pad; `mag`/`contains` ignoring the
  decoration; the predicate → `Real::is_poison`) and show which rows red;
  restore. Every door must have at least one red row — a door with none
  gets one (a `Trv` operand through it at a real consumer site).
- **The three `Real`-importing files**: one row each that drives a
  refused bracket through the renamed predicate at that site and asserts
  the refusal — the rename cannot be silently undone there.
- **`from_certified`'s cap**: a `Trv` crossing refuses at every door
  (RING-2's `a_refusal_can_carry_real_endpoints` rows re-homed).
- **Bit identity**: every corpus in §1 green unchanged, run at default and
  `--features interval`; `CAD_FUZZ_EFFORT=20` differential with
  `CAD_FUZZ_SEED` quoted.

## 4. Sweep

The class: every `RingInterval` site (by file, with its read shapes —
survey §5's counts), every `Enclosure` and `crossing_bracket` site, every
`.is_poison()` on a certification value (distinguish from evaluation-scalar
calls by the receiver's type — the compiler's rename errors list most;
the three `Real` files are the rest), dispositioned in the PR body by
file and count. The prose-shaped second pass (`ring`, `the ring's`,
`certification substrate`) for sentences that describe the retired type
without naming it.

## 5. Fence

This program claims no paths. This unit reaches PROPS
(`crates/geom-core/src/{interval,ring_interval,real,lib,sym,k_stats}.rs`,
`spline/*`, `props/*`, `geom/src/*`, `DUAL-DESIGN.md`), SSI
(`ssi/{certify,enclose,exhaust}.rs`), ENCL/OFFSET/SHELL (`offset_fit.rs`,
`patch_bound.rs`, `offset_meters.rs`), CHORD+TESS (`mesh/src/*`), EXCH
(`step-import`, doc only), GUARD (`bounds-allowlist.sh`), TCOST/TINT (the
test files), the unowned `topo/src/props.rs`, `crates/geom-brep/README.md`
and `DESIGN.md` (none of its text changes here). `interval-transcendentals/`
is in no program's paths: prose only. Not CI, not manifests, not the
feature. Announced by the orchestrator on the PR and in `work/scalar/log.md`.
Merge `origin/main` immediately before opening the PR; `python3
scripts/work.py territory --base origin/main` in the PR body.

## 6. Verification and report

Local: `cargo nextest run` per crate (`geom-core`, `geom`, `geom-brep`,
`mesh`, `topo`, `sweep`, `step-export`) at default and `--features
interval`, never `--workspace`; the three sweep digests at
`CAD_TOLERANCE_EPS` 1e-6 and 1e-12; the tess-budget `--sizing-only`
recipe + `tools/tess-lint`; `cargo check --workspace --all-targets` at
default and `--all-features`, `cargo check` in `demos/tour`; `cargo
clippy` on the touched crates `--all-targets -- -D warnings` at default
and `--all-features`; `cargo fmt --all --check` plus `--check` in
`demos/tour`, `demos/wild`, `benches`; `bounds-allowlist.sh` (+
`--selftest`), `interval-square-allowlist.sh`,
`check-interval-cfg-additive.py`, `probe-suite-census.sh`,
`scripts/doc-gate.sh`; `python3 scripts/work.py lint`. Private
`CARGO_TARGET_DIR`, `CARGO_INCREMENTAL=0`; delete it before reporting;
scratch under `/home/user/scalar-ring3-scratch/`. Hosted CI is the
verification of record; poll it to conclusion in the foreground and
report the run id. Report ≤ 120 lines: the refusal doors as landed and
where, the predicate's name, `from_certified`'s bound and the header
count, what went (`RingInterval`, `Enclosure`, `crossing_bracket`) and
what re-homed, the three `Real` files, the pins with red-first evidence
per door, the census re-key, bit identity per corpus, C9 and DL4 before
and after with `git log -S` writers, the gate's counts before and after,
deviations, rows filed.
