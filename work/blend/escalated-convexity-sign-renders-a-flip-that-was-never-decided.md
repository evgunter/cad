---
id: escalated-convexity-sign-renders-a-flip-that-was-never-decided
kind: issue
title: blend: an in-band fillet3_convexity_sign escalation renders the chain-flip recourse
status: closed
opened: 2026-09-07
closed: 2026-09-08
pr: 2123
---

## Finding

`fillet3_convexity_sign` is decided at ONE site,
`crates/sweep/src/blend/battery.rs:619` (`convexity_at`), at the link's
own lever. Its outcomes there are: a definite sign (the link
classifies Convex or Concave), a decided `Sign::Zero`
(`crates/sweep/src/blend/battery.rs:632` —
`BlendError::TangentialEdge`, rendering `FILLET3_TANGENTIAL_RECOURSE`,
"blend an edge whose supports meet at a definite angle"), and an
in-band margin, which escalates through `esc(site, e)`.

The `Display` match routes that escalation to
`FILLET3_CONVEXITY_RECOURSE`
(`crates/sweep/src/blend/mod.rs:1303`) — "split the chain at the
convexity flip and blend each run separately". No flip was decided.
The only refusal that sentence is true of is
`BlendError::ConvexitySignFlip`
(`crates/sweep/src/blend/battery.rs:1489`), which is a CHAIN-level
disagreement among links whose signs all resolved DEFINITELY — a
different situation, at a different site, reached only after
`convexity_at` succeeded on every link.

So the two-tolerance pair (D4 ¶1 addendum: a predicate's definite
refusal and its in-band escalation carry ONE recourse) pairs this
predicate's in-band arm with a refusal it cannot be adjacent to, and
leaves it un-paired with the one it IS adjacent to: below `ε_input`,
"the wedge decided Zero" and "the wedge is in the band" are one user
situation, and `FILLET3_TANGENTIAL_RECOURSE` is the sentence true of
both. Handing "split the chain at the convexity flip" to a reader
whose chain has no decided flip is the same shape of defect the F6
fall-through arm exists to avoid — an action that has nothing to do
with what escalated.

The trio family records the hole rather than closing it:
`trio_convexity_sign`
(`crates/sweep/tests/m5_pr12_refusals.rs:417`) does NOT call
`assert_same_recourse` — it stops at
`assert!(matches!(escalated, BlendError::Escalated { .. }))`, so no
row goes red for the mismatch. It is not the only untied row:
`trio_corner_independence` (`:479`) also stops at its escalation's
predicate name, though its pair agrees, so the family's guarantee is
weaker than "every trio ties" wherever it is read that way.

Found while adding the `fillet3_support_coaxiality` arm to the same
match (unit 1's sweep over every predicate the battery decides), and
not changed there: which door a near-tangential edge belongs to is a
design statement about the arms, the same reason the coaxiality
pairing was routed to its own unit rather than settled in FILLET-H7.

## Fix shape

Route `Some("fillet3_convexity_sign")` to
`FILLET3_TANGENTIAL_RECOURSE`, and extend `trio_convexity_sign` to
`assert_same_recourse(&flat, &escalated, …)` so the pair is pinned
rather than merely observed. `ConvexitySignFlip` keeps
`FILLET3_CONVEXITY_RECOURSE`, which is the refusal that sentence is
true of. Decide first whether a chain-level flip should carry a
predicate name in its escalation payload at all — if it should, the
match needs a second key, not a second arm.

## Closed

Fixed on PR 2123 (BLEND unit 1's fix pass), as the fix shape above
states and no wider. `Some("fillet3_convexity_sign")` routes to
`FILLET3_TANGENTIAL_RECOURSE` at
`crates/sweep/src/blend/mod.rs`'s `match source.predicate`;
`BlendError::ConvexitySignFlip` keeps `FILLET3_CONVEXITY_RECOURSE`,
which is the refusal that sentence is true of. The routing is a D4
pair rule applied at one site — the in-band arm takes the sentence its
own definite refusal carries — and not a design statement about the
arms, so it needed no ratification.

`trio_convexity_sign` and `trio_corner_independence` both end in
`assert_same_recourse` now, and so does every other `trio_*` row, so
the pair is pinned rather than observed.
`review_blend1_r1_probes::r1_in_band_convexity_sign_renders_the_tangential_sentence`
pins the rendered sentence and the absence of the flip one.

The question the fix shape reserves — whether a chain-level flip
should carry a predicate name in its escalation payload at all — did
not arise: `ConvexitySignFlip` is constructed directly
(`crates/sweep/src/blend/battery.rs:1489`) and never through `esc`, so
the match needs no second key.
