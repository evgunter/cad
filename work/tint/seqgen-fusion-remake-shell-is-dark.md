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
| **the body replaced by `panic!`** — the DISCRIMINATING control | nothing the caller can do satisfies a panic, so this separates *called* from *not called*, which no return-value plant can | **732 / 1** | 566 / 0 |

The red is
`seqgen::random_op_sequences::random_op_sequences_hold_all_properties`.
**So the site is reached**, on the default fuzz effort, and its answer
is wholly unasserted.

**The `-> None` row is not a control, and reading it as one was the
defect this row carried first.** This row wrote *"the site is
unasserted entirely"* off a pair of return-value plants — a conclusion
that happened to be true, by the shape of this tree rather than by the
measurement. The
general rule — **the single home for it**, and the program's method
item 19; rows that need it point here rather than restating it:

> To prove a site is REACHED, plant something no answer can satisfy; to
> prove its answer is unasserted, plant a different answer. **One plant
> cannot do both.** And **a plant that replaces a function's body with
> its own null return value is not a control — it is the SAME
> experiment with a wider swing.**

That last clause is the one that matters, because it is what made the
first reading look rigorous: `-> None` is a bigger change than
`.first()` and feels like a different kind of probe, so a rule phrased
only as *"plant a different answer"* does not obviously exclude it.
The program's 2026-09-20 log entry anticipates the shape in its "a
claim about a measurement" paragraph — a measurement that was real,
ran, came back green, and whose sentence about what it discriminated
was false.

The restore harness rewrote the pre-plant bytes after every run and
the diff against `HEAD` carried only the lane's own uncommitted work,
never a plant.

## What would close it

A row that reaches a `kef` fusion on a multi-shell solid and asserts
the roundtrip completes rather than skipping. `work/topo/movefac-row-skips-three-component-shells.md`
is the neighbouring skip and may be the same fixture problem.
