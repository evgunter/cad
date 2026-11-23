---
id: a-filleted-one-segment-hole-rims-band-exceeds-its-mesh-certificate
kind: issue
title: A plate hole whose one-segment rim is filleted refuses tessellation CertificateExceeded at some (radius, delta) where its two-arc twin meshes: r = 0.005 at delta 1e-3 is 7.7x over
status: closed
opened: 2026-11-21
priority: P1
cost: M
branch: tesslane/one-slit-band
closed: 2026-11-23
---



Found by PROFILE's `circle-lowers-to-one-segment` (branch `profile/circle-lowers-to-one-segment`), which lowers `circle()` to one full-turn segment, so a hole's rim is one closed edge at one vertex and its fillet band is one ring face with one slit where the two-arc rim's band was two.

## What

The plate `[−1, 1]² × [0, 1]` with a hole `circle(0, 0, 0.3)`, its bottom rim (or both rims) filleted through `Node::Fillet`, meshed by `mesh::tessellate` (`crates/editor-core/tests/edit_bore_two_rims.rs`, the lane's probe), against the same plate whose hole is `circle_split(2)`:

| fillet r | δ | one segment | two arcs |
|---|---|---|---|
| 0.05 | 1e-2 … 3e-5 | meshes | meshes |
| 0.005 | 1e-3 | `CertificateExceeded { face: 9v1, bound: 7.72e-3, requested: 1e-3 }` (bottom rim; `10v1` with both) | meshes, 452 triangles |
| 0.005 | 1e-2, 3e-4, 1e-4, 3e-5 | meshes | meshes |
| 0.0005 | 1e-2 … 3e-5 | meshes | meshes |

The viewer meets it on its startup document: `viewer`'s `a_held_face_on_a_copy_a_feature_re_points_stays_held` fillets the hole's bottom rim at `r = 0.0005`, and the re-pointed copy is undrawn, `Tessellate(CertificateExceeded { face: 9v1, bound: 2.24e-4, requested: 1e-4 })`.

So it is a lottery over `(r, δ)`, not a floor: one δ fails where coarser and finer both pass. Which face `9v1` is (the band, by the census, is the likeliest) and which bound the certificate exceeds are not measured.

## Why it matters

A filleted round hole is everyday geometry, and the refusal is the picture's and the export's: the body builds and validates, and the mesher refuses it at a display δ.

## What closing it takes

Read the refused face's certificate: which lane meshes it (a one-ring torus band with one slit, its chart window a whole turn), and why its bound comes out several times the requested δ at that δ alone. The two witnesses above are pinned red at their stops.

## Unit (2026-11-23, TESSLANE)

**Letter M. Tier: single review, FULL.** The cause is not known, so the change cannot be read and believed before it is measured: a certificate bound several times δ at one δ between two that pass is either a step chosen for the wrong span, a triangle the lane's bound does not cover, or a certificate reading too much, and each cure touches how a lane sizes a face.

Branch `tesslane/one-slit-band`, cut from main.

**Stage 1: find the cause, by measurement.** The witnesses are on `profile/circle-lowers-to-one-segment` (`crates/editor-core/tests/edit_bore_two_rims.rs`, and the viewer row the issue names), pinned red there. Read them on that tree without merging it: which face refuses, which lane and arm the route gives it, its chart window and loops, the target and steps the lane chose, the triangle whose certificate reads 7.72e-3 and how it was made (which points, placed by what), and why δ = 1e-3 fails where 1e-2 and 3e-4 pass. Then say what class this is: every (surface, loop shape, r, δ) the same cause reaches, found by sweeping r and δ and the band's other parameters, not the two cells in hand. `memories/refusal-text-is-not-cause.md` binds: the payload and the raising site are the instrument.

**Stage 2: cure it where it starts**, on main's tree, with a body `mesh`'s own tests can build on main (a torus band, a ring with one slit, its window a whole turn: say how it is made, and if no door on main makes one, stop and say so before stacking on PROFILE's branch). Rows pin a relation, not a count; a mutant per ingredient; the §5 sweep for the same cause on the other charts (a one-slit ring on a cylinder, a cone, a sphere); the three ε, with no ε read by anything that chooses the mesh (`docs/DESIGN.md` `[tolerance.chordal]`, `crates/mesh/README.md` `[mesh.winding]`); patch digests showing which faces move.

**Stop and report between the stages if** the cure is not local to one lane's step or one certificate, if it would change what `crates/mesh/README.md` says, or if the cause is one of this track's open rows (`a-quarter-arc-swept-annulus-exceeds-its-triangle-certificate` is a `CertificateExceeded` lottery over a count too, and may be the same cause).

When it lands, PROFILE's parked unit `circle-lowers-to-one-segment` merges main and flips its two pinned rows; a line goes on `work/profile/log.md`.

### Stage 1's result and the ruling (2026-11-23)

Measured by the implementer on PROFILE's tree and reproduced on main with a body `mesh` builds itself (a full revolve of a plate cross-section whose corner is a quarter arc: one torus face, one slit, the revolve's seam).

- **The face** is the fillet band, a torus rectangle (a whole turn by a quarter of the tube), and it is the rectangle lane's (`mesh::curved`), not the chart lane's. The two-arc twin's band is the same one face with one slit; it passes by the rounding of its slit's span.
- **The cause.** The slit, a quarter arc of the tube, is chorded into 3 where the face's grid has 2 rows: the chord count is `ceil(span / step)` with the step capped at π/4, and the slit's stored span is π/2 plus 5 ulps. Its chord points then sit between the grid's rows. The rectangle lane triangulates in raw chart coordinates, where this band's cell is 7.3 times longer along the tube than around the ring, so a point between two rows has an empty circle 3.5 columns wide and joins nodes four columns away: 26 triangles over δ, the worst 7.7 times.
- **Why one δ and not its neighbours.** At a coarser δ the columns are wider and the fan reaches one column; at a finer δ the grid's rows reach the slit's count and the points sit on them.
- **The class.** Rectangle-lane torus faces with a thin tube whose meridian side holds a chord point off the rows, by any source; the measured source is the count's knife edge on a quarter arc. Over fillet radius × δ × hole radius: 26 of 168 cells refuse, up to 17 times over. A sphere's cells are near square in its chart and a cylinder or cone has no rows, so they do not reach it.

**Ruled: the rectangle lane triangulates in the lattice metric**, as the chart lane's analytic arms do (`trimmed.rs`, "The lattice metric"), through the same `AnalyticRow::metric`. Measured: all 168 cells mesh, worst certificate 0.98 δ. Every rectangle-lane torus patch's bytes move (diagonals only, counts equal), and an off-equator sphere rectangle's; the goldens re-pin.

Not taken: grid rows laid through every meridian chord point (a constraint on the triangulation to get the bound, one axis, one source); removing the count's knife edge alone (the trigger, not the class). The knife edge is filed, and so is what stays unproved: the rectangle lane sizes at δ on a theorem about lattice cells, and a triangle beside a point its grid did not place reaches past a cell with nothing but the certificate holding it.
