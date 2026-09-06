---
id: fillet-tangency-is-not-the-constructors-node
kind: issue
title: the Fillet step's declared tangency cannot be registered: the node the constructor holds is not the node carrier_line_circle builds
status: open
opened: 2026-09-06
---


**Measured by M10-9** (the registered-identity door, branch
`m10/m10-9-registered-identity`), on R2's rounded-corner pad
(`crates/editor-core/tests/m10_8_r2_probes_interval.rs`, `pad`), the
document `work/m10/declared-tangency-needs-the-registered-identity-door`
filed as the door's live consumer.

The door was built and one registrant ships (the swept arc's rim
identity). The `Fillet` step's declared tangency is the second
registrant M10-9's spec named, and **it cannot be registered**: the door
aliases NODES, and the node the constructor could state is not the node
the joint classifier builds.

## The two forms, rendered

Transcribed from the two sites verbatim onto one corner's numbers
(`m10_9_evidence_interval::m10_9_the_fillet_tangency_residual_rendered`,
which prints both):

- **The consumer** (`crates/profile/src/seg.rs:326-343`,
  `line_circle_joint`) decides `carrier_line_circle` on
  `radius − |h|` with `h = line.unit.perp_dot(center − line.a)` —
  `line.unit` the EMITTED leg's `chord / len`, `line.a` its start
  vertex.
- **The registrant** (`crates/profile/src/path.rs:2356`,
  `fillet_arc_carrier`) holds `t2`, the arrival direction `u2` and the
  radius, and builds `center = t2 + n̂·(σ·r)`, `n̂ = (−u2.y, u2.x)`. The
  best `|h|` it can state is `|u2.perp_dot(center − t2)|`.

Rendered, on the most generous reading available — the registrant's
`u2` spelled as the same unit vector the leg's chord produces — the two
normal forms are **character-for-character identical**:

```
1·abs((-2·by·copysign(1, 1)·ay·r + 1·by^2·copysign(1, 1)·r
     + 1·copysign(1, 1)·ay^2·r + -2·copysign(1, 1)·r·bx·ax
     + 1·copysign(1, 1)·r·bx^2 + 1·copysign(1, 1)·r·ax^2)
    / (1·sqrt(-2·by·ay + 1·by^2 + 1·ay^2 + -2·bx·ax + 1·bx^2 + 1·ax^2)^2))
```

and the two NODES are not:

```
CONSUMER   |h| node 4ec13c47cb1264b772d59caa04b7ea3b
REGISTRANT |h| node b65a68dfe7dd09847fbf98930c355ebc
same node? false
```

The difference is the two subtractions: the consumer forms
`center − line.a`, the registrant `center − t2`, and `a ≠ t2` as nodes
even where the anchoring is such that the two differences denote the
same real. A registration on the registrant's node therefore reaches
nothing the consumer asks about. Registering the CONSUMER's node is
accepted by the door (`Recorded`) — the door is not the obstacle — but
the constructor does not hold `line.a` or `line.unit`, and building
them at the joint would be an edit at a funnel site, which M10-9's spec
and ERROR-DESIGN E12 both forbid.

**Measured consequence**: `carrier_line_circle` on the pad is 24
decisions at the nominal, all numeric, door open or shut. The pad's
whole-certifying ceiling is `[2.083e3, 2.091e3] · ε` at ε = 1e-6, 1e-9
and 1e-12 alike, unmoved by the door, and the first refusal beyond it
is not this predicate but `line_span`, enclosure
`[-1.666 · ε, 1.666 · ε]` — the real-margin dependency-widening class
(`work/m10/real-margin-dependency-widening`).

## Why a form-level store would not fix it either

Even as forms, the identity is not reached: the denominator is
`sqrt(‖d‖²)²` — the sqrt ATOM squared — so `|σ·r·‖d‖²/‖d‖²| = r` needs
rule A (`sqrt(X)² = X`), which is built, dial-selectable and off
because it adds no discharge on any measured document. And the REAL
pad's `carrier_line_circle` residual is worse than this transcription:
its radius arrives as a FROZEN atom and the rest carries `atan2`, `sin`
and `cos` atoms from the path algebra (the same probe prints it).

## What is owed

- A decision on which of the three doors, if any, is worth opening:
  (a) the fillet emitting its leg so that the constructor holds the
  node the joint classifier will build — a change to the path
  algebra's data flow, not to the tier; (b) rule A on, measured
  against a document where it pays; (c) leaving `carrier_line_circle`
  numeric and noting that it is not what bounds the pad anyway.
- Whichever is taken, the pad re-measured, and this row and
  `work/m10/declared-tangency-needs-the-registered-identity-door`
  re-cut against it.
