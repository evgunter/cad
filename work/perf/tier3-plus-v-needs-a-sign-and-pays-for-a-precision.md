---
id: tier3-plus-v-needs-a-sign-and-pays-for-a-precision
kind: issue
title: tier 3's +V check needs a sign and pays for a precision target
status: open
opened: 2026-09-10
parent: PERF-6
---



Tier 3's ninth check — the **+V global orientation invariant** — consumes
the SIGN of a volume enclosure and nothing else. `crates/topo/src/validate.rs`
says so at the top of the tier:

> the ninth — the +V global orientation invariant — reads a volume
> enclosure, and deciding its **sign** is an act of certification rather
> than a measurement.

It obtains that sign by running the full certified quadrature, refined to
`QUAD_TARGET_LEN_FACTOR·ε` (`crates/geom-brep/src/props/quad.rs:113`,
factor 1024). The two are coupled deliberately, at
`crates/topo/src/validate.rs:2600`:

```rust
// ONE certified quadrature, held and then handed on: the check
// decides on this object and the caller receives this object.
let certificate = crate::props::mass_properties_certified(body, band, tol);
let errors = plus_v_invariant(&certificate, band);
```

That comment is a good argument against computing the thing twice, and
this issue is not asking for a second quadrature. What it records is the
consequence: **validation inherits the convergence target of the
REPORTING use**, so a body can fail tier 3 for missing a precision that
tier 3 never reads.

**The case that found it**, from the teapot's canal spout while its loft
sections were still circles, at ε = 1e-12:

```
tier-3 geometric validation failed:
  VolumeUncomputable { source: Face { face: FaceKey(3v3),
    source: QuadratureBudget { width_len: 2.53e-8, target_len: 1.024e-9,
                               rounds: 1 } } }
```

`width_len` is the mean-boundary-displacement meter
`width(flux)/(3·area_mid)`, so the volume enclosure behind that refusal
is about `2.53e-8 × 3 × 0.0245 ≈ 1.9e-9 m³` on a body of
`V ≈ 4.96e-5 m³`. The enclosure excludes zero by roughly **five orders
of magnitude**. The sign was never in doubt at any round; the body was
refused for being 25× short of a target the check does not consume, and
a perfectly valid solid was reported as unvalidatable.

Note `rounds: 1`: the refusal came from `last_round_refuses`, the
after-round-0 predicate that proves the schedule's final round cannot
reach the target. So the work was not even spent — the face was
predicted unreachable and declined. The cost here is not CPU on this
body; it is a FALSE REFUSAL.

**Why the target cannot simply be loosened.** It is already loose, and
for a stated reason (`quad.rs:76`): *"the width floor is set by the
ε-scale endpoint/boundary slacks times the boundary length, so an
ε-tight target would refuse every real face; three decades above the
floor and far below display relevance is the useful band."* The factor
is a tuned compromise for the REPORTING use. Nothing there is an
argument about what an orientation sign needs, because the sign was not
the use being designed for.

**What a fix might look like**, cheapest first, none of them costed:

* **A sign-sufficient exit in the existing loop.** The refinement
  already computes an enclosure each round; if the caller only needs
  check 7, stop the first time the enclosure's margin against zero is
  DEFINITE under the band. That is not a second quadrature — it is an
  earlier exit from the one already running, so it strictly saves work
  and it converts this class of refusal into a pass. The check's own
  posture already speaks the language ("only a DEFINITE disagreement
  refuses").
* **Two entry points**, so `validate_geometric` asks for a
  sign-certified volume and a caller wanting the number asks for a
  target-certified one, with the second reusing the first's rounds.
  Keeps "one quadrature" for the common path where both are wanted.
* **Leave it and document it** — the honest floor, if the coupling is
  judged worth its cost. Then tier 3's docs should say that a valid
  solid can fail validation on quadrature budget, which they do not say
  today.

**Scope note.** This is not the same finding as the tour's ε sweep
needing every scene green at 1e-12; that is a demo-policy question. This
one is in the kernel and would stand if the tour did not exist: any
consumer validating a fitted rational body at a tight tolerance meets
it.

**What is verified here**: the check's stated use (module docs), the
coupling (the call site above), the factor and its rationale (quad.rs),
and the refusal's own numbers. **What is not**: whether a
sign-sufficient exit is sound in every band, and what it would cost on
the corpus. Both want measuring before anything moves.
