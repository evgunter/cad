---
id: coincidence-recourse-says-lower-where-d4-says-tighten
kind: issue
title: geom-core: COINCIDENCE_RECOURSE's third arm says 'lower the tolerance' where D4 ¶1 names it 'tighten the tolerance', conditional and valued
status: open
opened: 2026-09-28
---

(ENCL implementer, from the §5 sweep of PR 3382.) The rule is D4 ¶1 (i)
in `docs/DESIGN.md`; `geom_brep::recourse::SizedDecision` is the shared
ending table for a sized decision.

## What

`geom_core::COINCIDENCE_RECOURSE` (`crates/geom-core/src/predicate.rs`
~:1002) is "declare the coincidence, move the geometry, or lower the
tolerance". D4 ¶1 (i) names the three-arm sentence "declare the
coincidence / move the geometry / tighten the tolerance". It offers
tightening only on a band-decided arm of a decision that passes on a
nonzero sign, conditionally and valued. The constant is composed by
`Indeterminate`'s own `Display` and by every coincidence site, so its
third arm is unconditional and unvalued wherever it renders.

Related: `work/chrome/escalations-forwarded-whole-are-untriaged-for-a-declarations-object.md`
(which forwarding sites should render it at all).

## Repair shape

This touches every coincidence site's text, so it needs its own
decision on scope: whether the constant keeps a third arm at all, or
whether each coincidence decision composes its own valued conditional
from its margin (`Indeterminate` carries the margin and band).
