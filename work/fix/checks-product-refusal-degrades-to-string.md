---
id: checks-product-refusal-degrades-to-string
kind: issue
title: ChecksError::Product carries product::ProductError as reason: String — the same degradation one door over
status: review
opened: 2026-09-04
branch: fix/product-error-kind
---


Found by the `boolean-error-has-no-fieldless-kind` lane's sweep, in
the same file that item fixed.

**The defect.** `crates/editor-core/src/checks.rs:476` declares
`ChecksError::Product { reason: String }`, and `checks.rs:727` builds
it as `reason: source.to_string()` from `product::ProductError` — the
typed refusal degraded to prose at the checks door, the exact shape
`CheckEvidence::SeparationUnavailable` carried until
`BooleanErrorKind` landed beside it. The field's own doc points at
that variant for the reasoning, so the two moved together and now do
not.

**Why not taken in that unit.** The fix is a `ProductErrorKind` +
exhaustive `kind()` in `crates/editor-core/src/product.rs`, which is
neither the boolean fence that unit was cut on nor a path this
program's territory names; and one item is one PR. The `checks.rs`
half is one field and one call site once the kind exists.

Note the asymmetry the door now has: the SeparationUnavailable arm
carries its class and the Product arm does not, so a consumer that
learned to match on one still substring-matches the other.

## What landed

`editor_core::ProductErrorKind` — ten fieldless variants, one per
`ProductError` arm — plus `ProductError::kind()`, an exhaustive
projection, both in `crates/editor-core/src/product.rs` and re-exported
from `editor-core`'s root and `pncad::document`.

`Subject::Unavailable` carries `kind: Option<ProductErrorKind>` beside
its `reason`, and `ChecksError::Product` carries the same pair.
Neither is written at a raise site: `Subject::refused(&ProductError)`
pairs the class and the sentence off ONE error, and
`ChecksError::product_unavailable` forwards that one pair to the door.
`None` is the honest answer for the one absence that is not a refusal —
a run no enabled resident asked a subject for.

Two guard rows. `product.rs`'s
`each_kind_has_an_arm_and_each_built_arm_projects_to_its_own_kind`
carries an exhaustive match over `ProductErrorKind` — a phantom variant
fails to COMPILE, by name, at `error[E0004]`, in the crate that owns
both enums — plus all ten arms built, projected, and compared against
the variant name `Debug` prints for the error itself. `checks.rs`'s
`the_subject_door_carries_the_class_of_the_gather_refusal_it_saw` pins
both constructors, and
`a_subject_no_resident_asked_for_carries_no_refusal_class` pins the
`None`.
