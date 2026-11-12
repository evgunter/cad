---
id: a-lattice-drawn-rounded-outline-refuses-its-shared-section-row
kind: issue
title: A rounded outline drawn through the path lattice refuses its section's shared row where the bulge table of the same outline builds
status: closed
opened: 2026-11-10
closed: 2026-11-10
priority: P1
cost: M
---


## Finding

Found while rendering PR 6520 (`band/closed-mixed-chain`, head
`4b913f5c9a`) for Ev. The rounded rectangle of
`crates/sweep/tests/band_closed_mixed_chain.rs` (`[−1, 1] × [−0.6, 0.6]`,
corner radius 0.3, 0.5 thick) was drawn the way a user writes it through
`pncad`: `Open.at(..).line_to(..)`, then `.tangent().tangent_arc_to(..)`
and `.tangent().line(len)` around the outline, closing on
`Start.arrives_tangent()` (`docs/renders/band-2026-11/scenes.rs`,
`rounded_rect`). The lattice puts some of its vertices a few ulps off
the table's (`(1.0000000000000002, 0.3)`, `(-0.7, 0.5999999999999983)`,
`(-1.0000000000000042, -0.3)`).

Filleting every edge of that plate:

| outline | r = 0.05 | r = 0.14 |
| --- | --- | --- |
| the test's `bulge_loop` table | builds | builds |
| the lattice, about the origin | builds | refuses |

The pocket's mouth (the block less the rounded prism, z 0.5 to 1.5)
builds at both radii with either outline drawn about the origin, and
refuses at both radii with the lattice outline drawn at `(1.5, 1.0)`
in sketch coordinates (`p2(1.5 + x, 1.0 + y)`). Every refusal is the same:

    SurgeryInvariant { at: Edge(..), detail: "a mixed rim's section is
    not a shared row of its two bands' charts" }

raised where `geom_brep::analytic_shared_row` returns `None` for a
junction's section (`crates/sweep/src/blend/surgery.rs:6160-6171` on that
branch). The message says the surgery contradicted itself, a kernel bug.
So the shared-row reading decides on the outline's exact bits: a
correct outline a few ulps from the table's loses it. The PR's pins
all author the table, so none of them sees this.

## What would close it

The shared-row reading holds for an outline whose junctions are
tangent within the tolerance the profile validated them at, with a pin
on the lattice-drawn plate at r = 0.14 and the lattice-drawn pocket
mouth at `(1.5, 1.0)`.

## Closed

Closed on `band/closed-mixed-chain` (PR 6520), 2026-11-10, by the band-mixchain lane. On the branch's merged head the refusal no longer reproduces:
- PR 6514's admission (`c332d63401`, folded into main at `a9e10733da`) decides each row coefficient against the band instead of on exact bits, so a lattice vertex a few ulps off the table no longer loses the shared row.
- Where the row still does not certify, the refusal is now typed: `UnsupportedGeometry` at the requested link, not `SurgeryInvariant`.

Pinned in `crates/sweep/tests/band_closed_mixed_chain_shapes.rs`, at f64 and at Interval (the outline validated at f64 and lifted, as a user's sketch is), each at Pappus's closed form:
- `a_lattice_drawn_rounded_plate_fillets_whole`: the scene's `rounded_rect` about the origin, every edge, r = 0.05 and 0.14;
- `a_lattice_drawn_pockets_mouth_off_the_origin_fillets_whole`: the block less the lattice prism at (1.5, 1.0) from z = 0.5, its mouth, r = 0.05 and 0.14.

