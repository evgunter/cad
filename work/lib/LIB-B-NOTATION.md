---
id: LIB-B-NOTATION
kind: unit
title: binding census family B-NOTATION
status: open
opened: 2026-09-06
---

Queued mechanical census family (the B-RESOLVE shape): sweep the
family's bindings against the census contract, construct the
previously unconstructible pins where the surface now allows, and
re-cut the census rows honestly. Families share the census/tags/test
files, so at most two run concurrently, staggered.

## Derived scope

`crates/pncad-py/tests/test_binding_census.py` charters `B-NOTATION`
in `FAMILIES`: bind `quantity::WrittenLength` / `WrittenAngle` onto
the `DocParam` and expression constructors
(`crates/editor-core/src/doc.rs`, `DocParam::written_length` /
`written_angle`), so a Python caller who writes `25 * mm` gets a
parameter that REMEMBERS the millimetres. Today the unit erases at
the `Length` door and the document records the canonical row. The
two `NOT_BOUND` entries citing the family are `WrittenAngle` and
`WrittenLength`; the sweep decides what else the family wants bound
or listed.

## Home

LIB's. Filed 2026-09-06 at the program's reactivation because the
census charters the family and `work/README.md` wants every charter
to have a row; `LIB-B-FACE-FRAME` and `LIB-B-PART` named it as
"unscheduled alongside" and this is the file that sentence owed.
