---
id: tangent-offset-and-tilt-are-two-epsilon-checks-where-rest-is-one-displacement
kind: issue
title: Should C4/C7's Tangent decide its locus offsets and its tilt as one displacement, as Rest does, rather than as separate ε checks that together admit about 2ε?
status: closed
opened: 2026-10-10
priority: P3
cost: M
pr: 4640
closed: 2026-10-11
---


Surfaced by the review of PR 4558 (row
`contact-tangent-relation-decides-offsets-and-tilt-per-sample`, which
closed with no change because the split is the ratified text). This is
a question about that text, not a defect in the code.

## The question

C7 (`crates/geom-brep/README.md`, #85) certifies a `TangentIntersection`
per sample as "surface coincidence within ε, normal parallelism within
the derived angle at lever arm `1/κ_rel`". C4's `Tangent` must-verify
list (`crates/topo/README.md`, #178) names "first-order tangency along
the witnessed locus, the locus on both surfaces within ε" as separate
facts. `topo::boolean::contact_verify::tangent_locus_relation` and the
edge certifier (`geom_brep::certify`'s tangent arm) decide them
separately, as written.

The carrier is within `|r|` of each surface. Read at lever `1/κ_rel`, the
tilt puts the true tangency point within `sin θ / κ_rel` of the sample.
So the served verdict admits the carrier up to about
`|r| + sin θ/κ_rel`, which is nearly 2ε, from the locus it claims. C4's `Rest` was
given the other shape: "the pair read as one displacement over the
consumed extent", decided as one `datum ± swing` (`decide_across`).

Should `Tangent` be read as one displacement too, or is per-fact ε the
intended tolerance for a tangency? The answer changes both lanes
together, since the edge certificate and the declared verdict must
decide one set (`tangent_certificate_lane`'s docs).

## Ruled (Ev, PR 4595, 2026-10-11): (a), the pair's separation is served

A served `Tangent` claims the two surfaces touch within ε along the
locus (fork log row 111). C7 and C4 state it. What's left is the build,
which is this row:

- **Verifier** (`topo::boolean::contact_verify::on_surface` /
  `tangent_locus_relation`): per sample, decide `s = r₁ − σ·r₂` (σ from
  the normals' decided orientation) under its own predicate name. Zero
  passes; a definite sign contradicts (separation positive, crossing
  negative), carrying the `FIT_DEFERRAL` steer; in band escalates
  (must-verify, never bridged). The residual rows stay as cache rows,
  and their contradiction names a bad witness, with no Fit steer.
- **Certifier** (`geom_brep::certify`'s tangent arm): a third per-sample
  `check_residual` (`CertCheck::TangentGap`) beside the two surface
  residuals, with σ from the gradients' alignment, plus a span line
  beside `tangent_hull_sup` (max |s| plus both sags).
- The `TangentWitness` coincidence records `s`'s margin.
- Pins: a handed-in witness between two surfaces 1.8ε apart (escalates)
  and 15ε apart (contradicts as separation); a witness 11ε off one
  surface of a touching pair (contradicts as a bad witness, no Fit
  steer); the contact-door paths unchanged.

## Closed 2026-10-11 (PR 4640)

Both deciders read the touch as its own row (`contact_tangent_gap` / `CertCheck::TangentGap`), with a span line charging `gap_sag`. Residue: `tangent-parallelism-between-samples-is-certified-by-no-row`.
