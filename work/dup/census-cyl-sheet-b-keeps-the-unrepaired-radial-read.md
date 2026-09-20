---
id: census-cyl-sheet-b-keeps-the-unrepaired-radial-read
kind: issue
title: cyl_sheet_b's descending rim still projects for its radial direction, the read its siblings were repaired away from
status: open
opened: 2026-09-19
---


## Finding

`crates/topo/src/census.rs`'s `cyl_sheet_b` builds its descending rim's
carrier `u_ref` by **projecting a chart point back onto the plane
through the axis**:

```
let s = at(u1, v) - center - axis * ((at(u1, v) - center).dot(axis));
```

Every sibling spelling of that rim was repaired away from exactly this
read. The `tests/` family carried the repair with a comment saying why
— *"the projection-based read cancels catastrophically for
tilted/small frames and mints a non-structural chart image"* — and the
shared door `cyl_wall_sheet`
(`crates/topo/src/test_support_fixtures.rs`) now states it as
`CylFrame::radial`, reading the direction from the frame's own fields.
`cyl_sheet_b`'s frame is tilted in azimuth (`u_ref` at 0.7 rad) and
axis-reversed, which is the family of frames the repair was for.

## Status

**Not measured to fail.** `cyl_sheet_b` runs at radius 1 and a
half-radian window, where the cancellation is not fatal, and its rows
pass. What this row records is that one member of a repaired class
kept the pre-repair read — drift between copies, which is this
program's charter — and that nothing in the tree says which read is
authoritative.

## What a unit here owes

Re-take the read at the merge base, then either point `cyl_sheet_b` at
`CylFrame::radial` or state why the projection is right there. If the
repair matters, a row at a tilt where the two reads disagree is the
deliverable, not the edit.

## Why this row is not on `curved`'s slate

`crates/topo/src/census.rs` is `curved`'s territory. The finding is
drift between duplicate fixture builders, so it is filed with the rest
of that class on this program's slate; `curved` owns the file whenever
it wants the row.
