---
id: a-shells-pad-still-depends-on-which-quadrature-face-it-stores-first
kind: issue
title: "A shell's pad still depends on which quadrature face it stores first: 90× across rotations at 1e-6 when its quadrature faces lie a kilometre apart"
status: open
priority: P2
cost: M
refs: [a-quadrature-faces-pad-scales-with-its-shells-size-and-stored-face-order]
opened: 2026-10-20
---

(Found by the review of PR 5495, which closed
`work/flux/a-quadrature-faces-pad-scales-with-its-shells-size-and-stored-face-order.md`.)

## What

Each shell's faces are read about the stored start vertex of its first
face that takes the quadrature (`crates/topo/src/props.rs`,
`shell_reference`). That bounds a quadrature face's lever by the extent
of the shell's quadrature faces, not by the shell's. But which quadrature
face is first is the stored order's choice, and nothing bounds the extent
itself.

The body is one shell: `torus(2, 0.5)` at home, `torus(0.2, 0.05)` at
`x = −900.27`, both on one 900.6 m bar. Rotating its stored face list
moves the reference from one torus to the other, and the other torus's
walls are then read across the bar. Pads across the 12 rotations:

| ε | pad min–max | ratio | about the world origin |
|---|---|---|---|
| 1e-9 | 2.28e-5 – 5.88e-5 | 2.58× | order-free |
| 1e-12 | 2.14e-8 – 5.80e-8 | 2.71× | order-free |
| 1e-6 | 4.14e-5 – 3.74e-3 | 90.2× | order-free, 4.14e-5 |

The vertex rule (5442) reads the same numbers on this body. Nothing
refuses.

Pinned as known by
`crates/sweep/tests/a_spiric_cut_far_off_and_turned_reads_as_at_home.rs`
`a_shells_pad_moves_with_which_quadrature_face_it_stores_first_as_known`.
The same row pins the cut far off and turned (`P ∖ T`, 10 rotations) at
1.36×, 1.51× and 1.88× across the three ε, which is order dependence
within one compact quadrature extent.

## What the closed item measured

That item's table, in its Closed section and PR 5495's body, rules out:
- **a mean point**, whether the shell's box centre or the box centre of
  its quadrature faces: off the coordinate grid, so it rounds every moved
  coordinate;
- **a per-face point with a carry**: the carry restores the width;
- **a nearest-vertex choice**: it needs recorded comparisons.

What none of them addresses: one shell with two quadrature regions far
apart has no single point near both.

## Shapes worth measuring

- **One point per quadrature region.** Group a shell's quadrature faces by
  adjacency or by box overlap. Read each group about a vertex of its own,
  and carry each group's sum to one point. A group's carry is
  `(c_g − c)·Σ A⃗_f` over its faces. Whether that sum is narrower than its
  terms' widths is the measurement: a closed group's vector area sums to
  its boundary's, so a group bounded by closed-form faces may carry
  exactly.
- **Order-free first.** Make the choice order-free (a key the stored
  order does not set), so the pad is at least one value per solid, even
  where it is wide.

## Done when

The two-tori body reads within 2× across rotations at every ε, and the
row is restated to assert it. Or this item is closed with the measurement
that says no shape helps.
