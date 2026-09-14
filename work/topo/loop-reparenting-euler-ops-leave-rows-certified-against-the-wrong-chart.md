---
id: loop-reparenting-euler-ops-leave-rows-certified-against-the-wrong-chart
kind: unit
title: kfmrh and ring_move re-parent a loop onto a face with a different chart; its pcurve rows keep their keys and lose their meaning, and tier 3 is silent whenever the target is planar
status: dispatched
opened: 2026-09-14
pr: 2549
branch: topo/loop-reparenting-rows
---

Found by PR 2531's reviewers (R2 by execution on `kfmrh`, R1 by reading
`ring_move`) and re-measured by that PR's fix pass, which owed the
class receipt a row per loop-RE-PARENTING op.

**The mechanism.** A pcurve row is a curve stated in a FACE's chart, and
it is keyed on a half-edge. An op that moves a LOOP from one face to
another changes which chart every row on that loop is about, without
touching a single key. Two doors do it:

- `Body::kfmrh` (`crates/topo/src/euler_kill.rs`) — the second face's
  outer loop becomes a ring of the first;
- `Body::ring_move` (`crates/topo/src/euler_ring.rs`) — a ring moves
  between two faces of one shell.

**Measured** on the minted cylinder-wall sheet (the fixture of
`crates/topo/tests/split_edge_pcurve_rows.rs`, with the `mef` face put
on a fresh PLANE), `validate_pcurves` at `Band::linear(Tol::witness())`:

| op | what moves | after | tier 3 |
|---|---|---|---|
| `kfmrh(plane, cyl)` | the minted cylinder loop becomes a ring of a PLANAR face | 4 rows, all certified against the cylinder chart, on a face whose surface is a plane | `[]` — **silent** |
| `kfmrh(cyl, plane)` | the plane's rowless loop becomes a ring of the minted cylinder face | 4 rows + 4 rowless half-edges on one curved face | 4 × `MissingCache` — loud |
| `ring_move(ring of cylinder rows, PLANE face)` | the same rows again, through the other door | 4 cylinder rows on a planar face | `[]` — **silent** |
| `ring_move(rowless ring, CYLINDER face)` | a planar-origin ring onto a minted curved face | the curved face reads incomplete | `MissingCache` — loud |

So the two directions are not symmetric, and only one of them is
caught. Onto a CURVED face the target reads incomplete and tier 3 says
so. Onto a PLANAR face nothing is reported at all —
`validate_pcurves` skips a face whose surface does not `chart_mints`,
so rows that now describe a curve in a chart the face is not on are
accepted unmeasured. They are not stale by key and no walk will drop
them: `mint_pcurves` clears and re-derives, but a body that is never
re-minted keeps them, and every reader that trusts a stored row
(`props`, the tessellator, `chart_boundary`) reads a row about the
wrong surface.

**What the declared postures say today.** `pcurves.rs`'s
`staleness_posture::DECLARED` files both doors under `Neither`, with
`ring_move`'s note reading "ring surgery: re-parents a ring, mints no
half-edge" — true as written, and the reason the case was missed is
that no entry claims anything about MEANING. What a fix decides is
whether these doors should refuse, clear the moved loop's rows, or
re-certify them against the target chart (the third is
`split_edge`'s answer, and it is available here only where the door
has the bound).

Related: `work/trim/validate-pcurves-never-recertifies-a-face-it-finds-incomplete.md`
(the same pass, skipping passes 2 and 3 on an incomplete face) and
`work/topo/half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete.md`
(the minting half of the same surface).

## Brief (TOPO, 2026-09-14) — block TOPO-B3 slot 0, dual at review

**The answer to give.** After `Body::kfmrh` or `Body::ring_move` moves
a loop onto a face, no pcurve row on that loop describes a curve in a
chart the face is not on. The doors have two honest shapes and phase
1 picks per door, per direction, with the measurement in the item's
table as the red-first evidence: (a) DROP the moved loop's rows when
the destination face's surface key differs from the source's — the
existing tier-3 pass then reads the curved destination as incomplete
(`MissingCache`, loud) and a planar destination needs no rows
(`chart_mints` refuses planes) — keep them when the surface is the
same (`kfmrh`'s same-surface fusion, `ring_move` within one chart);
(b) RE-STATE them where the door has the bound and the parent row is
a restriction (it is not: a re-parented loop's rows are about a
different surface, so there is nothing to restrict — say so if phase
1 agrees, and take (a)). Refusing is not an option the row leaves
open: both doors are legal topology on a body whose rows are a cache.

**Rows.** The item's four-row table as tests: `kfmrh(plane, cyl)` and
`ring_move(ring of cylinder rows, plane)` — red-first: on the merge
base the rows survive under a planar face and tier 3 is `[]`; at the
head the rows are gone and tier 3 is `[]` for the right reason (a
planar face carries none). `kfmrh(cyl, plane)` and `ring_move(rowless
ring, cyl)` — the loud direction stays loud (`MissingCache`) on both
trees. Same-surface moves keep every row byte for byte. A row that
re-mints (`mint_pcurves`) after each move re-derives exactly the head's
surviving rows.

**Posture.** `pcurves.rs`'s `staleness_posture::DECLARED` files both
doors under `Neither`; re-state each note to say what the door does
with the moved loop's rows (TRIM's file — one note per door by
announced seam). The class receipt: every loop-re-parenting site
(`kfmrh`, `mfkrh`/`mfkrh_plug`, `ring_move`, `movefac`,
`move_shells_to_new_solid`, the merge door's ring drain) with its
disposition measured, not asserted.

Branch `topo/loop-reparenting-rows`. PR title: "TOPO: a re-parented
loop carries no rows about the chart it left". Do not close the item;
the dual runs at review.
