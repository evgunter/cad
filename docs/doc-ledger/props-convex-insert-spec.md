# PROPS-CONVEX-INSERT-SPEC.md

PROPS convex insertion — the width `insert_once_ring` gives away (PR #3524).

Deleted at PROPS' close, 2026-10-03.
Recover with `git show 63df2069c:docs/PROPS-CONVEX-INSERT-SPEC.md`.

**Spec note worth keeping: the clause that did the work was the one
naming the failure mode in the opposite direction from the change.** The
unit made a certified bound narrower, so the spec said in terms that a
narrower bound is the EXPECTED outcome and therefore not evidence of
correctness — containment is — and demanded a test against exact
rational arithmetic with cases chosen to break it. Both blinded
reviewers then attacked containment independently (46,416 and 1,390
comparisons) and neither could break it. Had the spec asked only for the
width to fall, a bound that was too tight would have read as a success.

The spec also got one thing wrong and the lane said so: it claimed
computing `β` as `1 − α` "gives back some of the width the change exists
to save". Measured, `1 − α` is within noise of the knot-derived β. The
ruling was right and its stated justification was not — what carries it
is that `β ∋ (1−λ)` follows from the knots without routing through α's
already-rounded value.
