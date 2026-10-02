---
id: a-frame-that-speaks-a-parts-refusal-holds-no-resolved-part
kind: unit
title: No frame holds a resolved part, so a part's carried level and fault keep their tags (line_in_part and PartFault::spoken have no caller)
status: open
opened: 2026-10-02
priority: P3
cost: M
design: true
needs_ev: true
pr: 3839
branch: emit/ev-part-labels-at-the-seam
parent: node-labels-are-document-data
refs: [viewer-panes-speak-the-kernel-refusals-they-draw]
---

Split from `viewer-panes-speak-the-kernel-refusals-they-draw`. That row asked the tree to speak a part's carried level and its `PartFault` through `CarriedLevel::line_in_part(part, tol)` and `PartFault::spoken(doc_ref, part, tol)`, "where it holds the resolved part at the pinned version … else the tag stands". **Today's behaviour is that row's else branch.** No frame holds a resolved part, so the tag stands:

- `parts::PartFiles` maps a document id to a file name. It holds no document.
- `session::LandedRun` holds the run's resolver, the landed document, the evaluation and `PartFiles`. It holds no resolved part.
- The evaluation keeps no part it resolved. `PartCache` (`editor-core/src/eval/parts.rs`) is one per `evaluate` call and is dropped with it, and the nested evaluation dies with the resolution.
- `docio::DirResolver` holds a path and never a cached store (its own doc). Every resolution re-scans.

So `tree::carried_lines` says a part's level by tag through `CarriedLevel::line_in`. `tree::status_of` says the instance row's own `PartFault` by tag too: it calls `NodeError::spoken`, and `NodeErrorKind`'s `Part` arm writes `{fault}` by `Display`. Making a frame able to say the part's labels is a choice with several viable answers, which is why this row is `design: true`.

**Options.**

- **(a) The landing resolves the parts a failure carries.** `DocSession`'s landing walks every failed row's `carried_chain` and resolves each `CarriedIn::Part(doc_ref)` once through the run's own resolver. It holds the results beside `PartFiles`. The tree speaks a level through `line_in_part` where one is held and by tag otherwise.
  - Cost: file I/O on the UI thread at landing, paid only for failed parts. The directory scan for `PartFiles` is paid there already.
  - A file changed since the run fails its pin at resolution, so its tag stands, and `assert_pinned` cannot fire on a document the resolver answered.
  - The viewer owns this cache, a second landing snapshot of the store beside `PartFiles`.
- **(b) The kernel hands the resolved part out on the fault.** `PartFault::PartRootFailed` / `PartRootPoisoned` keep an `Arc<ProfileDoc>` of the part at its pin, so any frame that holds the evaluation holds the part. Nothing else can carry it: the per-evaluation `PartCache` is gone once `evaluate` returns.
  - This is recipe's ground.
  - `PartFault` is `Clone + PartialEq + Eq` and is cached per reference, so equality would compare or skip a whole document, and each fault grows.
- **(d) A `HeldNodes` snapshot taken inside the nested evaluation**, where the part document is in hand. The fault keeps its bare ids plus the part's nodes as that version holds them (`held_by`), and any frame says them back with `Speaker::held`.
  - A part's labels are pinned content: another label is another pin (`editor-core/tests/docm6_seam_declarations.rs`, `a_carried_level_is_never_spoken_from_another_version_of_the_part`). So unlike a host document's labels, this snapshot cannot go stale.
  - No viewer state, no I/O at landing, and the instance row and the tree's level both get it.
  - Tension: `spoken.rs` says a value the evaluation memo reuses keeps its bare ids and is said by the frame that hands it out. That rule exists against staleness, which a pin rules out here, but it would be a stated exception.
  - Tension: `PartFault`'s `Eq` would take the snapshot into account. Two faults with the same ids and different labels are already different pins, so equality would not change in practice, but the derive would have to say so.
- **(c) Tags stand.** Retire `line_in_part` and `PartFault::spoken`. A reader opens the part to see its labels, and the carried line's document label (`CarriedLine::document`) already names the file.

**Under (a), (b) or (d)**, the instance row's own message needs a kernel door that speaks the `Part` arm's fault from the part (or from its snapshot). Without one, the row says its fault by tag while the level under it says labels. `tests/part_root_carried.rs` asserts each carried line equals `error.to_string()` (by tag) and calls that "as its part's own tree draws it". The part's own tree draws `error.spoken(part)`, so that row moves with this one.

**Why P3.** It reaches only an instance whose part fails, and only the part's node labels. The tag line still names the file (`CarriedLine::document`) and the repair ("open the part and repair node …"), and opening the part shows the labels. Nothing is mis-said: the tag is the honest spelling where no document is held. The parent's P2 was for host-document nodes said by tag, which a reader of this document cannot look up.
