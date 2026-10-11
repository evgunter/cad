---
id: material-jet-readings-take-plus-minus-order
kind: issue
title: tier 3, rim_wedge and contact_verify read the tangent jet in (plus, minus) order, a third order beside the certificate's key order
status: open
opened: 2026-10-06
priority: P3
cost: M
refs: [intersection-pair-order-is-unpinned-and-extrude-disagrees-with-itself]
---



## Finding

- **Where**: `crates/topo/src/validate.rs`, tier 3's must-carry and
  material arm (`tangent_jet(s_plus, s_minus, ..)`,
  `folded_lever_arm(s_plus, s_minus, ..)`);
  `crates/topo/src/boolean/rim_wedge.rs`
  (`tangent_second_order(s_plus, s_minus, ..)`); and
  `crates/topo/src/boolean/contact_verify.rs`
  (`tangent_jet(s1, s2, ..)` over a declared contact's A and B faces).
- **Raised by**: the review of PR 4189 (`SurfacePair`), 2026-10-06.

PR 4189 put the certificate and every constructor on one order for the
tangent reading: the surface pair's key order. These three readers take
a third order instead, (plus, minus). It is not a mechanical change,
because they feed the jet's **signed** κ_rel to
`material_kappa_rel(jet.kappa_rel, sense_plus)`, so the order there
carries the material side. `contact_verify`'s two surfaces belong to two
different bodies, so no surface pair exists for it at all.

Off exact tangency, κ_rel depends on argument order
(`geom_brep::tangent`'s docs). So tier 3's second-order margin
`|κ_rel|` can, in principle, read a different verdict from the
certificate's at the same station.
`contact_edge_must_carry::the_rule_reads_every_corpus_contact_the_same_in_both_orders_at_interval`
measured no difference on the carving corpus.

## The question

Should the must-carry margin read the jet in key order, with the
material sign read separately in (plus, minus)? Or is the (plus, minus)
order right, with the certificate taking it from the topology? The
second answer is what the weighed design rejected for the
description, since the description holds no topology.
