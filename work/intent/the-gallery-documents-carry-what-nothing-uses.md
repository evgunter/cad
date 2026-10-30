---
id: the-gallery-documents-carry-what-nothing-uses
kind: issue
title: Two gallery documents hold nodes nothing uses, which the unused check reports in the viewer
status: closed
opened: 2026-10-30
priority: P3
cost: E
branch: intent/gallery-keeps-nothing-unused
closed: 2026-10-30
pr: 6108
---

Found by the `unused` check (`Doc::unused`, `crates/editor-core/src/doc.rs`), counting its findings over the tour's gallery documents (`demos/tour/src/gallery.rs`, the list `gallery::run` writes) before its default landed at Warn.

Two of the nine gallery documents hold nodes nothing uses, and the viewer's Checks window, which runs the registry on every landing, now lists them:

- `teapot` (`demos/tour/src/teapot.rs`, `gallery_document`): two `Place` nodes. `build_doc` places the handle and the spout against the pot for the unions `gallery_document` then deletes (`handle_union`, `spout_union`, `pot`), and the gallery copies each body into the world afresh, so the placements against the pot are left with no reader.
- `diefillet` (`demos/tour/src/diefillet.rs`, `gallery_document`): one `Fillet` that `die.composed` does not read.

The gallery's own rule (`demos/tour/tests/gallery.rs`, `each_gallery_document_denotes_its_scene_or_says_why_not`) is that a document says what its scene means or says why not. Either the scene deletes what nothing uses, as the teapot already does for its unions, or its row says why the node stays. The corpus holds two more (`kitchen_sink`'s `Union`, `die_composed_tour`'s `Fillet`); those are test fixtures exercising node kinds and are not viewed, so they are not this row's.

## Closed

Each scene deletes what nothing reads, as the teapot already did for its unions. `teapot::gallery_document` also deletes the handle's and the spout's placements against the pot, which only the deleted unions read; the gallery copies each body into the world afresh. `diefillet::gallery_document` deletes the blank, a narration body nothing places or reads; `corpus_text` asserts its replay against the scene's placed document with the blank kept, so the corpus fixture is unchanged (regenerated and compared byte for byte). The gallery row `each_gallery_document_denotes_its_scene_or_says_why_not` asserts `Doc::unused()` is empty for every gallery document.
