---
id: a-plane-tangent-along-an-edge-that-cuts-elsewhere-refuses-to-split
kind: issue
title: A split whose plane grazes one part of a solid along an edge and genuinely cuts another refuses (one-sided tangency posture)
status: open
opened: 2026-09-24
priority: P0
cost: H
---


## What

A legal split refuses, and the author wants it to succeed.

- **Topo repro.** `brick(0..1.5, 0..1, 0..1) ∪ brick(1.2..1.3, −1..2,
  0.5..3)`, split by the plane y + z = 2 (through (0,1,1), normal
  (0,1,1)/√2). The plane touches the block only along its top/far edge
  y = z = 1, and it cuts the slab through its full width. `split`
  refuses `SplitJoinError::DegenerateSection`. The pinned row is
  `crates/topo/tests/split_tangent_spur.rs`.
- **Editor-core repro.** The same shape as a declared union of `a` =
  x(0,1) and `b` = x(0.5,1.5), both y,z ∈ (0,1) and flush-declared, with
  `g` = x(1.2,1.3), y(−1,2), z(0.5,3.0). The orders `[a,g,b]` and
  `[g,a,b]` fuse, and the split refuses. The pinned row is
  `crates/editor-core/tests/emit_split_duplicate.rs`.

## Why it refuses today

The contact along y = z = 1 is a one-sided tangency: there is material
only on the plane's below side. Refusing a one-sided tangency is the
posture pinned by `crates/topo/tests/m3_pr3_split.rs`,
`one_sided_tangency_refused_typed` (~440). That row is an apex prism
touching the plane along one edge, and nothing else is cut.

Before PR 3133 this document split "successfully" through `split`'s
pinch lane (`splitting/mod.rs`, the D7 mirror rerun). The mirrored run
joined the contact into the slab's section as a zero-width spur, and
both halves carried a slit. PR 3133 refuses that spur
(`SplitJoinError::SectionSpur`), so the document is back under the
tangency posture.

## The posture question

Should a plane that grazes one part of a solid and cuts another split
cleanly? The geometrically right answer exists:
- the slab's section is the only section;
- y = z = 1 stays an ordinary edge of the Below piece that happens to
  touch the plane.

Reaching it means the tangent contact mints no null edges: a
classification change in `rules.rs`/`classify.rs`. The same change
decides what `one_sided_tangency_refused_typed`'s standalone contact
should do. It could split into Below = the whole prism and Above =
empty, or it could keep refusing, and that is the open question.

## Found by

EMIT's `split-section-face-keeps-a-zero-area-spur-along-a-tangent-edge`
(PR 3133), from its review (m3).
