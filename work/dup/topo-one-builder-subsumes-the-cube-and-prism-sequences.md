---
id: topo-one-builder-subsumes-the-cube-and-prism-sequences
kind: issue
title: cube_ops and prism_z are one builder written twice; tprism is already the union of them
status: closed
opened: 2026-09-16
branch: dup/one-prism-builder
closed: 2026-09-18
pr: 2812
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

## Done (2026-09-17, `dup/one-prism-builder`)

One `common::prism_ops(body, profile, z, map) -> PrismOps`, with
`prism_z`, `prism`, `brick`, `geometric_cube`, `cube_into`,
`mapped_cube`, `review_m3_pr55`'s `tprism` and
`review_m3_pr3_consumer`'s `add_quad_prism` as its callers — eight, not
the six the sketch named, because `add_quad_prism` turned out to be a
member too and to be inside the same fence.

**The guard was extended first and watched pass on unmodified
builders**, at the merge base, before anything was unified. What it
samples now, beyond the four axis-aligned `f64` boxes it had: a
triangle, a pentagon and a reflex hexagon; a shear with determinant
1.015625; and `Interval`. The row it gained is the one that does not
compare doors to each other — it re-derives the body from the profile,
the z-range and the map, which is the only row that can survive the
doors becoming one function. Mutation-checked three ways, the last of
which is the case the file exists for: a re-ordering planted inside
`prism_ops` leaves **every door still agreeing with every other** —
`every_box_door_builds_one_body` stays green — while the independent
row reds.

It is **not** the only row that reds on it, and an earlier draft of
this paragraph said it was. The same mutation also reds three
pre-existing arena-order-sensitive rows:
`bool4_material_containment::the_embedded_cube_is_a_decided_interference`,
`bool4r2_probes::embedded_witness_is_the_fifth_vertex_in_arena_order`
and `mate4a_ef_bound_rung::the_bare_straddle_seat_is_untouched`. Four
rows in the default lane, not one. What the new row adds is not that
the mutation is caught at all but that the failure **names the
builder** instead of arriving as three unrelated fixture surprises.

`geometric_cube` still keeps its scaffolding: 12/12
`Scaffold(ExtrudedPoint …)`/`Declared`, measured before and after, not
reasoned about. The `describe: bool` flag stayed rejected for link 1's
reason.

The residue this row named is settled: `cube_into` is still `f64`-only
while the core is generic, unchanged here and still worth revisiting
once the family has a home in `src/`.

## Closed (2026-09-18, PR #2812)

One `prism_ops`; **eight callers**, not the six this row predicted —
`prism_z`, `prism`, `brick`, `geometric_cube`, `cube_into`,
`mapped_cube`, `tprism`, and `add_quad_prism`, which the lane folded in
beyond its brief on the correct judgement that leaving a proven-equal
copy standing was the worse error. `tprism` needed **only** the map,
which is the cleanest evidence for the factoring, and it has no describe
step — a second independent witness that the step belongs at the caller.

**Faithfulness measured at both ends**: 105 661 lines of derived-`Debug`
dump compared byte-for-byte across all eight callers either side of the
unification, by the lane and again independently by the review.
`geometric_cube` keeps 12/12 `Scaffold(ExtrudedPoint …)`/`Declared`.

**The guard was extended first and passed on unmodified `main` before
anything moved** — a guard written after the fix is a guard written to
pass the code that was just written.

### What a shared home cost, and how it was paid back

This unit's most valuable result is not the unification. Link 1's guard
compared doors **against each other**, which silently catches a surface
regression in any one of them; once the doors share a core, a change
that moves every door alike is invisible to that shape. The lane saw it
and added an independent row re-deriving the body from
`profile`/`z`/`map`. **The review then measured what that row still did
not cover**: it never read a surface, so `plane(&rev)` → `plane(&bot)`
reds `every_box_door_builds_one_body` at the merge base and **nothing**
at head.

The fix pass closed it rather than narrowing the claim:
`assert_prism_shaped` now reads **every face's outward normal**, stated
from the corners and compared by certified sign — top cap along the
strut vector, bottom against it, side *i* along
`(bot[j] − bot[i]) × (top[i] − bot[i])`, all three carrying the
determinant's sign through an affine map because `(M⁻ᵀa)·(Mb) = a·b`.
Both of the review's mutations now red it. The header also names the
axis that **remains** uncovered — no row here reads carrier geometry —
because stating a partial oracle's boundary is part of the fix and not a
substitute for it.

**Generalisation for this program: when a unit gives n spellings one
home, ask what the n-way comparison was silently buying, because the
shared home cannot buy it back.**

### The precondition that was not one

The orchestrator asked for a signed-area check to enforce `prism_ops`'
documented counterclockwise winding. The lane tried it and **it red
`review_m3_pr55::b_transformed_pocket_exact`**, which passes a reversed
profile with a reflecting matrix *on purpose*. So winding alone is not
the law; the composite `sign(winding)·sign(det M)` is. It tried that,
and **that red four rows of `review_m2_pr7`**, whose subject is a
deliberately mirrored cube asserting that tiers 1–2 cannot see
orientation while `mass_properties` can.

**So an inside-out prism is a first-class fixture and a refusal in the
builder would be wrong.** No assertion was kept. What landed instead is
the truth: of four documented preconditions the builder enforces one,
the winding rule is **not** a precondition and must not become one, and
simplicity/no-repeats are unchecked and the caller's. The place the rule
*is* enforced is `cube_doors_agree.rs`, which the same fix pass put
there. The negative is written at the claim site **with the two suites
that establish it**, so the next lane does not re-run the experiment.

Two red suites were the price of learning this, and they were worth it:
a request from the orchestrator is a hypothesis like any other.

### Residue

- `work/dup/thread-the-tolerance-through-the-prism-fixture-family.md` — link 2, which this row now gates
- `work/tint/tests-common-body-fixtures-triplicated.md` — **extended, and re-counted**: the shared profile literal is **30 occurrences across 28 files**, not the seven the review named. Reconciling the three `tests/common` copies leaves 28 standing, which the reconciliation must know before it is scoped
- `work/tint/topo-tests-has-two-vocabulary-homes-with-no-stated-boundary.md` — new; link 3 must decide which home moves
