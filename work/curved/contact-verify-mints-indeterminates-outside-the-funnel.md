---
id: contact-verify-mints-indeterminates-outside-the-funnel
kind: issue
title: contact_verify mints four Indeterminates after a definite sign, outside the funnel that would record them
status: open
opened: 2026-09-20
---


## What

`crates/topo/src/boolean/contact_verify.rs` holds four shipped sites
that ask the funnel, receive a DEFINITE sign, and then build an
`Indeterminate { margin: MarginDiag::Invalid, … }` by hand — so the
escalation (or contradiction) the caller receives is on no frame's
escalation log, and a consumer reaches it only through
`ContactRefusal`'s arms.

Found by the PROPS escalation-channel unit (PR 2928), which retired
exactly this shape at the eight `geom-brep` sites it owned and swept
for siblings. These four are `curved`'s ground, so they are filed
rather than fixed there.

- `contact_verify::…` — the `CarrierRelation::SameOriented` arm, minting
  under `contact_rest_senses_opposed`.
- the `CarrierRelation::Distinct` arm, minting under
  `contact_rest_ladder_invariant` (a ladder-contract violation).
- the two tangent-residual arms (`contact_tangent_on_1` /
  `contact_tangent_on_2`), minting under the decided predicate's own
  name after `Ok(Sign::Positive)`.
- the senses-opposed arm, minting under `contact_tangent_opposed` after
  `Ok(Sign::Positive)`.

## Note on the payload, which is the harder half

Two of these are not escalations at all: a DEFINITE `Positive` residual
is a contradiction of a declared contact, and dressing it as
`MarginDiag::Invalid` says "the margin was poison" about a margin that
was measured. `crates/geom-brep/src/props/quad.rs` names this class in
prose — *"turns into `Indeterminate{margin: Invalid}`: a MIS-TYPED
refusal"*. So the fix is not only to route the mint through a funnel
door; it is to decide, per arm, whether the fact being reported is an
indeterminacy (route it) or a definite contradiction (give it a payload
that says so, and it then belongs on no escalation log at all).

`geom_core::k_stats` gained `decide_positive`, `decide_nonzero` and
`gate_measured` in PR 2928; the first two are the doors for the arms
that really are gates.
