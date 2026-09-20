---
id: boolean-rebuild-folds-an-in-band-second-order-into-conventional
kind: issue
title: topo::boolean::ops folds an in-band tangent_second_order into the conventional posture, citing a tier-3 stance tier 3 does not take
status: open
opened: 2026-09-13
priority: P0
cost: D
---


## Finding (found by BLEND-9's sweep; filed here because `work/bool/program.md` claims `crates/topo/src/boolean`)

BLEND-9 removed this exact drift from `sweep::revolve::upgrade` — an in-band
`tangent_second_order` verdict folded into "not determinate" and the
conventional description stored silently, where the predicate's own contract
(D4 ¶3) says an in-band verdict escalates TYPED and is never silently either
side. The boolean rebuild still spells it.

**The fold.** `crates/topo/src/boolean/ops.rs`, the `jet_determinate` block
(`let jet_determinate = { … }`, ~`:1048`–`:1071`): the per-station `match` is
`Ok(Sign::Positive) => {}` and `_ => { det = false; break; }` (~`:1064`–`:1068`).
The wildcard swallows `Err(Indeterminate)` together with `Ok(Zero)` and
`Ok(Negative)`, so a near-osculating pair — certifiable as NEITHER — is built
with a conventional description and no escalation reaches the caller.

**The comment that justifies it.** Directly above (~`:1040`–`:1047`): *"a
zero-side or in-band sample keeps the CONVENTIONAL posture (tier 3's ratified
`SmoothUnderdetermined` stance …)"*, with the parenthetical arguing from
coplanar planes' exact-zero jet. The zero-side half of that sentence is right;
the in-band half attributes to tier 3 a stance tier 3 does not take.

**What tier 3 actually does.** `crates/topo/src/validate.rs`, the must-carry
arm's station loop: `match decide("tangent_second_order", margin, band)` at
~`:4029` separates the two — `Ok(Sign::Zero | Sign::Negative)` sets
`jet_determinate = false` and breaks (~`:4048`), while `Err(cause)` pushes
`ValidationError::SliverDihedral { edge, cause }` and breaks (~`:4051`). So an
in-band sample is an F6 REFUSAL at tier 3, not the `SmoothUnderdetermined`
posture. A body this fold builds is a body the at-rest gate then refuses under
the very predicate the fold swallowed — the same shape BLEND-9 measured in
revolve (`review_fillet_h6_r1_probes`'s row, before it was re-baselined).

**Not fixed here**: `crates/topo/src/boolean` is BOOL's ground, and the change
is a behaviour change (a boolean that builds today would start refusing), so it
wants its own unit. The same block also hand-rolls the folded lever arm
(`curvature_lever_arm(surf1).min(curvature_lever_arm(surf2)).min(extent)`,
~`:1056`) and the sagitta rather than calling `geom_brep::tangent_second_order`
— it is one of the two remaining second-order siblings that predicate's doc
names (issue 1439's work), and routing it through
`geom_brep::must_carry_over_edge` would settle the fold and the hand-roll
together.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to CURVED (its charter names S-BOOL's ceded ground and inherits at S-BOOL's exit) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.
