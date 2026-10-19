---
id: the-quadrature-position-term-could-use-the-shells-box-centre
kind: issue
title: The quadrature lane's position term could use the shell's box centre and halve the lever
status: open
priority: P3
cost: M
refs: [spiric-face-volume-budget-depends-on-world-position]
opened: 2026-10-19
---

(Filed by the unit that fixed
`work/flux/spiric-face-volume-budget-depends-on-world-position.md`.)

## What

Every face's flux is read about its shell's reference, a vertex of the
shell (`crates/topo/src/props.rs`, `shell_reference`). Its lever is at
most the shell's diameter wherever the body sits. For a body centred on
the world origin, the lever about the origin was about the radius, so a
vertex costs up to twice the width. A quadrature face's width scales
with the lever. Measured at ε 1e-9:

| what | about the origin | about the shell's vertex |
|---|---|---|
| `m8_3_rational_volume` arc prism volume pad | 0.98e-6 | 2.127e-6 |
| `cert5_offgrid_knot_rational` blade-8 volume pad | under 2.048e-6 | 2.127e-6 |
| step-export `loft_prism` pad (mm³) | 1.0436e-5 | 1.0695e-5 |
| step-export `nonuniform_loft` pad (mm³) | 2.0428e-5 | 2.7534e-5 |
| step-export `swept_elbow` pad (mm³) | 1.1657e-5 | 1.6727e-5 |

The step-export sidecar pads grow by 2–40%. The first two rows' pad
ceilings went from `2·1024ε` to `4·1024ε`, stated from the 2× lever bound
at each site.

## The candidate

Read the quadrature lane's position term (`quad_lane::cut_face_rounds`'s
centre) about the centre of the shell's vertex box, snapped to an `f64`
grid. Its lever is at most half the diameter, for every pose.

## The caveat

The box centre is not a point of the body, and its coordinates are not
the body's own. `corner_of`'s docs (`crates/topo/src/props.rs`) record
that a mean point, moving a disc's wall by it, read the wall's height,
exactly `Kε`, across the band's edge in the re-derivation's closed-form
decisions. Keep closed-form faces about the vertex. Before using the box
centre for the quadrature term, measure whether that term's rounding
moves any decision, and whether one rule per shell still holds: every
face of a shell must be read about one point, or the shell's sum is
wrong. A closed-form face read about the vertex and a quadrature face read
about the box centre must still be summed about one point, through the
vector area.

## Done when

Measured: the pads above, about the box centre, and the band decisions
that moved, if any. Then either the change, with its rows, or this row
closed with the measurement.
