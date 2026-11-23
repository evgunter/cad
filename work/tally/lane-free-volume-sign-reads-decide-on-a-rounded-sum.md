---
id: lane-free-volume-sign-reads-decide-on-a-rounded-sum
kind: issue
title: Lane-free reads of a body's volume sign decide on the rounded f64 sum, with no interval re-derivation to certify it
status: closed
closed: 2026-10-19
branch: flush/shell-role-reader
pr: 5119
opened: 2026-10-03
priority: P3
cost: M
refs: [3977]
---

Residue of REACH's check-7 fix for
`volume-door-reads-a-tiny-valid-boolean-result-wrong` (closed, PR 3977)
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

## The census bullet is answered (FUSE `fuse/one-home-shell-stands`)

The census's gate-shell filter now reads `census::gate_role`, which is
the one shell-role reader (`props::shell_role`) through
`T::quad_lane()`. `tier3_tests::the_census_gate_reads_a_shell_role_as_the_classification_does`
pins it on the far-anchored slab. The `_structural` doors and
`classify_shells_structural` are unchanged.

## Resolution (FLUSH, PR 5119 fix pass 3)

A lane-free read no longer decides on the rounded sum, and no longer
holds `V/A` against the band. `props::certify_role` reads a walk that
holds no lane as the enclosure `[V − b, V + b]`, `b` a stated bound on
the sum's rounding (`props::rounding_bound`: `16·u·Σ_f (n_f + 1)·R_f·D_f²`,
each face's edge count, reach from the origin and spread about its
surface's anchor), for its sign alone (`props::read_sign`). A volume
within it is `SignLimit::Scalar`: `ShellClassifyError::SignUnresolved` /
`ValidationError::VolumeSignUnresolved`, a capability limit ending
`NOT_YET_ENDING`, never a defect. So the `_structural` doors and
`classify_shells_structural` read the sign where the sums carry it and
refuse it as unresolved where they do not.

The witness pins it: `tier3_tests::a_lane_free_read_never_takes_the_rounded_sums_sign`
(the far-anchored slab, whose sum reads −1.2e-12 m³ for +1e-13: its
lane-free volume role is unresolved either way out, and the structural
door never refuses it `NegativeVolume`); and
`shell_role_reader::a_needles_orientation_through_the_structural_doors`
(at ε 1e-6 every needle bar validates and inside out is `NegativeVolume`;
at 1e-9 each is `VolumeSignUnresolved { Scalar }` but where an aimed ray
reads it first). The question of certifying the structural doors through
a closed-form lane stays open on no row: the bound answers what the
doors claim.
