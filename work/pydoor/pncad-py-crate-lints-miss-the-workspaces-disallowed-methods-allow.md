---
id: pncad-py-crate-lints-miss-the-workspaces-disallowed-methods-allow
kind: issue
title: pncad-py's [lints.clippy] lacks the workspace's disallowed_methods = allow, so crate_lints_match_the_workspace_minus_unsafe_code is red on main
status: open
opened: 2026-11-23
priority: P1
cost: E
refs: [7510, 7553]
---


Filed by GERM's `germ/harmonic-belly` lane (PR 7553), whose `test (app)` job went red on it after a merge with main.

## Finding

`pncad-py tests::crate_lints_match_the_workspace_minus_unsafe_code` (`crates/pncad-py/src/tests.rs`, the assertion near line 3315) is red on a clean `origin/main` at `64aeba04f9`:

```
[lints.clippy] has drifted from [workspace.lints.clippy].
  missing here: [("disallowed_methods", "allow")]
```

ONEROUND's PR 7510 (`6bf0e19ef7`, "conic readers") added `disallowed_methods = "allow"` to the workspace's `[workspace.lints.clippy]`. That is the conic centre-read census's arming. `crates/pncad-py/Cargo.toml` restates the workspace table minus `unsafe_code`, and did not gain the line. So this row reds every PR whose app job runs it.

## Settles when

`crates/pncad-py/Cargo.toml`'s `[lints.clippy]` carries `disallowed_methods = "allow"`. Its library root also needs the census's deny line if the census's gate (`scripts/gates/conic-centre-reads.sh`) requires it there. The row is then green on main.
