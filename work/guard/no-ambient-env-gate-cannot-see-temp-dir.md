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
