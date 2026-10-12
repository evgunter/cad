---
id: main-reds-on-the-impl-census-since-plane-nurbs
kind: issue
title: Main is red on test-utils hand_written_impl_census::the_walk_still_sees_every_file_holding_one: ssi/plane.rs holds a hand-written impl and is not in IMPL_FILES_TODAY
status: closed
opened: 2026-10-11
priority: P0
closed: 2026-10-11
---


Found by CONTACTHOLD's PR 4694, whose diff touches neither geom-brep nor
test-utils. Hosted CI run 38144565191 (head `db3bed398d`, merged with
main) fails
`test-utils hand_written_impl_census::the_walk_still_sees_every_file_holding_one`:
the census finds a file holding a hand-written impl that is not in
`IMPL_FILES_TODAY`, namely `crates/geom-brep/src/ssi/plane.rs`.

That file came in with main's plane × NURBS commits (`c2650d56af`,
`06c62ff81a`). Merging current main into any branch reproduces the
failure locally. Every PR whose CI selects test-utils is red on this row
until the census list names the file (or the impl moves to a covered
file).

Owed: add `ssi/plane.rs` to `IMPL_FILES_TODAY` with whatever coverage the
census expects of its impls, or derive the impl. Not bisected beyond the
two commits named.

## Closed

Fixed on main by be03d2357d ("test-utils: the hand-written impl census lists ssi/plane.rs (main red since #4677)"), 2026-10-11 14:30Z. PR 4677 added `ssi/plane.rs`'s hand-written `Debug` impl. Its own CI never selected test-utils, so the census row went red only on main.
