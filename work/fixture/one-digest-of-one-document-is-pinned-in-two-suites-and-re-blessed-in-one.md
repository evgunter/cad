---
id: one-digest-of-one-document-is-pinned-in-two-suites-and-re-blessed-in-one
kind: issue
title: One digest of one corpus document is pinned in two suites off the same feed, so a re-bless reaches one copy and misses the other (seat7 and seat8)
status: open
opened: 2026-11-14
priority: P4
cost: E
---


Found by the STYLE review of PR 6956 (`green/goldens`); filed by that lane.

`crates/editor-core/tests/seat7_sweep_lowering.rs` (`the_sweep_documents_evaluate_to_their_committed_digests`) and `crates/editor-core/tests/seat8_split_lowering.rs` (`the_split_documents_evaluate_to_their_committed_digests`) both pin `cut_cylinder` and `kitchen_sink`, each at the same constant, off the same feed (`fixture::digest`), under copies of the same RE-BLESSED prose. The feed has one home; the constants and the prose have two.

**What it cost.** PR 6820 moved `kitchen_sink`, re-blessed seat8's copy with its reason, and missed seat7's, which stayed red for the nightly and read as an unexplained move until it was bisected back to that same PR (`work/green/main-red-seat7-corner-table-moved-at-6832-and-kitchen-sink-after-it.md`). PR 6856 then moved both, and two lanes re-pinned one copy each.

**Other instances the review's grep for repeated constants turned up, not checked by this lane:** a pair repeated in `m10_p_fence.rs`; `0x05c8_0f15_d677_12fe` twice in `edit_placement_corpus_bits.rs` (two documents, `nested_islands_105` and `nested_islands_106_depth2`, so possibly one body read twice rather than one pin written twice); and seat4's documents against the per-document rows of `lib_g16_corpus_name_digests` and `perf2_name_keying_differential`, which pin different digests of the same documents and all move together.

**Owed.** One table of `(document, digest)` for the seat suites to read, so a document has one pin; then look at the other instances.
