---
id: mate-fault-arms-carry-payload-that-does-not-cross
kind: issue
title: Six MateFault arms carry payload no attribute crosses, four of them unconstructible through the facade
status: open
opened: 2026-09-08
refs: [mate-fault-accessors-wildcard-into-silence, pncad-py-seven-doors-lack-field-projection]
---

Disclosed by LIB-PROJ, which made the `MateFault` accessors exhaustive
and so had to name, at every arm, what that arm carries and what it
does not. The ratified rule is (A) on
`pncad-py-seven-doors-lack-field-projection`: every arm's payload is
an attribute, present on every arm, `None` where the arm carries none.
`MateFault` is on the projecting side of the split and its seventeen
attributes obey the rule; **six kernel fields over six arms are the
remainder**, and they are what the exhaustive match writes `_: _`
against today.

## The six

`crates/editor-core/src/mate.rs`, `pub enum MateFault`:

- `PosesOfAnotherDocument { expected, found }` — two `DocumentId`s.
  Both are curated (`pncad::document::DocumentId`, and `Doc.id`
  already crosses one), so this is the one row with no type obstacle
  at all.
- `Frame { error }` — a `FrameError`. **LIB-DOORS-2 territory**: the
  `frame_err` door is one of the four remaining on the seven-doors
  roster, and this field should cross in whatever vocabulary that door
  settles on, not ahead of it.
- `Band { error }` — a `BandError`. `BandError` is prelude-curated;
  `BandField`, which its `InvalidValue` arm carries, is not.
- `Indeterminate { diag }` — a `Box<Indeterminate>`. `Indeterminate` is
  prelude-curated; `MarginDiag`, which its `margin` field carries, is
  not.
- `Contradictory { lever }` — `Option<(f64, f64)>`, radians and metres,
  whose product is the deviation `clash` reports. The kernel's own
  comment says the arm is the solve's scale surrogate and NOT a contact
  feature, so a crossing has to carry that sentence with it.
- `Unleverable { refusal }` — a `LeverRefusal`, which
  `crates/editor-core/src/lib.rs:129` does not re-export at all. It is
  the arm the parent item used as its live test, and its scale numbers
  (`extent`, `floor`) are today readable only in `str(fault)`.

## The second half: four arms cannot be CONSTRUCTED here

`crates/pncad-py` depends on `pncad` and `quantity` and nothing else,
so a payload type the façade does not re-export is a value this crate
cannot name. `Frame`, `Band`, `Indeterminate` and `Unleverable` are
each unbuildable for that reason (`FrameError`, `BandField`,
`MarginDiag`, `LeverRefusal`), which is why
`src/tests.rs::every_mate_fault_arm_projects_the_payload_it_carries`
pins nine of thirteen arms rather than all of them. The projection is
still total — the match is exhaustive with no wildcard — but the
executable arm table is not, and the two guarantees are different.

## What it costs

Less than the wildcards did, and it should be sized honestly. Nothing
here is invisible: every one of the six reaches a caller as prose in
`str(fault)`, and the arm that carries it is named by `variant`. What
a caller cannot do is act on the numbers without parsing that prose —
the `clash` beside an unread `lever`, an `Unleverable`'s two scales,
the two document ids of a mispaired read.

## Shape of a fix

Not one unit. `PosesOfAnotherDocument`'s pair is mechanical and could
ride any mate unit. `Frame`'s waits for LIB-DOORS-2's `frame_err`.
The other four each need a curation decision on the payload type
first, which is the `next-payload-rung-under-the-cur3-cur4-carriages`
question and not this file's.

## Orchestrator note (2026-09-08, LIB)

LIB-DOORS-2 landed the same day (`#2228`), so the `Frame` row's wait
is over: `frame_err`'s vocabulary is settled — `variant` per input,
`inner_variant`/`field`/`value` for the band arm, and the
classifier's `margin`/`margin_low`/`margin_high`/`zero`/`escalate`/
`predicate` — and a mate unit crossing `MateFault::Frame`'s payload
should speak it. `Band`'s and `Indeterminate`'s type questions
(`BandField`, `MarginDiag`) are the re-measure filed as
`margin-diag-non-curation-was-measured-on-a-count-that-moved`;
`Unleverable`'s `LeverRefusal` and `Contradictory`'s `lever` remain
this file's.
