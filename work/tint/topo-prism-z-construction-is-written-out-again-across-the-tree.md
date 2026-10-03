---
id: topo-prism-z-construction-is-written-out-again-across-the-tree
kind: issue
title: prism_z's construction is written out again elsewhere in the tree, including once inside its own file
status: open
opened: 2026-09-16
priority: P3
cost: E
---


## Finding

- **Where**: `crates/topo/tests/common/mod.rs`'s own `cube_ops`
  (~`:121`), `crates/topo/src/splitting/reassembly.rs` (`quad_prism`,
  ~`:44`), `crates/topo/tests/review_m3_pr3_consumer.rs`
  (`add_quad_prism`, ~`:287`), `crates/topo/tests/review_m3_pr55.rs`
  (`tprism`, ~`:35`), `crates/topo/tests/review_m2_pr3.rs`
  (`triangle_prism`, ~`:48`) — against
  `crates/topo/tests/common/mod.rs`'s `prism_z` (~`:335`).
- **Importance**: medium
- **Confidence**: sure they are the same construction **by shape, not
  by execution**; **unmeasured** whether any two produce equal bodies
- **Raised by**: the `dup-cube-seq` lane (S-DUP), 2026-09-16

All of them are `prism_z`'s construction: profile corners lifted to two
z-levels, a bottom rim chain of `mev`s, the bottom `mef` over the
reversed corner list, one strut per corner, one side `mef` per profile
segment closing against the first side face's top edge, the seed face
capped by `set_face_surface`, and `describe_as_intersections` last.
Three of them match `prism_z`'s call-site counts exactly —
`{mvfs: 1, mev: 3, mef: 2, MefSite::Chords: 2, MevSite::Fan: 2,
set_face_surface: 1}`.

**`cube_ops` is a member of this class, and it is the one member whose
equality is already PROVED.** It is the cube half of
`crates/topo/tests/common/mod.rs`, sitting in the same file as
`prism_z`, and `prism_z`'s own doc has named the relationship since
`0765b4617`: *"the geometric_cube construction generalized to N
corners"*. At N = 4 they build equal bodies — group A of PR #2727's
measurement, and now a standing row in
`crates/topo/tests/cube_doors_agree.rs`. That unification is its own
unit (`topo-one-builder-subsumes-the-cube-and-prism-sequences`); it is
listed here so that this row is a census of the class rather than of
the class minus its nearest instance.

**Two of the others say it themselves.** `quad_prism`'s doc: *"the
tests/common builder's minimal in-crate copy"*. `tprism`'s: *"the
prism_z construction generalized to mapped corner points"* — which is
to `prism_z` exactly what `mapped_cube` is to `geometric_cube`, and
`mapped_cube` lives in `tests/common`.

## What distinguishes each from `prism_z`, as far as reading shows

- `quad_prism` — in `src/`, so it cannot name `tests/common` at all.
  Its only real difference is that it takes `tol: Tol` where `prism_z`
  reaches for `Tol::witness()`, which is the same signature change
  `work/dup/brick-has-two-constructions-and-two-homes.md`'s second link
  is about. It lands in the shared home for free once that link does.
- `add_quad_prism` — writes into an EXISTING body, as `cube_into` does
  for the cube. `prism_z` has no `prism_into`.
- `tprism` — a linear map over the corners, as `mapped_cube` is for the
  cube. `prism_z` takes bare `(f64, f64)`.
- `triangle_prism` — builds its points through a sketch placement
  (`Point2` + place), not from a profile list, and returns the `mef`
  bundle.

## This row owes a measurement before a fix

**The equality above is read off call-site counts and control flow, not
executed.** Nothing here has been run. That is exactly the unverified
claim `brick-has-two-constructions-and-two-homes` was opened to fix, and
the reason that row refused to be dispatched before its measurement is
that the answer **inverts the remedy**: the same construction means
share it, a different one means name them apart and retire the
duplication claim as false.

The instrument is cheap and has been run twice in this family already:
build each body and compare `format!("{body:#?}")` — `Body`'s derived
`Debug`, so every arena, key, slot version and `free_head` is in the
comparison and nothing is left out by the author choosing what to look
at. `crates/topo/tests/cube_doors_agree.rs` does it as a standing row
for the cube doors, and PR #2727's measurement did it as a throwaway for
the six-door group. Compare at more than one profile and more than one
z-range: two builders can agree at a square at the origin and disagree
on a reflex corner or an off-origin range, and `triangle_prism` is not
even in the other three's domain.

So the honest shape, **if the measurement comes back equal**, is the
treatment `cube_ops` got — which is not a model this row can point at
from outside, because `cube_ops` is in the class:
one sequence taking a body, a corner map and a z-range, with the
describe step and the key bundle at the caller. That is the same
factoring the `dup/cube-sequence-reconcile` unit landed for the cube
one file over, and the two families are neighbours in `tests/common`.

## Why no sweep had found it

`topo-tests-brick-copies` swept this file set with `\bfn brick\b`,
`fn [a-z_]*brick[a-z_]*` and `prism_z`, and closed at 66 spellings.
None of these four names `brick` or `prism_z`, so none of the three
instruments could reach them. What found them: parse every tracked
`.rs` into top-level `fn` bodies and count
`{mvfs, mev|mev_line, mef|mef_chord}` per body. **A construction grep
is still a name grep** when the construction is spelled out rather than
called — that is this row's lesson for the next census.

## Three of the five closed, one retired as false (2026-09-17, `dup/one-prism-builder`)

The measurement this row owed was taken, by execution, on the three
members inside `crates/topo/tests/`. All three are now callers of one
`common::prism_ops(body, profile, z, map)` — the N-general Euler
sequence with the describe step left to the caller.

| member | verdict | how |
| --- | --- | --- |
| `cube_ops` | **equal**, folded in | 42 door dumps (`format!("{body:#?}")`) across five profiles, three z-ranges, three boxes, a shear and a second solid — byte-identical before and after |
| `tprism` (`review_m3_pr55.rs`) | **equal**, folded in | 18 dumps over 3 profiles x 3 matrices (identity, shear, scale) x 2 z-ranges — byte-identical before and after |
| `add_quad_prism` (`review_m3_pr3_consumer.rs`) | **equal**, folded in | its two-solid body dumped before and after — byte-identical |
| `triangle_prism` (`review_m2_pr3.rs`) | **NOT equal** — retired from the class | dumped against `prism_ops` over the same triangle profile and z-range: they differ. Its rims are `MappedCurve::PlacedSegment` and its struts `ExtrudedPoint` with an explicit `place`/`vec`, where `prism_ops` mints `EdgeCurveSpec::line_between`. **That is what its suite measures** — the M2 PR 3 mini-extrude asserts on the sweep's own description forms — so it is a different fixture, correctly named for what it builds |
| `quad_prism` (`src/splitting/reassembly.rs`) | **residue** | in `src/`, so it cannot name `tests/common`; unblocks with link 3 of `brick-has-two-constructions-and-two-homes` exactly, as its own row says |

`tprism` and `add_quad_prism` are also the two members that prove the
factoring is the right one: `tprism` needed only the point map,
`add_quad_prism` only the `&mut Body` seat and the absence of the
describe step, and both are parameters of the one door.

**What this unit's sweep could not match.** The ladder census above was
re-run tree-wide at the merge base (`git grep -l -- 'mvfs('` with no
path argument, then `{mvfs, mev|mev_line, mef|mef_chord}` counted per
`fn`); it returns 33 bodies at `mvfs >= 1, mef >= 2`. It still counts
**call sites, not operators**, so a fixture that loops undercounts —
`prism_ops` itself reads `mev=3 mef=2` at any N. It cannot see a ladder
spelled through a helper of its own, and it cannot see one written
inside a `macro_rules!` body.

**And one reason given above for retiring `triangle_prism` was struck,
because it was not a difference.** It read: *"its bottom plane takes
`[c, b, a]` rather than the reversed-profile list."* For a profile
`[a, b, c]` the reversed list `prism_ops` builds is `[a, c, b]`, which
is a **cyclic rotation** of `[c, b, a]` — and `geom_brep::newell_plane`
anchors at the centroid and sums a cyclic cross product, so it is
rotation-invariant and both spellings give the same plane. The verdict
survives on the two substantive reasons (the `PlacedSegment` rims and
the `ExtrudedPoint` struts), which is what the dump comparison actually
turned on. Recorded rather than quietly deleted: a wrong reason in a
durable record is how the next census is misled, and the shape of the
error — reading a corner list as ordered when its consumer is
rotation-invariant — is worth a reader's guard.
