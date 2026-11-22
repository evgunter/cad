---
id: a-spheroid-writes-exact-step-as-a-surface-of-revolution
kind: unit
title: A spheroid face writes exact STEP: SURFACE_OF_REVOLUTION over an ELLIPSE
status: parked
opened: 2026-11-22
priority: P1
cost: E
parent: an-ovoid-cannot-be-authored
blocked_on: [the-revolve-mints-a-spheroid-from-an-ellipse-arc-centred-on-its-axis]
---



Unit 8 of `an-ovoid-cannot-be-authored` (Ev, PR 7289). The designers:
"STEP `SURFACE_OF_REVOLUTION` over an `ELLIPSE`, exact (entity
recalled, not checked)".

## What is true when it is done

- `step-export`'s surface printer (`crates/step-export/src/writer.rs`)
  writes a spheroid face as `SURFACE_OF_REVOLUTION(name, swept_curve,
  axis_position)`, the swept curve an `ELLIPSE` in a meridian plane
  whose one semi-axis lies along the `AXIS1_PLACEMENT`, the identity of
  the locus. The writer's surface table in `crates/step-export/src/lib.rs`
  gains the row, exact.
- Every body unit 5 builds writes, and the written file parses.

## Check first

No `SURFACE_OF_REVOLUTION` printer exists (`git grep SURFACE_OF_REVOLUTION
crates/step-export` is empty on 2026-11-22), so this is the first; read
ISO 10303-42's entity before writing it, not the designers' recall.
Also no importer reads one: `step-import` refuses it as an unsupported
entity (`StepImportError::UnsupportedEntity`), so the kernel's own
spheroid does not read back. That is EXCH's
`a-surface-of-revolution-is-refused-at-import`, not this unit's.

## Rows and ground

- Changes no other row.
- Crosses EXPORT (`writer.rs`). Announce on EXPORT's log.
- Runs beside units 6, 7 and 9.
- Review: **single (STYLE)**: one printer, its entity checked against
  the standard. Difficulty **S**.
