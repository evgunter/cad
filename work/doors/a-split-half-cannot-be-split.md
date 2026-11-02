---
id: a-split-half-cannot-be-split
kind: issue
title: Splitting a split half refuses NotABody: the split plane is read off the split node, not the half the seat names
status: open
opened: 2026-11-02
priority: P3
cost: E
---

Found by the PR 6331 correctness review (probe `probe_split_of_an_unplaced_split_half`). `SessionOp::AddSplit` with `target: Operand::output(split, port)` refuses `Refusal::Frame(GestureFrameFault::Geometry(PinRefusal::NotABody { node: split }))`. It does so on an unplaced split too (`tests/common/drawn.rs`'s `split_box`), so a body split once cannot be split again from the GUI.

The plane is read off the target's geometry by `frame_off` (`crates/viewer/src/session.rs`), which takes the node a port read names and matches the node's WHOLE value (`ev.result(node)`) against `ValuePayload::Body`. A split's value holds two bodies, so it is no `Body`, and the refusal fires before the port is consulted. The table lookup just below it already reads the port (`doc.output(node, port)`), so the half's own body is what the solid should be read from. The same holds for any node with more than one body output that a gesture reads a frame off (a circular pattern's axis seat goes through `read_off` as well).

**Fix when dispatched:** read the solid of the body the read names, by its port, not the node's whole value; then a row: a split of a split half commits and draws three pieces.
