---
id: gated-markers-name-too-few-src-paths
kind: issue
title: The gated_to! src/ half is unchecked by anything: a marker naming too few source paths skips its suite silently
status: open
opened: 2026-09-12
---


Filed 2026-09-12 by the orchestrator out of the style review of PR 2433,
which found the defect in that PR's own new marker. **The instance is
fixed in that PR; this row is the class**, and the class is the one the
gate mechanism cannot check.

## The asymmetry that makes this invisible

A `gated_to!` marker has two halves and only one of them is checked.

- The **`tests/` half** — the suite's sibling helper modules — got a
  mechanical arm on 2026-09-11 (`_unnamed_helper_imports` in
  `scripts/ci-filter.py`), because an omission there is derivable: the
  suite's own `use` statements name them.
- The **`src/` half** is the author's judgement and `ci-filter.py` says
  so in as many words. Nothing derives it, nothing checks it, and
  `--gated-check` passes a marker naming one source file as readily as
  one naming the right twelve. It verifies that every named path
  RESOLVES, never that every path that should be named IS.

So the failure mode is: a marker names too few source paths, the suite
is skipped on a diff that breaks it, and **a skipped test contributes no
row to the cost report**, so the instrument that would show it cannot
see it. This program has a live, expensive instance of exactly that
shape — `m10-3-chamber-row-reads-ten-times-its-recorded-cost`, a suite
gated to editor-core modules that a `geom-core` change made 15x slower,
unread for weeks.

## Measured on PR 2433's head, and this is the method for the sweep

`scripts/ci-filter.py --files <a file with one path per line>`, one path
at a time, reading whether the suite appears in the skip notices:

| changed path | `r1_dual_probes` |
|---|---|
| `crates/editor-core/src/product.rs` | **SKIPPED** |
| `crates/editor-core/src/edit.rs` | **SKIPPED** |
| `crates/topo/src/body.rs` | **SKIPPED** |
| `crates/editor-core/src/node.rs` | runs |
| `crates/geom-core/src/dual.rs` | runs |

The suite asserts directly on all three skipped files. The marker was
written that same day, by a lane that had read the rule, and was wrong on
the day it landed — which is the argument that this is structural and not
carelessness.

## What this row asks for

**Re-check every marker's `src/` half**, the way the table above was
built. `--gated-check` reports the live suite count (57 on PR 2433's
head); TCOST-9 already found ten markers omitting helper imports, and
that sweep was mechanical and therefore blind to this half. The two
questions per marker are different and both matter:

1. **Too few paths** — a file the suite asserts on that no named path
   covers. The suite is skipped on a change that breaks it.
2. **Too many** — a marker so wide it never skips has retired the gate
   while reading as a live one. Worth naming where found; it is the
   cheaper error but it is still a false report.

`crates/test-utils/src/lib.rs` states the standing rule — **ERR TOWARD
NAMING MORE**: an upstream file whose change would plausibly break the
suite belongs in the set.

## The question behind the sweep

A sweep fixes today's markers and the next one is written by hand
tomorrow. Worth deciding, with the evidence the sweep produces:
**can the `src/` half be derived at all?** A suite's `use` statements
reach its direct imports, and `cargo metadata` reaches the crates, but
the marker wants the FILES whose change would break it — which is
narrower than the crate and wider than the imports. If it cannot be
derived, the honest options are a review checklist item or a periodic
re-sweep, and either should be written down rather than assumed.

## Not this row

The nightly ungated re-take bounds the latency of exactly this failure
to one day, and is not re-opened here. Neither is the EFFORT policy
(`fuzz-depth-not-existence-run-everything-at-effort-1`), which would
retire the existence question entirely — **if that lands, this row's
consequence shrinks from "the suite does not run" to "the suite does not
get a raised EFFORT", which is the whole argument for that policy.** It
does not close this row: a marker that names the wrong files is still
wrong, and the depth selection reads the same named paths.
