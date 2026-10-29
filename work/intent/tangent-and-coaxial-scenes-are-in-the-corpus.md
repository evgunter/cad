---
id: tangent-and-coaxial-scenes-are-in-the-corpus
kind: unit
title: INTENT stage 6 PR A: the corpus holds the coaxial and tangent scenes stage 6 proves, each row's proof pinned as it is today
status: review
opened: 2026-10-29
priority: P0
cost: M
branch: intent/s6-a-corpus-scenes
pr: 6023
---


INTENT stage 6, PR A. Spec: `docs/INTENT-STAGE6-SPEC.md` §2.

The corpus holds no `Coaxial`, `SurfaceTangent` or `ProfileJunction` row and no at-rest coaxial seat (the per-row door census, `coincidence_door::census_the_door_per_row_over_the_corpus`, on `0cc1eaf9`: 299 rows, the only tangent and coaxial ones the blend battery's). Add the scenes `pin_in_bore`, `shaft_and_bearing`, `axis_projection`, `boss_tangent_by_value` and `cone_on_sphere` to `crates/editor-core/tests/corpus/`, and pin each row's relation, site and proof as main has it. B and D flip those pins. Needs no design decision; dispatchable now, so the designers weighing FORK-S6-1 and S6-4 have the measurement.
