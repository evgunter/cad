---
id: lane-free-volume-sign-reads-decide-on-a-rounded-sum
kind: issue
title: Lane-free reads of a body's volume sign decide on the rounded f64 sum, with no interval re-derivation to certify it
status: open
opened: 2026-10-03
priority: P3
cost: M
refs: [volume-door-reads-a-tiny-valid-boolean-result-wrong]
---


Residue of REACH's check-7 fix for
`work/contact/volume-door-reads-a-tiny-valid-boolean-result-wrong`.

A sign read off a body's signed volume stands only where the volume's
enclosure excludes zero. With a certified quadrature lane, check 7,
check 10's role read and `classify_shells` now certify their `f64`
reading through the interval re-derivation
(`topo::props::rederive`, read by `validate::plus_v_certify`). The
re-derivation lifts the stored geometry to the interval scalar, which
needs `T: CertifiedBounds`. So these reads, which hold no lane, still
decide on the `f64` sum. A body whose volume is below that sum's
rounding can read a sign there that the certified doors refuse to read
(witness: `topo`'s `tier3_tests::far_anchored_slab`).

- `boolean::solid_contain::at_infinity_side`: the point-in-solid walk's
  side at infinity, `T: Decide`, closed form only. A sliver solid read
  `Void` puts infinity inside it.
- `census.rs`'s gate-shell filter calls `validate::shell_role(…, None)`.
  A rounding `Void` drops a shell from the gate's containers. Unread is
  kept, the conservative way.
- The `_structural` validators (`validate_geometric_structural`,
  `validate_pseudomanifold_structural`, `contact_marks_structural`):
  check 7 through `PlusVCheck::Through(None)`, so the same body can
  pass `validate_geometric` and be refused `NegativeVolume` here at
  `f64`.
- `classify_shells_structural`, whose doc now says so.

**The likely shape.** The interval re-derivation of a closed-form face
is not quadrature, so it need not ride in `QuadLane`. As a per-scalar
door of its own (an `AtRestPolicy` method, `None` at duals), every
caller bounded on `AtRestPolicy` could certify its reading.
`at_infinity_side` and the census are `T: Decide` and would take it as a
parameter. Whether the doors that are lane-free BY CONTRACT (the
`_structural` family: "check 7 made through the closed form") may hold
it is a reading of that contract for the orchestrator.

**The pass side of check 7 is not re-derived.** Rounding to nearest
keeps each arithmetic step of the `f64` closed form inside its
outward-rounded interval twin, so a positive `f64` sum cannot sit beside
a definitely negative enclosure. A transcendental's last-ulp error is
the exception, and it reaches past the band only on coordinates of order
`K·ε·2⁵²` metres. Confirming passes too cost +55 % of
`validate_geometric` on a drilled block (measured, release, 200 runs).
That is why the arm was not taken.
