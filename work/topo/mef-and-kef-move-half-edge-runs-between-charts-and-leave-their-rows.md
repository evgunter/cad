---
id: mef-and-kef-move-half-edge-runs-between-charts-and-leave-their-rows
kind: issue
title: mef and kef move a RUN of half-edges between loops of different faces; the rows on that run keep their keys and change chart, silently where the new face mints nothing
status: review
opened: 2026-09-14
refs: [loop-reparenting-euler-ops-leave-rows-certified-against-the-wrong-chart, half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete]
priority: P0
cost: H
branch: topo/mef-kef-runs-carry-or-drop-rows
pr: 2603
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

## Brief (TOPO, 2026-09-14) — block TOPO-B5 slot 1, dual at review

**The answer to give.** `Body::mef`'s chord surgery and `Body::kef`'s
unsplice take the loop-re-parenting doors' answer one level down: the
RUN of half-edges that moves between two faces' loops carries its rows
when the two faces share a chart (`Body::same_chart`, key-or-provenance)
and drops them when they do not — deriving nothing, `Decide` bound
unmoved. `mef` holds its run in the plan phase (`run` in `mef_chords`;
`Inherit` and a `Shared` naming the old key keep the chart, `New` and
any other `Shared` change it). `kef` never resolves the surviving face
today: phase 1 decides how the target surface is read inside a core
kill operator's documented precondition order — a resolution added to
the plan phase (and what its refusal is, typed, if the face does not
resolve — a tier-1 corruption the operator already refuses elsewhere?)
— and says why, from `kef`'s contract, not from convenience. Read
`half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete`
first: it is `mef`'s OTHER pcurve defect (the two halves it mints carry
no row) and is Ev's question on the open `[ev]` PR — this unit moves
the run's rows and leaves the minted halves' posture where it is; say
so in the PR body and do not decide the posture.

**Rows.** Red-first, both measured on the row's fixture (the
three-face cylinder sheet of `loop_reparenting_pcurve_rows.rs`,
`validate_pcurves` at `Band::linear(Tol::witness())`): `mef` on a
curved panel with `FaceSurface::New(plane)` — on the merge base the two
re-chartered rows ride onto the planar face unreported beside one
`MissingCache`; at the head they are dropped and reported. `kef`
killing the curved panel into the planar face — on the merge base
three cylinder rows join the planar loop and tier 3 is silent; at the
head they drop. Controls: `mef` with `Inherit` and with `Shared(old
key)` carries every row bit-identical; `kef` between two faces on one
chart carries. Mutants: carry-always reds the red-first rows,
drop-always reds the controls. The existing `mef`/`kef` rows (the
Euler suites, `review_d18`'s hammer exposure table — re-derive its
counts if a plan-phase resolution changes what the hammer reaches) stay
green or are re-baselined with what moved said.

**Receipt.** Every `parent_loop = ` writer in production code, with
which door now carries or drops (the three loop doors, these two, and
any left — the sweep that found this row is the pattern; re-run it at
the head and say what is still outside). The class closes here or the
PR body says what is left and where it is filed.

**Seams.** `euler.rs` and `euler_kill.rs` are this program's;
`pcurves.rs`'s `loop_rows` (TRIM's) is read, and edited only if the
run needs a `run_rows` twin — announce on `work/trim/log.md`.

Branch `topo/mef-kef-runs-carry-or-drop-rows`. PR title: "TOPO: mef
and kef carry or drop the rows of the run they move". Do not close the
item; the dual runs at review.
