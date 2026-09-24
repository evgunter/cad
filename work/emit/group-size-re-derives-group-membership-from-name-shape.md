---
id: group-size-re-derives-group-membership-from-name-shape
kind: issue
title: group_resized re-derives fragment-group membership from name SHAPE while the emitters decide it from geometry; the from_tie merge and the split pass-through are where the two can disagree
status: open
opened: 2026-09-23
priority: P1
cost: D
---


`resolve::group_size` (`crates/editor-core/src/resolve/mod.rs`) decides
which rows belong to a vanished fragment's group by SPELLING: the rows
whose name is the fragment's base, bare or with one trailing `Fragment`
qualifier, at the same kind and minting node. The emitters decide the
group from GEOMETRY and descent: `emit_topo::name_fragment_group` and
`insert_ranked_or_tied` group by root operand entity (or seam pair),
and `name_split_faces` by `(root, side)`. Two implementations of one
membership rule, and they are known to disagree in two places:

- **The `from_tie` merge** (`names/defer.rs::put`). A group whose
  parent descends from a TIE is deferred into the tie lane, and the
  rows of every tied candidate's fragments land under the same names.
  Two tied parents each cut in two give two `Tied` rows of two, which
  `group_size` counts as a group of FOUR, where the emitter ranked two
  groups of two. A change to the tie's width then moves the count
  without either emitted group changing size.
- **The split pass-through** (`emit_topo::name_split_faces`). A face
  the split no longer cuts passes through under its UPSTREAM name, not
  `SplitFragment { side, parent }`. So a split fragment group that
  stops being divided counts `now = 0`, because the survivor's spelling
  is not the base's, even though the parent plainly still descends.
  The rung's text says only what the spellings count (it makes no claim
  about where the parent went), so the diagnosis is not false. But the
  number is not the emitter's number. The N3 `Merged` row is the same
  shape one level up: a merged survivor is spelled `Merged(..)`.

The fix is one membership rule with two readers: the emitter records
(or exposes) its group key, and the rung reads that key instead of
re-deriving it from the name's shape. Found by the code review of the
EMIT group-resized unit (PR 3115). This is a P1 because it is two
implementations of one logic.
