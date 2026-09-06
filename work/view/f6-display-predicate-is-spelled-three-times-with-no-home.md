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
    crates/viewer/tests/panel_edits.rs:523
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
