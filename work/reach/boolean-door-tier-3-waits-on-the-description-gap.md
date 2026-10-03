---
id: boolean-door-tier-3-waits-on-the-description-gap
kind: issue
title: "The boolean door gates tiers 1-2 only, citing a description gap closed since M3 PR 6a: what a door owes for what it ships and consumes (the finished-body contract)"
status: closed
opened: 2026-10-02
priority: P1
cost: H
design: true
branch: reach/ev-door-finished-body
pr: 3870
closed: 2026-10-03
---


The half of
`boolean-door-passes-a-geometrically-open-result-the-backstop-cannot-see`
that the volume backstop cannot close. A result missing a face whose
zip glues the two free edges is topologically closed (tiers 1 and 2
pass) and can enclose a short POSITIVE volume. No inequality over
`vol(A)`, `vol(B)` and the result bounds `vol(A ∩ B)` from below: that
lower bound is `vol(A) + vol(B) − vol(A ∪ B)`. Tier 3 sees such a body
(an arc edge in a plane face, `PlanarBoundaryResidual`), but
`boolean::ops::gate` runs tiers 1 and 2 only. Its doc says why: "tier 3
is an at-rest posture with the PR 3 description gap".

## Measured

The instrument is topo's `door-tier3-meter` feature
(`crates/topo/src/boolean/door_meter.rs`), run by
`python3 scripts/door-tier3-meter.py`. It runs `AtRestPolicy::gate_at_rest`
on every result `boolean_op_recut` builds, after `gate` and the volume
backstop, and on both operands. It times the op, tier 3 and the backstop,
and changes no result. Measured on `reach/door-backstop` (`ci` profile,
default ε):

| corpus | results | tier 3 refuses | of those, shipped | shipped with tier-3-clean operands | tier 3 / op time | median / p90 / max per result | backstop / op |
|---|---|---|---|---|---|---|---|
| topo | 693 | 55 | 55 | 3 | 15 % | 11 % / 27 % / 79 % | 6 % |
| sweep | 1168 | 42 | 0 (the backstop refuses the same bodies `VolumeUnmeasured`) | 0 | 13 % | 13 % / 23 % / 226 % | 25 % |

(On `cd49025f`, before the backstop's interval re-derivation, the same
probe found 52 of the 55 topo refusals shipped.)

- Every result the door ships from verb-built operands (the `sweep`
  corpus) passes tier 3.
- Of the 55 shipped topo results that fail tier 3, 52 come from
  operands that fail it too. The tier-3 errors are `ScaffoldAtRest` and
  `TransverseNotIntrinsic` on the hand-built `review_m3_pr55` fixtures,
  `surgery::tests`, and the two `refusal_routes::offer_rows`
  far-origin rows, which also carry `PlanarBoundaryResidual` and
  `PlanarFaceResidual`.
- The other 3 are `contact9_side_codes` slivers whose operands pass
  tier 3. The result carries a door-minted `ScaffoldAtRest` edge, and
  two of them also carry `LaminaWedge`.

So gating tier 3 at the door refuses bodies it ships today. Some have
tier-1/2-only operands, and some are slivers whose minted edge keeps a
scaffold description. That is the description-gap decision the gate's
doc names: whether the door's currency is tier 3, and what it does with
operands below it. The decision is not taken here. The cost of tier 3
is about 15 % of op time on both corpora.

## What would close it

Decide the description gap (designers first,
`memories/orchestration-model.md`). Then either gate tier 3 at the door,
or gate the part of it that sees an off-carrier edge (check 3's
planar vertex residuals and check 5's planar boundary containment).
With the decision taken, measure again what the corpus ships.

## Designers (2026-10-02)

Two designers weighed this (`docs/DESIGN-FORK-LOG.md`, row 47). Both
found the description gap closed: M3 PR 6a mints honest seam
descriptions, and the gate's doc is stale. The open question is the
door contract.

After two reconciliation rounds, the first of which crossed, they
recommend one final state:
- the finished body is a type (`AtRestBody`);
- every verb door takes it and returns it (tier 3, or tier 3′ with
  contacts), so each body is gated once, at the door that built it;
- Euler operators hand back construction state;
- D1's tier-2 sentence is changed;
- check 7 takes the backstop's interval re-derivation before or with
  the gate, and the positivity arm retires after.

Put to Ev on the `[ev]` PR from `reach/ev-door-finished-body`.


## Ev's answer, and the fold (PR 3870, 2026-10-02)

Ev approved the recommendation. Two follow-ups:
- Move the finished-body paragraph under tier 3. Done.
- Fold 3′ into 3? The 3′ extras that are not contact-only are the census.

A measurement lane answered the second (`analysis/reach-measure/census-fold`,
`report.md`). It ran 2,572 non-fixture, contact-free bodies that pass
tier 3, at three ε.

**Cost.** The census costs a median 0.09× tier 3 on single solids and
0.44× on multi-solid bodies. On the tour that is 0.02 s against 20 s of
construction.

**Real defects it catches that tier 3 misses (8).**
- 5 pinch unions whose record dropped the contact;
- a shell whose crossing walls build silently (#1055);
- an `mfkrh` doubled face;
- a twin-solid STEP file.

**What it cannot yet decide.** 26 `shell` results whose nested or
side-by-side curved solids the census answers `CensusUndecidable`: its
cross-solid backstop has no curved × curved or curved × planar lane.
`topo::shell` gates its result with `validate_geometric`, so turning on
the folded bar there refuses those calls until that lane exists.

**The ratified sentence it falsifies.** DESIGN's "the two gates agree
there" (empty contacts) was false on 2.6% of the population.

**Written.** The finished-body bar is tier 3′ against the body's own
declared contacts, empty or not, and the `BooleanBody` paragraph says
the same.

**Sequencing, not a fork.**
- The census's cross-solid curved lane must land before or with any door
  whose results carry several curved solids (shell first) adopting the
  bar, as check 7's interval re-derivation must before the boolean door's.
- The product gather must gate after the mates' declarations are
  minted, or with them.

## Measured again: tier 3 and the census at every result site (FUSE, 2026-10-02)

The FUSE unit `a-boolean-result-gate-ships-a-scaffold-at-rest` timed
`T::gate_at_rest_kept` then `T::gate_at_rest_declared` (tier 3, then
the census over the result's own contacts) inside `boolean::ops::gate`,
so at all four sites that build a `BooleanResult::Body` (seamed, the
fallback's two-operand arm, `finish_fallback`, the declared-REST union
in `rest.rs`), recording and refusing nothing. `ci` profile, default ε,
on `75276c4a8`. Times are summed over every result; "suite" is the sum
of the instrumented suite's test times.

| corpus | results | tiers 1–2 | tier 3 | census | suite |
|---|---|---|---|---|---|
| topo | 963 | 0.44 s | 1.90 s | 0.68 s | 186 s |
| sweep | 2113 | 0.73 s | 10.7 s | 0.73 s | 281 s |
| editor-core | 9922 | 6.2 s | 45.4 s | 9.2 s | 645 s |
| verbs | 6 | 2 ms | 10 ms | 3 ms | — |

So the census is the cheap half: 15–35 % of tier 3's time at the door.

What the full pass refuses, beyond what tiers 1–2 already refuse:
- **Tier 3:** `ScaffoldAtRest` on 57 `topo` results, which the result
  gate now refuses (FUSE's unit). `VolumeUncomputable` on 43 `sweep`
  results and 1 `editor-core` result, bodies the volume backstop
  already refuses as `VolumeUnmeasured`. Gating tier 3 ahead of the
  backstop changes their refusal from the backstop's to
  `ResultInvalid`.
- **The census, results with records** (3′ currency today): 40
  `StaleContactDeclaration` and 16 `UndeclaredContact`, filed as
  `work/fuse/a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms`
  and `work/fuse/a-two-pinch-union-ships-a-pinch-its-records-do-not-declare`.
- **The census, results without records** (tier-3 currency today, so
  refused only if 3′ folds into 3): 43 `editor-core` and 2 `topo`
  `UndeclaredContact` (pinch unions in member-order rows,
  `m3_pr6_tier3prime::closure_kiss_vs_mover`), and 11
  `CensusEscalated` (`contact9_side_codes::a_vertex_pair_reads_a_dipping_chord_at_its_far_vertex`
  ×6, `a_near_coincident_pair_at_a_corner_refuses_typed` ×2,
  `docm2_part_interval` ×3).
- **The decision log.** With the pass in the door and refusing nothing,
  7 `editor-core` rows went red: `edit_refusal_recourse::every_predicate_a_subtract_logs_has_words_or_a_reason`,
  `m4_pr4_ci::diagnosis_corpus_is_golden`, three
  `resolve_piece_ladder` and two `resolve_upstream_scope` rows. The
  pass's predicates (`planar_boundary_residual`, `planar_face_residual`,
  `bool_ring_run_winding`, the `pm_census_*` family) reach an
  evaluation's decision log, where they have no words and move the
  diagnosis goldens. A gate in the door needs them named first.

## Closed (2026-10-03, PR 3870)

Ev ratified the finished-body contract with the fold ("yes!", then
"sounds great!"). `docs/DESIGN.md` now says:
- a finished body (`AtRestBody`) passes tier 3′ against its own
  declared contacts, an empty list for a body that touches nothing;
- every door that returns or consumes one pays that gate once, at the
  door that built it;
- Euler operators hand back construction state.

The design is settled. Building it is
`work/reach/boolean-door-adopts-the-finished-body-type.md`, with its
sequencing: check 7's interval re-derivation, the census's predicate
names in the decision log, and CONTACT's cross-solid curved lane before
`shell` adopts the bar.
