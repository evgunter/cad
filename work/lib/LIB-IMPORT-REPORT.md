---
id: LIB-IMPORT-REPORT
kind: unit
title: import_step's success side crosses: StepImport curated, ImportReport with body and enclosure
status: closed
opened: 2026-09-09
branch: lib/import-report
pr: 2249
refs: [a-successful-step-imports-own-report-is-uncurated, pncad-py-import-step-drops-the-gates-enclosure]
closed: 2026-09-09
---



Closes both halves of `import_step`'s success side, ruled (A) and (A)
by Ev on PR 2232: the answer is spellable from the list that carries
the door, and Python's door hands back the gate's own measurement
instead of dropping it.

## Delivered

- **The prelude's `step_import` group gains seven names**
  (`crates/pncad/src/prelude.rs`): `StepImport` and the record
  vocabulary its `Solid` arm names —
  `StructureNormalization`, `NormalizationKind`, `CurvePromotion`,
  `PromotedCurveKind`, `PlacedInstance`, `FaceCensus`. The prose sits
  beside the refusal side's, and says which clause each half closes:
  the refusal was a MATCHABILITY gap, the answer a REACH one.
  `PromotedKind` was already carried and is now reached from two
  carriers, which the entry states rather than duplicating.
- **The `Wireframe` arm needs no new name.** Its `curves` are
  `Curve3`, already on the list at group 2's `topo` re-export, and its
  promotions are the same `CurvePromotion`. Recorded so the next
  reader does not re-derive it.
- **A curation guard** (`crates/pncad/tests/all.rs`,
  `the_import_answer_and_its_record_are_spellable_through_the_prelude`)
  binds every field of the `Solid` arm by name from the prelude alone
  and hands each to the monomorphic sink, so a type that stops being
  spellable stops compiling. It runs on a REAL import — the round-trip
  oracle's own exported text — and asserts the enclosure against a
  second `mass_properties` call bit for bit.
- **`NormalizationKind` is matched exhaustively in that guard**, so a
  sixth normalization minted kernel-side fails to compile rather than
  arriving under one of the five words.
- **Python's `import_step` answers `ImportReport`**
  (`crates/pncad-py/src/py/value.rs`), frozen, with `body`,
  `enclosure`, `eps_in`, `normalizations`, `promotions`, `instances`.
  `Body` gains nothing and `mass_properties` keeps its one meaning:
  the ruled (A) shape.
- **Four frozen row classes** beside it — `StructureNormalization`,
  `CurvePromotion`, `PlacedInstance`, `FaceCensus` — each projecting
  every field of its kernel record. The two discriminants cross as
  words from exhaustive tag maps (`normalization_kind_tag`,
  `promoted_curve_kind_tag`, `crates/pncad-py/src/tags.rs`), with the
  one arm that carries a payload projecting it BESIDE the word at
  `promoted_to` / `residual`, `None` on the other four.
- **DECISION: `surface_promotion` does not fold its kind into the
  word.** Five words rather than six, with `promoted_to` carrying
  `PromotedKind`'s own two. A caller reading "a patch was promoted"
  reads one word whichever kind it was; which kind is the second
  question, as it is on the refusal side.
- **DECISION: `PlacedInstance.placement` crosses as `Optional[Frame]`**
  — the placement value this surface already has, rather than a new
  matrix spelling. `None` stays `None` (the file stated nothing, or
  stated the identity at ε_in); materializing an identity `Frame`
  would read as a map the file chose. `Mat3`'s `NOT_BOUND` row is
  untouched.
- **DECISION: `FaceCensus` is a top-level class, not an attribute
  triple.** A normalization carries TWO of them and their difference
  is the whole record; three-plus-three flattened attributes would
  make the reader do the pairing. It gains `__eq__` and a consistent
  `__hash__`.
- **DEVIATION: `MassProperties` gains `Clone, Copy` and
  `skip_from_py_object`.** The report hands one out by value, which
  needs `Clone`; `skip_from_py_object` keeps the Python surface
  exactly as it was (a `Clone` pyclass would otherwise start accepting
  extraction by value).
- **Census rows** (`crates/pncad-py/tests/test_binding_census.py`):
  `StepImport` → `ImportReport` with the double-quadrature
  measurement, `NormalizationKind` → `StructureNormalization.kind`,
  `PromotedCurveKind` → `CurvePromotion.kind`. The four row classes
  are spelled identically and are accounted for by the census's rule
  1, so they take NO `BOUND_AS` entry — a mapping there is stale by
  the roster-decay check, which is how that was found.
- **`ImportOptions`' non-crossing argument stands** (`prelude.rs`, and
  the census's `ImportContact` entry). This unit changed the answer,
  not the argument; nothing in it makes that entry false.
- **Stub, docstrings, ty fixtures** (legal: every attribute of the
  report read at its type, with the optional ones narrowed; illegal:
  the report used as a `Body`, `mass_properties` called on it, a field
  assigned, and the two optional rows read unnarrowed).
- **Callers moved onto `.body` / `.enclosure`**: `test_document.py`,
  `examples/bracket.py`, and the Python journey in `docs/GUIDE.md` —
  each reading `.enclosure` rather than re-measuring, because that is
  what a user would now write.
- **The payload-rung sweep stays green** with seven more curated
  names: `--check` passes, narrowed 10 and cross-list 4 unchanged. The
  new carriers surface no rung because the vocabulary crossed whole.
