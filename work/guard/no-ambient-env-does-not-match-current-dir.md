---
id: no-ambient-env-does-not-match-current-dir
kind: issue
title: no-ambient-env's pattern does not match std::env::current_dir, which two files read
status: closed
opened: 2026-09-19
closed: 2026-10-26
priority: P3
cost: E
---


## Finding

`scripts/gates/no-ambient-env.sh` matches `env::var(_os)` / `env::vars(_os)`
only. The process working directory is an ambient read the same way,
and `std::env::current_dir` is outside that pattern. Two files under
`crates/*/src` read it today:

- `crates/viewer/src/platform.rs`, `launch_dir`: the file dialogs'
  last-resort starting directory (PR #2858). It lives in the gate's
  allowlisted viewer home and argues the four rows in its doc, but
  that is by convention; the gate would not catch it anywhere else.
- `crates/test-utils/src/source.rs`, `crate_dir`: the fallback when
  the baked `CARGO_MANIFEST_DIR` holds no `Cargo.toml`. test-utils is
  already allowlisted (dev-only leaf, discharged by reachability),
  so this one would pass even if the pattern matched.

The gate's own header lists what the viewer's home reads, and it now
says the cwd read is not pattern-enforced.

## What a fix has to decide

Whether to widen the pattern to `\benv::current_dir\s*\(` (and
`set_current_dir`). Both current hits sit in allowlisted homes, so
widening it goes green on the tree as it stands and would catch a
third one.

## Closed (2026-10-26, tracker sweep)

**Duplicate.** `no-ambient-env-gate-cannot-see-temp-dir` (opened
2026-10-02) asks for the same widening of the same pattern and more —
`env::temp_dir(`, `env::current_dir(` and `env::home_dir(` — and is the
survivor. This row's two facts (the viewer's `launch_dir` and
test-utils' `crate_dir` as the `current_dir` readers, and
`set_current_dir` as a member) were merged into it. The gate's pattern
is unchanged at main (`scripts/gates/no-ambient-env.sh`, still
`\benv::vars?(_os)?\s*\(`).
