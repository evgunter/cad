---
id: error-census-keyed-on-bare-type-name
kind: issue
title: The error-type census is keyed on the bare type name, which is ambiguous at seven names
status: review
opened: 2026-09-04
refs: [1111, 1741]
branch: fix/census-declaring-path-key
---


Filed from #1741 (the viewer cut of
`error-types-with-no-display-class`). **No live defect found** — this
is a finding about the METHOD, not about a rendering.

## Why it matters

The list in #1111 was produced by a census keyed on the bare type
name, and so was the re-sweep in #1741 that refuted most of it. The
same key produced both the claim and its correction, so its failure
modes are worth stating before it is used a third time.

At seven names the bare key is ambiguous, in two opposite directions.

**One key, two distinct types** (a false merge — a `Display` found on
either satisfies the census for both):

| name | declared at |
|---|---|
| `BlendError` | `sweep/src/blend/mod.rs`, `viewer/src/blend.rs` |
| `ComposeError` | `geom-core/src/spline/compose.rs`, `geom/src/curves/compose.rs` |
| `LiftRefusal` | `editor-core/src/stackup.rs`, `profile/src/lift.rs` |
| `ReplayError` | `profile/src/path/program.rs`, `viewer/src/history.rs` |
| `SplitError` | `editor-core/src/refactor.rs`, `topo/src/splitting/mod.rs` |

`ReplayError` is on #1111's own list, and that entry resolves to two
distinct types — which is exactly the hazard, since only one of them
was ever the subject.

**One type, two public paths** (a false duplicate — the same type
counted twice, inflating a hit list):

| name | reachable at |
|---|---|
| `PathError` | `profile`'s root and `pncad::prelude` |
| `Refusal` | `viewer::session` and `viewer`'s root re-export |

## The fix

Re-run the census keyed on `crate::path::Type` rather than on the bare
identifier, resolving re-exports to the declaring path. Both directions
close at once: distinct types stop sharing a key, and one type stops
appearing under two.

Worth doing before the next sweep of this class, not urgently — the
known consumers of the old key have both been checked by hand.

## Taken as `fix/census-declaring-path-key`

**The census this row names is not code.** #1111's error-type census
was a hand sweep, re-run by hand in #1741; there is nothing to re-run
keyed differently. The tree's one live census keyed on a bare type
name is `crates/pncad-py/src/prose_census.rs`'s type table, built by
this program (PR 1809), and that is where the re-key landed.

**Re-derived at the merge base** (`1818266`). Both tables above still
hold: `BlendError`, `ComposeError`, `LiftRefusal`, `ReplayError` and
`SplitError` each have two declarations; `PathError` and `Refusal`
each have one, reachable at two public paths.

Three corrections this row's reader should have:

1. **Only one of the two directions applies to a census that indexes
   declarations.** A re-export adds no declaration, so `PathError` and
   `Refusal` were never two entries in this table; the false duplicate
   is a hazard for a hit LIST, which is what #1111 produced and what
   this census is not. "Both directions close at once" is true of the
   method, not of this code.
2. **The bare key was never unsound HERE.** Rival declarations that
   disagree answer `Undecided` and rivals that agree give the verdict
   both share, so no site in the tree got a wrong answer from the
   merged key — measured at the merge base over all 27 colliding names
   and every site that reaches one. What the key cost was precision,
   and a key that is sound only because the disagreement rule catches
   it is one collision away from being asked a question it answers
   confidently and wrongly.
3. **The re-key moves no verdict in this tree today.** `KNOWN_BRACED`
   and `UNDECIDED` are byte-identical before and after, which is why
   the PR carries planted-tree rows that DO discriminate the two
   keyings, run red against the old reading first.
