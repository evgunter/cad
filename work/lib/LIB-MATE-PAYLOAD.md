---
id: LIB-MATE-PAYLOAD
kind: unit
title: the six MateFault fields that did not cross, in the frame door's vocabulary, with LeverRefusal curated and the arm table at 13/13
status: review
opened: 2026-09-09
branch: lib/mate-payload
refs: [mate-fault-arms-carry-payload-that-does-not-cross]
pr: 2258
---


Closes `mate-fault-arms-carry-payload-that-does-not-cross` under (A)
on `pncad-py-seven-doors-lack-field-projection`: every arm's payload
is an attribute, present on every arm, `None` where the arm carries
none.

## Delivered

- **Fourteen attributes**, 17 → 31 on `MateFault`, in the record's
  order: `expected_document`, `found_document`, `inner_variant`,
  `margin`, `margin_low`, `margin_high`, `zero`, `escalate`, `field`,
  `value`, `lever_tilt`, `lever_arm`, `extent`, `floor`
  (`crates/pncad-py/src/mate_payload.rs`, accessors in
  `src/py/mate.rs`, stub in `pncad.pyi`). `presence()` stays
  exhaustive with no `..`.
- **The frame door's vocabulary, reused not re-spelled.** The
  classifier's words are `py/place.rs::frame_err`'s, and the fork
  itself is ONE helper both doors call —
  `crates/pncad-py/src/escalation.rs`, sited outside `py` so it
  compiles and is testable under every feature.
- **`predicate` is SHARED**, not split. "The predicate that decided"
  and "the predicate that could not decide" are one concept asked of
  two outcomes, and the arm the caller already holds says which. Said
  so in the record's rustdoc and in the stub.
- **`inner_variant` is ONE level in.** `Frame` crosses under
  `frame_error_tag` (so a band-refusing frame reads `band`, exactly
  what `FrameError.variant` answers), `Band` under `band_error_tag`,
  `Unleverable` under `lever_refusal_tag`. A band two levels down
  under a frame is what `field`/`value`/`zero`/`escalate` then say.
  `Indeterminate` carries a struct, so it has no inner word.
- **Quantities.** `margin`/`margin_low`/`margin_high`, `lever_arm`,
  `extent`, `floor` are `Length`, `lever_tilt` is `Angle` — the
  metres and radians the funnel dimensions. `zero`/`escalate`/`value`
  stay plain reals, which is what `frame_err` already answers for
  them: a `Band`'s thresholds are whatever its predicate measures in
  and the same type carries angular ones, so dimensioning them here
  would decide a question the frame door left open.
- **`LeverRefusal` curated**, the unit's one curation decision, by
  the cross-list rule already on `pncad::document`'s payload-rule
  header: its only home is the refusal holding it, so it rides its
  carrier. Re-exported from `editor-core`'s `lib.rs` stanza beside
  `MateFault`, curated at `pncad::document` beside it,
  `lever_refusal_tag` added with its `TAG_INVENTORY` row, and
  `"LeverRefusal": "MateFault.inner_variant"` in the census's
  `BOUND_AS`.
- **The arm table is 13/13.** `every_mate_fault_arm_projects_the_
  payload_it_carries` builds every arm — sixteen values over the
  thirteen, so each `MarginDiag` arm and each `BandError` arm is
  executed — and reads every field of each. The "nine of thirteen"
  docstring is deleted.
- **Python rows** in `tests/test_assembly_author.py`
  (`TestMateFaultPayload`), each reached by authoring the mistake:
  the levered clash, the unlevered one, the mispaired solve, the
  too-small datum, the in-band mate frame. Every attribute is read
  where its arm carries it and read back `None` on an arm that does
  not.

## Deviations and residues

- **Four attributes have no Python row that CARRIES them**, and it is
  a fact about the doors rather than a gap: `margin_low`/`margin_high`
  are the interval scalar's enclosure, which no f64 solve produces,
  and `field`/`value` belong to a band the tolerance witness cannot
  fail to form. The Rust arm table builds all four; the Python rows
  own the other half, that they are present and `None`. Said in the
  test class's own docstring.
- **No new door, no kernel behaviour change.** The `editor-core`
  re-export and the façade curation are the only non-binding edits.
- **The sweep's six remaining hits are all outside this fence**, each
  argued at its site or already on a slate (`meta-unversioned-arm-has-
  no-inner-word` is the one open one). The hit list and its disposition
  are in the PR body.

## Closed

Delivered as above; `mate-fault-arms-carry-payload-that-does-not-cross`
is closed with it.
