---
id: offset-door-escalations-share-one-subject
kind: issue
title: offset doors: ~30 Escalated raise sites render one generic subject, not the decision each one asked
status: open
opened: 2026-10-06
priority: P3
cost: M
---


(SHELL refusal-text lane, PR 4163, the style review's S6.)

## What

`ReplaceFaceError::Escalated` (`crates/topo/src/replace_face.rs`,
`impl Display for ReplaceFaceError`) renders every escalation under
one subject: "whether the offset face and its edges land where they
should is too close to call". About thirty raise sites share it:
- `offset_axial.rs`: centre-on-axis, nappe side, the corner solves
  (`ReplaceFaceError::Escalated { source }` throughout);
- `replace_face.rs`: the apex window, `offset_vertex_agreement` and
  `offset_reanchor_on_carrier`;
- the transport and section escalations.

The sentence is plain, but it names no decision.

## Why not in PR 4163

The variant carries only an `Indeterminate`. Its `predicate` name is
routing and stays in `Debug`, so rendering a subject per decision needs
one of two things:
- a decision-words table over those predicate names, as `topo`'s Boolean
  and `profile`'s path validation keep;
- a `decision` field on the variant, as `sweep::blend::BlendError::Escalated` has.

Either touches every raise site. `offset_axial.rs` is being rewritten
by PR 4151, so wait until that lands.

## Repair shape

Add a decision field, or a words table keyed on the predicate, and
render it in place of the generic subject. `refusal_concision_chains.rs`
`replace_face()` renders the arm, so the guard checks the result.
