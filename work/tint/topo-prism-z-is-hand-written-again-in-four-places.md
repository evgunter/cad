---
id: topo-prism-z-is-hand-written-again-in-four-places
kind: issue
title: prism_z is hand-written again in four places, two of which say so in their own docs
status: open
opened: 2026-09-16
---


## Finding

- **Where**: `crates/topo/src/splitting/reassembly.rs` (`quad_prism`,
  ~`:44`), `crates/topo/tests/review_m3_pr3_consumer.rs`
  (`add_quad_prism`, ~`:287`), `crates/topo/tests/review_m3_pr55.rs`
  (`tprism`, ~`:35`), `crates/topo/tests/review_m2_pr3.rs`
  (`triangle_prism`, ~`:48`) — against
  `crates/topo/tests/common/mod.rs`'s `prism_z` (~`:335`).
- **Importance**: medium
- **Confidence**: sure they are the same construction **by shape, not
  by execution**; **unmeasured** whether any two produce equal bodies
- **Raised by**: the `dup-cube-seq` lane (S-DUP), 2026-09-16

All four are `prism_z`'s construction: profile corners lifted to two
z-levels, a bottom rim chain of `mev`s, the bottom `mef` over the
reversed corner list, one strut per corner, one side `mef` per profile
segment closing against the first side face's top edge, the seed face
capped by `set_face_surface`, and `describe_as_intersections` last.
Three of the four match `prism_z`'s call-site counts exactly —
`{mvfs: 1, mev: 3, mef: 2, MefSite::Chords: 2, MevSite::Fan: 2,
set_face_surface: 1}`.

**Two of them say it themselves.** `quad_prism`'s doc: *"the
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

So the honest shape, **if the measurement comes back equal**, is
probably `prism_z`'s own `cube_ops` treatment:
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
