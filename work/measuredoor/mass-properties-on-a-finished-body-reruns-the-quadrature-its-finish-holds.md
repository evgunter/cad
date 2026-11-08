---
id: mass-properties-on-a-finished-body-reruns-the-quadrature-its-finish-holds
kind: issue
title: mass_properties on a finished body re-runs from round 0 the certified quadrature that body's finish already holds (AtRestBody::measure continues it)
status: open
opened: 2026-11-08
---


Found by the ONEHOME finished-tail lane (branch `onehome/finished-tail`),
outside its fence.

Since that branch, every finished body at a certifying scalar
(`topo::AtRestBody`, `crates/topo/src/validate.rs`) keeps the certificate
its finish's check 7 decided on (`props::KeptSign`), and
`AtRestBody::measure` continues it under a caller's `Settle`, paying only
the rounds the finish did not run. That replaced the importer's second
finished type, `topo::CertifiedBody`.

The measurement door does not read it. `topo::mass_properties`
(`crates/topo/src/props.rs`, `mass_properties`) takes a finished body
(`F: FinishedBody`) and calls `measure_body`, which walks from round 0.
So the same finished body has two measurement spellings: one continues
the gate's rounds, and the other re-pays them. Each answers the same four
fields bit for bit (`validate_geometric_certificate`'s doc: "THE value,
not a second one"). The next author reaches for `mass_properties`, the
named door, and pays the quadrature twice.

The one place is probably `mass_properties` itself reading the finished
body's certificate: an `AtRestBody` measured through `measure`, and a
`Replay` (a dual) as it is now. That moves the decision-count goldens
(fewer `props_quad_*` rows wherever a finished body is measured), so it is
its own unit rather than a rider on the fold. It would also close
`tour-scenes-gate-and-measure-one-body-through-two-doors`'s two-door
spelling wherever those scenes measure a finished body.
