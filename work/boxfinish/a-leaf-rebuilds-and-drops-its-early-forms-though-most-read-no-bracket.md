---
id: a-leaf-rebuilds-and-drops-its-early-forms-though-most-read-no-bracket
kind: issue
title: A leaf rebuilds and drops its early forms in every replay: the per-replay teardown M9 measured is the early memo's, not the expression table's
status: open
opened: 2026-11-23
design: true
priority: P2
---


Found by the session-state unit
(`a-nodes-result-at-the-symbolic-scalar-reads-per-replay-session-state`)
when it measured the per-replay teardown after the expression table,
the plain forms and the atom records moved to the drive.

**Measured** (`demo-tour certified`: the 512-leaf study and the cell's
other drives, 1,460 sessions, 4 workers, CI's build profile, a timing
probe around `geom_core::sym::with_session_in`, each tree run alone):

| | wall | session body | publication and the `frozen` need | drop |
|---|---|---|---|---|
| before (main + PR 7394 + node-key) | 83.7 s | 148.6 s | 6.2 s | 32.2 s |
| after (the drive-owned table) | 83.7 s | 146.6 s | 6.4 s | 31.8 s |

Of the drop after, 19.6 s is the leaf's early forms (`Session::forms_early`,
24,464 forms per leaf on average), 0.2 s the reduction memo, and about
13 s is not attributed to a field. So the teardown M9 measured at
22.5 ms per replay is the early memo's: the tables that moved to the
drive were not what it cost, and keeping them warm across leaves
leaves the teardown where it was, within the run-to-run spread (one
repeat read 86.7 s wall).

**Why they are per leaf today.** An early form is a function of its
node's id, the tier's dials and the leaf's parameter brackets: the
decision read at `Select` / `min` / `max` (`sym::signed`) encloses a
form over the brackets and takes an arm. A form whose walk made no
such read, at its node or below, is the same form in every leaf of the
drive, and it is also built again in every leaf's body.

**Not measured:** what share of a leaf's early forms read a bracket;
what share of the body's time is the early walk.

**What it would take:** a mark on each early form saying whether its
walk read a bracket, and a drive-scoped early memo for the forms that
did not (the plain memo's argument, with the mark as the extra
premise). A new mechanism with its own soundness statement, so it is
a design fork and not a part of the session-state unit. The split
rule divides this cost with the replay count in any case.
