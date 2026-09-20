---
id: should-classify-replays-error-enum-arms-be-deleted
kind: issue
title: Should classify_replay's two error-enum arms be deleted? The precondition is discharged; the measurement is not
status: open
opened: 2026-09-20
---


## What

`drive::classify_replay` reads three things per node, in order: the
box-independent terminal classes, the node's escalation log, and then
two error-enum arms — `NodeErrorKind::Escalated { source, .. }` and
`ProfileLaneReplay`'s `StructureRefusalKind::Indeterminate(source)`.
Both hand their payload to `indeterminate(source)`, which applies the
terminal-sliver test; every other error kind takes `_ => Bisect`.

Those arms were kept because the log did not carry op-minted
escalations. **PR 2928 discharged that precondition** — the gate doors
put a predicate's own indeterminacy on the frame — so the question is
now open rather than answered, and this row is where it gets answered.

## The measurement so far, and it is a SEARCH, not a proof

Every mint of either kind is in `editor_core::eval::wire`
(`UnitVec3Error::Escalated` conversion, the revolution-axis refusal, and
the generic escalation conversion for `ProfileLaneReplay`), and all of
them run inside the `Bracket` `eval_node` opens around the op. So the
escalation is on the node's log and read (2) answers before read (3)
ever runs. Three independent searches — this lane's and both review
arms' — found no live path to either arm.

That says no path reaches them, not that none can, and a search is not
what licenses a deletion.

## Why deleting them is a BEHAVIOUR change, not a cleanup

**Read (2) and read (3) do not speak for the same escalation.** Read (2)
returns `escalations.first()` — the FIRST indeterminate outcome the node
recorded, in decision order. An arm returns the one the node's ERROR
carried, which is the LAST one, the one it failed on. On a node that
escalated once, recovered, and then failed on a second escalation, those
are different `Indeterminate`s with different margins, and
`indeterminate()` can classify one as a terminal sliver and the other as
a bisect.

That divergence pre-dates PR 2928 and is not introduced by it — but it
means "the arms are unreachable" and "deleting the arms changes nothing"
are different claims, and only the first has been measured.

## What closing this looks like

A red-first row that constructs a node reaching an arm — or proves none
can, by making the unreachability structural rather than incidental (a
`NodeError` that cannot carry an escalation its own log lacks) — then
the deletion, then the population digits for any leaf whose verdict
moves. `work/props/indeterminate-error-arms-sweep.md` is the sweep this
was split out of; it retires no variant and says why.
