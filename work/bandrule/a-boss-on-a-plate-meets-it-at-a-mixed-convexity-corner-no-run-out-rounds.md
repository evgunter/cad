---
id: a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds
kind: issue
title: A boss on a plate meets it at a mixed-convexity corner that no run-out rounds, so the bracket cannot be filleted all round
status: open
opened: 2026-11-02
priority: P0
cost: H
---

Filed by the REFS orchestrator (2026-11-02), from the designers' measurement on `work/refs/select-all-edges-picks-edges-a-fillet-refuses.md` (Ev: P0, "an ordinary part cannot be filleted all round in one click").

Once the bore's smooth struts are out of the request, a plate with an upright boss (flush or inset) refuses every edge at each vertex where the boss meets the plate: `UnsupportedCorner::MixedConvexity` ("2 of 3 edges convex … only a run-out policy would handle: feather the radius out"; 1 of 3 for an inset boss). This is the parked feather-out run-out (`docs/KERNEL-VERBS.md`, "fillet run-out": "no consumer has ever wanted either"), and the bracket is its consumer. The two-step workaround fails too: filleting the concave base rim first, then the convex edges, makes the convex band end on the base band's cylinder, a curved end face, which refuses typed.

Witness, committed: `crates/sweep/tests/pins_of_todays_blend_refusals.rs::pin_today_the_bracket_filleted_all_round_refuses_a_mixed_convexity_corner` (a 6 × 4 × 0.5 plate, a 0.5 × 3 boss flush with one side standing 2 tall, a two-arc bore; every edge requested refuses `UnsupportedCorner::MixedConvexity`). It pins today's refusal; this row's PR flips it to a build. The GUI replay script is in the REFS item.

Band P0 under work/README's definition ("a normal verb broken on normal geometry"), on Ev's own example; Ev was asked to confirm on the REFS `[ev]` PR. Priced H; it may want `X` once its shape is designed.
