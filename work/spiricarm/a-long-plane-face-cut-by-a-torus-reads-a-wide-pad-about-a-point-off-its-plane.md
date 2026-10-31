---
id: a-long-plane-face-cut-by-a-torus-reads-a-wide-pad-about-a-point-off-its-plane
kind: issue
title: "A long plane face cut by a torus reads a pad up to 1e3× wider at 1e-12 when its shell's point lies off its plane: 70× with one torus on a bar's top face"
status: open
opened: 2026-10-26
priority: P2
cost: M
refs: [5801]
---

(Found while closing
`a-shells-pad-still-depends-on-face-order-where-one-quadrature-region-spans-the-lever`, closed by PR 5801.
PR 5801's review found it on simpler bodies than the first one filed here,
with head and base reading the same.)

## What

A torus cut into a long plane face of a bar makes that face take the
quadrature, and the face's pad moves with where the shell's point `c`
falls: across rotations of the stored face list, `c` lands on the end cap,
off the top face's plane, or on it. Every body is on the 900.6 m bar of
`crates/sweep/tests/a_spiric_cut_far_off_and_turned_reads_as_at_home.rs`
(`tori_on_the_bar`) unless stated; `torus(R, r)`'s are at the centres
given. Pads across rotations, widest over narrowest, measured on PR 5801's
head (the reviewer's `review_5801_probes.rs`, `probe_bodies`):

| body | 1e-6 | 1e-9 | 1e-12 |
|---|---|---|---|
| **B6** (the lead row): the home `torus(2, 0.5)`, and `torus(2, 0.5)` on the top face at `(−5, 0, 5.7)` | 4.88× | 2.59× | 69.9× |
| B4: the same two on a 70.6 m bar, the second at `(−60, 0, 5.7)` | 36.6× | 2.59× | 36.1× |
| B2: B6 and `torus(0.6, 0.15)` at `(−895, 0, 5.85)`, both on the top face, one region spanning 890 m | 5.15× | 2.72× | 74.6× |
| B5: the home torus, `torus(2, 0.5)` astride the end edge at `(−0.7, 0, 5)`, and one at `(−895, 0, 5.7)` | 2.52× | 2.83× | 80.8× |
| the first body: `torus(2, 0.5)` at `(−0.7, 0, 5)` alone | 1.19× | | 1170× |

B5 is pinned at these numbers by
`a_shell_whose_tori_meet_along_circles_keeps_each_torus_a_region`, which
guards the region partition and is not this item's row.

For the first body at 1e-12, the two wide rotations put `c` on the end cap
at `(0.3, 0.5, 3.268)`, 2.73 m below the top face's plane. Per face
(`mass_properties_with_face_terms`):

| face | `c` on the end cap (wide) | `c` at `(0.3, 6, 6)`, on the top face |
|---|---|---|
| top face `12v1`, plane | 1.16e-5 | 0 (not listed) |
| the four torus faces | 2.5e-10 – 9.1e-9 | 2.5e-10 – 8.0e-9 |
| end cap `6v5`, plane | 4.5e-16 | 2.4e-28 |

So the whole spread is one plane face. A plane's flux about `c` is
`(n·(p − c))·A`, so its pad is the normal lever, 2.73, times its area's
width: here about 4e-6 m² on a face of about 5400 m², where 1e-12 targets
much less. The torus faces read alike whichever point they are read
about. The other bodies' per-face split is not measured.

## Why, as far as measured

Not measured past the table. The top face's loop runs the bar's length
and holds the torus's spiric rim, so its area is read by the curved-rim
lane about a reference that may lie a bar's length from the rim; the
width that lever puts on `A` is then multiplied by the normal distance.
Reading a plane's area about a point of its own loop (it is
translation-invariant), or levering a plane's flux about a point on its
plane, would test that.

## Done when

B6's rotations read pads within 1.1× of each other at each ε, pinned by a
row, and B4, B2 and B5 within 2×.
