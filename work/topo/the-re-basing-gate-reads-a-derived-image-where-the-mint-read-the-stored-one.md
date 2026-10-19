---
id: the-re-basing-gate-reads-a-derived-image-where-the-mint-read-the-stored-one
kind: issue
title: certify_rebased_run re-certifies an OnPair edge through a derived wall image where its mint and the validator read the stored one
status: open
opened: 2026-10-19
---


Found by ENCL's param-span-doors fix pass (PR 5365) while tracing
whether a derived image's span can reach a panic at a build.

`Body::certify_edge_spec` (`crates/topo/src/euler.rs`) mints an `OnPair`
edge on a plane × NURBS pair through the wall image the body already
stores for it (`Body::stated_pair_image`, handed to
`EdgeCurve::certify_via_stated`), and the tier-3 validator re-reads it
the same way (`validate.rs`, `rederive_via_stated` with `stated`).
`Body::certify_rebased_run`, the fan site's re-basing gate, re-certifies
the same edge through `policy_lane::recertify`, which passes no stated
image, so `plane_nurbs_incidence` picks an arc rim's exact image anew
(`edge_nurbs::exact_arc_rim`, `pick_arc_rim`) rather than metering the
stored one. An edge the mint admitted through its stored image (a split
child of an arc rim, whose parent's image covers it, is the case
`EdgeCurve::recertify_via_stated`'s docs name) can then be refused by
the re-basing gate through an image no other door reads.

PR 5365 closes the panic half: an escalation of the derived image's
span now ends by the image's own span decision (`CertCheck::ImageSpan`,
a stored contradiction), not the edge's unminted one, so it no longer
panics at a build. What stays open is the asymmetry itself.

**The fix shape:** `policy_lane::recertify` takes the stated image, and
`certify_rebased_run` reads it through `Body::stated_pair_image` as
`certify_edge_spec` does. No row exercises a re-based arc rim yet; the
fix owes one.
