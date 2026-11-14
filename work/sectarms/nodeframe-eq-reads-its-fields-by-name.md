---
id: nodeframe-eq-reads-its-fields-by-name
kind: issue
title: NodeFrame's hand-written PartialEq reads its fields by name, so a field added to it falls outside equality silently
status: open
opened: 2026-11-14
priority: P4
cost: E
---


Filed by GREEN (`green/censuses`, unit `nightly-reds-the-source-censuses`), from nightly 43371638371 (main `e36675c1`, 2026-11-13).

## What

`impl PartialEq for NodeFrame` (`crates/geom-brep/src/pair_section/spline_implicit/sweep/critical.rs`, beside the struct) compares bit for bit by reading `f.point`, `f.normal`, `f.lines`, `self.wall_above` and `self.width` by name. Nothing ties that list to the declaration: a sixth field added to `NodeFrame` would compile and sit outside equality. The test-utils census `hand_written_impl_census::every_hand_written_walk_is_held_to_its_declaration` reports it (`reads self.wall_above with no exhaustive pattern`), and `the_walk_still_sees_every_file_holding_one` reds on the file as a new arrival.

The impl arrived with PR 6811's merge (`ee63af5d67`, `sectarms/nurbs-implicit-arm-stage3b`): the file at its first parent has no `impl PartialEq for NodeFrame`, and at the merge it has one.

## Pinned meanwhile

GREEN pinned it in the census's `KNOWN_HAND_LISTED`, keyed `(critical.rs, PartialEq, NodeFrame)` and naming this row, and added the file to `IMPL_FILES_TODAY`. `every_known_hand_listed_impl_is_still_found` reds the day the impl stops reading by name, which is the cue to delete the entry and close this row.

## Owed

Destructure both sides exhaustively (`let Self { point, normal, lines, wall_above, width } = self;`, the same for `other`) and compare the bound names, so a new field is E0027. Then strike the `KNOWN_HAND_LISTED` entry.

Reproduce:

    cargo nextest run -p test-utils -E 'test(hand_written_impl_census)'
