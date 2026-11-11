---
id: no-node-error-kind-names-the-member-an-assembles-move-refused
kind: issue
title: No NodeErrorKind arm names the member whose move an Assemble refused
status: open
opened: 2026-11-11
priority: P3
cost: E
---


An error-naming follow-up from ONEHOME fork 259 (PR 6820).

## What

A copy moved past the session box refuses at the `Assemble` whose read
moved it, not at the copy's own placement: `NodeErrorKind`
(`crates/editor-core/src/eval/mod.rs:1453`) has no arm that names a
member, so the refusal reads `NodeErrorKind::NotFinished` on the
`Assemble` node. `a_copy_moved_past_the_box_refuses_at_the_box`
(`crates/editor-core/tests/product_finishes_once.rs:289`) pins that
behaviour as it stands.

A user reading the refusal sees which assembly refused but not which
member's move took it past the box.

## Done when

A `NodeErrorKind` arm names the member (its placement node) whose move
refused, the `Assemble` refusal carries it, and the pin above asserts the
member is named.
