---
id: main-red-two-offer-rows-quote-a-positive-margin-against-a-negative-case
kind: issue
title: Main red: offer_rows' every_offered_tolerance_passes_just_below_it quotes a positive margin on two negative cases
status: open
opened: 2026-11-10
priority: P2
cost: E
---


Found by SEAL's step-4 implementer and confirmed by its reviewer (PR 6693), from running the slow set; neither case is in that PR's diff.

At main `5b2bb0dcae`, `crates/topo/src/boolean/offer_rows.rs:1928` `every_offered_tolerance_passes_just_below_it` fails with exactly two false offers:

- `arc_root_just_outside_its_span` (`offer_rows.rs:385`, declared `-D`) quotes a margin of +5.500000048e-9 against the expected −5.5e-9;
- `roots_a_hair_apart` (`offer_rows.rs:422`, declared `-D`) quotes +5.499881e-9 against −5.5e-9.

The magnitudes agree and the signs do not, so either the root site now reports the margin unsigned (or of the other side), or the cases' declared sign is stale. The test is in the slow set, so this is a nightly red, not a PR gate.

Done when: both cases pass at default ε, with the sign fixed at whichever end is wrong and the reason in `## Closed`.
