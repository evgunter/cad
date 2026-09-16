# EDIT-PICK — a grazing ray's pick answer is decided by the geometry, not the candidate order (spec)

Binds one unit. Item:
`work/edit/pick-grazing-ray-answer-depends-on-candidate-order.md`.
Branch: `edit/pick-grazing`. Deleted post-merge with a
`docs/DOC-LEDGER.md` entry citing the merge SHA.

Read the item first, then `crates/editor-core/src/resolve/pick.rs`'s
module doc and `pick_face`'s doc (the documented tie-break contract),
then `crates/bvh/src/ray.rs`'s `RayCandidate::t_enter` doc (what the
lower bound certifies), then `crates/viewer/tests/index_memo.rs` from
its `FlatReference` down (the no-early-out reference and the tie-break
row).

## The defect, restated as the invariant it breaks

`pick_face` documents its answer as the lexicographic minimum of
`(t, target position, flat triangle position)` over every triangle's
exact test. That is a function of the per-triangle tests alone: the
early-out on `t_enter` is documented as an optimisation that cannot
change the answer, because `t_enter` is a certified lower bound on any
true hit in the box.

The item measures that this is false today for a ray lying in a
triangle's plane: Möller–Trumbore's determinant is rounding noise
(~1e-19 on the gallery ring), `u`, `v` and `t` are noise, and a `t`
BELOW the triangle's own box entry (`1.476` against `t_enter =
1.480 − 4 ulp`) passes the closed acceptance. Today's answer is the
right one only because a neighbour's hit rounds strictly below that
box's `t_enter` and the loop breaks first. Reorder the candidates and
the answer changes. The early-out is load-bearing, which its own
contract says it is not.

## What I verified, and where it sharpens the item

Against `origin/main` at the SHA in this spec's commit. Re-derive; do
not copy.

1. **The exact test has no relation to the box.** `ray_triangle`
   (`pick.rs`) takes `(ray, tri)` and returns `Option<f64>`; the
   candidate's `t_enter` is consumed only by the early-out in
   `pick_face`'s loop. So the guard the item proposes — refuse a hit
   whose `t` is below its box's certified entry — is currently
   unexpressible at the test site and has to be applied where both
   values are in hand.
2. **`t_enter` is documented as conservative in the DOWNWARD
   direction** (`bvh/src/ray.rs`: "a conservative LOWER bound on every
   `t ≥ 0` at which the ray … "). Read the widening it uses and
   satisfy yourself that a true hit's `t` can never be below it,
   including at `t = 0` and for a ray origin inside the box. If you
   find a case where it can, that is a finding on BVH's ground (file
   it) and the guard's premise is wrong — stop and report rather than
   adding a tolerance.
3. **The reference loop's early-out is the pin.**
   `index_memo.rs`'s `FlatReference::pick` keeps the same early-out
   "for that reason" (the item's words); the row `assert_flat_reference`
   compares the memo's answer to it. After this unit the reference
   can drop its early-out and must still agree — that is the
   acceptance that says the answer became a function of per-triangle
   tests.
4. **The ring case is reproducible from the tie-break row.**
   `tie_rays_for` aims rays at points two or more triangles share on
   the gallery documents; the item's ray is one of these (along +y
   through a chord point of the tube). Reproduce it as a RED probe
   first: the no-early-out reference answering `t = 1.476` on a face
   the ray does not cross there.

## The shape

**Recommended: the box-entry guard.** In `pick_face`'s loop, a
candidate's exact-test `t` that is strictly below the candidate's
`t_enter` is refused — not clamped, not repaired. A true hit is never
below a certified lower bound, so the guard removes only answers that
are already proven wrong, and it needs no tolerance and no new
vocabulary. Site it so that the exact test and the guard are one
predicate with one home (a function over `(ray, tri, t_enter)`, or the
guard immediately at the test's call site with the reason stated),
and so that the viewer's reference loop can call the same predicate
rather than restate it — `index_memo.rs` carries its own
`ray_triangle` copy today (Q1 of the style lane will ask about it; say
in the PR what you did about it).

**Then MEASURE whether the guard is sufficient.** A noise `t` can also
land ABOVE `t_enter` and inside the box. Run the shuffled-order sweep
below over the gallery corpus. If a residual order dependence
survives the guard, the second half of the item's proposal — refuse a
determinant that is not certifiably non-zero — is the next move, and
it must be a DERIVED bound (a rounding-error bound on `e1 · (d × e2)`
in terms of the operands' magnitudes and machine epsilon, stated at
the site), never a tuned constant. If you reach that point, say so in
the PR with the measurement that forced it. Do not add it
pre-emptively.

**Rejected: keeping the early-out load-bearing** and documenting it as
such. The candidate order is an index implementation detail
(`MeshPick::candidates` reproduces a single-tree sequence exactly so
that PERF-9's two-level index answers the same as before); an answer
that depends on it is an answer that changes whenever the index does.

## Acceptance

1. **The red probe, then green.** The item's ring case as a test that
   is red on main (the no-early-out reference answers below the
   corner) and green after: the answer is the corner, `t = 1.480`, on
   the face the ray actually grazes.
2. **Order independence, measured.** A row that runs every
   `tie_rays_for` and `rays_for` ray over the gallery corpus against
   (a) `pick_face`, (b) the reference with its early-out, (c) the
   reference with candidates in a permuted order and no early-out,
   and asserts all three agree on `(t, name)`. Name the runtime value
   that makes it red: any ray for which (c) disagrees with (a).
3. **The early-out made redundant, not removed.** `pick_face` keeps
   it for cost; `index_memo.rs`'s reference drops it, and the memo
   differential stays green. State in `pick_face`'s doc that the
   early-out cannot change the answer, and why that is now true.
4. **Grazing answers that moved are listed.** Any committed render,
   pick fixture or golden whose answer changed: re-baseline with the
   repo's tooling and list each in the PR with the old and new `t`
   and face (`docs/prompts/implementer-discipline.md` §3).
5. **`t_enter`'s premise pinned.** A row (in `bvh`'s tests if the
   bound's own suite has none, said in the PR) that goes red if the
   entry bound ever exceeds a true hit's `t` on a corpus of boxes and
   rays including origins inside boxes and axis-parallel rays.
6. The `viewer` edits are confined to `tests/index_memo.rs` (VIEW's
   and CHROME's ground; a re-baseline by announcement, said in the PR
   body). Anything beyond that in `viewer/src` is a stop: report,
   do not build.

## Fence

`crates/editor-core/src/resolve/pick.rs` and `hit.rs` are EDIT's.
`crates/bvh` is not: a change there is a finding filed on its owner's
slate (`python3 scripts/work.py territory --files -`), not this unit's
work, unless acceptance 5 needs a test file and nothing more.

## The trap

**The fix reproducing the defect it closes**: a guard that refuses
`t < t_enter` with a `<=` or an epsilon widens the closed-boundary
contract and changes edge-graze tie behaviour the tie-break row pins.
The predicate is `t < t_enter` on the bits, and acceptance 2's row is
what proves you did not move a tie.

## Stop clauses

- The `t_enter` premise fails (verification 2).
- The guard needs `viewer/src` to change.
- The derived determinant bound cannot be written without a constant
  you would have to tune.

Each: push what you have, say which fired and what you measured, and
end.
