---
id: the-readme-splits-camera-rs-across-two-decision-rows
kind: issue
title: crates/viewer/README.md now describes src/camera.rs in two Where-in-the-code rows and neither inventory is complete
status: open
opened: 2026-09-06
refs: [2089]
---



Found by the style review of #2089.

`crates/viewer/README.md`'s *Where in the code* table now names
`src/camera.rs` in two rows with two disjoint inventories, and neither
is the file:

- the **G1 layer 3** row (`README.md:219`) —
  *"`src/camera.rs` (`Camera`, `CameraOp`, `camera::apply`)"*;
- the **GQ7 picking** row (`README.md:224`) —
  *"… and `camera::cursor_projection` (the id pass's 1×1 target
  transform, which is projection algebra rather than a mark)"*.

A reader of the G1 row gets a three-item inventory of a module that has
four free `pub fn`s (`apply` `:746`, `fold_recorded` `:817`, `fold`
`:852`, `cursor_projection` `:882`), a `Folded` struct, three error
enums and twenty-odd `Camera` doors; a reader of the GQ7 row gets a
member with no file path beside four entries that are all file paths
(`src/pickindex.rs`, `src/marks.rs`, `src/pickcache.rs`,
`crates/bvh`). The GQ7 row is the only place in that table that
addresses a member by module path rather than by file, which is a fifth
spelling convention inside one row.

The edit is correct as far as it goes — `cursor_projection` genuinely
belongs to GQ7's subject and not to G1's — and the alternative
(repeating the file in both rows) has its own cost. What is worth
flagging is that the move's own finding was *a module header quietly
counting a function as a fourth mark*, and the README now has the
mirror of that: a decision row quietly not counting a function that is
in the file it names.

**Where else to look**: every other file this table names in more than
one row. `src/session.rs` and `src/app.rs` are the obvious candidates,
and `crates/viewer/README.md`'s own *Module boundaries* section says
those two "hold most of it".

## Confidence

`sure` on what the two rows say and on the member lists. `likely` that
this is worth a sentence in the table's preamble — *"a file may appear
under more than one decision and each row lists only that decision's
members"* — rather than an edit to either row. `unsure` whether anyone
but a reviewer reads the table this way.
