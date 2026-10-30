---
id: rods-sharing-a-wall-at-two-lengths-refuse-intersect-and-the-short-difference
kind: issue
title: Two rods sharing a wall at two lengths build ∪ and B ∖ A but refuse ∩ and A ∖ B at the section pass
status: dispatched
opened: 2026-10-29
priority: P1
cost: M
---


Found by the review of PR 5969 (`splitdoor/two-rims-coplanar`), in
`crates/sweep/tests/two_rims_review_probes.rs`
`rods_sharing_a_wall_at_different_lengths_are_sided_by_their_extent`. That test pins the refusals.

Two rods of radius 0.5 revolved about `y` from `y = 0`, A of height 1 and B of height 2. A's wall lies on B's wall's carrier, and A's top cap stands inside B. `A ∪ B` and `B ∖ A` build at their closed forms. `A ∩ B` and `A ∖ B` refuse `FallbackExtentUnsupported { operand: A, face: FaceKey(3v1) }` ("tangent or near-tangent carriers …"). That is the section pass's R-tan refusal, naming A's face `3v1`. The reviewer measured the same table on the merge base, before the `On` deferral. Measured at ε = 1e-6, 1e-9 and 1e-12.

- **Why the deferral does not answer it.** The wall pair is settled one carrier, so the section pass defers it (taken as the refusing pair, not traced). A's shell cannot read `On` (its top cap is inside B, on no face of B), so the pair keeps its refusal (`ops::OnDeferred::answered`). The `In` would be the right side here: A lies inside B.
- **Unmeasured.** Why `∪` and `B ∖ A` reach a body without that refusal is not traced: whether they take another path past the pass, or the pass reads them differently. That is the first step.
- **The answer owed.** `A ∩ B` is A, and `A ∖ B` is nothing. A certificate that the coincident patch is the whole of A's wall (A's wall's edges inside B's wall) would let the side verdict stand.
