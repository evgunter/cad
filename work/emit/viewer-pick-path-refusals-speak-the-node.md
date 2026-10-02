---
id: viewer-pick-path-refusals-speak-the-node
kind: unit
title: The pick path's refusals (PickIndexError, EdgeNameFault, PickError, BlendEvent, IdAnswer) say their nodes through the frame's speaker
status: review
pr: 3821
opened: 2026-10-02
priority: P2
cost: M
parent: node-labels-are-document-data
refs: [viewer-refusals-speak-the-node]
---


Split from `viewer-refusals-speak-the-node`. The pick index, its edge names and the id pass are built from an evaluation alone, in a vocabulary with no document (`pickindex.rs`, `pickcache.rs`, the index worker), so their refusals keep bare ids and gain `Say` (`Display` is the sentence said by `Speaker::TAG`); the frame that draws one speaks it from the document of the evaluation the index was built against (the landed run's).

The types and their reads, at this row's filing:

- `PickIndexError` (`pickindex.rs`, its `Display`: "root {} could not be indexed", "body {} of node {} is drawn by two parts", and the `Node`/`Names` arms forwarding `NodePickError`/`NameLookupError`, both `Say`). `IdMapError::Duplicate` ("patch {} of body {} on node {}") rides inside it.
- `EdgeNameFault` and `EdgeNamesRefused` (`pickindex.rs`; `NotDrawn`, `OutOfRange`, `Unnamed` forwarding `UnnamedEntity`, which is `Say`).
- `PickError` (`pickindex.rs`; `HitTest` forwards `HitTestError`, `Say`).
- `BlendTarget`'s `Display` ("node {node} body {body}", `blend.rs`) and `BlendEvent` (`TargetHasNoValue` forwards a `NodeStanding`; `EdgesUnnamed` an `EdgeNamesRefused`).
- `IdAnswer::Unnamed` and `Disagreement` (`idpass.rs`, forwarding `UnnamedEntity`).

The draw sites: `frame::index_badge` ("pick index: {error}"), `frame::unindexed_refusal`, `frame::pick_refusal` (whose `Ambiguous` arm re-renders each hit's name, `StableName`'s `Display`, which says its minter by tag: speak it as `SpokenName`), `frame.rs`'s held-edges line ("held edges: … {refused}"), the blend tool's notices, and `idpass::Disagreement::notice`.

Machine channels: none known in this path; a payload that must not collapse two ids keeps `.full()`.
