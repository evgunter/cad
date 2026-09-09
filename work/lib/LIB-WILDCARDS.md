---
id: LIB-WILDCARDS
kind: unit
title: the CheckEvidence and RefusedRef accessors exhaustive
status: review
opened: 2026-09-08
branch: lib/wildcards
refs: [payload-accessor-wildcards-remain-in-checks-and-assembly]
pr: 2243
---

The last two payload-accessor wildcards in `crates/pncad-py`, under
the ratified rule (A) on `pncad-py-seven-doors-lack-field-projection`:
every arm's payload is an attribute, present on every arm, `None`
where the arm carries none, from one exhaustive match with no
wildcard.

1. `crates/pncad-py/src/py/checks.rs` — `CheckEvidence`'s five
   accessors.
2. `crates/pncad-py/src/py/assembly.rs` — `RefusedRef`'s `width` and
   `kind`, which disagreed with `at` immediately above them.

## Delivered

- **`CheckEvidence` gets a record; `RefusedRef` does not.** The test
  LIB-PROJ stated is a record if it REMOVES matches, in place if a
  record would add a layer, and the two answers here are decided by
  `pick_payload`'s second reason rather than by arm arithmetic:
  **three of `CheckEvidence`'s six arms are unreachable from Python**
  (`escalated`, `unsupported`, `separation_unavailable`), so a
  projection sited in `py/` is one nothing can execute. Sited outside
  it, `src/tests.rs` builds the arms the façade's types allow and
  reads what each puts on the wire. `RefusedRef` has no such arm and
  three accessors over four arms, so it stays in place and matches
  the accessor it sat beside.
- `crates/pncad-py/src/check_payload.rs` (new) —
  `CheckEvidencePayload`, `presence()`, `NONE`, and `check_payload`,
  one exhaustive match over `CheckEvidence`'s six arms. `reason` is a
  `Cow<'a, str>`: borrowed from the separation arm's own sentence,
  owned where the shell arms render one.
- `crates/pncad-py/src/lib.rs` — the module registered.
- `crates/pncad-py/src/py/checks.rs` — the five accessors read fields
  off the record; the class docstring names it and lists the five in
  publication order.
- `crates/pncad-py/src/py/assembly.rs` — `width` and `kind` name all
  four arms; the class docstring states the invariant for all three.
- `crates/pncad-py/src/tests.rs` —
  `every_check_evidence_arm_projects_the_payload_it_carries` (four of
  six arms; the other two hold a payload type the façade does not
  re-export), pinning the field sets and, for the arm Python cannot
  reach, the sentence itself.
- The drift alarm, run: a seventh `CheckEvidence` arm added
  kernel-side fails `cargo check -p pncad-py` with no features at
  `crates/pncad-py/src/check_payload.rs:116` and at
  `crates/pncad-py/src/tags.rs:2061`. Reverted.
- No attribute added, renamed or removed, and no shipped `variant` or
  `kind` value moved, so `crates/pncad-py/pncad.pyi`, the binding
  census and the Python suite are untouched.

Deviation from the brief: **the brief's item 1 offered "five in-place
exhaustive matches" as the cheaper answer and item 3 asked for a
construction pin in `src/tests.rs`; the two cannot both hold.** The
`py/` accessors compile only under the `python` feature and are
private to their `#[pymethods]` block, so an in-place `CheckEvidence`
would have left three arms' projection pinned nowhere. The record is
what item 3 costs, and it is the same trade `pick_payload` made.

Residue, with its own file:

- `check-evidence-shell-refusal-crosses-as-prose-only` — the shell
  door's typed refusal under `escalated`/`unsupported` crosses as
  prose only, and the façade does not re-export its type. Its sibling
  at the third prose-carrying arm
  (`SeparationUnavailable { kind }`) is already filed, on FIX's slate.
