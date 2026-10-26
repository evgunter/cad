---
id: a-shells-pad-still-depends-on-face-order-where-one-quadrature-region-spans-the-lever
kind: issue
title: "A shell's pad still depends on its stored face order where one quadrature region spans the lever: 456× across rotations at 1e-6 with a torus on a bar's long top face"
status: closed
closed: 2026-10-26
opened: 2026-10-25
priority: P2
cost: M
refs: [a-shells-pad-still-depends-on-which-quadrature-face-it-stores-first]
---

(Found by both reviews of PR 5699, which closed
`work/flux/a-shells-pad-still-depends-on-which-quadrature-face-it-stores-first.md`
for regions separated by closed-form faces.)

## What

A shell's quadrature faces are grouped into regions by shared edges, and a
region far from the shell's point is read about a point of its own and
carried to the shell's (`crates/topo/src/props.rs`, `References`,
`region_carries`). That bounds a face's lever by its region's extent. It
does nothing where the region itself spans the lever: a quadrature face
that runs the length of the body joins every curved face it meets into
one region, which is read about one point, the stored start of its first
face, and its pads move with the stored order as the shell's did before
regions. The carry is not taken there at all: such a region is not far
from the shell's point in its own extent (`far_from`, `CARRY_REACH`), and
where it is the shell's first region it is never carried.

Both reviewers built it on the two tori's bar
(`crates/sweep/tests/a_spiric_cut_far_off_and_turned_reads_as_at_home.rs`,
`two_tori_on_a_bar`: `torus(2, 0.5)` at home, `torus(0.2, 0.05)` at
`x = −900.27`, a 900.6 m bar of 12 × 12 from `x = 0.3` along `−x`), with a
torus on one of the bar's long faces. The torus cuts that face, a plane
900.6 m long, which then takes the quadrature (a plane with a spiric rim)
and joins the torus's walls into one region spanning the bar. Pads across
the rotations of the stored face list, bit-identical to the merge base:

| body | ε | across rotations |
|---|---|---|
| a side torus `torus(2, 0.5)` at `(−895, 0, 5.7)` on the bar's top face (the first review) | 1e-6 | 456× |
| a mid torus `torus(2, 0.5)` at `(−450, 0, 6.3)` on a side, joining both ends' regions (the second review) | 1e-6 | 82.7–90× |

The first review's bodies are on `review/5699-018JLnDe`
(`crates/sweep/tests/review_5699_probe.rs`, `review_5699_multi_region_pads`:
`side_near` at `(−5, 0, 5.7)`, `side_far` at `(−895, 0, 5.7)` and `mid`
`torus(0.2, 0.05)` at `(−450, 0, 6.15)`); the second's on
`review/5699-019fZ6Ym` (`crates/sweep/tests/review_5699_probes.rs`,
`bodies`: "merged by a mid torus on a side", "merged by a small mid
torus" at `(−450, 0, 6.1)`).

## Shapes worth measuring

- **Split a region at a long face.** A quadrature face whose extent is
  the body's is the one that joins the region across the lever. Read it
  alone, about a point of its own, and the region's other faces as
  regions of their own, each carried by its boundary's vector area. The
  boundaries then cross the long face, along edges that are not
  closed-form (a spiric rim), so the carry is no longer exact but for
  rounding: whether those edges' vector areas are narrow enough is the
  measurement.
- **One point per face, with exact carries along shared closed-form
  edges.** Read each quadrature face about a vertex of its own and carry
  it by its own loops' vector area where every edge of it is closed-form;
  where an edge is shared with another quadrature face, carry the pair
  across it as one. That is the region rule at the finest grain where
  the carry is exact.

## Done when

The two bodies above read within 2× across rotations at every ε, pinned
by a row, or this item is closed with the measurement that says no shape
helps.

## Closed

Closed by `flux/shell-pad-joined-regions`, which narrows what a region is
(`crates/topo/src/props.rs`, `References`, "What a region is"):

- **Regions join only across rims**, the edges whose vector area does
  not read exactly (`curved_rim`: a spiric or rational carrier). A line,
  conic or polynomial-spline edge between two quadrature faces is a
  boundary, so a long plane no longer joins everything it touches across
  its straight edges.
- **A region's point is the stored start of its first curved face**
  (`region_point`), which lies on its rims and so on the planes they
  bound, not the start of its first face, which may be a long plane's a
  bar's length off.
- **A plane is levered off its plane.** A plane's flux about a point on
  its plane is zero, so its samples count for `far_from`'s extent by
  their normal distance `|n·(p − x)|`, and the region's nearness is
  measured over its curved faces where it has any.
- The first region stays read about the shell's point: carried about its
  own point it read the same pads to six digits on every body here.

Pads across rotations of the stored face list, measured, and pinned at
1.1× in `a_shell_joined_along_a_long_face_reads_alike_whichever_face_it_stores_first`:

| body | ε | before | after |
|---|---|---|---|
| R1: a side torus `torus(2, 0.5)` at `(−895, 0, 5.7)` on the top face | 1e-6 | 456× | 1.000005× |
| | 1e-9 | | 1.000005× |
| | 1e-12 | | 1.0054× |
| R2: a mid torus `torus(2, 0.5)` at `(−450, 0, 6.3)`, with `torus(0.2, 0.05)` at `(−900.27, 0, 0)` | 1e-6 | 83× | 1.000005× |
| | 1e-9 | | 1.0000067× |
| | 1e-12 | | 1.0077× |

A body these rules do not reach is filed as
`work/flux/a-long-plane-face-cut-by-a-torus-reads-a-wide-pad-about-a-point-off-its-plane.md`:
a torus astride the bar's end edge reads 1170× across rotations at
1e-12, all of it one long plane face's area width levered by the shell's
point's distance off its plane.

