---
id: naming-still-reads-the-folds-lineage-through-a-hidden-door
kind: issue
title: The union's naming still reads the fold's lineage (doc-hidden topo::MemberLineage, NaryStep::lineage); once it reads Holdings the lineage goes crate-private
status: closed
opened: 2026-11-02
closed: 2026-11-03
pr: 6381
---



## Finding

The kernel's booleans publish `topo::Holdings` (per finished face, and
per discard row, exactly the member faces whose area it holds; the
n-ary door's per step in `topo::NaryStep::holdings`). The fold's
lineage is now only the verdicts' router (`given_pairs`,
`restated_rows` in `crates/topo/src/boolean/nary.rs`), but three
readers outside topo still name it, so it stays public behind
`#[doc(hidden)]` (`topo::MemberLineage`, `NaryStep::lineage`):

- `crates/editor-core/src/names/emit_union.rs`, `Fold` (`Fold::new`,
  `Fold::step` through `MemberLineage::operand_faces`, keying each
  discard's obstacles) and `Parents::of` (`member_faces` of each
  finished face);
- `crates/editor-core/src/eval/wire.rs`, the union's fold driver
  (`fold.step(.., &stepped.lineage)`);
- `crates/sweep/tests/face_face_touch.rs`, a row on which accumulation
  pieces a tangency verdict is routed to: a reading of the router
  itself, which can move to a crate-internal row when the type goes
  private.

The naming rewrite (PR 6381's rule, built on this item's parent
`a-union-whose-fold-glues-a-run-no-judgement-opened`) replaces the
first two with `NaryStep::holdings` (faces) and `Holdings::lost`
(obstacles, by discard row). Then `MemberLineage` and
`NaryStep::lineage` become `pub(crate)`, and the sweep row moves inside
topo or reads the given pairs' rows instead.

## Closed (2026-11-03, PR 6381)

The naming reads `Holdings`: a union's faces from the last step's
`NaryStep::holdings`, and every step's obstacles from
`Holdings::lost` by discard row (`emit_union`'s `Fold`). `MemberLineage`
is `pub(crate)` in topo and `NaryStep` carries no lineage; the sweep row
reads the step's holdings instead.
