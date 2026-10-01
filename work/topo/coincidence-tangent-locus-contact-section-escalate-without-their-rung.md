---
id: coincidence-tangent-locus-contact-section-escalate-without-their-rung
kind: issue
title: topo: the tangent-locus, contact and section coincidences escalate without their rung, so none can offer the tolerance a length rung gives
status: open
opened: 2026-09-30
---


(TOPO, PR 3513's fourth fix pass: the coincfr3 review's Q6 style note.)

## What

Three coincidence questions end on their lever alone because their
escalation does not say which rung refused (`LeverPass::ByRung`,
`Coincide::ending` in `crates/topo/src/boolean/refusal_routes.rs`):

- `Coincide::TangentLocus`: `rest::tangent_locus` returns a bare
  `Indeterminate` (`TangentLocusError::Escalated`), wrapped at
  `boolean::verify_tangent_declaration` (`mod.rs`) and at
  `insert::record_germ_dir` and `sectors::tangent_lump`;
- `Coincide::Contact`: `contact::ContactRefusal::{Escalated, Undeclared}`
  at the declared-contact verdict door (`mod.rs`, `contact_pair_verdict`'s
  caller);
- `Coincide::Section`: `join::FrameError::Escalated` at
  `join::frame_refusal`, the section pose's rows.

Each family has rungs that pass on different sets. Some may be lengths
a user intends that pass on a nonzero sign (a section pose's offset,
say), where D4 ¶1 (i) would offer the tolerance the margin gives; others
pass only at zero (the review judges the tangent locus's gap and axis
rungs pass only at zero, so "no tolerance" is right there in practice).
No rung here has been walked for its pass set yet.
Without the rung the refusal cannot tell them apart, so it offers none,
which (i) permits but does not require.

This is the same shape as `circle-torus-lane-escalates-without-its-rung`
(the circle × torus lane, `ArcTorusRoots`).

## Repair shape

Carry the rung, as `solid_contain::WallRootFault` carries `WallRung`:
each producer returns a closed rung type, and each rung ends from its
own pass set: the length rungs sized, the zero-only rungs on the lever
alone. The executed-offer census (`boolean::refusal_routes::offer_rows`)
then asks for a case per sized rung.
