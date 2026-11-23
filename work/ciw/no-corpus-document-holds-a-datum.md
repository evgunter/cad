---
id: no-corpus-document-holds-a-datum
kind: issue
title: No corpus document holds a datum
status: open
opened: 2026-11-22
---


`Node::Datum` (D10 `[placement.datums]`, formwords/datum) is a node
kind no corpus document authors, so `corpus::NODE_KINDS`
(`crates/editor-core/tests/corpus/mod.rs`) leaves it out rather than
reporting a permanent miss in `m4_pr8_corpus::vocabulary_coverage_is_total`.
The natural witness is `face_sketch` (or a sibling) drawing its boss on
a datum flush on the box's top face (`Recording::draw_on_face`), which
moves that document's pinned rows (digests, latency manifest, interval
rows) and so wants its own unit. Once a document holds one, add
`"Datum"` to `NODE_KINDS` and drop its name from the roster's doc line.
