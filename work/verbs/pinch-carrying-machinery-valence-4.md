---
id: pinch-carrying-machinery-valence-4
kind: issue
title: design - pinch-carrying machinery to support the intersecting equal-radius boolean family (valence-4 section vertices)
status: parked
blocked_on: [parameter-identity-channel-to-boolean]
opened: 2026-08-31
github: 1377
refs: [1353, 1372]
---

## From GitHub issue 1377

Opened 2026-08-31; 0 comments.

## Ruling (Ev, in-chat 2026-08-31)

"we definitely need to support that geometry at some point." The typed refusal shipped by VERBS-GERMARMS PR-2 (#1353, `GermFrameCylinderPinch`) is the honest **interim** state, not the permanent answer.

## What this is

The intersecting equal-radius cyl×cyl family's section is two bisector-plane ellipses that always cross at `p ± r·(â₁×â₂)` — self-intersections of the section curve, physically the two sharp points of a Steinmetz solid where the walls are mutually tangent. Producing the union/intersection body requires representing **valence-4 pinch vertices** (four arc branches meeting at a point), which the current B-rep vocabulary excludes end to end.

## The measured inventory of what must widen (from PR-1/PR-2; the ordinal-109 dual is re-verifying — fold its findings in before speccing)

- **Vertex factory**: a split vertex currently joins exactly two fragments; the pinch joins four. PR-2's enabling correction already landed one adjacent premise fix (`vertex_on_curved_face` no longer assumes vertices are never interior to a face — falsified by PR-1's own pierce vertex).
- **Chord pairing / join**: runs are paired two at a time; at a pinch, four arcs must be distributed into loops with the correct crossing structure (branch selection is point-dependent — see the chord-lane widening below).
- **Loop walking / containment**: section traversal assumes simple loops; the pinch is a branch point.
- **Tier-3 validation / census**: valence expectations; the result body's census carries two valence-4 vertices.
- **Tangency honesty**: at the pinch the walls are mutually tangent — PR-1's sagitta charge (`bool_pierce_sector_side_curved`) is on the critical path; the second-order sector trilean is the open technical question for definite side-verdicts near the pinch.

## Prerequisites and sequencing

1. **#1372 first** — the family cannot even be *recognized* without the parameter-identity (declared radius equality) channel; that design conversation gates this one.
2. The **chord-lane widening** (PR-2's pre-registered STOP: `SectionCtx` generalized from one-plane-one-conic to a carrier set + point-resolved branch selection, under a bit-identity fence on the plane×wall path — Ev leans widen-in-place over a parallel variant, in-chat 2026-08-31) is naturally the first implementation unit; it is needed for ANY cyl×cyl join, pinch or not.
3. Pinch machinery proper (this issue) builds on both.

Fenced out deliberately (spec, ratified): NURBS-fitting/approximating around the pinch — a plausible body whose census and metering lie near the tangency.

Spec to be drafted as a design doc for ratification after the ordinal-109 dual and its union fix pass land; until then the typed refusal stands.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_016pYMaeU4woYZN8YGdTLfSK

## Home

VERBS: this is the cyl×cyl germ lane's geometry-side acceptance — Wave 2's curved boolean breadth over analytic pairs, VERBS' charter and its `crates/geom-brep/src/ssi*` / `intersect.rs` territory — and the issue names itself as VERBS-GERMARMS' successor.
