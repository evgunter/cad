---
id: the-viewer-cannot-author-a-mirror
kind: issue
title: The viewer offers no gesture that writes Node::Mirror, so a symmetric part cannot be authored through the UI
status: open
opened: 2026-10-22
priority: P0
cost: M
refs: [a-mirror-is-a-construction]
---


Found by `a-mirror-is-a-construction` (2026-10-22). `Node::Mirror { body, plane }` evaluates, saves and is bound in Python (`Node.mirror`), but no viewer form writes one: the viewer's sites match it only to classify it (`crates/viewer/src/session/refuse.rs` `seat_kind`, `crates/viewer/src/session.rs` `puts_an_instance`, `crates/viewer/src/tree.rs` `node_note`). The gesture is the split's: pick a body and a plane read off its own geometry (`crates/viewer/src/pane/create.rs`'s split form, `Node::Split`). A symmetric part is then `Union` over the body and its mirror.
