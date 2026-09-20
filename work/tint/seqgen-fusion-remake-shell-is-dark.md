---
id: seqgen-fusion-remake-shell-is-dark
kind: issue
title: seqgen::fusion_remake_shell can return None unconditionally and nothing in topo reds
status: open
opened: 2026-09-20
---

## Finding

- **Where**: `crates/topo/src/seqgen.rs`, `fusion_remake_shell`
  (~:1216).
- **Importance**: medium
- **Confidence**: sure. Measured by mutation, twice, at two efforts.
- **Raised by**: the `Body::shells_of_solid` fold, 2026-09-20.

`fusion_remake_shell` decides whether a `kef` fusion can be inverted by
re-making a shell, and its answer steers `roundtrip`'s outcome between
`Done` and `SkippedIrreversible`. **Its whole return value is
unasserted.**

Measured at `230c46738` (the fold's head, merge base `cd9fdfd6b`),
`cargo test -p topo --lib` and `--test all --no-fail-fast`, own target
directory. Baseline **733 lib / 566 integration**, each table row
summing to it.

| planted | direction argued before the run | lib | integration |
| --- | --- | --- | --- |
| `.last()` → `.first()` | permutes: a one-shell solid answers the same key either way, so this discriminates only where a fusion candidate sits on a solid with ≥2 shells | 733 / 0 | 566 / 0 |
| the body replaced by `None` — **the control** | the function never permits a re-make, so every `kef` fusion the generator reaches becomes `SkippedIrreversible` | 733 / 0 | 566 / 0 |
| the same, at `CAD_FUZZ_EFFORT=20` | twenty times the sequence depth | 733 / 0 | 566 / 0 |

**The control is what makes this a finding rather than a null about
order.** Had only the first row been run, the honest reading would
have been "the site's use of the list ORDER is unasserted"; the control
says the site is unasserted *entirely*, and the first row measures
nothing. (The program's own 2026-09-20 log entry names this shape — a
measurement that was real, ran, came back green, and whose sentence
about what it discriminated was false.)

The restore harness rewrote the pre-plant bytes and `git diff --stat`
against `HEAD` was empty after every run.

## What would close it

A row that reaches a `kef` fusion on a multi-shell solid and asserts
the roundtrip completes rather than skipping. `work/topo/movefac-row-skips-three-component-shells.md`
is the neighbouring skip and may be the same fixture problem.
