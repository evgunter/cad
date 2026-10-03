---
id: coincidence-recourse-says-lower-where-d4-says-tighten
kind: issue
title: geom-core: COINCIDENCE_RECOURSE's third arm says 'lower the tolerance' where D4 ¶1 names it 'tighten the tolerance', conditional and valued
status: review
opened: 2026-09-28
branch: props/recourse-grammar
pr: 3942
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

## Resolved (props/recourse-grammar)

**Scope taken: the constant loses the arm it cannot carry, and the
escalation composes it.** D4 ¶1 (i) wants the tolerance arm conditional
and valued, and the value comes from the margin — so no `&'static str`
can hold it. The three constants on this shape
(`COINCIDENCE_RECOURSE`, `NO_DECLARATION_RECOURSE`,
`SPLIT_PLANE_RECOURSE`) are now the LEVERS alone, and
`geom_core::Indeterminate::ending(levers)` composes the whole labelled
ending through `MarginDiag::sized_recourse` — the one home for the
valued offer, with no new spelling of any arm.

So every site keeps the spelling it had and gets the right sentence:
a site composing the constant bare is a definite arm, which D4 gives no
size to tighten below; a site rendering an escalation gets the valued
conditional from that escalation's own margin, and loses it exactly
where the margin gives no value (a straddling enclosure, an unreadable
one).

`DEFINITE_COINCIDENCE_RECOURSE` retires into `COINCIDENCE_RECOURSE`:
with the tolerance arm gone the two were one string, and two names for
one string is the defect this unit closes. Its two callers moved.

Also fixed on the same shape, inside the fence: `IndeterminateUnder`'s
Display now folds each margin kind's own first lever ("subdivide the
parameter box", "check the operation's inputs upstream") INTO the
labelled recourse rather than leaving it in front of an unlabelled
menu, which is `props-escalation-renders-the-coincidence-menu-unlabelled`'s
complaint about the same rendering.
