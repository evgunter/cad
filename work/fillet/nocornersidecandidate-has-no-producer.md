---
id: nocornersidecandidate-has-no-producer
kind: issue
title: `NoCornerReason::NoCornerSideCandidate` has no producer reachable through a shipped door (machinery with no producer)
status: open
opened: 2026-08-30
github: 1280
refs: [1267]
needs_ev: true
---

## From GitHub issue 1280

Opened 2026-08-30; 0 comments.

A **finding**, disclosed at the merge of BLEND-7 (PR #1267, the enclosing-tangency refusal) rather than fixed there: after that unit, the refusal reason `NoCornerReason::NoCornerSideCandidate` — "a corner exists, but every tangent circle of that radius touches a side past the corner" — is left with no witness anywhere in the workspace, and three independent searches found no construction that reaches it.

## What changed under it

`sugar::arc_fillet_trims` now classifies the enclosing (ρ < 0) class before the offset-carrier intersection and refuses it typed (`crates/profile/README.md`). Every construction that previously reached `NoCornerSideCandidate` in the suite was an enclosing request: the candidates existed, were computed, and failed the corner-side reach gate *because* the blend circle was bigger than both carriers. Those requests now refuse earlier and more specifically.

## The evidence, from three lanes

| search | shape | constructions found |
|---|---|---|
| implementer | 1 237 632-corner grid over ORDINARY (ρ > 0) arc×arc corners (R ∈ {0.2, 0.5, 1, 2} × {0.2, 0.35, 0.7, 1.5, 3}, both windings, turn angles 0.5…5.5 rad, leg extents 0.35…2.8 rad, r 0.05…2.0, bracketed anchors) | 0 |
| implementer | 400 000 random draws over the `review_s2` fuzz distribution, all leg-kind pairs | 0 (of any ρ sign) |
| reviewer 1 | 7 680-corner mining sweep including line×arc corners | 0 |
| reviewer 2 | 41 472-corner sweep; also verified the deleted fixture was the workspace's ONLY witness at the merge base | 0 |

**What these searches cannot match** (stated so the negative result is readable as what it is): near-tangent / hairline-lens carrier pairs, scale ratios past ~15×, corners far from the origin, line×line corners (which route to the closed form before this pass), and anything a bracketing harness's anchor clamp excludes. None of them is a proof of unreachability.

## What is open

Whether the reason has a producer at all through the shipped doors, and if not, what to do with it: delete it and let the two remaining reasons carry the taxonomy, or keep it with a stated reason (a defensive arm, like `fillet_bulge`'s major-arc branch, which is kept deliberately and says so at its site). That is a taxonomy decision, not a lane's, and it wants either a mined witness or a demonstration that none exists.

Not fixed in PR #1267: manufacturing a witness for a reason nobody can reach would be inventing coverage, and deleting a public variant on a negative search result is a decision this issue exists to put in front of a person.

— Filed by the BLEND-7 implementer lane, adjudicating both blinded reviews.

## Home

`work/issues/` — the taxonomy in `crates/profile`'s fillet refusal family was S-BLEND's ground; S-BOOL lists `crates/profile/*` as territory but its charter is boolean reach, not the fillet refusal vocabulary.

## For Ev — keep the variant as a stated defensive arm, or delete it?

The producer is one branch: `sugar::arc_fillet_trims`
(`crates/profile/src/sugar.rs:640`) returns `NoCornerSideCandidate` when
the offset carriers DO meet, every candidate was computed, none survived
the corner-side reach gate, and no candidate's failure was an anchor
overrun. Since the enclosing class refuses earlier, no construction is
known to reach it (the four searches above, blind spots stated).

1. **Keep it, as a defensive arm with its reason stated in one line
   (recommended).** Deleting the variant does not delete the branch:
   `survivors.is_empty() && overrun.is_none()` is a real code path, and
   without the variant it must either fold into
   `OffsetCarriersDisjoint` (a lie — the offset carriers met) or panic
   (D9). The precedent is `fillet_bulge`'s major-arc branch. The doc
   comment shrinks to the invariant — "the reach gate's own failure
   arm; no known producer since the enclosing class refuses first" —
   and the measurement narrative goes to git history.
2. **Delete it; the branch becomes `unreachable!` with a geometric
   argument.** Honest only once someone proves that a circle of radius
   r tangent to two carriers that are both non-enclosing (ρ > 0)
   always touches each carrier on the corner side. Nobody has; the
   searches are evidence, not a proof. Cheaper API, riskier.
3. **Delete it; fold the branch into `OffsetCarriersDisjoint`.**
   Rejected: it names the wrong situation.

A 👍 on 1 closes this as a one-line doc edit riding the next unit that
opens `validate.rs`; 2 becomes a unit with a proof obligation.
