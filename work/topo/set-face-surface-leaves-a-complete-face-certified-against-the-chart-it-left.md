---
id: set-face-surface-leaves-a-complete-face-certified-against-the-chart-it-left
kind: issue
title: set_face_surface swaps a face onto a chart that mints nothing and leaves its complete row set behind, which validate_pcurves skips entirely
status: open
opened: 2026-09-14
refs: [loop-reparenting-euler-ops-leave-rows-certified-against-the-wrong-chart, mef-and-kef-move-half-edge-runs-between-charts-and-leave-their-rows, attach-postconditions-validate-the-whole-body-and-panic]
---

Found by PR 2549's R1 reviewer, by execution, and filed by that PR's
fix pass. It is the same mechanism as the unit that found it — a
complete row set left describing a chart its face is no longer on,
where nothing reports it — through a door that moves no loop at all.

`Body::set_face_surface` (`crates/topo/src/attach.rs`) replaces a
face's surface and touches no pcurve row. Its posture entry in
`pcurves::staleness_posture::DECLARED` reads "a surface swap is content
staleness the tier-3 pass re-certifies against, not a key the map can
lose". That is true only where the NEW surface mints:
`validate_pcurves` skips a face whose surface fails `chart_mints`, so a
swap onto a plane or a NURBS placeholder leaves a complete, wrong row
set that nothing measures.

**Measured** (PR 2549's head, the minted cylinder-wall sheet of
`crates/topo/tests/loop_reparenting_pcurve_rows.rs`,
`validate_pcurves` at `Band::linear(Tol::witness())`), as
`(rows stored, half-edges with no row)`:

| swap | after | findings |
|---|---|---|
| a minted cylinder face onto `FaceSurface::New(Plane)` | `(4, 0)` — every row kept, all four stated in the cylinder chart | `[]` — **silent** |
| the same face onto `New(other cylinder)` | `(4, 0)` | four `Certify` — loud |

So the asymmetry is the unit's exactly: onto a MINTING chart the pass
measures the rows and refuses them; onto one that mints nothing it
says nothing, and `props`, the tessellator and `chart_boundary` read
four curves about a surface the face is not on.

**Not the same row as
`attach-postconditions-validate-the-whole-body-and-panic`**, which is
also about this door: that row's subject is the cost of the tier-1
postcondition and the release-profile panic it carries, and closing it
would not touch a pcurve. This is the map's half and wants its own
answer.

**What would close it.** The same three shapes the loop doors chose
between: drop the face's rows when the new surface is not the chart
they were stated in (`Body::same_chart` is the predicate, already
written and `pub(crate)` in `crates/topo/src/euler_ring.rs`); re-state
them, which needs a derivation bound this door does not carry; or
refuse, which a setter cannot. Dropping is the cheap one and would make
the door's `DECLARED` note true as written; a caller that wanted the
rows runs `pcurves::mint_pcurves`. Whoever takes it should take
`mef-and-kef-move-half-edge-runs-between-charts-and-leave-their-rows`
with it: three doors, one answer, and the class is then closed.
