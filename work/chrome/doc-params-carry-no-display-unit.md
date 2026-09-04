---
id: doc-params-carry-no-display-unit
kind: issue
title: The PANEL does not read a document parameter's display unit — the storage half landed, the authoring and readout halves did not
status: review
opened: 2026-09-01
github: 1459
refs: [1458]
branch: chrome/doc-params-carry-no-display-unit
pr: 1776
---

## From GitHub issue 1459

Opened 2026-09-01; 0 comments.

Found by the `story_parametric` integration lane, and partly self-disclosed in `crates/viewer/src/props.rs`'s module docs (the slot/param asymmetry is named there — this issue is the schedule that disclosure was owing).

Slots got the full GQ5 units treatment post-close: a literal remembers its written unit, panels render and author in it, `SetSlotUnit` changes notation without touching the value. Document *parameters* — the fields the parametric workflow exists to route every dimension through — have none of it: a user declaring `base_r` cannot author or read it as `50 mm`; the panel speaks canonical metres exactly where the units layer matters most. Downstream cost observed in the same walk: `ProbeBounds` on a Length param seeds from 1 written unit = 1 canonical metre, so a millimetre-scale part spends ~10 refinement halvings just getting from the seed floor down to its own scale (the driven-slot half of that is issue 1458).

The shape is the same one slots already have: a stored display unit beside `DocParam`, a `SetParamUnit` door mirroring `SetSlotUnit`, and the panel's param rows reading/writing through `in_written`/`written_unit` like the slot rows do. Persistence-wise it is one more field under GQ3's versioning discipline.

(story-suites orchestrator)

## What is left, re-cut 2026-09-04 (CHROME)

**The title and the body above are both stale, and this section is the
live statement of the item.** They are kept rather than rewritten
because two later units cite them.

Two of the three halves this item asked for have landed since it was
filed, neither by this program:

- **Storage — DONE**, elsewhere. `DocParam::Continuous` carries a
  `display_unit: UnitSym` beside `dim`, with `written_length` /
  `written_angle` as total authoring doors and the unit-measures-`dim`
  pairing checked by `persist::check`
  (`crates/editor-core/src/doc.rs`). So there is **no new persisted
  field**, and the GQ3 versioning announcement `work/chrome/plan.md`
  schedules against this item is owed on nothing.
- **The probe's seed — DONE**, by PR 1746. `ProbeBounds` on a Length
  parameter now seeds at one WRITTEN unit. The paragraph above citing
  "~10 refinement halvings" as this item's downstream cost is
  therefore describing a fixed defect.

What remains is the **panel**, plus the door it needs:

1. `ParamRow` (`crates/viewer/src/props.rs:459`) carries no `unit`
   field at all, so a parameter row renders and authors in canonical
   metres. `props`' own module docs (`props.rs:34-38`) state this as
   the residue.
2. **A readout that now contradicts the probe.** `app.rs:4646` renders
   a parameter's probe RESULT in the canonical unit while PR 1746
   searches it in the written one — so the panel says metres about a
   range found in millimetres. This is the sharpest half and it did
   not exist before 1746; a unit taking this item should fix it first.
3. **The drag door was missed by the same class.**
   `GestureTarget::Param` (`session.rs`) carries only `dimension`,
   where `GestureTarget::Slot` carries `unit: Option<UnitDef>` with a
   paragraph explaining why it is captured at `begin_gesture`. A
   parameter drag therefore still has no written unit.
4. **Two tell-free step constants.** `app.rs:2834` (param row) and
   `app.rs:2910` (add-parameter form) both spell
   `if Count { 1.0 } else { 0.0005 }` inline instead of calling
   `drag_tick(dimension)` (`app.rs:1059`), which answers `0.005`,
   `0.01` and `0.1` for Angle, Scalar and Count. So a parameter's drag
   tick disagrees with a slot's in three of four arms. These are
   exactly the bare constants PR 1746's sweep declared it could not
   match, found by the style lane running a differently-shaped one.
5. **A `SetDocParamUnit` edit is needed and does not exist.**
   `SetDocParam` is create-or-replace, so a unit-only change through
   it would silently delete a parameter's `Distribution` — the same
   trap the binding census's `B-DISTRIBUTIONS` charter documents. The
   door belongs in `crates/editor-core/src/edit.rs`, **outside
   CHROME's `paths`**, and needs announcing the way PR 1748's
   `mate.rs` crossing did.

Items 2 through 4 arrived as style-lane findings on PR 1746 (F2, F4,
F5, F15) and are recorded here because a finding with no durable home
cannot warn anyone.

## Home

CHROME (`work/chrome/`), re-homed at the program's opening from
`work/issues/`. The GQ5 units layer and the viewer property panel are
GUI-era ground and GUI is closed.

## Fixed (CHROME, 2026-09-04) — four residues closed, the fifth reported

**Residue 5 needs an `editor-core` door and was not taken.** `DocEdit`
carries exactly two parameter doors and neither is unit-only;
`SetDocParamValue`'s own rustdoc says the unit is deliberately not its
business. The slot trick does not transfer: a slot's whole state is one
`Expr`, so rebuilding a literal loses nothing, while a parameter's unit
sits beside `distribution` on the declaration — so the same rebuild
through create-or-replace must carry the annotation by hand, and no
authoring door can express that pairing. The only spelling that can is
the raw struct literal, which is the `B-DISTRIBUTIONS` trap itself.
What is needed is `DocEdit::SetDocParamUnit` or a
`DocParam::with_display_unit` mirroring `with_value`. Both are outside
this program's `paths`. Reported, not crossed.

**Closed**: the probe readout no longer says metres about a range
searched in millimetres (the residue that did not exist before PR
1746); `ParamRow` carries the authored unit and the row reads, drags
and authors through it; and the two tell-free step constants now share
`field_drag_tick` with the slot field.

**Two of this item's own residues were wrong, and the lane said so.**

- Residue 3 is not a defect. A unit on `GestureTarget::Param` would be
  **dead data**: a slot gesture needs its captured unit because its
  edit REBUILDS the literal, while a parameter's routes through
  `SetDocParamValue`, which writes a number into a standing
  declaration and cannot disturb the unit beside it. The reason is now
  recorded on the variant so the absence does not read as this bug
  again.
- Residue 4's "three of four arms" is **two**. `is_structural()` is
  `dimension == Count`, and a structural slot already dragged at 1.0 —
  so a `Count` parameter agreed already. Routing counts through
  `drag_tick(Count)` would have MINTED a disagreement and let a
  whole-number field land between integers.

Both came from a style-lane finding this item recorded verbatim. A
finding is a claim too.

**Still open beyond residue 5**, found in territory and left as unasked
scope: the add-parameter form could author a millimetre parameter today
through `written_length`/`written_angle` with no kernel change, at the
cost of a unit picker and one design call about `props`' canonical-value
rule. It is the one authoring affordance available before residue 5
lands.
