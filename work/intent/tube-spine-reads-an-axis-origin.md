---
id: tube-spine-reads-an-axis-origin
kind: issue
title: "The tube reads Datum::Axis's origin as its centre, which an Axis forgets; it reads a Frame from stage 2 unit B"
status: closed
opened: 2026-10-08
priority: P1
cost: E
blocked_on: [the-pose-kinds-are-one-order]
refs: [operands-are-reads]
closed: 2026-10-08
---

`tube_args` (`crates/editor-core/src/eval/wire.rs:1859`) takes the spine `Datum::Axis` whole: its origin is the tube's centre and its direction the frame's `w`, with a loose `u_ref` beside it. D10 compares an axis "modulo slide and spin along itself", so once stage 4 compares axes canonically two tubes on one line with different centres are one axis and two bodies. The circular pattern and the coaxial mate forget the origin; the tube alone reads it.

Found by FORK-1b's designers (`docs/DESIGN-FORK-LOG.md` row 86). Their answer: in stage 2 unit B the tube reads one `frame: Frame` slot in place of `spine` and the three `u_ref` scalars, so both axis datums define an honest `Axis`. The frame's own door then decides `u_ref` (orthonormalized, refusing a degenerate pair) where the tube's door now refuses one not exactly unit and perpendicular (`crates/editor-core/src/node.rs:2100`); unit B's PR states that change and rewrites that doc paragraph.

## Closed

By INTENT stage 2 unit B (`operands-are-reads`, branch
`intent/s2-b-reads`). `Node::Tube` and `Node::HollowTube` read one
`frame: Frame` operand in place of `spine` and the three `u_ref`
scalars (`crates/editor-core/src/node.rs`, the `Tube` doc paragraph
rewritten); `tube_args` reads the frame value whole, so the frame's
own door orthonormalizes `u` and `v` and refuses a degenerate pair one
node upstream. Both axis datums define an honest `Axis`. The corpus's
tube documents, the tour's teapot handle and Python's `Node.tube` /
`Node.hollow_tube` read a `Node.datum_frame`; the id-free geometry
fence (`m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`)
held.
