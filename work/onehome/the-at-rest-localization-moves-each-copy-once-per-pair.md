---
id: the-at-rest-localization-moves-each-copy-once-per-pair
kind: issue
title: The at-rest localization moves each copy once per interfering pair, where a cached moved copy could serve them all
status: open
opened: 2026-11-19
priority: P3
cost: E
refs: [a-body-records-its-parts-in-their-frames]
---



Since stage 1 of fork 288, the at-rest localization (`crates/editor-core/src/checks/at_rest.rs`, `Operands`) keeps each copy of the product finished in its own frame, once per localization, with its motion into the product's frame. Each interfering pair hands its two copies to the placed door (`topo::boolean_placed`), which moves each one not in the frame. A copy in several pairs is therefore moved once per pair: a move-and-ask (`FinishedBody::moved`), not a finish, so each copy is finished and placed once (`product_finishes_once::localizing_takes_each_copy_once`: 26 finishes, 31 before, the moved f64 leaves no longer finished again on their images; 7 placements, 15 were a copy taken per pair). Before, the copy was moved once and the image reused across its pairs.

Measured over the corpus's default checks at `f64`, debug build, median of three runs: 361.1 s before, 388.8 s after (+7.7%). The most-affected documents are near +9–10%: die_composed, axis_projection, revolves_on_one_axis.

This is stage 2's cached-copy question (`work/onehome/a-body-records-its-parts-in-their-frames.md`; PR 7079: "a moved copy may be kept as a cache, answering a reading only where it gives what the record read through the motion gives"). Either the placed door takes a cache of the image beside the record, or stage 2's reading through the relative motion makes the move go away.
