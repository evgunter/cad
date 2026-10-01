---
id: must-carry-in-band-verdict-does-not-say-which-decision-escalated
kind: issue
title: geom-brep: MustCarryVerdict::InBand carries a station's escalation without saying whether the wedge or the second-order question escalated
status: open
opened: 2026-10-01
---


(BAND implementer, from the sweep of `band/recourse-tables-decide-per-tag`.)

## What

`geom_brep::MustCarryVerdict::InBand(Indeterminate)`
(`crates/geom-brep/src/dihedral.rs`, the enum's doc) carries "either
station reading's" escalation: the first-order wedge
(`dihedral_wedge`/`dihedral_arm`) or the second-order sagitta
(`tangent_second_order`). The two are different decisions with
different pass sets: the second-order one builds on either definite
sign (intrinsic or conventional description), while a wedge decided
definitely transverse refutes the caller's smooth premise and refuses.

`sweep::blend`'s surgery (`surgery.rs`, the `MustCarryVerdict::InBand`
arm of the contact-edge description pass) now reports every in-band
verdict as `BlendDecision::ContactSecondOrder`, whose ending offers
"if this separation is intended, tighten the tolerance below m/K" —
true of the second-order reading and false of a wedge one, where a
smaller tolerance decides `Transverse`. The only way the blend could
tell them apart is by matching `source.predicate`, which D4 ¶1 (i)
rules out ("the decision is a closed type at its site ... never a
lookup by predicate name").

Not reached by any fixture: the blend's contact edges are tangent by
construction, so a wedge station reads Zero far inside the band.

## Repair shape

Let the verdict say which question escalated — `InBand` carrying a
closed station decision beside the `Indeterminate` (or two arms) — so a
caller maps it to its own decision exhaustively.
