---
id: curve-projection-orthogonality-field-has-no-reader
kind: issue
title: geom: Projection3's orthogonality field has no production reader, and the curve projection docs still say a consumer bands it with the distance
status: open
opened: 2026-10-10
priority: P3
cost: E
---



(Filed by the ENCL lane on `encl/retire-foot-orthogonality`, which retired the surface half's twin under Ev's ruling in PR 4498, fork-log row 104.)

## What

`geom::Projection3`'s `orthogonality` field (`crates/geom/src/curves/projection.rs`, the `Projection3` struct) is written and never read outside `crates/geom/tests/`. Its one consumer, `geom-brep`'s `certify.rs` (`project_from_seed` in the edge certificate's foot read), reads the distance alone.

The curve module doc still tells the clamp story the surface half lost: "a boundary clamp ⇒ an orthogonality residual fails it" and "a consumer bands the **pair**". By Cauchy–Schwarz, `|C′·r|/|C′| ≤ |r|` at the same `t` and the same `r`, so the orthogonality residual cannot refuse a foot whose distance passed. A clamped foot is a point of the curve, so its distance bounds the true one from above.

## Repair shape

Delete the field and reword the curve module doc to the distance-only reading the surface half now has (`crates/geom/src/surfaces/projection.rs`, "Honesty, in two parameters"). Keep `ProjectionInconclusive::last_orthogonality`: the refusal's Display reads it.

Tests that read the field:
- `crates/geom/tests/curves/projection.rs` (`domain_end_clamp_carries_an_honest_failing_orthogonality_residual` and others);
- `crates/geom/tests/curves/review_m5_pr4_adversarial.rs`;
- `crates/geom/tests/dual_foot_tangent.rs`'s curve row;
- `crates/editor-core/tests/refusal_concision_chains.rs` builds a `ProjectionInconclusive` (it is `last_orthogonality`, which stays).

Recompute from `ders` where a test checks convergence. Delete a test whose subject is the field.
