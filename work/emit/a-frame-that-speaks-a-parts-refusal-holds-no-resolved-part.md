---
id: a-frame-that-speaks-a-parts-refusal-holds-no-resolved-part
kind: unit
title: No frame holds a resolved part, so a part's carried level and fault keep their tags (line_in_part and PartFault::spoken have no caller)
status: open
opened: 2026-10-02
priority: P3
cost: M
design: true
parent: node-labels-are-document-data
refs: [viewer-panes-speak-the-kernel-refusals-they-draw]
---


Split from `viewer-panes-speak-the-kernel-refusals-they-draw`. That row asked the tree to speak a part's carried level and `PartFault` through `CarriedLevel::line_in_part(part, tol)` and `PartFault::spoken(doc_ref, part, tol)` "where it holds the resolved part at the pinned version". No frame holds one:

- `parts::PartFiles` is a map from document id to file name. It holds no document.
- `session::LandedRun` holds the run's resolver, the landed document, the evaluation and `PartFiles`. It holds no resolved part.
- The evaluation does not keep the parts it resolved. The nested evaluation dies with the resolution (`PartFault::PartRootFailed`'s doc, `editor-core/src/eval/parts.rs`).
- `docio::DirResolver` holds a path and never a cached store (its own doc). Each resolution re-scans.

So today a part's level is said by tag: `tree::carried_lines` calls `CarriedLevel::line_in`. So is the instance row's own `PartFault`: `tree::status_of` calls `NodeError::spoken`, and `NodeErrorKind`'s `Part` arm writes `{fault}` by `Display`. Getting a frame to hold the part is a choice with several viable answers, which is why this row is `design: true`.

**Options.**

- **(a) The landing resolves the parts a failure carries.** `DocSession`'s landing walks every failed row's `carried_chain` and resolves each `CarriedIn::Part(doc_ref)` once through the run's own resolver. It holds the results beside `PartFiles`. The tree speaks a level through `line_in_part` where one is held, and by tag otherwise.
  - Cost: file I/O on the UI thread at landing. It is paid only for failed parts. The directory scan for `PartFiles` is already paid there.
  - Staleness: a file changed since the run fails its pin at resolution, so the tag stands. `assert_pinned` cannot fire on a document the resolver answered.
  - The viewer owns the cache. That is a second place, beside `PartFiles`, where the landing snapshots the store.
- **(b) The kernel hands the resolved part out.** `PartFault::PartRootFailed` / `PartRootPoisoned` (or the evaluation's part memo) keep an `Arc<ProfileDoc>` of the part at its pin, so any frame holding the evaluation holds the part. This is recipe's ground. `PartFault` is `Clone + PartialEq + Eq` and is stored per reference in the memo, so its equality and the memo's footprint move.
- **(c) Tags stand.** Retire `line_in_part` and `PartFault::spoken`. A reader opens the part to see its labels; the carried line's document label (`CarriedLine::document`) already names the file.

**Under (a) or (b)** the instance row's own message needs a kernel door that takes the part. Something like `NodeError::spoken_with_part(doc, part, tol)` would say the `Part` arm's fault through `PartFault::spoken`. Without it, the row still says its own fault by tag while the level under it says labels. `tests/part_root_carried.rs` asserts each carried line equals `error.to_string()`, which is by tag, and calls that "as its part's own tree draws it". The part's own tree draws `error.spoken(part)`, so that row moves with this one.
