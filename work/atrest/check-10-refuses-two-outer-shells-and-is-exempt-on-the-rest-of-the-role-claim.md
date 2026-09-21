---
id: check-10-refuses-two-outer-shells-and-is-exempt-on-the-rest-of-the-role-claim
kind: issue
title: check 10 makes only the definite half of the shell-role claim: a zero, escalated or unmeasurable shell is exempt, so 'every other shell is a cavity' is unstated
status: open
priority: P3
cost: D
parent: ATREST-1
opened: 2026-09-20
---

Disclosed by ATREST-1 as a deviation from its spec's D-C, which states
the claim as *"each solid has exactly one shell that classifies
`Outer`, and every other shell of that solid classifies `Void`"*.

What landed refuses only the DEFINITE half of that: check 10 fires
when two or more of a solid's shells each enclose definitely-positive
volume. A shell whose signed volume is definitely ZERO, whose sign
escalates in the band, or whose quadrature produces no enclosure at
all, is EXEMPT — counted as neither an outer boundary nor a cavity.

**Why, measured.** The stated claim refuses the coplanar pillow, the
minimal tier-3-clean body in `crates/topo/src/tier3_tests.rs`
(`coplanar_pillow`): its single shell encloses exactly zero volume, so
it classifies neither `Outer` nor `Void` and the claim reads "zero
outer shells". Executed on the branch: the stated claim reds
`coplanar_split_is_smooth_and_tier3_clean`,
`the_net_state_ladder_decides_check_1s_verdict`,
`a_described_net_carrying_poison_is_named_by_the_surface_check`,
`a_finite_described_net_draws_no_surface_verdict`,
`a_scaffold_at_rest_is_refused_and_the_chart_description_is_not` and
`the_seam_is_legal_and_the_same_geometry_flipped_is_a_lamina`. Check
7's ratified posture is that `Zero` and escalated margins are exempt
— *"this check is an orientation probe, not a thinness gate"* — and
check 10 inherits it rather than re-litigating it in a lane.

The gap this leaves: a solid whose every shell is zero-volume or
in-band is not refused, and neither is a solid holding one outer
boundary beside one zero-volume shell. Closing it is a question about
the Zero exemption, which is ratified, so this row is a design
question before it is work.

`ValidationError::MultipleOuterShells`' rustdoc cites this file.
