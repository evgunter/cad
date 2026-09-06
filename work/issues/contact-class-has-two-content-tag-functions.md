---
id: contact-class-has-two-content-tag-functions
kind: issue
title: ContactClass is tagged into content keys by two functions: contact_class_tag (u8, wildcard arm) and ContactClass::content_tag (u64, exhaustive)
status: open
opened: 2026-09-06
---


Found by MSOLVE-4's style review (PR 1960), outside the unit's fence;
filed by the MSOLVE orchestrator. The content key is `eval/mod.rs`'s,
DOCM's ground; the enum is `topo`'s.

`crates/editor-core/src/eval/mod.rs`: the `Node::Mate` arm of
`content_key` writes `contact_class_tag(*class)` — a `u8` with a
wildcard `_ => 0` arm whose own doc admits the collision hazard — while
the `Node::Declare` arm fifteen lines away writes
`class.content_tag()` (`crates/topo/src/contact.rs`, `u64`,
exhaustive), and the interface-crossing feed is a third site. Same
numbers, two homes, one carrying the `_` arm the other exists to
forbid. One tag function, exhaustive, and a sweep for any other
`ContactClass` tagging.
