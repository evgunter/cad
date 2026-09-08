---
id: margin-diag-non-curation-was-measured-on-a-count-that-moved
kind: issue
title: MarginDiag's and BandField's non-curation was measured on a count the frame door has now moved
status: open
opened: 2026-09-08
---


`crates/pncad/src/prelude.rs` argues, at length and by measurement,
that two `geom_core` types stay OUT of the prelude:

- **`MarginDiag`** (`Indeterminate::margin`'s type). The argument's
  boundary half is a count: *"Every one of the thirteen crosses into
  Python as a single tag plus the kernel's own prose ... and no
  attribute on any bound exception carries a margin, an enclosure
  bound or a band."* The paragraph names its own trigger — *"This
  flips if a door ever projects the escalation's own shape — a caller
  told 'the enclosure straddles' can subdivide where one told 'the
  margin is in band' can only widen ε. Stated so the next curation
  pass re-measures rather than re-deriving."*
- **`BandField`** (`BandError::InvalidValue`'s discriminant), on a
  different measurement: at the prelude boundary the discriminant is
  CONSTANT, because every kernel verb derives its band from
  `Band::linear(tol)` and only `field: Escalate` is reachable from a
  prelude door.

## What moved

LIB-DOORS-2 projected `FrameError`'s payload
(`crates/pncad-py/src/py/place.rs::frame_err`). A degenerate frame
refusal now carries `margin` / `margin_low` / `margin_high`, `zero`,
`escalate` and `predicate`; the band arm carries `field` and `value`.
The binding reads `MarginDiag`'s three arms and `BandField`'s two
through `pncad::geom_core` — the module hop the prelude paragraph
itself names as the fallback — so the CARRIAGE decision does not
follow mechanically either way.

What has changed is the evidence both non-carriages were measured on:

- A bound exception now carries a margin, an enclosure bound and a
  band. `FrameError` is not one of the thirteen prelude-curated
  refusals (it is a `geom_core` refusal the frame constructors
  return), so the count of thirteen is intact; the sentence about
  bound exceptions is not.
- `BandField`'s discriminant is now read at a boundary, not only
  compared against a constant — `crates/pncad-py/src/tags.rs::band_field_tag`
  maps its two arms, and `FrameError::Band` reaches the binding from
  `Frame`'s three constructors.

Both prose notes were updated at the moment they went stale
(`crates/pncad/src/prelude.rs`, and the `Indeterminate` row in
`crates/pncad-py/tests/test_binding_census.py`, which moved from
`INTERIOR` to `BOUND_AS`). This item is the RE-MEASURE the prelude
asks for, which is a curation question and not a binding one: whether
`MarginDiag` and `BandField` should now be prelude-carried, or whether
the module hop remains the right answer for a payload the escalation
contract says not to branch on.

## Not urgent

Nothing is broken and nothing is unreachable: `pncad::geom_core` is a
whole-crate re-export, so every one of these names is already
spellable by a Rust consumer, one module below the prelude.
