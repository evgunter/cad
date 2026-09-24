# RING-4 — the `interval` feature is dropped: the certified code compiles in every build, CI's lane axis collapses, Q1 says what is true

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-24).** Binds
the implementer of unit RING-4; deleted at merge with a note under
`docs/doc-ledger/`. Read `docs/prompts/implementer-discipline.md` in full
first. The item is `work/scalar/ring-4-interval-feature-dropped.md`; the
ruling is `work/scalar/H5.md` §RATIFIED ruling 1 cut (iii) (PR 2701): "the
newtype dissolves into `Interval` **and the `interval` feature is dropped**
(measured ceiling: ~+36 % clean workspace build, ~+11 % test compile)".
The survey with every citation is `/home/user/scalar-briefs/survey-ring4.md`
(at `5a34a6342d`); re-derive every count at your merge base. **RING-3**
(the kernel dissolution, branch `scalar/ring-3`, in flight) is the other
half of the cut; the two are independent in code (the feature gates
instantiations and modules, not the ring) and meet only in C9's feature
sentence and a few shared files — merge `origin/main` before opening and
resolve against whatever of RING-3 has landed.

## 0. The cost gate — measure first

Ev ratified the drop at a measured ceiling. The survey's one local run
(`cargo build -p editor-core --lib`, other lanes compiling) read +66 % to
+103 %; the hosted build+archive rows read the interval lane no slower
than the default (4.7–6.5 min vs 5.2–7.4, sccache). **Before converting
anything**: take the hosted numbers from the three most recent green
code-tier runs on main (build+archive default vs interval, the six test
rows each), and one local clean `cargo build --workspace --lib` and
`--tests` each way with a private target (state the concurrent load).
Put the table in the PR body's first section. **If the hosted
build+archive cost of the feature exceeds +50 % over the default lane,
stop and report** — that is outside what Ev ratified and is Ev's call.
Otherwise proceed.

## 1. What this unit delivers

**The feature is deleted, not made a no-op** (fail-loud: every stray
`--features interval` then errors instead of silently meaning nothing):
the 16 manifest declarations and forwards; every `cfg(feature =
"interval")` site in `crates/*/src` (54), `demos/tour/src` (10),
examples (5) and the test files (123 whole-file gated, 129 with item
gates — survey §1–§2) removed so the code compiles unconditionally;
every `not(feature = "interval")` arm dispositioned by what it was for —
the six `loud_skip_marker!(feature = "interval")` rows (5 sweep, 1 topo)
and their kind are DELETED (they exist only to say the feature was off),
the `cfg_attr(not …)` in sweep's test and the four `not` arms in
`demos/tour` and the examples resolved to the feature-on branch. The
five editor-core modules (`clearance`, `drive`, `range`, `report`,
`stackup`), `eval::leaf` and pncad's ~40 re-exports become default API;
say so in the PR body (LIB/BIND's seam). **After the unit, `git grep -n
'feature *= *"interval"'` over the whole tree (including `demos/`,
`benches/`, `tools/`, scripts, docs) returns nothing but historical
records**; state the command and its output. `unexpected_cfgs` under `-D
warnings` guards the workspace; `demos/` has no such lint, so the grep is
its guard.

**Every invocation goes**: the 8 hosted CI sites (incl.
`k_probe_sweep.sh:140`'s `probe,interval`), `ci-local.sh`'s 9 lines, the
bench harnesses, READMEs and guides that tell a reader to pass the flag.

**CI's lane axis collapses** (CIW's ground — announced): the interval
lane is already the superset (it compiles and tests everything the
default lane does, plus the certified code), so the fold keeps the
interval lane's build/lint/test rows as THE rows and deletes the old
default ones — name the job renames in the PR body, because a reader
counting job names will otherwise think the matrix narrowed. The
backend (`interval-backend`) and oracle (`oracle-certify`) jobs test the
backend crate, not the feature, and stay (in `ci.yml` or a renamed
workflow — say which). `cache-prime-interval`, the `LANE` axis in
`scripts/ci-filter.py` (TCOST's), `interval-only-selection.py`, the
cache-prime and mirror parity scripts' lane keys, the sccache lane key,
`base-test-listing --lane` and `nightly.yml`'s interval legs collapse
with it; `check-interval-cfg-additive.py` and its step retire (nothing
left to guard); `ci-filter.py --selftest` and every parity script pass.
**Coverage must not shrink**: the PR body carries the list of test
binaries run by one code-tier run before and after (from the jobs'
nextest listings) and shows every test that ran before runs after.

**What must not change:** every test outcome, every golden, the K roster
(no `decide(` moves — k-lint's five rows green), `docs/tess-budget-data/`
(re-take it with `--sizing-only` and show it unchanged — the default tour
walk now runs the E6 driver's narration), the render goldens (the render
lanes green), the python suite. If anything moves, stop, characterise,
file.

## 2. Docs — Ev's text; this PR waits for Ev

- **Q1** (`DESIGN.md:1307-1310`): RING-4 retires the decision that the
  interval instantiation is optional; re-write the clause to what is true
  (the interval scalar and its lane impls compile in every build). Its
  current wording is agent commits `52aa7ccc81`/`b9e7caa378`; the
  original bottoms at grafts `a0d2f441fc`/`3f0ee3aeb6` (Ev's) — cite.
- **`DESIGN.md:266`** ("moves into `topo` behind `interval`", SHELL-3,
  ruled at #1737): naming-only, but Ev's words — re-word.
- **C9's feature sentence** (`crates/geom-brep/README.md` ≈`:262-264`):
  RING-3 re-writes C9 for the dissolution; if RING-3 has merged, re-word
  only the feature sentence; if not, leave C9 to RING-3 and say so.
- **`crates/viewer/GUI-DESIGN.md:291-296`** (ratified; the wasm step's
  `--features interval`): naming-only.
- **`memories/test-suite-cost.md:109`** uses the interval loud-skip row as
  its example: a `memories/` edit waits for Ev regardless — re-word it to
  a live example and name it in the decision section.
- `docs/GENERICS-BUILD-COST.md`: a dated §11 with §0's table.
- The PR is `[ev] RING-4: …`, the item carries `needs_ev: true`, and the
  body opens with the decision section: §0's cost table, then each
  ratified sentence before and after, one line on what decision retires.
- Also re-word: `interval.yml:375-377`'s "four loud-skip rows" goes with
  the file; `geom-core/Cargo.toml:22-25`'s no-op argument goes with the
  feature; `interval-transcendentals/README.md` if it names the flag.

## 3. The pins

- The grep in §1 is in the PR body; a CI-side assertion that no
  `feature = "interval"` survives in `crates/` is optional — if you add
  one, it is a gate script GUARD owns (announce it).
- Coverage before/after (§1) is the unit's main receipt.
- One deliberate-failure proof: re-add a stray `cfg(feature =
  "interval")` to one workspace `src` file and show `unexpected_cfgs`
  reds the clippy/check leg locally; restore.

## 4. Sweep

The class: every `interval` feature name anywhere in the tree — cfgs,
manifests, CI YAML, scripts, docs, READMEs, briefs under `docs/` — with
its disposition in the PR body by file and count. Historical records
(`docs/MODEL-AB-LOG.md`, `docs/DUAL-REVIEW-LOG.md`, `docs/doc-ledger/`,
`work/*/log.md`) are left. The items the unit moots
(`ciw/interval-only-selection-premise-restored`,
`ciw/interval-cfg-gate-names-the-wrong-cause-for-an-attribute-order`,
SCALAR's `gate-on-the-type-in-prose-outside-geom-core`) are closed by the
orchestrator at merge — list them in the body.

## 5. Fence

This program claims no paths. This unit reaches CIW
(`.github/workflows/*`, the lane scripts), TCOST (`scripts/ci-filter.py`,
the test files, with TINT), MIRROR (`ci-local.sh`,
`check-ci-mirror-parity.py`), GUARD (the probe census, any new gate),
LIB/BIND (pncad, pncad-py manifests and re-exports), CLEAR/PROPS/EDIT/
STACK/WIRE (the editor-core modules' cfgs), CHROME (viewer's manifest,
`GUI-DESIGN.md`), PROPS (geom-core's manifest and cfg(test) sites), the
unowned editor-core `lib.rs`/`report.rs`, `topo/src/props.rs`, seven
manifests, `demos/tour`, `DESIGN.md`. CLEAR's open SHELL-3 plans a
"behind `interval`" landing that this unit removes — the orchestrator
announces it. Merge `origin/main` immediately before opening the PR;
`python3 scripts/work.py territory --base origin/main` in the body.

## 6. Verification and report

Local: `cargo check --workspace --all-targets` at default and
`--all-features`; `cargo check --all-targets` in `demos/tour`,
`demos/wild`, `benches`; `cargo clippy --workspace --all-targets -- -D
warnings` ONCE (narrowed to touched crates if disk is short — say so);
`cargo nextest run` crate by crate for the crates whose cfgs moved (never
`--workspace`); `scripts/ci-filter.py --selftest`, every parity script,
`scripts/doc-gate.sh`, the gates, `python3 scripts/work.py lint`; the
tess-budget `--sizing-only` recipe + `tools/tess-lint`. Private
`CARGO_TARGET_DIR`, `CARGO_INCREMENTAL=0`; delete it before reporting;
scratch under `/home/user/scalar-ring4-scratch/`. Hosted CI is the
verification of record; poll it to conclusion in the foreground and
report the run id, establishing from the `change filter` log what ran.
Report ≤ 120 lines: §0's table and the go/no-go, the counts removed by
kind, the public API that became default, the CI fold (jobs kept,
deleted, renamed) and the coverage before/after, the grep, what did not
move, Q1 / `:266` / C9 / GUI-DESIGN / memory before and after with
writers, deviations, rows filed.
