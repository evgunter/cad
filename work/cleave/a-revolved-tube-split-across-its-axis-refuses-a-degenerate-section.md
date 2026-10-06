---
id: a-revolved-tube-split-across-its-axis-refuses-a-degenerate-section
kind: issue
title: a revolved tube split across its axis refuses DegenerateSection: its bore's section is a one-vertex loop
status: dispatched
opened: 2026-10-06
priority: P0
cost: M
branch: cleave/tube-across-axis
---



## What

A tube revolved about y (the profile rectangle (0.5, 0)–(1, 0)–(1, 1)–(0.5, 1),
`Revolution::Full`) split by the plane y = 0.5 refuses
`Join(DegenerateSection)`. It refuses under +y, under −y (the mirrored rerun
refuses again), and under a tilted normal (0.1, 1, 0). The solid cylinder
revolved from (0, 0)–(1, 0)–(1, 1)–(0, 1) splits cleanly at the same three
planes. Measured on main at 9d03eda, with the fixtures of
`crates/sweep/tests/split_tangent_edge_curved.rs` (`revolved`).

The section is an annulus. Instrumenting `Sweep::cut` (`splitting/join.rs`) on
the completed polygon shows the bore's section loop as ONE half-edge from the
vertex (0.5, 0.5, 0) back to itself, carrying a `Scaffold(RevolvedPoint)`
curve. `certify_section_area` reads its area as zero and refuses. The solid
cylinder's wall section, by contrast, completes with two vertices,
(±1, 0.5, 0). Not traced, but likely: the one-vertex loop is the self-loop
chord `chord_spec` leaves on the scaffolding-circle convention
(`chord_join.rs`, "Self-loop chords keep the scaffolding-circle convention"),
since the plane crosses the bore wall's single revolve seam once.

## Why it matters

A pipe cut across is an ordinary split, and it refuses, with a payload
naming a degenerate section that does not exist. The same one-vertex loop is
what reached `SplitFinishError::Corrupt` in the concave-graze row's socket
and counterbore poses (`below_chord_u_ref`, `splitting/finish.rs`: a loop of
fewer than two corners has no first chord). That row's fix refuses those
grazes before the join, so `Corrupt` is no longer reached there, but a
one-corner section loop that passes the area check would still reach it.

## Where to look

Why the solid cylinder's wall section gets two vertices and the bore's gets
one is the first question; the reversed-sense wall is the visible
difference. Then `certify_section_area`'s conic excess for a
`Scaffold(RevolvedPoint)` self-loop, and `below_chord_u_ref`'s one-corner
case. That case answers `SplitFinishError::Corrupt`, which blames the
body. `plane_section` meets the same fact at its own `chord_u_ref` call
(`splitting/section.rs`) and names it:
`SectionInvariant { "the section polygon has fewer than two points, …" }`.

## Found by

`cleave/concave-graze`'s sweep of the `Corrupt` sites the concave grazes
reached.
