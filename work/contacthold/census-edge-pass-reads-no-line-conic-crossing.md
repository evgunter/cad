---
id: census-edge-pass-reads-no-line-conic-crossing
kind: issue
title: The census's edge passes never read a line crossing a coplanar conic boundary arc as an event
status: closed
opened: 2026-10-08
priority: P3
cost: M
pr: 4559
closed: 2026-10-10
---


Found by CONTACT-12 (branch `contact/12-ef-crossing-cuts`).

`snapshot` (`crates/topo/src/census.rs`) keeps only `Line` edges, so
pass 5 (`sweep_edge_edge`) never pairs a line edge with a circle,
ellipse or spiric edge. A line edge of one part that crosses a coplanar
conic boundary arc of another part's planar face is therefore no
`EdgeEdgeCross` of its own. The edge-on-face lane now cuts the line at
that crossing (`boundary_crossings`, `CutAt::ConicCrossing`). The cell
bound there has no rung (`ef_bound_backed` answers `false`), so a
declared seat whose straight edge crosses a cap's rim refuses with an
`UndeclaredContact` `EdgeFaceOverlap` that no declaration can answer.
That outcome is loud and typed, but the vocabulary does not fit it: the
event underneath is the crossing.

Whether this wants a line × conic crossing class, or nothing because
D10's INTENT stage 4 retires declared pairs, is the owner's call. The
lane-level gap (pass 5 sees no curved edge) stands either way.

Parked 2026-10-09 at CONTACT's close-out: the row's symptom is a declared seat that no declaration can answer, which D10's stage-4 unit `declared-pairs-retire` removes; the lane gap is re-measured when that unit lands. (CONTACT orchestrator)

## Measured, and closed with evidence (2026-10-10, CONTACTHOLD)

F does not gate this row: the census's face-pair rungs read the op's
curve and patch `ContactRecords` (`Declared::index`), not user
declarations, so `declared-pairs-retire` leaves every reader here in
place, and since E an undeclared flush seat makes the same records.

Measured on main after E and B2 merged: a half-disc post
(`tier3_tests::half_disc(0.25, 0.5, (0.3, 0.2, 0.0))`) under the shelf
`[0, 0.9] × [0, 0.30] × [0.5, 0.54]`, whose `y = 0.30` underside edge
crosses the cap's rim at `x ≈ 0.071` and `x ≈ 0.529`.

- With a `PatchContact` (cap, underside) the census refuses with the
  row's `EdgeFaceOverlap` (shelf edge on the cap, witness
  `(0.3, 0.3, 0.5)`), and ALSO with `CensusUnsupported { FacePair(cap,
  underside), ChartRegion(NonPlanarTrim) }` from the confirm pass's
  Door 2, and with `CensusUndecidable` for the cylinder wall against the
  shelf's faces from the cross-solid backstop. Adding the rim's
  `CurveContact` (wall, underside) changes neither: the record itself is
  `ContactContradicted` (a wall square to the plane is no tangency).
- So a rung for the conic crossing cannot back anything today. Any
  region-confined rung needs `pair_region_verified` on a pair one of
  whose faces is the arc-bounded cap, and Door 2 refuses that face
  `NonPlanarTrim`. Filed on CHART's slate as
  `a-planar-face-with-a-conic-trim-has-no-region-door`.
- The lane gap is not a silent clear. A line edge crossing a rim
  transversally (a wedge whose edge touches the half-disc only at the
  rim point `(0.3, 0.45, 0.5)`): pass 4 reads the pierce `OnEdge` and
  reports nothing, pass 5 never sees the arc, and the backstop refuses
  `CensusUndecidable` on the wall and the cap against all three wedge
  faces. A coplanar crossing is refused by the overlap cell itself.
  Every census verdict on these configurations is a refusal, so the
  missing event costs vocabulary, not soundness.

No code change. The row closes at the PR's merge; if the chart row
lands, a line × conic crossing rung is a new row against the measured
seat above.

## Closed (2026-10-10, PR 4559)

Merged after a full single review and a verified fix pass.
