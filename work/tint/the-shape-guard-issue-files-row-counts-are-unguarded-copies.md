---
id: the-shape-guard-issue-files-row-counts-are-unguarded-copies
kind: issue
title: The shape-guard issue files carry hand-written row counts of FILED_NO_RECOURSE that nothing checks
status: open
opened: 2026-11-02
---


(Found by FORMS `an-empty-profile-is-a-document-state`, PR 6393's
review.)

## What

Each issue file that admits rows in `FILED_NO_RECOURSE`
(`crates/editor-core/tests/refusal_concision_chains.rs`, and the
viewer's `crates/viewer/tests/refusal_concision_edits.rs`) says how many
rows it admits: "N feature-tree rows", "N status-line rows". The list
in the test is the fact; the count in the issue file is a hand copy
that nothing reads. Measured on 2026-11-02, against the comment groups
in the editor-core list:

- `work/hone/reach-refusals-short-of-the-shape-guard.md`: says 24, the
  list holds 20.
- `work/wire/wire-refusals-short-of-the-shape-guard.md`: says 31 (dated
  2026-10-18), the list holds 11.
- `work/lathe/carve-refusals-short-of-the-shape-guard.md` and
  `work/profilerest/paths-refusals-short-of-the-shape-guard.md`: said
  18 and 32 against 13 and 29; PR 6393 set them to the measured
  numbers, which are copies again.

## Why it is not guarded cheaply

The groups are `//` comments inside one `&[&str]`, so a guard has to
parse the test's own source to know which entry belongs to which file.

## Repair shape

Either drop the counts from the issue files (they already say each row
is admitted by exact id under the comment naming the file, so the
count adds nothing), or make the list structured — `(file, row)` pairs
— and derive or check each file's count from it.
