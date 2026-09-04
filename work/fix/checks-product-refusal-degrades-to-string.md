---
id: checks-product-refusal-degrades-to-string
kind: issue
title: ChecksError::Product carries product::ProductError as reason: String — the same degradation one door over
status: open
opened: 2026-09-04
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
