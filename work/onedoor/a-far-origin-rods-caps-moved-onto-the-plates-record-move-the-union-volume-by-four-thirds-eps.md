---
id: a-far-origin-rods-caps-moved-onto-the-plates-record-move-the-union-volume-by-four-thirds-eps
kind: issue
title: Moving a rod's caps onto the plate's records moves the far-origin union's volume by 4/3 ε in one fold order (union_fold_face_face_touch, back 300)
status: open
opened: 2026-11-04
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

Not re-baselined. The open question is whether a volume that moves by
about one band when a face moves onto a carrier in band of its own is
correct. If it is, the bound is ε-relative and the row says so. If it
is not, the defect is in what the move leaves for the quadrature to
read: the cap's boundary circles stay certified on the leaning plane,
and its images are derived again on the plate's chart. It is unknown
why the first two orders do not show it.
