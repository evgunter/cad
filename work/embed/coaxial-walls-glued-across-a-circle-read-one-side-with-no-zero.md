---
id: coaxial-walls-glued-across-a-circle-read-one-side-with-no-zero
kind: issue
title: Two walls of one cylinder carrier glued across a circle (the corpus bushing) refuse: the carrier arm has no departure, and the circle's plane signs one wall while the other is presented with no zero side
status: open
opened: 2026-11-21
priority: P0
cost: M
---


## Finding

Found by EMBED's lane `embed/trace-cylinder` (2026-11-21), tracing the
cylinder buckets of the per-solid census (`measure/gate/census-c2a0c41f40.json`
on `embed/gate-measure`). **20 tests** of the cylinder/cylinder edge
bucket, and likely most of the 9 slow corpus rows of that bucket not
traced (they time out under the trace): every corpus row that
evaluates the document `coincidence_door::a_bushing_stacked_on_its_own_axis_glues_its_walls`
builds (`m4_pr8_corpus` 3, `display_budget` 3, `names_verbatim_edge_evaluator`
2, `m10_p_fence` 2, and 10 more corpus rows of `editor-core::all` and
`viewer::all`). They all refuse at the same cell pair.

- **The pair.** Cylinder `1v19` against cylinder `3v1` across one shared
  edge, the circle where the two coaxial walls of equal radius meet; no
  vertex of the shared cell in reach. Both faces lie on one carrier.
- **The cells, at depth 40.** `1v19`: `u ∈ (0.288388, 0.294524)`,
  `v ∈ (1.0, 1.000977)`, presented `Of(Side(YLo))`, the circle at
  `v = 1`. `3v1`: `u ∈ (0.294524, 0.300660)`, `v ∈ (0.999023, 1.0)`,
  presented `Of(Zero::None)`: no side of its rectangle is listed as the
  zero, though its top side lies on the circle.
- **The arms.** The carrier arm cannot certify (each wall lies in the
  other's carrier, so neither departs it). The circle's own plane
  (`planes_through`) signs `1v19` (`across = 9.77e-4`, `Positive`) and
  gives `None` on `3v1`: with `Zero::None`, `sign_on` reads `ψ` by
  `centred` over the whole rectangle, which reaches `ψ = 0` on the
  side. The graph separator reads the same. The pair splits to
  `Budget(Depth)`, reported `Uncertified { arm: Departure }`.
- **One level down: not traced.** Why `3v1` lists no zero side is the
  cause, and the trace does not print its clips on this path. `zero_of`
  lists a clip as zero only when every edge on it is shared with this
  partner, so a line `v = 1` holding a second edge would read so; but no
  vertex is in reach of the pair (`near.vertices` empty), which argues
  against a split of the circle at the cells' common side
  `u = 0.294524`. Unchecked either way.

## Valid or in band

Valid: a bushing unioned onto its own axis, two walls on one cylinder
meeting along a circle (an edge the result keeps), a finished solid
its test builds as an ordinary result.

## Evidence

- Cheapest reproducer: `editor-core::all coincidence_door::a_bushing_stacked_on_its_own_axis_glues_its_walls`
  (0.31 s).
- Run on `embed/trace-cylinder-measure` with `CV_TRACE=1` (`CVT`,
  `CVG` rows): every one of the 20 traced tests prints the same two
  cells and the same `(Some(Positive), None)` under both the plane and
  the graph separator.

## Trace: why `3v1` has no zero, and what follows it (EMBED, lane embed/trace-cylinder, 2026-11-21)

On `embed/trace-cylinder-measure` with `CV_CLIPS=1` (each chart
presentation's clips and `zero_of`'s answer, `CVC` rows).

- **The circle is two arcs, both shared.** The walls meet on the circle
  `v = 1`, split at `u = 0.3` into `18v5` (`u ∈ [0.3, 2π]`) and `20v5`
  (`u ∈ [0, 0.3]`). Both arcs are in the boundary of both `1v19` and
  `3v1`. At the refusing pair the near edges are `20v5` alone
  (`near.edges = [(3, 3)]`, no vertex): `1v19`'s cell, left of
  `u = 0.2945`, is decided clear of `18v5`.
- **The missing zero.** `3v1`'s cell, `u ∈ (0.2945, 0.3007)`, holds
  both arcs (`local = [2, 3]`), so its clip `YHi` at `v = 1` lists both.
  `zero_of` takes a clip as a zero side only when every edge on it is in
  `near_edges`, and `18v5` is not: `Zero::None`. The clip's edges are
  all shared by the pair, and the circle's own plane vanishes on both
  arcs, but `On` (from `planes_through`, built over the near rims only)
  lists `20v5` alone, so membership fails at `sign_on` too.
- **Not the wedge-π seam.** Both walls lie on one carrier, so the
  carrier arm reads no departure (`across` straddles 0). The circle's
  plane does read the seam: it signs `1v19` with `across = 9.77e-4`
  over a span of `9.77e-4`, a clean first-order departure. The departure
  arm has a reading for a smooth seam of one carrier. The missing zero
  is the defect, not a symptom of an unread seam.
- **Checked by a diagnostic flip, and a second defect behind it.** With
  `CV_HACK_ZERO=1` (a measuring-branch edit, not a fix: a clip is a zero
  side when ANY of its edges is near, and `On::holds` takes any listed
  edge), the pairs split on `20v5` fall from 40 to 19 and on `18v5`
  from 132 to 92. The body still refuses on `20v5`, now at the mirror
  cell: `1v19` at `u ∈ (0.2945, 0.3007)`, `v ∈ (1.0, 1.00098)` fails
  `sides_are_the_faces`. Its clip `YLo` at `v = 1` holds both arcs,
  with ranges `[0.3, 2π]` and `[2π, 2π + 0.3]`: `20v5`'s range sits on
  the copy past the join, not on the cell's sheet `[0, 0.3]`, so the
  side `(0.2945, 0.3007)` is not covered. That exit is independent of
  the edit.

**The unit this calls for (cost M):**
- (a) `zero_of` and `planes_through` read a coordinate line whose edges
  are all edges of both faces, on one conic, as one zero side. The
  conic's plane lists every arc of that conic in the face's boundary,
  not only the near ones. **Landed otherwise** (the dated section
  below): the side is read as running past the shared cell, and the
  plane's list is unchanged.
- (b) `edge_ranges` (or `covered`) places a periodic edge's range on the
  cell's own sheet, as `sheets` places lines.

Whether (a) and (b) together certify the bushing is unchecked. The 10
corpus tests that timed out under the trace are still not traced.

## The first defect landed; the body still refuses on the second (EMBED, lane embed/clip-vertex, 2026-11-22)

**What reads now.** A clip that holds a shared edge beside one not
shared in reach is no longer `Zero::None`: `zero_of` hands it to
`partial_zero`, which reads the side as running past the shared cell
(`Zero::Past`, on the near arc), with or without a shared vertex in
reach. `sign_on` reads a `Past` side zero under a plane only where the
plane holds the listed edge's whole line or circle and that is the
side's own chart line, which a conic's own plane does, so
`planes_through` lists the near arc as before. Listing every arc of the
conic was weighed and not needed: the unshared arc is on the side's
line, which the plane holds whole, and what keeps a crossing there from
certifying is `past_read`, not the list.

**Measured** (`embed/clip-vertex-measure`, main `b99f71c3d`, the gate
logging and passing):

- `coincidence_door::a_bushing_stacked_on_its_own_axis_glues_its_walls`:
  10 asks, 4 refused `Uncertified { a: 1v19, b: 3v1, Edge(20v5), Departure }`,
  before and after. Before, the refusing pair's last cells read
  `Side(YLo)` against `Zero::None`; after, the mirror cell, `1v19` at
  `u ∈ (0.2945, 0.3007)`, exits at `sides_are_the_faces`: the second
  defect, as the diagnostic flip predicted. The three corpus rows
  measured with it keep their bushing ask refused.
- **With `embed/collinear-cover` (`f10a4f1d1`) merged**, the same test:
  10 asks, none refused, both over this lane's reading and **without
  it** (collinear-cover alone over the harness). So the cover (b) is
  what blocks the bushing, and (a) does not: with the cover read, a
  cell that holds both arcs presents, and a `Zero::None` pair beside
  the joint is split once and its halves read. The missing zero was a
  symptom at the depth floor, where the cells that could not present
  had spent the budget. What (a) changes alone is the count of splits.

**Pinned** by `injective::presentation_rows::two_walls_glued_across_a_circle_of_two_arcs_certify_beside_its_joint`
(the pair of cells above, at the door `certify_pair`) and, for the
unsound direction, `a_face_through_the_unshared_arc_of_a_side_is_not_certified`.
No body row: two full-turn rods stacked and turned `0.3`
(`sweep::revolve`, united) hold on main already, in 268 splits against
264 with the reading, because their cells across the joint cover their
sides. The corpus bushing is the only measured body that reaches the
case.
