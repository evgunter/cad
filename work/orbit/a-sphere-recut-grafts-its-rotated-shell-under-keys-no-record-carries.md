---
id: a-sphere-recut-grafts-its-rotated-shell-under-keys-no-record-carries
kind: issue
title: apply_recuts re-keys a re-charted sphere shell through a graft whose map it drops, so the boolean's naming record never sees the operand's keys for it
status: open
opened: 2026-10-14
priority: P3
cost: M
---

Found reviewing EMIT's PR 5039 (`emit/naming-refusals`); not reached by
any document there.

## What

`topo::boolean::ops::apply_recuts` (ops.rs ~5894) rebuilds each re-cut
operand and `adopt`s the rebuild in the operand's place. It `carve`s the
kept shells out of the operand, which keeps their keys, and rotates each
re-charted sphere shell with `transform::transform_rigid`, which clones
and so keeps them too. But every rotated shell after the first, and
every one when a shell is kept, joins the rebuild through
`combine::graft_solids_disjoint`, a `Bridge::RemapKeys` graft, and its
`GraftMap` is dropped (`graft_solids_disjoint(base, &[base_solid],
&turned)?;`). Those faces, edges and vertices land under keys the
operand never had.

The comment at the call (`through_the_join`, "the re-charts
carve and graft other shells, and `carve` keeps every kept entity's
key") covers the carved shells only. The boolean that follows records
its naming rows (`BooleanNaming`: parents, fragments, discards, chart
cuts) in `a2`'s and `b2`'s keys, and the naming emitter reads them as
the operand's own (`OperandKeys::Direct`). No row says which operand
entity a grafted key is.

## Owed

- Check whether a document reaches it: an operand with a re-charted
  sphere shell beside another shell, or two re-charted shells.
- If so, carry the graft's map into the record (as `graft_edges` does
  for B's graft), or re-key the rows back to the operand's keys before
  they leave the op, and pin the result's names against the operand's
  in that document.

