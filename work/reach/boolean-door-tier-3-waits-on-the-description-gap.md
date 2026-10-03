---
id: boolean-door-tier-3-waits-on-the-description-gap
kind: issue
title: The boolean door gates tiers 1-2 only: 55 of 687 topo-corpus results it ships fail tier 3, so gating tier 3 there needs the description-gap decision
status: open
opened: 2026-10-02
priority: P1
cost: H
design: true
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
