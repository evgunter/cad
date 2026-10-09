---
id: must-carry-in-band-verdict-does-not-say-which-decision-escalated
kind: issue
title: geom-brep: MustCarryVerdict::InBand carries a station's escalation without saying whether the wedge or the second-order question escalated
status: closed
closed: 2026-10-09
branch: encl/sweep-must-carry-escalation
pr: 4450
opened: 2026-10-01
priority: P3
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
arm of the contact-edge description pass) reports every in-band
verdict as `BlendDecision::ContactSecondOrder`. A tolerance offer — "if
this separation is intended, tighten the tolerance below m/K" — is true
of the second-order reading and false of a wedge one, where a smaller
tolerance decides `Transverse` and the surgery refuses. Telling them
apart would mean matching `source.predicate`, which D4 ¶1 (i) rules out
("the decision is a closed type at its site ... never a lookup by
predicate name"), so the blend offers no tolerance at all on this
decision: the second-order reading is owed one and does not get it.

Not reached by any fixture: the blend's contact edges are tangent by
construction, so a wedge station reads Zero far inside the band.

## Repair shape

Let the verdict say which question escalated — `InBand` carrying a
closed station decision beside the `Indeterminate` (or two arms) — so a
caller maps it to its own decision exhaustively; the blend then gives
the second-order reading its `SizedPass::AnySign` ending back.

## Since (FUSE, PR 3889): the verdict says which question escalated

The repair shape above is in: `MustCarryVerdict::InBand` (and
`MustCarryRefusal::InBand`) carry a `geom_brep::MustCarryEscalation`,
`FirstOrder(LeverEscalation)` with its rung or `SecondOrder(Indeterminate)`,
with `.diag()` for a caller that wants only the escalation. The rule
also classifies every station first-order before it reads any second
order, so a first-order in-band station escalates wherever it sits.
`topo::boolean::ops::seam_must_carry` maps it exhaustively (the seam's
`LeverArm(Seam)`/`SeamWedge` by rung, `BooleanDecision::SeamJet` for the
sagitta).

**What remains of this row**: the sweep callers still call `.diag()`
and drop the reading. `sweep::blend::surgery`'s contact edge still ends
every in-band verdict as `BlendDecision::ContactSecondOrder`, so a
first-order station is reported as the second-order question and the
second-order reading is still owed its `AnySign` ending; `sweep::extrude`
(strut, cap rim) and `sweep::revolve::upgrade` fold both into their
sliver errors.

## Since (ENCL, PR 4450): the sweep callers map it exhaustively

`topo::DihedralReading::of_must_carry` (beside `of_lever`) is the one map
from the escalation to a reading; `sweep::blend` maps the reading to its own
decision (`BlendDecision::of_contact`): `ContactArm` / `ContactWedge` by rung, each
on its own lever with no tolerance, and `ContactSecondOrder` with its
`SizedPass::AnySign` ending. `sweep::extrude` (strut, cap rim) and
`sweep::revolve::upgrade` carry `reading: topo::DihedralReading` on their
sliver errors (`DihedralReading::of_must_carry`) and word the second-order bend
apart from the first-order sliver (`swept::sliver_text`).

## Closed

2026-10-09. PR 4450 merged at `59cdb05871` after a full review, a fix pass and a delta review; hosted CI was green.
- **Main fix ported.** Main's editor-core `lib_g17` crossing was ported in from PR 4451.
- **Moved texts:**
  - The blend's contact-edge in-band refusals: the arm and the wedge each get their own subject and lever, and the second order gains the tolerance offer.
  - The extrude/revolve second-order slivers now read "whether the faces at … curve apart there or share their curvature is undecided: …".
- **Follow-up recorded on CARVE's row** (`sweep-dihedral-readers-drop-the-arm-rung`): the false tolerance offer at extrude/revolve's first-order must-carry stations, where `Lever(rung)` cannot tell a must-carry station from the witness.
