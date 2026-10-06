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
`work/contact/volume-door-reads-a-tiny-valid-boolean-result-wrong`
(PR 3977).

A sign read off a body's signed volume stands only where the volume's
enclosure excludes zero. Where a sign walk holds a lane, the role is read
off the round's interval re-derivation (`topo::props::certify_role`).
That covers check 7, check 10's role read, `classify_shells` and point
containment's side at infinity (`at_infinity_side`, which measures in
closed form and certifies through `AtRestPolicy::quad_lane`).

The re-derivation lifts the stored geometry to the interval scalar,
which needs a lane. So these reads, which hold none, still decide on
the `f64` sum. A body whose volume is below that sum's rounding can read
a sign there that the certified doors refuse to read (witness: `topo`'s
`tier3_tests::far_anchored_slab` at 5 km, whose sum reads −1.2e-12 m³
for a +1e-13 m³ slab):

- `census.rs`'s gate-shell filter calls `validate::shell_role(…, None)`.
  A rounding `Void` drops a shell from the gate's containers. Unread is
  kept, the conservative way. The census is now bounded on
  `AtRestPolicy`, so `T::quad_lane()` is one argument away.
- The `_structural` validators (`validate_geometric_structural`,
  `validate_pseudomanifold_structural`, `contact_marks_structural`):
  check 7 through `PlusVCheck::Through(None)`, so the same body can
  pass `validate_geometric` and be refused `NegativeVolume` here at
  `f64`.
- `classify_shells_structural`, whose doc says so.

**The question.** `sign_walk` now takes the measuring lane and the
certifying lane separately (`at_infinity_side` measures in closed form
and certifies), so a `_structural` door could keep "check 7 made through
the closed form" and still certify its sign. Whether that reading of
the lane-free contract is allowed is for the orchestrator.
