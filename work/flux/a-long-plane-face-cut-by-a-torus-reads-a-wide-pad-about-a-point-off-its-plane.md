---
id: a-long-plane-face-cut-by-a-torus-reads-a-wide-pad-about-a-point-off-its-plane
kind: issue
title: "A long plane face cut by a torus reads a pad 1e3× wider at 1e-12 when its shell's point lies off its plane"
status: open
opened: 2026-10-26
priority: P2
cost: M
refs: [a-shells-pad-still-depends-on-face-order-where-one-quadrature-region-spans-the-lever]
---

(Found while closing
`work/flux/a-shells-pad-still-depends-on-face-order-where-one-quadrature-region-spans-the-lever.md`,
whose regions this does not involve.)

## What

The two tori's 900.6 m bar of
`crates/sweep/tests/a_spiric_cut_far_off_and_turned_reads_as_at_home.rs`
(`tori_on_the_bar`), with one torus `(R, r) = (2, 0.5)` at
`(−0.7, 0, 5)`, astride the bar's end edge: it cuts both the end cap
(`x = 0.3`) and the long top face (`z = 6`). The shell has two
quadrature regions of three faces each, neither carried (`far_from`
reads them near). Its pad over the 10 rotations of its stored face list:

| ε | narrowest | widest | apart |
|---|---|---|---|
| 1e-6 | 2.568e-5 | 3.058e-5 | 1.19× |
| 1e-12 | 3.25e-9 | 3.87e-6 | 1170× |

At 1e-12, the two wide rotations put the shell's point `c` on the end
cap at `(0.3, 0.5, 3.268)`, 2.73 m below the top face's plane. Per face
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
about.

## Why, as far as measured

Not measured past the table. The top face's loop runs the bar's length
and holds the torus's spiric rim, so its area is read by the curved-rim
lane about a reference that may lie a bar's length from the rim; the
width that lever puts on `A` is then multiplied by the normal distance.
Reading a plane's area about a point of its own loop (it is
translation-invariant), or levering a plane's flux about a point on its
plane, would test that.

## Done when

The rotations of this body read pads within 1.1× of each other at each
ε, pinned by a row.
