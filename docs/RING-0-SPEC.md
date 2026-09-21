# RING-0 — the poison differential, and the newtype dry run

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-21).** Binds
the implementer of unit RING-0; deleted at merge per `docs/DOC-LEDGER.md`.
Read `docs/prompts/implementer-discipline.md` in full first. The item is
`work/scalar/ring-0-poison-differential.md`; the ruling is
`work/scalar/H5.md` §RATIFIED, ruling 1 (PR 2701), whose cut (ii)
(RING-2: `RingInterval` a newtype over `DInterval`, poison `dec < Def`)
this unit is the acceptance and the dry run for. Nothing of the dry run
merges; the assertion does.

## 0. The finding, and what the tree says

`RingInterval` (`crates/geom-core/src/ring_interval.rs`) carries its
failure in the endpoints — poison is a NaN pair (`:114-124`,
`is_poison` `:262`) — and `DInterval` (`interval-transcendentals/src/
interval.rs`) carries it in the decoration (`Decoration`, ordered `Ill
< Trv < Def < Dac < Com`, `:13-23`; NaI `:72`, empty `:81`). The ring
poisons a division whose divisor is not proven one-signed
(`ring_interval.rs:516-548`); the backend answers `entire` or a
half-line with `dec = Trv` (`interval-transcendentals/src/arith.rs:
74-96`). `geom_core::Interval` refuses on `is_certified()` = `dec >=
Def` (`crates/geom-core/src/interval.rs`, the `CertifiedEnclosure`
impl). So the swap RING-2 makes replaces every `is_poison()` read (89
in `crates/*/src` today; re-count) with `dec < Def`, and the question
the survey could not answer by reading (`/home/user/scalar-briefs/
survey-h5.md` §1c, on the box) is whether the two VERDICTS agree on
every op and every consumer — not whether the endpoints do. The
existing differential, `crates/geom-core/tests/ring_interval_differential.rs`,
compares endpoints and **skips** every case where either side poisons,
empties or is NaI (`Tally::check_mode`, and the `is_nai() ||
is_empty()` guards in both lanes), so it asserts nothing about the
verdicts at all. The 28 one-sided endpoint comparisons the survey
counted across the 14 heaviest ring files are the sites where a
verdict difference would turn a refusal into a pass; grep cannot tell
which of them follow a division.

## 1. What this unit delivers

**(a) The op-level assertion, merged.** In
`crates/geom-core/tests/ring_interval_differential.rs`, both lanes
(`DInterval` oracle; `Interval` scalar oracle under the feature) gain a
verdict comparison per op that runs BEFORE the endpoint skip: for each
of `+ − × ÷ neg powi` (and `sqr`, which the endpoint lanes do not
compare — add it to both), assert
`ring.is_poison() == (oracle.is_nai() || oracle.is_empty() ||
oracle.decoration() < Decoration::Def)` (for the `Interval` scalar:
`== !oracle.is_certified()`), EXCEPT in a closed allowlist of
characterised disagreement classes, each detected by a predicate on
the INPUTS (not on the outputs), each counted, each printed by the
report line. The allowlist is what the row asserts: a disagreement
outside it fails with `fuzz::replay()`. The survey expected
`point(±inf)` (ring poison / backend `Com`), `inf − inf` and overflow
shapes, and zero disagreements on division; re-derive every class
against the tree (`DInterval::point(±inf)` is NaI at `interval.rs:
101-111`, so that class collapses to agreement — say so), and include
a division-touching-zero row explicitly. The input generator must
reach the classes: extend `ordered` with an adversarial corpus
(±inf endpoints of either sign, `[0,0]` and zero-touching divisors on
both sides, magnitudes that overflow under `×` and `powi`, subnormals,
`[-inf, +inf]`) at a fixed fraction of rounds, seeded through
`test_utils::fuzz` so `replay()` reproduces. The coverage floor of each
lane counts the verdict comparisons too (a lane that skipped its way to
zero verdict rows fails).

**(b) The site-level dry run, never merged.** On a branch
`scalar/ring-0-dry-run` from your merge base (pushed; it stays), make
`RingInterval` a newtype over `DInterval` with `is_poison := dec <
Def || is_nai || is_empty`, `from_bounds` → `DInterval::from_bounds`,
`point` → `DInterval::point`, `+ − × ÷ neg` → the backend's operators,
`powi`/`sqr` → the backend's, `hull`/`clamped_to`/`width`/`mag`/
`contains`/`lo`/`hi` as inherent methods over the backend's endpoints,
`sign_clamp` and the zero annihilator OFF (the backend's rules only),
`from_certified` unchanged in meaning. Keep the surface exactly (every
caller compiles unchanged — that is the point). Run at default and
`--features interval`: `cargo nextest run -p geom-core -p geom -p
geom-brep -p mesh -p topo -p sweep` (the 36 test files that name the
ring — 12 `geom-core`, 6 `geom`, 16 `geom-brep`, 2 `sweep` — plus the
certify/props/mesh suites that consume it), `--no-fail-fast`. Every red
row is a site that depends on a semantic difference. Deliver, in the
PR body and copied verbatim into the item's `## Closed` section at
merge, a table: red row → the production site it exercises
(`file:line`) → the class (division verdict / sign clamp or zero
annihilator / overflow or infinity / a pinned number that got tighter /
a pinned number that got looser / other, named) → whether RING-2 must
re-pin it or must change the consumer. Separate the division-caused
ones by the failure's payload. Counts by class and by crate. The
tighter/looser split is the number ruling 2 needs: RING-2 re-baselines
tighter bounds with the cause named; a LOOSER bound is a finding, not a
re-baseline, and is filed.

**(c) The 28 endpoint reads, dispositioned.** List every one-sided
`.lo()`/`.hi()` comparison on a ring value in `crates/*/src` (re-derive
the count; the survey's 28 over 14 files is the starting number) with
whether the value can have passed through a ring division on any path
(by reading; cite the producing expression), and whether the dry run's
red rows reached it. The ones that can follow a division and are NOT
covered by a red row are RING-2's hazard list; name them.

**What must not change:** every bit anywhere on the merged branch —
(a) adds assertions and a corpus to one test file and changes no
`src`; the dry-run branch changes `src` and never merges. If (a)'s
assertion is RED on the current ring against the current backend
outside the allowlist, that is a finding about the two arithmetics,
not a reason to widen the allowlist: stop, characterise it, file it,
and land the row with the class excluded BY NAME and the row that
reproduces it `#[ignore]`d with the reason and the tracker id.

## 2. Docs

The differential's module doc (`ring_interval_differential.rs:1-58`)
states what is now asserted about verdicts and what the allowlist is,
present tense, beside the endpoint paragraphs. Nothing else; the ring's
own doc is RING-2's.

## 3. The pin

The allowlist row is the pin: a new disagreement class fails the lane.
Show it red-first: with the allowlist emptied, the lane fails naming
its first class and `replay()` reproduces it; with the allowlist
restored, green. The `sqr` rows are new comparisons — show they compare
(the tally prints non-zero verdicts for them). D9: every other suite
green unchanged at default and `--features interval`; no `src` change
on the merged branch.

## 4. Sweep

The class: every operation on the ring's public surface
(`ring_interval.rs:113-548`: `poison`, `point`, `from_bounds`,
`from_certified`, `hull`, `clamped_to`, `zero`, `one`, `lo`, `hi`,
`is_poison`, `contains`, `width`, `mag`, `sqr`, `powi`, `Neg`, `Add`,
`Sub`, `Mul`, `Div`, and the `CertifiedEnclosure`/`Enclosure` impls).
List each with its disposition: compared for verdict in (a), compared
for endpoints already, or not comparable and why (no backend
counterpart; a construction rather than an op). The dry run's table
is the sweep of the consumers.

## 5. Fence

This program claims no paths. (a) reaches TCOST/TINT's
`crates/geom-core/tests/ring_interval_differential.rs` (and
`crates/test-utils/src/fuzz.rs` only if the corpus needs a helper that
does not exist — say so). (b) lives on `scalar/ring-0-dry-run` and
touches `crates/geom-core/src/ring_interval.rs` there only. (c) is
reading. Announced by the orchestrator in `work/scalar/log.md` and on
the PR. Merge `origin/main` immediately before opening the PR;
`python3 scripts/work.py territory --base origin/main` in the PR body.

## 6. Verification and report

Local: `cargo nextest run -p geom-core` at default and `--features
interval`, plus the two differential lanes once at
`CAD_FUZZ_EFFORT=20` (reporting the tally lines); the dry run's suites
as in (b); `cargo clippy -p geom-core -p test-utils --all-targets --
-D warnings` at default and `--all-features` (narrowed: disk is shared;
say so); `cargo fmt --all --check`. Private `CARGO_TARGET_DIR`,
`CARGO_INCREMENTAL=0`; delete the target before reporting; the
session scratchpad is shared between lanes — use `/home/user/
scalar-ring0-scratch/`. Hosted CI is the verification of record; poll
to conclusion in the foreground and report the run id. Report ≤120
lines: the allowlist with each class's predicate and count at EFFORT
20, the red-first evidence, the dry-run table with its counts, the
endpoint-read list, the §4 sweep, deviations, rows filed. Commits are
plain one-line messages: no trailers, no model or vendor names anywhere
in commits, files or the PR body (strip any auto-appended footer and
verify the live body).
