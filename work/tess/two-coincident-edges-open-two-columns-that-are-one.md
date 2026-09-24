---
id: two-coincident-edges-open-two-columns-that-are-one
kind: issue
title: mesh: two coincident edges open two columns that are one, and the face meshes as a hole
status: open
opened: 2026-09-22
---


Found by TESS-5's reviewer, who constructed the body TESS-5's sweep had
named as the blind spot of its guard and argued (without proof) was not
a valid body. It is reachable through the Euler doors, so the argument
was wrong and the residue is real. P1, cost D.

**The body.** A sphere slit bounded by two COINCIDENT edges: `mvfs` at
the north pole, `mev` along a great-circle arc to the south pole, then
`mef` (`MefSite::Chords`, `he_plus`/`he_minus` of that edge) with the
SAME `arc_of_circle` again — V2 / E2 / F2, two distinct edge keys on one
carrier between one pair of vertices. Tier 3 refuses it
(`ScaffoldAtRest ×2`), and `tessellate` does not re-validate.

**Measured on TESS-5's head** (`05063dc76`, default ε, δ = 0.05):

- both doors `curved::require_iso_rectangle_face` cites admit it;
- `walk::require_two_columns` ADMITS it: both poles are junctions, so
  both traversals open, and the two openings carry two distinct EDGE
  keys — the guard's premise, satisfied;
- the two edges state the same column (one carrier ⟹ one
  `chart_iso::mid_azimuth`), so the walked domain has zero width;
- debug assertions on: panic at the cross-face census, `chord segment
  (0, 2)` an edge of 0 face triangles. Off: `Ok`, `patches [0, 0]`,
  `check_mesh` = `Err(NoTriangles)` (TESS-4's arm; `tessellate` runs
  `check_mesh` in no build).

Rowed where it is reached, so the state is recorded rather than only
described: `crates/mesh/tests/loops_with_no_rim.rs`,
`two_coincident_edges_still_walk_to_zero_width_and_this_is_what_answers`
(both profiles pinned).

**Which structural rung is missing.** TESS-5's guard refuses a rim-free
loop whose iso-side openings are carried by ONE edge, on the reading
that one edge states one column. That is sound and it is not enough: two
DISTINCT edges can state one column too, when they share a carrier. The
rung above edge identity is CARRIER identity — two openings whose edges
resolve to the same curve geometry are one column, and the body-level
statement of that is "two coincident edges", which tier 3 already
refuses under another name (`ScaffoldAtRest`). So there are two honest
routes and they are different decisions:

1. **Tier-2/3's**: a body with two coincident edges is invalid, and tier
   3 says so today for this body. If the mesh lane is allowed to assume
   what tier 3 refuses, this row is a contract question (does
   `tessellate` require a tier-3-valid body?) and not a guard. D9 and
   `tessellate`'s contract say it does NOT re-validate, which is why
   TESS-1 and TESS-5 refused their own members rather than lean on a
   door in front of them — so this route needs the contract changed, not
   just a citation.
2. **The walk's**: compare the openings' carriers, or their `u_raw`
   values. The reviewer's note, recorded because it is the tempting
   shape and it is wrong for this guard: comparing the openings' `u_raw`
   **bitwise** would separate these two bodies and would need no ε at
   all (one carrier gives one `mid_azimuth`, bitwise). But it is a
   coordinate comparison, not the loop's incidence, and a zero-width
   refusal decided on two floats being equal is the shape D2 row 1 and
   TESS-1's precedent both steer away from. Carrier identity
   (`Edge::curve` keys, or the certified geometry behind them) is the
   incidence-shaped version of the same test and is where a unit taking
   this row should start.

**Reach.** The Euler doors, and any caller that meshes without
validating. Not import: an exporter stating one arc twice between one
vertex pair is what the normalization row
(`work/exch/import-normalizes-the-rim-only-cap.md`) is about for the
other member, and nothing has measured this one through the STEP door.
Not the cylinder or torus twin: with no pole, both junctions of a
coincident-edge slit are continuations, the loop opens one iso side, and
TESS-5's guard refuses it (measured by the reviewer). **The residue is
the sphere's and the cone's.**
