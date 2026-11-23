---
id: the-mesh-names-a-horns-cusp-copy-off-a-chord-point-beside-the-stored-lift
kind: issue
title: the mesh names a horn's cusp copy (v = ±π) off its own chord point and Chart::poles' one listing, beside the stored lift that holds it
status: open
opened: 2026-11-23
priority: P1
cost: M
refs: [a-horns-cusp-reads-as-its-upper-apex-so-a-face-below-it-turns-the-wrong-way, mesh-pole-junction-unwrap-picks-a-whole-turn-off-numbers]
---


Found by CHARTTAIL's horn-cusp unit (`charttail/horn-cusp`, 2026-11-23),
sweeping for readers that take one of several singular points at one
location.

**The fact.** A horn's two sheet apexes are one cusp, at `v = ±π` alike,
and which copy a face reaches is the face's, not the point's. Topo now
decides it once, off the joint's chart ends: `pcurves::decision`'s
`apex_reached` reads the sign of each stored image's `v` at the cusp end
(`pcurve_sheet_apex_copy`), and the joint's element, so the stored lift
(`Body::loop_lift`), holds the result.

**The mesh derives it again, twice over, beside the lift.**
- `topo::chart::Chart::poles` lists a horn's cusp once, at `+π` (it filters
  `SheetApexDown` when `Chart::is_horn`): a first-of-two pick at one point.
- `mesh::walk`'s loop walk (the pole-junction arm, "A horn's one pole is
  its cusp") patches that pick: `vp.copysign(chart.v_of(positions[cur.ids[1]]))`,
  the sign of `v` at the first chord point of the meridian leaving the
  cusp, an f64 read of a 3-D sample. It agrees with the stored image's copy
  on every suite body (the football rows mesh and measure), but it is a
  second derivation of the copy the lift stores.
- `mesh::chart_table::AnalyticRow::new` builds its `SingularPoint` rows from
  `Chart::poles`, so a horn gets one row, `RowEnd::Top`, and
  `SingularPoint::passed_by` (read by `trimmed::singular_row_shape`'s
  `PastPole` refusal) guards only `v > π`: a horn face reaching the cusp
  from below has no row guarding `v < −π`.

**Can the two share one home.** Not by sharing a function: the walk reads
a 3-D sample in f64 and topo decides a stored image's end at a band, and a
shared `sign(v) → apex` helper would be thinner than either call site. The
home is the stored lift, as `wrap_strip`'s period became (PR 7202): the
walk names the copy off the lifted row's `v` at the cusp end
(`LiftedRow::entry_exit_points`), the value topo decided from, and
`Chart::poles` lists the cusp at both copies (or the trimmed lane reads
the copy off the lift too) so the past-pole guard covers the face's own
side. That is the same move `mesh-pole-junction-unwrap-picks-a-whole-turn-off-numbers`
owes for `unwrap_tie` at a pole junction, and can land with it.

## A third derivation: props (PR 7511 review, 2026-11-23)

`geom_brep::props::curved`'s `sheet_apex_jumps` derives the same fact
again: which apex a loop passes, and with the face on which side, from
the arriving meridian's `rising` (it arrives up the tube at the upper
apex, down it at the lower), with the face's side from the sense bit
(`ApexSide`). It reads the arriving piece only, so it takes the two
pieces to agree. That agreement is what topo now checks and refuses as
`SingularMiss::CuspCrossed` where it fails. It agrees with the stored lift
on every suite body: the football rows measure their exact volume.

Props is not different in kind. It reads `LoopEdge`s rather than a body's
stored rows, because `geom-brep` sits below `topo`, so it cannot call
`Body::loop_lift`. But its caller in `topo` has the lift and could hand
props the apex copy each pass reaches, or the lifted chain itself. "Read
the copy off the lift" therefore covers props too. The repair crosses
`geom-brep`'s props, which belongs to the program that owns
`crates/geom-brep/src/props/`. A lane taking this row should read
`python3 scripts/work.py territory` for that file and say so on that
program's log.
