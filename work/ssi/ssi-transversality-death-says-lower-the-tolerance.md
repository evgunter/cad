---
id: ssi-transversality-death-says-lower-the-tolerance
kind: issue
title: ssi: the transversality-death refusal says 'lower the tolerance' unconditionally, unvalued
status: open
opened: 2026-09-28
---

(ENCL implementer, from the §5 sweep of PR 3382.) The rule is D4 ¶1 (i)
in `docs/DESIGN.md`; `geom_brep::recourse::SizedDecision` is the shared
ending table for a sized decision.

## What

`crates/geom-brep/src/ssi.rs` (~:437), the transversality-death
refusal's `Display`: "...separate the operands, declare the tangency,
or lower the tolerance". It carries `sin_theta` and `arm`, so the
margin that would value a conditional tighten is in the payload. It is
unconditional and unvalued, and unlabelled (no `Recourse:` marker).

## Repair shape

Decide whether the death is a band-decided arm of the transversality
decision (`geom_brep::certify::CertCheck::Transversality` is the same
question on an edge). If it is, end it through that decision's
`SizedDecision`, with "declare the tangency" kept only where the door
takes a declaration. If it is not, name the lever alone.
