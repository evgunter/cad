---
id: LIB-DOORS-2
kind: unit
title: the persist, frame and stl doors project every arm's payload as attributes
status: review
opened: 2026-09-08
branch: lib/doors-2
refs: [pncad-py-seven-doors-lack-field-projection]
---


The second unit under the rule ruled (A) on
`pncad-py-seven-doors-lack-field-projection`: **every arm's payload is
an attribute, every attribute present on every arm, `None` where the
arm does not carry one.** Three doors, three commits, one exhaustive
match each with no wildcard arm — so an arm added kernel-side arrives
as a compile error rather than as a silently unprojected payload.

- `persist_err` (`py/doc.rs`, raised by `py/store.rs` too) — thirteen
  arms, fifteen fields beside `variant`.
- `frame_err` (`py/place.rs`, raised by `py/mate.rs` too) — two arms
  (five variant words), nine fields beside `variant`.
- `stl_err` (`py/mesh.rs`) — three kernel enums plus the boundary's
  own `not_utf8`, unified as one local `StlRefusal` so the projection
  is ONE match; six fields beside `variant`.

No shipped `variant` value moved: the tag maps are untouched except
for three new ones.

## Delivered

Everything below is in this unit's PR; the deviations from the brief's
letter and their homes are the last three rows.

- `crates/pncad-py/src/py/{doc,place,mesh}.rs`: the three projections,
  each a positional tuple from one exhaustive match, on the single
  door function every raise site of the class already went through.
- `crates/pncad-py/src/tags.rs`: `program_fault_tag` (2 arms),
  `snapshot_error_tag` (19 arms, delegating the product-root arm to
  `root_fault_tag`) and `band_field_tag` (2 arms). 23 new tag
  literals, all on `inner_variant` or `field`.
- `pncad.pyi`, `src/py/mod.rs`: the three stubs and the three class
  docstrings.
- `src/tests.rs`: the three tag-inventory rows, and three construction
  pins for the arms no Python door can reach — the persist door's four
  nested arms, the frame door's `band` arm, the STL writers' four arms
  — each with the reason per arm.
- `tests/test_binding_census.py`: six rows moved into `BOUND_AS` with
  the measurement stated (`ProgramFault`, `SnapshotError`,
  `NonFiniteSite`, `SolidNameError`, `BinaryHeaderError`,
  `Indeterminate`), and the two prose bullets they left.
- `tests/{test_document,test_notation,test_placed_union,test_mesh}.py`:
  Python rows reaching five persist arms, three frame arms and three
  STL arms through real calls, each asserting the projected payload,
  plus the all-`None` shape at three doors.
- `tests/ty_fixtures/{legal,illegal}.py`: every payload attribute
  typed at all three classes, and the un-narrowed read rejected.
- `crates/pncad-py/README.md`: the taxonomy paragraph gains the
  payload rule and names the two doors that remain tag-plus-prose
  with their reasons.
- **DEVIATION — the frame door projects the escalation's own shape.**
  `FrameError::Degenerate` carries an `Option<Indeterminate>`, and
  the mechanical rule projects its leaves. `crates/pncad/src/prelude.rs`
  argued MarginDiag's and BandField's NON-carriage on a measurement
  that included "no attribute on any bound exception carries a margin,
  an enclosure bound or a band". That sentence is now false. Both
  prose notes are corrected in this PR; the curation question the
  prelude asks the next pass to re-measure is
  `work/lib/margin-diag-non-curation-was-measured-on-a-count-that-moved.md`.
  No re-export moved: the binding reads both types through
  `pncad::geom_core`, the module hop `prelude.rs` already names as the
  fallback.
- **DEVIATION — the STL door gained a local four-arm enum.** The
  three kernel refusals and the boundary's `not_utf8` share the
  exception class, and "one exhaustive match" over four types needs
  one value to match on. `StlRefusal` is private to `py/mesh.rs` and
  crosses nothing.
- **NOT taken**, per the brief: the `edit` and `declare` doors
  (LIB-DOORS-1), `path` (waits on the kernel `PathError`
  discriminant, SMELL D37/D39) and `step_import` (argued at its site).
  The carrying issue stays OPEN with those four rows and gains a
  `## Progress` line for these three.
