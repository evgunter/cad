---
id: load-door-refusals-tag-ids-the-file-spells-in-decimal
kind: issue
title: Load-door refusals name ids by a hex tag the file does not contain: the file spells ids in decimal
status: open
opened: 2026-10-01
priority: P4
cost: M
design: true
---


A person repairing a refused file cannot search it for the id a refusal names. Every load-door refusal says a node as DESIGN.md Band 1, "Node labels" rules: kind, label and **tag**, which is the id's high twelve hex digits (`editor_core::spoken`, `write_tag`), or `node <tag>` for an id the document does not hold. The file spells the same id as a **decimal** `u64`: node-map keys `"4611686018427387904":`, `order` entries, input fields and mint-log entries (`persist/wire.rs`, the derived `Serialize` of `RecipeNodeId`). So the tag in the sentence matches nothing in the file. Today the person locates the defect by the line and column a parse refusal carries (`PersistError::Parse`, `Unreadable`), or not at all for a `SnapshotError`, which carries neither.

The class covers:
- every `SnapshotError` arm (`persist/check.rs`);
- `PersistError::ProfileProgram`;
- `NonFiniteSite::Metadata`'s name;
- the parse-time duplicate-key refusals (`persist/strict.rs`'s `SaidKey` for `RecipeNodeId`, `persist/pairs.rs`'s appearance key via `strict::duplicate_key`).

Before #3741 those two duplicate-key sites printed `RecipeNodeId(<decimal>)` through `Debug`. That contained the file's spelling, though not in the ruled form.

This is a design question, not a defect to patch at a site. Ways it could go:
1. A file's spelling of an id joins the sentence beside the tag (a machine-channel spelling, as `FullId` is).
2. The format spells ids in hex, so the tag is a prefix of what the file holds.
3. `SnapshotError` carries a position into the file.

Each touches DESIGN.md's node-label rule or the wire format, so none is settled here. Found while building `emit/persist-door-refusals-speak-the-node` (#3741).
