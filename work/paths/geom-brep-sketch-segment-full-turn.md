---
id: geom-brep-sketch-segment-full-turn
kind: unit
title: geom-brep's SketchSegment takes the canonical arc form; certify, topo description readers and the symbolic tier re-keyed on Δθ
status: open
opened: 2026-09-25
priority: P1
cost: H
parent: lower-profiles-to-carrier-and-interval-not-vertex-and-bulge
---


Unit 2 of the #3218 lowering. `SketchSegment::Arc{a,b,bulge}` becomes the profile's arc form; `restrict` becomes Δθ·(s1−s0). Certification, the topo description readers (`split.rs`, `offset_axial.rs`, `replace_face.rs`, `transform.rs`), and `geom-core` sym/trig re-key from 4·atan b to Δθ (DECIDE's `rule-d-reaches-the-unit-bulge-only` is reshaped; announce there). Byte-identical for partial arcs. Measure the `m10_*_interval` rows. Survey §1d, §3(g).

Unit 1 found that tan(Δθ/4) does not round-trip the bulge: b = 1.0
comes back 0.9999999999999999. So `swept::sketch_segment`, `arc_span`
(`param_end`, `axis_arc_span`), `arc_apex` and `axis.rs::radial_extent`
still read the kept bulge. This unit moves them to Δθ along with
`register_span_identity` and the sym rules, and re-baselines what moves.

`arc_span`'s equality to |sweep| has only been shown at f64. At
Interval, `|4·atan b|` and `4·atan|b|` are different enclosures on a
box that straddles zero, so the move to Δθ has to decide which one the
span is (and what `register_span_identity` is stated about). Found in
review of #3224.
