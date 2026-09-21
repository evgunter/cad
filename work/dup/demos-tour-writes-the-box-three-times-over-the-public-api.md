---
id: demos-tour-writes-the-box-three-times-over-the-public-api
kind: issue
title: demos/tour writes the extruded box three times and one of them is already pub
status: open
opened: 2026-09-19
priority: P4
cost: E
---

## Finding

- **Where**: `demos/tour/src/bool_bodies.rs` (`pub fn slab`),
  `demos/tour/src/bossplate.rs` (`plate`), and
  `demos/tour/tests/verbs_teapot.rs` (`boxy`).
- **Importance**: low
- **Confidence**: sure about the three; the tour was not censused
  beyond the box shape. **`demos/tour/src/bodies.rs`'s `plate` is NOT
  one** — it is a rectangle with two circular holes, so it is not this
  fixture; an earlier draft of this row listed it and was wrong
- **Raised by**: the `dup/private-box-builders` lane, 2026-09-19

Three demo sites each spell *rectangle profile → extrude z0..z1*.
One of them, `bool_bodies::slab<S: Scalar>(x, y, z, tol)`, is already
`pub` and already the general form, so the other two are its special
cases written out — `bossplate::plate` a 4x4x1 at the origin, and
`verbs_teapot::boxy` a 0.2x0.3x0.25 authored through the `.line_to`
chain. The third sits in `demos/tour/tests/`, which is the tour's own
suite rather than the corpus, so a home has to serve both.

**The fixture doors are not the answer here and must not be proposed
as one.** `memories/demo-purpose.md` and
`docs/prompts/implementer-discipline.md` §3 both say the demos render
what the kernel produces *through the public API, from an outside
consumer's seat*; `sweep::test_support` is not a door a consumer has.
The lane that found this deliberately left all of `demos/`,
`benches/benches/kernel.rs` and `crates/pncad/tests/all.rs` alone for
exactly that reason. So the remedy is a **demo-side** home — one of the
three, or a new module beside them — and nothing that reaches into a
test-support feature.

`demos/tour/` is claimed by no open program, so this sits on S-DUP's
slate as a duplication finding with no other owner.
