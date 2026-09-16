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

## Amended at the fix pass (2026-09-16)

**The box-entry guard is withdrawn as unsound in `f64`.** It compared
a rounded `t` against a bound widened for the box's rounding and not
the test's, and on any axis-planar triangle — whose box has zero
extent along one axis, so the entry IS the hit's parameter everywhere
on it — it refused genuine well-conditioned hits: `pick.rs`'s
`a_fan_triangulated_cap_accepts_its_interior_hits` (7% of interior
hits lost on a fan cap, R1) and `review_pick_r2`'s corpus sweep (the
service answering `cut_cylinder`'s wall 0.3 beyond a rim vertex, R2)
are the two probes, now rows.

**The mechanism is the certified determinant, and the hit's `t` from
its point.** A candidate whose Möller–Trumbore determinant does not
exceed the forward rounding-error bound of its own evaluation
(`3 · EPSILON · Σ|e1_i|·S_i`, derived from the operation count at the
site, never tuned) is refused: its barycentrics would be noise over
noise. That alone does not close the item's case — the ring's
candidate has a determinant 31 times its bound, `u = v = 0` exactly,
and a quotient `e2·q / det` that cancels to `1.476` for a true `t` of
`1.480` — so `t` is taken as the parameter of the hit point
`a + u·e1 + v·e2` along the ray, which is conditioned by `u` and `v`
and not by the determinant.

**Acceptance 2(c) reads `Pruned == Every`.** A fold of a commutative
minimum under a total key agrees with every permutation by
construction, so the reversed walk could not go red and is gone; the
row's content is that the early-out does not change the answer, and
what it could change in principle is a near-tie the rounding of `t`
decides (`pick_face`'s docs, the filed tie-rounding row).

**Acceptance 4's table rides a pinned-count row.** The counts a row
can carry without `main`'s predicate in the tree are pinned in
`review_pick_r2` (rays, rays answered at the aimed vertex, candidates
refused at the determinant, rays carrying one) with the command that
re-derives them; the moved-answer table against `main`'s kernel is a
one-shot measurement in the PR, and the row says why it is not
pinned.
