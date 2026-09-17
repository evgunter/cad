---
id: topo-one-builder-subsumes-the-cube-and-prism-sequences
kind: issue
title: cube_ops and prism_z are one builder written twice; tprism is already the union of them
status: spec
opened: 2026-09-16
branch: dup/one-prism-builder
---


## Finding

- **Where**: `crates/topo/tests/common/mod.rs` — `cube_ops` (~`:121`,
  behind `geometric_cube` and `cube_into`/`mapped_cube`) against
  `prism_z` (~`:341`, behind `prism` and `brick`).
- **Importance**: medium
- **Confidence**: **sure**, and unusually so for a duplication row —
  hand-traced operator by operator, and equality proved by execution
- **Raised by**: the full review of PR #2727 (S-DUP), 2026-09-16
- **Refs**: `topo-prism-z-construction-is-written-out-again-across-the-tree`

`cube_ops` is `prism_z` at N = 4. Traced step for step: same `mvfs`,
same `MevSite::Lone` seed edge, same `MevSite::Fan` rim chain, same
`find_half_edge(seed.face, v3, v2)`, same reversed bottom corner list
for the bottom Newell plane, same strut anchors **including the
`f_bottom.he_plus` special case at the last corner**, same four side
planes, same `first_side_he_plus` closing for the last side face, same
`set_face_surface` over the top ring. The only structural difference is
that `prism_z` spells the rim, the struts and the sides as loops over N
and `cube_ops` unrolls them at four; and that `cube_ops` returns the
key bundle and leaves `describe_as_intersections` to the caller while
`prism_z` calls it and returns a `Prism`.

**`prism_z`'s own doc has said this since `0765b4617`**: *"the
geometric_cube construction generalized to N corners (reflex corners
welcome)"*. The relationship was documented before it was measured; PR
#2727 measured it (group A: `brick`, `prism`, `prism_z`, `mapped_cube`
and `cube_into` produce byte-identical derived-`Debug` dumps) and did
**not** unify it.

## Why PR #2727 did not do it, and why that reason is not a blocker

That PR's body argued `cube_into` could not go over `prism_z` because
`prism_z` can neither write into an existing body nor take a tilt map.
Both halves are weaker than they sound:

- *"cannot write into an existing body"* describes `prism_z`'s current
  signature, and adding `&mut Body` + a point map is the same ten-line
  edit that PR performed on `geometric_cube` to produce `cube_ops`.
- *"every `cube_into` call site uses a tilt"* is **false**. Of the four
  direct call sites, three pass a diagonal affine —
  `review_m3_pr6.rs:260` `(1+x, 3y, z)`, `:290` `(x, y, 1+z)`,
  `m3_pr6_tier3prime.rs:336` `(1+2x, 1+2y, 1+2z)` — and only
  `review_m3_pr6.rs:334` `(1+x-y, x+y, 2+z)` is a shear. One shear is
  enough to need a general point map; it is not four.

## The shape, and the pointer worth having

**`crates/topo/tests/review_m3_pr55.rs`'s `tprism` (~`:35`) is already
the union.** Its doc: *"a right prism over `profile` x [z0, z1] pushed
through the linear map `m`"* — `prism_z`'s N corners and z-range, plus
`cube_ops`' arbitrary point map. It is generic over `Decide`. What it
lacks is the `&mut Body` seat, the key bundle and the caller's choice
about the description step; those are the same three edits `cube_ops`
already carries. So the target signature is roughly

```
fn prism_ops<T: Decide>(body, profile: &[(f64,f64)], z: (f64,f64),
                        map: impl Fn(f64,f64,f64) -> Point3<T>) -> PrismKeys
```

with `prism_z`, `prism`, `brick`, `geometric_cube`, `cube_into` and
`mapped_cube` as its callers, and `tprism` folding in as a seventh.

## What makes this safe to attempt

`crates/topo/tests/cube_doors_agree.rs` (PR #2727) is the guard:
five doors, four boxes, compared on the full derived `Debug`, with a
negative row pinning that `geometric_cube` does NOT join them. A
unification that changes any body reds it. **Extend the guard before
the unification, not after** — it samples `f64`, four corners, no
reflex profile and no non-diagonal map, and a builder that subsumes
`prism_z` has to hold on all four of those.

**Do not take this as a mechanical merge.** `geometric_cube`'s missing
description step is the axis its whole suite rides on, and the
conditional shape that keeps it has to survive N-corner generality.
