---
id: test-aggregation-gate-hint-names-a-guard-body-that-no-longer-exists
kind: issue
title: test-aggregation.sh's violation hint tells a crate to copy a guard body that is now a macro invocation
status: open
opened: 2026-09-15
priority: P3
cost: E
---


## Finding

- **Where**: `scripts/gates/test-aggregation.sh`, one sentence — the
  violation hint printed by the embedded python (`"pattern, including
  the every_suite_file_is_aggregated guard to copy."`).
  `grep 'guard to copy'` returns exactly one hit in the tree; the file's
  own header comment says something different and stays true (below)
- **Importance**: low — the gate's verdict is unaffected; what is wrong
  is the remedy it prints to the person who just tripped it
- **Confidence**: sure — the text is quoted below and the change that
  invalidates it is on the same PR
- **Raised by**: the TINT-3 lane, S-TINT, while collapsing the fifteen
  copies of `every_suite_file_is_aggregated`

When the gate fires it tells the offending crate:

> Set `autotests = false` in the crate's Cargo.toml, aggregate every
> tests/*.rs into tests/all.rs via #[path], and declare one [[test]]
> named "all" — see crates/topo/tests/all.rs for the pattern,
> **including the every_suite_file_is_aggregated guard to copy.**

There is no longer a guard to copy. The row is
`test_utils::every_suite_file_is_aggregated!();` — one line, expanded
from `crates/test-utils/src/source.rs` — so the instruction names an
artefact the pattern crate no longer holds, and a reader who follows it
to `crates/topo/tests/all.rs` finds a macro invocation and has to work
out for themselves that invoking it is what was meant. The sentence in
the file's own header comment (*"The complementary half lives in each
crate's `tests/all.rs`: `every_suite_file_is_aggregated` fails when
…"*) is still TRUE — the generated `fn` keeps that name — so this is
one sentence, not a general rewrite.

**Every other citer in the tree names the ROW, not its body, so every
one stays true** — the generated `fn` keeps the name
`every_suite_file_is_aggregated`. The list is complete as of
2026-09-15, from `git grep -n 'every_suite_file_is_aggregated'`, so the
next taker does not re-check them:

1. `.github/workflows/ci.yml:1302` — the mirror of this file's header
   comment.
2. `docs/LOCAL-BUILD-PERF.md:177` — "the complement to each crate's
   `every_suite_file_is_aggregated` test".
3. `scripts/gates/test-aggregation.sh:22-26` — this file's own header,
   *"the complementary half lives in each crate's `tests/all.rs`"*.
4. `crates/step-import/Cargo.toml:78` — "which all.rs's own
   `every_suite_file_is_aggregated` catches".
5. `work/fix/band-helper-duplicated-across-suites.md:139`.
6. `work/guard/measurements-have-no-mechanical-guard.md:98` — cites it
   as an in-house precedent for a manifest-text guard.

Not in that six and deliberately: `work/tcost/`'s rows and the `log.md`
files in `work/chrome/`, `work/tcost/` and `work/wire/`, which are the
historical record of landed work rather than live claims; and the stored
run output under `review/r1-mesh4/`, where the name appears as a PASS
line and is unaffected.

**Why it was not fixed in the PR that caused it.** `scripts/**` is
outside S-TINT's fence and `scripts/gates/test-aggregation.sh` is
`guard`'s by `work.py territory`. The fix is one string: say *"and
invoke `test_utils::every_suite_file_is_aggregated!()`"* in place of
*"including the every_suite_file_is_aggregated guard to copy"*. The
gate's `--selftest` does not read the hint text, so nothing else moves.
