# TCOST-1 — the per-file test gate (spec)

**Program:** S-TCOST (`docs/S-TCOST-PLAN.md`, lever 3). **Track:**
test-infrastructure (scripts + workflow wiring + test-file markers; no
kernel logic) — Opus implementer, batched style review, no A/B row.
**Landing:** self-merged with a full PR writeup (Evan, in-chat,
2026-09-02).

## The claim

A test that exercises the logic of a few named source files runs on a
pull-request gate **only when one of those files, or the test's own
file, is in the diff** — instead of whenever any crate in the test's
dependency closure moved, which is today's rule (`scripts/ci-filter.py`,
TIER=closure). Everything about the gate stays derived, recorded and
fail-open; the coverage the PR gate gives up is re-taken daily.

## Ratified ground

- `memories/test-suite-cost.md`: a fuzzer is *"MARKED to run only on
  changes to the code it was written to test"*; an ungated fuzzer is
  a defect in the fuzzer. Until this unit there was no such mark.
- `docs/CI-MINUTES-2026-08.md` §*What is NOT sampled*: skipping is
  sound for a detector whose subject persists in the tree. A gated
  suite's break persists; the nightly row below is what finds it
  when the PR gate has not.
- The marker-at-the-test siting argument (`scripts/nightly-only-
  selection.py`'s header; `check-ci-mirror-parity.py`): a central
  roster drifts, a mark at the test cannot, and the set is DERIVED
  from the tree on every run.
- ci.yml's standing rule for every skip: **RECORDED, NEVER SILENT** —
  a skipped row leaves a line in the run's own log naming what was
  skipped and why (`--notices`, relayed by the `the configuration this
  run gates` step).

## Design

### The marker

At the top of a gated suite file — `crates/<c>/tests/<suite>.rs`, or a
`src/` file whose `#[cfg(test)]` module is the gated thing — a single
machine-readable declaration naming the repo-relative source paths the
suite is specific to. Suggested spelling (the implementer may improve
it; the properties below are binding):

```rust
//! Gated to the code it tests (TCOST-1): this suite runs on a PR gate
//! only when one of these paths, or this file, is in the diff.
test_utils::gated_to!["crates/geom-core/src/ring.rs", "crates/geom-core/src/interval/"];
```

Binding properties:

1. **Parseable by a stdlib-python reader of the SOURCE TEXT** (no
   cargo, no build) — `scripts/ci-filter.py` runs in the `filter` job
   with no toolchain.
2. **Every named path exists in the tree**, enforced LOUDLY by a
   discipline gate (`scripts/gates/gated-suite-paths.sh`, sited in the
   `discipline` job and `local-scripts/ci-local.sh`'s discipline row,
   with `--selftest`), so a rename cannot silently turn a gate into
   "never runs on PRs". A compile-time existence check
   (`include_str!` of each file path into an unused const) is welcome
   in addition, not instead.
3. Paths are files or directories (trailing `/`), repo-relative.
   Directories mean "anything under".
4. The marker's OWN file is always an implicit member of its path set.
5. One marker per suite file; a `src/` marker gates every test in that
   file's module (whole-file granularity, so the nextest prefix is the
   file's module path and nothing finer needs deriving).

### The selection

`scripts/ci-filter.py` gains one output key, **`TEST_FILTER`**: a
nextest filterset expression (`-E`) that EXCLUDES every gated suite
none of whose paths is in the diff, or the empty string when nothing
is excluded. Both run legs (`test` and `test-interval` in ci.yml, and
the corresponding rows in `local-scripts/ci-local.sh`) append
`-E "$TEST_FILTER"` when it is non-empty. The expression is built from
the tree, never from a list: for `crates/<c>/tests/<file>.rs` the term
is `(binary_id(<c>::all) & test(/^<mod>::/))` with `<mod>` read from
the `#[path = "<file>.rs"] mod <mod>;` line of that crate's
`tests/all.rs`; for `crates/<c>/src/<path>.rs` it is
`(binary_id(<c>) & test(/^<module::path>::/))`. A suite whose binary
or module cannot be derived FAILS OPEN (runs) with a notice.

**Fails open, always toward running:** on TIER=all (no diff to read),
on any marker whose paths do not all resolve, on a diff that touches
`crates/test-utils/` or `scripts/ci-filter.py` or any `tests/all.rs`,
and on any parse error. The empty `TEST_FILTER` is the ordinary
whole-suite run, byte-for-byte what runs today.

**Recorded, never silent:** the filter prints one notice line per
skipped suite (`gated: <suite> skipped — none of <paths> in the diff`)
into `--notices`, which the `filter` job already relays; ci.yml's run
steps echo `TEST_FILTER` before invoking nextest.

### The daily re-take

`.github/workflows/nightly.yml` gains a row that runs the WHOLE gated
set ungated — `-E "<union of every gated suite's term>"`, derived by
`scripts/ci-filter.py --gated-set` from the tree — on the days main
moved (the nightly's existing gate job). It must not report green
having executed nothing: an empty set is legitimate only when no
marker exists anywhere under `crates/` (the nightly-only-selection
pattern: markers present with an empty derived set is a broken rig
and fails). The implementer measures and states the row's billed
cost; the demoted job's build may be shared if that is cheaper.

### Selftests

`scripts/ci-filter.py --selftest` grows fixture cases (the stub-cargo
fixture already there) for: a gated suite excluded when untouched;
included when a named file changes; included when a named directory's
descendant changes; included when its own file changes; whole-suite
run (empty filter) on TIER=all, on a `test-utils` touch, on a missing
path (with the notice text asserted); and the `src/`-module shape.
`check-ci-mirror-parity.py` keeps both halves consuming the key.

### First users

Every row that draws `test_utils::fuzz` (`effort()` / the seed
logger) and every randomized sweep the tree names — the ratified rule
already requires them to be gated. The implementer greps the callers
(`grep -rln "fuzz::\|effort()" crates --include=*.rs`), reads each,
names the code it tests in its marker, and lists in the PR body every
caller with its disposition (gated with these paths / not gated
because …). Review-probe suites (`review_*`, `*_r1_probes`) that are
fuzz-shaped belong here; deterministic pins do not. A marker's path
set is a judgement about what the suite is specific to — err toward
naming MORE paths (an upstream file whose change would plausibly
break the suite), since the cost of a wide set is a run, and the cost
of a narrow one is a missed break until the nightly.

## Acceptance

- Hosted: a PR whose diff touches only an unrelated file shows the
  gated suites skipped in the run log with the notice; a PR touching
  a named path runs them; `TIER=all` runs everything. Both lanes.
- The discipline gate reds on a planted bad path (selftest).
- The nightly row executes the gated set (a dispatch of nightly.yml
  is acceptable evidence; state its run id).
- `local-scripts/ci-local.sh` mirrors the key; the parity check is
  green.
- The PR body carries the caller census and, for the test-cost
  reading, the before/after `Slowest N tests` block of one hosted run
  per lane.

## Out of scope

Deciding which NON-fuzz suites to gate (that is the content units,
TCOST-2…, cut from the timing census); any change to the sampling of
lane/ε/k-lint; any kernel change.
