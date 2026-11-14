---
id: main-red-two-offer-rows-quote-a-positive-margin-against-a-negative-case
kind: issue
title: Main red: offer_rows' every_offered_tolerance_passes_just_below_it quotes a positive margin on two negative cases
status: closed
opened: 2026-11-10
priority: P2
cost: E
parent: nightly-reds-default-eps-behaviour-rows
closed: 2026-11-14
---


Found by SEAL's step-4 implementer and confirmed by its reviewer (PR 6693), from running the slow set; neither case is in that PR's diff.

At main `5b2bb0dcae`, `crates/topo/src/boolean/offer_rows.rs:1928` `every_offered_tolerance_passes_just_below_it` fails with exactly two false offers:

- `arc_root_just_outside_its_span` (`offer_rows.rs:385`, declared `-D`) quotes a margin of +5.500000048e-9 against the expected −5.5e-9;
- `roots_a_hair_apart` (`offer_rows.rs:422`, declared `-D`) quotes +5.499881e-9 against −5.5e-9.

The magnitudes agree and the signs do not, so either the root site now reports the margin unsigned (or of the other side), or the cases' declared sign is stale. The test is in the slow set, so this is a nightly red, not a PR gate.

Done when: both cases pass at default ε, with the sign fixed at whichever end is wrong and the reason in `## Closed`.

## Closed

Greened by GREEN's default-eps lane (`green/default-eps`, 2026-11-14),
by a pin; the sign is not fixed here, so this row's "done when" passes
to the fix row.

The cause is PR 6687 (`onecopy/needle-basin`, merge `f875961fe`,
2026-11-09; the test is green at its parent and red at the merge): the
conic root lane now reads a root
against an end, and two roots against each other, by the chord between
their points (`curve_pair_span_end_chord`, `plane_crossing_roots_apart`,
both `Margin::norm3`), where it read signed parameter differences. So
the root site reports a length: neither guess above (an unsigned
report, or a stale declared sign) is wrong, and which end to change is
a question about the offering census, since re-declaring the two cases
`+D` leaves its negative `Crossing(OnEdge)` and `Crossing(Order)` arms
with no executed case.

`offer_rows.rs`'s `QUOTED_AS_A_LENGTH` pins the two quotes at
`|margin|`. The fix row is ONECOPY's:
`work/onecopy/the-conic-root-lanes-chord-rungs-quote-a-length-where-two-offer-cases-declare-a-negative-margin.md`.
