---
id: a-made-body-does-not-show-the-profile-it-was-made-from
kind: issue
title: viewer: a revolve's or extrude's Properties do not show or link the profile it was made from
status: closed
opened: 2026-11-22
priority: P1
cost: E
rides_with: a-placement-and-its-part-do-not-link-to-each-other
pr: 7356
branch: offer/a-node-shows-what-it-reads-and-what-reads-it
closed: 2026-11-23
---

**Ev reported this** (in chat, 2026-11-22): "you should be able to go from a revolve to the profile it was made from".

The Properties view of a node (`crates/viewer/src/pane/properties.rs:498-527`) is its heading, its label field and its slot rows. `props::slot_rows` (`crates/viewer/src/props.rs:898-944`) lists only `Node::slots()` expression slots, so a revolve's or extrude's profile operand is never shown. There is no "profile:" line and no link. The data is at hand: tests assert `doc.upstream(made) == vec![profile]` (`crates/viewer/src/app.rs:6533`, `:6627`).

The fix is an operand row ("made from: <profile>", a `widgets::message_link` selecting the node) on every node whose operands are nodes: revolve, extrude, boolean operands, mirror, pattern, fillet target and so on. This is one instance of `a-node-name-the-viewer-shows-is-not-a-link-to-it`.
