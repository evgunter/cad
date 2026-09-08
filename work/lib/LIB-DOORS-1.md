---
id: LIB-DOORS-1
kind: unit
title: the edit door projects every arm's payload as attributes
status: review
opened: 2026-09-08
branch: lib/doors-1
refs: [pncad-py-seven-doors-lack-field-projection]
pr: 2227
---

The first door under the rule ruled (A) on
`pncad-py-seven-doors-lack-field-projection`: **every arm's payload is
an attribute, every attribute present on every arm, `None` where the
arm does not carry one.**

`EditError`'s 58 arms flatten into one record of 21 fields, published
beside `variant`/`inner_variant` as 23 attributes on every raise site
of the class — the document layer's refusals, the declare sugar's, and
the three the boundary builds itself. One exhaustive match, no
wildcard: an arm added kernel-side is a compile error.

## Delivered

- `crates/pncad-py/src/edit_payload.rs`: `EditPayload` and
  `edit_payload`, the flattening, exhaustive over all 58 arms.
- `crates/pncad-py/src/tags.rs`: `slot_id_tag` (40 literals — one
  stable word per named slot, the axis spelled into the word) and
  `attr_kind_tag` (3).
- `crates/pncad-py/src/py/doc.rs`: `edit_fields`, and `edit_err` /
  `declare_err` / the three boundary raises through it.
- `pncad.pyi`, `src/py/mod.rs`: the 21 attributes typed
  `Optional[...]`, and the class docstring's flattening rules.
- `src/tests.rs`: `every_edit_arm_projects_the_payload_it_carries`
  (57 of 58 arms constructed, the attribute set asserted per arm) and
  two `TAG_INVENTORY` rows.
- `tests/test_binding_census.py`: `SlotId`, `AttrKind` and `ExprPath`
  into `BOUND_AS`, with the measurement stated.
- `tests/test_document.py`: eight Python rows reaching nine arms
  through real edits, the all-`None` shape, the declare sugar's own
  arm and a boundary raise.
- `tests/ty_fixtures/{legal,illegal}.py`: all 21 attributes typed, and
  two un-narrowed reads rejected.
- **DEVIATION — the match lives outside `py/`.** `readback_err` and
  `split_err` hold theirs inside the Python-gated module; this one is
  a Python-independent record in its own module, because the drift
  alarm belongs on the row hosted CI runs (no interpreter) and because
  the arms with no Python door have to be constructible by a test that
  can run there.
- **DEVIATION — a struct with `..Self::NONE`, not a positional
  tuple.** 21 fields over 58 arms is 1218 positional `none()`s, in
  which a mis-slotted field is invisible. The exhaustiveness that
  matters — the match over `EditError`, and `presence`'s
  `..`-less destructuring — is kept.
- **DEVIATION — the façade moved.** `AttrKind` and `ExprPath` left
  `NOT_CARRIED` (`crates/pncad/src/document.rs`,
  `crates/pncad/tests/all.rs`) under the façade's own payload rule,
  the way `ProgramRefusal` and `NamingError` did at LIB-ARMS.
- **DEVIATION — `slot` is a WORD, not an int.** The brief expected
  slot indices; `SlotId` is a named per-node-type enum, so the
  crossing is a stable tag like every other kernel discriminant.
- **DEVIATION — one arm is not construction-pinned.**
  `meta_unversioned` holds a `MetaVersionError` the façade does not
  carry: `work/lib/meta-unversioned-arm-has-no-inner-word.md`.
- **NOT taken**, per the brief: the other four doors (`persist`,
  `frame`, `stl`, `path`) and the `findings` sequence.
