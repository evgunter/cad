---
id: the-m8-4-mint-with-a-non-plane-pair-is-untested
kind: issue
title: The M8-4 mint fed a certified pair with no plane in it is untested: P-F refuses by adjacency before any certificate
status: open
priority: P3
cost: M
opened: 2026-10-11
refs: [a-wall-seam-between-two-fits-has-no-section]
---

Found by SHELL's NURBS × NURBS section unit (`a-wall-seam-between-two-fits-has-no-section`,
PR 4597), at its dual review. `crates/sweep/tests/review_probes_m8_4.rs`,
`probe_f_uncertifiable_pair_refuses_at_attachment`, was written to show
that a description whose pair has no plane never reaches the M8-4
mint because no certificate vouches for it. Measured, it never asked
for a certificate: the pair it names, the bowed wall against itself,
is not the two surfaces the edge's faces wear, so it refuses at
attachment by adjacency (`EulerOpError::DescriptionNotAdjacent`).
PR 4597 re-pinned the row to that refusal and rewrote its doc.

Since PR 4597 a NURBS × NURBS pair does certify (C5's `(Nurbs, Nurbs)`
row, `edge_nurbs::nurbs_pair_limbs`), so a description naming a seam's
two spline faces now reaches the mint, and the mint's indifference to
which of `s1`/`s2` is the plane is no longer covered by "some plane
was in the pair". No row feeds the mint such a pair.

Owed: a row that sets a two-spline seam's description to its two
faces' `Intersection` through `set_edge_curve` and pins what the mint
does with it (mints both pcurves, or refuses typed by name).
