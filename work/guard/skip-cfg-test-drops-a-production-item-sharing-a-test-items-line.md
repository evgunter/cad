---
id: skip-cfg-test-drops-a-production-item-sharing-a-test-items-line
kind: issue
title: lib.sh's --skip-cfg-test drops a production item written on the same line as a test-only one
status: open
opened: 2026-09-24
priority: P4
cost: E
---

## Finding

`scripts/gates/lib.sh`'s reader under `--skip-cfg-test` skips line by
line: a line carrying a test-only `cfg` opens a skip, and the skip ends
on the line where the braces it opened close (or on the first `;` if
none opened). Everything else on that closing line goes with it. RING-5's
second review (#3174, R2 N2) planted, in a listed certification file,

```rust
#[cfg(test)] fn z() {} pub fn prod(x: Interval) -> Interval { use geom_core::Real; x.sqrt() }
```

and `certification-doors.sh` stayed green: the production `prod` shares
the test function's line, so it was dropped as test code. Every gate
that passes `--skip-cfg-test` shares this, and
`crates/geom-core/tests/certified_endpoint_census.rs`'s `production`
now ports the same cut on purpose (so the two instruments agree), so it
shares it too.

## Why it is low

`cargo fmt --check` runs in CI and never leaves an item after a closing
`}` on the same line, so the shape cannot land on a formatted tree. The
fix, if wanted, is to end the skip at the closing brace's column and
hand the rest of the line back to the reader — or a selftest plant
that pins the blind spot as stated.
