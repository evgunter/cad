---
id: re-exec-child-harness-is-copied-per-suite-and-greens-when-it-does-not-run
kind: issue
title: The current_exe re-exec harness is copied per suite, and every copy greens when the child runs no tests
status: open
opened: 2026-09-15
priority: P4
cost: E
---


Filed by CHROME's `chrome/band-refusal-badging` lane, onto S-TINT's
slate: *"a row that cannot go red is not a test"* is this finding
exactly, and `crates/*/tests/*` plus `crates/test-utils/*` is where
both halves land. Announced to S-TCOST as a shared-territory row; it is
not justified by a second and is not a cost lever.

## The mechanism

A handful of suites need a process of their own — `geom_core::Tolerance`
commits once per process, and `tests/all.rs` aggregates every suite into
one binary, so a row that must run at a non-default ε re-execs
`current_exe()` filtered to a `#[test]` that no-ops unless its env var
is set. The pattern is sound and there is no alternative to it today.

## Half one: it is copied, constant and all

The harness is **near-verbatim across suites** — `current_exe()`, the
`module_path!().split_once("::")` filter derivation with its
standalone-layout fallback, `--exact --nocapture`, the `env_remove` of
`CAD_TOLERANCE_EPS` / `CAD_AMBIGUITY_K`, the child's early return. The
prose at each copy discloses the pattern; what it does not disclose is
that **the constants travel with it**. `crates/viewer/tests/
tree_badges.rs`'s `BANDLESS_EPS` is `f64::MAX / 2.0`, bit-for-bit
`crates/editor-core/tests/wire_band_cause.rs`'s `OVERFLOW_EPS`, with no
sentence at either site admitting the other — the undisclosed-copy
shape a constants grep finds and a prose grep does not. (That second
spelling is this lane's, written before the duplication was noticed;
it now carries a pointer at this row.)

`crates/test-utils/` hosts shared test mechanism for exactly this. What
makes it a question rather than a task: whether a second crate's test
tree can reach a spawn helper cheaply, and whether a helper that must
know its caller's `module_path!()` can be a function at all or wants a
macro. Not settled here.

## Half two, and it is the one that costs: the child can run nothing

**libtest exits 0 when its filter matches nothing** — verified on this
tree by renaming the probe in the parent's filter derivation and
reading the child's own stdout: `running 0 tests / test result: ok. 0
passed; 0 failed; … 559 filtered out`, exit status 0. Every assertion
such a row makes lives in the child. So a parent that checks only
`status.success()` reports green when:

- the suite file is renamed, or `all.rs`'s `#[path]` nesting changes,
  so `module_path!()` no longer derives the filter the child answers to;
- the child `#[test]` is renamed;
- the env var is renamed on one side only, so the child takes its
  no-op return.

None of those is exotic — the first is a rename away, and `all.rs`'s
nesting is exactly what S-TCOST changed when it aggregated the binaries.

## The measurement, and what it could not decide

Pattern: `grep -rl 'current_exe()' crates/*/tests/`, then per file the
count of `.output()` against `.status()`, then of `contains(`.
Thirteen files; `crates/pncad/tests/all.rs` is a different idiom (six
`.status()` spawns) and is not counted below.

- **Spawn and check only the exit status — the hole, open:**
  `crates/editor-core/tests/m4_pr6_eps_diff.rs`,
  `crates/editor-core/tests/wire_band_cause.rs`.
- **Capture the child's stdout but assert nothing about its content**
  (so the hole is open, and closing it is one `assert!` away):
  `crates/geom-core/tests/review_m0_pr2.rs`,
  `crates/mesh/tests/review_m2_pr6_determinism.rs`,
  `crates/stl/tests/export.rs`.
- **Capture and assert on the content, so a child that ran nothing
  reddens** — these are the ones to copy:
  `crates/geom-core/tests/tolerance_init.rs`,
  `crates/geom-core/tests/review_m2_pr7_k.rs`,
  `crates/geom-core/tests/eps_provenance.rs`,
  `crates/sweep/tests/fillet_h6_cap_rim.rs`,
  `crates/sweep/tests/review_blend_k_rk_probes.rs`,
  and `crates/viewer/tests/tree_badges.rs`, which is this lane's and
  was in the first group until this review.

**What the pattern could not match.** `contains(` is a proxy, not the
property: a row may call `contains` on something other than the child's
stdout, or may pin the child by a side effect (a file the child writes)
that no grep of the parent can see. The three-way split above is
therefore a starting list for a reader, not a verdict on five suites —
each of the five wants opening before it is called holed. The grep also
sees only `crates/*/tests/`, so `benches/`, `demos/`, `tools/` and
`interval-transcendentals/` are outside it entirely.

`crates/test-utils/src/vacuity.rs` is the tree's statement of this
defect class — *"it can observe nothing at all, which greens"* — but
its instrument does not fit: `Exposure`'s floors are counted and
asserted in-process, and here the process whose execution is in doubt
is the child. The fix shape that does fit is the one
`crates/geom-core/tests/ambiguity_k_env.rs` already uses: the child
prints a sentinel as its last act and the parent asserts on it, which
also catches the env-guard no-op that a matched-test count would not.

Signed: (CHROME implementer lane, `chrome/band-refusal-badging`)
