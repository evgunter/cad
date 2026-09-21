# LANE-2 — `ChartRegionLane` deleted: the chart-region doors are one `Option<RegionLane<T>>` parameter through the census, `None` keeping today's typed refusal

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-21).** Binds
the implementer of unit LANE-2; deleted at merge per `docs/DOC-LEDGER.md`.
Read `docs/prompts/implementer-discipline.md` in full first. The item is
`work/scalar/lane-2-chart-region-lane-deleted.md`; the ruling is
`work/scalar/H5.md` §RATIFIED, ruling 3 (PR 2701), which names this
trait in the no-trait cut and thereby supersedes the 2026-09-05 DEFER
(PR 1878) that kept it. LANE-0 and LANE-1 are the precedents: a `Copy`
hook value with one constructor at the bound that certifies, taken as
`Option<_>` by the passes that run at both scalars, `None` from the
`Dual` instantiation. The survey with every citation is
`/home/user/scalar-briefs/survey-lane2.md` on the box.

## 0. The finding, and what the tree says

`ChartRegionLane: Decide` (`crates/topo/src/chart_region.rs:458`) has two
methods, both `-> Option<Result<ChartOverlap, ChartRegionError>>`:
`chart_overlap` (`:462`) and `declared_overlap` (`:475`). Its five impls
forward verbatim to the two public doors `chart_region_overlap`
(`:682`) and `declared_pair_overlap` (`:758`), both already ruling 3's
shape — free functions at `T: Decide + CertifiedBounds`, with a
`compile_fail` doctest at `Dual64` (`:675-681`); the `Dual` arm
(`:597-623`) is two bare `None`s and carries no logic. The trait is a
bound on twelve `census.rs` signatures (`:608 :629 :645 :669 :945
:1724 :1749 :1894 :1990 :2192 :3736 :3820`) and is read at exactly
three sites: `T::chart_overlap` at `:2217` (`None` ⇒
`ValidationError::CensusLaneUnsupported { subject: FacePair }`,
`:2235`), `T::declared_overlap` at `:3956` (`None` ⇒
`CensusLaneUnsupported`, `:3964`) and at `:963` inside
`pair_region_verified` (`-> bool`; `None` folds to `false`, argued at
`:930-944`). The census's one production caller is
`pseudomanifold_certificate_via<T: Decide + Bounds + AtRestPolicy>`
(`validate.rs:5380`, calling `census_and_certify` at `:5414`), the
shared body of `validate_pseudomanifold_certificate` (certified,
`:5338`) and `validate_pseudomanifold_certificate_structural`
(`:5368`, the `Dual` door H-R3 protects) — so the `None` path is live,
typed, production-visible output (`CensusLaneUnsupported`, classified
by `editor-core/src/assembly.rs:1327,1350`, projected by
`pncad-py/src/{tags.rs:2728,validation.rs:86}`), reached through
`AtRestPolicy: … + ChartRegionLane` (`props.rs:2116`). No test observes
it produced at `Dual` (the two `Dual` rows that reach the door refuse
before the census runs); no test file names the trait; no roster test
pins its impls; `crates/pncad*` never name it.

## 1. What this unit delivers

**The trait goes.** `ChartRegionLane`, its five impls and docs
(`chart_region.rs:428-623`), the `pub use` at `topo/src/lib.rs:342`;
`AtRestPolicy`'s supertrait list becomes `Decide +
geom_brep::PcurveFittedLane` (the last lane trait it carries, until
LANE-4).

**The hook, in LANE-1's shape.** `topo::RegionLane<T>` (`Copy`; TWO
private `fn`-pointer fields, `chart_region_overlap`'s and
`declared_pair_overlap`'s signatures; one constructor
`RegionLane::certified()` in an `impl<T: Decide + CertifiedBounds>
RegionLane<T>` block — holding a value IS the statement that `T`
certifies; a `Dual` cannot construct one), beside `QuadLane` in
`props.rs` or in `chart_region.rs` (say which and why; the fields must
be private, so the wiring rows live in its file). The twelve census
signatures take `region: Option<RegionLane<T>>` where they took the
bound; the three read sites read it; `None` takes today's path
EXACTLY — the same `CensusLaneUnsupported { subject }` at `:2235` and
`:3964`, the same `false` at `:963`, the same payloads and `Display`.
`pair_region_verified` keeps returning `bool` (the plan's sentence
reads as if it were an `Option`; it is not — the hook is an extra
parameter whose `None` reproduces the silent `false`).

**Who supplies what.** `pseudomanifold_certificate_via` takes the hook
beside its `nurbs_lane` and `quad_lane` options (three lane parameters
— the shape LANE-4 will fold, not this unit): `validate_pseudomanifold_certificate`
supplies `Some(RegionLane::certified())` by name, the `_structural`
twin `None`. `census_traces` / `census_traces_planted` (`pub`, cfg
`sweep-testing`, re-exported `lib.rs:323`) gain the parameter; their
out-of-crate consumer `crates/editor-core/tests/perf12_census_bvh_diff.rs:71,338`
passes the certified value at `f64`. The ~15 in-file `#[cfg(test)]`
callers at `f64` pass it by name. Nothing dispatches on the scalar at
runtime; no blanket impl.

**What must not change:** every certificate, verdict, refusal and
`Display` at every scalar — `chart_region_overlap` and
`declared_pair_overlap` are untouched (the certified callers reach the
same functions on the same inputs); the `None` path is the base's
`Dual` path read-for-read; the `Dual` doors' outputs on the corpus
bodies byte for byte (`geometric_cube.rs:413-450`,
`lane0_r2_probes.rs:88-189`); the K roster (no `decide(` moves; k-lint
before/after in the PR body); `docs/tess-budget-data/`; `pncad-py`'s
tags and validation projections (the variant is unchanged). The
`compile_fail` doctest at `chart_region.rs:675-681` stays.

**`AtRestPolicy` loses a supertrait; one site depends on it**:
`pseudomanifold_certificate_via` — the hook parameter is what replaces
the inheritance. `EvalScalar` (`eval/mod.rs:2334`) and
`demos/tour/src/scalar.rs:24` inherit the term and use no method:
confirm with `cargo check --workspace --all-targets` at default and
`--all-features`, and `cargo check` in `demos/tour`; list every bound
edit in the PR body (the survey expects the twelve census signatures
and the `_via` body; say if the cascade reaches further).

## 2. Docs

`crates/geom-core/src/real.rs:1183-1202` (the M9-2 allowlist entry:
"`ChartRegionLane`'s refusing `Dual` impl is not redundant … it is
what lets the census, a MIXED pass, decline this one arm and keep
going") — re-worded to the hook's `None` (naming-only: the entry's
rule — the door's bound is `Decide + CertifiedBounds`, the census
declines one arm — is unchanged); `props.rs:2108-2113` (the
supertrait sentence) — one lane trait now; `chart_region.rs`'s trait
docs go, and what survives of them (the census is a mixed pass; the
predicate is uninstantiable at `Dual` by the door's bound) moves onto
`RegionLane`'s doc; `docs/DUAL-DESIGN.md:97-100` (DL3: "the census
door" among the three things structurally absent at `Dual`) — check
the sentence still describes the mechanism (a `None` hook at the
`_structural` twin); if it names a trait, naming-only per CLAUDE.md's
carve-out with the `git log -S` commit cited; `docs/GENERICS-BUILD-COST.md:386-389`
— drop `EdgeNurbsLane` if LANE-1's fix pass has not, and say the
region lane is a value; `work/scalar/H5.md`'s DEFER paragraph — one
sentence that ruling 3 superseded it (the tracker, not ratified text).
Gate ledgers (GUARD, naming-only, disclosed): `scripts/gates/bounds-allowlist.sh:1320-1326`'s
must-NOT-fire selftest fixture literally spells `impl<T> ChartRegionLane
for geom_core::Sym<T>` — replace the fixture with a live where-clause
shape of the same class (the gate's self-test must still pass and
still exercise the near-miss it was written for); the per-file counts
at `:1314` (`chart_region.rs` 26) and `:533` (`census.rs` 6) move with
the approved change — re-derive and state them; `scripts/gates/evalscalar-allowlist.sh:8-14`
— `AtRestPolicy`'s supertrait list as it is after this unit (LANE-1's
fix pass corrects the same line first; merge main and re-check).

## 3. The pin

- A `compile_fail` doctest that `RegionLane::<Dual64>::certified()`
  does not type-check (the bound, not an import), beside the existing
  door doctest.
- THE `None` PATH, OBSERVED: no test today sees `CensusLaneUnsupported`
  produced by a census run. Add a row that hands `None` to
  `census_and_certify` at `f64` on a body whose census reaches the
  face-pair arm (`:2217`) and one whose declarations reach the confirm
  arm (`:3956`), asserting the exact variant, subject and `Display`;
  and a row that `pair_region_verified` with `None` answers `false`
  where `Some` answers `true` on the mate9 crossing rung's isolator
  body (`mate9_crossing_rung.rs:298-315`). If a body that reaches the
  arms at `Dual64` through `validate_pseudomanifold_certificate_structural`
  can be built (past tier 2), add it; if not, say why in the row's doc
  and file it.
- Red-first: with the certified door's `Some(RegionLane::certified())`
  replaced by `None` and nothing else changed, the census rows that
  assert an overlap verdict go red (name them: `census_g2_carrier`,
  `mate5_cyl_eps_rung`, `r1_mate5_probe`, `r2_probes`,
  `m9_2_chart_region_loft`, the in-file census rows); restored, green.
- The wiring pinned by pointer identity (`std::ptr::fn_addr_eq` against
  the two free functions), LANE-1's `wiring_rows` shape, for both
  fields.
- D9 as in §1: the `Dual` corpus rows byte for byte; the three-verdict
  row at `Dual64`; `perf12_census_bvh_diff.rs` unchanged in output.

## 4. Sweep

The class: every site naming `ChartRegionLane` (the survey: 13 in
`census.rs`, 7 in `chart_region.rs`, 2 in `props.rs`, 1 in `lib.rs`, 1
in `real.rs`, 1 in `bounds-allowlist.sh`, plus dated docs and tracker
rows) — dispositioned one by one in the PR body (parameter / read
site / deleted / prose / gate fixture); re-derive at your merge base.
The sites relying on `AtRestPolicy ⟹ ChartRegionLane`: the one named
above; confirm no other by removing the supertrait first and reading
the compiler's list.

## 5. Fence

This program claims no paths. This unit reaches CONTACT's
`crates/topo/src/census.rs`; CHART's `crates/topo/src/chart_region.rs`;
ATREST's `crates/topo/src/validate.rs` (the `_via` body and the two
twins); the unowned `topo/src/props.rs` and `topo/src/lib.rs`; PROPS'
`crates/geom-core/src/real.rs` (allowlist prose) and `docs/DUAL-DESIGN.md`
(if a sentence names the trait); GUARD's two gate scripts (the
fixture, two counts, one header line); WIRE's `editor-core/src/eval/mod.rs`
(prose only, if any); TCOST/TINT's `crates/editor-core/tests/perf12_census_bvh_diff.rs`
and the topo census test files the pins touch; `docs/GENERICS-BUILD-COST.md`.
Not `pncad*` (no occurrence), not `demos/` (`cargo check` only).
Announced by the orchestrator in `work/scalar/log.md` and on the PR.
Merge `origin/main` immediately before opening the PR; `python3
scripts/work.py territory --base origin/main` in the PR body.

## 6. Verification and report

Local: `cargo nextest run -p topo -p sweep -p editor-core` at default,
`-p topo -p sweep` under `--features probe`, `--features interval` and
`--features sweep-testing` (the `census_traces` doors); `cargo check
--workspace --all-targets` at default and `--all-features`, `cargo
check` in `demos/tour`; `cargo clippy` on the touched crates
`--all-targets -- -D warnings` at default and `--all-features`
(narrowed: disk is shared — say so); `cargo fmt --all --check` plus
`--check` in `demos/tour`, `demos/wild`, `benches`; `scripts/gates/bounds-allowlist.sh`
(and `--selftest`), `evalscalar-allowlist.sh`, `probe-suite-census.sh`,
`check-interval-cfg-additive.py`, `scripts/doc-gate.sh`; `python3
scripts/work.py lint`. Private `CARGO_TARGET_DIR`, `CARGO_INCREMENTAL=0`;
delete the target before reporting; scratch under
`/home/user/scalar-lane2-scratch/`. Hosted CI is the verification of
record; poll to conclusion in the foreground and report the run id.
Report ≤120 lines: the hook as landed and where, the twelve signatures
and the `_via` body, the three read sites with their `None` outputs
quoted, the pins with the red-first evidence, the bound-edit list, D9
with the k-lint counts, the docs and gate edits with `git log -S`
commits, deviations, rows filed. Commits are plain one-line messages:
no trailers, no model or vendor names anywhere in commits, files or the
PR body (strip any auto-appended footer through the MCP write path and
verify the live body).
