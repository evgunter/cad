---
id: LIB-CUR8
kind: unit
title: three curation decisions: MarginDiag/BandField re-measured, ShellClassifyError, the cross-list rule
status: review
branch: lib/cur8
opened: 2026-09-09
refs: [margin-diag-non-curation-was-measured-on-a-count-that-moved, check-evidence-shell-refusal-crosses-as-prose-only, cross-list-payload-rungs-under-document-only-carriers]
pr: 2253
---

Three curation decisions under the standing rules, each written
where the decision lives.

## Delivered

1. **`MarginDiag` CARRIED on the prelude** beside `Indeterminate`,
   `Band` and `BandError` (`crates/pncad/src/prelude.rs`). The
   argument's own trigger has fired: a door projects the escalation's
   SHAPE (`crates/pncad-py/src/py/place.rs::frame_err` forks on the
   three arms), so the discriminant is read at a boundary and it
   varies. The carriage's falsifier is written beside it: no door
   projecting the shape anywhere, which makes the type telemetry with
   no consumer. `crates/pncad/tests/all.rs`'s
   `escalation_is_readable` now matches the three arms exhaustively
   by bare prelude name and BUILDS the struct it reads, where it
   previously bound `margin` and could name neither the type nor a
   value. Census: `MarginDiag` is `different-shape` with the
   measurement — all three arms reach a Python caller and the shape
   they arrive in is which attribute is set, not a word.

2. **`BandField` NOT carried**, argued anew on the new count in
   `crates/pncad/src/prelude.rs`. The discriminant IS read at a
   boundary now (`crates/pncad-py/src/tags.rs::band_field_tag`
   crosses the field's word on the frame refusal) and it is still not
   VARIABLE: every producer reaching that crossing is `Band::linear`,
   whose `zero` check cannot fire under `Tol`'s invariant, so the two
   arms are an exhaustive match's drift alarm rather than two
   reachable facts. Falsifier restated: a kernel caller of
   `Band::angular_at`, or a door taking a band's thresholds from its
   caller, makes `field: Zero` reachable.

3. **`ShellClassifyError` CARRIED at `pncad::document`** beside
   `CheckEvidence` (`crates/pncad/src/document.rs`), under the payload
   rule that list states — it is what `Escalated` and `Unsupported`
   hold, and a consumer could match either arm and not name what it
   caught. Projected as an inner word:
   `crates/pncad-py/src/tags.rs::shell_classify_error_tag` (four
   literals, exhaustive), `inner_variant` on `CheckEvidencePayload`
   and on the Python `CheckEvidence`, the stub, an inventory row, and
   the construction pin now covers **6 of 6** arms rather than four.
   Census: `ShellClassifyError` is `CheckEvidence.inner_variant`.

4. **The cross-list rule, decided for the general case.** The
   `NamingError` precedent generalises: a payload whose vocabulary one
   of the four curated lists owns lives on THAT list, spelled once,
   and the carrier's list points at it; a payload whose only home is
   the refusal holding it rides its carrier, which is
   `crate::document`'s `VerbKind` rule. Written beside the precedent
   (`crates/pncad/src/select.rs`) and, as the general sentence, at the
   payload-rule paragraph of `crates/pncad/src/document.rs`'s module
   header. `EntityKind` and `SplitHalf` stay on `select`+`prelude`;
   `CROSS_LIST_DISPOSITIONS` moves both rows from `filed` to `argued`
   with that home.

The sweep is the re-sweep: `payload-rung-sweep.py` reports curated
471, declared 821, raw 111, narrowed 9, cross-list 94 raw / 4
narrowed, and `--check` is green.

## Closed

- `margin-diag-non-curation-was-measured-on-a-count-that-moved` —
  re-measured; `MarginDiag` carried, `BandField` argued anew.
- `check-evidence-shell-refusal-crosses-as-prose-only` — the shell
  refusal has a branchable word at the Python door.
- `cross-list-payload-rungs-under-document-only-carriers` — reading 1
  holds and the general question is settled in the tree.

`CheckEvidence::SeparationUnavailable { kind }` is FIX's
(`work/fix/boolean-kind-not-published-at-the-python-door.md`) and is
untouched here.
