---
id: set-face-surface-leaves-a-complete-face-certified-against-the-chart-it-left
kind: issue
title: set_face_surface swaps a face onto a chart that mints nothing and leaves its complete row set behind, which validate_pcurves skips entirely
status: review
opened: 2026-09-14
refs: [loop-reparenting-euler-ops-leave-rows-certified-against-the-wrong-chart, mef-and-kef-move-half-edge-runs-between-charts-and-leave-their-rows, attach-postconditions-validate-the-whole-body-and-panic]
priority: P0
cost: D
pr: 2594
branch: topo/set-face-surface-drops-rows-on-chart-change
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

## Brief (TOPO, 2026-09-14) — block TOPO-B5 slot 0, dual at review

**The answer to give.** `Body::set_face_surface` takes the answer the
loop-re-parenting doors took (PR 2549): when the face's new surface is
not the chart its rows were stated in, the face's rows are DROPPED
through `Body::same_chart` (key-or-provenance, `pub(crate)` in
`euler_ring.rs`) and `pcurves::loop_rows`, deriving nothing, so the
`Decide` bound does not move; when it is the same chart, every row is
carried untouched. The door's `DECLARED` posture note in
`pcurves::staleness_posture` becomes true as written; a caller that
wants rows on the new chart runs `mint_pcurves`. Phase 1 confirms the
measurement in this row (a swap onto a plane keeps four cylinder rows
and tier 3 says nothing; onto another cylinder it refuses loud) and
decides where the drop lives: inside the setter, or one shared
`drop_rows_on_chart_change`-shaped door the three loop doors already
call — one home, not a fourth copy. `set_edge_curve` is the sibling
setter: say whether it has the same hole (an edge's row is stated
against its face's chart, not its curve — is a curve swap a row
staleness the tier-3 pass measures, or a silence?) and close it here
if it is the same two lines, else file.

**Rows.** Red-first: the row's own measurement — a minted cylinder
face swapped onto `FaceSurface::New(Plane)` reads `(4, 0)` rows and
`validate_pcurves` reports nothing on the merge base; at the head the
face is rowless `(0, 4)` and the pass reports the missing rows (or is
tier-3 complete again after `mint_pcurves`). Control: the same face
swapped onto `Shared(the same key)` and onto a key sharing its
`GeomSource` keeps all four rows bit-identical; onto another cylinder
the rows drop and the loud `Certify` refusals go with them. The
mutant that carries on every swap reds the red-first row; the mutant
that drops on every swap reds the control.

**Receipt.** Every caller of `set_face_surface` in the workspace with
what its rows are after the unit (the fixtures, the graft, `shell`,
the boolean, `merge_faces`, `replace_face`, the recipe layer's
re-stamp path) — any caller that swaps a chart and then reads rows
without re-minting is a defect this unit exposes; say which and file
on the owner's slate. The `attach-postconditions-validate-the-whole-body-and-panic`
row is the same door's other half (cost, panic) and is Ev's — leave
the tier-1 postcondition alone.

**Seams.** `attach.rs` is this program's; `pcurves.rs` is TRIM's
(the posture table row and `loop_rows` — announce on `work/trim/log.md`);
`euler_ring.rs`'s `same_chart` is this program's.

Branch `topo/set-face-surface-drops-rows-on-chart-change`. PR title:
"TOPO: set_face_surface drops the rows a chart swap leaves behind". Do
not close the item; the dual runs at review.
