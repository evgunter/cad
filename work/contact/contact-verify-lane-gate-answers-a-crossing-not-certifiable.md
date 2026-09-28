---
id: contact-verify-lane-gate-answers-a-crossing-not-certifiable
kind: issue
title: tangent_locus_relation refuses a declared contact across a transverse out-of-lane crossing NotCertifiable, where in lane it refuses Contradicted: the lane gate runs before the first-order reading
status: open
opened: 2026-09-28
priority: P3
cost: M
design: true
---


## Finding

`crates/topo/src/boolean/contact_verify.rs`'s `tangent_locus_relation`
asks `geom_brep::tangent_certificate_lane` before anything else and
refuses `ContactRefusal::NotCertifiable` ("outside the jet certificate's
span-bound lane (the order-k boundary)") for a triple outside it. The
per-sample checks that can refuse `Contradicted` never run there: the
senses check (`"contact_tangent_opposed"`, which refuses under
`"contact_tangent_independent"` when the normals are definitely
perpendicular) and the first-order check (`"contact_tangent_parallel"`,
when the tangent planes are definitely distinct). For a right-angle
crossing — the plane `y = 0` against the 45° cone about `z`, along the
ruling in the `x = z` half-plane, ENCL's
`out_of_lane_crossing` in `crates/sweep/tests/must_carry_rule.rs` —
the `Contradicted` a walk past the gate would give comes from
`contact_tangent_opposed` reading `Zero` (predicate
`contact_tangent_independent`), before `contact_tangent_parallel` is
asked. So a declared tangent contact whose two surfaces actually CROSS
answers by carrier kind: `Contradicted` on a pair the
lane admits, `NotCertifiable` on one it refuses (a `Line` locus on a
cone or torus, any `Nurbs` carrier or surface). The second tells the
user the configuration is beyond what the kernel certifies, when the
declaration is simply false. D4 ¶1 (iv) asks one geometric fact to
tell one story with one recourse.

Found by reading, in the sweep of ENCL's
`must-carry-lane-gate-hides-a-transverse-out-of-lane-pair`.

## Not a reorder

`geom_brep::must_carry_over_edge` was fixed by moving its lane gate
behind a first-order reading that needs no span bounds. That shape
does not carry over: the per-sample walk reads
`geom_brep::tangent_span_bounds` throughout — the on-surface residual
checks that open every sample pad by `residual_sag`, and
`contact_tangent_parallel`'s lever is chosen by the second-order
margin, which subtracts `kappa_drift` — and `tangent_span_bounds`
returns `None` out of lane. Moving the gate down leaves the walk with
no bounds to read. The fix needs a first-order path that reads no span
bound (the opposed and parallel readings at the ordinary angular
lever, which is `contact_tangent_opposed`'s already), taken for an
out-of-lane triple before it refuses `NotCertifiable`.

## Measure first

**Unmeasured**: no row has driven a declared contact across
an out-of-lane crossing through this door; whether any public path
reaches it with such a pair is the first thing to measure. Both answers
are refusals, so nothing is stored wrong; what differs is the refusal's
kind and recourse.
