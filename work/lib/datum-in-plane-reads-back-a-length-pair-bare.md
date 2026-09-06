---
id: datum-in-plane-reads-back-a-length-pair-bare
kind: issue
title: Datum.in_plane reads a length pair back as bare floats where the write door takes Length
status: open
opened: 2026-09-03
---


Found by the correctness review of LIB-MECH1 (#1696), while confirming
that the `Datum.in_plane` stub that unit ADDED matches what the compiled
property returns. It does — and what it returns is the finding.

## The asymmetry

`Node.datum_axis_in_plane` writes the axis origin as a length pair
(`crates/pncad-py/pncad.pyi`, `origin: tuple[Length, Length]`), which is
right: it is a position in the sketch plane, in metres.

`Datum.in_plane` reads it back as
`Optional[tuple[tuple[float, float], tuple[float, float]]]`
(`crates/pncad-py/src/py/value.rs`, the `#[pyo3(get)] in_plane` field) —
so the origin half crosses BARE. Measured through the built extension:

```pycon
>>> doc.insert(Node.datum_axis_in_plane(frame, (0.25 * m, 0.5 * m), (0.0, 1.0)))
>>> evaluate(doc).value(axis).datum().in_plane
((0.25, 0.5), (0.0, 1.0))
```

`0.25 * m` in, `0.25` out.

## Why it is a finding rather than a convention

`crates/pncad-py/src/py/place.rs`'s direction rule is that "a bare float
appears only where the Rust side is itself a dimensionless direction or a
matrix entry". The SECOND pair of `in_plane` is exactly that and is
correct bare. The FIRST is a position, and the sibling field on the same
class — `Datum.origin` — carries `(Length, Length, Length)`. So the class
de-dimensions in one field what it dimensions in another, and what its
own write door dimensions.

## What closing it takes, and why LIB-MECH1 did not

Projecting the origin half as `(Length, Length)` is a change to a
published Python type, so it wants its `pncad.pyi` entry, its binding
census row and a stub-test pass like any other surface change — not a
mechanical bundle's business. The alternative is to argue the bare shape
(a frame-local coordinate is not a world length), which is a real
argument and would then be written at the field rather than left implicit.

Until then the stub states the asymmetry at the door and points here.
