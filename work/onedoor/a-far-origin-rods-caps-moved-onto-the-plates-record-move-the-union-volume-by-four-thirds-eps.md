---
id: a-far-origin-rods-caps-moved-onto-the-plates-record-move-the-union-volume-by-four-thirds-eps
kind: issue
title: Moving a rod's caps onto the plate's records moves the far-origin union's volume by 4/3 ε in one fold order (union_fold_face_face_touch, back 300)
status: closed
opened: 2026-11-04
closed: 2026-11-04
---



Found by the one-surface-record unit's final run (branch
`onedoor/one-surface-record` at `586255da0a`); green on main
`55f1a93ff1`.

`crates/editor-core/tests/union_fold_face_face_touch.rs`
`a_carrier_whose_origin_stands_far_off_hands_on_the_pass_s_line` asserts
the union's volume to `1e-9` absolute. Only the cell "back 300: rod,
block, plate" is the first that fails:

| ε | volume | want | error |
|---|---|---|---|
| 1e-9 | 17.78539816473077 | 17.78539816339745 | 1.33e-9 |
| 1e-6 | 17.785399496730943 | 17.78539816339745 | 1.33e-6 |
| 1e-12 | passes | | under the bound |

The error is `4/3 ε` at both rows that fail, so it scales with the
band, not with rounding. The rod's caps lean `0.1 ε` and are in band of
the plate's top and bottom planes. The boolean decides each pair one
carrier, and `Body::unify_surfaces` moves the rod's cap faces onto the
plate's two plane records (`z = 1` and `z = 0`), which every fold order
does. Before this unit each cap kept its own leaning record, and the
volume read its closed form. The two fold orders before it read it
within `1e-9`. The test stops at the first failing cell, so the order
after it and the `back 1000` row are unread.

## Closed: correct, bound re-derived (2026-11-04)

The shift is independent of `back` (10, 300 and 1000 read alike), and it
shows only in the orders that fold the rod first. Those are the orders
that keep the rod's records and move the plate's faces onto them. At a
lean of `0.2 ε` it is absent, because the caps are no longer glued one
carrier there. At a lean of `0` it is absent too.

It is derived exactly. A plane face contributes `(1/6) a · ∮ p × dp`,
where `a` is its record's anchor (`geom_brep::props`'s per-surface
anchors). The plate's top loop stays at `z = 1`, so `∮ p × dp = 32 ẑ`.
The rod's top record is anchored at `y = 2.5`, where it stands
`2.5 · lean` above `z = 1`. So the top face reads `16 · 2.5 · lean / 3`
more, which is `4/3 ε` at `lean = 0.1 ε`. The bottom record is anchored
at the origin, on both bottom planes, so the bottom face reads the same.
That is the measured `1.3333 ε` at ε 1e-9 and 1e-6. It is within the
band times the moved face's area (`16 ε`), so it is correct under the
tolerance model.

The row's bound is now `1e-9 + 16 · 2.5 · lean / 3`, with that
derivation in a comment (`touches_within`). It passes at all three ε
rows.

The derivation also shows where the bound comes from: the gap is read
at the kept record's anchor, not over the faces. That caveat is filed
as `work/onedoor/a-face-moved-onto-a-record-reads-its-volume-at-the-records-anchor.md`.

## The shift is gone (2026-11-10)

The `4/3 ε` above was the plate's loop read at the rod's record anchor.
Each face is read at the face now
(`work/onedoor/a-face-moved-onto-a-record-reads-its-volume-at-the-records-anchor.md`,
fixed), so the row asserts main's `1e-9` again and `touches_within` is
gone.
