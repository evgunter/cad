---
id: contact-verify-lane-gate-answers-a-crossing-not-certifiable
kind: issue
title: tangent_locus_relation refuses a declared contact across a transverse out-of-lane crossing NotCertifiable, where in lane it refuses Contradicted: the lane gate runs before the first-order reading
status: open
opened: 2026-09-28
---


## Finding

`crates/topo/src/boolean/contact_verify.rs`'s `tangent_locus_relation`
asks `geom_brep::tangent_certificate_lane` before anything else and
refuses `ContactRefusal::NotCertifiable` ("outside the jet certificate's
span-bound lane (the order-k boundary)") for a triple outside it. The
per-sample first-order check (`"contact_tangent_parallel"`, which
refuses `Contradicted` when the tangent planes are definitely distinct)
never runs there. So a declared tangent contact whose two surfaces
actually CROSS answers by carrier kind: `Contradicted` on a pair the
lane admits, `NotCertifiable` on one it refuses (a `Line` locus on a
cone or torus, any `Nurbs` carrier or surface). The second tells the
user the configuration is beyond what the kernel certifies, when the
declaration is simply false. D4 ¶1 (iv) asks one geometric fact to
tell one story with one recourse.

Found by reading, in the sweep of ENCL's
`must-carry-lane-gate-hides-a-transverse-out-of-lane-pair` (the same
shape in `geom_brep::must_carry_over_edge`, fixed there by walking
every pair first-order and letting the lane gate only the second-order
reading). **Unmeasured**: no row has driven a declared contact across
an out-of-lane crossing through this door; whether any public path
reaches it with such a pair is the first thing to measure. Both answers
are refusals, so nothing is stored wrong; what differs is the refusal's
kind and recourse.
