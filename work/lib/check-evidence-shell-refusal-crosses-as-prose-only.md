---
id: check-evidence-shell-refusal-crosses-as-prose-only
kind: issue
title: the shell door's typed refusal under escalated/unsupported crosses to Python as prose only
status: open
opened: 2026-09-08
refs: [mate-fault-arms-carry-payload-that-does-not-cross, pncad-py-seven-doors-lack-field-projection]
---

Disclosed by LIB-WILDCARDS, which made the `CheckEvidence` accessors
exhaustive and so had to name, at every arm, what that arm carries and
what it does not. The ratified rule is (A) on
`pncad-py-seven-doors-lack-field-projection`: every arm's payload is
an attribute, present on every arm, `None` where the arm carries none.
`CheckEvidence`'s five attributes obey it; **one kernel field over two
arms is the remainder**, and it is what the exhaustive match writes
`source` against without projecting.

## The field

`crates/editor-core/src/checks.rs`, `pub enum CheckEvidence`:

- `Escalated { source }` and `Unsupported { source }` — a
  `topo::ShellClassifyError`. It crosses as `reason`, the sentence it
  renders (`crates/pncad-py/src/check_payload.rs`), so a Python
  consumer that wants to know WHICH shell refusal escalated the count
  has the two options the sibling row calls the defect: substring-match
  the sentence, or do without. `escalated` and `unsupported` are two
  different arms of the same shape, so the tag already separates the
  flux-inventory case from the orientation case; what it does not
  separate is the shell door's own arms inside each.

## It cannot be CONSTRUCTED here either

`crates/pncad-py` depends on `pncad` and `quantity` and nothing else,
and the façade does not re-export `ShellClassifyError`, so this crate
cannot name a value to put in either arm. That is why
`src/tests.rs::every_check_evidence_arm_projects_the_payload_it_carries`
pins four of six arms rather than all of them, and why
`check_registry_tags_are_stable` above it covers the same two by the
exhaustive match alone. A crossing therefore needs the curation half
first (the type re-exported and a tag minted for it), on the
`lib-per-arm-error-tags` shape, not just an accessor.

## The sibling half, already filed elsewhere

`CheckEvidence::SeparationUnavailable { kind }` is the same defect at
the third prose-carrying arm, and it has its own file on another
program's slate:
`work/fix/boolean-kind-not-published-at-the-python-door.md` (FIX,
2026-09-04) — a `boolean_error_tag` beside `path_error_tag` plus an
accessor on the evidence object. Nothing here duplicates it; this row
is the shell door's half of the same sentence, and the two should
probably be decided together.

## What it costs

Less than the wildcards did. The projection is total — the match in
`crates/pncad-py/src/check_payload.rs` is exhaustive with no wildcard,
so an arm added kernel-side stops the default build — and every
attribute the arms DO carry crosses. What is missing is a branchable
word for the refusal inside two arms, on a path (`run_checks` over a
subject whose shells will not classify) that no Python test reaches
today.
