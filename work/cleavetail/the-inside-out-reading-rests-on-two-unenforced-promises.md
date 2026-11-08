---
id: the-inside-out-reading-rests-on-two-unenforced-promises
kind: issue
title: The inside-out reading rests on two promises nothing enforces: the census never reads as given, and a whole-operand inside-out read names a solid
status: open
opened: 2026-11-02
---


Split out of `work/onehome/inside-out-operand-reading-has-two-homes-and-four-spellings.md`
by ONEHOME's `onehome/small-bundle` lane, which gave the inside-out recourse one home
(`PointInSolidError::INSIDE_OUT_RECOURSE`) and made `shell_witness::inside_out_operand`
refuse a solid whose faces do not select instead of skipping it. These bullets of that row
are robustness, not duplication, and stay open.

## What

- **The census's `unreachable!` on `PointInSolidError::InsideOut`** (`census.rs`,
  `of_point_in_solid`: "the census reads its instances' own selections, never one read
  as given"). It holds only because no census selection calls
  `SolidFaces::read_as_given`. Nothing in the types says so: `as_given` is a `bool` field
  of `SolidFaces`, and any selection the census builds could set it. A type that a census
  selection cannot be read as given through would make the arm unrepresentable
  (DESIGN `[unrepresentable]`).
- **`shell_witness::inside_out_operand` can return `InsideOutOperand { errors: [] }`.** It
  is called when the whole operand, read as given, holds `q`'s side at infinity; it then
  reads each solid on its own and names the ones that answer `InsideOut`. A solid whose own
  read refuses otherwise (in band, a face kind with no arm) is not named, so a body where
  the whole read decides and no single solid's does refuses as inside-out naming nothing.
  Not reached by any measured fixture.
- **Which story wins is not pinned.** The Boolean reads an operand inside-out at its first
  witness, before more specific refusals: PR 4725's review saw a valid thin tube cut across
  move from `CurvedPierceUnsupported` to a `Containment` refusal. Whether that ordering
  still holds after the gate moved into `complex_side`'s witness ladder is unmeasured;
  nothing pins either story.

## Owed

A type, or a check that runs, for the first; a decision on the second (name the solids,
or refuse as the whole read refused); a pinned fixture for the third.

- 2026-11-03 — moved from CLEAVE to CLEAVETAIL by `git mv` when CLEAVE closed, id and body unchanged (CLEAVE had no dispatchable row above P2 left, and CLEAVETAIL is its own tail on the same ground; Ev, in chat).
