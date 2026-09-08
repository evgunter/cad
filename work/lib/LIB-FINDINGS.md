---
id: LIB-FINDINGS
kind: unit
title: a validation refusal carries its findings as a sequence
status: review
opened: 2026-09-08
branch: lib/findings
refs: [census-findings-cross-without-a-per-arm-tag, two-validation-payload-discriminants-still-uncrossed]
pr: 2225
---

The unit under Ev's ruling (A) on
`census-findings-cross-without-a-per-arm-tag`: **`ValidationError`
keeps `door` and `failure_count` and gains
`findings: list[ValidationFinding]`.**

`ValidationFinding` is a frozen value class carrying `variant` — the
kernel `ValidationError` arm's snake_case word, minted by an
exhaustive map over all seventy-one arms — plus the arm's payload
where one is carried: `subject_kind` and `entity_kind` for the two
census-unsupported arms' `CensusSubject`, and `contact_kind` for
`UndeclaredContact`'s `CensusContact`. Every attribute is present on
every finding, `None` where the arm carries none. The joined message
is byte-identical, `failure_count` keeps its semantics, and the door
still raises ONCE.

**The single exception to "`variant` is a scalar"**, argued at the
door by the door's own shape — it is the one door that reports MANY
refusals at once — so it does not become a second convention. Said in
three places a reader meets it: `ValidationFinding`'s class docstring,
`pncad.pyi`, and the README's error-taxonomy paragraph.

## Delivered

- `crates/pncad-py/src/tags.rs`: four exhaustive maps with no wildcard
  — `validation_error_tag` (71 arms), `census_contact_tag` (8),
  `entity_id_tag` (7), `census_subject_tag` (2). 88 new tag literals.
- `crates/pncad-py/src/validation.rs` (new, Python-independent): the
  `Finding` shape and `project`, which assembles the four words. The
  two payload EXTRACTORS carry the kernel enum's own wildcard licence
  ("a site that extracts is asking a question, not giving an answer");
  the site that CLASSIFIES is `validation_error_tag`, which is
  exhaustive, so a new kernel arm stops this crate compiling there.
- `crates/pncad-py/src/py/value.rs`: the `ValidationFinding` pyclass
  (frozen, no Python constructor, `__eq__`/`__hash__`/`__repr__`) and
  the `findings` attribute on every `run_validator` raise.
- `pncad.pyi`, `crates/pncad-py/README.md`: the stub and the stated
  exception.
- `src/tests.rs`: inventory rows for the four maps, and
  `every_validation_finding_carries_every_word_its_arm_has` — the
  Rust construction pin for the arms Python cannot produce.
- `tests/test_validate.py`: six rows, including the absence pin
  rewritten into its positive form (it is a rewrite, not a delete —
  the two scalar words a caller might reach for are still absent, and
  the row now says where the arms live instead).
- `tests/ty_fixtures/{legal,illegal}.py`: the sequence and the three
  optional words typed, and three off-lattice lines rejected.
- `tests/test_binding_census.py`: `CensusContact` and `CensusSubject`
  moved from `INTERIOR` into `BOUND_AS` together, with the
  measurement and the sequence argument at the block; `EntityId`,
  `ContactFinding`, `RingContact` and `StaleDeclaration` re-argued
  where their entries named the old measurement.
- `crates/pncad/src/prelude.rs`: group 5's closing note said "no
  Python tag moves"; two of the four now do, and the note says which.
  Comment only — no re-export, and no façade name moved.
- **DEVIATION — a fourth word beyond the ruling's letter.**
  `entity_kind` rides beside `subject_kind`. The ruling says
  "`subject_kind` plus the entity kind or the pair", and this is that
  second half: `subject_kind` says entity-or-pair, `entity_kind` says
  which kind of carrier when it is an entity, `None` for a pair whose
  two sides are faces by construction. It is CUR6's own shape at the
  façade (`census_subject_is_matchable` answers
  `("entity", Some(EntityId))` / `("face_pair", None)`).
- **DEVIATION — no arena key crosses, so the subject is words only.**
  The ruling asks for the subject "in the same opaque-name alphabet
  the rest of the surface speaks". There is no such name to speak: a
  raw `topo::Body` has no naming, Python holds an opaque handle, and
  arena keys never cross. So which face or vertex a finding names
  stays in the kernel's own prose on the joined message — where it
  already was — and the crossing is the discriminant, which is what
  every other row on the curated lists projects. Stated at the class
  docstring and in `pncad.pyi`.
- **RESIDUE, with its own file.**
  `two-validation-payload-discriminants-still-uncrossed`:
  `StaleDeclaration` and `RingContact` are `ValidationError` payload
  discriminants that did not move, because the ruling named two types
  and these are not them.
