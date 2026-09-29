---
id: the-standing-ladder-speaks-as-a-hit-test-at-every-door
kind: issue
title: The standing arms of HitTestError say "hit test:" through NodePick::build and the name doors, where no hit test runs
status: review
pr: 3463
branch: edit/node-standing-one-type
opened: 2026-09-29
priority: P4
cost: M
rides_with: C6
---

Filed by `edit/unnamed-patch-is-a-lookup`
(`work/edit/an-unnamed-patch-is-reported-as-a-hit-test.md`), whose
sweep for the shape *a `HitTestError` sentence out of a door that ran
no hit test* found this sibling and left it, because the row's ruling
was about the per-entity lane and this is the call's.

## The finding

`HitTestError`'s three standing arms (`NodeNotEvaluated`,
`NodeFailed`, `NodePoisoned`; `crates/editor-core/src/resolve/hit.rs`,
the `Display`) each open *"hit test: node N …"*. They are raised by
`standing`, the one ladder every door of `resolve/hit.rs` and
`resolve/pick.rs` climbs, and two of those doors run no hit test:

- `NodePick::build` / `build_all` (`crates/editor-core/src/resolve/pick.rs`,
  `standing_value`) carry the refusal as `NodePickError::Standing`,
  whose `Display` forwards it verbatim, so a pick index that could not
  be built over a failed node reads *"hit test: node 4 failed, so it
  has no name table to invert"*. The viewer's banner forwards that
  sentence (`crates/viewer/tests/frame_policy.rs`, the
  `"pick index: root 99's bodies could not be tessellated or indexed:
  hit test: node 99 …"` pin).
- `NodePick::patch_names` / `boundary_names` (same file, `names_of`)
  refuse the call with the same arms for a later evaluation of the
  same document in which the node has no table.

The lookup's own refusal (`UnnamedEntity`) already names a lookup and
no hit test; the standing refusal is the remaining sentence of this
shape.

## What a fix would be

The standing vocabulary becomes a type of its own, whose `Display`
names the standing and not a door (*"node N failed, so it has no name
table"*), and `HitTestError`, `NodePickError::Standing` and the name
doors carry it, each under its own prefix where one is true. The
Python binding's tags for the three arms (`crates/pncad-py/src/tags.rs`,
`hit_test_error_tag`, forwarded by `node_pick_error_tag`) are the
stable words and do not move.

## Fence

`crates/editor-core/src/resolve/hit.rs` and `resolve/pick.rs` (EDIT);
the forwarding readers are `NodePickError`'s `Display` (same file)
and the viewer's `PickIndexError` (`crates/viewer/src/pickindex.rs`),
which forward unchanged.

## Ruled (2026-09-29, EDIT orchestrator) — rides with C6

The standing becomes one type whose `Display` names the standing and no
door. That is the "no usable value" member of `C6`, specced there. This
row closes when that unit merges.

## Built (2026-09-29) — PR 3463

The standing is `eval::NodeStanding`, whose `Display` names the node,
its state and where the repair is, and no door. `HitTestError::Standing`
renders it as `hit test: {standing}`; `NodePickError::Standing` (the
`NodePick::build`/`build_all` refusal) as `pick: {standing}`; and the
name doors' refusal of the whole call is its own type,
`NameLookupError`, rendering `name lookup: {standing}` — its pairing
arm says `name lookup:` too, where it used to say `hit test:`. The
viewer's `PickIndexError::Names` carries `NameLookupError`, and the
`frame_policy.rs` badge pin is re-baselined. The Python binding raises
the name doors' refusal as `HitTestError` under the same tags
(`node_standing_tag`, `name_lookup_error_tag`), with the lookup's own
message. Rows: `crates/editor-core/tests/node_standing.rs`
(`no_standing_refusal_says_hit_test_outside_the_hit_test`,
`every_standing_renders_one_way_through_every_door`).
