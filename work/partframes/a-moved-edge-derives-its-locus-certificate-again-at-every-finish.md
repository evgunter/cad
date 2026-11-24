---
id: a-moved-edge-derives-its-locus-certificate-again-at-every-finish
kind: issue
title: A move drops an edge's locus certificate, so every finish of a body holding a moved edge derives limbs 2-3 again
status: open
opened: 2026-11-24
priority: P2
cost: M
---


Found by the gather-carries lane (2026-11-24), measuring what check 2's
"locus certificate read where its support is unchanged" compares.

A move drops an edge's locus certificate:
`geom_brep::EdgeCurve::carried` (`crates/geom-brep/src/certify.rs`),
`topo::transform::map_isometry`'s one caller, builds the image with
`locus: None`. So every later certification of a moved edge
(`policy_lane::certifying_reattach` at a finish's check 2, and
`certify_stated` at a door that keeps the edge) derives C2's limbs 2-3
again over the image, at every scalar. DESIGN `[one-way.places]`' at-rest
row says an edge's certificate is "carried by every door that keeps its
support"; a move keeps it up to an isometry, and the certificate's own
`reads` compares by value (`LocusCertificate::reads`), which a moved
support never matches even if it were kept, because it holds the
pre-image's carrier and surfaces.

The gather no longer meets this (PR on `boxfinish/gather-carries`: a
gather of finished members carries their verdicts and re-certifies no
edge). It remains wherever a door re-certifies a moved operand's edges:
a Boolean over a placed operand (`topo::nary_placed`,
`topo::boolean_placed`) finishes its result, whose check 2 derives limbs
2-3 again on every edge it kept from the moved operand. At `Sym` that is
the derivation the tier records anyway (`Sym::same_value` is false); at
`f64` and `Interval` it is repeated work, and at `Interval` over a boxed
motion it is two enclosures of one curve compared. Not measured: how
much of a finish's check 2 time it is. A carry would move the
certificate's support with the map, or key the read on "the same support
up to this isometry" rather than on values. (gather-carries lane)
