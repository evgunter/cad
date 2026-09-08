---
id: shell-doors-still-walk-the-whole-body
kind: issue
title: the simultaneous offset doors still walk the whole body around a scoped solve
status: open
opened: 2026-09-08
---


SHELL-8 made the two simultaneous offset doors solve and write over a
SCOPE — the solids a move set touches — so a scoped call offsets and
re-authors nothing outside it (`crates/sweep/tests/shell8_multi_solid.rs`,
`a_simultaneous_door_moves_one_solid_and_leaves_the_other_bitwise` and
`the_lift_re_authors_only_the_designated_faces_solid`; R1's and R2's
deep-comparison rows). Three whole-body walks remain AROUND that solve,
and the unit disclosed them rather than moving them: they are reads and
re-derivations, not offsets, and narrowing them is its own change with
its own evidence.

1. **The scope's own construction is a whole-body structural walk.**
   `crates/topo/src/offset_together.rs`, `Scope::of_solids` — it walks
   every shell, face, loop and half-edge of the body to build the
   face/edge/vertex → solid partition, and returns `None` (which both
   doors raise as `ReplaceFaceError::Corrupt`) for a corrupt entity on
   ANY solid, including ones the moves do not name. A move set about
   one solid therefore refuses on another solid's corruption. The maps
   do not depend on the scope — `Scope::re_scope` exists precisely
   because they do not — so a lazy or per-solid construction is
   available; what it costs is that a caller with N scopes over one
   body would walk N times unless the partition is shared, which is
   what the shell verb already does.

2. **`mint_pcurves` runs over the whole clone.**
   `offset_together.rs` and `crates/topo/src/offset_axial.rs`, at the
   end of each door's mutation phase. It clears every pcurve row of the
   body and re-mints them face by face. On the fixtures this workspace
   builds the re-derivation is bit-identical for an out-of-scope face
   (measured: the SHELL-8 differentials, 3014 dump lines), so nothing
   moves — but a face whose chart cannot mint would refuse a call about
   a solid it has nothing to do with, the same shape as (1).

3. **`validate_closed` runs over the whole clone**, same two sites,
   immediately after. A read, and the cheapest of the three to leave
   whole-body, since closure is a property the door should not be able
   to break anywhere; listed for completeness.

What would close this: a pcurve pass and a closure check restricted to
the scope's faces, and a scope construction that walks only the solids
it names (or is built once and shared, as `shell_open` now does with
`Scope::re_scope`). The evidence a change here owes is the same
differential the unit took — the dump corpora at the merge base and the
head, diffed line by line.
