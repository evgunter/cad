---
id: python-cannot-ask-what-a-node-reads-or-what-reads-it
kind: issue
title: Python cannot ask what a node reads or what reads it
status: open
opened: 2026-11-23
priority: P3
cost: M
---


PR #7356 (OFFER) gave the kernel two answers to "what does this node read, and what reads it", and the viewer's Properties draw them as links:
- `editor_core::OperandRead`, the element of `Doc::operand_reads` (`crates/editor-core/src/doc.rs`): one read a node's operand seat makes, and the node it reaches.
- `editor_core::ReadIndex`, what `Doc::read_index` answers: the whole document's reads and read-by, built in one walk.

Both are re-exported through `pncad::document` (`crates/pncad/src/document.rs`). Python binds neither. It also has no reader for a node's payload, so a Python caller cannot compose the answer from the document either. The binding census lists both as `gap: B-READS`.

Closing it needs:
- a `Doc.reads(node)` / `Doc.read_by(node)` door, with `pncad.pyi` entries;
- one Python row asserting, on a corpus document, that A reads B at seat S exactly when B's read-by holds A at S.

(OFFER orchestrator)
