---
id: roots-unsettled-folds-band-precision-and-fault-into-one-frontier
kind: issue
title: geom_brep::roots::Unsettled folds an in-band reading, a precision limit and a kernel fault into one value, and every consumer surfaces all three as the frontier CurvedPierceUnsupported
status: closed
opened: 2026-10-29
priority: P1
cost: H
refs: [two-rims-read-coplanar-refuse-unsupported-or-untyped, curved-pierce-frontier-tells-one-story-for-several-decisions]
closed: 2026-10-30
pr: 5993
---


The SPLITDOOR carrier-vs-rim designer pair agreed on this after reconciliation. The behaviour was confirmed by Ev on PR 5973 (2026-10-29): "this seems correct". No DESIGN change was made, since `[verdicts.closeness]` and `[tolerance.recourse]` already require it.

## What

`geom_brep::roots::Unsettled` ("the finder could not certify a count") lumps three different things together:

- **Band outcomes.**
  - `SignInBand`: a sign the finder must read definite did not.
  - `Tangency`: a piece the band cannot cut further.
- **Precision limits.** `Meter`, `Exhausted`.
- **Kernel faults.** `ImpossibleCount`, `Resolvent`, `CountDisagrees`.

Every consumer flattens all three to one outcome. There are two paths:

- **Meet path.** `MeetFault::Unsettled` → `BoundaryRefusal::of` (`crates/geom-brep/src/curve_face.rs`, folded with `Unsupported` into `Unread`) → `carrier_cross::boundary_refusal` → `BoundaryCrossing::Unread` → `reduce::interior`'s `frontier()`.
- **Span path.** `reduce::curved_face_arm`'s `SpanVerdict::Unsettled => Err(frontier())`.

Both end in `CurvedPierceUnsupported`. So an in-band meeting tells the user "the kernel can't yet", which offers no lever, against `[tolerance.recourse]`. A fault is hidden behind the same message.

The witness is `two-rims-read-coplanar-refuse-unsupported-or-untyped` at k = 30 and 80. Two rims cut from two glued planes stay Zero over most of their length and part through the band to kε near the grazing tips. Where they meet is in band, and today the op refuses as a frontier.

## Owed

- Split `Unsettled` three ways.
  - **Band:** report a band outcome as `Indeterminate` (row, in-band margin, band), carried as `RootRefusal::Escalated`, then `MeetFault::Escalated`, then `BooleanError::Escalated { Crossing(OnEdge) }`.
    - That last variant is what `carrier_cross::escalated` already writes. Its recourse is the meeting's lever: make the two cuts one construction, move the geometry, or tighten ε.
    - The finder re-cuts first. A `SignInBand` at a cut the finder chose itself is not a finding about the geometry; only a sign that stays in band wherever the event can lie is.
  - **Precision limits:** these stay the typed frontier.
  - **Faults:** these become kernel-defect endings.
- Audit every other wholesale `Unsettled(_)` match for the same flattening. The designers named:
  - `census.rs` (~4445);
  - `census/curved.rs` (~401);
  - `ring_path.rs` (~1587);
  - `boolean/slit.rs` (~1538).
- Re-baseline every row pinned on `CurvedPierceUnsupported` that moves to `Escalated`.
- The two-rims pin then asserts `Escalated` at 1 < k ≲ 1000.

Related: TOPOTAIL's `curved-pierce-frontier-tells-one-story-for-several-decisions` is the same frontier telling one story for several decisions, seen from the declaration offer. Whoever takes either row reads both.

## Closed (2026-10-30, PR 5993)

`geom_brep::roots` now tells three things apart:
- **Band:** `RootRefusal::Touch(Indeterminate)`, minted only from gap readings: the side cut, the first gap of an uncut piece, the width arm's clear, and a located root's residual. It escalates located (`CurvedPierceEscalated`, with `CurveMeeting` on the carrier and `BoundaryMeeting` off it).
- **Limits:** `Unsettled` (`Meter`, `Exhausted`, `AtApex`, `Degenerate`, `Crease`, `Floor`, `Resolvent`, `CountDisagrees`) keep the frontier.
- **Faults:** `RootFault`, now only the subdivision's impossible count, ends as `ClassificationInvariant`. Capacity sites are `unreachable!`.

A decided Zero keeps main's frontier story. Call (d), escalating it, was reverted, and the question is filed as `a-decided-zero-tangency-tells-the-frontier-story-where-recourse-iv-would-escalate`. Probe paths (rays, `ring_path`) say nothing on a touch.

Review: a DUAL concurrent pair (both NOT-MERGEABLE-AS-IS; the bilateral MAJOR was `Resolvent` on legal input) and two fix passes. A delta review outside the pair gave APPROVE-WITH-FIXES; its MAJOR, the meeting text off the carrier, is fixed.
