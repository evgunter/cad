---
id: tube-spine-reads-an-axis-origin
kind: issue
title: "The tube reads Datum::Axis's origin as its centre, which an Axis forgets; it reads a Frame from stage 2 unit B"
status: open
opened: 2026-10-08
priority: P1
cost: E
blocked_on: [the-pose-kinds-are-one-order]
refs: [operands-are-reads]
---

`tube_args` (`crates/editor-core/src/eval/wire.rs:1859`) takes the spine `Datum::Axis` whole: its origin is the tube's centre and its direction the frame's `w`, with a loose `u_ref` beside it. D10 compares an axis "modulo slide and spin along itself", so once stage 4 compares axes canonically two tubes on one line with different centres are one axis and two bodies. The circular pattern and the coaxial mate forget the origin; the tube alone reads it.

Found by FORK-1b's designers (`docs/DESIGN-FORK-LOG.md` row 86). Their answer: in stage 2 unit B the tube reads one `frame: Frame` slot in place of `spine` and the three `u_ref` scalars, so both axis datums define an honest `Axis`. The frame's own door then decides `u_ref` (orthonormalized, refusing a degenerate pair) where the tube's door now refuses one not exactly unit and perpendicular (`crates/editor-core/src/node.rs:2100`); unit B's PR states that change and rewrites that doc paragraph.
