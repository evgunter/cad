---
id: budget-refusal-drops-the-enclosure-the-caller-needs
kind: issue
title: a budget refusal from refine_to_target drops the enclosure, so a consumer pattern-matches two crates down to get it back
status: open
opened: 2026-09-12
---


## The finding

Raised while writing the consumer side of
`gate-then-measure-pays-two-quadratures` (PR 2440), and it is a
DEMO-FOUND gap in the sense `memories/demo-purpose.md` means: the
awkwardness is in the library, not in the demo that hit it.

After Ev's 2026-09-12 ruling a consumer that gates and then measures
has to answer one question — *is this the budget refusal my
certificate warned about, or a body with no volume at all?* — and then
report the bracket in the first case. Neither half is reachable
without reaching past the door it came from.

**The refusal drops what the caller needs.** `refine_to_target`
(`crates/topo/src/props.rs`) consumes the certificate and answers
`Err(MassPropsError)`. The enclosure the gate certified is gone with
the certificate, so the consumer must read it BEFORE the call — before
there is anything to decide on — and hold it across the call on the
chance it turns out to be wanted:

```rust
let sign_level = certificate.enclosure();   // in case
match certificate.refine_to_target() { .. }
```

`SignCertificate::target_refusal` exists and answers exactly the
classification question, but it is a method on the certificate, so it
is unreachable once the call that produces the refusal has consumed
it. Asking it first means asking before the walk that decides has run.

**So the classification is spelled two crates down.** With no answer
on the error, both consumers written for that ruling hand-spell the
arm:

- `demos/tour/src/main.rs`, `run_body`'s tier-3 arm;
- `crates/pncad-py/src/py/value.rs`,
  `Body::validate_geometric_measured`.

Both match
`topo::MassPropsError::Face { source: geom_brep::PropsError::QuadratureBudget { .. }, .. }`
— a `topo` door's error opened to name a `geom-brep` variant. A
consumer of `topo` should not have to depend on `geom-brep`'s refusal
vocabulary to tell a valid-but-unmeasurable body from a corrupt one,
and every future consumer of this ruling will copy the same match.

## What a fix is

Kernel territory, so this is a finding and not a plan. Two shapes,
either of which removes both halves:

1. **The refusal carries the enclosure.** A budget refusal from
   `refine_to_target` is exactly the case that HAS a certified
   bracket; answering it as a typed arm that carries
   `VolumeEnclosure<T>` makes the consumer's match one level deep and
   deletes the read-before-the-call.
2. **The certificate survives the call.** `refine_to_target(&self)`,
   or a `refine_to_target(self) -> Result<_, (Self, MassPropsError)>`
   that hands the certificate back on refusal, leaves
   `target_refusal` and `enclosure` reachable where the answer is
   needed.

(1) is the smaller change and the one the ruling's shape asks for:
what a consumer wants at that point is the bracket, not the
certificate.

## Scope note

This does not touch the reporting contract or the quadrature
schedule — it is about what a refusal CARRIES, not about which bodies
refuse.
