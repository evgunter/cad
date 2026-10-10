---
id: content-keys-feed-an-expression-spelling-no-body-carries
kind: issue
title: Content keys still feed a flow-bearing slot's expression spelling, though no body carries a token of it
status: open
opened: 2026-10-09
priority: P3
cost: E
---


A flow-bearing slot's expression enters its node's content key as a
canonical encoding beside its value (`crates/editor-core/src/param_source.rs:149`
`feed_content_key`, fed at `crates/editor-core/src/eval/mod.rs:5456`
for a profile edge's radius and `:6393` for a blend's size). The
reason was the lowered `ParamSource` token the minted fields carried:
a value-preserving respelling that memo-hit would have handed back a
body whose tokens named the old expression.

Stage 4 PR E deleted `ParamSource` (no body carries a token now), so
the feed's reason is gone. What it costs: a respelling that keeps
every value re-runs the node and its cone
(`intent_literals_a_definitions.rs`'
`a_respelled_definition_reruns_its_flow_bearing_reader` pins the
re-run). What it might still buy: nothing a body reads; the
`unproven-coincidence` door reads the document, not the key.

The question is whether to drop the feed (a memo hit on a
value-preserving respelling) or keep it for a reader not yet named.
