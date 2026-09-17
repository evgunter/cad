---
id: topo-tests-geometric-cube-and-cube-into-are-one-sequence-twice
kind: issue
title: geometric_cube and cube_into are the same ninety-line Euler sequence written twice, in S-DUP's own working file
status: closed
opened: 2026-09-16
branch: dup/cube-sequence-reconcile
closed: 2026-09-16
pr: 2727
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

## Claimed by S-DUP (2026-09-16), and promoted to the next unit

Claimed off S-TINT's slate because it is the **first link of three** in
`work/dup/brick-has-two-constructions-and-two-homes.md`'s adjudication,
not merely because it is a duplication. The measurement that row now
carries established that `line`, `plane` and `describe_as_intersections`
are called by all three of this file's builders, so any move of the
fixture family down into `crates/topo/src/test_support_impl.rs` drags
them along — and moving `geometric_cube`/`cube_into` as they stand
would relocate this duplication rather than close it. **A relocated
duplication is a second copy with a forwarding address.**

So this reconciles first, and it is worth doing whether or not the move
ever happens.

**Full review tier** (`work/dup/plan.md`): it reconciles rather than
merges. `geometric_cube` stops before `describe_as_intersections` and
`cube_into` does not, and `geometric_cube`'s rows assert on exactly
that absence — `assert_every_chord_named_by_both_rules` is about the
conventional chords a body keeps when the step is skipped. So a
careless unification changes what those rows measure, which is the
definition of a unit whose diff can be green and wrong.

## Closed (2026-09-16, PR #2727)

One private `cube_ops`; `geometric_cube` calls it without
`describe_as_intersections` and `cube_into` with it. **A `describe:
bool` parameter was rejected by the lane and the argument is the
keeper**: a flag makes the difference between the two doors a literal
at a call site rather than a line, and the whole risk here was a reader
not noticing that step.

**The axis held, measured both ways.** Thirteen bodies dumped as
`format!("{body:#?}")` and SHA-256 compared before and after: all
identical. The reviewer re-took it independently by executing at both
SHAs. `geometric_cube` keeps 12/12
`Scaffold(ExtrudedPoint …)`/`Declared`; every other door is 12/12
`Intersection`/`Derived`.

**X4 done by enumeration, which is what the sibling unit's failure
taught.** Every sibling door dumped and grouped by hash rather than
checking the neighbour that came to mind: **five doors, one body**
(`brick`, `prism`, `prism_z`, `mapped_cube`, `cube_into` into a fresh
body), checked away from the unit ranges so the agreement is a fact
about the domain. `review_m2_pr7`'s private sixth copy folded in and
was deleted.

**A guard, because the unit's doc claim outran its measurement.**
`crates/topo/tests/cube_doors_agree.rs` pins the identity at four boxes
**with `geometric_cube` as the negative row in the same file** — a test
that only pins agreement goes green if someone makes every door
identical by deleting the distinction, which was this unit's nearest
failure mode. Both halves verified by mutation, each catching its own
and not the other's; the mutations also red 23 and 4 pre-existing rows
respectively, so what the new rows add is the **silent** case — a
re-ordering or re-keying that leaves every count and every consumer's
verdict intact.

### What this unit did not do

`cube_ops` and `prism_z` are **the same builder**, hand-traced at
`n = 4` by the review, and `prism_z`'s own doc has said so since
`0765b4617`. This unit measured that and did not act on it. The
justification it first shipped for keeping two cores was withdrawn on
review: *"`prism_z` can neither write into an existing body nor take a
tilt map"* described a signature this unit had just changed on the
other function, and three of four direct `cube_into` call sites pass a
diagonal affine rather than a shear. Carried as link 1b of
`work/dup/brick-has-two-constructions-and-two-homes.md`, ahead of the
tolerance threading so that `tol` is threaded through one builder
family rather than two.
`work/tint/topo-one-builder-subsumes-the-cube-and-prism-sequences.md`
is the row, and it says to **extend the guard before the
unification**, since it samples `f64`, four corners, no reflex profile
and no non-diagonal map.

### Residue, each with its own file

- `work/tint/topo-one-builder-subsumes-the-cube-and-prism-sequences.md`
- `work/tint/topo-prism-z-construction-is-written-out-again-across-the-tree.md` — `cube_ops` is listed in it as a fifth member, and as the only one whose equality is proved
- `work/tint/topo-tests-mapped-cube-call-sites-that-are-brick-with-ceremony.md` — visible only once the equality was measured
- `work/tint/topo-tests-cube-door-roster-is-a-hand-written-partition-claim.md`
- `work/dup/topo-src-cert-m3r1-probes-holds-an-in-src-copy-of-the-cube-family.md` — unblocks with link 3 exactly, which is now the better argument for that link than the brick was
- `work/dup/topo-tests-review-m2-pr7-rederives-the-shared-cube.md` — closed by this unit, and moved onto this slate so the board says who closed it
