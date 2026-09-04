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

## Fixed (CHROME, 2026-09-04) — four residues closed, six threads filed

**Everything this unit found and did not take has its own file.** A
residue disclosed in prose here would read as a record of work done,
not as an open thread, and would die with this directory
(`work/README.md:100-106`). What follows names the files; the findings
themselves are in them.

**Closed here.**

- The probe readout no longer says metres about a range searched in
  millimetres (residue 2, which did not exist before PR 1746). It is
  now closed BY CONSTRUCTION rather than by two reads agreeing:
  `session::BoundsReading` carries the unit the search stepped by
  beside the range it found, and both panel readings — a slot's and a
  parameter's — are `BoundsReading::wording()`. A row that probes and
  then reads pins it (`panel_display.rs`,
  `a_parameters_range_reads_in_the_unit_it_was_searched_in`).
- `ParamRow` carries the authored unit, and the row reads, drags and
  authors through it (residue 1).
- The two tell-free step constants, plus a third the first sweep
  missed (the free-move probe's `.speed(0.5)`), now go through
  `app::FieldWriting` — one value answering the unit a field is
  written in AND the tick it moves at, for a slot field and a
  parameter's alike (residue 4).

**Open, each with a file.**

- Residue 5, the `editor-core` door:
  `work/issues/doc-param-unit-edit-has-no-door.md`. Outside CHROME's
  `paths`, so it is homed in `work/issues/` and needs announcing the
  way PR 1748's `mate.rs` crossing did.
- The add-parameter form's canonical-only authoring, which needs NO
  kernel change: `work/chrome/add-parameter-form-authors-canonical-only.md`.
- A parameter row's value field has no text door where a slot's does —
  no parser, no unit authoring, no no-op guard:
  `work/chrome/parameter-row-field-has-no-text-door.md`.
- The drag tick has three homes and a `Count` field has two live
  answers (the panel's `1.0` against the pattern form's `0.1`):
  `work/chrome/drag-tick-has-three-homes.md`.
- Three doc-comment merge scars found reading `app.rs` end to end, none
  of them this unit's: `work/chrome/app-rs-doc-comment-merge-scars.md`.
- The `#[cfg]`-gated loud-skip marker is now an eight-copy hand-written
  idiom, and this unit added the eighth:
  `work/issues/loud-skip-marker-is-a-hand-kept-idiom.md`.

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
  whole-number field land between integers. What it was blind to is
  the disagreement already standing between the panel and the pattern
  form, which is why that is now a file rather than a correction.

Both came from a style-lane finding this item recorded verbatim. A
finding is a claim too.

**One defect this unit's own first pass carried, found by its style
review.** `probe_scale`'s two arms read two different documents — the
slot arm the shown one, the parameter arm the committed one. They
agreed only because a probe refuses while a gesture is in flight, so no
scratch document can exist at the moment either is read. `probe_scale`
now takes the document the probe SEARCHES and both arms read it, which
makes the agreement structural rather than circumstantial.
