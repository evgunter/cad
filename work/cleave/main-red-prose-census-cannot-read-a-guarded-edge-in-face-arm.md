---
id: main-red-prose-census-cannot-read-a-guarded-edge-in-face-arm
kind: issue
title: Main red (slow set): the prose census reads BooleanError::EdgeInFace's guarded Display arm as undecided since PR 5546
status: closed
opened: 2026-10-26
closed: 2026-10-27
pr: 5830
priority: P1
cost: E
---


## What

`pncad-py`'s
`prose_census::tests::every_site_this_census_cannot_decide_is_named_with_its_reason`
fails on `origin/main` (reproduced at `2f9e6f9e`). Its slow-set run is
not in hosted CI. Found by SPLICE's slow-set triage lane (branch
`splice/slow-set-reds`).

**Bisect** (`git bisect run --first-parent`): the census reads the
source tree at run time, so one test binary was replayed over each
commit's sources.
- first bad: `ec2dab8d`, the merge of PR 5546
  (`cleave/boolean-parts-an-edge-running-past-its-face`);
- last good: `e9680c52`, its first parent.

**Failure:** the found set has two entries more than `UNDECIDED`
(`crates/pncad-py/src/prose_census.rs`):

    ("crates/topo/src/boolean/mod.rs", "BooleanError", "edge"): 1
    ("crates/topo/src/boolean/mod.rs", "BooleanError", "face"): 1

PR 5546 rewrote `BooleanError`'s `Display` arm for `EdgeInFace` as
`Self::EdgeInFace { edge, face, gap } if gap.is_defect() => …`, and
that arm's `{edge:?}` and `{face:?}` are the two sites. The census's
`arm_pattern` scans back from `=>`. It passes the guard's `()` first, so
it stops at the pattern's closing `}` and reads `if gap.is_defect()` as
the arm's whole pattern. That has no bindings, so both placeholders
answer Undecided. The unguarded `EdgeInFace` arm after it is still
decided.

## What is owed

Either rewrite the arm so the census can type its bindings (for
example, branch on `gap.is_defect()` inside one unguarded arm), or
teach the census to read a pattern that carries a match guard. Add
`UNDECIDED` lines only if neither can be done. The census itself is
CENSUS's ground (`work/census/`).

## Closed

2026-10-27, PR 5830. `BooleanError`'s `Display` (`crates/topo/src/boolean/mod.rs`)
writes `EdgeInFace` as one unguarded arm that branches on
`gap.is_defect()` inside it, so the census reads the arm's own pattern
and types `edge` and `face`; both messages are byte-identical. The
census is unchanged and `UNDECIDED` gains no line. The census's blind
spot for guarded arms is filed on CENSUS's slate as
`prose-census-reads-a-match-guard-as-the-arms-pattern` (P3).
