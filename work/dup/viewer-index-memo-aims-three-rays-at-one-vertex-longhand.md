---
id: viewer-index-memo-aims-three-rays-at-one-vertex-longhand
kind: issue
title: Three longhand aimed-vertex rays and three copies of one reach constant in index_memo
status: open
opened: 2026-09-20
---


## Finding

- **Where**: `crates/viewer/tests/index_memo.rs`, three rows that aim a
  ray at a known ring vertex from a fixed standoff — the rows named
  `a_wide_candidates_interval_reaches_the_aimed_vertex_and_the_tie_break_takes_it`,
  `a_wide_candidates_interval_reaching_the_aimed_vertex_refuses_with_both`
  and the corner row above them. Each writes `let reach = 1.48;` and
  then `Ray { origin: Point3::new(p.x, p.y ∓ reach, p.z), dir:
  Vec3::new(0.0, ±1.0, 0.0) }` longhand.
- **The construction, which is one**: a ray that meets `p` at exactly
  `t = reach`, travelling along ±y. Two of the three aim from −y, one
  from +y; the origin's sign follows the direction, so one helper
  `aimed_at(p, dir)` covers all three.
- **Why it is not folded already**: `reach` is load-bearing INSIDE each
  row — `t.to_bits() == reach.to_bits()`, `wide.t < reach`, and three
  format strings name it — so a fold has to keep a binding with that
  name in scope, not just replace the `Ray` literal. That is a
  judgement about a bit-exact suite, which is why the unit that found
  it declined rather than doing it in passing.
- **Importance**: low. No oracle rides on the construction: a wrong
  origin reds the row that aimed it, loudly, on a `to_bits` compare.
- **Instrument, and its blind spot**: `git grep -n 'Ray {' --
  crates/viewer/tests/`, then a read of each hit. It cannot see a ray
  built by a method or a `From` impl (none exists in this tree,
  measured: `git grep 'Ray::' -- crates/` returns only
  `Ray::slab_enter`, a reader), nor one assembled in a loop from a
  table.
- **Raised by**: the S-DUP lane closing the four viewer-suite door rows,
  2026-09-20, measured at `cd9fdfd6b`.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` (`work.py territory`), so there is no single ground-owner to
file it with, and the subject — one construction spelled more than once
— is S-DUP's charter. Any of the five may claim it by `git mv`.

