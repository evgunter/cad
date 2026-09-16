---
id: topo-tests-geometric-cube-and-cube-into-are-one-sequence-twice
kind: issue
title: geometric_cube and cube_into are the same ninety-line Euler sequence written twice, in S-DUP's own working file
status: open
opened: 2026-09-16
---

## Finding

- **Where**: `crates/topo/tests/common/mod.rs` — `geometric_cube`
  (`:107–223`) against `cube_into` (`:495–592`), with
  `mapped_cube` (`:487–491`) the three-line wrapper over the second.
- **Importance**: medium
- **Confidence**: sure that they are one sequence; the remedy needs a
  decision
- **Raised by**: the style review of the `dup-brick` lane's PR (S-DUP),
  2026-09-16

Both build the unit cube through `mvfs` → three rim `mev`s → the bottom
`mef` with the same reversed-plane corner list → four struts in the same
order → four side `mef`s with the same `he1`/`he2` and the same
`first_side_he_plus` closing → `set_face_surface` on the surviving seed
face with the same top plane. After normalising the body receiver
(`&mut body` local vs the `body: &mut Body<f64>` parameter) the only
differences are:

- `T: Decide` against a hard `f64`;
- `c(x, y, z)` building a `Point3<T>` against `map(x, y, z)` supplying
  one;
- three `let f_* =` bindings that `cube_into` drops because it returns
  no key bundle;
- the comments;
- the trailing `describe_as_intersections`, which `cube_into` calls and
  `geometric_cube` does not.

That last one is why this **reconciles rather than merges**: the two do
not build the same body today, and `geometric_cube`'s consumers assert
on exactly what the missing step leaves behind — every edge still at the
scaffolding door, named by both at-rest rules
(`common::assert_every_chord_named_by_both_rules`,
`geometric_cube.rs`). Collapsing the two changes what those rows see, so
this is a full-review unit, not a mechanical one.

Executed, not read: `mapped_cube(Point3::new)` and
`brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0))` produce bodies whose
vertex, edge, face, half-edge, loop, surface, curve and point arenas are
each equal element for element — so `cube_into` under the identity map
is already reachable through the prism builder, and only
`geometric_cube` is a genuinely distinct fixture. That is the shape of
the answer: one Euler cube (the scaffold-keeping one) plus the prism
family, not three.

**This is S-DUP's charter shape inside S-DUP's own working file.** The
`dup-brick` PR named it only as a stated blind spot; a stated blind spot
is a work order, not an absolution.
