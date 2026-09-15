---
id: sibling-display-contract-suites-hand-mirror-their-enums-too
kind: issue
title: mesh and topo display-contract suites hand-mirror their enums too, and mesh's list is already one variant short
status: open
opened: 2026-09-15
---


Found by TINT-1's sweep
(`work/tint/assert-f6-dump-lists-are-hand-written-mirrors-of-error-enums`),
whose fence was `crates/editor-core/tests/**` — these three lists sit
outside it, so they were disposed of rather than fixed.

## The finding

TINT-1's row said `grep -rn assert_f6` finds the helper and its callers
only in `crates/editor-core/tests/display_contract.rs`. That is wrong as
a repo-wide claim: **`crates/topo/tests/display_contract.rs` declares
its own `assert_f6`**, a near-verbatim copy with the same
`dumps: &[&str]` hand-written ban list. A sweep for the ban-list SHAPE
(`for dump in`, `dumps = [`, `dumps: &[&str]`) rather than the helper
name finds it, and finds a third inlined copy in `crates/mesh/tests/errors.rs`.

**One of them has already drifted**, which is the same live failure
TINT-1 found three of in editor-core:

- `crates/mesh/tests/errors.rs`, `let dumps = [...]` inside the
  tessellate-error Display row, holds **14** identifiers.
  `mesh::TessellateError` (`crates/mesh/src/types.rs`, `pub enum
  TessellateError`) has **15** variants: `Band` is missing. The
  suite's own case list constructs a `TessellateError::Band { .. }`
  case, so a rendering of that arm IS exercised with its identifier
  unbanned.

  **What that actually leaves uncaught, corrected.** As filed this row
  said *"the one identifier that would catch it going back to `Debug`
  is the one the ban list does not hold"*. That is false, and TINT-5's
  fix pass corrected it. The same inline check carried
  `!shown.contains('{')` **and** `assert_ne!(shown, format!("{err:?}"))`,
  and `TessellateError::Band` is a STRUCT variant — so an arm that went
  back to its full `Debug` dump reddened on the brace whatever the
  roster held. What the missing entry left uncaught is a **brace-free
  leak of the word `Band`** (`write!(f, "Band: …")`), which is real and
  strictly smaller. The red-on-arrival evidence in TINT-5's PR stands:
  the weld does red on a live defect and names `["Band"]`; only the
  claim about what would otherwise have passed was wrong.
- `crates/topo/tests/display_contract.rs`:
  `["Contradicted", "Escalated", "Undeclared", "NotCertifiable"]` against
  `topo::ContactRefusal` (4 variants) and
  `["Dangling", "NoCanonicalFrame", "NoCarrier"]` against
  `topo::readback::ReadbackError` (3 variants) are both complete **as of
  2026-09-15**. Nothing welds either to its enum, so both are the
  forecast TINT-1's row wrote, not the instance.
  (`together_edge_disagreement_display_is_true_at_all_three_meters`'s
  one-entry `&["TogetherEdgeDisagreement"]` is a deliberate single-variant
  row, not an enum mirror — leave it.)

## The fix

The shape TINT-1 landed, applied per suite: `test_utils::f6::assert_f6`
instead of a local copy of the predicate, a wildcard-free `match` used
as an exhaustiveness TOKEN (returning `()`, naming no identifiers), the
identifier roster written out once, and a set difference that welds the
roster to the cases — with the covered identifiers read off each value's
own `Debug` (`test_utils::f6::variant_identifier`) rather than typed
beside a pattern, because rustc checks the pattern and never the string.
See `crates/editor-core/tests/display_contract.rs`'s
`assert_f6_every_variant` and its per-enum `*_is_exhaustive` /
`*_VARIANTS` pairs. Adding `TessellateError::Band` to mesh's list is the
smaller half; the weld is what stops the next one.

`topo::ContactRefusal` and `topo::readback::ReadbackError` are ordinary
enums, so the compiler can be the census there with no exception of the
kind `SelectRefusal` forced (`work/wire/select-refusal-coverage-is-not-compiler-enforced-from-the-test-crate`).
Check `mesh::TessellateError` for `#[non_exhaustive]` before assuming the
same.

## Territory

Both paths are S-TINT's by `paths: [crates/*/tests/*]`;
`scripts/work.py territory` reports `crates/mesh/tests/errors.rs` also
claimed by `mesh` and `tcost`, and `crates/topo/tests/display_contract.rs`
also claimed by `tcost` — double claims on the documented `*/tests/*`
seam, not crossings.

## Sweep blind spot inherited

The shape sweep (`for dump in`, `dumps = [`, `dumps: &[&str]`, `&dumps`)
cannot match a ban list that spells the loop differently — an
`assert!(!shown.contains("Variant"))` written out once per identifier,
a list named something other than `dumps`, or a ban list held in a
`const` and passed positionally. `crates/viewer/tests/error_display.rs`'s
lone `assert!(!duplicate.contains("PatchId"))` is that shape at N=1 and
is not an enum mirror.

**And TINT-1 added two instances of that blind spot itself**:
`display_contract.rs`'s `*_VARIANTS` consts and `m4_pr4_hit.rs`'s
`HIT_TEST_ERROR_VARIANTS` are exactly "a ban list held in a `const` and
passed positionally", so a `dumps = [` / `for dump in` sweep will not
find them. They are welded to their cases by a set difference, which is
why they are not the defect this row is about — but a sweep for the
CLASS has to look for the const shape as well as the inline one, and
this is the note that says so.
