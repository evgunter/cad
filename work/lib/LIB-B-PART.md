---
id: LIB-B-PART
kind: unit
title: binding census family B-PART
status: open
opened: 2026-09-04
---


Queued mechanical census family (the B-RESOLVE shape): sweep the
family's bindings against the census contract, construct the
previously unconstructible pins where the surface now allows, and
re-cut the census rows honestly. Families share the census/tags/test
files, so at most two run concurrently, staggered.

## Derived scope

`crates/pncad-py/tests/test_binding_census.py` charters `B-PART`
(DOCM-2, PR 1860): `Node.part` (the projection node over a split's
half or a pattern's instance, `PartSelect::{SplitHalf, Instance}`),
`SlotId.Instance` (the Count-typed structural index slot), and the two
refusal tags `empty_half` / `instance_out_of_range` the exhaustive
`NodeErrorKind` mirror already carries. `PartSelect` is the census's
listed gap; the sweep decides what else the family wants bound or
listed.

## Home

LIB's, filed by DOCM at DOCM-2's review (the Python surface is outside
DOCM's fence). Same class, same shape, unscheduled alongside it:
B-FACE-FRAME (`LIB-B-FACE-FRAME`), B-DISTRIBUTIONS, B-MEASURES,
B-NOTATION.
