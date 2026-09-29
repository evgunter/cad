---
id: create-pane-words-mate-admission-through-a-binding-catch-all
kind: issue
title: pane/create.rs words a mate class's admission through a binding catch-all over ClassAdmission
status: open
opened: 2026-09-24
priority: P3
cost: E
---


## Finding

`crates/viewer/src/pane/create.rs`, the mate form's admission line
(`ViewerBehavior`'s mate-tool body, ~`:225`):

```rust
match entry.admission {
    pncad::document::ClassAdmission::Mints => "mints an at-rest record",
    other => other.no_record_reason(),
}
```

`other` is a binding catch-all over `ClassAdmission` (`Mints`,
`NoAtRestRecord { why }`, `NotAdmitted`), an `editor-core` enum. It is
the shape `crates/viewer/README.md`'s *A policy over an enum names
every variant* excludes: a fourth admission would be worded through
`no_record_reason` with nobody deciding that it should be.

`tree::node_note` made the same decision over the same enum and is
exhaustive since `chrome/subset-policy`
(`caveat @ (NoAtRestRecord { .. } | NotAdmitted)`), so the fix is that
shape, one arm.

## Why it is filed rather than fixed

Found by the `chrome/subset-policy` sweep's second pass (bare binding
catch-alls, `^\s*[a-z_]+ =>`). `pane/create.rs` was fenced out of that
lane because `chrome/create-messages` was working in it.
