---
id: no-ambient-env-gate-cannot-see-temp-dir
kind: issue
title: no-ambient-env.sh's pattern matches env::var( only, so a runtime TMPDIR read through std::env::temp_dir() in crates/*/src passes it
status: open
opened: 2026-10-02
priority: P3
cost: E
---


Found in the dual review of PR 3844 (`reach/door-backstop`). The first
cut of topo's `door-tier3-meter` wrote its records to
`std::env::temp_dir()`, which reads `TMPDIR` at run time from kernel
source. `scripts/gates/no-ambient-env.sh` greps
`\benv::vars?(_os)?\s*\(` only, so it could not see that. The meter now
takes its sink path at compile time (`option_env!`), so it no longer
reads `TMPDIR`. The gap remains for anything else.

Other `std::env::temp_dir()` and `env::current_dir()` reads under
`crates/*/src`, measured on PR 3844's merge base: `pncad-py/src/tests.rs`,
`test-utils/src/source.rs` (three, plus a `current_dir`),
`viewer/src/parts.rs`, `viewer/src/app.rs` (two). Most of them are in
test code.

## What would close it

Extend the pattern to `env::temp_dir(`, `env::current_dir(` and
`env::home_dir(`, with the gate's selftest fixtures. Allowlist each
test-only and app-shell reader, with its reason, or move it behind
`#[cfg(test)]` where the gate already skips.

## Merged from `no-ambient-env-does-not-match-current-dir` (2026-10-26, tracker sweep)

That row (opened 2026-09-19) asked for the `current_dir` arm alone and
was closed into this one. What it added:

- The two `current_dir` readers it named are both in allowlisted homes:
  `crates/viewer/src/platform.rs`'s `launch_dir` (the file dialogs'
  last-resort starting directory, PR #2858; the gate's header already
  says the viewer home's cwd read is not pattern-enforced) and
  `crates/test-utils/src/source.rs`'s `crate_dir` (the fallback when the
  baked `CARGO_MANIFEST_DIR` holds no `Cargo.toml`). So widening the
  pattern goes green on those two as the tree stands.
- `env::set_current_dir(` belongs in the widened pattern too.

## Moved to BLIND (2026-10-26, GUARD's split)

GUARD measured 43 points against 30 and split three ways. This row is a
spelling a kernel-discipline gate's matcher misses (or refuses wrongly),
which is BLIND's charter (`work/blind/plan.md`). Id and body unchanged.
