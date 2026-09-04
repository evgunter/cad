---
id: reader-census-red-on-main-docm1-hand-rolled-doc-reader
kind: issue
title: "main is red: docm1_face_frame hand-rolls a module-doc reader and owes the census a disposition"
status: open
opened: 2026-09-04
---

Found by VIEW while gating an unrelated tracker PR (#1857), whose only
diff is `scripts/work.py` and `work/README.md`. **The failure is not
that PR's and reproduces on `main`.**

## The red

```
FAIL test-utils::reader_census every_site_that_reads_rust_source_is_in_the_ledger
  arrived or moved (owe a ledger line): [
      "crates/editor-core/tests/docm1_face_frame.rs",
  ]
```

Reproduced locally at `origin/main` (`cargo test -p test-utils --test
reader_census`). It fails on **both** shard-1 jobs — `test (eps =
default, 1/2)` and `test (eps = 1e-12, 1/2)` — so it is lane- and
eps-independent, as a source-text census should be.

## What arrived

`crates/editor-core/tests/docm1_face_frame.rs:258-272`
(`rule_one_names_numeric_predicates_in_both_statements`) reads two
Rust files as text and extracts their module docs by hand:

```rust
const KERNEL: &str = include_str!("../../topo/src/readback.rs");
const DOOR: &str = include_str!("../src/names/interrogate.rs");
let kernel_doc: String = KERNEL
    .lines()
    .take_while(|l| l.starts_with("//!"))
    ...
```

That is a **new hand-rolled Rust reader**, which is precisely the
population `crates/test-utils/tests/reader_census.rs` exists to detect.
**The row worked.** It landed in `17bb8fb18` (DOCM-1's fix pass) and
`main` has been red since 2026-09-04 18:08 UTC.

## The disposition is DOCM's to pick, and the two candidates differ

The census's module header and its `Disposition` enum do not say quite
the same thing, and which applies here is the decision:

- The **header** says a new hand-rolled reader gets *no* ledger line —
  *"Use the shared lexer. That is the whole point of the row."* On that
  reading the fix is to convert the test to `test_utils::source`.
- The **enum** offers `Unconverted(track)` for a reader that is not on
  the shared lexer, whose own doc says such an entry *"is not an
  exemption, it is a second finding stacked on the first"*.

The header reads as governing NEWLY ARRIVED readers and the
`Unconverted` arm as grandfathering the ones that predate the row. If
that is right, conversion is the fix and a ledger line would be the
wrong one. **VIEW has no standing to pick** — `crates/editor-core/*` is
DOCM's and the census file is not VIEW's either — and did not.

## Why this is filed rather than fixed

`docs/prompts/implementer-discipline.md` §6: a VIEW branch may not edit
DOCM's ground, and the fix is a change to DOCM's test rather than a
one-line ledger append. Widening an unrelated `[ev]` tracker PR to
carry it would be worse.

## Why nobody saw it — measured, not the shape first guessed

This section first read *"this is the other shape"* from
`work/issues/ci-draw-can-hide-a-compile-break-on-main.md`, and guessed
a draft PR as the path. **Both were wrong, and the truth is simpler.**

**A `main` push run draws no test job at all.** On the `#1829` merge
push (runs `33905366880` / `33905368338`) every `test`, `clippy`,
`build`, `k-lint`, `discipline`, `python suite` and `rustfmt` job is
**skipped**; only the mirror check, the change filter, the cache primes
and the render lanes run. Same on the two `main` pushes after it, both
of which reported **success** over a red tree.

So no draft is needed and no unlucky draw is needed. This is the SAME
hole as `ci-draw-can-hide-a-compile-break-on-main`, firing a second
time on the same day, and that file now carries the measurement.

Signed: (VIEW orchestrator)
