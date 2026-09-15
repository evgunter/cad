---
id: test-aggregation-gate-hint-names-a-guard-body-that-no-longer-exists
kind: issue
title: test-aggregation.sh's violation hint tells a crate to copy a guard body that is now a macro invocation
status: open
opened: 2026-09-15
---


## Finding

- **Where**: `scripts/gates/test-aggregation.sh`, the violation hint
  printed by the embedded python (`"pattern, including the
  every_suite_file_is_aggregated guard to copy."`), and the same
  sentence one comment block above it
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

`.github/workflows/ci.yml`'s mirror of that header comment and
`docs/LOCAL-BUILD-PERF.md`'s paragraph both name the row rather than
its body, and both stay true; they are listed here so the next taker
does not re-check them.

**Why it was not fixed in the PR that caused it.** `scripts/**` is
outside S-TINT's fence and `scripts/gates/test-aggregation.sh` is
`guard`'s by `work.py territory`. The fix is one string: say *"and
invoke `test_utils::every_suite_file_is_aggregated!()`"* in place of
*"including the every_suite_file_is_aggregated guard to copy"*. The
gate's `--selftest` does not read the hint text, so nothing else moves.
