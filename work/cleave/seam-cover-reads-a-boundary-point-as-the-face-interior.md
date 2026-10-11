---
id: seam-cover-reads-a-boundary-point-as-the-face-interior
kind: issue
title: seam cover reads a touch piece on a face's boundary as its interior, and ships a decided SeamFaceRunsOn contradiction
status: closed
opened: 2026-10-11
priority: P2
cost: E
branch: cleave/seam-cover-boundary
closed: 2026-10-11
pr: 4734
---

Found by the review of PR 4613 (CLEAVE mints unit 4).

## What

The seam cover reads each touch piece of a locus at its midpoint, and
asks a face that does not ride the piece whether it contains it:

- `crates/topo/src/boolean/rim_wedge.rs:546` (`contains`) answers
  `Some(true)` for every placement but `Out`, so a midpoint the face's
  containment door places `OnEdge` or `OnVertex` (on its BOUNDARY)
  reads as contained.
- `rim_wedge.rs:344` (`departures`): a piece both faces contain where
  one has no riding edge is `Departure::NoEdge`, "the locus runs
  through its interior".
- `crates/topo/src/boolean/mod.rs:5806` and `:5903` turn `NoEdge` into
  the DECIDED contradiction `SeamFaceRunsOn` (`seam_locus_no_edge`).

So a midpoint the door reads on the face's boundary, not in its
interior, becomes a definite contradiction of the declared seam. A
boundary reading answers neither "interior" nor "outside" for the
question `NoEdge` asks.

## Repair shape

`contains` keeps the door's placement (`In`, `Out`, on the boundary),
and `departures` decides what a boundary midpoint means for the
piece: a riding edge the cut walk missed (a cut or ride defect), or a
piece the read does not place (`EdgeKindUnread`). Not a decided
`NoEdge`.

The arc-end case (`ContainError::ArcEndClearOfCorners`) already routes
to `EdgeKindUnread` (PR 4613).

## Built (branch cleave/seam-cover-boundary)

`rim_wedge::contains` answers `In` → `Some(true)`, `Out` → `Some(false)`,
and `OnEdge`/`OnVertex` → `None` (not placed), so the piece is
`Cover::Unread` and `departures` refuses the claim as outside its
envelope (`Departure::EdgeKindUnread`, the claim's `unsupported`), as
the arc-end arm already did. A boundary reading at a piece's midpoint
is the door and the cut walk parting (the walk cuts every point where
the boundary meets the locus), so it places nothing. It is not typed a
defect: valid geometry reaches it (a tangent arc, a barely turning
corner, a cut within the band at `K < 2`), and the walk reads a ride's
span at its carrier ends while the door reads the stored corners, which
part by up to the band without any bug.

Measured on `origin/main` at 1e-9, 1e-6 and 1e-12 (all identical):

- line locus, three poses, an arc tangent at the piece's start and a
  corner turning off it by `4K·zero`: midpoint `0.25`/`0.5` zero off the
  boundary → `NoEdge`; `3` zero → the door's escalation
  (`bool_contact_arc`/`bool_contact_edge`); `20` zero → `Untouched` /
  `Opposite`.
- corner midpoints at `K = 1.25` (piece `1.5·zero`): `NoEdge`.
- through `verify_tangency_declaration` (shallow-cornered block top ×
  a cylinder wall sheet tangent along the ride): `SeamContradicted {
  fact: SeamFaceRunsOn }` at `0.5` zero, verified at `20` zero.

After: every on-boundary row is `EdgeKindUnread` /
`UnsupportedDeclarationClass { class: Seam }`; the in-band and clear rows
are unchanged. Pinned in `rim_wedge::boundary_midpoint_rows` (line and
rim families, corner rows, the door row), red on the old reading at all
three ε.

No public operation surfaced the contradiction: the declaration door's
only production caller, `glue::tangency`, reads any refusal as "not
glued", before and after.

## Closed (PR 4734, 2026-10-11)

`rim_wedge::contains` maps a boundary placement to "not read", so the piece goes to `EdgeKindUnread` and the claim refuses typed, the same route the arc-end case takes. The seam check no longer manufactures a decided `SeamFaceRunsOn`. There is no defect arm: the walk's span ends and the door's corner vertices can differ by the band with no bug. Pinned on lines, corners, a rim and end-to-end, at every ε.
