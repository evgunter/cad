---
id: tangent-offset-and-tilt-are-two-epsilon-checks-where-rest-is-one-displacement
kind: issue
title: Should C4/C7's Tangent decide its locus offsets and its tilt as one displacement, as Rest does, rather than as separate ε checks that together admit about 2ε?
status: open
opened: 2026-10-10
design: true
priority: P3
cost: M
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
