---
id: ring-run-winding-is-a-second-spelling-of-the-loop-winding-sum
kind: issue
title: The join's ring_run_ccw spells the loop-winding sum a second time, and winds a spline or spiric run edge by its chord where Body::planar_loop_winding answers Unsupported
status: closed
opened: 2026-10-02
priority: P1
cost: M
refs: [verbs-1031b-assigner-checker-divergence, blind-d-pocket-subtract-refuses-with-join-internal-words, unclaimed-half-edge-read-as-a-minus-half-in-zip]
branch: join/ring-run-winding
closed: 2026-10-02
pr: 3771
---


Found at JOIN's opening, closing `verbs-1031b-assigner-checker-divergence`.

## What

The `bool_ring_run_winding` predicate has two arithmetic homes:

- `crates/topo/src/loop_winding.rs`, `Body::planar_loop_winding_decided`
  — a stored closed loop; the merge's role assigner and tier 3's check 6
  both read it;
- `crates/topo/src/boolean/join.rs`, `ring_run_ccw` — an open run
  `h1 → h2` closed by the chord `end(h2) → p₀`; the join's ring lane
  reads it to pick an island's new outer boundary.

Both accumulate the Newell sum, the arc-length perimeter and the conic
bulge, and decide `Margin::over_lever(n · newell, perimeter)`. They share
only `loop_winding::conic_segment_term`. They have drifted in two ways:

- **Carrier set.** The loop spelling answers `LoopWinding::Unsupported`
  for a spiric or NURBS edge and for a null-edge scaffold, so no sign
  is asked. The run spelling's `run_term` folds every non-conic,
  scaffold included, into its CHORD (`None => Ok((zero, chord()?))`),
  so a spline run edge on a planar face is wound by its chord. That
  is a different area, and the sign can come out wrong when the
  chord crosses the curve.
- **Half claim.** The loop spelling reads the half through
  `edge.claim(he)` and refuses an unclaimed half (`TornLoop::Unclaimed`).
  The run spelling reads `edge.he_plus == he`, which is
  the `ring_run_ccw` site of `unclaimed-half-edge-read-as-a-minus-half-in-zip`.

## What the taker owes

One home for the sum, with the open run's closing chord as the only thing
the ring lane adds, so that the carrier set and the claim are read
once. Check whether `blind-d-pocket-subtract-refuses-with-join-internal-words`'s
top-entry refusal ("ring-run winding is degenerate (zero enclosed
area)") is this function deciding a run it should not have been handed,
or one it winds wrongly.

## Built (`join/ring-run-winding`)

The sum's one home is `loop_winding.rs`'s `Body::winding_of_halves`,
with a `Closing::Cycle` or `Closing::Chord` arm. `ring_run_ccw` reads it
through `Body::planar_run_winding_decided`. Both callers read
`edge.claim(he)`.

- **Spline or spiric run edge:** the ring lane refuses it as
  `SectionInvariant`, since the operand gates refuse both kinds.
- **Null-edge scaffold:** both spellings now wind it as its
  zero-length chord. This is a deliberate change to the loop spelling,
  which used to answer `Unsupported`. A ring run always opens and
  closes on null halves.

The sweep's third spelling, `splitting/join.rs`
`certify_section_area`, is REACH's
`split-section-area-spells-the-planar-winding-sum-a-third-time`.
`blind-d-pocket-subtract-refuses-with-join-internal-words` is
diagnosed, not fixed, in its own `## Diagnosed` section.

## Closed 2026-10-02 — PR 3771

One home for the sum, built as above. The review found the run spelling's
conic arm unguarded, and the fix pass pinned it
(`a_run_on_arcs_is_decided_by_its_bulge`), with the ring lane's real
shape (`a_run_between_two_null_halves_is_the_region_they_bracket`). The
remaining spellings of the planar winding functional are a class on
REACH's re-titled row.
