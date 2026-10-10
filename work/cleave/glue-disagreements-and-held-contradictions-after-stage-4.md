---
id: glue-disagreements-and-held-contradictions-after-stage-4
kind: unit
title: after D10 stage 4: type the glue door's disagreements and whatever declared-pair contradiction survives (mints C20-C22, C24, and the 35 held arms)
status: parked
opened: 2026-10-10
priority: P1
cost: M
design: true
parent: topo-mints-indeterminates-outside-the-funnel
blocked_on: [intent-stage4-is-built]
refs: [decided-coincidence-carries-a-synthetic-invalid-margin, contact-gate-readers-drop-the-arm-verdict-or-mint-invalid, declared-pairs-retire]
---

Unit 5 of `topo-mints-indeterminates-outside-the-funnel`'s re-scope
(2026-10-10, main `98a3817a1d`). This unit is the residue of the
design's step 3.

## Why it is parked

Of the row's 24 class-(b) arms, 23 are D10-held. Stage 4 PR F
(`declared-pairs-retire`) and `mates-declare-no-contact` delete the
variants that carry them, so there is nothing yet to type.

## Sites

**The 35 held arms.** They are listed in the parent's Re-scoped section,
and they sit in these files:
- `contact_verify.rs` ×7
- `carrier_eq.rs` `definite()` ×3 and `unsettled` ×2
- `boolean/mod.rs` labels ×14
- `vtxfac.rs:549`
- `rim_wedge.rs` ×5
- `merge_faces.rs:2344`, `:2381`
- `flush.rs:315`

Re-census them once stage 4 is built. Each arm that survives takes one
of two paths:
- **A contradiction that still carries evidence:** carry
  `(predicate, geom_core::Decided)` beside the band its error already
  holds. Do not add `geom_core::Definite`, which would be a third
  spelling beside `Decided` and `geom_brep::recourse::Classified`. End
  it through `RefusedArm::SignCertain(Some(margin))` and D4 ¶1 (iv)'s
  defect ending.
- **A structural fault (no margin read):** give it a typed arm, not an
  `Indeterminate`.

**Plus the non-held sites on the glue door's ground, which stage 4
reshapes:**
- **C20, `carrier_eq.rs:168` (`CoincidenceMeasure::decide`'s
  `Unreadable`).** This is honest poison. It needs a door after the seal:
  `gate_measured`, or a payload that is not an `Indeterminate`.
- **C21, `carrier_eq.rs:836` (`coincident_as_declared`).** The sum
  decided nonzero where every datum decided zero.
- **C22, `boolean/mod.rs:1238` (`unglued_coincidence`).** A corner
  decided Zero that the pair door did not glue.
  - C21 and C22 are two `Decided` readings of different extents that
    disagree. D10's glue door is "one verdict per carrier pair", which
    may settle them.
  - Otherwise each becomes a typed finding carrying both readings
    (design item 7).
- **C24, `sweep/src/blend/battery.rs:128` (`measured`'s NaN →
  `INVALID`, on BAND and CARVE's ground).** A reporting projection that
  needs a public poison reading once `INVALID` is crate-private.

## Overlap

TANG's `decided-coincidence-carries-a-synthetic-invalid-margin` is
C22's payload question together with the held contradictions. Claim it
(by `git mv`) when this unit is dispatched, or close it into this one.
SECTOR's `contact-gate-readers-drop-the-arm-verdict-or-mint-invalid`
holds `contact_verify`'s seven, and it releases them with stage 4.

`design: true`: what a surviving contradiction or glue disagreement
carries is a choice to weigh once stage 4 has said what survives.
