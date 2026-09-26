---
id: refusal-checker-stays-green-when-a-message-loses-its-recourse
kind: issue
title: test_utils::refusal::problems flags only a second recourse, so a message that loses its repair stays green
status: open
opened: 2026-09-26
---


(ENCL implementer, filed from the fix pass of PR 3269 at its review's
request.)

## What

`test_utils::refusal::problems` (`crates/test-utils/src/refusal.rs`)
is the one statement of the shape a refusal on screen must have, and
it checks the recourse marker only from above:

```rust
let markers = recourse_markers(text);
if markers > 1 { … "states {markers} recourses, not one" … }
```

A message that LOSES its repair (zero markers) is green. The standard
says the recourse is "the part never to drop", yet the one shared
checker cannot see it dropped. Of the three callers, only
`editor-core/tests/refusal_concision_at_rest.rs`
`every_at_rest_finding_renders_to_the_standard` adds a `== 0` check of
its own. The feature-tree and edit-refusal chains
(`refusal_concision_chains.rs` `over_budget`,
`viewer/tests/refusal_concision_edits.rs`) do not, so a rewrite there
that trims the repair away passes the budget rows.

## Why it is not a one-line fix

Some rendered rows legitimately carry no marker today, because their
repair is written as an unlabelled clause (`PatchBoundError::note()`'s
": split the face at the crease", `FitError`'s "— supply the missing
samples"), and some kernel-defect endings omit "There is no way
through" (filed on ENCL's slate as
`work/encl/kernel-defect-endings-and-repair-labels-have-no-shared-home.md`).
Making `problems` flag zero markers needs those spellings settled
first, or an admitted list, and the census of rows it turns red is the
first measurement.

## Repair shape

Flag `markers == 0` in `problems` (the at-rest row's own check, moved
into the shared tool), after the marker vocabulary is settled, so that
every caller holds both directions.
