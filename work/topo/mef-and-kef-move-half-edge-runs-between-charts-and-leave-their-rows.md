---
id: mef-and-kef-move-half-edge-runs-between-charts-and-leave-their-rows
kind: issue
title: mef and kef move a RUN of half-edges between loops of different faces; the rows on that run keep their keys and change chart, silently where the new face mints nothing
status: open
opened: 2026-09-14
refs: [loop-reparenting-euler-ops-leave-rows-certified-against-the-wrong-chart, half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete]
---

Found and measured by the class sweep of
`loop-reparenting-euler-ops-leave-rows-certified-against-the-wrong-chart`,
which closed the same defect for the three doors that move a whole
LOOP (`kfmrh`, `mfkrh`, `ring_move`). These two move a RUN of
half-edges instead, between loops of two DIFFERENT faces, which changes
the chart of every row on that run in exactly the same way.

**The sites**, from the sweep's grep (`parent_loop = ` over
`crates/topo/src`, production code only):

- `Body::mef`'s chord surgery (`crates/topo/src/euler.rs`,
  `mef_chords`) — the run `[he1 .. he2)` moves into the NEW loop, on a
  new face whose surface is the caller's `FaceSurface`. `Inherit` and a
  `Shared` naming the old key keep the chart; `New` and any other
  `Shared` change it.
- `Body::kef`'s unsplice (`crates/topo/src/euler_kill.rs`) — the dying
  loop's `remnant` joins the mate's loop on the SURVIVING face, whose
  surface is unrelated to the dying one's.

**Measured** on the sweep's fixture (the three-face cylinder sheet of
`crates/topo/tests/loop_reparenting_pcurve_rows.rs`: two minted curved
panels and a planar back), `validate_pcurves` at
`Band::linear(Tol::witness())`:

| op | what moves | after | tier 3 |
|---|---|---|---|
| `mef` on a curved panel with `FaceSurface::New(plane)` | a run of 2 cylinder rows becomes the new PLANAR face's outer loop | new face 2 rows + 1 rowless; old face 2 rows + 1 rowless | one `MissingCache`, and it is the OLD face's newly minted half — the two re-chartered rows are not reported at all |
| `kef` on an edge between the curved panel and the planar face, killing the curved one | the dying face's remnant of 3 cylinder rows joins the planar face's loop | planar face 3 rows + 5 rowless | `[]` — **silent** |

So the shape, the silence and the reader set (`props`, the
tessellator, `chart_boundary`) are the loop row's exactly, one level
down.

**Why it was not closed with the loop doors.** The fix there is
`Body::drop_rows_on_chart_change` (`crates/topo/src/euler_ring.rs`):
compare the two faces' surface KEYS, carry every row when they agree
and drop the loop's rows when they do not, deriving nothing so the
`Decide` bound does not move. Applying it here needs the moved RUN
rather than a loop, and both ops already hold theirs in the plan phase
(`run` in `mef_chords`, `remnant` in `kef`), so `mef` is a two-line
change. `kef` is not: it never resolves the SURVIVING face, only that
loop's `face` key, so reading the target surface adds a resolution to
a core kill operator's documented precondition check order — a
decision about `kef`'s contract, not a rider on another unit's fix.
Splitting the pair would leave half a class open, so both wait here for
one unit.

Sibling rows: the loop half above, now closed by the doors carrying or
dropping; and
`half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete`,
which is `mef`'s OTHER pcurve defect (the two half-edges it mints carry
no row). A unit taking this row should read that one first: the same
`mef` call is in both, and the answers compose — the moved run's rows
go, the minted halves' rows are still nobody's.
