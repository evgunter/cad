---
id: contact-tangent-relation-decides-offsets-and-tilt-per-sample
kind: issue
title: check whether contact_verify's tangent relation decides the offsets and the tilt one at a time
status: open
opened: 2026-10-07
priority: P3
cost: M
---

## What

Investigation, found by PR 4280's sweep and not traced to a served
verdict there. `topo::boolean::contact_verify`'s `tangent_locus_relation`
(`crates/topo/src/boolean/contact_verify.rs:286`) decides, per sample,
`contact_tangent_on_1` / `contact_tangent_on_2` (residual less sag,
near 326) and `contact_tangent_parallel` (the normals' sine levered,
near 427) as separate rows, then serves a tangency. It runs inside a
jet certificate, which may already cover the sum by design.

## The shape of a fix

Trace whether a sample whose offset and tilt each read just inside
the band reaches a served tangency; if it does, decide their sum
(`decide_across` in `crates/geom-brep/src/intersect.rs`).
