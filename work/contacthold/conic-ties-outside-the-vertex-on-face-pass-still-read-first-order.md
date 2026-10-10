---
id: conic-ties-outside-the-vertex-on-face-pass-still-read-first-order
kind: issue
title: Outside the vertex-on-face pass a conic tangent to a face still reads On at first order: the sector passes, recl's flank readings, curved faces and fitted edges
status: open
opened: 2026-10-10
priority: P2
cost: H
refs: [boolean-conic-side-code-zero-is-first-order, vtxfac-tangent-sector-should-descend-per-bound]
---


## Finding

`boolean-conic-side-code-zero-is-first-order` reads a conic's
first-order tie with a PLANE one order down in the vertex-on-face pass
only (`sectors::pierce_side`, called from `vtxfac::classify_vertex_on_face`
and `vtxfac::partner_side`). Every other reading of a conic bound still
returns `On` at the tie:

1. **The sector passes** (`sectors::side_code` through `plane_side_code`:
   `pair_codes`, `wedge_classes`, `cone_side`) **and recl's flank
   readings** (`recl::flank_key`, `resolve_edge_edge`'s membership,
   `wedge_is_reflex`'s `bool_wedge_reflex`, which before INTENT's E
   refused the `flush` posture's union at margin `-6.1e-17`, the arc's
   tangent). Descending there was measured on the fix's branch: 15 sweep
   rows (`join2_r2_probes`, `rest_zip_admission`, `reach_continuation`,
   `rest_nested_strut`) went from building to `CurvedBooleanUnsupported`
   in `recl::require_same`, because the on-edge and tangent machinery
   keys on first-order ties: an arc tangent to a plane reads as an
   on-edge event there, and its neighbours' records are built on it.
   Descending these needs those gateways keyed on first-order ties
   (as the vertex-on-face pass's whole-sector lump now is,
   `PierceSide::tied`) with the descended codes carried beside them.
2. **A curved face.** Against a cylinder, sphere, cone or torus a tie
   means the bound rides the face to first order, and which way it
   leaves is the two carriers' relative curvature along the tangent
   (`sectors::tangent_relative_side`'s quantity), which needs the face's
   second fundamental form at the vertex in the reading, and a bound on
   the face's third-order remainder for a certified verdict.
3. **A fitted (NURBS) edge's chord** (`Reach::Extent`): a secant. A
   Zero says the far end is in the band; where the edge goes between is
   not read. Its descent needs a certified jet and curvature bound of
   the fitted carrier.
4. **`recl::flank_rep`'s offset off the common line** (`bool_flank_offset`):
   a conic whose tangent runs along the line has a zero projection and
   refuses, though its bend carries it off the line.
5. **The vertex-on-face pass's tangent sectors**: a sector tangent to
   the pierced face whose bounds both tie lumps whole
   (`sectors::tangent_lump`), and a conic bound's own descended side is
   overwritten by the lump. Reading such sectors per bound instead
   breaks `wall_face_tangent_reach`'s subtract (`BisectorSide`: the
   wall sector's bisector rides the tangency locus between two bounds
   read `Out`), and is
   `work/tang/vtxfac-tangent-sector-should-descend-per-bound.md`'s.

A direction-only reading is listed so the next sweep does not re-find
it: `sectors::arc_side` (`bool_cone_arc`) reads a bound's direction
against a great arc of the cone of directions, and a conic tangent to
that arc ties there whatever its bend. Whether that can decide a verdict
is unmeasured.

Fixture: `crates/sweep/tests/an_arc_tangent_at_a_boolean_vertex.rs`
(the `flush` and `corner` postures reach the vertex-vertex pass's
`pair_codes` with the arc tied; their refusal is not this one,
`a-flush-quarter-round-refuses-an-odd-crossing-count-at-its-rim-vertex`).
