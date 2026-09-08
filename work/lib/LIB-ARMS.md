---
id: LIB-ARMS
kind: unit
title: the inner arm beside the op word on every Python refusal
status: review
opened: 2026-09-08
branch: lib/arms
refs: [lib-per-arm-error-tags, tag-inventory-prose-counts-are-stale, witness-bifurcation-arm-has-no-inner-word]
---

The first unit under the standing rule ruled (A) on
`lib-per-arm-error-tags`: **every refusal whose carrier already
projects a word gains its inner arm as a second attribute.**

`EvaluationError` gains `inner_kind` and `EditError` gains
`inner_variant` — the kernel refusal's own arm as a snake_case word,
`None` where the refusal has no arms. `kind`/`variant` are untouched:
no shipped value moved. Twenty new tag maps in
`crates/pncad-py/src/tags.rs`, each exhaustive with no wildcard, and
two `Option`-returning dispatchers over the two carriers, also
exhaustive.

## Delivered

Everything below is in this unit's PR; the deviations from the
brief's letter and their homes are the last four rows.

- `crates/pncad-py/src/tags.rs`: `node_inner_kind_tag` and
  `edit_inner_variant_tag` (exhaustive over 68 and 58 carrier arms),
  plus twenty inner maps — profile, replay, structure-refusal,
  extrude, revolve, tube, split-op, blend, boolean, transform, skin,
  loft, band, naming, param-attach, shell, program-refusal,
  param-box, seed. 206 new tag literals.
- `crates/pncad-py/src/py/value.rs`, `src/py/doc.rs`: the attribute on
  every raise site of each class, present on all of them.
- `pncad.pyi`, `src/py/mod.rs`: the stub and the two class docstrings,
  which answer Ev's recorded reservation at the door — the two words
  are two enums' discriminants, projected where each lives.
- `src/tests.rs`: inventory rows for all twenty-two new functions, the
  two pair pins (`inner_arm_tags_are_stable`,
  `edit_inner_variant_tags_are_stable`), and the tag-table reader
  taught `Option<&'static str>`, `Some(..)` and `None`.
- `tests/test_binding_census.py`: eleven rows moved into `BOUND_AS`
  at the two carriers' second words, with the measurement stated.
- `tests/test_document.py`: six Python rows — two inner words per
  carrier through real documents, the `None` case at each, the
  poisoned path, and a pin on why `EditError` reaches one inner word
  and not four.
- `tests/ty_fixtures/{legal,illegal}.py`: both attributes typed, and
  the un-narrowed `str` rejected.
- **DEVIATION — the façade moved.** `NamingError`, `ProgramRefusal`
  and `SeedError` left `NOT_CARRIED` and `ParamBoxError` left the
  `interval` block (`crates/pncad/src/{select,document,analysis}.rs`,
  `crates/pncad/tests/all.rs`). Four `NodeErrorKind`/`EditError` arms
  hold those types and `pncad-py` depends on `pncad` alone, so
  without them the rule stops at four arms for a reason inside LIB's
  own territory. The façade's own payload rule ("the discriminant
  crosses with the refusal") is what carries them.
- **DEVIATION — `BifurcationKind` did NOT move**, so its arm answers
  `None`: `work/lib/witness-bifurcation-arm-has-no-inner-word.md`.
- **DEVIATION — the tag-table prose counts were deleted, not
  rewritten.** `tag-inventory-prose-counts-are-stale` offered two
  ways out and this took (1), with one extra measurement: a name in
  the roster it deleted was already false. Closed there.
- **NOT taken**, per the brief: the payload half
  (`pncad-py-seven-doors-lack-field-projection`) and the `findings`
  sequence (`census-findings-cross-without-a-per-arm-tag`).
