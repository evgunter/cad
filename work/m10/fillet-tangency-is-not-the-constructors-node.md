---
id: fillet-tangency-is-not-the-constructors-node
kind: issue
title: the Fillet step's declared tangency cannot be registered: the CENTRE carrier_line_circle asks about is re-derived by build_seg, not the constructor's node
status: open
opened: 2026-09-06
---

**Measured by M10-9** (the registered-identity door, branch
`m10/m10-9-registered-identity`), on R2's rounded-corner pad
(`crates/editor-core/tests/m10_8_r2_probes_interval.rs`, `pad`), the
document `work/m10/declared-tangency-needs-the-registered-identity-door`
filed as the door's live consumer.

The `Fillet` step's declared tangency is the second registrant M10-9's
spec named, and **it cannot be registered as the constructor stands**.
This row is the RE-CUT of a first version that named the wrong node.

## The obstacle is the CENTRE, not the vertex

The first cut of this row said the emitted vertex was not the
constructor's node, on the strength of a hand transcription that
anchored the leg at a different point from the real one. Read against
the code, that is false:

- `ProfilePathBuilder::push_arc` (`crates/profile/src/path.rs:1989-2002`,
  called at `path.rs:2605` as `self.push_arc(trims.t2, trims.bulge,
  arc)?`) stores `trims.t2` **verbatim** as the emitted vertex's `pos`.
  Nothing is recomputed. So the vertex the joint classifier reads —
  `line.a`, `line.b`, the arc's start — IS the constructor's own node,
  and a registration stated about it reaches the consumer.

What never reaches the joint is the **CENTRE**. The classifier does not
receive it: `validate.rs:1328` calls
`seg::build_seg(a.pos, b.pos, a.bulge, band)`, which RE-DERIVES the
carrier from the three stored numbers alone
(`crates/profile/src/seg.rs:140-148`):

```
mid    = a.lerp(b, 1/2)
n      = perp((b − a)/len)
apothem = len·(1 − β²)/(4β)
center = mid + n·apothem
```

whereas `fillet_arc_carrier` (`crates/profile/src/path.rs:2356`) holds
`center = t2 + n̂·(σ·r)` with `n̂ = (−u2.y, u2.x)`. Two different node
graphs for one point, and it is the centre that
`carrier_line_circle` asks about — `h = line.unit.perp_dot(center −
line.a)`, `crates/profile/src/seg.rs:498-505`. That is the whole
obstacle, and it is a data-flow fact about the profile representation
(a vertex list plus bulges; no carrier travels with a segment), not a
limit of the door.

**Why the constructor cannot simply state it.** The registration would
have to be `center_consumer = center_registrant`, and the constructor
cannot BUILD `center_consumer`: `build_seg`'s closed form needs the
arc's FAR endpoint `b`, which does not exist yet at `push_arc` — it is
pushed by whatever follows the fillet. The same-object condition is not
the problem; the constructor's not holding the operands is.

## The two rendered forms

`m10_9_evidence_interval::m10_9_the_fillet_tangency_residual_rendered`
prints both, on one corner's numbers, and the door ANSWERS `Recorded`
when the consumer's own node is handed to it: the door is not the
obstacle. The pad's real residual is worse than any transcription —
its radius arrives as a FROZEN atom and the rest carries `atan2`, `sin`
and `cos` atoms from the path algebra (the same probe prints it).

## Measured consequence: it is not what bounds the pad

- `carrier_line_circle` on the pad is 24 decisions at the nominal, all
  numeric, door open or shut.
- The staged-ceiling dial settles it directly (`k_stats::identity_pass`,
  the test-only `identity-pass-testing` feature): PASSING
  `carrier_line_circle` — measuring the pad as if the tangency were
  discharged — leaves the ceiling exactly where it was, and so does
  passing `line_span`, and so does passing both.

  | passed | pad's whole-certifying ceiling |
  | --- | --- |
  | — | `[2.0831e3, 2.0839e3] · ε` |
  | `carrier_line_circle` | `[2.0831e3, 2.0839e3] · ε` — unmoved |
  | `line_span` | `[2.0831e3, 2.0839e3] · ε` — unmoved |

- The pad's ceiling is `[2.083e3, 2.084e3] · ε` at ε = 1e-6, 1e-9 and
  1e-12 alike, unmoved by the door, and what is over the band at
  ceiling + δ is `carrier_matches_mapped_source` — not this predicate
  and not `line_span`.

So the tangency is a real obstacle to the tier's REACH and not a
constraint on this document's ceiling. **Do not build it for the
ceiling's sake.**

## The two routes, if it is ever built

1. **The constructor states the centre later.** Register at a point in
   the builder where both arc endpoints are known, re-deriving
   `build_seg`'s closed form there. No funnel edit — but it duplicates
   `seg.rs:140-148` inside `path.rs`, and a registration whose truth
   depends on two spellings staying character-identical is a theorem
   the next edit silently breaks.
2. **The segment carries its declared carrier.** `build_seg` accepts a
   declared centre instead of re-deriving one, so the constructor's
   node reaches the joint. This is the honest fix and it is an edit at
   a funnel site: a change to the profile representation and to what
   validation trusts, which M10-9's spec and ERROR-DESIGN E12 both put
   outside a tier unit.

Both are decisions for the unit that owns the path algebra, taken with
`work/m10/declared-tangency-needs-the-registered-identity-door`.
