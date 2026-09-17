---
id: f6-display-predicate-is-spelled-three-times-with-no-home
kind: issue
title: F6's Display predicate is now spelled three times in three crates with no shared home
status: open
opened: 2026-09-06
refs: [2053]
---


Found by the style review of PR 2053 (`view/refusal-all`), which added
the third spelling.

## The three copies

    crates/editor-core/tests/display_contract.rs:42
        !shown.contains('{') && !shown.contains("node:") && !shown.contains("name:")
    crates/viewer/tests/panel_edits.rs:538
        !rendered.contains('{') && !rendered.contains("node:") && !rendered.contains("name:")
    crates/editor-core/tests/m4_pr4_hit.rs:358
        !shown.contains('{') && !shown.contains("node:")

The first is `assert_f6`, the ratified `Display` contract's own helper.
The third is a partial copy inside editor-core itself. The second is
new, and its doc comment announces the duplication in prose — *"The
shape asserted below is F6's — editor-core's ratified `Display`
contract (`crates/editor-core/tests/display_contract.rs`)"* — which is
the comment that exists to reconcile two spellings of one rule.

## What drifts

`assert_f6` takes a `dumps` roster as well as the punctuation
predicate, and asserts each named variant identifier absent. The viewer
copy approximates that half with `!rendered.contains(arm)` — one
identifier per arm rather than the enum's roster — so a rendering that
leaks a SIBLING arm's identifier passes there and fails under
`assert_f6`. A fourth clause added to F6 reaches one of the three
copies.

`assert_f6` lives in a `tests/` file, so it is not importable across
crates as it stands; giving the predicate a home means a shared
test-support item (`crates/test-utils`, which viewer already depends
on) rather than a further copy. That is the decision, and it is not
this finding's to make.

## The home now exists (2026-09-15, S-TINT's TINT-1)

`crates/test-utils/src/f6.rs` — `test_utils::f6::assert_f6(err, wants,
dumps, fields)`, plus `variant_identifier`, which reads a value's
variant name off its own derived `Debug`. TINT-1 needed the predicate
for its own work and `crates/test-utils` was inside its fence, so it
made the decision this row says is not the finding's to make, in the
place this row names.

**Two of the three copies are gone.** `display_contract.rs`'s
`assert_f6` is now a three-line wrapper that supplies the binary's field
roster and delegates; `m4_pr4_hit.rs`'s partial copy is deleted and that
row calls the same wrapper. **The divergence this row predicted had
already happened**: `m4_pr4_hit.rs` banned `"node:"` while
`display_contract.rs` banned `"node:"` and `"name:"`, so one of the two
spellings of one rule was a clause short. It is now one spelling.

**What remains, and it is this row's:** `crates/viewer/tests/panel_edits.rs`
still carries the third spelling, and still approximates `dumps` as one
identifier per arm rather than the enum's roster — the half this row
already describes, and the half that actually differs in what it
catches. `viewer` already dev-depends on `test-utils`, so the
conversion is a call-site change. Two more copies of the same predicate
live outside this row's scope, inlined in
`crates/topo/tests/display_contract.rs` and `crates/mesh/tests/errors.rs`,
each with its own field roster; they are carried on
`work/tint/sibling-display-contract-suites-hand-mirror-their-enums-too`.

**One thing the new home did NOT do**: derive `fields` from the
payload's `Debug`. It was tried and false-positives on a door whose
prose PREFIX is a field name (`MeshPickError::PositionOutOfRange`
renders "pick index: triangle 5 …" and has an `index` field), so the
roster is still the caller's. The reasoning is at the module doc rather
than only here.
